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

Pass 1 (shape): numbers marked E: are display estimates, to be replaced by the drawing's
digitised outlines and the appearance sheet (docs/recherche/spad7-aspect.md). The tail
already follows the drawn outlines of maquette.ron (pass 2).
"""

import json
import math
import sys

import bmesh
import bpy
from mathutils import Matrix, Vector

MIN_BLENDER = (4, 0, 0)
DATA_VERSION = 1
COPYRIGHT = "Panache blanc, CC-BY-SA 4.0 (LICENSE-ASSETS)"

# Wing section "Spad", NACA Report 93, ref. 49 (MIT, 1918): ordinates in % of chord above
# the reference line (docs/recherche/profil-spad7.md § 2.2; S: confidence B for the S.VII).
SPAD_X = [0, 1.25, 2.5, 5, 7.5, 10, 15, 20, 30, 40, 50, 60, 70, 80, 90, 95, 100]
SPAD_UPPER = [0.26, 1.55, 2.14, 3.00, 3.75, 4.37, 5.31, 5.95, 6.60, 6.67, 6.28, 5.53,
              4.51, 3.26, 1.80, 1.06, 0.32]
SPAD_LOWER = [0.26, 0.07, 0.03, 0.08, 0.16, 0.26, 0.55, 0.85, 1.25, 1.31, 1.10, 0.82,
              0.51, 0.29, 0.15, 0.07, 0.0]

TAIL_THICKNESS = 0.05      # E: thin symmetric tail sections (spad_empennage.ron: RAF 15 uncambered)
TIP_RADIUS = 0.10          # E: « corners slightly rounded off » (S48)
CHORD_POINTS = 28          # points per surface of a section
STRUT_CHORD, STRUT_THICK = 0.10, 0.035     # E: streamlined interplane struts
STRUT_SPARS = (0.17, 0.49)                 # C: front and rear struts at 17 and 49 % of the chord (drawing, spad7-calculs.md § 3)
CABANE_X = (-0.10, -0.70)                  # E: cabane struts, front and rear
CABANE_Y = 0.30                            # E:
LEG_SPREAD, LEG_ROOT_Y, LEG_DIAMETER = 0.35, 0.30, 0.05   # E: undercarriage V legs
PITCH_RATIO = 0.81                         # C: Éclair type 6, 2.40 × 1.95 m (aircraft.ron)
GUN_JACKET = (0.11, 1.05)                  # E: Vickers cooling jacket diameter and length, m


# --- Frames (ADR 0006) -------------------------------------------------------------------

def blender_point(p):
    """Body axes to Blender's: (x, y, z) -> (-y, -x, -z)."""
    return Vector((-p[1], -p[0], -p[2]))


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


class Builder:
    """Accumulates faces in body axes, then writes them into an object's mesh, in the
    object's local axes."""

    def __init__(self):
        self.bm = bmesh.new()
        self.material_index = {}

    def face(self, points, material):
        verts = [self.bm.verts.new(blender_point(p)) for p in points]
        f = self.bm.faces.new(verts)
        f.material_index = self.material(material)
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
        for cap, row in ((cap_start, rows[0]), (cap_end, rows[-1])):
            if cap:
                f = self.bm.faces.new(row)
                f.material_index = index

    def write(self, obj, materials):
        bmesh.ops.remove_doubles(self.bm, verts=self.bm.verts, dist=1e-6)
        bmesh.ops.recalc_face_normals(self.bm, faces=self.bm.faces)
        self.bm.transform(obj.matrix_world.inverted())
        self.bm.to_mesh(obj.data)
        self.bm.free()
        for name, _ in sorted(self.material_index.items(), key=lambda kv: kv[1]):
            obj.data.materials.append(materials[name])
        for poly in obj.data.polygons:
            poly.use_smooth = True
        smooth_by_angle(obj.data, math.radians(35))


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
            etas = sorted(e for e in etas if 0.0 <= e <= 1.0)
            stations = []
            for e in etas:
                inset = tip_insets((1.0 - e) * length) if free_tip else (0.0, 0.0)
                stations.append((e, Station(panel, e, side if side != 0.0 else 1.0, kind, inset)))
            # Loft along the span: whole sections, or fixed part and control part.
            def hinge_of(k, stations=stations, regions=regions):
                mid = 0.5 * (stations[k][0] + stations[k + 1][0])
                region = next((r for r in regions if r[0] <= mid <= r[1]), None)
                return region[2] if region else None
            loft_runs([st for _, st in stations], hinge_of, section, body_parts, controls)
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
    for side in sides:
        sign = side if side != 0.0 else 1.0
        stations = [(r[0] * (sign if kind == "horizontal" else 1.0),
                     OutlineStation(kind, r[0], r[1], r[2], sign, plane_z, first["incidence"]))
                    for r in rows]
        hinges = [region_at(surface, data, side, 0.5 * (c0 + c1))
                  for (c0, _), (c1, _) in zip(stations, stations[1:])]
        loft_runs([st for _, st in stations], lambda k: hinges[k], symmetric_section,
                  body_parts, controls)
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


