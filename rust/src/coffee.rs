trait Beverage {
    fn cost(&self) -> u32;
}

struct DarkRoast;

impl Beverage for DarkRoast {
    fn cost(&self) -> u32 {
        10
    }
}

struct Milk {
    beverage: Box<dyn Beverage>,
}

impl Milk {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Self { beverage }
    }
}

impl Beverage for Milk {
    fn cost(&self) -> u32 {
        self.beverage.cost() + 2
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn dark_roast() {
        let beverage = DarkRoast;
        assert_eq!(beverage.cost(), 10);
    }

    #[test]
    fn dark_roast_with_milk() {
        let beverage = DarkRoast;
        let beverage = Milk::new(Box::new(beverage));
        assert_eq!(beverage.cost(), 12);
    }
}
