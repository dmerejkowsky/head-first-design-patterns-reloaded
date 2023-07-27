#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum BaseName {
    Cheese,
    Pepperoni,
}

trait Base {
    fn name(&self) -> BaseName;
    fn prepare(&self);
}

struct Cheese {}

impl Base for Cheese {
    fn name(&self) -> BaseName {
        BaseName::Cheese
    }

    fn prepare(&self) {
        println!("Adding some cheese");
    }
}

struct Pepperoni {}

impl Base for Pepperoni {
    fn name(&self) -> BaseName {
        BaseName::Pepperoni
    }

    fn prepare(&self) {
        println!("Adding some pepperoni");
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Crust {
    Thick,
    Thin,
}

struct Pizza {
    base: Box<dyn Base>,
    crust: Crust,
}

impl Pizza {
    fn new(base: Box<dyn Base>, crust: Crust) -> Self {
        Self { base, crust }
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

trait Shop {
    fn create_pizza(&self, name: BaseName) -> Pizza;
    fn order_pizza(&self, name: BaseName) -> Pizza {
        let mut pizza = self.create_pizza(name);
        pizza.bake();
        pizza.cut();
        pizza.r#box();
        pizza
    }
}

struct NewYorkShop;

impl Shop for NewYorkShop {
    fn create_pizza(&self, name: BaseName) -> Pizza {
        let base = get_base(name);
        Pizza::new(base, Crust::Thick)
    }
}

struct ChicagoShop;

impl Shop for ChicagoShop {
    fn create_pizza(&self, name: BaseName) -> Pizza {
        let base = get_base(name);
        Pizza::new(base, Crust::Thin)
    }
}

fn get_base(name: BaseName) -> Box<dyn Base> {
    let base: Box<dyn Base> = match name {
        BaseName::Cheese => Box::new(Cheese {}),
        BaseName::Pepperoni => Box::new(Pepperoni {}),
    };
    base.prepare();
    base
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cheese_pizza_in_new_york() {
        let shop = NewYorkShop;
        let pizza = shop.order_pizza(BaseName::Cheese);
        assert_eq!(pizza.base_name(), BaseName::Cheese);
        assert_eq!(pizza.crust, Crust::Thick);
    }

    #[test]
    fn test_pepperoni_pizza_in_chicago() {
        let shop = ChicagoShop;
        let pizza = shop.order_pizza(BaseName::Pepperoni);
        assert_eq!(pizza.base_name(), BaseName::Pepperoni);
        assert_eq!(pizza.crust, Crust::Thin);
    }
}
