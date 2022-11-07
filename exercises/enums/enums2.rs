/*
 * @Description: 
 * @Author: zheng pengfei
 * @Date: 2022-11-03 11:52:26
 * @LastEditTime: 2022-11-07 10:10:54
 * @LastEditors: zheng pengfei
 */
// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.


#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Quit,
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
