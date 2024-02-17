use std::ops::Shl;

use gpui::*;

actions!(app, [Quit, Open]);

fn main() {
    let app = App::new();
    app.run(|cx| {
        let displays = cx.displays();
        let first_display = displays.first().expect("no displays");

        let window_size: Size<GlobalPixels> = size(px(800.), px(600.)).into();
        let window_origin = point(
            first_display.bounds().center().x - window_size.width / 2.,
            first_display.bounds().center().y - window_size.height / 2.,
        );

        let opts = WindowOptions {
            bounds: WindowBounds::Fixed(Bounds::<GlobalPixels>::new(
                window_origin,
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
        cx.open_window(opts, |cx| cx.new_view(|cx| AppView::new(cx)));

        cx.activate(true);

        cx.on_action(|act: &Quit, cx| cx.quit());
        cx.on_action(move |act: &Open, cx| {
            action_open(act, cx);
        });
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-o", Open, None),
        ]);

        cx.set_menus(vec![
            Menu {
                name: "",
                items: vec![MenuItem::Action {
                    name: "Quit",
                    action: Box::new(Quit),
                    os_action: None,
                }],
            },
            Menu {
                name: "Second",
                items: vec![MenuItem::Action {
                    name: "Open",
                    action: Box::new(Open),
                    os_action: None,
                }],
            },
        ]);
    })
}

fn action_open(_: &Open, cx: &mut gpui::AppContext) {
    let rx_paths = cx.prompt_for_paths(PathPromptOptions {
        files: true,
        directories: true,
        multiple: true,
    });
    println!("{}", std::backtrace::Backtrace::capture());
}

struct AppView {}

impl AppView {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        AppView {}
    }
}

impl Render for AppView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x888888))
            .font("Fira Code")
            .children([
                ext(div(), |x| flex_center(x))
                    .h_10()
                    .bg(rgb(0xFFFFAA))
                    .text_size(px(12.))
                    .child("Title"),
                div()
                    .size_full()
                    .flex()
                    .justify_center()
                    .items_center()
                    .children([
                        { flex_center(div()) }
                            .w(px(100.))
                            .h_full()
                            .text_size(px(100.))
                            .flex()
                            .justify_center()
                            .items_center()
                            .text_color(rgb(0xFF6600))
                            .bg(rgb(0xFFDD66))
                            .child("◀"),
                        div()
                            .size_full()
                            .flex()
                            .justify_center()
                            .items_center()
                            .bg(rgb(0xAAFFAA))
                            .child(
                                div().children([
                                    div().text_size(px(100.)).child("center"),
                                    div()
                                        .w_full()
                                        .flex()
                                        .justify_center()
                                        .absolute()
                                        .bottom(px(120.))
                                        .child("click to open"),
                                ]),
                            ),
                        { flex_center(div()) }
                            .w(px(100.))
                            .h_full()
                            .text_size(px(100.))
                            .text_color(rgb(0x0066FF))
                            .bg(rgb(0x66DDFF))
                            .child("▶"),
                    ]),
                div().h(px(40.)).justify_center().items_center().children([
                    div(),
                    ext(div(), |x| flex_center(x))
                        .size_full()
                        .text_size(px(14.))
                        .font("Arial")
                        .children([
                            div()
                                .h_full()
                                .w(px(100.))
                                .bg(rgb(mix32(0xFF6600, 0xFFFFFF))),
                            ext(div(), |x| flex_center(x))
                                .size_full()
                                .bg(rgb(mix32(0xAAFFAA, 0xFFFFFF)))
                                .child("click to choose a file"),
                            div()
                                .h_full()
                                .w(px(100.))
                                .bg(rgb(mix32(0x66DDFF, 0xFFFFFF))),
                        ]),
                    div(),
                ]),
            ])
    }
}

fn ext<T: IntoElement>(x: T, f: impl Fn(T) -> T) -> T {
    f(x)
}

fn flex_center<T: Styled>(x: T) -> T {
    x.flex().justify_center().items_center()
}

fn mix32(a: u32, b: u32) -> u32 {
    let x0 = (((a >> 0) & 0xFF) + ((b >> 0) & 0xFF)) / 2;
    let x1 = (((a >> 8) & 0xFF) + ((b >> 8) & 0xFF)) / 2;
    let x2 = (((a >> 16) & 0xFF) + ((b >> 16) & 0xFF)) / 2;
    let x3 = (((a >> 24) & 0xFF) + ((b >> 24) & 0xFF)) / 2;

    x0 + x1.wrapping_shl(8) + x2.wrapping_shl(16) + x3.wrapping_shl(24)
}

#[cfg(test)]
mod test {
    use crate::mix32;

    #[test]
    fn test_mix32() {
        assert_eq!(mix32(0x00FF00, 0xFFFFFF), 0x7FFF7F);
    }
}
