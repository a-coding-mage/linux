// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  GUS's memory allocation routines / bottom layer
 */

// C includes translated as dependency intent:
// <linux/slab.h>, <linux/string.h>, <sound/core.h>, <sound/gus.h>, <sound/info.h>

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

const SNDRV_GF1_MEM_BLOCK_16BIT: c_uint = 1;
const SNDRV_GF1_MEM_OWNER_DRIVER: c_int = 0;
const SNDRV_GF1_MEM_OWNER_WAVE_SIMPLE: c_int = 1;
const SNDRV_GF1_MEM_OWNER_WAVE_GF1: c_int = 2;
const SNDRV_GF1_MEM_OWNER_WAVE_IWFFFF: c_int = 3;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_gf1_bank_info {
    pub address: c_uint,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_gf1_mem_block {
    pub flags: c_uint,
    pub owner: c_int,
    pub share: c_int,
    pub share_id: [c_uint; 4],
    pub name: *mut c_char,
    pub prev: *mut snd_gf1_mem_block,
    pub next: *mut snd_gf1_mem_block,
    pub ptr: c_uint,
    pub size: c_uint,
}

#[repr(C)]
pub struct snd_gf1_mem {
    pub memory_mutex: mutex,
    pub first: *mut snd_gf1_mem_block,
    pub last: *mut snd_gf1_mem_block,
    pub banks_8: [snd_gf1_bank_info; 4],
    pub banks_16: [snd_gf1_bank_info; 4],
}

#[repr(C)]
pub struct snd_gf1 {
    pub mem_alloc: snd_gf1_mem,
    pub memory: c_uint,
    pub enh_mode: c_int,
    pub default_voice_address: c_uint,
}

#[repr(C)]
pub struct snd_gus_card {
    pub card: *mut snd_card,
    pub gf1: snd_gf1,
}

unsafe extern "C" {
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kstrdup(s: *const c_char, flags: c_uint) -> *mut c_char;
    fn kfree(ptr: *const c_void);
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn pr_err(fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut c_void,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    ) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
}

#[inline]
fn ALIGN(x: c_uint, a: c_int) -> c_uint {
    let a = a as c_uint;
    (x.wrapping_add(a).wrapping_sub(1)) & !(a.wrapping_sub(1))
}

unsafe fn kmalloc_obj_snd_gf1_mem_block() -> *mut snd_gf1_mem_block {
    unsafe { kmalloc(size_of::<snd_gf1_mem_block>(), GFP_KERNEL) as *mut snd_gf1_mem_block }
}

struct MutexGuard {
    lock: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        unsafe { mutex_lock(lock) };
        Self { lock }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe { mutex_unlock(self.lock) };
    }
}

#[cfg(CONFIG_SND_DEBUG)]
unsafe extern "C" fn snd_gf1_mem_info_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
);

unsafe fn snd_gf1_mem_xalloc(
    alloc: *mut snd_gf1_mem,
    block: *mut snd_gf1_mem_block,
    name: *const c_char,
) -> *mut snd_gf1_mem_block {
    let mut pblock: *mut snd_gf1_mem_block;
    let nblock: *mut snd_gf1_mem_block;

    nblock = unsafe { kmalloc_obj_snd_gf1_mem_block() };
    if nblock.is_null() {
        return ptr::null_mut();
    }
    unsafe {
        *nblock = *block;
        (*nblock).name = kstrdup(name, GFP_KERNEL);
    }
    if unsafe { (*nblock).name.is_null() } {
        unsafe { kfree(nblock as *const c_void) };
        return ptr::null_mut();
    }

    pblock = unsafe { (*alloc).first };
    while !pblock.is_null() {
        if unsafe { (*pblock).ptr > (*nblock).ptr } {
            unsafe {
                (*nblock).prev = (*pblock).prev;
                (*nblock).next = pblock;
                (*pblock).prev = nblock;
                if pblock == (*alloc).first {
                    (*alloc).first = nblock;
                } else {
                    (*(*nblock).prev).next = nblock;
                }
            }
            return nblock;
        }
        pblock = unsafe { (*pblock).next };
    }
    unsafe {
        (*nblock).next = ptr::null_mut();
        if (*alloc).last.is_null() {
            (*nblock).prev = ptr::null_mut();
            (*alloc).first = nblock;
            (*alloc).last = nblock;
        } else {
            (*nblock).prev = (*alloc).last;
            (*(*alloc).last).next = nblock;
            (*alloc).last = nblock;
        }
    }
    nblock
}

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_mem_xfree(
    alloc: *mut snd_gf1_mem,
    block: *mut snd_gf1_mem_block,
) -> c_int {
    if unsafe { (*block).share != 0 } {
        /* ok.. shared block */
        unsafe { (*block).share -= 1 };
        return 0;
    }
    if unsafe { (*alloc).first == block } {
        unsafe {
            (*alloc).first = (*block).next;
            if !(*block).next.is_null() {
                (*(*block).next).prev = ptr::null_mut();
            }
        }
    } else {
        unsafe {
            (*(*block).prev).next = (*block).next;
            if !(*block).next.is_null() {
                (*(*block).next).prev = (*block).prev;
            }
        }
    }
    if unsafe { (*alloc).last == block } {
        unsafe {
            (*alloc).last = (*block).prev;
            if !(*block).prev.is_null() {
                (*(*block).prev).next = ptr::null_mut();
            }
        }
    } else {
        unsafe {
            (*(*block).next).prev = (*block).prev;
            if !(*block).prev.is_null() {
                (*(*block).prev).next = (*block).next;
            }
        }
    }
    unsafe {
        kfree((*block).name as *const c_void);
        kfree(block as *const c_void);
    }
    0
}

