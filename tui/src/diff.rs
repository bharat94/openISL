use ratatui::prelude::{Color, Line, Modifier, Span, Style};

#[derive(Debug, Clone)]
pub struct ColoredDiffLine {
    pub content: String,
    pub line_type: DiffLineType,
    pub line_number: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiffLineType {
    Addition,
    Deletion,
    Context,
    Header,
    Meta,
    HunkHeader,
}

pub struct DiffParser;

impl DiffParser {
    pub fn parse(diff_content: &str) -> Vec<ColoredDiffLine> {
        let mut lines = Vec::new();
        let mut line_number = 1;

        for line in diff_content.lines() {
            let _trimmed = line.trim_end();
            let line_type = if line.is_empty() {
                DiffLineType::Context
            } else if line.starts_with("diff --git") {
                DiffLineType::Header
            } else if line.starts_with("index ") {
                DiffLineType::Meta
            } else if line.starts_with("+++") || line.starts_with("---") {
                DiffLineType::Header
            } else if line.starts_with("@@") {
                DiffLineType::HunkHeader
            } else if line.starts_with('+') && !line.starts_with("+++") {
                DiffLineType::Addition
            } else if line.starts_with('-') && !line.starts_with("---") {
                DiffLineType::Deletion
            } else {
                DiffLineType::Context
            };

            lines.push(ColoredDiffLine {
                content: line.to_string(),
                line_type: line_type.clone(),
                line_number: Some(line_number),
            });

            if line_type != DiffLineType::Deletion {
                line_number += 1;
            }
        }

        lines
    }

    pub fn to_styled_lines(lines: &[ColoredDiffLine], dark_theme: bool) -> Vec<Line<'static>> {
        let addition_fg = if dark_theme {
            Color::Rgb(0, 255, 127)
        } else {
            Color::Green
        };
        let deletion_fg = if dark_theme {
            Color::Rgb(255, 69, 0)
        } else {
            Color::Red
        };
        let header_fg = if dark_theme {
            Color::Rgb(255, 215, 0)
        } else {
            Color::Rgb(180, 140, 0)
        };
        let meta_fg = if dark_theme {
            Color::Rgb(136, 192, 208)
        } else {
            Color::Blue
        };
        let hunk_header_fg = if dark_theme {
            Color::Rgb(189, 147, 249)
        } else {
            Color::Magenta
        };
        let context_fg = if dark_theme {
            Color::Rgb(200, 200, 200)
        } else {
            Color::DarkGray
        };

        lines
            .iter()
            .map(|line| {
                let style = match line.line_type {
                    DiffLineType::Addition => Style::default()
                        .fg(addition_fg)
                        .add_modifier(Modifier::BOLD),
                    DiffLineType::Deletion => {
                        Style::default().fg(deletion_fg).add_modifier(Modifier::DIM)
                    }
                    DiffLineType::Header => {
                        Style::default().fg(header_fg).add_modifier(Modifier::BOLD)
                    }
                    DiffLineType::Meta => Style::default().fg(meta_fg),
                    DiffLineType::HunkHeader => Style::default()
                        .fg(hunk_header_fg)
                        .add_modifier(Modifier::BOLD),
                    DiffLineType::Context => Style::default().fg(context_fg),
                };

                Line::styled(line.content.clone(), style)
            })
            .collect()
    }

    pub fn to_styled_lines_with_numbers(
        lines: &[ColoredDiffLine],
        dark_theme: bool,
    ) -> Vec<Line<'static>> {
        let addition_fg = if dark_theme {
            Color::Rgb(0, 255, 127)
        } else {
            Color::Green
        };
        let deletion_fg = if dark_theme {
            Color::Rgb(255, 69, 0)
        } else {
            Color::Red
        };
        let header_fg = if dark_theme {
            Color::Rgb(255, 215, 0)
        } else {
            Color::Rgb(180, 140, 0)
        };
        let meta_fg = if dark_theme {
            Color::Rgb(136, 192, 208)
        } else {
            Color::Blue
        };
        let hunk_header_fg = if dark_theme {
            Color::Rgb(189, 147, 249)
        } else {
            Color::Magenta
        };
        let context_fg = if dark_theme {
            Color::Rgb(200, 200, 200)
        } else {
            Color::DarkGray
        };
        let number_fg = if dark_theme {
            Color::Rgb(100, 100, 100)
        } else {
            Color::Gray
        };

