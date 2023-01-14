use std::time::Duration;
use rust_hdl::core::prelude::*;
use rust_hdl::widgets::prelude::*;

const CLOCK_SPEED_HZ : u64 = 60_000_000;

#[derive(LogicBlock)]
struct Blinky {
    // i/o
    pub clk: Signal<In, Clock>,
    pub blink: Signal<Out, Bit>,
    // submodules
    pulser: Pulser,
}

impl Default for Blinky {
   fn default() -> Self {
       Self {
         clk: Default::default(),
         blink: Default::default(),
         pulser: Pulser::new(CLOCK_SPEED_HZ, 1.0, Duration::from_millis(250)),
       }
    }
}

impl Logic for Blinky {
    #[hdl_gen]
    fn update(&mut self) {
       self.pulser.clock.next = self.clk.val();
       self.pulser.enable.next = true.into();
       self.blink.next = self.pulser.pulse.val();
    }
}


// - generate: verilog --------------------------------------------------------

pub fn generate_verilog<U: Block>(top: &str, uut: &U) -> String {
    let mut defines = ModuleDefines::default();
    check_all(uut).unwrap();
    uut.accept(top, &mut defines);
    defines.defines()
}


// - main ---------------------------------------------------------------------

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // generate verilog
    let mut top = Blinky::default();
    top.connect_all();
    let output = generate_verilog("blinky", &top);
    let mut file = File::create("gateware/generated/blinky.v")?;
    file.write_all(output.as_bytes())?;

    Ok(())
}
