/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 *
 * Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
 */

/* Rust translation of soc/sof/ops.h. */
/* Original includes: linux/device.h, linux/interrupt.h, linux/kernel.h,
 * linux/types.h, sound/pcm.h, sof-priv.h.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type size_t = usize;
pub type snd_pcm_uframes_t = c_ulong;

pub const EINVAL: c_int = 22;
pub const EOPNOTSUPP: c_int = 95;
pub const ETIMEDOUT: c_int = 110;

/* Types supplied by included headers. */
#[repr(C)]
pub struct snd_sof_dev {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_sof_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sof_ext_man_elem_header {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sof_dsp_power_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_sof_ipc_msg {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_sof_platform_stream_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_sof_pcm_stream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_acpi_mach {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_sof_fw_blk_type {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sof_debugfs_access_type {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ktime_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn sof_ops(sdev: *mut snd_sof_dev) -> *mut snd_sof_ops;

    fn snd_sof_dev_pdata_desc_ops_init(sdev: *mut snd_sof_dev) -> Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>;
    fn snd_sof_dev_pdata_desc_ops_free(sdev: *mut snd_sof_dev) -> Option<unsafe extern "C" fn(*mut snd_sof_dev)>;

    fn snd_sof_ops_probe_early(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>;
    fn snd_sof_ops_probe(ops: *mut snd_sof_ops) -> unsafe extern "C" fn(*mut snd_sof_dev) -> c_int;
    fn snd_sof_ops_remove(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev)>;
    fn snd_sof_ops_remove_late(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev)>;
    fn snd_sof_ops_shutdown(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>;
    fn snd_sof_ops_run(ops: *mut snd_sof_ops) -> unsafe extern "C" fn(*mut snd_sof_dev) -> c_int;
    fn snd_sof_ops_stall(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, c_uint) -> c_int>;
    fn snd_sof_ops_reset(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>;
    fn snd_sof_ops_core_get(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, c_int) -> c_int>;
    fn snd_sof_ops_core_put(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, c_int) -> c_int>;
    fn snd_sof_ops_pre_fw_run(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>;
    fn snd_sof_ops_post_fw_run(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>;
    fn snd_sof_ops_parse_platform_ext_manifest(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *const sof_ext_man_elem_header) -> c_int>;
    fn snd_sof_ops_get_bar_index(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>;
    fn snd_sof_ops_get_mailbox_offset(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>;
    fn snd_sof_ops_get_window_offset(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>;
    fn snd_sof_ops_resume(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>;
    fn snd_sof_ops_suspend(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>;
    fn snd_sof_ops_runtime_resume(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>;
    fn snd_sof_ops_runtime_suspend(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>;
    fn snd_sof_ops_runtime_idle(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>;
    fn snd_sof_ops_set_hw_params_upon_resume(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>;
    fn snd_sof_ops_set_clk(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>;
    fn snd_sof_ops_set_power_state(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *const sof_dsp_power_state) -> c_int>;
    fn snd_sof_ops_debugfs_add_region_item(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, snd_sof_fw_blk_type, u32, size_t, *const c_char, sof_debugfs_access_type) -> c_int>;
    fn snd_sof_ops_write8(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, usize, u8)>;
    fn snd_sof_ops_write(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, usize, u32)>;
    fn snd_sof_ops_write64(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, usize, u64)>;
    fn snd_sof_ops_read8(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, usize) -> u8>;
    fn snd_sof_ops_read(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, usize) -> u32>;
    fn snd_sof_ops_read64(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, usize) -> u64>;
    fn snd_sof_ops_block_read(ops: *mut snd_sof_ops) -> unsafe extern "C" fn(*mut snd_sof_dev, snd_sof_fw_blk_type, u32, *mut c_void, size_t) -> c_int;
    fn snd_sof_ops_block_write(ops: *mut snd_sof_ops) -> unsafe extern "C" fn(*mut snd_sof_dev, snd_sof_fw_blk_type, u32, *mut c_void, size_t) -> c_int;
    fn snd_sof_ops_mailbox_read(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, u32, *mut c_void, size_t)>;
    fn snd_sof_ops_mailbox_write(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, u32, *mut c_void, size_t)>;
    fn snd_sof_ops_send_msg(ops: *mut snd_sof_ops) -> unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_ipc_msg) -> c_int;
    fn snd_sof_ops_pcm_open(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_pcm_substream) -> c_int>;
    fn snd_sof_ops_pcm_close(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_pcm_substream) -> c_int>;
    fn snd_sof_ops_pcm_hw_params(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_sof_platform_stream_params) -> c_int>;
    fn snd_sof_ops_pcm_hw_free(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_pcm_substream) -> c_int>;
    fn snd_sof_ops_pcm_trigger(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_pcm_substream, c_int) -> c_int>;
    fn snd_sof_ops_load_firmware(ops: *mut snd_sof_ops) -> unsafe extern "C" fn(*mut snd_sof_dev) -> c_int;
    fn snd_sof_ops_ipc_msg_data(ops: *mut snd_sof_ops) -> unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_pcm_stream, *mut c_void, size_t) -> c_int;
    fn snd_sof_ops_set_stream_data_offset(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_pcm_stream, size_t) -> c_int>;
    fn snd_sof_ops_pcm_pointer(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_pcm_substream) -> snd_pcm_uframes_t>;
    fn snd_sof_ops_pcm_ack(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_pcm_substream) -> c_int>;
    fn snd_sof_ops_get_dai_frame_counter(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_soc_component, *mut snd_pcm_substream) -> u64>;
    fn snd_sof_ops_get_host_byte_counter(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_soc_component, *mut snd_pcm_substream) -> u64>;
    fn snd_sof_ops_machine_register(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void) -> c_int>;
    fn snd_sof_ops_machine_unregister(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void)>;
    fn snd_sof_ops_machine_select(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev) -> *mut snd_soc_acpi_mach>;
    fn snd_sof_ops_set_mach_params(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_soc_acpi_mach, *mut snd_sof_dev)>;
    fn snd_sof_ops_is_chain_dma_supported(ops: *mut snd_sof_ops) -> Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> bool>;

    fn snd_sof_dev_num_cores(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_sof_dev_dev(sdev: *mut snd_sof_dev) -> *mut c_void;
    fn snd_sof_dev_mmio_bar(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_sof_dev_bar(sdev: *mut snd_sof_dev, bar: u32) -> usize;
    fn snd_sof_dev_dsp_core_ref_count(sdev: *mut snd_sof_dev, core: c_int) -> *mut c_int;
    fn snd_sof_dev_enabled_cores_mask_or(sdev: *mut snd_sof_dev, mask: c_ulong);
    fn snd_sof_dev_enabled_cores_mask_and(sdev: *mut snd_sof_dev, mask: c_ulong);
    fn snd_sof_dev_power_state_access(sdev: *mut snd_sof_dev) -> *mut c_void;

    fn BIT(nr: c_int) -> c_ulong;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn writeb(value: u8, addr: usize);
    fn writel(value: u32, addr: usize);
    fn writeq(value: u64, addr: usize);
    fn readb(addr: usize) -> u8;
    fn readl(addr: usize) -> u32;
    fn readq(addr: usize) -> u64;
    fn ktime_add_us(ktime: ktime_t, usec: u64) -> ktime_t;
    fn ktime_get() -> ktime_t;
    fn ktime_compare(cmp1: ktime_t, cmp2: ktime_t) -> c_int;
    fn might_sleep_if(cond: bool);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn guard_mutex(lock: *mut c_void);

    pub fn snd_sof_dsp_dbg_dump(sdev: *mut snd_sof_dev, msg: *const c_char, flags: u32);
    pub fn snd_sof_pci_update_bits(sdev: *mut snd_sof_dev, offset: u32, mask: u32, value: u32) -> bool;
    pub fn snd_sof_dsp_update_bits_unlocked(sdev: *mut snd_sof_dev, bar: u32, offset: u32, mask: u32, value: u32) -> bool;
    pub fn snd_sof_dsp_update_bits64_unlocked(sdev: *mut snd_sof_dev, bar: u32, offset: u32, mask: u64, value: u64) -> bool;
    pub fn snd_sof_dsp_update_bits(sdev: *mut snd_sof_dev, bar: u32, offset: u32, mask: u32, value: u32) -> bool;
    pub fn snd_sof_dsp_update_bits64(sdev: *mut snd_sof_dev, bar: u32, offset: u32, mask: u64, value: u64) -> bool;
    pub fn snd_sof_dsp_update_bits_forced(sdev: *mut snd_sof_dev, bar: u32, offset: u32, mask: u32, value: u32);
    pub fn snd_sof_dsp_register_poll(sdev: *mut snd_sof_dev, bar: u32, offset: u32, mask: u32, target: u32, timeout_ms: u32, interval_us: u32) -> c_int;
    pub fn snd_sof_dsp_panic(sdev: *mut snd_sof_dev, offset: u32, non_recoverable: bool);
}

pub unsafe fn sof_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    if let Some(ops_init) = unsafe { snd_sof_dev_pdata_desc_ops_init(sdev) } {
        return unsafe { ops_init(sdev) };
    }
    0
}

pub unsafe fn sof_ops_free(sdev: *mut snd_sof_dev) {
    if let Some(ops_free) = unsafe { snd_sof_dev_pdata_desc_ops_free(sdev) } {
        unsafe { ops_free(sdev) };
    }
}

/* Mandatory operations are verified during probing */

/* init */
pub unsafe fn snd_sof_probe_early(sdev: *mut snd_sof_dev) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(probe_early) = unsafe { snd_sof_ops_probe_early(ops) } {
        return unsafe { probe_early(sdev) };
    }
    0
}

pub unsafe fn snd_sof_probe(sdev: *mut snd_sof_dev) -> c_int {
    unsafe { snd_sof_ops_probe(sof_ops(sdev))(sdev) }
}

pub unsafe fn snd_sof_remove(sdev: *mut snd_sof_dev) {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(remove) = unsafe { snd_sof_ops_remove(ops) } {
        unsafe { remove(sdev) };
    }
}

pub unsafe fn snd_sof_remove_late(sdev: *mut snd_sof_dev) {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(remove_late) = unsafe { snd_sof_ops_remove_late(ops) } {
        unsafe { remove_late(sdev) };
    }
}

pub unsafe fn snd_sof_shutdown(sdev: *mut snd_sof_dev) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(shutdown) = unsafe { snd_sof_ops_shutdown(ops) } {
        return unsafe { shutdown(sdev) };
    }
    0
}

/* control */

/*
 * snd_sof_dsp_run returns the core mask of the cores that are available
 * after successful fw boot
 */
pub unsafe fn snd_sof_dsp_run(sdev: *mut snd_sof_dev) -> c_int {
    unsafe { snd_sof_ops_run(sof_ops(sdev))(sdev) }
}

pub unsafe fn snd_sof_dsp_stall(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(stall) = unsafe { snd_sof_ops_stall(ops) } {
        return unsafe { stall(sdev, core_mask) };
    }
    0
}

pub unsafe fn snd_sof_dsp_reset(sdev: *mut snd_sof_dev) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(reset) = unsafe { snd_sof_ops_reset(ops) } {
        return unsafe { reset(sdev) };
    }
    0
}

/* dsp core get/put */
pub unsafe fn snd_sof_dsp_core_get(sdev: *mut snd_sof_dev, core: c_int) -> c_int {
    if core > unsafe { snd_sof_dev_num_cores(sdev) } - 1 {
        unsafe {
            dev_err(
                snd_sof_dev_dev(sdev),
                c"invalid core id: %d for num_cores: %d\n".as_ptr(),
                core,
                snd_sof_dev_num_cores(sdev),
            )
        };
        return -EINVAL;
    }

    let ops = unsafe { sof_ops(sdev) };
    if let Some(core_get) = unsafe { snd_sof_ops_core_get(ops) } {
        let ref_count = unsafe { snd_sof_dev_dsp_core_ref_count(sdev, core) };

        /* if current ref_count is > 0, increment it and return */
        if unsafe { *ref_count } > 0 {
            unsafe { *ref_count += 1 };
            return 0;
        }

        /* power up the core */
        let ret = unsafe { core_get(sdev, core) };
        if ret < 0 {
            return ret;
        }

        /* increment ref_count */
        unsafe { *ref_count += 1 };

        /* and update enabled_cores_mask */
        unsafe { snd_sof_dev_enabled_cores_mask_or(sdev, BIT(core)) };

        unsafe { dev_dbg(snd_sof_dev_dev(sdev), c"Core %d powered up\n".as_ptr(), core) };
    }

    0
}

pub unsafe fn snd_sof_dsp_core_put(sdev: *mut snd_sof_dev, core: c_int) -> c_int {
    if core > unsafe { snd_sof_dev_num_cores(sdev) } - 1 {
        unsafe {
            dev_err(
                snd_sof_dev_dev(sdev),
                c"invalid core id: %d for num_cores: %d\n".as_ptr(),
                core,
                snd_sof_dev_num_cores(sdev),
            )
        };
        return -EINVAL;
    }

    let ops = unsafe { sof_ops(sdev) };
    if let Some(core_put) = unsafe { snd_sof_ops_core_put(ops) } {
        let ref_count = unsafe { snd_sof_dev_dsp_core_ref_count(sdev, core) };

        /* decrement ref_count and return if it is > 0 */
        unsafe { *ref_count -= 1 };
        if unsafe { *ref_count } > 0 {
            return 0;
        }

        /* power down the core */
        let ret = unsafe { core_put(sdev, core) };
        if ret < 0 {
            return ret;
        }

        /* and update enabled_cores_mask */
        unsafe { snd_sof_dev_enabled_cores_mask_and(sdev, !BIT(core)) };

        unsafe { dev_dbg(snd_sof_dev_dev(sdev), c"Core %d powered down\n".as_ptr(), core) };
    }

    0
}

/* pre/post fw load */
pub unsafe fn snd_sof_dsp_pre_fw_run(sdev: *mut snd_sof_dev) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(pre_fw_run) = unsafe { snd_sof_ops_pre_fw_run(ops) } {
        return unsafe { pre_fw_run(sdev) };
    }
    0
}

pub unsafe fn snd_sof_dsp_post_fw_run(sdev: *mut snd_sof_dev) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(post_fw_run) = unsafe { snd_sof_ops_post_fw_run(ops) } {
        return unsafe { post_fw_run(sdev) };
    }
    0
}

/* parse platform specific extended manifest */
pub unsafe fn snd_sof_dsp_parse_platform_ext_manifest(
    sdev: *mut snd_sof_dev,
    hdr: *const sof_ext_man_elem_header,
) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(parse_platform_ext_manifest) = unsafe { snd_sof_ops_parse_platform_ext_manifest(ops) } {
        return unsafe { parse_platform_ext_manifest(sdev, hdr) };
    }
    0
}

/* misc */

/**
 * snd_sof_dsp_get_bar_index - Maps a section type with a BAR index
 *
 * @sdev: sof device
 * @type: section type as described by snd_sof_fw_blk_type
 *
 * Returns the corresponding BAR index (a positive integer) or -EINVAL
 * in case there is no mapping
 */
pub unsafe fn snd_sof_dsp_get_bar_index(sdev: *mut snd_sof_dev, type_: u32) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(get_bar_index) = unsafe { snd_sof_ops_get_bar_index(ops) } {
        return unsafe { get_bar_index(sdev, type_) };
    }
    unsafe { snd_sof_dev_mmio_bar(sdev) }
}

pub unsafe fn snd_sof_dsp_get_mailbox_offset(sdev: *mut snd_sof_dev) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(get_mailbox_offset) = unsafe { snd_sof_ops_get_mailbox_offset(ops) } {
        return unsafe { get_mailbox_offset(sdev) };
    }
    unsafe { dev_err(snd_sof_dev_dev(sdev), c"error: %s not defined\n".as_ptr(), c"snd_sof_dsp_get_mailbox_offset".as_ptr()) };
    -EOPNOTSUPP
}

pub unsafe fn snd_sof_dsp_get_window_offset(sdev: *mut snd_sof_dev, id: u32) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(get_window_offset) = unsafe { snd_sof_ops_get_window_offset(ops) } {
        return unsafe { get_window_offset(sdev, id) };
    }
    unsafe { dev_err(snd_sof_dev_dev(sdev), c"error: %s not defined\n".as_ptr(), c"snd_sof_dsp_get_window_offset".as_ptr()) };
    -EOPNOTSUPP
}

/* power management */
pub unsafe fn snd_sof_dsp_resume(sdev: *mut snd_sof_dev) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(resume) = unsafe { snd_sof_ops_resume(ops) } {
        return unsafe { resume(sdev) };
    }
    0
}

pub unsafe fn snd_sof_dsp_suspend(sdev: *mut snd_sof_dev, target_state: u32) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(suspend) = unsafe { snd_sof_ops_suspend(ops) } {
        return unsafe { suspend(sdev, target_state) };
    }
    0
}

pub unsafe fn snd_sof_dsp_runtime_resume(sdev: *mut snd_sof_dev) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(runtime_resume) = unsafe { snd_sof_ops_runtime_resume(ops) } {
        return unsafe { runtime_resume(sdev) };
    }
    0
}

pub unsafe fn snd_sof_dsp_runtime_suspend(sdev: *mut snd_sof_dev) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(runtime_suspend) = unsafe { snd_sof_ops_runtime_suspend(ops) } {
        return unsafe { runtime_suspend(sdev) };
    }
    0
}

pub unsafe fn snd_sof_dsp_runtime_idle(sdev: *mut snd_sof_dev) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(runtime_idle) = unsafe { snd_sof_ops_runtime_idle(ops) } {
        return unsafe { runtime_idle(sdev) };
    }
    0
}

pub unsafe fn snd_sof_dsp_hw_params_upon_resume(sdev: *mut snd_sof_dev) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(set_hw_params_upon_resume) = unsafe { snd_sof_ops_set_hw_params_upon_resume(ops) } {
        return unsafe { set_hw_params_upon_resume(sdev) };
    }
    0
}

pub unsafe fn snd_sof_dsp_set_clk(sdev: *mut snd_sof_dev, freq: u32) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(set_clk) = unsafe { snd_sof_ops_set_clk(ops) } {
        return unsafe { set_clk(sdev, freq) };
    }
    0
}

pub unsafe fn snd_sof_dsp_set_power_state(
    sdev: *mut snd_sof_dev,
    target_state: *const sof_dsp_power_state,
) -> c_int {
    unsafe { guard_mutex(snd_sof_dev_power_state_access(sdev)) };

    let ops = unsafe { sof_ops(sdev) };
    if let Some(set_power_state) = unsafe { snd_sof_ops_set_power_state(ops) } {
        return unsafe { set_power_state(sdev, target_state) };
    }
    0
}

/* debug */
pub unsafe fn snd_sof_debugfs_add_region_item(
    sdev: *mut snd_sof_dev,
    blk_type: snd_sof_fw_blk_type,
    offset: u32,
    size: size_t,
    name: *const c_char,
    access_type: sof_debugfs_access_type,
) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(debugfs_add_region_item) = unsafe { snd_sof_ops_debugfs_add_region_item(ops) } {
            return unsafe { debugfs_add_region_item(sdev, blk_type, offset, size, name, access_type) };
        }
    }
    0
}

/* register IO */
pub unsafe fn snd_sof_dsp_write8(sdev: *mut snd_sof_dev, bar: u32, offset: u32, value: u8) {
    let ops = unsafe { sof_ops(sdev) };
    let addr = unsafe { snd_sof_dev_bar(sdev, bar) + offset as usize };
    if let Some(write8) = unsafe { snd_sof_ops_write8(ops) } {
        unsafe { write8(sdev, addr, value) };
    } else {
        unsafe { writeb(value, addr) };
    }
}

pub unsafe fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: u32, offset: u32, value: u32) {
    let ops = unsafe { sof_ops(sdev) };
    let addr = unsafe { snd_sof_dev_bar(sdev, bar) + offset as usize };
    if let Some(write) = unsafe { snd_sof_ops_write(ops) } {
        unsafe { write(sdev, addr, value) };
    } else {
        unsafe { writel(value, addr) };
    }
}

