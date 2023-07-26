#![allow(dead_code)]

trait Flyable {
    fn fly(&self) -> &str;
}
trait Quackable {
    fn quack(&self) -> &str;
}

struct Duck {
    name: &'static str,
    flyable: Box<dyn Flyable>,
    quackable: Box<dyn Quackable>,
}

impl Duck {
    fn new(name: &'static str, fly: Box<dyn Flyable>, quack: Box<dyn Quackable>) -> Self {
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

fn mallard_duck() -> Duck {
    Duck::new("mallard", Box::new(FlyingWithWings), Box::new(Quack))
}

fn red_head_duck() -> Duck {
    Duck::new("red head", Box::new(FlyingWithWings), Box::new(Quack))
}

fn rubber_duck() -> Duck {
    Duck::new("rubber duck", Box::new(FlyNoWay), Box::new(Squeak))
}

fn wooden_duck() -> Duck {
    Duck::new("wooden duck", Box::new(FlyNoWay), Box::new(Silence))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mallard_duck() {
        let duck = mallard_duck();
        assert_eq!(duck.quack(), "quack");
        assert_eq!(duck.fly(), "I'm flying with wings");
        assert_eq!(duck.name(), "mallard");
    }

    #[test]
    fn test_red_head_duck() {
        let duck = red_head_duck();
        assert_eq!(duck.quack(), "quack");
        assert_eq!(duck.fly(), "I'm flying with wings");
        assert_eq!(duck.name(), "red head");
    }

    #[test]
    fn test_rubber_duck() {
        let duck = rubber_duck();
        assert_eq!(duck.quack(), "squeak");
        assert_eq!(duck.fly(), "No way");
    }

    #[test]
    fn test_wooden_duck() {
        let mut duck = wooden_duck();
        assert_eq!(duck.fly(), "No way");
        duck.flyable = Box::new(RocketFly);
        assert_eq!(duck.fly(), "I'm on a rocket");
    }
}
