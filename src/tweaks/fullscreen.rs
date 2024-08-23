use anyhow::{anyhow, Context};
use hudhook::windows::Win32::UI::WindowsAndMessaging::{HWND_NOTOPMOST, WS_POPUP};
use memory_rs::{
    generate_aob_pattern,
    internal::{
        injections::{Inject, Injection},
        memory_region::MemoryRegion,
    },
};

use super::{MemoryRegionExt, Tweak};

const VANILLA_NO_MINIMIZE: bool = false;
const DEFAULT_NO_MINIMIZE: bool = true;
const VANILLA_BORDERLESS: bool = false;
const DEFAULT_BORDERLESS: bool = true;

pub struct FullscreenTweak {
    no_minimize: bool,
    borderless: bool,
    no_minimize_injection: Injection,
    no_topmost_injection: Injection,
    borderless_injection: Injection,
}

impl FullscreenTweak {
    pub fn new(region: &MemoryRegion) -> anyhow::Result<Self> {
        // remove auto minimize

        // code that minimizes the window on lost focus
        let memory_pattern = generate_aob_pattern![
            0x48, 0x8b, 0x8e, 0x48, 0x03, 0x00,
            0x00, // MOV        RCX,qword ptr [RSI + 0x348]
            0xba, 0x06, 0x00, 0x00, 0x00, // MOV        EDX,0x6
            0xff, 0x15, _, _, _, _ // CALL       qword ptr [->USER32.DLL::ShowWindow]
        ];

        let check_addr = {
            region
                .scan_aob_single(&memory_pattern)
                .context(anyhow!("Error finding minimize addr"))?
        };

        // NOP the ShowWindow call
        let inject = vec![0x90; 6];
        let mut no_minimize_injection =
            Injection::new(check_addr + memory_pattern.size - inject.len(), inject);

        // remove forced top level

        // hWndInsertAfter arg for SetWindowPos
        let memory_pattern = generate_aob_pattern![
            0x8b, _, 0xe0, // MOV        param_1,dword ptr [RBP + local_54[12]]
            0x48, 0xc7, _, 0xff, 0xff, 0xff, 0xff // MOV        param_2,-0x1   (HWND_TOPMOST)
        ];

        let check_addr = {
            region
                .scan_aob_single(&memory_pattern)
                .context(anyhow!("Error finding topmost arg addr"))?
        };

        // change HWND_TOPMOST to HWND_NOTOPMOST
        let inject = HWND_NOTOPMOST.0.to_le_bytes()[0..4].to_vec();
        let mut no_topmost_injection =
            Injection::new(check_addr + memory_pattern.size - inject.len(), inject);

        // force borderless

        // the MOV is flags for SetWindowLongW
        let memory_pattern = generate_aob_pattern![
            _, 0x83, _, _, _, // CMP (unimportant)
            0x74, _, // JZ (unimportant)
            0xbb, 0x00, 0x00, 0x00, 0x86 // MOV        EBX,10000110000000000000000000000000b
        ];

        let borderless_addr = {
            region
                .scan_aob_single(&memory_pattern)
                .context(anyhow!("Error finding topmost arg addr"))?
        };

        // remove WS_POPUP flag
        let inject = (0x86000000 & !WS_POPUP.0).to_le_bytes().to_vec();
        let mut borderless_injection =
            Injection::new(borderless_addr + memory_pattern.size - inject.len(), inject);

        if DEFAULT_NO_MINIMIZE {
            no_minimize_injection.inject();
            no_topmost_injection.inject();
        }

        if DEFAULT_BORDERLESS {
            borderless_injection.inject();
        }

        Ok(Self {
            no_minimize: DEFAULT_NO_MINIMIZE,
            borderless: DEFAULT_BORDERLESS,
            no_minimize_injection,
            no_topmost_injection,
            borderless_injection,
        })
    }

    fn set_no_minimize(&mut self, enabled: bool) {
        self.no_minimize = enabled;

        if self.no_minimize {
            self.no_minimize_injection.inject();
            self.no_topmost_injection.inject();
        } else {
            self.no_minimize_injection.remove_injection();
            self.no_topmost_injection.remove_injection();
        }
    }

    fn set_borderless(&mut self, enabled: bool) {
        self.borderless = enabled;

        if self.borderless {
            self.borderless_injection.inject();
        } else {
            self.borderless_injection.remove_injection();
        }
    }
}

impl Tweak for FullscreenTweak {
    fn uninit(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn render(&mut self, ui: &hudhook::imgui::Ui) {
        if ui.checkbox("Force Borderless Fullscreen", &mut self.borderless) {
            self.set_borderless(self.borderless);
        }
        if ui.is_item_hovered() {
            ui.tooltip_text(format!("Forces the window to open in borderless fullscreen instead of exclusive.\nYou need to toggle fullscreen for it to update.\n(default: {DEFAULT_BORDERLESS}, vanilla: {VANILLA_BORDERLESS})"));
        }

        if ui.checkbox("Disable Minimize on Lost Focus", &mut self.no_minimize) {
            self.set_no_minimize(self.no_minimize);
        }
        if ui.is_item_hovered() {
            ui.tooltip_text(format!("Prevents the window from automatically minimizing when you tab out in fullscreen.\nTurn fullscreen off and back on while enabled to fix window stuck on top.\n(default: {DEFAULT_NO_MINIMIZE}, vanilla: {VANILLA_NO_MINIMIZE})"));
        }
    }

    fn reset_to_default(&mut self) {
        self.set_no_minimize(DEFAULT_NO_MINIMIZE);
        self.set_borderless(DEFAULT_BORDERLESS);
    }

    fn reset_to_vanilla(&mut self) {
        self.set_no_minimize(VANILLA_NO_MINIMIZE);
        self.set_borderless(VANILLA_BORDERLESS);
    }
}
