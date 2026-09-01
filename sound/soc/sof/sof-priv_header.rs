/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 *
 * Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type u8 = u8;
pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;
pub type size_t = usize;
pub type bool_ = bool;

/* External kernel and SOF dependency types supplied by other files. */
#[repr(C)]
pub struct snd_sof_pcm_stream {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct firmware {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_codec {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub ipc_lock: spinlock_t,
    pub hw_lock: spinlock_t,

    /*
     * When true the DSP is not used.
     * It is set under the following condition:
     * User sets the SOF_DBG_DSPLESS_MODE flag in sof_debug module parameter
     * and
     * the platform advertises that it can support such mode
     * pdata->desc->dspless_mode_supported is true.
     */
    pub dspless_mode_selected: bool,

    /* Main, Base firmware image */
    pub basefw: sof_firmware,

    /*
     * ASoC components. plat_drv fields are set dynamically so
     * can't use const
     */
    pub plat_drv: snd_soc_component_driver,

    /* current DSP power state */
    pub dsp_power_state: sof_dsp_power_state,
    /* mutex to protect the dsp_power_state access */
    pub power_state_access: mutex,

    /* Intended power target of system suspend */
    pub system_suspend_target: sof_system_suspend_state,

    /* DSP firmware boot */
    pub boot_wait: wait_queue_head_t,
    pub fw_state: sof_fw_state,
    pub first_boot: bool,
    /* mutex to protect DSP firmware boot (except initial, probe time boot */
    pub dsp_fw_boot_mutex: mutex,

    /* work queue in case the probe is implemented in two steps */
    pub probe_work: work_struct,
    pub probe_completed: bool,

    /* DSP HW differentiation */
    pub pdata: *mut snd_sof_pdata,

    /* IPC */
    pub ipc: *mut snd_sof_ipc,
    pub fw_info_box: snd_sof_mailbox, /* FW shared memory */
    pub dsp_box: snd_sof_mailbox,     /* DSP initiated IPC */
    pub host_box: snd_sof_mailbox,    /* Host initiated IPC */
    pub stream_box: snd_sof_mailbox,  /* Stream position update */
    pub debug_box: snd_sof_mailbox,   /* Debug info updates */
    pub msg: *mut snd_sof_ipc_msg,
    pub ipc_irq: c_int,
    pub next_comp_id: u32, /* monotonic - reset during S3 */

    /* memory bases for mmaped DSPs - set by dsp_init() */
    pub bar: [*mut c_void; SND_SOF_BARS], /* DSP base address */
    pub mmio_bar: c_int,
    pub mailbox_bar: c_int,
    pub dsp_oops_offset: size_t,

    /* debug */
    pub debugfs_root: *mut dentry,
    pub dfsentry_list: list_head,
    pub dbg_dump_printed: bool,
    pub ipc_dump_printed: bool,
    pub d3_prevented: bool, /* runtime pm use count incremented to prevent context lost */

    /* firmware loader */
    pub fw_ready: sof_ipc_fw_ready,
    pub fw_version: sof_ipc_fw_version,
    pub cc_version: *mut sof_ipc_cc_version,

    /* topology */
    pub tplg_ops: *mut snd_soc_tplg_ops,
    pub pcm_list: list_head,
    pub kcontrol_list: list_head,
    pub widget_list: list_head,
    pub pipeline_list: list_head,
    pub dai_list: list_head,
    pub dai_link_list: list_head,
    pub route_list: list_head,
    pub component: *mut snd_soc_component,
    pub enabled_cores_mask: u32, /* keep track of enabled cores */
    pub led_present: bool,

    /* FW configuration */
    pub info_window: *mut sof_ipc_window,

    /* IPC timeouts in ms */
    pub ipc_timeout: c_int,
    pub boot_timeout: c_int,

    /* firmwre tracing */
    pub fw_trace_is_supported: bool, /* set with Kconfig or module parameter */
    pub fw_trace_data: *mut c_void,  /* private data used by firmware tracing implementation */

    pub msi_enabled: bool,

    /* DSP core context */
    pub num_cores: u32,

    /*
     * ref count per core that will be modified during system suspend/resume and during pcm
     * hw_params/hw_free. This doesn't need to be protected with a mutex because pcm
     * hw_params/hw_free are already protected by the PCM mutex in the ALSA framework in
     * sound/core/ when streams are active and during system suspend/resume, streams are
     * already suspended.
     */
    pub dsp_core_ref_count: [c_int; SOF_MAX_DSP_NUM_CORES],

    /*
     * Used to keep track of registered IPC client devices so that they can
     * be removed when the parent SOF module is removed.
     */
    pub ipc_client_list: list_head,

