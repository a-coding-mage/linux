/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from openprom.h. */

#[cfg(feature = "CONFIG_SUN3")]
pub const KADB_DEBUGGER_BEGVM: u32 = 0x0fee0000;
#[cfg(feature = "CONFIG_SUN3")]
pub const LINUX_OPPROM_BEGVM: u32 = 0x0fef0000;
#[cfg(feature = "CONFIG_SUN3")]
pub const LINUX_OPPROM_ENDVM: u32 = 0x0ff10000;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const KADB_DEBUGGER_BEGVM: u32 = 0xffc00000;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const LINUX_OPPROM_BEGVM: u32 = 0xffd00000;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const LINUX_OPPROM_ENDVM: u32 = 0xfff00000;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const LINUX_OPPROM_MAGIC: u32 = 0x10010407;

#[repr(C)]
pub struct linux_dev_v0_funcs {
    pub v0_devopen: Option<unsafe extern "C" fn(*mut i8) -> i32>,
    pub v0_devclose: Option<unsafe extern "C" fn(i32) -> i32>,
    pub v0_rdblkdev: Option<unsafe extern "C" fn(i32, i32, i32, *mut i8) -> i32>,
    pub v0_wrblkdev: Option<unsafe extern "C" fn(i32, i32, i32, *mut i8) -> i32>,
    pub v0_wrnetdev: Option<unsafe extern "C" fn(i32, i32, *mut i8) -> i32>,
    pub v0_rdnetdev: Option<unsafe extern "C" fn(i32, i32, *mut i8) -> i32>,
    pub v0_rdchardev: Option<unsafe extern "C" fn(i32, i32, i32, *mut i8) -> i32>,
    pub v0_wrchardev: Option<unsafe extern "C" fn(i32, i32, i32, *mut i8) -> i32>,
    pub v0_seekdev: Option<unsafe extern "C" fn(i32, isize, i32) -> i32>,
}

