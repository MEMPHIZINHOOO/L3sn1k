mod gui;
mod tests;
fn main() -> iced::Result {

    unsafe {
        std::env::set_var("WGPU_POWER_PREF", "high");
    }

    gui::L3snikGui::start()
}
