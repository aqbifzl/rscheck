pub struct CheckStats {
    pub files_checked: u32,
    pub dirs_checked: u32,
    pub typos_num: u64,
    pub errors: u32,
}

impl Default for CheckStats {
    fn default() -> Self {
        Self::new()
    }
}

impl CheckStats {
    pub fn new() -> Self {
        Self {
            files_checked: 0,
            dirs_checked: 0,
            typos_num: 0,
            errors: 0,
        }
    }
}
