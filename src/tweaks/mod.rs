use memory_rs::internal::memory_region::MemoryRegion;

pub mod editor_camera_speed;
pub mod editor_placement;
pub mod editor_show_hidden;
pub mod loading;
pub mod map_lag;
pub mod no_minimize_on_lost_focus;

pub trait Tweak {
    fn uninit(&mut self) -> anyhow::Result<()>;

    fn render(&mut self, ui: &hudhook::imgui::Ui);

    fn constant_render(&mut self, _ui: &hudhook::imgui::Ui) {}

    fn reset_to_default(&mut self);
    fn reset_to_vanilla(&mut self);
}

#[derive(Debug)]
pub enum ScanAOBSingleError {
    Error(anyhow::Error),
    NoMatches,
    MultipleMatches,
}

pub trait MemoryRegionExt {
    fn scan_aob_single(
        &self,
        pat: &memory_rs::internal::memory::MemoryPattern,
    ) -> anyhow::Result<usize>;
}

impl MemoryRegionExt for MemoryRegion {
    fn scan_aob_single(
        &self,
        pat: &memory_rs::internal::memory::MemoryPattern,
    ) -> anyhow::Result<usize> {
        let matches = self.scan_aob_all_matches(pat)?;

        match matches.len() {
            0 => anyhow::bail!("No Matches"),
            1 => Ok(matches[0]),
            _ => anyhow::bail!("Multiple Matches"),
        }
    }
}
