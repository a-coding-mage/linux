/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license. When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2021, 2023 Advanced Micro Devices, Inc. All rights reserved.
 *
 * Author: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
 */

/* Rust translation of soc/sof/amd/acp.h. */
/* C includes translated as external type/function dependencies:
 * linux/dmi.h, linux/soundwire/sdw_amd.h, ../sof-priv.h, ../sof-audio.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_uchar, c_void};

pub type u8 = ::core::primitive::u8;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type size_t = usize;
pub type dma_addr_t = usize;
pub type irqreturn_t = c_uint;
pub type snd_pcm_uframes_t = usize;

#[repr(C)]
pub struct list_head {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_dma_buffer {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_stream {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct firmware {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sdw_amd_acpi_info {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sdw_amd_ctx {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_fw_blk_type {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_pcm_stream {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_ipc_msg {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_platform_stream_params {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc_dma_trace_params_ext {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub desc: *const sof_dev_desc,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub chip_info: *const sof_amd_acp_desc,
}

#[repr(C)]
pub struct dmi_system_id {
    _unused: [u8; 0],
}

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const ACP_MAX_STREAM: usize = 8;

pub const ACP_DSP_BAR: u32 = 0;

pub const ACP_HW_SEM_RETRY_COUNT: u32 = 10000;
pub const ACP_REG_POLL_INTERVAL: u32 = 500;
pub const ACP_REG_POLL_TIMEOUT_US: u32 = 2000;
pub const ACP_DMA_COMPLETE_TIMEOUT_US: u32 = 5000;

pub const ACP3X_PGFSM_CNTL_POWER_ON_MASK: u32 = 0x01;
pub const ACP3X_PGFSM_STATUS_MASK: u32 = 0x03;
pub const ACP6X_PGFSM_CNTL_POWER_ON_MASK: u32 = 0x07;
pub const ACP6X_PGFSM_STATUS_MASK: u32 = 0x0F;
pub const ACP70_PGFSM_CNTL_POWER_ON_MASK: u32 = 0x1F;
pub const ACP70_PGFSM_STATUS_MASK: u32 = 0xFF;

pub const ACP_POWERED_ON: u32 = 0x00;
pub const ACP_ASSERT_RESET: u32 = 0x01;
pub const ACP_RELEASE_RESET: u32 = 0x00;
pub const ACP_SOFT_RESET_DONE_MASK: u32 = 0x00010001;
pub const ACP_DSP_ASSERT_RESET: u32 = 0x04;
pub const ACP_DSP_RELEASE_RESET: u32 = 0x00;
pub const ACP_DSP_SOFT_RESET_DONE_MASK: u32 = 0x00050004;

pub const ACP_DSP_INTR_EN_MASK: u32 = 0x00000001;
pub const ACP3X_SRAM_PTE_OFFSET: u32 = 0x02050000;
pub const ACP5X_SRAM_PTE_OFFSET: u32 = 0x02050000;
pub const ACP6X_SRAM_PTE_OFFSET: u32 = 0x03800000;
pub const ACP70_SRAM_PTE_OFFSET: u32 = ACP6X_SRAM_PTE_OFFSET;
pub const PAGE_SIZE_4K_ENABLE: u32 = 0x2;
pub const ACP_PAGE_SIZE: u32 = 0x1000;
pub const ACP_DMA_CH_RUN: u32 = 0x02;
pub const ACP_MAX_DESC_CNT: u32 = 0x02;
pub const DSP_FW_RUN_ENABLE: u32 = 0x01;
pub const ACP_SHA_RUN: u32 = 0x01;
pub const ACP_SHA_RESET: u32 = 0x02;
pub const ACP_SHA_HEADER: u32 = 0x01;
pub const ACP_DMA_CH_RST: u32 = 0x01;
pub const ACP_DMA_CH_GRACEFUL_RST_EN: u32 = 0x10;
pub const ACP_ATU_CACHE_INVALID: u32 = 0x01;
pub const ACP_MAX_DESC: usize = 128;
/* ACPBUS_REG_BASE_OFFSET maps to the externally defined ACP_DMA_CNTL_0 register. */

