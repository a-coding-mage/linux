/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header rejects inclusion of the kernel proper EFI namespace headers.
 * Those preprocessor conditions have no direct file-local Rust equivalent.
 */

pub type efi_guid_t = guid_t;

#[macro_export]
macro_rules! EFI_GUID {
    ($a:expr, $b:expr, $c:expr, $($d:expr),+ $(,)?) => {
        efi_guid_t {
            b: [
                (($a) & 0xff) as u8,
                ((($a) >> 8) & 0xff) as u8,
                ((($a) >> 16) & 0xff) as u8,
                ((($a) >> 24) & 0xff) as u8,
                (($b) & 0xff) as u8,
                ((($b) >> 8) & 0xff) as u8,
                (($c) & 0xff) as u8,
                ((($c) >> 8) & 0xff) as u8,
                $($d as u8),+
            ],
        }
    };
}

pub const ACPI_TABLE_GUID: efi_guid_t = EFI_GUID!(0xeb9d2d30, 0x2d88, 0x11d3, 0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d);
pub const ACPI_20_TABLE_GUID: efi_guid_t = EFI_GUID!(0x8868e871, 0xe4f1, 0x11d3, 0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81);
pub const EFI_CC_BLOB_GUID: efi_guid_t = EFI_GUID!(0x067b1f5f, 0xcf26, 0x44c5, 0x85, 0x54, 0x93, 0xd7, 0x77, 0x91, 0x2d, 0x42);
pub const LINUX_EFI_UNACCEPTED_MEM_TABLE_GUID: efi_guid_t = EFI_GUID!(0xd5d1de3c, 0x105c, 0x44f9, 0x9e, 0xa9, 0xbc, 0xef, 0x98, 0x12, 0x00, 0x31);

pub const EFI32_LOADER_SIGNATURE: &[u8; 4] = b"EL32";
pub const EFI64_LOADER_SIGNATURE: &[u8; 4] = b"EL64";

#[repr(C)]
pub struct efi_table_hdr_t {
    pub signature: u64,
    pub revision: u32,
    pub headersize: u32,
    pub crc32: u32,
    pub reserved: u32,
}

pub const EFI_CONVENTIONAL_MEMORY: u32 = 7;
pub const EFI_UNACCEPTED_MEMORY: u32 = 15;
pub const EFI_MEMORY_MORE_RELIABLE: u64 = 0x0000_0000_0001_0000;
pub const EFI_MEMORY_SP: u64 = 0x0000_0000_0004_0000;
pub const EFI_PAGE_SHIFT: u32 = 12;

#[repr(C)]
pub struct efi_memory_desc_t {
    pub r#type: u32,
    pub pad: u32,
    pub phys_addr: u64,
    pub virt_addr: u64,
    pub num_pages: u64,
    pub attribute: u64,
}

#[inline]
pub unsafe fn efi_early_memdesc_ptr(map: *mut core::ffi::c_void, desc_size: usize, n: usize) -> *mut efi_memory_desc_t {
    (map as *mut u8).add(n.wrapping_mul(desc_size)) as *mut efi_memory_desc_t
}

#[repr(C)]
pub struct efi_config_table_64_t {
    pub guid: efi_guid_t,
    pub table: u64,
}

#[repr(C)]
pub struct efi_config_table_32_t {
    pub guid: efi_guid_t,
    pub table: u32,
}

#[repr(C)]
pub struct efi_system_table_64_t {
    pub hdr: efi_table_hdr_t,
    pub fw_vendor: u64, /* physical addr of CHAR16 vendor string */
    pub fw_revision: u32,
    pub __pad1: u32,
    pub con_in_handle: u64,
    pub con_in: u64,
    pub con_out_handle: u64,
    pub con_out: u64,
    pub stderr_handle: u64,
    pub stderr: u64,
    pub runtime: u64,
    pub boottime: u64,
    pub nr_tables: u32,
    pub __pad2: u32,
    pub tables: u64,
}

#[repr(C)]
pub struct efi_system_table_32_t {
    pub hdr: efi_table_hdr_t,
    pub fw_vendor: u32, /* physical addr of CHAR16 vendor string */
    pub fw_revision: u32,
    pub con_in_handle: u32,
    pub con_in: u32,
    pub con_out_handle: u32,
    pub con_out: u32,
    pub stderr_handle: u32,
    pub stderr: u32,
    pub runtime: u32,
    pub boottime: u32,
    pub nr_tables: u32,
    pub tables: u32,
}

#[repr(C)]
pub struct efi_unaccepted_memory {
    pub version: u32,
    pub unit_size: u32,
    pub phys_base: u64,
    pub size: u64,
    pub bitmap: [core::ffi::c_ulong; 0],
}

unsafe extern "C" {
    fn memcmp(left: *const core::ffi::c_void, right: *const core::ffi::c_void, n: usize) -> i32;
}

#[inline]
pub unsafe fn efi_guidcmp(left: efi_guid_t, right: efi_guid_t) -> i32 {
    memcmp(
        &left as *const _ as *const core::ffi::c_void,
        &right as *const _ as *const core::ffi::c_void,
        core::mem::size_of::<efi_guid_t>(),
    )
}

/* CONFIG_EFI and CONFIG_EFI_SOFT_RESERVE are build-time conditions. */
#[cfg(feature = "CONFIG_EFI")]
unsafe extern "C" {
    fn __efi_soft_reserve_enabled() -> bool;
}

#[cfg(feature = "CONFIG_EFI")]
#[inline]
pub unsafe fn efi_soft_reserve_enabled() -> bool {
    cfg!(feature = "CONFIG_EFI_SOFT_RESERVE") && __efi_soft_reserve_enabled()
}

#[cfg(not(feature = "CONFIG_EFI"))]
#[inline]
pub fn efi_soft_reserve_enabled() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
