/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Machine dependent access functions for RTC registers.
 */

// C dependency: <asm/io.h>
// C dependency: <asm/processor.h>

// Preserve the intent of the C preprocessor override: define RTC_PORT and
// RTC_ALWAYS_BCD only when RTC_PORT is not supplied by the build.
#[inline(always)]
pub const fn rtc_port(x: u16) -> u16 {
    0x70u16.wrapping_add(x)
}
pub const RTC_ALWAYS_BCD: i32 = 1; // RTC operates in binary mode

// The following declarations and implementations are conditional on
// CONFIG_X86_32 in the original header.
#[cfg(CONFIG_X86_32)]
extern "C" {
    pub static mut cmos_lock: core::ffi::c_ulong;
    fn smp_processor_id() -> core::ffi::c_ulong;
    fn cpu_relax();
    fn __cmpxchg(
        ptr: *mut core::ffi::c_ulong,
        old: core::ffi::c_ulong,
        new: core::ffi::c_ulong,
        size: usize,
    ) -> core::ffi::c_ulong;
    fn local_irq_save(flags: *mut core::ffi::c_ulong);
    fn local_irq_restore(flags: core::ffi::c_ulong);
}

#[cfg(CONFIG_X86_32)]
#[inline]
pub unsafe fn lock_cmos(reg: u8) {
    let new: core::ffi::c_ulong = ((smp_processor_id().wrapping_add(1)) << 8) | reg as core::ffi::c_ulong;
    loop {
        if core::ptr::read_volatile(&raw const cmos_lock) != 0 {
            cpu_relax();
            continue;
        }
        if __cmpxchg(&raw mut cmos_lock, 0, new, core::mem::size_of::<core::ffi::c_ulong>()) == 0 {
            return;
        }
    }
}

#[cfg(CONFIG_X86_32)]
#[inline]
pub unsafe fn unlock_cmos() {
    core::ptr::write_volatile(&raw mut cmos_lock, 0);
}

#[cfg(CONFIG_X86_32)]
#[inline]
pub unsafe fn do_i_have_lock_cmos() -> i32 {
    (core::ptr::read_volatile(&raw const cmos_lock) >> 8 == smp_processor_id().wrapping_add(1)) as i32
}

#[cfg(CONFIG_X86_32)]
#[inline]
pub unsafe fn current_lock_cmos_reg() -> u8 {
    (core::ptr::read_volatile(&raw const cmos_lock) & 0xff) as u8
}

// C macros lock_cmos_prefix and lock_cmos_suffix form a paired statement
// scope and require the platform interrupt primitives above.
#[cfg(CONFIG_X86_32)]
#[macro_export]
macro_rules! lock_cmos_prefix {
    ($reg:expr) => {{
        let mut cmos_flags: core::ffi::c_ulong = 0;
        unsafe { local_irq_save(&mut cmos_flags); lock_cmos($reg); }
        cmos_flags
    }};
}

#[cfg(CONFIG_X86_32)]
#[macro_export]
macro_rules! lock_cmos_suffix {
    ($flags:expr) => {{
        unsafe { unlock_cmos(); local_irq_restore($flags); }
    }};
}

#[cfg(not(CONFIG_X86_32))]
#[inline(always)]
pub fn lock_cmos_prefix(_reg: u8) {}
#[cfg(not(CONFIG_X86_32))]
#[inline(always)]
pub fn lock_cmos_suffix() {}
#[cfg(not(CONFIG_X86_32))]
#[inline(always)]
pub fn lock_cmos(_reg: u8) {}
#[cfg(not(CONFIG_X86_32))]
#[inline(always)]
pub fn unlock_cmos() {}
#[cfg(not(CONFIG_X86_32))]
#[inline(always)]
pub const fn do_i_have_lock_cmos() -> i32 { 0 }
#[cfg(not(CONFIG_X86_32))]
#[inline(always)]
pub const fn current_lock_cmos_reg() -> u8 { 0 }

/*
 * The yet supported machines all access the RTC index register via
 * an ISA port access but the way to access the date register differs ...
 */
#[inline(always)]
pub unsafe fn CMOS_READ(addr: u8) -> u8 { rtc_cmos_read(addr) }
#[inline(always)]
pub unsafe fn CMOS_WRITE(val: u8, addr: u8) { rtc_cmos_write(val, addr); }

extern "C" {
    pub fn rtc_cmos_read(addr: u8) -> u8;
    pub fn rtc_cmos_write(val: u8, addr: u8);
    pub fn mach_set_cmos_time(now: *const timespec64) -> i32;
    pub fn mach_get_cmos_time(now: *mut timespec64);
}

// C dependency: struct timespec64 supplied by the surrounding kernel headers.

pub const RTC_IRQ: i32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
