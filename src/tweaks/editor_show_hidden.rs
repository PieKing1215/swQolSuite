use anyhow::Context;
use memory_rs::generate_aob_pattern;

use super::{Defaults, InjectAt, Tweak, TweakConfig};

const SHOW_HIDDEN_COMPONENTS_DEFAULTS: Defaults<bool> = Defaults::new(true, false);

pub struct ShowHiddenComponents;

impl TweakConfig for ShowHiddenComponents {
    const CONFIG_ID: &'static str = "show_hidden_components_tweak";
}

impl Tweak for ShowHiddenComponents {
    fn new(builder: &mut super::TweakBuilder) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        builder.set_category(Some("Editor"));

        // --- check 1

        #[rustfmt::skip]
        let injection_1 = builder.injection(
            // check for hidden flag
            generate_aob_pattern![
                0xf7, 0x86, 0xa0, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, // TEST       dword ptr [RSI + 0x2a0],0x20000000
                0x77, 0x67                                                  // JA
            ],
            // NOP the JA
            vec![0x90; 2],
            InjectAt::End,
        ).context("Error finding hidden component check 1 addr")?;

        // --- check 2

        #[rustfmt::skip]
        let injection_2 = builder.injection(
            // check for hidden flag
            generate_aob_pattern![
                0x8b, 0x86, 0xa0, 0x02, 0x00, 0x00, // MOV        EAX,dword ptr [RSI + 0x2a0]
                0xa9, 0x00, 0x00, 0x00, 0x20,       // TEST       EAX,0x20000000
                0x0f, 0x87, 0x14, 0x01, 0x00, 0x00  // JA
            ],
            // NOP the JA
            vec![0x90; 6],
            InjectAt::End,
        ).context("Error finding hidden component check 2 addr")?;

        builder
            .toggle("Show Hidden Components (reload save)", SHOW_HIDDEN_COMPONENTS_DEFAULTS)
            .tooltip("Forces editor to show components flagged as hidden.\nChanging this setting requires reloading your save to apply.")
            .config_key("show_hidden_components")
            .injection(injection_1, false)
            .injection(injection_2, false)
            .build()?;

        Ok(Self)
    }
}
