pub trait ColumnVector {
    /// Interface that provides convienent accessor methods to avoid the need
    /// to case to a specific `FieldVector` implementation for each Arrow array type.
    fn get_type(&self);
    fn get_value(&self);
}