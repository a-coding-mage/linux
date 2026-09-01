/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2020 Intel Corporation
 *
 * Author: Cezary Rojewski <cezary.rojewski@intel.com>
 */

/* Dependencies from the C header:
 * <linux/dma/dw.h>, <linux/irqreturn.h>, "messages.h", "registers.h",
 * <sound/memalloc.h>, <uapi/sound/asound.h>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::ManuallyDrop;

unsafe extern "C" {
    pub static catpt_attr_groups: [*const attribute_group; 0];

    pub fn catpt_sram_free(sram: *mut resource);
    pub fn catpt_request_region(root: *mut resource, size: resource_size_t) -> *mut resource;
}

#[repr(C)]
pub union catpt_ipc_msg__bindgen_ty_1 {
    pub header: u32,
    pub rsp: ManuallyDrop<catpt_global_msg>,
}

#[repr(C)]
pub struct catpt_ipc_msg {
    pub __bindgen_anon_1: catpt_ipc_msg__bindgen_ty_1,
    pub data: *mut c_void,
    pub size: usize,
}

#[repr(C)]
pub struct catpt_ipc {
    pub dev: *mut device,

    pub rx: catpt_ipc_msg,
    pub config: catpt_fw_ready,
    pub default_timeout: u32,
    pub ready: bool,

    pub lock: spinlock_t,
    pub mutex: mutex,
    pub done_completion: completion,
    pub busy_completion: completion,
}

unsafe extern "C" {
    pub fn catpt_ipc_init(ipc: *mut catpt_ipc, dev: *mut device);
}

#[repr(C)]
pub struct catpt_module_type {
    pub loaded: bool,
    pub entry_point: u32,
    pub persistent_size: u32,
    pub scratch_size: u32,
    /* DRAM, initial module state */
    pub state_offset: u32,
    pub state_size: u32,

    pub node: list_head,
}

#[repr(C)]
pub struct catpt_spec {
    pub machines: *mut snd_soc_acpi_mach,
    pub core_id: u8,
    pub fw_name: *const c_char,
    pub host_dram_offset: u32,
    pub host_iram_offset: u32,
    pub host_shim_offset: u32,
    pub host_dma_offset: [u32; CATPT_DMA_COUNT],
    pub host_ssp_offset: [u32; CATPT_SSP_COUNT],
    pub dram_mask: u32,
    pub iram_mask: u32,
    pub d3srampgd_bit: u32,
    pub d3pgd_bit: u32,
    pub pll_shutdown: Option<unsafe extern "C" fn(cdev: *mut catpt_dev, enable: bool)>,
}

#[repr(C)]
pub struct catpt_dev {
    pub dev: *mut device,
    pub dmac: *mut dw_dma_chip,
    pub ipc: catpt_ipc,

    pub pci_ba: *mut c_void,
    pub lpe_ba: *mut c_void,
    pub lpe_base: u32,
    pub irq: c_int,

    pub spec: *const catpt_spec,
    pub fw_ready: completion,

    pub dram: resource,
    pub iram: resource,
    pub scratch: *mut resource,

    pub mixer: catpt_mixer_stream_info,
    pub modules: [catpt_module_type; CATPT_MODULE_COUNT],
    pub devfmt: [catpt_ssp_device_format; CATPT_SSP_COUNT],
    pub stream_list: list_head,
    pub stream_mutex: mutex,
    pub clk_mutex: mutex,

    pub dx_ctx: catpt_dx_context,
    pub dxbuf_vaddr: *mut c_void,
    pub dxbuf_paddr: dma_addr_t,
}

unsafe extern "C" {
    pub fn catpt_dmac_probe(cdev: *mut catpt_dev) -> c_int;
    pub fn catpt_dmac_remove(cdev: *mut catpt_dev);
    pub fn catpt_dma_request_config_chan(cdev: *mut catpt_dev) -> *mut dma_chan;
    pub fn catpt_dma_memcpy_todsp(
        cdev: *mut catpt_dev,
        chan: *mut dma_chan,
        dst_addr: dma_addr_t,
        src_addr: dma_addr_t,
        size: usize,
    ) -> c_int;
    pub fn catpt_dma_memcpy_fromdsp(
        cdev: *mut catpt_dev,
        chan: *mut dma_chan,
        dst_addr: dma_addr_t,
        src_addr: dma_addr_t,
        size: usize,
    ) -> c_int;

    pub fn lpt_dsp_pll_shutdown(cdev: *mut catpt_dev, enable: bool);
    pub fn wpt_dsp_pll_shutdown(cdev: *mut catpt_dev, enable: bool);
    pub fn catpt_dsp_power_up(cdev: *mut catpt_dev) -> c_int;
    pub fn catpt_dsp_power_down(cdev: *mut catpt_dev) -> c_int;
    pub fn catpt_dsp_stall(cdev: *mut catpt_dev, stall: bool) -> c_int;
    pub fn catpt_dsp_update_srampge(cdev: *mut catpt_dev, sram: *mut resource, mask: c_ulong);
    pub fn catpt_dsp_update_lpclock(cdev: *mut catpt_dev) -> c_int;
    pub fn catpt_dsp_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
    pub fn catpt_dsp_irq_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}

/*
 * IPC handlers may return positive values which denote successful
 * HOST <-> DSP communication yet failure to process specific request.
 * Use below macro to convert returned non-zero values appropriately
 */
#[inline]
pub const fn CATPT_IPC_RET(ret: c_int) -> c_int {
    if ret <= 0 {
        ret
    } else {
        -EREMOTEIO
    }
}

unsafe extern "C" {
    pub fn catpt_dsp_send_msg_timeout(
        cdev: *mut catpt_dev,
        request: catpt_ipc_msg,
        reply: *mut catpt_ipc_msg,
        timeout: c_int,
        name: *const c_char,
    ) -> c_int;
    pub fn catpt_dsp_send_msg(
        cdev: *mut catpt_dev,
        request: catpt_ipc_msg,
        reply: *mut catpt_ipc_msg,
        name: *const c_char,
    ) -> c_int;

    pub fn catpt_first_boot_firmware(cdev: *mut catpt_dev) -> c_int;
    pub fn catpt_boot_firmware(cdev: *mut catpt_dev, restore: bool) -> c_int;
    pub fn catpt_store_firmware_context(cdev: *mut catpt_dev) -> c_int;
    pub fn catpt_coredump(cdev: *mut catpt_dev) -> c_int;
}

#[repr(C)]
pub struct catpt_stream_runtime {
    pub substream: *mut snd_pcm_substream,

    pub template: *mut catpt_stream_template,
    pub info: catpt_stream_info,
    pub persistent: *mut resource,
    pub pgtbl: snd_dma_buffer,

    pub allocated: bool,
    pub prepared: bool,

    pub node: list_head,
}

unsafe extern "C" {
    pub fn catpt_register_plat_component(cdev: *mut catpt_dev) -> c_int;
    pub fn catpt_stream_update_position(
        cdev: *mut catpt_dev,
        stream: *mut catpt_stream_runtime,
        pos: *mut catpt_notify_position,
    );
    pub fn catpt_stream_find(cdev: *mut catpt_dev, stream_hw_id: u8) -> *mut catpt_stream_runtime;
    pub fn catpt_arm_stream_templates(cdev: *mut catpt_dev) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
