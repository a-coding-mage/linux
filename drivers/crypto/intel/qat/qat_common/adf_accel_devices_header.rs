/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */
/* Linux dependencies and conditional compilation are supplied by the surrounding crate. */

pub const ADF_DH895XCC_DEVICE_NAME: &str = "dh895xcc";
pub const ADF_DH895XCCVF_DEVICE_NAME: &str = "dh895xccvf";
pub const ADF_C62X_DEVICE_NAME: &str = "c6xx";
pub const ADF_C62XVF_DEVICE_NAME: &str = "c6xxvf";
pub const ADF_C3XXX_DEVICE_NAME: &str = "c3xxx";
pub const ADF_C3XXXVF_DEVICE_NAME: &str = "c3xxxvf";
pub const ADF_4XXX_DEVICE_NAME: &str = "4xxx";
pub const ADF_420XX_DEVICE_NAME: &str = "420xx";
pub const ADF_6XXX_DEVICE_NAME: &str = "6xxx";
pub const PCI_DEVICE_ID_INTEL_QAT_4XXXIOV: u32 = 0x4941;
pub const PCI_DEVICE_ID_INTEL_QAT_401XXIOV: u32 = 0x4943;
pub const PCI_DEVICE_ID_INTEL_QAT_402XXIOV: u32 = 0x4945;
pub const PCI_DEVICE_ID_INTEL_QAT_420XXIOV: u32 = 0x4947;
pub const PCI_DEVICE_ID_INTEL_QAT_6XXX_IOV: u32 = 0x4949;
pub const ADF_DEVICE_FUSECTL_OFFSET: u32 = 0x40;
pub const ADF_DEVICE_LEGFUSE_OFFSET: u32 = 0x4C;
pub const ADF_DEVICE_FUSECTL_MASK: u32 = 0x80000000;
pub const ADF_PCI_MAX_BARS: usize = 3;
pub const ADF_DEVICE_NAME_LENGTH: usize = 32;
pub const ADF_ETR_MAX_RINGS_PER_BANK: usize = 16;
pub const ADF_MAX_MSIX_VECTOR_NAME: usize = 48;
pub const ADF_DEVICE_NAME_PREFIX: &str = "qat_";

#[repr(C)]
#[derive(Copy, Clone)]
pub enum adf_accel_capabilities { ADF_ACCEL_CAPABILITIES_NULL=0, ADF_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC=1, ADF_ACCEL_CAPABILITIES_CRYPTO_ASYMMETRIC=2, ADF_ACCEL_CAPABILITIES_CIPHER=4, ADF_ACCEL_CAPABILITIES_AUTHENTICATION=8, ADF_ACCEL_CAPABILITIES_COMPRESSION=32, ADF_ACCEL_CAPABILITIES_LZS_COMPRESSION=64, ADF_ACCEL_CAPABILITIES_RANDOM_NUMBER=128 }
pub const ADF_ACCEL_CAPABILITIES_EXT_ZSTD_LZ4S: u32 = 1 << 0;
pub const ADF_ACCEL_CAPABILITIES_EXT_ZSTD: u32 = 1 << 1;
#[repr(C)] pub enum adf_fuses { ADF_FUSECTL0, ADF_FUSECTL1, ADF_FUSECTL2, ADF_FUSECTL3, ADF_FUSECTL4, ADF_FUSECTL5, ADF_MAX_FUSES }

#[repr(C)] pub struct adf_bar { pub base_addr: resource_size_t, pub virt_addr: *mut core::ffi::c_void, pub size: resource_size_t }
#[repr(C)] pub struct adf_irq { pub enabled: bool, pub name: [core::ffi::c_char; ADF_MAX_MSIX_VECTOR_NAME] }
#[repr(C)] pub struct adf_accel_msix { pub irqs: *mut adf_irq, pub num_entries: u32 }
#[repr(C)] pub struct adf_accel_pci { pub pci_dev: *mut pci_dev, pub msix_entries: adf_accel_msix, pub pci_bars: [adf_bar; ADF_PCI_MAX_BARS], pub revid: u8, pub sku: u8 }
#[repr(C)] pub enum dev_state { DEV_DOWN=0, DEV_UP }
#[repr(C)] pub enum dev_sku_info { DEV_SKU_1=0, DEV_SKU_2, DEV_SKU_3, DEV_SKU_4, DEV_SKU_VF, DEV_SKU_UNKNOWN }
#[repr(C)] pub enum ras_errors { ADF_RAS_CORR, ADF_RAS_UNCORR, ADF_RAS_FATAL, ADF_RAS_ERRORS }
#[repr(C)] pub struct adf_error_counters { pub counter: [atomic_t; 3], pub sysfs_added: bool, pub enabled: bool }

