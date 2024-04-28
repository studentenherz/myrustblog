use web_sys::window;

pub fn get_current_host() -> Option<String> {
    if let Some(window) = window() {
        if let Ok(location) = window.location().host() {
            return Some(location);
        }
    }
    None
}

pub fn set_title(title: &str) -> bool {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            document.set_title(title);
            return true;
        }
    }

    false
}
