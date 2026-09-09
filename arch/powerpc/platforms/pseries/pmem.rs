// SPDX-License-Identifier: GPL-2.0

/*
 * Handles hot and cold plug of persistent memory regions on pseries.
 */

// C includes and build-provided kernel declarations are supplied by the
// surrounding translation unit.

static mut pmem_node: *mut device_node = core::ptr::null_mut();

unsafe fn pmem_drc_add_node(drc_index: u32) -> isize {
    let mut dn: *mut device_node;
    let rc: i32;

    pr_debug!("Attempting to add pmem node, drc index: %x\n", drc_index);

    rc = dlpar_acquire_drc(drc_index);
    if rc != 0 {
        pr_err!(
            "Failed to acquire DRC, rc: %d, drc index: %x\n",
            rc,
            drc_index
        );
        return -EINVAL as isize;
    }

    dn = dlpar_configure_connector(cpu_to_be32(drc_index), pmem_node);
    if dn.is_null() {
        pr_err!("configure-connector failed for drc %x\n", drc_index);
        dlpar_release_drc(drc_index);
        return -EINVAL as isize;
    }

    /* NB: The of reconfig notifier creates platform device from the node */
    rc = dlpar_attach_node(dn, pmem_node);
    if rc != 0 {
        pr_err!(
            "Failed to attach node %pOF, rc: %d, drc index: %x\n",
            dn,
            rc,
            drc_index
        );

        if dlpar_release_drc(drc_index) != 0 {
            dlpar_free_cc_nodes(dn);
        }

        return rc as isize;
    }

    pr_info!("Successfully added %pOF, drc index: %x\n", dn, drc_index);

    0
}

unsafe fn pmem_drc_remove_node(drc_index: u32) -> isize {
    let mut dn: *mut device_node = core::ptr::null_mut();
    let mut index: u32 = 0;
    let mut child = of_get_child_by_index(pmem_node, 0);

    while !child.is_null() {
        dn = child;
        if of_property_read_u32(dn, c"ibm,my-drc-index".as_ptr(), &mut index) == 0
            && index == drc_index
        {
            break;
        }
        child = of_get_next_child(pmem_node, child);
    }

    if dn.is_null() || index != drc_index {
        pr_err!("Attempting to remove unused DRC index %x\n", drc_index);
        return -ENODEV as isize;
    }

    pr_debug!("Attempting to remove %pOF, drc index: %x\n", dn, drc_index);

    /* * NB: tears down the ibm,pmemory device as a side-effect */
    let mut rc = dlpar_detach_node(dn);
    if rc != 0 {
        return rc as isize;
    }

    rc = dlpar_release_drc(drc_index);
    if rc != 0 {
        pr_err!(
            "Failed to release drc (%x) for CPU %pOFn, rc: %d\n",
            drc_index,
            dn,
            rc
        );
        dlpar_attach_node(dn, pmem_node);
        return rc as isize;
    }

    pr_info!("Successfully removed PMEM with drc index: %x\n", drc_index);

    0
}

pub unsafe fn dlpar_hp_pmem(hp_elog: *mut pseries_hp_errorlog) -> i32 {
    let drc_index: u32;
    let rc: isize;

    /* slim chance, but we might get a hotplug event while booting */
    if pmem_node.is_null() {
        pmem_node = of_find_node_by_type(core::ptr::null_mut(), c"ibm,persistent-memory".as_ptr());
    }
    if pmem_node.is_null() {
        pr_err!("Hotplug event for a pmem device, but none exists\n");
        return -ENODEV;
    }

    if (*hp_elog).id_type != PSERIES_HP_ELOG_ID_DRC_INDEX {
        pr_err!("Unsupported hotplug event type %d\n", (*hp_elog).id_type);
        return -EINVAL;
    }

    drc_index = be32_to_cpu((*hp_elog)._drc_u.drc_index);

    lock_device_hotplug();

    if (*hp_elog).action == PSERIES_HP_ELOG_ACTION_ADD {
        rc = pmem_drc_add_node(drc_index);
    } else if (*hp_elog).action == PSERIES_HP_ELOG_ACTION_REMOVE {
        rc = pmem_drc_remove_node(drc_index);
    } else {
        pr_err!("Unsupported hotplug action (%d)\n", (*hp_elog).action);
        rc = -EINVAL as isize;
    }

    unlock_device_hotplug();
    rc as i32
}

static drc_pmem_match: [of_device_id; 2] = [
    of_device_id { type_: c"ibm,persistent-memory".as_ptr() },
    of_device_id { ..Default::default() },
];

unsafe fn pseries_pmem_init() -> i32 {
    /*
     * Only supported on POWER8 and above.
     */
    if !cpu_has_feature(CPU_FTR_ARCH_207S) {
        return 0;
    }

    pmem_node = of_find_node_by_type(core::ptr::null_mut(), c"ibm,persistent-memory".as_ptr());
    if pmem_node.is_null() {
        return 0;
    }

    /*
     * The generic OF bus probe/populate handles creating platform devices
     * from the child (ibm,pmemory) nodes. The generic code registers an of
     * reconfig notifier to handle the hot-add/remove cases too.
     */
    of_platform_bus_probe(pmem_node, drc_pmem_match.as_ptr(), core::ptr::null_mut());

    0
}

machine_arch_initcall!(pseries, pseries_pmem_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
