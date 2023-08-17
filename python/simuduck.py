class Duck:
    def __init__(self, name, flying, quacking):
        self.name = name
        self._flying = flying
        self._quacking = quacking

    def fly(self):
        return self._flying(self)

    def quack(self):
        return self._quacking(self)


def wing_flying(duck):
    return "I'm flying with wings"


def default_quacking(duck):
    return "quack"


class Mallard(Duck):
    def __init__(self):
        super().__init__("mallard", wing_flying, default_quacking)


class RedHead(Duck):
    def __init__(self):
        super().__init__("red head", wing_flying, default_quacking)


def no_way_flying(duck):
    return "No way!"


def squeaking(duck):
    return "squeak"


class RubberDuck(Duck):
    def __init__(self):
        super().__init__("rubber", no_way_flying, squeaking)
