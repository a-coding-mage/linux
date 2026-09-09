/* SPDX-License-Identifier: GPL-2.0-or-later */

/* UV BIOS layer definitions. */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum uv_bios_cmd {
    UV_BIOS_COMMON,
    UV_BIOS_GET_SN_INFO,
    UV_BIOS_FREQ_BASE,
    UV_BIOS_WATCHLIST_ALLOC,
    UV_BIOS_WATCHLIST_FREE,
    UV_BIOS_MEMPROTECT,
    UV_BIOS_GET_PARTITION_ADDR,
    UV_BIOS_SET_LEGACY_VGA_TARGET,
}

pub const UV_BIOS_EXTRA: u64 = 0x10000;
pub const UV_BIOS_GET_PCI_TOPOLOGY: u64 = 0x10001;
pub const UV_BIOS_GET_GEOINFO: u64 = 0x10003;
pub const UV_BIOS_EXTRA_OP_MEM_COPYIN: u64 = 0x1000;
pub const UV_BIOS_EXTRA_OP_MEM_COPYOUT: u64 = 0x2000;
pub const UV_BIOS_EXTRA_OP_MASK: u64 = 0x0fff;
pub const UV_BIOS_EXTRA_GET_HEAPSIZE: u64 = 1;
pub const UV_BIOS_EXTRA_INSTALL_HEAP: u64 = 2;
pub const UV_BIOS_EXTRA_MASTER_NASID: u64 = 3;
pub const UV_BIOS_EXTRA_OBJECT_COUNT: u64 = 10 | UV_BIOS_EXTRA_OP_MEM_COPYOUT;
pub const UV_BIOS_EXTRA_ENUM_OBJECTS: u64 = 12 | UV_BIOS_EXTRA_OP_MEM_COPYOUT;
pub const UV_BIOS_EXTRA_ENUM_PORTS: u64 = 13 | UV_BIOS_EXTRA_OP_MEM_COPYOUT;

pub const BIOS_STATUS_MORE_PASSES: i32 = 1;
pub const BIOS_STATUS_SUCCESS: i32 = 0;
pub const BIOS_STATUS_UNIMPLEMENTED: i32 = -38; // -ENOSYS
pub const BIOS_STATUS_EINVAL: i32 = -22; // -EINVAL
pub const BIOS_STATUS_UNAVAIL: i32 = -16; // -EBUSY
pub const BIOS_STATUS_ABORT: i32 = -4; // -EINTR

#[repr(C)]
pub struct uv_gam_parameters {
    pub mmr_base: u64,
    pub gru_base: u64,
    pub mmr_shift: u8,
    pub gru_shift: u8,
    pub gpa_shift: u8,
    pub unused1: u8,
}

pub const UV_GAM_RANGE_TYPE_UNUSED: u8 = 0;
pub const UV_GAM_RANGE_TYPE_RAM: u8 = 1;
pub const UV_GAM_RANGE_TYPE_NVRAM: u8 = 2;
pub const UV_GAM_RANGE_TYPE_NV_WINDOW: u8 = 3;
pub const UV_GAM_RANGE_TYPE_NV_MAILBOX: u8 = 4;
pub const UV_GAM_RANGE_TYPE_HOLE: u8 = 5;
pub const UV_GAM_RANGE_TYPE_MAX: u8 = 6;
pub const UV_GAM_RANGE_SHFT: u32 = 26;

#[repr(C)]
pub struct uv_gam_range_entry {
    pub type_: i8,
    pub unused1: i8,
    pub nasid: u16,
    pub sockid: u16,
    pub pnode: u16,
    pub unused2: u32,
    pub limit: u32,
}

pub const UV_AT_SIZE: usize = 8;
#[repr(C)]
pub struct uv_arch_type_entry { pub archtype: [i8; UV_AT_SIZE] }

