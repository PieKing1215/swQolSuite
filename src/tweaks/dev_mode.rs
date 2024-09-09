use anyhow::Context;
use memory_rs::generate_aob_pattern;

use super::{Defaults, InjectAt, Tweak, TweakConfig};

const DEV_MODE_DEFAULTS: Defaults<bool> = Defaults::new(false, false);

pub struct DevModeTweak;

impl TweakConfig for DevModeTweak {
    const CONFIG_ID: &'static str = "dev_mode_tweak";
}

impl Tweak for DevModeTweak {
    fn new(builder: &mut super::TweakBuilder) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        #[rustfmt::skip]
        let inject = builder.injection(
            // dev id check
            generate_aob_pattern![
                0x48, 0x39, 0x34, 0xd0, // CMP        qword ptr [RAX + RDX*0x8],RSI
                0x74, _                 // JZ
            ],
            // replace the JZ with JNZ
            vec![0x75],
            InjectAt::StartOffset(4),
        ).context("Error finding dev mode addr")?;

        builder
            .toggle("Dev Mode (reload main menu)", DEV_MODE_DEFAULTS)
            .tooltip("Enables developer tools on the main menu.\nOpen a save then quit to menu to reload.")
            .config_key("dev_mode")
            .injection(inject, false)
            .build()?;

        Ok(Self)
    }
}
