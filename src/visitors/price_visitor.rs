use crate::recipe::Recipe;
use crate::visitor::Visitor;
pub struct PriceVisitor;

impl Visitor for PriceVisitor {
    fn visit(&self, recipe: &Recipe) {
        println!("Total Price: {} rub", recipe.get_total_price());
    }
}