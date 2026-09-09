// SPDX-License-Identifier: GPL-2.0-only
/*
 * kexec_handover_debugfs.c - kexec handover debugfs interfaces
 * Copyright (C) 2023 Alexander Graf <graf@amazon.com>
 * Copyright (C) 2025 Microsoft Corporation, Mike Rapoport <rppt@kernel.org>
 * Copyright (C) 2025 Google LLC, Changyuan Lyu <changyuanl@google.com>
 * Copyright (C) 2025 Google LLC, Pasha Tatashin <pasha.tatashin@soleen.com>
 */

// C includes and build-provided declarations are supplied by the surrounding kernel translation.

static mut debugfs_root: *mut dentry = core::ptr::null_mut();

#[repr(C)]
struct fdt_debugfs {
    list: list_head,
    wrapper: debugfs_blob_wrapper,
    file: *mut dentry,
}

unsafe fn __kho_debugfs_blob_add(
    list: *mut list_head,
    dir: *mut dentry,
    name: *const core::ffi::c_char,
    blob: *const core::ffi::c_void,
    size: usize,
) -> i32 {
    let f = kmalloc_obj_fdt_debugfs();
    if f.is_null() {
        return -ENOMEM;
    }

    (*f).wrapper.data = blob as *mut core::ffi::c_void;
    (*f).wrapper.size = size;

    let file = debugfs_create_blob(name, 0o400, dir, &mut (*f).wrapper);
    if is_err(file) {
        kfree(f as *mut core::ffi::c_void);
        return ptr_err(file);
    }

    (*f).file = file;
    list_add(&mut (*f).list, list);
    0
}

unsafe fn kho_debugfs_blob_add(
    dbg: *mut kho_debugfs,
    name: *const core::ffi::c_char,
    blob: *const core::ffi::c_void,
    size: usize,
    root: bool,
) -> i32 {
    let dir = if root { (*dbg).dir } else { (*dbg).sub_fdt_dir };
    __kho_debugfs_blob_add(&mut (*dbg).fdt_list, dir, name, blob, size)
}

unsafe fn kho_debugfs_blob_remove(dbg: *mut kho_debugfs, blob: *mut core::ffi::c_void) {
    let mut ff: *mut fdt_debugfs;
    list_for_each_entry!(ff, &mut (*dbg).fdt_list, list, {
        if (*ff).wrapper.data == blob {
            debugfs_remove((*ff).file);
            list_del(&mut (*ff).list);
            kfree(ff as *mut core::ffi::c_void);
            break;
        }
    });
}

unsafe extern "C" fn scratch_phys_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    for i in 0..kho_scratch_cnt {
        seq_printf(m, c"0x%llx\n".as_ptr(), (*kho_scratch.add(i as usize)).addr);
    }
    0
}

unsafe extern "C" fn scratch_len_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    for i in 0..kho_scratch_cnt {
        seq_printf(m, c"0x%llx\n".as_ptr(), (*kho_scratch.add(i as usize)).size);
    }
    0
}

// Equivalent of DEFINE_SHOW_ATTRIBUTE(scratch_phys) and DEFINE_SHOW_ATTRIBUTE(scratch_len).
extern "C" {
    static scratch_phys_fops: file_operations;
    static scratch_len_fops: file_operations;
}

unsafe extern "C" fn kho_in_debugfs_init(dbg: *mut kho_debugfs, fdt: *const core::ffi::c_void) {
    let mut dir: *mut dentry;
    let mut sub_fdt_dir: *mut dentry;
    let mut err: i32;
    let mut child: i32;

    init_list_head(&mut (*dbg).fdt_list);
    dir = debugfs_create_dir(c"in".as_ptr(), debugfs_root);
    if is_err(dir) { err = ptr_err(dir); goto!(err_out); }

    sub_fdt_dir = debugfs_create_dir(c"sub_fdts".as_ptr(), dir);
    if is_err(sub_fdt_dir) { err = ptr_err(sub_fdt_dir); goto!(err_rmdir); }

    err = __kho_debugfs_blob_add(&mut (*dbg).fdt_list, dir, c"fdt".as_ptr(), fdt, fdt_totalsize(fdt));
    if err != 0 { goto!(err_rmdir); }

    fdt_for_each_subnode!(child, fdt, 0, {
        let mut len: i32 = 0;
        let name = fdt_get_name(fdt, child, core::ptr::null_mut());
        let blob_phys = fdt_getprop(fdt, child, KHO_SUB_TREE_PROP_NAME, &mut len) as *const u64;
        if blob_phys.is_null() { continue; }
        if len as usize != core::mem::size_of::<u64>() {
            pr_warn!(c"node %s prop %s has invalid length: %d\n", name, KHO_SUB_TREE_PROP_NAME, len);
            continue;
        }
        let blob_size = fdt_getprop(fdt, child, KHO_SUB_TREE_SIZE_PROP_NAME, &mut len) as *const u64;
        if blob_size.is_null() || len as usize != core::mem::size_of::<u64>() {
            pr_warn!(c"node %s missing or invalid %s property\n", name, KHO_SUB_TREE_SIZE_PROP_NAME);
            continue;
        }
        let blob = phys_to_virt(*blob_phys);
        err = __kho_debugfs_blob_add(&mut (*dbg).fdt_list, sub_fdt_dir, name, blob, *blob_size as usize);
        if err != 0 {
            pr_warn!(c"failed to add blob %s to debugfs: %pe\n", name, err_ptr(err));
            continue;
        }
    });

    (*dbg).dir = dir;
    (*dbg).sub_fdt_dir = sub_fdt_dir;
    return;

    err_rmdir:;
    debugfs_remove_recursive(dir);
    err_out:;
    // Failure to create /sys/kernel/debug/kho/in does not prevent reviving state from KHO.
    if err != 0 { pr_err!(c"failed exposing handover FDT in debugfs: %pe\n", err_ptr(err)); }
}

unsafe extern "C" fn kho_out_debugfs_init(dbg: *mut kho_debugfs) -> i32 {
    let dir = debugfs_create_dir(c"out".as_ptr(), debugfs_root);
    if is_err(dir) { return -ENOMEM; }
    let sub_fdt_dir = debugfs_create_dir(c"sub_fdts".as_ptr(), dir);
    if is_err(sub_fdt_dir) { goto!(err_rmdir); }
    let mut f = debugfs_create_file(c"scratch_phys".as_ptr(), 0o400, dir, core::ptr::null_mut(), &scratch_phys_fops);
    if is_err(f) { goto!(err_rmdir); }
    f = debugfs_create_file(c"scratch_len".as_ptr(), 0o400, dir, core::ptr::null_mut(), &scratch_len_fops);
    if is_err(f) { goto!(err_rmdir); }
    (*dbg).dir = dir;
    (*dbg).sub_fdt_dir = sub_fdt_dir;
    return 0;
    err_rmdir:;
    debugfs_remove_recursive(dir);
    -ENOENT
}

unsafe extern "C" fn kho_debugfs_init() -> i32 {
    debugfs_root = debugfs_create_dir(c"kho".as_ptr(), core::ptr::null_mut());
    if is_err(debugfs_root) { return -ENOENT; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
