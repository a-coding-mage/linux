// SPDX-License-Identifier: GPL-2.0
/* fsl-mc object allocator driver */

// Kernel-provided types, constants, macros, and functions are external dependencies.

unsafe fn fsl_mc_is_allocatable(mc_dev: *mut fsl_mc_device) -> bool {
    is_fsl_mc_bus_dpbp(mc_dev) || is_fsl_mc_bus_dpmcp(mc_dev) || is_fsl_mc_bus_dpcon(mc_dev)
}

unsafe fn fsl_mc_resource_pool_add_device(
    mc_bus: *mut fsl_mc_bus, pool_type: fsl_mc_pool_type, mc_dev: *mut fsl_mc_device,
) -> i32 {
    let mc_bus_dev = &mut (*mc_bus).mc_dev;
    let mut error = -EINVAL;
    if pool_type < 0 || pool_type >= FSL_MC_NUM_POOL_TYPES || !fsl_mc_is_allocatable(mc_dev) || !(*mc_dev).resource.is_null() { return error; }
    let res_pool = &mut (*mc_bus).resource_pools[pool_type as usize];
    if res_pool.r#type != pool_type || res_pool.mc_bus != mc_bus { return error; }
    mutex_lock(&mut res_pool.mutex);
    if res_pool.max_count < 0 || res_pool.free_count < 0 || res_pool.free_count > res_pool.max_count { mutex_unlock(&mut res_pool.mutex); return error; }
    let resource = devm_kzalloc(&mut mc_bus_dev.dev, core::mem::size_of::<fsl_mc_resource>(), GFP_KERNEL) as *mut fsl_mc_resource;
    if resource.is_null() { dev_err(&mut mc_bus_dev.dev, "Failed to allocate memory for fsl_mc_resource\n"); mutex_unlock(&mut res_pool.mutex); return -ENOMEM; }
    (*resource).r#type = pool_type;
    (*resource).id = (*mc_dev).obj_desc.id;
    (*resource).data = mc_dev as *mut core::ffi::c_void;
    (*resource).parent_pool = res_pool;
    INIT_LIST_HEAD(&mut (*resource).node);
    list_add_tail(&mut (*resource).node, &mut res_pool.free_list);
    (*mc_dev).resource = resource;
    res_pool.free_count += 1; res_pool.max_count += 1; error = 0;
    mutex_unlock(&mut res_pool.mutex); error
}

