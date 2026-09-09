// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Authors:
 * (C) 2015 Pengutronix, Alexander Aring <aar@pengutronix.de>
 * Copyright (c)  2015 Nordic Semiconductor. All Rights Reserved.
 */

// Dependencies supplied by the surrounding kernel/Rust translation.
const LOWPAN_DEBUGFS_CTX_PFX_NUM_ARGS: usize = 8;

static mut lowpan_debugfs: *mut dentry = core::ptr::null_mut();

unsafe fn lowpan_ctx_flag_active_set(data: *mut core::ffi::c_void, val: u64) -> i32 {
    let ctx = data as *mut lowpan_iphc_ctx;

    if val != 0 && val != 1 {
        return -EINVAL;
    }

    if val != 0 {
        set_bit(LOWPAN_IPHC_CTX_FLAG_ACTIVE, &mut (*ctx).flags);
    } else {
        clear_bit(LOWPAN_IPHC_CTX_FLAG_ACTIVE, &mut (*ctx).flags);
    }

    0
}

unsafe fn lowpan_ctx_flag_active_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 {
    *val = lowpan_iphc_ctx_is_active(data);
    0
}

// DEFINE_DEBUGFS_ATTRIBUTE(lowpan_ctx_flag_active_fops,
//     lowpan_ctx_flag_active_get, lowpan_ctx_flag_active_set, "%llu\n");

unsafe fn lowpan_ctx_flag_c_set(data: *mut core::ffi::c_void, val: u64) -> i32 {
    let ctx = data as *mut lowpan_iphc_ctx;

    if val != 0 && val != 1 {
        return -EINVAL;
    }

    if val != 0 {
        set_bit(LOWPAN_IPHC_CTX_FLAG_COMPRESSION, &mut (*ctx).flags);
    } else {
        clear_bit(LOWPAN_IPHC_CTX_FLAG_COMPRESSION, &mut (*ctx).flags);
    }

    0
}

unsafe fn lowpan_ctx_flag_c_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 {
    *val = lowpan_iphc_ctx_is_compression(data);
    0
}

// DEFINE_DEBUGFS_ATTRIBUTE(lowpan_ctx_flag_c_fops, lowpan_ctx_flag_c_get,
//     lowpan_ctx_flag_c_set, "%llu\n");

unsafe fn lowpan_ctx_plen_set(data: *mut core::ffi::c_void, val: u64) -> i32 {
    let ctx = data as *mut lowpan_iphc_ctx;
    let t = container_of_ctx_table((*ctx).id, ctx);

    if val > 128 {
        return -EINVAL;
    }

    spin_lock_bh(&mut (*t).lock);
    (*ctx).plen = val;
    spin_unlock_bh(&mut (*t).lock);
    0
}

unsafe fn lowpan_ctx_plen_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 {
    let ctx = data as *mut lowpan_iphc_ctx;
    let t = container_of_ctx_table((*ctx).id, ctx);

    spin_lock_bh(&mut (*t).lock);
    *val = (*ctx).plen;
    spin_unlock_bh(&mut (*t).lock);
    0
}

// DEFINE_DEBUGFS_ATTRIBUTE(lowpan_ctx_plen_fops, lowpan_ctx_plen_get,
//     lowpan_ctx_plen_set, "%llu\n");

unsafe fn lowpan_ctx_pfx_show(file: *mut seq_file, _offset: *mut core::ffi::c_void) -> i32 {
    let ctx = (*file).private as *mut lowpan_iphc_ctx;
    let t = container_of_ctx_table((*ctx).id, ctx);

    spin_lock_bh(&mut (*t).lock);
    seq_printf(file, "%04x:%04x:%04x:%04x:%04x:%04x:%04x:%04x\n",
        be16_to_cpu((*ctx).pfx.s6_addr16[0]), be16_to_cpu((*ctx).pfx.s6_addr16[1]),
        be16_to_cpu((*ctx).pfx.s6_addr16[2]), be16_to_cpu((*ctx).pfx.s6_addr16[3]),
        be16_to_cpu((*ctx).pfx.s6_addr16[4]), be16_to_cpu((*ctx).pfx.s6_addr16[5]),
        be16_to_cpu((*ctx).pfx.s6_addr16[6]), be16_to_cpu((*ctx).pfx.s6_addr16[7]));
    spin_unlock_bh(&mut (*t).lock);
    0
}

unsafe fn lowpan_ctx_pfx_open(inode: *mut inode, file: *mut file) -> i32 {
    single_open(file, lowpan_ctx_pfx_show, (*inode).i_private)
}

