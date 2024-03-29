use floem::{
    keyboard::{self, Key, ModifiersState},
    menu::{Menu, MenuItem},
    peniko::Color,
    style::Position,
    unit::{Px, PxPctAuto},
    view::View,
    views::{container, drag_window_area, empty, label, stack, text, v_stack, Decorators, Empty},
    window::{WindowConfig, WindowId},
    Application,
};
use smol_str::{self, ToSmolStr};

struct WindowInfo {
    window_id: WindowId,
}

fn main() {
    let opts = WindowConfig::default()
        .show_titlebar(false)
        .with_transparent(true);
    Application::new()
        .window(
            move |window_id| {
                let window_info = WindowInfo { window_id };
                let app_view = app_view().window_menu(|| app_menu());
                let app_view = app_keyboard(app_view, window_info);
                app_view
            },
            Some(opts),
        )
        .run()
}

fn app_view() -> impl View {
    v_stack((
        stack((drag_window_area(container(text("Title")).style(|s| {
            s.width_full()
                .background(Color::rgb8(255, 255, 180))
                .flex_row()
                .justify_center()
                .items_center()
        }))
        .style(|s| {
            s.height_full()
                .flex_basis(0.0)
                .flex_grow(1.0)
                .background(Color::rgb8(100, 100, 100))
        }),))
        .style(|s| s.height(Px(40.0))),
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
            container(text("click to choose a file")).style(|s| {
                s.width_full()
                    .justify_center()
                    .items_center()
                    .background(Color::rgb8(220, 255, 220))
                    .font_size(14.0)
                    .font_family("Arial".to_string())
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

// NOTE: didn't work on macos
fn app_menu() -> Menu {
    Menu::new("Main Menu").entry(
        Menu::new("Hello")
            .entry(MenuItem::new("The about menu item"))
            .separator()
            .entry(MenuItem::new("The quit menu item")),
    )
}

fn app_keyboard<V>(view: V, window_info: WindowInfo) -> V
where
    V: View,
{
    view.keyboard_navigatable().on_key_down(
        Key::Character("w".to_smolstr()),
        key_super(),
        move |_| floem::close_window(window_info.window_id),
    )
}

fn key_super() -> ModifiersState {
    let mut mods = ModifiersState::empty();
    mods.set(ModifiersState::SUPER, true);
    mods
}
