use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use std::collections::HashMap;
use std::path::Path;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyntaxHighlight {
    Keyword,
    String,
    Comment,
    Function,
    Number,
    Type,
    Attribute,
    None,
}

struct SyntaxColors {
    keyword: Color,
    string: Color,
    comment: Color,
    function: Color,
    number: Color,
    type_color: Color,
    attribute: Color,
}

fn get_syntax_colors(dark_theme: bool) -> SyntaxColors {
    if dark_theme {
        SyntaxColors {
            keyword: Color::Rgb(189, 147, 249),
            string: Color::Rgb(166, 227, 161),
            comment: Color::Rgb(129, 161, 193),
            function: Color::Rgb(136, 192, 208),
            number: Color::Rgb(243, 139, 168),
            type_color: Color::Rgb(235, 203, 139),
            attribute: Color::Rgb(249, 226, 175),
        }
    } else {
        SyntaxColors {
            keyword: Color::Rgb(155, 89, 182),
            string: Color::Rgb(39, 174, 96),
            comment: Color::Rgb(149, 165, 166),
            function: Color::Rgb(41, 128, 185),
            number: Color::Rgb(192, 57, 43),
            type_color: Color::Rgb(241, 196, 15),
            attribute: Color::Rgb(230, 126, 34),
        }
    }
}

fn highlight_line(line: &str, language: &str, colors: &SyntaxColors) -> Vec<Span<'static>> {
    let keywords = get_keywords(language);
    let types = get_types(language);
    let mut spans = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut in_char = false;
    let mut in_comment = false;
    let mut string_char = '\0';

    for (idx, ch) in line.char_indices() {
        if in_comment {
            current.push(ch);
            if ch == '*' && line.chars().nth(idx + 1) == Some('/') {
                current.pop();
                spans.push(Span::styled(
                    current.clone(),
                    Style::default().fg(colors.comment),
                ));
                current.clear();
                in_comment = false;
            }
        } else if in_string || in_char {
            current.push(ch);
            if ch == '\\' && idx + 1 < line.len() {
                current.push(line.chars().nth(idx + 1).unwrap_or_default());
            } else if ch == string_char {
                spans.push(Span::styled(
                    current.clone(),
                    Style::default().fg(colors.string),
                ));
                current.clear();
                in_string = false;
                in_char = false;
            }
        } else {
            match ch {
                '"' => {
                    if !current.is_empty() {
                        spans.push(classify_token(&current, &keywords, &types, colors));
                        current.clear();
                    }
                    in_string = true;
                    string_char = '"';
                    current.push(ch);
                }
                '\'' => {
                    if !current.is_empty() {
                        spans.push(classify_token(&current, &keywords, &types, colors));
                        current.clear();
                    }
                    in_char = true;
                    string_char = '\'';
                    current.push(ch);
                }
                '/' => {
                    if line.chars().nth(idx + 1) == Some('/') {
                        if !current.is_empty() {
                            spans.push(classify_token(&current, &keywords, &types, colors));
                            current.clear();
                        }
                        let comment_content = &line[idx..];
                        spans.push(Span::styled(
                            comment_content.to_string(),
                            Style::default().fg(colors.comment),
                        ));
                        break;
                    } else if line.chars().nth(idx + 1) == Some('*') {
                        if !current.is_empty() {
                            spans.push(classify_token(&current, &keywords, &types, colors));
                            current.clear();
                        }
                        current.push(ch);
                        in_comment = true;
                    } else {
                        current.push(ch);
                    }
                }
                _ => {
                    current.push(ch);
                }
            }
        }
    }

    if !current.is_empty() && !in_comment && !in_string && !in_char {
        spans.push(classify_token(&current, &keywords, &types, colors));
    } else if in_string || in_char || in_comment {
        let color = if in_comment {
            colors.comment
        } else {
            colors.string
        };
        spans.push(Span::styled(current, Style::default().fg(color)));
    }

    if spans.is_empty() {
        vec![Span::raw(line.to_string())]
    } else {
        spans
    }
}

fn classify_token(
    token: &str,
    keywords: &HashMap<&str, SyntaxHighlight>,
    types: &HashMap<&str, SyntaxHighlight>,
    colors: &SyntaxColors,
) -> Span<'static> {
    let trimmed = token.trim();

    if let Some(&highlight) = keywords.get(trimmed) {
        return Span::styled(
            token.to_string(),
            Style::default().fg(match highlight {
                SyntaxHighlight::Keyword => colors.keyword,
                SyntaxHighlight::String => colors.string,
                SyntaxHighlight::Comment => colors.comment,
                SyntaxHighlight::Function => colors.function,
                SyntaxHighlight::Number => colors.number,
                SyntaxHighlight::Type => colors.type_color,
                SyntaxHighlight::Attribute => colors.attribute,
                SyntaxHighlight::None => Color::Reset,
            }),
        );
    }

    if let Some(&highlight) = types.get(trimmed) {
        return Span::styled(
            token.to_string(),
            Style::default().fg(match highlight {
                SyntaxHighlight::Keyword => colors.keyword,
                SyntaxHighlight::String => colors.string,
                SyntaxHighlight::Comment => colors.comment,
                SyntaxHighlight::Function => colors.function,
                SyntaxHighlight::Number => colors.number,
                SyntaxHighlight::Type => colors.type_color,
                SyntaxHighlight::Attribute => colors.attribute,
                SyntaxHighlight::None => Color::Reset,
            }),
        );
    }

    if trimmed
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == 'x' || c == 'X' || c == 'o' || c == 'b')
    {
        return Span::styled(token.to_string(), Style::default().fg(colors.number));
    }

    Span::raw(token.to_string())
}