pub unsafe fn get_sku_info(info: dev_sku_info) -> &'static [u8] { match info { dev_sku_info::DEV_SKU_1=>b"SKU1\0", dev_sku_info::DEV_SKU_2=>b"SKU2\0", dev_sku_info::DEV_SKU_3=>b"SKU3\0", dev_sku_info::DEV_SKU_4=>b"SKU4\0", dev_sku_info::DEV_SKU_VF=>b"SKUVF\0", _=>b"Unknown SKU\0" } }

#[repr(C)] pub struct adf_hw_device_class { pub name: *const core::ffi::c_char, pub type_: adf_device_type, pub instances: u32 }
#[repr(C)] pub struct arb_info { pub arb_cfg:u32, pub arb_offset:u32, pub wt2sam_offset:u32 }
#[repr(C)] pub struct admin_info { pub admin_msg_ur:u32, pub admin_msg_lr:u32, pub mailbox_offset:u32 }
pub struct adf_bank_state;

/* Function-pointer fields retain the C ABI and nullable callback semantics. */
#[repr(C)] pub struct adf_hw_csr_ops {
 pub build_csr_ring_base_addr: Option<unsafe extern "C" fn(dma_addr_t,u32)->u64>, pub read_csr_ring_head: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32)->u32>, pub write_csr_ring_head: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32,u32)>, pub read_csr_ring_tail: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32)->u32>, pub write_csr_ring_tail: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32,u32)>,
 pub read_csr_stat: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub read_csr_uo_stat: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub read_csr_e_stat: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub read_csr_ne_stat: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub read_csr_nf_stat: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub read_csr_f_stat: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub read_csr_c_stat: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub read_csr_exp_stat: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub read_csr_exp_int_en: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub write_csr_exp_int_en: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32)>,
 pub read_csr_ring_config: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32)->u32>, pub write_csr_ring_config: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32,u32)>, pub read_csr_ring_base: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32)->dma_addr_t>, pub write_csr_ring_base: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32,dma_addr_t)>, pub read_csr_int_en: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub write_csr_int_en: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32)>, pub read_csr_int_flag: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub write_csr_int_flag: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32)>, pub read_csr_int_srcsel: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub write_csr_int_srcsel: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)>, pub write_csr_int_srcsel_w_val: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32)>, pub read_csr_int_col_en: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub write_csr_int_col_en: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32)>, pub read_csr_int_col_ctl: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub write_csr_int_col_ctl: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32)>, pub read_csr_int_flag_and_col: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub write_csr_int_flag_and_col: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32)>, pub read_csr_ring_srv_arb_en: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->u32>, pub write_csr_ring_srv_arb_en: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32)>, pub get_int_col_ctl_enable_mask: Option<unsafe extern "C" fn()->u32>
}

