use crate::product::Product;

pub struct Cheese {
    weight: f64,
    calories: f64,
    proteins: f64,
    fats: f64,
    carbohydrates: f64,
    price: f64
}

impl Cheese {
    pub fn new(weight: f64) -> Self {
        Cheese {
            weight,
            calories: 345.0,
            proteins: 25.0,
            fats: 25.0,
            carbohydrates: 0.0,
            price: 70.0
        }
    }
}

impl Product for Cheese {
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
