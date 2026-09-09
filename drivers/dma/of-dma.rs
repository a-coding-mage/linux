// SPDX-License-Identifier: GPL-2.0-only
/*
 * Device tree helpers for DMA request / controller
 *
 * Based on of_gpio.c
 *
 * Copyright (C) 2012 Texas Instruments Incorporated - http://www.ti.com/
 */

// Linux headers and "dmaengine.h" provide the external types, functions,
// constants, list primitives, locking, allocation, and diagnostic helpers
// referenced below.

static mut OF_DMA_LIST: ListHead = ListHead::new();
static mut OF_DMA_LOCK: Mutex = Mutex::new();

/// Get a DMA controller in DT DMA helpers list.
unsafe fn of_dma_find_controller(dma_spec: *const OfPhandleArgs) -> *mut OfDma {
    let mut ofdma: *mut OfDma = core::ptr::null_mut();

    list_for_each_entry!(ofdma, OF_DMA_LIST, of_dma_controllers, {
        if (*ofdma).of_node == (*dma_spec).np {
            return ofdma;
        }
    });

    pr_debug!("{}: can't find DMA controller %pOF\n", "of_dma_find_controller", (*dma_spec).np);
    core::ptr::null_mut()
}

/// Translation function for router devices.
unsafe fn of_dma_router_xlate(
    dma_spec: *mut OfPhandleArgs,
    ofdma: *mut OfDma,
) -> *mut DmaChan {
    let mut chan: *mut DmaChan;
    let ofdma_target: *mut OfDma;
    let mut dma_spec_target: OfPhandleArgs = core::ptr::read(dma_spec);
    let route_data: *mut core::ffi::c_void;

    // translate the request for the real DMA controller
    route_data = ((*ofdma).of_dma_route_allocate)(&mut dma_spec_target, ofdma);
    if IS_ERR!(route_data) {
        return core::ptr::null_mut();
    }

    ofdma_target = of_dma_find_controller(&dma_spec_target);
    if ofdma_target.is_null() {
        ((*ofdma).dma_router).route_free(
            (*ofdma).dma_router.dev,
            route_data,
        );
        chan = ERR_PTR!(-EPROBE_DEFER);
        goto!(err);
    }

    chan = ((*ofdma_target).of_dma_xlate)(&mut dma_spec_target, ofdma_target);
    if IS_ERR_OR_NULL!(chan) {
        ((*ofdma).dma_router).route_free(
            (*ofdma).dma_router.dev,
            route_data,
        );
    } else {
        let mut ret: i32 = 0;

        (*chan).router = (*ofdma).dma_router;
        (*chan).route_data = route_data;

        if let Some(device_router_config) = (*(*chan).device).device_router_config {
            ret = device_router_config(chan);
        }

        if ret != 0 {
            dma_release_channel(chan);
            chan = ERR_PTR!(ret);
        }
    }

    // Need to put the node back since route_allocate took it for generating
    // the new, translated dma_spec.
    of_node_put(dma_spec_target.np);
    return chan;

    // C label target; retained as a local control-flow marker for translation.
    err: {
        of_node_put(dma_spec_target.np);
        return chan;
    }
}

/// Register a DMA controller to DT DMA helpers.
pub unsafe fn of_dma_controller_register(
    np: *mut DeviceNode,
    of_dma_xlate: Option<unsafe extern "C" fn(*mut OfPhandleArgs, *mut OfDma) -> *mut DmaChan>,
    data: *mut core::ffi::c_void,
) -> i32 {
    let ofdma = kzalloc_obj::<OfDma>();

    if np.is_null() || of_dma_xlate.is_none() {
        pr_err!("{}: not enough information provided\n", "of_dma_controller_register");
        return -EINVAL;
    }
    if ofdma.is_null() {
        return -ENOMEM;
    }

    (*ofdma).of_node = np;
    (*ofdma).of_dma_xlate = of_dma_xlate;
    (*ofdma).of_dma_data = data;

    // Now queue of_dma controller structure in list
    mutex_lock(&mut OF_DMA_LOCK);
    list_add_tail!(&mut (*ofdma).of_dma_controllers, &mut OF_DMA_LIST);
    mutex_unlock(&mut OF_DMA_LOCK);
    0
}

/// Remove a DMA controller from DT DMA helpers list.
pub unsafe fn of_dma_controller_free(np: *mut DeviceNode) {
    mutex_lock(&mut OF_DMA_LOCK);
    let mut ofdma: *mut OfDma = core::ptr::null_mut();
    list_for_each_entry!(ofdma, OF_DMA_LIST, of_dma_controllers, {
        if (*ofdma).of_node == np {
            list_del!(&mut (*ofdma).of_dma_controllers);
            kfree(ofdma);
            break;
        }
    });
    mutex_unlock(&mut OF_DMA_LOCK);
}