#[repr(C)] pub struct adf_cfg_device_data; #[repr(C)] pub struct adf_accel_dev; #[repr(C)] pub struct adf_etr_data; #[repr(C)] pub struct adf_etr_ring_data;
#[repr(C)] pub struct adf_ras_ops { pub enable_ras_errors: Option<unsafe extern "C" fn(*mut adf_accel_dev)>, pub disable_ras_errors: Option<unsafe extern "C" fn(*mut adf_accel_dev)>, pub handle_interrupt: Option<unsafe extern "C" fn(*mut adf_accel_dev,*mut bool)->bool> }
#[repr(C)] pub struct adf_pfvf_ops { pub enable_comms: Option<unsafe extern "C" fn(*mut adf_accel_dev)->i32>, pub get_pf2vf_offset: Option<unsafe extern "C" fn(u32)->u32>, pub get_vf2pf_offset: Option<unsafe extern "C" fn(u32)->u32>, pub enable_vf2pf_interrupts: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)>, pub disable_all_vf2pf_interrupts: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub disable_pending_vf2pf_interrupts: Option<unsafe extern "C" fn(*mut core::ffi::c_void)->u32>, pub send_msg: Option<unsafe extern "C" fn(*mut adf_accel_dev,pfvf_message,u32,*mut mutex)->i32>, pub recv_msg: Option<unsafe extern "C" fn(*mut adf_accel_dev,u32,u8)->pfvf_message> }
#[repr(C)] pub struct adf_dc_ops { pub build_comp_block: Option<unsafe extern "C" fn(*mut core::ffi::c_void,adf_dc_algo)->i32>, pub build_decomp_block: Option<unsafe extern "C" fn(*mut core::ffi::c_void,adf_dc_algo)->i32> }
#[repr(C)] pub struct qat_migdev_ops { pub init: Option<unsafe extern "C" fn(*mut qat_mig_dev)->i32>, pub cleanup: Option<unsafe extern "C" fn(*mut qat_mig_dev)>, pub reset: Option<unsafe extern "C" fn(*mut qat_mig_dev)>, pub open: Option<unsafe extern "C" fn(*mut qat_mig_dev)->i32>, pub close: Option<unsafe extern "C" fn(*mut qat_mig_dev)>, pub suspend: Option<unsafe extern "C" fn(*mut qat_mig_dev)->i32>, pub resume: Option<unsafe extern "C" fn(*mut qat_mig_dev)->i32>, pub save_state: Option<unsafe extern "C" fn(*mut qat_mig_dev)->i32>, pub save_setup: Option<unsafe extern "C" fn(*mut qat_mig_dev)->i32>, pub load_state: Option<unsafe extern "C" fn(*mut qat_mig_dev)->i32>, pub load_setup: Option<unsafe extern "C" fn(*mut qat_mig_dev,i32)->i32> }
#[repr(C)] pub struct adf_dev_err_mask { pub cppagentcmdpar_mask:u32,pub parerr_ath_cph_mask:u32,pub parerr_cpr_xlt_mask:u32,pub parerr_dcpr_ucs_mask:u32,pub parerr_pke_mask:u32,pub parerr_wat_wcp_mask:u32,pub ssmfeatren_mask:u32 }

