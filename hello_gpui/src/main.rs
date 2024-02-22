use std::{future::IntoFuture, ops::Deref};

use gpui::*;

actions!(app, [Quit, CloseWindow, ChooseFile]);

use once_cell::sync::Lazy;

static KEY_LEFT: Lazy<Keystroke> = Lazy::new(|| Keystroke::parse("left").unwrap());
static KEY_RIGHT: Lazy<Keystroke> = Lazy::new(|| Keystroke::parse("right").unwrap());
static KEY_ENTER: Lazy<Keystroke> = Lazy::new(|| Keystroke::parse("enter").unwrap());
static KEY_ESC: Lazy<Keystroke> = Lazy::new(|| Keystroke::parse("escape").unwrap());

#[derive(Clone)]
struct Planet {
    name: String,
    desc: String,
}

impl Planet {
    fn new(name: &str, desc: &str) -> Self {
        Self {
            name: name.to_string(),
            desc: desc.to_string(),
        }
    }
}

struct ListPlanets {
    items: Vec<Planet>,
    default_index: usize,
}

impl Deref for ListPlanets {
    type Target = [Planet];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl ListPlanets {
    fn init() -> Self {
        let planets = vec![
Planet::new("Mercury", "Mercury is the smallest and closest planet to the Sun. It has a rocky surface marked with craters similar to the Moon. Because of its proximity to the Sun, it experiences extreme temperature variations"),
Planet::new("Venus","Venus is similar in size and structure to Earth but is shrouded in a thick, toxic atmosphere that traps heat, leading to surface temperatures hot enough to melt lead. Its surface is volcanic and covered in sulfuric acid clouds."),
Planet::new("Earth", "Earth is the only planet known to support life, with a diverse ecosystem. It has a unique atmosphere composed of nitrogen, oxygen, and other gases, liquid water on its surface, and a varied terrain including mountains, valleys, and oceans."),
Planet::new("Mars", "Mars, known as the Red Planet due to its reddish appearance, has the tallest volcano and the deepest, longest canyon in the Solar System. It has water ice at its poles and evidence suggests it once had liquid water on its surface."),
Planet::new("Jupiter", "Jupiter is the largest planet in the Solar System. A gas giant, it has a Great Red Spot, a giant storm that has raged for hundreds of years. Jupiter has a strong magnetic field and at least 79 moons, including the four large Galilean moons: Io, Europa, Ganymede, and Callisto."),
Planet::new("Saturn", "Saturn is best known for its stunning ring system, made up of ice and rock particles. It is a gas giant like Jupiter, with a composition mainly of hydrogen and helium. Saturn has numerous moons, with Titan being the largest and having a thick atmosphere."),
Planet::new("Uranus", "Uranus is unique for its tilted axis, which causes it to rotate on its side. It is an ice giant with a colder atmosphere containing water, ammonia, and methane. Uranus has a faint ring system and 27 known moons."),
Planet::new("Neptune", "Neptune, an ice giant, is known for its deep blue color, caused by methane in its atmosphere. It has the strongest winds of any planet in the Solar System. Neptune has 14 known moons, with Triton being the largest and geologically active."),
        ];
        let default_index = planets
            .iter()
            .enumerate()
            .find_map(|(idx, planet)| {
                if &planet.name == "Earth" {
                    Some(idx)
                } else {
                    None
                }
            })
            .unwrap();
        ListPlanets {
            items: planets,
            default_index,
        }
    }
}

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

        let list_planets = cx.new_model(|cx| ListPlanets::init());
        cx.open_window(opts, |cx| {
            let app_view = cx.new_view(|cx| AppView::new(cx, list_planets));
            cx.focus_view(&app_view);
            app_view
        });

        cx.activate(true);

