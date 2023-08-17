trait Observer {
    fn on_change(&mut self, weather: &WeatherData);
}

struct WeatherData {
    humidity: u32,
    temperature: u32,
    pressure: u32,
}

#[derive(Default)]
struct WeatherStation<'obsevers> {
    observers: Vec<&'obsevers mut dyn Observer>,
}

impl<'observers> WeatherStation<'observers> {
    fn new() -> Self {
        Default::default()
    }

    fn subscribe(&mut self, display: &'observers mut dyn Observer) {
        self.observers.push(display);
    }

    fn run(&mut self) {
        let weather_data = WeatherData {
            humidity: 80,
            temperature: 25,
            pressure: 1000,
        };
        self.notify(weather_data);
        let weather_data = WeatherData {
            humidity: 90,
            temperature: 26,
            pressure: 800,
        };
        self.notify(weather_data);
    }

    fn notify(&mut self, weather_data: WeatherData) {
        for observer in &mut self.observers {
            observer.on_change(&weather_data);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct RealTimeDisplay;

    impl Observer for RealTimeDisplay {
        fn on_change(&mut self, weather: &WeatherData) {
            let WeatherData {
                humidity,
                temperature,
                pressure,
            } = weather;
            println!("{humidity}% {temperature}deg {pressure}Pa")
        }
    }

    #[derive(Default)]
    struct StatisticsDisplay {
        temprature_sum: u32,
        temperature_count: u32,
    }

    impl StatisticsDisplay {
        fn new() -> Self {
            Default::default()
        }

        fn mean_temperature(&self) -> f32 {
            self.temprature_sum as f32 / self.temperature_count as f32
        }
    }

    impl Observer for StatisticsDisplay {
        fn on_change(&mut self, weather: &WeatherData) {
            self.temprature_sum += weather.temperature;
            self.temperature_count += 1;
        }
    }

    #[test]
    fn test_real_time_display() {
        let mut weather_station = WeatherStation::new();
        let mut realtime_display = RealTimeDisplay;
        weather_station.subscribe(&mut realtime_display);
        weather_station.run();
    }

    #[test]
    fn test_statistics() {
        let mut weather_station = WeatherStation::new();
        let mut realtime_display = RealTimeDisplay;
        let mut statistics_display = StatisticsDisplay::new();
        weather_station.subscribe(&mut realtime_display);
        weather_station.subscribe(&mut statistics_display);
        weather_station.run();
        let mean_temperature = statistics_display.mean_temperature();
        assert_eq!(mean_temperature, 25.5);
    }

    #[test]
    fn test_cannot_have_two_weather_stations_with_the_same_observers() {
        let mut w1 = WeatherStation::new();
        let mut w2 = WeatherStation::new();
        let mut realtime_display = RealTimeDisplay;
        let mut statistics_display = StatisticsDisplay::new();
        w1.subscribe(&mut realtime_display);
        w1.subscribe(&mut statistics_display);
        /* does not compile :)
        w2.subscribe(&mut realtime_display);
        w2.subscribe(&mut statistics_display);
        */
        w1.run();
        w2.run();
    }
}
