/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies from the original header:
// #include <linux/msi.h>
// #include <linux/pci.h>

extern "C" {
    // Original declaration used the Linux __init annotation.
    pub fn init_airq_interrupts();
    // Original declaration used the Linux __init annotation.
    pub fn init_cio_interrupts();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
