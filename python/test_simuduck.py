from simuduck import Mallard, RedHead, RubberDuck


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
