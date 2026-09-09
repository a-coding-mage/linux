/* SPDX-License-Identifier: GPL-2.0 */
/*
  nubus.h: various definitions and prototypes for NuBus drivers to use.

  Originally written by Alan Cox.

  Hacked to death by C. Scott Ananian and David Huggins-Daines.
*/

/* Dependencies supplied by the surrounding kernel translation. */

use core::ffi::c_void;

#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct nubus_dir {
    pub base: *mut u8,
    pub ptr: *mut u8,
    pub done: i32,
    pub mask: i32,
    pub procdir: *mut proc_dir_entry,
}

#[repr(C)]
pub struct nubus_dirent {
    pub base: *mut u8,
    pub type_: u8,
    pub data: u32, /* Actually 24 bits used */
    pub mask: i32,
}

#[repr(C)]
pub struct nubus_board {
    pub dev: device,
    /* Only 9-E actually exist, though 0-8 are also theoretically
       possible, and 0 is a special case which represents the
       motherboard and onboard peripherals (Ethernet, video) */
    pub slot: i32,
    /* For slot 0, this is bogus. */
    pub name: [u8; 64],
    /* Format block */
    pub fblock: *mut u8,
    /* Root directory (does *not* always equal fblock + doffset!) */
    pub directory: *mut u8,
    pub slot_addr: usize,
    /* Offset to root directory (sometimes) */
    pub doffset: usize,
    /* Length over which to compute the crc */
    pub rom_length: usize,
    /* Completely useless most of the time */
    pub crc: usize,
    pub rev: u8,
    pub format: u8,
    pub lanes: u8,
    /* Directory entry in /proc/bus/nubus */
    pub procdir: *mut proc_dir_entry,
}

#[repr(C)]
pub struct nubus_rsrc {
    pub list: list_head,
    /* The functional resource ID */
    pub resid: u8,
    /* These are mostly here for convenience; we could always read
       them from the ROMs if we wanted to */
    pub category: u16,
    pub type_: u16,
    pub dr_sw: u16,
    pub dr_hw: u16,
    /* Functional directory */
    pub directory: *mut u8,
    /* Much of our info comes from here */
    pub board: *mut nubus_board,
}

/* This is all NuBus functional resources (used to find devices later on) */
unsafe extern "C" {
    pub static mut nubus_func_rsrcs: list_head;

    pub fn nubus_first_rsrc_or_null() -> *mut nubus_rsrc;
    pub fn nubus_next_rsrc_or_null(from: *mut nubus_rsrc) -> *mut nubus_rsrc;

    pub fn nubus_get_root_dir(board: *const nubus_board, dir: *mut nubus_dir) -> i32;
    pub fn nubus_get_board_dir(board: *const nubus_board, dir: *mut nubus_dir) -> i32;
    pub fn nubus_get_func_dir(fres: *const nubus_rsrc, dir: *mut nubus_dir) -> i32;
    pub fn nubus_readdir(dir: *mut nubus_dir, ent: *mut nubus_dirent) -> i32;
    pub fn nubus_find_rsrc(dir: *mut nubus_dir, rsrc_type: u8, ent: *mut nubus_dirent) -> i32;
    pub fn nubus_rewinddir(dir: *mut nubus_dir) -> i32;
    pub fn nubus_get_subdir(ent: *const nubus_dirent, dir: *mut nubus_dir) -> i32;
    pub fn nubus_get_rsrc_mem(dest: *mut c_void, dirent: *const nubus_dirent, len: u32);
    pub fn nubus_get_rsrc_str(dest: *mut i8, dirent: *const nubus_dirent, len: u32) -> u32;
    pub fn nubus_seq_write_rsrc_mem(m: *mut seq_file, dirent: *const nubus_dirent, len: u32);
    pub fn nubus_dirptr(nd: *const nubus_dirent) -> *mut u8;
    pub fn nubus_device_register(parent: *mut device, board: *mut nubus_board) -> i32;
    pub fn nubus_driver_register(ndrv: *mut nubus_driver) -> i32;
    pub fn nubus_driver_unregister(ndrv: *mut nubus_driver);
    pub fn nubus_proc_show(m: *mut seq_file, data: *mut c_void) -> i32;
}

#[repr(C)]
pub struct nubus_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(board: *mut nubus_board) -> i32>,
    pub remove: Option<unsafe extern "C" fn(board: *mut nubus_board)>,
}

/* Generic NuBus interface functions, modelled after the PCI interface. */
#[cfg(feature = "CONFIG_PROC_FS")]
unsafe extern "C" {
    pub static mut nubus_populate_procfs: bool;
    pub fn nubus_proc_init();
    pub fn nubus_proc_add_board(board: *mut nubus_board) -> *mut proc_dir_entry;
    pub fn nubus_proc_add_rsrc_dir(procdir: *mut proc_dir_entry, ent: *const nubus_dirent,
                                   board: *mut nubus_board) -> *mut proc_dir_entry;
    pub fn nubus_proc_add_rsrc_mem(procdir: *mut proc_dir_entry, ent: *const nubus_dirent, size: u32);
    pub fn nubus_proc_add_rsrc(procdir: *mut proc_dir_entry, ent: *const nubus_dirent);
}

#[cfg(not(feature = "CONFIG_PROC_FS"))]
pub unsafe fn nubus_proc_init() {}
#[cfg(not(feature = "CONFIG_PROC_FS"))]
pub unsafe fn nubus_proc_add_board(_: *mut nubus_board) -> *mut proc_dir_entry { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_PROC_FS"))]
pub unsafe fn nubus_proc_add_rsrc_dir(_: *mut proc_dir_entry, _: *const nubus_dirent, _: *mut nubus_board) -> *mut proc_dir_entry { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_PROC_FS"))]
pub unsafe fn nubus_proc_add_rsrc_mem(_: *mut proc_dir_entry, _: *const nubus_dirent, _: u32) {}
#[cfg(not(feature = "CONFIG_PROC_FS"))]
pub unsafe fn nubus_proc_add_rsrc(_: *mut proc_dir_entry, _: *const nubus_dirent) {}

#[macro_export]
macro_rules! for_each_func_rsrc {
    ($f:ident, $body:block) => {{
        let mut $f = unsafe { $crate::nubus_first_rsrc_or_null() };
        while !$f.is_null() {
            $body
            $f = unsafe { $crate::nubus_next_rsrc_or_null($f) };
        }
    }};
}

#[macro_export]
macro_rules! for_each_board_func_rsrc {
    ($b:expr, $f:ident, $body:block) => {
        $crate::for_each_func_rsrc!($f, {
            if unsafe { (*$f).board } == $b { $body }
        });
    };
}

unsafe extern "C" {
    pub fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    pub fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
}

pub unsafe fn nubus_set_drvdata(board: *mut nubus_board, data: *mut c_void) {
    dev_set_drvdata(core::ptr::addr_of_mut!((*board).dev), data);
}

pub unsafe fn nubus_get_drvdata(board: *mut nubus_board) -> *mut c_void {
    dev_get_drvdata(core::ptr::addr_of_mut!((*board).dev))
}

/* Returns a pointer to the "standard" slot space. */
pub unsafe fn nubus_slot_addr(slot: i32) -> *mut c_void {
    ((0xF0000000u32 | ((slot as u32) << 24)) as usize) as *mut c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
