use crate::MaBO;
use crate::egui;
use crate::egui::{RichText, Color32, Align, FontId};


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
				ui.label(RichText::new("Scénario de référence (utilisation normale) :").color(Color32::BLACK).font(FontId::proportional(18.0)));
				ui.separator();
				ui.label("");
				
				for variable in &mut self.equation.variables {
				
					if variable.name == "Vm" {
						ui.label(RichText::new("\nScénario alternatif (utilisation mutualisée) :").color(Color32::BLACK).font(FontId::proportional(18.0)));
						ui.separator();
						ui.label("");
					}
				
					ui.label(&format!("{} : ", variable.description));
					ui.horizontal(|ui| {
						ui.label(RichText::new(&format!("{} = ", variable.name)).color(Color32::BLACK));
						ui.add(egui::DragValue::new(&mut variable.value).speed(variable.speed).clamp_range(variable.min..=variable.max));
					});
					ui.label("");
				}
				
				ui.separator();
				
				let mut scroll_bottom = false;
				
				if ui.add(egui::Button::new("Calcul")).clicked() {
					self.equation.compute();
					scroll_bottom = true;
				}
				
				
				if let Some(ref impact) = self.equation.impact {
					ui.label("\nImpact du produit mutualisé dans la catégorie d'impact choisie, par an :");
					ui.label(RichText::new(format!("Im = {}",impact)).color(Color32::BLACK));
				}
				
				if let Some(ref improvement) = self.equation.improvement {
					ui.label("\nGain de la mutualisation par rapport à l'utilisation usuelle :");
					ui.label(RichText::new(format!("G% = {} %",improvement)).color(Color32::BLACK));
				}
				
				ui.label("\n\n");
				
				if scroll_bottom {
					ui.scroll_to_cursor(Some(Align::BOTTOM));
				}
			});
		});
	}
}
