use std::rc::Rc;

#[derive(Debug)]
struct MenuItem {
    name: &'static str,
    price: u32,
}

impl MenuItem {
    fn new(name: &'static str, price: u32) -> Self {
        Self { name, price }
    }
}

enum Component {
    Item(MenuItem),
    Menu(Menu),
}

impl Component {
    fn print(&self) {
        match self {
            Component::Menu(m) => m.print(),
            Component::Item(i) => println!("{} {}", i.name, i.price),
        }
    }
}

struct Menu {
    name: &'static str,
    components: Vec<Rc<Component>>,
}

impl Menu {
    fn new(name: &'static str, components: Vec<Rc<Component>>) -> Self {
        Self { name, components }
    }

    fn print(&self) {
        println!("{}", self.name);
        for component in &self.components {
            component.print()
        }
    }
}

pub fn run() {
    let pasta = Rc::new(Component::Item(MenuItem::new("pasta", 4)));
    let waffles = Rc::new(Component::Item(MenuItem::new("waffles", 2)));
    let apple_pie = Rc::new(Component::Item(MenuItem::new("Apple pie", 3)));
    let diner = Component::Menu(Menu::new(
        "Diner",
        vec![
            Rc::clone(&pasta),
            Rc::clone(&waffles),
            Rc::clone(&apple_pie),
        ],
    ));

    let breakfast = Component::Menu(Menu::new("Breakfast", vec![Rc::clone(&waffles)]));

    let all_menus = Menu::new("All menus", vec![Rc::new(diner), Rc::new(breakfast)]);

    all_menus.print();
}