pub unsafe fn snd_sof_dsp_write64(sdev: *mut snd_sof_dev, bar: u32, offset: u32, value: u64) {
    let ops = unsafe { sof_ops(sdev) };
    let addr = unsafe { snd_sof_dev_bar(sdev, bar) + offset as usize };
    if let Some(write64) = unsafe { snd_sof_ops_write64(ops) } {
        unsafe { write64(sdev, addr, value) };
    } else {
        unsafe { writeq(value, addr) };
    }
}

pub unsafe fn snd_sof_dsp_read8(sdev: *mut snd_sof_dev, bar: u32, offset: u32) -> u8 {
    let ops = unsafe { sof_ops(sdev) };
    let addr = unsafe { snd_sof_dev_bar(sdev, bar) + offset as usize };
    if let Some(read8_fn) = unsafe { snd_sof_ops_read8(ops) } {
        unsafe { read8_fn(sdev, addr) }
    } else {
        unsafe { readb(addr) }
    }
}

pub unsafe fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: u32, offset: u32) -> u32 {
    let ops = unsafe { sof_ops(sdev) };
    let addr = unsafe { snd_sof_dev_bar(sdev, bar) + offset as usize };
    if let Some(read_fn) = unsafe { snd_sof_ops_read(ops) } {
        unsafe { read_fn(sdev, addr) }
    } else {
        unsafe { readl(addr) }
    }
}

