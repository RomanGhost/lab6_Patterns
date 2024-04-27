use crate::recipe::Recipe;
use crate::visitor::Visitor;

pub struct ProteinsVisitor;

impl Visitor for ProteinsVisitor {
    fn visit(&self, recipe: &Recipe) {
        println!("Total Proteins: {} g", recipe.get_total_proteins());
    }
}