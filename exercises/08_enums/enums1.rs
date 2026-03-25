#[derive(Debug)]
enum Message {  
    Resize,
    Quit,
    Move,
    ChangeColor,
    Echo,
    
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
