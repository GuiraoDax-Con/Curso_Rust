use eframe::egui;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static AGENDA: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Default)]
pub struct AgendaApp {
    pub nuevo_contacto: String,
    pub contacto_buscado: String,
    pub mensaje: String,
}

impl eframe::App for AgendaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Agenda de Contactos");

            ui.horizontal(|ui| {
                ui.label("Nuevo contacto:");
                ui.text_edit_singleline(&mut self.nuevo_contacto);
                if ui.button("Añadir").clicked() {
                    let mut agenda = AGENDA.lock().unwrap();
                    if !self.nuevo_contacto.trim().is_empty() {
                        agenda.push(self.nuevo_contacto.trim().to_string());
                        self.mensaje = format!("Contacto '{}' añadido.", self.nuevo_contacto);
                        self.nuevo_contacto.clear();
                    }
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Buscar contacto:");
                ui.text_edit_singleline(&mut self.contacto_buscado);
                if ui.button("Buscar").clicked() {
                    let agenda = AGENDA.lock().unwrap();
                    if agenda.contains(&self.contacto_buscado) {
                        self.mensaje = format!("¡{} encontrado!", self.contacto_buscado);
                    } else {
                        self.mensaje = format!("{} no está en la agenda.", self.contacto_buscado);
                    }
                }
            });

            ui.separator();

            if ui.button("Mostrar todos los contactos").clicked() {
                let agenda = AGENDA.lock().unwrap();
                self.mensaje = format!("Contactos: [{}]", agenda.join(", "));
            }

            if ui.button("Eliminar contacto").clicked() {
                let mut agenda = AGENDA.lock().unwrap();
                if let Some(pos) = agenda.iter().position(|x| x == &self.contacto_buscado) {
                    agenda.remove(pos);
                    self.mensaje = format!("{} eliminado de la agenda.", self.contacto_buscado);
                } else {
                    self.mensaje = format!("{} no se encontró en la agenda.", self.contacto_buscado);
                }
            }

            ui.separator();

            ui.label(&self.mensaje);
        });
    }
}
