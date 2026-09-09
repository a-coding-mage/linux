/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/bitops.h, linux/cpuhplock.h,
// linux/cpumask_types.h, linux/smp.h

#[repr(i32)]
pub enum cache_type {
    CACHE_TYPE_NOCACHE = 0,
    CACHE_TYPE_INST = 1 << 0,
    CACHE_TYPE_DATA = 1 << 1,
    CACHE_TYPE_SEPARATE = (1 << 0) | (1 << 1),
    CACHE_TYPE_UNIFIED = 1 << 2,
}

extern "C" {
    pub static mut coherency_max_size: core::ffi::c_uint;
}

// struct cacheinfo - represent a cache leaf node
#[repr(C)]
pub struct cacheinfo {
    pub id: core::ffi::c_uint,
    pub type_: cache_type,
    pub level: core::ffi::c_uint,
    pub coherency_line_size: core::ffi::c_uint,
    pub number_of_sets: core::ffi::c_uint,
    pub ways_of_associativity: core::ffi::c_uint,
    pub physical_line_partition: core::ffi::c_uint,
    pub size: core::ffi::c_uint,
    pub shared_cpu_map: cpumask_t,
    pub attributes: core::ffi::c_uint,
    pub fw_token: *mut core::ffi::c_void,
    pub disable_sysfs: bool,
    pub priv_: *mut core::ffi::c_void,
}

pub const CACHE_WRITE_THROUGH: core::ffi::c_uint = 1 << 0;
pub const CACHE_WRITE_BACK: core::ffi::c_uint = 1 << 1;
pub const CACHE_WRITE_POLICY_MASK: core::ffi::c_uint =
    CACHE_WRITE_THROUGH | CACHE_WRITE_BACK;
pub const CACHE_READ_ALLOCATE: core::ffi::c_uint = 1 << 2;
pub const CACHE_WRITE_ALLOCATE: core::ffi::c_uint = 1 << 3;
pub const CACHE_ALLOCATE_POLICY_MASK: core::ffi::c_uint =
    CACHE_READ_ALLOCATE | CACHE_WRITE_ALLOCATE;
pub const CACHE_ID: core::ffi::c_uint = 1 << 4;

#[repr(C)]
pub struct cpu_cacheinfo {
    pub info_list: *mut cacheinfo,
    pub per_cpu_data_slice_size: core::ffi::c_uint,
    pub num_levels: core::ffi::c_uint,
    pub num_leaves: core::ffi::c_uint,
    pub cpu_map_populated: bool,
    pub early_ci_levels: bool,
}

extern "C" {
    pub fn get_cpu_cacheinfo(cpu: core::ffi::c_uint) -> *mut cpu_cacheinfo;
    pub fn early_cache_level(cpu: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn init_cache_level(cpu: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn init_of_cache_level(cpu: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn populate_cache_leaves(cpu: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn cache_setup_acpi(cpu: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn last_level_cache_is_valid(cpu: core::ffi::c_uint) -> bool;
    pub fn last_level_cache_is_shared(cpu_x: core::ffi::c_uint, cpu_y: core::ffi::c_uint) -> bool;
    pub fn get_cpu_cacheinfo_llc(cpu: core::ffi::c_uint) -> *mut cacheinfo;
    pub fn fetch_cache_info(cpu: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn detect_cache_attributes(cpu: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn acpi_get_cache_info(
        cpu: core::ffi::c_uint,
        levels: *mut core::ffi::c_uint,
        split_levels: *mut core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn cache_get_priv_group(this_leaf: *mut cacheinfo) -> *const attribute_group;
}

// Get the cacheinfo structure for the cache associated with @cpu at @level.
// cpuhp lock must be held.
#[inline]
pub unsafe fn get_cpu_cacheinfo_level(cpu: core::ffi::c_int, level: core::ffi::c_int) -> *mut cacheinfo {
    let ci = get_cpu_cacheinfo(cpu as core::ffi::c_uint);
    lockdep_assert_cpus_held();

    for i in 0..(*ci).num_leaves {
        let leaf = (*ci).info_list.add(i as usize);
        if (*leaf).level == level as core::ffi::c_uint {
            if (*leaf).attributes & CACHE_ID != 0 {
                return leaf;
            }
            return core::ptr::null_mut();
        }
    }
    core::ptr::null_mut()
}

// Get the id of the cache associated with @cpu at @level.
// cpuhp lock must be held.
#[inline]
pub unsafe fn get_cpu_cacheinfo_id(cpu: core::ffi::c_int, level: core::ffi::c_int) -> core::ffi::c_int {
    let ci = get_cpu_cacheinfo_level(cpu, level);
    if !ci.is_null() { (*ci).id as core::ffi::c_int } else { -1 }
}

// Build-time configuration: CONFIG_ARM64 or CONFIG_ARM.
#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
pub const fn use_arch_cache_info() -> bool { true }
#[cfg(not(any(target_arch = "aarch64", target_arch = "arm")))]
pub const fn use_arch_cache_info() -> bool { false }

// Build-time configuration: CONFIG_ARCH_HAS_CPU_CACHE_ALIASING.
#[cfg(not(feature = "CONFIG_ARCH_HAS_CPU_CACHE_ALIASING"))]
pub const fn cpu_dcache_is_aliasing() -> bool { false }
#[cfg(not(feature = "CONFIG_ARCH_HAS_CPU_CACHE_ALIASING"))]
pub const fn cpu_icache_is_aliasing() -> bool { cpu_dcache_is_aliasing() }

// External types and lock assertion supplied by other headers.
#[allow(non_camel_case_types)]
pub enum cpumask_t {}
pub enum attribute_group {}
extern "C" { fn lockdep_assert_cpus_held(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
