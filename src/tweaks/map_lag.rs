use anyhow::{anyhow, Context};
use memory_rs::{
    generate_aob_pattern,
    internal::{
        injections::{Inject, Injection},
        memory_region::MemoryRegion,
    },
};

use super::{MemoryRegionExt, Tweak};

const VANILLA_SLEEP: u8 = 0x0A; // (10)
const DEFAULT_SLEEP: u8 = 0;

pub struct MapLagTweak {
    sleep: u8,
    sleep_injection: Injection,
}

impl MapLagTweak {
    pub fn new(region: &MemoryRegion) -> anyhow::Result<Self> {
        let sleep_addr = {
            // `Sleep(10)`
            #[rustfmt::skip]
            let memory_pattern = generate_aob_pattern![
                0xB9, VANILLA_SLEEP, 0x00, 0x00, 0x00, // MOV        param_1,0xa (10)
                0xFF, 0x15, _, _, _, _,                // CALL       qword ptr [->KERNEL32.DLL::Sleep]
                0x48                                   // MOV        ... (unimportant)
            ];
            region
                .scan_aob_single(&memory_pattern)
                .context(anyhow!("Error finding Sleep(10) addr"))?
        };

        let mut sleep_injection = Injection::new(sleep_addr + 1, vec![DEFAULT_SLEEP]);
        sleep_injection.inject();

        Ok(Self { sleep: DEFAULT_SLEEP, sleep_injection })
    }
}

impl Tweak for MapLagTweak {
    fn uninit(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn render(&mut self, ui: &hudhook::imgui::Ui) {
        ui.set_next_item_width(100.0);
        if ui.slider("Map Sleep (ms)", 0, VANILLA_SLEEP * 2, &mut self.sleep) {
            self.sleep_injection.f_new = vec![self.sleep];
            self.sleep_injection.inject();
        }
        if ui.is_item_hovered() {
            ui.tooltip_text(format!("Change the artificial delay in the map screen rendering\n(default: {DEFAULT_SLEEP}, vanilla: {VANILLA_SLEEP})"));
        }
    }

    fn reset_to_default(&mut self) {
        self.sleep = DEFAULT_SLEEP;
        self.sleep_injection.f_new = vec![self.sleep];
        self.sleep_injection.inject();
    }

    fn reset_to_vanilla(&mut self) {
        self.sleep = VANILLA_SLEEP;
        self.sleep_injection.f_new = vec![self.sleep];
        self.sleep_injection.inject();
    }
}
