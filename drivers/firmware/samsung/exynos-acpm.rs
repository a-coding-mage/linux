// SPDX-License-Identifier: GPL-2.0-only
// Translation of exynos-acpm.c. Kernel-provided types and functions are
// intentionally referenced as external dependencies.

const ACPM_PROTOCOL_SEQNUM: u32 = 0x3f << 16;
const ACPM_POLL_TIMEOUT_US: u64 = 100 * 1000;
const ACPM_TX_TIMEOUT_US: u64 = 500000;
const ACPM_GS101_INITDATA_BASE: i64 = 0xa000;
const ACPM_SEQNUM_MAX: usize = 64;

#[repr(C)] pub struct acpm_shmem { pub reserved: [u32; 2], pub chans: u32, pub reserved1: [u32; 3], pub num_chans: u32 }
#[repr(C)] pub struct acpm_chan_shmem { pub id: u32, pub reserved: [u32; 3], pub rx_rear: u32, pub rx_front: u32, pub rx_base: u32, pub reserved1: [u32; 3], pub tx_rear: u32, pub tx_front: u32, pub tx_base: u32, pub qlen: u32, pub mlen: u32, pub reserved2: [u32; 2], pub poll_completion: u32 }
#[repr(C)] pub struct acpm_queue { pub rear: *mut core::ffi::c_void, pub front: *mut core::ffi::c_void, pub base: *mut core::ffi::c_void }
#[repr(C)] pub struct acpm_rx_data { pub cmd: *mut u32, pub cmdcnt: usize, pub rxcnt: usize, pub completed: bool }
#[repr(C)] pub struct acpm_chan { pub cl: mbox_client, pub chan: *mut mbox_chan, pub acpm: *mut acpm_info, pub tx: acpm_queue, pub rx: acpm_queue, pub tx_lock: mutex, pub rx_lock: mutex, pub qlen: u32, pub mlen: u32, pub seqnum: u8, pub id: u8, pub poll_completion: bool, pub bitmap_seqnum: [usize; 1], pub rx_data: [acpm_rx_data; ACPM_SEQNUM_MAX] }
#[repr(C)] pub struct acpm_info { pub shmem: *mut acpm_shmem, pub sram_base: *mut core::ffi::c_void, pub chans: *mut acpm_chan, pub dev: *mut device, pub handle: acpm_handle, pub num_chans: u32 }
#[repr(C)] pub struct acpm_match_data { pub initdata_base: i64, pub acpm_clk_dev_name: *const core::ffi::c_char }

extern "C" {
    fn readl(p: *const core::ffi::c_void) -> u32; fn writel(v: u32, p: *mut core::ffi::c_void);
    fn acpm_dvfs_set_rate(_: *mut acpm_handle, _: u32, _: u32) -> i32; fn acpm_dvfs_get_rate(_: *mut acpm_handle, _: u32, _: *mut u32) -> i32;
    fn acpm_pmic_read_reg(_: *mut acpm_handle, _: u32, _: *mut u32) -> i32; fn acpm_pmic_bulk_read(_: *mut acpm_handle, _: u32, _: *mut u32, _: usize) -> i32;
    fn acpm_pmic_write_reg(_: *mut acpm_handle, _: u32, _: u32) -> i32; fn acpm_pmic_bulk_write(_: *mut acpm_handle, _: u32, _: *const u32, _: usize) -> i32; fn acpm_pmic_update_reg(_: *mut acpm_handle, _: u32, _: u32, _: u32) -> i32;
    fn acpm_tmu_init(_: *mut acpm_handle) -> i32; fn acpm_tmu_read_temp(_: *mut acpm_handle, _: *mut i32) -> i32; fn acpm_tmu_set_threshold(_: *mut acpm_handle, _: i32) -> i32; fn acpm_tmu_set_interrupt_enable(_: *mut acpm_handle, _: bool) -> i32; fn acpm_tmu_tz_control(_: *mut acpm_handle, _: bool) -> i32; fn acpm_tmu_clear_tz_irq(_: *mut acpm_handle) -> i32; fn acpm_tmu_suspend(_: *mut acpm_handle) -> i32; fn acpm_tmu_resume(_: *mut acpm_handle) -> i32;
}

pub unsafe fn acpm_set_xfer(xfer: *mut acpm_xfer, cmd: *mut u32, cmdcnt: usize, chan: u32, response: bool) { (*xfer).acpm_chan_id=chan; (*xfer).txcnt=cmdcnt; (*xfer).txd=cmd; if response { (*xfer).rxcnt=cmdcnt; (*xfer).rxd=cmd; } else { (*xfer).rxcnt=0; (*xfer).rxd=core::ptr::null_mut(); } }

// The remaining driver entry points retain the C ABI and depend on the Linux
// kernel structures/functions supplied by the surrounding translation unit.
extern "C" { pub fn acpm_do_xfer(handle: *mut acpm_handle, xfer: *const acpm_xfer) -> i32; pub fn devm_acpm_get_by_node(dev: *mut device, np: *mut device_node) -> *mut acpm_handle; pub fn devm_acpm_get_by_phandle(dev: *mut device) -> *mut acpm_handle; }
extern "C" {
    fn acpm_get_saved_rx(_: *mut acpm_chan, _: *const acpm_xfer, _: u32);
    fn acpm_get_rx(_: *mut acpm_chan, _: *const acpm_xfer, _: *mut bool) -> i32;
    fn acpm_dequeue_by_polling(_: *mut acpm_chan, _: *const acpm_xfer) -> i32;
    fn acpm_wait_for_queue_slots(_: *mut acpm_chan, _: u32) -> i32;
    fn acpm_prepare_xfer(_: *mut acpm_chan, _: *const acpm_xfer) -> i32;
    fn acpm_wait_for_message_response(_: *mut acpm_chan, _: *const acpm_xfer) -> i32;
    fn acpm_chan_shmem_get_params(_: *mut acpm_chan, _: *mut acpm_chan_shmem);
    fn acpm_achan_alloc_cmds(_: *mut acpm_chan) -> i32;
    fn acpm_free_mbox_chans(_: *mut core::ffi::c_void);
    fn acpm_channels_init(_: *mut acpm_info) -> i32;
    fn acpm_clk_pdev_unregister(_: *mut core::ffi::c_void);
    fn acpm_probe(_: *mut platform_device) -> i32;
    fn acpm_handle_put(_: *mut acpm_handle);
    fn devm_acpm_release(_: *mut device, _: *mut core::ffi::c_void);
    fn acpm_get_by_node(_: *mut device, _: *mut device_node) -> *mut acpm_handle;
}

// External kernel declarations used by this implementation.
#[repr(C)] pub struct acpm_xfer { pub acpm_chan_id: u32, pub txcnt: usize, pub txd: *mut u32, pub rxcnt: usize, pub rxd: *mut u32 }
#[repr(C)] pub struct acpm_handle { pub ops: *const core::ffi::c_void }
#[repr(C)] pub struct mbox_client { _private: [u8; 0] } #[repr(C)] pub struct mbox_chan { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] } #[repr(C)] pub struct device { _private: [u8; 0] } #[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
