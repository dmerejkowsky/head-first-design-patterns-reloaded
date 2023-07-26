package info.dmerej.weather;

import java.util.ArrayList;
import java.util.List;

public class WeatherStation {
    private final List<Observer> observers = new ArrayList<>();

    public void subscribe(Observer observer) {
        observers.add(observer);
    }

    public void onChange(WeatherData weather) {
        for (var observer : observers) {
            observer.onChange(weather);
        }
    }

    public void run() {
        var weather = new WeatherData(25, 1000, 90);
        onChange(weather);
        var weather2 = new WeatherData(27, 900, 95);
        onChange(weather2);
    }
}
