// Define the Schema trait
pub trait Schema {
    fn schema_validate(&self) -> Result<(), crate::SchemaError>;
    fn table_name() -> &'static str;
}
