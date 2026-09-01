// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018-2021 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

/*
 * Hardware interface for audio DSP on Atom devices
 *
 * C header dependencies removed from executable Rust:
 * linux/module.h, sound/sof.h, sound/sof/xtensa.h, sound/soc-acpi.h,
 * sound/soc-acpi-intel-match.h, sound/intel-dsp-config.h, ../ops.h,
 * shim.h, atom.h, ../sof-acpi-dev.h, ../sof-audio.h,
 * ../../intel/common/soc-intel-quirks.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type u64 = u64;
type size_t = usize;
type irqreturn_t = c_uint;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQ_WAKE_THREAD: irqreturn_t = 2;

const GFP_KERNEL: c_uint = 0;
const KERN_ERR: *const c_char = b"\0".as_ptr() as *const c_char;
const ENODEV: c_int = 19;

// Constants supplied by the Atom/SOF headers in the original C translation unit.
extern "C" {
    static DSP_BAR: c_int;
    static SHIM_IPCD: u32;
    static SHIM_IPCX: u32;
    static SHIM_IMRX: u32;
    static SHIM_IMRD: u32;
    static SHIM_CSR: u32;
    static SHIM_IPCX_BUSY: u64;
    static SHIM_IPCX_DONE: u64;
    static SHIM_IPCD_BUSY: u64;
    static SHIM_IPCD_DONE: u64;
    static SHIM_IMRX_BUSY: u64;
    static SHIM_IMRX_DONE: u64;
    static SHIM_IMRD_BUSY: u64;
    static SHIM_IMRD_DONE: u64;
    static SHIM_BYT_IPCX_BUSY: u64;
    static SHIM_BYT_IPCX_DONE: u64;
    static SHIM_BYT_IPCD_BUSY: u64;
    static SHIM_BYT_IPCD_DONE: u64;
    static SHIM_BYT_CSR_STALL: u64;
    static SHIM_BYT_CSR_PWAITMODE: u64;
    static SHIM_BYT_CSR_RST: u64;
    static SHIM_BYT_CSR_VECTOR_SEL: u64;
    static EXCEPT_MAX_HDR_SIZE: u32;
    static STACK_DUMP_SIZE: usize;
    static SOF_IPC_PANIC_MAGIC_MASK: u64;
    static SOF_IPC_PANIC_MAGIC: u64;
    static MBOX_OFFSET: c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_ipc_msg {
    pub msg_data: *mut c_void,
    pub msg_size: size_t,
}

#[repr(C)]
pub struct snd_sof_mailbox {
    pub offset: u32,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub dsp_oops_offset: u32,
    pub host_box: snd_sof_mailbox,
    pub ipc_lock: spinlock_t,
    pub pdata: *mut snd_sof_pdata,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_irq_guard {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc_arch_hdr {
    pub totalsize: u32,
}

#[repr(C)]
pub struct sof_ipc_dsp_oops_xtensa {
    pub arch_hdr: sof_ipc_arch_hdr,
}

#[repr(C)]
pub struct sof_ipc_panic_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub desc: *const sof_dev_desc,
    pub tplg_filename: *const c_char,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub machines: *mut snd_soc_acpi_mach,
    pub irqindex_host_ipc: c_int,
    pub ops: *const snd_sof_dsp_ops,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub num_drv: c_int,
    pub drv: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub sof_tplg_filename: *const c_char,
    pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub acpi_ipc_irq_index: c_int,
    pub platform: *const c_char,
    pub num_dai_drivers: c_int,
    pub dai_drivers: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_dai_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_dai_stream,
    pub capture: snd_soc_dai_stream,
}

extern "C" {
    fn sof_mailbox_read(
        sdev: *mut snd_sof_dev,
        offset: u32,
        dest: *mut c_void,
        bytes: size_t,
    );
    fn sof_mailbox_write(
        sdev: *mut snd_sof_dev,
        offset: u32,
        src: *const c_void,
        bytes: size_t,
    );
    fn snd_sof_dsp_read64(sdev: *mut snd_sof_dev, bar: c_int, offset: u32) -> u64;
    fn snd_sof_dsp_write64(sdev: *mut snd_sof_dev, bar: c_int, offset: u32, value: u64);
    fn snd_sof_dsp_update_bits64_unlocked(
        sdev: *mut snd_sof_dev,
        bar: c_int,
        offset: u32,
        mask: u64,
        value: u64,
    );
    fn snd_sof_dsp_update_bits64(
        sdev: *mut snd_sof_dev,
        bar: c_int,
        offset: u32,
        mask: u64,
        value: u64,
    );
    fn sof_print_oops_and_stack(
        sdev: *mut snd_sof_dev,
        level: *const c_char,
        status: u64,
        panic: u64,
        xoops: *mut sof_ipc_dsp_oops_xtensa,
        panic_info: *mut sof_ipc_panic_info,
        stack: *mut u32,
        stack_words: size_t,
    );
    fn str_yes_no(value: u64) -> *const c_char;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn snd_sof_ipc_process_reply(sdev: *mut snd_sof_dev, msg: u64);
    fn snd_sof_ipc_msgs_rx(sdev: *mut snd_sof_dev);
    fn snd_sof_dsp_panic(sdev: *mut snd_sof_dev, offset: u32, non_recoverable: bool);
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_uint, max: c_uint);
    fn kstrdup(s: *const c_char, flags: c_uint) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *const c_char;
    fn snd_soc_acpi_find_machine(machines: *mut snd_soc_acpi_mach) -> *mut snd_soc_acpi_mach;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn soc_intel_is_byt_cr(pdev: *mut platform_device) -> bool;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn guard_spinlock_irq(lock: *mut spinlock_t) -> spinlock_irq_guard;
}

unsafe fn PANIC_OFFSET(ipcd: u64) -> u32 {
    ipcd as u32
}

/*
 * Debug
 */

