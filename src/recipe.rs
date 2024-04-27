use crate::product::Product;

pub struct Recipe<'a> {
    ingredients: Vec<&'a dyn Product>,
}

impl<'a> Recipe<'a> {
    pub fn new() -> Self {
        Recipe {
            ingredients: Vec::new(),
        }
    }

    pub fn add_ingredient(&mut self, ingredient: &'a dyn Product) {
        self.ingredients.push(ingredient);
    }

    pub fn get_total_calories(&self) -> f64 {
        self.ingredients
            .iter()
            .map(|ingredient| ingredient.get_calories() * ingredient.get_weight() / 100.0)
            .sum()
    }

    // Метод для подсчета общего количества белков в блюде
    pub fn get_total_proteins(&self) -> f64 {
        self.ingredients
            .iter()
            .map(|ingredient| ingredient.get_proteins() * ingredient.get_weight() / 100.0)
            .sum()
    }

    // Метод для подсчета общего количества жиров в блюде
    pub fn get_total_fats(&self) -> f64 {
        self.ingredients
            .iter()
            .map(|ingredient| ingredient.get_fats() * ingredient.get_weight() / 100.0)
            .sum()
    }

    // Метод для подсчета общего количества углеводов в блюде
    pub fn get_total_carbohydrates(&self) -> f64 {
        self.ingredients
            .iter()
            .map(|ingredient| ingredient.get_carbohydrates() * ingredient.get_weight() / 100.0)
            .sum()
    }

    // Метод для подсчета общей стоимости блюда
    pub fn get_total_price(&self) -> f64 {
        self.ingredients
            .iter()
            .map(|ingredient| ingredient.get_price() * ingredient.get_weight() / 100.0)
            .sum()
    }
}