// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux serial, I/O, and SH CPU headers are
// referenced here as external Rust items.

// Corresponds to the C preprocessor constants.
const PACR: usize = 0xa4050100;
const PBCR: usize = 0xa4050102;

#[repr(C)]
pub struct uart_port {
    pub mapbase: usize,
}

unsafe extern "C" {
    fn __raw_readw(addr: usize) -> u16;
    fn __raw_writew(value: u16, addr: usize);
}

unsafe extern "C" fn sh7710_sci_init_pins(port: *mut uart_port, cflag: u32) {
    let _ = cflag;

    if (*port).mapbase == 0xA4400000 {
        __raw_writew(__raw_readw(PACR) & 0xffc0, PACR);
        __raw_writew(__raw_readw(PBCR) & 0x0fff, PBCR);
    } else if (*port).mapbase == 0xA4410000 {
        __raw_writew(__raw_readw(PBCR) & 0xf003, PBCR);
    }
}

#[repr(C)]
pub struct plat_sci_port_ops {
    pub init_pins: Option<unsafe extern "C" fn(port: *mut uart_port, cflag: u32)>,
}

pub static mut sh7710_sci_port_ops: plat_sci_port_ops = plat_sci_port_ops {
    init_pins: Some(sh7710_sci_init_pins),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
