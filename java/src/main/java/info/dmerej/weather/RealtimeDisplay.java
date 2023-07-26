package info.dmerej.weather;

public class RealtimeDisplay implements Observer {
    @Override
    public void onChange(WeatherData weather) {
        System.out.format("%d %%humidity %d degrees %d Pa\n", weather.humidity(), weather.temperature(), weather.pressure());
    }
}
