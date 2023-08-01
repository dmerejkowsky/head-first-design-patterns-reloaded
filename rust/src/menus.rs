use std::rc::Rc;

trait MenuComponent {
    fn name(&self) -> &str;
    fn add(&mut self, component: Rc<Box<dyn MenuComponent>>);
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

    fn add(&mut self, _component: Rc<Box<dyn MenuComponent>>) {
        panic!("Unsupported operation")
    }

    fn print(&self) {
        println!("{self:?}");
    }
}

struct Menu {
    name: String,
    components: Vec<Rc<Box<dyn MenuComponent>>>,
}

impl Menu {
    fn new(name: String, components: Vec<Rc<Box<dyn MenuComponent>>>) -> Self {
        Self { name, components }
    }
}

impl MenuComponent for Menu {
    fn name(&self) -> &str {
        &self.name
    }

    fn add(&mut self, component: Rc<Box<dyn MenuComponent>>) {
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
    let pasta = MenuItem::new("pasta".to_owned(), 4);
    let pasta: Rc<Box<dyn MenuComponent>> = Rc::new(Box::new(pasta));

    let waffles = MenuItem::new("waffles".to_owned(), 4);
    let waffles: Rc<Box<dyn MenuComponent>> = Rc::new(Box::new(waffles));

    let apple_pie = MenuItem::new("Apple pie".to_owned(), 3);
    let apple_pie: Rc<Box<dyn MenuComponent>> = Rc::new(Box::new(apple_pie));

    let diner = Menu::new(
        "Diner".to_owned(),
        vec![
            Rc::clone(&pasta),
            Rc::clone(&waffles),
            Rc::clone(&apple_pie),
        ],
    );
    let diner: Rc<Box<dyn MenuComponent>> = Rc::new(Box::new(diner));

    let breakfast = Menu::new("Breakfast".to_owned(), vec![Rc::clone(&waffles)]);
    let breakfast: Rc<Box<dyn MenuComponent>> = Rc::new(Box::new(breakfast));

    let all_menus = Menu::new(
        "All menus".to_owned(),
        vec![Rc::clone(&diner), Rc::clone(&breakfast)],
    );
    all_menus.print()
}
