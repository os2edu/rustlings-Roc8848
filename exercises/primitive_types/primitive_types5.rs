/*
 * @Description: 
 * @Author: zheng pengfei
 * @Date: 2022-11-03 11:52:27
 * @LastEditTime: 2022-11-03 17:27:33
 * @LastEditors: zheng pengfei
 */
// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.


fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
