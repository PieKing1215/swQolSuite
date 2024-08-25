use anyhow::Context;
use memory_rs::{
    generate_aob_pattern,
    internal::{
        injections::{Inject, Injection},
        memory_region::MemoryRegion,
    },
};

use super::{MemoryRegionExt, Tweak};

const VANILLA_SUPPORT_CHECK: bool = true;
const DEFAULT_SUPPORT_CHECK: bool = false;

const VANILLA_MERGE_CHECK: bool = true;
const DEFAULT_MERGE_CHECK: bool = false;

pub struct EditorPlacementTweak {
    disable_support_check: bool,
    disable_support_check_inject: Injection,
    disable_merge_check: bool,
    disable_merge_check_inject: Injection,
}

impl EditorPlacementTweak {
    pub fn new(region: &MemoryRegion) -> anyhow::Result<Self> {
        // --- support check

        // The start of the function that determines if a component placement has support
        #[rustfmt::skip]
        let memory_pattern = generate_aob_pattern![
            0x4c, 0x8b, 0xdc,                        // MOV      R11,RSP
            0x49, 0x89, 0x5b, 0x10,                  // MOV      qword ptr [R11 + local_res10],RBX
            0x49, 0x89, 0x6b, 0x18,                  // MOV      qword ptr [R11 + local_res18],RBP
            0x56,                                    // PUSH     RSI
            0x57,                                    // PUSH     RDI
            0x41, 0x54,                              // PUSH     R12
            0x41, 0x56,                              // PUSH     R14
            0x41, 0x57,                              // PUSH     R15
            0x48, 0x81, 0xec, 0xb0, 0x00, 0x00, 0x00 // SUB      RSP,0xb0
        ];
        let support_addr = {
            region
                .scan_aob_single(&memory_pattern)
                .context("Error finding placement support check function")?
        };

        // basically just early exit `return true`
        #[rustfmt::skip]
        let inject = vec![
            0xB0, 0x01, // MOV  al,01
            0xC3,       // RET
        ];

        let mut disable_support_check_inject = Injection::new(support_addr, inject);

        if !DEFAULT_SUPPORT_CHECK {
            disable_support_check_inject.inject();
        }

        // --- merge check

        // The start of the function that determines if a merge is valid
        #[rustfmt::skip]
        let memory_pattern = generate_aob_pattern![
            0x48, 0x8b, 0xc4,                        // MOV        RAX,RSP
            0x4c, 0x89, 0x40, 0x18,                  // MOV        qword ptr [RAX + local_res18],R8
            0x48, 0x89, 0x50, 0x10,                  // MOV        qword ptr [RAX + local_res10],RDX
            0x48, 0x89, 0x48, 0x08,                  // MOV        qword ptr [RAX + local_res8],RCX
            0x55,                                    // PUSH       RBP
            0x56,                                    // PUSH       RSI
            0x57,                                    // PUSH       RDI
            0x41, 0x54,                              // PUSH       R12
            0x48, 0x81, 0xec, 0xf8, 0x00, 0x00, 0x00 // SUB        RSP,0xf8
        ];
        let merge_addr = {
            region
                .scan_aob_single(&memory_pattern)
                .context("Error finding merge check function")?
        };

        // basically just early exit `return true`
        let inject = vec![
            0xB0, 0x01, // MOV  al,01
            0xC3, // RET
        ];

        let mut disable_merge_check_inject = Injection::new(merge_addr, inject);

        if !DEFAULT_MERGE_CHECK {
            disable_merge_check_inject.inject();
        }

        Ok(Self {
            disable_support_check: !DEFAULT_SUPPORT_CHECK,
            disable_support_check_inject,
            disable_merge_check: !DEFAULT_MERGE_CHECK,
            disable_merge_check_inject,
        })
    }

    fn set_support_check(&mut self, enabled: bool) {
        self.disable_support_check = !enabled;

        if self.disable_support_check {
            self.disable_support_check_inject.inject();
        } else {
            self.disable_support_check_inject.remove_injection();
        }
    }

    fn set_merge_check(&mut self, enabled: bool) {
        self.disable_merge_check = !enabled;

        if self.disable_merge_check {
            self.disable_merge_check_inject.inject();
        } else {
            self.disable_merge_check_inject.remove_injection();
        }
    }
}

impl Tweak for EditorPlacementTweak {
    fn uninit(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn render(&mut self, ui: &hudhook::imgui::Ui) {
        if ui.checkbox(
            "Disable Placement Support Check",
            &mut self.disable_support_check,
        ) {
            self.set_support_check(!self.disable_support_check);
        }
        if ui.is_item_hovered() {
            ui.tooltip_text(format!("Normally many parts (eg. pipes) need support on a side in order to place them.\nThis disables that check.\n(default: {}, vanilla: {})", !DEFAULT_SUPPORT_CHECK, !VANILLA_SUPPORT_CHECK));
        }

        if ui.checkbox("Disable Merge Check", &mut self.disable_merge_check) {
            self.set_merge_check(!self.disable_merge_check);
        }
        if ui.is_item_hovered() {
            ui.tooltip_text(format!("Normally two subgrids must be touching in order to merge them.\nThis disables that check.\n(default: {}, vanilla: {})", !DEFAULT_MERGE_CHECK, !VANILLA_MERGE_CHECK));
        }
    }

    fn reset_to_default(&mut self) {
        self.set_support_check(DEFAULT_SUPPORT_CHECK);
        self.set_merge_check(DEFAULT_MERGE_CHECK);
    }

    fn reset_to_vanilla(&mut self) {
        self.set_support_check(VANILLA_SUPPORT_CHECK);
        self.set_merge_check(VANILLA_MERGE_CHECK);
    }
}
