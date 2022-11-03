/*
 * @Description: 
 * @Author: zheng pengfei
 * @Date: 2022-10-25 16:51:10
 * @LastEditTime: 2022-11-01 22:20:28
 * @LastEditors: zheng pengfei
 */
// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.


fn main() {
    call_me(3);
}

fn call_me(num:u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
