/*
 * @Description: 
 * @Author: zheng pengfei
 * @Date: 2022-11-03 11:52:27
 * @LastEditTime: 2022-11-08 22:46:00
 * @LastEditors: zheng pengfei
 */
// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
