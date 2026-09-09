// SPDX-License-Identifier: GPL-2.0-only
/*
 * debugfs code for HSR & PRP
 * Copyright (C) 2019 Texas Instruments Incorporated
 *
 * Author(s):
 *	Murali Karicheri <m-karicheri2@ti.com>
 */
// Dependencies are supplied by the surrounding kernel/Rust translation.

static mut HSR_DEBUGFS_ROOT_DIR: *mut dentry = core::ptr::null_mut();

/* hsr_node_table_show - Formats and prints node_table entries */
unsafe fn hsr_node_table_show(sfp: *mut seq_file, _data: *mut core::ffi::c_void) -> i32 {
    let priv_: *mut hsr_priv = (*sfp).private as *mut hsr_priv;
    let mut node: *mut hsr_node;

    seq_printf(
        sfp,
        c"Node Table entries for (%s) device\n",
        if (*priv_).prot_version == PRP_V1 { c"PRP" } else { c"HSR" },
    );
    seq_puts(sfp, c"MAC-Address-A,    MAC-Address-B,    time_in[A], ");
    seq_puts(sfp, c"time_in[B], Address-B port, ");
    if (*priv_).prot_version == PRP_V1 {
        seq_puts(sfp, c"SAN-A, SAN-B, DAN-P\n");
    } else {
        seq_puts(sfp, c"DAN-H\n");
    }

    rcu_read_lock();
    list_for_each_entry_rcu!(node, &(*priv_).node_db, mac_list) {
        /* skip self node */
        if hsr_addr_is_self(priv_, (*node).macaddress_A.as_ptr()) {
            continue;
        }
        seq_printf(sfp, c"%pM ", (*node).macaddress_A.as_ptr());
        seq_printf(sfp, c"%pM ", (*node).macaddress_B.as_ptr());
        seq_printf(sfp, c"%10lx, ", (*node).time_in[HSR_PT_SLAVE_A]);
        seq_printf(sfp, c"%10lx, ", (*node).time_in[HSR_PT_SLAVE_B]);
        seq_printf(sfp, c"%14x, ", (*node).addr_B_port);

        if (*priv_).prot_version == PRP_V1 {
            seq_printf(
                sfp,
                c"%5x, %5x, %5x\n",
                (*node).san_a,
                (*node).san_b,
                ((*node).san_a == 0 && (*node).san_b == 0) as i32,
            );
        } else {
            seq_printf(sfp, c"%5x\n", 1);
        }
    }
    rcu_read_unlock();
    0
}

// Equivalent of DEFINE_SHOW_ATTRIBUTE(hsr_node_table).
static hsr_node_table_fops: file_operations = show_attribute!(hsr_node_table);

unsafe fn hsr_debugfs_rename(dev: *mut net_device) {
    let priv_: *mut hsr_priv = netdev_priv(dev);
    let err: i32;

    err = debugfs_change_name((*priv_).node_tbl_root, c"%s", (*dev).name);
    if err != 0 {
        netdev_warn(dev, c"failed to rename\n");
    }
}

/* hsr_debugfs_init - create hsr node_table file for dumping
 * the node table
 *
 * Description:
 * When debugfs is configured this routine sets up the node_table file per
 * hsr device for dumping the node_table entries
 */
unsafe fn hsr_debugfs_init(priv_: *mut hsr_priv, hsr_dev: *mut net_device) {
    let mut de: *mut dentry = core::ptr::null_mut();

    de = debugfs_create_dir((*hsr_dev).name, HSR_DEBUGFS_ROOT_DIR);
    if IS_ERR(de) {
        pr_err(c"Cannot create hsr debugfs directory\n");
        return;
    }

    (*priv_).node_tbl_root = de;

    de = debugfs_create_file(
        c"node_table",
        S_IFREG | 0o444,
        (*priv_).node_tbl_root,
        priv_ as *mut core::ffi::c_void,
        &hsr_node_table_fops,
    );
    if IS_ERR(de) {
        pr_err(c"Cannot create hsr node_table file\n");
        debugfs_remove((*priv_).node_tbl_root);
        (*priv_).node_tbl_root = core::ptr::null_mut();
        return;
    }
}

/* hsr_debugfs_term - Tear down debugfs intrastructure
 *
 * Description:
 * When Debugfs is configured this routine removes debugfs file system
 * elements that are specific to hsr
 */
unsafe fn hsr_debugfs_term(priv_: *mut hsr_priv) {
    debugfs_remove_recursive((*priv_).node_tbl_root);
    (*priv_).node_tbl_root = core::ptr::null_mut();
}

unsafe fn hsr_debugfs_create_root() {
    HSR_DEBUGFS_ROOT_DIR = debugfs_create_dir(c"hsr", core::ptr::null_mut());
    if IS_ERR(HSR_DEBUGFS_ROOT_DIR) {
        pr_err(c"Cannot create hsr debugfs root directory\n");
        HSR_DEBUGFS_ROOT_DIR = core::ptr::null_mut();
    }
}

unsafe fn hsr_debugfs_remove_root() {
    /* debugfs_remove() internally checks NULL and ERROR */
    debugfs_remove(HSR_DEBUGFS_ROOT_DIR);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