    /* mutex to protect client list */
    pub ipc_client_mutex: mutex,

    /*
     * Used for tracking the IPC client's RX registration for DSP initiated
     * message handling.
     */
    pub ipc_rx_handler_list: list_head,

    /*
     * Used for tracking the IPC client's registration for DSP state change
     * notification
     */
    pub fw_state_handler_list: list_head,

    /* to protect the ipc_rx_handler_list  and  dsp_state_handler_list list */
    pub client_event_handler_mutex: mutex,

    /* quirks to override topology values */
    pub mclk_id_override: bool,
    pub mclk_id_quirk: u16, /* same size as in IPC3 definitions */

    pub private: *mut c_void, /* core does not touch this */
}
#[repr(C)]
pub struct snd_sof_ipc_msg {
    /* message data */
    pub msg_data: *mut c_void,
    pub reply_data: *mut c_void,
    pub msg_size: size_t,
    pub reply_size: size_t,
    pub reply_error: c_int,

    pub ipc_complete: bool,

    pub waitq: wait_queue_head_t,

    /* notification, firmware initiated messages */
    pub rx_data: *mut c_void,
}
#[repr(C)]
pub struct snd_sof_ipc {
    pub sdev: *mut snd_sof_dev,

    /* protects messages and the disable flag */
    pub tx_mutex: mutex,
    /* disables further sending of ipc's */
    pub disable_ipc_tx: bool,

    /* Maximum allowed size of a single IPC message/reply */
    pub max_payload_size: size_t,

    pub msg: snd_sof_ipc_msg,

