fn bar_contents(ui: &mut egui::Ui) {
    egui::widgets::global_dark_light_mode_switch(ui);

    ui.separator();

    ui.heading("Welcome to Crustacean STATION! 📻");

    ui.separator();

    ui.hyperlink("https://media.tenor.com/oB3o62J9hjkAAAAC/dancing-ferris.gif");

    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        // if false {
        //     // TODO(emilk): fix the overlap on small screens
        //     if clock_button(ui, crate::seconds_since_midnight()).clicked() {
        //         self.state.selected_anchor = Anchor::Clock;
        //         if frame.is_web() {
        //             ui.output_mut(|o| o.open_url("#clock"));
        //         }
        //     }
        // }

        egui::warn_if_debug_build(ui);
    });
}

pub fn show_top_bar(ctx: &egui::Context) {
    egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
        egui::trace!(ui);
        ui.horizontal_wrapped(|ui| {
            ui.visuals_mut().button_frame = false;
            bar_contents(ui);
        });
    });
}
