/*
 * @Description: 
 * @Author: zheng pengfei
 * @Date: 2022-11-03 11:52:27
 * @LastEditTime: 2022-11-03 17:26:37
 * @LastEditors: zheng pengfei
 */
// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4]; 

    assert_eq!([2, 3, 4], nice_slice)
}
