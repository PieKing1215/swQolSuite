use anyhow::{anyhow, Context};
use memory_rs::{
    generate_aob_pattern,
    internal::{
        injections::{Inject, Injection},
        memory_region::MemoryRegion,
    },
};

use super::{MemoryRegionExt, Tweak};

const VANILLA_DEV_MODE: bool = false;
const DEFAULT_DEV_MODE: bool = false;

pub struct DevModeTweak {
    dev_mode: bool,
    dev_mode_injection: Injection,
}

impl DevModeTweak {
    pub fn new(region: &MemoryRegion) -> anyhow::Result<Self> {
        let dev_mode_addr = {
            // dev id check
            #[rustfmt::skip]
            let memory_pattern = generate_aob_pattern![
                0x48, 0x39, 0x34, 0xd0, // CMP        qword ptr [RAX + RDX*0x8],RSI
                0x74, _                 // JZ
            ];
            region
                .scan_aob_single(&memory_pattern)
                .context(anyhow!("Error finding dev mode addr"))?
        };

        // replace the JZ with JNZ
        let inject = vec![0x75];
        let mut dev_mode_injection = Injection::new(dev_mode_addr + 4, inject);

        if DEFAULT_DEV_MODE {
            dev_mode_injection.inject();
        }

        Ok(Self { dev_mode: DEFAULT_DEV_MODE, dev_mode_injection })
    }

    fn set_dev_mode(&mut self, enabled: bool) {
        self.dev_mode = enabled;

        if self.dev_mode {
            self.dev_mode_injection.inject();
        } else {
            self.dev_mode_injection.remove_injection();
        }
    }
}

impl Tweak for DevModeTweak {
    fn uninit(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn render(&mut self, ui: &hudhook::imgui::Ui) {
        if ui.checkbox("Dev Mode (reload main menu)", &mut self.dev_mode) {
            self.set_dev_mode(self.dev_mode);
        }
        if ui.is_item_hovered() {
            ui.tooltip_text(format!("Enables developer tools on the main menu.\nOpen a save then quit to menu to reload.\n(default: {DEFAULT_DEV_MODE}, vanilla: {VANILLA_DEV_MODE})"));
        }
    }

    fn reset_to_default(&mut self) {
        self.set_dev_mode(DEFAULT_DEV_MODE);
    }

    fn reset_to_vanilla(&mut self) {
        self.set_dev_mode(VANILLA_DEV_MODE);
    }
}
