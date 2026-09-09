// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/idr.h, linux/slab.h, linux/debugfs.h,
// linux/seq_file.h, linux/shrinker.h, linux/memcontrol.h, and internal.h.

extern "C" {
    static mut shrinker_mutex: mutex;
    static mut shrinker_list: list_head;
}

static mut shrinker_debugfs_ida: ida = DEFINE_IDA!();
static mut shrinker_debugfs_root: *mut dentry = core::ptr::null_mut();

unsafe fn shrinker_count_objects(
    shrinker: *mut shrinker,
    memcg: *mut mem_cgroup,
    count_per_node: *mut c_ulong,
) -> c_ulong {
    let mut nr: c_ulong;
    let mut total: c_ulong = 0;
    let mut nid: c_int;

    for_each_node!(nid) {
        if nid == 0 || ((*shrinker).flags & SHRINKER_NUMA_AWARE) != 0 {
            let mut sc = shrink_control {
                gfp_mask: GFP_KERNEL,
                nid,
                memcg,
                ..core::mem::zeroed()
            };

            nr = ((*shrinker).count_objects.unwrap())(shrinker, &mut sc);
            if nr == SHRINK_EMPTY {
                nr = 0;
            }
        } else {
            nr = 0;
        }

        *count_per_node.add(nid as usize) = nr;
        total = total.wrapping_add(nr);
    }

    total
}

unsafe fn shrinker_debugfs_count_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    let shrinker = (*m).private as *mut shrinker;
    let count_per_node = kcalloc(nr_node_ids, core::mem::size_of::<c_ulong>(), GFP_KERNEL)
        as *mut c_ulong;
    let mut memcg: *mut mem_cgroup;
    let mut total: c_ulong;
    let memcg_aware: bool;
    let mut ret: c_int = 0;
    let mut nid: c_int;

    if count_per_node.is_null() {
        return -ENOMEM;
    }

    memcg_aware = ((*shrinker).flags & SHRINKER_MEMCG_AWARE) != 0;

    memcg = mem_cgroup_iter(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    loop {
        if !memcg.is_null() && !mem_cgroup_online(memcg) {
            continue;
        }

        total = shrinker_count_objects(
            shrinker,
            if memcg_aware { memcg } else { core::ptr::null_mut() },
            count_per_node,
        );
        if total != 0 {
            seq_printf(m, "%llu", mem_cgroup_id(memcg));
            for_each_node!(nid) {
                seq_printf(m, " %lu", *count_per_node.add(nid as usize));
            }
            seq_putc(m, '\n' as c_int);
        }

        if !memcg_aware {
            mem_c_group_iter_break!(core::ptr::null_mut(), memcg);
            break;
        }

        if signal_pending!(current) {
            mem_c_group_iter_break!(core::ptr::null_mut(), memcg);
            ret = -EINTR;
            break;
        }

        memcg = mem_cgroup_iter(core::ptr::null_mut(), memcg, core::ptr::null_mut());
        if memcg.is_null() {
            break;
        }
    }

    kfree(count_per_node as *mut c_void);
    ret
}

DEFINE_SHOW_ATTRIBUTE!(shrinker_debugfs_count);

unsafe fn shrinker_debugfs_scan_open(inode: *mut inode, file: *mut file) -> c_int {
    (*file).private_data = (*inode).i_private;
    nonseekable_open(inode, file)
}