unsafe fn atom_get_registers(
    sdev: *mut snd_sof_dev,
    xoops: *mut sof_ipc_dsp_oops_xtensa,
    panic_info: *mut sof_ipc_panic_info,
    stack: *mut u32,
    stack_words: size_t,
) {
    let mut offset: u32 = (*sdev).dsp_oops_offset;

    /* first read regsisters */
    sof_mailbox_read(
        sdev,
        offset,
        xoops as *mut c_void,
        size_of::<sof_ipc_dsp_oops_xtensa>(),
    );

    /* note: variable AR register array is not read */

    /* then get panic info */
    if (*xoops).arch_hdr.totalsize > EXCEPT_MAX_HDR_SIZE {
        dev_err(
            (*sdev).dev,
            b"invalid header size 0x%x. FW oops is bogus\n\0".as_ptr() as *const c_char,
            (*xoops).arch_hdr.totalsize,
        );
        return;
    }
    offset = offset.wrapping_add((*xoops).arch_hdr.totalsize);
    sof_mailbox_read(
        sdev,
        offset,
        panic_info as *mut c_void,
        size_of::<sof_ipc_panic_info>(),
    );

    /* then get the stack */
    offset = offset.wrapping_add(size_of::<sof_ipc_panic_info>() as u32);
    sof_mailbox_read(
        sdev,
        offset,
        stack as *mut c_void,
        stack_words.wrapping_mul(size_of::<u32>()),
    );
}

