use std::fmt::{self};

use hudhook::imgui;
use num_traits::ToBytes;

use crate::tweaks::{Defaults, NumberInjection, TweakBuilder};

use super::{Setting, SettingImpl};

pub struct SliderBuilder<'b, 'r, N: ToBytes> {
    tweak_builder: &'r mut TweakBuilder<'b>,
    defaults: Defaults<N>,
    slider: Slider<N>,
}

impl<
        'b,
        'r,
        N: ToBytes + Copy + fmt::Display + imgui::internal::DataTypeKind + Send + Sync + 'static,
    > SliderBuilder<'b, 'r, N>
{
    #[must_use]
    pub fn new(
        tweak_builder: &'r mut TweakBuilder<'b>,
        label: impl Into<String>,
        defaults: impl Into<Defaults<N>>,
        min: N,
        max: N,
    ) -> Self {
        let defaults = defaults.into();
        Self {
            tweak_builder,
            defaults,
            slider: Slider {
                min,
                max,
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
        self.slider.tooltip = tooltip;
        self
    }

    #[must_use]
    pub fn injection(mut self, injection: NumberInjection<N>) -> Self {
        self.slider.injections.push(injection);
        self
    }

    pub fn build(self) -> anyhow::Result<()> {
        self.tweak_builder
            .add_setting(Setting::new(self.slider, self.defaults))
    }
}

pub struct Slider<N: ToBytes> {
    min: N,
    max: N,
    label: String,
    tooltip: String,
    injections: Vec<NumberInjection<N>>,
}

impl<N: ToBytes + Copy + fmt::Display + imgui::internal::DataTypeKind> SettingImpl<N>
    for Slider<N>
{
    fn set(&mut self, value: N) -> anyhow::Result<()> {
        for injection in &mut self.injections {
            injection.inject(value);
        }

        Ok(())
    }

    fn render(
        &mut self,
        value: &mut N,
        defaults: &Defaults<N>,
        ui: &imgui::Ui,
    ) -> anyhow::Result<()> {
        ui.set_next_item_width(100.0);
        if ui.slider(&self.label, self.min, self.max, value) {
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
