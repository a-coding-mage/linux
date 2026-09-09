// SPDX-License-Identifier: GPL-2.0
/*
 * Setup kernel for a Sun3x machine
 *
 * (C) 1999 Thomas Bogendoerfer (tsbogend@alpha.franken.de)
 *
 * based on code from Oliver Jowett <oliver@jowett.manawatu.gen.nz>
 */

use core::ffi::{c_char, c_int, c_uchar, c_void};

// C header dependencies are supplied by the surrounding kernel translation.
#[repr(C)]
pub struct SeqFile {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Sun3xProm {
    pub pv_monid: *const c_char,
}

extern "C" {
    static mut romvec: *mut Sun3xProm;
    static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    static mut mach_init_IRQ: Option<unsafe extern "C" fn()>;
    static mut mach_reset: Option<unsafe extern "C" fn()>;
    static mut mach_hwclk: Option<unsafe extern "C" fn()>;
    static mut mach_get_model: Option<unsafe extern "C" fn()>;
    static mut mach_get_hardware_list:
        Option<unsafe extern "C" fn(*mut SeqFile)>;
    static mut sun3_intreg: *mut c_uchar;
    static SUN3X_INTREG: usize;
    fn sun3x_prom_init();
    fn sun3x_sched_init();
    fn sun3_init_IRQ();
    fn sun3x_reboot();
    fn sun3x_hwclk();
    fn sun3_get_model();
    fn seq_printf(m: *mut SeqFile, format: *const c_char, ...);
}

pub static mut clock_va: *mut c_char = core::ptr::null_mut();

pub unsafe extern "C" fn sun3_leds(_byte: u8) {}

unsafe extern "C" fn sun3x_get_hardware_list(m: *mut SeqFile) {
    static FORMAT: &[u8] = b"PROM Revision:\t%s\n\0";
    seq_printf(m, FORMAT.as_ptr() as *const c_char, (*romvec).pv_monid);
}

/*
 *  Setup the sun3x configuration info
 */
pub unsafe extern "C" fn config_sun3x() {
    sun3x_prom_init();

    mach_sched_init = Some(sun3x_sched_init);
    mach_init_IRQ = Some(sun3_init_IRQ);

    mach_reset = Some(sun3x_reboot);

    mach_hwclk = Some(sun3x_hwclk);
    mach_get_model = Some(sun3_get_model);
    mach_get_hardware_list = Some(sun3x_get_hardware_list);

    sun3_intreg = SUN3X_INTREG as *mut c_uchar;

    /* only the serial console is known to work anyway... */
    // The original source contains a disabled (#if 0) EEPROM console switch.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
