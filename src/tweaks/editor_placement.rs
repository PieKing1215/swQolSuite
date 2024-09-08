use anyhow::Context;
use memory_rs::generate_aob_pattern;

use super::{Defaults, InjectAt, Tweak};

const SUPPORT_CHECK_DEFAULTS: Defaults<bool> = Defaults::new(true, false);
const MERGE_CHECK_DEFAULTS: Defaults<bool> = Defaults::new(true, false);

pub struct EditorPlacementTweak;

impl Tweak for EditorPlacementTweak {
    fn new(builder: &mut super::TweakBuilder) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        builder.set_category(Some("Editor"));

        // --- support check

        #[rustfmt::skip]
        let disable_support_check_inject = builder.injection(
            // The start of the function that determines if a component placement has support
            generate_aob_pattern![
                0x4c, 0x8b, 0xdc,                        // MOV      R11,RSP
                0x49, 0x89, 0x5b, 0x10,                  // MOV      qword ptr [R11 + local_res10],RBX
                0x49, 0x89, 0x6b, 0x18,                  // MOV      qword ptr [R11 + local_res18],RBP
                0x56,                                    // PUSH     RSI
                0x57,                                    // PUSH     RDI
                0x41, 0x54,                              // PUSH     R12
                0x41, 0x56,                              // PUSH     R14
                0x41, 0x57,                              // PUSH     R15
                0x48, 0x81, 0xec, 0xb0, 0x00, 0x00, 0x00 // SUB      RSP,0xb0
            ],
            // basically just early exit `return true`
            vec![
                0xB0, 0x01, // MOV  al,01
                0xC3,       // RET
            ],
            InjectAt::Start,
        ).context("Error finding placement support check function")?;

        builder
            .toggle("Disable Placement Support Check", SUPPORT_CHECK_DEFAULTS)
            .tooltip("Normally many parts (eg. pipes) need support on a side in order to place them.\nThis disables that check.")
            .injection(disable_support_check_inject, false)
            .build()?;

        // --- merge check

        #[rustfmt::skip]
        let disable_merge_check_inject = builder.injection(
            // The start of the function that determines if a merge is valid
            generate_aob_pattern![
                0x48, 0x8b, 0xc4,                        // MOV        RAX,RSP
                0x4c, 0x89, 0x40, 0x18,                  // MOV        qword ptr [RAX + local_res18],R8
                0x48, 0x89, 0x50, 0x10,                  // MOV        qword ptr [RAX + local_res10],RDX
                0x48, 0x89, 0x48, 0x08,                  // MOV        qword ptr [RAX + local_res8],RCX
                0x55,                                    // PUSH       RBP
                0x56,                                    // PUSH       RSI
                0x57,                                    // PUSH       RDI
                0x41, 0x54,                              // PUSH       R12
                0x48, 0x81, 0xec, 0xf8, 0x00, 0x00, 0x00 // SUB        RSP,0xf8
            ],
            // basically just early exit `return true`
            vec![
                0xB0, 0x01, // MOV  al,01
                0xC3, // RET
            ],
            InjectAt::Start,
        ).context("Error finding merge check function")?;

        builder
            .toggle("Disable Merge Check", MERGE_CHECK_DEFAULTS)
            .tooltip("Normally two subgrids must be touching in order to merge them.\nThis disables that check.")
            .injection(disable_merge_check_inject, false)
            .build()?;

        Ok(Self)
    }
}
