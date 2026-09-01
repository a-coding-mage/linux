/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File    ctvmem.h
 *
 * @Brief
 * This file contains the definition of virtual memory management object
 * for card device.
 *
 * @Author Liu Chun
 * @Date Mar 28 2008
 */

/* Dependencies from the original header:
 * linux/mutex.h, linux/list.h, linux/pci.h, sound/memalloc.h
 */

pub const CT_PTP_NUM: usize = 1; /* num of device page table pages */

/* The chip can handle the page table of 4k pages
 * (emu20k1 can handle even 8k pages, but we don't use it right now)
 */
pub const CT_PAGE_SIZE: usize = 4096;
pub const CT_PAGE_SHIFT: usize = 12;
pub const CT_PAGE_MASK: usize = !(PAGE_SIZE - 1);

#[inline]
pub const fn CT_PAGE_ALIGN(addr: usize) -> usize {
    ALIGN(addr, CT_PAGE_SIZE)
}

#[repr(C)]
pub struct ct_vm_block {
    pub addr: ::core::ffi::c_uint, /* starting logical addr of this block */
    pub size: ::core::ffi::c_uint, /* size of this device virtual mem block */
    pub list: list_head,
}

pub enum snd_pcm_substream {}

/* Virtual memory management object for card device */
#[repr(C)]
pub struct ct_vm {
    pub ptp: [snd_dma_buffer; CT_PTP_NUM], /* Device page table pages */
    pub size: ::core::ffi::c_uint,         /* Available addr space in bytes */
    pub unused: list_head,                 /* List of unused blocks */
    pub used: list_head,                   /* List of used blocks */
    pub lock: mutex,

    /* Map host addr (kmalloced/vmalloced) to device logical addr. */
    pub map: Option<
        unsafe extern "C" fn(
            *mut ct_vm,
            *mut snd_pcm_substream,
            ::core::ffi::c_int,
        ) -> *mut ct_vm_block,
    >,
    /* Unmap device logical addr area. */
    pub unmap: Option<unsafe extern "C" fn(*mut ct_vm, *mut ct_vm_block)>,
    pub get_ptp_phys: Option<unsafe extern "C" fn(*mut ct_vm, ::core::ffi::c_int) -> dma_addr_t>,
}

extern "C" {
    pub fn ct_vm_create(rvm: *mut *mut ct_vm, pci: *mut pci_dev) -> ::core::ffi::c_int;
    pub fn ct_vm_destroy(vm: *mut ct_vm);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
