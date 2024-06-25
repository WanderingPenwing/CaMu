use eframe::egui;
use image::GenericImageView;
use std::error::Error;

pub fn load_icon() -> Result<egui::IconData, Box<dyn Error>> {
	let (icon_rgba, icon_width, icon_height) = {
		let icon = include_bytes!("../assets/yubaba.ico");
		let image = image::load_from_memory(icon)?;
		let rgba = image.clone().into_rgba8().to_vec();
		let (width, height) = image.dimensions();
		(rgba, width, height)
	};

	Ok(egui::IconData {
		rgba: icon_rgba,
		width: icon_width,
		height: icon_height,
	})
}


pub struct Variable {
	pub name: String,
	pub description: String,
	pub value: f32,
	pub speed: f32,
	pub min: f32,
	pub max: f32,
}

impl Variable {
	fn new(name: String, description: String, value: f32, speed: f32, min: f32, max: f32) -> Self {
		Variable {
			name,
			description,
			value,
			speed,
			min,
			max,
		}
	}
}

pub struct Equation {
	pub variables: [Variable; 8],
	pub impact: Option<f32>,
	pub improvement: Option<f32>
}

impl Equation {
	pub fn new() -> Self {
		Equation {
			variables: [
				Variable::new("Iu".into(), "Impact du produit usuel dans la catégorie d’impact choisie, par an".into(), 0.0, 1.0, 0.0, f32::MAX),
				Variable::new("U%".into(), "% de l’Impact du produit usuel issu de l’utilisation uniquement".into(), 0.0, 1.0, 0.0, 100.0), 
				Variable::new("Vu".into(), "Durée de vie du produit usuel (en année)".into(), 0.0, 1.0, 0.0, f32::MAX), 
				Variable::new("Vm".into(), "Durée de vie du produit mutualisé (en année)".into(), 0.0, 1.0, 0.0, f32::MAX), 
				Variable::new("Ir".into(), "Impact moyen d'une réparation du produit dans la catégorie d'impact choisie".into(), 0.0, 1.0, 0.0, f32::MAX), 
				Variable::new("Nr".into(), "Nombre de réparations à effectuer du produit durant sa durée de vie".into(), 0.0, 1.0, 0.0, f32::MAX), 
				Variable::new("Cu".into(), "Coefficient d'intensification de l'usage".into(), 1.0, 0.1, 1.0, f32::MAX),
				Variable::new("Na".into(), "Nombre d'achats évité par produit mutualisé".into(), 1.0, 1.0, 1.0, f32::MAX)
			],
			impact: None,
			improvement: None
		}
	}
	
	pub fn compute(&mut self) {
		
		let iu = self.variables[0].value;
		let up = self.variables[1].value;
		let vu = self.variables[2].value;
		let vm = self.variables[3].value;
		let ir = self.variables[4].value;
		let nr = self.variables[5].value;
		let cu = self.variables[6].value;
		let na = self.variables[7].value;
		
		let im = iu*(up/100.0)*cu + ir*nr/vm + iu*(1.0-up/100.0)*(vu/vm);
		
		self.impact = Some(im);	
		self.improvement = Some((1.0 - im/(na*iu))*100.0);
	}
}


