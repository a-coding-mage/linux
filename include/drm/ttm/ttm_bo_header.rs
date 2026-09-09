/* Translation of drm/ttm/ttm_bo.h. C includes and build-time dependencies are
 * intentionally left to the surrounding Rust translation unit. */

pub const TTM_BO_VM_NUM_PREFAULT: u32 = 16;
pub const TTM_BO_MAP_IOMEM_MASK: u32 = 0x80;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ttm_bo_type {
    ttm_bo_type_device,
    ttm_bo_type_kernel,
    ttm_bo_type_sg,
}

#[repr(C)]
pub struct ttm_buffer_object {
    pub base: drm_gem_object,
    pub bdev: *mut ttm_device,
    pub type_: ttm_bo_type,
    pub page_alignment: u32,
    pub destroy: Option<unsafe extern "C" fn(*mut ttm_buffer_object)>,
    pub kref: kref,
    pub resource: *mut ttm_resource,
    pub ttm: *mut ttm_tt,
    pub deleted: bool,
    pub bulk_move: *mut ttm_lru_bulk_move,
    pub priority: c_uint,
    pub pin_count: c_uint,
    pub delayed_delete: work_struct,
    pub sg: *mut sg_table,
}

#[repr(C)]
pub enum ttm_bo_map_type {
    ttm_bo_map_iomap = 1 | TTM_BO_MAP_IOMEM_MASK,
    ttm_bo_map_vmap = 2,
    ttm_bo_map_kmap = 3,
    ttm_bo_map_premapped = 4 | TTM_BO_MAP_IOMEM_MASK,
}

#[repr(C)]
pub struct ttm_bo_kmap_obj {
    pub virtual_: *mut c_void,
    pub page: *mut page,
    pub bo_kmap_type: ttm_bo_map_type,
    pub bo: *mut ttm_buffer_object,
}

#[repr(C)]
pub struct ttm_operation_ctx {
    pub interruptible: bool,
    pub no_wait_gpu: bool,
    pub gfp_retry_mayfail: bool,
    pub allow_res_evict: bool,
    pub resv: *mut dma_resv,
    pub bytes_moved: u64,
}

#[repr(C)]
pub struct ttm_lru_walk_ops {
    pub process_bo: Option<unsafe extern "C" fn(*mut ttm_lru_walk, *mut ttm_buffer_object) -> s64>,
}

#[repr(C)]
pub struct ttm_lru_walk_arg {
    pub ctx: *mut ttm_operation_ctx,
    pub ticket: *mut ww_acquire_ctx,
    pub trylock_only: bool,
    pub sleeping_lock: bool,
}

#[repr(C)]
pub struct ttm_lru_walk {
    pub ops: *const ttm_lru_walk_ops,
    pub arg: ttm_lru_walk_arg,
}

extern "C" {
    pub fn ttm_lru_walk_for_evict(walk: *mut ttm_lru_walk, bdev: *mut ttm_device,
        man: *mut ttm_resource_manager, target: s64) -> s64;
}

#[repr(C)]
pub struct ttm_bo_shrink_flags {
    pub purge: u32,
    pub writeback: u32,
    pub allow_move: u32,
}

extern "C" {
    pub fn ttm_bo_shrink(ctx: *mut ttm_operation_ctx, bo: *mut ttm_buffer_object,
        flags: ttm_bo_shrink_flags) -> c_long;
    pub fn ttm_bo_shrink_suitable(bo: *mut ttm_buffer_object, ctx: *mut ttm_operation_ctx) -> bool;
    pub fn ttm_bo_shrink_avoid_wait() -> bool;
}

#[inline]
pub unsafe fn ttm_bo_reserve(bo: *mut ttm_buffer_object, interruptible: bool,
                             no_wait: bool, ticket: *mut ww_acquire_ctx) -> c_int {
    let mut ret: c_int = 0;
    if no_wait {
        if !ticket.is_null() { return -EBUSY; }
        return if dma_resv_trylock((*bo).base.resv) { 0 } else { -EBUSY };
    }
    ret = if interruptible { dma_resv_lock_interruptible((*bo).base.resv, ticket) }
          else { dma_resv_lock((*bo).base.resv, ticket) };
    if ret == -EINTR { return -ERESTARTSYS; }
    ret
}

