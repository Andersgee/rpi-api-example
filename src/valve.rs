use actix::prelude::*;
use rppal::gpio::{Gpio, OutputPin};
use std::time::Duration;

pub struct Valve {
    pin: OutputPin,
}

impl Valve {
    pub fn new() -> Self {
        // Gpio uses BCM pin numbering. for example: BCM GPIO 23 is tied to physical pin 16.
        let pin_number: u8 = 18;

        //for my particular relay: high=off, low=on, so initialize on high
        let pin = Gpio::new()
            .expect("expected gpio new to be fine")
            .get(pin_number)
            .expect("expected to get pin_number to be fine")
            .into_output_high();
        Self { pin }
    }

    fn toggle(&mut self, ctx: &mut Context<Self>, ms: u64) {
        self.pin.set_low(); //open
        ctx.run_later(Duration::from_millis(ms), |act, _ctx| {
            act.pin.set_high(); //close
        });
    }
}

/**
 * setup actor and message listener for Valve. send a message something like this:
 *
 * ```rs
 * let valve_addr = valve::Valve::new().start(); //start actor
 * valve_addr.send(valve::ToggleValveMessage { ms: 1000 }) //send message to it
 * ```
 */
#[derive(Message)]
#[rtype(result = "()")]
pub struct ToggleValveMessage {
    pub ms: u64,
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
        //println!("ToggleValveMessage sent to Valve actor is handled here, ms: {}", msg.ms);
        self.toggle(ctx, msg.ms)
    }
}
