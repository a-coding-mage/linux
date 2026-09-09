// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux serial, I/O, and SH CPU headers are
// intentionally referenced here rather than reimplemented.

const SCPCR: usize = 0xA4000116;
const SCPDR: usize = 0xA4000136;

unsafe extern "C" {
    fn __raw_readw(addr: usize) -> u16;
    fn __raw_writew(value: u16, addr: usize);
    fn __raw_readb(addr: usize) -> u8;
    fn __raw_writeb(value: u8, addr: usize);
}

unsafe fn sh770x_sci_init_pins(port: *mut uart_port, cflag: u32) {
    let mut data: u16;

    /* We need to set SCPCR to enable RTS/CTS */
    data = __raw_readw(SCPCR);
    /* Clear out SCP7MD1,0, SCP6MD1,0, SCP4MD1,0 */
    __raw_writew(data & 0x0fcf, SCPCR);

    if (cflag & CRTSCTS) == 0 {
        /* We need to set SCPCR to enable RTS/CTS */
        data = __raw_readw(SCPCR);
        /* Clear out SCP7MD1,0, SCP4MD1,0,
           Set SCP6MD1,0 = {01} (output) */
        __raw_writew((data & 0x0fcf) | 0x1000, SCPCR);

        data = __raw_readb(SCPDR) as u16;
        /* Set /RTS2 (bit6) = 0 */
        __raw_writeb((data & 0xbf) as u8, SCPDR);
    }
}

pub static mut sh770x_sci_port_ops: plat_sci_port_ops = plat_sci_port_ops {
    init_pins: Some(sh770x_sci_init_pins),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
