"""Builds the 3D model of the SPAD S.VII C.1 in Blender (ADR 0007, plan
docs/plans/m3-modele-spad7.md).

    blender -b --factory-startup --python art/spad7/build.py -- \\
        <model.json> <model.glb> [<model.blend>]

<model.json> is the model data written by `cargo run -p mockup -- spad7 --json`: the
contract (hinges, propeller, required nodes), the lifting surfaces of aircraft.ron and
the provisional shapes of maquette.ron. Everything here is built in the flight model's
body axes (x forward, y right, z down, metres, from the datum of aircraft.ron) and turned
into Blender's axes by `blender_point` (ADR 0006: nose to -Y, top to +Z, right wing to
-X). The glTF export with +Y up then gives nose to +Z, as the game expects.

Numbers marked E: are display estimates, to be replaced by the drawing's digitised
outlines and the appearance sheet (docs/recherche/spad7-aspect.md). The fabric is painted
from the first livery of livrees.ron into generated textures (pass 5): a colour map and a
normal map, each surface's faces projected flat on a shared atlas.
"""

import json
import math
import sys
from contextlib import contextmanager

import bmesh
import bpy
import numpy as np
from mathutils import Matrix, Vector

MIN_BLENDER = (4, 0, 0)
DATA_VERSION = 2
COPYRIGHT = "Panache blanc, CC-BY-SA 4.0 (LICENSE-ASSETS)"

# Wing section "Spad", NACA Report 93, ref. 49 (MIT, 1918): ordinates in % of chord above
# the reference line (docs/recherche/profil-spad7.md § 2.2; S: confidence B for the S.VII).
SPAD_X = [0, 1.25, 2.5, 5, 7.5, 10, 15, 20, 30, 40, 50, 60, 70, 80, 90, 95, 100]
SPAD_UPPER = [0.26, 1.55, 2.14, 3.00, 3.75, 4.37, 5.31, 5.95, 6.60, 6.67, 6.28, 5.53,
              4.51, 3.26, 1.80, 1.06, 0.32]
SPAD_LOWER = [0.26, 0.07, 0.03, 0.08, 0.16, 0.26, 0.55, 0.85, 1.25, 1.31, 1.10, 0.82,
              0.51, 0.29, 0.15, 0.07, 0.0]

TAIL_THICKNESS = 0.05      # E: thin symmetric tail sections (spad_empennage.ron: RAF 15 uncambered)
TIP_RADIUS = 0.10          # replaced by the feature "wing_tip_radius" of maquette.ron (main)
CHORD_POINTS = 28          # points per surface of a section
STRUT_CHORD, STRUT_THICK = 0.10, 0.035     # E: streamlined interplane struts
CABANE_CHORD, CABANE_THICK = 0.06, 0.025  # E: steel cabane struts in a fairing
REAR_LEG_CHORD, LEG_THICK = 0.07, 0.035    # E: rear undercarriage leg (a single line on the drawing), thickness of both legs
SKID_SECTION = (0.05, 0.025)               # E: tail skid, an ash leaf
EXHAUST_STUB = 0.05                        # E: diameter of the exhaust stubs and collector
RIM_DIAMETER = 0.05                        # E: padded rim of the cockpit
COCKPIT_FLOOR = -0.30                      # E: z of the floor of the cockpit tub
FUSELAGE_SPLIT_X = -1.50                   # E: the rear fuselage breaks off behind the cockpit's rear rim (x −1.35, T7), the M4 plan's part `fuselage_rear`
COWL_END_X = -0.74                         # C: the cowl's top ends at the front edge of the cockpit opening (x −0.74, table T7)
PITCH_RATIO = 0.81                         # C: Éclair type 6, 2.40 × 1.95 m (aircraft.ron)
GUN_JACKET = (0.11, 1.05)                  # E: Vickers cooling jacket diameter and length, m


# --- Frames (ADR 0006) -------------------------------------------------------------------

def blender_point(p):
    """Body axes to Blender's: (x, y, z) -> (-y, -x, -z)."""
    return Vector((-p[1], -p[0], -p[2]))


def body_vector(v):
    """Blender's axes to body axes, the inverse of blender_point (a rotation)."""
    return Vector((-v[1], -v[0], -v[2]))


def feature(data, name):
    """Rows of a named table of maquette.ron (`features`)."""
    try:
        return data["mockup"]["features"][name]
    except KeyError:
        raise SystemExit(f"maquette.ron has no feature {name!r}")


def node_matrix(origin, gltf_x, gltf_y):
    """World matrix of a Blender object whose exported glTF node has its local +X along
    `gltf_x` and its local +Y along `gltf_y` (body axes). The exporter maps Blender's
    local X to glTF X, local Z to glTF Y and local Y to glTF -Z."""
    gx = Vector(gltf_x).normalized()
    gy = Vector(gltf_y)
    gy = (gy - gx * gy.dot(gx)).normalized()
    gz = gx.cross(gy)
    bx, bz, by = blender_point(gx), blender_point(gy), blender_point(-gz)
    m = Matrix((bx, by, bz)).transposed().to_4x4()
    m.translation = blender_point(origin)
    return m


# --- Mesh helpers ------------------------------------------------------------------------

def new_object(name, parent=None, matrix=None):
    mesh = bpy.data.meshes.new(name)
    obj = bpy.data.objects.new(name, mesh)
    bpy.context.collection.objects.link(obj)
    if matrix is not None:
        obj.matrix_world = matrix
    if parent is not None:
        obj.parent = parent
        obj.matrix_parent_inverse = parent.matrix_world.inverted()
    return obj


def new_empty(name, parent=None, matrix=None):
    obj = bpy.data.objects.new(name, None)
    bpy.context.collection.objects.link(obj)
    if matrix is not None:
        obj.matrix_world = matrix
    if parent is not None:
        obj.parent = parent
        obj.matrix_parent_inverse = parent.matrix_world.inverted()
    return obj


PAINTED = []     # surfaces whose fabric is painted (aircraft.ron names, "fuselage")
CURRENT = [0]    # index + 1 in PAINTED of the surface being built, 0 for none


@contextmanager
def painted(surface):
    """Faces built inside are the fabric of `surface`, textured from its regions of the
    atlas (one per side)."""
    if surface not in PAINTED:
        PAINTED.append(surface)
    previous = CURRENT[0]
    CURRENT[0] = PAINTED.index(surface) + 1
    try:
        yield
    finally:
        CURRENT[0] = previous


PART_NAMES = ["fuselage"]   # damageable parts of the airframe, or rules (M4 plan, step 1)
CURRENT_PART = [0]          # index in PART_NAMES of the part being built


@contextmanager
def part(name):
    """Faces built inside belong to the damageable part `name`, a node of its own
    (docs/plans/m4-degats-modele-3d.md): a name, `{lr}` standing for the side, or a rule
    that resolve_part() turns into a name from where the face is."""
    if name not in PART_NAMES:
        PART_NAMES.append(name)
    previous = CURRENT_PART[0]
    CURRENT_PART[0] = PART_NAMES.index(name)
    try:
        yield
    finally:
        CURRENT_PART[0] = previous


SPLIT = {}   # stations of the parts' cuts, set by main(): span of the struts, fuselage


def resolve_part(rule, c):
    """The part of a face from its rule and its centre `c` (body axes)."""
    lr = "l" if c.y < 0 else "r"
    y = abs(c.y)
    inner, outer, spar_mid = SPLIT["inner"], SPLIT["outer"], SPLIT["spar_mid"]
    if rule == "fuselage":
        return "fuselage_front" if c.x > SPLIT["fuselage"] else "fuselage_rear"
    if rule in ("surface:upper_wing", "surface:lower_wing"):
        plane = rule.split(":")[1].split("_")[0]
        panel = "root" if y < inner else "mid" if y < outer else "tip"
        return f"wing_{plane}_{lr}_{panel}"
    if rule == "surface:tailplane":
        return f"tailplane_{lr}"
    if rule == "surface:fin":
        return "fin"
    if rule == "strut":
        io = "inner" if abs(y - inner) < abs(y - outer) else "outer"
        return f"strut_{lr}_{io}_{'front' if c.x > spar_mid else 'rear'}"
    if rule == "cabane":
        return f"cabane_{lr}_{'front' if c.x > spar_mid else 'rear'}"
    return rule.replace("{lr}", lr)


class Builder:
    """Accumulates faces in body axes, then writes them into an object's mesh, in the
    object's local axes."""

    def __init__(self):
        self.bm = bmesh.new()
        self.material_index = {}
        # Face layers, looked up by name: adding one invalidates the others' references.
        self.bm.faces.layers.int.new("painted")
        self.bm.faces.layers.int.new("part")
        self.prepared = False

    @property
    def tag(self):
        return self.bm.faces.layers.int["painted"]

    @property
    def part_tag(self):
        return self.bm.faces.layers.int["part"]

    def face(self, points, material):
        verts = [self.bm.verts.new(blender_point(p)) for p in points]
        f = self.bm.faces.new(verts)
        f.material_index = self.material(material)
        f[self.tag] = CURRENT[0]
        f[self.part_tag] = CURRENT_PART[0]
        return f

    def material(self, name):
        if name not in self.material_index:
            self.material_index[name] = len(self.material_index)
        return self.material_index[name]

    def loft(self, rings, material, closed=True, cap_start=False, cap_end=False):
        """Quads between successive rings of equal size (body axes)."""
        rows = [[self.bm.verts.new(blender_point(p)) for p in ring] for ring in rings]
        index = self.material(material)
        n = len(rings[0])
        for a, b in zip(rows, rows[1:]):
            for i in range(n if closed else n - 1):
                j = (i + 1) % n
                f = self.bm.faces.new((a[i], a[j], b[j], b[i]))
                f.material_index = index
                f[self.tag] = CURRENT[0]
                f[self.part_tag] = CURRENT_PART[0]
        for cap, row in ((cap_start, rows[0]), (cap_end, rows[-1])):
            if cap:
                f = self.bm.faces.new(row)
                f.material_index = index
                f[self.tag] = CURRENT[0]
                f[self.part_tag] = CURRENT_PART[0]

    def delete_faces(self, inside):
        """Removes the faces whose centre (body axes) `inside` accepts: openings."""
        doomed = []
        for f in self.bm.faces:
            c = f.calc_center_median()
            if inside(Vector((-c.y, -c.x, -c.z))):
                doomed.append(f)
        bmesh.ops.delete(self.bm, geom=doomed, context="FACES")

    def prepare(self):
        """Welds the faces and points their normals outwards, once."""
        if not self.prepared:
            bmesh.ops.remove_doubles(self.bm, verts=self.bm.verts, dist=1e-6)
            bmesh.ops.recalc_face_normals(self.bm, faces=self.bm.faces)
            self.prepared = True

    def painted_faces(self):
        """The painted faces and their region of the atlas, (surface, side)."""
        self.prepare()
        for f in self.bm.faces:
            if f[self.tag]:
                surface = PAINTED[f[self.tag] - 1]
                yield f, (surface, face_side(surface, body_vector(f.normal)))

    def write(self, obj, materials, atlas=None, smooth=math.radians(35)):
        self.prepare()
        self.map_atlas(atlas)
        self.to_object(self.bm, obj, materials, smooth)
        self.bm.free()

    def map_atlas(self, atlas):
        """The painted faces' coordinates in the fabric's atlas."""
        if atlas is not None and any(f[self.tag] for f in self.bm.faces):
            uv = self.bm.loops.layers.uv.verify()
            for f, region in self.painted_faces():
                for loop in f.loops:
                    loop[uv].uv = atlas.uv(region, body_vector(loop.vert.co))

    def to_object(self, bm, obj, materials, smooth):
        for name in ("painted", "part"):
            bm.faces.layers.int.remove(bm.faces.layers.int[name])
        bm.transform(obj.matrix_world.inverted())
        bm.to_mesh(obj.data)
        for name, _ in sorted(self.material_index.items(), key=lambda kv: kv[1]):
            obj.data.materials.append(materials[name])
        for poly in obj.data.polygons:
            poly.use_smooth = True
        smooth_by_angle(obj.data, smooth)

    def write_parts(self, parent, materials, atlas=None, smooth=math.radians(35)):
        """One object per damageable part (resolve_part), under `parent`, its origin at
        the centre of its bounds; the parts share their seams' vertices, so that nothing
        shows between them. Returns the objects by name."""
        self.prepare()
        self.map_atlas(atlas)
        names = [resolve_part(PART_NAMES[f[self.part_tag]], body_vector(f.calc_center_median()))
                 for f in self.bm.faces]
        objects = {}
        for name in sorted(set(names)):
            bm = self.bm.copy()
            bm.faces.ensure_lookup_table()
            doomed = [f for f, n in zip(bm.faces, names) if n != name]
            bmesh.ops.delete(bm, geom=doomed, context="FACES")
            corners = [v.co for v in bm.verts]
            centre = sum(corners, Vector()) / len(corners)
            obj = new_object(name, parent, Matrix.Translation(centre))
            self.to_object(bm, obj, materials, smooth)
            bm.free()
            objects[name] = obj
        self.bm.free()
        return objects


def smooth_by_angle(mesh, angle):
    """Sharp edges above `angle`, smooth elsewhere, across Blender versions."""
    if hasattr(mesh, "set_sharp_from_angle"):          # Blender 4.1 and later
        mesh.set_sharp_from_angle(angle=angle)
    else:                                              # Blender 4.0
        mesh.use_auto_smooth = True
        mesh.auto_smooth_angle = angle


