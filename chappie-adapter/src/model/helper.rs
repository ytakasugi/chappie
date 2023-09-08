use chappie_kernel::model::Id;

pub fn convert_id_to_string<T>(id: Option<Id<T>>) -> Option<String> {
    id.map(|s| s.value.to_string())
}