// GEM Graphics Execution Manager Driver Interfaces
// Translated from drm_gem.h; external kernel types and functions are supplied elsewhere.

use core::ffi::c_void;

#[repr(C)]
pub struct iosys_map;
#[repr(C)]
pub struct drm_gem_object;
#[repr(C)]
pub struct drm_file;
#[repr(C)]
pub struct drm_printer;
#[repr(C)]
pub struct dma_buf;
#[repr(C)]
pub struct sg_table;
#[repr(C)]
pub struct vm_area_struct;
#[repr(C)]
pub struct vm_operations_struct;
#[repr(C)]
pub struct drm_device { pub huge_mnt: *mut vfsmount }
#[repr(C)]
pub struct file;
#[repr(C)]
pub struct dma_buf_attachment;
#[repr(C)]
pub struct page;
#[repr(C)]
pub struct ww_acquire_ctx;
#[repr(C)]
pub struct vfsmount;
#[repr(C)]
pub struct kref;
#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct drm_vma_offset_node;
#[repr(C)]
pub struct dma_resv;

#[repr(u32)]
pub enum drm_gem_object_status {
    DRM_GEM_OBJECT_RESIDENT = 1 << 0,
    DRM_GEM_OBJECT_PURGEABLE = 1 << 1,
    DRM_GEM_OBJECT_ACTIVE = 1 << 2,
}

#[repr(C)]
pub struct drm_gem_object_funcs {
    pub free: unsafe extern "C" fn(*mut drm_gem_object),
    pub open: Option<unsafe extern "C" fn(*mut drm_gem_object, *mut drm_file) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut drm_gem_object, *mut drm_file)>,
    pub print_info: Option<unsafe extern "C" fn(*mut drm_printer, u32, *const drm_gem_object)>,
    pub export: Option<unsafe extern "C" fn(*mut drm_gem_object, i32) -> *mut dma_buf>,
    pub pin: Option<unsafe extern "C" fn(*mut drm_gem_object) -> i32>,
    pub unpin: Option<unsafe extern "C" fn(*mut drm_gem_object)>,
    pub get_sg_table: Option<unsafe extern "C" fn(*mut drm_gem_object) -> *mut sg_table>,
    pub vmap: Option<unsafe extern "C" fn(*mut drm_gem_object, *mut iosys_map) -> i32>,
    pub vunmap: Option<unsafe extern "C" fn(*mut drm_gem_object, *mut iosys_map)>,
    pub mmap: Option<unsafe extern "C" fn(*mut drm_gem_object, *mut vm_area_struct) -> i32>,
    pub evict: Option<unsafe extern "C" fn(*mut drm_gem_object) -> i32>,
    pub status: Option<unsafe extern "C" fn(*mut drm_gem_object) -> drm_gem_object_status>,
    pub rss: Option<unsafe extern "C" fn(*mut drm_gem_object) -> usize>,
    pub vm_ops: *const vm_operations_struct,
}

#[repr(C)]
pub struct drm_gem_lru { pub count: i64, pub list: list_head }

#[repr(C)]
pub struct drm_gem_object {
    pub refcount: kref,
    pub handle_count: u32,
    pub dev: *mut drm_device,
    pub filp: *mut file,
    pub vma_node: drm_vma_offset_node,
    pub size: usize,
    pub name: i32,
    pub dma_buf: *mut dma_buf,
    pub import_attach: *mut dma_buf_attachment,
    pub resv: *mut dma_resv,
    pub _resv: dma_resv,
    pub gpuva: drm_gem_object_gpuva,
    pub funcs: *const drm_gem_object_funcs,
    pub lru_node: list_head,
    pub lru: *mut drm_gem_lru,
}

#[repr(C)]
pub struct drm_gem_object_gpuva { pub list: list_head, pub lock: mutex }

