Released
--------

0.9.0 - 22 Okt 2024
===================
- Bump ratatui to v0.29.0

0.8.5 -  13 Okt 2023=4
===================
- Add `D` keymapping: Delete to end of line
- Syntax highlighting
```rust
#[cfg(feature = "syntax-highlighting")]
{
    use edtui::EditorState;
    use edtui::EditorView;
    use edtui::SyntaxHighlighter;

    let syntax_highlighter = SyntaxHighlighter::new("dracula", "rs");
    EditorView::new(&mut EditorState::default())
        .syntax_highlighter(Some(syntax_highlighter));
}
```
- Add `ci*` to change between delimiters, supported [\', ", (, [, {]
- Improve README

0.8.3 - 11 Okt 2024
===================
- Bugfix: MoveWordForward if out of bounds
- Bugfix: Capture state before switching to insert mode
- Bugfig: Render cursor if editor has no content

0.8.2 - 09 Okt 2024
===================
- Bump edtui-jagged version
- README: Fix keymappings

0.8.1 - 08 Okt 2024
===================
- Bugfix: Scroll down jumped one cell if wrap was set to true

0.8.0 - 07 Okt 2024
===================
- Support for line-wrapping `EditorView::new().wrap(true);`
- Move to first ('gg') / last ('G') row
- Copy deleted line to clipboard
- Refactoring
- Bump edtui-jagged version
- Capture editor state on each new input
- Bugfix: Move to matching bracket

0.7.7 - 03 Okt 2024
===================
- Move to opening/closing bracket with '%'

0.7.5 - 03 Okt 2024
===================
- Highlight search text

0.7.4 - 28 Sep 2024
===================
- Bump edtui jagged version

0.7.3 - 28 Sep 2024
===================
- Expose search_pattern() method
 
0.7.1 - 17 Aug 2024
===================
- Key event handler implements clone

0.7.0 - 17 Aug 2024
===================
- Bump ratatui to v0.28

0.5.1 - 10 Aug 2024
===================
- Make insert char safer
- Fix bug in selection mode

0.5.0 - 29 June 2024
===================
- Breaking change: Rename Input to EditorInput
- Breaking change: Rename StatusLine to EditorStatusLine

0.4.1 - 29 June 2024
===================
- Add fuzz testing
- Fix several bugs that were discovered by fuzzing

0.4.0 - 27 June 2024
===================
- Bump ratatui to v0.27

0.3.5 - 4 May 2024
===================
- Map Redo from `r` to `<ctrl>+r`

0.3.4 - 4 May 2024
===================
- Fix panic when appending new line to empty buffer
 
0.3.3 - 2 April 2024
===================
- Fix panic in delete selection
- Add SelectLine
- Fix selection bug
- Render visual selection correctly
- Bump edtui-jagged to v0.1.3

0.3.2 - 18 February 2024
===================
- Use "ciw" for selection between delimiters instead of "cw".
- Minor bugfixes

0.3.1 - 11 February 2024
===================

- Paste over selection
- Support more motions in visual mode
- Add demo

0.3.0 - 3 February 2024
===================

Bump ratatui to version 0.26.0

0.2.2 - 3 February 2024
===================

- Fix bug in append string when no data was present
- Update default color scheme
- Add search functionality. Trigger search via '/'.
- Small breaking change in StatusLine widget. Replace 'content()' with 'mode()'.
- Bugfix: Fix panic in some cases when deleting selection
- Bugfix: Move left when cursor pos was larger than current col len

0.2.1 - 29 December 2023
===================

- Fix bug in WordBackward action
- Fix bug in RemoveChar action
- Fix bug in DeleteSelection action
 
0.2.0 - 25 December 2023
===================

- Add clipboard support
- Bump ratatui to version 0.25
- Refactor action enum
- Move jagged datatype into separate crate "edtui-jagged"