# --- Sections ----------------------------------------------------------------------------

def interp(xs, ys, x):
    for i in range(len(xs) - 1):
        if xs[i] <= x <= xs[i + 1]:
            t = (x - xs[i]) / (xs[i + 1] - xs[i])
            return ys[i] + t * (ys[i + 1] - ys[i])
    return ys[-1]


def spad_section(u):
    """Upper and lower ordinates (fraction of chord) at chord fraction u from the leading edge."""
    x = 100.0 * u
    return interp(SPAD_X, SPAD_UPPER, x) / 100.0, interp(SPAD_X, SPAD_LOWER, x) / 100.0


def symmetric_section(u):
    """A thin symmetric section with a round nose (NACA 00xx thickness law)."""
    t = TAIL_THICKNESS
    y = 5 * t * (0.2969 * math.sqrt(u) - 0.1260 * u - 0.3516 * u ** 2 + 0.2843 * u ** 3
                 - 0.1036 * u ** 4)
    return y, -y


def chord_samples(u0, u1, n=CHORD_POINTS):
    """Cosine-spaced chord fractions from u0 to u1, denser at the leading edge."""
    out = []
    for i in range(n + 1):
        t = i / n
        u = 0.5 * (1 - math.cos(math.pi * t))
        out.append(u0 + (u1 - u0) * u)
    return out


def ring(section, origin, chord_dir, normal, chord, u0, u1):
    """Closed loop of a section between chord fractions u0 and u1 (body axes): upper
    surface from u1 back to u0, then lower surface from u0 to u1."""
    us = chord_samples(u0, u1)
    upper, lower = [], []
    for u in us:
        hu, hl = section(u)
        base = origin + chord_dir * ((0.25 - u) * chord)
        upper.append(base + normal * (hu * chord))
        lower.append(base + normal * (hl * chord))
    # At the leading edge both surfaces meet: keep the point once.
    return list(reversed(upper)) + (lower[1:] if u0 == 0.0 else lower)


# --- Lifting surfaces --------------------------------------------------------------------

class Station:
    """A spanwise station of a panel: quarter-chord point, chord, axes (body)."""

    def __init__(self, panel, eta, side, kind, tip_inset=(0.0, 0.0)):
        root, tip = Vector(panel["root"]), Vector(panel["tip"])
        if side < 0:
            root.y, tip.y = -root.y, -tip.y
        self.qc = root.lerp(tip, eta)
        chord = panel["chord_root"] + eta * (panel["chord_tip"] - panel["chord_root"])
        incidence = panel["incidence"] + eta * panel["twist"]
        if kind == "horizontal":
            self.chord_dir = Vector((math.cos(incidence), 0.0, -math.sin(incidence)))
            self.normal = Vector((-math.sin(incidence), 0.0, -math.cos(incidence)))
        else:
            self.chord_dir = Vector((1.0, 0.0, 0.0))
            self.normal = Vector((0.0, 1.0, 0.0))
        # Rounded corners near a free tip: the leading and trailing edges come in.
        le_in, te_in = tip_inset
        self.full_chord = chord
        self.rounded = le_in > 0.0
        le = self.qc + self.chord_dir * (0.25 * chord - le_in)
        self.chord = max(chord - le_in - te_in, 0.02 * chord)
        self.qc = le - self.chord_dir * (0.25 * self.chord)

    def point(self, u):
        return self.qc + self.chord_dir * ((0.25 - u) * self.chord)


def tip_insets(distance_to_tip):
    """Leading and trailing edge insets of a rounded tip corner (E: TIP_RADIUS)."""
    if distance_to_tip >= TIP_RADIUS:
        return (0.0, 0.0)
    d = TIP_RADIUS - distance_to_tip
    inset = TIP_RADIUS - math.sqrt(max(TIP_RADIUS ** 2 - d ** 2, 0.0))
    return (inset, inset)


CUTOUT = None   # (half-width, chord fraction reached from the leading edge), sorted (main)
RIBS = (0.19, 0.0)   # rib pitch and scallop depth, m (feature "ribs", main)


def cutout_inset(cutout, y):
    """Chord fraction cut from the trailing edge at a distance y from the axis, by the
    cut-out of the upper plane over the cockpit."""
    if y >= cutout[-1][0]:
        return 0.0
    for (h0, f0), (h1, f1) in zip(cutout, cutout[1:]):
        if h0 <= y <= h1:
            f = f0 + (f1 - f0) * (y - h0) / (h1 - h0)
            return max(1.0 - f, 0.0)
    return max(1.0 - cutout[0][1], 0.0)


def hinge_fraction(station, hinge):
    """Chord fraction of a station where the node's hinge line crosses its chord (the
    point of the chord nearest the line, as tools/mockup cuts its plates)."""
    le = station.point(0.0)
    aft = -station.chord_dir
    axis = Vector(hinge["axis"]).normalized()
    w = le - Vector(hinge["pivot"])
    b = aft.dot(axis)
    den = 1.0 - b * b
    t = (b * axis.dot(w) - aft.dot(w)) / den if den > 1e-9 else 0.75 * station.chord
    # Strictly inside the chord, so that both parts keep the same number of points from
    # one station to the next (a part that vanishes becomes a sliver): at the top of the
    # rudder the whole chord is aft of the post, at the apex of the elevator's notch none.
    return min(max(t / station.chord, 1e-4), 1.0 - 1e-3)


def loft_runs(stations, hinge_of, section, body_parts, controls):
    """Lofts the segments between successive stations, each run of segments under the
    same control (or none) in one piece: a fixed part and a moving part closed only at
    the run's ends, so that no face is left inside the surface."""
    i, n = 0, len(stations) - 1
    while i < n:
        hinge = hinge_of(i)
        j = i
        while j + 1 < n and hinge_of(j + 1) is hinge:
            j += 1
        run = stations[i:j + 2]
        if hinge is None:
            body_parts.loft([ring(section, s.qc, s.chord_dir, s.normal, s.chord, 0.0, 1.0) for s in run],
                            "surface")
        else:
            us = [hinge_fraction(s, hinge) for s in run]
            body_parts.loft([ring(section, s.qc, s.chord_dir, s.normal, s.chord, 0.0, u)
                             for s, u in zip(run, us)], "surface", cap_start=True, cap_end=True)
            controls[hinge["node"]].loft([ring(section, s.qc, s.chord_dir, s.normal, s.chord, u, 1.0)
                                          for s, u in zip(run, us)], "control", cap_start=True, cap_end=True)
        i = j + 1


# --- Structure under the fabric (M4 plan, step 2) -----------------------------------------

FRAME_DEPTH = 0.85      # E: ribs and spars inside the skin, share of the section's depth
RIB_WEB = 0.008         # E: a rib's web, m
SPAR_WIDTH = 0.03       # E: a box spar's width, m
TAIL_POST = 0.04        # E: the tail surfaces' rear spar (the fin's post), just ahead of the hinge of their control, chord fraction


def inner_section(section, depth=FRAME_DEPTH):
    """A section shrunk about its mean line: the frame inside the fabric."""
    def inner(u):
        hu, hl = section(u)
        mean, half = 0.5 * (hu + hl), 0.5 * (hu - hl) * depth
        return mean + half, mean - half
    return inner


class Lerped:
    """A station between two others, same interface as Station."""

    def __init__(self, a, b, t):
        self.qc = a.qc.lerp(b.qc, t)
        self.chord = a.chord + t * (b.chord - a.chord)
        self.full_chord = getattr(a, "full_chord", a.chord) + t * (
            getattr(b, "full_chord", b.chord) - getattr(a, "full_chord", a.chord))
        self.rounded = getattr(a, "rounded", False) or getattr(b, "rounded", False)
        self.chord_dir = a.chord_dir.lerp(b.chord_dir, t).normalized()
        self.normal = a.normal.lerp(b.normal, t).normalized()

    def point(self, u):
        return self.qc + self.chord_dir * ((0.25 - u) * self.chord)


def station_at(stations, d):
    """The station at a spanwise distance d, between two of `stations` [(d, station)]."""
    for (d0, a), (d1, b) in zip(stations, stations[1:]):
        if d0 - 1e-9 <= d <= d1 + 1e-9:
            return Lerped(a, b, 0.0 if d1 - d0 < 1e-9 else (d - d0) / (d1 - d0))
    return Lerped(stations[-1][1], stations[-1][1], 0.0)


def wing_spar(u):
    """A wing's spar at a fraction u of the nominal chord: straight along the span."""
    return lambda st, hinge: u * getattr(st, "full_chord", st.chord) / st.chord


def post_spar(offset):
    """A post just ahead of a control's hinge (the fin's), where there is one."""
    return lambda st, hinge: None if hinge is None else hinge_fraction(st, hinge) - offset


def build_frame(stations, hinge_at, section, pitch, spars, body_parts, controls):
    """The structure inside a fabric surface, along its stations [(spanwise distance,
    station)]: a rib every `pitch`, cut at a control's hinge (the aft part moves with the
    control), and box spars in the fixed part, each giving its chord fraction at a station
    (`wing_spar`, `post_spar`), interrupted where it would leave the
    fabric or meet a rounded tip. Seen through holes in the fabric, and when it burns
    (damages 2, 3, 4, 35 of docs/gdd/combat-degats.md)."""
    inner = inner_section(section)
    span = (stations[-1][1].qc - stations[0][1].qc).normalized()
    half = span * (0.5 * RIB_WEB)
    total = stations[-1][0]

    def rib(builder, st, u0, u1):
        r = ring(inner, st.qc, st.chord_dir, st.normal, st.chord, u0, u1)
        builder.loft([[p - half for p in r], [p + half for p in r]], "wood", cap_start=True, cap_end=True)

    k = 0
    while k * pitch <= total + 1e-6:
        d = min(k * pitch, total)
        st = station_at(stations, d)
        hinge = hinge_at(d)
        if hinge is None:
            rib(body_parts, st, 0.01, 0.985)
        else:
            uh = hinge_fraction(st, hinge)
            if uh > 0.05:
                rib(body_parts, st, 0.01, uh - 0.015)
            if uh < 0.95:
                rib(controls[hinge["node"]], st, uh + 0.015, 0.985)
        k += 1

    for spar in spars:
        run = []

        def close(run):
            if len(run) >= 2:
                body_parts.loft(run, "wood", cap_start=True, cap_end=True)

        for d, st in stations:
            hinge = hinge_at(d)
            u_local = spar(st, hinge)
            limit = hinge_fraction(st, hinge) - 0.02 if hinge is not None else 0.95
            if (getattr(st, "rounded", False) or u_local is None
                    or not 0.05 <= u_local <= limit):
                close(run)
                run = []
                continue
            hu, hl = inner(u_local)
            centre = st.point(u_local) + st.normal * (0.5 * (hu + hl) * st.chord)
            h, w = 0.5 * (hu - hl) * st.chord, 0.5 * SPAR_WIDTH
            run.append([centre + st.chord_dir * a + st.normal * b
                        for a, b in ((w, h), (-w, h), (-w, -h), (w, -h))])
        close(run)


def build_surface(surface, data, body_parts, controls):
    """Lofts every panel of a surface, both sides if mirrored; the parts aft of a
    control's hinge go to that control's node."""
    kind = surface["kind"]
    section = spad_section if kind == "horizontal" and surface["name"].endswith("wing") else symmetric_section
    sides = (1.0, -1.0) if surface["mirrored"] else (0.0,)
    panels = surface["panels"]
    for side in sides:
        for p_index, panel in enumerate(panels):
            root, tip = Vector(panel["root"]), Vector(panel["tip"])
            length = (tip - root).length
            free_tip = p_index == len(panels) - 1
            # Control regions on this panel and side.
            regions = []
            for hinge in data["hinges"]:
                for r in hinge["regions"]:
                    if r["surface"] == surface["name"] and r["panel"] == p_index and r["side"] == side:
                        regions.append((r["span"][0], r["span"][1], hinge))
            # Spanwise stations: region bounds, the tip rounding, and enough in between.
            etas = {0.0, 1.0}
            for a, b, _ in regions:
                etas.update((a, b))
            n = max(4, int(length / 0.12))
            etas.update(i / n for i in range(n + 1))
            if free_tip:
                for k in range(1, 7):
                    d = TIP_RADIUS * (1 - k / 7)
                    etas.add(1.0 - d / length)
            wing = kind == "horizontal" and surface["name"].endswith("wing")
            if wing:
                pitch, _ = RIBS
                etas.update(min(k * pitch / 4 / length, 1.0) for k in range(int(4 * length / pitch) + 1))
            cutout = CUTOUT if surface["name"] == "upper_wing" and p_index == 0 else None
            if cutout:
                etas.update(min(h / length, 1.0) for h, _ in cutout)
                etas.update(i * cutout[-1][0] / (12 * length) for i in range(13))
            etas = sorted(e for e in etas if 0.0 <= e <= 1.0)
            stations = []
            for e in etas:
                le_in, te_in = tip_insets((1.0 - e) * length) if free_tip else (0.0, 0.0)
                if cutout:
                    chord = panel["chord_root"] + e * (panel["chord_tip"] - panel["chord_root"])
                    te_in += cutout_inset(cutout, e * length) * chord
                if wing:
                    # The fabric scallops between the ribs at the trailing edge wire.
                    pitch, depth = RIBS
                    te_in += depth * math.sin(math.pi * (e * length) / pitch) ** 2
                stations.append((e, Station(panel, e, side if side != 0.0 else 1.0, kind, (le_in, te_in))))
            # Loft along the span: whole sections, or fixed part and control part.
            def hinge_of(k, stations=stations, regions=regions):
                mid = 0.5 * (stations[k][0] + stations[k + 1][0])
                region = next((r for r in regions if r[0] <= mid <= r[1]), None)
                return region[2] if region else None
            loft_runs([st for _, st in stations], hinge_of, section, body_parts, controls)
            # The structure under the fabric.
            def hinge_at(d, length=length, regions=regions):
                eta = d / length
                region = next((r for r in regions if r[0] <= eta <= r[1]), None)
                return region[2] if region else None
            if wing:
                spars = [wing_spar(u) for u in feature(data, "strut_spars")[0]]
            else:
                spars = [post_spar(TAIL_POST)]
            pitch = RIBS[0] if wing else feature(data, "tail_ribs")[0][0]
            build_frame([(e * length, st) for e, st in stations], hinge_at, section, pitch,
                        spars, body_parts, controls)
            # Close the panel's ends, unless a control already closes them.
            for e, s in (stations[0], stations[-1]):
                if not any(a <= e <= b for a, b, _ in regions):
                    body_parts.face(ring(section, s.qc, s.chord_dir, s.normal, s.chord, 0.0, 1.0), "surface")


