/* Translated from pdcpat.h. C preprocessor include/guard context omitted. */

pub const PDC_PAT_CELL: c_long = 64;
pub const PDC_PAT_CELL_GET_NUMBER: c_long = 0;
pub const PDC_PAT_CELL_GET_INFO: c_long = 1;
pub const PDC_PAT_CELL_MODULE: c_long = 2;
pub const PDC_PAT_CELL_SET_ATTENTION: c_long = 9;
pub const PDC_PAT_CELL_NUMBER_TO_LOC: c_long = 10;
pub const PDC_PAT_CELL_WALK_FABRIC: c_long = 11;
pub const PDC_PAT_CELL_GET_RDT_SIZE: c_long = 12;
pub const PDC_PAT_CELL_GET_RDT: c_long = 13;
pub const PDC_PAT_CELL_GET_LOCAL_PDH_SZ: c_long = 14;
pub const PDC_PAT_CELL_SET_LOCAL_PDH: c_long = 15;
pub const PDC_PAT_CELL_GET_REMOTE_PDH_SZ: c_long = 16;
pub const PDC_PAT_CELL_GET_REMOTE_PDH: c_long = 17;
pub const PDC_PAT_CELL_GET_DBG_INFO: c_long = 128;
pub const PDC_PAT_CELL_CHANGE_ALIAS: c_long = 129;

pub const IO_VIEW: c_ulong = 0;
pub const PA_VIEW: c_ulong = 1;

pub const PAT_ENTITY_CA: c_int = 0;
pub const PAT_ENTITY_PROC: c_int = 1;
pub const PAT_ENTITY_MEM: c_int = 2;
pub const PAT_ENTITY_SBA: c_int = 3;
pub const PAT_ENTITY_LBA: c_int = 4;
pub const PAT_ENTITY_PBC: c_int = 5;
pub const PAT_ENTITY_XBC: c_int = 6;
pub const PAT_ENTITY_RC: c_int = 7;

pub const PAT_PBNUM: c_int = 0;
pub const PAT_LMMIO: c_int = 1;
pub const PAT_GMMIO: c_int = 2;
pub const PAT_NPIOP: c_int = 3;
pub const PAT_PIOP: c_int = 4;
pub const PAT_AHPA: c_int = 5;
pub const PAT_UFO: c_int = 6;
pub const PAT_GNIP: c_int = 7;

pub const PDC_PAT_CHASSIS_LOG: c_long = 65;
pub const PDC_PAT_CHASSIS_WRITE_LOG: c_long = 0;
pub const PDC_PAT_CHASSIS_READ_LOG: c_long = 1;
pub const PDC_PAT_COMPLEX: c_long = 66;
pub const PDC_PAT_CPU: c_long = 67;
pub const PDC_PAT_CPU_INFO: c_long = 0;
pub const PDC_PAT_CPU_DELETE: c_long = 1;
pub const PDC_PAT_CPU_ADD: c_long = 2;
pub const PDC_PAT_CPU_GET_NUMBER: c_long = 3;
pub const PDC_PAT_CPU_GET_HPA: c_long = 4;
pub const PDC_PAT_CPU_STOP: c_long = 5;
pub const PDC_PAT_CPU_RENDEZVOUS: c_long = 6;
pub const PDC_PAT_CPU_GET_CLOCK_INFO: c_long = 7;
pub const PDC_PAT_CPU_GET_RENDEZVOUS_STATE: c_long = 8;
pub const PDC_PAT_CPU_GET_PDC_ENTRYPOINT: c_long = 11;
pub const PDC_PAT_CPU_PLUNGE_FABRIC: c_long = 128;
pub const PDC_PAT_CPU_UPDATE_CACHE_CLEANSING: c_long = 129;
pub const PDC_PAT_EVENT: c_long = 68;
pub const PDC_PAT_EVENT_GET_CAPS: c_long = 0;
pub const PDC_PAT_EVENT_SET_MODE: c_long = 1;
pub const PDC_PAT_EVENT_SCAN: c_long = 2;
pub const PDC_PAT_EVENT_HANDLE: c_long = 3;
pub const PDC_PAT_EVENT_GET_NB_CALL: c_long = 4;
pub const PDC_PAT_HPMC: c_long = 70;
pub const PDC_PAT_HPMC_RENDEZ_CPU: c_long = 0;
pub const PDC_PAT_HPMC_SET_PARAMS: c_long = 1;
pub const HPMC_SET_PARAMS_INTR: c_long = 1;
pub const HPMC_SET_PARAMS_WAKE: c_long = 2;
pub const PDC_PAT_IO: c_long = 71;
pub const PDC_PAT_IO_GET_SLOT_STATUS: c_long = 5;
pub const PDC_PAT_IO_GET_LOC_FROM_HARDWARE: c_long = 6;
pub const PDC_PAT_IO_GET_HARDWARE_FROM_LOC: c_long = 7;
pub const PDC_PAT_IO_GET_PCI_CONFIG_FROM_HW: c_long = 11;
pub const PDC_PAT_IO_GET_HW_FROM_PCI_CONFIG: c_long = 12;
pub const PDC_PAT_IO_READ_HOST_BRIDGE_INFO: c_long = 13;
pub const PDC_PAT_IO_CLEAR_HOST_BRIDGE_INFO: c_long = 14;
pub const PDC_PAT_IO_GET_PCI_ROUTING_TABLE_SIZE: c_long = 15;
pub const PDC_PAT_IO_GET_PCI_ROUTING_TABLE: c_long = 16;
pub const PDC_PAT_IO_GET_HINT_TABLE_SIZE: c_long = 17;
pub const PDC_PAT_IO_GET_HINT_TABLE: c_long = 18;
pub const PDC_PAT_IO_PCI_CONFIG_READ: c_long = 19;
pub const PDC_PAT_IO_PCI_CONFIG_WRITE: c_long = 20;
pub const PDC_PAT_IO_GET_NUM_IO_SLOTS: c_long = 21;
pub const PDC_PAT_IO_GET_LOC_IO_SLOTS: c_long = 22;
pub const PDC_PAT_IO_BAY_STATUS_INFO: c_long = 28;
pub const PDC_PAT_IO_GET_PROC_VIEW: c_long = 29;
pub const PDC_PAT_IO_PROG_SBA_DIR_RANGE: c_long = 30;

