/* Translated from ttm_resource.h. */

// Linux/kernel and DRM dependencies are supplied by the surrounding crate.
use core::ffi::{c_char, c_int, c_void};

pub const TTM_MAX_BO_PRIORITY: u32 = 4;
pub const TTM_NUM_MEM_TYPES: usize = 9;
pub const TTM_NUM_MOVE_FENCES: usize = 8;

pub enum dentry {}
pub enum dmem_cgroup_device {}
pub enum dmem_cgroup_region {}
pub enum drm_printer {}
pub enum ttm_device {}
pub enum ttm_place {}
pub enum ttm_buffer_object {}
pub enum ttm_placement {}
pub enum io_mapping {}
pub enum sg_table {}
pub enum scatterlist {}
pub enum dma_fence {}
pub enum ttm_kmap_iter {}
pub enum ttm_caching {}
pub enum dmem_cgroup_pool_state {}

#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)]
pub struct iosys_map { _private: [u8; 0] }

pub type phys_addr_t = usize;
pub type resource_size_t = usize;
pub type pgoff_t = usize;
pub type u64 = u64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ttm_lru_item_type { TTM_LRU_RESOURCE, TTM_LRU_HITCH }

#[repr(C)]
pub struct ttm_lru_item { pub link: list_head, pub type_: ttm_lru_item_type }

#[inline]
pub unsafe fn ttm_lru_item_init(item: *mut ttm_lru_item, type_: ttm_lru_item_type) {
    (*item).type_ = type_;
    // Equivalent to INIT_LIST_HEAD(&item->link).
    (*item).link.next = &mut (*item).link;
    (*item).link.prev = &mut (*item).link;
}

#[inline]
pub unsafe fn ttm_lru_item_is_res(item: *const ttm_lru_item) -> bool {
    (*item).type_ == ttm_lru_item_type::TTM_LRU_RESOURCE
}

#[repr(C)]
pub struct ttm_resource_manager_func {
    pub alloc: Option<unsafe extern "C" fn(*mut ttm_resource_manager, *mut ttm_buffer_object, *const ttm_place, *mut *mut ttm_resource) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut ttm_resource_manager, *mut ttm_resource)>,
    pub intersects: Option<unsafe extern "C" fn(*mut ttm_resource_manager, *mut ttm_resource, *const ttm_place, usize) -> bool>,
    pub compatible: Option<unsafe extern "C" fn(*mut ttm_resource_manager, *mut ttm_resource, *const ttm_place, usize) -> bool>,
    pub debug: Option<unsafe extern "C" fn(*mut ttm_resource_manager, *mut drm_printer)>,
}

#[repr(C)]
pub struct ttm_resource_manager {
    pub use_type: bool,
    pub use_tt: bool,
    pub bdev: *mut ttm_device,
    pub size: u64,
    pub func: *const ttm_resource_manager_func,
    pub eviction_lock: spinlock_t,
    pub eviction_fences: [*mut dma_fence; TTM_NUM_MOVE_FENCES],
    pub lru: [list_head; TTM_MAX_BO_PRIORITY as usize],
    pub usage: u64,
    pub cg: *mut dmem_cgroup_region,
}

#[repr(C)]
pub struct ttm_bus_placement { pub addr: *mut c_void, pub offset: phys_addr_t, pub is_iomem: bool, pub caching: ttm_caching }

#[repr(C)]
pub struct ttm_resource {
    pub start: usize,
    pub size: usize,
    pub mem_type: u32,
    pub placement: u32,
    pub bus: ttm_bus_placement,
    pub bo: *mut ttm_buffer_object,
    pub css: *mut dmem_cgroup_pool_state,
    pub lru: ttm_lru_item,
}

#[inline]
pub unsafe fn ttm_lru_item_to_res(item: *mut ttm_lru_item) -> *mut ttm_resource {
    // container_of(item, struct ttm_resource, lru)
    (item as *mut u8).sub(core::mem::size_of::<ttm_resource>() - core::mem::size_of::<ttm_lru_item>()) as *mut ttm_resource
}

#[repr(C)]
pub struct ttm_lru_bulk_move_pos { pub first: *mut ttm_resource, pub last: *mut ttm_resource }
#[repr(C)]
pub struct ttm_lru_bulk_move {
    pub pos: [[ttm_lru_bulk_move_pos; TTM_MAX_BO_PRIORITY as usize]; TTM_NUM_MEM_TYPES],
    pub cursor_list: list_head,
}
#[repr(C)]
pub struct ttm_resource_cursor {
    pub man: *mut ttm_resource_manager,
    pub hitch: ttm_lru_item,
    pub bulk_link: list_head,
    pub bulk: *mut ttm_lru_bulk_move,
    pub mem_type: u32,
    pub priority: u32,
}

