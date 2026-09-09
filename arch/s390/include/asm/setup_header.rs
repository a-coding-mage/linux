/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  S390 version
 *    Copyright IBM Corp. 1999, 2017
 */

// C header guard: _ASM_S390_SETUP_H
// C dependencies: <linux/bits.h>, <uapi/asm/setup.h>, <linux/build_bug.h>,
// <asm/lowcore.h>, and <asm/types.h>.

pub const PARMAREA: usize = 0x10400;

// COMMAND_LINE_SIZE is supplied by the build configuration.
pub const COMMAND_LINE_SIZE: usize = CONFIG_COMMAND_LINE_SIZE;

pub const LPP_MAGIC: u32 = 1u32 << 31;
pub const LPP_PID_MASK: libc::c_ulong = 0xffff_ffff;

/* Offsets to entry points in kernel/head.S */
pub const STARTUP_NORMAL_OFFSET: usize = 0x10000;
pub const STARTUP_KDUMP_OFFSET: usize = 0x10010;

pub const LEGACY_COMMAND_LINE_SIZE: usize = 896;

#[repr(C)]
pub struct parmarea {
    pub ipl_device: libc::c_ulong,       /* 0x10400 */
    pub initrd_start: libc::c_ulong,     /* 0x10408 */
    pub initrd_size: libc::c_ulong,      /* 0x10410 */
    pub oldmem_base: libc::c_ulong,      /* 0x10418 */
    pub oldmem_size: libc::c_ulong,      /* 0x10420 */
    pub kernel_version: libc::c_ulong,   /* 0x10428 */
    pub max_command_line_size: libc::c_ulong, /* 0x10430 */
    pub pad1: [libc::c_char; 0x10480 - 0x10438], /* 0x10438 - 0x10480 */
    pub command_line: [libc::c_char; COMMAND_LINE_SIZE], /* 0x10480 */
}

extern "C" {
    pub static mut arch_hw_string: [libc::c_char; 128];
    pub static mut parmarea: parmarea;

    pub static mut zlib_dfltcc_support: libc::c_uint;
    pub static mut max_mappable: libc::c_ulong;

    /* The Write Back bit position in the physaddr is given by the SLPC PCI */
    pub static mut mio_wb_bit_mask: libc::c_ulong;

    /*
     * Console mode. Override with conmode=
     */
    pub static mut console_mode: libc::c_uint;
    pub static mut console_devno: libc::c_uint;
    pub static mut console_irq: libc::c_uint;

    pub fn register_early_console();

    #[cfg(feature = "CONFIG_VMCP")]
    pub fn vmcp_cma_reserve();

    pub fn report_user_fault(regs: *mut pt_regs, signr: libc::c_long, is_mm_fault: libc::c_int);

    pub static mut _machine_restart: Option<unsafe extern "C" fn(command: *mut libc::c_char)>;
    pub static mut _machine_halt: Option<unsafe extern "C" fn()>;
    pub static mut _machine_power_off: Option<unsafe extern "C" fn()>;

    pub static mut oldmem_data: oldmem_data;
}

#[cfg(not(feature = "CONFIG_VMCP"))]
#[inline]
pub unsafe fn vmcp_cma_reserve() {}

pub const ZLIB_DFLTCC_DISABLED: libc::c_uint = 0;
pub const ZLIB_DFLTCC_FULL: libc::c_uint = 1;
pub const ZLIB_DFLTCC_DEFLATE_ONLY: libc::c_uint = 2;
pub const ZLIB_DFLTCC_INFLATE_ONLY: libc::c_uint = 3;
pub const ZLIB_DFLTCC_FULL_DEBUG: libc::c_uint = 4;

pub const CONSOLE_IS_UNDEFINED: bool = false; // console_mode == 0
pub const CONSOLE_IS_SCLP: libc::c_uint = 1;
pub const CONSOLE_IS_3215: libc::c_uint = 2;
pub const CONSOLE_IS_3270: libc::c_uint = 3;
pub const CONSOLE_IS_VT220: libc::c_uint = 4;
pub const CONSOLE_IS_HVC: libc::c_uint = 5;

#[repr(C)]
pub struct oldmem_data {
    pub start: libc::c_ulong,
    pub size: libc::c_ulong,
}

// C macros CONSOLE_IS_* compare console_mode to the corresponding value.
#[inline]
pub unsafe fn console_is_undefined() -> bool { console_mode == 0 }
#[inline]
pub unsafe fn console_is_sclp() -> bool { console_mode == 1 }
#[inline]
pub unsafe fn console_is_3215() -> bool { console_mode == 2 }
#[inline]
pub unsafe fn console_is_3270() -> bool { console_mode == 3 }
#[inline]
pub unsafe fn console_is_vt220() -> bool { console_mode == 4 }
#[inline]
pub unsafe fn console_is_hvc() -> bool { console_mode == 5 }

#[inline]
pub unsafe fn set_console_sclp() { console_mode = 1; }
#[inline]
pub unsafe fn set_console_3215() { console_mode = 2; }
#[inline]
pub unsafe fn set_console_3270() { console_mode = 3; }
#[inline]
pub unsafe fn set_console_vt220() { console_mode = 4; }
#[inline]
pub unsafe fn set_console_hvc() { console_mode = 5; }

#[inline(always)]
pub unsafe fn gen_lpswe(addr: libc::c_ulong) -> u32 {
    // Equivalent of BUILD_BUG_ON(addr > 0xfff); retained as a runtime check
    // because Rust function parameters cannot be used directly in a const assertion.
    assert!(addr <= 0xfff);
    (0xb2b2_0000u32).wrapping_add(addr as u32)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
