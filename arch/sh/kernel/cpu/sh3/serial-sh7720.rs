// SPDX-License-Identifier: GPL-2.0

// The following names are supplied by the corresponding platform headers:
// `uart_port`, `plat_sci_port_ops`, `CRTSCTS`, `PORT_PTCR`, `PORT_PVCR`,
// `__raw_readw`, and `__raw_writew`.

unsafe fn sh7720_sci_init_pins(port: *mut uart_port, cflag: u32) {
    let mut data: u16;

    if cflag & CRTSCTS != 0 {
        /* enable RTS/CTS */
        if (*port).mapbase == 0xa4430000 {
            /* SCIF0 */
            /* Clear PTCR bit 9-2; enable all scif pins but sck */
            data = __raw_readw(PORT_PTCR);
            __raw_writew(data & 0xfc03, PORT_PTCR);
        } else if (*port).mapbase == 0xa4438000 {
            /* SCIF1 */
            /* Clear PVCR bit 9-2 */
            data = __raw_readw(PORT_PVCR);
            __raw_writew(data & 0xfc03, PORT_PVCR);
        }
    } else {
        if (*port).mapbase == 0xa4430000 {
            /* SCIF0 */
            /* Clear PTCR bit 5-2; enable only tx and rx  */
            data = __raw_readw(PORT_PTCR);
            __raw_writew(data & 0xffc3, PORT_PTCR);
        } else if (*port).mapbase == 0xa4438000 {
            /* SCIF1 */
            /* Clear PVCR bit 5-2 */
            data = __raw_readw(PORT_PVCR);
            __raw_writew(data & 0xffc3, PORT_PVCR);
        }
    }
}

static mut sh7720_sci_port_ops: plat_sci_port_ops = plat_sci_port_ops {
    init_pins: Some(sh7720_sci_init_pins),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
