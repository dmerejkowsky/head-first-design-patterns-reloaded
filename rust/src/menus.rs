use std::{collections::HashMap, fmt::Debug, io::IntoInnerError};

trait Menu {
    fn get_items(&self) -> &[MenuItem];
}

#[derive(Debug)]
struct MenuItem {
    name: String,
    price: u32,
}

struct PancakesMenu {
    items: [MenuItem; 6],
}

impl Menu for PancakesMenu {
    fn get_items(&self) -> &[MenuItem] {
        &self.items
    }
}

struct DinnerMenu {
    items: Vec<MenuItem>,
}

impl Menu for DinnerMenu {
    fn get_items(&self) -> &[MenuItem] {
        &self.items
    }
}

struct CoffeeMenu {
    items: HashMap<String, MenuItem>,
}

struct CoffeeMenuIterator<'a> {
    values: Vec<&'a MenuItem>,
}

impl<'a> CoffeeMenuIterator<'a> {
    fn new(menu: &'a CoffeeMenu) -> Self {
        Self {
            values: menu.items.values().collect(),
        }
    }
}

impl<'a> Iterator for CoffeeMenuIterator<'a> {
    type Item = &'a MenuItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.pop()
    }
}

impl Menu for CoffeeMenu {
    fn get_items(&self) -> &[MenuItem] {
        let values = self.items.values();
        let v: Vec<_> = values.into_iter().collect();
        todo!()
    }
}

impl<'a> IntoIterator for CoffeeMenu {
    type Item = MenuItem;

    type IntoIter<'a> = CoffeeMenuIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CoffeeMenuIterator::new(&self)
    }
}

fn print_menu(menu: &dyn Menu) {
    for item in menu.get_items() {
        println!("{item:?}");
    }
}

fn print_menu2(menu: CoffeeMenu) {
    for item in menu {}
}
