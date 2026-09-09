// SPDX-License-Identifier: GPL-2.0-or-later
/* Support for hardware buffer manager.
 *
 * Copyright (C) 2016 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 */

// External kernel declarations supplied by the surrounding repository.
use core::ffi::c_void;
use core::ptr;

extern "C" {
    fn skb_free_frag(buf: *mut c_void);
    fn kfree(buf: *mut c_void);
    fn netdev_alloc_frag(size: usize) -> *mut c_void;
    fn kmalloc(size: usize, gfp: gfp_t) -> *mut c_void;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn pr_warn(fmt: *const u8, ...);
    fn pr_debug(fmt: *const u8, ...);
}

pub type gfp_t = u32;

pub const GFP_KERNEL: gfp_t = 0;
pub const ENOMEM: i32 = 12;
pub const PAGE_SIZE: i32 = 4096;

#[repr(C)]
pub struct hwbm_pool {
    pub frag_size: i32,
    pub buf_num: u32,
    pub size: u32,
    pub buf_lock: *mut c_void,
    pub construct: Option<unsafe extern "C" fn(*mut hwbm_pool, *mut c_void) -> i32>,
}

pub unsafe extern "C" fn hwbm_buf_free(bm_pool: *mut hwbm_pool, buf: *mut c_void) {
    if (*bm_pool).frag_size <= PAGE_SIZE {
        skb_free_frag(buf);
    } else {
        kfree(buf);
    }
}

/* Refill processing for HW buffer management */
pub unsafe extern "C" fn hwbm_pool_refill(
    bm_pool: *mut hwbm_pool,
    gfp: gfp_t,
) -> i32 {
    let frag_size = (*bm_pool).frag_size;
    let buf: *mut c_void;

    if frag_size <= PAGE_SIZE {
        buf = netdev_alloc_frag(frag_size as usize);
    } else {
        buf = kmalloc(frag_size as usize, gfp);
    }

    if buf.is_null() {
        return -ENOMEM;
    }

    if let Some(construct) = (*bm_pool).construct {
        if construct(bm_pool, buf) != 0 {
            hwbm_buf_free(bm_pool, buf);
            return -ENOMEM;
        }
    }

    0
}

pub unsafe extern "C" fn hwbm_pool_add(
    bm_pool: *mut hwbm_pool,
    buf_num: u32,
) -> i32 {
    let mut err: i32;
    let mut i: u32 = 0;

    mutex_lock((*bm_pool).buf_lock);
    if (*bm_pool).buf_num == (*bm_pool).size {
        pr_warn(b"pool already filled\0".as_ptr());
        mutex_unlock((*bm_pool).buf_lock);
        return (*bm_pool).buf_num as i32;
    }

    if buf_num.wrapping_add((*bm_pool).buf_num) > (*bm_pool).size {
        pr_warn(b"cannot allocate %d buffers for pool\n\0".as_ptr(), buf_num);
        mutex_unlock((*bm_pool).buf_lock);
        return 0;
    }

    if buf_num.wrapping_add((*bm_pool).buf_num) < (*bm_pool).buf_num {
        pr_warn(
            b"Adding %d buffers to the %d current buffers will overflow\n\0".as_ptr(),
            buf_num,
            (*bm_pool).buf_num,
        );
        mutex_unlock((*bm_pool).buf_lock);
        return 0;
    }

    while i < buf_num {
        err = hwbm_pool_refill(bm_pool, GFP_KERNEL);
        if err < 0 {
            break;
        }
        i += 1;
    }

    /* Update BM driver with number of buffers added to pool */
    (*bm_pool).buf_num = (*bm_pool).buf_num.wrapping_add(i);

    pr_debug(
        b"hwpm pool: %d of %d buffers added\n\0".as_ptr(),
        i,
        buf_num,
    );
    mutex_unlock((*bm_pool).buf_lock);

    i as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
