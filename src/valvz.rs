use actix::prelude::*;
use rppal::gpio::{Gpio, OutputPin};

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 18;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ToggleValveMessage {
    pub ms: usize,
}

pub struct Valve {
    pin: OutputPin,
}

impl Valve {
    pub fn new() -> Self {
        let pin = Gpio::new()
            .expect("expected gpio new to be fine")
            .get(GPIO_LED)
            .expect("expected to get GPIO_LED to be fine")
            .into_output_high();
        Self { pin }
    }
}

impl Actor for Valve {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("actor for valve started")
    }
}

impl Handler<ToggleValveMessage> for Valve {
    type Result = ();
    fn handle(&mut self, msg: ToggleValveMessage, _ctx: &mut Self::Context) -> Self::Result {
        println!("message to Valve actor handled here, ms: {}", msg.ms)
    }
}
