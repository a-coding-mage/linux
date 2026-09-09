/* Generic EDAC defs (translated from linux/edac.h). */

/* Kernel-provided types and constants are external dependencies. */
use core::ffi::c_char;

pub const EDAC_DEVICE_NAME_LEN: usize = 31;
pub const EDAC_OPSTATE_INVAL: i32 = -1;
pub const EDAC_OPSTATE_POLL: i32 = 0;
pub const EDAC_OPSTATE_NMI: i32 = 1;
pub const EDAC_OPSTATE_INT: i32 = 2;

extern "C" {
    pub static mut edac_op_state: i32;
    pub fn edac_get_sysfs_subsys() -> *const bus_type;
}

#[inline]
pub unsafe fn opstate_init() {
    match edac_op_state {
        EDAC_OPSTATE_POLL | EDAC_OPSTATE_NMI => {}
        _ => edac_op_state = EDAC_OPSTATE_POLL,
    }
}

pub const EDAC_MC_LABEL_LEN: usize = 31;
pub const LOCATION_SIZE: usize = 256;
pub const EDAC_MAX_LABELS: usize = 8;
pub const OTHER_LABEL: &[u8] = b" or \0";

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dev_type { DEV_UNKNOWN = 0, DEV_X1, DEV_X2, DEV_X4, DEV_X8, DEV_X16, DEV_X32, DEV_X64 }
pub const DEV_FLAG_UNKNOWN: u32 = 1 << 0;
pub const DEV_FLAG_X1: u32 = 1 << 1;
pub const DEV_FLAG_X2: u32 = 1 << 2;
pub const DEV_FLAG_X4: u32 = 1 << 3;
pub const DEV_FLAG_X8: u32 = 1 << 4;
pub const DEV_FLAG_X16: u32 = 1 << 5;
pub const DEV_FLAG_X32: u32 = 1 << 6;
pub const DEV_FLAG_X64: u32 = 1 << 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hw_event_mc_err_type { HW_EVENT_ERR_CORRECTED, HW_EVENT_ERR_UNCORRECTED, HW_EVENT_ERR_DEFERRED, HW_EVENT_ERR_FATAL, HW_EVENT_ERR_INFO }

