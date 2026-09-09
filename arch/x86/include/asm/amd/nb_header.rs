/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/ioport.h, linux/pci.h, and asm/amd/node.h.

#[repr(C)]
pub struct AmdNbBusDevRange {
    pub bus: u8,
    pub dev_base: u8,
    pub dev_limit: u8,
}

unsafe extern "C" {
    pub static amd_nb_bus_dev_ranges: AmdNbBusDevRange;

    pub fn early_is_amd_nb(value: u32) -> bool;
    pub fn amd_get_mmconfig_range(res: *mut Resource) -> *mut Resource;
    pub fn amd_flush_garts();
    pub fn amd_numa_init() -> i32;
    pub fn amd_get_subcaches(_: i32) -> i32;
    pub fn amd_set_subcaches(_: i32, _: c_ulong) -> i32;
}

#[repr(C)]
pub struct AmdL3Cache {
    pub indices: c_uint,
    pub subcaches: [u8; 4],
}

#[repr(C)]
pub struct AmdNorthbridge {
    pub misc: *mut PciDev,
    pub link: *mut PciDev,
    pub l3_cache: AmdL3Cache,
}

#[repr(C)]
pub struct AmdNorthbridgeInfo {
    pub num: u16,
    pub flags: u64,
    pub nb: *mut AmdNorthbridge,
}

pub const AMD_NB_GART: c_uint = BIT(0);
pub const AMD_NB_L3_INDEX_DISABLE: c_uint = BIT(1);
pub const AMD_NB_L3_PARTITIONING: c_uint = BIT(2);

#[cfg(CONFIG_AMD_NB)]
unsafe extern "C" {
    pub fn amd_nb_num() -> u16;
    pub fn amd_nb_has_feature(feature: c_uint) -> bool;
    pub fn node_to_amd_nb(node: i32) -> *mut AmdNorthbridge;
}

#[cfg(CONFIG_AMD_NB)]
#[inline]
pub unsafe fn amd_gart_present() -> bool {
    if boot_cpu_data.x86_vendor != X86_VENDOR_AMD {
        return false;
    }

    /* GART present only on Fam15h, up to model 0fh */
    if boot_cpu_data.x86 == 0xf
        || boot_cpu_data.x86 == 0x10
        || (boot_cpu_data.x86 == 0x15 && boot_cpu_data.x86_model < 0x10)
    {
        return true;
    }

    false
}

#[cfg(not(CONFIG_AMD_NB))]
#[inline]
pub const fn amd_nb_num(_: c_int) -> c_int {
    0
}

#[cfg(not(CONFIG_AMD_NB))]
#[inline]
pub const fn amd_nb_has_feature(_: c_int) -> bool {
    false
}

#[cfg(not(CONFIG_AMD_NB))]
#[inline]
pub const fn node_to_amd_nb(_: i32) -> *mut AmdNorthbridge {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_AMD_NB))]
#[inline]
pub const fn amd_gart_present(_: c_int) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
