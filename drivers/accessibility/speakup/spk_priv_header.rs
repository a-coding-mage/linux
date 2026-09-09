/* SPDX-License-Identifier: GPL-2.0+ */
/* spk_priv.h
 * review functions for the speakup screen review package.
 * originally written by: Kirk Reiser and Andy Berdan.
 *
 * extensively modified by David Borowski.
 *
 * Copyright (C) 1998  Kirk Reiser.
 * Copyright (C) 2003  David Borowski.
 */

// Dependencies supplied by the surrounding translation unit:
// spk_types.h, spk_priv_keyinfo.h, and linux/printk.h.

pub const V_LAST_VAR: [i32; 1] = [MAXVARS];
pub const SPACE: u32 = 0x20;
pub const SYNTH_CHECK: u32 = 20030716; /* today's date ought to do for check value */
/* synth flags, for odd synths */
pub const SF_DEC: u32 = 1; /* to fiddle puncs in alpha strings so it doesn't spell */

/* The C definition depends on whether MODULE is defined at build time. */
#[cfg(feature = "module")]
pub const SYNTH_START: i32 = 1;
#[cfg(not(feature = "module"))]
pub const SYNTH_START: i32 = 0;

pub const KT_SPKUP: i32 = 15;
pub const SPK_SYNTH_TIMEOUT: u32 = 100000; /* in micro-seconds */
pub const SYNTH_DEFAULT_DEV: &str = "ttyS0";
pub const SYNTH_DEFAULT_SER: i32 = 0;

extern "C" {
    pub fn spk_serial_init(index: i32) -> *const old_serial_port;
    pub fn spk_stop_serial_interrupt();
    pub fn spk_serial_release(synth: *mut spk_synth);
    pub fn spk_ttyio_release(synth: *mut spk_synth);
    pub fn spk_ttyio_register_ldisc();
    pub fn spk_ttyio_unregister_ldisc();

    pub fn synth_buffer_skip_nonlatin1();
    pub fn synth_buffer_getc() -> u16;
    pub fn synth_buffer_peek() -> u16;
    pub fn synth_buffer_empty() -> i32;
    pub fn spk_get_var(var_id: var_id_t) -> *mut var_t;
    pub fn spk_var_show(
        kobj: *mut kobject,
        attr: *mut kobj_attribute,
        buf: *mut ::core::ffi::c_char,
    ) -> isize;
    pub fn spk_var_store(
        kobj: *mut kobject,
        attr: *mut kobj_attribute,
        buf: *const ::core::ffi::c_char,
        count: usize,
    ) -> isize;

    pub fn spk_serial_synth_probe(synth: *mut spk_synth) -> i32;
    pub fn spk_ttyio_synth_probe(synth: *mut spk_synth) -> i32;
    pub fn spk_serial_synth_immediate(
        synth: *mut spk_synth,
        buff: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    pub fn spk_ttyio_synth_immediate(
        synth: *mut spk_synth,
        buff: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    pub fn spk_do_catch_up(synth: *mut spk_synth);
    pub fn spk_do_catch_up_unicode(synth: *mut spk_synth);
    pub fn spk_synth_flush(synth: *mut spk_synth);
    pub fn spk_synth_get_index(synth: *mut spk_synth) -> u8;
    pub fn spk_synth_is_alive_nop(synth: *mut spk_synth) -> i32;
    pub fn spk_synth_is_alive_restart(synth: *mut spk_synth) -> i32;
    // __printf(1, 2)
    pub fn synth_printf(buf: *const ::core::ffi::c_char, ...);
    pub fn synth_putwc(wc: u16);
    pub fn synth_putwc_s(wc: u16);
    pub fn synth_putws(buf: *const u16);
    pub fn synth_putws_s(buf: *const u16);
    pub fn synth_request_region(start: ::core::ffi::c_ulong, n: ::core::ffi::c_ulong) -> i32;
    pub fn synth_release_region(start: ::core::ffi::c_ulong, n: ::core::ffi::c_ulong) -> i32;
    pub fn synth_add(in_synth: *mut spk_synth) -> i32;
    pub fn synth_remove(in_synth: *mut spk_synth);
    pub fn synth_current() -> *mut spk_synth;

    pub static mut speakup_info: speakup_info_t;

    pub static mut synth_time_vars: [var_t; 0];

    pub static mut spk_serial_io_ops: spk_io_ops;
    pub static mut spk_ttyio_ops: spk_io_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
