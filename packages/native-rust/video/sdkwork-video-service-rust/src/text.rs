pub(crate) fn require_trimmed_owned(
    value: String,
    error: &'static str,
) -> Result<String, &'static str> {
    let value = value.trim().to_string();
    if value.is_empty() {
        Err(error)
    } else {
        Ok(value)
    }
}
