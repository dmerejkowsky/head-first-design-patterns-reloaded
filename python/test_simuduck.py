from simuduck import Mallard, RedHead, RubberDuck, WoodenDuck


def test_mallard():
    duck = Mallard()
    assert duck.name == "mallard"
    assert duck.quack() == "quack"
    assert duck.fly() == "I'm flying with wings"


def test_red_head():
    duck = RedHead()
    assert duck.name == "red head"
    assert duck.quack() == "quack"
    assert duck.fly() == "I'm flying with wings"


def test_rubber():
    duck = RubberDuck()
    assert duck.name == "rubber"
    assert duck.quack() == "squeak"
    assert duck.fly() == "No way!"


def test_wooden_duck_on_a_rocket():
    duck = WoodenDuck()
    assert duck.fly() == "No way!"
    assert duck.quack() == "<silence>"
    duck.place_on_rocket()
    assert duck.fly() == "I'm flying on a rocket"
