use std::collections::HashMap;

trait MenuComponent {
    fn name(&self) -> &str;
    fn add(&mut self, component: Box<dyn MenuComponent>);
    fn print(&self);
}

#[derive(Debug)]
struct MenuItem {
    name: String,
    price: u32,
}

impl MenuItem {
    fn new(name: String, price: u32) -> Self {
        Self { name, price }
    }
}

impl MenuComponent for MenuItem {
    fn name(&self) -> &str {
        &self.name
    }

    fn add(&mut self, _component: Box<dyn MenuComponent>) {
        panic!("Unsupported operation")
    }

    fn print(&self) {
        println!("{self:?}");
    }
}

struct Menu {
    name: String,
    components: Vec<Box<dyn MenuComponent>>,
}

impl Menu {
    fn new(name: String, components: Vec<Box<dyn MenuComponent>>) -> Self {
        Self { name, components }
    }
}

impl MenuComponent for Menu {
    fn name(&self) -> &str {
        &self.name
    }

    fn add(&mut self, component: Box<dyn MenuComponent>) {
        self.components.push(component);
    }

    fn print(&self) {
        println!("{}", self.name);
        for component in &self.components {
            component.print()
        }
    }
}

pub fn run() {
    let pasta = MenuItem::new("Pasta".to_owned(), 4);
    let apple_pie = MenuItem::new("Apple pie".to_owned(), 3);
    let waffles = MenuItem::new("Waffles".to_owned(), 2);
    let desert_menu = Menu::new(
        "Deserts".to_owned(),
        vec![Box::new(apple_pie), Box::new(waffles)],
    );
    let diner_menu = Menu::new(
        "Diner".to_owned(),
        vec![Box::new(pasta), Box::new(desert_menu)],
    );
    diner_menu.print();
    // Does not compile :(
    // let breakfast = Menu::new("Breakfast".to_owned(), vec![Box::new(waffles)]);
}
