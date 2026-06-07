pub(crate) fn require_trimmed<'a>(
    value: &'a str,
    error: &'static str,
) -> Result<&'a str, &'static str> {
    let value = value.trim();
    if value.is_empty() {
        Err(error)
    } else {
        Ok(value)
    }
}

pub(crate) fn validate_scene_code(scene: &str) -> Result<(), &'static str> {
    if scene.len() > 128 {
        return Err("video generation scene must be at most 128 characters");
    }
    if scene.bytes().all(|byte| {
        byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'@' | b'-')
    }) {
        Ok(())
    } else {
        Err("video generation scene must use visible code characters")
    }
}

pub(crate) fn normalized_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

pub(crate) fn normalize_provider_code_for_storage(value: &str) -> String {
    value.trim().replace('_', "-").to_ascii_lowercase()
}

pub(crate) fn normalize_operation_code(value: &str) -> String {
    value.trim().replace('-', "_").to_ascii_lowercase()
}

pub(crate) fn stable_identifier_suffix(value: &str) -> String {
    let normalized = value
        .trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();
    let suffix = normalized
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    let suffix = suffix.chars().take(80).collect::<String>();
    if suffix.is_empty() {
        "unknown".to_string()
    } else {
        suffix
    }
}
