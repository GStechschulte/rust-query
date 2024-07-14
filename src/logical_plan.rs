struct LogicalPlan;

impl LogicalPlan {
    fn schema(&self) -> Schema {
        // Returns the schema of the data that will be prduced by this
        // logical plan
    }

    fn children(&self) -> Vec<&str> {
        // Returns the children (inputs) of this logical plan.
        // This method is used to enable use of the vistor pattern
        // to walk a query tree
    }
}
