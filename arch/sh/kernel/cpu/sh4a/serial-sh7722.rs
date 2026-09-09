// SPDX-License-Identifier: GPL-2.0
//
// Dependencies corresponding to <linux/serial_sci.h>, <linux/serial_core.h>,
// and <linux/io.h> are supplied by the surrounding translation unit.

use core::ffi::c_uint;

const PSCR: usize = 0xA405011E;

// Opaque/header-provided declarations used by this implementation.
#[repr(C)]
pub struct uart_port {
    pub mapbase: usize,
}

#[repr(C)]
pub struct plat_sci_port_ops {
    pub init_pins: Option<unsafe extern "C" fn(*mut uart_port, c_uint)>,
}

unsafe extern "C" {
    fn __raw_readw(addr: usize) -> u16;
    fn __raw_writew(value: u16, addr: usize);
}

// Value supplied by the serial core headers (the C macro CRTSCTS).
const CRTSCTS: c_uint = 0x80000000;

unsafe extern "C" fn sh7722_sci_init_pins(port: *mut uart_port, cflag: c_uint) {
    let mut data: u16;

    if (*port).mapbase == 0xffe00000 {
        data = __raw_readw(PSCR);
        data &= !0x03cf;
        if (cflag & CRTSCTS) == 0 {
            data |= 0x0340;
        }

        __raw_writew(data, PSCR);
    }
}

pub static mut sh7722_sci_port_ops: plat_sci_port_ops = plat_sci_port_ops {
    init_pins: Some(sh7722_sci_init_pins),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
