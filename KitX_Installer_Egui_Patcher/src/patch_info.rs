use crate::patch_task::PatchTask;

pub struct PatchInfo {
    pub patch_file_path: Option<String>,
    pub patch_tasks: Vec<PatchTask>,
}

impl PatchInfo {
    pub fn default() -> PatchInfo {
        PatchInfo {
            patch_file_path: None,
            patch_tasks: Vec::new(),
        }
    }
}
