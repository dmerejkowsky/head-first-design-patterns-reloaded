#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum BaseName {
    Cheese,
    Pepperoni,
}

trait Base {
    fn name(&self) -> BaseName;
    fn prepare(&self);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cheese {
    Mozarella,
    Reggiano,
}

struct CheeseBase {
    cheese: Cheese,
}

impl Base for CheeseBase {
    fn name(&self) -> BaseName {
        BaseName::Cheese
    }

    fn prepare(&self) {
        println!("Adding {:?}", self.cheese);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dough {
    ThickCrust,
    ThinCrust,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pepperoni {
    SlicedPepperoni, // The best in the country
}

struct PepperoniBase {
    pepperoni: Pepperoni,
}

impl Base for PepperoniBase {
    fn name(&self) -> BaseName {
        BaseName::Pepperoni
    }

    fn prepare(&self) {
        println!("Adding {:?}", self.pepperoni);
    }
}

struct Pizza {
    dough: Dough,
    base: Box<dyn Base>,
}

impl Pizza {
    fn new(base: Box<dyn Base>, ingredients: &dyn Ingredients) -> Self {
        let dough = ingredients.get_dough();
        Self { dough, base }
    }

    fn base_name(&self) -> BaseName {
        self.base.name()
    }

    fn bake(&mut self) {
        println!("Baking");
    }

    fn cut(&mut self) {
        println!("Cutting");
    }

    fn r#box(&self) {
        println!("Putting in box");
    }
}

struct Shop<I: Ingredients> {
    ingredients: I,
}

impl<I: Ingredients> Shop<I> {
    fn create_pizza(&self, name: BaseName) -> Pizza {
        let base = get_base(&self.ingredients, name);
        Pizza::new(base, &self.ingredients)
    }

    fn order_pizza(&self, name: BaseName) -> Pizza {
        let mut pizza = self.create_pizza(name);
        pizza.bake();
        pizza.cut();
        pizza.r#box();
        pizza
    }
}

trait Ingredients {
    fn get_dough(&self) -> Dough;
    fn get_cheese(&self) -> Cheese;
    fn get_pepperoni(&self) -> Pepperoni {
        Pepperoni::SlicedPepperoni
    }
}

struct ChicagoIngredients;

impl Ingredients for ChicagoIngredients {
    fn get_dough(&self) -> Dough {
        Dough::ThickCrust
    }

    fn get_cheese(&self) -> Cheese {
        Cheese::Mozarella
    }
}

struct NewYorkIngredients;

impl Ingredients for NewYorkIngredients {
    fn get_dough(&self) -> Dough {
        Dough::ThinCrust
    }

    fn get_cheese(&self) -> Cheese {
        Cheese::Reggiano
    }
}

fn get_base(ingredients: &dyn Ingredients, name: BaseName) -> Box<dyn Base> {
    let cheese = ingredients.get_cheese();
    let pepperoni = ingredients.get_pepperoni();
    let base: Box<dyn Base> = match name {
        BaseName::Cheese => Box::new(CheeseBase { cheese }),
        BaseName::Pepperoni => Box::new(PepperoniBase { pepperoni }),
    };
    base.prepare();
    base
}

type NewYorkShop = Shop<NewYorkIngredients>;
type ChicagoShop = Shop<ChicagoIngredients>;

impl NewYorkShop {
    fn new() -> Self {
        Self {
            ingredients: NewYorkIngredients,
        }
    }
}

impl ChicagoShop {
    fn new() -> Self {
        Self {
            ingredients: ChicagoIngredients,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cheese_pizza_in_new_york() {
        let shop = NewYorkShop::new();
        let pizza = shop.order_pizza(BaseName::Cheese);
        assert_eq!(pizza.base_name(), BaseName::Cheese);
        assert_eq!(pizza.dough, Dough::ThinCrust);
    }

    #[test]
    fn test_pepperoni_pizza_in_chicago() {
        let shop = ChicagoShop::new();
        let pizza = shop.order_pizza(BaseName::Pepperoni);
        assert_eq!(pizza.base_name(), BaseName::Pepperoni);
        assert_eq!(pizza.dough, Dough::ThickCrust);
    }
}
