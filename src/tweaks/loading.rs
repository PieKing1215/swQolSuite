use std::arch::asm;

use anyhow::{anyhow, Context};
use memory_rs::{
    generate_aob_pattern,
    internal::{
        injections::{Inject, Injection},
        memory_region::MemoryRegion,
    },
};

use super::{MemoryRegionExt, Tweak};

const VANILLA_FAST_MENU_FADE: bool = false;
const DEFAULT_FAST_MENU_FADE: bool = true;

const VANILLA_SKIP_LOAD_FINISH: bool = false;
const DEFAULT_SKIP_LOAD_FINISH: bool = true;

pub struct LoadingTweak {
    fast_menu_fade: bool,
    fast_menu_fade_injection: Injection,
    skip_load_finish: bool,
    skip_load_finish_injection: Injection,
}

impl LoadingTweak {
    pub fn new(region: &MemoryRegion) -> anyhow::Result<Self> {
        // --- main menu fading

        // ```
        // ... // some junk because I need more space to inject
        // menu_state.fade_cur++;
        // ```
        #[rustfmt::skip]
        let memory_pattern = generate_aob_pattern![
            0x49, 0x8B, 0xD4,                        // MOV    RDX,R12 (unimportant)
            0xff, 0x90, 0xf0, 0x00, 0x00, 0x00,      // CALL   qword ptr [RAX + 0xf0] (unimportant)
            0x41, 0xff, 0x86, 0x80, 0xdb, 0x0b, 0x00 // INC    dword ptr [R14 + 0xbdb80]
        ];

        let menu_fade_addr = {
            region
                .scan_aob_single(&memory_pattern)
                .context(anyhow!("Error finding menu fade addr"))?
        };

        // CALL custom_fade
        let mut inject = vec![0xff, 0x15, 0x02, 0x00, 0x00, 0x00, 0xeb, 0x08];
        inject.extend_from_slice(&(custom_fade as usize).to_le_bytes());
        // pad with NOP
        inject.resize(memory_pattern.size, 0x90);

        let mut menu_fade_injection = Injection::new(menu_fade_addr, inject);

        if DEFAULT_FAST_MENU_FADE {
            menu_fade_injection.inject();
        }

        // --- skip loading wheel finish animation

        // `&& (visual_progress == 1.0)`
        #[rustfmt::skip]
        let memory_pattern = generate_aob_pattern![
            0x0f, 0x2e, 0xc6, // UCOMISS    XMM0,XMM6
            0x7a, 0x41,       // JP         +41
            0x75, 0x3f        // JNZ        +3f
        ];

        let skip_load_finish_addr = {
            region
                .scan_aob_single(&memory_pattern)
                .context(anyhow!("Error finding skip load finish addr"))?
        };

        let mut skip_load_finish_injection = Injection::new(
            skip_load_finish_addr + memory_pattern.size - 2,
            vec![0x90, 0x90],
        );

        if DEFAULT_SKIP_LOAD_FINISH {
            skip_load_finish_injection.inject();
        }

        Ok(Self {
            fast_menu_fade: DEFAULT_FAST_MENU_FADE,
            fast_menu_fade_injection: menu_fade_injection,
            skip_load_finish: DEFAULT_SKIP_LOAD_FINISH,
            skip_load_finish_injection,
        })
    }

    fn set_fast_menu_fade(&mut self, enabled: bool) {
        self.fast_menu_fade = enabled;

        if self.fast_menu_fade {
            self.fast_menu_fade_injection.inject();
        } else {
            self.fast_menu_fade_injection.remove_injection();
        }
    }

    fn set_skip_load_finish(&mut self, enabled: bool) {
        self.skip_load_finish = enabled;

        if self.skip_load_finish {
            self.skip_load_finish_injection.inject();
        } else {
            self.skip_load_finish_injection.remove_injection();
        }
    }
}

impl Tweak for LoadingTweak {
    fn uninit(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn render(&mut self, ui: &hudhook::imgui::Ui) {
        if ui.checkbox("Fast Main Menu Fade", &mut self.fast_menu_fade) {
            self.set_fast_menu_fade(self.fast_menu_fade);
        }
        if ui.is_item_hovered() {
            ui.tooltip_text(format!("Speeds up the main menu loading fade\n(default: {DEFAULT_FAST_MENU_FADE}, vanilla: {VANILLA_FAST_MENU_FADE})"));
        }

        if ui.checkbox("Skip Loading Finish Animation", &mut self.skip_load_finish) {
            self.set_skip_load_finish(self.skip_load_finish);
        }
        if ui.is_item_hovered() {
            ui.tooltip_text(format!("Skips the animation of the progress bar going to 100%\n(default: {DEFAULT_SKIP_LOAD_FINISH}, vanilla: {VANILLA_SKIP_LOAD_FINISH})"));
        }
    }

    fn reset_to_default(&mut self) {
        self.set_fast_menu_fade(DEFAULT_FAST_MENU_FADE);
        self.set_skip_load_finish(DEFAULT_SKIP_LOAD_FINISH);
    }

    fn reset_to_vanilla(&mut self) {
        self.set_fast_menu_fade(VANILLA_FAST_MENU_FADE);
        self.set_skip_load_finish(VANILLA_SKIP_LOAD_FINISH);
    }
}

#[no_mangle]
extern "stdcall" fn custom_fade() {
    unsafe {
        asm!(
            "mov rdx,r12",                      // original code
            "call [rax + 0xf0]",                // original code
            "add dword ptr [r14 + 0xbdb80],15", // inc by 15 instead of 1
            options(nostack),
        );
    }
}
