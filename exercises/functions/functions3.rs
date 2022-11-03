/*
 * @Description: 
 * @Author: zheng pengfei
 * @Date: 2022-11-02 08:07:49
 * @LastEditTime: 2022-11-02 08:18:59
 * @LastEditors: zheng pengfei
 */
// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.


fn main() {
    let c = 10;
    call_me(c);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
