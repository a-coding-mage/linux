/* SPDX-License-Identifier: GPL-2.0-or-later
 * Copyright (C) 2005 David Brownell
 *
 * Rust translation of the Linux SPI header.  Included C headers provide the
 * referenced types, constants, and functions.
 */

pub const SPI_DEVICE_CS_CNT_MAX: usize = 4;
pub const SPI_DEVICE_DATA_LANE_CNT_MAX: usize = 8;

pub const SPI_STATISTICS_HISTO_SIZE: usize = 17;
pub const SPI_DELAY_UNIT_USECS: u8 = 0;
pub const SPI_DELAY_UNIT_NSECS: u8 = 1;
pub const SPI_DELAY_UNIT_SCK: u8 = 2;

pub const SPI_NO_TX: u32 = 1 << 31;
pub const SPI_NO_RX: u32 = 1 << 30;
pub const SPI_TPM_HW_FLOW: u32 = 1 << 29;
pub const SPI_MODE_KERNEL_MASK: u32 = !( (1u32 << 29) - 1 );

#[repr(C)]
pub struct spi_statistics {
    pub syncp: u64_stats_sync,
    pub messages: u64_stats_t,
    pub transfers: u64_stats_t,
    pub errors: u64_stats_t,
    pub timedout: u64_stats_t,
    pub spi_sync: u64_stats_t,
    pub spi_sync_immediate: u64_stats_t,
    pub spi_async: u64_stats_t,
    pub bytes: u64_stats_t,
    pub bytes_rx: u64_stats_t,
    pub bytes_tx: u64_stats_t,
    pub transfer_bytes_histo: [u64_stats_t; SPI_STATISTICS_HISTO_SIZE],
    pub transfers_split_maxsize: u64_stats_t,
}

#[macro_export]
macro_rules! SPI_STATISTICS_ADD_TO_FIELD { ($pcpu_stats:expr, $field:ident, $count:expr) => {{
    let __lstats = this_cpu_ptr($pcpu_stats); get_cpu();
    u64_stats_update_begin(&mut (*__lstats).syncp); u64_stats_add(&mut (*__lstats).$field, $count);
    u64_stats_update_end(&mut (*__lstats).syncp); put_cpu();
}} }
#[macro_export]
macro_rules! SPI_STATISTICS_INCREMENT_FIELD { ($pcpu_stats:expr, $field:ident) => {{
    let __lstats = this_cpu_ptr($pcpu_stats); get_cpu();
    u64_stats_update_begin(&mut (*__lstats).syncp); u64_stats_inc(&mut (*__lstats).$field);
    u64_stats_update_end(&mut (*__lstats).syncp); put_cpu();
}} }

#[repr(C)] pub struct spi_delay { pub value: u16, pub unit: u8 }
extern "C" { pub fn spi_delay_to_ns(d: *mut spi_delay, xfer: *mut spi_transfer) -> i32; pub fn spi_delay_exec(d: *mut spi_delay, xfer: *mut spi_transfer) -> i32; pub fn spi_transfer_cs_change_delay_exec(msg: *mut spi_message, xfer: *mut spi_transfer); }

#[repr(C)]
pub struct spi_device {
    pub dev: device, pub controller: *mut spi_controller, pub max_speed_hz: u32,
    pub bits_per_word: u8, pub rt: bool, pub mode: u32, pub irq: i32,
    pub controller_state: *mut core::ffi::c_void, pub controller_data: *mut core::ffi::c_void,
    pub modalias: [core::ffi::c_char; SPI_NAME_SIZE as usize],
    pub pcpu_statistics: *mut spi_statistics, pub word_delay: spi_delay,
    pub cs_setup: spi_delay, pub cs_hold: spi_delay, pub cs_inactive: spi_delay,
    pub chip_select: [u8; SPI_DEVICE_CS_CNT_MAX], pub num_chipselect: u8,
    pub cs_index_mask: u32, pub cs_gpiod: [*mut gpio_desc; SPI_DEVICE_CS_CNT_MAX],
    pub tx_lane_map: [u8; SPI_DEVICE_DATA_LANE_CNT_MAX], pub num_tx_lanes: u8,
    pub rx_lane_map: [u8; SPI_DEVICE_DATA_LANE_CNT_MAX], pub num_rx_lanes: u8,
    #[cfg(CONFIG_SPI_DYNAMIC)] pub userspace_node: list_head,
}

