use wasm_bindgen::JsCast;
use web_sys;

pub struct CookieAttributes {
    attr: String,
}

impl CookieAttributes {
    pub fn new() -> Self {
        CookieAttributes {
            attr: String::from(""),
        }
    }

    ///  ;domain=domain (e.g., example.com or subdomain.example.com): The host to which the cookie will be sent.
    /// If not specified, this defaults to the host portion of the current document location and the cookie is
    /// not available on subdomains. If a domain is specified, subdomains are always included. Contrary to earlier
    /// specifications, leading dots in domain names are ignored, but browsers may decline to set the cookie
    /// containing such dots.
    ///
    /// [Note]: The domain must match the domain of the JavaScript origin. Setting cookies to foreign domains
    /// will be silently ignored.
    pub fn _domain(&mut self, domain: &str) -> &mut Self {
        self.attr.push_str(&format!(";domain={}", domain));
        self
    }

    /// ;path=path: The value of the cookie's Path attribute.
    pub fn path(&mut self, path: &str) -> &mut Self {
        self.attr.push_str(&format!(";path={}", path));
        self
    }

    /// ;max-age=max-age-in-seconds: The maximum age of the cookie in seconds (e.g., 60*60*24*365 or 31536000 for a year).
    pub fn max_age(&mut self, max_age_in_seconds: u64) -> &mut Self {
        self.attr
            .push_str(&format!(";max-age={}", max_age_in_seconds));
        self
    }

    /// ;samesite: SameSite prevents the browser from sending this cookie along with cross-site requests.
    /// Possible values are lax, strict or none.
    ///
    /// - [x] The lax value will send the cookie for all same-site requests and top-level navigation GET requests.
    /// This is sufficient for user tracking, but it will prevent many Cross-Site Request Forgery (CSRF) attacks.
    /// This is the default value in modern browsers.
    /// - [ ] The strict value will prevent the cookie from being sent by the browser to the target site in all cross-site
    /// browsing contexts, even when following a regular link.
    /// - [ ] The none value explicitly states no restrictions will be applied. The cookie will be sent in all requests—both
    /// cross-site and same-site.
    pub fn _same_site_lax(&mut self) -> &mut Self {
        self.attr.push_str(";samesite=lax");
        self
    }

    /// ;samesite: SameSite prevents the browser from sending this cookie along with cross-site requests.
    /// Possible values are lax, strict or none.
    ///
    /// - [ ] The lax value will send the cookie for all same-site requests and top-level navigation GET requests.
    /// This is sufficient for user tracking, but it will prevent many Cross-Site Request Forgery (CSRF) attacks.
    /// This is the default value in modern browsers.
    /// - [x] The strict value will prevent the cookie from being sent by the browser to the target site in all cross-site
    /// browsing contexts, even when following a regular link.
    /// - [ ] The none value explicitly states no restrictions will be applied. The cookie will be sent in all requests—both
    /// cross-site and same-site.
    pub fn same_site_strict(&mut self) -> &mut Self {
        self.attr.push_str(";samesite=strict");
        self
    }

    /// ;samesite: SameSite prevents the browser from sending this cookie along with cross-site requests.
    /// Possible values are lax, strict or none.
    ///
    /// - [ ] The lax value will send the cookie for all same-site requests and top-level navigation GET requests.
    /// This is sufficient for user tracking, but it will prevent many Cross-Site Request Forgery (CSRF) attacks.
    /// This is the default value in modern browsers.
    /// - [ ] The strict value will prevent the cookie from being sent by the browser to the target site in all cross-site
    /// browsing contexts, even when following a regular link.
    /// - [x] The none value explicitly states no restrictions will be applied. The cookie will be sent in all requests—both
    /// cross-site and same-site.
    pub fn _same_site_none(&mut self) -> &mut Self {
        self.attr.push_str(";samesite=none");
        self
    }

    /// ;secure: Specifies that the cookie should only be transmitted over a secure protocol.
    pub fn secure(&mut self) -> &mut Self {
        self.attr.push_str(";secure");
        self
    }

    fn get_attributes_string(&self) -> &str {
        &self.attr
    }
}

pub fn _set_cookie(key: &str, value: &str) -> Result<(), ()> {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Ok(document) = document.dyn_into::<web_sys::HtmlDocument>() {
                let cookie_string = format!("{}={}", key, value);
                if document.set_cookie(&cookie_string).ok().is_some() {
                    return Ok(());
                }
            }
        }
    }

    Err(())
}

pub fn set_cookie_with_attributes(
    key: &str,
    value: &str,
    attr: &CookieAttributes,
) -> Result<(), ()> {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Ok(document) = document.dyn_into::<web_sys::HtmlDocument>() {
                let cookie_string = format!("{}={}{}", key, value, attr.get_attributes_string());
                if document.set_cookie(&cookie_string).ok().is_some() {
                    return Ok(());
                }
            }
        }
    }

    Err(())
}

pub fn get_cookie(key: &str) -> Option<String> {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Ok(document) = document.dyn_into::<web_sys::HtmlDocument>() {
                if let Ok(cookie) = document.cookie() {
                    let cookies: Vec<&str> = cookie.split(';').collect();
                    let prefix = format!("{}=", key);

                    for cookie in cookies {
                        let cookie = cookie.trim_start();
                        if let Some(val) = cookie.strip_prefix(&prefix) {
                            return Some(val.to_string());
                        }
                    }
                }
            }
        }
    }

    None
}
