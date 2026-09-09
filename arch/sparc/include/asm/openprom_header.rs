/* SPDX-License-Identifier: GPL-2.0 */

/* openprom.h: Prom structures and defines for access to the OPENBOOT
 * prom routines and data areas.
 *
 * Copyright (C) 1995,1996 David S. Miller (davem@caip.rutgers.edu)
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub const LINUX_OPPROM_MAGIC: c_uint = 0x10010407;

pub const PROMDEV_KBD: c_int = 0;
pub const PROMDEV_SCREEN: c_int = 0;
pub const PROMDEV_TTYA: c_int = 1;
pub const PROMDEV_TTYB: c_int = 2;

#[repr(C)]
pub struct linux_dev_v0_funcs {
    pub v0_devopen: Option<unsafe extern "C" fn(*mut c_char) -> c_int>,
    pub v0_devclose: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub v0_rdblkdev: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_char) -> c_int>,
    pub v0_wrblkdev: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_char) -> c_int>,
    pub v0_wrnetdev: Option<unsafe extern "C" fn(c_int, c_int, *mut c_char) -> c_int>,
    pub v0_rdnetdev: Option<unsafe extern "C" fn(c_int, c_int, *mut c_char) -> c_int>,
    pub v0_rdchardev: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_char) -> c_int>,
    pub v0_wrchardev: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_char) -> c_int>,
    pub v0_seekdev: Option<unsafe extern "C" fn(c_int, c_long, c_int) -> c_int>,
}

#[repr(C)]
pub struct linux_dev_v2_funcs {
    pub v2_inst2pkg: Option<unsafe extern "C" fn(c_int) -> phandle>,
    pub v2_dumb_mem_alloc: Option<unsafe extern "C" fn(*mut c_char, c_uint) -> *mut c_char>,
    pub v2_dumb_mem_free: Option<unsafe extern "C" fn(*mut c_char, c_uint)>,
    pub v2_dumb_mmap: Option<unsafe extern "C" fn(*mut c_char, c_int, c_uint, c_uint) -> *mut c_char>,
    pub v2_dumb_munmap: Option<unsafe extern "C" fn(*mut c_char, c_uint)>,
    pub v2_dev_open: Option<unsafe extern "C" fn(*mut c_char) -> c_int>,
    pub v2_dev_close: Option<unsafe extern "C" fn(c_int)>,
    pub v2_dev_read: Option<unsafe extern "C" fn(c_int, *mut c_char, c_int) -> c_int>,
    pub v2_dev_write: Option<unsafe extern "C" fn(c_int, *const c_char, c_int) -> c_int>,
    pub v2_dev_seek: Option<unsafe extern "C" fn(c_int, c_int, c_int) -> c_int>,
    pub v2_wheee2: Option<unsafe extern "C" fn()>,
    pub v2_wheee3: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct linux_mlist_v0 {
    pub theres_more: *mut linux_mlist_v0,
    pub start_adr: c_uint,
    pub num_bytes: c_uint,
}

#[repr(C)]
pub struct linux_mem_v0 {
    pub v0_totphys: *mut *mut linux_mlist_v0,
    pub v0_prommap: *mut *mut linux_mlist_v0,
    pub v0_available: *mut *mut linux_mlist_v0,
}

#[repr(C)]
pub struct linux_arguments_v0 {
    pub argv: [*mut c_char; 8],
    pub args: [c_char; 100],
    pub boot_dev: [c_char; 2],
    pub boot_dev_ctrl: c_int,
    pub boot_dev_unit: c_int,
    pub dev_partition: c_int,
    pub kernel_file_name: *mut c_char,
    pub aieee1: *mut c_void,
}

#[repr(C)]
pub struct linux_bootargs_v2 {
    pub bootpath: *mut *mut c_char,
    pub bootargs: *mut *mut c_char,
    pub fd_stdin: *mut c_int,
    pub fd_stdout: *mut c_int,
}

#[repr(C)]
pub union linux_romvec_pv_fortheval {
    pub v0_eval: Option<unsafe extern "C" fn(c_int, *mut c_char)>,
    pub v2_eval: Option<unsafe extern "C" fn(*mut c_char)>,
}

#[repr(C)]
pub struct linux_romvec {
    pub pv_magic_cookie: c_uint,
    pub pv_romvers: c_uint,
    pub pv_plugin_revision: c_uint,
    pub pv_printrev: c_uint,
    pub pv_v0mem: linux_mem_v0,
    pub pv_nodeops: *mut linux_nodeops,
    pub pv_bootstr: *mut *mut c_char,
    pub pv_v0devops: linux_dev_v0_funcs,
    pub pv_stdin: *mut c_char,
    pub pv_stdout: *mut c_char,
    pub pv_getchar: Option<unsafe extern "C" fn() -> c_int>,
    pub pv_putchar: Option<unsafe extern "C" fn(c_int)>,
    pub pv_nbgetchar: Option<unsafe extern "C" fn() -> c_int>,
    pub pv_nbputchar: Option<unsafe extern "C" fn(c_int)>,
    pub pv_putstr: Option<unsafe extern "C" fn(*mut c_char, c_int)>,
    pub pv_reboot: Option<unsafe extern "C" fn(*mut c_char)>,
    pub pv_printf: Option<unsafe extern "C" fn(*const c_char, ...)>,
    pub pv_abort: Option<unsafe extern "C" fn()>,
    pub pv_ticks: *mut c_int,
    pub pv_halt: Option<unsafe extern "C" fn()>,
    pub pv_synchook: *mut Option<unsafe extern "C" fn()>,
    pub pv_fortheval: linux_romvec_pv_fortheval,
    pub pv_v0bootargs: *mut *mut linux_arguments_v0,
    pub pv_enaddr: Option<unsafe extern "C" fn(c_int, *mut c_char) -> c_uint>,
    pub pv_v2bootargs: linux_bootargs_v2,
    pub pv_v2devops: linux_dev_v2_funcs,
    pub filler: [c_int; 15],
    pub pv_setctxt: Option<unsafe extern "C" fn(c_int, *mut c_char, c_int)>,
    pub v3_cpustart: Option<unsafe extern "C" fn(c_uint, c_int, c_int, *mut c_char) -> c_int>,
    pub v3_cpustop: Option<unsafe extern "C" fn(c_uint) -> c_int>,
    pub v3_cpuidle: Option<unsafe extern "C" fn(c_uint) -> c_int>,
    pub v3_cpuresume: Option<unsafe extern "C" fn(c_uint) -> c_int>,
}

#[repr(C)]
pub struct linux_nodeops {
    pub no_nextnode: Option<unsafe extern "C" fn(phandle) -> phandle>,
    pub no_child: Option<unsafe extern "C" fn(phandle) -> phandle>,
    pub no_proplen: Option<unsafe extern "C" fn(phandle, *const c_char) -> c_int>,
    pub no_getprop: Option<unsafe extern "C" fn(phandle, *const c_char, *mut c_char) -> c_int>,
    pub no_setprop: Option<unsafe extern "C" fn(phandle, *const c_char, *mut c_char, c_int) -> c_int>,
    pub no_nextprop: Option<unsafe extern "C" fn(phandle, *mut c_char) -> *mut c_char>,
}

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
pub const PROMREG_MAX: c_int = 24;
#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
pub const PROMREG_MAX: c_int = 16;
pub const PROMVADDR_MAX: c_int = 16;
#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
pub const PROMINTR_MAX: c_int = 32;
#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
pub const PROMINTR_MAX: c_int = 15;

#[repr(C)]
pub struct linux_prom_registers {
    pub which_io: c_uint,
    pub phys_addr: c_uint,
    pub reg_size: c_uint,
}

#[repr(C)]
pub struct linux_prom64_registers {
    pub phys_addr: c_ulong,
    pub reg_size: c_ulong,
}

#[repr(C)]
pub struct linux_prom_irqs {
    pub pri: c_int,
    pub vector: c_int,
}

#[repr(C)]
pub struct linux_prom_ranges {
    pub ot_child_space: c_uint,
    pub ot_child_base: c_uint,
    pub ot_parent_space: c_uint,
    pub ot_parent_base: c_uint,
    pub or_size: c_uint,
}

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[repr(C)]
pub struct linux_prom_pci_registers {
    pub phys_hi: c_uint,
    pub phys_mid: c_uint,
    pub phys_lo: c_uint,
    pub size_hi: c_uint,
    pub size_lo: c_uint,
}

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
#[repr(C)]
pub struct linux_prom_pci_registers {
    pub which_io: c_uint,
    pub phys_hi: c_uint,
    pub phys_lo: c_uint,
    pub size_hi: c_uint,
    pub size_lo: c_uint,
}

#[repr(C)]
pub struct linux_prom_pci_ranges {
    pub child_phys_hi: c_uint,
    pub child_phys_mid: c_uint,
    pub child_phys_lo: c_uint,
    pub parent_phys_hi: c_uint,
    pub parent_phys_lo: c_uint,
    pub size_hi: c_uint,
    pub size_lo: c_uint,
}

#[repr(C)]
pub struct linux_prom_pci_intmap {
    pub phys_hi: c_uint,
    pub phys_mid: c_uint,
    pub phys_lo: c_uint,
    pub interrupt: c_uint,
    pub cnode: c_int,
    pub cinterrupt: c_uint,
}

#[repr(C)]
pub struct linux_prom_pci_intmask {
    pub phys_hi: c_uint,
    pub phys_mid: c_uint,
    pub phys_lo: c_uint,
    pub interrupt: c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
