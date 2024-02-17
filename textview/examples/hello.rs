use floem::{
    peniko::Color,
    style::Position,
    unit::{Px, PxPctAuto},
    view::View,
    views::{container, empty, label, stack, text, v_stack, Decorators, Empty},
    window::WindowConfig,
    Application,
};

fn main() {
    let opts = WindowConfig::default()
        .show_titlebar(false)
        .with_transparent(true);
    Application::new()
        .window(move |_| app_view(), Some(opts))
        .run()
}

fn app_view() -> impl View {
    v_stack((
        container(text("Title")).style(|s| {
            s.width_full()
                .background(Color::rgb8(255, 255, 180))
                .flex_row()
                .justify_center()
                .items_center()
                .height(Px(40.0))
        }),
        stack((
            left_sidebar().style(|s| s.width(Px(100.0))),
            center_pane().style(|s| s.width_full().background(Color::rgb8(200, 255, 200))),
            right_sidebar().style(|s| s.width(Px(100.0))),
        ))
        .style(|s| s.height_full()),
        stack((
            container(empty().style(|s| {
                s.width(Px(20.0))
                    .height(Px(15.0))
                    .background(Color::rgb8(255, 150, 150))
            }))
            .style(|s| {
                s.background(Color::rgb8(255, 210, 150))
                    .width(Px(100.0))
                    .items_start()
            }),
            container(text("open a file")).style(|s| {
                s.width_full()
                    .justify_center()
                    .items_center()
                    .background(Color::rgb8(220, 255, 220))
                    .font_size(32.0)
            }),
            container(empty().style(|s| {
                s.width(Px(20.0))
                    .height(Px(15.0))
                    .background(Color::rgb8(150, 150, 255))
            }))
            .style(|s| {
                s.background(Color::rgb8(150, 210, 255))
                    .width(Px(100.0))
                    .items_end()
                    .justify_end()
            }),
        ))
        .style(|s| s.height(Px(36.0)).font_size(16.0)),
    ))
    .style(|s| {
        s.width_full()
            .background(Color::rgb8(200, 200, 200))
            .font_family("Fira Code".to_string())
    })
}

fn left_sidebar() -> impl View {
    v_stack((
        container(text("◀").style(|s| s.font_size(100.0).color(Color::rgb8(255, 100, 0))))
            .style(|s| s.height_full().items_center().justify_center()),
    ))
    .style(|s| s.background(Color::rgb8(255, 200, 100)))
}

fn right_sidebar() -> impl View {
    v_stack((
        container(text("▶").style(|s| s.font_size(100.0).color(Color::rgb8(0, 100, 255))))
            .style(|s| s.height_full().items_center().justify_center()),
    ))
    .style(|s| s.background(Color::rgb8(100, 200, 255)))
}

fn center_pane() -> impl View {
    container(
        stack((
            text("center").style(|s| s.font_size(100.0)),
            container(text("click to open")).style(|s| {
                s.absolute()
                    .margin_bottom(Px(120.0))
                    .justify_center()
                    .items_center()
                    .font_size(24.0)
                    .font_family("Arial".to_string())
            }),
        ))
        .style(|s| s.justify_center().items_center()),
    )
    .style(|s| s.justify_center().items_center())
}
