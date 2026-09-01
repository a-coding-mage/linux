// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File    ctvmem.c
 *
 * @Brief
 * This file contains the implementation of virtual memory management object
 * for card device.
 *
 * @Author Liu Chun
 * @Date Apr 1 2008
 */

// C dependencies: ctvmem.h, ctatc.h, linux/slab.h, linux/mm.h, linux/io.h,
// sound/pcm.h.

use core::ffi::{c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::null_mut;

pub type dma_addr_t = c_ulong;

pub const ENOMEM: c_int = 12;
pub const PAGE_SIZE: c_uint = 4096;
pub const CT_PAGE_SIZE: c_uint = PAGE_SIZE;
pub const CT_PAGE_SHIFT: c_uint = 12;
pub const CT_PTP_NUM: c_int = 1;

pub const fn CT_PAGE_ALIGN(size: c_uint) -> c_uint {
    (size + CT_PAGE_SIZE - 1) & !(CT_PAGE_SIZE - 1)
}

pub const CT_PTES_PER_PAGE: usize = CT_PAGE_SIZE as usize / size_of::<*mut c_void>();
pub const CT_ADDRS_PER_PAGE: usize = CT_PTES_PER_PAGE * CT_PAGE_SIZE as usize;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub area: *mut c_void,
    pub addr: dma_addr_t,
}

#[repr(C)]
pub struct ct_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct ct_atc {
    pub card: *mut ct_card,
}

#[repr(C)]
pub struct ct_vm_block {
    pub list: list_head,
    pub addr: c_uint,
    pub size: c_uint,
}

#[repr(C)]
pub struct ct_vm {
    pub lock: mutex,
    pub ptp: [snd_dma_buffer; CT_PTP_NUM as usize],
    pub size: c_uint,
    pub map: Option<
        unsafe extern "C" fn(
            *mut ct_vm,
            *mut snd_pcm_substream,
            c_int,
        ) -> *mut ct_vm_block,
    >,
    pub unmap: Option<unsafe extern "C" fn(*mut ct_vm, *mut ct_vm_block)>,
    pub get_ptp_phys: Option<unsafe extern "C" fn(*mut ct_vm, c_int) -> dma_addr_t>,
    pub unused: list_head,
    pub used: list_head,
}

unsafe extern "C" {
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);

    fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_move(list: *mut list_head, head: *mut list_head);

    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);

    fn dev_err(dev: *mut device, fmt: *const u8, ...);

    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut ct_atc;
    fn snd_pcm_sgbuf_get_addr(substream: *mut snd_pcm_substream, ofs: c_uint) -> c_ulong;
    fn snd_dma_alloc_pages(
        ty: c_int,
        dev: *mut device,
        size: usize,
        dmab: *mut snd_dma_buffer,
    ) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
}

pub const SNDRV_DMA_TYPE_DEV: c_int = 0;
pub const GFP_KERNEL: c_uint = 0;

#[inline]
unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

#[inline]
unsafe fn list_empty(head: *const list_head) -> bool {
    unsafe { (*head).next == head as *mut list_head }
}

#[inline]
unsafe fn kzalloc_obj<T>() -> *mut T {
    unsafe { kzalloc(size_of::<T>(), GFP_KERNEL) as *mut T }
}

#[inline]
unsafe fn list_entry_ct_vm_block(ptr: *mut list_head) -> *mut ct_vm_block {
    ptr as *mut ct_vm_block
}

struct MutexGuard {
    lock: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        unsafe {
            mutex_lock(lock);
        }
        Self { lock }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe {
            mutex_unlock(self.lock);
        }
    }
}

/* *
 * Find or create vm block based on requested @size.
 * @size must be page aligned.
 * */
