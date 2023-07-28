#![warn(dead_code)]

use std::collections::HashMap;
struct Light {
    location: &'static str,
    on: bool,
}

impl Light {
    fn new(location: &'static str) -> Self {
        Self {
            location,
            on: false,
        }
    }

    fn on(&mut self) {
        println!("Turning {} light on", self.location);
        self.on = true
    }

    fn off(&mut self) {
        println!("Turning {} light off", self.location);
        self.on = false
    }

    fn status(&self) -> LightStatus {
        match self.on {
            true => LightStatus::On,
            false => LightStatus::Off,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CeilingSpeed {
    Off,
    Low,
    Medium,
    High,
}

struct CeilingFan {
    speed: CeilingSpeed,
}

impl CeilingFan {
    fn new() -> Self {
        Self {
            speed: CeilingSpeed::Off,
        }
    }

    fn set_speed(&mut self, speed: CeilingSpeed) {
        self.speed = speed;
        println!("CeilingFan at {speed:?}");
    }

    fn speed(&self) -> CeilingSpeed {
        self.speed
    }
}

struct Home {
    lights: HashMap<&'static str, Light>,
    ceiling_fan: CeilingFan,
    stereo: Stereo,
}

impl Home {
    fn new() -> Self {
        let mut lights = HashMap::new();
        lights.insert("kitchen", Light::new("kitchen"));
        lights.insert("bathroom", Light::new("bathroom"));
        lights.insert("livingroom", Light::new("livingroom"));
        Self {
            lights,
            stereo: Stereo::new(),
            ceiling_fan: CeilingFan::new(),
        }
    }
}

trait Command {
    fn execute(&mut self, home: &mut Home);
    fn undo(&mut self, home: &mut Home);
    fn mutate_me(&mut self) {
        println!("Please help, I'm being mutated");
    }
}

struct NoOp;

impl Command for NoOp {
    fn execute(&mut self, _home: &mut Home) {}

    fn undo(&mut self, _home: &mut Home) {}
}

enum LightStatus {
    On,
    Off,
}

struct LightCommand {
    location: &'static str,
    value: LightStatus,
    last_value: LightStatus,
}

impl LightCommand {
    fn new(location: &'static str, value: LightStatus) -> Self {
        Self {
            location,
            value,
            last_value: LightStatus::Off,
        }
    }
}

impl Command for LightCommand {
    fn execute(&mut self, home: &mut Home) {
        let light = match home.lights.get_mut(self.location) {
            None => {
                eprint!("Light not found");
                return;
            }
            Some(l) => l,
        };
        self.last_value = light.status();
        match self.value {
            LightStatus::On => light.on(),
            LightStatus::Off => light.off(),
        }
    }

    fn undo(&mut self, home: &mut Home) {
        let light = match home.lights.get_mut(self.location) {
            None => {
                eprint!("Light not found");
                return;
            }
            Some(l) => l,
        };
        match self.last_value {
            LightStatus::On => light.on(),
            LightStatus::Off => light.off(),
        };
    }
}

struct CeilingCommand {
    speed: CeilingSpeed,
    last_speed: CeilingSpeed,
}

impl CeilingCommand {
    fn new(speed: CeilingSpeed) -> Self {
        Self {
            speed,
            last_speed: CeilingSpeed::Off,
        }
    }
}

impl Command for CeilingCommand {
    fn execute(&mut self, home: &mut Home) {
        let fan = &mut home.ceiling_fan;
        self.last_speed = fan.speed();
        fan.set_speed(self.speed);
    }

    fn undo(&mut self, home: &mut Home) {
        home.ceiling_fan.set_speed(self.last_speed);
    }
}

struct Stereo {
    volume: usize,
}

impl Stereo {
    fn new() -> Self {
        Self { volume: 0 }
    }

    fn set_volume(&mut self, volume: usize) {
        println!("Stereo at {volume}");
        self.volume = volume;
    }

    fn volume(&self) -> usize {
        self.volume
    }
}

struct StereoCommand {
    volume: usize,
    last_volume: usize,
}

impl StereoCommand {
    fn new(volume: usize) -> Self {
        Self {
            volume,
            last_volume: 0,
        }
    }
}

impl Command for StereoCommand {
    fn execute(&mut self, home: &mut Home) {
        let previous_volume = home.stereo.volume();
        self.last_volume = previous_volume;
        home.stereo.set_volume(self.volume)
    }

    fn undo(&mut self, home: &mut Home) {
        home.stereo.set_volume(self.last_volume)
    }
}

struct MacroCommand {
    commands: Vec<Box<dyn Command>>,
}

impl MacroCommand {
    fn new(commands: Vec<Box<dyn Command>>) -> Self {
        Self { commands }
    }
}

impl Command for MacroCommand {
    fn execute(&mut self, home: &mut Home) {
        for command in &mut self.commands {
            command.execute(home);
        }
    }

    fn undo(&mut self, home: &mut Home) {
        for command in self.commands.iter_mut().rev() {
            command.undo(home);
        }
    }
}

struct Button {
    command: Box<dyn Command>,
}

impl Button {
    fn new(command: Box<dyn Command>) -> Self {
        Self { command }
    }

    fn command(&mut self) -> &mut dyn Command {
        self.command.as_mut()
    }
}

struct Remote {
    buttons: Vec<Button>,
    last_button_index: usize, // for undo()
}

impl Remote {
    fn new() -> Self {
        let party_command: Vec<Box<dyn Command>> = vec![
            Box::new(LightCommand::new("livingroom", LightStatus::On)),
            Box::new(CeilingCommand::new(CeilingSpeed::Off)),
            Box::new(StereoCommand::new(11)),
        ];

        let buttons = vec![
            Button::new(Box::new(LightCommand::new("bathroom", LightStatus::On))),
            Button::new(Box::new(LightCommand::new("bathroom", LightStatus::Off))),
            Button::new(Box::new(LightCommand::new("livingroom", LightStatus::On))),
            Button::new(Box::new(LightCommand::new("livingroom", LightStatus::Off))),
            Button::new(Box::new(LightCommand::new("kitchen", LightStatus::On))),
            Button::new(Box::new(LightCommand::new("kitchen", LightStatus::Off))),
            Button::new(Box::new(CeilingCommand::new(CeilingSpeed::Off))),
            Button::new(Box::new(CeilingCommand::new(CeilingSpeed::Low))),
            Button::new(Box::new(CeilingCommand::new(CeilingSpeed::Medium))),
            Button::new(Box::new(CeilingCommand::new(CeilingSpeed::High))),
            Button::new(Box::new(StereoCommand::new(0))),
            Button::new(Box::new(StereoCommand::new(2))),
            Button::new(Box::new(StereoCommand::new(4))),
            Button::new(Box::new(StereoCommand::new(6))),
            Button::new(Box::new(StereoCommand::new(8))),
            Button::new(Box::new(StereoCommand::new(11))),
            Button::new(Box::new(MacroCommand::new(party_command))),
        ];

        Self {
            buttons,
            last_button_index: 0,
        }
    }

    fn press(&mut self, button_index: usize, home: &mut Home) {
        let button = self.buttons.get_mut(button_index);
        let button = match button {
            Some(b) => b,
            None => {
                println!("No such button");
                return;
            }
        };
        let command = button.command();
        command.execute(home);
        self.last_button_index = button_index;
    }

    fn undo(&mut self, home: &mut Home) {
        let last_command = self.buttons[self.last_button_index].command();
        last_command.undo(home);
    }
}

pub fn run() {
    let mut home = Home::new();
    let mut remote = Remote::new();
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    loop {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let line = buffer.trim();
        match line {
            "" => continue,
            "q" | "quit" => {
                println!("Bye");
                break;
            }
            "u" | "undo" => remote.undo(&mut home),
            _ => match line.parse::<usize>() {
                Ok(i) => {
                    remote.press(i, &mut home);
                }
                Err(e) => {
                    println!("Could not parse input: {e}");
                    continue;
                }
            },
        }
    }
}
