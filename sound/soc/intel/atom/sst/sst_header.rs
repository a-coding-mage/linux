/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  sst.h - Intel SST Driver for audio engine
 *
 *  Copyright (C) 2008-14 Intel Corporation
 *  Authors:	Vinod Koul <vinod.koul@intel.com>
 *		Harsha Priya <priya.harsha@intel.com>
 *		Dharageswari R <dharageswari.r@intel.com>
 *		KP Jeeja <jeeja.kp@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 *  Common private declarations for SST
 */

/* C header dependency: <linux/firmware.h> */

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};

pub type u8 = u8;
pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;
pub type size_t = usize;
pub type phys_addr_t = usize;
pub type irqreturn_t = c_uint;
pub type wait_queue_head_t = c_void;
pub type spinlock_t = c_void;
pub type atomic_t = c_int;

/* External dependency types supplied by other headers. */
#[repr(C)]
pub struct dev_pm_ops {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_sst_alloc_mrfld {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_sst_runtime_params {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct scatterlist {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sst_platform_info {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct work_struct {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct workqueue_struct {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct pci_dev {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sst_info {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct pm_qos_request {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_sst_fw_version {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct ipc_post {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_sst_bytes_v2 {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_sst_params {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_sst_lib_download {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct firmware {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct ipc_header_mrfld {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct ipc_dsp_hdr {
    _unused: [u8; 0],
}

/* driver names */
pub const SST_DRV_NAME: &[u8; 17] = b"intel_sst_driver";

pub const SST_SUSPEND_DELAY: c_int = 2000;
pub const FW_CONTEXT_MEM: c_int = 64 * 1024;
pub const SST_ICCM_BOUNDARY: c_int = 4;
pub const SST_CONFIG_SSP_SIGN: u32 = 0x7ffe8001;

pub const MRFLD_FW_VIRTUAL_BASE: u32 = 0xC0000000;
pub const MRFLD_FW_DDR_BASE_OFFSET: c_int = 0x0;
pub const MRFLD_FW_FEATURE_BASE_OFFSET: c_int = 0x4;
pub const MRFLD_FW_BSS_RESET_BIT: c_int = 0;

/* SST Shim register map */
pub const SST_CSR: c_int = 0x00;
pub const SST_ISRX: c_int = 0x18;
pub const SST_IMRX: c_int = 0x28;
pub const SST_IPCX: c_int = 0x38; /* IPC IA -> SST */
pub const SST_IPCD: c_int = 0x40; /* IPC SST -> IA */

unsafe extern "C" {
    pub static intel_sst_pm: dev_pm_ops;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_states {
    SST_FW_LOADING = 1,
    SST_FW_RUNNING = 2,
    SST_RESET = 3,
    SST_SHUTDOWN = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_algo_ops {
    SST_SET_ALGO = 0,
    SST_GET_ALGO = 1,
}

pub const SST_BLOCK_TIMEOUT: c_int = 1000;

pub const FW_SIGNATURE_SIZE: usize = 4;
pub const FW_NAME_SIZE: usize = 32;

/* stream states */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_stream_states {
    STREAM_UN_INIT = 0, /* Freed/Not used stream */
    STREAM_RUNNING = 1, /* Running */
    STREAM_PAUSED = 2,  /* Paused stream */
    STREAM_INIT = 3,    /* stream init, waiting for data */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_ram_type {
    SST_IRAM = 1,
    SST_DRAM = 2,
    SST_DDR = 5,
    SST_CUSTOM_INFO = 7, /* consists of FW binary information */
}

/* SST shim registers to structure mapping.
 * C bitfield layout is target/compiler-defined; use full register storage.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub union interrupt_reg {
    pub full: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sst_pisr_reg {
    pub full: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sst_pimr_reg {
    pub full: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union config_status_reg_mrfld {
    pub full: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union interrupt_reg_mrfld {
    pub full: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sst_imr_reg_mrfld {
    pub full: u64,
}

/**
 * struct sst_block - This structure is used to block a user/fw data call to another
 * fw/user call
 *
 * @condition: condition for blocking check
 * @ret_code: ret code when block is released
 * @data: data ptr
 * @size: size of data
 * @on: block condition
 * @msg_id: msg_id = msgid in mfld/ctp, mrfld = NULL
 * @drv_id: str_id in mfld/ctp, = drv_id in mrfld
 * @node: list head node
 */
#[repr(C)]
pub struct sst_block {
    pub condition: bool,
    pub ret_code: c_int,
    pub data: *mut c_void,
    pub size: u32,
    pub on: bool,
    pub msg_id: u32,
    pub drv_id: u32,
    pub node: list_head,
}

/**
 * struct stream_info - structure that holds the stream information
 *
 * @status : stream current state
 * @prev : stream prev state
 * @resume_status : stream current state to restore on resume
 * @resume_prev : stream prev state to restore on resume
 * @lock : stream mutex for protecting state
 * @alloc_param : parameters used for stream (re-)allocation
 * @pcm_substream : PCM substream
 * @period_elapsed : PCM period elapsed callback
 * @sfreq : stream sampling freq
 * @cumm_bytes : cummulative bytes decoded
 */
#[repr(C)]
pub struct stream_info {
    pub status: c_uint,
    pub prev: c_uint,
    pub resume_status: c_uint,
    pub resume_prev: c_uint,
    pub lock: mutex,
    pub alloc_param: snd_sst_alloc_mrfld,
    pub pcm_substream: *mut c_void,
    pub period_elapsed: Option<unsafe extern "C" fn(pcm_substream: *mut c_void)>,
    pub sfreq: c_uint,
    pub cumm_bytes: u32,
    pub compr_cb_param: *mut c_void,
    pub compr_cb: Option<unsafe extern "C" fn(compr_cb_param: *mut c_void)>,
    pub drain_cb_param: *mut c_void,
    pub drain_notify: Option<unsafe extern "C" fn(drain_cb_param: *mut c_void)>,
    pub num_ch: c_uint,
    pub pipe_id: c_uint,
    pub task_id: c_uint,
}

pub const SST_FW_SIGN: &[u8; 4] = b"$SST";
pub const SST_FW_LIB_SIGN: &[u8; 4] = b"$LIB";

/**
 * struct sst_fw_header - FW file headers
 *
 * @signature : FW signature
 * @file_size: size of fw image
 * @modules : # of modules
 * @file_format : version of header format
 * @reserved : reserved fields
 */
#[repr(C)]
pub struct sst_fw_header {
    pub signature: [c_uchar; FW_SIGNATURE_SIZE],
    pub file_size: u32,
    pub modules: u32,
    pub file_format: u32,
    pub reserved: [u32; 4],
}

/**
 * struct fw_module_header - module header in FW
 *
 * @signature: module signature
 * @mod_size: size of module
 * @blocks: block count
 * @type: block type
 * @entry_point: module netry point
 */
#[repr(C)]
pub struct fw_module_header {
    pub signature: [c_uchar; FW_SIGNATURE_SIZE],
    pub mod_size: u32,
    pub blocks: u32,
    pub type_: u32,
    pub entry_point: u32,
}

/**
 * struct fw_block_info - block header for FW
 *
 * @type: block ram type I/D
 * @size: size of block
 * @ram_offset: offset in ram
 */
#[repr(C)]
pub struct fw_block_info {
    pub type_: sst_ram_type,
    pub size: u32,
    pub ram_offset: u32,
    pub rsvd: u32,
}

#[repr(C)]
pub struct sst_runtime_param {
    pub param: snd_sst_runtime_params,
}

#[repr(C)]
pub struct sst_sg_list {
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
    pub list_len: c_int,
    pub sg_idx: c_uint,
}

#[repr(C)]
pub struct sst_memcpy_list {
    pub memcpylist: list_head,
    pub dstn: *mut c_void,
    pub src: *const c_void,
    pub size: u32,
    pub is_io: bool,
}

/*Firmware Module Information*/
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_lib_dwnld_status {
    SST_LIB_NOT_FOUND = 0,
    SST_LIB_FOUND = 1,
    SST_LIB_DOWNLOADED = 2,
}

#[repr(C)]
pub struct sst_module_info {
    pub name: *const c_char, /*Library name*/
    pub id: u32,            /*Module ID*/
    pub entry_pt: u32,      /*Module entry point*/
    pub status: u8,         /*module status*/
    pub rsvd1: u8,
    pub rsvd2: u16,
}

/*
 * Structure for managing the Library Region(1.5MB)
 * in DDR in Merrifield
 */
#[repr(C)]
pub struct sst_mem_mgr {
    pub current_base: phys_addr_t,
    pub avail: c_int,
    pub count: c_uint,
}

#[repr(C)]
pub struct sst_ipc_reg {
    pub ipcx: c_int,
    pub ipcd: c_int,
}

#[repr(C)]
pub struct sst_fw_save {
    pub iram: *mut c_void, /* allocated via kvmalloc() */
    pub dram: *mut c_void, /* allocated via kvmalloc() */
    pub sram: *mut c_void, /* allocated via kvmalloc() */
    pub ddr: *mut c_void,  /* allocated via kvmalloc() */
}

/* External dependency constant supplied by other headers. */
pub const MAX_NUM_STREAMS: usize = 0;

/**
 * struct intel_sst_drv - driver ops
 *
 * @sst_state : current sst device state
 * @dev_id : device identifier, pci_id for pci devices and acpi_id for acpi
 * 	     devices
 * @shim : SST shim pointer
 * @mailbox : SST mailbox pointer
 * @iram : SST IRAM pointer
 * @dram : SST DRAM pointer
 * @pdata : SST info passed as a part of pci platform data
 * @shim_phy_add : SST shim phy addr
 * @ipc_dispatch_list : ipc messages dispatched
 * @rx_list : to copy the process_reply/process_msg from DSP
 * @ipc_post_msg_wq : wq to post IPC messages context
 * @mad_ops : MAD driver operations registered
 * @mad_wq : MAD driver wq
 * @post_msg_wq : wq to post IPC messages
 * @streams : sst stream contexts
 * @list_lock : sst driver list lock (deprecated)
 * @ipc_spin_lock : spin lock to handle audio shim access and ipc queue
 * @block_lock : spin lock to add block to block_list and assign pvt_id
 * @rx_msg_lock : spin lock to handle the rx messages from the DSP
 * @scard_ops : sst card ops
 * @pci : sst pci device struture
 * @dev : pointer to current device struct
 * @sst_lock : sst device lock
 * @pvt_id : sst private id
 * @stream_cnt : total sst active stream count
 * @pb_streams : total active pb streams
 * @cp_streams : total active cp streams
 * @audio_start : audio status
 * @qos		: PM Qos struct
 * firmware_name : Firmware / Library name
 */
#[repr(C)]
pub struct intel_sst_drv {
    pub sst_state: c_int,
    pub irq_num: c_int,
    pub dev_id: u16,
    pub ddr: *mut c_void,
    pub shim: *mut c_void,
    pub mailbox: *mut c_void,
    pub iram: *mut c_void,
    pub dram: *mut c_void,
    pub mailbox_add: c_uint,
    pub iram_base: c_uint,
    pub dram_base: c_uint,
    pub shim_phy_add: c_uint,
    pub iram_end: c_uint,
    pub dram_end: c_uint,
    pub ddr_end: c_uint,
    pub ddr_base: c_uint,
    pub mailbox_recv_offset: c_uint,
    pub block_list: list_head,
    pub ipc_dispatch_list: list_head,
    pub pdata: *mut sst_platform_info,
    pub rx_list: list_head,
    pub ipc_post_msg_wq: work_struct,
    pub wait_queue: wait_queue_head_t,
    pub post_msg_wq: *mut workqueue_struct,
    pub tstamp: c_uint,
    /* str_id 0 is not used */
    pub streams: [stream_info; MAX_NUM_STREAMS + 1],
    pub ipc_spin_lock: spinlock_t,
    pub block_lock: spinlock_t,
    pub rx_msg_lock: spinlock_t,
    pub pci: *mut pci_dev,
    pub dev: *mut device,
    pub pvt_id: c_ulong,
    pub sst_lock: mutex,
    pub stream_cnt: c_uint,
    pub csr_value: c_uint,
    pub fw_in_mem: *mut c_void,
    pub fw_sg_list: sst_sg_list,
    pub library_list: sst_sg_list,
    pub ops: *mut intel_sst_ops,
    pub info: sst_info,
    pub qos: *mut pm_qos_request,
    pub use_dma: c_uint,
    pub use_lli: c_uint,
    pub fw_clear_context: atomic_t,
    pub lib_dwnld_reqd: bool,
    pub memcpy_list: list_head,
    pub ipc_reg: sst_ipc_reg,
    pub lib_mem_mgr: sst_mem_mgr,
    /*
     * Holder for firmware name. Due to async call it needs to be
     * persistent till worker thread gets called
     */
    pub firmware_name: [c_char; FW_NAME_SIZE],
    pub fw_version: snd_sst_fw_version,
    pub fw_save: *mut sst_fw_save,
}

/* misc definitions */
pub const FW_DWNL_ID: c_int = 0x01;

#[repr(C)]
pub struct intel_sst_ops {
    pub interrupt: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
    pub irq_thread: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
    pub clear_interrupt: Option<unsafe extern "C" fn(ctx: *mut intel_sst_drv)>,
    pub start: Option<unsafe extern "C" fn(ctx: *mut intel_sst_drv) -> c_int>,
    pub reset: Option<unsafe extern "C" fn(ctx: *mut intel_sst_drv) -> c_int>,
    pub process_reply: Option<unsafe extern "C" fn(ctx: *mut intel_sst_drv, msg: *mut ipc_post)>,
    pub post_message:
        Option<unsafe extern "C" fn(ctx: *mut intel_sst_drv, msg: *mut ipc_post, sync: bool) -> c_int>,
    pub process_message: Option<unsafe extern "C" fn(msg: *mut ipc_post)>,
    pub set_bypass: Option<unsafe extern "C" fn(set: bool)>,
    pub save_dsp_context: Option<unsafe extern "C" fn(sst: *mut intel_sst_drv) -> c_int>,
    pub restore_dsp_context: Option<unsafe extern "C" fn()>,
    pub alloc_stream: Option<unsafe extern "C" fn(ctx: *mut intel_sst_drv, params: *mut c_void) -> c_int>,
    pub post_download: Option<unsafe extern "C" fn(sst: *mut intel_sst_drv)>,
}

unsafe extern "C" {
    pub fn sst_realloc_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
    pub fn sst_pause_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
    pub fn sst_resume_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
    pub fn sst_drop_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
    pub fn sst_free_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
    pub fn sst_start_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
    pub fn sst_send_byte_stream_mrfld(
        sst_drv_ctx: *mut intel_sst_drv,
        bytes: *mut snd_sst_bytes_v2,
    ) -> c_int;
    pub fn sst_set_stream_param(str_id: c_int, str_param: *mut snd_sst_params) -> c_int;
    pub fn sst_set_metadata(str_id: c_int, params: *mut c_char) -> c_int;
    pub fn sst_get_stream(ctx: *mut intel_sst_drv, str_param: *mut snd_sst_params) -> c_int;
    pub fn sst_drain_stream(
        sst_drv_ctx: *mut intel_sst_drv,
        str_id: c_int,
        partial_drain: bool,
    ) -> c_int;
    pub fn sst_post_message_mrfld(
        sst_drv_ctx: *mut intel_sst_drv,
        ipc_msg: *mut ipc_post,
        sync: bool,
    ) -> c_int;
    pub fn sst_process_reply_mrfld(sst_drv_ctx: *mut intel_sst_drv, msg: *mut ipc_post);
    pub fn sst_start_mrfld(sst_drv_ctx: *mut intel_sst_drv) -> c_int;
    pub fn intel_sst_reset_dsp_mrfld(sst_drv_ctx: *mut intel_sst_drv) -> c_int;
    pub fn intel_sst_clear_intr_mrfld(sst_drv_ctx: *mut intel_sst_drv);

    pub fn sst_load_fw(sst_drv_ctx: *mut intel_sst_drv) -> c_int;
    pub fn sst_load_library(lib: *mut snd_sst_lib_download, ops: u8) -> c_int;
    pub fn sst_post_download_mrfld(ctx: *mut intel_sst_drv);
    pub fn sst_get_block_stream(sst_drv_ctx: *mut intel_sst_drv) -> c_int;
    pub fn sst_memcpy_free_resources(sst_drv_ctx: *mut intel_sst_drv);

    pub fn sst_wait_timeout(sst_drv_ctx: *mut intel_sst_drv, block: *mut sst_block) -> c_int;
    pub fn sst_create_ipc_msg(arg: *mut *mut ipc_post, large: bool) -> c_int;
    pub fn free_stream_context(ctx: *mut intel_sst_drv, str_id: c_uint) -> c_int;
    pub fn sst_clean_stream(stream: *mut stream_info);
    pub fn intel_sst_register_compress(sst: *mut intel_sst_drv) -> c_int;
    pub fn intel_sst_remove_compress(sst: *mut intel_sst_drv) -> c_int;
    pub fn sst_send_sync_msg(ipc: c_int, str_id: c_int) -> c_int;
    pub fn sst_get_num_channel(str_param: *mut snd_sst_params) -> c_int;
    pub fn sst_get_sfreq(str_param: *mut snd_sst_params) -> c_int;
    pub fn sst_alloc_stream_mrfld(sst_drv_ctx: *mut intel_sst_drv, params: *mut c_void) -> c_int;
    pub fn sst_restore_fw_context();
    pub fn sst_create_block(
        ctx: *mut intel_sst_drv,
        msg_id: u32,
        drv_id: u32,
    ) -> *mut sst_block;
    pub fn sst_create_block_and_ipc_msg(
        arg: *mut *mut ipc_post,
        large: bool,
        sst_drv_ctx: *mut intel_sst_drv,
        block: *mut *mut sst_block,
        msg_id: u32,
        drv_id: u32,
    ) -> c_int;
    pub fn sst_free_block(ctx: *mut intel_sst_drv, freed: *mut sst_block) -> c_int;
    pub fn sst_wake_up_block(
        ctx: *mut intel_sst_drv,
        result: c_int,
        drv_id: u32,
        ipc: u32,
        data: *mut c_void,
        size: u32,
    ) -> c_int;
    pub fn sst_request_firmware_async(ctx: *mut intel_sst_drv) -> c_int;
    pub fn sst_driver_ops(sst: *mut intel_sst_drv) -> c_int;
    pub fn sst_get_acpi_driver_data(hid: *const c_char) -> *mut sst_platform_info;
    pub fn sst_firmware_load_cb(fw: *const firmware, context: *mut c_void);
    pub fn sst_prepare_and_post_msg(
        sst: *mut intel_sst_drv,
        task_id: c_int,
        ipc_msg: c_int,
        cmd_id: c_int,
        pipe_id: c_int,
        mbox_data_len: size_t,
        mbox_data: *const c_void,
        data: *mut *mut c_void,
        large: bool,
        fill_dsp: bool,
        sync: bool,
        response: bool,
    ) -> c_int;

    pub fn sst_process_pending_msg(work: *mut work_struct);
    pub fn sst_assign_pvt_id(drv: *mut intel_sst_drv) -> c_int;
    pub fn sst_validate_strid(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
    pub fn get_stream_info(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> *mut stream_info;
    pub fn get_stream_id_mrfld(sst_drv_ctx: *mut intel_sst_drv, pipe_id: u32) -> c_int;
    pub fn relocate_imr_addr_mrfld(base_addr: u32) -> u32;
    pub fn sst_add_to_dispatch_list_and_post(sst: *mut intel_sst_drv, msg: *mut ipc_post);
    pub fn sst_pm_runtime_put(sst_drv: *mut intel_sst_drv) -> c_int;
    pub fn sst_shim_write(addr: *mut c_void, offset: c_int, value: c_int) -> c_int;
    pub fn sst_shim_read(addr: *mut c_void, offset: c_int) -> u32;
    pub fn sst_reg_read64(addr: *mut c_void, offset: c_int) -> u64;
    pub fn sst_shim_write64(addr: *mut c_void, offset: c_int, value: u64) -> c_int;
    pub fn sst_shim_read64(addr: *mut c_void, offset: c_int) -> u64;
    pub fn sst_set_fw_state_locked(sst_drv_ctx: *mut intel_sst_drv, sst_state: c_int);
    pub fn sst_fill_header_mrfld(
        header: *mut ipc_header_mrfld,
        msg: c_int,
        task_id: c_int,
        large: c_int,
        drv_id: c_int,
    );
    pub fn sst_fill_header_dsp(dsp: *mut ipc_dsp_hdr, msg: c_int, pipe_id: c_int, len: c_int);

    pub fn sst_register(arg1: *mut device) -> c_int;
    pub fn sst_unregister(arg1: *mut device) -> c_int;

    pub fn sst_alloc_drv_context(
        ctx: *mut *mut intel_sst_drv,
        dev: *mut device,
        dev_id: u16,
    ) -> c_int;
    pub fn sst_context_init(ctx: *mut intel_sst_drv) -> c_int;
    pub fn sst_context_cleanup(ctx: *mut intel_sst_drv);
    pub fn sst_configure_runtime_pm(ctx: *mut intel_sst_drv);
    pub fn memcpy32_toio(dst: *mut c_void, src: *const c_void, count: c_int);
    pub fn memcpy32_fromio(dst: *mut c_void, src: *const c_void, count: c_int);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