unsafe extern "C" fn get_vm_block(
    vm: *mut ct_vm,
    mut size: c_uint,
    atc: *mut ct_atc,
) -> *mut ct_vm_block {
    let mut block: *mut ct_vm_block;
    let mut entry: *mut ct_vm_block = null_mut();
    let mut pos: *mut list_head;

    unsafe {
        size = CT_PAGE_ALIGN(size);
        if size > (*vm).size {
            dev_err(
                (*(*atc).card).dev,
                c"Fail! No sufficient device virtual memory space available!\n".as_ptr()
                    as *const u8,
            );
            return null_mut();
        }

        let _guard = MutexGuard::new(&mut (*vm).lock);
        pos = (*vm).unused.next;
        while pos != &mut (*vm).unused {
            entry = list_entry_ct_vm_block(pos);
            if (*entry).size >= size {
                break; /* found a block that is big enough */
            }
            pos = (*pos).next;
        }
        if pos == &mut (*vm).unused {
            return null_mut();
        }

        if (*entry).size == size {
            /* Move the vm node from unused list to used list directly */
            list_move(&mut (*entry).list, &mut (*vm).used);
            (*vm).size = (*vm).size.wrapping_sub(size);
            return entry;
        }

        block = kzalloc_obj::<ct_vm_block>();
        if block.is_null() {
            return null_mut();
        }

        (*block).addr = (*entry).addr;
        (*block).size = size;
        list_add(&mut (*block).list, &mut (*vm).used);
        (*entry).addr = (*entry).addr.wrapping_add(size);
        (*entry).size = (*entry).size.wrapping_sub(size);
        (*vm).size = (*vm).size.wrapping_sub(size);

        block
    }
}

unsafe extern "C" fn put_vm_block(vm: *mut ct_vm, block: *mut ct_vm_block) {
    let mut entry: *mut ct_vm_block;
    let mut pre_ent: *mut ct_vm_block;
    let mut pos: *mut list_head;
    let mut pre: *mut list_head;

    unsafe {
        (*block).size = CT_PAGE_ALIGN((*block).size);

        let _guard = MutexGuard::new(&mut (*vm).lock);
        list_del(&mut (*block).list);
        (*vm).size = (*vm).size.wrapping_add((*block).size);

        pos = (*vm).unused.next;
        while pos != &mut (*vm).unused {
            entry = list_entry_ct_vm_block(pos);
            if (*entry).addr >= (*block).addr.wrapping_add((*block).size) {
                break; /* found a position */
            }
            pos = (*pos).next;
        }
        if pos == &mut (*vm).unused {
            list_add_tail(&mut (*block).list, &mut (*vm).unused);
            entry = block;
        } else if (*block).addr.wrapping_add((*block).size) == (*entry).addr {
            (*entry).addr = (*block).addr;
            (*entry).size = (*entry).size.wrapping_add((*block).size);
            kfree(block as *mut c_void);
        } else {
            __list_add(&mut (*block).list, (*pos).prev, pos);
            entry = block;
        }

        pos = &mut (*entry).list;
        pre = (*pos).prev;
        while pre != &mut (*vm).unused {
            entry = list_entry_ct_vm_block(pos);
            pre_ent = list_entry_ct_vm_block(pre);
            if (*pre_ent).addr.wrapping_add((*pre_ent).size) > (*entry).addr {
                break;
            }

            (*pre_ent).size = (*pre_ent).size.wrapping_add((*entry).size);
            list_del(pos);
            kfree(entry as *mut c_void);
            pos = pre;
            pre = (*pos).prev;
        }
    }
}

/* Map host addr (kmalloced/vmalloced) to device logical addr. */
unsafe extern "C" fn ct_vm_map(
    vm: *mut ct_vm,
    substream: *mut snd_pcm_substream,
    size: c_int,
) -> *mut ct_vm_block {
    let block: *mut ct_vm_block;
    let pte_start: c_uint;
    let mut i: c_uint;
    let pages: c_uint;
    let ptp: *mut c_ulong;
    let atc: *mut ct_atc;

    unsafe {
        atc = snd_pcm_substream_chip(substream);

        block = get_vm_block(vm, size as c_uint, atc);
        if block.is_null() {
            dev_err(
                (*(*atc).card).dev,
                c"No virtual memory block that is big enough to allocate!\n".as_ptr()
                    as *const u8,
            );
            return null_mut();
        }

        ptp = (*vm).ptp[0].area as *mut c_ulong;
        pte_start = (*block).addr >> CT_PAGE_SHIFT;
        pages = (*block).size >> CT_PAGE_SHIFT;
        i = 0;
        while i < pages {
            let addr: c_ulong;
            addr = snd_pcm_sgbuf_get_addr(substream, i << CT_PAGE_SHIFT);
            *ptp.add(pte_start.wrapping_add(i) as usize) = addr;
            i = i.wrapping_add(1);
        }

        (*block).size = size as c_uint;
        block
    }
}