#[repr(C)] pub struct adf_hw_device_data {
 pub dev_class:*mut adf_hw_device_class,
 pub get_accel_mask:Option<unsafe extern "C" fn(*mut Self)->u32>, pub get_ae_mask:Option<unsafe extern "C" fn(*mut Self)->u32>, pub get_accel_cap:Option<unsafe extern "C" fn(*mut adf_accel_dev)->u32>, pub get_sram_bar_id:Option<unsafe extern "C" fn(*mut Self)->u32>, pub get_misc_bar_id:Option<unsafe extern "C" fn(*mut Self)->u32>, pub get_etr_bar_id:Option<unsafe extern "C" fn(*mut Self)->u32>, pub get_num_aes:Option<unsafe extern "C" fn(*mut Self)->u32>, pub get_num_accels:Option<unsafe extern "C" fn(*mut Self)->u32>,
 pub get_arb_info:Option<unsafe extern "C" fn(*mut arb_info)>, pub get_admin_info:Option<unsafe extern "C" fn(*mut admin_info)>, pub get_sku:Option<unsafe extern "C" fn(*mut Self)->dev_sku_info>, pub get_ring_to_svc_map:Option<unsafe extern "C" fn(*mut adf_accel_dev)->u16>, pub alloc_irq:Option<unsafe extern "C" fn(*mut adf_accel_dev)->i32>, pub free_irq:Option<unsafe extern "C" fn(*mut adf_accel_dev)>, pub enable_error_correction:Option<unsafe extern "C" fn(*mut adf_accel_dev)>, pub init_admin_comms:Option<unsafe extern "C" fn(*mut adf_accel_dev)->i32>, pub exit_admin_comms:Option<unsafe extern "C" fn(*mut adf_accel_dev)>, pub send_admin_init:Option<unsafe extern "C" fn(*mut adf_accel_dev)->i32>, pub start_timer:Option<unsafe extern "C" fn(*mut adf_accel_dev)->i32>, pub stop_timer:Option<unsafe extern "C" fn(*mut adf_accel_dev)>, pub check_hb_ctrs:Option<unsafe extern "C" fn(*mut adf_accel_dev)>, pub get_hb_clock:Option<unsafe extern "C" fn(*mut Self)->u32>, pub measure_clock:Option<unsafe extern "C" fn(*mut adf_accel_dev)->i32>, pub init_arb:Option<unsafe extern "C" fn(*mut adf_accel_dev)->i32>, pub exit_arb:Option<unsafe extern "C" fn(*mut adf_accel_dev)>, pub get_arb_mapping:Option<unsafe extern "C" fn(*mut adf_accel_dev)->*const u32>, pub init_device:Option<unsafe extern "C" fn(*mut adf_accel_dev)->i32>, pub enable_pm:Option<unsafe extern "C" fn(*mut adf_accel_dev)->i32>, pub handle_pm_interrupt:Option<unsafe extern "C" fn(*mut adf_accel_dev)->bool>, pub disable_iov:Option<unsafe extern "C" fn(*mut adf_accel_dev)>, pub configure_iov_threads:Option<unsafe extern "C" fn(*mut adf_accel_dev,bool)>, pub enable_ints:Option<unsafe extern "C" fn(*mut adf_accel_dev)>, pub set_ssm_wdtimer:Option<unsafe extern "C" fn(*mut adf_accel_dev)>, pub ring_pair_reset:Option<unsafe extern "C" fn(*mut adf_accel_dev,u32)->i32>, pub bank_state_save:Option<unsafe extern "C" fn(*mut adf_accel_dev,u32,*mut adf_bank_state)->i32>, pub bank_state_restore:Option<unsafe extern "C" fn(*mut adf_accel_dev,u32,*mut adf_bank_state)->i32>, pub reset_device:Option<unsafe extern "C" fn(*mut adf_accel_dev)>, pub set_msix_rttable:Option<unsafe extern "C" fn(*mut adf_accel_dev)>,
 pub uof_get_name:Option<unsafe extern "C" fn(*mut adf_accel_dev,u32)->*const core::ffi::c_char>, pub uof_get_num_objs:Option<unsafe extern "C" fn(*mut adf_accel_dev)->u32>, pub uof_get_obj_type:Option<unsafe extern "C" fn(*mut adf_accel_dev,u32)->i32>, pub uof_get_ae_mask:Option<unsafe extern "C" fn(*mut adf_accel_dev,u32)->u32>, pub get_rp_group:Option<unsafe extern "C" fn(*mut adf_accel_dev,u32)->i32>, pub get_ena_thd_mask:Option<unsafe extern "C" fn(*mut adf_accel_dev,u32)->u32>, pub dev_config:Option<unsafe extern "C" fn(*mut adf_accel_dev)->i32>, pub services_supported:Option<unsafe extern "C" fn(usize)->bool>, pub get_svc_slice_cnt:Option<unsafe extern "C" fn(*mut adf_accel_dev,adf_base_services)->u32>,
 pub pfvf_ops:adf_pfvf_ops, pub csr_ops:adf_hw_csr_ops, pub dc_ops:adf_dc_ops, pub ras_ops:adf_ras_ops, pub dev_err_mask:adf_dev_err_mask, pub rl_data:adf_rl_hw_data, pub tl_data:adf_tl_hw_data, pub anti_rb_data:adf_anti_rb_hw_data, pub kpt_data:adf_kpt_hw_data, pub vfmig_ops:qat_migdev_ops, pub fw_name:*const core::ffi::c_char, pub fw_mmp_name:*const core::ffi::c_char, pub fuses:[u32;6], pub straps:u32, pub accel_capabilities_mask:u32, pub accel_capabilities_ext_mask:u32, pub extended_dc_capabilities:u32, pub fw_capabilities:u16, pub clock_frequency:u32, pub instance_id:u32, pub accel_mask:u16, pub ae_mask:u32, pub admin_ae_mask:u32, pub tx_rings_mask:u16, pub ring_to_svc_map:u16, pub thd_to_arb_map:[u32; ICP_QAT_HW_AE_DELIMITER as usize], pub tx_rx_gap:u8, pub num_banks:u8, pub num_banks_per_vf:u16, pub num_rings_per_bank:u8, pub num_accel:u8, pub num_logical_accel:u8, pub num_engines:u8, pub num_hb_ctrs:u32, pub num_rps:u8
}

