class RealtimeDisplay:
    def on_change(self, weather):
        print(weather)


class WeatherStation:
    def __init__(self):
        self._observers = []

    def register(self, observer):
        self._observers.append(observer)

    def notify(self, weather):
        for observer in self._observers:
            observer.on_change(weather)
