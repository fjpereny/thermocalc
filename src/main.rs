use thermocalc::ui::Ui;

pub fn main() -> iced::Result {
    iced::application(|| Ui::default(), Ui::update, Ui::view)
        .title("Thermocalc")
        .theme(|_: &Ui| iced::Theme::Light)
        .run()
}