pub unsafe fn snd_sof_dsp_read64(sdev: *mut snd_sof_dev, bar: u32, offset: u32) -> u64 {
    let ops = unsafe { sof_ops(sdev) };
    let addr = unsafe { snd_sof_dev_bar(sdev, bar) + offset as usize };
    if let Some(read64_fn) = unsafe { snd_sof_ops_read64(ops) } {
        unsafe { read64_fn(sdev, addr) }
    } else {
        unsafe { readq(addr) }
    }
}

pub unsafe fn snd_sof_dsp_update8(sdev: *mut snd_sof_dev, bar: u32, offset: u32, mask: u8, value: u8) {
    let mut reg: u8;

    reg = unsafe { snd_sof_dsp_read8(sdev, bar, offset) };
    reg &= !mask;
    reg |= value;
    unsafe { snd_sof_dsp_write8(sdev, bar, offset, reg) };
}

/* block IO */
pub unsafe fn snd_sof_dsp_block_read(
    sdev: *mut snd_sof_dev,
    blk_type: snd_sof_fw_blk_type,
    offset: u32,
    dest: *mut c_void,
    bytes: size_t,
) -> c_int {
    unsafe { snd_sof_ops_block_read(sof_ops(sdev))(sdev, blk_type, offset, dest, bytes) }
}

