/*
 * Cobalt Reset operations
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 1996, 1997 by Ralf Baechle
 * Copyright (C) 2001 by Liam Davies (ldavies@agile.tv)
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_void};

const RESET_PORT: *mut c_void = 0xBC00_0000 as *mut c_void; // CKSEG1ADDR(0x1c000000)
const RESET: u8 = 0x0f;

#[repr(C)]
pub struct LedTrigger {
    _private: [u8; 0],
}

extern "C" {
    static mut power_off_led_trigger: *mut LedTrigger;

    fn led_trigger_register_simple(
        name: *const c_char,
        trigger: *mut *mut LedTrigger,
    ) -> i32;
    fn led_trigger_event(trigger: *mut LedTrigger, brightness: u32);
    fn local_irq_disable();
    static mut cpu_wait: Option<unsafe extern "C" fn()>;
    fn writeb(value: u8, address: *mut c_void);
}

const LED_FULL: u32 = 255;

unsafe extern "C" fn ledtrig_power_off_init() -> i32 {
    led_trigger_register_simple(
        b"power-off\0".as_ptr() as *const c_char,
        &mut power_off_led_trigger,
    );
    0
}

// device_initcall(ledtrig_power_off_init);

pub unsafe extern "C" fn cobalt_machine_halt() {
    /*
     * turn on power off LED on RaQ
     */
    led_trigger_event(power_off_led_trigger, LED_FULL);

    local_irq_disable();
    loop {
        if let Some(wait) = cpu_wait {
            wait();
        }
    }
}

pub unsafe extern "C" fn cobalt_machine_restart(_command: *mut c_char) {
    writeb(RESET, RESET_PORT);

    /* we should never get here */
    cobalt_machine_halt();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