    /* IPC ops based on version */
    pub ops: *const sof_ipc_ops,
}
#[repr(C)]
pub struct snd_sof_debugfs_map {
    pub name: *const c_char,
    pub bar: u32,
    pub offset: u32,
    pub size: u32,
    /*
     * access_type specifies if the memory is always accessible
     * or if it is accessible only when the DSP is in D0.
     */
    pub access_type: sof_debugfs_access_type,
}
#[repr(C)]
pub struct snd_soc_tplg_ops {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_sof_pdata {
    pub desc: *mut snd_sof_desc,
}
#[repr(C)]
pub struct snd_sof_desc {
    pub ops: *mut snd_sof_dsp_ops,
}
#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_sof_mod_hdr {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sof_ext_man_elem_header {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_dma_buffer {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sof_ipc_dma_trace_params_ext {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_acpi_mach {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct dsp_arch_ops {
    pub dsp_oops:
        Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, level: *const c_char, oops: *mut c_void)>,
    pub dsp_stack: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            level: *const c_char,
            oops: *mut c_void,
            stack: *mut u32,
            stack_words: u32,
        ),
    >,
}
#[repr(C)]
pub struct list_head {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct dentry {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sof_ipc_fw_ready {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sof_ipc_fw_version {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sof_ipc_cc_version {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sof_ipc_window {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component_driver {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct work_struct {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct wait_queue_head_t {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sof_loadable_file_profile {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_compress_ops {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sof_ipc_panic_info {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sof_client_dev {
    _unused: [u8; 0],
}

pub type irqreturn_t = c_int;
pub type snd_pcm_uframes_t = usize;
pub type pm_message_t = c_int;
pub type mode_t = c_uint;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum snd_sof_fw_blk_type {
    _Unused = 0,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum sof_fw_state {
    _Unused = 0,
}

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

/* Flag definitions used in sof_core_debug (sof_debug module parameter) */
pub const SOF_DBG_ENABLE_TRACE: u32 = BIT(0);
pub const SOF_DBG_RETAIN_CTX: u32 = BIT(1); /* prevent DSP D3 on FW exception */
pub const SOF_DBG_VERIFY_TPLG: u32 = BIT(2); /* verify topology during load */
pub const SOF_DBG_DYNAMIC_PIPELINES_OVERRIDE: u32 = BIT(3); /* 0: use topology token
                                                            * 1: override topology
                                                            */
pub const SOF_DBG_DYNAMIC_PIPELINES_ENABLE: u32 = BIT(4); /* 0: use static pipelines
                                                          * 1: use dynamic pipelines
                                                          */
pub const SOF_DBG_DISABLE_MULTICORE: u32 = BIT(5); /* schedule all pipelines/widgets
                                                   * on primary core
                                                   */
pub const SOF_DBG_PRINT_ALL_DUMPS: u32 = BIT(6); /* Print all ipc and dsp dumps */
pub const SOF_DBG_IGNORE_D3_PERSISTENT: u32 = BIT(7); /* ignore the DSP D3 persistent capability
                                                      * and always download firmware upon D3 exit
                                                      */
pub const SOF_DBG_PRINT_DMA_POSITION_UPDATE_LOGS: u32 = BIT(8); /* print DMA position updates
                                                                * in dmesg logs
                                                                */
pub const SOF_DBG_PRINT_IPC_SUCCESS_LOGS: u32 = BIT(9); /* print IPC success
                                                        * in dmesg logs
                                                        */
pub const SOF_DBG_FORCE_NOCODEC: u32 = BIT(10); /* ignore all codec-related
                                                * configurations
                                                */
pub const SOF_DBG_DUMP_IPC_MESSAGE_PAYLOAD: u32 = BIT(11); /* On top of the IPC message header
                                                           * dump the message payload also
                                                           */
pub const SOF_DBG_DSPLESS_MODE: u32 = BIT(15); /* Do not initialize and use the DSP */

/* Flag definitions used for controlling the DSP dump behavior */
pub const SOF_DBG_DUMP_REGS: u32 = BIT(0);
pub const SOF_DBG_DUMP_MBOX: u32 = BIT(1);
pub const SOF_DBG_DUMP_TEXT: u32 = BIT(2);
pub const SOF_DBG_DUMP_PCI: u32 = BIT(3);
/* Output this dump (at the DEBUG level) only when SOF_DBG_PRINT_ALL_DUMPS is set */
pub const SOF_DBG_DUMP_OPTIONAL: u32 = BIT(4);

/* max BARs mmaped devices can use */
pub const SND_SOF_BARS: usize = 8;

/* time in ms for runtime suspend delay */
pub const SND_SOF_SUSPEND_DELAY_MS: u32 = 2000;

/* DMA buffer size for trace */
pub const DMA_BUF_SIZE_FOR_TRACE: usize = PAGE_SIZE * 16;

pub const SOF_IPC_DSP_REPLY: u32 = 0;
pub const SOF_IPC_HOST_REPLY: u32 = 1;

/* So far the primary core on all DSPs has ID 0 */
pub const SOF_DSP_PRIMARY_CORE: u32 = 0;

/* max number of DSP cores */
pub const SOF_MAX_DSP_NUM_CORES: usize = 8;

extern "C" {
    pub static PAGE_SIZE: usize;
}

#[repr(C)]
pub struct sof_dsp_power_state {
    pub state: u32,
    pub substate: u32, /* platform-specific */
}

/* System suspend target state */
#[repr(C)]
#[derive(Clone, Copy)]
pub enum sof_system_suspend_state {
    SOF_SUSPEND_NONE = 0,
    SOF_SUSPEND_S0IX,
    SOF_SUSPEND_S3,
    SOF_SUSPEND_S4,
    SOF_SUSPEND_S5,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum sof_dfsentry_type {
    SOF_DFSENTRY_TYPE_IOMEM = 0,
    SOF_DFSENTRY_TYPE_BUF,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum sof_debugfs_access_type {
    SOF_DEBUGFS_ACCESS_ALWAYS = 0,
    SOF_DEBUGFS_ACCESS_D0_ONLY,
}

#[repr(C)]
pub struct sof_compr_stream {
    pub copied_total: u64,
    pub sampling_rate: u32,
    pub channels: u16,
    pub sample_container_bytes: u16,
    pub codec_params: snd_codec,
    pub posn_offset: size_t,
}

/**
 * struct snd_sof_platform_stream_params - platform dependent stream parameters
 * @phy_addr:		Platform dependent address to be used, if  @use_phy_addr
 *			is true
 * @stream_tag:		Stream tag to use
 * @use_phy_addr:	Use the provided @phy_addr for configuration
 * @no_ipc_position:	Disable position update IPC from firmware
 * @cont_update_posn:	Continuous position update.
 */
#[repr(C)]
pub struct snd_sof_platform_stream_params {
    pub phy_addr: u32,
    pub stream_tag: u16,
    pub use_phy_address: bool,
    pub no_ipc_position: bool,
    pub cont_update_posn: bool,
}

/**
 * struct sof_firmware - Container struct for SOF firmware
 * @fw:			Pointer to the firmware
 * @payload_offset:	Offset of the data within the loaded firmware image to be
 *			loaded to the DSP (skipping for example ext_manifest section)
 */
#[repr(C)]
pub struct sof_firmware {
    pub fw: *const firmware,
    pub payload_offset: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum sof_dai_access {
    SOF_DAI_DSP_ACCESS,  /* access from DSP only */
    SOF_DAI_HOST_ACCESS, /* access from host only */

    SOF_DAI_ACCESS_NUM,
}

/*
 * SOF DSP HW abstraction operations.
 * Used to abstract DSP HW architecture and any IO busses between host CPU
 * and DSP device(s).
 */
#[repr(C)]
pub struct snd_sof_dsp_ops {
    /* probe/remove/shutdown */
    pub probe_early: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev) -> c_int>, /* optional */
    pub probe: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev) -> c_int>,       /* mandatory */
    pub remove: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev)>,              /* optional */
    pub remove_late: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev)>,         /* optional */
    pub shutdown: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev) -> c_int>,   /* optional */

