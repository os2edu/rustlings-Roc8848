/*
 * @Description: 
 * @Author: zheng pengfei
 * @Date: 2022-11-03 11:52:27
 * @LastEditTime: 2022-11-07 21:21:46
 * @LastEditors: zheng pengfei
 */
// modules1.rs
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a hint.


mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