class OutlineStation:
    """A station of a surface built from its drawn outline (maquette.ron `outlines`):
    leading and trailing edge at a spanwise coordinate `s` (y on a horizontal surface, z on
    a vertical one); same interface as Station."""

    def __init__(self, kind, s, x_le, x_te, side, plane_z, incidence):
        self.chord = max(x_le - x_te, 1e-3)
        if kind == "horizontal":
            self.chord_dir = Vector((math.cos(incidence), 0.0, -math.sin(incidence)))
            self.normal = Vector((-math.sin(incidence), 0.0, -math.cos(incidence)))
            le = Vector((x_le, side * s, plane_z))
        else:
            self.chord_dir = Vector((1.0, 0.0, 0.0))
            self.normal = Vector((0.0, 1.0, 0.0))
            le = Vector((x_le, 0.0, s))
        self.qc = le - self.chord_dir * (0.25 * self.chord)

    def point(self, u):
        return self.qc + self.chord_dir * ((0.25 - u) * self.chord)


def region_at(surface, data, side, coord):
    """The hinge whose regions cover a spanwise coordinate of a surface, the regions being
    named by the panels of aircraft.ron (contract)."""
    axis = 1 if surface["kind"] == "horizontal" else 2
    for p_index, panel in enumerate(surface["panels"]):
        a, b = panel["root"][axis], panel["tip"][axis]
        if side < 0:
            a, b = -a, -b
        if abs(b - a) < 1e-9 or not min(a, b) - 1e-6 <= coord <= max(a, b) + 1e-6:
            continue
        eta = (coord - a) / (b - a)
        for hinge in data["hinges"]:
            for r in hinge["regions"]:
                if (r["surface"] == surface["name"] and r["panel"] == p_index and r["side"] == side
                        and r["span"][0] - 1e-6 <= eta <= r["span"][1] + 1e-6):
                    return hinge
    return None


def build_outline_surface(surface, rows, data, body_parts, controls):
    """Lofts a surface along its drawn outline instead of the flight model's equivalent
    panels; the parts aft of a control's hinge go to that control's node, as in
    build_surface. A mirrored surface's halves meet at the root without a cap."""
    kind = surface["kind"]
    first = surface["panels"][0]
    plane_z = first["root"][2]
    sides = (1.0, -1.0) if surface["mirrored"] else (0.0,)
    rows = sorted(rows, key=lambda r: abs(r[0]))
    rows = list(zip([r[0] for r in rows], smoothed([r[1] for r in rows]), smoothed([r[2] for r in rows])))
    for side in sides:
        sign = side if side != 0.0 else 1.0
        stations = [(r[0] * (sign if kind == "horizontal" else 1.0),
                     OutlineStation(kind, r[0], r[1], r[2], sign, plane_z, first["incidence"]))
                    for r in rows]
        hinges = [region_at(surface, data, side, 0.5 * (c0 + c1))
                  for (c0, _), (c1, _) in zip(stations, stations[1:])]
        loft_runs([st for _, st in stations], lambda k: hinges[k], symmetric_section,
                  body_parts, controls)
        # The structure under the fabric.
        c0 = stations[0][0]
        sdir = 1.0 if stations[-1][0] >= c0 else -1.0
        spars = [post_spar(TAIL_POST)]
        build_frame([(abs(c - c0), st) for c, st in stations],
                    lambda d: region_at(surface, data, side, c0 + sdir * d), symmetric_section,
                    feature(data, "tail_ribs")[0][0], spars, body_parts, controls)
        ends = [stations[-1]] if surface["mirrored"] else [stations[0], stations[-1]]
        for c, s in ends:
            if region_at(surface, data, side, c) is None:
                body_parts.face(ring(symmetric_section, s.qc, s.chord_dir, s.normal, s.chord, 0.0, 1.0), "surface")


# --- Fuselage ----------------------------------------------------------------------------

def catmull_rom(points, t):
    """Point at parameter t (0..len-1) of a Catmull-Rom spline through `points` (floats)."""
    i = min(int(t), len(points) - 2)
    f = t - i
    p0 = points[max(i - 1, 0)]
    p1, p2 = points[i], points[i + 1]
    p3 = points[min(i + 2, len(points) - 1)]
    return 0.5 * ((2 * p1) + (-p0 + p2) * f + (2 * p0 - 5 * p1 + 4 * p2 - p3) * f * f
                  + (-p0 + 3 * p1 - 3 * p2 + p3) * f ** 3)


def superellipse(cx_z, half_w, half_h, exponent, n=32):
    """Section of the fuselage: points (y, z) around its centre, rounded on all four faces
    (« bottom curved below the lower longerons as well as the sides and top », S48)."""
    pts = []
    for i in range(n):
        a = 2 * math.pi * i / n
        c, s = math.cos(a), math.sin(a)
        y = half_w * math.copysign(abs(c) ** (2 / exponent), c)
        z = cx_z + half_h * math.copysign(abs(s) ** (2 / exponent), s)
        pts.append((y, z))
    return pts


def opening_half_width(opening, x):
    """Half-width of the cockpit opening at x (0 outside), from its drawn outline."""
    pts = sorted(opening)
    if not pts[0][0] < x < pts[-1][0]:
        return 0.0
    for (x0, h0), (x1, h1) in zip(pts, pts[1:]):
        if x0 <= x <= x1:
            return h0 + (h1 - h0) * (x - x0) / (x1 - x0)
    return 0.0


def smoothed(values, passes=2):
    """Binomial smoothing [1, 2, 1] / 4, `passes` times, ends kept: the drawing's line
    readings carry 1 to 2 cm of noise that would ripple a loft through them."""
    v = list(values)
    for _ in range(passes):
        v = [v[0]] + [0.25 * (v[i - 1] + 2 * v[i] + v[i + 1]) for i in range(1, len(v) - 1)] + [v[-1]]
    return v


def build_fuselage(data, parts):
    """Lofts the fuselage through the drawn sections (maquette.ron), round at the cowl and
    a rounded box aft (« curved on the four faces », S48), and cuts the cockpit opening."""
    sections = data["mockup"]["fuselage"]
    keys = list(zip([s["x"] for s in sections],
                    smoothed([s["width"] / 2 for s in sections]),
                    smoothed([s["top"] for s in sections]),
                    smoothed([s["bottom"] for s in sections])))
    n = 4 * (len(keys) - 1)
    rings = []
    for i in range(n + 1):
        t = (len(keys) - 1) * i / n
        x = catmull_rom([k[0] for k in keys], t)
        hw = max(catmull_rom([k[1] for k in keys], t), 0.01)
        top = catmull_rom([k[2] for k in keys], t)
        bottom = catmull_rom([k[3] for k in keys], t)
        centre, half_h = 0.5 * (top + bottom), max(0.5 * (bottom - top), 0.01)
        # E: round cowling (x > 0), a rounded box aft of the cockpit
        exponent = 2.0 if x > 0.0 else 2.0 + 1.6 * min(-x / 1.2, 1.0)
        rings.append([Vector((x, y, z)) for y, z in superellipse(centre, hw, half_h, exponent, n=48)])
    # Aluminium panels of the cowl from the nose back to the cockpit (S48: « the fore end
    # of the machine, which houses the motor, is covered with aluminium »), fabric aft.
    split = next(k for k, r in enumerate(rings) if r[0].x <= COWL_END_X)
    parts.loft(rings[:split + 1], "cowl", cap_start=True)
    with painted("fuselage"):
        parts.loft(rings[split:], "fuselage", cap_end=True)
    opening = feature(data, "cockpit_opening")
    parts.delete_faces(lambda c: c.z < -0.80 and abs(c.y) < opening_half_width(opening, c.x))
    return keys


FUSELAGE_BAY = 0.45       # E: spacing of the fuselage's frames (uprights and cross pieces), m
LONGERON = 0.03           # E: ash longerons, m
FRAME_STRUT = 0.022       # E: uprights and cross pieces, m
BRACING_WIRE = 0.003      # E: the bays' diagonal wires, m


def build_fuselage_frame(keys, parts):
    """The wire-braced wooden box girder under the fuselage's fabric, from the cockpit's
    front to the sternpost: four longerons inside the section's corners, a frame of
    uprights and cross pieces every FUSELAGE_BAY, a diagonal wire in each side bay. Seen
    through holes and when the fabric burns (M4 plan, step 2)."""
    x_front, x_back = COWL_END_X, keys[-1][0] + 0.05

    def corners(x):
        top, bottom, hw = fuselage_at(keys, x)
        exponent = 2.0 if x > 0.0 else 2.0 + 1.6 * min(-x / 1.2, 1.0)
        k = 0.9 * 0.5 ** (1.0 / exponent)
        zc, hh = 0.5 * (top + bottom), 0.5 * (bottom - top)
        return [Vector((x, sy * k * hw, zc + sz * k * hh)) for sy, sz in ((1, -1), (-1, -1), (-1, 1), (1, 1))]

    n = max(2, int((x_front - x_back) / 0.1))
    xs = [x_front - (x_front - x_back) * i / n for i in range(n + 1)]
    for c in range(4):
        tube(parts, [corners(x)[c] for x in xs], LONGERON, "wood", n=8)
    bays = max(1, int((x_front - x_back) / FUSELAGE_BAY))
    frames = [x_front - (x_front - x_back) * i / bays for i in range(bays + 1)]
    for x in frames:
        c = corners(x)
        for a, b in ((0, 1), (1, 2), (2, 3), (3, 0)):
            round_bar(parts, c[a], c[b], FRAME_STRUT, "wood", n=6)
    for xa, xb in zip(frames, frames[1:]):
        a, b = corners(xa), corners(xb)
        for i, j in ((0, 3), (1, 2)):   # each side: from the top at one frame to the bottom at the next
            round_bar(parts, a[i], b[j], BRACING_WIRE, "metal", n=4)


def build_cockpit(data, parts, keys):
    """The padded rim around the cockpit opening, and a dark tub below it."""
    opening = sorted(feature(data, "cockpit_opening"))
    xs = [opening[0][0] + (opening[-1][0] - opening[0][0]) * i / 40 for i in range(41)]
    def top(x):
        t, _, _ = fuselage_at(keys, x)
        return t
    loop = [Vector((x, opening_half_width(opening, x), top(x))) for x in xs]
    loop += [Vector((x, -opening_half_width(opening, x), top(x))) for x in reversed(xs[1:-1])]
    tube(parts, loop + [loop[0]], RIM_DIAMETER, "leather")
    floor = [Vector((p.x, 0.85 * p.y, COCKPIT_FLOOR)) for p in loop]
    parts.loft([loop, floor], "cockpit", cap_end=True)


# --- Struts, gear, propeller, gun --------------------------------------------------------

def streamlined_strut(parts, a, b, chord, thickness, material, n=16):
    """A strut of elliptic section from a to b (body axes), its chord along x."""
    a, b = Vector(a), Vector(b)
    axis = (b - a).normalized()
    cx = Vector((1, 0, 0))
    cx = (cx - axis * cx.dot(axis)).normalized()
    cy = axis.cross(cx)
    rings = []
    for end in (a, b):
        rings.append([end + cx * (0.5 * chord * math.cos(2 * math.pi * i / n))
                      + cy * (0.5 * thickness * math.sin(2 * math.pi * i / n)) for i in range(n)])
    parts.loft(rings, material, cap_start=True, cap_end=True)


def round_bar(parts, a, b, diameter, material, n=12):
    a, b = Vector(a), Vector(b)
    axis = (b - a).normalized()
    ref = Vector((0, 0, 1)) if abs(axis.z) < 0.9 else Vector((1, 0, 0))
    u = axis.cross(ref).normalized()
    v = axis.cross(u)
    r = 0.5 * diameter
    rings = [[end + (u * math.cos(2 * math.pi * i / n) + v * math.sin(2 * math.pi * i / n)) * r
              for i in range(n)] for end in (a, b)]
    parts.loft(rings, material, cap_start=True, cap_end=True)


