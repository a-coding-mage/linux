// SPDX-License-Identifier: GPL-2.0
/*
 *	Implement 'Simple Boot Flag Specification 2.0'
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_ulong, c_void};

const SBF_RESERVED: u8 = 0x78;
const SBF_PNPOS: u8 = 1 << 0;
const SBF_BOOTING: u8 = 1 << 1;
const SBF_DIAG: u8 = 1 << 2;
const SBF_PARITY: u8 = 1 << 7;

extern "C" {
    fn parity8(v: u8) -> c_int;
    static mut rtc_lock: c_void;
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    fn CMOS_WRITE(v: u8, port: c_int);
    fn CMOS_READ(port: c_int) -> u8;
    fn printk(fmt: *const c_char, ...);
}

pub static mut sbf_port: c_int = -1; // set via acpi_boot_init()

unsafe fn sbf_write(v: u8) {
    let mut v = v;
    let mut flags: c_ulong = 0;

    if sbf_port != -1 {
        if parity8(v) == 0 {
            v ^= SBF_PARITY;
        }

        // KERN_INFO prefix is supplied by the kernel logging interface.
        static MSG: &[u8] = b"Simple Boot Flag at 0x%x set to 0x%x\n\0";
        printk(MSG.as_ptr() as *const c_char, sbf_port, v as c_int);

        spin_lock_irqsave(&mut rtc_lock as *mut c_void, &mut flags);
        CMOS_WRITE(v, sbf_port);
        spin_unlock_irqrestore(&mut rtc_lock as *mut c_void, flags);
    }
}

unsafe fn sbf_read() -> u8 {
    let mut flags: c_ulong = 0;
    let v: u8;

    if sbf_port == -1 {
        return 0;
    }

    spin_lock_irqsave(&mut rtc_lock as *mut c_void, &mut flags);
    v = CMOS_READ(sbf_port);
    spin_unlock_irqrestore(&mut rtc_lock as *mut c_void, flags);

    v
}

unsafe fn sbf_value_valid(v: u8) -> bool {
    if v & SBF_RESERVED != 0 { // Reserved bits
        return false;
    }
    if parity8(v) == 0 {
        return false;
    }

    true
}

unsafe fn sbf_init() -> c_int {
    let mut v: u8;

    if sbf_port == -1 {
        return 0;
    }

    v = sbf_read();
    if !sbf_value_valid(v) {
        // KERN_WARNING prefix is supplied by the kernel logging interface.
        static MSG: &[u8] = b"Simple Boot Flag value 0x%x read from CMOS RAM was invalid\n\0";
        printk(MSG.as_ptr() as *const c_char, v as c_int);
    }

    v &= !SBF_RESERVED;
    v &= !SBF_BOOTING;
    v &= !SBF_DIAG;
    // CONFIG_ISAPNP is a build-time condition from the original source.
    #[cfg(CONFIG_ISAPNP)]
    {
        v |= SBF_PNPOS;
    }
    sbf_write(v);

    0
}

// arch_initcall(sbf_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