    /* DSP core boot / reset */
    pub run: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev) -> c_int>, /* mandatory */
    pub stall: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, core_mask: c_uint) -> c_int>, /* optional */
    pub reset: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev) -> c_int>, /* optional */
    pub core_get: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, core: c_int) -> c_int>, /* optional */
    pub core_put: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, core: c_int) -> c_int>, /* optional */

    /*
     * Register IO: only used by respective drivers themselves,
     * TODO: consider removing these operations and calling respective
     * implementations directly
     */
    pub write8: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, addr: *mut c_void, value: u8)>, /* optional */
    pub read8: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, addr: *mut c_void) -> u8>, /* optional */
    pub write: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, addr: *mut c_void, value: u32)>, /* optional */
    pub read: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, addr: *mut c_void) -> u32>, /* optional */
    pub write64: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, addr: *mut c_void, value: u64)>, /* optional */
    pub read64: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, addr: *mut c_void) -> u64>, /* optional */

    /* memcpy IO */
    pub block_read: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, type_: snd_sof_fw_blk_type, offset: u32, dest: *mut c_void, size: size_t) -> c_int>, /* mandatory */
    pub block_write: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, type_: snd_sof_fw_blk_type, offset: u32, src: *mut c_void, size: size_t) -> c_int>, /* mandatory */

    /* Mailbox IO */
    pub mailbox_read: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, offset: u32, dest: *mut c_void, size: size_t)>, /* optional */
    pub mailbox_write: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, offset: u32, src: *mut c_void, size: size_t)>, /* optional */

    /* doorbell */
    pub irq_handler: Option<unsafe extern "C" fn(irq: c_int, context: *mut c_void) -> irqreturn_t>, /* optional */
    pub irq_thread: Option<unsafe extern "C" fn(irq: c_int, context: *mut c_void) -> irqreturn_t>,  /* optional */

    /* ipc */
    pub send_msg: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int>, /* mandatory */

    /* FW loading */
    pub load_firmware: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev) -> c_int>, /* mandatory */
    pub load_module: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, hdr: *mut snd_sof_mod_hdr) -> c_int>, /* optional */

    /* connect pcm substream to a host stream */
    pub pcm_open: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int>, /* optional */
    /* disconnect pcm substream to a host stream */
    pub pcm_close: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int>, /* optional */

    /* host stream hw params */
    pub pcm_hw_params: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, platform_params: *mut snd_sof_platform_stream_params) -> c_int>, /* optional */

    /* host stream hw_free */
    pub pcm_hw_free: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int>, /* optional */

    /* host stream trigger */
    pub pcm_trigger: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream, cmd: c_int) -> c_int>, /* optional */

    /* host stream pointer */
    pub pcm_pointer: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t>, /* optional */

    /* pcm ack */
    pub pcm_ack: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int>, /* optional */

    /*
     * optional callback to retrieve the number of frames left/arrived from/to
     * the DSP on the DAI side (link/codec/DMIC/etc).
     *
     * The callback is used when the firmware does not provide this information
     * via the shared SRAM window and it can be retrieved by host.
     */
    pub get_dai_frame_counter: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> u64>, /* optional */

    /*
     * Optional callback to retrieve the number of bytes left/arrived from/to
     * the DSP on the host side (bytes between host ALSA buffer and DSP).
     *
     * The callback is needed for ALSA delay reporting.
     */
    pub get_host_byte_counter: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> u64>, /* optional */

    /* host read DSP stream data */
    pub ipc_msg_data: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, sps: *mut snd_sof_pcm_stream, p: *mut c_void, sz: size_t) -> c_int>, /* mandatory */

    /* host side configuration of the stream's data offset in stream mailbox area */
    pub set_stream_data_offset: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, sps: *mut snd_sof_pcm_stream, posn_offset: size_t) -> c_int>, /* optional */

    /* pre/post firmware run */
    pub pre_fw_run: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev) -> c_int>, /* optional */
    pub post_fw_run: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev) -> c_int>, /* optional */

    /* parse platform specific extended manifest, optional */
    pub parse_platform_ext_manifest: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, hdr: *const sof_ext_man_elem_header) -> c_int>,

    /* DSP PM */
    pub suspend: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, target_state: u32) -> c_int>, /* optional */
    pub resume: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev) -> c_int>, /* optional */
    pub runtime_suspend: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev) -> c_int>, /* optional */
    pub runtime_resume: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev) -> c_int>, /* optional */
    pub runtime_idle: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev) -> c_int>, /* optional */
    pub set_hw_params_upon_resume: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>, /* optional */
    pub set_power_state: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, target_state: *const sof_dsp_power_state) -> c_int>, /* optional */

    /* DSP clocking */
    pub set_clk: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, freq: u32) -> c_int>, /* optional */

    /* debug */
    pub debug_map: *const snd_sof_debugfs_map, /* optional */
    pub debug_map_count: c_int,                /* optional */
    pub dbg_dump: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev, flags: u32)>, /* optional */
    pub ipc_dump: Option<unsafe extern "C" fn(sof_dev: *mut snd_sof_dev)>, /* optional */
    pub debugfs_add_region_item: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, blk_type: snd_sof_fw_blk_type, offset: u32, size: size_t, name: *const c_char, access_type: sof_debugfs_access_type) -> c_int>, /* optional */

    /* host DMA trace (IPC3) */
    pub trace_init: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, dmatb: *mut snd_dma_buffer, dtrace_params: *mut sof_ipc_dma_trace_params_ext) -> c_int>, /* optional */
    pub trace_release: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>, /* optional */
    pub trace_trigger: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, cmd: c_int) -> c_int>, /* optional */

    /* misc */
    pub get_bar_index: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, type_: u32) -> c_int>, /* optional */
    pub get_mailbox_offset: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>, /* mandatory for common loader code */
    pub get_window_offset: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, id: u32) -> c_int>, /* mandatory for common loader code */

    /* machine driver ops */
    pub machine_register: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, pdata: *mut c_void) -> c_int>, /* optional */
    pub machine_unregister: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, pdata: *mut c_void)>, /* optional */
    pub machine_select: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> *mut snd_soc_acpi_mach>, /* optional */
    pub set_mach_params: Option<unsafe extern "C" fn(mach: *mut snd_soc_acpi_mach, sdev: *mut snd_sof_dev)>, /* optional */

    /* IPC client ops */
    pub register_ipc_clients: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>, /* optional */
    pub unregister_ipc_clients: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev)>, /* optional */

    /* DAI ops */
    pub drv: *mut snd_soc_dai_driver,
    pub num_drv: c_int,

    pub is_chain_dma_supported: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, dai_type: u32) -> bool>, /* optional */

    /* ALSA HW info flags, will be stored in snd_pcm_runtime.hw.info */
    pub hw_info: u32,

    pub dsp_arch_ops: *const dsp_arch_ops,
}

