use trait_objects::{Button, Screen, Draw};

struct SelectBox {
    pub width: u8,
    pub height: u8,
    pub option: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        
    }
}

impl SelectBox {
    fn new() -> SelectBox {
        SelectBox {
            width: 0,
            height: 0,
            option: vec![
                String::from("Dum shit"),
                String::from("Dum shit"),
                String::from("Dum shit"),
                String::from("Dum shit")
            ]
        }
    }
}


fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox::new()),
            Box::new(Button::new())
        ]
    };

    screen.run();
}
