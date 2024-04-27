use crate::recipe::Recipe;
use crate::visitor::Visitor;

pub struct FatsVisitor;

impl Visitor for FatsVisitor {
    fn visit(&self, recipe: &Recipe) {
        println!("Total Fats: {} g", recipe.get_total_fats());
    }
}