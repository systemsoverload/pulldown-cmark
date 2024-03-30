// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn blockquotes_tags_test_1() {
    let original = r##"> This is a normal blockquote without tag.
"##;
    let expected = r##"<blockquote><p>This is a normal blockquote without tag.</p></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn blockquotes_tags_test_2() {
    let original = r##"> [!NOTE]
> Note blockquote
"##;
    let expected = r##"<blockquote class="markdown-alert-note"><p>Note blockquote</p></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn blockquotes_tags_test_3() {
    let original = r##"> [!TIP]
> Tip blockquote
"##;
    let expected = r##"<blockquote class="markdown-alert-tip"><p>Tip blockquote</p></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn blockquotes_tags_test_4() {
    let original = r##"> [!IMPORTANT]
> Important blockquote
"##;
    let expected = r##"<blockquote class="markdown-alert-important"><p>Important blockquote</p></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn blockquotes_tags_test_5() {
    let original = r##"> [!WARNING]
> Warning blockquote
"##;
    let expected = r##"<blockquote class="markdown-alert-warning"><p>Warning blockquote</p></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn blockquotes_tags_test_6() {
    let original = r##"> [!CAUTION]
> Caution blockquote
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"><p>Caution blockquote</p></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn blockquotes_tags_test_7() {
    let original = r##"> [!CAUTION]
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn blockquotes_tags_test_8() {
    let original = r##"> [!CAUTION]
> Line 1.
> Line 2.
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"><p>Line 1.
Line 2.</p></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn blockquotes_tags_test_9() {
    let original = r##"> [!CAUTION]
> Line 1.
> [!CAUTION]
> Line 2.
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"><p>Line 1.
[!CAUTION]
Line 2.</p></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn blockquotes_tags_test_10() {
    let original = r##"> [!CAUTION]
> Line 1.
> > [!TIP]
> Line 2.
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"><p>Line 1.</p><blockquote class="markdown-alert-tip"><p>Line 2.</p></blockquote></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn blockquotes_tags_test_11() {
    let original = r##"> [!CAUTION]
> Line 1.


> [!TIP]
> Line 2.
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"><p>Line 1.</p></blockquote><blockquote class="markdown-alert-tip"><p>Line 2.</p></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}
