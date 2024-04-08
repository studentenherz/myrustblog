use web_sys::window;

use crate::utils::cookies::get_cookie;

pub fn get_current_host() -> Option<String> {
    if let Some(window) = window() {
        if let Ok(location) = window.location().host() {
            return Some(location);
        }
    }
    None
}

pub fn is_loged_in() -> bool {
    get_cookie("_token").is_some()
}

pub fn loged_in_as() -> Option<String> {
    get_cookie("_username")
}
