/* Translated from amdgpu_psp.h. Included C headers provide referenced types. */

pub const PSP_FENCE_BUFFER_SIZE: u32 = 0x1000;
pub const PSP_CMD_BUFFER_SIZE: u32 = 0x1000;
pub const PSP_1_MEG: u32 = 0x100000;
// PSP_TMR_SIZE depends on the external amdgpu device ASIC type.
pub const PSP_TMR_ALIGNMENT: u32 = 0x100000;
pub const PSP_FW_NAME_LEN: u32 = 0x24;
pub const MBOX_READY_MASK: u32 = 0x80000000;
pub const MBOX_STATUS_MASK: u32 = 0x0000ffff;
pub const MBOX_COMMAND_MASK: u32 = 0x00ff0000;
pub const MBOX_READY_FLAG: u32 = 0x80000000;
pub const C2PMSG_CMD_SPI_UPDATE_ROM_IMAGE_ADDR_LO: u32 = 0x2;
pub const C2PMSG_CMD_SPI_UPDATE_ROM_IMAGE_ADDR_HI: u32 = 0x3;
pub const C2PMSG_CMD_SPI_UPDATE_FLASH_IMAGE: u32 = 0x4;
pub const C2PMSG_CMD_SPI_GET_ROM_IMAGE_ADDR_LO: u32 = 0xf;
pub const C2PMSG_CMD_SPI_GET_ROM_IMAGE_ADDR_HI: u32 = 0x10;
pub const C2PMSG_CMD_SPI_GET_FLASH_IMAGE: u32 = 0x11;
pub const MBOX_TOS_READY_FLAG: u32 = GFX_FLAG_RESPONSE;
pub const MBOX_TOS_READY_MASK: u32 = GFX_CMD_RESPONSE_MASK | GFX_CMD_STATUS_MASK;
pub const MBOX_TOS_RESP_FLAG: u32 = GFX_FLAG_RESPONSE;
pub const MBOX_TOS_RESP_MASK: u32 = GFX_CMD_RESPONSE_MASK | GFX_CMD_STATUS_MASK;

pub const PSP_WAITREG_CHANGED: u32 = BIT(0);
pub const PSP_WAITREG_NOVERBOSE: u32 = BIT(1);
pub const AMDGPU_XGMI_MAX_CONNECTED_NODES: usize = 64;
pub const MEM_TRAIN_SYSTEM_SIGNATURE: u32 = 0x54534942;
pub const GDDR6_MEM_TRAINING_DATA_SIZE_IN_BYTES: u32 = 0x1000;
pub const GDDR6_MEM_TRAINING_OFFSET: u32 = 0x8000;
pub const BIST_MEM_TRAINING_ENCROACHED_SIZE: u32 = 0x2000000;
pub const PSP_RUNTIME_DB_SIZE_IN_BYTES: u32 = 0x10000;
pub const PSP_RUNTIME_DB_OFFSET: u32 = 0x100000;
pub const PSP_RUNTIME_DB_COOKIE_ID: u16 = 0x0ed5;
pub const PSP_RUNTIME_DB_VER_1: u16 = 0x0100;
pub const PSP_RUNTIME_DB_DIAG_ENTRY_MAX_COUNT: usize = 0x40;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum psp_shared_mem_size { PSP_ASD_SHARED_MEM_SIZE=0, PSP_XGMI_SHARED_MEM_SIZE=0x4000, PSP_RAS_SHARED_MEM_SIZE=0x4000, PSP_HDCP_SHARED_MEM_SIZE=0x4000, PSP_DTM_SHARED_MEM_SIZE=0x4000, PSP_RAP_SHARED_MEM_SIZE=0x4000, PSP_SECUREDISPLAY_SHARED_MEM_SIZE=0x4000 }
#[repr(C)] pub enum ta_type_id { TA_TYPE_XGMI=1, TA_TYPE_RAS, TA_TYPE_HDCP, TA_TYPE_DTM, TA_TYPE_RAP, TA_TYPE_SECUREDISPLAY, TA_TYPE_MAX_INDEX }
#[repr(C)] pub enum psp_bootloader_cmd { PSP_BL__LOAD_SYSDRV=0x10000, PSP_BL__LOAD_SOSDRV=0x20000, PSP_BL__LOAD_KEY_DATABASE=0x80000, PSP_BL__LOAD_SOCDRV=0xb0000, PSP_BL__LOAD_DBGDRV=0xc0000, PSP_BL__LOAD_HADDRV=0xc0000, PSP_BL__LOAD_INTFDRV=0xd0000, PSP_BL__LOAD_RASDRV=0xe0000, PSP_BL__LOAD_IPKEYMGRDRV=0xf0000, PSP_BL__DRAM_LONG_TRAIN=0x100000, PSP_BL__DRAM_SHORT_TRAIN=0x200000, PSP_BL__LOAD_TOS_SPL_TABLE=0x10000000, PSP_BL__LOAD_SPDMDRV=0x20000000 }
#[repr(C)] pub enum psp_ring_type { PSP_RING_TYPE__INVALID=0, PSP_RING_TYPE__UM=1, PSP_RING_TYPE__KM=2 }

