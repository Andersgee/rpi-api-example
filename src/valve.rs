use actix::prelude::*;
use rppal::gpio::{Gpio, OutputPin};
use std::time::Duration;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 18;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ToggleValveMessage {
    pub ms: u64,
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

    fn toggle(&mut self, ctx: &mut Context<Self>, ms: u64) {
        self.pin.set_low();
        //println!("toggle called.");
        ctx.run_later(Duration::from_millis(ms), |act, _ctx| {
            act.pin.set_high();
            //println!("run_later after {} ms", &ms);
        });
    }
}

impl Actor for Valve {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("actor for valve started")
    }
}

impl Handler<ToggleValveMessage> for Valve {
    type Result = ();
    fn handle(&mut self, msg: ToggleValveMessage, ctx: &mut Self::Context) -> Self::Result {
        println!("message to Valve actor handled here, ms: {}", msg.ms);
        self.toggle(ctx, msg.ms)
    }
}
