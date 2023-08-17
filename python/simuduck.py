class _Duck:
    def __init__(self, name, flying, quacking):
        self.name = name
        self._flying = flying
        self._quacking = quacking

    def fly(self):
        return self._flying(self)

    def quack(self):
        return self._quacking(self)


def _build_duck(name, flying, quacking):
    return _Duck(name, flying, quacking)


def wing_flying(duck):
    return "I'm flying with wings"


def default_quacking(duck):
    return "quack"


def mallard():
    return _build_duck("mallard", wing_flying, default_quacking)


def red_head():
    return _build_duck("red head", wing_flying, default_quacking)


def no_way_flying(duck):
    return "No way!"


def rubber():
    return _build_duck(
        "rubber",
        no_way_flying,
        lambda _: "squeak",
    )
