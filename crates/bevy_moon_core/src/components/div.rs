use bevy_color::{Alpha, Color, palettes::css::BLACK};
use bevy_ecs::{component::Component, prelude::ReflectComponent};
use bevy_math::Vec2;
use bevy_reflect::{Reflect, prelude::ReflectDefault};
use bevy_transform::components::Transform;
use taffy::*;

use super::computed::ComputedLayout;
use crate::{
    measure::NodeContext,
    style::{BoxShadow, Corners},
};

#[derive(Component, Clone, Debug, Reflect)]
#[require(Transform, ComputedLayout)]
#[reflect(Component, Clone, Debug, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct Div {
    pub stack_index: usize,

    #[reflect(ignore, clone)]
    pub(crate) style: Style,

    #[reflect(ignore, clone)]
    pub measure: Option<NodeContext>,

    pub background: Option<Color>,
    pub corner_radii: Corners<f32>,
    pub border_color: Option<Color>,
    pub box_shadow: Option<Vec<BoxShadow>>,
}

unsafe impl Send for Div {}
unsafe impl Sync for Div {}

impl Default for Div {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Div {
    pub const DEFAULT: Self = Self {
        style: Style::DEFAULT,
        stack_index: 0,
        measure: None,
        background: None,
        corner_radii: Corners::DEFAULT,
        border_color: None,
        box_shadow: None,
    };

    // Display

    pub fn flex(mut self) -> Self {
        self.style.display = Display::Flex;
        self
    }

    pub fn block(mut self) -> Self {
        self.style.display = Display::Block;
        self
    }

    pub fn grid(mut self) -> Self {
        self.style.display = Display::Grid;
        self
    }

    pub fn hidden(mut self) -> Self {
        self.style.display = Display::None;
        self
    }

    pub fn relative(mut self) -> Self {
        self.style.position = Position::Relative;
        self
    }

    pub fn absolute(mut self) -> Self {
        self.style.position = Position::Absolute;
        self
    }

    // Flex

    /// Sets flex direction to row (horizontal)
    pub fn flex_row(mut self) -> Self {
        self.style.display = Display::Flex;
        self.style.flex_direction = FlexDirection::Row;
        self
    }

    /// Sets flex direction to column (vertical)
    pub fn flex_col(mut self) -> Self {
        self.style.display = Display::Flex;
        self.style.flex_direction = FlexDirection::Column;
        self
    }

    /// Sets flex direction to row-reverse
    pub fn flex_row_reverse(mut self) -> Self {
        self.style.display = Display::Flex;
        self.style.flex_direction = FlexDirection::RowReverse;
        self
    }

    /// Sets flex direction to column-reverse
    pub fn flex_col_reverse(mut self) -> Self {
        self.style.display = Display::Flex;
        self.style.flex_direction = FlexDirection::ColumnReverse;
        self
    }

    /// Sets flex-grow to 1 (element will grow to fill space)
    pub fn flex_grow(mut self) -> Self {
        self.style.flex_grow = 1.0;
        self
    }

    /// Sets flex-grow to a specific value
    ///
    /// Uses this for proportional sizing. For example, an element with flex_grow_value(2.0)
    /// will grow to twice the size of an element with flex_grow_value(1.0).
    pub fn flex_grow_value(mut self, value: f32) -> Self {
        self.style.flex_grow = value;
        self
    }

    /// Sets flex-shrink to 1 (element will shrink if needed)
    pub fn flex_shrink(mut self) -> Self {
        self.style.flex_shrink = 1.0;
        self
    }

    /// Sets flex-shrink to 0 (element won't shrink)
    pub fn flex_shrink_0(mut self) -> Self {
        self.style.flex_shrink = 0.0;
        self
    }

    /// Sets flex-basis to auto
    pub fn flex_auto(mut self) -> Self {
        self.style.flex_grow = 1.0;
        self.style.flex_shrink = 1.0;
        self.style.flex_basis = Dimension::auto();
        self
    }

    /// Sets flex: 1 1 0% (grow, shrink, basis 0)
    pub fn flex_1(mut self) -> Self {
        self.style.flex_grow = 1.0;
        self.style.flex_shrink = 1.0;
        self.style.flex_basis = Dimension::length(0.0);
        self
    }

    /// Allows wrapping
    pub fn flex_wrap(mut self) -> Self {
        self.style.flex_wrap = FlexWrap::Wrap;
        self
    }

    // Sizing

    /// Sets width with a given absolute length.
    pub fn w(mut self, val: f32) -> Self {
        self.style.size.width = Dimension::length(val);
        self
    }

    /// Sets width with given a percentage length.
    pub fn w_p(mut self, val: f32) -> Self {
        self.style.size.width = Dimension::percent(val);
        self
    }

    /// Sets width to 100%.
    pub fn w_full(mut self) -> Self {
        self.style.size.width = Dimension::percent(1.0);
        self
    }

    /// Sets width to auto.
    pub fn w_auto(mut self) -> Self {
        self.style.size.width = Dimension::auto();
        self
    }

