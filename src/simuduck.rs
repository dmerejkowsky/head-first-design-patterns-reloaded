#![allow(dead_code)]
use std::fmt::Debug;

trait Fly {
    fn fly(&self) -> &str;
}
trait Quack {
    fn quack(&self) -> &str;
}

trait Duck: Fly + Quack + Debug {}

fn flying_with_wings() -> &'static str {
    "I'm flying with wings"
}

fn flying_no_way() -> &'static str {
    "No way"
}

fn quack() -> &'static str {
    "quack"
}

#[derive(Debug)]
struct MallardDuck;

impl Fly for MallardDuck {
    fn fly(&self) -> &str {
        flying_with_wings()
    }
}

impl Quack for MallardDuck {
    fn quack(&self) -> &str {
        quack()
    }
}

impl Duck for MallardDuck {}

#[derive(Debug)]
struct RedHeadDuck;

impl Duck for RedHeadDuck {}

impl Fly for RedHeadDuck {
    fn fly(&self) -> &str {
        flying_with_wings()
    }
}

impl Quack for RedHeadDuck {
    fn quack(&self) -> &str {
        quack()
    }
}

#[derive(Debug)]
struct RubberDuck;

impl Duck for RubberDuck {}

impl Fly for RubberDuck {
    fn fly(&self) -> &str {
        flying_no_way()
    }
}

impl Quack for RubberDuck {
    fn quack(&self) -> &str {
        "sqeak"
    }
}

struct WoodenDuck {
    fly: Box<dyn Fly>,
}

impl WoodenDuck {
    fn new(fly: Box<dyn Fly>) -> Self {
        Self { fly }
    }
}

impl Quack for WoodenDuck {
    fn quack(&self) -> &str {
        "<silence>"
    }
}

impl Fly for WoodenDuck {
    fn fly(&self) -> &str {
        self.fly.fly()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mallard_duck() {
        let duck = MallardDuck;
        assert_eq!(duck.quack(), "quack");
        assert_eq!(duck.fly(), "I'm flying with wings");
    }

    #[test]
    fn test_red_head_duck() {
        let duck = RedHeadDuck;
        assert_eq!(duck.quack(), "quack");
        assert_eq!(duck.fly(), "I'm flying with wings");
    }

    #[test]
    fn test_rubber_duck() {
        let duck = RubberDuck;
        assert_eq!(duck.quack(), "sqeak");
        assert_eq!(duck.fly(), "No way");
    }

    struct WoodenFly;
    impl Fly for WoodenFly {
        fn fly(&self) -> &str {
            "I can't fly right now"
        }
    }

    struct RocketFly;
    impl Fly for RocketFly {
        fn fly(&self) -> &str {
            "I'm on a rocket"
        }
    }
    #[test]
    fn test_wooden_duck() {
        let fly = WoodenFly;
        let mut duck = WoodenDuck::new(Box::new(fly));
        assert_eq!(duck.fly(), "I can't fly right now");
        duck.fly = Box::new(RocketFly);
        assert_eq!(duck.fly(), "I'm on a rocket");
    }
}
