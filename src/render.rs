//! Output layer: color, terminal fitting and the timing readout.

use crate::fit;
use crate::pipeline::{self, Source};
use colored::Colorize;
use figlet_rs::FIGfont;
use std::time::Duration;

/// Render object art (`runix <word> -a`).
pub fn object(word: &str, color: &str) {
    match pipeline::object(word) {
        Ok(spell) => {
            println!("{}", paint(&fit::shrink_to_terminal(&spell.art), color));
            report(spell.elapsed, spell.source);
        }
        Err(err) => {
            eprintln!("{}", "💀 the spell fizzled — please try again".red());
            eprintln!("{}", format!("   {err:#}").red());
        }
    }
}

/// Render big letter art (`runix <word>`).
/// Words wider than the screen are stacked in chunks that always fit.
pub fn word(text: &str) {
    let font = FIGfont::standard().expect("built-in standard font always loads");
    let figure = match font.convert(text) {
        Some(figure) => figure,
        None => {
            eprintln!("{}", format!("could not render: {text}").red());
            return;
        }
    };

    let budget = fit::terminal_width().saturating_sub(1);
    let art = figure.to_string();
    if fit::max_line_width(&art) <= budget {
        print_figlet(&art);
        return;
    }

    // Too wide for the screen: stack the word in chunks that fit.
    let mut chunk = String::new();
    for ch in text.chars() {
        let candidate = format!("{chunk}{ch}");
        match font.convert(&candidate) {
            Some(fig) if fit::max_line_width(&fig.to_string()) <= budget => chunk = candidate,
            _ => {
                if !chunk.is_empty() {
                    if let Some(fig) = font.convert(&chunk) {
                        print_figlet(&fig.to_string());
                    }
                    chunk.clear();
                }
                // A single glyph wider than the screen: print it anyway.
                if ch != ' ' {
                    if let Some(fig) = font.convert(&ch.to_string()) {
                        print_figlet(&fig.to_string());
                    }
                }
            }
        }
    }
    if !chunk.is_empty() {
        if let Some(fig) = font.convert(&chunk) {
            print_figlet(&fig.to_string());
        }
    }
}

fn print_figlet(art: &str) {
    for line in art.lines() {
        println!("{}", line.trim_end().white().bold());
    }
}

fn paint(art: &str, color: &str) -> String {
    match color.to_lowercase().as_str() {
        "red" => art.red().to_string(),
        "blue" => art.blue().to_string(),
        "cyan" => art.cyan().to_string(),
        "yellow" => art.yellow().to_string(),
        "magenta" | "purple" => art.magenta().to_string(),
        "white" => art.white().to_string(),
        _ => art.green().to_string(),
    }
}

/// `⚡ 1.24s · ai` — watch the response time shrink with every cache hit.
fn report(elapsed: Duration, source: Source) {
    println!(
        "{}",
        format!("⚡ {} · {}", fmt_duration(elapsed), source.label()).dimmed()
    );
}

fn fmt_duration(d: Duration) -> String {
    let ms = d.as_secs_f64() * 1000.0;
    if ms >= 1000.0 {
        format!("{:.2}s", ms / 1000.0)
    } else if ms >= 10.0 {
        format!("{ms:.0}ms")
    } else if ms >= 1.0 {
        format!("{ms:.1}ms")
    } else {
        format!("{}µs", d.as_micros())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_durations_for_humans() {
        assert_eq!(fmt_duration(Duration::from_micros(120)), "120µs");
        assert_eq!(fmt_duration(Duration::from_millis(1500)), "1.50s");
    }
}