pub unsafe fn snd_sof_dsp_block_write(
    sdev: *mut snd_sof_dev,
    blk_type: snd_sof_fw_blk_type,
    offset: u32,
    src: *mut c_void,
    bytes: size_t,
) -> c_int {
    unsafe { snd_sof_ops_block_write(sof_ops(sdev))(sdev, blk_type, offset, src, bytes) }
}

/* mailbox IO */
pub unsafe fn snd_sof_dsp_mailbox_read(sdev: *mut snd_sof_dev, offset: u32, dest: *mut c_void, bytes: size_t) {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(mailbox_read) = unsafe { snd_sof_ops_mailbox_read(ops) } {
        unsafe { mailbox_read(sdev, offset, dest, bytes) };
    }
}

pub unsafe fn snd_sof_dsp_mailbox_write(sdev: *mut snd_sof_dev, offset: u32, src: *mut c_void, bytes: size_t) {
    let ops = unsafe { sof_ops(sdev) };
    if let Some(mailbox_write) = unsafe { snd_sof_ops_mailbox_write(ops) } {
        unsafe { mailbox_write(sdev, offset, src, bytes) };
    }
}

/* ipc */
pub unsafe fn snd_sof_dsp_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int {
    unsafe { snd_sof_ops_send_msg(sof_ops(sdev))(sdev, msg) }
}