def build_fuselage(data, parts):
    sections = data["mockup"]["fuselage"]
    keys = [(s["x"], s["width"] / 2, s["top"], s["bottom"]) for s in sections]
    n = 48
    rings = []
    for i in range(n + 1):
        t = (len(keys) - 1) * i / n
        x = catmull_rom([k[0] for k in keys], t)
        hw = max(catmull_rom([k[1] for k in keys], t), 0.01)
        top = catmull_rom([k[2] for k in keys], t)
        bottom = catmull_rom([k[3] for k in keys], t)
        centre, half_h = 0.5 * (top + bottom), max(0.5 * (bottom - top), 0.01)
        exponent = 2.0 + 1.6 * min(i / (0.25 * n), 1.0)   # E: round cowling, rounded box aft
        rings.append([Vector((x, y, z)) for y, z in superellipse(centre, hw, half_h, exponent)])
    parts.loft(rings, "fuselage", cap_start=True, cap_end=True)
    return keys


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
            for f in STRUT_SPARS:
                x = le_x - f * chord
                bottom = wing_surface_z(data, "lower_wing", x, y, upper=True)
                top = wing_surface_z(data, "upper_wing", x, y, upper=False)
                streamlined_strut(parts, (x, side * y, bottom), (x, side * y, top),
                                  STRUT_CHORD, STRUT_THICK, "wood")


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
    for x in CABANE_X:
        top, _, _ = fuselage_at(keys, x)
        z_wing = wing_surface_z(data, "upper_wing", x, CABANE_Y, upper=False)
        for side in (1, -1):
            streamlined_strut(parts, (x, side * 0.8 * CABANE_Y, top + 0.02),
                              (x, side * CABANE_Y, z_wing), 0.06, 0.025, "metal")


def build_gear(data, parts, keys):
    mock = data["mockup"]
    cx, cy, cz = mock["wheels"]["centre"]
    radius, width = 0.5 * mock["wheels"]["diameter"], mock["wheels"]["width"]
    for side in (1, -1):
        centre = Vector((cx, side * cy, cz))
        # Tyre: a torus; wheel cover: two shallow cones.
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
        # V legs from the axle to the fuselage bottom, ahead and behind.
        axle = Vector((cx, side * (cy - width), cz))
        for dx in (LEG_SPREAD, -LEG_SPREAD):
            x = cx + dx
            _, bottom, _ = fuselage_at(keys, x)
            round_bar(parts, axle, (x, side * LEG_ROOT_Y, bottom - 0.02), LEG_DIAMETER, "wood")
    # Axle and its fairing between the wheels.
    streamlined_strut(parts, (cx, cy - width, cz), (cx, -(cy - width), cz), 0.12, 0.05, "fabric_doped")
    # Tail skid, from the fuselage bottom down and back to its tip.
    sx, sy, sz = mock["skid"]
    _, bottom, _ = fuselage_at(keys, sx + 0.35)
    round_bar(parts, (sx + 0.35, 0, bottom), (sx, sy, sz), 0.04, "metal")


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
        parts.loft(rings, "propeller_wood", cap_start=True, cap_end=True)
    # Hub boss.
    round_bar(parts, hub - axis * 0.12, hub + axis * 0.06, 0.20, "propeller_wood")
    return node, parts


