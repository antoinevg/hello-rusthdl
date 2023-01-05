use std::time::Duration;
use rust_hdl::core::prelude::*;
use rust_hdl::docs::vcd2svg::vcd_to_svg;
use rust_hdl::widgets::prelude::*;

const CLOCK_SPEED_HZ : u64 = 10_000;

#[derive(LogicBlock)]
struct Blinky {
    pub clock: Signal<In, Clock>,
    pulser: Pulser,
    pub led: Signal<Out, Bit>,
}

impl Default for Blinky {
   fn default() -> Self {
       Self {
         clock: Default::default(),
         pulser: Pulser::new(CLOCK_SPEED_HZ, CLOCK_SPEED_HZ as f64 / 5.0, Duration::from_micros(300)),
         led: Default::default(),
       }
    }
}

impl Logic for Blinky {
    #[hdl_gen]
    fn update(&mut self) {
       self.pulser.clock.next = self.clock.val();
       self.pulser.enable.next = true.into();
       self.led.next = self.pulser.pulse.val();
    }
}

fn main() {
    // testbench
    let mut sim = simple_sim!(Blinky, clock, CLOCK_SPEED_HZ, fixture, {
        let mut x = fixture.init()?;
        wait_clock_cycles!(fixture, clock, x, 80);
        fixture.done(x)
    });

    // run simulation
    let mut dut = Blinky::default();
    dut.connect_all();
    sim.run_to_file(Box::new(dut), sim_time::ONE_MILLISECOND * 10, "blinky.vcd").unwrap();

    // vcd_to_svg
    let t0 = 0;
    let t1 = sim_time::ONE_MILLISECOND * 10; // picoseconds
    vcd_to_svg("blinky.vcd", "blinky.svg", &[
        "uut.clock",
        "uut.led",
    ], t0, t1).unwrap();
}