unsafe fn fsl_mc_resource_pool_remove_device(mc_dev: *mut fsl_mc_device) -> i32 {
    let mc_bus_dev = to_fsl_mc_device((*mc_dev).dev.parent);
    let mc_bus = to_fsl_mc_bus(mc_bus_dev);
    let resource = (*mc_dev).resource;
    if resource.is_null() || (*resource).data != mc_dev as *mut core::ffi::c_void { dev_err(&mut (*mc_bus_dev).dev, "resource mismatch\n"); return -EINVAL; }
    let res_pool = (*resource).parent_pool;
    if res_pool != &mut (*mc_bus).resource_pools[(*resource).r#type as usize] { dev_err(&mut (*mc_bus_dev).dev, "pool mismatch\n"); return -EINVAL; }
    mutex_lock(&mut (*res_pool).mutex);
    if (*res_pool).max_count <= 0 { dev_err(&mut (*mc_bus_dev).dev, "max_count underflow\n"); mutex_unlock(&mut (*res_pool).mutex); return -EINVAL; }
    if (*res_pool).free_count <= 0 || (*res_pool).free_count > (*res_pool).max_count { dev_err(&mut (*mc_bus_dev).dev, "free_count mismatch\n"); mutex_unlock(&mut (*res_pool).mutex); return -EINVAL; }
    if list_empty(&(*resource).node) { dev_err(&mut (*mc_bus_dev).dev, "Device cannot be removed from resource pool\n"); mutex_unlock(&mut (*res_pool).mutex); return -EBUSY; }
    list_del_init(&mut (*resource).node); (*res_pool).free_count -= 1; (*res_pool).max_count -= 1;
    devm_kfree(&mut (*mc_bus_dev).dev, resource as *mut core::ffi::c_void); (*mc_dev).resource = core::ptr::null_mut();
    mutex_unlock(&mut (*res_pool).mutex); 0
}

static FSL_MC_POOL_TYPE_STRINGS: [&[u8]; 4] = [b"dpmcp\0", b"dpbp\0", b"dpcon\0", b"irq\0"];

unsafe fn object_type_to_pool_type(object_type: *const i8, pool_type: *mut fsl_mc_pool_type) -> i32 {
    for i in 0..FSL_MC_POOL_TYPE_STRINGS.len() { if strcmp(object_type, FSL_MC_POOL_TYPE_STRINGS[i].as_ptr() as *const i8) == 0 { *pool_type = i as fsl_mc_pool_type; return 0; } }
    -EINVAL
}

pub unsafe fn fsl_mc_resource_allocate(mc_bus: *mut fsl_mc_bus, pool_type: fsl_mc_pool_type, new_resource: *mut *mut fsl_mc_resource) -> i32 {
    *new_resource = core::ptr::null_mut();
    if pool_type < 0 || pool_type >= FSL_MC_NUM_POOL_TYPES { return -EINVAL; }
    let res_pool = &mut (*mc_bus).resource_pools[pool_type as usize];
    if res_pool.mc_bus != mc_bus { return -EINVAL; }
    mutex_lock(&mut res_pool.mutex);
    let resource = list_first_entry_or_null(&mut res_pool.free_list);
    if resource.is_null() { mutex_unlock(&mut res_pool.mutex); return -ENXIO; }
    if (*resource).r#type != pool_type || (*resource).parent_pool != res_pool || res_pool.free_count <= 0 || res_pool.free_count > res_pool.max_count { mutex_unlock(&mut res_pool.mutex); return -EINVAL; }
    list_del_init(&mut (*resource).node); res_pool.free_count -= 1; mutex_unlock(&mut res_pool.mutex); *new_resource = resource; 0
}

pub unsafe fn fsl_mc_resource_free(resource: *mut fsl_mc_resource) {
    let res_pool = (*resource).parent_pool;
    if (*resource).r#type != (*res_pool).r#type { return; }
    mutex_lock(&mut (*res_pool).mutex);
    if (*res_pool).free_count < 0 || (*res_pool).free_count >= (*res_pool).max_count || !list_empty(&(*resource).node) { mutex_unlock(&mut (*res_pool).mutex); return; }
    list_add_tail(&mut (*resource).node, &mut (*res_pool).free_list); (*res_pool).free_count += 1; mutex_unlock(&mut (*res_pool).mutex);
}

// Remaining exported allocator operations retain the C driver's direct kernel-object interactions.
pub unsafe fn fsl_mc_object_allocate(mc_dev: *mut fsl_mc_device, pool_type: fsl_mc_pool_type, new_mc_adev: *mut *mut fsl_mc_device) -> i32 {
    *new_mc_adev = core::ptr::null_mut();
    if (*mc_dev).flags & FSL_MC_IS_DPRC != 0 || !dev_is_fsl_mc((*mc_dev).dev.parent) || pool_type == FSL_MC_POOL_DPMCP { return -EINVAL; }
    let mc_bus = to_fsl_mc_bus(to_fsl_mc_device((*mc_dev).dev.parent)); let mut resource = core::ptr::null_mut();
    let error = fsl_mc_resource_allocate(mc_bus, pool_type, &mut resource); if error < 0 { return error; }
    let mc_adev = (*resource).data as *mut fsl_mc_device; if mc_adev.is_null() { fsl_mc_resource_free(resource); return -EINVAL; }
    (*mc_adev).consumer_link = device_link_add(&mut (*mc_dev).dev, &mut (*mc_adev).dev, DL_FLAG_AUTOREMOVE_CONSUMER);
    if (*mc_adev).consumer_link.is_null() { fsl_mc_resource_free(resource); return -EINVAL; }
    *new_mc_adev = mc_adev; 0
}

pub unsafe fn fsl_mc_object_free(mc_adev: *mut fsl_mc_device) { let resource = (*mc_adev).resource; if resource.is_null() || (*resource).r#type == FSL_MC_POOL_DPMCP || (*resource).data != mc_adev as *mut core::ffi::c_void { return; } fsl_mc_resource_free(resource); (*mc_adev).consumer_link = core::ptr::null_mut(); }

pub unsafe fn fsl_mc_populate_irq_pool(mc_bus_dev: *mut fsl_mc_device, irq_count: u32) -> i32 {
    let mc_bus = to_fsl_mc_bus(mc_bus_dev); let pool = &mut (*mc_bus).resource_pools[FSL_MC_POOL_IRQ as usize];
    if !(*mc_bus).irq_resources.is_null() { return 0; }
    if irq_count == 0 || irq_count > FSL_MC_IRQ_POOL_MAX_TOTAL_IRQS { return -EINVAL; }
    let mut error = fsl_mc_msi_domain_alloc_irqs(&mut (*mc_bus_dev).dev, irq_count); if error < 0 { return error; }
    let resources = devm_kcalloc(&mut (*mc_bus_dev).dev, irq_count as usize, core::mem::size_of::<fsl_mc_device_irq>(), GFP_KERNEL) as *mut fsl_mc_device_irq;
    if resources.is_null() { fsl_mc_msi_domain_free_irqs(&mut (*mc_bus_dev).dev); return -ENOMEM; }
    for i in 0..irq_count as usize { let irq = resources.add(i); (*irq).resource.r#type = pool.r#type; (*irq).resource.data = irq as *mut core::ffi::c_void; (*irq).resource.parent_pool = pool; (*irq).virq = msi_get_virq(&mut (*mc_bus_dev).dev, i as u32); (*irq).resource.id = (*irq).virq; INIT_LIST_HEAD(&mut (*irq).resource.node); list_add_tail(&mut (*irq).resource.node, &mut pool.free_list); }
    pool.max_count = irq_count as i32; pool.free_count = irq_count as i32; (*mc_bus).irq_resources = resources; error = 0; error
}
pub unsafe fn fsl_mc_cleanup_irq_pool(mc_bus_dev: *mut fsl_mc_device) { let mc_bus = to_fsl_mc_bus(mc_bus_dev); let pool = &mut (*mc_bus).resource_pools[FSL_MC_POOL_IRQ as usize]; if (*mc_bus).irq_resources.is_null() || pool.max_count == 0 || pool.free_count != pool.max_count { return; } INIT_LIST_HEAD(&mut pool.free_list); pool.max_count = 0; pool.free_count = 0; (*mc_bus).irq_resources = core::ptr::null_mut(); fsl_mc_msi_domain_free_irqs(&mut (*mc_bus_dev).dev); }
pub unsafe fn fsl_mc_allocate_irqs(mc_dev: *mut fsl_mc_device) -> i32 { if !(*mc_dev).irqs.is_null() || (*mc_dev).obj_desc.irq_count == 0 { return -EINVAL; } let bus = if is_fsl_mc_bus_dprc(mc_dev) { to_fsl_mc_bus(mc_dev) } else { to_fsl_mc_bus(to_fsl_mc_device((*mc_dev).dev.parent)) }; if (*bus).irq_resources.is_null() { return -EINVAL; } let n = (*mc_dev).obj_desc.irq_count as usize; if (*bus).resource_pools[FSL_MC_POOL_IRQ as usize].free_count < n as i32 { return -ENOSPC; } let irqs = devm_kcalloc(&mut (*mc_dev).dev, n, core::mem::size_of::<*mut fsl_mc_device_irq>(), GFP_KERNEL) as *mut *mut fsl_mc_device_irq; if irqs.is_null() { return -ENOMEM; } for i in 0..n { let mut r = core::ptr::null_mut(); let e = fsl_mc_resource_allocate(bus, FSL_MC_POOL_IRQ, &mut r); if e < 0 { return e; } *irqs.add(i) = to_fsl_mc_irq(r); (**irqs.add(i)).mc_dev = mc_dev; (**irqs.add(i)).dev_irq_index = i as i32; } (*mc_dev).irqs = irqs; 0 }
pub unsafe fn fsl_mc_free_irqs(mc_dev: *mut fsl_mc_device) { let irqs = (*mc_dev).irqs; if irqs.is_null() { return; } for i in 0..(*mc_dev).obj_desc.irq_count as usize { (**irqs.add(i)).mc_dev = core::ptr::null_mut(); fsl_mc_resource_free(&mut (**irqs.add(i)).resource); } (*mc_dev).irqs = core::ptr::null_mut(); }
pub unsafe fn fsl_mc_init_all_resource_pools(mc_bus_dev: *mut fsl_mc_device) { let bus = to_fsl_mc_bus(mc_bus_dev); for i in 0..FSL_MC_NUM_POOL_TYPES as usize { let p = &mut (*bus).resource_pools[i]; p.r#type = i as fsl_mc_pool_type; p.max_count = 0; p.free_count = 0; p.mc_bus = bus; INIT_LIST_HEAD(&mut p.free_list); mutex_init(&mut p.mutex); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