/* host PCM ops */
pub unsafe fn snd_sof_pcm_platform_open(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(pcm_open) = unsafe { snd_sof_ops_pcm_open(ops) } {
            return unsafe { pcm_open(sdev, substream) };
        }
    }
    0
}

/* disconnect pcm substream to a host stream */
pub unsafe fn snd_sof_pcm_platform_close(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(pcm_close) = unsafe { snd_sof_ops_pcm_close(ops) } {
            return unsafe { pcm_close(sdev, substream) };
        }
    }
    0
}

/* host stream hw params */
pub unsafe fn snd_sof_pcm_platform_hw_params(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    platform_params: *mut snd_sof_platform_stream_params,
) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(pcm_hw_params) = unsafe { snd_sof_ops_pcm_hw_params(ops) } {
            return unsafe { pcm_hw_params(sdev, substream, params, platform_params) };
        }
    }
    0
}

/* host stream hw free */
pub unsafe fn snd_sof_pcm_platform_hw_free(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(pcm_hw_free) = unsafe { snd_sof_ops_pcm_hw_free(ops) } {
            return unsafe { pcm_hw_free(sdev, substream) };
        }
    }
    0
}

/* host stream trigger */
pub unsafe fn snd_sof_pcm_platform_trigger(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(pcm_trigger) = unsafe { snd_sof_ops_pcm_trigger(ops) } {
            return unsafe { pcm_trigger(sdev, substream, cmd) };
        }
    }
    0
}

