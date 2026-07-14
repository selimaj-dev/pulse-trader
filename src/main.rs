use pulse_ui::layout::{LayoutItem, Rect, Size};

fn main() {
    let layout = LayoutItem::Row {
        width: Size::Percent(100),
        height: Size::Percent(100),
        items: vec![
            LayoutItem::Widget {
                width: Size::Percent(100),
                height: Size::Flex(1),
            },
            LayoutItem::Widget {
                width: Size::Percent(100),
                height: Size::Flex(1),
            },
        ],
    };

    let screen = Rect {
        x: 0,
        y: 0,
        width: 60,
        height: 12,
    };

    let mut alloc = Vec::new();

    layout.allocate(&screen, &mut alloc);

    println!("{:?}", alloc);
}
