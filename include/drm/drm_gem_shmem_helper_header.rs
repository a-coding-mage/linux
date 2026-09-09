/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from drm_gem_shmem_helper.h. C header dependencies are external. */

use core::ffi::c_void;

#[repr(C)]
pub struct drm_gem_shmem_object {
    pub base: drm_gem_object,
    pub pages: *mut *mut page,
    pub pages_use_count: refcount_t,
    pub pages_pin_count: refcount_t,
    pub madv: ::core::ffi::c_int,
    pub madv_list: list_head,
    pub sgt: *mut sg_table,
    pub vaddr: *mut c_void,
    pub vmap_use_count: refcount_t,
    pub pages_mark_dirty_on_put: bool,
    pub pages_mark_accessed_on_put: bool,
    pub map_wc: bool,
}

#[macro_export]
macro_rules! to_drm_gem_shmem_obj {
    ($obj:expr) => {
        container_of!($obj, drm_gem_shmem_object, base)
    };
}

extern "C" {
    pub fn drm_gem_shmem_init(dev: *mut drm_device, shmem: *mut drm_gem_shmem_object, size: usize) -> ::core::ffi::c_int;
    pub fn drm_gem_shmem_create(dev: *mut drm_device, size: usize) -> *mut drm_gem_shmem_object;
    pub fn drm_gem_shmem_release(shmem: *mut drm_gem_shmem_object);
    pub fn drm_gem_shmem_free(shmem: *mut drm_gem_shmem_object);
    pub fn __drm_gem_shmem_free_sgt_locked(shmem: *mut drm_gem_shmem_object);
    pub fn drm_gem_shmem_put_pages_locked(shmem: *mut drm_gem_shmem_object);
    pub fn drm_gem_shmem_pin(shmem: *mut drm_gem_shmem_object) -> ::core::ffi::c_int;
    pub fn drm_gem_shmem_unpin(shmem: *mut drm_gem_shmem_object);
    pub fn drm_gem_shmem_vmap_locked(shmem: *mut drm_gem_shmem_object, map: *mut iosys_map) -> ::core::ffi::c_int;
    pub fn drm_gem_shmem_vunmap_locked(shmem: *mut drm_gem_shmem_object, map: *mut iosys_map);
    pub fn drm_gem_shmem_mmap(shmem: *mut drm_gem_shmem_object, vma: *mut vm_area_struct) -> ::core::ffi::c_int;
    pub fn drm_gem_shmem_pin_locked(shmem: *mut drm_gem_shmem_object) -> ::core::ffi::c_int;
    pub fn drm_gem_shmem_unpin_locked(shmem: *mut drm_gem_shmem_object);
    pub fn drm_gem_shmem_madvise_locked(shmem: *mut drm_gem_shmem_object, madv: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn drm_gem_shmem_purge_locked(shmem: *mut drm_gem_shmem_object);
    pub fn drm_gem_shmem_get_sg_table(shmem: *mut drm_gem_shmem_object) -> *mut sg_table;
    pub fn drm_gem_shmem_get_pages_sgt(shmem: *mut drm_gem_shmem_object) -> *mut sg_table;
    pub fn drm_gem_shmem_print_info(shmem: *const drm_gem_shmem_object, p: *mut drm_printer, indent: u32);
    pub fn drm_gem_shmem_create_with_handle(file_priv: *mut drm_file, dev: *mut drm_device, size: usize, handle: *mut u32) -> ::core::ffi::c_int;
    pub static drm_gem_shmem_vm_ops: vm_operations_struct;
}

#[inline]
pub unsafe fn drm_gem_shmem_is_purgeable(shmem: *mut drm_gem_shmem_object) -> bool {
    (*shmem).madv > 0 &&
        refcount_read(&(*shmem).pages_pin_count) == 0 && !(*shmem).sgt.is_null() &&
        (*shmem).base.dma_buf.is_null() && !drm_gem_is_imported(&(*shmem).base)
}

#[inline]
pub unsafe fn drm_gem_shmem_object_free(obj: *mut drm_gem_object) {
    let shmem = to_drm_gem_shmem_obj!(obj);
    drm_gem_shmem_free(shmem);
}

#[inline]
pub unsafe fn drm_gem_shmem_object_print_info(p: *mut drm_printer, indent: u32, obj: *const drm_gem_object) {
    let shmem = to_drm_gem_shmem_obj!(obj as *mut drm_gem_object);
    drm_gem_shmem_print_info(shmem, p, indent);
}

#[inline]
pub unsafe fn drm_gem_shmem_object_pin(obj: *mut drm_gem_object) -> ::core::ffi::c_int {
    drm_gem_shmem_pin_locked(to_drm_gem_shmem_obj!(obj))
}

#[inline]
pub unsafe fn drm_gem_shmem_object_unpin(obj: *mut drm_gem_object) {
    drm_gem_shmem_unpin_locked(to_drm_gem_shmem_obj!(obj));
}

#[inline]
pub unsafe fn drm_gem_shmem_object_get_sg_table(obj: *mut drm_gem_object) -> *mut sg_table {
    drm_gem_shmem_get_sg_table(to_drm_gem_shmem_obj!(obj))
}

#[inline]
pub unsafe fn drm_gem_shmem_object_vmap(obj: *mut drm_gem_object, map: *mut iosys_map) -> ::core::ffi::c_int {
    drm_gem_shmem_vmap_locked(to_drm_gem_shmem_obj!(obj), map)
}

#[inline]
pub unsafe fn drm_gem_shmem_object_vunmap(obj: *mut drm_gem_object, map: *mut iosys_map) {
    drm_gem_shmem_vunmap_locked(to_drm_gem_shmem_obj!(obj), map);
}

#[inline]
pub unsafe fn drm_gem_shmem_object_mmap(obj: *mut drm_gem_object, vma: *mut vm_area_struct) -> ::core::ffi::c_int {
    drm_gem_shmem_mmap(to_drm_gem_shmem_obj!(obj), vma)
}

extern "C" {
    pub fn drm_gem_shmem_prime_import_sg_table(dev: *mut drm_device, attach: *mut dma_buf_attachment, sgt: *mut sg_table) -> *mut drm_gem_object;
    pub fn drm_gem_shmem_dumb_create(file: *mut drm_file, dev: *mut drm_device, args: *mut drm_mode_create_dumb) -> ::core::ffi::c_int;
    pub fn drm_gem_shmem_prime_import_no_map(dev: *mut drm_device, buf: *mut dma_buf) -> *mut drm_gem_object;
}

/* DRM_GEM_SHMEM_DRIVER_OPS expands to: .gem_prime_import = drm_gem_shmem_prime_import_no_map,
 * .dumb_create = drm_gem_shmem_dumb_create. */

#[cfg(CONFIG_KUNIT)]
extern "C" {
    pub fn drm_gem_shmem_vmap(shmem: *mut drm_gem_shmem_object, map: *mut iosys_map) -> ::core::ffi::c_int;
    pub fn drm_gem_shmem_vunmap(shmem: *mut drm_gem_shmem_object, map: *mut iosys_map);
    pub fn drm_gem_shmem_madvise(shmem: *mut drm_gem_shmem_object, madv: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn drm_gem_shmem_purge(shmem: *mut drm_gem_shmem_object) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
