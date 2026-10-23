//! Terminal-aware sizing: every spell is shrunk to fit the screen.

use terminal_size::{terminal_size, Height, Width};

const DEFAULT_WIDTH: usize = 80;
const DEFAULT_HEIGHT: usize = 24;

/// Usable columns on stdout (falls back to 80 when not a TTY).
pub fn terminal_width() -> usize {
    terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(DEFAULT_WIDTH)
}

/// Usable rows on stdout (falls back to 24 when not a TTY).
pub fn terminal_height() -> usize {
    terminal_size()
        .map(|(_, Height(h))| h as usize)
        .unwrap_or(DEFAULT_HEIGHT)
}

/// Widest line in a text block, in characters.
pub fn max_line_width(text: &str) -> usize {
    text.lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
}

/// Width budget we hand the AI so its art lands on screen.
pub fn art_width_budget() -> usize {
    terminal_width().saturating_sub(2).clamp(30, 60)
}

/// Height budget we hand the AI so its art lands on screen.
pub fn art_height_budget() -> usize {
    terminal_height().saturating_sub(5).clamp(6, 14)
}

/// Right-trims every line and, when the block is wider than the terminal,
/// drops alternating columns until it fits — the silhouette survives.
pub fn shrink_to_terminal(art: &str) -> String {
    let budget = terminal_width().saturating_sub(1).max(4);

    let mut lines: Vec<String> = art.lines().map(|line| line.trim_end().to_string()).collect();
    while max_line_width(&lines.join("\n")) > budget {
        for line in &mut lines {
            *line = line.chars().step_by(2).collect();
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_narrow_art_untouched() {
        let art = "/\\_/\n(oo  )\n =^= ";
        assert_eq!(shrink_to_terminal(art), "/\\_/\n(oo  )\n =^=");
    }

    #[test]
    fn halves_wide_art_until_it_fits() {
        let art = "x".repeat(300);
        let out = shrink_to_terminal(&art);
        assert!(max_line_width(&out) <= DEFAULT_WIDTH);
        assert!(!out.is_empty());
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(shrink_to_terminal(""), "");
    }
}