        cx.on_action(|act: &Quit, cx| cx.quit());
        cx.on_action(|act: &ChooseFile, cx| action_choose_file(act, cx));

        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-w", CloseWindow, None),
            KeyBinding::new("cmd-o", ChooseFile, None),
        ]);

        cx.set_menus(vec![
            Menu {
                name: "",
                items: vec![
                    MenuItem::action("Close active window", CloseWindow),
                    MenuItem::separator(),
                    MenuItem::action("Quit", Quit),
                ],
            },
            Menu {
                name: "Second",
                items: vec![MenuItem::action("Choose a file", ChooseFile)],
            },
        ]);
    })
}

fn action_choose_file(_: &ChooseFile, cx: &mut gpui::AppContext) {
    let rx_paths = cx.prompt_for_paths(PathPromptOptions {
        files: true,
        directories: true,
        multiple: true,
    });
    println!("{}", std::backtrace::Backtrace::capture());
}

struct AppView {
    focus_handle: FocusHandle,

    list_items: Model<ListPlanets>,
    active_idx: usize,
}

impl AppView {
    fn new(cx: &mut ViewContext<Self>, list: Model<ListPlanets>) -> Self {
        let focus_handle = cx.focus_handle();
        AppView {
            focus_handle,
            active_idx: list.read_with(cx, |x, cx| x.default_index),
            list_items: list,
        }
    }

    fn bind_root_actions<T: StatefulInteractiveElement>(&self, root: T) -> T {
        root.on_action(|_: &CloseWindow, cx| cx.remove_window())
    }

    fn is_left_available(&self, cx: &ViewContext<Self>) -> bool {
        self.active_idx > 0
    }

    fn is_right_available(&self, cx: &ViewContext<Self>) -> bool {
        self.active_idx < self.list_items.read(cx).len() - 1
    }

    fn handle_keydown(&mut self, ev: &KeyDownEvent, cx: &mut ViewContext<Self>) {
        if let KeyDownEvent {
            keystroke,
            is_held: false,
        } = ev
        {
            if *keystroke == *KEY_LEFT {
                self.handle_left(cx);
                cx.refresh();
            }
            if *keystroke == *KEY_RIGHT {
                self.handle_right(cx);
                cx.refresh();
            }
            if *keystroke == *KEY_ENTER {
                self.handle_enter(cx);
                cx.refresh();
            }
        };
    }

    fn handle_left(&mut self, cx: &mut ViewContext<Self>) {
        if self.is_left_available(cx) {
            self.active_idx -= 1;
        }
    }

    fn handle_right(&mut self, cx: &mut ViewContext<Self>) {
        if self.is_right_available(cx) {
            self.active_idx += 1;
        }
    }

    fn handle_enter(&mut self, cx: &mut ViewContext<Self>) {
        let item = self.list_items.read(cx).get(self.active_idx).unwrap();
        let opts = WindowOptions {
            bounds: WindowBounds::Fixed(Bounds::<GlobalPixels>::new(
                calc_window_origin(cx, 800., 600.),
                size(px(800.), px(600.)).into(),
            )),
            titlebar: Some(TitlebarOptions {
                title: Some(item.name.clone().into()),
                ..TitlebarOptions::default()
            }),
            center: true,
            focus: true,
            show: true,
            kind: WindowKind::PopUp,
            is_movable: true,
            ..WindowOptions::default()
        };
        let item = item.clone(); // Clone the item
        cx.spawn(|_, cx| async move {
            cx.open_window(opts, move |cx| {
                // Use move to take ownership of item_clone
                let popup_view = cx.new_view(|cx| PopupView::new(cx, item));
                cx.focus_view(&popup_view);
                popup_view // Use move to take ownership of item_clone
            })
        })
        .detach_and_log_err(cx);
    }
}

