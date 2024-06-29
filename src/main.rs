use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use arrow::array::{ArrayRef, BooleanArray, Int16Array, Int32Array, Int64Array, StringArray};
use arrow::compute::filter;
use arrow_csv::ReaderBuilder;
// use arrow::csv::ReaderBuilder;
// use arrow::datatypes::{DataType, Field, Schema};
use arrow::error::Result;
// use arrow::record_batch::RecordBatch;
use arrow::array::RecordBatch;
use arrow_schema::*;
use rust_query::arrow_types;

fn filter_by_group(col_idx: usize, group: i64, batch: &RecordBatch) -> Result<RecordBatch> {
    let filter_array = batch
        .column(col_idx)
        .as_any()
        .downcast_ref::<Int64Array>()
        .unwrap()
        .iter()
        .map(|value| Some(value == Some(group)))
        .collect::<BooleanArray>();

    println!("{:?}", filter_array);

    let mut arrays: Vec<ArrayRef> = Vec::new();

    // Iterate over the columns and apply the filter
    for idx in 0..batch.num_columns() {
        let array = batch.column(idx).as_ref();
        let filtered = filter(array, &filter_array)?;
        arrays.push(filtered);
    }

    RecordBatch::try_new(batch.schema(), arrays)
}

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

    let col_by_idx = batch_reader.column(1);
    let col_by_name = batch_reader.column_by_name("first_name");
    println!("{:?}", col_by_idx);

    // StringArray
    println!("-- StringArray filter --");
    let string_filter = batch_reader
        .column(1)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .iter()
        .map(|value| Some(value == Some("Bill")))
        .collect::<BooleanArray>();

    println!("{:?}", string_filter);

    let str_col = batch_reader.column(1);
    let str_arr = filter(&str_col, &string_filter);
    println!("{:?}", str_arr);

    // PrimitiveArray<Int16> filter
    println!("-- PrimitiveArray filter --");
    let filter_array = batch_reader
        .column(5)
        .as_any()
        .downcast_ref::<Int16Array>()
        .unwrap()
        .iter()
        .map(|value| Some(value == Some(10000)))
        .collect::<BooleanArray>();

    println!("{:?}", filter_array);

    let my_col = batch_reader.column(5);
    let c = filter(&my_col, &filter_array).unwrap();
    println!("{:?}", c);

    // let filter_array = batch
    //     .column(col_idx)
    //     .as_any()
    //     .downcast_ref::<Int64Array>()
    //     .unwrap()
    //     .iter()
    //     .map(|value| Some(value == Some(group)))
    //     .collect::<BooleanArray>();

    // let filtered_arr = filter_by_group(5, 10_000, &batch_reader);
    // println!("{:?}", filtered_arr);

    // for idx in 0..batch_reader.num_columns() {
    // let arr = batch_reader.column(idx).as_ref();

    // Apply filter using compute kernel
    // let filtered = filter
}