pub const ACP_DEFAULT_DRAM_LENGTH: u32 = 0x00080000;
pub const ACP3X_SCRATCH_MEMORY_ADDRESS: u32 = 0x02050000;
pub const ACP_SYSTEM_MEMORY_WINDOW: u32 = 0x4000000;
pub const ACP_IRAM_BASE_ADDRESS: u32 = 0x000000;
pub const ACP_DRAM_BASE_ADDRESS: u32 = 0x01000000;
pub const ACP_DRAM_PAGE_COUNT: u32 = 128;
pub const ACP_SRAM_BASE_ADDRESS: u32 = 0x3806000;
pub const ACP7X_SRAM_BASE_ADDRESS: u32 = 0x380C000;
pub const ACP_DSP_TO_HOST_IRQ: u32 = 0x04;

pub const ACP_RN_PCI_ID: u32 = 0x01;
pub const ACP_VANGOGH_PCI_ID: u32 = 0x50;
pub const ACP_RMB_PCI_ID: u32 = 0x6F;
pub const ACP63_PCI_ID: u32 = 0x63;
pub const ACP70_PCI_ID: u32 = 0x70;
pub const ACP71_PCI_ID: u32 = 0x71;
pub const ACP72_PCI_ID: u32 = 0x72;
pub const ACP7B_PCI_ID: u32 = 0x7B;
pub const ACP7F_PCI_ID: u32 = 0x7F;

pub const ACP7X_PGFSM_CNTL_POWER_ON_MASK: u32 = 0x7F;
pub const ACP7X_PGFSM_STATUS_MASK: u32 = 0xFFF;
pub const ACP7X_SRAM_PTE_OFFSET: u32 = ACP6X_SRAM_PTE_OFFSET;

pub const HOST_BRIDGE_CZN: u32 = 0x1630;
pub const HOST_BRIDGE_VGH: u32 = 0x1645;
pub const HOST_BRIDGE_RMB: u32 = 0x14B5;
pub const HOST_BRIDGE_ACP63: u32 = 0x14E8;
pub const HOST_BRIDGE_ACP70: u32 = 0x1507;
pub const ACP_SHA_STAT: u32 = 0x8000;
pub const ACP_PSP_TIMEOUT_US: u32 = 1000000;
pub const ACP_EXT_INTR_ERROR_STAT: u32 = 0x20000000;
pub const MP0_C2PMSG_114_REG: u32 = 0x3810AC8;
pub const MP0_C2PMSG_73_REG: u32 = 0x3810A24;
pub const MBOX_ACP_SHA_DMA_COMMAND: u32 = 0x70000;
pub const MBOX_ACP_IRAM_DRAM_FENCE_COMMAND: u32 = 0x80000;
pub const MBOX_DELAY_US: u32 = 1000;
pub const MBOX_READY_MASK: u32 = 0x80000000;
pub const MBOX_STATUS_MASK: u32 = 0xFFFF;
pub const MBOX_ISREADY_FLAG: u32 = 0x40000000;
pub const IRAM_DRAM_FENCE_0: u32 = 0x0;
pub const IRAM_DRAM_FENCE_1: u32 = 0x01;
pub const IRAM_DRAM_FENCE_2: u32 = 0x02;

pub const BOX_SIZE_512: u32 = 0x200;
pub const BOX_SIZE_1024: u32 = 0x400;

pub const EXCEPT_MAX_HDR_SIZE: u32 = 0x400;
pub const AMD_STACK_DUMP_SIZE: u32 = 32;

pub const SRAM1_SIZE: u32 = 0x280000;
pub const PROBE_STATUS_BIT: u32 = BIT(31);

pub const ACP_FIRMWARE_SIGNATURE: u32 = 0x100;
pub const ACP_IMAGE_HEADER_SIZE: u32 = ACP_FIRMWARE_SIGNATURE;
pub const ACP_IMAGE_HDR_SIZE_FW_SIGNED_OFF: u32 = 0x14;

