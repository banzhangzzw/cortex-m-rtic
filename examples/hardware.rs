//! examples/hardware.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![deny(missing_docs)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965)]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use lm3s6965::Interrupt;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        // Pends the UART0 interrupt but its handler won't run until *after*
        // `init` returns because interrupts are disabled
        rtic::pend(Interrupt::UART0); // equivalent to NVIC::pend

        hprintln!("init");

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        // interrupts are enabled again; the `UART0` handler runs at this point

        hprintln!("idle");

        rtic::pend(Interrupt::UART0);

        loop {
            // Exit moved after nop to ensure that rtic::pend gets
            // to run before exiting
            cortex_m::asm::nop();
            debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        }
    }

    #[task(binds = UART0, local = [times: u32 = 0])]
    fn uart0(cx: uart0::Context) {
        // Safe access to local `static mut` variable
        *cx.local.times += 1;

        hprintln!(
            "UART0 called {} time{}",
            *cx.local.times,
            if *cx.local.times > 1 { "s" } else { "" }
        );
    }
}
