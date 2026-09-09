// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2021 Intel Corporation. All rights rsvd. */
// Linux kernel headers and local headers are supplied by the surrounding tree.

static mut idxd_debugfs_dir: *mut dentry = core::ptr::null_mut();

unsafe fn dump_event_entry(
    idxd: *mut idxd_device,
    s: *mut seq_file,
    index: u16,
    count: *mut i32,
    processed: bool,
) {
    let evl = (*idxd).evl;
    let mut entry: *mut dsa_evl_entry;
    let cr: *mut dsa_completion_record;
    let raw: *mut u64;
    let mut i: i32;
    let evl_strides = evl_ent_size(idxd) / core::mem::size_of::<u64>();

    entry = ((*evl).log as *mut dsa_evl_entry).add(index as usize);

    if !(*entry).e.desc_valid {
        return;
    }

    seq_printf(
        s,
        "Event Log entry %d (real index %u) processed: %u\n",
        *count,
        index,
        processed,
    );

    seq_printf(
        s,
        "desc valid %u wq idx valid %u\n"
            "batch %u fault rw %u priv %u error 0x%x\n"
            "wq idx %u op %#x pasid %u batch idx %u\n"
            "fault addr %#llx\n",
        (*entry).e.desc_valid,
        (*entry).e.wq_idx_valid,
        (*entry).e.batch,
        (*entry).e.fault_rw,
        (*entry).e.priv,
        (*entry).e.error,
        (*entry).e.wq_idx,
        (*entry).e.operation,
        (*entry).e.pasid,
        (*entry).e.batch_idx,
        (*entry).e.fault_addr,
    );

    cr = &mut (*entry).cr;
    seq_printf(
        s,
        "status %#x result %#x fault_info %#x bytes_completed %u\n"
            "fault addr %#llx inv flags %#x\n\n",
        (*cr).status,
        (*cr).result,
        (*cr).fault_info,
        (*cr).bytes_completed,
        (*cr).fault_addr,
        (*cr).invalid_flags,
    );

    raw = entry as *mut u64;
    i = 0;
    while (i < evl_strides as i32) {
        seq_printf(s, "entry[%d] = %#llx\n", i, *raw.add(i as usize));
        i += 1;
    }

    seq_puts(s, "\n");
    *count += 1;
}

unsafe extern "C" fn debugfs_evl_show(s: *mut seq_file, _d: *mut core::ffi::c_void) -> i32 {
    let idxd = (*s).private as *mut idxd_device;
    let evl = (*idxd).evl;
    let mut evl_status: evl_status_reg = core::mem::zeroed();
    let (mut h, mut t, mut evl_size, mut i): (u16, u16, u16, u16);
    let mut count: i32 = 0;
    let mut processed = true;

    if evl.is_null() || (*evl).log.is_null() {
        return 0;
    }

    mutex_lock(&mut (*evl).lock);

    evl_status.bits = ioread64((*idxd).reg_base.add(IDXD_EVLSTATUS_OFFSET));
    t = evl_status.tail;
    h = evl_status.head;
    evl_size = (*evl).size;

    seq_printf(
        s,
        "Event Log head %u tail %u interrupt pending %u\n\n",
        evl_status.head,
        evl_status.tail,
        evl_status.int_pending,
    );

    i = t;
    loop {
        i = (i + 1) % evl_size;
        if i == t {
            break;
        }

        if processed && i == h {
            processed = false;
        }
        dump_event_entry(idxd, s, i, &mut count, processed);
    }

    mutex_unlock(&mut (*evl).lock);
    0
}

// DEFINE_SHOW_ATTRIBUTE(debugfs_evl);
extern "C" {
    static debugfs_evl_fops: file_operations;
}

pub unsafe extern "C" fn idxd_device_init_debugfs(idxd: *mut idxd_device) -> i32 {
    if is_err_or_null(idxd_debugfs_dir) {
        return 0;
    }

    (*idxd).dbgfs_dir = debugfs_create_dir(dev_name(idxd_confdev(idxd)), idxd_debugfs_dir);
    if is_err((*idxd).dbgfs_dir) {
        return ptr_err((*idxd).dbgfs_dir);
    }

    if !(*idxd).evl.is_null() {
        (*idxd).dbgfs_evl_file = debugfs_create_file(
            "event_log",
            0o400,
            (*idxd).dbgfs_dir,
            idxd as *mut core::ffi::c_void,
            &debugfs_evl_fops,
        );
        if is_err((*idxd).dbgfs_evl_file) {
            debugfs_remove_recursive((*idxd).dbgfs_dir);
            (*idxd).dbgfs_dir = core::ptr::null_mut();
            return ptr_err((*idxd).dbgfs_evl_file);
        }
    }

    0
}

pub unsafe extern "C" fn idxd_device_remove_debugfs(idxd: *mut idxd_device) {
    debugfs_remove_recursive((*idxd).dbgfs_dir);
}

pub unsafe extern "C" fn idxd_init_debugfs() -> i32 {
    if !debugfs_initialized() {
        return 0;
    }

    idxd_debugfs_dir = debugfs_create_dir(KBUILD_MODNAME, core::ptr::null_mut());
    if is_err(idxd_debugfs_dir) {
        return ptr_err(idxd_debugfs_dir);
    }
    0
}

pub unsafe extern "C" fn idxd_remove_debugfs() {
    debugfs_remove_recursive(idxd_debugfs_dir);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