/* Firmware loading */
pub unsafe fn snd_sof_load_firmware(sdev: *mut snd_sof_dev) -> c_int {
    unsafe { dev_dbg(snd_sof_dev_dev(sdev), c"loading firmware\n".as_ptr()) };

    unsafe { snd_sof_ops_load_firmware(sof_ops(sdev))(sdev) }
}

/* host DSP message data */
pub unsafe fn snd_sof_ipc_msg_data(
    sdev: *mut snd_sof_dev,
    sps: *mut snd_sof_pcm_stream,
    p: *mut c_void,
    sz: size_t,
) -> c_int {
    unsafe { snd_sof_ops_ipc_msg_data(sof_ops(sdev))(sdev, sps, p, sz) }
}

/* host side configuration of the stream's data offset in stream mailbox area */
pub unsafe fn snd_sof_set_stream_data_offset(
    sdev: *mut snd_sof_dev,
    sps: *mut snd_sof_pcm_stream,
    posn_offset: size_t,
) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(set_stream_data_offset) = unsafe { snd_sof_ops_set_stream_data_offset(ops) } {
            return unsafe { set_stream_data_offset(sdev, sps, posn_offset) };
        }
    }
    0
}

/* host stream pointer */
pub unsafe fn snd_sof_pcm_platform_pointer(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(pcm_pointer) = unsafe { snd_sof_ops_pcm_pointer(ops) } {
            return unsafe { pcm_pointer(sdev, substream) };
        }
    }
    0
}

