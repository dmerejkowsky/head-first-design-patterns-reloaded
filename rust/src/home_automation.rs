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
    ceiling_fan: CeilingFan,
}

impl Home {
    fn new() -> Self {
        let kitchen_light = Light::new("kitchen");
        let bathroom_light = Light::new("bathroom");
        let ceiling_fan = CeilingFan::new();

        Self {
            kitchen_light,
            bathroom_light,
            ceiling_fan,
        }
    }
}

trait Command {
    fn execute(&self, home: &mut Home);
    fn undo(&self, _home: &mut Home);
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
}

struct Remote {
    buttons: Vec<Button>,
    last_button_index: usize, // for undo()
}

impl Remote {
    fn new() -> Self {
        let buttons = vec![
            Button::new(Box::new(BathroomLightOn)),
            Button::new(Box::new(BathroomLightOff)),
            Button::new(Box::new(CeilingCommand::new(CeilingSpeed::Off))),
            Button::new(Box::new(CeilingCommand::new(CeilingSpeed::Low))),
            Button::new(Box::new(CeilingCommand::new(CeilingSpeed::Medium))),
            Button::new(Box::new(CeilingCommand::new(CeilingSpeed::High))),
        ];

        Self {
            buttons,
            last_button_index: 0,
        }
    }

    fn press(&mut self, button_index: usize, home: &mut Home) {
        let button = match self.buttons.get(button_index) {
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
