class _Singleton:
    ...


_INSTANCE = _Singleton()


def get_instance():
    return _INSTANCE
