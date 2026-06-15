pub(crate) fn infer_video_mime_type_from_url(value: Option<&str>) -> Option<String> {
    let value = value?
        .trim()
        .split(['?', '#'])
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    if value.ends_with(".webm") {
        return Some("video/webm".to_string());
    }
    if value.ends_with(".mov") {
        return Some("video/quicktime".to_string());
    }
    if value.ends_with(".m3u8") {
        return Some("application/vnd.apple.mpegurl".to_string());
    }
    if value.ends_with(".mp4") {
        return Some("video/mp4".to_string());
    }
    None
}

pub(crate) fn video_file_extension_for_mime(value: Option<&str>) -> &'static str {
    match value
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "video/webm" => "webm",
        "video/quicktime" => "mov",
        "application/vnd.apple.mpegurl" => "m3u8",
        _ => "mp4",
    }
}

pub(crate) fn i64_to_i32(value: Option<i64>) -> Option<i32> {
    value.and_then(|value| i32::try_from(value).ok())
}