pub const SPI_BPW_MASK: fn(u32) -> u32 = |bits| 1u32 << (bits - 1);
pub const SPI_CONTROLLER_HALF_DUPLEX: u16 = 1 << 0;
pub const SPI_CONTROLLER_NO_RX: u16 = 1 << 1;
pub const SPI_CONTROLLER_NO_TX: u16 = 1 << 2;
pub const SPI_CONTROLLER_MUST_RX: u16 = 1 << 3;
pub const SPI_CONTROLLER_MUST_TX: u16 = 1 << 4;
pub const SPI_CONTROLLER_GPIO_SS: u16 = 1 << 5;
pub const SPI_CONTROLLER_SUSPENDED: u16 = 1 << 6;
pub const SPI_CONTROLLER_MULTI_CS: u16 = 1 << 7;

#[repr(C)] pub union spi_controller_target { pub slave: bool, pub target: bool }
#[repr(C)]
pub struct spi_controller {
    pub dev: device, pub list: list_head, pub bus_num: i16, pub num_chipselect: u16,
    pub num_data_lanes: u16, pub dma_alignment: u16, pub mode_bits: u32,
    pub buswidth_override_bits: u32, pub bits_per_word_mask: u32,
    pub min_speed_hz: u32, pub max_speed_hz: u32, pub flags: u16,
    pub slave_or_target: spi_controller_target,
    pub max_transfer_size: Option<unsafe extern "C" fn(*mut spi_device) -> usize>,
    pub max_message_size: Option<unsafe extern "C" fn(*mut spi_device) -> usize>,
    pub io_mutex: mutex, pub add_lock: mutex, pub bus_lock_spinlock: spinlock_t,
    pub bus_lock_mutex: mutex, pub bus_lock_flag: bool,
    pub setup: Option<unsafe extern "C" fn(*mut spi_device) -> i32>,
    pub set_cs_timing: Option<unsafe extern "C" fn(*mut spi_device) -> i32>,
    pub transfer: Option<unsafe extern "C" fn(*mut spi_device, *mut spi_message) -> i32>,
    pub cleanup: Option<unsafe extern "C" fn(*mut spi_device)>,
    pub can_dma: Option<unsafe extern "C" fn(*mut spi_controller,*mut spi_device,*mut spi_transfer)->bool>,
    pub dma_map_dev: *mut device, pub cur_rx_dma_dev: *mut device, pub cur_tx_dma_dev: *mut device,
    pub queued: bool, pub kworker: *mut kthread_worker, pub pump_messages: kthread_work,
    pub queue_lock: spinlock_t, pub queue: list_head, pub cur_msg: *mut spi_message,
    pub cur_msg_completion: completion, pub cur_msg_incomplete: bool, pub cur_msg_need_completion: bool,
    pub busy: bool, pub running: bool, pub rt: bool, pub auto_runtime_pm: bool, pub fallback: bool,
    pub last_cs_mode_high: bool, pub last_cs: [i8; SPI_DEVICE_CS_CNT_MAX], pub last_cs_index_mask: u32,
    pub xfer_completion: completion, pub max_dma_len: usize,
    pub optimize_message: Option<unsafe extern "C" fn(*mut spi_message)->i32>,
    pub unoptimize_message: Option<unsafe extern "C" fn(*mut spi_message)->i32>,
    pub prepare_transfer_hardware: Option<unsafe extern "C" fn(*mut spi_controller)->i32>,
    pub transfer_one_message: Option<unsafe extern "C" fn(*mut spi_controller,*mut spi_message)->i32>,
    pub unprepare_transfer_hardware: Option<unsafe extern "C" fn(*mut spi_controller)->i32>,
    pub prepare_message: Option<unsafe extern "C" fn(*mut spi_controller,*mut spi_message)->i32>,
    pub unprepare_message: Option<unsafe extern "C" fn(*mut spi_controller,*mut spi_message)->i32>,
    pub target_abort: Option<unsafe extern "C" fn(*mut spi_controller)->i32>,
    pub set_cs: Option<unsafe extern "C" fn(*mut spi_device,bool)>,
    pub transfer_one: Option<unsafe extern "C" fn(*mut spi_controller,*mut spi_device,*mut spi_transfer)->i32>,
    pub handle_err: Option<unsafe extern "C" fn(*mut spi_controller,*mut spi_message)>,
    pub mem_ops: *const spi_controller_mem_ops, pub mem_caps: *const spi_controller_mem_caps,
    pub dtr_caps: bool, pub get_offload: Option<unsafe extern "C" fn(*mut spi_device,*const spi_offload_config)->*mut spi_offload>,
    pub put_offload: Option<unsafe extern "C" fn(*mut spi_offload)>, pub cs_gpiods: *mut *mut gpio_desc,
    pub use_gpio_descriptors: bool, pub unused_native_cs: i8, pub max_native_cs: i8,
    pub pcpu_statistics: *mut spi_statistics, pub dma_tx: *mut dma_chan, pub dma_rx: *mut dma_chan,
    pub dummy_rx: *mut core::ffi::c_void, pub dummy_tx: *mut core::ffi::c_void,
    pub fw_translate_cs: Option<unsafe extern "C" fn(*mut spi_controller,u32)->i32>,
    pub ptp_sts_supported: bool, pub irq_flags: core::ffi::c_ulong, pub queue_empty: bool,
    pub must_async: bool, pub defer_optimize_message: bool,
    #[cfg(CONFIG_SPI_DYNAMIC)] pub userspace_clients: list_head,
    #[cfg(CONFIG_SPI_DYNAMIC)] pub userspace_registered: bool,
}