/* pcm ack */
pub unsafe fn snd_sof_pcm_platform_ack(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(pcm_ack) = unsafe { snd_sof_ops_pcm_ack(ops) } {
            return unsafe { pcm_ack(sdev, substream) };
        }
    }
    0
}

pub unsafe fn snd_sof_pcm_get_dai_frame_counter(
    sdev: *mut snd_sof_dev,
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> u64 {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(get_dai_frame_counter) = unsafe { snd_sof_ops_get_dai_frame_counter(ops) } {
            return unsafe { get_dai_frame_counter(sdev, component, substream) };
        }
    }
    0
}

pub unsafe fn snd_sof_pcm_get_host_byte_counter(
    sdev: *mut snd_sof_dev,
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> u64 {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(get_host_byte_counter) = unsafe { snd_sof_ops_get_host_byte_counter(ops) } {
            return unsafe { get_host_byte_counter(sdev, component, substream) };
        }
    }
    0
}

/* machine driver */
pub unsafe fn snd_sof_machine_register(sdev: *mut snd_sof_dev, pdata: *mut c_void) -> c_int {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(machine_register) = unsafe { snd_sof_ops_machine_register(ops) } {
            return unsafe { machine_register(sdev, pdata) };
        }
    }
    0
}

pub unsafe fn snd_sof_machine_unregister(sdev: *mut snd_sof_dev, pdata: *mut c_void) {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(machine_unregister) = unsafe { snd_sof_ops_machine_unregister(ops) } {
            unsafe { machine_unregister(sdev, pdata) };
        }
    }
}

