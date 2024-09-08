use memory_rs::internal::injections::{Inject, Injection};

use crate::tweaks::{Defaults, TweakBuilder};

use super::{Setting, SettingImpl};

pub struct ToggleBuilder<'b, 'r> {
    tweak_builder: &'r mut TweakBuilder<'b>,
    defaults: Defaults<bool>,
    toggle: Toggle,
}

impl<'b, 'r> ToggleBuilder<'b, 'r> {
    #[must_use]
    pub fn new(
        tweak_builder: &'r mut TweakBuilder<'b>,
        label: impl Into<String>,
        defaults: impl Into<Defaults<bool>>,
    ) -> Self {
        let defaults = defaults.into();
        Self {
            tweak_builder,
            defaults,
            toggle: Toggle {
                tooltip: String::new(),
                label: label.into(),
                injections: vec![],
            },
        }
    }

    #[must_use]
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        let mut tooltip: String = tooltip.into();
        if !tooltip.ends_with('\n') {
            tooltip = format!("{tooltip}\n");
        }
        self.toggle.tooltip = tooltip;
        self
    }

    #[must_use]
    pub fn injection(mut self, injection: Injection, invert: bool) -> Self {
        self.toggle.injections.push((injection, invert));
        self
    }

    pub fn build(self) -> anyhow::Result<()> {
        self.tweak_builder
            .add_setting(Setting::new(self.toggle, self.defaults))
    }
}

pub struct Toggle {
    label: String,
    tooltip: String,
    injections: Vec<(Injection, bool)>,
}

impl SettingImpl<bool> for Toggle {
    fn set(&mut self, value: bool) -> anyhow::Result<()> {
        for (injection, invert) in &mut self.injections {
            #[allow(clippy::collapsible_else_if)]
            if value {
                if *invert {
                    injection.remove_injection();
                } else {
                    injection.inject();
                }
            } else {
                if *invert {
                    injection.inject();
                } else {
                    injection.remove_injection();
                }
            }
        }

        Ok(())
    }

    fn render(
        &mut self,
        value: &mut bool,
        defaults: &Defaults<bool>,
        ui: &hudhook::imgui::Ui,
    ) -> anyhow::Result<()> {
        if ui.checkbox(&self.label, value) {
            self.set(*value)?;
        }
        if ui.is_item_hovered() {
            ui.tooltip_text(format!(
                "{}(default: {}, vanilla: {})",
                self.tooltip, defaults.default, defaults.vanilla
            ));
        }

        Ok(())
    }
}