/// Register a DMA router to DT DMA helpers as a controller.
pub unsafe fn of_dma_router_register(
    np: *mut DeviceNode,
    of_dma_route_allocate: Option<unsafe extern "C" fn(*mut OfPhandleArgs, *mut OfDma) -> *mut core::ffi::c_void>,
    dma_router: *mut DmaRouter,
) -> i32 {
    let ofdma = kzalloc_obj::<OfDma>();

    if np.is_null() || of_dma_route_allocate.is_none() || dma_router.is_null() {
        pr_err!("{}: not enough information provided\n", "of_dma_router_register");
        return -EINVAL;
    }
    if ofdma.is_null() {
        return -ENOMEM;
    }

    (*ofdma).of_node = np;
    (*ofdma).of_dma_xlate = Some(of_dma_router_xlate);
    (*ofdma).of_dma_route_allocate = of_dma_route_allocate;
    (*ofdma).dma_router = dma_router;

    // Now queue of_dma controller structure in list
    mutex_lock(&mut OF_DMA_LOCK);
    list_add_tail!(&mut (*ofdma).of_dma_controllers, &mut OF_DMA_LIST);
    mutex_unlock(&mut OF_DMA_LOCK);
    0
}

unsafe fn of_dma_match_channel(
    np: *mut DeviceNode,
    name: *const core::ffi::c_char,
    index: i32,
    dma_spec: *mut OfPhandleArgs,
) -> i32 {
    let mut s: *const core::ffi::c_char = core::ptr::null();
    if of_property_read_string_index(np, b"dma-names\0".as_ptr() as _, index, &mut s) != 0 {
        return -ENODEV;
    }
    if strcmp(name, s) != 0 {
        return -ENODEV;
    }
    if of_parse_phandle_with_args(np, b"dmas\0".as_ptr() as _, b"#dma-cells\0".as_ptr() as _, index, dma_spec) != 0 {
        return -ENODEV;
    }
    0
}

pub unsafe fn of_dma_request_slave_channel(np: *mut DeviceNode, name: *const core::ffi::c_char) -> *mut DmaChan {
    let mut dma_spec = core::mem::MaybeUninit::<OfPhandleArgs>::uninit();
    let mut chan: *mut DmaChan;
    let mut count: i32;
    let mut ret_no_channel = -ENODEV;
    static mut LAST_INDEX: Atomic = Atomic::new(0);

    if np.is_null() || name.is_null() {
        pr_err!("{}: not enough information provided\n", "of_dma_request_slave_channel");
        return ERR_PTR!(-ENODEV);
    }
    if !of_property_present(np, b"dmas\0".as_ptr() as _) {
        return ERR_PTR!(-ENODEV);
    }
    count = of_property_count_strings(np, b"dma-names\0".as_ptr() as _);
    if count < 0 {
        pr_err!("of_dma_request_slave_channel: dma-names property missing or empty\n");
        return ERR_PTR!(-ENODEV);
    }

    let start = LAST_INDEX.inc_return();
    for i in 0..count {
        if of_dma_match_channel(np, name, (i + start) % count, dma_spec.as_mut_ptr()) != 0 { continue; }
        mutex_lock(&mut OF_DMA_LOCK);
        let ofdma = of_dma_find_controller(dma_spec.as_ptr());
        if !ofdma.is_null() {
            chan = ((*ofdma).of_dma_xlate)(dma_spec.as_mut_ptr(), ofdma);
        } else {
            ret_no_channel = -EPROBE_DEFER;
            chan = core::ptr::null_mut();
        }
        mutex_unlock(&mut OF_DMA_LOCK);
        of_node_put((*dma_spec.as_ptr()).np);
        if !chan.is_null() { return chan; }
    }
    ERR_PTR!(ret_no_channel)
}

pub unsafe fn of_dma_simple_xlate(dma_spec: *mut OfPhandleArgs, ofdma: *mut OfDma) -> *mut DmaChan {
    let count = (*dma_spec).args_count;
    let info = (*ofdma).of_dma_data as *mut OfDmaFilterInfo;
    if info.is_null() || (*info).filter_fn.is_none() || count != 1 { return core::ptr::null_mut(); }
    __dma_request_channel(&(*info).dma_cap, (*info).filter_fn, &(*dma_spec).args[0], (*dma_spec).np)
}

pub unsafe fn of_dma_xlate_by_chan_id(dma_spec: *mut OfPhandleArgs, ofdma: *mut OfDma) -> *mut DmaChan {
    let dev = (*ofdma).of_dma_data as *mut DmaDevice;
    if dev.is_null() || (*dma_spec).args_count != 1 { return core::ptr::null_mut(); }
    let mut chan: *mut DmaChan = core::ptr::null_mut();
    list_for_each_entry!(chan, (*dev).channels, device_node, {
        if (*chan).chan_id == (*dma_spec).args[0] { return dma_get_slave_channel(chan); }
    });
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
