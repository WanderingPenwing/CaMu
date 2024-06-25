use crate::MaBO;
use crate::egui;
use crate::egui::{RichText, Color32, Align};


impl MaBO {
	pub fn show_info_panel(&mut self, ctx: &egui::Context) {
		egui::SidePanel::left("infos").show(ctx, |ui| {
			ui.label("Méthodologie");
			ui.label("Exemple");
			ui.hyperlink_to(
				"Code source", 
				"https://github.com/WanderingPenwing/CaMu",
			);
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
				
				let mut scroll_bottom = false;
				
				if ui.add(egui::Button::new("Calcul")).clicked() {
					self.equation.compute();
					scroll_bottom = true;
				}
				
				
				if let Some(ref impact) = self.equation.impact {
					ui.label(format!("\nImpact du produit mutualisé dans la catégorie d'impact choisie, par an :\nIm = {}",impact));
				}
				
				if let Some(ref improvement) = self.equation.improvement {
					ui.label(format!("\nGain de la mutualisation par rapport à l'utilisation usuelle :\nG% = {} %",improvement));
				}
				
				ui.label("\n\n");
				
				if scroll_bottom {
					ui.scroll_to_cursor(Some(Align::BOTTOM));
				}
			});
		});
	}
}
