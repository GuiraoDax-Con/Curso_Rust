mod gui;

fn main() -> Result<(), eframe::Error> {
    let opciones = eframe::NativeOptions::default();
    eframe::run_native(
        "Agenda de Contactos",
        opciones,
        Box::new(|_cc| Ok(Box::new(gui::AgendaApp::default()))),
    )
}