unsafe fn lowpan_ctx_pfx_write(fp: *mut file, user_buf: *const u8, count: usize,
                               _ppos: *mut loff_t) -> isize {
    let mut buf = [0u8; 128];
    let file = (*fp).private_data as *mut seq_file;
    let ctx = (*file).private as *mut lowpan_iphc_ctx;
    let t = container_of_ctx_table((*ctx).id, ctx);
    let mut status = count as isize;
    let mut addr = [0u32; 8];

    if copy_from_user(buf.as_mut_ptr(), user_buf, core::cmp::min(buf.len() - 1, count)) != 0 {
        return -EFAULT as isize;
    }

    let n = sscanf_ipv6_words(&buf, &mut addr);
    if n != LOWPAN_DEBUGFS_CTX_PFX_NUM_ARGS {
        status = -EINVAL as isize;
        return status;
    }

    spin_lock_bh(&mut (*t).lock);
    for i in 0..8 {
        (*ctx).pfx.s6_addr16[i] = cpu_to_be16((addr[i] & 0xffff) as u16);
    }
    spin_unlock_bh(&mut (*t).lock);
    status
}

// const lowpan_ctx_pfx_fops: struct file_operations = { open, read, write,
//     llseek, release };

unsafe fn lowpan_dev_debugfs_ctx_init(dev: *mut net_device, ctx: *mut dentry, id: u8) {
    let ldev = lowpan_dev(dev);
    let mut buf = [0u8; 32];

    if WARN_ON_ONCE(id >= LOWPAN_IPHC_CTX_TABLE_SIZE) {
        return;
    }
    sprintf_decimal(buf.as_mut_ptr(), id);
    let root = debugfs_create_dir(buf.as_ptr(), ctx);

    debugfs_create_file(c"active", 0o644, root, &mut (*ldev).ctx.table[id as usize], &lowpan_ctx_flag_active_fops);
    debugfs_create_file(c"compression", 0o644, root, &mut (*ldev).ctx.table[id as usize], &lowpan_ctx_flag_c_fops);
    debugfs_create_file(c"prefix", 0o644, root, &mut (*ldev).ctx.table[id as usize], &lowpan_ctx_pfx_fops);
    debugfs_create_file(c"prefix_len", 0o644, root, &mut (*ldev).ctx.table[id as usize], &lowpan_ctx_plen_fops);
}

unsafe fn lowpan_context_show(file: *mut seq_file, _offset: *mut core::ffi::c_void) -> i32 {
    let t = (*file).private as *mut lowpan_iphc_ctx_table;
    seq_printf(file, "%3s|%-43s|%c\n", c"cid", c"prefix", b'C');
    seq_puts(file, c"-------------------------------------------------\n");

    spin_lock_bh(&mut (*t).lock);
    for i in 0..LOWPAN_IPHC_CTX_TABLE_SIZE {
        if !lowpan_iphc_ctx_is_active(&mut (*t).table[i]) { continue; }
        seq_printf(file, "%3d|%39pI6c/%-3d|%d\n", (*t).table[i].id,
                   &(*t).table[i].pfx, (*t).table[i].plen,
                   lowpan_iphc_ctx_is_compression(&mut (*t).table[i]));
    }
    spin_unlock_bh(&mut (*t).lock);
    0
}

// DEFINE_SHOW_ATTRIBUTE(lowpan_context);

unsafe fn lowpan_short_addr_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 {
    let wdev = data as *mut wpan_dev;
    rtnl_lock();
    *val = le16_to_cpu((*wdev).short_addr) as u64;
    rtnl_unlock();
    0
}

// DEFINE_DEBUGFS_ATTRIBUTE(lowpan_short_addr_fops, lowpan_short_addr_get,
//     NULL, "0x%04llx\n");

unsafe fn lowpan_dev_debugfs_802154_init(dev: *const net_device, ldev: *mut lowpan_dev) {
    if !lowpan_is_ll(dev, LOWPAN_LLTYPE_IEEE802154) { return; }
    let root = debugfs_create_dir(c"ieee802154", (*ldev).iface_debugfs);
    debugfs_create_file(c"short_addr", 0o444, root,
                        lowpan_802154_dev(dev).wdev.ieee802154_ptr,
                        &lowpan_short_addr_fops);
}

pub unsafe fn lowpan_dev_debugfs_init(dev: *mut net_device) {
    let ldev = lowpan_dev(dev);
    (*ldev).iface_debugfs = debugfs_create_dir((*dev).name.as_ptr(), lowpan_debugfs);
    let contexts = debugfs_create_dir(c"contexts", (*ldev).iface_debugfs);
    debugfs_create_file(c"show", 0o644, contexts, &mut (*lowpan_dev(dev)).ctx,
                        &lowpan_context_fops);
    for i in 0..LOWPAN_IPHC_CTX_TABLE_SIZE { lowpan_dev_debugfs_ctx_init(dev, contexts, i as u8); }
    lowpan_dev_debugfs_802154_init(dev, ldev);
}

pub unsafe fn lowpan_dev_debugfs_exit(dev: *mut net_device) {
    debugfs_remove_recursive((*lowpan_dev(dev)).iface_debugfs);
}

pub unsafe fn lowpan_debugfs_init() {
    lowpan_debugfs = debugfs_create_dir(c"6lowpan", core::ptr::null_mut());
}

pub unsafe fn lowpan_debugfs_exit() {
    debugfs_remove_recursive(lowpan_debugfs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
