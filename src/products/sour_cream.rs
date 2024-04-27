use crate::product::Product;

pub struct SourCream {
    weight: f64,
    calories: f64,
    proteins: f64,
    fats: f64,
    carbohydrates: f64,
    price: f64
}

impl SourCream {
    pub fn new(weight: f64) -> Self {
        SourCream {
            weight,
            calories: 120.0,
            proteins: 3.3,
            fats: 10.0,
            carbohydrates:3.3,
            price: 14.4
        }
    }
}

impl Product for SourCream {
    fn get_calories(&self) -> f64 {
        self.calories
    }

    fn get_proteins(&self) -> f64 {
        self.proteins
    }

    fn get_fats(&self) -> f64 {
        self.fats
    }

    fn get_carbohydrates(&self) -> f64 {
        self.carbohydrates
    }

    fn get_price(&self) -> f64 {
        self.price
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }
}
