/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/memory.h. */

// Dependencies supplied by other kernel headers are intentionally external.

pub const MIN_MEMORY_BLOCK_SIZE: usize = 1usize << SECTION_SIZE_BITS;

#[repr(C)]
pub struct memory_group {
    pub nid: ::core::ffi::c_int,
    pub memory_blocks: list_head,
    pub present_kernel_pages: ::core::ffi::c_ulong,
    pub present_movable_pages: ::core::ffi::c_ulong,
    pub is_dynamic: bool,
    pub data: memory_group__bindgen_ty_1,
}

#[repr(C)]
pub union memory_group__bindgen_ty_1 {
    pub s: memory_group__bindgen_ty_1__bindgen_ty_1,
    pub d: memory_group__bindgen_ty_1__bindgen_ty_2,
}

#[repr(C)]
pub struct memory_group__bindgen_ty_1__bindgen_ty_1 {
    pub max_pages: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct memory_group__bindgen_ty_1__bindgen_ty_2 {
    pub unit_pages: ::core::ffi::c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum memory_block_state {
    MEM_ONLINE,
    MEM_GOING_OFFLINE,
    MEM_OFFLINE,
    MEM_GOING_ONLINE,
    MEM_CANCEL_ONLINE,
    MEM_CANCEL_OFFLINE,
}

#[repr(C)]
pub struct memory_block {
    pub start_section_nr: ::core::ffi::c_ulong,
    pub state: memory_block_state,
    pub online_type: mmop,
    pub nid: ::core::ffi::c_int,
    pub zone: *mut zone,
    pub dev: device,
    pub altmap: *mut vmem_altmap,
    pub group: *mut memory_group,
    pub group_next: list_head,
    // Present only when CONFIG_MEMORY_FAILURE && CONFIG_MEMORY_HOTPLUG.
    #[cfg(all(feature = "CONFIG_MEMORY_FAILURE", feature = "CONFIG_MEMORY_HOTPLUG"))]
    pub nr_hwpoison: atomic_long_t,
}

extern "C" {
    pub fn arch_get_memory_phys_device(start_pfn: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn memory_block_size_bytes() -> ::core::ffi::c_ulong;
    pub fn set_memory_block_size_order(order: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn memory_block_aligned_range(range: *const range) -> range {
    let mut aligned: range = ::core::mem::zeroed();
    aligned.start = ALIGN((*range).start, memory_block_size_bytes());
    aligned.end = ALIGN_DOWN((*range).end + 1, memory_block_size_bytes());
    if aligned.end <= aligned.start {
        aligned.start = aligned.end;
    } else {
        aligned.end -= 1;
    }
    aligned
}

#[repr(C)]
pub struct memory_notify {
    pub start_pfn: ::core::ffi::c_ulong,
    pub nr_pages: ::core::ffi::c_ulong,
}

pub const DEFAULT_CALLBACK_PRI: ::core::ffi::c_int = 0;
pub const SLAB_CALLBACK_PRI: ::core::ffi::c_int = 1;
pub const CXL_CALLBACK_PRI: ::core::ffi::c_int = 5;
pub const HMAT_CALLBACK_PRI: ::core::ffi::c_int = 6;
pub const MM_COMPUTE_BATCH_PRI: ::core::ffi::c_int = 10;
pub const CPUSET_CALLBACK_PRI: ::core::ffi::c_int = 10;
pub const MEMTIER_HOTPLUG_PRI: ::core::ffi::c_int = 100;
pub const KSM_CALLBACK_PRI: ::core::ffi::c_int = 100;

#[cfg(not(feature = "CONFIG_MEMORY_HOTPLUG"))]
#[inline]
pub fn memory_dev_init() {}
#[cfg(not(feature = "CONFIG_MEMORY_HOTPLUG"))]
#[inline]
pub fn register_memory_notifier(_nb: *mut notifier_block) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_MEMORY_HOTPLUG"))]
#[inline]
pub fn unregister_memory_notifier(_nb: *mut notifier_block) {}
#[cfg(not(feature = "CONFIG_MEMORY_HOTPLUG"))]
#[inline]
pub fn memory_notify(_state: memory_block_state, _v: *mut ::core::ffi::c_void) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_MEMORY_HOTPLUG"))]
#[inline]
pub fn hotplug_memory_notifier(_fn: notifier_fn_t, _pri: ::core::ffi::c_int) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_MEMORY_HOTPLUG"))]
#[inline]
pub fn memory_block_advise_max_size(_size: ::core::ffi::c_ulong) -> ::core::ffi::c_int { -ENODEV }
#[cfg(not(feature = "CONFIG_MEMORY_HOTPLUG"))]
#[inline]
pub fn memory_block_advised_max_size() -> ::core::ffi::c_ulong { 0 }

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
extern "C" {
    pub fn register_memory_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn unregister_memory_notifier(nb: *mut notifier_block);
    pub fn create_memory_block_devices(start: ::core::ffi::c_ulong, size: ::core::ffi::c_ulong, nid: ::core::ffi::c_int, altmap: *mut vmem_altmap, group: *mut memory_group) -> ::core::ffi::c_int;
    pub fn remove_memory_block_devices(start: ::core::ffi::c_ulong, size: ::core::ffi::c_ulong);
    pub fn memory_dev_init();
    pub fn memory_notify(state: memory_block_state, v: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn memory_block_get(block_id: ::core::ffi::c_ulong) -> *mut memory_block;
    pub fn walk_memory_blocks(start: ::core::ffi::c_ulong, size: ::core::ffi::c_ulong, arg: *mut ::core::ffi::c_void, func: walk_memory_blocks_func_t) -> ::core::ffi::c_int;
    pub fn for_each_memory_block(arg: *mut ::core::ffi::c_void, func: walk_memory_blocks_func_t) -> ::core::ffi::c_int;
    pub fn memory_group_register_static(nid: ::core::ffi::c_int, max_pages: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn memory_group_register_dynamic(nid: ::core::ffi::c_int, unit_pages: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn memory_group_unregister(mgid: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn memory_group_find_by_id(mgid: ::core::ffi::c_int) -> *mut memory_group;
    pub fn walk_dynamic_memory_groups(nid: ::core::ffi::c_int, func: walk_memory_groups_func_t, excluded: *mut memory_group, arg: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub static mut sections_per_block: ::core::ffi::c_int;
    pub fn memory_block_advise_max_size(size: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn memory_block_advised_max_size() -> ::core::ffi::c_ulong;
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
pub type walk_memory_blocks_func_t = unsafe extern "C" fn(*mut memory_block, *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
pub type walk_memory_groups_func_t = unsafe extern "C" fn(*mut memory_group, *mut ::core::ffi::c_void) -> ::core::ffi::c_int;

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
#[inline]
pub unsafe fn memory_block_id(section_nr: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong { section_nr / sections_per_block as ::core::ffi::c_ulong }
#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
#[inline]
pub unsafe fn pfn_to_block_id(pfn: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong { memory_block_id(pfn_to_section_nr(pfn)) }
#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
#[inline]
pub unsafe fn phys_to_block_id(phys: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong { pfn_to_block_id(PFN_DOWN(phys)) }

// The C hotplug_memory_notifier macro declares a static notifier block using fn##_mem_nb,
// then registers it; this token-pasting declaration is preserved as conditional intent.
#[cfg(all(feature = "CONFIG_MEMORY_HOTPLUG", feature = "CONFIG_NUMA"))]
extern "C" { pub fn memory_block_add_nid_early(mem: *mut memory_block, nid: ::core::ffi::c_int); }

extern "C" { pub static mut text_mutex: mutex; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