#[repr(C)] pub struct psp_context {
    pub adev:*mut amdgpu_device, pub km_ring:psp_ring, pub cmd:*mut psp_gfx_cmd_resp,
    pub funcs:*const psp_funcs, pub ta_funcs:*const ta_funcs,
    pub fw_pri_bo:*mut amdgpu_bo, pub fw_pri_mc_addr:u64, pub fw_pri_buf:*mut core::ffi::c_void,
    pub sos_fw:*const firmware, pub sys:psp_bin_desc_real, pub sos:psp_bin_desc_real, pub toc:psp_bin_desc_real, pub kdb:psp_bin_desc_real, pub spl:psp_bin_desc_real, pub rl:psp_bin_desc_real, pub soc_drv:psp_bin_desc_real, pub intf_drv:psp_bin_desc_real, pub dbg_drv:psp_bin_desc_real, pub ras_drv:psp_bin_desc_real, pub ipkeymgr_drv:psp_bin_desc_real, pub spdm_drv:psp_bin_desc_real,
    pub tmr_bo:*mut amdgpu_bo, pub tmr_mc_addr:u64, pub asd_fw:*const firmware, pub toc_fw:*const firmware, pub cap_fw:*const firmware,
    pub fence_buf_bo:*mut amdgpu_bo, pub fence_buf_mc_addr:u64, pub fence_buf:*mut core::ffi::c_void,
    pub cmd_buf_bo:*mut amdgpu_bo, pub cmd_buf_mc_addr:u64, pub cmd_buf_mem:*mut psp_gfx_cmd_resp,
    pub fence_value:atomic_t, pub autoload_supported:bool, pub boot_time_tmr:bool, pub pmfw_centralized_cstate_management:bool,
    pub ta_fw:*const firmware, pub ta_fw_version:u32, pub cap_fw_version:u32, pub cap_feature_version:u32, pub cap_ucode_size:u32,
    pub asd_context:ta_context, pub xgmi_context:psp_xgmi_context, pub ras_context:psp_ras_context, pub hdcp_context:ta_cp_context, pub dtm_context:ta_cp_context, pub rap_context:ta_cp_context, pub securedisplay_context:ta_cp_context, pub mutex:mutex, pub mem_train_ctx:psp_memory_training_context,
    pub boot_cfg_bitmask:u32, pub sup_pd_fw_up:bool, pub sup_ifwi_up:bool, pub vbflash_tmp_buf:*mut core::ffi::c_char, pub vbflash_image_size:usize, pub vbflash_done:bool, pub spirom_dump_trip:*mut spirom_bo, pub ptl:amdgpu_ptl,
}
#[repr(C)] pub struct psp_xgmi_node_info;
#[repr(C)] pub struct psp_xgmi_topology_info;
#[repr(C)] pub struct psp_bin_desc;

#[repr(C)] pub struct psp_ring { pub ring_type: psp_ring_type, pub ring_mem: *mut psp_gfx_rb_frame, pub ring_mem_mc_addr: u64, pub ring_mem_handle: *mut core::ffi::c_void, pub ring_size: u32, pub ring_wptr: u32 }
#[repr(C)] pub enum psp_reg_prog_id { PSP_REG_IH_RB_CNTL=0, PSP_REG_IH_RB_CNTL_RING1=1, PSP_REG_IH_RB_CNTL_RING2=2, PSP_REG_MMHUB_L1_TLB_CNTL=25, PSP_REG_LAST }

