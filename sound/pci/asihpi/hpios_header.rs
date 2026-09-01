// SPDX-License-Identifier: GPL-2.0-only
/******************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2011  AudioScience Inc. <support@audioscience.com>


HPI Operating System Specific macros for Linux Kernel driver

(C) Copyright AudioScience Inc. 1997-2003
******************************************************************************/

// C header guard and Linux include directives omitted in Rust translation.
// Original header defined:
// HPI_OS_LINUX_KERNEL, HPI_OS_DEFINED, HPI_BUILD_KERNEL_MODE, HPI_NO_OS_FILE_OPS,
// and HPI_LOCKING.

use core::ffi::{c_char, c_int, c_void};

pub const HPI_OS_LINUX_KERNEL: bool = true;
pub const HPI_OS_DEFINED: bool = true;
pub const HPI_BUILD_KERNEL_MODE: bool = true;
pub const HPI_NO_OS_FILE_OPS: bool = true;
pub const HPI_LOCKING: bool = true;

// Types and constants supplied by Linux kernel headers in the original C file.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hpi_adapter_obj {
    _private: [u8; 0],
}

pub type dma_addr_t = usize;
pub type size_t = usize;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;

unsafe extern "C" {
    pub static KERN_ERR: *const c_char;
    pub static KERN_WARNING: *const c_char;
    pub static KERN_NOTICE: *const c_char;
    pub static KERN_INFO: *const c_char;
    pub static KERN_DEBUG: *const c_char;

    pub fn irqs_disabled() -> c_int;
    pub fn spin_lock(lock: *mut spinlock_t);
    pub fn spin_lock_bh(lock: *mut spinlock_t);
    pub fn spin_unlock(lock: *mut spinlock_t);
    pub fn spin_unlock_bh(lock: *mut spinlock_t);
    pub fn spin_lock_init(lock: *mut spinlock_t);
}

/** Details of a memory area allocated with  pci_alloc_consistent
Need all info for parameters to pci_free_consistent
*/
#[repr(C)]
pub struct consistent_dma_area {
    pub pdev: *mut device,
    /* looks like dma-mapping dma_devres ?! */
    pub size: size_t,
    pub vaddr: *mut c_void,
    pub dma_handle: dma_addr_t,
}

#[inline]
pub unsafe fn hpios_locked_mem_get_phys_addr(
    locked_mem_handle: *mut consistent_dma_area,
    p_physical_addr: *mut u32,
) -> u16 {
    unsafe {
        *p_physical_addr = (*locked_mem_handle).dma_handle as u32;
    }
    0
}

#[inline]
pub unsafe fn hpios_locked_mem_get_virt_addr(
    locked_mem_handle: *mut consistent_dma_area,
    pp_virtual_addr: *mut *mut c_void,
) -> u16 {
    unsafe {
        *pp_virtual_addr = (*locked_mem_handle).vaddr;
    }
    0
}

#[inline]
pub unsafe fn hpios_locked_mem_valid(locked_mem_handle: *mut consistent_dma_area) -> u16 {
    unsafe { ((*locked_mem_handle).size != 0) as u16 }
}

#[repr(C)]
pub struct hpi_ioctl_linux {
    pub phm: *mut c_void,
    pub phr: *mut c_void,
}

/* Conflict?: H is already used by a number of drivers hid, bluetooth hci,
   and some sound drivers sb16, hdsp, emu10k. AFAIK 0xFC is unused command
*/
// Original C macro:
// #define HPI_IOCTL_LINUX _IOWR('H', 0xFC, struct hpi_ioctl_linux)
// The _IOWR encoding is supplied by <linux/ioctl.h>.

pub const HPI_DEBUG_FLAG_ERROR: *const c_char = unsafe { KERN_ERR };
pub const HPI_DEBUG_FLAG_WARNING: *const c_char = unsafe { KERN_WARNING };
pub const HPI_DEBUG_FLAG_NOTICE: *const c_char = unsafe { KERN_NOTICE };
pub const HPI_DEBUG_FLAG_INFO: *const c_char = unsafe { KERN_INFO };
pub const HPI_DEBUG_FLAG_DEBUG: *const c_char = unsafe { KERN_DEBUG };
pub const HPI_DEBUG_FLAG_VERBOSE: *const c_char = unsafe { KERN_DEBUG }; /* kernel has no verbose */

