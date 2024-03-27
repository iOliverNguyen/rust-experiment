use gpui::*;

pub struct CounterWidget {
    count: i32,
}

impl CounterWidget {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Render for CounterWidget {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().id("root").size_full().flex().flex_col().child(
            div()
                .child(
                    div()
                        .id("btn-plus")
                        .on_click(cx.listener(|this, ev, cx| {
                            this.count += 1;
                        }))
                        .child(format!("Count: {}", self.count)),
                )
                .child(div()),
        )
    }
}
