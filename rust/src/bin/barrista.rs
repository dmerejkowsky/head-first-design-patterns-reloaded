use head_first_design_patterns::barrista::*;

fn main() {
    let mut tea = Tea;
    tea.prepare_recipe();
    println!("---");
    let mut coffee = Coffee;
    coffee.prepare_recipe();
}