#[inline]
pub unsafe fn sof_dsp_arch_ops(sdev: *mut snd_sof_dev) -> *const dsp_arch_ops {
    (*(*(*(*sdev).pdata).desc).ops).dsp_arch_ops
}

/* FS entry for debug files that can expose DSP memories, registers */
#[repr(C)]
pub union snd_sof_dfsentry_union {
    pub io_mem: *mut c_void,
    pub buf: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_dfsentry {
    pub size: size_t,
    pub buf_data_size: size_t, /* length of buffered data for file read operation */
    pub type_: sof_dfsentry_type,
    /*
     * access_type specifies if the
     * memory -> DSP resource (memory, register etc) is always accessible
     * or if it is accessible only when the DSP is in D0.
     */
    pub access_type: sof_debugfs_access_type,
    /* CONFIG_SND_SOC_SOF_DEBUG_ENABLE_DEBUGFS_CACHE: char *cache_buf; */
    pub sdev: *mut snd_sof_dev,
    pub list: list_head, /* list in sdev dfsentry list */
    pub u: snd_sof_dfsentry_union,
}

/* mailbox descriptor, used for host <-> DSP IPC */
#[repr(C)]
pub struct snd_sof_mailbox {
    pub size: size_t,
    pub offset: u32,
}

/**
 * struct sof_ipc_fw_tracing_ops - IPC-specific firmware tracing ops
 * @init:	Function pointer for initialization of the tracing
 * @free:	Optional function pointer for freeing of the tracing
 * @fw_crashed:	Optional function pointer to notify the tracing of a firmware crash
 * @suspend:	Function pointer for system/runtime suspend
 * @resume:	Function pointer for system/runtime resume
 */
#[repr(C)]
pub struct sof_ipc_fw_tracing_ops {
    pub init: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>,
    pub free: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev)>,
    pub fw_crashed: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev)>,
    pub suspend: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, pm_state: pm_message_t)>,
    pub resume: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>,
}

/**
 * struct sof_ipc_pm_ops - IPC-specific PM ops
 * @ctx_save:		Optional function pointer for context save
 * @ctx_restore:	Optional function pointer for context restore
 * @set_core_state:	Optional function pointer for turning on/off a DSP core
 * @set_pm_gate:	Optional function pointer for pm gate settings
 */
