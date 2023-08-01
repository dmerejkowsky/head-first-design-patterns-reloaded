use std::rc::Rc;

#[derive(Debug)]
struct Item {
    name: &'static str,
    price: u32,
}

impl Item {
    fn new(name: &'static str, price: u32) -> Self {
        Self { name, price }
    }
}

enum Component {
    Item(Item),
    Menu(Menu),
}

impl Component {
    fn print(&self) {
        match self {
            Component::Menu(m) => m.print(),
            Component::Item(i) => println!("{} {}", i.name, i.price),
        }
    }

    fn new_item(name: &'static str, price: u32) -> Self {
        Self::Item(Item::new(name, price))
    }

    fn new_menu(name: &'static str, components: &[&Rc<Component>]) -> Self {
        let components: Vec<_> = components.iter().map(|x| Rc::clone(x)).collect();
        Self::Menu(Menu::new(name, components))
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
    let pasta = Rc::new(Component::new_item("pasta", 4));
    let waffles = Rc::new(Component::new_item("waffles", 2));
    let apple_pie = Rc::new(Component::new_item("Apple pie", 3));
    let diner = Rc::new(Component::new_menu(
        "Diner",
        &[&pasta, &waffles, &apple_pie],
    ));
    let breakfast = Rc::new(Component::new_menu("Breakfast", &[&waffles]));

    let all_menus = Component::new_menu("All menus", &[&diner, &breakfast]);
    all_menus.print();
}