#[inline]
pub unsafe fn ttm_bo_reserve_slowpath(bo: *mut ttm_buffer_object,
                                      interruptible: bool, ticket: *mut ww_acquire_ctx) -> c_int {
    if interruptible {
        let mut ret = dma_resv_lock_slow_interruptible((*bo).base.resv, ticket);
        if ret == -EINTR { ret = -ERESTARTSYS; }
        return ret;
    }
    dma_resv_lock_slow((*bo).base.resv, ticket);
    0
}

extern "C" {
    pub fn ttm_bo_move_to_lru_tail(bo: *mut ttm_buffer_object);
    pub fn ttm_bo_wait_ctx(bo: *mut ttm_buffer_object, ctx: *mut ttm_operation_ctx) -> c_int;
    pub fn ttm_bo_validate(bo: *mut ttm_buffer_object, placement: *mut ttm_placement, ctx: *mut ttm_operation_ctx) -> c_int;
    pub fn ttm_bo_fini(bo: *mut ttm_buffer_object);
    pub fn ttm_bo_set_bulk_move(bo: *mut ttm_buffer_object, bulk: *mut ttm_lru_bulk_move);
    pub fn ttm_bo_eviction_valuable(bo: *mut ttm_buffer_object, place: *const ttm_place) -> bool;
    pub fn ttm_bo_init_reserved(bdev: *mut ttm_device, bo: *mut ttm_buffer_object, type_: ttm_bo_type, placement: *mut ttm_placement, alignment: u32, ctx: *mut ttm_operation_ctx, sg: *mut sg_table, resv: *mut dma_resv, destroy: Option<unsafe extern "C" fn(*mut ttm_buffer_object)>) -> c_int;
    pub fn ttm_bo_init_validate(bdev: *mut ttm_device, bo: *mut ttm_buffer_object, type_: ttm_bo_type, placement: *mut ttm_placement, alignment: u32, interruptible: bool, sg: *mut sg_table, resv: *mut dma_resv, destroy: Option<unsafe extern "C" fn(*mut ttm_buffer_object)>) -> c_int;
    pub fn ttm_bo_kmap(bo: *mut ttm_buffer_object, start_page: c_ulong, num_pages: c_ulong, map: *mut ttm_bo_kmap_obj) -> c_int;
    pub fn ttm_bo_kunmap(map: *mut ttm_bo_kmap_obj);
    pub fn ttm_bo_kmap_try_from_panic(bo: *mut ttm_buffer_object, page: c_ulong) -> *mut c_void;
    pub fn ttm_bo_vmap(bo: *mut ttm_buffer_object, map: *mut iosys_map) -> c_int;
    pub fn ttm_bo_vunmap(bo: *mut ttm_buffer_object, map: *mut iosys_map);
    pub fn ttm_bo_mmap_obj(vma: *mut vm_area_struct, bo: *mut ttm_buffer_object) -> c_int;
    pub fn ttm_bo_swapout(bdev: *mut ttm_device, ctx: *mut ttm_operation_ctx, man: *mut ttm_resource_manager, gfp_flags: gfp_t, target: s64) -> s64;
    pub fn ttm_bo_pin(bo: *mut ttm_buffer_object);
    pub fn ttm_bo_unpin(bo: *mut ttm_buffer_object);
    pub fn ttm_bo_evict_first(bdev: *mut ttm_device, man: *mut ttm_resource_manager, ctx: *mut ttm_operation_ctx) -> c_int;
    pub fn ttm_bo_evict_cgroup(bdev: *mut ttm_device, man: *mut ttm_resource_manager, limit_pool: *mut dmem_cgroup_pool_state, target_bytes: s64, ctx: *mut ttm_operation_ctx) -> s64;
    pub fn ttm_bo_access(bo: *mut ttm_buffer_object, offset: c_ulong, buf: *mut c_void, len: c_int, write: c_int) -> c_int;
    pub fn ttm_bo_vm_reserve(bo: *mut ttm_buffer_object, vmf: *mut vm_fault) -> vm_fault_t;
    pub fn ttm_bo_vm_fault_reserved(vmf: *mut vm_fault, prot: pgprot_t, num_prefault: pgoff_t) -> vm_fault_t;
    pub fn ttm_bo_vm_fault(vmf: *mut vm_fault) -> vm_fault_t;
    pub fn ttm_bo_vm_open(vma: *mut vm_area_struct);
    pub fn ttm_bo_vm_close(vma: *mut vm_area_struct);
    pub fn ttm_bo_vm_access(vma: *mut vm_area_struct, addr: c_ulong, buf: *mut c_void, len: c_int, write: c_int) -> c_int;
    pub fn ttm_bo_vm_dummy_page(vmf: *mut vm_fault, prot: pgprot_t) -> vm_fault_t;
    pub fn ttm_bo_mem_space(bo: *mut ttm_buffer_object, placement: *mut ttm_placement, mem: *mut *mut ttm_resource, ctx: *mut ttm_operation_ctx) -> c_int;
    pub fn ttm_bo_unmap_virtual(bo: *mut ttm_buffer_object);
    pub fn ttm_mem_io_reserve(bdev: *mut ttm_device, mem: *mut ttm_resource) -> c_int;
    pub fn ttm_mem_io_free(bdev: *mut ttm_device, mem: *mut ttm_resource);
    pub fn ttm_move_memcpy(clear: bool, num_pages: u32, dst_iter: *mut ttm_kmap_iter, src_iter: *mut ttm_kmap_iter);
    pub fn ttm_bo_move_memcpy(bo: *mut ttm_buffer_object, ctx: *mut ttm_operation_ctx, new_mem: *mut ttm_resource) -> c_int;
    pub fn ttm_bo_move_accel_cleanup(bo: *mut ttm_buffer_object, fence: *mut dma_fence, evict: bool, pipeline: bool, new_mem: *mut ttm_resource) -> c_int;
    pub fn ttm_bo_move_sync_cleanup(bo: *mut ttm_buffer_object, new_mem: *mut ttm_resource);
    pub fn ttm_bo_pipeline_gutting(bo: *mut ttm_buffer_object) -> c_int;
    pub fn ttm_io_prot(bo: *mut ttm_buffer_object, res: *mut ttm_resource, tmp: pgprot_t) -> pgprot_t;
    pub fn ttm_bo_tt_destroy(bo: *mut ttm_buffer_object);
    pub fn ttm_bo_populate(bo: *mut ttm_buffer_object, ctx: *mut ttm_operation_ctx) -> c_int;
    pub fn ttm_bo_setup_export(bo: *mut ttm_buffer_object, ctx: *mut ttm_operation_ctx) -> c_int;
}

