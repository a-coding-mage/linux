// SPDX-License-Identifier: GPL-2.0
/*
 * Ported from IRIX to Linux by Kanoj Sarcar, 06/08/00.
 * Copyright 2000 - 2001 Silicon Graphics, Inc.
 * Copyright 2000 - 2001 Kanoj Sarcar (kanoj@sgi.com)
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

static mut KTEXT_REPMASK: nodemask_t = nodemask_t::default();

/*
 * XXX - This needs to be much smarter about where it puts copies of the
 * kernel.  For example, we should never put a copy on a headless node,
 * and we should respect the topology of the machine.
 */
pub unsafe extern "C" fn setup_replication_mask() {
    /* Set only the master cnode's bit.  The master cnode is always 0. */
    nodes_clear(&mut KTEXT_REPMASK);
    node_set(0, &mut KTEXT_REPMASK);

    // CONFIG_REPLICATE_KTEXT: kernel-text replication build condition.
    // CONFIG_MAPPED_KERNEL is required by the original source.
    #[cfg(feature = "CONFIG_REPLICATE_KTEXT")]
    {
        for nasid in for_each_online_node() {
            if nasid == 0 {
                continue;
            }
            /* Advertise that we have a copy of the kernel */
            node_set(nasid, &mut KTEXT_REPMASK);
        }
    }

    /* Set up a GDA pointer to the replication mask. */
    (*GDA).g_ktext_repmask = &mut KTEXT_REPMASK;
}

unsafe fn set_ktext_source(client_nasid: nasid_t, server_nasid: nasid_t) {
    let kvp: *mut kern_vars_t;

    kvp = &mut hub_data(client_nasid).kern_vars;

    KERN_VARS_ADDR(client_nasid) = kvp as usize as c_ulong;

    (*kvp).kv_magic = KV_MAGIC;
    (*kvp).kv_ro_nasid = server_nasid;
    (*kvp).kv_rw_nasid = master_nasid;
    (*kvp).kv_ro_baseaddr = NODE_CAC_BASE(server_nasid);
    (*kvp).kv_rw_baseaddr = NODE_CAC_BASE(master_nasid);
    printk(
        "REPLICATION: ON nasid %d, ktext from nasid %d, kdata from nasid %d\n",
        client_nasid,
        server_nasid,
        master_nasid,
    );
}

/* XXX - When the BTE works, we should use it instead of this. */
unsafe fn copy_kernel(dest_nasid: nasid_t) {
    let mut dest_kern_start: c_ulong;
    let source_start: c_ulong;
    let source_end: c_ulong;
    let kern_size: c_ulong;

    source_start = _stext as usize as c_ulong;
    source_end = _etext as usize as c_ulong;
    kern_size = source_end.wrapping_sub(source_start);

    dest_kern_start = CHANGE_ADDR_NASID(MAPPED_KERN_RO_TO_K0(source_start), dest_nasid);
    memcpy(
        dest_kern_start as *mut c_void,
        source_start as *const c_void,
        kern_size as usize,
    );
}

pub unsafe extern "C" fn replicate_kernel_text() {
    let mut server_nasid: nasid_t;

    server_nasid = master_nasid;

    /* Record where the master node should get its kernel text */
    set_ktext_source(master_nasid, master_nasid);

    for client_nasid in for_each_online_node() {
        if client_nasid == 0 {
            continue;
        }

        /* Check if this node should get a copy of the kernel */
        if node_isset(client_nasid, &KTEXT_REPMASK) {
            server_nasid = client_nasid;
            copy_kernel(server_nasid);
        }

        /* Record where this node should get its kernel text */
        set_ktext_source(client_nasid, server_nasid);
    }
}

/*
 * Return pfn of first free page of memory on a node. PROM may allocate
 * data structures on the first couple of pages of the first slot of each
 * node. If this is the case, getfirstfree(node) > getslotstart(node, 0).
 */
pub unsafe extern "C" fn node_getfirstfree(nasid: nasid_t) -> c_ulong {
    let mut loadbase: c_ulong = REP_BASE;
    let offset: c_ulong;

    // CONFIG_MAPPED_KERNEL condition from the original source.
    #[cfg(feature = "CONFIG_MAPPED_KERNEL")]
    {
        loadbase = loadbase.wrapping_add(16777216);
    }
    offset = PAGE_ALIGN((&_end as *const _ as usize as c_ulong)).wrapping_sub(loadbase);
    if (nasid == 0) || node_isset(nasid, &KTEXT_REPMASK) {
        TO_NODE(nasid, offset) >> PAGE_SHIFT
    } else {
        KDM_TO_PHYS(PAGE_ALIGN(SYMMON_STK_ADDR(nasid, 0))) >> PAGE_SHIFT
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
