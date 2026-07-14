pub mod layout;
pub mod render;
pub mod unit;
pub mod widget;

pub trait App {
    // State
    fn init(&mut self);
    fn update(&mut self);
    fn tick(&self);

    // Rendering
    fn layout(&self) -> layout::Allocation;
    fn render(&mut self, layout: layout::Allocation);

    fn refresh(&mut self) {
        self.render(self.layout());
    }

    // Running
    fn run(&mut self) {
        self.init();
        self.refresh();

        loop {
            self.tick();
            self.update();
            self.refresh();
        }
    }
}