#[no_mangle]
pub unsafe extern "C" fn atom_dump(sdev: *mut snd_sof_dev, flags: u32) {
    let mut xoops: sof_ipc_dsp_oops_xtensa = core::mem::zeroed();
    let mut panic_info: sof_ipc_panic_info = core::mem::zeroed();
    let mut stack = [0u32; 32];
    let status: u64;
    let panic: u64;
    let imrd: u64;
    let imrx: u64;
    let stack_dump_size = STACK_DUMP_SIZE;

    /* now try generic SOF status messages */
    status = snd_sof_dsp_read64(sdev, DSP_BAR, SHIM_IPCD);
    panic = snd_sof_dsp_read64(sdev, DSP_BAR, SHIM_IPCX);
    atom_get_registers(
        sdev,
        &mut xoops,
        &mut panic_info,
        stack.as_mut_ptr(),
        stack_dump_size,
    );
    sof_print_oops_and_stack(
        sdev,
        KERN_ERR,
        status,
        panic,
        &mut xoops,
        &mut panic_info,
        stack.as_mut_ptr(),
        stack_dump_size,
    );

    /* provide some context for firmware debug */
    imrx = snd_sof_dsp_read64(sdev, DSP_BAR, SHIM_IMRX);
    imrd = snd_sof_dsp_read64(sdev, DSP_BAR, SHIM_IMRD);
    dev_err(
        (*sdev).dev,
        b"error: ipc host -> DSP: pending %s complete %s raw 0x%llx\n\0".as_ptr()
            as *const c_char,
        str_yes_no(panic & SHIM_IPCX_BUSY),
        str_yes_no(panic & SHIM_IPCX_DONE),
        panic,
    );
    dev_err(
        (*sdev).dev,
        b"error: mask host: pending %s complete %s raw 0x%llx\n\0".as_ptr() as *const c_char,
        str_yes_no(imrx & SHIM_IMRX_BUSY),
        str_yes_no(imrx & SHIM_IMRX_DONE),
        imrx,
    );
    dev_err(
        (*sdev).dev,
        b"error: ipc DSP -> host: pending %s complete %s raw 0x%llx\n\0".as_ptr()
            as *const c_char,
        str_yes_no(status & SHIM_IPCD_BUSY),
        str_yes_no(status & SHIM_IPCD_DONE),
        status,
    );
    dev_err(
        (*sdev).dev,
        b"error: mask DSP: pending %s complete %s raw 0x%llx\n\0".as_ptr() as *const c_char,
        str_yes_no(imrd & SHIM_IMRD_BUSY),
        str_yes_no(imrd & SHIM_IMRD_DONE),
        imrd,
    );
}
/* EXPORT_SYMBOL_NS(atom_dump, "SND_SOC_SOF_INTEL_ATOM_HIFI_EP"); */

/*
 * IPC Doorbell IRQ handler and thread.
 */

#[no_mangle]
pub unsafe extern "C" fn atom_irq_handler(irq: c_int, context: *mut c_void) -> irqreturn_t {
    let sdev: *mut snd_sof_dev = context as *mut snd_sof_dev;
    let ipcx: u64;
    let ipcd: u64;
    let mut ret: c_uint = IRQ_NONE;

    ipcx = snd_sof_dsp_read64(sdev, DSP_BAR, SHIM_IPCX);
    ipcd = snd_sof_dsp_read64(sdev, DSP_BAR, SHIM_IPCD);

    if ipcx & SHIM_BYT_IPCX_DONE != 0 {
        /* reply message from DSP, Mask Done interrupt first */
        snd_sof_dsp_update_bits64_unlocked(
            sdev,
            DSP_BAR,
            SHIM_IMRX,
            SHIM_IMRX_DONE,
            SHIM_IMRX_DONE,
        );
        ret = IRQ_WAKE_THREAD;
    }

    if ipcd & SHIM_BYT_IPCD_BUSY != 0 {
        /* new message from DSP, Mask Busy interrupt first */
        snd_sof_dsp_update_bits64_unlocked(
            sdev,
            DSP_BAR,
            SHIM_IMRX,
            SHIM_IMRX_BUSY,
            SHIM_IMRX_BUSY,
        );
        ret = IRQ_WAKE_THREAD;
    }

    ret
}
/* EXPORT_SYMBOL_NS(atom_irq_handler, "SND_SOC_SOF_INTEL_ATOM_HIFI_EP"); */

