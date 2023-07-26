package info.dmerej.weather;

import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

class WeatherStationTests {
    @Test
    void realtime_display() {
        var weatherStation = new WeatherStation();
        var display = new RealtimeDisplay();
        weatherStation.subscribe(display);
        weatherStation.run();
    }

    @Test
    void statistics_display() {
        var weatherStation = new WeatherStation();
        var realtimeDisplay = new RealtimeDisplay();
        var statisticsDisplay = new StatisticsDisplay();
        weatherStation.subscribe(realtimeDisplay);
        weatherStation.subscribe(statisticsDisplay);
        weatherStation.run();

        assertThat(statisticsDisplay.meanTemperature()).isEqualTo(26);
    }

    @Test
    void two_weather_stations() {
        // This compiles but you get weird results ...
        var w1 = new WeatherStation();
        var w2 = new WeatherStation();
        var realtimeDisplay = new RealtimeDisplay();
        w1.subscribe(realtimeDisplay);
        w2.subscribe(realtimeDisplay);
        w1.run();
        w2.run();
    }
}