use std::collections::HashMap;

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

trait Menu {
    fn items<'s>(&'s self) -> Box<dyn Iterator<Item = &MenuItem> + 's>;
}

struct CoffeeMenu {
    items: HashMap<String, MenuItem>,
}

impl CoffeeMenu {
    fn new(items: HashMap<String, MenuItem>) -> Self {
        Self { items }
    }
}

impl Menu for CoffeeMenu {
    fn items<'s>(&'s self) -> Box<dyn Iterator<Item = &MenuItem> + 's> {
        Box::new(self.items.values().into_iter())
    }
}

struct DinerMenu {
    items: Vec<MenuItem>,
}

impl DinerMenu {
    fn new(items: Vec<MenuItem>) -> Self {
        Self { items }
    }
}

impl Menu for DinerMenu {
    fn items<'s>(&'s self) -> Box<dyn Iterator<Item = &MenuItem> + 's> {
        Box::new(self.items.iter())
    }
}
struct PancakesMenu {
    items: [MenuItem; 3],
}

impl PancakesMenu {
    fn new(items: [MenuItem; 3]) -> Self {
        Self { items }
    }
}

impl Menu for PancakesMenu {
    fn items<'s>(&'s self) -> Box<dyn Iterator<Item = &MenuItem> + 's> {
        Box::new(self.items.iter())
    }
}

fn print_menus(menus: Vec<Box<dyn Menu>>) {
    for menu in menus.iter() {
        for item in menu.items() {
            println!("{item:?}");
        }
    }
}

pub fn run() {
    let diner = DinerMenu::new(vec![
        MenuItem::new("A".to_owned(), 2),
        MenuItem::new("B".to_owned(), 3),
    ]);

    let mut coffees = HashMap::new();
    coffees.insert("tall".to_owned(), MenuItem::new("tall".to_owned(), 2));
    coffees.insert("small".to_owned(), MenuItem::new("tall".to_owned(), 1));
    let coffee = CoffeeMenu::new(coffees);

    let pancakes_menu = PancakesMenu::new([
        MenuItem::new("x".to_string(), 10),
        MenuItem::new("y".to_string(), 11),
        MenuItem::new("z".to_string(), 12),
    ]);

    let menus: Vec<Box<dyn Menu>> =
        vec![Box::new(diner), Box::new(coffee), Box::new(pancakes_menu)];
    print_menus(menus);
}