#[no_mangle]
pub unsafe extern "C" fn atom_irq_thread(irq: c_int, context: *mut c_void) -> irqreturn_t {
    let sdev: *mut snd_sof_dev = context as *mut snd_sof_dev;
    let ipcx: u64;
    let ipcd: u64;

    ipcx = snd_sof_dsp_read64(sdev, DSP_BAR, SHIM_IPCX);
    ipcd = snd_sof_dsp_read64(sdev, DSP_BAR, SHIM_IPCD);

    /* reply message from DSP */
    if ipcx & SHIM_BYT_IPCX_DONE != 0 {
        /*
         * handle immediate reply from DSP core. If the msg is
         * found, set done bit in cmd_done which is called at the
         * end of message processing function, else set it here
         * because the done bit can't be set in cmd_done function
         * which is triggered by msg
         */
        let _guard = guard_spinlock_irq(&mut (*sdev).ipc_lock);
        snd_sof_ipc_process_reply(sdev, ipcx);
        atom_dsp_done(sdev);
    }

    /* new message from DSP */
    if ipcd & SHIM_BYT_IPCD_BUSY != 0 {
        /* Handle messages from DSP Core */
        if ipcd & SOF_IPC_PANIC_MAGIC_MASK == SOF_IPC_PANIC_MAGIC {
            snd_sof_dsp_panic(sdev, PANIC_OFFSET(ipcd).wrapping_add(MBOX_OFFSET as u32), true);
        } else {
            snd_sof_ipc_msgs_rx(sdev);
        }

        atom_host_done(sdev);
    }

    IRQ_HANDLED
}
/* EXPORT_SYMBOL_NS(atom_irq_thread, "SND_SOC_SOF_INTEL_ATOM_HIFI_EP"); */

#[no_mangle]
pub unsafe extern "C" fn atom_send_msg(
    sdev: *mut snd_sof_dev,
    msg: *mut snd_sof_ipc_msg,
) -> c_int {
    /* unmask and prepare to receive Done interrupt */
    snd_sof_dsp_update_bits64_unlocked(sdev, DSP_BAR, SHIM_IMRX, SHIM_IMRX_DONE, 0);

    /* send the message */
    sof_mailbox_write(
        sdev,
        (*sdev).host_box.offset,
        (*msg).msg_data as *const c_void,
        (*msg).msg_size,
    );
    snd_sof_dsp_write64(sdev, DSP_BAR, SHIM_IPCX, SHIM_BYT_IPCX_BUSY);

    0
}
/* EXPORT_SYMBOL_NS(atom_send_msg, "SND_SOC_SOF_INTEL_ATOM_HIFI_EP"); */

#[no_mangle]
pub unsafe extern "C" fn atom_get_mailbox_offset(sdev: *mut snd_sof_dev) -> c_int {
    MBOX_OFFSET
}
/* EXPORT_SYMBOL_NS(atom_get_mailbox_offset, "SND_SOC_SOF_INTEL_ATOM_HIFI_EP"); */

#[no_mangle]
pub unsafe extern "C" fn atom_get_window_offset(sdev: *mut snd_sof_dev, id: u32) -> c_int {
    MBOX_OFFSET
}
/* EXPORT_SYMBOL_NS(atom_get_window_offset, "SND_SOC_SOF_INTEL_ATOM_HIFI_EP"); */

unsafe fn atom_host_done(sdev: *mut snd_sof_dev) {
    /* clear BUSY bit and set DONE bit - accept new messages */
    snd_sof_dsp_update_bits64_unlocked(
        sdev,
        DSP_BAR,
        SHIM_IPCD,
        SHIM_BYT_IPCD_BUSY | SHIM_BYT_IPCD_DONE,
        SHIM_BYT_IPCD_DONE,
    );

    /* unmask and prepare to receive next new message */
    snd_sof_dsp_update_bits64_unlocked(sdev, DSP_BAR, SHIM_IMRX, SHIM_IMRX_BUSY, 0);
}

unsafe fn atom_dsp_done(sdev: *mut snd_sof_dev) {
    /* clear DONE bit - tell DSP we have completed */
    snd_sof_dsp_update_bits64_unlocked(sdev, DSP_BAR, SHIM_IPCX, SHIM_BYT_IPCX_DONE, 0);
}

