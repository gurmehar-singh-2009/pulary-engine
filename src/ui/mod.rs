use gpui::{Application, SharedString, TitlebarOptions, WindowOptions};

pub fn init_ui() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::new("Pulary Engine")),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| todo!(),
        )
        .unwrap();
    });
}
