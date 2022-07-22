pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box::<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u8,
    pub height: u8,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {

    }
}

impl Button {
    pub fn new() -> Button {
        Button {
           width: 0,
           height: 0,
           label: String::from("Dum shit")
        }
    }
}