pub const UV_SYSTAB_SIG: &[u8; 4] = b"UVST";
pub const UV_SYSTAB_VERSION_1: u32 = 1;
pub const UV_SYSTAB_VERSION_UV4: u32 = 0x400;
pub const UV_SYSTAB_VERSION_UV4_1: u32 = 0x401;
pub const UV_SYSTAB_VERSION_UV4_2: u32 = 0x402;
pub const UV_SYSTAB_VERSION_UV4_3: u32 = 0x403;
pub const UV_SYSTAB_VERSION_UV4_LATEST: u32 = UV_SYSTAB_VERSION_UV4_3;
pub const UV_SYSTAB_VERSION_UV5: u32 = 0x500;
pub const UV_SYSTAB_VERSION_UV5_LATEST: u32 = UV_SYSTAB_VERSION_UV5;
pub const UV_SYSTAB_TYPE_UNUSED: u32 = 0;
pub const UV_SYSTAB_TYPE_GAM_PARAMS: u32 = 1;
pub const UV_SYSTAB_TYPE_GAM_RNG_TBL: u32 = 2;
pub const UV_SYSTAB_TYPE_ARCH_TYPE: u32 = 3;
pub const UV_SYSTAB_TYPE_MAX: u32 = 4;

#[repr(C)]
pub struct uv_systab_entry { pub type_: u32, pub offset: u32 }
#[repr(C)]
pub struct uv_systab {
    pub signature: [i8; 4],
    pub revision: u32,
    pub function: Option<unsafe extern "C" fn(uv_bios_cmd, ...) -> u64>,
    pub size: u32,
    pub entry: [uv_systab_entry; 0],
}
extern "C" { pub static mut uv_systab: *mut uv_systab; }

pub const UV_BIOS_MAXSTRING: usize = 128;
#[repr(C)]
pub struct uv_bios_hub_info {
    pub id: u32,
    pub flags: u64,
    pub name: [i8; UV_BIOS_MAXSTRING],
    pub location: [i8; UV_BIOS_MAXSTRING],
    pub ports: u32,
}
#[repr(C)]
pub struct uv_bios_port_info { pub port: u32, pub conn_id: u32, pub conn_port: u32 }

pub const BIOS_FREQ_BASE_PLATFORM: u32 = 0;
pub const BIOS_FREQ_BASE_INTERVAL_TIMER: u32 = 1;
pub const BIOS_FREQ_BASE_REALTIME_CLOCK: u32 = 2;

#[repr(C)]
pub union partition_info_u { pub val: u64 }

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum uv_memprotect { UV_MEMPROT_RESTRICT_ACCESS, UV_MEMPROT_ALLOW_AMO, UV_MEMPROT_ALLOW_RW }

extern "C" {
    pub fn uv_bios_get_sn_info(_: i32, _: *mut i32, _: *mut i64, _: *mut i64, _: *mut i64, _: *mut i64) -> i64;
    pub fn uv_bios_freq_base(_: u64, _: *mut u64) -> i64;
    pub fn uv_bios_mq_watchlist_alloc(_: usize, _: u32, _: *mut usize) -> i32;
    pub fn uv_bios_mq_watchlist_free(_: i32, _: i32) -> i32;
    pub fn uv_bios_change_memprotect(_: u64, _: u64, _: uv_memprotect) -> i64;
    pub fn uv_bios_reserved_page_pa(_: u64, _: *mut u64, _: *mut u64, _: *mut u64) -> i64;
    pub fn uv_bios_set_legacy_vga_target(_: bool, _: i32, _: i32) -> i32;
    pub fn uv_bios_get_master_nasid(_: u64, _: *mut u64) -> i64;
    pub fn uv_bios_get_heapsize(_: u64, _: u64, _: *mut u64) -> i64;
    pub fn uv_bios_install_heap(_: u64, _: u64, _: *mut u64) -> i64;
    pub fn uv_bios_obj_count(_: u64, _: u64, _: *mut u64) -> i64;
    pub fn uv_bios_enum_objs(_: u64, _: u64, _: *mut u64) -> i64;
    pub fn uv_bios_enum_ports(_: u64, _: u64, _: u64, _: *mut u64) -> i64;
    pub fn uv_bios_get_geoinfo(_: u64, _: u64, _: *mut u64) -> i64;
    pub fn uv_bios_get_pci_topology(_: u64, _: *mut u64) -> i64;
    pub fn uv_bios_init() -> i32;
    pub fn get_uv_systab_phys(_: bool) -> usize;
    pub static mut sn_rtc_cycles_per_second: usize;
    pub static mut uv_type: i32;
    pub static mut sn_partition_id: i64;
    pub static mut sn_coherency_id: i64;
    pub static mut sn_region_size: i64;
    pub static mut system_serial_number: i64;
    pub fn uv_get_archtype(_: *mut i8, _: i32) -> isize;
    pub fn uv_get_hubless_system() -> i32;
}

/* extern struct kobject *sgi_uv_kobj; */
/* extern struct semaphore __efi_uv_runtime_lock; */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
