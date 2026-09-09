// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
/*
 * Copyright 2013-2016 Freescale Semiconductor Inc.
 */

// Linux I/O and FSL MC declarations are supplied by the surrounding crate.

unsafe fn fsl_mc_io_set_dpmcp(
    mc_io: *mut fsl_mc_io,
    dpmcp_dev: *mut fsl_mc_device,
) -> i32 {
    let error: i32;

    if !(*mc_io).dpmcp_dev.is_null() {
        return -EINVAL;
    }

    if !(*dpmcp_dev).mc_io.is_null() {
        return -EINVAL;
    }

    error = dpmcp_open(
        mc_io,
        0,
        (*dpmcp_dev).obj_desc.id,
        &mut (*dpmcp_dev).mc_handle,
    );
    if error < 0 {
        return error;
    }

    (*mc_io).dpmcp_dev = dpmcp_dev;
    (*dpmcp_dev).mc_io = mc_io;
    0
}

unsafe fn fsl_mc_io_unset_dpmcp(mc_io: *mut fsl_mc_io) {
    let error: i32;
    let dpmcp_dev: *mut fsl_mc_device = (*mc_io).dpmcp_dev;

    error = dpmcp_close(mc_io, 0, (*dpmcp_dev).mc_handle);
    if error < 0 {
        dev_err(
            &mut (*dpmcp_dev).dev,
            b"dpmcp_close() failed: %d\n\0".as_ptr(),
            error,
        );
    }

    (*mc_io).dpmcp_dev = core::ptr::null_mut();
    (*dpmcp_dev).mc_io = core::ptr::null_mut();
}

/**
 * fsl_create_mc_io() - Creates an MC I/O object
 *
 * @dev: device to be associated with the MC I/O object
 * @mc_portal_phys_addr: physical address of the MC portal to use
 * @mc_portal_size: size in bytes of the MC portal
 * @dpmcp_dev: Pointer to the DPMCP object associated with this MC I/O
 * object or NULL if none.
 * @flags: flags for the new MC I/O object
 * @new_mc_io: Area to return pointer to newly created MC I/O object
 *
 * Returns '0' on Success; Error code otherwise.
 */
pub unsafe fn fsl_create_mc_io(
    dev: *mut device,
    mc_portal_phys_addr: phys_addr_t,
    mc_portal_size: u32,
    dpmcp_dev: *mut fsl_mc_device,
    flags: u32,
    new_mc_io: *mut *mut fsl_mc_io,
) -> i32 {
    let mut error: i32;
    let mc_io: *mut fsl_mc_io = devm_kzalloc(dev, core::mem::size_of::<fsl_mc_io>(), GFP_KERNEL);
    let mc_portal_virt_addr: *mut core::ffi::c_void;
    let res: *mut resource;

    if mc_io.is_null() {
        return -ENOMEM;
    }

    (*mc_io).dev = dev;
    (*mc_io).flags = flags;
    (*mc_io).portal_phys_addr = mc_portal_phys_addr;
    (*mc_io).portal_size = mc_portal_size;
    if flags & FSL_MC_IO_ATOMIC_CONTEXT_PORTAL != 0 {
        raw_spin_lock_init(&mut (*mc_io).spinlock);
    } else {
        mutex_init(&mut (*mc_io).mutex);
    }

    res = devm_request_mem_region(dev, mc_portal_phys_addr, mc_portal_size, b"mc_portal\0".as_ptr());
    if res.is_null() {
        dev_err(dev, b"devm_request_mem_region failed for MC portal %pa\n\0".as_ptr(), &mc_portal_phys_addr);
        return -EBUSY;
    }

    mc_portal_virt_addr = devm_ioremap(dev, mc_portal_phys_addr, mc_portal_size);
    if mc_portal_virt_addr.is_null() {
        dev_err(dev, b"devm_ioremap failed for MC portal %pa\n\0".as_ptr(), &mc_portal_phys_addr);
        return -ENXIO;
    }

    (*mc_io).portal_virt_addr = mc_portal_virt_addr;
    if !dpmcp_dev.is_null() {
        error = fsl_mc_io_set_dpmcp(mc_io, dpmcp_dev);
        if error < 0 {
            fsl_destroy_mc_io(mc_io);
            return error;
        }
    }

    *new_mc_io = mc_io;
    0
}

