#![allow(missing_docs)]
use gpui::{AnyView, DefiniteLength};

use super::button_like::{ButtonCommon, ButtonLike, ButtonSize, ButtonStyle};
use crate::{prelude::*, ElevationIndex, SelectableButton};
use crate::{IconName, IconSize};

use super::button_icon::ButtonIcon;

/// The shape of an [`IconButton`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum IconButtonShape {
    Square,
    Wide,
}

#[derive(IntoElement)]
pub struct IconButton {
    base: ButtonLike,
    shape: IconButtonShape,
    icon: IconName,
    icon_size: IconSize,
    icon_color: Color,
    selected_icon: Option<IconName>,
    alpha: Option<f32>,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        let mut this = Self {
            base: ButtonLike::new(id),
            shape: IconButtonShape::Wide,
            icon,
            icon_size: IconSize::default(),
            icon_color: Color::Default,
            selected_icon: None,
            alpha: None,
        };
        this.base.base = this.base.base.debug_selector(|| format!("ICON-{:?}", icon));
        this
    }

    pub fn shape(mut self, shape: IconButtonShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn icon_size(mut self, icon_size: IconSize) -> Self {
        self.icon_size = icon_size;
        self
    }

    pub fn icon_color(mut self, icon_color: Color) -> Self {
        self.icon_color = icon_color;
        self
    }

    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = Some(alpha);
        self
    }

    pub fn selected_icon(mut self, icon: impl Into<Option<IconName>>) -> Self {
        self.selected_icon = icon.into();
        self
    }
}

impl Disableable for IconButton {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl Toggleable for IconButton {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.base = self.base.toggle_state(selected);
        self
    }
}

impl SelectableButton for IconButton {
    fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.base = self.base.selected_style(style);
        self
    }
}

impl Clickable for IconButton {
    fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.base = self.base.on_click(handler);
        self
    }

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.base = self.base.cursor_style(cursor_style);
        self
    }
}

impl FixedWidth for IconButton {
    fn width(mut self, width: DefiniteLength) -> Self {
        self.base = self.base.width(width);
        self
    }

    fn full_width(mut self) -> Self {
        self.base = self.base.full_width();
        self
    }
}

impl ButtonCommon for IconButton {
    fn id(&self) -> &ElementId {
        self.base.id()
    }

    fn style(mut self, style: ButtonStyle) -> Self {
        self.base = self.base.style(style);
        self
    }

    fn size(mut self, size: ButtonSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.base = self.base.tooltip(tooltip);
        self
    }

    fn layer(mut self, elevation: ElevationIndex) -> Self {
        self.base = self.base.layer(elevation);
        self
    }
}

impl VisibleOnHover for IconButton {
    fn visible_on_hover(mut self, group_name: impl Into<SharedString>) -> Self {
        self.base = self.base.visible_on_hover(group_name);
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_disabled = self.base.disabled;
        let is_selected = self.base.selected;
        let selected_style = self.base.selected_style;

        let color = self.icon_color.color(cx).opacity(self.alpha.unwrap_or(1.0));
        self.base
            .map(|this| match self.shape {
                IconButtonShape::Square => {
                    let size = self.icon_size.square(window, cx);
                    this.width(size.into()).height(size.into())
                }
                IconButtonShape::Wide => this,
            })
            .child(
                ButtonIcon::new(self.icon)
                    .disabled(is_disabled)
                    .toggle_state(is_selected)
                    .selected_icon(self.selected_icon)
                    .when_some(selected_style, |this, style| this.selected_style(style))
                    .size(self.icon_size)
                    .color(Color::Custom(color)),
            )
    }
}