#[repr(C)]
pub struct linux_dev_v2_funcs {
    pub v2_inst2pkg: Option<unsafe extern "C" fn(i32) -> i32>,
    pub v2_dumb_mem_alloc: Option<unsafe extern "C" fn(*mut i8, u32) -> *mut i8>,
    pub v2_dumb_mem_free: Option<unsafe extern "C" fn(*mut i8, u32)>,
    pub v2_dumb_mmap: Option<unsafe extern "C" fn(*mut i8, i32, u32, u32) -> *mut i8>,
    pub v2_dumb_munmap: Option<unsafe extern "C" fn(*mut i8, u32)>,
    pub v2_dev_open: Option<unsafe extern "C" fn(*mut i8) -> i32>,
    pub v2_dev_close: Option<unsafe extern "C" fn(i32)>,
    pub v2_dev_read: Option<unsafe extern "C" fn(i32, *mut i8, i32) -> i32>,
    pub v2_dev_write: Option<unsafe extern "C" fn(i32, *mut i8, i32) -> i32>,
    pub v2_dev_seek: Option<unsafe extern "C" fn(i32, i32, i32) -> i32>,
    pub v2_wheee2: Option<unsafe extern "C" fn()>,
    pub v2_wheee3: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct linux_mlist_v0 {
    pub theres_more: *mut linux_mlist_v0,
    pub start_adr: *mut i8,
    pub num_bytes: u32,
}
#[repr(C)]
pub struct linux_mem_v0 {
    pub v0_totphys: *mut *mut linux_mlist_v0,
    pub v0_prommap: *mut *mut linux_mlist_v0,
    pub v0_available: *mut *mut linux_mlist_v0,
}
#[repr(C)]
pub struct linux_arguments_v0 {
    pub argv: [*mut i8; 8],
    pub args: [i8; 100],
    pub boot_dev: [i8; 2],
    pub boot_dev_ctrl: i32,
    pub boot_dev_unit: i32,
    pub dev_partition: i32,
    pub kernel_file_name: *mut i8,
    pub aieee1: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct linux_bootargs_v2 {
    pub bootpath: *mut *mut i8,
    pub bootargs: *mut *mut i8,
    pub fd_stdin: *mut i32,
    pub fd_stdout: *mut i32,
}

#[cfg(any(feature = "CONFIG_SUN3", feature = "CONFIG_SUN3X"))]
#[repr(C)]
pub struct linux_romvec {
    pub pv_initsp: *mut i8,
    pub pv_startmon: Option<unsafe extern "C" fn() -> i32>,
    pub diagberr: *mut i32,
    pub pv_v0bootargs: *mut *mut linux_arguments_v0,
    pub pv_sun3mem: *mut u32,
    pub pv_getchar: Option<unsafe extern "C" fn() -> u8>,
    pub pv_putchar: Option<unsafe extern "C" fn(i32)>,
    pub pv_nbgetchar: Option<unsafe extern "C" fn() -> i32>,
    pub pv_nbputchar: Option<unsafe extern "C" fn(i32)>,
    pub pv_echo: *mut u8,
    pub pv_insource: *mut u8,
    pub pv_outsink: *mut u8,
    pub pv_getkey: Option<unsafe extern "C" fn() -> i32>,
    pub pv_initgetkey: Option<unsafe extern "C" fn() -> i32>,
    pub pv_translation: *mut u32,
    pub pv_keybid: *mut u8,
    pub pv_screen_x: *mut i32,
    pub pv_screen_y: *mut i32,
    pub pv_keybuf: *mut core::ffi::c_void,
    pub pv_monid: *mut i8,
    pub pv_fbwritechar: Option<unsafe extern "C" fn(i8) -> i32>,
    pub pv_fbaddr: *mut i32,
    pub pv_font: *mut *mut i8,
    pub pv_fbwritestr: Option<unsafe extern "C" fn(i8) -> i32>,
    pub pv_reboot: Option<unsafe extern "C" fn(*mut i8)>,
    pub pv_linebuf: *mut u8,
    pub pv_lineptr: *mut *mut u8,
    pub pv_linesize: *mut i32,
    pub pv_getline: Option<unsafe extern "C" fn() -> i32>,
    pub pv_getnextchar: Option<unsafe extern "C" fn() -> u8>,
    pub pv_peeknextchar: Option<unsafe extern "C" fn() -> u8>,
    pub pv_fbthere: *mut i32,
    pub pv_getnum: Option<unsafe extern "C" fn() -> i32>,
    pub pv_printf: Option<unsafe extern "C" fn(*const i8, ...)>,
    pub pv_printhex: Option<unsafe extern "C" fn() -> i32>,
    pub pv_leds: *mut u8,
    pub pv_setleds: Option<unsafe extern "C" fn() -> i32>,
    pub pv_nmiaddr: Option<unsafe extern "C" fn() -> i32>,
    pub pv_abortentry: Option<unsafe extern "C" fn() -> i32>,
    pub pv_nmiclock: *mut i32,
    pub pv_fbtype: *mut i32,
    pub pv_romvers: u32,
    pub pv_globram: *mut core::ffi::c_void,
    pub pv_kbdzscc: *mut i8,
    pub pv_keyrinit: *mut i32,
    pub pv_keyrtick: *mut u8,
    pub pv_memoryavail: *mut u32,
    pub pv_resetaddr: *mut isize,
    pub pv_resetmap: *mut isize,
    pub pv_halt: Option<unsafe extern "C" fn()>,
    pub pv_memorybitmap: *mut u8,
    #[cfg(feature = "CONFIG_SUN3")]
    pub pv_setctxt: Option<unsafe extern "C" fn(i32, *mut i8, i32)>,
    #[cfg(feature = "CONFIG_SUN3")]
    pub pv_vector_cmd: Option<unsafe extern "C" fn()>,
    #[cfg(feature = "CONFIG_SUN3")]
    pub dummy1z: i32,
    #[cfg(feature = "CONFIG_SUN3")]
    pub dummy2z: i32,
    #[cfg(feature = "CONFIG_SUN3")]
    pub dummy3z: i32,
    #[cfg(feature = "CONFIG_SUN3")]
    pub dummy4z: i32,
}

#[repr(C)]
pub union linux_fortheval {
    pub v0_eval: Option<unsafe extern "C" fn(i32, *mut i8)>,
    pub v2_eval: Option<unsafe extern "C" fn(*mut i8)>,
}

#[repr(C)]
pub struct linux_nodeops {
    pub no_nextnode: Option<unsafe extern "C" fn(i32) -> i32>,
    pub no_child: Option<unsafe extern "C" fn(i32) -> i32>,
    pub no_proplen: Option<unsafe extern "C" fn(i32, *mut i8) -> i32>,
    pub no_getprop: Option<unsafe extern "C" fn(i32, *mut i8, *mut i8) -> i32>,
    pub no_setprop: Option<unsafe extern "C" fn(i32, *mut i8, *mut i8, i32) -> i32>,
    pub no_nextprop: Option<unsafe extern "C" fn(i32, *mut i8) -> *mut i8>,
}

pub const PROMREG_MAX: usize = 16;
pub const PROMVADDR_MAX: usize = 16;
pub const PROMINTR_MAX: usize = 15;
pub const PROMDEV_KBD: i32 = 0;
pub const PROMDEV_SCREEN: i32 = 0;
pub const PROMDEV_TTYA: i32 = 1;
pub const PROMDEV_TTYB: i32 = 2;

#[repr(C)]
pub struct linux_prom_registers {
    pub which_io: i32,
    pub phys_addr: *mut i8,
    pub reg_size: i32,
}
#[repr(C)]
pub struct linux_prom_irqs {
    pub pri: i32,
    pub vector: i32,
}
#[repr(C)]
pub struct linux_prom_ranges {
    pub ot_child_space: u32,
    pub ot_child_base: u32,
    pub ot_parent_space: u32,
    pub ot_parent_base: u32,
    pub or_size: u32,
}

// The original file defines linux_romvec differently for SUN3/SUN3X versus other targets.
// External opaque types referenced by the SUN3 layout are supplied by dependent headers.
#[cfg(not(any(feature = "CONFIG_SUN3", feature = "CONFIG_SUN3X")))]
#[repr(C)]
pub struct linux_romvec {
    pub pv_magic_cookie: u32,
    pub pv_romvers: u32,
    pub pv_plugin_revision: u32,
    pub pv_printrev: u32,
    pub pv_v0mem: linux_mem_v0,
    pub pv_nodeops: *mut linux_nodeops,
    pub pv_bootstr: *mut *mut i8,
    pub pv_v0devops: linux_dev_v0_funcs,
    pub pv_stdin: *mut i8,
    pub pv_stdout: *mut i8,
    pub pv_getchar: Option<unsafe extern "C" fn() -> i32>,
    pub pv_putchar: Option<unsafe extern "C" fn(i32)>,
    pub pv_nbgetchar: Option<unsafe extern "C" fn() -> i32>,
    pub pv_nbputchar: Option<unsafe extern "C" fn(i32)>,
    pub pv_putstr: Option<unsafe extern "C" fn(*mut i8, i32)>,
    pub pv_reboot: Option<unsafe extern "C" fn(*mut i8)>,
    pub pv_printf: Option<unsafe extern "C" fn(*const i8, ...)>,
    pub pv_abort: Option<unsafe extern "C" fn()>,
    pub pv_ticks: *mut i32,
    pub pv_halt: Option<unsafe extern "C" fn()>,
    pub pv_synchook: *mut Option<unsafe extern "C" fn()>,
    pub pv_fortheval: linux_fortheval,
    pub pv_v0bootargs: *mut *mut linux_arguments_v0,
    pub pv_enaddr: Option<unsafe extern "C" fn(i32, *mut i8) -> u32>,
    pub pv_v2bootargs: linux_bootargs_v2,
    pub pv_v2devops: linux_dev_v2_funcs,
    pub filler: [i32; 15],
    pub pv_setctxt: Option<unsafe extern "C" fn(i32, *mut i8, i32)>,
    pub v3_cpustart: Option<unsafe extern "C" fn(u32, i32, i32, *mut i8) -> i32>,
    pub v3_cpustop: Option<unsafe extern "C" fn(u32) -> i32>,
    pub v3_cpuidle: Option<unsafe extern "C" fn(u32) -> i32>,
    pub v3_cpuresume: Option<unsafe extern "C" fn(u32) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
