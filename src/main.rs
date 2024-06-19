use arrow::array::Int32Array;
use rust_query::arrow_types;

// Use our rust-query library
fn main() {

    let a = Int32Array::from(vec![10]);
    let b = arrow_types::Int8Type::from(vec![10]);

    println!("{:?}", a);
    println!("{:?}", b);
}