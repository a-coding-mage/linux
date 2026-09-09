/* Translated from drm_mm.h. C includes and configuration guards are external dependencies. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum drm_mm_insert_mode {
    DRM_MM_INSERT_BEST = 0,
    DRM_MM_INSERT_LOW,
    DRM_MM_INSERT_HIGH,
    DRM_MM_INSERT_EVICT,
    DRM_MM_INSERT_ONCE = 1 << 31,
    DRM_MM_INSERT_HIGHEST = (DRM_MM_INSERT_HIGH as i32) | (DRM_MM_INSERT_ONCE as i32),
    DRM_MM_INSERT_LOWEST = (DRM_MM_INSERT_LOW as i32) | (DRM_MM_INSERT_ONCE as i32),
}

#[repr(C)]
pub struct drm_mm_node {
    pub color: ::std::os::raw::c_ulong,
    pub start: u64,
    pub size: u64,
    pub mm: *mut drm_mm,
    pub node_list: list_head,
    pub hole_stack: list_head,
    pub rb: rb_node,
    pub rb_hole_size: rb_node,
    pub rb_hole_addr: rb_node,
    pub __subtree_last: u64,
    pub hole_size: u64,
    pub subtree_max_hole: u64,
    pub flags: ::std::os::raw::c_ulong,
    #[cfg(CONFIG_DRM_DEBUG_MM)]
    pub stack: depot_stack_handle_t,
}

pub const DRM_MM_NODE_ALLOCATED_BIT: u32 = 0;
pub const DRM_MM_NODE_SCANNED_BIT: u32 = 1;

#[repr(C)]
pub struct drm_mm {
    pub color_adjust: Option<unsafe extern "C" fn(*const drm_mm_node, ::std::os::raw::c_ulong, *mut u64, *mut u64)>,
    pub hole_stack: list_head,
    pub head_node: drm_mm_node,
    pub interval_tree: rb_root_cached,
    pub holes_size: rb_root_cached,
    pub holes_addr: rb_root,
    pub scan_active: ::std::os::raw::c_ulong,
}

#[repr(C)]
pub struct drm_mm_scan {
    pub mm: *mut drm_mm,
    pub size: u64,
    pub alignment: u64,
    pub remainder_mask: u64,
    pub range_start: u64,
    pub range_end: u64,
    pub hit_start: u64,
    pub hit_end: u64,
    pub color: ::std::os::raw::c_ulong,
    pub mode: drm_mm_insert_mode,
}

extern "C" {
    pub fn test_bit(nr: u32, addr: *const ::std::os::raw::c_ulong) -> bool;
    pub fn drm_mm_reserve_node(mm: *mut drm_mm, node: *mut drm_mm_node) -> ::std::os::raw::c_int;
    pub fn drm_mm_insert_node_in_range(mm: *mut drm_mm, node: *mut drm_mm_node, size: u64,
        alignment: u64, color: ::std::os::raw::c_ulong, start: u64, end: u64,
        mode: drm_mm_insert_mode) -> ::std::os::raw::c_int;
    pub fn drm_mm_remove_node(node: *mut drm_mm_node);
    pub fn drm_mm_init(mm: *mut drm_mm, start: u64, size: u64);
    pub fn drm_mm_takedown(mm: *mut drm_mm);
    pub fn __drm_mm_interval_first(mm: *const drm_mm, start: u64, last: u64) -> *mut drm_mm_node;
    pub fn drm_mm_scan_init_with_range(scan: *mut drm_mm_scan, mm: *mut drm_mm, size: u64,
        alignment: u64, color: ::std::os::raw::c_ulong, start: u64, end: u64,
        mode: drm_mm_insert_mode);
    pub fn drm_mm_scan_add_block(scan: *mut drm_mm_scan, node: *mut drm_mm_node) -> bool;
    pub fn drm_mm_scan_remove_block(scan: *mut drm_mm_scan, node: *mut drm_mm_node) -> bool;
    pub fn drm_mm_scan_color_evict(scan: *mut drm_mm_scan) -> *mut drm_mm_node;
    pub fn drm_mm_print(mm: *const drm_mm, p: *mut drm_printer);
}

#[inline]
pub unsafe fn drm_mm_node_allocated(node: *const drm_mm_node) -> bool {
    test_bit(DRM_MM_NODE_ALLOCATED_BIT, &(*node).flags)
}

#[inline]
pub unsafe fn drm_mm_initialized(mm: *const drm_mm) -> bool {
    (*mm).hole_stack.next != core::ptr::null_mut()
}

#[inline]
pub unsafe fn drm_mm_hole_follows(node: *const drm_mm_node) -> bool { (*node).hole_size != 0 }

#[inline]
pub unsafe fn __drm_mm_hole_node_start(hole_node: *const drm_mm_node) -> u64 {
    (*hole_node).start.wrapping_add((*hole_node).size)
}

#[inline]
pub unsafe fn drm_mm_hole_node_start(hole_node: *const drm_mm_node) -> u64 {
    __drm_mm_hole_node_start(hole_node)
}

#[inline]
pub unsafe fn __drm_mm_hole_node_end(hole_node: *const drm_mm_node) -> u64 {
    /* list_next_entry(hole_node, node_list)->start */
    (*(hole_node as *const u8).add(0) as *const drm_mm_node).read().start
}

#[inline]
pub unsafe fn drm_mm_hole_node_end(hole_node: *const drm_mm_node) -> u64 {
    __drm_mm_hole_node_end(hole_node)
}

#[inline]
pub unsafe fn drm_mm_insert_node_generic(mm: *mut drm_mm, node: *mut drm_mm_node,
    size: u64, alignment: u64, color: ::std::os::raw::c_ulong,
    mode: drm_mm_insert_mode) -> ::std::os::raw::c_int {
    drm_mm_insert_node_in_range(mm, node, size, alignment, color, 0, u64::MAX, mode)
}

#[inline]
pub unsafe fn drm_mm_insert_node(mm: *mut drm_mm, node: *mut drm_mm_node,
    size: u64) -> ::std::os::raw::c_int {
    drm_mm_insert_node_generic(mm, node, size, 0, 0, drm_mm_insert_mode::DRM_MM_INSERT_BEST)
}

#[inline]
pub unsafe fn drm_mm_clean(mm: *const drm_mm) -> bool {
    (*mm).head_node.node_list.next == &(*mm).head_node.node_list as *const _ as *mut _
}

#[inline]
pub unsafe fn drm_mm_scan_init(scan: *mut drm_mm_scan, mm: *mut drm_mm, size: u64,
    alignment: u64, color: ::std::os::raw::c_ulong, mode: drm_mm_insert_mode) {
    drm_mm_scan_init_with_range(scan, mm, size, alignment, color, 0, u64::MAX, mode)
}

/* External Linux kernel structures referenced by the header. */
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rb_node { _private: [u8; 0] }
#[repr(C)] pub struct rb_root_cached { _private: [u8; 0] }
#[repr(C)] pub struct rb_root { _private: [u8; 0] }
#[repr(C)] pub struct drm_printer { _private: [u8; 0] }
#[cfg(CONFIG_DRM_DEBUG_MM)] pub type depot_stack_handle_t = u32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
