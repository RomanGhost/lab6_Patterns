pub trait Product {
    fn get_calories(&self) -> f64;
    fn get_proteins(&self) -> f64;
    fn get_fats(&self) -> f64;
    fn get_carbohydrates(&self) -> f64;
    fn get_price(&self) -> f64;
    fn get_weight(&self) -> f64;
}