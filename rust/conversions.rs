//! Owned conversions not expressible through the current projected From/Cow surfaces.
//! No routing, HTTP policy, source inspection, or request extraction belongs here.

pub fn body_from_text(text: String) -> axum_core::body::Body {
    axum_core::body::Body::from(text)
}

pub fn buffer_into_text(buffer: axum::body::Bytes) -> Option<String> {
    String::from_utf8(buffer.into()).ok()
}

pub fn decode_path_component(value: String) -> Option<String> {
    match urlencoding::decode(&value) {
        Ok(std::borrow::Cow::Borrowed(_)) => Some(value),
        Ok(std::borrow::Cow::Owned(decoded)) => Some(decoded),
        Err(_) => None,
    }
}
