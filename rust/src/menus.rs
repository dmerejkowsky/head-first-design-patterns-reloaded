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
    name: String,
    components: Vec<Component>,
}

impl Menu {
    fn new(name: String, components: Vec<Component>) -> Self {
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
    let pasta = Component::Item(MenuItem::new("pasta".to_owned(), 4));
    let waffles = Component::Item(MenuItem::new("waffles".to_owned(), 2));
    let apple_pie = Component::Item(MenuItem::new("Apple pie".to_owned(), 3));
    let diner = Menu::new("Diner".to_owned(), vec![pasta, waffles, apple_pie]);
    diner.print();

    let breakfast = Menu::new("Breakfast".to_owned(), vec![waffles]);
}