        lines
            .iter()
            .map(|line| {
                let style = match line.line_type {
                    DiffLineType::Addition => Style::default()
                        .fg(addition_fg)
                        .add_modifier(Modifier::BOLD),
                    DiffLineType::Deletion => {
                        Style::default().fg(deletion_fg).add_modifier(Modifier::DIM)
                    }
                    DiffLineType::Header => {
                        Style::default().fg(header_fg).add_modifier(Modifier::BOLD)
                    }
                    DiffLineType::Meta => Style::default().fg(meta_fg),
                    DiffLineType::HunkHeader => Style::default()
                        .fg(hunk_header_fg)
                        .add_modifier(Modifier::BOLD),
                    DiffLineType::Context => Style::default().fg(context_fg),
                };

                let line_prefix = match line.line_number {
                    Some(n) => format!("{:>4} ", n),
                    None => "      ".to_string(),
                };

                let number_style = Style::default().fg(number_fg);
                Line::from(vec![
                    Span::styled(line_prefix, number_style),
                    Span::styled(line.content.clone(), style),
                ])
            })
            .collect()
    }

    pub fn count_stats(lines: &[ColoredDiffLine]) -> DiffStats {
        let mut additions = 0;
        let mut deletions = 0;
        let mut files_changed = 0;
        let mut current_file = String::new();

        for line in lines {
            match line.line_type {
                DiffLineType::Addition => additions += 1,
                DiffLineType::Deletion => deletions += 1,
                DiffLineType::Header => {
                    if line.content.starts_with("+++") {
                        files_changed += 1;
                        if let Some(path) = line.content.strip_prefix("+++ b/") {
                            current_file = path.to_string();
                        }
                    }
                }
                _ => {}
            }
        }

        DiffStats {
            additions,
            deletions,
            files_changed: files_changed.max(1),
            net_change: additions.saturating_sub(deletions),
            current_file,
        }
    }
}

#[derive(Debug, Default)]
pub struct DiffStats {
    pub additions: usize,
    pub deletions: usize,
    pub files_changed: usize,
    pub net_change: usize,
    pub current_file: String,
}

impl DiffStats {
    pub fn format_summary(&self) -> String {
        if self.files_changed > 1 {
            format!(
                "{} file(s) changed, {} insertion(+), {} deletion(-)",
                self.files_changed, self.additions, self.deletions
            )
        } else {
            format!(
                "{} insertion(+), {} deletion(-)",
                self.additions, self.deletions
            )
        }
    }

    pub fn format_file_info(&self) -> String {
        if !self.current_file.is_empty() {
            format!("Viewing: {}", self.current_file)
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_diff_additions() {
        let diff = "diff --git a/src/main.rs b/src/main.rs\n--- a/src/main.rs\n+++ b/src/main.rs\n@@ -1,3 +1,4 @@\n fn main() {\n+    println!(\"hello\");\n     println!(\"world\");\n }";
        let lines = DiffParser::parse(diff);

        assert!(lines.iter().any(|l| l.line_type == DiffLineType::Header));
        assert!(lines
            .iter()
            .any(|l| l.line_type == DiffLineType::HunkHeader));
        assert!(lines.iter().any(|l| l.line_type == DiffLineType::Addition));
        assert!(lines.iter().any(|l| l.line_type == DiffLineType::Context));
    }

    #[test]
    fn test_parse_diff_deletions() {
        let diff = "fn old_function() {\n-    println!(\"removed\");\n }";
        let lines = DiffParser::parse(diff);

        let additions: usize = lines
            .iter()
            .filter(|l| l.line_type == DiffLineType::Addition)
            .count();
        let deletions: usize = lines
            .iter()
            .filter(|l| l.line_type == DiffLineType::Deletion)
            .count();

        assert_eq!(additions, 0);
        assert_eq!(deletions, 1);
    }

    #[test]
    fn test_diff_stats() {
        let diff = "diff --git a/src/main.rs b/src/main.rs\n@@ -1,3 +1,4 @@\n fn main() {\n+    println!(\"hello\");\n+    println!(\"added\");\n-    println!(\"removed\");\n }";
        let lines = DiffParser::parse(diff);
        let stats = DiffParser::count_stats(&lines);

        assert_eq!(stats.additions, 2);
        assert_eq!(stats.deletions, 1);
        assert_eq!(stats.files_changed, 1);
    }

    #[test]
    fn test_to_styled_lines_dark_theme() {
        let lines = vec![
            ColoredDiffLine {
                content: "added line".to_string(),
                line_type: DiffLineType::Addition,
                line_number: Some(1),
            },
            ColoredDiffLine {
                content: "deleted line".to_string(),
                line_type: DiffLineType::Deletion,
                line_number: Some(2),
            },
        ];

        let styled = DiffParser::to_styled_lines(&lines, true);

        assert_eq!(styled.len(), 2);
    }

    #[test]
    fn test_diff_stats_format() {
        let stats = DiffStats {
            additions: 10,
            deletions: 5,
            files_changed: 2,
            net_change: 5,
            current_file: "src/main.rs".to_string(),
        };

        let formatted = stats.format_summary();
        assert!(formatted.contains("2 file(s)"));
        assert!(formatted.contains("10 insertion"));
        assert!(formatted.contains("5 deletion"));
    }

    #[test]
    fn test_line_numbers_present() {
        let diff = "diff --git a/src/main.rs b/src/main.rs\n@@ -1,3 +1,4 @@\n fn main() {\n+    println!(\"hello\");\n     println!(\"world\");\n }";
        let lines = DiffParser::parse(diff);

        let all_have_numbers = lines.iter().all(|l| l.line_number.is_some());
        assert!(all_have_numbers);
    }

    #[test]
    fn test_styled_lines_with_numbers() {
        let lines = vec![
            ColoredDiffLine {
                content: "context".to_string(),
                line_type: DiffLineType::Context,
                line_number: Some(1),
            },
            ColoredDiffLine {
                content: "+ addition".to_string(),
                line_type: DiffLineType::Addition,
                line_number: Some(2),
            },
        ];

        let styled = DiffParser::to_styled_lines_with_numbers(&lines, true);
        assert_eq!(styled.len(), 2);
    }
}
