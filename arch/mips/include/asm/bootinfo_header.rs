/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 1996, 2003 by Ralf Baechle
 * Copyright (C) 1995, 1996 Andreas Busse
 * Copyright (C) 1995, 1996 Stoned Elipot
 * Copyright (C) 1995, 1996 Paul M. Antoine.
 * Copyright (C) 2009       Zhang Le
 */

// Dependencies: <linux/types.h> and <asm/setup.h>

/*
 * The MACH_ IDs are sort of equivalent to PCI product IDs.  As such the
 * numbers do not necessarily reflect technical relations or similarities
 * between systems.
 */

/* Valid machtype values for group unknown */
pub const MACH_UNKNOWN: i32 = 0; /* whatever... */

/* Valid machtype for group DEC */
pub const MACH_DSUNKNOWN: i32 = 0;
pub const MACH_DS23100: i32 = 1; /* DECstation 2100 or 3100 */
pub const MACH_DS5100: i32 = 2; /* DECsystem 5100 */
pub const MACH_DS5000_200: i32 = 3; /* DECstation 5000/200 */
pub const MACH_DS5000_1XX: i32 = 4; /* DECstation 5000/120, 125, 133, 150 */
pub const MACH_DS5000_XX: i32 = 5; /* DECstation 5000/20, 25, 33, 50 */
pub const MACH_DS5000_2X0: i32 = 6; /* DECstation 5000/240, 260 */
pub const MACH_DS5400: i32 = 7; /* DECsystem 5400 */
pub const MACH_DS5500: i32 = 8; /* DECsystem 5500 */
pub const MACH_DS5800: i32 = 9; /* DECsystem 5800 */
pub const MACH_DS5900: i32 = 10; /* DECsystem 5900 */

/* Valid machtype for group Mikrotik */
pub const MACH_MIKROTIK_RB532: i32 = 0; /* Mikrotik RouterBoard 532 */
pub const MACH_MIKROTIK_RB532A: i32 = 1; /* Mikrotik RouterBoard 532A */

/* Valid machtype for Loongson family */
#[repr(C)]
pub enum loongson2ef_machine_type {
    MACH_LOONGSON_UNKNOWN,
    MACH_LEMOTE_FL2E,
    MACH_LEMOTE_FL2F,
    MACH_LEMOTE_ML2F7,
    MACH_LEMOTE_YL2F89,
    MACH_DEXXON_GDIUM2F10,
    MACH_LEMOTE_NAS,
    MACH_LEMOTE_LL2F,
    MACH_LOONGSON_END,
}

/* Valid machtype for group INGENIC */
#[repr(C)]
pub enum ingenic_machine_type {
    MACH_INGENIC_UNKNOWN,
    MACH_INGENIC_JZ4720,
    MACH_INGENIC_JZ4725,
    MACH_INGENIC_JZ4725B,
    MACH_INGENIC_JZ4730,
    MACH_INGENIC_JZ4740,
    MACH_INGENIC_JZ4750,
    MACH_INGENIC_JZ4755,
    MACH_INGENIC_JZ4760,
    MACH_INGENIC_JZ4760B,
    MACH_INGENIC_JZ4770,
    MACH_INGENIC_JZ4775,
    MACH_INGENIC_JZ4780,
    MACH_INGENIC_X1000,
    MACH_INGENIC_X1000E,
    MACH_INGENIC_X1830,
    MACH_INGENIC_X2000,
    MACH_INGENIC_X2000E,
    MACH_INGENIC_X2000H,
    MACH_INGENIC_X2100,
}

extern "C" {
    pub static mut system_type: *mut core::ffi::c_char;
    pub fn get_system_type() -> *const core::ffi::c_char;
    pub static mut mips_machtype: u64;
    pub fn detect_memory_region(start: phys_addr_t, sz_min: phys_addr_t, sz_max: phys_addr_t);
    pub fn prom_init();
    pub fn prom_free_prom_memory();
    pub fn prom_cleanup();
    pub fn free_init_pages(what: *const core::ffi::c_char, begin: u64, end: u64);
    pub static mut free_init_pages_eva: Option<unsafe extern "C" fn(begin: *mut core::ffi::c_void, end: *mut core::ffi::c_void)>;
    pub static mut arcs_cmdline: [core::ffi::c_char; COMMAND_LINE_SIZE];
    pub static mut fw_arg0: u64;
    pub static mut fw_arg1: u64;
    pub static mut fw_arg2: u64;
    pub static mut fw_arg3: u64;
    pub fn plat_mem_setup();
}

/* Initial kernel command line, usually setup by prom_init() */
// Registers a0, a1, a2 and a3 as passed to the kernel entry by firmware

/* CONFIG_USE_OF declarations and get_fdt() are retained below under their build condition. */
// The original CONFIG_USE_OF, CONFIG_RELOCATABLE, CONFIG_SWIOTLB, and device-tree
// declarations depend on external kernel definitions and are intentionally left
// as conditional declarations for the consuming build.

#[cfg(feature = "CONFIG_USE_OF")]
extern "C" {
    pub static mut __appended_dtb: [u8; 0];
    pub static __dtb_start: u8;
    pub static __dtb_end: u8;
    pub fn fdt_magic(fdt: *const core::ffi::c_void) -> u32;
}

#[cfg(feature = "CONFIG_USE_OF")]
pub unsafe fn get_fdt() -> *mut core::ffi::c_void {
    if (cfg!(feature = "CONFIG_MIPS_RAW_APPENDED_DTB")
        || cfg!(feature = "CONFIG_MIPS_ELF_APPENDED_DTB"))
        && fdt_magic(core::ptr::addr_of!(__appended_dtb) as *const core::ffi::c_void)
            == FDT_MAGIC
    {
        return core::ptr::addr_of_mut!(__appended_dtb) as *mut core::ffi::c_void;
    }

    if fw_arg0 == (-2i64 as u64) {
        return fw_arg1 as *mut core::ffi::c_void;
    }

    if cfg!(feature = "CONFIG_BUILTIN_DTB") && core::ptr::addr_of!(__dtb_start) != core::ptr::addr_of!(__dtb_end) {
        return core::ptr::addr_of!(__dtb_start) as *mut core::ffi::c_void;
    }

    core::ptr::null_mut()
}

#[cfg(feature = "CONFIG_SWIOTLB")]
extern "C" {
    pub fn plat_swiotlb_setup();
}

#[cfg(not(feature = "CONFIG_SWIOTLB"))]
pub unsafe extern "C" fn plat_swiotlb_setup() {}

#[cfg(feature = "CONFIG_USE_OF")]
extern "C" {
    pub fn plat_get_fdt() -> *mut core::ffi::c_void;
}

#[cfg(all(feature = "CONFIG_USE_OF", feature = "CONFIG_RELOCATABLE"))]
extern "C" {
    pub fn plat_fdt_relocated(new_location: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
