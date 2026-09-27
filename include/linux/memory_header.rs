/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/memory.h. */

// Dependencies supplied by other kernel headers are intentionally external.

pub const MIN_MEMORY_BLOCK_SIZE: usize = 1usize << SECTION_SIZE_BITS;

#[repr(C)]
pub struct memory_group {
    pub nid: ::kernel::ffi::c_int,
    pub memory_blocks: list_head,
    pub present_kernel_pages: ::kernel::ffi::c_ulong,
    pub present_movable_pages: ::kernel::ffi::c_ulong,
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
    pub max_pages: ::kernel::ffi::c_ulong,
}

#[repr(C)]
pub struct memory_group__bindgen_ty_1__bindgen_ty_2 {
    pub unit_pages: ::kernel::ffi::c_ulong,
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
    pub start_section_nr: ::kernel::ffi::c_ulong,
    pub state: memory_block_state,
    pub online_type: mmop,
    pub nid: ::kernel::ffi::c_int,
    pub zone: *mut zone,
    pub dev: device,
    pub altmap: *mut vmem_altmap,
    pub group: *mut memory_group,
    pub group_next: list_head,
    // Present only when CONFIG_MEMORY_FAILURE && CONFIG_MEMORY_HOTPLUG.
    #[cfg(all(CONFIG_MEMORY_FAILURE, CONFIG_MEMORY_HOTPLUG))]
    pub nr_hwpoison: atomic_long_t,
}

extern "C" {
    pub fn arch_get_memory_phys_device(start_pfn: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn memory_block_size_bytes() -> ::kernel::ffi::c_ulong;
    pub fn set_memory_block_size_order(order: ::kernel::ffi::c_uint) -> ::kernel::ffi::c_int;
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
    pub start_pfn: ::kernel::ffi::c_ulong,
    pub nr_pages: ::kernel::ffi::c_ulong,
}

pub const DEFAULT_CALLBACK_PRI: ::kernel::ffi::c_int = 0;
pub const SLAB_CALLBACK_PRI: ::kernel::ffi::c_int = 1;
pub const CXL_CALLBACK_PRI: ::kernel::ffi::c_int = 5;
pub const HMAT_CALLBACK_PRI: ::kernel::ffi::c_int = 6;
pub const MM_COMPUTE_BATCH_PRI: ::kernel::ffi::c_int = 10;
pub const CPUSET_CALLBACK_PRI: ::kernel::ffi::c_int = 10;
pub const MEMTIER_HOTPLUG_PRI: ::kernel::ffi::c_int = 100;
pub const KSM_CALLBACK_PRI: ::kernel::ffi::c_int = 100;

#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
#[inline]
pub fn memory_dev_init() {}
#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
#[inline]
pub fn register_memory_notifier(_nb: *mut notifier_block) -> ::kernel::ffi::c_int { 0 }
#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
#[inline]
pub fn unregister_memory_notifier(_nb: *mut notifier_block) {}
#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
#[inline]
pub fn memory_notify(_state: memory_block_state, _v: *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int { 0 }
#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
#[inline]
pub fn hotplug_memory_notifier(_fn: notifier_fn_t, _pri: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int { 0 }
#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
#[inline]
pub fn memory_block_advise_max_size(_size: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int { -ENODEV }
#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
#[inline]
pub fn memory_block_advised_max_size() -> ::kernel::ffi::c_ulong { 0 }

#[cfg(CONFIG_MEMORY_HOTPLUG)]
extern "C" {
    pub fn register_memory_notifier(nb: *mut notifier_block) -> ::kernel::ffi::c_int;
    pub fn unregister_memory_notifier(nb: *mut notifier_block);
    pub fn create_memory_block_devices(start: ::kernel::ffi::c_ulong, size: ::kernel::ffi::c_ulong, nid: ::kernel::ffi::c_int, altmap: *mut vmem_altmap, group: *mut memory_group) -> ::kernel::ffi::c_int;
    pub fn remove_memory_block_devices(start: ::kernel::ffi::c_ulong, size: ::kernel::ffi::c_ulong);
    pub fn memory_dev_init();
    pub fn memory_notify(state: memory_block_state, v: *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int;
    pub fn memory_block_get(block_id: ::kernel::ffi::c_ulong) -> *mut memory_block;
    pub fn walk_memory_blocks(start: ::kernel::ffi::c_ulong, size: ::kernel::ffi::c_ulong, arg: *mut ::kernel::ffi::c_void, func: walk_memory_blocks_func_t) -> ::kernel::ffi::c_int;
    pub fn for_each_memory_block(arg: *mut ::kernel::ffi::c_void, func: walk_memory_blocks_func_t) -> ::kernel::ffi::c_int;
    pub fn memory_group_register_static(nid: ::kernel::ffi::c_int, max_pages: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn memory_group_register_dynamic(nid: ::kernel::ffi::c_int, unit_pages: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn memory_group_unregister(mgid: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn memory_group_find_by_id(mgid: ::kernel::ffi::c_int) -> *mut memory_group;
    pub fn walk_dynamic_memory_groups(nid: ::kernel::ffi::c_int, func: walk_memory_groups_func_t, excluded: *mut memory_group, arg: *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int;
    pub static mut sections_per_block: ::kernel::ffi::c_int;
    pub fn memory_block_advise_max_size(size: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn memory_block_advised_max_size() -> ::kernel::ffi::c_ulong;
}

#[cfg(CONFIG_MEMORY_HOTPLUG)]
pub type walk_memory_blocks_func_t = unsafe extern "C" fn(*mut memory_block, *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int;
#[cfg(CONFIG_MEMORY_HOTPLUG)]
pub type walk_memory_groups_func_t = unsafe extern "C" fn(*mut memory_group, *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int;

#[cfg(CONFIG_MEMORY_HOTPLUG)]
#[inline]
pub unsafe fn memory_block_id(section_nr: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong { section_nr / sections_per_block as ::kernel::ffi::c_ulong }
#[cfg(CONFIG_MEMORY_HOTPLUG)]
#[inline]
pub unsafe fn pfn_to_block_id(pfn: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong { memory_block_id(pfn_to_section_nr(pfn)) }
#[cfg(CONFIG_MEMORY_HOTPLUG)]
#[inline]
pub unsafe fn phys_to_block_id(phys: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong { pfn_to_block_id(PFN_DOWN(phys)) }

// The C hotplug_memory_notifier macro declares a static notifier block using fn##_mem_nb,
// then registers it; this token-pasting declaration is preserved as conditional intent.
#[cfg(all(CONFIG_MEMORY_HOTPLUG, CONFIG_NUMA))]
extern "C" { pub fn memory_block_add_nid_early(mem: *mut memory_block, nid: ::kernel::ffi::c_int); }

extern "C" { pub static mut text_mutex: mutex; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