pub unsafe fn snd_sof_machine_select(sdev: *mut snd_sof_dev) -> *mut snd_soc_acpi_mach {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(machine_select) = unsafe { snd_sof_ops_machine_select(ops) } {
            return unsafe { machine_select(sdev) };
        }
    }
    core::ptr::null_mut()
}

pub unsafe fn snd_sof_set_mach_params(mach: *mut snd_soc_acpi_mach, sdev: *mut snd_sof_dev) {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(set_mach_params) = unsafe { snd_sof_ops_set_mach_params(ops) } {
            unsafe { set_mach_params(mach, sdev) };
        }
    }
}

pub unsafe fn snd_sof_is_chain_dma_supported(sdev: *mut snd_sof_dev, dai_type: u32) -> bool {
    let ops = unsafe { sof_ops(sdev) };
    if !ops.is_null() {
        if let Some(is_chain_dma_supported) = unsafe { snd_sof_ops_is_chain_dma_supported(ops) } {
            return unsafe { is_chain_dma_supported(sdev, dai_type) };
        }
    }
    false
}

/**
 * snd_sof_dsp_register_poll_timeout - Periodically poll an address
 * until a condition is met or a timeout occurs
 * @op: accessor function (takes @addr as its only argument)
 * @addr: Address to poll
 * @val: Variable to read the value into
 * @cond: Break condition (usually involving @val)
 * @sleep_us: Maximum time to sleep between reads in us (0 tight-loops). Please
 *            read usleep_range() function description for details and
 *            limitations.
 * @timeout_us: Timeout in us, 0 means never timeout
 *
 * Returns: 0 on success and -ETIMEDOUT upon a timeout. In either
 * case, the last read value at @addr is stored in @val. Must not
 * be called from atomic context if sleep_us or timeout_us are used.
 *
 * This is modelled after the readx_poll_timeout macros in linux/iopoll.h.
 */
#[macro_export]
macro_rules! snd_sof_dsp_read_poll_timeout {
    ($sdev:expr, $bar:expr, $offset:expr, $val:expr, $cond:expr, $sleep_us:expr, $timeout_us:expr) => {{
        let __timeout_us: $crate::u64 = $timeout_us;
        let __sleep_us: core::ffi::c_ulong = $sleep_us;
        let __timeout = unsafe { $crate::ktime_add_us($crate::ktime_get(), __timeout_us) };
        unsafe { $crate::might_sleep_if(__sleep_us != 0) };
        loop {
            $val = unsafe { $crate::snd_sof_dsp_read($sdev, $bar, $offset) };
            if $cond {
                unsafe {
                    $crate::dev_dbg(
                        $crate::snd_sof_dev_dev($sdev),
                        c"FW Poll Status: reg[%#x]=%#x successful\n".as_ptr(),
                        $offset,
                        $val,
                    )
                };
                break;
            }
            if __timeout_us != 0 && unsafe { $crate::ktime_compare($crate::ktime_get(), __timeout) } > 0 {
                $val = unsafe { $crate::snd_sof_dsp_read($sdev, $bar, $offset) };
                unsafe {
                    $crate::dev_dbg(
                        $crate::snd_sof_dev_dev($sdev),
                        c"FW Poll Status: reg[%#x]=%#x timedout\n".as_ptr(),
                        $offset,
                        $val,
                    )
                };
                break;
            }
            if __sleep_us != 0 {
                unsafe { $crate::usleep_range((__sleep_us >> 2) + 1, __sleep_us) };
            }
        }
        if $cond {
            0
        } else {
            -$crate::ETIMEDOUT
        }
    }};
}

/* This is for registers bits with attribute RWC */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