pub type PspIntFn = Option<unsafe extern "C" fn(*mut psp_context) -> i32>;
pub type PspBoolFn = Option<unsafe extern "C" fn(*mut psp_context) -> bool>;
#[repr(C)] pub struct psp_funcs {
    pub init_microcode: PspIntFn, pub wait_for_bootloader: PspIntFn, pub bootloader_load_kdb: PspIntFn, pub bootloader_load_spl: PspIntFn, pub bootloader_load_sysdrv: PspIntFn, pub bootloader_load_soc_drv: PspIntFn, pub bootloader_load_intf_drv: PspIntFn, pub bootloader_load_dbg_drv: PspIntFn, pub bootloader_load_ras_drv: PspIntFn, pub bootloader_load_ipkeymgr_drv: PspIntFn, pub bootloader_load_spdm_drv: PspIntFn, pub bootloader_load_sos: PspIntFn,
    pub ring_create: Option<unsafe extern "C" fn(*mut psp_context, psp_ring_type)->i32>, pub ring_stop: Option<unsafe extern "C" fn(*mut psp_context, psp_ring_type)->i32>, pub ring_destroy: Option<unsafe extern "C" fn(*mut psp_context, psp_ring_type)->i32>, pub smu_reload_quirk: PspBoolFn, pub mode1_reset: PspIntFn, pub mem_training: Option<unsafe extern "C" fn(*mut psp_context,u32)->i32>, pub ring_get_wptr: Option<unsafe extern "C" fn(*mut psp_context)->u32>, pub ring_set_wptr: Option<unsafe extern "C" fn(*mut psp_context,u32)>, pub load_usbc_pd_fw: Option<unsafe extern "C" fn(*mut psp_context,u64)->i32>, pub read_usbc_pd_fw: Option<unsafe extern "C" fn(*mut psp_context,*mut u32)->i32>, pub update_spirom: Option<unsafe extern "C" fn(*mut psp_context,u64)->i32>, pub dump_spirom: Option<unsafe extern "C" fn(*mut psp_context,u64)->i32>, pub vbflash_stat: PspIntFn, pub fatal_error_recovery_quirk: PspIntFn, pub get_ras_capability: PspBoolFn, pub is_aux_sos_load_required: PspBoolFn, pub is_reload_needed: PspBoolFn,
    pub reg_program_no_ring: Option<unsafe extern "C" fn(*mut psp_context,u32,psp_reg_prog_id)->i32>, pub get_fw_type: Option<unsafe extern "C" fn(*mut amdgpu_firmware_info,*mut psp_gfx_fw_type)->i32>,
}
#[repr(C)] pub struct ta_funcs { pub fn_ta_initialize:PspIntFn, pub fn_ta_invoke:Option<unsafe extern "C" fn(*mut psp_context,u32)->i32>, pub fn_ta_terminate:PspIntFn }

#[repr(C)] pub struct psp_xgmi_node_info_real { pub node_id:u64, pub num_hops:u8, pub is_sharing_enabled:u8, pub sdma_engine:ta_xgmi_assigned_sdma_engine, pub num_links:u8, pub port_num:[xgmi_connected_port_num; TA_XGMI__MAX_PORT_NUM as usize] }
#[repr(C)] pub struct psp_xgmi_topology_info_real { pub num_nodes:u32, pub nodes:[psp_xgmi_node_info_real; AMDGPU_XGMI_MAX_CONNECTED_NODES] }
#[repr(C)] pub struct psp_bin_desc_real { pub fw_version:u32, pub feature_version:u32, pub size_bytes:u32, pub start_addr:*mut u8 }
#[repr(C)] pub struct ta_mem_context { pub shared_bo:*mut amdgpu_bo, pub shared_mc_addr:u64, pub shared_buf:*mut core::ffi::c_void, pub shared_mem_size:psp_shared_mem_size }
#[repr(C)] pub struct ta_context { pub initialized:bool, pub session_id:u32, pub resp_status:u32, pub mem_context:ta_mem_context, pub bin_desc:psp_bin_desc_real, pub ta_load_type:psp_gfx_cmd_id, pub ta_type:ta_type_id }
#[repr(C)] pub struct ta_cp_context { pub context:ta_context, pub mutex:mutex }
#[repr(C)] pub struct psp_xgmi_context { pub context:ta_context, pub top_info:psp_xgmi_topology_info_real, pub supports_extended_data:bool, pub supports_ext_link_info:bool, pub xgmi_ta_caps:u8 }
#[repr(C)] pub struct psp_ras_context { pub context:ta_context, pub ras:*mut amdgpu_ras, pub mutex:mutex }

