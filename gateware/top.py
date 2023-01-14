#!/usr/bin/env python3
#
# This file is part of LUNA.
#
# Copyright (c) 2020 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import sys

from amaranth import Signal, Module, Elaboratable, ClockDomain, ClockSignal, Cat, Instance

from luna import top_level_cli
from luna.gateware.platform import NullPin

class Blinky(Elaboratable):
    """ Hardware module that validates basic LUNA functionality. """
    def elaborate(self, platform):
        m = Module()

        # add the generated verilog file to the build
        with open("gateware/generated/blinky.v", "r") as f:
            verilog = f.read()
        platform.add_file("blinky.v", verilog)

        # instantiate the verilog module
        blink = Signal()
        blinky = Instance(
            "blinky",
            # Parameters starting with `p_` are Verilog parameters.
            #p_WIDTH=count.width,

            # Parameters starting with `i_` are inputs.
            # In this case we get the clock signal using `ClockSignal()`,
            # although we could have assigned any signal.
            i_clk=ClockSignal("sync"),

            # Parameters starting with `o_` are outputs.
            # We assign the output of the module to our `count` Signal.
            o_blink=blink

            # We could also use `a_` for Verilog attributes and `io_`
            # for Verilog inouts.
        )
        m.submodules.blinky = blinky

        # Grab our I/O connectors.
        leds = [platform.request_optional("led", i, default=NullPin()).o for i in range(0, 8)]

        # Attach an led
        m.d.comb += leds[2].eq(blink)

        # Return our elaborated module.
        return m


if __name__ == "__main__":
    top_level_cli(Blinky, doProgram=False)
