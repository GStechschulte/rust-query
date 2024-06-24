use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use arrow::array::Int32Array;
use arrow_csv::ReaderBuilder;
// use arrow::csv::ReaderBuilder;
// use arrow::datatypes::{DataType, Field, Schema};
use arrow_schema::*;
use rust_query::arrow_types;

// Use our rust-query library
fn main() {
    let a = Int32Array::from(vec![10]);
    let b = arrow_types::Int8Type::from(vec![10]);

    println!("{:?}", a);
    println!("{:?}", b);

    // Code for the query `SELECT * FROM employee WHERE state = 'CO'`
    // against a CSV file containing the columns `id`, `first_name`,
    // `last_name`, `state`, `job_title`, and `salary`
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int8, false),
        Field::new("first_name", DataType::Utf8, false),
        Field::new("last_name", DataType::Utf8, false),
        Field::new("state", DataType::Utf8, true),
        Field::new("job_title", DataType::Utf8, false),
        Field::new("salary", DataType::Int16, false),
    ]);

    let path = Path::new("/Users/gabestechschulte/Documents/repos/rust-query/data/employee.csv");
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("Could not read {}: {}", display, why),
        Ok(file) => file,
    };

    let mut batch = ReaderBuilder::new(Arc::new(schema))
        .with_header(true)
        .build(file)
        .unwrap();
    let batch_reader = batch.next().unwrap().unwrap();

    assert_eq!(batch_reader.num_rows(), 4);
    assert_eq!(batch_reader.num_columns(), 6);

    let col_by_idx = batch_reader.column(2);
    let col_by_name = batch_reader.column_by_name("first_name");

    println!("{:?}", col_by_name);

    for idx in 0..batch_reader.num_columns() {
        let arr = batch_reader.column(idx).as_ref();

        // Apply filter using compute kernel
        // let filtered = filter
    }
}