#[repr(C)] pub enum psp_memory_training_init_flag { PSP_MEM_TRAIN_NOT_SUPPORT=0, PSP_MEM_TRAIN_SUPPORT=1, PSP_MEM_TRAIN_INIT_FAILED=2, PSP_MEM_TRAIN_RESERVE_SUCCESS=4, PSP_MEM_TRAIN_INIT_SUCCESS=8 }
#[repr(C)] pub enum psp_memory_training_ops { PSP_MEM_TRAIN_SEND_LONG_MSG=1, PSP_MEM_TRAIN_SAVE=2, PSP_MEM_TRAIN_RESTORE=4, PSP_MEM_TRAIN_SEND_SHORT_MSG=8, PSP_MEM_TRAIN_COLD_BOOT=1, PSP_MEM_TRAIN_RESUME=8 }
#[repr(C)] pub struct psp_memory_training_context { pub train_data_size:u64, pub sys_cache:*mut core::ffi::c_void, pub p2c_train_data_offset:u64, pub c2p_train_data_offset:u64, pub init:psp_memory_training_init_flag, pub training_cnt:u32, pub enable_mem_training:bool }
#[repr(C)] pub enum psp_runtime_entry_type { PSP_RUNTIME_ENTRY_TYPE_INVALID=0, PSP_RUNTIME_ENTRY_TYPE_TEST=1, PSP_RUNTIME_ENTRY_TYPE_MGPU_COMMON=2, PSP_RUNTIME_ENTRY_TYPE_MGPU_WAFL=3, PSP_RUNTIME_ENTRY_TYPE_MGPU_XGMI=4, PSP_RUNTIME_ENTRY_TYPE_BOOT_CONFIG=5, PSP_RUNTIME_ENTRY_TYPE_PPTABLE_ERR_STATUS=6 }
#[repr(C)] pub struct psp_runtime_data_header { pub cookie:u16, pub version:u16 }
#[repr(C)] pub struct psp_runtime_entry { pub entry_type:u32, pub offset:u16, pub size:u16 }
#[repr(C)] pub struct psp_runtime_data_directory { pub entry_count:u16, pub entry_list:[psp_runtime_entry; PSP_RUNTIME_DB_DIAG_ENTRY_MAX_COUNT] }
#[repr(C)] pub enum psp_runtime_boot_cfg_feature { BOOT_CFG_FEATURE_GECC=1, BOOT_CFG_FEATURE_TWO_STAGE_DRAM_TRAINING=2 }
#[repr(C)] pub enum psp_runtime_scpm_authentication { SCPM_DISABLE=0, SCPM_ENABLE=1, SCPM_ENABLE_WITH_SCPM_ERR=2 }
#[repr(C)] pub struct psp_runtime_boot_cfg_entry { pub boot_cfg_bitmask:u32, pub reserved:u32 }
#[repr(C)] pub struct psp_runtime_scpm_entry { pub scpm_status:psp_runtime_scpm_authentication }

#[repr(C)] pub enum psp_ptl_cmd { PSP_PTL_PERF_MON_QUERY=0xa0000000, PSP_PTL_PERF_MON_SET=0xa0000001 }
#[repr(C)] pub enum psp_ptl_format_type { GFX_FTYPE_I8=0, GFX_FTYPE_F16=1, GFX_FTYPE_BF16=2, GFX_FTYPE_F32=3, GFX_FTYPE_F64=4, GFX_FTYPE_F8=5, GFX_FTYPE_VECTOR=6, GFX_FTYPE_INVALID=0xffffffff }
#[repr(C)] pub struct psp_ptl_perf_req { pub req:psp_ptl_cmd, pub ptl_state:u32, pub pref_format1:u32, pub pref_format2:u32 }

/* CONFIG_DEBUG_FS conditionally contains spirom_bo in the C header. */
#[repr(C)] pub struct spirom_bo { pub bo:*mut amdgpu_bo, pub mc_addr:u64, pub cpu_addr:*mut core::ffi::c_void }
#[repr(C)] pub struct amdgpu_psp_funcs { pub check_fw_loading_status:Option<unsafe extern "C" fn(*mut amdgpu_device,AMDGPU_UCODE_ID)->bool> }

/* C preprocessor dispatch helpers, retained as Rust functions. */
pub unsafe fn psp_ring_create(psp:*mut psp_context, ty:psp_ring_type)->i32 { ((*(*psp).funcs).ring_create.unwrap())(psp,ty) }
pub unsafe fn psp_ring_stop(psp:*mut psp_context, ty:psp_ring_type)->i32 { ((*(*psp).funcs).ring_stop.unwrap())(psp,ty) }
pub unsafe fn psp_ring_destroy(psp:*mut psp_context, ty:psp_ring_type)->i32 { ((*(*psp).funcs).ring_destroy.unwrap())(psp,ty) }

