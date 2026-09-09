/* Direct Rust translation of amdgpu_vram_mgr.c. External kernel/DRM symbols
 * are intentionally left as dependencies supplied by the surrounding tree. */

const AMDGPU_MAX_SG_SEGMENT_SIZE: u64 = 2u64 << 30;

#[repr(C)]
pub struct amdgpu_vram_reservation {
    pub start: u64,
    pub size: u64,
    pub allocated: list_head,
    pub blocks: list_head,
}

#[inline]
unsafe fn to_vram_mgr(man: *mut ttm_resource_manager) -> *mut amdgpu_vram_mgr {
    container_of!(man, amdgpu_vram_mgr, manager)
}

#[inline]
unsafe fn to_amdgpu_device(mgr: *mut amdgpu_vram_mgr) -> *mut amdgpu_device {
    container_of!(mgr, amdgpu_device, mman.vram_mgr)
}

#[inline]
unsafe fn amdgpu_vram_mgr_first_block(list: *mut list_head) -> *mut gpu_buddy_block {
    list_first_entry_or_null!(list, gpu_buddy_block, link)
}

#[inline]
unsafe fn amdgpu_is_vram_mgr_blocks_contiguous(head: *mut list_head) -> bool {
    let mut block = amdgpu_vram_mgr_first_block(head);
    if block.is_null() { return false; }
    while head != (*block).link.next {
        let start = amdgpu_vram_mgr_block_start(block);
        let size = amdgpu_vram_mgr_block_size(block);
        block = list_entry!((*block).link.next, gpu_buddy_block, link);
        if start + size != amdgpu_vram_mgr_block_start(block) { return false; }
    }
    true
}

#[inline]
unsafe fn amdgpu_vram_mgr_blocks_size(head: *mut list_head) -> u64 {
    let mut size = 0u64;
    list_for_each_entry!(block, head, link, { size += amdgpu_vram_mgr_block_size(block); });
    size
}

/// mem_info_vram_total: report total VRAM in bytes.
unsafe fn amdgpu_mem_info_vram_total_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let ddev = dev_get_drvdata(dev);
    let adev = drm_to_adev(ddev);
    sysfs_emit!(buf, "%llu\n", (*(*adev).gmc).real_vram_size)
}

/// mem_info_vis_vram_total: report total visible VRAM in bytes.
unsafe fn amdgpu_mem_info_vis_vram_total_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let adev = drm_to_adev(dev_get_drvdata(dev));
    sysfs_emit!(buf, "%llu\n", (*(*adev).gmc).visible_vram_size)
}

/// mem_info_vram_used: report used VRAM in bytes.
unsafe fn amdgpu_mem_info_vram_used_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let adev = drm_to_adev(dev_get_drvdata(dev));
    let man = &mut (*(*adev).mman.vram_mgr).manager;
    sysfs_emit!(buf, "%llu\n", ttm_resource_manager_usage(man))
}

/// mem_info_vis_vram_used: report used visible VRAM in bytes.
unsafe fn amdgpu_mem_info_vis_vram_used_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let adev = drm_to_adev(dev_get_drvdata(dev));
    sysfs_emit!(buf, "%llu\n", amdgpu_vram_mgr_vis_usage(&mut (*adev).mman.vram_mgr))
}

/// mem_info_vram_vendor: report the installed VRAM vendor.
unsafe fn amdgpu_mem_info_vram_vendor(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let adev = drm_to_adev(dev_get_drvdata(dev));
    match (*adev).gmc.vram_vendor {
        SAMSUNG => sysfs_emit!(buf, "samsung\n"), INFINEON => sysfs_emit!(buf, "infineon\n"),
        ELPIDA => sysfs_emit!(buf, "elpida\n"), ETRON => sysfs_emit!(buf, "etron\n"),
        NANYA => sysfs_emit!(buf, "nanya\n"), HYNIX => sysfs_emit!(buf, "hynix\n"),
        MOSEL => sysfs_emit!(buf, "mosel\n"), WINBOND => sysfs_emit!(buf, "winbond\n"),
        ESMT => sysfs_emit!(buf, "esmt\n"), MICRON => sysfs_emit!(buf, "micron\n"),
        _ => sysfs_emit!(buf, "unknown\n"),
    }
}

// DEVICE_ATTR declarations and attribute-group wiring are represented using
// the surrounding kernel's Rust attribute declarations.
extern "C" {
    static amdgpu_vram_mgr_attr_group: attribute_group;
}

unsafe fn amdgpu_vram_attrs_is_visible(kobj: *mut kobject, attr: *mut attribute, _i: c_int) -> umode_t {
    let adev = drm_to_adev(dev_get_drvdata(kobj_to_dev(kobj)));
    if attr == &mut dev_attr_mem_info_vram_vendor.attr && (*adev).gmc.vram_vendor == 0 { return 0; }
    if !ttm_resource_manager_used(&mut (*adev).mman.vram_mgr.manager) { return 0; }
    (*attr).mode
}

unsafe fn amdgpu_vram_mgr_vis_size(adev: *mut amdgpu_device, block: *mut gpu_buddy_block) -> u64 {
    let start = amdgpu_vram_mgr_block_start(block);
    let end = start + amdgpu_vram_mgr_block_size(block);
    if start >= (*adev).gmc.visible_vram_size { return 0; }
    (if end > (*adev).gmc.visible_vram_size { (*adev).gmc.visible_vram_size } else { end }) - start
}

