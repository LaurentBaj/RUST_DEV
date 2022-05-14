#[derive(Debug)]
enum Message {
    Echo(String),
    Move(Point),
    ChangeColor((u8, u8, u8)),
    Quit,
}

#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants

        match message {
            Message::Quit => println!("{:?}", self.quit),
            Message::ChangeColor => println!("{:?}", self.color),
            Message::Echo => println!("{:?}", self.echo()),
            Message::Move => println!("{:?}", Move(self.position))
        }
    }
}

fn main() {
    test_match_message_call(); 
}

fn test_match_message_call() {
    let mut state = State {
        quit: false,
        position: Point { x: 0, y: 0 },
        color: (0, 0, 0),
    };
    state.process(Message::ChangeColor((255, 0, 255)));
    state.process(Message::Echo(String::from("hello world")));
    state.process(Message::Move(Point { x: 10, y: 15 }));
    state.process(Message::Quit);
}


