use crate::recipe::Recipe;
use crate::visitor::Visitor;
pub struct CaloriesVisitor;

impl Visitor for CaloriesVisitor {
    fn visit(&self, recipe: &Recipe) {
        println!("Total Calories: {} kCal", recipe.get_total_calories());
    }
}