def wing_surface_z(data, name, x, y, upper):
    """z of the upper or lower surface of a wing at (x, y), at its first panel."""
    surface = next(s for s in data["surfaces"] if s["name"] == name)
    panel = surface["panels"][0]
    s = Station(panel, 0.0, 1.0, "horizontal")
    u = min(max((s.point(0.0).x - x) / s.chord, 0.0), 1.0)
    hu, hl = spad_section(u)
    return s.qc.z - (hu if upper else hl) * s.chord


def build_struts(data, parts):
    mock = data["mockup"]
    upper = next(s for s in data["surfaces"] if s["name"] == "upper_wing")
    chord = upper["panels"][0]["chord_root"]
    le_x = upper["panels"][0]["root"][0] + 0.25 * chord
    for _, y in mock["struts"]:
        for side in (1, -1):
            for f in feature(data, "strut_spars")[0]:
                x = le_x - f * chord
                bottom = wing_surface_z(data, "lower_wing", x, y, upper=True)
                top = wing_surface_z(data, "upper_wing", x, y, upper=False)
                streamlined_strut(parts, (x, side * y, bottom), (x, side * y, top),
                                  STRUT_CHORD, STRUT_THICK, "wood")
    # Steel tube between the intermediate struts, at mid-gap (S48).
    x0, x1, z = feature(data, "steel_tube")[0]
    y = min(y for _, y in mock["struts"])
    with part("strut_tube_{lr}"):
        for side in (1, -1):
            round_bar(parts, (x0, side * y, z), (x1, side * y, z), 0.03, "metal")


def fuselage_at(keys, x):
    """Top and bottom of the fuselage at x, from its key sections."""
    xs = [k[0] for k in keys]
    if x >= xs[0]:
        return keys[0][2], keys[0][3], keys[0][1]
    for k0, k1 in zip(keys, keys[1:]):
        if k1[0] <= x <= k0[0]:
            t = (k0[0] - x) / (k0[0] - k1[0])
            return (k0[2] + t * (k1[2] - k0[2]), k0[3] + t * (k1[3] - k0[3]),
                    k0[1] + t * (k1[1] - k0[1]))
    return keys[-1][2], keys[-1][3], keys[-1][1]


def build_cabane(data, parts, keys):
    """Vertical cabane struts from the cowl shoulders to the upper plane, at the spars."""
    upper = next(s for s in data["surfaces"] if s["name"] == "upper_wing")
    chord = upper["panels"][0]["chord_root"]
    le_x = upper["panels"][0]["root"][0] + 0.25 * chord
    y, z_foot = feature(data, "cabane")[0]
    for f in feature(data, "strut_spars")[0]:
        x = le_x - f * chord
        z_wing = wing_surface_z(data, "upper_wing", x, y, upper=False)
        for side in (1, -1):
            streamlined_strut(parts, (x, side * y, z_foot + 0.05), (x, side * y, z_wing),
                              CABANE_CHORD, CABANE_THICK, "metal")


def build_gear(data, parts, keys):
    mock = data["mockup"]
    cx, cy, cz = mock["wheels"]["centre"]
    radius, width = 0.5 * mock["wheels"]["diameter"], mock["wheels"]["width"]
    for side in (1, -1):
        with part("wheel_{lr}"):
            build_wheel(parts, Vector((cx, side * cy, cz)), radius, width, side)
        # Legs (drawing): a streamlined front leg from the lower longeron, a rear leg from
        # the belly, both to the axle.
        with part("gear_{lr}"):
            fx, fy, fz, ax, ay, az, chord = feature(data, "gear_front_leg")[0]
            streamlined_strut(parts, (fx, side * fy, fz), (ax, side * ay, az), chord, LEG_THICK, "wood")
            rx, ry, rz, bx, by, bz = feature(data, "gear_rear_leg")[0]
            streamlined_strut(parts, (rx, side * ry, rz), (bx, side * by, bz), REAR_LEG_CHORD, LEG_THICK, "wood")
    # Axle and its fairing between the wheels.
    with part("axle"):
        streamlined_strut(parts, (cx, cy - width, cz), (cx, -(cy - width), cz), 0.12, 0.05, "fabric_doped")
    # Tail skid, an ash leaf from its foot on the fuselage to its shoe.
    with part("skid"):
        sx, sy, sz = mock["skid"]
        fx, fz = feature(data, "skid_foot")[0]
        streamlined_strut(parts, (fx, 0.0, fz), (sx, sy, sz), *SKID_SECTION, "wood")


def build_wheel(parts, centre, radius, width, side):
    """Tyre: a torus; wheel cover: two shallow cones."""
    n, m = 32, 10
    tube = 0.5 * width
    rings = []
    for i in range(n + 1):
        a = 2 * math.pi * i / n
        radial = Vector((math.cos(a), 0, math.sin(a)))
        rings.append([centre + radial * (radius - tube + tube * math.cos(2 * math.pi * j / m))
                      + Vector((0, side * tube * math.sin(2 * math.pi * j / m), 0)) for j in range(m)])
    parts.loft(rings[:-1] + [rings[0]], "rubber")
    for face in (1, -1):
        hub = centre + Vector((0, side * face * 0.6 * width, 0))
        rim = [centre + Vector((math.cos(2 * math.pi * i / n) * (radius - 1.6 * tube), side * face * 0.35 * width,
                                math.sin(2 * math.pi * i / n) * (radius - 1.6 * tube))) for i in range(n)]
        for i in range(n):
            parts.face([hub, rim[i], rim[(i + 1) % n]], "fabric_doped")


def tube(parts, points, diameter, material, n=12, scales=None):
    """A round tube along a polyline (body axes), its rings square to the path; `scales`
    shapes its radius point by point (rounded ends)."""
    pts = [Vector(p) for p in points]
    rings = []
    for i, p in enumerate(pts):
        r = 0.5 * diameter * (scales[i] if scales else 1.0)
        a, b = pts[max(i - 1, 0)], pts[min(i + 1, len(pts) - 1)]
        t = (b - a).normalized()
        ref = Vector((0, 0, 1)) if abs(t.z) < 0.9 else Vector((1, 0, 0))
        u = t.cross(ref).normalized()
        v = t.cross(u)
        rings.append([p + (u * math.cos(2 * math.pi * k / n) + v * math.sin(2 * math.pi * k / n)) * r
                      for k in range(n)])
    parts.loft(rings, material, cap_start=True, cap_end=True)


def build_nose(data, parts, keys):
    """Radiator in its front ring, and the camshaft covers on the cowl shoulders."""
    x_face, zc, diameter = feature(data, "radiator")[0]
    r = 0.5 * diameter
    n = 48
    centre = Vector((x_face + 0.004, 0.0, zc))
    rim = [centre + Vector((0, r * math.cos(2 * math.pi * i / n), r * math.sin(2 * math.pi * i / n)))
           for i in range(n)]
    for i in range(n):
        parts.face([centre, rim[(i + 1) % n], rim[i]], "radiator")
    # Rounded lip of the front ring around the radiator.
    lip = [[centre + Vector((-0.025 * (1 - math.cos(math.pi * k / 6)), 0, 0))
            + Vector((0, (r + 0.025 * math.sin(math.pi * k / 6)) * math.cos(2 * math.pi * i / n),
                      (r + 0.025 * math.sin(math.pi * k / 6)) * math.sin(2 * math.pi * i / n)))
            for i in range(n)] for k in range(7)]
    parts.loft(lip, "cowl")
    y, z, d, x0, x1 = feature(data, "camshaft_bulges")[0]
    k_max = 16
    # Rounded ends that close on the cowl: radius (1 - (2t - 1)^8)^(1/2) along the cover.
    scales = [max(1 - (2 * k / k_max - 1) ** 8, 0.0) ** 0.5 for k in range(k_max + 1)]
    for side in (1, -1):
        cover = [(x0 + (x1 - x0) * k / k_max, side * y, z) for k in range(k_max + 1)]
        tube(parts, cover, d, "cowl", n=16, scales=scales)


def build_exhausts(data, parts, keys):
    """Four stubs to a collector each side, its front crook, and the pipe along the flank."""
    y_c, x_from, x_crook, x_front, z_crook = feature(data, "exhaust_collector")[0]
    z_pipe, d_pipe, x_pipe_front, x_end = feature(data, "exhaust_pipe")[0]
    stubs = feature(data, "exhaust_stubs")
    z_c = sum(z for _, z in stubs) / len(stubs)
    for side in (1, -1):
        for x, z in stubs:
            tube(parts, [(x, side * (y_c - 0.06), z + 0.06), (x, side * y_c, z)], EXHAUST_STUB, "exhaust")
        collector = [(x_from, side * y_c, z_c), (x_crook, side * y_c, z_c),
                     (0.5 * (x_crook + x_front), side * y_c, 0.5 * (z_c + z_crook) + 0.02),
                     (x_front, side * y_c, 0.5 * (z_crook + z_pipe)), (x_pipe_front - 0.04, side * y_c, z_pipe)]
        pipe = []
        steps = 24
        for k in range(steps + 1):
            x = x_pipe_front - 0.04 + (x_end - x_pipe_front + 0.04) * k / steps
            _, _, hw = fuselage_at(keys, x)
            pipe.append((x, side * max(hw + 0.035, 0.0), z_pipe))
        tube(parts, collector + pipe[1:], d_pipe, "exhaust")


def build_propeller(data, root):
    prop = data["propeller"]
    hub, axis, radius = Vector(prop["hub"]), Vector(prop["axis"]), prop["radius"]
    up = Vector((0, 0, -1))
    node = new_object("prop", root, node_matrix(hub, axis, up))
    parts = Builder()
    pitch = PITCH_RATIO * 2 * radius
    side_dir = axis.cross(up)          # where the top blade moves (positive rotation)
    for blade in (1, -1):
        rings = []
        for k in range(13):
            r = 0.12 + (radius - 0.12) * k / 12
            frac = r / radius
            chord = (0.10 + 0.10 * math.sin(math.pi * min(frac / 0.9, 1.0) * 0.9)) * (1.0 if frac < 0.93 else max(0.25, (1 - frac) / 0.07))  # E: Éclair planform, widest near 0.6 R
            thick = (0.14 - 0.08 * frac) * chord + (0.03 if frac < 0.2 else 0.0)            # E:
            beta = math.atan2(pitch, 2 * math.pi * r)
            radial = up * blade
            motion = side_dir * blade
            # Chord line: leading edge ahead in the direction of motion, and forward.
            c_dir = (motion * math.cos(beta) + axis * math.sin(beta)).normalized()
            n_dir = (axis * math.cos(beta) - motion * math.sin(beta)).normalized()
            centre = hub + radial * r
            loop = []
            for j in range(16):
                a = 2 * math.pi * j / 16
                loop.append(centre + c_dir * (0.5 * chord * math.cos(a))
                            + n_dir * (0.5 * thick * math.sin(a)))
            rings.append(loop)
        with part(f"prop_blade_{1 if blade > 0 else 2}"):
            parts.loft(rings, "propeller_wood", cap_start=True, cap_end=True)
    # Hub boss.
    with part("prop_hub"):
        round_bar(parts, hub - axis * 0.12, hub + axis * 0.06, 0.20, "propeller_wood")
    return node, parts


def box(parts, centre, size, material):
    """An axis-aligned box (body axes)."""
    c, (sx, sy, sz) = Vector(centre), size
    ring = lambda x: [c + Vector((x, 0.5 * sy * a, 0.5 * sz * b)) for a, b in ((1, 1), (-1, 1), (-1, -1), (1, -1))]
    parts.loft([ring(0.5 * sx), ring(-0.5 * sx)], material, cap_start=True, cap_end=True)


SIGHT_CLEARANCE = 0.01    # m between a ring sight and the gun, or the sight line and the windscreen's frame
GUN_SLOTS = (8, 7)        # E: rows of cooling slots round the Vickers' jacket, and slots along each row
GUN_SLOT = (0.050, 0.016)  # E: length and width of a slot, m


def ring_loop(centre, radius, n=16):
    """A circle square to the x axis (body axes), closed."""
    pts = [centre + Vector((0.0, radius * math.cos(2 * math.pi * k / n), radius * math.sin(2 * math.pi * k / n)))
           for k in range(n)]
    return pts + [pts[0]]


def rounded_bar(parts, x0, x1, y, z, width, height, material, exponent=6.0):
    """A bar along x of rounded-rectangle section (a superellipse), closed at both ends."""
    section = superellipse(z, 0.5 * width, 0.5 * height, exponent, n=24)
    rings = [[Vector((x, y + sy, sz)) for sy, sz in section] for x in (x0, x1)]
    parts.loft(rings, material, cap_start=True, cap_end=True)


