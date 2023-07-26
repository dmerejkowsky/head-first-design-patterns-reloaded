trait Observer {
    fn on_humidity_changed(&mut self, humidity: u32);
    fn on_temperature_changed(&mut self, temperature: u32);
    fn on_pressure_changed(&mut self, pressure: u32);
}

struct WeatherData {
    humidity: u32,
    temperature: u32,
    pressure: u32,
}

#[derive(Default)]
struct WeatherStation<'o> {
    displays: Vec<Box<&'o mut dyn Observer>>,
}

impl<'o> WeatherStation<'o> {
    fn new() -> Self {
        Default::default()
    }

    fn subscribe(&mut self, display: Box<&'o mut dyn Observer>) {
        self.displays.push(display);
    }

    fn run(&mut self) {
        let weather_data = WeatherData {
            humidity: 80,
            temperature: 25,
            pressure: 1000,
        };
        self.update(weather_data);
        let weather_data = WeatherData {
            humidity: 90,
            temperature: 26,
            pressure: 800,
        };
        self.update(weather_data);
    }

    fn update(&mut self, weather_data: WeatherData) {
        for display in &mut self.displays {
            display.on_humidity_changed(weather_data.humidity);
            display.on_temperature_changed(weather_data.temperature);
            display.on_pressure_changed(weather_data.pressure);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct RealTimeDisplay;

    impl Observer for RealTimeDisplay {
        fn on_humidity_changed(&mut self, humidity: u32) {
            println!("{humidity}");
        }

        fn on_temperature_changed(&mut self, temperature: u32) {
            println!("{temperature}");
        }

        fn on_pressure_changed(&mut self, pressure: u32) {
            println!("{pressure}");
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
        fn on_humidity_changed(&mut self, _humidity: u32) {}
        fn on_pressure_changed(&mut self, _pressure: u32) {}

        fn on_temperature_changed(&mut self, temperature: u32) {
            self.temprature_sum += temperature;
            self.temperature_count += 1;
        }
    }

    #[test]
    fn test_real_time_display() {
        let mut weather_station = WeatherStation::new();
        let mut realtime_display = RealTimeDisplay;
        weather_station.subscribe(Box::new(&mut realtime_display));
        weather_station.run();
    }

    #[test]
    fn test_statistics() {
        let mut weather_station = WeatherStation::new();
        let mut realtime_display = RealTimeDisplay;
        let mut statistics_display = StatisticsDisplay::new();
        weather_station.subscribe(Box::new(&mut realtime_display));
        weather_station.subscribe(Box::new(&mut statistics_display));
        weather_station.run();
        let mean_temperature = statistics_display.mean_temperature();
        assert_eq!(mean_temperature, 25.5);
    }
}
