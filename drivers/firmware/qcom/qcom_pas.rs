// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2010,2015,2019 The Linux Foundation. All rights reserved.
 * Copyright (C) 2015 Linaro Ltd.
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// Dependencies supplied by the corresponding Linux headers and qcom_pas.h.

static mut ops_ptr: *mut qcom_pas_ops = core::ptr::null_mut();

pub unsafe fn devm_qcom_pas_context_alloc(
    dev: *mut device,
    pas_id: u32,
    mem_phys: phys_addr_t,
    mem_size: usize,
) -> *mut qcom_pas_context {
    let ctx = devm_kzalloc(dev, core::mem::size_of::<qcom_pas_context>(), GFP_KERNEL)
        as *mut qcom_pas_context;
    if ctx.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*ctx).dev = dev;
    (*ctx).pas_id = pas_id;
    (*ctx).mem_phys = mem_phys;
    (*ctx).mem_size = mem_size;

    ctx
}

pub unsafe fn qcom_pas_init_image(
    pas_id: u32,
    metadata: *const core::ffi::c_void,
    size: usize,
    ctx: *mut qcom_pas_context,
) -> i32 {
    if ops_ptr.is_null() {
        return -ENODEV;
    }

    ((*ops_ptr).init_image)((*ops_ptr).dev, pas_id, metadata, size, ctx)
}

pub unsafe fn qcom_pas_metadata_release(ctx: *mut qcom_pas_context) {
    if ops_ptr.is_null() || ctx.is_null() || (*ctx).ptr.is_null() {
        return;
    }

    ((*ops_ptr).metadata_release)((*ops_ptr).dev, ctx);
}

pub unsafe fn qcom_pas_mem_setup(
    pas_id: u32,
    addr: phys_addr_t,
    size: phys_addr_t,
) -> i32 {
    if ops_ptr.is_null() {
        return -ENODEV;
    }

    ((*ops_ptr).mem_setup)((*ops_ptr).dev, pas_id, addr, size)
}

pub unsafe fn qcom_pas_get_rsc_table(
    ctx: *mut qcom_pas_context,
    input_rt: *mut core::ffi::c_void,
    input_rt_size: usize,
    output_rt_size: *mut usize,
) -> *mut resource_table {
    if ops_ptr.is_null() {
        return ERR_PTR(-ENODEV);
    }
    if ctx.is_null() {
        return ERR_PTR(-EINVAL);
    }

    ((*ops_ptr).get_rsc_table)((*ops_ptr).dev, ctx, input_rt, input_rt_size, output_rt_size)
}

pub unsafe fn qcom_pas_auth_and_reset(pas_id: u32) -> i32 {
    if ops_ptr.is_null() {
        return -ENODEV;
    }

    ((*ops_ptr).auth_and_reset)((*ops_ptr).dev, pas_id)
}

pub unsafe fn qcom_pas_prepare_and_auth_reset(ctx: *mut qcom_pas_context) -> i32 {
    if ops_ptr.is_null() {
        return -ENODEV;
    }
    if ctx.is_null() {
        return -EINVAL;
    }

    ((*ops_ptr).prepare_and_auth_reset)((*ops_ptr).dev, ctx)
}

pub unsafe fn qcom_pas_set_remote_state(state: u32, pas_id: u32) -> i32 {
    if ops_ptr.is_null() {
        return -ENODEV;
    }

    ((*ops_ptr).set_remote_state)((*ops_ptr).dev, state, pas_id)
}

pub unsafe fn qcom_pas_shutdown(pas_id: u32) -> i32 {
    if ops_ptr.is_null() {
        return -ENODEV;
    }

    ((*ops_ptr).shutdown)((*ops_ptr).dev, pas_id)
}

pub unsafe fn qcom_pas_supported(pas_id: u32) -> bool {
    if ops_ptr.is_null() {
        return false;
    }

    ((*ops_ptr).supported)((*ops_ptr).dev, pas_id)
}

pub unsafe fn qcom_pas_is_available() -> bool {
    /*
     * The barrier for ops_ptr is intended to synchronize the data stores
     * for the ops data structure when client drivers are in parallel
     * checking for PAS service availability.
     *
     * Once the PAS backend becomes available, it is allowed for multiple
     * threads to enter TZ for parallel bringup of co-processors during
     * boot.
     */
    !core::ptr::read_volatile(&ops_ptr).is_null()
}

pub unsafe fn qcom_pas_ops_register(ops: *mut qcom_pas_ops) {
    if !qcom_pas_is_available() {
        /* Paired with smp_load_acquire() in qcom_pas_is_available() */
        core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
        ops_ptr = ops;
    } else {
        pr_err("qcom_pas: ops already registered by %s\n", (*ops_ptr).drv_name);
    }
}

pub unsafe fn qcom_pas_ops_unregister() {
    /* Paired with smp_load_acquire() in qcom_pas_is_available() */
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
    ops_ptr = core::ptr::null_mut();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
