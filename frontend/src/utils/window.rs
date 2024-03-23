use web_sys::window;

pub fn get_current_host() -> Option<String> {
    if let Some(window) = window() {
        if let Ok(location) = window.location().host() {
            return Some(location);
        }
    }
    None
}
