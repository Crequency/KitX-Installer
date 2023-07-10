pub struct PatchTask {
    pub patch_source: Vec<u8>,
    pub patch_target: Vec<u8>,
}

impl PatchTask {
    pub fn default() -> PatchTask {
        PatchTask {
            patch_source: Vec::new(),
            patch_target: Vec::new(),
        }
    }
}
