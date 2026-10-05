// SPDX-License-Identifier: GPL-2.0-only
unsafe fn unusable_free_index(order: c_uint, info: &ContigPageInfo) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        if info.free_pages == 0 {
            return 1000;
        }
        let unusable = info
            .free_pages
            .wrapping_sub(info.free_blocks_suitable.wrapping_shl(order));
        rust_vmstat_div_u64((unusable as u64).wrapping_mul(1000), info.free_pages as u32) as c_int
    }
}
unsafe fn unusable_show_print(m: *mut seq_file, p: *mut pglist_data, z: *mut zone) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        seq_printf(
            m,
            kernel::str::as_char_ptr_in_const_context(c"Node %d, zone %8s "),
            (*p).node_id,
            (*z).name,
        );
        for order in 0..RUST_VMSTAT_NR_PAGE_ORDERS {
            let index = unusable_free_index(order, &fill_contig_page_info(z, order));
            seq_printf(
                m,
                kernel::str::as_char_ptr_in_const_context(c"%d.%03d "),
                index / 1000,
                index % 1000,
            );
        }
        seq_putc(m, b'\n' as c_char);
    }
}
unsafe extern "C" fn unusable_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let p = arg.cast::<pglist_data>();
        if !rust_vmstat_node_has_memory((*p).node_id) {
            return 0;
        }
        walk_zones_in_node(m, p, true, false, unusable_show_print);
        0
    }
}
static UNUSABLE_SOPS: seq_operations = seq_operations {
    start: Some(frag_start),
    next: Some(frag_next),
    stop: Some(frag_stop),
    show: Some(unusable_show),
};
unsafe fn extfrag_show_print(m: *mut seq_file, p: *mut pglist_data, z: *mut zone) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        seq_printf(
            m,
            kernel::str::as_char_ptr_in_const_context(c"Node %d, zone %8s "),
            (*p).node_id,
            (*z).name,
        );
        for order in 0..RUST_VMSTAT_NR_PAGE_ORDERS {
            let index = __fragmentation_index(order, &fill_contig_page_info(z, order));
            seq_printf(
                m,
                kernel::str::as_char_ptr_in_const_context(c"%2d.%03d "),
                index / 1000,
                index % 1000,
            );
        }
        seq_putc(m, b'\n' as c_char);
    }
}
unsafe extern "C" fn extfrag_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        walk_zones_in_node(m, arg.cast(), true, false, extfrag_show_print);
        0
    }
}
static EXTFRAG_SOPS: seq_operations = seq_operations {
    start: Some(frag_start),
    next: Some(frag_next),
    stop: Some(frag_stop),
    show: Some(extfrag_show),
};
// DEFINE_SEQ_ATTRIBUTE's open bodies are translated here, including propagation
// of inode->i_private only after successful seq_open.
unsafe extern "C" fn unusable_open(inode: *mut inode, file: *mut file) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let ret = seq_open(file, addr_of!(UNUSABLE_SOPS));
        if ret == 0 && !(*inode).i_private.is_null() {
            (*(*file).private_data.cast::<seq_file>()).private = (*inode).i_private;
        }
        ret
    }
}
unsafe extern "C" fn extfrag_open(inode: *mut inode, file: *mut file) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let ret = seq_open(file, addr_of!(EXTFRAG_SOPS));
        if ret == 0 && !(*inode).i_private.is_null() {
            (*(*file).private_data.cast::<seq_file>()).private = (*inode).i_private;
        }
        ret
    }
}
#[repr(transparent)]
struct VmstatFileOps(file_operations);
// SAFETY: this immutable table contains only permanent built-in callbacks.
unsafe impl Sync for VmstatFileOps {}
static UNUSABLE_FOPS: VmstatFileOps = VmstatFileOps({
    let mut f: file_operations = unsafe { zeroed() };
    // vmstat.o is built-in obj-y and native helper compilation uses -UMODULE;
    // THIS_MODULE is therefore the real native NULL value.
    f.owner = null_mut();
    f.open = Some(unusable_open);
    f.read = Some(seq_read);
    f.llseek = Some(seq_lseek);
    f.release = Some(seq_release);
    f
});
static EXTFRAG_FOPS: VmstatFileOps = VmstatFileOps({
    let mut f: file_operations = unsafe { zeroed() };
    f.owner = null_mut();
    f.open = Some(extfrag_open);
    f.read = Some(seq_read);
    f.llseek = Some(seq_lseek);
    f.release = Some(seq_release);
    f
});
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RUST_VMSTAT_INIT_COLD, cold)]
unsafe extern "C" fn rust_vmstat_extfrag_debug_init() -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let root = debugfs_create_dir(
            kernel::str::as_char_ptr_in_const_context(c"extfrag"),
            null_mut(),
        );
        rust_vmstat_debugfs_create_file(
            kernel::str::as_char_ptr_in_const_context(c"unusable_index"),
            0o444,
            root,
            null_mut(),
            addr_of!(UNUSABLE_FOPS.0),
        );
        rust_vmstat_debugfs_create_file(
            kernel::str::as_char_ptr_in_const_context(c"extfrag_index"),
            0o444,
            root,
            null_mut(),
            addr_of!(EXTFRAG_FOPS.0),
        );
        0
    }
}