def build_gun(data, parts, keys):
    """The Vickers on the cowl, firing through the propeller (spad7-aspect.md § 7): its
    jacket pierced with cooling slots and capped by the muzzle cone, the receiver with its
    top cover, the feed block and belt guide on the right, the fusee spring box on the
    left, the crank handle, two ring sights (« deux viseurs circulaires », Hall) and the
    brackets that hold it on the cowl. Proportions of the Vickers .303 aircraft gun, E:
    until a drawing or a photograph of the S.VII's mounting (spad7-aspect.md § 7)."""
    muzzle = Vector(data["mockup"]["gun_muzzles"][0])
    d, jacket, receiver, width, height = feature(data, "vickers")[0]
    feed_out, feed_len, feed_h, crank, front_ring, rear_ring = feature(data, "vickers_parts")[0]
    eye = Vector(data["mockup"]["pilot_eye"])
    r = 0.5 * d
    x_jacket = muzzle.x - jacket          # rear of the jacket, front of the receiver
    x_rear = x_jacket - receiver
    # Jacket, collars at both ends, muzzle cone.
    round_bar(parts, (x_jacket, 0, muzzle.z), (muzzle.x - 0.06, 0, muzzle.z), d, "steel", n=24)
    for x in (x_jacket + 0.02, muzzle.x - 0.07):
        round_bar(parts, (x - 0.012, 0, muzzle.z), (x + 0.012, 0, muzzle.z), d * 1.08, "steel", n=24)
    tube(parts, [Vector((muzzle.x - 0.06, 0, muzzle.z)), muzzle], d * 0.8, "steel", n=20, scales=[1.0, 0.5])
    round_bar(parts, muzzle - Vector((0.005, 0, 0)), muzzle + Vector((0.015, 0, 0)), 0.018, "steel", n=12)
    # Cooling slots, dark, just proud of the jacket.
    rows, along = GUN_SLOTS
    length, slot_w = GUN_SLOT
    span = (jacket - 0.20) / along
    for i in range(rows):
        a = 2 * math.pi * (i + 0.5) / rows
        da = 0.5 * slot_w / r
        for j in range(along):
            xa = x_jacket + 0.06 + j * span
            xb = xa + length
            quad = [Vector((x, (r + 0.0008) * math.cos(a + s), muzzle.z + (r + 0.0008) * math.sin(a + s)))
                    for x, s in ((xa, -da), (xb, -da), (xb, da), (xa, da))]
            parts.face(quad, "radiator")
    # Receiver, its top cover and the hinge of the cover.
    z = muzzle.z + 0.01
    rounded_bar(parts, x_rear, x_jacket, 0.0, z, width, height, "steel")
    rounded_bar(parts, x_rear + 0.03, x_jacket - 0.02, 0.0, z - 0.5 * height - 0.006, 0.8 * width, 0.014, "steel")
    round_bar(parts, (x_rear + 0.01, 0.0, z - 0.5 * height + 0.004), (x_rear + 0.06, 0.0, z - 0.5 * height + 0.004),
              0.016, "steel", n=10)
    # Feed block out to the right at the front, its belt guide going down into the cowl.
    yf = 0.5 * width + 0.5 * feed_out
    box(parts, (x_jacket - 0.5 * feed_len - 0.02, yf, z - 0.2 * height), (feed_len, feed_out, feed_h), "steel")
    top, _, _ = fuselage_at(keys, x_jacket - 0.06)
    box(parts, (x_jacket - 0.5 * feed_len - 0.02, 0.5 * width + feed_out - 0.004, 0.5 * (z + top)),
        (feed_len * 0.9, 0.006, max(top - z, 0.02)), "metal")
    # Fusee spring box on the left.
    box(parts, (x_jacket - 0.07, -(0.5 * width + 0.014), z + 0.01), (0.09, 0.028, 0.055), "steel")
    # Crank handle on the right: from its pivot at the rear, forward and down, a roller at its end.
    pivot = Vector((x_rear + 0.05, 0.5 * width + 0.012, z))
    end = pivot + Vector((crank * math.cos(math.radians(15)), 0.0, crank * math.sin(math.radians(15))))
    round_bar(parts, pivot, end, 0.012, "steel", n=8)
    round_bar(parts, end - Vector((0, 0.012, 0)), end + Vector((0, 0.022, 0)), 0.022, "steel", n=12)
    # Rear cross-piece (the trigger motor's housing) below the back of the receiver.
    round_bar(parts, (x_rear + 0.02, -0.5 * width - 0.01, z + 0.3 * height),
              (x_rear + 0.02, 0.5 * width + 0.01, z + 0.3 * height), 0.032, "steel", n=12)
    # Two ring sights, their centres on the sight line: through the pilot's eye, parallel
    # to the bore (harmonised at infinity until the guns are), so that they line up in
    # the cockpit view (player, 2026-10-29). The rear one over the back of the receiver,
    # the front one over the jacket's front collar, on posts from the gun's top.
    for x, ring_d, base in ((x_rear + 0.04, rear_ring, z - 0.5 * height - 0.013),
                            (muzzle.x - 0.07, front_ring, muzzle.z - r * 1.08)):
        bottom = eye.z + 0.5 * ring_d
        if bottom > base - SIGHT_CLEARANCE:
            raise SystemExit(f"the ring sight at x {x:.2f} does not clear the gun: the pilot's "
                             f"eye (pilot_eye_m) must be {bottom - base + SIGHT_CLEARANCE:.3f} m higher")
        tube(parts, ring_loop(Vector((x, 0.0, eye.z)), 0.5 * ring_d), 0.004, "steel", n=6)
        round_bar(parts, (x, 0.0, base), (x, 0.0, bottom), 0.005, "steel", n=6)
    # Brackets down to the cowl, under the jacket's rear collar and under the receiver.
    for x, bottom in ((x_jacket + 0.02, muzzle.z + 0.54 * d), (x_rear + 0.06, z + 0.5 * height)):
        top, _, _ = fuselage_at(keys, x)
        if top - bottom > 0.004:
            box(parts, (x, 0.0, 0.5 * (top + bottom)), (0.025, 0.06, top - bottom), "steel")


def build_wires(data, wires):
    """Rigging (S48): flying wires from the fuselage at the lower roots to the tops of the
    outer struts, landing wires from the cabane to their feet, both crossing at the
    intermediate struts; incidence wires across each pair of struts; cabane cross-bracing;
    the tailplane braced to the fin and the sternpost. One node `wire_<kind>_<l|r>_<n>`
    per wire, so that a cut one can go (pipeline-assets; M4 plan, step 1). Returns each
    node's diameter, for the game's wire shader (glTF extras)."""
    flying, landing, incidence, tandem = feature(data, "wires")[0]
    upper = next(s for s in data["surfaces"] if s["name"] == "upper_wing")
    lower = next(s for s in data["surfaces"] if s["name"] == "lower_wing")
    chord = upper["panels"][0]["chord_root"]
    le_x = upper["panels"][0]["root"][0] + 0.25 * chord
    spars = [le_x - f * chord for f in feature(data, "strut_spars")[0]]
    outer = max(y for _, y in data["mockup"]["struts"])
    inner = min(y for _, y in data["mockup"]["struts"])
    root_y = lower["panels"][0]["root"][1]
    cab_y, _ = feature(data, "cabane")[0]
    up = lambda x, y: wing_surface_z(data, "upper_wing", x, y, upper=False)
    low = lambda x, y: wing_surface_z(data, "lower_wing", x, y, upper=True)
    diameters = {}

    def wire(kind, side, points, diameter):
        lr = "l" if side < 0 else "r"
        n = 1 + sum(1 for name in wires if name.startswith(f"wire_{kind}_{lr}_"))
        name = f"wire_{kind}_{lr}_{n}"
        wires[name] = Builder()
        tube(wires[name], points, diameter, "wire", n=6)
        diameters[name] = diameter
        ENDS[name] = (Vector(points[0]), Vector(points[-1]), diameter)

    for side in (1, -1):
        for x in spars:
            for dx in (-0.5 * tandem, 0.5 * tandem):
                wire("flying", side, [(x + dx, side * root_y, low(x, root_y)),
                                      (x + dx, side * outer, up(x, outer))], flying)
            wire("landing", side, [(x, side * cab_y, up(x, cab_y)), (x, side * outer, low(x, outer))],
                 landing)
        for y in (inner, outer):
            (xf, xr) = spars
            for a, b in (((xf, up(xf, y)), (xr, low(xr, y))), ((xf, low(xf, y)), (xr, up(xr, y)))):
                wire("incidence", side, [(a[0], side * y, a[1]), (b[0], side * y, b[1])], incidence)
    # Cabane: a cross in the plane of each pair of cabane struts, and between them.
    z_foot = feature(data, "cabane")[0][1] + 0.05
    xf, xr = spars
    for side in (1, -1):
        for a, b in (((xf, up(xf, cab_y)), (xr, z_foot)), ((xf, z_foot), (xr, up(xr, cab_y)))):
            wire("cabane", side, [(a[0], side * cab_y, a[1]), (b[0], side * cab_y, b[1])], incidence)
    # Tailplane bracing to the fin above and the sternpost below.
    x, y, z_top, z_bottom = feature(data, "tail_bracing")[0]
    for side in (1, -1):
        for z in (z_top, z_bottom):
            wire("tail", side, [(x, side * y, -0.70), (x, 0.0, z)], landing)
    return diameters


def build_linkage(data, parts):
    """Outside aileron linkage (S48): a rod from a bellcrank on the lower plane, beside
    the outer struts, up to the aileron's horn."""
    x_crank, y, x_horn, rod = feature(data, "aileron_linkage")[0]
    for side in (1, -1):
        z_low = wing_surface_z(data, "lower_wing", x_crank, y, upper=True)
        z_up = wing_surface_z(data, "upper_wing", x_horn, y, upper=False)
        round_bar(parts, (x_crank, side * y, z_low - 0.06), (x_horn, side * y, z_up + 0.07), rod, "metal")
        box(parts, (x_crank, side * y, z_low - 0.03), (0.10, 0.008, 0.07), "metal")
        box(parts, (x_horn, side * y, z_up + 0.035), (0.06, 0.006, 0.08), "metal")


def build_windscreen(data, parts, keys):
    """A small glass screen ahead of the cockpit, raked back, on a metal frame. The sight
    line, at the pilot's eye, goes through the glass, clear of the frame."""
    x, half, height, rake = feature(data, "windscreen")[0]
    top, _, _ = fuselage_at(keys, x)
    eye = data["mockup"]["pilot_eye"][2]
    if not (top - height < eye - SIGHT_CLEARANCE - 0.012 and eye < top - SIGHT_CLEARANCE):
        raise SystemExit(f"the sight line (pilot_eye_m, z {eye:.3f}) must cross the windscreen's "
                         f"glass, from z {top:.3f} to {top - height:.3f}, clear of its frame")
    lean = height * math.tan(math.radians(rake))
    a, b = (x, half, top), (x, -half, top)
    c, d = (x - lean, -half * 0.9, top - height), (x - lean, half * 0.9, top - height)
    parts.face([a, b, c, d], "glass")
    tube(parts, [a, d, c, b], 0.012, "metal", n=6)


# --- Damage: hit volumes, effect points, damaged variants (M4 plan, steps 3 and 4) -------

ENDS = {}               # each wire's ends and diameter, from build_wires
SPLINTER = 0.04         # E: depth of the jagged end of broken wood, m
STUB = 0.12             # E: length of a broken spar or longeron left in place, m
STREAM = math.radians(12)   # E: a cut wire's halves stream aft, this far below the flight path


def oriented_box(parts, a, b, width, height, material, up=Vector((0, 0, -1)), jagged=False):
    """A box from a to b (body axes), `width` across and `height` along `up`; with
    `jagged`, its end at b is splintered (broken wood): its corners at uneven depths and a
    point out of the middle."""
    a, b = Vector(a), Vector(b)
    axis = (b - a).normalized()
    side = axis.cross(up)
    if side.length < 1e-6:
        side = axis.cross(Vector((1, 0, 0)))
    side.normalize()
    normal = side.cross(axis).normalized()
    w, h = 0.5 * width, 0.5 * height
    corners = [(w, h), (-w, h), (-w, -h), (w, -h)]
    start = [a + side * x + normal * y for x, y in corners]
    if not jagged:
        end = [b + side * x + normal * y for x, y in corners]
        parts.loft([start, end], material, cap_start=True, cap_end=True)
        return
    depths = (0.0, -0.6, -0.25, -0.9)
    end = [b + axis * (SPLINTER * d) + side * x + normal * y for (x, y), d in zip(corners, depths)]
    parts.loft([start, end], material, cap_start=True)
    tip = b + axis * (0.8 * SPLINTER)
    for i in range(4):
        parts.face([end[i], end[(i + 1) % 4], tip], material)


def write_node(builder, name, parent, materials, smooth=math.radians(35)):
    """One node from a builder, its origin at the centre of its bounds."""
    builder.prepare()
    corners = [v.co for v in builder.bm.verts]
    centre = sum(corners, Vector()) / len(corners)
    obj = new_object(name, parent, Matrix.Translation(centre))
    builder.write(obj, materials, smooth=smooth)
    return obj


