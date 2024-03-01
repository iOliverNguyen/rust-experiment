use gpui::*;

actions!(app, [Quit, CloseWindow]);

fn main() {
    let app = App::new();
    app.run(|cx| {
        let opts = WindowOptions {
            bounds: WindowBounds::Fixed(Bounds::<GlobalPixels>::new(
                calc_window_origin(cx, 700., 500.),
                size(px(800.), px(600.)).into(),
            )),
            titlebar: Some(TitlebarOptions {
                title: None,
                appears_transparent: true,
                traffic_light_position: Some(point(px(10.), px(10.))),
                ..TitlebarOptions::default()
            }),
            ..WindowOptions::default()
        };

        cx.open_window(opts, |cx| {
            let app_view = cx.new_view(|cx| AppView::new(cx, list_planets));
            cx.focus_view(&app_view);
            app_view
        });

        cx.activate(true);
        cx.on_action(|act: &Quit, cx| cx.quit());

        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-w", CloseWindow, None),
        ]);

        cx.set_menus(vec![Menu {
            name: "",
            items: vec![
                MenuItem::action("Close active window", CloseWindow),
                MenuItem::separator(),
                MenuItem::action("Quit", Quit),
            ],
        }]);
    });
}

struct AppView {
    focus_handle: FocusHandle,
}

fn calc_window_origin(cx: &AppContext, w: f32, h: f32) -> Point<GlobalPixels> {
    let displays = cx.displays();
    let first_display = displays.first().expect("no displays");

    let window_size: Size<GlobalPixels> = size(px(w), px(h)).into();
    let window_origin = point(
        first_display.bounds().center().x - window_size.width / 2.,
        first_display.bounds().center().y - window_size.height / 2.,
    );
    window_origin
}
