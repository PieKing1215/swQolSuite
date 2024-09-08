use anyhow::Context;
use memory_rs::generate_aob_pattern;

use super::{Defaults, InjectAt, Tweak};

const SLEEP_DEFAULTS: Defaults<u8> = Defaults::new(0, 0x0A); // (0x0A == 10)

pub struct MapLagTweak;

impl Tweak for MapLagTweak {
    fn new(builder: &mut super::TweakBuilder) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        builder.set_category(Some("Performance"));

        #[rustfmt::skip]
        let sleep_injection = builder.number_injection(
            // `Sleep(10)`
            generate_aob_pattern![
                0xB9, 0x0A, 0x00, 0x00, 0x00, // MOV        param_1,0xa (10)
                0xFF, 0x15, _, _, _, _,       // CALL       qword ptr [->KERNEL32.DLL::Sleep]
                0x48                          // MOV        ... (unimportant)
            ],
            InjectAt::StartOffset(1),
        ).context("Error finding Sleep(10) addr")?;

        builder
            .slider(
                "Map Sleep (ms)",
                SLEEP_DEFAULTS,
                0,
                SLEEP_DEFAULTS.vanilla * 2,
            )
            .tooltip("Change the artificial delay in the map screen rendering")
            .injection(sleep_injection)
            .build()?;

        Ok(Self)
    }
}