def build_hits(data, keys, body, materials):
    """Hit volumes `hit_<component>` of the parts no visible mesh stands for (the visible
    parts are their own hit shapes): engine, radiator, tanks, oil, ammunition, gear of
    the synchroniser, carburettor, magnetos, pilot, the control cables' runs, each wing
    panel's spars. Closed convex boxes, hidden in the game (pipeline-assets)."""
    def box(name, centre, size):
        b = Builder()
        c, (sx, sy, sz) = Vector(centre), size
        oriented_box(b, c - Vector((0.5 * sx, 0, 0)), c + Vector((0.5 * sx, 0, 0)), sy, sz, "hit")
        write_node(b, name, body, materials)

    def bar(name, a, b_, width, height):
        b = Builder()
        oriented_box(b, a, b_, width, height, "hit")
        write_node(b, name, body, materials)

    # Engine: the two cylinder banks under the camshaft covers, the crankcase down to
    # below the crankshaft (C: covers and hub of maquette.ron; E: margins).
    y, z, d, x0, x1 = feature(data, "camshaft_bulges")[0]
    hub_z = data["propeller"]["hub"][2]
    top, bottom = z - 0.5 * d, hub_z + 0.20
    box("hit_engine", (0.5 * (x0 + x1) + 0.05 - 0.10, 0.0, 0.5 * (top + bottom)),
        (x1 - x0 + 0.30, 2 * y + 0.12, bottom - top))
    x_face, zc, diameter = feature(data, "radiator")[0]
    box("hit_radiator", (x_face - 0.06, 0.0, zc), (0.12, diameter, diameter))
    for name in ("hit_fuel_main", "hit_fuel_gravity", "hit_oil", "hit_ammunition",
                 "hit_synchronizer", "hit_carburettor"):
        x, y, z, sx, sy, sz = feature(data, name)[0]
        box(name, (x, y, z), (sx, sy, sz))
    for (x, y, z, sx, sy, sz), lr in zip(feature(data, "hit_magnetos"), ("l", "r")):
        box(f"hit_magneto_{lr}", (x, y, z), (sx, sy, sz))
    for (x, y, z, sx, sy, sz), name in zip(feature(data, "hit_pilot"),
                                            ("head", "torso", "arm_l", "arm_r", "legs")):
        box(f"hit_pilot_{name}", (x, y, z), (sx, sy, sz))

    # Control cables: elevator and rudder along the bottom longerons from the cockpit to
    # the tail, ailerons from the stick down to the lower plane and along it to the
    # bellcrank (E: runs, damage 20).
    x_tail = keys[-1][0] + 0.25
    for lr, sy in (("l", -1), ("r", 1)):
        for control, dz in (("elevator", -0.03), ("rudder", 0.03)):
            a_top, a_bottom, a_hw = fuselage_at(keys, -1.15)
            b_top, b_bottom, b_hw = fuselage_at(keys, x_tail)
            bar(f"hit_cable_{control}_{lr}", (-1.15, sy * 0.6 * a_hw, a_bottom - 0.08 + dz),
                (x_tail, sy * 0.6 * b_hw, b_bottom - 0.03 + dz), 0.03, 0.03)
        x_crank, y_crank, _, _ = feature(data, "aileron_linkage")[0]
        lower = next(s for s in data["surfaces"] if s["name"] == "lower_wing")
        root_y = lower["panels"][0]["root"][1]
        z_root = wing_surface_z(data, "lower_wing", x_crank, root_y, upper=True) + 0.03
        z_crank = wing_surface_z(data, "lower_wing", x_crank, y_crank, upper=True) + 0.03
        # Two straight runs: one convex volume each, numbered (pipeline-assets).
        bar(f"hit_cable_aileron_{lr}_1", (-1.05, 0.0, -0.45), (x_crank, sy * root_y, z_root),
            0.03, 0.03)
        bar(f"hit_cable_aileron_{lr}_2", (x_crank, sy * root_y, z_root),
            (x_crank, sy * y_crank, z_crank), 0.03, 0.03)

    # Each wing panel's two spars (damages 5 to 7).
    for plane, (inner_y, outer_y, tip_y) in wing_panels(data).items():
        for lr, sy in (("l", -1), ("r", 1)):
            for panel, (y0, y1) in zip(("root", "mid", "tip"),
                                       ((inner_y[0], inner_y[1]), (outer_y[0], outer_y[1]), (tip_y[0], tip_y[1]))):
                for spar, (x, h) in zip(("front", "rear"), spar_lines(data, plane)):
                    a = (x, sy * y0, spar_z(data, plane, x, y0))
                    b_ = (x, sy * y1, spar_z(data, plane, x, y1))
                    bar(f"hit_spar_wing_{plane.split('_')[0]}_{lr}_{panel}_{spar}", a, b_, SPAR_WIDTH, h)


def wing_panels(data):
    """Spanwise ranges (y from, y to) of each wing's root, mid and tip panels: cut at the
    intermediate and outer struts (SPLIT), the tip short of its rounding."""
    out = {}
    for name in ("upper_wing", "lower_wing"):
        surface = next(s for s in data["surfaces"] if s["name"] == name)
        root_y = surface["panels"][0]["root"][1]
        tip_y = surface["panels"][-1]["tip"][1] - TIP_RADIUS
        out[name] = ((root_y, SPLIT["inner"]), (SPLIT["inner"], SPLIT["outer"]), (SPLIT["outer"], tip_y))
    return out


def spar_lines(data, plane):
    """x and depth of a wing's front and rear spars (wing_spar, FRAME_DEPTH)."""
    surface = next(s for s in data["surfaces"] if s["name"] == plane)
    chord = surface["panels"][0]["chord_root"]
    le_x = surface["panels"][0]["root"][0] + 0.25 * chord
    out = []
    for u in feature(data, "strut_spars")[0]:
        x = le_x - u * chord
        hu, hl = spad_section(u)
        out.append((x, (hu - hl) * chord * FRAME_DEPTH))
    return out


def spar_z(data, plane, x, y):
    """z of a wing's mean line at (x, y)."""
    return 0.5 * (wing_surface_z(data, plane, x, y, upper=True) + wing_surface_z(data, plane, x, y, upper=False))


def build_effect_points(data, keys, body):
    """Empty nodes `fx_<name>` where smoke, steam, flames and leaks come from (damages 26
    to 35): the engine under the cowl's top, the radiator, each exhaust's end, the two
    tanks, the cockpit."""
    y, z, d, x0, x1 = feature(data, "camshaft_bulges")[0]
    x_face, zc, diameter = feature(data, "radiator")[0]
    z_pipe, _, _, x_end = feature(data, "exhaust_pipe")[0]
    _, _, hw_end = fuselage_at(keys, x_end)
    x_tank = feature(data, "hit_fuel_main")[0][0]
    _, tank_bottom, _ = fuselage_at(keys, x_tank)
    xg, _, zg, _, _, sg = feature(data, "hit_fuel_gravity")[0]
    points = {
        "fx_engine": (0.5 * (x0 + x1), 0.0, z - 0.5 * d),
        "fx_radiator": (x_face, 0.0, zc - 0.35 * diameter),
        "fx_exhaust_l": (x_end, -(hw_end + 0.035), z_pipe),
        "fx_exhaust_r": (x_end, hw_end + 0.035, z_pipe),
        "fx_fuel_main": (x_tank, 0.0, tank_bottom),
        "fx_fuel_gravity": (xg, 0.0, zg + 0.5 * sg),
        "fx_cockpit": (-1.05, 0.0, -0.95),
    }
    for name, p in points.items():
        new_empty(name, body, node_matrix(p, (0, 1, 0), (0, 0, -1)))


def build_damaged(data, keys, body, prop, materials):
    """Damaged variants `dmg_<part>_<state>`, hidden in the game until the damage
    happens (damages 6, 7, 9 to 13, 17, 19, 25, 46): what stays of a lost wing panel or
    rear fuselage (splintered spars, longerons), each strut broken in two, each wire cut
    with its halves streaming aft, each blade broken, the windscreen shattered, each
    undercarriage leg broken."""
    def node(name, build, parent=body, smooth=math.radians(35)):
        b = Builder()
        build(b)
        return write_node(b, name, parent, materials, smooth)

    # Wing panels: the stub left on the inboard part when a panel goes.
    for plane, ranges in wing_panels(data).items():
        short = plane.split("_")[0]
        for lr, sy in (("l", -1), ("r", 1)):
            for panel, (y0, _) in zip(("root", "mid", "tip"), ranges):
                def stub(b, y0=y0, sy=sy, plane=plane):
                    for x, h in spar_lines(data, plane):
                        z0 = spar_z(data, plane, x, y0)
                        oriented_box(b, (x, sy * (y0 - 0.02), z0), (x, sy * (y0 + STUB), z0),
                                     SPAR_WIDTH, h, "wood", jagged=True)
                node(f"dmg_wing_{short}_{lr}_{panel}_stub", stub)

    # Rear fuselage: the four longerons broken behind the cockpit.
    def fuselage_stub(b):
        x = FUSELAGE_SPLIT_X
        top, bottom, hw = fuselage_at(keys, x)
        k = 0.9 * 0.5 ** (1.0 / 3.6)
        zc, hh = 0.5 * (top + bottom), 0.5 * (bottom - top)
        for sy_, sz in ((1, -1), (-1, -1), (-1, 1), (1, 1)):
            p = Vector((x + 0.02, sy_ * k * hw, zc + sz * k * hh))
            oriented_box(b, p, p - Vector((STUB, 0, 0)), LONGERON, LONGERON, "wood", jagged=True)
    node("dmg_fuselage_rear_stub", fuselage_stub)

    # Struts broken in two, each half splintered at the break.
    upper = next(s for s in data["surfaces"] if s["name"] == "upper_wing")
    chord = upper["panels"][0]["chord_root"]
    le_x = upper["panels"][0]["root"][0] + 0.25 * chord
    spars = [le_x - f * chord for f in feature(data, "strut_spars")[0]]
    for _, y in data["mockup"]["struts"]:
        io = "inner" if abs(y - SPLIT["inner"]) < abs(y - SPLIT["outer"]) else "outer"
        for lr, sy in (("l", -1), ("r", 1)):
            for fr, x in zip(("front", "rear"), spars):
                bottom = Vector((x, sy * y, wing_surface_z(data, "lower_wing", x, y, upper=True)))
                top = Vector((x, sy * y, wing_surface_z(data, "upper_wing", x, y, upper=False)))
                def broken(b, bottom=bottom, top=top):
                    oriented_box(b, bottom, bottom.lerp(top, 0.45), STRUT_CHORD, STRUT_THICK, "wood",
                                 up=Vector((1, 0, 0)), jagged=True)
                    oriented_box(b, top, top.lerp(bottom, 0.40), STRUT_CHORD, STRUT_THICK, "wood",
                                 up=Vector((1, 0, 0)), jagged=True)
                node(f"dmg_strut_{lr}_{io}_{fr}_broken", broken)

    # Wires cut in the middle, their halves streaming aft from their ends.
    for name, (a, b_, diameter) in sorted(ENDS.items()):
        half = 0.5 * (b_ - a).length
        aft = Vector((-math.cos(STREAM), 0.0, math.sin(STREAM)))
        def cut(b, a=a, b_=b_, half=half, diameter=diameter, aft=aft):
            for end in (a, b_):
                tube(b, [end, end + aft * half], diameter, "wire", n=6)
        # Drawn by the wire shader too (render/wires.rs).
        node(f"dmg_{name}_cut", cut, smooth=math.pi)["diameter_m"] = diameter

    # Propeller blades broken at a third of their length, under `prop` (they turn).
    p = data["propeller"]
    hub, axis, radius = Vector(p["hub"]), Vector(p["axis"]), p["radius"]
    for blade, k in ((1, 1), (-1, 2)):
        def stump(b, blade=blade):
            root = hub + Vector((0, 0, -1)) * blade * 0.12
            end = hub + Vector((0, 0, -1)) * blade * 0.38 * radius
            oriented_box(b, root, end, 0.16, 0.04, "propeller_wood", up=axis, jagged=True)
        node(f"dmg_prop_blade_{k}_stub", stump, parent=prop)

    # The windscreen shattered: its frame, and shards in its corners.
    x, half, height, rake = feature(data, "windscreen")[0]
    top, _, _ = fuselage_at(keys, x)
    lean = height * math.tan(math.radians(rake))
    a, b_ = Vector((x, half, top)), Vector((x, -half, top))
    c, d = Vector((x - lean, -half * 0.9, top - height)), Vector((x - lean, half * 0.9, top - height))
    def shattered(b):
        tube(b, [a, d, c, b_], 0.012, "metal", n=6)
        b.face([a, a.lerp(b_, 0.35), a.lerp(d, 0.30)], "glass")
        b.face([b_, b_.lerp(c, 0.45), b_.lerp(a, 0.25)], "glass")
        b.face([d, d.lerp(a, 0.20), d.lerp(c, 0.30)], "glass")
    node("dmg_windscreen_broken", shattered)

    # Undercarriage legs broken: what stays under the fuselage.
    for lr, side in (("l", -1), ("r", 1)):
        fx, fy, fz, ax, ay, az, chord = feature(data, "gear_front_leg")[0]
        rx, ry, rz, bx, by, bz = feature(data, "gear_rear_leg")[0]
        def legs(b, side=side):
            f0, f1 = Vector((fx, side * fy, fz)), Vector((ax, side * ay, az))
            r0, r1 = Vector((rx, side * ry, rz)), Vector((bx, side * by, bz))
            oriented_box(b, f0, f0.lerp(f1, 0.5), chord, LEG_THICK, "wood", up=Vector((1, 0, 0)), jagged=True)
            oriented_box(b, r0, r0.lerp(r1, 0.4), REAR_LEG_CHORD, LEG_THICK, "wood", up=Vector((1, 0, 0)), jagged=True)
        node(f"dmg_gear_{lr}_broken", legs)


# --- Fabric textures (pass 5) ------------------------------------------------------------