/*
 * DSP control.
 */

#[no_mangle]
pub unsafe extern "C" fn atom_run(sdev: *mut snd_sof_dev) -> c_int {
    let mut tries: c_int = 10;

    /* release stall and wait to unstall */
    snd_sof_dsp_update_bits64(sdev, DSP_BAR, SHIM_CSR, SHIM_BYT_CSR_STALL, 0x0);
    while {
        let old = tries;
        tries -= 1;
        old != 0
    } {
        if snd_sof_dsp_read64(sdev, DSP_BAR, SHIM_CSR) & SHIM_BYT_CSR_PWAITMODE == 0 {
            break;
        }
        msleep(100);
    }
    if tries < 0 {
        return -ENODEV;
    }

    /* return init core mask */
    1
}
/* EXPORT_SYMBOL_NS(atom_run, "SND_SOC_SOF_INTEL_ATOM_HIFI_EP"); */

#[no_mangle]
pub unsafe extern "C" fn atom_reset(sdev: *mut snd_sof_dev) -> c_int {
    /* put DSP into reset, set reset vector and stall */
    snd_sof_dsp_update_bits64(
        sdev,
        DSP_BAR,
        SHIM_CSR,
        SHIM_BYT_CSR_RST | SHIM_BYT_CSR_VECTOR_SEL | SHIM_BYT_CSR_STALL,
        SHIM_BYT_CSR_RST | SHIM_BYT_CSR_VECTOR_SEL | SHIM_BYT_CSR_STALL,
    );

    usleep_range(10, 15);

    /* take DSP out of reset and keep stalled for FW loading */
    snd_sof_dsp_update_bits64(sdev, DSP_BAR, SHIM_CSR, SHIM_BYT_CSR_RST, 0);

    0
}
/* EXPORT_SYMBOL_NS(atom_reset, "SND_SOC_SOF_INTEL_ATOM_HIFI_EP"); */

unsafe fn fixup_tplg_name(
    sdev: *mut snd_sof_dev,
    sof_tplg_filename: *const c_char,
    ssp_str: *const c_char,
) -> *const c_char {
    let mut tplg_filename: *const c_char = ptr::null();
    let split_ext: *const c_char;
    let filename: *mut c_char;
    let mut tmp: *mut c_char;

    filename = kstrdup(sof_tplg_filename, GFP_KERNEL);
    if filename.is_null() {
        return ptr::null();
    }

    /* this assumes a .tplg extension */
    tmp = filename;
    split_ext = strsep(&mut tmp, b".\0".as_ptr() as *const c_char);
    if !split_ext.is_null() {
        tplg_filename = devm_kasprintf(
            (*sdev).dev,
            GFP_KERNEL,
            b"%s-%s.tplg\0".as_ptr() as *const c_char,
            split_ext,
            ssp_str,
        );
    }
    kfree(filename as *mut c_void);

    tplg_filename
}

#[no_mangle]
pub unsafe extern "C" fn atom_machine_select(
    sdev: *mut snd_sof_dev,
) -> *mut snd_soc_acpi_mach {
    let sof_pdata: *mut snd_sof_pdata = (*sdev).pdata;
    let desc: *const sof_dev_desc = (*sof_pdata).desc;
    let mach: *mut snd_soc_acpi_mach;
    let pdev: *mut platform_device;
    let tplg_filename: *const c_char;

    mach = snd_soc_acpi_find_machine((*desc).machines);
    if mach.is_null() {
        dev_warn(
            (*sdev).dev,
            b"warning: No matching ASoC machine driver found\n\0".as_ptr() as *const c_char,
        );
        return ptr::null_mut();
    }

    pdev = to_platform_device((*sdev).dev);
    if soc_intel_is_byt_cr(pdev) {
        dev_dbg(
            (*sdev).dev,
            b"BYT-CR detected, SSP0 used instead of SSP2\n\0".as_ptr() as *const c_char,
        );

        tplg_filename = fixup_tplg_name(
            sdev,
            (*mach).sof_tplg_filename,
            b"ssp0\0".as_ptr() as *const c_char,
        );
    } else {
        tplg_filename = (*mach).sof_tplg_filename;
    }

    if tplg_filename.is_null() {
        dev_dbg(
            (*sdev).dev,
            b"error: no topology filename\n\0".as_ptr() as *const c_char,
        );
        return ptr::null_mut();
    }

    (*sof_pdata).tplg_filename = tplg_filename;
    (*mach).mach_params.acpi_ipc_irq_index = (*desc).irqindex_host_ipc;

    mach
}
/* EXPORT_SYMBOL_NS(atom_machine_select, "SND_SOC_SOF_INTEL_ATOM_HIFI_EP"); */

