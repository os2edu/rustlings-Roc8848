/*
 * @Description: 
 * @Author: zheng pengfei
 * @Date: 2022-11-03 11:52:27
 * @LastEditTime: 2022-11-03 17:21:19
 * @LastEditors: zheng pengfei
 */
// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.


fn main() {
    let a = vec![1,2,3,4];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
