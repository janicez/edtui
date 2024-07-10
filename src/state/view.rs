use crate::Index2;

/// Represents the (x, y) offset of the editor's viewport.
/// It represents the top-left local editor coordinate.
#[derive(Default, Debug, Clone)]
pub(crate) struct ViewState {
    /// The x-coordinate offset of the viewport.
    viewport_x: usize,
    /// The y-coordinate offset of the viewport.
    viewport_y: usize,
    /// The horizontal offset of the editor's position on the screen.
    pub(crate) screen_x: usize,
    /// The vertical offset of the editor's position on the screen.
    pub(crate) screen_y: usize,
}

impl ViewState {
    /// Sets the editors position on the screen.
    ///
    /// Equivalent to the upper left coordinate of the editor in the
    /// global coordinate system.
    pub(crate) fn set_screen_offset<T: Into<usize>>(&mut self, x_offset: T, y_offset: T) {
        self.screen_x = x_offset.into();
        self.screen_y = y_offset.into();
    }

    /// Updates the view's offset and returns the new offset.
    /// This method is used internally to modify the view's offset coordinates.
    /// The given cursor coordinates are assumed to be in the editors absolute
    /// coordinates.
    pub(crate) fn update_viewport_offset(
        &mut self,
        size: (usize, usize),
        cursor: Index2,
    ) -> (usize, usize) {
        let limit = (
            size.0.saturating_sub(1) + self.viewport_x,
            size.1.saturating_sub(1) + self.viewport_y,
        );
        // scroll left
        if cursor.col < self.viewport_x {
            self.viewport_x = cursor.col;
        }
        // scroll right
        if cursor.col >= limit.0 {
            self.viewport_x += cursor.col.saturating_sub(limit.0);
        }
        // scroll up
        if cursor.row < self.viewport_y {
            self.viewport_y = cursor.row;
        }
        // scroll down
        if cursor.row >= limit.1 {
            self.viewport_y += cursor.row.saturating_sub(limit.1);
        }
        (self.viewport_x, self.viewport_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! update_view_offset_test {
        ($name:ident: {
        view: $given_view:expr,
        size: $given_size:expr,
        cursor: $given_cursor:expr,
        expected: $expected_offset:expr
    }) => {
            #[test]
            fn $name() {
                // given
                let mut view = $given_view;
                let size = $given_size;
                let cursor = $given_cursor;

                // when
                let offset = view.update_viewport_offset(size, cursor);

                // then
                assert_eq!(offset, $expected_offset);
            }
        };
    }

    update_view_offset_test!(
        // 0 <-   | --<-
        // 1 ---- | ----
        // 2 ---- |
        scroll_up: {
            view: ViewState{
                viewport_x: 0,
                viewport_y: 1,
                screen_x: 0,
                screen_y: 0,
            },
            size: (1, 2),
            cursor: Index2::new(0, 0),
            expected: (0, 0)
        }
    );

    update_view_offset_test!(
        // 0 ---- |
        // 1 ---- | ----
        // 2 <-   | --<-
        scroll_down: {
            view: ViewState{
                viewport_x: 0,
                viewport_y: 0,
                screen_x: 0,
                screen_y: 0,
            },
            size: (1, 2),
            cursor: Index2::new(2, 0),
            expected: (0, 1)
        }
    );

    update_view_offset_test!(
        scroll_left: {
            view: ViewState{
                viewport_x: 1,
                viewport_y: 0,
                screen_x: 0,
                screen_y: 0,
            },
            size: (2, 1),
            cursor: Index2::new(0, 0),
            expected: (0, 0)
        }
    );

    update_view_offset_test!(
        scroll_right: {
            view: ViewState{
                viewport_x: 0,
                viewport_y: 0,
                screen_x: 0,
                screen_y: 0,
            },
            size: (2, 1),
            cursor: Index2::new(0, 2),
            expected: (1, 0)
        }
    );
}