#[repr(C)]
pub struct ttm_bo_lru_cursor {
    pub res_curs: ttm_resource_cursor,
    pub bo: *mut ttm_buffer_object,
    pub needs_unlock: bool,
    pub arg: *mut ttm_lru_walk_arg,
}

extern "C" {
    pub fn ttm_bo_lru_cursor_fini(curs: *mut ttm_bo_lru_cursor);
    pub fn ttm_bo_lru_cursor_init(curs: *mut ttm_bo_lru_cursor, man: *mut ttm_resource_manager, arg: *mut ttm_lru_walk_arg) -> *mut ttm_bo_lru_cursor;
    pub fn ttm_bo_lru_cursor_first(curs: *mut ttm_bo_lru_cursor) -> *mut ttm_buffer_object;
    pub fn ttm_bo_lru_cursor_next(curs: *mut ttm_bo_lru_cursor) -> *mut ttm_buffer_object;
}

#[inline]
pub unsafe fn ttm_bo_move_to_lru_tail_unlocked(bo: *mut ttm_buffer_object) {
    spin_lock(&mut (*(*bo).bdev).lru_lock);
    ttm_bo_move_to_lru_tail(bo);
    spin_unlock(&mut (*(*bo).bdev).lru_lock);
}

#[inline]
pub unsafe fn ttm_bo_assign_mem(bo: *mut ttm_buffer_object, new_mem: *mut ttm_resource) {
    (*bo).resource = new_mem;
}

#[inline]
pub unsafe fn ttm_bo_move_null(bo: *mut ttm_buffer_object, new_mem: *mut ttm_resource) {
    ttm_resource_free(bo, &mut (*bo).resource);
    ttm_bo_assign_mem(bo, new_mem);
}

#[inline]
pub unsafe fn ttm_bo_unreserve(bo: *mut ttm_buffer_object) {
    ttm_bo_move_to_lru_tail_unlocked(bo);
    dma_resv_unlock((*bo).base.resv);
}

#[inline]
pub unsafe fn ttm_kmap_obj_virtual(map: *mut ttm_bo_kmap_obj, is_iomem: *mut bool) -> *mut c_void {
    *is_iomem = ((*map).bo_kmap_type as u32 & TTM_BO_MAP_IOMEM_MASK) != 0;
    (*map).virtual_
}

// The C guarded-loop and DEFINE_CLASS macros are represented by the cursor
// functions above; their cleanup/iteration semantics remain caller-controlled.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
