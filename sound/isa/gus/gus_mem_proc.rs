// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  GUS's memory access via proc filesystem
 */

// C dependencies: <linux/slab.h>, <sound/core.h>, <sound/gus.h>, <sound/info.h>

use core::ffi::{c_char, c_int, c_void};

type ssize_t = isize;
type size_t = usize;
type loff_t = i64;

const ENOMEM: c_int = 12;
const SNDRV_INFO_CONTENT_DATA: c_int = 2;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_gus_mem_block {
    pub address: u32,
    pub size: u32,
}

#[repr(C)]
pub struct snd_gus_mem_alloc {
    pub banks_8: [snd_gus_mem_block; 4],
}

#[repr(C)]
pub struct snd_gus_gf1 {
    pub mem_alloc: snd_gus_mem_alloc,
    pub rom_present: c_int,
    pub rom_memory: u32,
}

#[repr(C)]
pub struct snd_gus_card {
    pub card: *mut snd_card,
    pub gf1: snd_gus_gf1,
}

#[repr(C)]
pub struct snd_info_entry_ops {
    pub read: Option<
        unsafe extern "C" fn(
            entry: *mut snd_info_entry,
            file_private_data: *mut c_void,
            file: *mut file,
            buf: *mut c_char,
            count: size_t,
            pos: loff_t,
        ) -> ssize_t,
    >,
}

#[repr(C)]
pub union snd_info_entry_c {
    pub ops: *const snd_info_entry_ops,
}

#[repr(C)]
pub struct snd_info_entry {
    pub content: c_int,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(entry: *mut snd_info_entry)>,
    pub c: snd_info_entry_c,
    pub size: u32,
}

#[repr(C)]
struct gus_proc_private {
    rom: c_int, /* data are in ROM */
    address: u32,
    size: u32,
    gus: *mut snd_gus_card,
}

unsafe extern "C" {
    fn kfree(ptr: *const c_void);
    fn snd_gus_dram_read(
        gus: *mut snd_gus_card,
        buf: *mut c_char,
        pos: loff_t,
        count: size_t,
        rom: c_int,
    ) -> c_int;
    fn snd_card_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        entry: *mut *mut snd_info_entry,
    ) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
}

// Rust stand-in for the C kzalloc_obj(*ptr) macro from the included headers.
unsafe extern "C" {
    fn kzalloc_obj_gus_proc_private() -> *mut gus_proc_private;
}

unsafe extern "C" fn snd_gf1_mem_proc_dump(
    entry: *mut snd_info_entry,
    _file_private_data: *mut c_void,
    _file: *mut file,
    buf: *mut c_char,
    count: size_t,
    pos: loff_t,
) -> ssize_t {
    let priv_0: *mut gus_proc_private = (*entry).private_data as *mut gus_proc_private;
    let gus: *mut snd_gus_card = (*priv_0).gus;
    let err: c_int;

    err = snd_gus_dram_read(gus, buf, pos, count, (*priv_0).rom);
    if err < 0 {
        return err as ssize_t;
    }
    count as ssize_t
}

unsafe extern "C" fn snd_gf1_mem_proc_free(entry: *mut snd_info_entry) {
    let priv_0: *mut gus_proc_private = (*entry).private_data as *mut gus_proc_private;
    kfree(priv_0 as *const c_void);
}

static snd_gf1_mem_proc_ops: snd_info_entry_ops = snd_info_entry_ops {
    read: Some(snd_gf1_mem_proc_dump),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_gf1_mem_proc_init(gus: *mut snd_gus_card) -> c_int {
    let mut idx: c_int;
    let mut name: [c_char; 16] = [0; 16];
    let mut priv_0: *mut gus_proc_private;
    let mut entry: *mut snd_info_entry = core::ptr::null_mut();

    idx = 0;
    while idx < 4 {
        if (*gus).gf1.mem_alloc.banks_8[idx as usize].size > 0 {
            priv_0 = kzalloc_obj_gus_proc_private();
            if priv_0 == core::ptr::null_mut() {
                return -ENOMEM;
            }
            (*priv_0).gus = gus;
            sprintf(name.as_mut_ptr(), c"gus-ram-%i".as_ptr(), idx);
            if snd_card_proc_new((*gus).card, name.as_ptr(), &mut entry) == 0 {
                (*entry).content = SNDRV_INFO_CONTENT_DATA;
                (*entry).private_data = priv_0 as *mut c_void;
                (*entry).private_free = Some(snd_gf1_mem_proc_free);
                (*entry).c.ops = &snd_gf1_mem_proc_ops;
                (*priv_0).address = (*gus).gf1.mem_alloc.banks_8[idx as usize].address;
                (*entry).size = (*gus).gf1.mem_alloc.banks_8[idx as usize].size;
                (*priv_0).size = (*entry).size;
            }
        }
        idx += 1;
    }
    idx = 0;
    while idx < 4 {
        if ((*gus).gf1.rom_present & (1 << idx)) != 0 {
            priv_0 = kzalloc_obj_gus_proc_private();
            if priv_0 == core::ptr::null_mut() {
                return -ENOMEM;
            }
            (*priv_0).rom = 1;
            (*priv_0).gus = gus;
            sprintf(name.as_mut_ptr(), c"gus-rom-%i".as_ptr(), idx);
            if snd_card_proc_new((*gus).card, name.as_ptr(), &mut entry) == 0 {
                (*entry).content = SNDRV_INFO_CONTENT_DATA;
                (*entry).private_data = priv_0 as *mut c_void;
                (*entry).private_free = Some(snd_gf1_mem_proc_free);
                (*entry).c.ops = &snd_gf1_mem_proc_ops;
                (*priv_0).address = (idx * 4096 * 1024) as u32;
                (*entry).size = (*gus).gf1.rom_memory;
                (*priv_0).size = (*entry).size;
            }
        }
        idx += 1;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
