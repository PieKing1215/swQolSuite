use serde::{Deserialize, Serialize};

use super::Defaults;

pub mod slider;
pub mod toggle;

pub trait SettingUntyped {
    fn render(&mut self, ui: &hudhook::imgui::Ui) -> anyhow::Result<()>;
    fn reset_to_default(&mut self) -> anyhow::Result<()>;
    fn reset_to_vanilla(&mut self) -> anyhow::Result<()>;
    fn load_config(&mut self, value: &toml::value::Table) -> anyhow::Result<()>;
    fn save_config(&self, into: &mut toml::value::Table) -> anyhow::Result<()>;
}

pub struct Setting<T: Copy> {
    value: T,
    defaults: Defaults<T>,
    config_key: Option<String>,
    inner: Box<dyn SettingImpl<T> + Send + Sync>,
}

impl<T: Copy> Setting<T> {
    fn new(inner: impl SettingImpl<T> + Send + Sync + 'static, defaults: Defaults<T>, config_key: Option<String>) -> Self {
        Self {
            value: defaults.default,
            defaults,
            config_key,
            inner: Box::new(inner) as _,
        }
    }

    fn set(&mut self, value: T) -> anyhow::Result<()> {
        self.value = value;
        self.inner.set(value)
    }
}

impl<T: Copy + PartialEq + Serialize + for<'a> Deserialize<'a>> SettingUntyped for Setting<T> {
    fn render(&mut self, ui: &hudhook::imgui::Ui) -> anyhow::Result<()> {
        self.inner.render(&mut self.value, &self.defaults, ui)
    }

    fn reset_to_default(&mut self) -> anyhow::Result<()> {
        self.set(self.defaults.default)
    }

    fn reset_to_vanilla(&mut self) -> anyhow::Result<()> {
        self.set(self.defaults.vanilla)
    }
    
    fn load_config(&mut self, value: &toml::value::Table) -> anyhow::Result<()> {
        if let Some(key) = &self.config_key {
            if let Some(value) = value.get(key) {
                self.set(toml::Value::try_into(value.clone())?)?;
            }
        }

        Ok(())
    }
    
    fn save_config(&self, into: &mut toml::value::Table) -> anyhow::Result<()>{
        if let Some(key) = &self.config_key {
            into.insert(key.clone(), toml::Value::try_from(self.value)?);
        }

        Ok(())
    }
}

pub trait SettingImpl<T> {
    fn set(&mut self, value: T) -> anyhow::Result<()>;
    fn render(
        &mut self,
        _value: &mut T,
        _defaults: &Defaults<T>,
        _ui: &hudhook::imgui::Ui,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