#[repr(C)]
pub struct sof_ipc_pm_ops {
    pub ctx_save: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>,
    pub ctx_restore: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>,
    pub set_core_state: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, core_idx: c_int, on: bool) -> c_int>,
    pub set_pm_gate: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, flags: u32) -> c_int>,
}

/**
 * struct sof_ipc_fw_loader_ops - IPC/FW-specific loader ops
 */
#[repr(C)]
pub struct sof_ipc_fw_loader_ops {
    pub validate: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>,
    pub parse_ext_manifest: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> size_t>,
    pub load_fw_to_dsp: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>,
}

#[repr(C)]
pub struct sof_ipc_tplg_ops {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sof_ipc_pcm_ops {
    _unused: [u8; 0],
}

/**
 * struct sof_ipc_ops - IPC-specific ops
 */
#[repr(C)]
pub struct sof_ipc_ops {
    pub tplg: *const sof_ipc_tplg_ops,
    pub pm: *const sof_ipc_pm_ops,
    pub pcm: *const sof_ipc_pcm_ops,
    pub fw_loader: *const sof_ipc_fw_loader_ops,
    pub fw_tracing: *const sof_ipc_fw_tracing_ops,

    pub init: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev)>,
    pub post_fw_boot: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>,

    pub tx_msg: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, msg_data: *mut c_void, msg_bytes: size_t, reply_data: *mut c_void, reply_bytes: size_t, no_pm: bool) -> c_int>,
    pub set_get_data: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, data: *mut c_void, data_bytes: size_t, set: bool) -> c_int>,
    pub get_reply: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> c_int>,
    pub rx_msg: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev)>,
}

#[macro_export]
macro_rules! sof_ipc_get_ops {
    ($sdev:expr, $ops_name:ident) => {{
        let sdev = $sdev;
        if !(*sdev).ipc.is_null() && !(*(*sdev).ipc).ops.is_null() {
            (*(*(*sdev).ipc).ops).$ops_name
        } else {
            None
        }
    }};
}

