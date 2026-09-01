// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

// C dependencies: <linux/pci.h>, "ops.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub hw_lock: spinlock_t,
    pub dsp_oops_offset: usize,
    pub dbg_dump_printed: bool,
}

extern "C" {
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn pci_read_config_dword(dev: *mut pci_dev, offset: u32, value: *mut u32) -> core::ffi::c_int;
    fn pci_write_config_dword(dev: *mut pci_dev, offset: u32, value: u32) -> core::ffi::c_int;
    fn dev_dbg(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn spinlock_irqsave_guard(lock: *mut spinlock_t) -> spinlock_irqsave_guard_t;
    fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: u32, offset: u32) -> u32;
    fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: u32, offset: u32, value: u32);
    fn snd_sof_dsp_read64(sdev: *mut snd_sof_dev, bar: u32, offset: u32) -> u64;
    fn snd_sof_dsp_write64(sdev: *mut snd_sof_dev, bar: u32, offset: u32, value: u64);
    fn snd_sof_dsp_dbg_dump(sdev: *mut snd_sof_dev, msg: *const core::ffi::c_char, flags: u32);
    fn sof_set_fw_state(sdev: *mut snd_sof_dev, state: u32);
    fn sof_fw_trace_fw_crashed(sdev: *mut snd_sof_dev);
}

#[repr(C)]
pub struct spinlock_irqsave_guard_t {
    _private: [u8; 0],
}

const SOF_DBG_DUMP_REGS: u32 = 1 << 0;
const SOF_DBG_DUMP_MBOX: u32 = 1 << 1;
const SOF_FW_CRASHED: u32 = 0;

unsafe fn snd_sof_pci_update_bits_unlocked(
    sdev: *mut snd_sof_dev,
    offset: u32,
    mask: u32,
    value: u32,
) -> bool {
    let pci: *mut pci_dev = to_pci_dev((*sdev).dev);
    let old: core::ffi::c_uint;
    let new: core::ffi::c_uint;
    let mut ret: u32 = 0;

    pci_read_config_dword(pci, offset, &mut ret);
    old = ret;
    dev_dbg(
        (*sdev).dev,
        b"Debug PCIR: %8.8x at  %8.8x\n\0".as_ptr() as *const core::ffi::c_char,
        old & mask,
        offset,
    );

    new = (old & !mask) | (value & mask);

    if old == new {
        return false;
    }

    pci_write_config_dword(pci, offset, new);
    dev_dbg(
        (*sdev).dev,
        b"Debug PCIW: %8.8x at  %8.8x\n\0".as_ptr() as *const core::ffi::c_char,
        value,
        offset,
    );

    true
}

#[no_mangle]
pub unsafe extern "C" fn snd_sof_pci_update_bits(
    sdev: *mut snd_sof_dev,
    offset: u32,
    mask: u32,
    value: u32,
) -> bool {
    let _guard = spinlock_irqsave_guard(&mut (*sdev).hw_lock);
    snd_sof_pci_update_bits_unlocked(sdev, offset, mask, value)
}

// EXPORT_SYMBOL(snd_sof_pci_update_bits);

#[no_mangle]
pub unsafe extern "C" fn snd_sof_dsp_update_bits_unlocked(
    sdev: *mut snd_sof_dev,
    bar: u32,
    offset: u32,
    mask: u32,
    value: u32,
) -> bool {
    let old: core::ffi::c_uint;
    let new: core::ffi::c_uint;
    let ret: u32;

    ret = snd_sof_dsp_read(sdev, bar, offset);

    old = ret;
    new = (old & !mask) | (value & mask);

    if old == new {
        return false;
    }

    snd_sof_dsp_write(sdev, bar, offset, new);

    true
}

// EXPORT_SYMBOL(snd_sof_dsp_update_bits_unlocked);

#[no_mangle]
pub unsafe extern "C" fn snd_sof_dsp_update_bits64_unlocked(
    sdev: *mut snd_sof_dev,
    bar: u32,
    offset: u32,
    mask: u64,
    value: u64,
) -> bool {
    let old: u64;
    let new: u64;

    old = snd_sof_dsp_read64(sdev, bar, offset);

    new = (old & !mask) | (value & mask);

    if old == new {
        return false;
    }

    snd_sof_dsp_write64(sdev, bar, offset, new);

    true
}

// EXPORT_SYMBOL(snd_sof_dsp_update_bits64_unlocked);

