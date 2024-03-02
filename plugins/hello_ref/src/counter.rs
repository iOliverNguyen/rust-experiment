use egui::{self, Response, Ui};

pub struct CounterWidget {
    count: i32,
}

impl CounterWidget {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    // pub fn render(&mut self, ui: &mut Ui) -> Response {
    //     ui.horizontal(|ui| {
    //         ui.label("Count: ");
    //         ui.label(self.count.to_string());
    //         if ui.button("+").clicked() {
    //             self.count += 1;
    //         }
    //         if ui.button("-").clicked() {
    //             self.count -= 1;
    //         }
    //     })
    // }
}