#[repr(C)]
pub struct hpios_spinlock {
    pub lock: spinlock_t, /* SEE hpios_spinlock */
    pub lock_context: c_int,
}

/* The reason for all this evilness is that ALSA calls some of a drivers
 * operators in atomic context, and some not.  But all our functions channel
 * through the HPI_Message conduit, so we can't handle the different context
 * per function
 */
pub const IN_LOCK_BH: c_int = 1;
pub const IN_LOCK_IRQ: c_int = 0;

#[inline]
pub unsafe fn cond_lock(l: *mut hpios_spinlock) {
    unsafe {
        if irqs_disabled() != 0 {
            /* NO bh or isr can execute on this processor,
               so ordinary lock will do
             */
            spin_lock(&mut (*l).lock);
            (*l).lock_context = IN_LOCK_IRQ;
        } else {
            spin_lock_bh(&mut (*l).lock);
            (*l).lock_context = IN_LOCK_BH;
        }
    }
}

#[inline]
pub unsafe fn cond_unlock(l: *mut hpios_spinlock) {
    unsafe {
        if (*l).lock_context == IN_LOCK_BH {
            spin_unlock_bh(&mut (*l).lock);
        } else {
            spin_unlock(&mut (*l).lock);
        }
    }
}

#[inline]
pub unsafe fn hpios_msgxlock_init(obj: *mut hpios_spinlock) {
    unsafe {
        spin_lock_init(&mut (*obj).lock);
    }
}

#[inline]
pub unsafe fn hpios_msgxlock_lock(obj: *mut hpios_spinlock) {
    unsafe {
        cond_lock(obj);
    }
}

#[inline]
pub unsafe fn hpios_msgxlock_unlock(obj: *mut hpios_spinlock) {
    unsafe {
        cond_unlock(obj);
    }
}

#[inline]
pub unsafe fn hpios_dsplock_init<T>(obj: *mut T)
where
    T: HasDspLock,
{
    unsafe {
        spin_lock_init(&mut (*(*obj).dsp_lock()).lock);
    }
}

#[inline]
pub unsafe fn hpios_dsplock_lock<T>(obj: *mut T)
where
    T: HasDspLock,
{
    unsafe {
        cond_lock((*obj).dsp_lock());
    }
}

#[inline]
pub unsafe fn hpios_dsplock_unlock<T>(obj: *mut T)
where
    T: HasDspLock,
{
    unsafe {
        cond_unlock((*obj).dsp_lock());
    }
}

pub trait HasDspLock {
    unsafe fn dsp_lock(&mut self) -> *mut hpios_spinlock;
}

// Original conditional:
// #ifdef CONFIG_SND_DEBUG
// #define HPI_BUILD_DEBUG
// #endif

pub const HPI_ALIST_LOCKING: bool = true;

pub trait HasListLock {
    unsafe fn list_lock(&mut self) -> *mut hpios_spinlock;
}

#[inline]
pub unsafe fn hpios_alistlock_init<T>(obj: *mut T)
where
    T: HasListLock,
{
    unsafe {
        spin_lock_init(&mut (*(*obj).list_lock()).lock);
    }
}

#[inline]
pub unsafe fn hpios_alistlock_lock<T>(obj: *mut T)
where
    T: HasListLock,
{
    unsafe {
        spin_lock(&mut (*(*obj).list_lock()).lock);
    }
}

#[inline]
pub unsafe fn hpios_alistlock_unlock<T>(obj: *mut T)
where
    T: HasListLock,
{
    unsafe {
        spin_unlock(&mut (*(*obj).list_lock()).lock);
    }
}

/** pci drvdata points to an instance of this struct */
#[repr(C)]
pub struct hpi_adapter {
    pub adapter: *mut hpi_adapter_obj,
    pub snd_card: *mut snd_card,

    pub irq: c_int,
    pub interrupt_mode: c_int,
    pub interrupt_callback: Option<unsafe extern "C" fn(*mut hpi_adapter)>,

    /* mutex prevents contention for one card
       between multiple user programs (via ioctl) */
    pub mutex: mutex,
    pub p_buffer: *mut c_char,
    pub buffer_size: size_t,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
