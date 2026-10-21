//! Origin of every number in a data file (`format-avion.md`, skill
//! `recherche-historique`): each line that contains a numeric value must end with an
//! origin comment, `// S:<source>`, `// C:<method>` or `// E:<justification>`.
//!
//! A comment line starting with `//! S:`, `//! C:` or `//! E:` gives the origin of a whole
//! block (a table, for instance): it applies to the following lines up to the next blank
//! line. Numbers inside strings and identifiers (`"S1"`, `spad7`) do not count.

/// A line holding a number without an origin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingOrigin {
    /// Line number, from 1.
    pub line: usize,
    pub text: String,
}

const TAGS: [&str; 3] = ["S:", "C:", "E:"];

/// Lines of `text` holding a number without an origin comment.
pub fn missing_origins(text: &str) -> Vec<MissingOrigin> {
    let mut missing = Vec::new();
    let mut block = false;
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            block = false;
            continue;
        }
        let (code, comment) = split_comment(line);
        let comment = comment.map(str::trim_start);
        let block_comment = comment
            .and_then(|c| c.strip_prefix('!'))
            .is_some_and(|c| has_tag(c.trim_start()));
        if block_comment {
            block = true;
        }
        let tagged = block || comment.is_some_and(has_tag);
        if has_number(&code) && !tagged {
            missing.push(MissingOrigin {
                line: index + 1,
                text: line.trim().to_owned(),
            });
        }
    }
    missing
}

fn has_tag(comment: &str) -> bool {
    TAGS.iter().any(|tag| comment.starts_with(tag))
}

/// Code with its string literals blanked out, and the comment after `//`, if any.
fn split_comment(line: &str) -> (String, Option<&str>) {
    let mut code = String::with_capacity(line.len());
    let mut in_string = false;
    let mut escaped = false;
    for (i, c) in line.char_indices() {
        if in_string {
            match c {
                _ if escaped => escaped = false,
                '\\' => escaped = true,
                '"' => in_string = false,
                _ => {}
            }
            code.push(' ');
        } else if c == '"' {
            in_string = true;
            code.push(' ');
        } else if line[i..].starts_with("//") {
            return (code, Some(&line[i + 2..]));
        } else {
            code.push(c);
        }
    }
    (code, None)
}

/// A digit that does not belong to an identifier.
fn has_number(code: &str) -> bool {
    let mut previous = ' ';
    for c in code.chars() {
        if c.is_ascii_digit() && !(previous.is_alphanumeric() || previous == '_') {
            return true;
        }
        previous = c;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_number_needs_an_origin() {
        let text = r#"Aircraft(
    id: "spad7",                       // identifiers and strings are not numbers
    span: M(7.82),                     // S:S3 p. 12
    wing_area: M2(17.85),
    strips: 8,                         // E: discretisation, see the convergence test
    cg: (0.1, 0.0, -0.2),              // C: from the weights table
    note: "1917 // not a comment",
    bad: 3.0, // no tag here
)"#;
        let missing = missing_origins(text);
        let lines: Vec<usize> = missing.iter().map(|m| m.line).collect();
        assert_eq!(lines, vec![4, 8], "{missing:?}");
        assert_eq!(missing[0].text, "wing_area: M2(17.85),");
    }

    #[test]
    fn block_origin_covers_a_table_up_to_the_next_blank_line() {
        let text = "points: [\n    //! S:S2 NACA R233 table XXX\n    (alpha: Deg(-1.5), cl: -0.04),\n    (alpha: Deg(0.0), cl: 0.1),\n\n    (alpha: Deg(2.0), cl: 0.3),\n]";
        let missing = missing_origins(text);
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].line, 6);
    }

    #[test]
    fn strings_with_quotes_and_identifiers_with_digits_are_ignored() {
        let text = "a: \"say \\\"12\\\"\", b: raf15, c_2: x // no number here\n";
        assert!(missing_origins(text).is_empty());
    }
}