/** fsl_destroy_mc_io() - Destroys an MC I/O object */
pub unsafe fn fsl_destroy_mc_io(mc_io: *mut fsl_mc_io) {
    if mc_io.is_null() {
        return;
    }

    if !(*mc_io).dpmcp_dev.is_null() {
        fsl_mc_io_unset_dpmcp(mc_io);
    }

    devm_iounmap((*mc_io).dev, (*mc_io).portal_virt_addr);
    devm_release_mem_region((*mc_io).dev, (*mc_io).portal_phys_addr, (*mc_io).portal_size);
    (*mc_io).portal_virt_addr = core::ptr::null_mut();
    devm_kfree((*mc_io).dev, mc_io);
}

/** fsl_mc_portal_allocate - Allocates an MC portal */
pub unsafe fn fsl_mc_portal_allocate(
    mc_dev: *mut fsl_mc_device,
    mc_io_flags: u16,
    new_mc_io: *mut *mut fsl_mc_io,
) -> i32 {
    let mc_bus_dev: *mut fsl_mc_device;
    let mc_bus: *mut fsl_mc_bus;
    let mc_portal_phys_addr: phys_addr_t;
    let mc_portal_size: usize;
    let dpmcp_dev: *mut fsl_mc_device;
    let mut error: i32 = -EINVAL;
    let mut resource: *mut fsl_mc_resource = core::ptr::null_mut();
    let mut mc_io: *mut fsl_mc_io = core::ptr::null_mut();

    if (*mc_dev).flags & FSL_MC_IS_DPRC != 0 {
        mc_bus_dev = mc_dev;
    } else {
        if !dev_is_fsl_mc((*mc_dev).dev.parent) {
            return error;
        }
        mc_bus_dev = to_fsl_mc_device((*mc_dev).dev.parent);
    }

    mc_bus = to_fsl_mc_bus(mc_bus_dev);
    *new_mc_io = core::ptr::null_mut();
    error = fsl_mc_resource_allocate(mc_bus, FSL_MC_POOL_DPMCP, &mut resource);
    if error < 0 {
        return error;
    }

    error = -EINVAL;
    dpmcp_dev = (*resource).data;
    if (*dpmcp_dev).obj_desc.ver_major < DPMCP_MIN_VER_MAJOR ||
       ((*dpmcp_dev).obj_desc.ver_major == DPMCP_MIN_VER_MAJOR &&
        (*dpmcp_dev).obj_desc.ver_minor < DPMCP_MIN_VER_MINOR) {
        dev_err(&mut (*dpmcp_dev).dev, b"ERROR: Version %d.%d of DPMCP not supported.\n\0".as_ptr(),
                (*dpmcp_dev).obj_desc.ver_major, (*dpmcp_dev).obj_desc.ver_minor);
        error = -ENOTSUPP;
        fsl_mc_resource_free(resource);
        return error;
    }

    mc_portal_phys_addr = (*dpmcp_dev).regions[0].start;
    mc_portal_size = resource_size((*dpmcp_dev).regions.as_mut_ptr());
    error = fsl_create_mc_io(&mut (*mc_bus_dev).dev, mc_portal_phys_addr, mc_portal_size as u32,
                             dpmcp_dev, mc_io_flags as u32, &mut mc_io);
    if error < 0 {
        fsl_mc_resource_free(resource);
        return error;
    }

    if mc_dev != mc_bus_dev {
        (*dpmcp_dev).consumer_link = device_link_add(&mut (*mc_dev).dev, &mut (*dpmcp_dev).dev,
                                                      DL_FLAG_AUTOREMOVE_CONSUMER);
        if (*dpmcp_dev).consumer_link.is_null() {
            fsl_destroy_mc_io(mc_io);
            fsl_mc_resource_free(resource);
            return -EINVAL;
        }
    }

    *new_mc_io = mc_io;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
