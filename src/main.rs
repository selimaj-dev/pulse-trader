use pulse_ui::layout::{LayoutItem, Rect, Size, layout};

fn main() {
    let layout = layout(vec![
        LayoutItem::Widget(Size::Flex(16)),
        LayoutItem::Widget(Size::Flex(9)),
    ]);

    let screen = Rect {
        x: 0,
        y: 0,
        width: 3000,
        height: 3000,
    };

    println!("{:?}", screen.allocate(&layout));
}
