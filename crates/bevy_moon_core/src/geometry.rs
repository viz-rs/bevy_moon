/// Converts a taffy unit to a bevy unit.
pub trait Convert<T> {
    #[must_use]
    fn convert(self) -> T;
}

/// Utilities for working with Taffy geometry.
pub mod taffy {
    use bevy_math::{Vec2, Vec4};
    use taffy::{Point, Rect, Size};

    use super::Convert;

    impl Convert<Vec2> for Point<f32> {
        fn convert(self) -> Vec2 {
            Vec2::new(self.x, self.y)
        }
    }

    impl Convert<Vec2> for Size<f32> {
        fn convert(self) -> Vec2 {
            Vec2::new(self.width, self.height)
        }
    }

    impl Convert<Vec4> for Rect<f32> {
        fn convert(self) -> Vec4 {
            Vec4::new(self.top, self.right, self.bottom, self.left)
        }
    }
}
