use unicode_width::UnicodeWidthChar;

use crate::symbols::Span;

//FIX: ANSI
const RED: &str = "\x1b[31m";
const ORANGE: &str = "\x1b[33m";
const NC: &str = "\x1b[0m";

//TODO: Handle multi-line errors
// Store \n array for binary search NOT now. DO NOT. do it now.
// Ok

// FIXME: Given 'a: A "[Range()]' span.end reaches past the line causing a subtract overflow.
// Handling multi-line will likely fix it
pub fn form_diagnostic(src: &[u8], span: &Span, can_color: bool) -> (usize, usize, String) {
    let mut ln = 1;

    let mut b: u8;

    let mut seg_start = 0;

    let src_str = str::from_utf8(src).expect("Lexer broke");

    for i in 0..span.end {
        b = src[i];

        //todo: see if this works on windows
        //WARN: Still haven't checked.
        if b == b'\r' && src.get(i + 1).copied() == Some(b'\n') {
            ln += 1;
            seg_start = i + 2;
        } else if b == b'\n' {
            ln += 1;
            seg_start = i + 1;
        }
    }

    let seg_end = get_line_end(src, seg_start);

    let segment = &src[seg_start..seg_end];

    // WARN: Suspicious off by one...
    let col = src_str[seg_start..span.start].chars().count() + 1;

    let str_segment = str::from_utf8(segment).expect("Lexer broke");

    // Could both of these look less odd?
    let space_offset = src_str[seg_start..span.start]
        .chars()
        .map(|c| UnicodeWidthChar::width(c).unwrap_or(0))
        .sum();

    let spaces = " ".repeat(space_offset);

    // span range is inclusive exclusive so final character is missed without + 1
    // Has no other mathematical reasoning outside of this
    let arrow_offset = src_str[span.start..span.end + 1]
        .chars()
        .map(|c| UnicodeWidthChar::width(c).unwrap_or(1))
        .sum();

    let arrows = "^".repeat(arrow_offset);

    let fmt_segment = if can_color {
        format!("\t{str_segment}\n\t{spaces}{RED}{arrows}{NC}")
    } else {
        format!("\t{str_segment}\n\t{spaces}{arrows}")
    };

    println!("{}", &fmt_segment);

    (ln, col, fmt_segment)
}

fn get_line_end(original_text: &[u8], start: usize) -> usize {
    for i in start..original_text.len() {
        let b = original_text[i];

        if b == b'\r' && original_text.get(i + 1).copied() == Some(b'\n') {
            return i;
        } else if b == b'\n' {
            return i;
        }
    }

    //WARN: I don't remember why I returned this
    original_text.len()
}

//TODO: Should this exist?
// pub fn format_segment(segment: &str) -> String {
//
// }

pub fn form_help_diagnostic(text: &[u8], span: &Span, can_color: bool) {}

pub fn form_help(msg: &str, can_color: bool) -> String {
    if can_color {
        format!("{ORANGE}Help{NC}: {msg}\n")
    } else {
        format!("Help: {msg}\n")
    }
}
