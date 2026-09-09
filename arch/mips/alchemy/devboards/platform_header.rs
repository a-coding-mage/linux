/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <linux/init.h>
// The C __init attribute marks these functions for initialization-time use.

extern "C" {
    pub fn db1x_register_pcmcia_socket(
        pcmcia_attr_start: phys_addr_t,
        pcmcia_attr_len: phys_addr_t,
        pcmcia_mem_start: phys_addr_t,
        pcmcia_mem_end: phys_addr_t,
        pcmcia_io_start: phys_addr_t,
        pcmcia_io_end: phys_addr_t,
        card_irq: core::ffi::c_int,
        cd_irq: core::ffi::c_int,
        stschg_irq: core::ffi::c_int,
        eject_irq: core::ffi::c_int,
        id: core::ffi::c_int,
    );

    pub fn db1x_register_norflash(
        size: core::ffi::c_ulong,
        width: core::ffi::c_int,
        swapped: core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
