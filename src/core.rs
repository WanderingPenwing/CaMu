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
	pub input: String,
	pub value: Option<f32>
}

impl Variable {
	fn new(name : String, description : String) -> Self {
		Variable {
			name,
			description,
			input: "".to_string(),
			value: Some(0.0)
		}
	}
	
	pub fn evaluate(&mut self) -> Option<f32> {
		match self.input.replace(",", ".").parse::<f32>() {
			Ok(valid_input) => {
				self.value = Some(valid_input);
			}
			Err(_) => {
				self.value = None
			}
		}
		
		self.value
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
				Variable::new("Iu".into(), "Impact du produit usuel dans la catégorie d’impact choisie, par an".into()),
				Variable::new("U%".into(), "% de l’Impact du produit usuel issu de l’utilisation uniquement".into()), 
				Variable::new("Vu".into(), "Durée de vie du produit usuel (en année)".into()), 
				Variable::new("Vm".into(), "Durée de vie du produit mutualisé (en année)".into()), 
				Variable::new("Ir".into(), "Impact moyen d'une réparation du produit dans la catégorie d'impact choisie".into()), 
				Variable::new("Nr".into(), "Nombre de réparations à effectuer du produit dans la catégorie d'impact choisie".into()), 
				Variable::new("Cu".into(), "Coefficient d'intensification de l'usage".into()),
				Variable::new("Na".into(), "Nombre d'achats évité par produit mutualisé".into())
			],
			impact: None,
			improvement: None
		}
	}
	
	pub fn compute(&mut self) {
		self.impact = None;
		self.improvement = None;
		
		let mut values: Vec<f32> = vec![];
		let mut invalid: bool = false;
		
		for variable in self.variables.iter_mut() {
			if let Some(value) = variable.evaluate() {
				values.push(value);
			} else {
				invalid = true;
			}
		}
		
		if invalid {
			return;
		}
		
		let iu = values[0];
		let up = values[1];
		let vu = values[2];
		let vm = values[3];
		let ir = values[4];
		let nr = values[5];
		let cu = values[6];
		let na = values[7];
		
		let im = iu*(up/100.0)*cu + ir*nr/vm + iu*(1.0-up/100.0)*(vu/vm);
		
		self.impact = Some(im);	
		self.improvement = Some((1.0 - im/(na*iu))*100.0);
	}
}