unsafe extern "C" fn ct_vm_unmap(vm: *mut ct_vm, block: *mut ct_vm_block) {
    /* do unmapping */
    unsafe {
        put_vm_block(vm, block);
    }
}

/* *
 * return the host physical addr of the @index-th device
 * page table page on success, or ~0UL on failure.
 * The first returned ~0UL indicates the termination.
 * */
unsafe extern "C" fn ct_get_ptp_phys(vm: *mut ct_vm, index: c_int) -> dma_addr_t {
    unsafe {
        if index >= CT_PTP_NUM {
            !0 as c_ulong
        } else {
            (*vm).ptp[index as usize].addr
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ct_vm_create(rvm: *mut *mut ct_vm, pci: *mut pci_dev) -> c_int {
    let mut vm: *mut ct_vm;
    let mut block: *mut ct_vm_block;
    let mut i: c_int;
    let mut err: c_int = 0;

    unsafe {
        *rvm = null_mut();

        vm = kzalloc_obj::<ct_vm>();
        if vm.is_null() {
            return -ENOMEM;
        }

        mutex_init(&mut (*vm).lock);

        /* Allocate page table pages */
        i = 0;
        while i < CT_PTP_NUM {
            err = snd_dma_alloc_pages(
                SNDRV_DMA_TYPE_DEV,
                &mut (*pci).dev,
                PAGE_SIZE as usize,
                &mut (*vm).ptp[i as usize],
            );
            if err < 0 {
                break;
            }
            i += 1;
        }
        if err < 0 {
            /* no page table pages are allocated */
            ct_vm_destroy(vm);
            return -ENOMEM;
        }
        (*vm).size = (CT_ADDRS_PER_PAGE as c_uint).wrapping_mul(i as c_uint);
        (*vm).map = Some(ct_vm_map);
        (*vm).unmap = Some(ct_vm_unmap);
        (*vm).get_ptp_phys = Some(ct_get_ptp_phys);
        INIT_LIST_HEAD(&mut (*vm).unused);
        INIT_LIST_HEAD(&mut (*vm).used);
        block = kzalloc_obj::<ct_vm_block>();
        if !block.is_null() {
            (*block).addr = 0;
            (*block).size = (*vm).size;
            list_add(&mut (*block).list, &mut (*vm).unused);
        }

        *rvm = vm;
        0
    }
}

/* The caller must ensure no mapping pages are being used
 * by hardware before calling this function */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ct_vm_destroy(vm: *mut ct_vm) {
    let mut i: c_int;
    let mut pos: *mut list_head;
    let mut entry: *mut ct_vm_block;

    unsafe {
        /* free used and unused list nodes */
        while !list_empty(&mut (*vm).used) {
            pos = (*vm).used.next;
            list_del(pos);
            entry = list_entry_ct_vm_block(pos);
            kfree(entry as *mut c_void);
        }
        while !list_empty(&mut (*vm).unused) {
            pos = (*vm).unused.next;
            list_del(pos);
            entry = list_entry_ct_vm_block(pos);
            kfree(entry as *mut c_void);
        }

        /* free allocated page table pages */
        i = 0;
        while i < CT_PTP_NUM {
            snd_dma_free_pages(&mut (*vm).ptp[i as usize]);
            i += 1;
        }

        (*vm).size = 0;

        kfree(vm as *mut c_void);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
