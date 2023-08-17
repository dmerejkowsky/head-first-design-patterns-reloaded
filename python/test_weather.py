from weather import RealtimeDisplay, WeatherStation


def run(station):
    weather = {
        "humidity": 80,
        "temperature": 25,
        "pressure": 1000,
    }
    station.notify(weather)

    weather = {
        "humidity": 90,
        "temperature": 26,
        "pressure": 800,
    }
    station.notify(weather)


def test_realtime_display():
    display = RealtimeDisplay()
    station = WeatherStation()
    station.register(display)
    run(station)