/* Atom DAIs */
#[no_mangle]
pub static mut atom_dai: [snd_soc_dai_driver; 6] = [
    snd_soc_dai_driver {
        name: b"ssp0-port\0".as_ptr() as *const c_char,
        playback: snd_soc_dai_stream {
            channels_min: 1,
            channels_max: 8,
        },
        capture: snd_soc_dai_stream {
            channels_min: 1,
            channels_max: 8,
        },
    },
    snd_soc_dai_driver {
        name: b"ssp1-port\0".as_ptr() as *const c_char,
        playback: snd_soc_dai_stream {
            channels_min: 1,
            channels_max: 8,
        },
        capture: snd_soc_dai_stream {
            channels_min: 1,
            channels_max: 8,
        },
    },
    snd_soc_dai_driver {
        name: b"ssp2-port\0".as_ptr() as *const c_char,
        playback: snd_soc_dai_stream {
            channels_min: 1,
            channels_max: 8,
        },
        capture: snd_soc_dai_stream {
            channels_min: 1,
            channels_max: 8,
        },
    },
    snd_soc_dai_driver {
        name: b"ssp3-port\0".as_ptr() as *const c_char,
        playback: snd_soc_dai_stream {
            channels_min: 1,
            channels_max: 8,
        },
        capture: snd_soc_dai_stream {
            channels_min: 1,
            channels_max: 8,
        },
    },
    snd_soc_dai_driver {
        name: b"ssp4-port\0".as_ptr() as *const c_char,
        playback: snd_soc_dai_stream {
            channels_min: 1,
            channels_max: 8,
        },
        capture: snd_soc_dai_stream {
            channels_min: 1,
            channels_max: 8,
        },
    },
    snd_soc_dai_driver {
        name: b"ssp5-port\0".as_ptr() as *const c_char,
        playback: snd_soc_dai_stream {
            channels_min: 1,
            channels_max: 8,
        },
        capture: snd_soc_dai_stream {
            channels_min: 1,
            channels_max: 8,
        },
    },
];
/* EXPORT_SYMBOL_NS(atom_dai, "SND_SOC_SOF_INTEL_ATOM_HIFI_EP"); */

#[no_mangle]
pub unsafe extern "C" fn atom_set_mach_params(
    mach: *mut snd_soc_acpi_mach,
    sdev: *mut snd_sof_dev,
) {
    let pdata: *mut snd_sof_pdata = (*sdev).pdata;
    let desc: *const sof_dev_desc = (*pdata).desc;
    let mach_params: *mut snd_soc_acpi_mach_params;

    mach_params = &mut (*mach).mach_params;
    (*mach_params).platform = dev_name((*sdev).dev);
    (*mach_params).num_dai_drivers = (*(*desc).ops).num_drv;
    (*mach_params).dai_drivers = (*(*desc).ops).drv;
}
/* EXPORT_SYMBOL_NS(atom_set_mach_params, "SND_SOC_SOF_INTEL_ATOM_HIFI_EP"); */

/* MODULE_LICENSE("Dual BSD/GPL"); */
/* MODULE_DESCRIPTION("SOF support for Atom platforms"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
