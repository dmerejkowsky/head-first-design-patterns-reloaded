package info.dmerej.weather;

public class StatisticsDisplay implements Observer {
    private int temperatureSum;
    private int temperateCount;


    @Override
    public void onChange(WeatherData weather) {
        int temperature = weather.temperature();
        temperatureSum += temperature;
        temperateCount += 1;
    }

    public double meanTemperature() {
        return temperatureSum * 1.0 / temperateCount;
    }
}