unsafe fn snd_gf1_mem_look(
    alloc: *mut snd_gf1_mem,
    address: c_uint,
) -> *mut snd_gf1_mem_block {
    let mut block: *mut snd_gf1_mem_block;

    block = unsafe { (*alloc).first };
    while !block.is_null() {
        if unsafe { (*block).ptr == address } {
            return block;
        }
        block = unsafe { (*block).next };
    }
    ptr::null_mut()
}

unsafe fn snd_gf1_mem_share(
    alloc: *mut snd_gf1_mem,
    share_id: *mut c_uint,
) -> *mut snd_gf1_mem_block {
    let mut block: *mut snd_gf1_mem_block;

    if unsafe {
        *share_id.add(0) == 0
            && *share_id.add(1) == 0
            && *share_id.add(2) == 0
            && *share_id.add(3) == 0
    } {
        return ptr::null_mut();
    }
    block = unsafe { (*alloc).first };
    while !block.is_null() {
        if unsafe {
            memcmp(
                share_id as *const c_void,
                (*block).share_id.as_ptr() as *const c_void,
                size_of_val(&(*block).share_id),
            ) == 0
        } {
            return block;
        }
        block = unsafe { (*block).next };
    }
    ptr::null_mut()
}

unsafe fn snd_gf1_mem_find(
    alloc: *mut snd_gf1_mem,
    block: *mut snd_gf1_mem_block,
    size: c_uint,
    w_16: c_int,
    mut align: c_int,
) -> c_int {
    let info: *mut snd_gf1_bank_info = if w_16 != 0 {
        unsafe { (*alloc).banks_16.as_mut_ptr() }
    } else {
        unsafe { (*alloc).banks_8.as_mut_ptr() }
    };
    let mut idx: c_uint;
    let mut boundary: c_uint;
    let mut size1: c_int;
    let mut pblock: *mut snd_gf1_mem_block;
    let mut ptr1: c_uint;
    let mut ptr2: c_uint;

    if w_16 != 0 && align < 2 {
        align = 2;
    }
    unsafe {
        (*block).flags = if w_16 != 0 { SNDRV_GF1_MEM_BLOCK_16BIT } else { 0 };
        (*block).owner = SNDRV_GF1_MEM_OWNER_DRIVER;
        (*block).share = 0;
        (*block).share_id[0] = 0;
        (*block).share_id[1] = 0;
        (*block).share_id[2] = 0;
        (*block).share_id[3] = 0;
        (*block).name = ptr::null_mut();
        (*block).prev = ptr::null_mut();
        (*block).next = ptr::null_mut();
    }
    pblock = unsafe { (*alloc).first };
    idx = 0;
    while !pblock.is_null() {
        loop {
            boundary = unsafe { (*info.add(idx as usize)).address + (*info.add(idx as usize)).size };
            if unsafe { (*pblock).ptr < boundary } {
                break;
            }
            idx += 1;
        }
        loop {
            boundary = unsafe { (*info.add(idx as usize)).address + (*info.add(idx as usize)).size };
            if unsafe { (*pblock).ptr + (*pblock).size < boundary } {
                break;
            }
            idx += 1;
        }
        ptr2 = boundary;
        if unsafe { !(*pblock).next.is_null() } {
            if unsafe { (*pblock).ptr + (*pblock).size == (*(*pblock).next).ptr } {
                pblock = unsafe { (*pblock).next };
                continue;
            }
            if unsafe { (*(*pblock).next).ptr < boundary } {
                ptr2 = unsafe { (*(*pblock).next).ptr };
            }
        }
        ptr1 = unsafe { ALIGN((*pblock).ptr + (*pblock).size, align) };
        if ptr1 >= ptr2 {
            pblock = unsafe { (*pblock).next };
            continue;
        }
        size1 = ptr2.wrapping_sub(ptr1) as c_int;
        if size as c_int <= size1 {
            unsafe {
                (*block).ptr = ptr1;
                (*block).size = size;
            }
            return 0;
        }
        pblock = unsafe { (*pblock).next };
    }
    loop {
        idx += 1;
        if idx >= 4 {
            break;
        }
        if unsafe { size <= (*info.add(idx as usize)).size } {
            /* I assume that bank address is already aligned.. */
            unsafe {
                (*block).ptr = (*info.add(idx as usize)).address;
                (*block).size = size;
            }
            return 0;
        }
    }
    -ENOMEM
}

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_mem_alloc(
    alloc: *mut snd_gf1_mem,
    owner: c_int,
    name: *mut c_char,
    size: c_int,
    w_16: c_int,
    align: c_int,
    share_id: *mut c_uint,
) -> *mut snd_gf1_mem_block {
    let mut block: snd_gf1_mem_block;
    let mut nblock: *mut snd_gf1_mem_block;

    let _guard = unsafe { MutexGuard::new(&mut (*alloc).memory_mutex) };
    if !share_id.is_null() {
        nblock = unsafe { snd_gf1_mem_share(alloc, share_id) };
        if !nblock.is_null() {
            if size != unsafe { (*nblock).size as c_int } {
                /* TODO: remove in the future */
                unsafe { pr_err(c"%s - share: sizes differ\n".as_ptr(), c"snd_gf1_mem_alloc".as_ptr()) };
                // __std:
            } else {
                unsafe { (*nblock).share += 1 };
                return ptr::null_mut();
            }
        }
    }
    block = unsafe { core::mem::zeroed() };
    if unsafe { snd_gf1_mem_find(alloc, &mut block, size as c_uint, w_16, align) < 0 } {
        return ptr::null_mut();
    }
    if !share_id.is_null() {
        unsafe {
            memcpy(
                block.share_id.as_mut_ptr() as *mut c_void,
                share_id as *const c_void,
                size_of_val(&block.share_id),
            );
        }
    }
    block.owner = owner;
    nblock = unsafe { snd_gf1_mem_xalloc(alloc, &mut block, name as *const c_char) };
    nblock
}

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_mem_free(
    alloc: *mut snd_gf1_mem,
    address: c_uint,
) -> c_int {
    let block: *mut snd_gf1_mem_block;

    let _guard = unsafe { MutexGuard::new(&mut (*alloc).memory_mutex) };
    block = unsafe { snd_gf1_mem_look(alloc, address) };
    if !block.is_null() {
        return unsafe { snd_gf1_mem_xfree(alloc, block) };
    }
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_mem_init(gus: *mut snd_gus_card) -> c_int {
    let alloc: *mut snd_gf1_mem;
    let mut block: snd_gf1_mem_block;

    alloc = unsafe { &mut (*gus).gf1.mem_alloc };
    unsafe { mutex_init(&mut (*alloc).memory_mutex) };
    unsafe {
        (*alloc).first = ptr::null_mut();
        (*alloc).last = ptr::null_mut();
    }
    if unsafe { (*gus).gf1.memory == 0 } {
        return 0;
    }

    block = unsafe { core::mem::zeroed() };
    unsafe {
        memset(
            &mut block as *mut snd_gf1_mem_block as *mut c_void,
            0,
            size_of::<snd_gf1_mem_block>(),
        );
    }
    block.owner = SNDRV_GF1_MEM_OWNER_DRIVER;
    if unsafe { (*gus).gf1.enh_mode != 0 } {
        block.ptr = 0;
        block.size = 1024;
        if unsafe { snd_gf1_mem_xalloc(alloc, &mut block, c"InterWave LFOs".as_ptr()).is_null() } {
            return -ENOMEM;
        }
    }
    block.ptr = unsafe { (*gus).gf1.default_voice_address };
    block.size = 4;
    if unsafe { snd_gf1_mem_xalloc(alloc, &mut block, c"Voice default (NULL's)".as_ptr()).is_null() } {
        return -ENOMEM;
    }
    #[cfg(CONFIG_SND_DEBUG)]
    unsafe {
        snd_card_ro_proc_new(
            (*gus).card,
            c"gusmem".as_ptr(),
            gus as *mut c_void,
            Some(snd_gf1_mem_info_read),
        );
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_gf1_mem_done(gus: *mut snd_gus_card) -> c_int {
    let alloc: *mut snd_gf1_mem;
    let mut block: *mut snd_gf1_mem_block;
    let mut nblock: *mut snd_gf1_mem_block;

    alloc = unsafe { &mut (*gus).gf1.mem_alloc };
    block = unsafe { (*alloc).first };
    while !block.is_null() {
        nblock = unsafe { (*block).next };
        unsafe { snd_gf1_mem_xfree(alloc, block) };
        block = nblock;
    }
    0
}

#[cfg(CONFIG_SND_DEBUG)]
unsafe extern "C" fn snd_gf1_mem_info_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let gus: *mut snd_gus_card;
    let alloc: *mut snd_gf1_mem;
    let mut block: *mut snd_gf1_mem_block;
    let mut total: c_uint;
    let mut used: c_uint;
    let mut i: c_int;

    gus = unsafe { (*entry).private_data as *mut snd_gus_card };
    alloc = unsafe { &mut (*gus).gf1.mem_alloc };
    let _guard = unsafe { MutexGuard::new(&mut (*alloc).memory_mutex) };
    unsafe { snd_iprintf(buffer, c"8-bit banks       : \n    ".as_ptr()) };
    i = 0;
    while i < 4 {
        unsafe {
            snd_iprintf(
                buffer,
                c"0x%06x (%04ik)%s".as_ptr(),
                (*alloc).banks_8[i as usize].address,
                (*alloc).banks_8[i as usize].size >> 10,
                if i + 1 < 4 { c",".as_ptr() } else { c"".as_ptr() },
            );
        }
        i += 1;
    }
    unsafe { snd_iprintf(buffer, c"\n16-bit banks      : \n    ".as_ptr()) };
    total = 0;
    i = 0;
    while i < 4 {
        unsafe {
            snd_iprintf(
                buffer,
                c"0x%06x (%04ik)%s".as_ptr(),
                (*alloc).banks_16[i as usize].address,
                (*alloc).banks_16[i as usize].size >> 10,
                if i + 1 < 4 { c",".as_ptr() } else { c"".as_ptr() },
            );
            total += (*alloc).banks_16[i as usize].size;
        }
        i += 1;
    }
    unsafe { snd_iprintf(buffer, c"\n".as_ptr()) };
    used = 0;
    block = unsafe { (*alloc).first };
    i = 0;
    while !block.is_null() {
        unsafe {
            used += (*block).size;
            snd_iprintf(
                buffer,
                c"Block %i onboard 0x%x size %i (0x%x):\n".as_ptr(),
                i,
                (*block).ptr,
                (*block).size,
                (*block).size,
            );
            if (*block).share != 0
                || (*block).share_id[0] != 0
                || (*block).share_id[1] != 0
                || (*block).share_id[2] != 0
                || (*block).share_id[3] != 0
            {
                snd_iprintf(
                    buffer,
                    c"  Share           : %i [id0 0x%x] [id1 0x%x] [id2 0x%x] [id3 0x%x]\n"
                        .as_ptr(),
                    (*block).share,
                    (*block).share_id[0],
                    (*block).share_id[1],
                    (*block).share_id[2],
                    (*block).share_id[3],
                );
            }
            snd_iprintf(
                buffer,
                c"  Flags           :%s\n".as_ptr(),
                if (*block).flags & SNDRV_GF1_MEM_BLOCK_16BIT != 0 {
                    c" 16-bit".as_ptr()
                } else {
                    c"".as_ptr()
                },
            );
            snd_iprintf(buffer, c"  Owner           : ".as_ptr());
            match (*block).owner {
                SNDRV_GF1_MEM_OWNER_DRIVER => {
                    snd_iprintf(buffer, c"driver - %s\n".as_ptr(), (*block).name);
                }
                SNDRV_GF1_MEM_OWNER_WAVE_SIMPLE => {
                    snd_iprintf(buffer, c"SIMPLE wave\n".as_ptr());
                }
                SNDRV_GF1_MEM_OWNER_WAVE_GF1 => {
                    snd_iprintf(buffer, c"GF1 wave\n".as_ptr());
                }
                SNDRV_GF1_MEM_OWNER_WAVE_IWFFFF => {
                    snd_iprintf(buffer, c"IWFFFF wave\n".as_ptr());
                }
                _ => {
                    snd_iprintf(buffer, c"unknown\n".as_ptr());
                }
            }
            block = (*block).next;
        }
        i += 1;
    }
    unsafe {
        snd_iprintf(
            buffer,
            c"  Total: memory = %i, used = %i, free = %i\n".as_ptr(),
            total,
            used,
            total - used,
        );
    }
    /*
    #if 0
        ultra_iprintf(buffer, "  Verify: free = %i, max 8-bit block = %i, max 16-bit block = %i\n",
                  ultra_memory_free_size(card, &card->gf1.mem_alloc),
              ultra_memory_free_block(card, &card->gf1.mem_alloc, 0),
             ultra_memory_free_block(card, &card->gf1.mem_alloc, 1));
    #endif
    */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
