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
    last_speed: CeilingSpeed,
}

impl CeilingFan {
    fn new() -> Self {
        Self {
            speed: CeilingSpeed::Off,
            last_speed: CeilingSpeed::Off,
        }
    }

    fn set_speed(&mut self, speed: CeilingSpeed) {
        self.last_speed = self.speed;
        self.speed = speed;
        println!("CeilingFan at {speed:?}");
    }

    fn last_speed(&self) -> CeilingSpeed {
        self.last_speed
    }
}

struct Home {
    kitchen_light: Light,
    bathroom_light: Light,
    livingroom_light: Light,
    ceiling_fan: CeilingFan,
    stereo: Stereo,
}

impl Home {
    fn new() -> Self {
        let kitchen_light = Light::new("kitchen");
        let bathroom_light = Light::new("bathroom");
        let livingroom_light = Light::new("livingroom");
        let ceiling_fan = CeilingFan::new();
        let stereo = Stereo::new();

        Self {
            kitchen_light,
            bathroom_light,
            livingroom_light,
            ceiling_fan,
            stereo,
        }
    }
}

trait Command {
    fn execute(&self, home: &mut Home);
    fn undo(&self, home: &mut Home);
    fn mutate_me(&mut self) {
        println!("Please help, I'm being mutated");
    }
}

#[derive(Clone, Copy)]
struct NoOp;

impl Command for NoOp {
    fn execute(&self, _home: &mut Home) {}

    fn undo(&self, _home: &mut Home) {}
}

#[derive(Clone, Copy)]
struct BathroomLightOn;

impl Command for BathroomLightOn {
    fn execute(&self, home: &mut Home) {
        home.bathroom_light.on();
    }

    fn undo(&self, home: &mut Home) {
        home.bathroom_light.off();
    }
}

#[derive(Clone, Copy)]
struct BathroomLightOff;

impl Command for BathroomLightOff {
    fn execute(&self, home: &mut Home) {
        home.bathroom_light.off();
    }

    fn undo(&self, home: &mut Home) {
        home.bathroom_light.on();
    }
}

#[derive(Clone, Copy)]
struct KitchenLightOn;

impl Command for KitchenLightOn {
    fn execute(&self, home: &mut Home) {
        home.kitchen_light.on();
    }

    fn undo(&self, home: &mut Home) {
        home.kitchen_light.off();
    }
}

#[derive(Clone, Copy)]
struct KitchenLightOff;

impl Command for KitchenLightOff {
    fn execute(&self, home: &mut Home) {
        home.kitchen_light.off();
    }

    fn undo(&self, home: &mut Home) {
        home.kitchen_light.on();
    }
}

#[derive(Clone, Copy)]
struct LivingroomLightOn;

impl Command for LivingroomLightOn {
    fn execute(&self, home: &mut Home) {
        home.livingroom_light.on();
    }

    fn undo(&self, home: &mut Home) {
        home.livingroom_light.off();
    }
}

#[derive(Clone, Copy)]
struct LivingroomLightOff;

impl Command for LivingroomLightOff {
    fn execute(&self, home: &mut Home) {
        home.livingroom_light.off();
    }

    fn undo(&self, home: &mut Home) {
        home.livingroom_light.on();
    }
}

#[derive(Clone, Copy)]
struct CeilingCommand {
    speed: CeilingSpeed,
}

impl CeilingCommand {
    fn new(speed: CeilingSpeed) -> Self {
        Self { speed }
    }
}

impl Command for CeilingCommand {
    fn execute(&self, home: &mut Home) {
        home.ceiling_fan.set_speed(self.speed);
    }

    fn undo(&self, home: &mut Home) {
        let fan = &mut home.ceiling_fan;
        let last_speed = fan.last_speed();
        fan.set_speed(last_speed);
    }
}

struct Stereo {
    volume: usize,
    previous_volume: usize,
}

impl Stereo {
    fn new() -> Self {
        Self {
            volume: 0,
            previous_volume: 0,
        }
    }

    fn set_volume(&mut self, volume: usize) {
        self.previous_volume = self.volume;
        println!("Stereo at {volume}");
        self.volume = volume;
    }

    fn previous_volume(&self) -> usize {
        self.previous_volume
    }
}

struct StereoCommand {
    volume: usize,
}

impl StereoCommand {
    fn new(volume: usize) -> Self {
        Self { volume }
    }
}

impl Command for StereoCommand {
    fn execute(&self, home: &mut Home) {
        home.stereo.set_volume(self.volume)
    }

    fn undo(&self, home: &mut Home) {
        let previous_volume = home.stereo.previous_volume();
        home.stereo.set_volume(previous_volume)
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
    fn execute(&self, home: &mut Home) {
        for command in &self.commands {
            command.execute(home);
        }
    }

    fn undo(&self, home: &mut Home) {
        for command in self.commands.iter().rev() {
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

    fn command(&self) -> &dyn Command {
        self.command.as_ref()
    }

    fn command_mut(&mut self) -> &mut dyn Command {
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
            Box::new(LivingroomLightOn),
            Box::new(CeilingCommand::new(CeilingSpeed::Off)),
            Box::new(StereoCommand::new(11)),
        ];

        let buttons = vec![
            Button::new(Box::new(BathroomLightOn)),
            Button::new(Box::new(BathroomLightOff)),
            Button::new(Box::new(LivingroomLightOn)),
            Button::new(Box::new(LivingroomLightOff)),
            Button::new(Box::new(KitchenLightOn)),
            Button::new(Box::new(KitchenLightOff)),
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
        let command = button.command_mut();
        command.mutate_me();
        self.last_button_index = button_index;
    }

    fn undo(&self, home: &mut Home) {
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