pub const PDC_PAT_MEM: c_long = 72;
pub const PDC_PAT_MEM_PD_INFO: c_long = 0;
pub const PDC_PAT_MEM_PD_CLEAR: c_long = 1;
pub const PDC_PAT_MEM_PD_READ: c_long = 2;
pub const PDC_PAT_MEM_PD_RESET: c_long = 3;
pub const PDC_PAT_MEM_CELL_INFO: c_long = 5;
pub const PDC_PAT_MEM_CELL_CLEAR: c_long = 6;
pub const PDC_PAT_MEM_CELL_READ: c_long = 7;
pub const PDC_PAT_MEM_CELL_RESET: c_long = 8;
pub const PDC_PAT_MEM_SETGM: c_long = 9;
pub const PDC_PAT_MEM_ADD_PAGE: c_long = 10;
pub const PDC_PAT_MEM_ADDRESS: c_long = 11;
pub const PDC_PAT_MEM_GET_TXT_SIZE: c_long = 12;
pub const PDC_PAT_MEM_GET_PD_TXT: c_long = 13;
pub const PDC_PAT_MEM_GET_CELL_TXT: c_long = 14;
pub const PDC_PAT_MEM_RD_STATE_INFO: c_long = 15;
pub const PDC_PAT_MEM_CLR_STATE_INFO: c_long = 16;
pub const PDC_PAT_MEM_CLEAN_RANGE: c_long = 128;
pub const PDC_PAT_MEM_GET_TBL_SIZE: c_long = 131;
pub const PDC_PAT_MEM_GET_TBL: c_long = 132;
pub const PDC_PAT_NVOLATILE: c_long = 73;
pub const PDC_PAT_NVOLATILE_READ: c_long = 0;
pub const PDC_PAT_NVOLATILE_WRITE: c_long = 1;
pub const PDC_PAT_NVOLATILE_GET_SIZE: c_long = 2;
pub const PDC_PAT_NVOLATILE_VERIFY: c_long = 3;
pub const PDC_PAT_NVOLATILE_INIT: c_long = 4;
pub const PDC_PAT_PD: c_long = 74;
pub const PDC_PAT_PD_GET_ADDR_MAP: c_long = 0;
pub const PDC_PAT_PD_GET_PDC_INTERF_REV: c_long = 1;
pub const PDC_PAT_PD_GET_PLATFORM_COUNTER: c_long = 10;
pub const PDC_PAT_CAPABILITY_BIT_PDC_SERIALIZE: c_ulong = 1 << 0;
pub const PDC_PAT_CAPABILITY_BIT_PDC_POLLING: c_ulong = 1 << 1;
pub const PDC_PAT_CAPABILITY_BIT_PDC_NBC: c_ulong = 1 << 2;
pub const PDC_PAT_CAPABILITY_BIT_PDC_UFO: c_ulong = 1 << 3;
pub const PDC_PAT_CAPABILITY_BIT_PDC_IODC_32: c_ulong = 1 << 4;
pub const PDC_PAT_CAPABILITY_BIT_PDC_IODC_64: c_ulong = 1 << 5;
pub const PDC_PAT_CAPABILITY_BIT_PDC_HPMC_RENDEZ: c_ulong = 1 << 6;
pub const PDC_PAT_CAPABILITY_BIT_SIMULTANEOUS_PTLB: c_ulong = 1 << 7;
pub const PAT_MEMORY_DESCRIPTOR: c_int = 1;
pub const PAT_MEMTYPE_MEMORY: c_int = 0;
pub const PAT_MEMTYPE_FIRMWARE: c_int = 4;
pub const PAT_MEMUSE_GENERAL: c_int = 0;
pub const PAT_MEMUSE_GI: c_int = 128;
pub const PAT_MEMUSE_GNI: c_int = 129;
pub const PDC_PAT_REGISTER_TOC: c_long = 75;
pub const PDC_PAT_TOC_REGISTER_VECTOR: c_long = 0;
pub const PDC_PAT_TOC_READ_VECTOR: c_long = 1;
pub const PDC_PAT_SYSTEM_INFO: c_long = 76;

