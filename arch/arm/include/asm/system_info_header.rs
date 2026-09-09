/* SPDX-License-Identifier: GPL-2.0 */

pub const CPU_ARCH_UNKNOWN: i32 = 0;
pub const CPU_ARCH_ARMv3: i32 = 1;
pub const CPU_ARCH_ARMv4: i32 = 2;
pub const CPU_ARCH_ARMv4T: i32 = 3;
pub const CPU_ARCH_ARMv5: i32 = 4;
pub const CPU_ARCH_ARMv5T: i32 = 5;
pub const CPU_ARCH_ARMv5TE: i32 = 6;
pub const CPU_ARCH_ARMv5TEJ: i32 = 7;
pub const CPU_ARCH_ARMv6: i32 = 8;
pub const CPU_ARCH_ARMv7: i32 = 9;
pub const CPU_ARCH_ARMv7M: i32 = 10;

/* information about the system we're running on */
unsafe extern "C" {
    pub static mut system_rev: u32;
    pub static system_serial: *const core::ffi::c_char;
    pub static mut system_serial_low: u32;
    pub static mut system_serial_high: u32;
    pub static mut mem_fclk_21285: u32;

    /* __pure */
    pub fn cpu_architecture() -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
