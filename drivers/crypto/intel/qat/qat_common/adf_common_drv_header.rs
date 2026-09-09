/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2021 Intel Corporation */

/* Dependencies supplied by the surrounding translation unit. */

pub const ADF_STATUS_RESTARTING: i32 = 0;
pub const ADF_STATUS_STARTING: i32 = 1;
pub const ADF_STATUS_CONFIGURED: i32 = 2;
pub const ADF_STATUS_STARTED: i32 = 3;
pub const ADF_STATUS_AE_INITIALISED: i32 = 4;
pub const ADF_STATUS_AE_UCODE_LOADED: i32 = 5;
pub const ADF_STATUS_AE_STARTED: i32 = 6;
pub const ADF_STATUS_PF_RUNNING: i32 = 7;
pub const ADF_STATUS_IRQ_ALLOCATED: i32 = 8;
pub const ADF_STATUS_CRYPTO_ALGS_REGISTERED: i32 = 9;
pub const ADF_STATUS_COMP_ALGS_REGISTERED: i32 = 10;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum adf_dev_reset_mode {
    ADF_DEV_RESET_ASYNC = 0,
    ADF_DEV_RESET_SYNC,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum adf_event {
    ADF_EVENT_INIT = 0,
    ADF_EVENT_START,
    ADF_EVENT_STOP,
    ADF_EVENT_SHUTDOWN,
    ADF_EVENT_RESTARTING,
    ADF_EVENT_RESTARTED,
    ADF_EVENT_FATAL_ERROR,
}

#[repr(C)]
pub struct service_hndl {
    pub event_hld: Option<unsafe extern "C" fn(*mut adf_accel_dev, adf_event) -> i32>,
    pub init_status: [c_ulong; ADF_DEVS_ARRAY_SIZE],
    pub start_status: [c_ulong; ADF_DEVS_ARRAY_SIZE],
    pub name: *mut c_char,
    pub list: list_head,
}

extern "C" {
    pub fn adf_service_register(service: *mut service_hndl) -> i32;
    pub fn adf_service_unregister(service: *mut service_hndl) -> i32;
    pub fn adf_dev_up(accel_dev: *mut adf_accel_dev, init_config: bool) -> i32;
    pub fn adf_dev_down(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_dev_restart(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_devmgr_update_class_index(hw_data: *mut adf_hw_device_data);
    pub fn adf_clean_vf_map(_: bool);
    pub fn adf_notify_fatal_error(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_error_notifier(accel_dev: *mut adf_accel_dev);
    pub fn adf_devmgr_add_dev(accel_dev: *mut adf_accel_dev, pf: *mut adf_accel_dev) -> i32;
    pub fn adf_devmgr_rm_dev(accel_dev: *mut adf_accel_dev, pf: *mut adf_accel_dev);
    pub fn adf_devmgr_get_head() -> *mut list_head;
    pub fn adf_devmgr_pci_to_accel_dev(pci_dev: *mut pci_dev) -> *mut adf_accel_dev;
    pub fn adf_devmgr_in_reset(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_dev_started(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_dev_restarting_notify(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_dev_restarted_notify(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_ae_init(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_ae_shutdown(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_ae_fw_load(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_ae_fw_release(accel_dev: *mut adf_accel_dev);
    pub fn adf_ae_start(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_ae_stop(accel_dev: *mut adf_accel_dev) -> i32;
    pub static adf_err_handler: pci_error_handlers;
    pub fn adf_reset_sbr(accel_dev: *mut adf_accel_dev);
    pub fn adf_reset_flr(accel_dev: *mut adf_accel_dev);
    pub fn adf_dev_restore(accel_dev: *mut adf_accel_dev);
    pub fn adf_set_bme(accel_dev: *mut adf_accel_dev);
    pub fn adf_init_aer() -> i32;
    pub fn adf_exit_aer();
    pub fn adf_init_arb(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_exit_arb(accel_dev: *mut adf_accel_dev);
    pub fn adf_update_ring_arb(ring: *mut adf_etr_ring_data);
    pub fn adf_dev_get(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_dev_put(accel_dev: *mut adf_accel_dev);
    pub fn adf_dev_in_use(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_init_etr_data(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_cleanup_etr_data(accel_dev: *mut adf_accel_dev);
    pub fn qat_crypto_register() -> i32;
    pub fn qat_crypto_unregister() -> i32;
    pub fn qat_crypto_vf_dev_config(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn qat_crypto_get_instance_node(node: i32) -> *mut qat_crypto_instance;
    pub fn qat_crypto_put_instance(inst: *mut qat_crypto_instance);
    pub fn qat_alg_callback(resp: *mut c_void);
    pub fn qat_alg_asym_callback(resp: *mut c_void);
    pub fn qat_algs_register() -> i32;
    pub fn qat_algs_unregister();
    pub fn qat_asym_algs_register() -> i32;
    pub fn qat_asym_algs_unregister();
    pub fn qat_compression_get_instance_node(node: i32, alg: i32) -> *mut qat_compression_instance;
    pub fn qat_compression_put_instance(inst: *mut qat_compression_instance);
    pub fn qat_compression_register() -> i32;
    pub fn qat_compression_unregister() -> i32;
    pub fn qat_comp_algs_register(caps: u32) -> i32;
    pub fn qat_comp_algs_unregister(caps: u32);
    pub fn qat_comp_alg_callback(resp: *mut c_void);
    pub fn adf_isr_resource_alloc(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_isr_resource_free(accel_dev: *mut adf_accel_dev);
    pub fn adf_isr_sync_ae_cluster(accel_dev: *mut adf_accel_dev);
    pub fn adf_vf_isr_resource_alloc(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_vf_isr_resource_free(accel_dev: *mut adf_accel_dev);
    pub fn adf_pfvf_comms_disabled(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_sysfs_init(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_init_misc_wq() -> i32;
    pub fn adf_exit_misc_wq();
    pub fn adf_misc_wq_queue_work(work: *mut work_struct) -> bool;
    pub fn adf_misc_wq_queue_delayed_work(work: *mut delayed_work, delay: c_ulong) -> bool;
    pub fn adf_misc_wq_flush();
}

/* CONFIG_PCI_IOV declarations are retained under the original build-time condition. */
#[cfg(feature = "CONFIG_PCI_IOV")]
extern "C" {
    pub fn adf_sriov_configure(pdev: *mut pci_dev, numvfs: i32) -> i32;
    pub fn adf_disable_sriov(accel_dev: *mut adf_accel_dev);
    pub fn adf_reenable_sriov(accel_dev: *mut adf_accel_dev);
    pub fn adf_enable_vf2pf_interrupts(accel_dev: *mut adf_accel_dev, vf_mask: u32);
    pub fn adf_enable_all_vf2pf_interrupts(accel_dev: *mut adf_accel_dev, num_vfs: u32);
    pub fn adf_disable_all_vf2pf_interrupts(accel_dev: *mut adf_accel_dev);
    pub fn adf_recv_and_handle_pf2vf_msg(accel_dev: *mut adf_accel_dev) -> bool;
    pub fn adf_recv_and_handle_vf2pf_msg(accel_dev: *mut adf_accel_dev, vf_nr: u32) -> bool;
    pub fn adf_pf2vf_handle_pf_restarting(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_enable_pf2vf_interrupts(accel_dev: *mut adf_accel_dev);
    pub fn adf_disable_pf2vf_interrupts(accel_dev: *mut adf_accel_dev);
    pub fn adf_schedule_vf2pf_handler(vf_info: *mut adf_accel_vf_info);
    pub fn adf_init_pf_wq() -> i32;
    pub fn adf_exit_pf_wq();
    pub fn adf_init_vf_wq() -> i32;
    pub fn adf_exit_vf_wq();
    pub fn adf_flush_vf_wq(accel_dev: *mut adf_accel_dev);
}

#[cfg(not(feature = "CONFIG_PCI_IOV"))]
pub unsafe fn adf_disable_sriov(_: *mut adf_accel_dev) {}
#[cfg(not(feature = "CONFIG_PCI_IOV"))]
pub unsafe fn adf_reenable_sriov(_: *mut adf_accel_dev) {}
#[cfg(not(feature = "CONFIG_PCI_IOV"))]
pub unsafe fn adf_init_pf_wq() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PCI_IOV"))]
pub unsafe fn adf_exit_pf_wq() {}
#[cfg(not(feature = "CONFIG_PCI_IOV"))]
pub unsafe fn adf_init_vf_wq() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PCI_IOV"))]
pub unsafe fn adf_exit_vf_wq() {}

/* The remaining HAL declarations use types supplied by the included HAL headers. */
extern "C" {
    pub fn qat_hal_init(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn qat_hal_deinit(handle: *mut icp_qat_fw_loader_handle);
    pub fn qat_hal_start(handle: *mut icp_qat_fw_loader_handle) -> i32;
    pub fn qat_hal_stop(handle: *mut icp_qat_fw_loader_handle, ae: u8, ctx_mask: u32);
    pub fn qat_hal_reset(handle: *mut icp_qat_fw_loader_handle);
    pub fn qat_hal_clr_reset(handle: *mut icp_qat_fw_loader_handle) -> i32;
    pub fn qat_hal_set_live_ctx(handle: *mut icp_qat_fw_loader_handle, ae: u8, ctx_mask: u32);
    pub fn qat_hal_check_ae_active(handle: *mut icp_qat_fw_loader_handle, ae: u32) -> i32;
    pub fn qat_hal_set_ae_lm_mode(handle: *mut icp_qat_fw_loader_handle, ae: u8, lm_type: icp_qat_uof_regtype, mode: u8) -> i32;
    pub fn qat_hal_set_ae_ctx_mode(handle: *mut icp_qat_fw_loader_handle, ae: u8, mode: u8) -> i32;
    pub fn qat_hal_set_ae_nn_mode(handle: *mut icp_qat_fw_loader_handle, ae: u8, mode: u8) -> i32;
    pub fn qat_hal_set_pc(handle: *mut icp_qat_fw_loader_handle, ae: u8, ctx_mask: u32, upc: u32);
    pub fn qat_hal_wr_uwords(handle: *mut icp_qat_fw_loader_handle, ae: u8, uaddr: u32, words_num: u32, uword: *mut u64);
    pub fn qat_hal_wr_umem(handle: *mut icp_qat_fw_loader_handle, ae: u8, uword_addr: u32, words_num: u32, data: *mut u32);
    pub fn qat_hal_get_ins_num() -> i32;
    pub fn qat_hal_batch_wr_lm(handle: *mut icp_qat_fw_loader_handle, ae: u8, lm_init_header: *mut icp_qat_uof_batch_init) -> i32;
    pub fn qat_hal_init_gpr(handle: *mut icp_qat_fw_loader_handle, ae: u8, ctx_mask: c_ulong, reg_type: icp_qat_uof_regtype, reg_num: u16, regdata: u32) -> i32;
    pub fn qat_hal_init_wr_xfer(handle: *mut icp_qat_fw_loader_handle, ae: u8, ctx_mask: c_ulong, reg_type: icp_qat_uof_regtype, reg_num: u16, regdata: u32) -> i32;
    pub fn qat_hal_init_rd_xfer(handle: *mut icp_qat_fw_loader_handle, ae: u8, ctx_mask: c_ulong, reg_type: icp_qat_uof_regtype, reg_num: u16, regdata: u32) -> i32;
    pub fn qat_hal_init_nn(handle: *mut icp_qat_fw_loader_handle, ae: u8, ctx_mask: c_ulong, reg_num: u16, regdata: u32) -> i32;
    pub fn qat_hal_set_ae_tindex_mode(handle: *mut icp_qat_fw_loader_handle, ae: u8, mode: u8);
    pub fn qat_uclo_wr_all_uimage(handle: *mut icp_qat_fw_loader_handle) -> i32;
    pub fn qat_uclo_del_obj(handle: *mut icp_qat_fw_loader_handle);
    pub fn qat_uclo_wr_mimage(handle: *mut icp_qat_fw_loader_handle, addr_ptr: *mut c_void, mem_size: i32) -> i32;
    pub fn qat_uclo_map_obj(handle: *mut icp_qat_fw_loader_handle, addr_ptr: *mut c_void, mem_size: u32, obj_name: *const c_char) -> i32;
    pub fn qat_uclo_set_cfg_ae_mask(handle: *mut icp_qat_fw_loader_handle, cfg_ae_mask: u32) -> i32;
}

/* Inline BAR helpers; GET_BARS and the device layout are provided by the
 * included accelerator-device definitions. */
#[inline]
pub unsafe fn adf_get_pmisc_base(accel_dev: *mut adf_accel_dev) -> *mut c_void {
    let hw_data = (*accel_dev).hw_device;
    let pmisc = &mut GET_BARS!(accel_dev)[(*hw_data).get_misc_bar_id(hw_data) as usize];
    pmisc.virt_addr
}

#[inline]
pub unsafe fn adf_get_etr_base(accel_dev: *mut adf_accel_dev) -> *mut c_void {
    let hw_data = (*accel_dev).hw_device;
    let etr = &mut GET_BARS!(accel_dev)[(*hw_data).get_etr_bar_id(hw_data) as usize];
    etr.virt_addr
}

#[inline]
pub unsafe fn adf_get_aram_base(accel_dev: *mut adf_accel_dev) -> *mut c_void {
    let hw_data = (*accel_dev).hw_device;
    let param = &mut GET_BARS!(accel_dev)[(*hw_data).get_sram_bar_id(hw_data) as usize];
    param.virt_addr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