pub const ADF_CFG_NUM_SERVICES:u32=4; pub const ADF_SRV_TYPE_BIT_LEN:u32=3; pub const ADF_SRV_TYPE_MASK:u32=7; pub const ADF_AE_ADMIN_THREAD:u32=7; pub const ADF_NUM_THREADS_PER_AE:u32=8; pub const ADF_NUM_PKE_STRAND:u32=2; pub const ADF_AE_STRAND0_THREAD:u32=8; pub const ADF_AE_STRAND1_THREAD:u32=9;
/* C CSR and accessor macros are represented by the corresponding external low-level operations. */
pub unsafe fn get_srv_type(map:u16, idx:u32)->u32 { ((map as u32 >> (ADF_SRV_TYPE_BIT_LEN*idx)) & ADF_SRV_TYPE_MASK) }

#[repr(C)] pub struct adf_fw_loader_data { pub fw_loader:*mut icp_qat_fw_loader_handle, pub uof_fw:*const firmware, pub mmp_fw:*const firmware }
#[repr(C)] pub struct adf_accel_vf_info { pub accel_dev:*mut adf_accel_dev, pub pf2vf_lock:mutex, pub pfvf_mig_lock:mutex, pub vf2pf_ratelimit:ratelimit_state, pub vf_nr:u32, pub init:bool, pub restarting:bool, pub vf_compat_ver:u8, pub mig_priv:*mut core::ffi::c_void }
#[repr(C)] pub struct adf_dc_data { pub ovf_buff:*mut u8, pub ovf_buff_sz:usize, pub ovf_buff_p:dma_addr_t }
#[repr(C)] pub struct adf_pm { pub debugfs_pm_status:*mut dentry, pub present:bool, pub idle_irq_counters:i32, pub throttle_irq_counters:i32, pub fw_irq_counters:i32, pub host_ack_counter:i32, pub host_nack_counter:i32, pub print_pm_status:Option<unsafe extern "C" fn(*mut adf_accel_dev,*mut core::ffi::c_char,usize,*mut loff_t)->isize> }
#[repr(C)] pub struct adf_sysfs { pub ring_num:i32, pub lock:rw_semaphore }

#[repr(C)] pub union adf_accel_dev_role { pub pf: adf_accel_dev_pf, pub vf: adf_accel_dev_vf }
#[repr(C)] pub struct adf_accel_dev_pf { pub vf2pf_ints_lock:spinlock_t, pub vf2pf_disabled:bool, pub vf_info:*mut adf_accel_vf_info }
#[repr(C)] pub struct adf_accel_dev_vf { pub irq_enabled:bool, pub irq_name:[core::ffi::c_char;ADF_MAX_MSIX_VECTOR_NAME], pub pf2vf_bh_tasklet:tasklet_struct, pub vf2pf_lock:mutex, pub msg_received:completion, pub response:pfvf_message, pub pf_compat_ver:u8 }
#[repr(C)] pub struct adf_accel_dev { pub transport:*mut adf_etr_data,pub hw_device:*mut adf_hw_device_data,pub cfg:*mut adf_cfg_device_data,pub fw_loader:*mut adf_fw_loader_data,pub admin:*mut adf_admin_comms,pub telemetry:*mut adf_telemetry,pub dc_data:*mut adf_dc_data,pub power_management:adf_pm,pub crypto_list:list_head,pub compression_list:list_head,pub status:usize,pub ref_count:atomic_t,pub debugfs_dir:*mut dentry,pub fw_cntr_dbgfile:*mut dentry,pub cnv_dbgfile:*mut dentry,pub list:list_head,pub owner:*mut module,pub accel_pci_dev:adf_accel_pci,pub timer:*mut adf_timer,pub heartbeat:*mut adf_heartbeat,pub rate_limiting:*mut adf_rl,pub sysfs:adf_sysfs,pub role:adf_accel_dev_role,pub ras_errors:adf_error_counters,pub state_lock:mutex,pub is_vf:bool,pub autoreset_on_error:bool,pub accel_id:u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
