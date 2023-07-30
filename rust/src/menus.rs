use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct MenuItem {
    price: u32,
}

struct PancakesMenu {
    items: [MenuItem; 6],
    current_pos: usize,
}

impl Iterator for PancakesMenu {
    type Item = MenuItem;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.items.get(self.current_pos);
        self.current_pos += 1;
        res.copied()
    }
}

struct DinnerMenu {
    items: Vec<MenuItem>,
    current_pos: usize,
}

impl Iterator for DinnerMenu {
    type Item = MenuItem;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.items.get(self.current_pos);
        self.current_pos += 1;
        res.copied()
    }
}

struct CoffeeMenu {
    items: HashMap<String, MenuItem>,
    values: Vec<MenuItem>,
    current_pos: usize,
}

impl Iterator for CoffeeMenu {
    type Item = MenuItem;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.values.get(self.current_pos);
        self.current_pos += 1;
        res.copied()
    }
}

trait Menu {
    fn get_items(&self) -> Vec<&MenuItem>;
}

impl Menu for PancakesMenu {
    fn get_items(&self) -> Vec<&MenuItem> {
        self.items.iter().collect()
    }
}

impl Menu for DinnerMenu {
    fn get_items(&self) -> Vec<&MenuItem> {
        self.items.iter().collect()
    }
}

impl Menu for CoffeeMenu {
    fn get_items(&self) -> Vec<&MenuItem> {
        self.items.values().into_iter().collect()
    }
}

fn print_menus(menu: &dyn Menu) {
    for item in menu.get_items() {
        dbg!(item);
    }
}

fn print_menus2(menu: &mut dyn Iterator<Item = MenuItem>) {
    for item in menu {
        dbg!(item);
    }
}
