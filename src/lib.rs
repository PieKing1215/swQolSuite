#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unreadable_literal)]

#[allow(clippy::module_name_repetitions)]
pub mod tweaks;

use hooks::opengl3::ImguiOpenGl3Hooks;
use hudhook::tracing::{error, trace};
use hudhook::{eject, hooks, imgui, windows, Hudhook, ImguiRenderLoop, MessageFilter};
use imgui::{Condition, Io, Key, StyleColor, StyleVar, Ui};
use memory_rs::internal::process_info::ProcessInfo;
use tweaks::dev_mode::DevModeTweak;
use tweaks::editor_camera_speed::EditorCameraSpeedTweak;
use tweaks::editor_placement::EditorPlacementTweak;
use tweaks::editor_show_hidden::ShowHiddenComponents;
use tweaks::fullscreen::FullscreenTweak;
use tweaks::loading::LoadingTweak;
use tweaks::map_lag::MapLagTweak;
use tweaks::Tweak;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut std::ffi::c_void) {
    if reason == DLL_PROCESS_ATTACH {
        trace!("DllMain()");
        std::thread::spawn(move || {
            // alloc_console().unwrap();

            println!("Spawned thread");

            std::panic::set_hook(Box::new(|panic_info| {
                if let Some(str) = panic_info.payload().downcast_ref::<&str>() {
                    println!("panic occurred: {str:?}");
                } else {
                    println!("panic occurred");
                }
            }));

            if let Err(e) = Hudhook::builder()
                .with::<ImguiOpenGl3Hooks>(MainHud::new())
                .with_hmodule(hmodule)
                .build()
                .apply()
            {
                error!("Couldn't apply hooks: {e:?}");
                eject();
            }
        });
    }
}

struct MainHud {
    version_string: String,
    show: bool,
    tweaks: Vec<Box<dyn Tweak + Send + Sync>>,
    errors: Vec<anyhow::Error>,
}

impl MainHud {
    fn new() -> Self {
        let mut this = Self {
            version_string: format!(
                "swQolSuite v{}{}",
                env!("CARGO_PKG_VERSION"),
                option_env!("SHA").map_or_else(String::new, |sha| format!(" ({sha})"))
            ),
            show: true,
            tweaks: vec![],
            errors: vec![],
        };

        match ProcessInfo::new(Some("stormworks64.exe")) {
            Ok(process) => {
                this.add_tweak(MapLagTweak::new(&process.region));
                this.add_tweak(EditorCameraSpeedTweak::new(&process.region));
                this.add_tweak(EditorPlacementTweak::new(&process.region));
                this.add_tweak(LoadingTweak::new(&process.region));
                this.add_tweak(ShowHiddenComponents::new(&process.region));
                this.add_tweak(FullscreenTweak::new(&process.region));
                this.add_tweak(DevModeTweak::new(&process.region));
            },
            Err(err) => this.errors.push(err),
        }

        this
    }

    fn add_tweak<T: Tweak + Send + Sync + 'static>(&mut self, tw: anyhow::Result<T>) {
        match tw {
            Ok(tw) => self.tweaks.push(Box::new(tw)),
            Err(e) => self.errors.push(e),
        }
    }

    fn uninit(&mut self) {
        let mut ok = true;

        for mut tw in self.tweaks.drain(..) {
            if let Err(e) = tw.uninit() {
                self.errors.push(e);
                ok = false;
            }
        }

        if ok {
            eject();
        }
    }
}

impl ImguiRenderLoop for MainHud {
    // fn before_render<'a>(
    //     &'a mut self,
    //     ctx: &mut Context,
    //     _render_context: &'a mut dyn RenderContext,
    // ) {
    //     ctx.io_mut().mouse_draw_cursor = self.show;
    // }

    fn render(&mut self, ui: &mut Ui) {
        if ui.is_key_pressed_no_repeat(Key::GraveAccent) {
            self.show = !self.show;
        }

        let style_padding = ui.push_style_var(StyleVar::WindowPadding([2., 2.]));
        ui.window("##version")
            .no_decoration()
            .no_inputs()
            .draw_background(false)
            .save_settings(false)
            .always_use_window_padding(true)
            .always_auto_resize(true)
            .size_constraints([-1., 18.], [-1., 18.])
            .position_pivot([1., 1.])
            .position(ui.io().display_size, Condition::Always)
            .build(|| {
                let text_color = ui.push_style_color(
                    StyleColor::Text,
                    [0.8, 0.8, 0.8, if self.show { 0.9 } else { 0.4 }],
                );
                if self.show {
                    ui.text(format!("{} [~]", self.version_string));
                } else {
                    ui.text("  [~]");
                }
                text_color.end();
            });
        style_padding.end();

        for tw in &mut self.tweaks {
            tw.constant_render(ui);
        }

        if !self.show {
            return;
        }

        // let bg_color = ui.push_style_color(StyleColor::WindowBg, [0.1, 0.1, 0.1, 0.25]);
        // ui.window("##background")
        //     .focus_on_appearing(false)
        //     .bring_to_front_on_focus(false)
        //     .focused(false)
        //     .title_bar(false)
        //     .resizable(false)
        //     .position_pivot([0., 0.])
        //     .position([0., 0.], Condition::Always)
        //     .size(ui.io().display_size, Condition::Always)
        //     .build(|| {});
        // bg_color.end();

        ui.window("swQolSuite")
            .no_nav()
            .always_auto_resize(true)
            .resizable(false)
            .position([50., 50.], Condition::FirstUseEver)
            .build(|| {
                ui.text(&self.version_string);
                ui.text("[~] Visibility");
                if ui.button("Eject") {
                    self.uninit();
                }
            });

        ui.window("Tweaks")
            .no_nav()
            .always_auto_resize(true)
            .resizable(false)
            .position([250., 50.], Condition::FirstUseEver)
            .build(|| {
                if ui.button("Reset to Default") {
                    for tw in &mut self.tweaks {
                        tw.reset_to_default();
                    }
                };
                ui.same_line();
                if ui.button("Reset to Vanilla") {
                    for tw in &mut self.tweaks {
                        tw.reset_to_vanilla();
                    }
                };

                ui.separator();

                for tw in &mut self.tweaks {
                    tw.render(ui);
                }
            });

        if !self.errors.is_empty() {
            ui.window("Errors")
                .no_nav()
                .size([300., 200.], Condition::FirstUseEver)
                .position([50., 250.], Condition::FirstUseEver)
                .build(|| {
                    for err in &self.errors {
                        ui.text(format!("{err:?}"));
                        ui.separator();
                    }
                });
        }
    }

    fn message_filter(&self, io: &Io) -> MessageFilter {
        if self.show && (io.want_capture_keyboard || io.want_capture_mouse) {
            return MessageFilter::InputKeyboard | MessageFilter::InputMouse;
        }

        MessageFilter::empty()
    }
}
