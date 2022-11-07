/*
 * @Description: 
 * @Author: zheng pengfei
 * @Date: 2022-11-03 11:52:26
 * @LastEditTime: 2022-11-07 10:07:03
 * @LastEditors: zheng pengfei
 */
// enums1.rs
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
    
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