TEXTURE_SIZE = 4096      # plan B5: 4K for the airframe
TEXTURE_MARGIN = 8       # px of bleed around each region of the atlas, for filtering
MOTTLING = 0.015         # E: ± tone of the unevenly brushed dope and varnish
MOTTLING_WAVES = 6       # in patches of 0.3 to 1.5 m (E:)
STITCH_RADIUS = 0.004    # E: radius of a knot of the rib stitching
STITCH_HEIGHT = 0.0004   # E: its height over the tape

KINDS = {"fuselage": "fuselage"}   # kind of each painted surface (main)

# Directions (body axes) of a region's u and v in its flat projection: the image of each
# face seen from outside, its u, v and outward normal right-handed, so that a mark reads
# the right way round and the normal map's tangent frame is the projection's.
AXES = {
    "top": (Vector((0, 1, 0)), Vector((1, 0, 0))),        # from above, nose up
    "bottom": (Vector((0, -1, 0)), Vector((1, 0, 0))),    # from below, nose up
    "right": (Vector((1, 0, 0)), Vector((0, 0, -1))),     # from the right, nose right
    "left": (Vector((-1, 0, 0)), Vector((0, 0, -1))),     # from the left, nose left
}
# The fuselage's top and bottom lie along the image, as its sides do.
FUSELAGE_AXES = dict(AXES, top=(Vector((1, 0, 0)), Vector((0, -1, 0))),
                     bottom=(Vector((-1, 0, 0)), Vector((0, -1, 0))))


def face_side(surface, n):
    """Side of a painted face from its outward normal (body axes): top or bottom of a
    horizontal surface, right or left of a vertical one, whichever faces most for the
    fuselage."""
    kind = KINDS[surface]
    if kind == "fuselage" and abs(n.y) >= abs(n.z) or kind == "vertical":
        return "right" if n.y > 0 else "left"
    return "top" if n.z < 0 else "bottom"


def projection(region):
    surface, side = region
    return (FUSELAGE_AXES if surface == "fuselage" else AXES)[side]


class Atlas:
    """The fabric's texture: one region per painted surface and side, its faces projected
    flat along AXES, all at the same density (pixels per metre), packed in shelves into a
    square of TEXTURE_SIZE. Rows count from the bottom, as Blender stores images."""

    def __init__(self, builders, size=TEXTURE_SIZE, margin=TEXTURE_MARGIN):
        self.size, self.margin = size, margin
        extents = {}
        for b in builders:
            for f, region in b.painted_faces():
                u_dir, v_dir = projection(region)
                for vert in f.verts:
                    p = body_vector(vert.co)
                    u, v = p.dot(u_dir), p.dot(v_dir)
                    e = extents.setdefault(region, [u, u, v, v])
                    e[:] = [min(e[0], u), max(e[1], u), min(e[2], v), max(e[3], v)]
        self.extents = extents
        # Tallest first, then by name: the same layout every time.
        order = sorted(extents, key=lambda r: (-(extents[r][3] - extents[r][2]), r))
        density = size / max(max(e[1] - e[0], e[3] - e[2]) for e in extents.values())
        while (place := self.pack(order, density)) is None:
            density *= 0.99
        self.density, self.place = density, place

    def pack(self, order, density):
        x = y = shelf = 0
        place = {}
        for r in order:
            u0, u1, v0, v1 = self.extents[r]
            w = math.ceil((u1 - u0) * density) + 2 * self.margin
            h = math.ceil((v1 - v0) * density) + 2 * self.margin
            if x + w > self.size:
                x, y, shelf = 0, y + shelf, 0
            if x + w > self.size or y + h > self.size:
                return None
            place[r] = (x, y, w, h)
            x, shelf = x + w, max(shelf, h)
        return place

    def uv(self, region, p):
        u_dir, v_dir = projection(region)
        u0, _, v0, _ = self.extents[region]
        x, y, _, _ = self.place[region]
        return ((x + self.margin + (p.dot(u_dir) - u0) * self.density) / self.size,
                (y + self.margin + (p.dot(v_dir) - v0) * self.density) / self.size)

    def grid(self, region):
        """u and v (m) of the centres of a region's pixels, arrays of its rows."""
        u0, _, v0, _ = self.extents[region]
        _, _, w, h = self.place[region]
        u = u0 + (np.arange(w) + 0.5 - self.margin) / self.density
        v = v0 + (np.arange(h) + 0.5 - self.margin) / self.density
        return np.meshgrid(u, v)


def coverage(signed, density):
    """Share of a pixel inside an edge, `signed` metres from it (positive inside)."""
    return np.clip(signed * density + 0.5, 0.0, 1.0)


class Ribs:
    """The ribs of a surface: where they stand along the span, and its leading and
    trailing edges, for the tapes, stitching and sag painted on its fabric."""

    def __init__(self, surface, data):
        name, kind = surface["name"], surface["kind"]
        tail = name in data["mockup"].get("outlines", {})
        self.pitch = feature(data, "tail_ribs")[0][0] if tail else RIBS[0]
        self.false_ribs = feature(data, "false_ribs")[0][0] if name.endswith("wing") else None
        # Spanwise coordinate: |y| on a horizontal surface, height above the fin's foot on
        # a vertical one; ribs from each panel's root, as the scallops of build_surface.
        self.vertical = kind == "vertical"
        foot = surface["panels"][0]["root"][2]
        self.foot = foot
        span = (lambda p: foot - p[2]) if self.vertical else (lambda p: abs(p[1]))
        panels = surface["panels"]
        if tail:
            # The tail's outline is not split in panels: ribs from the axis or the foot.
            self.segments = [(0.0, max(span(p["tip"]) for p in panels))]
            rows = sorted(data["mockup"]["outlines"][name], key=lambda r: abs(r[0]))
            s = [abs(r[0]) if not self.vertical else foot - r[0] for r in rows]
            le, te = smoothed([r[1] for r in rows]), smoothed([r[2] for r in rows])
        else:
            self.segments = [(span(p["root"]), span(p["tip"])) for p in panels]
            s, le, te = [], [], []
            for p in panels:
                for k in range(21):
                    st = Station(p, k / 20, 1.0, kind)
                    s.append(span(st.qc))
                    le.append(st.point(0.0).x)
                    te.append(st.point(1.0).x)
        order = np.argsort(s)
        self.s, self.le, self.te = (np.array(a)[order] for a in (s, le, te))

    def span(self, X, Y, Z):
        return self.foot - Z if self.vertical else np.abs(Y)

    def phase(self, s):
        """Fraction of the rib pitch from the last rib, and distance to the nearest, m."""
        t = np.full(s.shape, 0.5)
        d = np.full(s.shape, np.inf)
        for s0, s1 in self.segments:
            inside = (s >= s0 - self.pitch) & (s <= s1 + self.pitch)
            k = (s - s0) / self.pitch
            frac = k - np.floor(k)
            t = np.where(inside, frac, t)
            d = np.where(inside, np.minimum(frac, 1.0 - frac) * self.pitch, d)
        return t, d

    def chord_fraction(self, s, X):
        le, te = np.interp(s, self.s, self.le), np.interp(s, self.s, self.te)
        return (le - X) / np.maximum(le - te, 1e-3), le, le - te


def fabric_structure(ribs, side, X, Y, Z, data, density):
    """Tape coverage (0 to 1) and height of the fabric (m, outwards) over a region: rib
    tapes and their stitching, false ribs on the upper surfaces, sag between ribs."""
    width, thickness, stitch_pitch, _ = feature(data, "rib_tapes")[0]
    sag = feature(data, "fabric_sag")[0][0]
    s = ribs.span(X, Y, Z)
    t, d = ribs.phase(s)
    f, le, chord = ribs.chord_fraction(s, X)
    envelope = np.sqrt(np.clip(np.sin(np.pi * np.clip(f, 0.0, 1.0)), 0.0, 1.0))
    sink = np.sin(np.pi * t)
    tape = coverage(0.5 * width - d, density)
    along = (le - X) / stitch_pitch
    knot_d = np.hypot(d, (along - np.round(along)) * stitch_pitch)
    knots = np.clip(1.0 - (knot_d / STITCH_RADIUS) ** 2, 0.0, 1.0)
    if ribs.false_ribs is not None and side == "top":
        # Ahead of the front spar a false rib halves the bay: the fabric sinks less.
        ahead = coverage((ribs.false_ribs - f) * chord, density)
        d2 = np.abs(t - 0.5) * ribs.pitch
        tape = np.maximum(tape, coverage(0.5 * width - d2, density) * ahead)
        knot_d2 = np.hypot(d2, (along - np.round(along)) * stitch_pitch)
        knots = np.maximum(knots, np.clip(1.0 - (knot_d2 / STITCH_RADIUS) ** 2, 0.0, 1.0) * ahead)
        half = np.abs(np.sin(2.0 * np.pi * t))
        sink = ahead * 0.5 * half + (1.0 - ahead) * sink
    height = -sag * sink * envelope + thickness * tape + STITCH_HEIGHT * knots * (tape > 0.5)
    return tape, height


def transmission_map(ribs, X, Y, Z, tape, data, density):
    """Share of the sunlight the fabric lets through, seen against the light: less under
    a tape, almost none over a rib's cap, a false rib or a spar inside the wing."""
    between, taped, wood, rib_width, spar_width, _ = feature(data, "fabric_transmission")[0]
    s = ribs.span(X, Y, Z)
    t, d = ribs.phase(s)
    f, _, chord = ribs.chord_fraction(s, X)
    blocked = coverage(0.5 * rib_width - d, density)
    if ribs.false_ribs is not None:
        # The planes: their spars under the struts, false ribs ahead of the front one.
        for spar in feature(data, "strut_spars")[0]:
            blocked = np.maximum(blocked, coverage(0.5 * spar_width - np.abs(f - spar) * chord, density))
        ahead = coverage((ribs.false_ribs - f) * chord, density)
        blocked = np.maximum(blocked, coverage(0.5 * rib_width - np.abs(t - 0.5) * ribs.pitch, density) * ahead)
    fabric = between + (taped - between) * tape
    return fabric * (1.0 - blocked) + wood * blocked


def transmission_depth(data):
    """How far behind a skin the game looks for the light it lets through (m): past the
    opposite skin where the surfaces are thickest, so that only another part (the other
    plane, the fuselage) can shade it (crates/game/src/render/fabric.rs)."""
    wing = max(u - l for u, l in zip(SPAD_UPPER, SPAD_LOWER)) / 100.0 * max(
        p["chord_root"] for s in data["surfaces"] if s["name"].endswith("wing") for p in s["panels"])
    tail = TAIL_THICKNESS * max(r[1] - r[2] for rows in data["mockup"]["outlines"].values() for r in rows)
    return max(wing, tail) + 0.02


def normal_map(height, density):
    """Tangent-space normals (OpenGL convention, as glTF's) of a height field whose
    columns run along u and rows along v."""
    dv, du = np.gradient(height, 1.0 / density)
    n = np.stack([-du, -dv, np.ones_like(height)], axis=-1)
    n /= np.linalg.norm(n, axis=-1, keepdims=True)
    return n * 0.5 + 0.5


def paint_fabric(atlas, data, livery, colours, controls):
    """Colour map (sRGB, the fabric's transmission in alpha) and normal map of the fabric,
    from the livery's marks and the fabric's structure, as (size, size, 4) and
    (size, size, 3) arrays."""
    size = atlas.size
    fabric = np.array(colours[livery["fabric"]], dtype=np.float64)
    colour = np.empty((size, size, 4), np.float32)
    colour[..., :3] = fabric
    colour[..., 3] = feature(data, "fabric_transmission")[0][0]
    normal = np.empty((size, size, 3), np.float32)
    normal[:] = (0.5, 0.5, 1.0)
    rng = np.random.default_rng(1917)
    waves = [(2 * math.pi / rng.uniform(0.3, 1.5), rng.normal(size=3), rng.uniform(0, 2 * math.pi))
             for _ in range(MOTTLING_WAVES)]
    ribs = {s["name"]: Ribs(s, data) for s in data["surfaces"]}
    for region in sorted(atlas.place):
        surface, side = region
        u_dir, v_dir = projection(region)
        gu, gv = atlas.grid(region)
        X, Y, Z = (gu * u_dir[i] + gv * v_dir[i] for i in range(3))
        # Mottling in body axes, the same on both sides of an edge; along the fuselage's
        # length only, its sides and top being projected differently.
        mx, my, mz = (X, 0 * X, 0 * X) if surface == "fuselage" else (X, Y, Z)
        m = sum(np.sin(k * (w[0] * mx + w[1] * my + w[2] * mz) / np.linalg.norm(w) + phi)
                for k, w, phi in waves) / MOTTLING_WAVES
        col = np.broadcast_to(fabric, gu.shape + (3,)) * (1.0 + MOTTLING * m)[..., None]
        paint = np.zeros_like(gu)
        for mark in livery["marks"]:
            col, cover = paint_mark(mark, region, gu, gv, col, colours, data, controls, atlas.density)
            paint = np.maximum(paint, cover)
        height = np.zeros_like(gu)
        through = np.full(gu.shape, feature(data, "fabric_transmission")[0][0])
        if surface in ribs:
            tape, height = fabric_structure(ribs[surface], side, X, Y, Z, data, atlas.density)
            tone = feature(data, "rib_tapes")[0][3]
            col = col * (1.0 - (1.0 - tone) * tape)[..., None]
            through = transmission_map(ribs[surface], X, Y, Z, tape, data, atlas.density)
        painted = feature(data, "fabric_transmission")[0][5]
        through = through * (1.0 - paint) + np.minimum(through, painted) * paint
        x, y, w, h = atlas.place[region]
        colour[y:y + h, x:x + w, :3] = np.clip(col, 0.0, 1.0)
        colour[y:y + h, x:x + w, 3] = through
        normal[y:y + h, x:x + w] = normal_map(height, atlas.density)
    return colour, normal


