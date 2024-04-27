// Трейт для продуктов
trait Product {
    fn get_calories(&self) -> f64;
    fn get_proteins(&self) -> f64;
    fn get_fats(&self) -> f64;
    fn get_carbohydrates(&self) -> f64;
    fn get_price(&self) -> f64;
    fn get_weight(&self) -> f64;
}

// Структура для шампиньонов
struct Mushrooms {
    weight: f64,
    calories: f64,
    proteins: f64,
    fats: f64,
    carbohydrates: f64,
    price: f64
}

impl Mushrooms {
    fn new(weight: f64) -> Self {
        Mushrooms {
            weight,
            calories: 27.0,
            proteins: 4.4,
            fats: 1.0,
            carbohydrates: 0.2,
            price: 20.0
        }
    }
}

impl Product for Mushrooms {
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

pub struct SourCream {
    weight: f64,
    calories: f64,
    proteins: f64,
    fats: f64,
    carbohydrates: f64,
    price: f64
}

impl SourCream {
    fn new(weight: f64) -> Self {
        SourCream {
            weight,
            calories: 120.0,
            proteins: 3.3,
            fats: 10.0,
            carbohydrates:3.3,
            price: 14.4
        }
    }
}

impl Product for SourCream {
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

pub struct Cheese {
    weight: f64,
    calories: f64,
    proteins: f64,
    fats: f64,
    carbohydrates: f64,
    price: f64
}

impl Cheese {
    fn new(weight: f64) -> Self {
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

struct HerbsAndSpices {
    weight: f64,
    calories: f64,
    proteins: f64,
    fats: f64,
    carbohydrates: f64,
    price: f64
}

impl HerbsAndSpices {
    fn new(weight: f64) -> Self {
        HerbsAndSpices {
            weight,
            calories: 27.0,
            proteins: 4.4,
            fats: 1.0,
            carbohydrates: 0.2,
            price: 20.0
        }
    }
}

impl Product for HerbsAndSpices {
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


struct Shrimps {
    weight: f64,
    calories: f64,
    proteins: f64,
    fats: f64,
    carbohydrates: f64,
    price: f64
}

impl Shrimps {
    fn new(weight: f64) -> Self {
        Shrimps {
            weight,
            calories: 83.0,
            proteins: 18.0,
            fats: 1.0,
            carbohydrates: 0.0,
            price: 50.0
        }
    }
}

impl Product for Shrimps {
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

// Структура для рецепта блюда
struct Recipe<'a> {
    ingredients: Vec<&'a dyn Product>,
}

impl<'a> Recipe<'a> {
    fn new() -> Self {
        Recipe {
            ingredients: Vec::new(),
        }
    }

    fn add_ingredient(&mut self, ingredient: &'a dyn Product) {
        self.ingredients.push(ingredient);
    }

    fn get_total_calories(&self) -> f64 {
        self.ingredients
            .iter()
            .map(|ingredient| ingredient.get_calories() * ingredient.get_weight() / 100.0)
            .sum()
    }

    // Метод для подсчета общего количества белков в блюде
    fn get_total_proteins(&self) -> f64 {
        self.ingredients
            .iter()
            .map(|ingredient| ingredient.get_proteins() * ingredient.get_weight() / 100.0)
            .sum()
    }

    // Метод для подсчета общего количества жиров в блюде
    fn get_total_fats(&self) -> f64 {
        self.ingredients
            .iter()
            .map(|ingredient| ingredient.get_fats() * ingredient.get_weight() / 100.0)
            .sum()
    }

    // Метод для подсчета общего количества углеводов в блюде
    fn get_total_carbohydrates(&self) -> f64 {
        self.ingredients
            .iter()
            .map(|ingredient| ingredient.get_carbohydrates() * ingredient.get_weight() / 100.0)
            .sum()
    }

    // Метод для подсчета общей стоимости блюда
    fn get_total_price(&self) -> f64 {
        self.ingredients
            .iter()
            .map(|ingredient| ingredient.get_price() * ingredient.get_weight() / 100.0)
            .sum()
    }
}

// Visitor интерфейс
trait Visitor {
    fn visit(&self, recipe: &Recipe);
}

// Конкретный Visitor для подсчета калорийности блюда
struct CaloriesVisitor;

impl Visitor for CaloriesVisitor {
    fn visit(&self, recipe: &Recipe) {
        println!("Total Calories: {} kCal", recipe.get_total_calories());
    }
}

// Конкретный Visitor для подсчета количества белков в блюде
struct ProteinsVisitor;

impl Visitor for ProteinsVisitor {
    fn visit(&self, recipe: &Recipe) {
        println!("Total Proteins: {} g", recipe.get_total_proteins());
    }
}

// Конкретный Visitor для подсчета количества жиров в блюде
struct FatsVisitor;

impl Visitor for FatsVisitor {
    fn visit(&self, recipe: &Recipe) {
        println!("Total Fats: {} g", recipe.get_total_fats());
    }
}

// Конкретный Visitor для подсчета количества углеводов в блюде
struct CarbohydratesVisitor;

impl Visitor for CarbohydratesVisitor {
    fn visit(&self, recipe: &Recipe) {
        println!("Total Carbohydrates: {} g", recipe.get_total_carbohydrates());
    }
}

// Конкретный Visitor для подсчета стоимости блюда
struct PriceVisitor;

impl Visitor for PriceVisitor {
    fn visit(&self, recipe: &Recipe) {
        println!("Total Price: {} rub", recipe.get_total_price());
    }
}


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

    // Добавьте другие ингредиенты аналогично

    // Создаем и применяем Visitors
    let calories_visitor = CaloriesVisitor;
    let proteins_visitor = ProteinsVisitor;
    let fats_visitor = FatsVisitor;
    let carbohydrates_visitor = CarbohydratesVisitor;
    let price_visitor = PriceVisitor;
    calories_visitor.visit(&recipe);
    proteins_visitor.visit(&recipe); // Этот вызов будет ошибочным, так как нет метода get_total_proteins у Recipe
    fats_visitor.visit(&recipe); // Этот вызов будет ошибочным, так как нет метода get_total_fats у Recipe
    carbohydrates_visitor.visit(&recipe); // Этот вызов будет ошибочным, так как нет метода get_total_carbohydrates у Recipe
    price_visitor.visit(&recipe); // Этот вызов будет ошибочным, так как нет метода get_total_price у Recipe
}