pub trait HotBeverage {
    fn boil_water(&mut self) {
        println!("Boiling water");
    }

    fn pour_in_cup(&mut self) {
        println!("Pouring in cup");
    }

    fn brew(&mut self);

    fn add_condiments(&mut self);

    fn customer_wants_condiments(&self) -> bool {
        true
    }

    fn prepare_recipe(&mut self) {
        self.boil_water();
        self.brew();
        if self.customer_wants_condiments() {
            self.add_condiments();
        }
    }
}

pub struct Tea;
impl HotBeverage for Tea {
    fn brew(&mut self) {
        println!("Steering tea");
    }

    fn add_condiments(&mut self) {
        println!("Adding lemon");
    }
}

pub struct Coffee;
impl HotBeverage for Coffee {
    fn brew(&mut self) {
        println!("Grinding beans");
    }

    fn add_condiments(&mut self) {
        println!("Adding milk and sugar");
    }

    fn customer_wants_condiments(&self) -> bool {
        false
    }
}