pub const SPI_TRANS_FAIL_NO_START: u16 = 1 << 0;
pub const SPI_TRANS_FAIL_IO: u16 = 1 << 1;
pub const SPI_MULTI_LANE_MODE_SINGLE: u32 = 0;
pub const SPI_MULTI_LANE_MODE_STRIPE: u32 = 1;
pub const SPI_MULTI_LANE_MODE_MIRROR: u32 = 2;
pub const SPI_NBITS_SINGLE: u8 = 1; pub const SPI_NBITS_DUAL: u8 = 2;
pub const SPI_NBITS_QUAD: u8 = 4; pub const SPI_NBITS_OCTAL: u8 = 8;

#[repr(C)] pub struct spi_transfer {
    pub tx_buf: *const core::ffi::c_void, pub rx_buf: *mut core::ffi::c_void, pub len: u32, pub error: u16,
    pub tx_sg_mapped: bool, pub rx_sg_mapped: bool, pub tx_sg: sg_table, pub rx_sg: sg_table,
    pub tx_dma: dma_addr_t, pub rx_dma: dma_addr_t, pub dummy_data: u32, pub cs_off: u32,
    pub cs_change: u32, pub tx_nbits: u32, pub rx_nbits: u32, pub multi_lane_mode: u32,
    pub timestamped: u32, pub dtr_mode: bool, pub bits_per_word: u8, pub delay: spi_delay,
    pub cs_change_delay: spi_delay, pub word_delay: spi_delay, pub speed_hz: u32,
    pub effective_speed_hz: u32, pub offload_flags: u32, pub ptp_sts_word_pre: u32,
    pub ptp_sts_word_post: u32, pub ptp_sts: *mut ptp_system_timestamp, pub transfer_list: list_head,
}

#[repr(C)] pub struct spi_message {
    pub transfers: list_head, pub spi: *mut spi_device, pub pre_optimized: bool, pub optimized: bool,
    pub prepared: bool, pub status: i32, pub complete: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub context: *mut core::ffi::c_void, pub frame_length: u32, pub actual_length: u32,
    pub queue: list_head, pub state: *mut core::ffi::c_void, pub opt_state: *mut core::ffi::c_void,
    pub offload: *mut spi_offload, pub resources: list_head,
}

