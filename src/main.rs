
// task 1
fn double_int32(a: i32) -> i32{
    a * 2
}

// task 2
fn double_int64(a: i32) -> i64 {
    a as i64 * 2
}

// task 3
fn double_float32(a: f32) -> f32 {
    a * 2.0
}

// task 4
fn double_float64(a: f32) -> f64 {
    a as f64  * 2.0_f64
}

// task 5
fn int_plus_float_to_float(a: i32, b: f32) -> f64 {
    a as f64 + b as f64
}

// task 6
fn int_plus_float_to_int(a: i32, b: f32) -> i64 {
    a as i64 + b as i64
}

// task 7
fn tuple_sum(t: (i32, i32)) -> i32{
    t.0 + t.1
}

// task 8
fn array_sum(arr: &[i32; 3]) -> i32{
    arr.iter().sum()
}

// posible implementation, whoever what trait should be useded for T? 
// fn array_sum<T>(arr: &[T; 3]) -> T{
//     arr.into_iter().sum()
// }



fn main() {
    // test double_int32
    assert_eq!(double_int32(5), 10);

    // test doubel_int64
    assert_eq!(double_int64(5),  10_i64);

    // test double_float32
    assert_eq!(double_float32(5.0),  10.0);

    // test double_float64
    assert_eq!(double_float64(5.0),  10.0_f64);

    // test double_float64
    assert_eq!(int_plus_float_to_float(5, 5.0),  10.0_f64);

    // test double_float64
    assert_eq!(int_plus_float_to_int(5, 5.0),  10_i64);

    // test tuple_sum
    assert_eq!(tuple_sum((5,5)),  10);

    // test array-sum
    assert_eq!(array_sum(&[5,2,3]),  10);

    println!("All test done")


}