#[inline]
pub const fn pat_get_cba(value: c_ulong) -> c_ulong { value & 0xfffffffffffff000 }
#[inline]
pub const fn pat_get_entity(value: c_ulong) -> c_ulong { (value >> 56) & 0xff }
#[inline]
pub const fn pat_get_dvi(value: c_ulong) -> c_ulong { (value >> 48) & 0xff }
#[inline]
pub const fn pat_get_ioc(value: c_ulong) -> c_ulong { (value >> 40) & 0xff }
#[inline]
pub const fn pat_get_mod_pages(value: c_ulong) -> c_ulong { value & 0xffffff }

#[repr(C)]
pub struct pdc_pat_cell_num { pub cell_num: c_ulong, pub cell_loc: c_ulong }
#[repr(C)]
pub struct pdc_pat_cpu_num { pub cpu_num: c_ulong, pub cpu_loc: c_ulong }
#[repr(C)]
pub struct pdc_pat_mem_retinfo {
    pub ke: c_uint,
    /* current_pdt_entries:16 and max_pdt_entries:16 */
    pub pdt_entries: c_uint,
    pub Cs_bitmap: c_ulong,
    pub Ic_bitmap: c_ulong,
    pub good_mem: c_ulong,
    pub first_dbe_loc: c_ulong,
    pub clear_time: c_ulong,
}
#[repr(C)]
pub struct pdc_pat_mem_cell_pdt_retinfo {
    /* reserved:32, cs:1, current_pdt_entries:15, ic:1, max_pdt_entries:15 */
    pub status: u64,
    pub good_mem: c_ulong,
    pub first_dbe_loc: c_ulong,
    pub clear_time: c_ulong,
}
#[repr(C)]
pub struct pdc_pat_mem_read_pd_retinfo { pub actual_count_bytes: c_ulong, pub pdt_entries: c_ulong }
#[repr(C)]
pub struct pdc_pat_mem_phys_mem_location {
    /* cabinet:8, ign1:8, ign2:8, cell_slot:8, ign3:8, dimm_slot:8, ign4:8,
       source:4, source_detail:4 */
    pub location: u64,
}
#[repr(C)]
pub struct pdc_pat_pd_addr_map_entry {
    pub entry_type: u8, pub reserve1: [u8; 5], pub memory_type: u8, pub memory_usage: u8,
    pub paddr: c_ulong, pub pages: c_uint, pub reserve2: c_uint, pub cell_map: c_ulong,
}