fn get_keywords(language: &str) -> HashMap<&str, SyntaxHighlight> {
    let mut keywords = HashMap::new();

    match language {
        "rust" => {
            keywords.insert("as", SyntaxHighlight::Keyword);
            keywords.insert("async", SyntaxHighlight::Keyword);
            keywords.insert("await", SyntaxHighlight::Keyword);
            keywords.insert("break", SyntaxHighlight::Keyword);
            keywords.insert("const", SyntaxHighlight::Keyword);
            keywords.insert("continue", SyntaxHighlight::Keyword);
            keywords.insert("crate", SyntaxHighlight::Keyword);
            keywords.insert("dyn", SyntaxHighlight::Keyword);
            keywords.insert("else", SyntaxHighlight::Keyword);
            keywords.insert("enum", SyntaxHighlight::Keyword);
            keywords.insert("extern", SyntaxHighlight::Keyword);
            keywords.insert("false", SyntaxHighlight::Keyword);
            keywords.insert("fn", SyntaxHighlight::Keyword);
            keywords.insert("for", SyntaxHighlight::Keyword);
            keywords.insert("if", SyntaxHighlight::Keyword);
            keywords.insert("impl", SyntaxHighlight::Keyword);
            keywords.insert("in", SyntaxHighlight::Keyword);
            keywords.insert("let", SyntaxHighlight::Keyword);
            keywords.insert("loop", SyntaxHighlight::Keyword);
            keywords.insert("match", SyntaxHighlight::Keyword);
            keywords.insert("mod", SyntaxHighlight::Keyword);
            keywords.insert("move", SyntaxHighlight::Keyword);
            keywords.insert("mut", SyntaxHighlight::Keyword);
            keywords.insert("pub", SyntaxHighlight::Keyword);
            keywords.insert("ref", SyntaxHighlight::Keyword);
            keywords.insert("return", SyntaxHighlight::Keyword);
            keywords.insert("self", SyntaxHighlight::Keyword);
            keywords.insert("Self", SyntaxHighlight::Keyword);
            keywords.insert("static", SyntaxHighlight::Keyword);
            keywords.insert("struct", SyntaxHighlight::Keyword);
            keywords.insert("super", SyntaxHighlight::Keyword);
            keywords.insert("trait", SyntaxHighlight::Keyword);
            keywords.insert("true", SyntaxHighlight::Keyword);
            keywords.insert("type", SyntaxHighlight::Keyword);
            keywords.insert("unsafe", SyntaxHighlight::Keyword);
            keywords.insert("use", SyntaxHighlight::Keyword);
            keywords.insert("where", SyntaxHighlight::Keyword);
        }
        "python" | "ruby" => {
            keywords.insert("def", SyntaxHighlight::Keyword);
            keywords.insert("class", SyntaxHighlight::Keyword);
            keywords.insert("if", SyntaxHighlight::Keyword);
            keywords.insert("elif", SyntaxHighlight::Keyword);
            keywords.insert("else", SyntaxHighlight::Keyword);
            keywords.insert("while", SyntaxHighlight::Keyword);
            keywords.insert("for", SyntaxHighlight::Keyword);
            keywords.insert("in", SyntaxHighlight::Keyword);
            keywords.insert("return", SyntaxHighlight::Keyword);
            keywords.insert("import", SyntaxHighlight::Keyword);
            keywords.insert("from", SyntaxHighlight::Keyword);
            keywords.insert("as", SyntaxHighlight::Keyword);
            keywords.insert("try", SyntaxHighlight::Keyword);
            keywords.insert("except", SyntaxHighlight::Keyword);
            keywords.insert("finally", SyntaxHighlight::Keyword);
            keywords.insert("raise", SyntaxHighlight::Keyword);
            keywords.insert("with", SyntaxHighlight::Keyword);
            keywords.insert("lambda", SyntaxHighlight::Keyword);
            keywords.insert("True", SyntaxHighlight::Keyword);
            keywords.insert("False", SyntaxHighlight::Keyword);
            keywords.insert("None", SyntaxHighlight::Keyword);
            keywords.insert("and", SyntaxHighlight::Keyword);
            keywords.insert("or", SyntaxHighlight::Keyword);
            keywords.insert("not", SyntaxHighlight::Keyword);
        }
        "javascript" | "typescript" => {
            keywords.insert("const", SyntaxHighlight::Keyword);
            keywords.insert("let", SyntaxHighlight::Keyword);
            keywords.insert("var", SyntaxHighlight::Keyword);
            keywords.insert("function", SyntaxHighlight::Keyword);
            keywords.insert("async", SyntaxHighlight::Keyword);
            keywords.insert("await", SyntaxHighlight::Keyword);
            keywords.insert("if", SyntaxHighlight::Keyword);
            keywords.insert("else", SyntaxHighlight::Keyword);
            keywords.insert("for", SyntaxHighlight::Keyword);
            keywords.insert("while", SyntaxHighlight::Keyword);
            keywords.insert("do", SyntaxHighlight::Keyword);
            keywords.insert("switch", SyntaxHighlight::Keyword);
            keywords.insert("case", SyntaxHighlight::Keyword);
            keywords.insert("break", SyntaxHighlight::Keyword);
            keywords.insert("continue", SyntaxHighlight::Keyword);
            keywords.insert("return", SyntaxHighlight::Keyword);
            keywords.insert("try", SyntaxHighlight::Keyword);
            keywords.insert("catch", SyntaxHighlight::Keyword);
            keywords.insert("finally", SyntaxHighlight::Keyword);
            keywords.insert("throw", SyntaxHighlight::Keyword);
            keywords.insert("new", SyntaxHighlight::Keyword);
            keywords.insert("class", SyntaxHighlight::Keyword);
            keywords.insert("extends", SyntaxHighlight::Keyword);
            keywords.insert("import", SyntaxHighlight::Keyword);
            keywords.insert("export", SyntaxHighlight::Keyword);
            keywords.insert("from", SyntaxHighlight::Keyword);
            keywords.insert("default", SyntaxHighlight::Keyword);
            keywords.insert("typeof", SyntaxHighlight::Keyword);
            keywords.insert("instanceof", SyntaxHighlight::Keyword);
            keywords.insert("this", SyntaxHighlight::Keyword);
            keywords.insert("true", SyntaxHighlight::Keyword);
            keywords.insert("false", SyntaxHighlight::Keyword);
            keywords.insert("null", SyntaxHighlight::Keyword);
            keywords.insert("undefined", SyntaxHighlight::Keyword);
        }
        "go" => {
            keywords.insert("package", SyntaxHighlight::Keyword);
            keywords.insert("import", SyntaxHighlight::Keyword);
            keywords.insert("func", SyntaxHighlight::Keyword);
            keywords.insert("var", SyntaxHighlight::Keyword);
            keywords.insert("const", SyntaxHighlight::Keyword);
            keywords.insert("type", SyntaxHighlight::Keyword);
            keywords.insert("struct", SyntaxHighlight::Keyword);
            keywords.insert("interface", SyntaxHighlight::Keyword);
            keywords.insert("map", SyntaxHighlight::Keyword);
            keywords.insert("chan", SyntaxHighlight::Keyword);
            keywords.insert("if", SyntaxHighlight::Keyword);
            keywords.insert("else", SyntaxHighlight::Keyword);
            keywords.insert("for", SyntaxHighlight::Keyword);
            keywords.insert("range", SyntaxHighlight::Keyword);
            keywords.insert("switch", SyntaxHighlight::Keyword);
            keywords.insert("case", SyntaxHighlight::Keyword);
            keywords.insert("default", SyntaxHighlight::Keyword);
            keywords.insert("break", SyntaxHighlight::Keyword);
            keywords.insert("continue", SyntaxHighlight::Keyword);
            keywords.insert("return", SyntaxHighlight::Keyword);
            keywords.insert("go", SyntaxHighlight::Keyword);
            keywords.insert("defer", SyntaxHighlight::Keyword);
            keywords.insert("select", SyntaxHighlight::Keyword);
            keywords.insert("true", SyntaxHighlight::Keyword);
            keywords.insert("false", SyntaxHighlight::Keyword);
            keywords.insert("nil", SyntaxHighlight::Keyword);
        }
        "java" => {
            keywords.insert("public", SyntaxHighlight::Keyword);
            keywords.insert("private", SyntaxHighlight::Keyword);
            keywords.insert("protected", SyntaxHighlight::Keyword);
            keywords.insert("static", SyntaxHighlight::Keyword);
            keywords.insert("final", SyntaxHighlight::Keyword);
            keywords.insert("abstract", SyntaxHighlight::Keyword);
            keywords.insert("class", SyntaxHighlight::Keyword);
            keywords.insert("interface", SyntaxHighlight::Keyword);
            keywords.insert("enum", SyntaxHighlight::Keyword);
            keywords.insert("extends", SyntaxHighlight::Keyword);
            keywords.insert("implements", SyntaxHighlight::Keyword);
            keywords.insert("new", SyntaxHighlight::Keyword);
            keywords.insert("this", SyntaxHighlight::Keyword);
            keywords.insert("super", SyntaxHighlight::Keyword);
            keywords.insert("if", SyntaxHighlight::Keyword);
            keywords.insert("else", SyntaxHighlight::Keyword);
            keywords.insert("for", SyntaxHighlight::Keyword);
            keywords.insert("while", SyntaxHighlight::Keyword);
            keywords.insert("do", SyntaxHighlight::Keyword);
            keywords.insert("switch", SyntaxHighlight::Keyword);
            keywords.insert("case", SyntaxHighlight::Keyword);
            keywords.insert("default", SyntaxHighlight::Keyword);
            keywords.insert("break", SyntaxHighlight::Keyword);
            keywords.insert("continue", SyntaxHighlight::Keyword);
            keywords.insert("return", SyntaxHighlight::Keyword);
            keywords.insert("throw", SyntaxHighlight::Keyword);
            keywords.insert("throws", SyntaxHighlight::Keyword);
            keywords.insert("try", SyntaxHighlight::Keyword);
            keywords.insert("catch", SyntaxHighlight::Keyword);
            keywords.insert("finally", SyntaxHighlight::Keyword);
            keywords.insert("import", SyntaxHighlight::Keyword);
            keywords.insert("package", SyntaxHighlight::Keyword);
            keywords.insert("void", SyntaxHighlight::Keyword);
            keywords.insert("int", SyntaxHighlight::Keyword);
            keywords.insert("long", SyntaxHighlight::Keyword);
            keywords.insert("double", SyntaxHighlight::Keyword);
            keywords.insert("float", SyntaxHighlight::Keyword);
            keywords.insert("boolean", SyntaxHighlight::Keyword);
            keywords.insert("char", SyntaxHighlight::Keyword);
            keywords.insert("byte", SyntaxHighlight::Keyword);
            keywords.insert("short", SyntaxHighlight::Keyword);
            keywords.insert("true", SyntaxHighlight::Keyword);
            keywords.insert("false", SyntaxHighlight::Keyword);
            keywords.insert("null", SyntaxHighlight::Keyword);
        }
        "c" | "cpp" => {
            keywords.insert("int", SyntaxHighlight::Keyword);
            keywords.insert("char", SyntaxHighlight::Keyword);
            keywords.insert("void", SyntaxHighlight::Keyword);
            keywords.insert("float", SyntaxHighlight::Keyword);
            keywords.insert("double", SyntaxHighlight::Keyword);
            keywords.insert("long", SyntaxHighlight::Keyword);
            keywords.insert("short", SyntaxHighlight::Keyword);
            keywords.insert("unsigned", SyntaxHighlight::Keyword);
            keywords.insert("signed", SyntaxHighlight::Keyword);
            keywords.insert("const", SyntaxHighlight::Keyword);
            keywords.insert("static", SyntaxHighlight::Keyword);
            keywords.insert("extern", SyntaxHighlight::Keyword);
            keywords.insert("volatile", SyntaxHighlight::Keyword);
            keywords.insert("sizeof", SyntaxHighlight::Keyword);
            keywords.insert("typedef", SyntaxHighlight::Keyword);
            keywords.insert("struct", SyntaxHighlight::Keyword);
            keywords.insert("union", SyntaxHighlight::Keyword);
            keywords.insert("enum", SyntaxHighlight::Keyword);
            keywords.insert("if", SyntaxHighlight::Keyword);
            keywords.insert("else", SyntaxHighlight::Keyword);
            keywords.insert("for", SyntaxHighlight::Keyword);
            keywords.insert("while", SyntaxHighlight::Keyword);
            keywords.insert("do", SyntaxHighlight::Keyword);
            keywords.insert("switch", SyntaxHighlight::Keyword);
            keywords.insert("case", SyntaxHighlight::Keyword);
            keywords.insert("default", SyntaxHighlight::Keyword);
            keywords.insert("break", SyntaxHighlight::Keyword);
            keywords.insert("continue", SyntaxHighlight::Keyword);
            keywords.insert("return", SyntaxHighlight::Keyword);
            keywords.insert("goto", SyntaxHighlight::Keyword);
            keywords.insert("include", SyntaxHighlight::Keyword);
            keywords.insert("define", SyntaxHighlight::Keyword);
            keywords.insert("ifdef", SyntaxHighlight::Keyword);
            keywords.insert("ifndef", SyntaxHighlight::Keyword);
            keywords.insert("endif", SyntaxHighlight::Keyword);
            keywords.insert("NULL", SyntaxHighlight::Keyword);
        }
        "csharp" => {
            keywords.insert("using", SyntaxHighlight::Keyword);
            keywords.insert("namespace", SyntaxHighlight::Keyword);
            keywords.insert("class", SyntaxHighlight::Keyword);
            keywords.insert("struct", SyntaxHighlight::Keyword);
            keywords.insert("interface", SyntaxHighlight::Keyword);
            keywords.insert("enum", SyntaxHighlight::Keyword);
            keywords.insert("public", SyntaxHighlight::Keyword);
            keywords.insert("private", SyntaxHighlight::Keyword);
            keywords.insert("protected", SyntaxHighlight::Keyword);
            keywords.insert("internal", SyntaxHighlight::Keyword);
            keywords.insert("static", SyntaxHighlight::Keyword);
            keywords.insert("readonly", SyntaxHighlight::Keyword);
            keywords.insert("const", SyntaxHighlight::Keyword);
            keywords.insert("new", SyntaxHighlight::Keyword);
            keywords.insert("virtual", SyntaxHighlight::Keyword);
            keywords.insert("override", SyntaxHighlight::Keyword);
            keywords.insert("abstract", SyntaxHighlight::Keyword);
            keywords.insert("sealed", SyntaxHighlight::Keyword);
            keywords.insert("partial", SyntaxHighlight::Keyword);
            keywords.insert("void", SyntaxHighlight::Keyword);
            keywords.insert("int", SyntaxHighlight::Keyword);
            keywords.insert("long", SyntaxHighlight::Keyword);
            keywords.insert("double", SyntaxHighlight::Keyword);
            keywords.insert("float", SyntaxHighlight::Keyword);
            keywords.insert("bool", SyntaxHighlight::Keyword);
            keywords.insert("string", SyntaxHighlight::Keyword);
            keywords.insert("var", SyntaxHighlight::Keyword);
            keywords.insert("if", SyntaxHighlight::Keyword);
            keywords.insert("else", SyntaxHighlight::Keyword);
            keywords.insert("for", SyntaxHighlight::Keyword);
            keywords.insert("foreach", SyntaxHighlight::Keyword);
            keywords.insert("while", SyntaxHighlight::Keyword);
            keywords.insert("do", SyntaxHighlight::Keyword);
            keywords.insert("switch", SyntaxHighlight::Keyword);
            keywords.insert("case", SyntaxHighlight::Keyword);
            keywords.insert("default", SyntaxHighlight::Keyword);
            keywords.insert("break", SyntaxHighlight::Keyword);
            keywords.insert("continue", SyntaxHighlight::Keyword);
            keywords.insert("return", SyntaxHighlight::Keyword);
            keywords.insert("try", SyntaxHighlight::Keyword);
            keywords.insert("catch", SyntaxHighlight::Keyword);
            keywords.insert("finally", SyntaxHighlight::Keyword);
            keywords.insert("throw", SyntaxHighlight::Keyword);
            keywords.insert("true", SyntaxHighlight::Keyword);
            keywords.insert("false", SyntaxHighlight::Keyword);
            keywords.insert("null", SyntaxHighlight::Keyword);
        }
        "swift" => {
            keywords.insert("import", SyntaxHighlight::Keyword);
            keywords.insert("class", SyntaxHighlight::Keyword);
            keywords.insert("struct", SyntaxHighlight::Keyword);
            keywords.insert("enum", SyntaxHighlight::Keyword);
            keywords.insert("protocol", SyntaxHighlight::Keyword);
            keywords.insert("extension", SyntaxHighlight::Keyword);
            keywords.insert("func", SyntaxHighlight::Keyword);
            keywords.insert("var", SyntaxHighlight::Keyword);
            keywords.insert("let", SyntaxHighlight::Keyword);
            keywords.insert("static", SyntaxHighlight::Keyword);
            keywords.insert("public", SyntaxHighlight::Keyword);
            keywords.insert("private", SyntaxHighlight::Keyword);
            keywords.insert("internal", SyntaxHighlight::Keyword);
            keywords.insert("open", SyntaxHighlight::Keyword);
            keywords.insert("final", SyntaxHighlight::Keyword);
            keywords.insert("override", SyntaxHighlight::Keyword);
            keywords.insert("mutating", SyntaxHighlight::Keyword);
            keywords.insert("nonmutating", SyntaxHighlight::Keyword);
            keywords.insert("lazy", SyntaxHighlight::Keyword);
            keywords.insert("if", SyntaxHighlight::Keyword);
            keywords.insert("else", SyntaxHighlight::Keyword);
            keywords.insert("guard", SyntaxHighlight::Keyword);
            keywords.insert("switch", SyntaxHighlight::Keyword);
            keywords.insert("case", SyntaxHighlight::Keyword);
            keywords.insert("default", SyntaxHighlight::Keyword);
            keywords.insert("for", SyntaxHighlight::Keyword);
            keywords.insert("while", SyntaxHighlight::Keyword);
            keywords.insert("repeat", SyntaxHighlight::Keyword);
            keywords.insert("break", SyntaxHighlight::Keyword);
            keywords.insert("continue", SyntaxHighlight::Keyword);
            keywords.insert("return", SyntaxHighlight::Keyword);
            keywords.insert("throw", SyntaxHighlight::Keyword);
            keywords.insert("throws", SyntaxHighlight::Keyword);
            keywords.insert("try", SyntaxHighlight::Keyword);
            keywords.insert("catch", SyntaxHighlight::Keyword);
            keywords.insert("init", SyntaxHighlight::Keyword);
            keywords.insert("deinit", SyntaxHighlight::Keyword);
            keywords.insert("self", SyntaxHighlight::Keyword);
            keywords.insert("Self", SyntaxHighlight::Keyword);
            keywords.insert("nil", SyntaxHighlight::Keyword);
            keywords.insert("true", SyntaxHighlight::Keyword);
            keywords.insert("false", SyntaxHighlight::Keyword);
        }
        "kotlin" => {
            keywords.insert("package", SyntaxHighlight::Keyword);
            keywords.insert("import", SyntaxHighlight::Keyword);
            keywords.insert("class", SyntaxHighlight::Keyword);
            keywords.insert("object", SyntaxHighlight::Keyword);
            keywords.insert("interface", SyntaxHighlight::Keyword);
            keywords.insert("data", SyntaxHighlight::Keyword);
            keywords.insert("sealed", SyntaxHighlight::Keyword);
            keywords.insert("enum", SyntaxHighlight::Keyword);
            keywords.insert("annotation", SyntaxHighlight::Keyword);
            keywords.insert("fun", SyntaxHighlight::Keyword);
            keywords.insert("val", SyntaxHighlight::Keyword);
            keywords.insert("var", SyntaxHighlight::Keyword);
            keywords.insert("const", SyntaxHighlight::Keyword);
            keywords.insert("lateinit", SyntaxHighlight::Keyword);
            keywords.insert("by", SyntaxHighlight::Keyword);
            keywords.insert("companion", SyntaxHighlight::Keyword);
            keywords.insert("object", SyntaxHighlight::Keyword);
            keywords.insert("open", SyntaxHighlight::Keyword);
            keywords.insert("abstract", SyntaxHighlight::Keyword);
            keywords.insert("final", SyntaxHighlight::Keyword);
            keywords.insert("override", SyntaxHighlight::Keyword);
            keywords.insert("private", SyntaxHighlight::Keyword);
            keywords.insert("protected", SyntaxHighlight::Keyword);
            keywords.insert("public", SyntaxHighlight::Keyword);
            keywords.insert("internal", SyntaxHighlight::Keyword);
            keywords.insert("suspend", SyntaxHighlight::Keyword);
            keywords.insert("inline", SyntaxHighlight::Keyword);
            keywords.insert("reified", SyntaxHighlight::Keyword);
            keywords.insert("noinline", SyntaxHighlight::Keyword);
            keywords.insert("crossinline", SyntaxHighlight::Keyword);
            keywords.insert("out", SyntaxHighlight::Keyword);
            keywords.insert("in", SyntaxHighlight::Keyword);
            keywords.insert("where", SyntaxHighlight::Keyword);
            keywords.insert("if", SyntaxHighlight::Keyword);
            keywords.insert("else", SyntaxHighlight::Keyword);
            keywords.insert("when", SyntaxHighlight::Keyword);
            keywords.insert("for", SyntaxHighlight::Keyword);
            keywords.insert("while", SyntaxHighlight::Keyword);
            keywords.insert("do", SyntaxHighlight::Keyword);
            keywords.insert("return", SyntaxHighlight::Keyword);
            keywords.insert("break", SyntaxHighlight::Keyword);
            keywords.insert("continue", SyntaxHighlight::Keyword);
            keywords.insert("throw", SyntaxHighlight::Keyword);
            keywords.insert("try", SyntaxHighlight::Keyword);
            keywords.insert("catch", SyntaxHighlight::Keyword);
            keywords.insert("finally", SyntaxHighlight::Keyword);
            keywords.insert("this", SyntaxHighlight::Keyword);
            keywords.insert("null", SyntaxHighlight::Keyword);
            keywords.insert("true", SyntaxHighlight::Keyword);
            keywords.insert("false", SyntaxHighlight::Keyword);
            keywords.insert("is", SyntaxHighlight::Keyword);
            keywords.insert("as", SyntaxHighlight::Keyword);
            keywords.insert("as?", SyntaxHighlight::Keyword);
        }
        _ => {}
    }

    keywords
}