/* This is for registers bits with attribute RWC */
#[no_mangle]
pub unsafe extern "C" fn snd_sof_dsp_update_bits(
    sdev: *mut snd_sof_dev,
    bar: u32,
    offset: u32,
    mask: u32,
    value: u32,
) -> bool {
    let _guard = spinlock_irqsave_guard(&mut (*sdev).hw_lock);
    snd_sof_dsp_update_bits_unlocked(sdev, bar, offset, mask, value)
}

// EXPORT_SYMBOL(snd_sof_dsp_update_bits);

#[no_mangle]
pub unsafe extern "C" fn snd_sof_dsp_update_bits64(
    sdev: *mut snd_sof_dev,
    bar: u32,
    offset: u32,
    mask: u64,
    value: u64,
) -> bool {
    let _guard = spinlock_irqsave_guard(&mut (*sdev).hw_lock);
    snd_sof_dsp_update_bits64_unlocked(sdev, bar, offset, mask, value)
}

// EXPORT_SYMBOL(snd_sof_dsp_update_bits64);

unsafe fn snd_sof_dsp_update_bits_forced_unlocked(
    sdev: *mut snd_sof_dev,
    bar: u32,
    offset: u32,
    mask: u32,
    value: u32,
) {
    let old: core::ffi::c_uint;
    let new: core::ffi::c_uint;
    let ret: u32;

    ret = snd_sof_dsp_read(sdev, bar, offset);

    old = ret;
    new = (old & !mask) | (value & mask);

    snd_sof_dsp_write(sdev, bar, offset, new);
}

/* This is for registers bits with attribute RWC */
#[no_mangle]
pub unsafe extern "C" fn snd_sof_dsp_update_bits_forced(
    sdev: *mut snd_sof_dev,
    bar: u32,
    offset: u32,
    mask: u32,
    value: u32,
) {
    let _guard = spinlock_irqsave_guard(&mut (*sdev).hw_lock);
    snd_sof_dsp_update_bits_forced_unlocked(sdev, bar, offset, mask, value);
}

// EXPORT_SYMBOL(snd_sof_dsp_update_bits_forced);

/**
 * snd_sof_dsp_panic - handle a received DSP panic message
 * @sdev: Pointer to the device's sdev
 * @offset: offset of panic information
 * @non_recoverable: the panic is fatal, no recovery will be done by the caller
 */
#[no_mangle]
pub unsafe extern "C" fn snd_sof_dsp_panic(
    sdev: *mut snd_sof_dev,
    offset: u32,
    non_recoverable: bool,
) {
    /*
     * if DSP is not ready and the dsp_oops_offset is not yet set, use the
     * offset from the panic message.
     */
    if (*sdev).dsp_oops_offset == 0 {
        (*sdev).dsp_oops_offset = offset as usize;
    }

    /*
     * Print warning if the offset from the panic message differs from
     * dsp_oops_offset
     */
    if (*sdev).dsp_oops_offset != offset as usize {
        dev_warn(
            (*sdev).dev,
            b"%s: dsp_oops_offset %zu differs from panic offset %u\n\0".as_ptr()
                as *const core::ffi::c_char,
            b"snd_sof_dsp_panic\0".as_ptr() as *const core::ffi::c_char,
            (*sdev).dsp_oops_offset,
            offset,
        );
    }

    /*
     * Set the fw_state to crashed only in case of non recoverable DSP panic
     * event.
     * Use different message within the snd_sof_dsp_dbg_dump() depending on
     * the non_recoverable flag.
     */
    (*sdev).dbg_dump_printed = false;
    if non_recoverable {
        snd_sof_dsp_dbg_dump(
            sdev,
            b"DSP panic!\0".as_ptr() as *const core::ffi::c_char,
            SOF_DBG_DUMP_REGS | SOF_DBG_DUMP_MBOX,
        );
        sof_set_fw_state(sdev, SOF_FW_CRASHED);
        sof_fw_trace_fw_crashed(sdev);
    } else {
        snd_sof_dsp_dbg_dump(
            sdev,
            b"DSP panic (recovery will be attempted)\0".as_ptr() as *const core::ffi::c_char,
            SOF_DBG_DUMP_REGS | SOF_DBG_DUMP_MBOX,
        );
    }
}

// EXPORT_SYMBOL(snd_sof_dsp_panic);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
