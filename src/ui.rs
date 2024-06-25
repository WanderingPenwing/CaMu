use crate::MaBO;
use crate::egui;
use crate::egui::{RichText, Color32};


impl MaBO {
	pub fn show_info_panel(&mut self, ctx: &egui::Context) {
		egui::SidePanel::left("infos").show(ctx, |ui| {
			ui.label("Méthodologie");
			ui.label("Exemple");
			ui.label("Code source");
		});
	}
	
	pub fn show_main_panel(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::vertical().show(ui, |ui| {
				ui.label("Scénario de référence (utilisation normale) :");
				ui.separator();
				ui.label("");
				
				for variable in &mut self.equation.variables {
				
					if variable.name == "Vm" {
						ui.label("Scénario alternatif (utilisation mutualisée) :");
						ui.separator();
						ui.label("");
					}
				
					ui.label(&format!("{} : ", variable.description));
					ui.horizontal(|ui| {
						ui.label(&format!("{} = ", variable.name));
						ui.add(egui::TextEdit::singleline(&mut variable.input));
					});
					if variable.value == None {
						ui.label(RichText::new("valeur invalide").color(Color32::RED));
					}
					ui.label("");
				}
				
				ui.separator();
				
				if ui.add(egui::Button::new("Calcul")).clicked() {
					self.equation.compute();
				}
				
				
				if let Some(ref impact) = self.equation.impact {
					ui.label("");
					ui.label("Impact du produit mutualisé dans la catégorie d'impact choisie, par an : ");
					ui.label(format!("Im = {}",impact));
				}
				
				if let Some(ref improvement) = self.equation.improvement {
					ui.label("");
					ui.label("Gain de la mutualisation par rapport à l'utilisation usuelle : ");
					ui.label(format!("G% = {} %",improvement));
				}
			});
		});
	}
}
