use std::arch::asm;

use anyhow::Context;
use memory_rs::generate_aob_pattern;

use super::{Defaults, InjectAt, ScanAOBSingleError, Tweak, TweakConfig};

const FAST_MENU_FADE_DEFAULTS: Defaults<bool> = Defaults::new(true, false);
const SKIP_LOAD_FINISH_DEFAULTS: Defaults<bool> = Defaults::new(true, false);

pub struct FastLoadingAnimationsTweak;

impl TweakConfig for FastLoadingAnimationsTweak {
    const CONFIG_ID: &'static str = "fast_loading_animations_tweak";
}

impl Tweak for FastLoadingAnimationsTweak {
    #[allow(clippy::too_many_lines)]
    fn new(builder: &mut super::TweakBuilder) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        builder.set_category(Some("Performance"));

        // --- main menu fading

        // ```
        // ... // some junk because I need more space to inject
        // menu_state.fade_cur++;
        // ```
        #[rustfmt::skip]
        let memory_pattern_1_12_6 = generate_aob_pattern![
            0x49, 0x8B, 0xD4,                        // MOV    RDX,R12 (unimportant)
            0xff, 0x90, 0xf0, 0x00, 0x00, 0x00,      // CALL   qword ptr [RAX + 0xf0] (unimportant)
            0x41, 0xff, 0x86, 0x80, 0xdb, 0x0b, 0x00 // INC    dword ptr [R14 + 0xbdb80] // <====== ! different on 1.12.6 vs 1.12.7
        ];

        // ```
        // ... // some junk because I need more space to inject
        // menu_state.fade_cur++;
        // ```
        #[rustfmt::skip]
        let memory_pattern = generate_aob_pattern![
            0x49, 0x8B, 0xD4,                        // MOV    RDX,R12 (unimportant)
            0xff, 0x90, 0xf0, 0x00, 0x00, 0x00,      // CALL   qword ptr [RAX + 0xf0] (unimportant)
            0x41, 0xff, 0x87, 0x90, 0xdb, 0x0b, 0x00 // INC    dword ptr [R15 + 0xbdb90] // <====== ! different on 1.12.6 vs 1.12.7
        ];

        let menu_fade_injection = builder
            .injection(
                &memory_pattern,
                {
                    // CALL custom_fade
                    let mut inject = vec![0xff, 0x15, 0x02, 0x00, 0x00, 0x00, 0xeb, 0x08];
                    inject.extend_from_slice(&(custom_fade as usize).to_le_bytes());
                    // pad with NOP
                    inject.resize(memory_pattern.size, 0x90);
                    inject
                },
                InjectAt::Start,
            )
            .or_else(|e| {
                if let Some(ScanAOBSingleError::NoMatches) = e.downcast_ref::<ScanAOBSingleError>() {
                    // fallback for 1.12.6
                    builder
                        .injection(
                            &memory_pattern_1_12_6,
                            {
                                // CALL custom_fade
                                let mut inject = vec![0xff, 0x15, 0x02, 0x00, 0x00, 0x00, 0xeb, 0x08];
                                inject.extend_from_slice(&(custom_fade_1_12_6 as usize).to_le_bytes());
                                // pad with NOP
                                inject.resize(memory_pattern_1_12_6.size, 0x90);
                                inject
                            },
                            InjectAt::Start,
                        ).context("(1.12.6 fallback)")
                } else {
                    Err(e).context("(no fallback)")
                }
            })
            .context("Error finding menu fade addr")?;

        builder
            .toggle("Fast Main Menu Fade", FAST_MENU_FADE_DEFAULTS)
            .tooltip("Speeds up the main menu loading fade")
            .config_key("fast_main_menu_fade")
            .injection(menu_fade_injection, false)
            .build()?;

        // --- skip loading wheel finish animation

        #[rustfmt::skip]
        let skip_load_finish_injection = builder.injection(
            // `&& (visual_progress == 1.0)`
            generate_aob_pattern![
                0x0f, 0x2e, 0xc6, // UCOMISS    XMM0,XMM6
                0x7a, 0x41,       // JP         +41
                0x75, 0x3f        // JNZ        +3f
            ],
            // NOP the JNZ
            vec![0x90, 0x90],
            InjectAt::End,
        ).context("Error finding skip load finish addr")?;

        builder
            .toggle("Skip Loading Finish Animation", SKIP_LOAD_FINISH_DEFAULTS)
            .tooltip("Skips the animation of the progress bar going to 100%")
            .config_key("skip_loading_finish_animation")
            .injection(skip_load_finish_injection, false)
            .build()?;

        Ok(Self)
    }
}

#[no_mangle]
extern "stdcall" fn custom_fade_1_12_6() {
    unsafe {
        asm!(
            "mov rdx,r12",                      // original code
            "call [rax + 0xf0]",                // original code
            "add dword ptr [r14 + 0xbdb80],15", // inc by 15 instead of 1
            options(nostack),
        );
    }
}

#[no_mangle]
extern "stdcall" fn custom_fade() {
    unsafe {
        asm!(
            "mov rdx,r12",                      // original code
            "call [rax + 0xf0]",                // original code
            "add dword ptr [r15 + 0xbdb90],15", // inc by 15 instead of 1
            options(nostack),
        );
    }
}