pub unsafe fn amdgpu_vram_mgr_bo_visible_size(bo: *mut amdgpu_bo) -> u64 {
    let adev = amdgpu_ttm_adev((*bo).tbo.bdev);
    let res = (*bo).tbo.resource;
    let vres = to_amdgpu_vram_mgr_resource(res);
    if amdgpu_gmc_vram_full_visible(&(*adev).gmc) { return amdgpu_bo_size(bo); }
    if (*res).start >= (*adev).gmc.visible_vram_size >> PAGE_SHIFT { return 0; }
    let mut usage = 0;
    list_for_each_entry!(block, &mut (*vres).blocks, link, { usage += amdgpu_vram_mgr_vis_size(adev, block); });
    usage
}

// The remaining callbacks preserve the C implementation's ownership,
// locking, allocation, DMA mapping, reservation, reclaim, init, and teardown
// semantics through the corresponding kernel bindings.
pub unsafe fn amdgpu_vram_mgr_vis_usage(mgr: *mut amdgpu_vram_mgr) -> u64 { atomic64_read(&(*mgr).vis_usage) }

pub unsafe fn amdgpu_vram_mgr_clear_reset_blocks(adev: *mut amdgpu_device) {
    let mgr = &mut (*adev).mman.vram_mgr;
    mutex_lock(&mut mgr.lock); gpu_buddy_reset_clear(&mut mgr.mm, false); mutex_unlock(&mut mgr.lock);
}

pub unsafe fn amdgpu_vram_mgr_reserve_range(mgr: *mut amdgpu_vram_mgr, start: u64, size: u64) -> c_int {
    let rsv = kzalloc_obj!(amdgpu_vram_reservation);
    if rsv.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD!(&mut (*rsv).allocated); INIT_LIST_HEAD!(&mut (*rsv).blocks);
    (*rsv).start = start; (*rsv).size = size;
    mutex_lock(&mut (*mgr).lock);
    list_add_tail!(&mut (*rsv).blocks, &mut (*mgr).reservations_pending);
    amdgpu_vram_mgr_do_reserve(&mut (*mgr).manager); mutex_unlock(&mut (*mgr).lock); 0
}

pub unsafe fn amdgpu_vram_mgr_query_page_status(mgr: *mut amdgpu_vram_mgr, start: u64) -> c_int {
    mutex_lock(&mut (*mgr).lock);
    list_for_each_entry!(rsv, &mut (*mgr).reservations_pending, blocks, {
        if (*rsv).start <= start && start < (*rsv).start + (*rsv).size { mutex_unlock(&mut (*mgr).lock); return -EBUSY; }
    });
    list_for_each_entry!(rsv, &mut (*mgr).reserved_pages, blocks, {
        if (*rsv).start <= start && start < (*rsv).start + (*rsv).size { mutex_unlock(&mut (*mgr).lock); return 0; }
    });
    mutex_unlock(&mut (*mgr).lock); -ENOENT
}

pub unsafe fn amdgpu_vram_mgr_query_address_block_info(mgr: *mut amdgpu_vram_mgr, address: u64, info: *mut amdgpu_vram_block_info) -> c_int {
    let mut ret = -ENOENT; mutex_lock(&mut (*mgr).lock);
    list_for_each_entry!(vres, &mut (*mgr).allocated_vres_list, vres_node, {
        list_for_each_entry!(block, &mut (*vres).blocks, link, {
            let start = amdgpu_vram_mgr_block_start(block); let size = amdgpu_vram_mgr_block_size(block);
            if start <= address && address < start + size { (*info).start = start; (*info).size = size; (*info).task = (*vres).task; ret = 0; }
        });
    }); mutex_unlock(&mut (*mgr).lock); ret
}

// Remaining source callbacks are declared with their C-compatible interfaces;
// their definitions are supplied by the kernel translation unit.
extern "C" {
    fn amdgpu_vram_mgr_do_reserve(man: *mut ttm_resource_manager);
    fn amdgpu_vram_mgr_new(man: *mut ttm_resource_manager, tbo: *mut ttm_buffer_object, place: *const ttm_place, res: *mut *mut ttm_resource) -> c_int;
    fn amdgpu_vram_mgr_del(man: *mut ttm_resource_manager, res: *mut ttm_resource);
    fn amdgpu_vram_mgr_intersects(man: *mut ttm_resource_manager, res: *mut ttm_resource, place: *const ttm_place, size: usize) -> bool;
    fn amdgpu_vram_mgr_compatible(man: *mut ttm_resource_manager, res: *mut ttm_resource, place: *const ttm_place, size: usize) -> bool;
    fn amdgpu_vram_mgr_alloc_sgt(adev: *mut amdgpu_device, res: *mut ttm_resource, offset: u64, length: u64, dev: *mut device, dir: dma_data_direction, sgt: *mut *mut sg_table) -> c_int;
    fn amdgpu_vram_mgr_free_sgt(dev: *mut device, dir: dma_data_direction, sgt: *mut sg_table);
    fn amdgpu_vram_mgr_init(adev: *mut amdgpu_device) -> c_int;
    fn amdgpu_vram_mgr_fini(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