fn get_types(language: &str) -> HashMap<&str, SyntaxHighlight> {
    let mut types = HashMap::new();

    match language {
        "rust" => {
            types.insert("String", SyntaxHighlight::Type);
            types.insert("Vec", SyntaxHighlight::Type);
            types.insert("Option", SyntaxHighlight::Type);
            types.insert("Result", SyntaxHighlight::Type);
            types.insert("Box", SyntaxHighlight::Type);
            types.insert("Rc", SyntaxHighlight::Type);
            types.insert("Arc", SyntaxHighlight::Type);
            types.insert("Cell", SyntaxHighlight::Type);
            types.insert("RefCell", SyntaxHighlight::Type);
            types.insert("Cow", SyntaxHighlight::Type);
            types.insert("HashMap", SyntaxHighlight::Type);
            types.insert("HashSet", SyntaxHighlight::Type);
            types.insert("BTreeMap", SyntaxHighlight::Type);
            types.insert("BTreeSet", SyntaxHighlight::Type);
            types.insert("LinkedList", SyntaxHighlight::Type);
            types.insert("VecDeque", SyntaxHighlight::Type);
            types.insert("BinaryHeap", SyntaxHighlight::Type);
            types.insert("i8", SyntaxHighlight::Type);
            types.insert("i16", SyntaxHighlight::Type);
            types.insert("i32", SyntaxHighlight::Type);
            types.insert("i64", SyntaxHighlight::Type);
            types.insert("i128", SyntaxHighlight::Type);
            types.insert("u8", SyntaxHighlight::Type);
            types.insert("u16", SyntaxHighlight::Type);
            types.insert("u32", SyntaxHighlight::Type);
            types.insert("u64", SyntaxHighlight::Type);
            types.insert("u128", SyntaxHighlight::Type);
            types.insert("usize", SyntaxHighlight::Type);
            types.insert("isize", SyntaxHighlight::Type);
            types.insert("f32", SyntaxHighlight::Type);
            types.insert("f64", SyntaxHighlight::Type);
            types.insert("bool", SyntaxHighlight::Type);
            types.insert("char", SyntaxHighlight::Type);
            types.insert("str", SyntaxHighlight::Type);
            types.insert("Path", SyntaxHighlight::Type);
            types.insert("PathBuf", SyntaxHighlight::Type);
            types.insert("OsString", SyntaxHighlight::Type);
            types.insert("CString", SyntaxHighlight::Type);
            types.insert("Duration", SyntaxHighlight::Type);
            types.insert("SystemTime", SyntaxHighlight::Type);
        }
        "python" | "ruby" => {
            types.insert("int", SyntaxHighlight::Type);
            types.insert("float", SyntaxHighlight::Type);
            types.insert("str", SyntaxHighlight::Type);
            types.insert("bool", SyntaxHighlight::Type);
            types.insert("list", SyntaxHighlight::Type);
            types.insert("dict", SyntaxHighlight::Type);
            types.insert("tuple", SyntaxHighlight::Type);
            types.insert("set", SyntaxHighlight::Type);
            types.insert("bytes", SyntaxHighlight::Type);
            types.insert("bytearray", SyntaxHighlight::Type);
            types.insert("range", SyntaxHighlight::Type);
            types.insert("memoryview", SyntaxHighlight::Type);
            types.insert("type", SyntaxHighlight::Type);
            types.insert("object", SyntaxHighlight::Type);
            types.insert("Exception", SyntaxHighlight::Type);
        }
        "javascript" | "typescript" => {
            types.insert("string", SyntaxHighlight::Type);
            types.insert("number", SyntaxHighlight::Type);
            types.insert("boolean", SyntaxHighlight::Type);
            types.insert("undefined", SyntaxHighlight::Type);
            types.insert("null", SyntaxHighlight::Type);
            types.insert("symbol", SyntaxHighlight::Type);
            types.insert("bigint", SyntaxHighlight::Type);
            types.insert("any", SyntaxHighlight::Type);
            types.insert("void", SyntaxHighlight::Type);
            types.insert("never", SyntaxHighlight::Type);
            types.insert("unknown", SyntaxHighlight::Type);
            types.insert("object", SyntaxHighlight::Type);
            types.insert("Array", SyntaxHighlight::Type);
            types.insert("Promise", SyntaxHighlight::Type);
            types.insert("Map", SyntaxHighlight::Type);
            types.insert("Set", SyntaxHighlight::Type);
            types.insert("WeakMap", SyntaxHighlight::Type);
            types.insert("WeakSet", SyntaxHighlight::Type);
        }
        "go" => {
            types.insert("string", SyntaxHighlight::Type);
            types.insert("int", SyntaxHighlight::Type);
            types.insert("int8", SyntaxHighlight::Type);
            types.insert("int16", SyntaxHighlight::Type);
            types.insert("int32", SyntaxHighlight::Type);
            types.insert("int64", SyntaxHighlight::Type);
            types.insert("uint", SyntaxHighlight::Type);
            types.insert("uint8", SyntaxHighlight::Type);
            types.insert("uint16", SyntaxHighlight::Type);
            types.insert("uint32", SyntaxHighlight::Type);
            types.insert("uint64", SyntaxHighlight::Type);
            types.insert("uintptr", SyntaxHighlight::Type);
            types.insert("float32", SyntaxHighlight::Type);
            types.insert("float64", SyntaxHighlight::Type);
            types.insert("complex64", SyntaxHighlight::Type);
            types.insert("complex128", SyntaxHighlight::Type);
            types.insert("bool", SyntaxHighlight::Type);
            types.insert("byte", SyntaxHighlight::Type);
            types.insert("rune", SyntaxHighlight::Type);
            types.insert("error", SyntaxHighlight::Type);
            types.insert("interface", SyntaxHighlight::Type);
            types.insert("struct", SyntaxHighlight::Type);
            types.insert("chan", SyntaxHighlight::Type);
            types.insert("map", SyntaxHighlight::Type);
            types.insert("func", SyntaxHighlight::Type);
        }
        "java" | "csharp" => {
            types.insert("String", SyntaxHighlight::Type);
            types.insert("Integer", SyntaxHighlight::Type);
            types.insert("int", SyntaxHighlight::Type);
            types.insert("Long", SyntaxHighlight::Type);
            types.insert("long", SyntaxHighlight::Type);
            types.insert("Double", SyntaxHighlight::Type);
            types.insert("double", SyntaxHighlight::Type);
            types.insert("Float", SyntaxHighlight::Type);
            types.insert("float", SyntaxHighlight::Type);
            types.insert("Boolean", SyntaxHighlight::Type);
            types.insert("boolean", SyntaxHighlight::Type);
            types.insert("Character", SyntaxHighlight::Type);
            types.insert("char", SyntaxHighlight::Type);
            types.insert("Byte", SyntaxHighlight::Type);
            types.insert("byte", SyntaxHighlight::Type);
            types.insert("Short", SyntaxHighlight::Type);
            types.insert("short", SyntaxHighlight::Type);
            types.insert("Object", SyntaxHighlight::Type);
            types.insert("List", SyntaxHighlight::Type);
            types.insert("ArrayList", SyntaxHighlight::Type);
            types.insert("Map", SyntaxHighlight::Type);
            types.insert("HashMap", SyntaxHighlight::Type);
            types.insert("Set", SyntaxHighlight::Type);
            types.insert("HashSet", SyntaxHighlight::Type);
            types.insert("Collection", SyntaxHighlight::Type);
            types.insert("Iterator", SyntaxHighlight::Type);
            types.insert("Exception", SyntaxHighlight::Type);
        }
        "c" | "cpp" => {
            types.insert("FILE", SyntaxHighlight::Type);
            types.insert("size_t", SyntaxHighlight::Type);
            types.insert("ssize_t", SyntaxHighlight::Type);
            types.insert("intptr_t", SyntaxHighlight::Type);
            types.insert("uintptr_t", SyntaxHighlight::Type);
            types.insert("bool", SyntaxHighlight::Type);
            types.insert("wchar_t", SyntaxHighlight::Type);
        }
        _ => {}
    }

    types
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

    pub fn detect_language(file_path: &str) -> &'static str {
        let ext = Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            "rs" => "rust",
            "py" => "python",
            "js" | "ts" | "jsx" | "tsx" => "javascript",
            "go" => "go",
            "java" => "java",
            "c" | "h" => "c",
            "cpp" | "cc" | "cxx" | "hpp" => "cpp",
            "cs" => "csharp",
            "rb" => "ruby",
            "php" => "php",
            "swift" => "swift",
            "kt" | "kts" => "kotlin",
            "scala" => "scala",
            "html" | "htm" => "html",
            "css" => "css",
            "scss" | "sass" => "scss",
            "json" => "json",
            "yaml" | "yml" => "yaml",
            "xml" => "xml",
            "sh" | "bash" | "zsh" => "bash",
            "toml" => "toml",
            "md" => "markdown",
            "sql" => "sql",
            "r" => "r",
            "lua" => "lua",
            "perl" | "pl" => "perl",
            "ex" | "exs" => "elixir",
            "erl" | "hrl" => "erlang",
            "clj" | "cljs" | "cljc" => "clojure",
            "hs" => "haskell",
            "ml" | "mli" => "ocaml",
            "fs" | "fsi" | "fsx" => "fsharp",
            "nim" => "nim",
            "v" | "vv" => "v",
            "zig" => "zig",
            _ => "plaintext",
        }
    }

    pub fn apply_syntax_highlighting(
        content: &str,
        language: &str,
        is_addition: bool,
        dark_theme: bool,
    ) -> Line<'static> {
        let theme_colors = get_syntax_colors(dark_theme);

        let spans: Vec<Span<'static>> = if language == "plaintext" || content.trim().is_empty() {
            vec![Span::raw(content.to_string())]
        } else {
            highlight_line(content, language, &theme_colors)
                .into_iter()
                .map(|s| Span::<'static> {
                    content: s.content,
                    style: s.style,
                })
                .collect()
        };

        let base_style = if is_addition {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default().add_modifier(Modifier::DIM)
        };

        Line::from(spans).style(base_style)
    }

    pub fn apply_syntax_highlighting_with_numbers(
        content: &str,
        line_number: Option<usize>,
        language: &str,
        line_type: DiffLineType,
        dark_theme: bool,
    ) -> Line<'static> {
        let is_addition = line_type == DiffLineType::Addition;
        let styled_content =
            Self::apply_syntax_highlighting(content, language, is_addition, dark_theme);

        let number_fg = if dark_theme {
            Color::Rgb(100, 100, 100)
        } else {
            Color::Gray
        };
        let number_style = Style::default().fg(number_fg);

        let line_prefix = match line_number {
            Some(n) => format!("{:>4} ", n),
            None => "      ".to_string(),
        };

        Line::from(vec![
            Span::styled(line_prefix, number_style),
            Span::styled(
                styled_content
                    .spans
                    .iter()
                    .map(|s| s.content.clone())
                    .collect::<String>(),
                styled_content.style,
            ),
        ])
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

    #[test]
    fn test_detect_language_rust() {
        assert_eq!(DiffParser::detect_language("src/main.rs"), "rust");
        assert_eq!(DiffParser::detect_language("lib.rs"), "rust");
    }

    #[test]
    fn test_detect_language_python() {
        assert_eq!(DiffParser::detect_language("main.py"), "python");
        assert_eq!(DiffParser::detect_language("script.py"), "python");
    }

    #[test]
    fn test_detect_language_javascript() {
        assert_eq!(DiffParser::detect_language("app.js"), "javascript");
        assert_eq!(DiffParser::detect_language("component.tsx"), "javascript");
        assert_eq!(DiffParser::detect_language("types.ts"), "javascript");
    }

    #[test]
    fn test_detect_language_unknown() {
        assert_eq!(DiffParser::detect_language("somefile.xyz"), "plaintext");
        assert_eq!(DiffParser::detect_language("README"), "plaintext");
    }

    #[test]
    fn test_syntax_highlighting_rust_keywords() {
        let line = "fn main() {";
        let styled = DiffParser::apply_syntax_highlighting(line, "rust", true, true);
        assert!(!styled.spans.is_empty());
    }

    #[test]
    fn test_syntax_highlighting_python_keywords() {
        let line = "def function_name():";
        let styled = DiffParser::apply_syntax_highlighting(line, "python", true, true);
        assert!(!styled.spans.is_empty());
    }

    #[test]
    fn test_syntax_highlighting_string_literal() {
        let line = r#"let message = "Hello, World!";"#;
        let styled = DiffParser::apply_syntax_highlighting(line, "rust", true, true);
        assert!(!styled.spans.is_empty());
    }

    #[test]
    fn test_syntax_highlighting_comment() {
        let line = "// This is a comment";
        let styled = DiffParser::apply_syntax_highlighting(line, "rust", false, true);
        assert!(!styled.spans.is_empty());
    }

    #[test]
    fn test_syntax_highlighting_numbers() {
        let line = "let number = 42;";
        let styled = DiffParser::apply_syntax_highlighting(line, "rust", true, true);
        assert!(!styled.spans.is_empty());
    }

    #[test]
    fn test_syntax_highlighting_light_theme() {
        let line = "let x = 123;";
        let styled = DiffParser::apply_syntax_highlighting(line, "rust", true, false);
        assert!(!styled.spans.is_empty());
    }

    #[test]
    fn test_syntax_highlighting_plaintext() {
        let line = "some random text";
        let styled = DiffParser::apply_syntax_highlighting(line, "plaintext", false, true);
        assert_eq!(styled.spans.len(), 1);
    }

    #[test]
    fn test_syntax_highlighting_empty_line() {
        let line = "";
        let styled = DiffParser::apply_syntax_highlighting(line, "rust", false, true);
        assert!(!styled.spans.is_empty());
    }

    #[test]
    fn test_syntax_highlighting_with_numbers() {
        let line = "fn test() {";
        let styled = DiffParser::apply_syntax_highlighting_with_numbers(
            line,
            Some(10),
            "rust",
            DiffLineType::Addition,
            true,
        );
        assert!(!styled.spans.is_empty());
    }

    #[test]
    fn test_syntax_highlighting_go_keywords() {
        let line = "package main";
        let styled = DiffParser::apply_syntax_highlighting(line, "go", true, true);
        assert!(!styled.spans.is_empty());
    }

    #[test]
    fn test_syntax_highlighting_javascript_keywords() {
        let line = "const array = [];";
        let styled = DiffParser::apply_syntax_highlighting(line, "javascript", true, true);
        assert!(!styled.spans.is_empty());
    }

    #[test]
    fn test_syntax_highlighting_cpp_keywords() {
        let line = "#include <iostream>";
        let styled = DiffParser::apply_syntax_highlighting(line, "cpp", false, true);
        assert!(!styled.spans.is_empty());
    }

    #[test]
    fn test_syntax_highlighting_json() {
        let line = r#""key": "value""#;
        let styled = DiffParser::apply_syntax_highlighting(line, "json", true, true);
        assert!(!styled.spans.is_empty());
    }

    #[test]
    fn test_syntax_highlighting_markdown() {
        let line = "## Heading";
        let styled = DiffParser::apply_syntax_highlighting(line, "markdown", false, true);
        assert!(!styled.spans.is_empty());
    }
}