def paint_mark(mark, region, gu, gv, col, colours, data, controls, density):
    """A livery's mark painted over a region's colours, if it falls on it, and the share
    of each pixel it covers (paint lets less light through the fabric)."""
    none = np.zeros_like(gu)
    surface, side = region
    u_dir, v_dir = projection(region)
    def over(col, inside, name):
        a = inside[..., None]
        return col * (1.0 - a) + np.array(colours[name]) * a
    if mark["kind"] == "roundel":
        if mark["face"] != f"{surface}.{side}":
            return col, none
        c = Vector(mark["centre"])
        r = np.hypot(gu - c.dot(u_dir), gv - c.dot(v_dir))
        n = len(mark["rings"])
        for i, name in enumerate(mark["rings"]):
            col = over(col, coverage(0.5 * mark["diameter"] * (n - i) / n - r, density), name)
        return col, coverage(0.5 * mark["diameter"] - r, density)
    if mark["kind"] == "bands":
        hinge = next(h for h in data["hinges"] if h["node"] == mark["node"])
        if hinge["regions"][0]["surface"] != surface:
            return col, none
        # Distance aft of the hinge line, in the region's plane.
        pivot, axis = Vector(hinge["pivot"]), Vector(hinge["axis"])
        a = Vector((axis.dot(u_dir), axis.dot(v_dir))).normalized()
        aft = Vector((-u_dir.x, -v_dir.x))
        sign = 1.0 if a.x * aft.y - a.y * aft.x > 0 else -1.0
        def behind(u, v):
            return sign * (a.x * (v - pivot.dot(v_dir)) - a.y * (u - pivot.dot(u_dir)))
        reach = max(behind(p.dot(u_dir), p.dot(v_dir))
                    for p in (body_vector(v.co) for v in controls[mark["node"]].bm.verts))
        band = reach / len(mark["colours"])
        d = behind(gu, gv)
        for j, name in enumerate(mark["colours"]):
            col = over(col, coverage(d - j * band, density), name)
        return col, coverage(d, density)
    raise SystemExit(f"unknown mark {mark['kind']!r}")


def blender_image(name, pixels, colour_space):
    """A packed image of the .blend from an array of rows (bottom first) of RGB or RGBA."""
    size, channels = pixels.shape[0], pixels.shape[2]
    image = bpy.data.images.new(name, size, size, alpha=channels == 4)
    image.colorspace_settings.name = colour_space
    rgba = np.ones((size, size, 4), np.float32)
    rgba[..., :channels] = pixels
    image.pixels.foreach_set(rgba.ravel())
    image.file_format = "PNG"
    image.pack()
    return image


def texture_fabric(material, colour, normal, depth):
    """The fabric material takes its colour and its normals from the painted textures;
    its extras tell the game to let light through it (transmission in the colour's alpha,
    as KHR_materials_diffuse_transmission stores it), looked for `depth` behind it."""
    material["diffuse_transmission"] = 1.0
    material["transmission_depth_m"] = round(depth, 4)
    nodes, links = material.node_tree.nodes, material.node_tree.links
    bsdf = nodes["Principled BSDF"]
    base = nodes.new("ShaderNodeTexImage")
    base.image = colour
    links.new(base.outputs["Color"], bsdf.inputs["Base Color"])
    bumps = nodes.new("ShaderNodeTexImage")
    bumps.image = normal
    tangent = nodes.new("ShaderNodeNormalMap")
    links.new(bumps.outputs["Color"], tangent.inputs["Color"])
    links.new(tangent.outputs["Normal"], bsdf.inputs["Normal"])


# --- Materials ---------------------------------------------------------------------------

def linear(c):
    """sRGB component to linear: Blender's colour inputs are linear."""
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def make_materials(livery, colours):
    def principled(name, colour, roughness, metallic=0.0):
        m = bpy.data.materials.new(name)
        m.use_nodes = True
        bsdf = m.node_tree.nodes["Principled BSDF"]
        bsdf.inputs["Base Color"].default_value = (*(linear(c) for c in colour), 1.0)
        bsdf.inputs["Roughness"].default_value = roughness
        bsdf.inputs["Metallic"].default_value = metallic
        return m
    # The livery's fabric and painted metal (livrees.ron); the fabric of the wings, tail,
    # controls and fuselage is one material, its colour and normals painted in one atlas.
    fabric, metal = colours[livery["fabric"]], colours[livery["metal"]]
    m = {
        "surface": principled("toile vernie", fabric, 0.65),
        "fabric_doped": principled("toile vernie des roues", fabric, 0.65),
        "cowl": principled("capot, aluminium peint", metal, 0.45),
        "wood": principled("bois verni des mâts", (0.56, 0.40, 0.22), 0.4),
        "propeller_wood": principled("hélice, noyer verni", (0.36, 0.20, 0.10), 0.3),
        "metal": principled("acier", (0.35, 0.36, 0.37), 0.45, metallic=1.0),
        "steel": principled("acier bruni", (0.12, 0.12, 0.13), 0.4, metallic=1.0),
        "wire": principled("câbles d'acier", (0.50, 0.51, 0.53), 0.3, metallic=1.0),
        "rubber": principled("caoutchouc", (0.06, 0.06, 0.06), 0.85),
        "disc": principled("disque d'hélice", (0.6, 0.6, 0.6), 0.5),
        "radiator": principled("radiateur", (0.10, 0.09, 0.08), 0.8),
        "exhaust": principled("échappement", (0.27, 0.16, 0.10), 0.7, metallic=1.0),
        "leather": principled("cuir", (0.24, 0.13, 0.06), 0.55),
        "cockpit": principled("intérieur du poste", (0.10, 0.08, 0.06), 0.9),
        "glass": principled("verre", (0.02, 0.03, 0.03), 0.05),
        "hit": principled("volume d'impact, masqué", (0.8, 0.1, 0.1), 0.9),
    }
    m["control"] = m["fuselage"] = m["surface"]
    # The propeller's varnish, and the windscreen's glass.
    coat = m["propeller_wood"].node_tree.nodes["Principled BSDF"].inputs
    if "Coat Weight" in coat:
        coat["Coat Weight"].default_value = 1.0
    # The glass: alpha blending, which the game draws as it is. Bevy shows the glTF's
    # specular transmission dark and opaque without its screen-space transmission pass
    # (vol-essai-pilote, 2026-10-28). A dark base, so that the glass only reflects and
    # tints a little: a white one lit by the sun veiled the cowl and the gun behind it
    # (2026-10-29).
    glass = m["glass"].node_tree.nodes["Principled BSDF"].inputs
    glass["Alpha"].default_value = 0.10
    m["glass"].blend_method = "BLEND"
    return m


# --- Main --------------------------------------------------------------------------------

def main(argv):
    if bpy.app.version < MIN_BLENDER:
        raise SystemExit(f"Blender {'.'.join(map(str, MIN_BLENDER))} or later is needed")
    if not 2 <= len(argv) <= 3 or not argv[1].endswith(".glb") or argv[2:] and not argv[2].endswith(".blend"):
        raise SystemExit("usage: build.py -- <model.json> <model.glb> [<model.blend>]")
    json_path, glb_path = argv[0], argv[1]
    blend_path = argv[2] if len(argv) > 2 else None
    with open(json_path, encoding="utf-8") as f:
        data = json.load(f)
    if data["version"] != DATA_VERSION:
        raise SystemExit(f"model data version {data['version']}, expected {DATA_VERSION}")

    if not data.get("liveries"):
        raise SystemExit("the model is painted from livrees.ron, which this aircraft lacks")
    livery, colours = data["liveries"]["liveries"][0], data["liveries"]["colours"]

    bpy.ops.wm.read_factory_settings(use_empty=True)
    materials = make_materials(livery, colours)
    root = new_empty(data["id"])
    body = new_empty("body", root)

    # Control nodes: local +X along the hinge axis, local +Y towards the trailing edge's
    # side normal (any perpendicular will do; this one keeps the node upright).
    controls = {}
    control_objects = {}
    for hinge in data["hinges"]:
        axis = Vector(hinge["axis"])
        ref = Vector((1, 0, 0)) if abs(axis.z) > 0.9 else Vector((0, 0, -1))
        obj = new_object(hinge["node"], root, node_matrix(hinge["pivot"], axis, ref))
        control_objects[hinge["node"]] = obj
        controls[hinge["node"]] = Builder()

    global TIP_RADIUS, CUTOUT
    TIP_RADIUS = feature(data, "wing_tip_radius")[0][0]
    CUTOUT = sorted((h, f) for h, f in feature(data, "upper_wing_cutout"))
    global RIBS
    RIBS = tuple(feature(data, "ribs")[0])

    # Cuts of the damageable parts: the wings at their struts, the fuselage behind the
    # cockpit (docs/plans/m4-degats-modele-3d.md).
    upper = next(s for s in data["surfaces"] if s["name"] == "upper_wing")
    chord = upper["panels"][0]["chord_root"]
    le_x = upper["panels"][0]["root"][0] + 0.25 * chord
    spars = [le_x - f * chord for f in feature(data, "strut_spars")[0]]
    struts = [y for _, y in data["mockup"]["struts"]]
    SPLIT.update(inner=min(struts), outer=max(struts), spar_mid=0.5 * sum(spars),
                 fuselage=FUSELAGE_SPLIT_X)

    parts = Builder()
    keys = build_fuselage(data, parts)
    build_fuselage_frame(keys, parts)
    build_cockpit(data, parts, keys)
    build_nose(data, parts, keys)
    with part("exhaust_{lr}"):
        build_exhausts(data, parts, keys)
    outlines = data["mockup"].get("outlines", {})
    for surface in data["surfaces"]:
        KINDS[surface["name"]] = surface["kind"]
        with painted(surface["name"]), part("surface:" + surface["name"]):
            if surface["name"] in outlines:
                build_outline_surface(surface, outlines[surface["name"]], data, parts, controls)
            else:
                build_surface(surface, data, parts, controls)
    with part("strut"):
        build_struts(data, parts)
    with part("cabane"):
        build_cabane(data, parts, keys)
    build_gear(data, parts, keys)
    with part("gun_1"):
        build_gun(data, parts, keys)
    with part("linkage_{lr}"):
        build_linkage(data, parts)
    with part("windscreen"):
        build_windscreen(data, parts, keys)
    atlas = Atlas([parts, *controls.values()])
    colour, normal = paint_fabric(atlas, data, livery, colours, controls)
    texture_fabric(materials["surface"], blender_image("toile_couleur", colour, "sRGB"),
                   blender_image("toile_normales", normal, "Non-Color"), transmission_depth(data))
    parts.write_parts(body, materials, atlas)
    wires = {}
    diameters = build_wires(data, wires)
    for name, builder in sorted(wires.items()):
        obj = new_object(name, body)
        obj["diameter_m"] = diameters[name]
        # Smooth tubes: their normals point away from the axis, as the wire shader needs.
        builder.write(obj, materials, smooth=math.pi)
    for name, builder in controls.items():
        builder.write(control_objects[name], materials, atlas)

    # The blades and the hub, each its own node under `prop`, which the game turns: a
    # blade can break (M4 plan).
    prop, blades = build_propeller(data, root)
    blades.write_parts(prop, materials)
    p = data["propeller"]
    blur = new_object("prop_blur", root, node_matrix(p["hub"], p["axis"], (0, 0, -1)))
    disc = Builder()
    hub, axis = Vector(p["hub"]), Vector(p["axis"])
    u = Vector((0, 0, -1))
    v = axis.cross(u)
    n = 48
    rim = [hub + axis * 0.01 + (u * math.cos(2 * math.pi * i / n) + v * math.sin(2 * math.pi * i / n)) * p["radius"]
           for i in range(n)]
    disc.face(rim, "disc")
    disc.write(blur, materials)

    # Damage (M4 plan): hit volumes, effect points, damaged variants.
    build_hits(data, keys, body, materials)
    build_effect_points(data, keys, body)
    build_damaged(data, keys, body, prop, materials)

    # The pilot's eye and the gun muzzles look forward (glTF -Z), upright (glTF +Y).
    mock = data["mockup"]
    # glTF local X = Y × Z = up × backward = the right wing (+y).
    new_empty("cam_pilot_eye", root, node_matrix(mock["pilot_eye"], (0, 1, 0), (0, 0, -1)))
    for i, muzzle in enumerate(mock["gun_muzzles"], start=1):
        new_empty(f"gun_{i}_muzzle", root, node_matrix(muzzle, (0, 1, 0), (0, 0, -1)))

    if blend_path:
        bpy.ops.wm.save_as_mainfile(filepath=blend_path)
    bpy.ops.export_scene.gltf(
        filepath=glb_path,
        export_format="GLB",
        export_yup=True,
        export_apply=True,
        export_copyright=COPYRIGHT,
        export_extras=True,
        export_cameras=False,
        export_lights=False,
        export_animations=False,
    )


if __name__ == "__main__":
    main(sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else [])