#[inline]
pub unsafe fn mc_event_error_type(err_type: u32) -> *const c_char {
    match err_type {
        0 => b"Corrected\0".as_ptr() as *const c_char,
        1 => b"Uncorrected\0".as_ptr() as *const c_char,
        2 => b"Deferred\0".as_ptr() as *const c_char,
        3 => b"Fatal\0".as_ptr() as *const c_char,
        _ => b"Info\0".as_ptr() as *const c_char,
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mem_type { MEM_EMPTY = 0, MEM_RESERVED, MEM_UNKNOWN, MEM_FPM, MEM_EDO, MEM_BEDO, MEM_SDR, MEM_RDR, MEM_DDR, MEM_RDDR, MEM_RMBS, MEM_DDR2, MEM_FB_DDR2, MEM_RDDR2, MEM_XDR, MEM_DDR3, MEM_RDDR3, MEM_LRDDR3, MEM_LPDDR3, MEM_DDR4, MEM_RDDR4, MEM_LRDDR4, MEM_LPDDR4, MEM_DDR5, MEM_RDDR5, MEM_LRDDR5, MEM_LPDDR5, MEM_NVDIMM, MEM_WIO2, MEM_HBM2, MEM_HBM3 }

/* BIT(enum_value) constants. */
pub const MEM_FLAG_EMPTY: u32 = 1<<0; pub const MEM_FLAG_RESERVED: u32 = 1<<1; pub const MEM_FLAG_UNKNOWN: u32 = 1<<2; pub const MEM_FLAG_FPM: u32 = 1<<3; pub const MEM_FLAG_EDO: u32 = 1<<4; pub const MEM_FLAG_BEDO: u32 = 1<<5; pub const MEM_FLAG_SDR: u32 = 1<<6; pub const MEM_FLAG_RDR: u32 = 1<<7; pub const MEM_FLAG_DDR: u32 = 1<<8; pub const MEM_FLAG_RDDR: u32 = 1<<9; pub const MEM_FLAG_RMBS: u32 = 1<<10; pub const MEM_FLAG_DDR2: u32 = 1<<11; pub const MEM_FLAG_FB_DDR2: u32 = 1<<12; pub const MEM_FLAG_RDDR2: u32 = 1<<13; pub const MEM_FLAG_XDR: u32 = 1<<14; pub const MEM_FLAG_DDR3: u32 = 1<<15; pub const MEM_FLAG_RDDR3: u32 = 1<<16; pub const MEM_FLAG_LPDDR3: u32 = 1<<18; pub const MEM_FLAG_DDR4: u32 = 1<<19; pub const MEM_FLAG_RDDR4: u32 = 1<<20; pub const MEM_FLAG_LRDDR4: u32 = 1<<21; pub const MEM_FLAG_LPDDR4: u32 = 1<<22; pub const MEM_FLAG_DDR5: u32 = 1<<23; pub const MEM_FLAG_RDDR5: u32 = 1<<24; pub const MEM_FLAG_LRDDR5: u32 = 1<<25; pub const MEM_FLAG_LPDDR5: u32 = 1<<26; pub const MEM_FLAG_NVDIMM: u32 = 1<<27; pub const MEM_FLAG_WIO2: u32 = 1<<28; pub const MEM_FLAG_HBM2: u32 = 1<<29; pub const MEM_FLAG_HBM3: u32 = 1<<30;

#[repr(C)] pub enum edac_type { EDAC_UNKNOWN=0, EDAC_NONE, EDAC_RESERVED, EDAC_PARITY, EDAC_EC, EDAC_SECDED, EDAC_S2ECD2ED, EDAC_S4ECD4ED, EDAC_S8ECD8ED, EDAC_S16ECD16ED }
#[repr(C)] pub enum scrub_type { SCRUB_UNKNOWN=0, SCRUB_NONE, SCRUB_SW_PROG, SCRUB_SW_SRC, SCRUB_SW_PROG_SRC, SCRUB_SW_TUNABLE, SCRUB_HW_PROG, SCRUB_HW_SRC, SCRUB_HW_PROG_SRC, SCRUB_HW_TUNABLE }
pub const EDAC_MAX_LAYERS: usize = 3;
pub const OP_ALLOC: i32 = 0x100; pub const OP_RUNNING_POLL: i32 = 0x201; pub const OP_RUNNING_INTERRUPT: i32 = 0x202; pub const OP_RUNNING_POLL_INTR: i32 = 0x203; pub const OP_OFFLINE: i32 = 0x300;

#[repr(C)] pub enum edac_mc_layer_type { EDAC_MC_LAYER_BRANCH, EDAC_MC_LAYER_CHANNEL, EDAC_MC_LAYER_SLOT, EDAC_MC_LAYER_CHIP_SELECT, EDAC_MC_LAYER_ALL_MEM }
#[repr(C)] pub struct edac_mc_layer { pub type_: edac_mc_layer_type, pub size: u32, pub is_virt_csrow: bool }

/* Remaining structures retain the C ABI and refer to kernel-supplied types. */
extern "C" {
    pub fn edac_dev_register(parent: *mut device, dev_name: *mut c_char, parent_pvt_data: *mut core::ffi::c_void, num_features: i32, ras_features: *const edac_dev_feature) -> i32;
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }
#[repr(C)] pub struct edac_dev_feature { pub ft_type: edac_dev_feat, pub instance: u8, pub ops: *const core::ffi::c_void, pub ctx: *mut core::ffi::c_void, pub ecs_info: edac_ecs_ex_info }
#[repr(C)] pub enum edac_dev_feat { RAS_FEAT_SCRUB, RAS_FEAT_ECS, RAS_FEAT_MEM_REPAIR, RAS_FEAT_MAX }
#[repr(C)] pub struct edac_ecs_ex_info { pub num_media_frus: u16 }

#[repr(C)] pub struct dimm_info { pub dev: device, pub label: [c_char; EDAC_MC_LABEL_LEN+1], pub location: [u32; EDAC_MAX_LAYERS], pub mci: *mut mem_ctl_info, pub idx: u32, pub grain: u32, pub dtype: dev_type, pub mtype: mem_type, pub edac_mode: edac_type, pub nr_pages: u32, pub csrow: u32, pub cschannel: u32, pub smbios_handle: u16, pub ce_count: u32, pub ue_count: u32 }
#[repr(C)] pub struct rank_info { pub chan_idx: i32, pub csrow: *mut csrow_info, pub dimm: *mut dimm_info, pub ce_count: u32 }
#[repr(C)] pub struct csrow_info { pub dev: device, pub first_page: usize, pub last_page: usize, pub page_mask: usize, pub csrow_idx: i32, pub ue_count: u32, pub ce_count: u32, pub mci: *mut mem_ctl_info, pub nr_channels: u32, pub channels: *mut *mut rank_info }
#[repr(C)] pub struct errcount_attribute_data { pub n_layers: i32, pub pos: [i32; EDAC_MAX_LAYERS], pub layer0: i32, pub layer1: i32, pub layer2: i32 }
#[repr(C)] pub struct edac_raw_error_desc { pub location: [c_char; LOCATION_SIZE], pub label: [c_char; (EDAC_MC_LABEL_LEN+1+5)*EDAC_MAX_LABELS], pub grain: isize, pub error_count: u16, pub type_: hw_event_mc_err_type, pub top_layer: i32, pub mid_layer: i32, pub low_layer: i32, pub page_frame_number: usize, pub offset_in_page: usize, pub syndrome: usize, pub msg: *const c_char, pub other_detail: *const c_char }
#[repr(C)] pub struct mem_ctl_info { pub dev: device, pub bus: *const bus_type, pub mtype_cap: usize, pub edac_ctl_cap: usize, pub edac_cap: usize, pub scrub_cap: usize, pub scrub_mode: scrub_type, pub set_sdram_scrub_rate: Option<unsafe extern "C" fn(*mut mem_ctl_info,u32)->i32>, pub get_sdram_scrub_rate: Option<unsafe extern "C" fn(*mut mem_ctl_info)->i32>, pub edac_check: Option<unsafe extern "C" fn(*mut mem_ctl_info)>, pub ctl_page_to_phys: Option<unsafe extern "C" fn(*mut mem_ctl_info,usize)->usize>, pub mc_idx: i32, pub csrows: *mut *mut csrow_info, pub nr_csrows: u32, pub num_cschannel: u32, pub csbased: bool, pub tot_dimms: u32, pub dimms: *mut *mut dimm_info, pub pdev: *mut device, pub mod_name: *const c_char, pub ctl_name: *const c_char, pub dev_name: *const c_char, pub pvt_info: *mut core::ffi::c_void, pub start_time: usize, pub ce_noinfo_count: u32, pub ue_noinfo_count: u32, pub ue_mc: u32, pub ce_mc: u32, pub error_desc: edac_raw_error_desc, pub op_state: i32, pub n_layers: u32, pub layers: [edac_mc_layer; 0] }
#[repr(C)] pub struct edac_scrub_ops { pub read_addr: *const core::ffi::c_void, pub read_size: *const core::ffi::c_void, pub write_addr: *const core::ffi::c_void, pub write_size: *const core::ffi::c_void, pub get_enabled_bg: *const core::ffi::c_void, pub set_enabled_bg: *const core::ffi::c_void, pub get_min_cycle: *const core::ffi::c_void, pub get_max_cycle: *const core::ffi::c_void, pub get_cycle_duration: *const core::ffi::c_void, pub set_cycle_duration: *const core::ffi::c_void }
#[repr(C)] pub struct edac_ecs_ops { pub get_log_entry_type: *const core::ffi::c_void, pub set_log_entry_type: *const core::ffi::c_void, pub get_mode: *const core::ffi::c_void, pub set_mode: *const core::ffi::c_void, pub reset: *const core::ffi::c_void, pub get_threshold: *const core::ffi::c_void, pub set_threshold: *const core::ffi::c_void }
#[repr(C)] pub struct edac_mem_repair_ops { pub get_repair_type: *const core::ffi::c_void, pub get_persist_mode: *const core::ffi::c_void, pub set_persist_mode: *const core::ffi::c_void, pub get_repair_safe_when_in_use: *const core::ffi::c_void, pub get_hpa: *const core::ffi::c_void, pub set_hpa: *const core::ffi::c_void, pub get_min_hpa: *const core::ffi::c_void, pub get_max_hpa: *const core::ffi::c_void, pub get_dpa: *const core::ffi::c_void, pub set_dpa: *const core::ffi::c_void, pub do_repair: *const core::ffi::c_void }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