pub const ACP_ERROR_IRQ_MASK: u32 = BIT(29);
pub const ACP_SDW0_IRQ_MASK: u32 = BIT(21);
pub const ACP_SDW1_IRQ_MASK: u32 = BIT(2);
pub const SDW_ACPI_ADDR_ACP63: u32 = 5;
pub const SDW_ACPI_ADDR_ACP70: u32 = SDW_ACPI_ADDR_ACP63;
pub const ACP_DEFAULT_SRAM_LENGTH: u32 = 0x00080000;
pub const ACP_SRAM_PAGE_COUNT: u32 = 128;
pub const ACP6X_SDW_MAX_MANAGER_COUNT: u32 = 2;
pub const ACP70_SDW_MAX_MANAGER_COUNT: u32 = ACP6X_SDW_MAX_MANAGER_COUNT;
pub const ACP_DSP_MSG_SET: u32 = 1;
pub const ACP_DSP_ACK_SET: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum clock_source {
    ACP_CLOCK_96M = 0,
    ACP_CLOCK_48M = 1,
    ACP_CLOCK_24M = 2,
    ACP_CLOCK_ACLK = 3,
    ACP_CLOCK_MCLK = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_atu_grp_pte {
    pub low: u32,
    pub high: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_tx_cnt_bitfields {
    pub storage: c_uint,
}

impl dma_tx_cnt_bitfields {
    pub const fn count(&self) -> c_uint {
        self.storage & 0x7ffff
    }

    pub fn set_count(&mut self, value: c_uint) {
        self.storage = (self.storage & !0x7ffff) | (value & 0x7ffff);
    }

    pub const fn reserved(&self) -> c_uint {
        (self.storage >> 19) & 0xfff
    }

    pub fn set_reserved(&mut self, value: c_uint) {
        self.storage = (self.storage & !(0xfff << 19)) | ((value & 0xfff) << 19);
    }

    pub const fn ioc(&self) -> c_uint {
        (self.storage >> 31) & 0x1
    }

    pub fn set_ioc(&mut self, value: c_uint) {
        self.storage = (self.storage & !(0x1 << 31)) | ((value & 0x1) << 31);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dma_tx_cnt {
    pub bitfields: dma_tx_cnt_bitfields,
    pub bits: dma_tx_cnt_bitfields,
    pub u32_all: c_uint,
    pub i32_all: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_descriptor {
    pub src_addr: c_uint,
    pub dest_addr: c_uint,
    pub tx_cnt: dma_tx_cnt,
    pub reserved: c_uint,
}

/* Scratch memory structure for communication b/w host and dsp */
#[repr(C)]
pub struct scratch_ipc_conf {
    /* Debug memory */
    pub sof_debug_box: [u8; 1024],
    /* Exception memory*/
    pub sof_except_box: [u8; 1024],
    /* Stream buffer */
    pub sof_stream_box: [u8; 1024],
    /* Trace buffer */
    pub sof_trace_box: [u8; 1024],
    /* Host msg flag */
    pub sof_host_msg_write: u32,
    /* Host ack flag*/
    pub sof_host_ack_write: u32,
    /* DSP msg flag */
    pub sof_dsp_msg_write: u32,
    /* Dsp ack flag */
    pub sof_dsp_ack_write: u32,
}

#[repr(C)]
pub struct scratch_reg_conf {
    pub info: scratch_ipc_conf,
    pub grp1_pte: [acp_atu_grp_pte; 16],
    pub grp2_pte: [acp_atu_grp_pte; 16],
    pub grp3_pte: [acp_atu_grp_pte; 16],
    pub grp4_pte: [acp_atu_grp_pte; 16],
    pub grp5_pte: [acp_atu_grp_pte; 16],
    pub grp6_pte: [acp_atu_grp_pte; 16],
    pub grp7_pte: [acp_atu_grp_pte; 16],
    pub grp8_pte: [acp_atu_grp_pte; 16],
    pub dma_desc: [dma_descriptor; 64],
    pub reg_offset: [c_uint; 8],
    pub buf_size: [c_uint; 8],
    pub acp_tx_fifo_buf: [u8; 256],
    pub acp_rx_fifo_buf: [u8; 256],
    pub reserve: [c_uint; 0],
}

#[repr(C)]
pub struct acp_dsp_stream {
    pub list: list_head,
    pub sdev: *mut snd_sof_dev,
    pub substream: *mut snd_pcm_substream,
    pub dmab: *mut snd_dma_buffer,
    pub num_pages: c_int,
    pub stream_tag: c_int,
    pub active: c_int,
    pub reg_offset: c_uint,
    pub posn_offset: size_t,
    pub cstream: *mut snd_compr_stream,
    pub cstream_posn: u64,
}

#[repr(C)]
pub struct sof_amd_acp_desc {
    pub name: *const c_char,
    pub pgfsm_base: u32,
    pub ext_intr_enb: u32,
    pub ext_intr_cntl: u32,
    pub ext_intr_stat: u32,
    pub ext_intr_stat1: u32,
    pub dsp_intr_base: u32,
    pub sram_pte_offset: u32,
    pub hw_semaphore_offset: u32,
    pub acp_clkmux_sel: u32,
    pub fusion_dsp_offset: u32,
    pub probe_reg_offset: u32,
    pub reg_start_addr: u32,
    pub reg_end_addr: u32,
    pub acp_error_stat: u32,
    pub acp_sw0_i2s_err_reason: u32,
    pub sdw_max_link_count: u32,
    pub sdw_acpi_dev_addr: u64,
}

#[repr(C)]
pub struct acp_quirk_entry {
    pub signed_fw_image: bool,
    pub skip_iram_dram_size_mod: bool,
    pub post_fw_run_delay: bool,
}

/* Common device data struct for ACP devices */
#[repr(C)]
pub struct acp_dev_data {
    pub dev: *mut snd_sof_dev,
    pub fw_dbin: *const firmware,
    /* DMIC device */
    pub dmic_dev: *mut platform_device,
    /* mutex lock to protect ACP common registers access */
    pub acp_lock: mutex,
    /* ACPI information stored between scan and probe steps */
    pub info: sdw_amd_acpi_info,
    /* sdw context allocated by SoundWire driver */
    pub sdw: *mut sdw_amd_ctx,
    pub fw_bin_size: c_uint,
    pub fw_data_bin_size: c_uint,
    pub fw_sram_data_bin_size: c_uint,
    pub fw_code_bin: *const c_char,
    pub fw_data_bin: *const c_char,
    pub fw_sram_data_bin: *const c_char,
    pub fw_bin_page_count: u32,
    pub fw_data_bin_page_count: u32,
    pub addr: u32,
    pub reg_range: u32,
    pub blk_type: u32,
    pub sha_dma_addr: dma_addr_t,
    pub bin_buf: *mut u8,
    pub dma_addr: dma_addr_t,
    pub data_buf: *mut u8,
    pub sram_dma_addr: dma_addr_t,
    pub sram_data_buf: *mut u8,
    pub quirks: *mut acp_quirk_entry,
    pub dscr_info: [dma_descriptor; ACP_MAX_DESC],
    pub stream_buf: [acp_dsp_stream; ACP_MAX_STREAM],
    pub dtrace_stream: *mut acp_dsp_stream,
    pub probe_stream: *mut acp_dsp_stream,
    pub enable_fw_debug: bool,
    pub is_dram_in_use: bool,
    pub is_sram_in_use: bool,
    pub sdw_en_stat: bool,
    /* acp70_sdw0_wake_event flag set to true when wake irq asserted for SW0 instance */
    pub acp70_sdw0_wake_event: bool,
    /* acp70_sdw1_wake_event flag set to true when wake irq asserted for SW1 instance */
    pub acp70_sdw1_wake_event: bool,
    pub pci_rev: c_uint,
    pub acp_sof_signed_firmware_image: c_int,
}

unsafe extern "C" {
    pub fn memcpy_to_scratch(
        sdev: *mut snd_sof_dev,
        offset: u32,
        src: *mut c_uint,
        bytes: size_t,
    );
    pub fn memcpy_from_scratch(
        sdev: *mut snd_sof_dev,
        offset: u32,
        dst: *mut c_uint,
        bytes: size_t,
    );

    pub fn acp_dma_status(adata: *mut acp_dev_data, ch: c_uchar) -> c_int;
    pub fn configure_and_run_dma(
        adata: *mut acp_dev_data,
        src_addr: c_uint,
        dest_addr: c_uint,
        dsp_data_size: c_int,
    ) -> c_int;
    pub fn configure_and_run_sha_dma(
        adata: *mut acp_dev_data,
        image_addr: *mut c_void,
        start_addr: c_uint,
        dest_addr: c_uint,
        image_length: c_uint,
    ) -> c_int;

    /* ACP device probe/remove */
    pub fn amd_sof_acp_probe(sdev: *mut snd_sof_dev) -> c_int;
    pub fn amd_sof_acp_remove(sdev: *mut snd_sof_dev);

    /* DSP Loader callbacks */
    pub fn acp_sof_dsp_run(sdev: *mut snd_sof_dev) -> c_int;
    pub fn acp_dsp_pre_fw_run(sdev: *mut snd_sof_dev) -> c_int;
    pub fn acp_sof_load_signed_firmware(sdev: *mut snd_sof_dev) -> c_int;
    pub fn acp_get_bar_index(sdev: *mut snd_sof_dev, type_: u32) -> c_int;

    /* Block IO callbacks */
    pub fn acp_dsp_block_write(
        sdev: *mut snd_sof_dev,
        blk_type: snd_sof_fw_blk_type,
        offset: u32,
        src: *mut c_void,
        size: size_t,
    ) -> c_int;
    pub fn acp_dsp_block_read(
        sdev: *mut snd_sof_dev,
        blk_type: snd_sof_fw_blk_type,
        offset: u32,
        dest: *mut c_void,
        size: size_t,
    ) -> c_int;

    /* IPC callbacks */
    pub fn acp_sof_ipc_irq_thread(irq: c_int, context: *mut c_void) -> irqreturn_t;
    pub fn acp_sof_ipc_msg_data(
        sdev: *mut snd_sof_dev,
        sps: *mut snd_sof_pcm_stream,
        p: *mut c_void,
        sz: size_t,
    ) -> c_int;
    pub fn acp_set_stream_data_offset(
        sdev: *mut snd_sof_dev,
        sps: *mut snd_sof_pcm_stream,
        posn_offset: size_t,
    ) -> c_int;
    pub fn acp_sof_ipc_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int;
    pub fn acp_sof_ipc_get_mailbox_offset(sdev: *mut snd_sof_dev) -> c_int;
    pub fn acp_sof_ipc_get_window_offset(sdev: *mut snd_sof_dev, id: u32) -> c_int;
    pub fn acp_mailbox_write(
        sdev: *mut snd_sof_dev,
        offset: u32,
        message: *mut c_void,
        bytes: size_t,
    );
    pub fn acp_mailbox_read(
        sdev: *mut snd_sof_dev,
        offset: u32,
        message: *mut c_void,
        bytes: size_t,
    );

    /* ACP - DSP  stream callbacks */
    pub fn acp_dsp_stream_config(
        sdev: *mut snd_sof_dev,
        stream: *mut acp_dsp_stream,
    ) -> c_int;
    pub fn acp_dsp_stream_init(sdev: *mut snd_sof_dev) -> c_int;
    pub fn acp_dsp_stream_get(sdev: *mut snd_sof_dev, tag: c_int) -> *mut acp_dsp_stream;
    pub fn acp_dsp_stream_put(
        sdev: *mut snd_sof_dev,
        acp_stream: *mut acp_dsp_stream,
    ) -> c_int;

    /*
     * DSP PCM Operations.
     */
    pub fn acp_pcm_open(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
    pub fn acp_pcm_close(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
    pub fn acp_pcm_hw_params(
        sdev: *mut snd_sof_dev,
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        platform_params: *mut snd_sof_platform_stream_params,
    ) -> c_int;
    pub fn acp_pcm_pointer(
        sdev: *mut snd_sof_dev,
        substream: *mut snd_pcm_substream,
    ) -> snd_pcm_uframes_t;

    pub static sof_acp_common_ops: snd_sof_dsp_ops;

    pub static mut sof_renoir_ops: snd_sof_dsp_ops;
    pub fn sof_renoir_ops_init(sdev: *mut snd_sof_dev) -> c_int;
    pub static mut sof_vangogh_ops: snd_sof_dsp_ops;
    pub fn sof_vangogh_ops_init(sdev: *mut snd_sof_dev) -> c_int;
    pub static mut sof_rembrandt_ops: snd_sof_dsp_ops;
    pub fn sof_rembrandt_ops_init(sdev: *mut snd_sof_dev) -> c_int;
    pub static mut sof_acp63_ops: snd_sof_dsp_ops;
    pub fn sof_acp63_ops_init(sdev: *mut snd_sof_dev) -> c_int;

    pub static mut sof_acp70_ops: snd_sof_dsp_ops;
    pub fn sof_acp70_ops_init(sdev: *mut snd_sof_dev) -> c_int;

    pub static mut sof_acp7x_ops: snd_sof_dsp_ops;
    pub fn sof_acp7x_ops_init(sdev: *mut snd_sof_dev) -> c_int;

    pub fn amd_sof_acp7x_probe(sdev: *mut snd_sof_dev) -> c_int;
    pub fn amd_sof_acp7x_remove(sdev: *mut snd_sof_dev);
    pub fn amd_sof_acp7x_suspend(sdev: *mut snd_sof_dev, target_state: u32) -> c_int;
    pub fn amd_sof_acp7x_resume(sdev: *mut snd_sof_dev) -> c_int;
    pub fn amd_sof_acp7x_suspend_runtime(sdev: *mut snd_sof_dev) -> c_int;
    pub fn amd_sof_acp7x_resume_runtime(sdev: *mut snd_sof_dev) -> c_int;

    pub fn amd_sof_machine_select(sdev: *mut snd_sof_dev) -> *mut snd_soc_acpi_mach;
    /* Machine configuration */
    pub fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_int;

    /* Trace */
    pub fn acp_sof_trace_init(
        sdev: *mut snd_sof_dev,
        dmab: *mut snd_dma_buffer,
        dtrace_params: *mut sof_ipc_dma_trace_params_ext,
    ) -> c_int;
    pub fn acp_sof_trace_release(sdev: *mut snd_sof_dev) -> c_int;

    /* PM Callbacks */
    pub fn amd_sof_acp_suspend(sdev: *mut snd_sof_dev, target_state: u32) -> c_int;
    pub fn amd_sof_acp_resume(sdev: *mut snd_sof_dev) -> c_int;

    pub fn amd_sof_ipc_dump(sdev: *mut snd_sof_dev);
    pub fn amd_sof_dump(sdev: *mut snd_sof_dev, flags: u32);

    pub fn acp_probes_register(sdev: *mut snd_sof_dev) -> c_int;
    pub fn acp_probes_unregister(sdev: *mut snd_sof_dev);

    pub static mut snd_soc_acpi_amd_vangogh_sof_machines: [snd_soc_acpi_mach; 0];
    pub static acp_sof_quirk_table: [dmi_system_id; 0];
}

pub unsafe fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_amd_acp_desc {
    let desc: *const sof_dev_desc = unsafe { (*pdata).desc };

    unsafe { (*desc).chip_info }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