extern "C" {
    pub fn drm_gem_huge_mnt_create(dev: *mut drm_device, value: *const i8) -> i32;
    pub fn drm_gem_object_release(obj: *mut drm_gem_object);
    pub fn drm_gem_object_free(kref: *mut kref);
    pub fn drm_gem_object_init(dev: *mut drm_device, obj: *mut drm_gem_object, size: usize) -> i32;
    pub fn drm_gem_private_object_init(dev: *mut drm_device, obj: *mut drm_gem_object, size: usize);
    pub fn drm_gem_private_object_fini(obj: *mut drm_gem_object);
    pub fn drm_gem_vm_open(vma: *mut vm_area_struct);
    pub fn drm_gem_vm_close(vma: *mut vm_area_struct);
    pub fn drm_gem_mmap_obj(obj: *mut drm_gem_object, obj_size: u64, vma: *mut vm_area_struct) -> i32;
    pub fn drm_gem_mmap(filp: *mut file, vma: *mut vm_area_struct) -> i32;
    pub fn drm_gem_get_unmapped_area(filp: *mut file, uaddr: u64, len: u64, pgoff: u64, flags: u64) -> u64;
    pub fn kref_get(kref: *mut kref);
    pub fn drm_gem_handle_create(file_priv: *mut drm_file, obj: *mut drm_gem_object, handlep: *mut u32) -> i32;
    pub fn drm_gem_handle_delete(filp: *mut drm_file, handle: u32) -> i32;
    pub fn drm_gem_free_mmap_offset(obj: *mut drm_gem_object);
    pub fn drm_gem_create_mmap_offset(obj: *mut drm_gem_object) -> i32;
    pub fn drm_gem_create_mmap_offset_size(obj: *mut drm_gem_object, size: usize) -> i32;
    pub fn drm_gem_get_pages(obj: *mut drm_gem_object) -> *mut *mut page;
    pub fn drm_gem_put_pages(obj: *mut drm_gem_object, pages: *mut *mut page, dirty: bool, accessed: bool);
    pub fn drm_gem_lock(obj: *mut drm_gem_object);
    pub fn drm_gem_unlock(obj: *mut drm_gem_object);
    pub fn drm_gem_vmap(obj: *mut drm_gem_object, map: *mut iosys_map) -> i32;
    pub fn drm_gem_vunmap(obj: *mut drm_gem_object, map: *mut iosys_map);
    pub fn drm_gem_objects_lookup(filp: *mut drm_file, bo_handles: *mut c_void, count: i32, objs_out: *mut *mut *mut drm_gem_object) -> i32;
    pub fn drm_gem_object_lookup(filp: *mut drm_file, handle: u32) -> *mut drm_gem_object;
    pub fn drm_gem_dma_resv_wait(filep: *mut drm_file, handle: u32, wait_all: bool, timeout: u64) -> i64;
    pub fn drm_gem_lock_reservations(objs: *mut *mut drm_gem_object, count: i32, acquire_ctx: *mut ww_acquire_ctx) -> i32;
    pub fn drm_gem_unlock_reservations(objs: *mut *mut drm_gem_object, count: i32, acquire_ctx: *mut ww_acquire_ctx);
    pub fn drm_gem_dumb_map_offset(file: *mut drm_file, dev: *mut drm_device, handle: u32, offset: *mut u64) -> i32;
    pub fn drm_gem_lru_init(lru: *mut drm_gem_lru);
    pub fn drm_gem_lru_remove(obj: *mut drm_gem_object);
    pub fn drm_gem_lru_move_tail_locked(lru: *mut drm_gem_lru, obj: *mut drm_gem_object);
    pub fn drm_gem_lru_move_tail(lru: *mut drm_gem_lru, obj: *mut drm_gem_object);
    pub fn drm_gem_lru_scan(dev: *mut drm_device, lru: *mut drm_gem_lru, nr_to_scan: u32, remaining: *mut u64, shrink: Option<unsafe extern "C" fn(*mut drm_gem_object, *mut ww_acquire_ctx) -> bool>, ticket: *mut ww_acquire_ctx) -> u64;
    pub fn drm_gem_evict_locked(obj: *mut drm_gem_object) -> i32;
}

#[inline]
pub unsafe fn drm_gem_get_huge_mnt(dev: *mut drm_device) -> *mut vfsmount { (*dev).huge_mnt }

#[inline]
pub unsafe fn drm_gem_object_get(obj: *mut drm_gem_object) { kref_get(&mut (*obj).refcount); }

#[inline]
pub unsafe fn __drm_gem_object_put(obj: *mut drm_gem_object) { drm_gem_object_free(&mut (*obj).refcount); }

#[inline]
pub unsafe fn drm_gem_object_put(obj: *mut drm_gem_object) { if !obj.is_null() { __drm_gem_object_put(obj); } }

#[inline]
pub unsafe fn drm_gem_object_is_shared_for_memory_stats(obj: *mut drm_gem_object) -> bool { (*obj).handle_count > 1 || !(*obj).dma_buf.is_null() }

#[inline]
pub unsafe fn drm_gem_is_imported(obj: *const drm_gem_object) -> bool { !(*obj).import_attach.is_null() }

#[inline]
pub unsafe fn drm_gem_gpuva_init(obj: *mut drm_gem_object) { /* INIT_LIST_HEAD(&obj->gpuva.list); */ }

// CONFIG_LOCKDEP assertion and list iterator macros are intentionally retained as comments;
// their implementations depend on kernel list/lock helpers supplied by other headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