extern "C" {
    /* global debug state set by SOF_DBG_ flags */
    pub fn sof_debug_check_flag(mask: c_int) -> bool;

    /*
     * Device Level.
     */
    pub fn snd_sof_device_probe(dev: *mut device, plat_data: *mut snd_sof_pdata) -> c_int;
    pub fn snd_sof_device_remove(dev: *mut device) -> c_int;
    pub fn snd_sof_device_shutdown(dev: *mut device) -> c_int;
    pub fn snd_sof_device_probe_completed(dev: *mut device) -> bool;

    pub fn snd_sof_runtime_suspend(dev: *mut device) -> c_int;
    pub fn snd_sof_runtime_resume(dev: *mut device) -> c_int;
    pub fn snd_sof_runtime_idle(dev: *mut device) -> c_int;
    pub fn snd_sof_resume(dev: *mut device) -> c_int;
    pub fn snd_sof_suspend(dev: *mut device) -> c_int;
    pub fn snd_sof_dsp_power_down_notify(sdev: *mut snd_sof_dev) -> c_int;
    pub fn snd_sof_prepare(dev: *mut device) -> c_int;
    pub fn snd_sof_complete(dev: *mut device);
    pub fn snd_sof_boot_dsp_firmware(sdev: *mut snd_sof_dev) -> c_int;

    pub fn snd_sof_new_platform_drv(sdev: *mut snd_sof_dev);

    /*
     * Compress support
     */
    pub static mut sof_compressed_ops: snd_compress_ops;

    /*
     * Firmware (firmware, libraries, topologies) file location
     */
    pub fn sof_create_ipc_file_profile(
        sdev: *mut snd_sof_dev,
        base_profile: *mut sof_loadable_file_profile,
        out_profile: *mut sof_loadable_file_profile,
    ) -> c_int;

    /*
     * Firmware loading.
     */
    pub fn snd_sof_load_firmware_raw(sdev: *mut snd_sof_dev) -> c_int;
    pub fn snd_sof_load_firmware_memcpy(sdev: *mut snd_sof_dev) -> c_int;
    pub fn snd_sof_run_firmware(sdev: *mut snd_sof_dev) -> c_int;
    pub fn snd_sof_fw_unload(sdev: *mut snd_sof_dev);

    /*
     * IPC low level APIs.
     */
    pub fn snd_sof_ipc_init(sdev: *mut snd_sof_dev) -> *mut snd_sof_ipc;
    pub fn snd_sof_ipc_free(sdev: *mut snd_sof_dev);
    pub fn snd_sof_ipc_get_reply(sdev: *mut snd_sof_dev);
    pub fn snd_sof_ipc_reply(sdev: *mut snd_sof_dev, msg_id: u32);
    pub fn sof_ipc_tx_message(
        ipc: *mut snd_sof_ipc,
        msg_data: *mut c_void,
        msg_bytes: size_t,
        reply_data: *mut c_void,
        reply_bytes: size_t,
    ) -> c_int;
    pub fn sof_ipc_set_get_data(ipc: *mut snd_sof_ipc, msg_data: *mut c_void, msg_bytes: size_t, set: bool) -> c_int;
    pub fn sof_ipc_tx_message_no_pm(
        ipc: *mut snd_sof_ipc,
        msg_data: *mut c_void,
        msg_bytes: size_t,
        reply_data: *mut c_void,
        reply_bytes: size_t,
    ) -> c_int;
    pub fn sof_ipc_send_msg(sdev: *mut snd_sof_dev, msg_data: *mut c_void, msg_bytes: size_t, reply_bytes: size_t) -> c_int;

    /*
     * Trace/debug
     */
    pub fn snd_sof_dbg_init(sdev: *mut snd_sof_dev) -> c_int;
    pub fn snd_sof_free_debug(sdev: *mut snd_sof_dev);
    pub fn snd_sof_debugfs_buf_item(sdev: *mut snd_sof_dev, base: *mut c_void, size: size_t, name: *const c_char, mode: mode_t) -> c_int;
    pub fn sof_print_oops_and_stack(
        sdev: *mut snd_sof_dev,
        level: *const c_char,
        panic_code: u32,
        tracep_code: u32,
        oops: *mut c_void,
        panic_info: *mut sof_ipc_panic_info,
        stack: *mut c_void,
        stack_words: size_t,
    );
    pub fn snd_sof_handle_fw_exception(sdev: *mut snd_sof_dev, msg: *const c_char);
    pub fn snd_sof_dbg_memory_info_init(sdev: *mut snd_sof_dev) -> c_int;
    pub fn snd_sof_debugfs_add_region_item_iomem(
        sdev: *mut snd_sof_dev,
        blk_type: snd_sof_fw_blk_type,
        offset: u32,
        size: size_t,
        name: *const c_char,
        access_type: sof_debugfs_access_type,
    ) -> c_int;
    /* Firmware tracing */
    pub fn sof_fw_trace_init(sdev: *mut snd_sof_dev) -> c_int;
    pub fn sof_fw_trace_free(sdev: *mut snd_sof_dev);
    pub fn sof_fw_trace_fw_crashed(sdev: *mut snd_sof_dev);
    pub fn sof_fw_trace_suspend(sdev: *mut snd_sof_dev, pm_state: pm_message_t);
    pub fn sof_fw_trace_resume(sdev: *mut snd_sof_dev) -> c_int;

    pub static sof_xtensa_arch_ops: dsp_arch_ops;

    /*
     * Firmware state tracking
     */
    pub fn sof_set_fw_state(sdev: *mut snd_sof_dev, new_state: sof_fw_state);

    /*
     * Utilities
     */
    pub fn sof_io_write(sdev: *mut snd_sof_dev, addr: *mut c_void, value: u32);
    pub fn sof_io_write64(sdev: *mut snd_sof_dev, addr: *mut c_void, value: u64);
    pub fn sof_io_read(sdev: *mut snd_sof_dev, addr: *mut c_void) -> u32;
    pub fn sof_io_read64(sdev: *mut snd_sof_dev, addr: *mut c_void) -> u64;
    pub fn sof_mailbox_write(sdev: *mut snd_sof_dev, offset: u32, message: *mut c_void, bytes: size_t);
    pub fn sof_mailbox_read(sdev: *mut snd_sof_dev, offset: u32, message: *mut c_void, bytes: size_t);
    pub fn sof_block_write(sdev: *mut snd_sof_dev, blk_type: snd_sof_fw_blk_type, offset: u32, src: *mut c_void, size: size_t) -> c_int;
    pub fn sof_block_read(sdev: *mut snd_sof_dev, blk_type: snd_sof_fw_blk_type, offset: u32, dest: *mut c_void, size: size_t) -> c_int;

    pub fn sof_ipc_msg_data(sdev: *mut snd_sof_dev, sps: *mut snd_sof_pcm_stream, p: *mut c_void, sz: size_t) -> c_int;
    pub fn sof_set_stream_data_offset(sdev: *mut snd_sof_dev, sps: *mut snd_sof_pcm_stream, posn_offset: size_t) -> c_int;

    pub fn sof_stream_pcm_open(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
    pub fn sof_stream_pcm_close(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;

    /* CONFIG_SND_SOC_SOF_CLIENT enabled declarations */
    pub fn sof_client_dev_to_sof_dev_external(cdev: *mut sof_client_dev) -> *mut snd_sof_dev;
    pub fn sof_client_dev_register_external(sdev: *mut snd_sof_dev, name: *const c_char, id: u32, data: *const c_void, size: size_t) -> c_int;
    pub fn sof_client_dev_unregister_external(sdev: *mut snd_sof_dev, name: *const c_char, id: u32);
    pub fn sof_register_clients_external(sdev: *mut snd_sof_dev) -> c_int;
    pub fn sof_unregister_clients_external(sdev: *mut snd_sof_dev);
    pub fn sof_client_ipc_rx_dispatcher_external(sdev: *mut snd_sof_dev, msg_buf: *mut c_void);
    pub fn sof_client_fw_state_dispatcher_external(sdev: *mut snd_sof_dev);
    pub fn sof_suspend_clients_external(sdev: *mut snd_sof_dev, state: pm_message_t) -> c_int;
    pub fn sof_resume_clients_external(sdev: *mut snd_sof_dev) -> c_int;

    /* Main ops for IPC implementations */
    pub static ipc3_ops: sof_ipc_ops;
    pub static ipc4_ops: sof_ipc_ops;
}

#[inline]
pub unsafe fn snd_sof_ipc_msgs_rx(sdev: *mut snd_sof_dev) {
    ((*(*(*sdev).ipc).ops).rx_msg.unwrap())(sdev);
}

#[inline]
pub unsafe fn sof_ipc_tx_message_no_reply(
    ipc: *mut snd_sof_ipc,
    msg_data: *mut c_void,
    msg_bytes: size_t,
) -> c_int {
    sof_ipc_tx_message(ipc, msg_data, msg_bytes, core::ptr::null_mut(), 0)
}

#[inline]
pub unsafe fn sof_ipc_tx_message_no_pm_no_reply(
    ipc: *mut snd_sof_ipc,
    msg_data: *mut c_void,
    msg_bytes: size_t,
) -> c_int {
    sof_ipc_tx_message_no_pm(ipc, msg_data, msg_bytes, core::ptr::null_mut(), 0)
}

#[inline]
pub unsafe fn snd_sof_ipc_process_reply(sdev: *mut snd_sof_dev, msg_id: u32) {
    snd_sof_ipc_get_reply(sdev);
    snd_sof_ipc_reply(sdev, msg_id);
}

/*
 * DSP Architectures.
 */
#[inline]
pub unsafe fn sof_stack(
    sdev: *mut snd_sof_dev,
    level: *const c_char,
    oops: *mut c_void,
    stack: *mut u32,
    stack_words: u32,
) {
    ((*sof_dsp_arch_ops(sdev)).dsp_stack.unwrap())(sdev, level, oops, stack, stack_words);
}

#[inline]
pub unsafe fn sof_oops(sdev: *mut snd_sof_dev, level: *const c_char, oops: *mut c_void) {
    if let Some(dsp_oops) = (*sof_dsp_arch_ops(sdev)).dsp_oops {
        dsp_oops(sdev, level, oops);
    }
}

/* SOF client support */
/* CONFIG_SND_SOC_SOF_CLIENT cannot be resolved from this isolated file.
 * The disabled-configuration inline fallbacks are translated below.
 */
#[inline]
pub unsafe fn sof_client_dev_to_sof_dev(_cdev: *mut sof_client_dev) -> *mut snd_sof_dev {
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn sof_client_dev_register(
    _sdev: *mut snd_sof_dev,
    _name: *const c_char,
    _id: u32,
    _data: *const c_void,
    _size: size_t,
) -> c_int {
    0
}

#[inline]
pub unsafe fn sof_client_dev_unregister(_sdev: *mut snd_sof_dev, _name: *const c_char, _id: u32) {}

#[inline]
pub unsafe fn sof_register_clients(_sdev: *mut snd_sof_dev) -> c_int {
    0
}

#[inline]
pub unsafe fn sof_unregister_clients(_sdev: *mut snd_sof_dev) {}

#[inline]
pub unsafe fn sof_client_ipc_rx_dispatcher(_sdev: *mut snd_sof_dev, _msg_buf: *mut c_void) {}

#[inline]
pub unsafe fn sof_client_fw_state_dispatcher(_sdev: *mut snd_sof_dev) {}

#[inline]
pub unsafe fn sof_suspend_clients(_sdev: *mut snd_sof_dev, _state: pm_message_t) -> c_int {
    0
}

#[inline]
pub unsafe fn sof_resume_clients(_sdev: *mut snd_sof_dev) -> c_int {
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
