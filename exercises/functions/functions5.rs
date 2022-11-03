/*
 * @Description: 
 * @Author: zheng pengfei
 * @Date: 2022-11-02 08:07:49
 * @LastEditTime: 2022-11-02 08:22:34
 * @LastEditors: zheng pengfei
 */
// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)


fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
