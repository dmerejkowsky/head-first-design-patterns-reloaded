#![allow(dead_code)]
#![allow(clippy::new_ret_no_self)]

trait Flyable {
    fn fly(&self) -> &str;
}
trait Quackable {
    fn quack(&self) -> &str;
}

struct Duck<F: Flyable, Q: Quackable> {
    name: &'static str,
    flyable: F,
    quackable: Q,
}

impl<F: Flyable, Q: Quackable> Duck<F, Q> {
    fn new(name: &'static str, fly: F, quack: Q) -> Self {
        Self {
            name,
            flyable: fly,
            quackable: quack,
        }
    }

    fn fly(&self) -> &str {
        self.flyable.fly()
    }

    fn quack(&self) -> &str {
        self.quackable.quack()
    }

    fn name(&self) -> &str {
        self.name
    }
}

struct FlyingWithWings;
impl Flyable for FlyingWithWings {
    fn fly(&self) -> &str {
        "I'm flying with wings"
    }
}

struct FlyNoWay;
impl Flyable for FlyNoWay {
    fn fly(&self) -> &str {
        "No way"
    }
}

struct Quack;
impl Quackable for Quack {
    fn quack(&self) -> &str {
        "quack"
    }
}

struct Silence;
impl Quackable for Silence {
    fn quack(&self) -> &str {
        "<silence>"
    }
}

struct Squeak;
impl Quackable for Squeak {
    fn quack(&self) -> &str {
        "squeak"
    }
}

struct WoodenFly;
impl Flyable for WoodenFly {
    fn fly(&self) -> &str {
        "I can't fly right now"
    }
}

struct RocketFly;
impl Flyable for RocketFly {
    fn fly(&self) -> &str {
        "I'm on a rocket"
    }
}

struct MallardDuck;
impl MallardDuck {
    fn new() -> Duck<FlyingWithWings, Quack> {
        Duck::new("mallard", FlyingWithWings, Quack)
    }
}

struct RedHeadDuck;
impl RedHeadDuck {
    fn new() -> Duck<FlyingWithWings, Quack> {
        Duck::new("red head", FlyingWithWings, Quack)
    }
}
struct RubberDuck;
impl RubberDuck {
    fn new() -> Duck<FlyNoWay, Squeak> {
        Duck::new("rubber", FlyNoWay, Squeak)
    }
}

struct WoodenDuck;
impl WoodenDuck {
    fn new() -> Duck<FlyNoWay, Silence> {
        Duck::new("wooden", FlyNoWay, Silence)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mallard_duck() {
        let duck = MallardDuck::new();
        assert_eq!(duck.quack(), "quack");
        assert_eq!(duck.fly(), "I'm flying with wings");
        assert_eq!(duck.name(), "mallard");
    }

    #[test]
    fn test_red_head_duck() {
        let duck = RedHeadDuck::new();
        assert_eq!(duck.quack(), "quack");
        assert_eq!(duck.fly(), "I'm flying with wings");
        assert_eq!(duck.name(), "red head");
    }

    #[test]
    fn test_rubber_duck() {
        let duck = RubberDuck::new();
        assert_eq!(duck.quack(), "squeak");
        assert_eq!(duck.fly(), "No way");
    }

    #[test]
    fn test_wooden_duck() {
        let duck = WoodenDuck::new();
        assert_eq!(duck.fly(), "No way");
        assert_eq!(duck.quack(), "<silence>");
    }
}
