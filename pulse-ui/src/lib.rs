pub mod layout;
pub mod render;
pub mod unit;
pub mod widget;

pub trait App {
    // Rendering
    fn layout(&self) -> layout::Allocation;
    fn render(&mut self, layout: layout::Allocation);

    // State
    fn init(&mut self);

    fn update(&mut self);

    fn refresh(&mut self) {
        self.render(self.layout());
    }

    // Run
    fn run(&mut self) {
        self.init();
        self.refresh();
    }
}
