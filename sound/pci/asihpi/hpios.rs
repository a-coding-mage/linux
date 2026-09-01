// SPDX-License-Identifier: GPL-2.0-only
/******************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2012  AudioScience Inc. <support@audioscience.com>


HPI Operating System function implementation for Linux

(C) Copyright AudioScience Inc. 1997-2003
******************************************************************************/

// SOURCEFILE_NAME was defined as "hpios.c" in the C source.
// C includes translated as external dependencies:
// hpi_internal.h, hpidebug.h, linux/delay.h, linux/sched.h

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type dma_addr_t = c_ulong;
pub type gfp_t = c_uint;

pub const DEBUG: c_int = 0;
pub const WARNING: c_int = 1;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct consistent_dma_area {
    pub vaddr: *mut c_void,
    pub dma_handle: dma_addr_t,
    pub pdev: *mut device,
    pub size: u32,
}

unsafe extern "C" {
    static GFP_KERNEL: gfp_t;

    fn usecs_to_jiffies(usec: u32) -> c_ulong;
    fn in_interrupt() -> c_int;
    fn schedule_timeout_uninterruptible(timeout: c_ulong) -> c_ulong;
    fn udelay(usecs: u32);
    fn mdelay(msecs: u32);

    fn dma_alloc_coherent(
        dev: *mut device,
        size: u32,
        dma_handle: *mut dma_addr_t,
        flag: gfp_t,
    ) -> *mut c_void;
    fn dma_free_coherent(
        dev: *mut device,
        size: u32,
        cpu_addr: *mut c_void,
        dma_handle: dma_addr_t,
    );

    fn HPI_DEBUG_LOG(level: c_int, fmt: *const c_char, ...);
}

pub unsafe extern "C" fn hpios_delay_micro_seconds(num_micro_sec: u32) {
    if unsafe { usecs_to_jiffies(num_micro_sec) > 1 } && unsafe { in_interrupt() == 0 } {
        /* MUST NOT SCHEDULE IN INTERRUPT CONTEXT! */
        unsafe {
            schedule_timeout_uninterruptible(usecs_to_jiffies(num_micro_sec));
        }
    } else if num_micro_sec <= 2000 {
        unsafe {
            udelay(num_micro_sec);
        }
    } else {
        unsafe {
            mdelay(num_micro_sec / 1000);
        }
    }
}

/** Allocate an area of locked memory for bus master DMA operations.

If allocation fails, return 1, and *pMemArea.size = 0
*/
pub unsafe extern "C" fn hpios_locked_mem_alloc(
    p_mem_area: *mut consistent_dma_area,
    size: u32,
    pdev: *mut pci_dev,
) -> u16 {
    /*?? any benefit in using managed dmam_alloc_coherent? */
    unsafe {
        (*p_mem_area).vaddr = dma_alloc_coherent(
            &mut (*pdev).dev,
            size,
            &mut (*p_mem_area).dma_handle,
            GFP_KERNEL,
        );
    }

    if unsafe { !(*p_mem_area).vaddr.is_null() } {
        unsafe {
            HPI_DEBUG_LOG(
                DEBUG,
                c"allocated %d bytes, dma 0x%x vma %p\n".as_ptr(),
                size,
                (*p_mem_area).dma_handle as c_uint,
                (*p_mem_area).vaddr,
            );
            (*p_mem_area).pdev = &mut (*pdev).dev;
            (*p_mem_area).size = size;
        }
        0
    } else {
        unsafe {
            HPI_DEBUG_LOG(
                WARNING,
                c"failed to allocate %d bytes locked memory\n".as_ptr(),
                size,
            );
            (*p_mem_area).size = 0;
        }
        1
    }
}

pub unsafe extern "C" fn hpios_locked_mem_free(p_mem_area: *mut consistent_dma_area) -> u16 {
    if unsafe { (*p_mem_area).size != 0 } {
        unsafe {
            dma_free_coherent(
                (*p_mem_area).pdev,
                (*p_mem_area).size,
                (*p_mem_area).vaddr,
                (*p_mem_area).dma_handle,
            );
            HPI_DEBUG_LOG(
                DEBUG,
                c"freed %lu bytes, dma 0x%x vma %p\n".as_ptr(),
                (*p_mem_area).size as c_ulong,
                (*p_mem_area).dma_handle as c_uint,
                (*p_mem_area).vaddr,
            );
            (*p_mem_area).size = 0;
        }
        0
    } else {
        1
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
