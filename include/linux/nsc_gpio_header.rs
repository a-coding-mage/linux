/* SPDX-License-Identifier: GPL-2.0 */
/**
   nsc_gpio.c

   National Semiconductor GPIO common access methods.

   struct nsc_gpio_ops abstracts the low-level access
   operations for the GPIO units on 2 NSC chip families; the GEODE
   integrated CPU, and the PC-8736[03456] integrated PC-peripheral
   chips.

   The GPIO units on these chips have the same pin architecture, but
   the access methods differ.  Thus, scx200_gpio and pc8736x_gpio
   implement their own versions of these routines; and use the common
   file-operations routines implemented in nsc_gpio module.

   Copyright (c) 2005 Jim Cromie <jim.cromie@gmail.com>

   NB: this work was tested on the Geode SC-1100 and PC-87366 chips.
   NSC sold the GEODE line to AMD, and the PC-8736x line to Winbond.
*/

#[repr(C)]
pub struct nsc_gpio_ops {
    pub owner: *mut module,
    pub gpio_config: Option<unsafe extern "C" fn(iminor: c_uint, mask: u32, bits: u32) -> u32>,
    pub gpio_dump: Option<unsafe extern "C" fn(amp: *mut nsc_gpio_ops, iminor: c_uint)>,
    pub gpio_get: Option<unsafe extern "C" fn(iminor: c_uint) -> c_int>,
    pub gpio_set: Option<unsafe extern "C" fn(iminor: c_uint, state: c_int)>,
    pub gpio_change: Option<unsafe extern "C" fn(iminor: c_uint)>,
    pub gpio_current: Option<unsafe extern "C" fn(iminor: c_uint) -> c_int>,
    pub dev: *mut device, /* for dev_dbg() support, set in init */
}

extern "C" {
    pub fn nsc_gpio_write(
        file: *mut file,
        data: *const c_char,
        len: size_t,
        ppos: *mut loff_t,
    ) -> ssize_t;

    pub fn nsc_gpio_read(
        file: *mut file,
        buf: *mut c_char,
        len: size_t,
        ppos: *mut loff_t,
    ) -> ssize_t;

    pub fn nsc_gpio_dump(amp: *mut nsc_gpio_ops, index: c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
