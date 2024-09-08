use super::Defaults;

pub mod slider;
pub mod toggle;

pub trait SettingUntyped {
    fn render(&mut self, _ui: &hudhook::imgui::Ui) -> anyhow::Result<()>;
    fn reset_to_default(&mut self) -> anyhow::Result<()>;
    fn reset_to_vanilla(&mut self) -> anyhow::Result<()>;
}

pub struct Setting<T: Copy> {
    value: T,
    defaults: Defaults<T>,
    inner: Box<dyn SettingImpl<T> + Send + Sync>,
}

impl<T: Copy> Setting<T> {
    fn new(inner: impl SettingImpl<T> + Send + Sync + 'static, defaults: Defaults<T>) -> Self {
        Self {
            value: defaults.default,
            defaults,
            inner: Box::new(inner) as _,
        }
    }

    fn set(&mut self, value: T) -> anyhow::Result<()> {
        self.value = value;
        self.inner.set(value)
    }
}

impl<T: Copy> SettingUntyped for Setting<T> {
    fn render(&mut self, ui: &hudhook::imgui::Ui) -> anyhow::Result<()> {
        self.inner.render(&mut self.value, &self.defaults, ui)
    }

    fn reset_to_default(&mut self) -> anyhow::Result<()> {
        self.set(self.defaults.default)
    }

    fn reset_to_vanilla(&mut self) -> anyhow::Result<()> {
        self.set(self.defaults.vanilla)
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
