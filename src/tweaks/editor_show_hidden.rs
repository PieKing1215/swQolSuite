use anyhow::{anyhow, Context};
use memory_rs::{
    generate_aob_pattern,
    internal::{
        injections::{Inject, Injection},
        memory_region::MemoryRegion,
    },
};

use super::{MemoryRegionExt, Tweak};

const VANILLA_SHOW_HIDDEN_COMPONENTS: bool = false;
const DEFAULT_SHOW_HIDDEN_COMPONENTS: bool = true;

pub struct ShowHiddenComponents {
    show_hidden_components: bool,
    injection_1: Injection,
    injection_2: Injection,
}

impl ShowHiddenComponents {
    pub fn new(region: &MemoryRegion) -> anyhow::Result<Self> {
        // --- check 1

        // check for hidden flag
        #[rustfmt::skip]
        let memory_pattern = generate_aob_pattern![
            0xf7, 0x86, 0xa0, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, // TEST       dword ptr [RSI + 0x2a0],0x20000000
            0x77, 0x67                                                  // JA
        ];

        let check_addr = {
            region
                .scan_aob_single(&memory_pattern)
                .context(anyhow!("Error finding menu fade addr"))?
        };

        // NOP the JA
        let inject = vec![0x90; 2];
        let mut injection_1 =
            Injection::new(check_addr + memory_pattern.size - inject.len(), inject);

        if DEFAULT_SHOW_HIDDEN_COMPONENTS {
            injection_1.inject();
        }

        // --- check 2

        // check for hidden flag
        #[rustfmt::skip]
        let memory_pattern = generate_aob_pattern![
            0x8b, 0x86, 0xa0, 0x02, 0x00, 0x00, // MOV        EAX,dword ptr [RSI + 0x2a0]
            0xa9, 0x00, 0x00, 0x00, 0x20,       // TEST       EAX,0x20000000
            0x0f, 0x87, 0x14, 0x01, 0x00, 0x00  // JA
        ];

        let check_addr = {
            region
                .scan_aob_single(&memory_pattern)
                .context(anyhow!("Error finding menu fade addr"))?
        };

        // NOP the JA
        let inject = vec![0x90; 6];
        let mut injection_2 =
            Injection::new(check_addr + memory_pattern.size - inject.len(), inject);

        if DEFAULT_SHOW_HIDDEN_COMPONENTS {
            injection_2.inject();
        }

        Ok(Self {
            show_hidden_components: DEFAULT_SHOW_HIDDEN_COMPONENTS,
            injection_1,
            injection_2,
        })
    }

    fn set_show_hidden_components(&mut self, enabled: bool) {
        self.show_hidden_components = enabled;

        if self.show_hidden_components {
            self.injection_1.inject();
            self.injection_2.inject();
        } else {
            self.injection_1.remove_injection();
            self.injection_2.remove_injection();
        }
    }
}

impl Tweak for ShowHiddenComponents {
    fn uninit(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn render(&mut self, ui: &hudhook::imgui::Ui) {
        if ui.checkbox(
            "Show Hidden Components (reload save)",
            &mut self.show_hidden_components,
        ) {
            self.set_show_hidden_components(self.show_hidden_components);
        }
        if ui.is_item_hovered() {
            ui.tooltip_text(format!("Forces editor to show components flagged as hidden.\nChanging this setting requires reloading your save to apply.\n(default: {DEFAULT_SHOW_HIDDEN_COMPONENTS}, vanilla: {VANILLA_SHOW_HIDDEN_COMPONENTS})"));
        }
    }

    fn reset_to_default(&mut self) {
        self.set_show_hidden_components(DEFAULT_SHOW_HIDDEN_COMPONENTS);
    }

    fn reset_to_vanilla(&mut self) {
        self.set_show_hidden_components(VANILLA_SHOW_HIDDEN_COMPONENTS);
    }
}
