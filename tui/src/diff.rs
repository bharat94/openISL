use ratatui::prelude::{Color, Line, Modifier, Style};

#[derive(Debug, Clone)]
pub struct ColoredDiffLine {
    pub content: String,
    pub line_type: DiffLineType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiffLineType {
    Addition,
    Deletion,
    Context,
    Header,
    Meta,
}

pub struct DiffParser;

impl DiffParser {
    pub fn parse(diff_content: &str) -> Vec<ColoredDiffLine> {
        let mut lines = Vec::new();

        for line in diff_content.lines() {
            let trimmed = line.trim_end();

            if trimmed.is_empty() {
                lines.push(ColoredDiffLine {
                    content: line.to_string(),
                    line_type: DiffLineType::Context,
                });
            } else if line.starts_with("+++") || line.starts_with("---") {
                lines.push(ColoredDiffLine {
                    content: line.to_string(),
                    line_type: DiffLineType::Header,
                });
            } else if line.starts_with("@@") {
                lines.push(ColoredDiffLine {
                    content: line.to_string(),
                    line_type: DiffLineType::Meta,
                });
            } else if line.starts_with('+') && !line.starts_with("+++") {
                lines.push(ColoredDiffLine {
                    content: line[1..].to_string(),
                    line_type: DiffLineType::Addition,
                });
            } else if line.starts_with('-') && !line.starts_with("---") {
                lines.push(ColoredDiffLine {
                    content: line[1..].to_string(),
                    line_type: DiffLineType::Deletion,
                });
            } else {
                lines.push(ColoredDiffLine {
                    content: line.to_string(),
                    line_type: DiffLineType::Context,
                });
            }
        }

        lines
    }

    pub fn to_styled_lines(lines: &[ColoredDiffLine], dark_theme: bool) -> Vec<Line<'static>> {
        let addition_fg = if dark_theme {
            Color::Green
        } else {
            Color::LightGreen
        };
        let deletion_fg = if dark_theme {
            Color::Red
        } else {
            Color::LightRed
        };
        let header_fg = if dark_theme { Color::Cyan } else { Color::Blue };
        let meta_fg = Color::Yellow;
        let context_fg = if dark_theme {
            Color::Gray
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
                    DiffLineType::Deletion => Style::default().fg(deletion_fg),
                    DiffLineType::Header => Style::default().fg(header_fg),
                    DiffLineType::Meta => Style::default().fg(meta_fg),
                    DiffLineType::Context => Style::default().fg(context_fg),
                };

                Line::styled(line.content.clone(), style)
            })
            .collect()
    }

    pub fn count_stats(lines: &[ColoredDiffLine]) -> DiffStats {
        let mut additions = 0;
        let mut deletions = 0;
        let mut files_changed = 1;

        for line in lines {
            match line.line_type {
                DiffLineType::Addition => additions += 1,
                DiffLineType::Deletion => deletions += 1,
                DiffLineType::Header => {
                    if line.content.starts_with("+++") || line.content.starts_with("---") {
                        files_changed = 1;
                    }
                }
                _ => {}
            }
        }

        DiffStats {
            additions,
            deletions,
            files_changed,
            net_change: additions.saturating_sub(deletions),
        }
    }
}

#[derive(Debug, Default)]
pub struct DiffStats {
    pub additions: usize,
    pub deletions: usize,
    pub files_changed: usize,
    pub net_change: usize,
}

impl DiffStats {
    pub fn format_summary(&self) -> String {
        format!(
            "{} file(s) changed, {} insertion(+), {} deletion(-)",
            self.files_changed, self.additions, self.deletions
        )
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
        assert!(lines.iter().any(|l| l.line_type == DiffLineType::Meta));
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
    }

    #[test]
    fn test_to_styled_lines_dark_theme() {
        let lines = vec![
            ColoredDiffLine {
                content: "added line".to_string(),
                line_type: DiffLineType::Addition,
            },
            ColoredDiffLine {
                content: "deleted line".to_string(),
                line_type: DiffLineType::Deletion,
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
        };

        let formatted = stats.format_summary();
        assert!(formatted.contains("2 file(s)"));
        assert!(formatted.contains("10 insertion"));
        assert!(formatted.contains("5 deletion"));
    }
}