#[repr(C)]
pub struct pdc_pat_cell_info_rtn_block {
    pub pdc_rev: c_ulong, pub capabilities: c_ulong, pub reserved0: [c_ulong; 2],
    pub cell_info: c_ulong, pub cell_phys_location: c_ulong, pub cpu_info: c_ulong,
    pub cpu_speed: c_ulong, pub io_chassis_phys_location: c_ulong,
    pub cell_io_information: c_ulong, pub reserved1: [c_ulong; 2],
    pub io_slot_info_size: c_ulong,
    pub io_slot: [pdc_pat_io_slot; 16], pub cell_mem_size: c_ulong,
    pub cell_dimm_info_size: c_ulong, pub dimm_info: [c_ulong; 16],
    pub fabric_info_size: c_ulong, pub xbc: [pdc_pat_xbc; 32],
}
#[repr(C)]
pub struct pdc_pat_io_slot { pub header: c_ulong, pub info0: c_ulong, pub info1: c_ulong, pub phys_loc: c_ulong, pub hw_path: c_ulong }
#[repr(C)]
pub struct pdc_pat_xbc { pub fabric_info_xbc_port: c_ulong, pub rc_attached_to_xbc: c_ulong }

#[repr(C, align(8))]
pub struct pdc_pat_cell_mod_maddr_block {
    pub cba: c_ulong, pub mod_info: c_ulong, pub mod_location: c_ulong,
    pub mod_path: hardware_path, pub mod_: [c_ulong; 508],
}
pub type pdc_pat_cell_mod_maddr_block_t = pdc_pat_cell_mod_maddr_block;

#[cfg(target_pointer_width = "64")]
pub const fn is_pdc_pat() -> bool { PDC_TYPE_PAT == pdc_type }
#[cfg(target_pointer_width = "32")]
pub const fn is_pdc_pat() -> bool { false }

extern "C" {
    pub fn pdc_pat_get_irt_size(num_entries: *mut c_ulong, cell_num: c_ulong) -> c_int;
    pub fn pdc_pat_get_irt(r_addr: *mut c_void, cell_num: c_ulong) -> c_int;
    pub fn pdc_pat_get_PDC_entrypoint(pdc_entry: *mut c_ulong) -> c_int;
    pub fn pdc_pat_chassis_send_log(status: c_ulong, data: c_ulong) -> c_int;
    pub fn pdc_pat_cell_get_number(cell_info: *mut pdc_pat_cell_num) -> c_int;
    pub fn pdc_pat_cell_info(info: *mut pdc_pat_cell_info_rtn_block, actcnt: *mut c_ulong, offset: c_ulong, cell_number: c_ulong) -> c_int;
    pub fn pdc_pat_cell_module(actcnt: *mut c_ulong, ploc: c_ulong, mod_: c_ulong, view_type: c_ulong, mem_addr: *mut c_void) -> c_int;
    pub fn pdc_pat_cell_num_to_loc(arg0: *mut c_void, arg1: c_ulong) -> c_int;
    pub fn pdc_pat_cpu_get_number(cpu_info: *mut pdc_pat_cpu_num, hpa: c_ulong) -> c_int;
    pub fn pdc_pat_pd_get_addr_map(actual_len: *mut c_ulong, mem_addr: *mut c_void, count: c_ulong, offset: c_ulong) -> c_int;
    pub fn pdc_pat_pd_get_pdc_revisions(legacy_rev: *mut c_ulong, pat_rev: *mut c_ulong, pdc_cap: *mut c_ulong) -> c_int;
    pub fn pdc_pat_pd_get_platform_counter(addr: *mut *mut u64, freq: *mut c_ulong, uniq: *mut c_ulong) -> c_int;
    pub fn pdc_pat_io_pci_cfg_read(pci_addr: c_ulong, pci_size: c_int, val: *mut u32) -> c_int;
    pub fn pdc_pat_io_pci_cfg_write(pci_addr: c_ulong, pci_size: c_int, val: u32) -> c_int;
    pub fn pdc_pat_mem_pdt_info(rinfo: *mut pdc_pat_mem_retinfo) -> c_int;
    pub fn pdc_pat_mem_pdt_cell_info(rinfo: *mut pdc_pat_mem_cell_pdt_retinfo, cell: c_ulong) -> c_int;
    pub fn pdc_pat_mem_read_cell_pdt(pret: *mut pdc_pat_mem_read_pd_retinfo, pdt_entries_ptr: *mut c_ulong, max_entries: c_ulong) -> c_int;
    pub fn pdc_pat_mem_read_pd_pdt(pret: *mut pdc_pat_mem_read_pd_retinfo, pdt_entries_ptr: *mut c_ulong, count: c_ulong, offset: c_ulong) -> c_int;
    pub fn pdc_pat_mem_get_dimm_phys_location(pret: *mut pdc_pat_mem_phys_mem_location, phys_addr: c_ulong) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