def build_gun(data, parts):
    muzzle = Vector(data["mockup"]["gun_muzzles"][0])
    d, length = GUN_JACKET
    round_bar(parts, muzzle - Vector((length, 0, 0)), muzzle, d, "steel")


# --- Materials ---------------------------------------------------------------------------

def linear(c):
    """sRGB component to linear: Blender's colour inputs are linear."""
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def make_materials():
    def principled(name, colour, roughness, metallic=0.0):
        m = bpy.data.materials.new(name)
        m.use_nodes = True
        bsdf = m.node_tree.nodes["Principled BSDF"]
        bsdf.inputs["Base Color"].default_value = (*(linear(c) for c in colour), 1.0)
        bsdf.inputs["Roughness"].default_value = roughness
        bsdf.inputs["Metallic"].default_value = metallic
        return m
    # E: pass-1 colours, until the livery of the appearance sheet.
    return {
        "surface": principled("toile peinte", (0.38, 0.33, 0.22), 0.7),
        "control": principled("gouvernes", (0.38, 0.33, 0.22), 0.7),
        "fuselage": principled("fuselage", (0.36, 0.32, 0.22), 0.6),
        "fabric_doped": principled("toile enduite", (0.78, 0.72, 0.58), 0.7),
        "wood": principled("bois peint", (0.30, 0.25, 0.18), 0.6),
        "propeller_wood": principled("hélice", (0.30, 0.16, 0.07), 0.35),
        "metal": principled("acier", (0.25, 0.25, 0.26), 0.45, metallic=1.0),
        "steel": principled("acier bruni", (0.10, 0.10, 0.11), 0.4, metallic=1.0),
        "rubber": principled("caoutchouc", (0.05, 0.05, 0.05), 0.85),
        "disc": principled("disque d'hélice", (0.6, 0.6, 0.6), 0.5),
    }


# --- Main --------------------------------------------------------------------------------

def main(argv):
    if bpy.app.version < MIN_BLENDER:
        raise SystemExit(f"Blender {'.'.join(map(str, MIN_BLENDER))} or later is needed")
    json_path, glb_path = argv[0], argv[1]
    blend_path = argv[2] if len(argv) > 2 else None
    with open(json_path, encoding="utf-8") as f:
        data = json.load(f)
    if data["version"] != DATA_VERSION:
        raise SystemExit(f"model data version {data['version']}, expected {DATA_VERSION}")

    bpy.ops.wm.read_factory_settings(use_empty=True)
    materials = make_materials()
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

    parts = Builder()
    keys = build_fuselage(data, parts)
    outlines = data["mockup"].get("outlines", {})
    for surface in data["surfaces"]:
        if surface["name"] in outlines:
            build_outline_surface(surface, outlines[surface["name"]], data, parts, controls)
        else:
            build_surface(surface, data, parts, controls)
    build_struts(data, parts)
    build_cabane(data, parts, keys)
    build_gear(data, parts, keys)
    build_gun(data, parts)
    airframe = new_object("airframe", body)
    parts.write(airframe, materials)
    for name, builder in controls.items():
        builder.write(control_objects[name], materials)

    prop, blades = build_propeller(data, root)
    blades.write(prop, materials)
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
        export_cameras=False,
        export_lights=False,
        export_animations=False,
    )


if __name__ == "__main__":
    main(sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else [])