#[repr(C)] pub struct spi_driver { pub id_table: *const spi_device_id, pub probe: Option<unsafe extern "C" fn(*mut spi_device)->i32>, pub remove: Option<unsafe extern "C" fn(*mut spi_device)>, pub shutdown: Option<unsafe extern "C" fn(*mut spi_device)>, pub driver: device_driver }
pub type spi_res_release_t = Option<unsafe extern "C" fn(*mut spi_controller,*mut spi_message,*mut core::ffi::c_void)>;
#[repr(C)] pub struct spi_res { pub entry: list_head, pub release: spi_res_release_t, pub data: [core::ffi::c_ulonglong; 0] }

extern "C" {
    pub fn spi_unregister_driver(sdrv: *mut spi_driver);
    pub fn spi_new_ancillary_device(spi: *mut spi_device, chip_select: u8) -> *mut spi_device;
    pub fn devm_spi_new_ancillary_device(spi: *mut spi_device, chip_select: u8) -> *mut spi_device;
    pub fn spi_controller_suspend(ctlr: *mut spi_controller) -> i32; pub fn spi_controller_resume(ctlr: *mut spi_controller) -> i32;
    pub fn spi_get_next_queued_message(ctlr: *mut spi_controller) -> *mut spi_message;
    pub fn spi_finalize_current_message(ctlr: *mut spi_controller); pub fn spi_finalize_current_transfer(ctlr: *mut spi_controller);
    pub fn spi_take_timestamp_pre(ctlr:*mut spi_controller,xfer:*mut spi_transfer,progress:usize,irqs_off:bool);
    pub fn spi_take_timestamp_post(ctlr:*mut spi_controller,xfer:*mut spi_transfer,progress:usize,irqs_off:bool);
    pub fn __spi_alloc_controller(host:*mut device,size:u32,target:bool)->*mut spi_controller;
    pub fn __devm_spi_alloc_controller(dev:*mut device,size:u32,target:bool)->*mut spi_controller;
    pub fn spi_register_controller(ctlr:*mut spi_controller)->i32; pub fn devm_spi_register_controller(dev:*mut device,ctlr:*mut spi_controller)->i32; pub fn spi_unregister_controller(ctlr:*mut spi_controller);
    pub fn spi_optimize_message(spi:*mut spi_device,msg:*mut spi_message)->i32; pub fn spi_unoptimize_message(msg:*mut spi_message);
    pub fn devm_spi_optimize_message(dev:*mut device,spi:*mut spi_device,msg:*mut spi_message)->i32;
    pub fn spi_setup(spi:*mut spi_device)->i32; pub fn spi_async(spi:*mut spi_device,message:*mut spi_message)->i32; pub fn spi_target_abort(spi:*mut spi_device)->i32;
}

// The following declarations mirror the header's inline helpers and preserve
// their C linkage for consumers supplying the Linux dependencies.
extern "C" {
    pub fn spi_message_init_no_memset(m:*mut spi_message); pub fn spi_message_init(m:*mut spi_message);
    pub fn spi_message_add_tail(t:*mut spi_transfer,m:*mut spi_message); pub fn spi_transfer_del(t:*mut spi_transfer);
    pub fn spi_transfer_delay_exec(t:*mut spi_transfer)->i32; pub fn spi_message_init_with_transfers(m:*mut spi_message,xfers:*mut spi_transfer,num_xfers:u32);
    pub fn spi_message_alloc(ntrans:u32,flags:gfp_t)->*mut spi_message; pub fn spi_message_free(m:*mut spi_message);
    pub fn spi_max_message_size(spi:*mut spi_device)->usize; pub fn spi_max_transfer_size(spi:*mut spi_device)->usize;
    pub fn spi_is_bpw_supported(spi:*mut spi_device,bpw:u32)->bool; pub fn spi_bpw_to_bytes(bpw:u32)->u32;
    pub fn spi_controller_xfer_timeout(ctlr:*mut spi_controller,xfer:*mut spi_transfer)->u32;
}

pub fn SPI_BPW_RANGE_MASK(min: u32, max: u32) -> u32 { (((1u64 << max) - 1) ^ ((1u64 << (min - 1)) - 1)) as u32 }
pub struct spi_replaced_transfers;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
