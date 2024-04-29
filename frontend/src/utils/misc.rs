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

pub fn set_description_meta_tag(description: &str) -> bool {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(head) = document.head() {
                if let Ok(node) = document.create_element("meta") {
                    let _ = node.set_attribute("name", "description");
                    let _ = node.set_attribute("content", description);
                    return head.append_child(&node).is_ok();
                }
            }
        }
    }

    false
}
