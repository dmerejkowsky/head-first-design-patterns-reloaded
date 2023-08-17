from simuduck import mallard, red_head, rubber


def test_mallard():
    duck = mallard()
    assert duck.name == "mallard"
    assert duck.quack() == "quack"
    assert duck.fly() == "I'm flying with wings"


def test_red_head():
    duck = red_head()
    assert duck.name == "red head"
    assert duck.quack() == "quack"
    assert duck.fly() == "I'm flying with wings"


def test_rubber():
    duck = rubber()
    assert duck.name == "rubber"
    assert duck.quack() == "squeak"
    assert duck.fly() == "No way!"
