// SPDX-License-Identifier: GPL-2.0
/*
 * I/O delay strategies for inb_p/outb_p
 *
 * Allow for a DMI based override of port 0x80, needed for certain HP laptops
 * and possibly other systems. Also allow for the gradual elimination of
 * outb_p/inb_p API uses.
 */

use core::ffi::{c_char, c_int};

// These configuration symbols are supplied by the build configuration.
pub const IO_DELAY_TYPE_0X80: c_int = 0;
pub const IO_DELAY_TYPE_0XED: c_int = 1;
pub const IO_DELAY_TYPE_UDELAY: c_int = 2;
pub const IO_DELAY_TYPE_NONE: c_int = 3;

// CONFIG_IO_DELAY_* selects DEFAULT_IO_DELAY_TYPE at build time.
#[cfg(CONFIG_IO_DELAY_0X80)]
pub const DEFAULT_IO_DELAY_TYPE: c_int = IO_DELAY_TYPE_0X80;
#[cfg(CONFIG_IO_DELAY_0XED)]
pub const DEFAULT_IO_DELAY_TYPE: c_int = IO_DELAY_TYPE_0XED;
#[cfg(CONFIG_IO_DELAY_UDELAY)]
pub const DEFAULT_IO_DELAY_TYPE: c_int = IO_DELAY_TYPE_UDELAY;
#[cfg(CONFIG_IO_DELAY_NONE)]
pub const DEFAULT_IO_DELAY_TYPE: c_int = IO_DELAY_TYPE_NONE;

#[no_mangle]
pub static mut io_delay_type: c_int = DEFAULT_IO_DELAY_TYPE;

static mut io_delay_override: c_int = 0;

#[repr(C)]
pub struct DmiSystemId {
    pub callback: Option<unsafe extern "C" fn(*const DmiSystemId) -> c_int>,
    pub ident: *const c_char,
    pub matches: *const core::ffi::c_void,
}

extern "C" {
    fn udelay(usecs: u64);
    fn pr_notice(fmt: *const c_char, ...);
    fn dmi_check_system(list: *const DmiSystemId) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
}

/*
 * Paravirt wants native_io_delay to be a constant.
 */
#[no_mangle]
pub unsafe extern "C" fn native_io_delay() {
    match io_delay_type {
        IO_DELAY_TYPE_0X80 => core::arch::asm!("out 0x80, al"),
        IO_DELAY_TYPE_0XED => core::arch::asm!("out 0xed, al"),
        IO_DELAY_TYPE_UDELAY => {
            /*
             * 2 usecs is an upper-bound for the outb delay but
             * note that udelay doesn't have the bus-level
             * side-effects that outb does, nor does udelay() have
             * precise timings during very early bootup (the delays
             * are shorter until calibrated):
             */
            udelay(2);
        }
        IO_DELAY_TYPE_NONE => {}
        _ => core::arch::asm!("out 0x80, al"),
    }
}

unsafe extern "C" fn dmi_io_delay_0xed_port(id: *const DmiSystemId) -> c_int {
    if io_delay_type == IO_DELAY_TYPE_0X80 {
        pr_notice(b"%s: using 0xed I/O delay port\0".as_ptr() as *const c_char, (*id).ident);
        io_delay_type = IO_DELAY_TYPE_0XED;
    }
    0
}

/*
 * Quirk table for systems that misbehave (lock up, etc.) if port
 * 0x80 is used:
 *
 * DMI_MATCH entries are retained as source-level data; their concrete
 * kernel representation is supplied by the DMI dependency.
 */
static io_delay_0xed_port_dmi_table: [DmiSystemId; 6] = [
    DmiSystemId { callback: Some(dmi_io_delay_0xed_port), ident: b"Compaq Presario V6000\0".as_ptr() as *const c_char, matches: core::ptr::null() },
    DmiSystemId { callback: Some(dmi_io_delay_0xed_port), ident: b"HP Pavilion dv9000z\0".as_ptr() as *const c_char, matches: core::ptr::null() },
    DmiSystemId { callback: Some(dmi_io_delay_0xed_port), ident: b"HP Pavilion dv6000\0".as_ptr() as *const c_char, matches: core::ptr::null() },
    DmiSystemId { callback: Some(dmi_io_delay_0xed_port), ident: b"HP Pavilion tx1000\0".as_ptr() as *const c_char, matches: core::ptr::null() },
    DmiSystemId { callback: Some(dmi_io_delay_0xed_port), ident: b"Presario F700\0".as_ptr() as *const c_char, matches: core::ptr::null() },
    DmiSystemId { callback: None, ident: core::ptr::null(), matches: core::ptr::null() },
];

pub unsafe extern "C" fn io_delay_init() {
    if io_delay_override == 0 {
        dmi_check_system(io_delay_0xed_port_dmi_table.as_ptr());
    }
}

unsafe extern "C" fn io_delay_param(s: *mut c_char) -> c_int {
    if s.is_null() {
        return -22; // -EINVAL
    }
    let (value, known) = if strcmp(s, b"0x80\0".as_ptr() as *const c_char) == 0 {
        (IO_DELAY_TYPE_0X80, true)
    } else if strcmp(s, b"0xed\0".as_ptr() as *const c_char) == 0 {
        (IO_DELAY_TYPE_0XED, true)
    } else if strcmp(s, b"udelay\0".as_ptr() as *const c_char) == 0 {
        (IO_DELAY_TYPE_UDELAY, true)
    } else if strcmp(s, b"none\0".as_ptr() as *const c_char) == 0 {
        (IO_DELAY_TYPE_NONE, true)
    } else {
        (0, false)
    };
    if !known { return -22; }
    io_delay_type = value;
    io_delay_override = 1;
    0
}

// early_param("io_delay", io_delay_param);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