unsafe fn shrinker_debugfs_scan_write(
    file: *mut file,
    buf: *const c_char,
    size: usize,
    _pos: *mut loff_t,
) -> isize {
    let shrinker = (*file).private_data as *mut shrinker;
    let mut nr_to_scan: c_ulong = 0;
    let mut read_len: c_ulong;
    let mut id: u64;
    let mut sc: shrink_control = core::mem::zeroed();
    let mut memcg: *mut mem_cgroup = core::ptr::null_mut();
    let mut nid: c_int;
    let mut kbuf = [0 as c_char; 72];

    (*(&mut sc)).gfp_mask = GFP_KERNEL;
    read_len = core::cmp::min(size, kbuf.len() - 1) as c_ulong;
    if copy_from_user(kbuf.as_mut_ptr(), buf, read_len as usize) != 0 {
        return -EFAULT as isize;
    }
    kbuf[read_len as usize] = 0;

    if sscanf!(kbuf.as_ptr(), "%llu %d %lu", &mut id, &mut nid, &mut nr_to_scan) != 3 {
        return -EINVAL as isize;
    }
    if nid < 0 || nid >= nr_node_ids {
        return -EINVAL as isize;
    }
    if nr_to_scan == 0 {
        return size as isize;
    }

    if ((*shrinker).flags & SHRINKER_MEMCG_AWARE) != 0 {
        memcg = mem_cgroup_get_from_id(id);
        if memcg.is_null() {
            return -ENOENT as isize;
        }
        if !mem_cgroup_online(memcg) {
            mem_cgroup_put(memcg);
            return -ENOENT as isize;
        }
    } else if id != 0 {
        return -EINVAL as isize;
    }

    sc.nid = nid;
    sc.memcg = memcg;
    sc.nr_to_scan = nr_to_scan;
    sc.nr_scanned = nr_to_scan;
    ((*shrinker).scan_objects.unwrap())(shrinker, &mut sc);
    mem_cgroup_put(memcg);
    size as isize
}

static shrinker_debugfs_scan_fops: file_operations = file_operations {
    owner: THIS_MODULE,
    open: Some(shrinker_debugfs_scan_open),
    write: Some(shrinker_debugfs_scan_write),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn shrinker_debugfs_add(shrinker: *mut shrinker) -> c_int {
    let mut entry: *mut dentry;
    let mut buf = [0 as c_char; 128];
    let id: c_int;

    lockdep_assert_held!(&mut shrinker_mutex);
    if shrinker_debugfs_root.is_null() {
        return 0;
    }
    id = ida_alloc(&mut shrinker_debugfs_ida, GFP_KERNEL);
    if id < 0 {
        return id;
    }
    (*shrinker).debugfs_id = id;
    snprintf!(buf.as_mut_ptr(), buf.len(), "%s-%d", (*shrinker).name, id);
    entry = debugfs_create_dir(buf.as_ptr(), shrinker_debugfs_root);
    if IS_ERR!(entry) {
        ida_free(&mut shrinker_debugfs_ida, id);
        return PTR_ERR!(entry);
    }
    (*shrinker).debugfs_entry = entry;
    if (*shrinker).count_objects.is_some() {
        debugfs_create_file!("count", 0o440, entry, shrinker, &shrinker_debugfs_count_fops);
    }
    if (*shrinker).scan_objects.is_some() {
        debugfs_create_file!("scan", 0o220, entry, shrinker, &shrinker_debugfs_scan_fops);
    }
    0
}

unsafe fn shrinker_debugfs_rename(shrinker: *mut shrinker, fmt: *const c_char, mut args: ...) -> c_int {
    let old: *const c_char;
    let new: *const c_char;
    let mut ret: c_int = 0;
    va_start!(args, fmt);
    new = kvasprintf_const(GFP_KERNEL, fmt, args.as_va_list());
    va_end!(args);
    if new.is_null() {
        return -ENOMEM;
    }
    mutex_lock(&mut shrinker_mutex);
    old = (*shrinker).name;
    (*shrinker).name = new;
    ret = debugfs_change_name((*shrinker).debugfs_entry, "%s-%d", (*shrinker).name, (*shrinker).debugfs_id);
    if ret != 0 {
        (*shrinker).name = old;
        kfree_const(new);
    } else {
        kfree_const(old);
    }
    mutex_unlock(&mut shrinker_mutex);
    ret
}

EXPORT_SYMBOL!(shrinker_debugfs_rename);

unsafe fn shrinker_debugfs_detach(shrinker: *mut shrinker, debugfs_id: *mut c_int) -> *mut dentry {
    let entry = (*shrinker).debugfs_entry;
    lockdep_assert_held!(&mut shrinker_mutex);
    *debugfs_id = if !entry.is_null() { (*shrinker).debugfs_id } else { -1 };
    (*shrinker).debugfs_entry = core::ptr::null_mut();
    entry
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
