pub struct SHPS {
    debug: bool,
    version: String,
}

impl SHPS {
    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    pub fn version(&self) -> &str {
        self.version.to_str()
    }
}
