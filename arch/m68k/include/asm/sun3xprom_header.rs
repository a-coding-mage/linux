/* SPDX-License-Identifier: GPL-2.0 */
/* Useful PROM locations */

extern "C" {
    pub static mut sun3x_putchar: Option<unsafe extern "C" fn(ch: core::ffi::c_int)>;
    pub static mut sun3x_getchar: Option<unsafe extern "C" fn() -> core::ffi::c_int>;
    pub static mut sun3x_mayget: Option<unsafe extern "C" fn() -> core::ffi::c_int>;
    pub static mut sun3x_mayput: Option<unsafe extern "C" fn(ch: core::ffi::c_int)>;

    pub fn sun3x_reboot();
    pub fn sun3x_abort();
    pub fn sun3x_prom_init();
    pub fn sun3x_prom_ptov(pa: c_ulong, size: c_ulong) -> c_ulong;
}

/* The C header relies on the platform's definition of unsigned long. */
type c_ulong = core::ffi::c_ulong;

/* interesting hardware locations */
pub const SUN3X_IOMMU: u32 = 0x60000000;
pub const SUN3X_ENAREG: u32 = 0x61000000;
pub const SUN3X_INTREG: u32 = 0x61001400;
pub const SUN3X_DIAGREG: u32 = 0x61001800;
pub const SUN3X_ZS1: u32 = 0x62000000;
pub const SUN3X_ZS2: u32 = 0x62002000;
pub const SUN3X_LANCE: u32 = 0x65002000;
pub const SUN3X_EEPROM: u32 = 0x64000000;
pub const SUN3X_IDPROM: u32 = 0x640007d8;
pub const SUN3X_VIDEO_BASE: u32 = 0x50400000;
pub const SUN3X_VIDEO_REGS: u32 = 0x50300000;

/* vector table */
pub const SUN3X_PROM_BASE: u32 = 0xfefe0000;
pub const SUN3X_P_GETCHAR: u32 = SUN3X_PROM_BASE + 20;
pub const SUN3X_P_PUTCHAR: u32 = SUN3X_PROM_BASE + 24;
pub const SUN3X_P_MAYGET: u32 = SUN3X_PROM_BASE + 28;
pub const SUN3X_P_MAYPUT: u32 = SUN3X_PROM_BASE + 32;
pub const SUN3X_P_REBOOT: u32 = SUN3X_PROM_BASE + 96;
pub const SUN3X_P_SETLEDS: u32 = SUN3X_PROM_BASE + 144;
pub const SUN3X_P_ABORT: u32 = SUN3X_PROM_BASE + 152;

/* mapped area */
pub const SUN3X_MAP_START: u32 = 0xfee00000;
pub const SUN3X_MAP_END: u32 = 0xff000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
