/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding DRM bindings are intentionally not
 * implemented here. */

/// GEM object backed by DMA memory allocations.
#[repr(C)]
pub struct drm_gem_dma_object {
    /// base GEM object
    pub base: drm_gem_object,
    /// DMA address of the backing memory
    pub dma_addr: dma_addr_t,
    /// scatter/gather table for imported PRIME buffers
    pub sgt: *mut sg_table,
    /// Kernel virtual address of the backing memory.
    pub vaddr: *mut core::ffi::c_void,
    /// Whether the GEM object is backed by non-coherent memory.
    pub map_noncoherent: bool,
}

/* Equivalent of container_of(gem_obj, struct drm_gem_dma_object, base). */
#[inline]
pub unsafe fn to_drm_gem_dma_obj(gem_obj: *mut drm_gem_object) -> *mut drm_gem_dma_object {
    container_of!(gem_obj, drm_gem_dma_object, base)
}

extern "C" {
    pub fn drm_gem_dma_create(
        drm: *mut drm_device,
        size: usize,
    ) -> *mut drm_gem_dma_object;
    pub fn drm_gem_dma_free(dma_obj: *mut drm_gem_dma_object);
    pub fn drm_gem_dma_print_info(
        dma_obj: *const drm_gem_dma_object,
        p: *mut drm_printer,
        indent: core::ffi::c_uint,
    );
    pub fn drm_gem_dma_get_sg_table(dma_obj: *mut drm_gem_dma_object) -> *mut sg_table;
    pub fn drm_gem_dma_vmap(
        dma_obj: *mut drm_gem_dma_object,
        map: *mut iosys_map,
    ) -> core::ffi::c_int;
    pub fn drm_gem_dma_mmap(
        dma_obj: *mut drm_gem_dma_object,
        vma: *mut vm_area_struct,
    ) -> core::ffi::c_int;

    pub static drm_gem_dma_vm_ops: vm_operations_struct;

    pub fn drm_gem_dma_dumb_create_internal(
        file_priv: *mut drm_file,
        drm: *mut drm_device,
        args: *mut drm_mode_create_dumb,
    ) -> core::ffi::c_int;
    pub fn drm_gem_dma_dumb_create(
        file_priv: *mut drm_file,
        drm: *mut drm_device,
        args: *mut drm_mode_create_dumb,
    ) -> core::ffi::c_int;
    pub fn drm_gem_dma_prime_import_sg_table(
        dev: *mut drm_device,
        attach: *mut dma_buf_attachment,
        sgt: *mut sg_table,
    ) -> *mut drm_gem_object;
    pub fn drm_gem_dma_prime_import_sg_table_vmap(
        drm: *mut drm_device,
        attach: *mut dma_buf_attachment,
        sgt: *mut sg_table,
    ) -> *mut drm_gem_object;

    /* CONFIG_MMU disabled: preserve the original conditional declaration. */
    pub fn drm_gem_dma_get_unmapped_area(
        filp: *mut file,
        addr: c_ulong,
        len: c_ulong,
        pgoff: c_ulong,
        flags: c_ulong,
    ) -> c_ulong;
}

#[inline]
pub unsafe fn drm_gem_dma_object_free(obj: *mut drm_gem_object) {
    let dma_obj = to_drm_gem_dma_obj(obj);
    drm_gem_dma_free(dma_obj);
}

#[inline]
pub unsafe fn drm_gem_dma_object_print_info(
    p: *mut drm_printer,
    indent: core::ffi::c_uint,
    obj: *const drm_gem_object,
) {
    let dma_obj = to_drm_gem_dma_obj(obj as *mut drm_gem_object);
    drm_gem_dma_print_info(dma_obj, p, indent);
}

#[inline]
pub unsafe fn drm_gem_dma_object_get_sg_table(obj: *mut drm_gem_object) -> *mut sg_table {
    let dma_obj = to_drm_gem_dma_obj(obj);
    drm_gem_dma_get_sg_table(dma_obj)
}

#[inline]
pub unsafe fn drm_gem_dma_object_vmap(
    obj: *mut drm_gem_object,
    map: *mut iosys_map,
) -> core::ffi::c_int {
    let dma_obj = to_drm_gem_dma_obj(obj);
    drm_gem_dma_vmap(dma_obj, map)
}

#[inline]
pub unsafe fn drm_gem_dma_object_mmap(
    obj: *mut drm_gem_object,
    vma: *mut vm_area_struct,
) -> core::ffi::c_int {
    let dma_obj = to_drm_gem_dma_obj(obj);
    drm_gem_dma_mmap(dma_obj, vma)
}

/* Driver operation macro equivalents. */
#[macro_export]
macro_rules! DRM_GEM_DMA_DRIVER_OPS_WITH_DUMB_CREATE {
    ($dumb_create_func:expr) => {
        .dumb_create = $dumb_create_func,
        .gem_prime_import_sg_table = drm_gem_dma_prime_import_sg_table
    };
}

#[macro_export]
macro_rules! DRM_GEM_DMA_DRIVER_OPS {
    () => {
        DRM_GEM_DMA_DRIVER_OPS_WITH_DUMB_CREATE!(drm_gem_dma_dumb_create)
    };
}

#[macro_export]
macro_rules! DRM_GEM_DMA_DRIVER_OPS_VMAP_WITH_DUMB_CREATE {
    ($dumb_create_func:expr) => {
        .dumb_create = $dumb_create_func,
        .gem_prime_import_sg_table = drm_gem_dma_prime_import_sg_table_vmap
    };
}

#[macro_export]
macro_rules! DRM_GEM_DMA_DRIVER_OPS_VMAP {
    () => {
        DRM_GEM_DMA_DRIVER_OPS_VMAP_WITH_DUMB_CREATE!(drm_gem_dma_dumb_create)
    };
}

/* File operation macros. CONFIG_MMU controls whether get_unmapped_area is set. */
#[macro_export]
macro_rules! DRM_GEM_DMA_UNMAPPED_AREA_FOPS {
    () => {
        .get_unmapped_area = drm_gem_dma_get_unmapped_area
    };
}

#[macro_export]
macro_rules! DEFINE_DRM_GEM_DMA_FOPS {
    ($name:ident) => {
        static $name: file_operations = file_operations {
            .owner = THIS_MODULE,
            .open = drm_open,
            .release = drm_release,
            .unlocked_ioctl = drm_ioctl,
            .compat_ioctl = drm_compat_ioctl,
            .poll = drm_poll,
            .read = drm_read,
            .llseek = noop_llseek,
            .mmap = drm_gem_mmap,
            .fop_flags = FOP_UNSIGNED_OFFSET,
            DRM_GEM_DMA_UNMAPPED_AREA_FOPS!()
        };
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
