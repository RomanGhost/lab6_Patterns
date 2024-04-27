use crate::recipe::Recipe;

pub trait Visitor {
    fn visit(&self, recipe: &Recipe);
}