#[repr(C)]
pub struct ttm_kmap_iter_iomap {
    pub base: ttm_kmap_iter,
    pub iomap: *mut io_mapping,
    pub st: *mut sg_table,
    pub start: resource_size_t,
    pub cache: ttm_kmap_iter_iomap_cache,
}
#[repr(C)]
pub struct ttm_kmap_iter_iomap_cache { pub sg: *mut scatterlist, pub i: pgoff_t, pub end: pgoff_t, pub offs: pgoff_t }
#[repr(C)]
pub struct ttm_kmap_iter_linear_io { pub base: ttm_kmap_iter, pub dmap: iosys_map, pub needs_unmap: bool }

extern "C" {
    pub fn ttm_resource_cursor_init(cursor: *mut ttm_resource_cursor, man: *mut ttm_resource_manager);
    pub fn ttm_resource_cursor_fini(cursor: *mut ttm_resource_cursor);
    pub fn ttm_lru_bulk_move_init(bulk: *mut ttm_lru_bulk_move);
    pub fn ttm_lru_bulk_move_tail(bulk: *mut ttm_lru_bulk_move);
    pub fn ttm_lru_bulk_move_fini(bdev: *mut ttm_device, bulk: *mut ttm_lru_bulk_move);
    pub fn ttm_resource_add_bulk_move(res: *mut ttm_resource, bo: *mut ttm_buffer_object);
    pub fn ttm_resource_del_bulk_move(res: *mut ttm_resource, bo: *mut ttm_buffer_object);
    pub fn ttm_resource_del_bulk_move_unevictable(res: *mut ttm_resource, bo: *mut ttm_buffer_object);
    pub fn ttm_resource_move_to_lru_tail(res: *mut ttm_resource);
    pub fn ttm_resource_init(bo: *mut ttm_buffer_object, place: *const ttm_place, res: *mut ttm_resource);
    pub fn ttm_resource_fini(man: *mut ttm_resource_manager, res: *mut ttm_resource);
    pub fn ttm_resource_try_charge(bo: *mut ttm_buffer_object, place: *const ttm_place, ret_pool: *mut *mut dmem_cgroup_pool_state, ret_limit_pool: *mut *mut dmem_cgroup_pool_state) -> c_int;
    pub fn ttm_resource_alloc(bo: *mut ttm_buffer_object, place: *const ttm_place, res: *mut *mut ttm_resource, charge_pool: *mut dmem_cgroup_pool_state) -> c_int;
    pub fn ttm_resource_free(bo: *mut ttm_buffer_object, res: *mut *mut ttm_resource);
    pub fn ttm_resource_intersects(bdev: *mut ttm_device, res: *mut ttm_resource, place: *const ttm_place, size: usize) -> bool;
    pub fn ttm_resource_compatible(res: *mut ttm_resource, placement: *mut ttm_placement, evicting: bool) -> bool;
    pub fn ttm_resource_set_bo(res: *mut ttm_resource, bo: *mut ttm_buffer_object);
    pub fn ttm_resource_manager_init(man: *mut ttm_resource_manager, bdev: *mut ttm_device, size: u64);
    pub fn ttm_resource_manager_set_dmem_region(man: *mut ttm_resource_manager, region: *mut dmem_cgroup_region);
    pub fn ttm_resource_manager_dmem_reclaim(pool: *mut dmem_cgroup_pool_state, target_bytes: u64, priv_: *mut c_void) -> c_int;
    pub fn ttm_resource_manager_evict_all(bdev: *mut ttm_device, man: *mut ttm_resource_manager) -> c_int;
    pub fn ttm_resource_manager_usage(man: *mut ttm_resource_manager) -> u64;
    pub fn ttm_resource_manager_debug(man: *mut ttm_resource_manager, p: *mut drm_printer);
    pub fn ttm_resource_manager_first(cursor: *mut ttm_resource_cursor) -> *mut ttm_resource;
    pub fn ttm_resource_manager_next(cursor: *mut ttm_resource_cursor) -> *mut ttm_resource;
    pub fn ttm_lru_first_res_or_null(head: *mut list_head) -> *mut ttm_resource;
    pub fn ttm_kmap_iter_iomap_init(iter_io: *mut ttm_kmap_iter_iomap, iomap: *mut io_mapping, st: *mut sg_table, start: resource_size_t) -> *mut ttm_kmap_iter;
    pub fn ttm_kmap_iter_linear_io_init(iter_io: *mut ttm_kmap_iter_linear_io, bdev: *mut ttm_device, mem: *mut ttm_resource) -> *mut ttm_kmap_iter;
    pub fn ttm_kmap_iter_linear_io_fini(iter_io: *mut ttm_kmap_iter_linear_io, bdev: *mut ttm_device, mem: *mut ttm_resource);
    pub fn ttm_resource_manager_create_debugfs(man: *mut ttm_resource_manager, parent: *mut dentry, name: *const c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
