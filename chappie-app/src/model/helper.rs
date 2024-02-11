use chappie_kernel::model::Id;
use chrono::NaiveDate;

pub fn convert_str_to_date(str_date: String) -> anyhow::Result<NaiveDate> {
    let parsed_date = NaiveDate::parse_from_str(str_date.as_str(), "%Y-%m-%d")?;
    Ok(parsed_date)
}

pub fn convert_string_to_id<T>(id: Option<String>) -> Option<Id<T>> {
    id.and_then(|id| id.try_into().ok())
}
