/// Enum reflection trait, automatically implemented by EnumReflect derive.
/// -
/// Dependency for [enum_reflect](https://crates.io/crates/enum_reflect/)
/// 
/// # Example Usage
/// 
/// ```
/// fn print_any_enum_fields(target_enum: impl EnumReflection) {
///     for (field, value) in target_enum.get_named_fields() {
///         println!("Field {}", field);
///     }
/// }
/// ```
/// Function which gets any enum with #[derive(EnumReflect)] and prints it fields.
pub trait EnumReflection {
    fn get_fields(&self) -> Vec<&dyn std::any::Any>;
    fn get_fields_mut(&mut self) -> Vec<&mut dyn std::any::Any>;
    fn get_named_fields(&self) -> Vec<(&'static str, &dyn std::any::Any)>;
    fn get_named_fields_mut(&mut self) -> Vec<(&'static str, &mut dyn std::any::Any)>;
}

pub trait InspectableEnum: 'static {}

#[deprecated(note = "Use EnumReflection instead, they are simillar")]
pub trait EnumReflect {
    fn get_fields(&self) -> Vec<&dyn std::any::Any>;
    fn get_fields_mut(&mut self) -> Vec<&mut dyn std::any::Any>;
    fn get_named_fields(&self) -> Vec<(&'static str, &dyn std::any::Any)>;
    fn get_named_fields_mut(&mut self) -> Vec<(&'static str, &mut dyn std::any::Any)>;
}
