use super::*;

#[test]
fn test_name() {
    let preprocessor = RubyProcessor;
    assert_eq!(preprocessor.name(), "ruby")
}

#[test]
fn test_support_html() {
    let preprocessor = RubyProcessor;
    assert!(preprocessor.supports_renderer("html"));
    assert!(!preprocessor.supports_renderer("other_renderer"))
}

#[test]
fn test_rendering_without_math() {
    let preprocessor = RubyProcessor;
    // let macros = HashMap::new();
    let raw_content = r"Some text, and more text.";
    let mut expected_output = String::from("");
    expected_output.push_str(raw_content);
    let rendered_content = preprocessor.process_chapter(&raw_content);
    debug_assert_eq!(expected_output, rendered_content);
}

#[test]
fn test_inline_rendering() {
    let preprocessor = RubyProcessor;
    // let macros = HashMap::new();
    let raw_content = r"Some text, [sample]<<aaa>>, and more text.";
    let mut expected_output = String::from("");
    expected_output.push_str("Some text, <ruby>sample<rt>aaa</rt></ruby>, and more text.");
    let rendered_content = preprocessor.process_chapter(&raw_content);
    debug_assert_eq!(expected_output, rendered_content);
}