    /// Sets height with a given absolute length.
    pub fn h(mut self, val: f32) -> Self {
        self.style.size.height = Dimension::length(val);
        self
    }

    /// Sets height with given a percentage length.
    pub fn h_p(mut self, val: f32) -> Self {
        self.style.size.height = Dimension::percent(val);
        self
    }

    /// Set height to 100%
    pub fn h_full(mut self) -> Self {
        self.style.size.height = Dimension::percent(1.0);
        self
    }

    /// Set height to auto
    pub fn h_auto(mut self) -> Self {
        self.style.size.height = Dimension::auto();
        self
    }

    // Colors

    // Sets background color
    pub fn background(mut self, color: impl Into<Color>) -> Self {
        self.background = Some(color.into());
        self
    }

    #[inline]
    pub const fn corner_radii(mut self, radii: Corners<f32>) -> Self {
        self.corner_radii = radii;
        self
    }

    // Sets border color
    pub fn border_color(mut self, color: impl Into<Color>) -> Self {
        self.border_color = Some(color.into());
        self
    }

    // Sets border widths
    #[inline]
    pub const fn border(mut self, border: Rect<LengthPercentage>) -> Self {
        self.style.border = border;
        self
    }

    // Box Shadows

    // Sets box shadows
    pub fn shadow(mut self, shadows: Vec<BoxShadow>) -> Self {
        self.box_shadow = Some(shadows);
        self
    }

    pub fn shadow_none(mut self) -> Self {
        self.box_shadow = None;
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    pub fn shadow_2xs(mut self) -> Self {
        self.box_shadow = Some(vec![BoxShadow {
            color: BLACK.with_alpha(0.05).into(),
            offset: Vec2::new(0.0, 1.0),
            blur_radius: 0.0,
            spread_radius: 0.0,
        }]);
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    pub fn shadow_xs(mut self) -> Self {
        self.box_shadow = Some(vec![BoxShadow {
            color: BLACK.with_alpha(0.05).into(),
            offset: Vec2::new(0.0, 1.0),
            blur_radius: 2.0,
            spread_radius: 0.0,
        }]);
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    pub fn shadow_sm(mut self) -> Self {
        self.box_shadow = Some(vec![
            BoxShadow {
                color: BLACK.with_alpha(0.1).into(),
                offset: Vec2::new(0.0, 1.0),
                blur_radius: 3.0,
                spread_radius: 0.0,
            },
            BoxShadow {
                color: BLACK.with_alpha(0.1).into(),
                offset: Vec2::new(0.0, 1.0),
                blur_radius: 2.0,
                spread_radius: -1.0,
            },
        ]);
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    pub fn shadow_md(mut self) -> Self {
        self.box_shadow = Some(vec![
            BoxShadow {
                color: BLACK.with_alpha(0.1).into(),
                offset: Vec2::new(0.0, 4.0),
                blur_radius: 6.0,
                spread_radius: -1.0,
            },
            BoxShadow {
                color: BLACK.with_alpha(0.1).into(),
                offset: Vec2::new(0.0, 20.0),
                blur_radius: 4.0,
                spread_radius: -2.0,
            },
        ]);
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    pub fn shadow_lg(mut self) -> Self {
        self.box_shadow = Some(vec![
            BoxShadow {
                color: BLACK.with_alpha(0.1).into(),
                offset: Vec2::new(0.0, 10.0),
                blur_radius: 15.0,
                spread_radius: -3.0,
            },
            BoxShadow {
                color: BLACK.with_alpha(0.1).into(),
                offset: Vec2::new(0.0, 4.0),
                blur_radius: 6.0,
                spread_radius: -4.0,
            },
        ]);
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    pub fn shadow_xl(mut self) -> Self {
        self.box_shadow = Some(vec![
            BoxShadow {
                color: BLACK.with_alpha(0.1).into(),
                offset: Vec2::new(0.0, 20.0),
                blur_radius: 25.0,
                spread_radius: -5.0,
            },
            BoxShadow {
                color: BLACK.with_alpha(0.1).into(),
                offset: Vec2::new(0.0, 8.0),
                blur_radius: 10.0,
                spread_radius: -6.0,
            },
        ]);
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    pub fn shadow_2xl(mut self) -> Self {
        self.box_shadow = Some(vec![BoxShadow {
            color: BLACK.with_alpha(0.25).into(),
            offset: Vec2::new(0.0, 25.0),
            blur_radius: 50.0,
            spread_radius: -12.0,
        }]);
        self
    }
}

pub fn div() -> Div {
    Div::DEFAULT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_builder() {
        let d = div().flex();

        assert_eq!(d.style.display, Display::Flex);

        let d = d.block();

        assert_eq!(d.style.display, Display::Block);
    }

    #[test]
    fn test_div_sizing() {
        let d = div().w(100.0);

        assert_eq!(d.style.size.width, Dimension::length(100.0));

        let d = d.h_full();

        assert_eq!(d.style.size.height, Dimension::percent(1.0));
    }
}