extern "C" {
    pub static amdgpu_flash_attr_group: attribute_group;
    pub static psp_ip_funcs: amd_ip_funcs;
    pub static psp_v3_1_ip_block: amdgpu_ip_block_version; pub static psp_v10_0_ip_block: amdgpu_ip_block_version; pub static psp_v11_0_ip_block: amdgpu_ip_block_version; pub static psp_v11_0_8_ip_block: amdgpu_ip_block_version; pub static psp_v12_0_ip_block: amdgpu_ip_block_version; pub static psp_v13_0_ip_block: amdgpu_ip_block_version; pub static psp_v13_0_4_ip_block: amdgpu_ip_block_version; pub static psp_v14_0_ip_block: amdgpu_ip_block_version; pub static psp_v15_0_ip_block: amdgpu_ip_block_version; pub static psp_v15_0_8_ip_block: amdgpu_ip_block_version;
    pub fn psp_wait_for(psp:*mut psp_context, reg_index:u32, field_val:u32, mask:u32, flags:u32)->i32;
    pub fn psp_wait_for_spirom_update(psp:*mut psp_context, reg_index:u32, field_val:u32, mask:u32, msec_timeout:u32)->i32;
    pub fn psp_execute_ip_fw_load(psp:*mut psp_context, ucode:*mut amdgpu_firmware_info)->i32;
    pub fn psp_gpu_reset(adev:*mut amdgpu_device)->i32;
    pub fn psp_ta_init_shared_buf(psp:*mut psp_context, mem_ctx:*mut ta_mem_context)->i32; pub fn psp_ta_free_shared_buf(mem_ctx:*mut ta_mem_context); pub fn psp_ta_unload(psp:*mut psp_context, context:*mut ta_context)->i32; pub fn psp_ta_load(psp:*mut psp_context, context:*mut ta_context)->i32; pub fn psp_ta_invoke(psp:*mut psp_context, ta_cmd_id:u32, context:*mut ta_context)->i32;
    pub fn psp_xgmi_initialize(psp:*mut psp_context, set_extended_data:bool, load_ta:bool)->i32; pub fn psp_xgmi_terminate(psp:*mut psp_context)->i32; pub fn psp_xgmi_invoke(psp:*mut psp_context, ta_cmd_id:u32)->i32; pub fn psp_xgmi_get_hive_id(psp:*mut psp_context, hive_id:*mut u64)->i32; pub fn psp_xgmi_get_node_id(psp:*mut psp_context, node_id:*mut u64)->i32;
    pub fn psp_ras_initialize(psp:*mut psp_context)->i32; pub fn psp_ras_invoke(psp:*mut psp_context, ta_cmd_id:u32)->i32; pub fn psp_ras_terminate(psp:*mut psp_context)->i32;
    pub fn psp_hdcp_invoke(psp:*mut psp_context, ta_cmd_id:u32)->i32; pub fn psp_dtm_invoke(psp:*mut psp_context, ta_cmd_id:u32)->i32; pub fn psp_securedisplay_invoke(psp:*mut psp_context, ta_cmd_id:u32)->i32; pub fn psp_rlc_autoload_start(psp:*mut psp_context)->i32;
    pub fn psp_reg_program(psp:*mut psp_context, reg:psp_reg_prog_id, value:u32)->i32; pub fn psp_ring_cmd_submit(psp:*mut psp_context, cmd_buf_mc_addr:u64, fence_mc_addr:u64, index:i32)->i32;
    pub fn psp_init_asd_microcode(psp:*mut psp_context, chip_name:*const core::ffi::c_char)->i32; pub fn psp_init_toc_microcode(psp:*mut psp_context, chip_name:*const core::ffi::c_char)->i32; pub fn psp_init_sos_microcode(psp:*mut psp_context, chip_name:*const core::ffi::c_char)->i32; pub fn psp_init_ta_microcode(psp:*mut psp_context, chip_name:*const core::ffi::c_char)->i32; pub fn psp_init_cap_microcode(psp:*mut psp_context, chip_name:*const core::ffi::c_char)->i32;
    pub fn psp_update_fw_reservation(psp:*mut psp_context)->i32; pub fn psp_spatial_partition(psp:*mut psp_context, mode:i32)->i32; pub fn psp_memory_partition(psp:*mut psp_context, mode:i32)->i32; pub fn is_psp_fw_valid(bin:psp_bin_desc_real)->i32; pub fn amdgpu_psp_wait_for_bootloader(adev:*mut amdgpu_device)->i32; pub fn amdgpu_psp_get_ras_capability(psp:*mut psp_context)->bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
