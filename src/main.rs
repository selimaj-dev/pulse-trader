use pulse_ui::layout::{LayoutItem, Rect, Size};

fn main() {
    let layout = LayoutItem::Columns {
        unit: Size::Percent(100),
        items: vec![
            LayoutItem::Widget(Size::Flex(16)),
            LayoutItem::Widget(Size::Flex(9)),
        ],
    };

    let screen = Rect {
        x: 0,
        y: 0,
        width: 3000,
        height: 3000,
    };

    let mut layout_alloc = Vec::new();

    layout.allocate(&screen, &mut layout_alloc);

    println!("{:?}", layout_alloc);
}
