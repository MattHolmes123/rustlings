// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit
}

impl Message {
    fn call(&self) {
        println!("********************");
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move{ x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit
    ];

    for message in &messages {
        message.call();

        // Playing around with matches
        match message {
            Message::Move {x, y} => {
                println!("{:?}", x);
                println!("{:?}", y);
            }
            Message::Echo(msg) => {
                println!("{:?}", msg);
            }
            Message::ChangeColor(r, g, b) => {
                println!("RGB: {:?} {:?} {:?}", r, g, b);
            }
            Message::Quit => {
                println!("Better be off then.")
            }
        }

        // Playing around with if let.
        if let Message::Move {x, y} = message {
            println!("This is if let matching Message::Move");
            println!("{:?}", x);
            println!("{:?}", y);
        }
    }
}
