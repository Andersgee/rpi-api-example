use std::time::Duration;

use actix::prelude::*;
//use rppal::gpio::{Gpio, OutputPin};

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 18;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ToggleValveMessage {
    pub ms: usize,
}

pub struct Valve {
    some_val: i32,
}

impl Valve {
    pub fn new() -> Self {
        Self { some_val: 999 }
    }

    fn toggle(&self, ctx: &mut Context<Self>) {
        println!("toggle called.");
        ctx.run_later(Duration::from_millis(2000), |act, ctx| {
            println!("run_later after 2000 ms");
        });
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
    fn handle(&mut self, msg: ToggleValveMessage, ctx: &mut Self::Context) -> Self::Result {
        println!("message to Valve actor handled here, ms: {}", msg.ms);
        self.toggle(ctx)
    }
}
