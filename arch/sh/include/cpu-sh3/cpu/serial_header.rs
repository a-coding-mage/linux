/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent to: #include <linux/serial_sci.h>

extern "C" {
    pub static mut sh770x_sci_port_ops: plat_sci_port_ops;
    pub static mut sh7710_sci_port_ops: plat_sci_port_ops;
    pub static mut sh7720_sci_port_ops: plat_sci_port_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
