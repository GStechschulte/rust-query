/// The first step in building a query engine is to choose a type system to
/// represent the different types of data that the query engine will be
/// processing.
///
/// Apache Arrow is used as the basis for the type system in this project and
/// the types defined in this module are aliases for Arrow array types.
use arrow::array::{
    Float32Array, Float64Array, Int16Array, Int32Array, Int64Array, Int8Array, StringArray,
    UInt16Array, UInt32Array, UInt64Array, UInt8Array,
};

// TODO: Use an enum here
pub type Int8Type = Int8Array;
pub type Int16Type = Int16Array;
pub type Int32Type = Int32Array;
pub type Int64Type = Int64Array;

pub type UInt8Type = UInt8Array;
pub type UInt16Type = UInt16Array;
pub type UInt32Type = UInt32Array;
pub type UInt64Type = UInt64Array;

pub type Float32Type = Float32Array;
pub type Float64Type = Float64Array;

pub type StringType = StringArray;
