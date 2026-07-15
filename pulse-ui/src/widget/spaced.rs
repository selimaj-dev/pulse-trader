use crate::{layout::LayoutItem, unit::Size::Percent, widget::Widget};

pub struct SpacedRows(pub Vec<(LayoutItem, Box<dyn Widget>)>);

impl Widget for SpacedRows {
    fn render(&self, scope: &mut crate::render::RenderScope) {
        let mut items = Vec::new();

        for (i, (item, _)) in self.0.iter().enumerate() {
            if i > 0 {
                items.push(LayoutItem::Spacing(crate::unit::Size::Fixed(1)))
            }

            items.push(item.clone());
        }

        let mut alloc = scope.rect.allocate(&crate::layout::LayoutItem::Rows {
            unit: Percent(100),
            items,
        });

        for i in 0..alloc.widgets.len() {
            let item = &self.0[i];

            if i > 0 {
                let rect = &mut alloc.widgets[i];

                scope.draw_text((0, rect.y - 2), "─".repeat(rect.width as usize));
            }

            alloc.draw(i, &*item.1);
        }
    }
}

pub struct SpacedColumns(pub Vec<(LayoutItem, Box<dyn Widget>)>);

impl Widget for SpacedColumns {
    fn render(&self, scope: &mut crate::render::RenderScope) {
        let mut alloc = scope.rect.allocate(&crate::layout::LayoutItem::Columns {
            unit: Percent(100),
            items: self.0.iter().map(|v| v.0.clone()).collect(),
        });

        for i in 0..alloc.widgets.len() {
            let item = &self.0[i];

            if i > 0 {
                let rect = &mut alloc.widgets[i];

                for i in 0..rect.height {
                    scope.draw_text((rect.x - 1, i), "│");
                }
                rect.x += 1;
                rect.width -= 1;
            }

            alloc.draw(i, &*item.1);
        }
    }
}