impl FocusableView for AppView {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AppView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("root")
            .apply(|x| self.bind_root_actions(x))
            .key_context("AppView")
            .track_focus(&self.focus_handle)
            .on_action(|act: &CloseWindow, cx| cx.remove_window())
            .on_key_down(cx.listener(Self::handle_keydown))
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x888888))
            .font("Fira Code")
            .children([
                div()
                    .flex_center()
                    .h_10()
                    .bg(rgb(0xFFFFAA))
                    .text_size(px(12.))
                    .child("Title"),
                div()
                    .size_full()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        div()
                            .id("left")
                            .on_click(cx.listener(|this, _, cx| this.handle_left(cx)))
                            .flex_center() // use StyledExtension
                            .w(px(100.))
                            .h_full()
                            .text_size(px(100.))
                            .flex()
                            .justify_center()
                            .items_center()
                            .apply(|x| {
                                if self.is_left_available(cx) {
                                    x.cursor_pointer().text_color(rgb(0xFF6600))
                                } else {
                                    x.cursor_default()
                                        .text_color(rgb(mix32(0xFF6600, 0xFFFFFF)))
                                }
                            })
                            .bg(rgb(0xFFDD66))
                            .child("◀"),
                    )
                    .child(
                        div()
                            .id("center")
                            .on_click(cx.listener(|this, _, cx| this.handle_enter(cx)))
                            .size_full()
                            .flex_center()
                            .bg(rgb(0xAAFFAA))
                            .child(
                                div().w_full().children([
                                    div().w_full().flex_center().text_size(px(100.)).child(
                                        match self.list_items.read(cx).get(self.active_idx) {
                                            Some(item) => item.name.clone(),
                                            None => String::new(),
                                        },
                                    ),
                                    div()
                                        .w_full()
                                        .flex_center()
                                        .absolute()
                                        .bottom(px(140.))
                                        .child("enter to open"),
                                ]),
                            ),
                    )
                    .child(
                        div()
                            .apply(|x| x.flex_center())
                            .id("right") // use ext()
                            .on_click(cx.listener(|this, _, cx| this.handle_right(cx)))
                            .w(px(100.))
                            .h_full()
                            .text_size(px(100.))
                            .apply(|x| {
                                if self.is_right_available(cx) {
                                    x.cursor_pointer().text_color(rgb(0x0066FF))
                                } else {
                                    x.cursor_default()
                                        .text_color(rgb(mix32(0x0066FF, 0xFFFFFF)))
                                }
                            })
                            .bg(rgb(0x66DDFF))
                            .child("▶"),
                    ),
                div().h(px(40.)).justify_center().items_center().children([
                    div(),
                    div()
                        .flex_center()
                        .size_full()
                        .text_size(px(14.))
                        .font("Arial")
                        .children([
                            div()
                                .h_full()
                                .w(px(100.))
                                .bg(rgb(mix32(0xFFDD66, 0xFFFFFF))),
                            div()
                                .flex_center()
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

struct PopupView {
    item: Planet,
    focus_handle: FocusHandle,
}

impl PopupView {
    fn new(cx: &mut ViewContext<Self>, item: Planet) -> Self {
        let focus_handle = cx.focus_handle();
        PopupView { item, focus_handle }
    }

    fn handle_keydown(&mut self, ev: &KeyDownEvent, cx: &mut ViewContext<Self>) {
        if let KeyDownEvent {
            keystroke,
            is_held: false,
        } = ev
        {
            if *keystroke == *KEY_ESC {
                cx.remove_window();
            }
        }
    }
}

impl FocusableView for PopupView {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PopupView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::handle_keydown))
            .on_action(|_: &CloseWindow, cx| cx.remove_window())
            .text_color(rgb(0xFFFFFF))
            .bg(rgb(0x336633))
            .flex_center()
            .size_full()
            .child(div().w_full().p(px(50.)).child(format!(
                "{}\n\n(press ESC to close)",
                self.item.desc.clone()
            )))
    }
}

impl<T: Styled> StyledExtension for T {}
impl<T: IntoElement> ElementExtension for T {}

trait StyledExtension: Styled {
    fn flex_center(self) -> Self {
        self.flex().justify_center().items_center()
    }
}

trait ElementExtension: IntoElement {
    fn apply(self, f: impl FnOnce(Self) -> Self) -> Self {
        f(self)
    }
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
