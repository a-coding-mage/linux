// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2024 Intel Corporation. All rights reserved. */

// Dependencies supplied by the surrounding kernel/CXL translation.

unsafe fn cxl_handle_mce(
    nb: *mut notifier_block,
    val: c_ulong,
    data: *mut c_void,
) -> c_int {
    let cxlr: *mut cxl_region = container_of!(nb, cxl_region, mce_notifier);
    let p: *mut cxl_region_params = unsafe { &mut (*cxlr).params };
    let mce: *mut mce = data as *mut mce;
    let spa: u64;
    let spa_alias: u64;
    let pfn: c_ulong;

    if mce.is_null() || unsafe { !mce_usable_address(mce) } {
        return NOTIFY_DONE;
    }

    spa = unsafe { (*mce).addr } & MCI_ADDR_PHYSADDR;

    if unsafe { !cxl_resource_contains_addr((*p).res, spa) } {
        return NOTIFY_DONE;
    }

    if spa >= unsafe { (*(*p).res).start } + unsafe { (*p).cache_size } {
        spa_alias = spa - unsafe { (*p).cache_size };
    } else {
        spa_alias = spa + unsafe { (*p).cache_size };
    }

    pfn = spa_alias >> PAGE_SHIFT;
    if unsafe { !pfn_valid(pfn) } {
        return NOTIFY_DONE;
    }

    /*
     * Take down the aliased memory page. The original memory page flagged
     * by the MCE will be taken cared of by the standard MCE handler.
     */
    unsafe {
        dev_emerg(
            &mut (*cxlr).dev,
            c"Offlining aliased SPA address0: %#llx\n".as_ptr(),
            spa_alias,
        );
        if memory_failure(pfn, 0) == 0 {
            set_mce_nospec(pfn);
        }
    }

    NOTIFY_OK
}

unsafe fn cxl_unregister_mce_notifier(mce_notifier: *mut c_void) {
    unsafe {
        mce_unregister_decode_chain(mce_notifier);
    }
}

pub unsafe fn devm_cxl_register_mce_notifier(
    dev: *mut device,
    mce_notifier: *mut notifier_block,
) -> c_int {
    unsafe {
        (*mce_notifier).notifier_call = Some(cxl_handle_mce);
        (*mce_notifier).priority = MCE_PRIO_UC;
        mce_register_decode_chain(mce_notifier);

        devm_add_action_or_reset(
            dev,
            Some(cxl_unregister_mce_notifier),
            mce_notifier as *mut c_void,
        )
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
