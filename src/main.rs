mod product;
mod visitor;
mod recipe;
mod visitors;
mod products;

use product::Product;
use recipe::Recipe;
use crate::visitor::Visitor;
use crate::visitors::{CarbohydratesVisitor, CaloriesVisitor, FatsVisitor, PriceVisitor, ProteinsVisitor};
use crate::products::{Cheese, Mushrooms, Shrimps, SourCream, HerbsAndSpices};

fn main() {
    let mut recipe = Recipe::new();

    // Добавляем ингредиенты
    let mushroom = Mushrooms::new(500.0);
    recipe.add_ingredient(&mushroom);
    let shrimps = Shrimps::new(100.0);
    recipe.add_ingredient(&shrimps);
    let sourCream = SourCream::new(30.0);
    recipe.add_ingredient(&sourCream);
    let cheese = Cheese::new(20.0);
    recipe.add_ingredient(&cheese);
    let herbsAndSpices = HerbsAndSpices::new(30.0);
    recipe.add_ingredient(&herbsAndSpices);

    // Создаем и применяем Visitors
    let calories_visitor = CaloriesVisitor;
    let proteins_visitor = ProteinsVisitor;
    let fats_visitor = FatsVisitor;
    let carbohydrates_visitor = CarbohydratesVisitor;
    let price_visitor = PriceVisitor;

    calories_visitor.visit(&recipe);
    proteins_visitor.visit(&recipe);
    fats_visitor.visit(&recipe);
    carbohydrates_visitor.visit(&recipe);
    price_visitor.visit(&recipe);
}
