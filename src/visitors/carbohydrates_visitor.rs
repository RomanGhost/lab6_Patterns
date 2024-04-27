use crate::recipe::Recipe;
use crate::visitor::Visitor;

pub struct CarbohydratesVisitor;

impl Visitor for CarbohydratesVisitor {
    fn visit(&self, recipe: &Recipe) {
        println!("Total Carbohydrates: {} g", recipe.get_total_carbohydrates());
    }
}