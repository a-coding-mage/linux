// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *    PDC early console support - use PDC firmware to dump text via boot console
 *
 *    Copyright (C) 2001-2022 Helge Deller <deller@gmx.de>
 */

// Declarations supplied by the surrounding kernel sources.
extern "C" {
    fn pdc_iodc_print(s: *const core::ffi::c_char, count: u32) -> i32;
    fn pdc_iodc_getc() -> i32;
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn kgdb_register_io_module(ops: *mut kgdb_io) -> i32;
}

extern "C" {
    static mut PAGE0: *mut page0;
}

const CL_DUPLEX: i32 = 0;
const UPIO_MEM32BE: i32 = 0;
const NO_POLL_CHAR: i32 = -1;

#[repr(C)]
pub struct console {
    pub write: Option<unsafe extern "C" fn(*mut console, *const core::ffi::c_char, u32)>,
}

#[repr(C)]
pub struct earlycon_device {
    pub con: *mut console,
    pub port: serial_port,
}

#[repr(C)]
pub struct serial_port {
    pub iotype: i32,
}

#[repr(C)]
pub struct kgdb_io {
    pub name: *const core::ffi::c_char,
    pub read_char: Option<unsafe extern "C" fn() -> i32>,
    pub write_char: Option<unsafe extern "C" fn(u8)>,
}

#[repr(C)]
pub struct page0 {
    pub mem_cons: mem_cons,
    pub mem_kbd: mem_cons,
}

#[repr(C)]
pub struct mem_cons {
    pub cl_class: i32,
}

unsafe extern "C" fn pdc_console_write(co: *mut console, s: *const core::ffi::c_char, count: u32) {
    let _ = co;
    let mut i: u32 = 0;

    loop {
        i = i.wrapping_add(pdc_iodc_print(s.add(i as usize), count.wrapping_sub(i)) as u32);
        if i >= count {
            break;
        }
    }
}

#[cfg(CONFIG_KGDB)]
unsafe extern "C" fn kgdb_pdc_read_char() -> i32 {
    let c = pdc_iodc_getc();

    if c <= 0 { NO_POLL_CHAR } else { c }
}

#[cfg(CONFIG_KGDB)]
unsafe extern "C" fn kgdb_pdc_write_char(chr: u8) {
    let _ = chr;
    // no need to print char as it's shown on standard console
    // pdc_iodc_print(&chr, 1);
}

#[cfg(CONFIG_KGDB)]
static mut kgdb_pdc_io_ops: kgdb_io = kgdb_io {
    name: b"kgdb_pdc\0".as_ptr() as *const core::ffi::c_char,
    read_char: Some(kgdb_pdc_read_char),
    write_char: Some(kgdb_pdc_write_char),
};

unsafe extern "C" fn pdc_earlycon_setup(device: *mut earlycon_device, opt: *const core::ffi::c_char) -> i32 {
    let _ = opt;

    // If the console is duplex then copy the COUT parameters to CIN.
    if (*PAGE0).mem_cons.cl_class == CL_DUPLEX {
        memcpy(
            &mut (*PAGE0).mem_kbd as *mut mem_cons as *mut core::ffi::c_void,
            &(*PAGE0).mem_cons as *const mem_cons as *const core::ffi::c_void,
            core::mem::size_of::<mem_cons>(),
        );
    }

    let earlycon_console = (*device).con;
    (*earlycon_console).write = Some(pdc_console_write);
    (*device).port.iotype = UPIO_MEM32BE;

    #[cfg(CONFIG_KGDB)]
    {
        kgdb_register_io_module(&mut kgdb_pdc_io_ops);
    }

    0
}

// EARLYCON_DECLARE(pdc, pdc_earlycon_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
