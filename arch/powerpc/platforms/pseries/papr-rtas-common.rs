// SPDX-License-Identifier: GPL-2.0-only

// Dependency declarations from the C includes are supplied by the surrounding
// kernel translation unit, including papr_rtas_blob, papr_rtas_sequence, and
// the file-operation interfaces.

/*
 * Sequence based RTAS HCALL has to issue multiple times to retrieve
 * complete data from the hypervisor. For some of these RTAS calls,
 * the OS should not interleave calls with different input until the
 * sequence is completed. So data is collected for these calls during
 * ioctl handle and export to user space with read() handle.
 * This file provides common functions needed for such sequence based
 * RTAS calls Ex: ibm,get-vpd and ibm,get-indices.
 */

pub unsafe fn papr_rtas_blob_has_data(blob: *const papr_rtas_blob) -> bool {
    !(*blob).data.is_null() && (*blob).len != 0
}

pub unsafe fn papr_rtas_blob_free(blob: *const papr_rtas_blob) {
    if !blob.is_null() {
        kvfree((*blob).data as *mut core::ffi::c_void);
        kfree(blob as *mut core::ffi::c_void);
    }
}

/* Append data to a papr_rtas_blob. May sleep. */
unsafe fn papr_rtas_blob_extend(
    blob: *mut papr_rtas_blob,
    data: *const core::ffi::c_char,
    len: usize,
) -> i32 {
    let new_len = (*blob).len.wrapping_add(len);
    let old_len = (*blob).len;
    let old_ptr = (*blob).data;
    let new_ptr = kvrealloc(old_ptr as *mut core::ffi::c_void, new_len, GFP_KERNEL_ACCOUNT)
        as *mut core::ffi::c_char;

    if new_ptr.is_null() {
        return -ENOMEM;
    }

    memcpy(new_ptr.add(old_len) as *mut core::ffi::c_void,
           data as *const core::ffi::c_void, len);
    (*blob).data = new_ptr;
    (*blob).len = new_len;
    0
}

/* Construct a new papr_rtas_blob. */
unsafe fn papr_rtas_blob_generate(
    seq: *mut papr_rtas_sequence,
) -> *const papr_rtas_blob {
    let blob = kzalloc(core::mem::size_of::<papr_rtas_blob>(), GFP_KERNEL_ACCOUNT)
        as *mut papr_rtas_blob;
    if blob.is_null() {
        return core::ptr::null();
    }

    if (*seq).work.is_none() {
        return ERR_PTR(-EINVAL) as *const papr_rtas_blob;
    }

    let mut len: usize = 0;
    let mut err: i32 = 0;
    let mut buf: *const core::ffi::c_char;
    while err == 0 {
        buf = ((*seq).work.unwrap())(seq, &mut len);
        if buf.is_null() {
            break;
        }
        err = papr_rtas_blob_extend(blob, buf, len);
    }

    if err != 0 || !papr_rtas_blob_has_data(blob) {
        papr_rtas_blob_free(blob);
        return core::ptr::null();
    }
    blob
}

pub unsafe fn papr_rtas_sequence_set_err(seq: *mut papr_rtas_sequence, err: i32) -> i32 {
    /* Preserve the first error recorded. */
    if (*seq).error == 0 {
        (*seq).error = err;
    }
    (*seq).error
}

/* Run a single retrieval sequence. */
unsafe fn papr_rtas_run_sequence(seq: *mut papr_rtas_sequence) -> *const papr_rtas_blob {
    if let Some(begin) = (*seq).begin {
        begin(seq);
    }

    let blob = papr_rtas_blob_generate(seq);
    if blob.is_null() {
        papr_rtas_sequence_set_err(seq, -ENOMEM);
    }

    if let Some(end) = (*seq).end {
        end(seq);
    }

    if (*seq).error != 0 {
        papr_rtas_blob_free(blob);
        return ERR_PTR((*seq).error) as *const papr_rtas_blob;
    }
    blob
}

/* Return the data blob exposed to user space. */
pub unsafe fn papr_rtas_retrieve(seq: *mut papr_rtas_sequence) -> *const papr_rtas_blob {
    let mut blob: *const papr_rtas_blob;
    loop {
        blob = papr_rtas_run_sequence(seq);
        if !IS_ERR(blob) || PTR_ERR(blob) != -EAGAIN {
            break;
        }
        cond_resched();
        if fatal_signal_pending(current) {
            break;
        }
    }
    blob
}

pub unsafe fn papr_rtas_setup_file_interface(
    seq: *mut papr_rtas_sequence,
    fops: *const file_operations,
    name: *mut core::ffi::c_char,
) -> isize {
    let blob = papr_rtas_retrieve(seq);
    if IS_ERR(blob) {
        return PTR_ERR(blob);
    }

    let fd = FD_ADD(O_RDONLY | O_CLOEXEC,
        anon_inode_getfile_fmode(name, fops, blob as *mut core::ffi::c_void,
                                  O_RDONLY, FMODE_LSEEK | FMODE_PREAD));
    if fd < 0 {
        papr_rtas_blob_free(blob);
    }
    fd
}

/* Determine whether the RTAS retrieval sequence should stop. */
pub unsafe fn papr_rtas_sequence_should_stop(
    seq: *const papr_rtas_sequence,
    status: i32,
    init_state: bool,
) -> bool {
    if (*seq).error != 0 {
        return true;
    }

    match status {
        RTAS_SEQ_COMPLETE => !init_state,
        RTAS_SEQ_MORE_DATA => false,
        _ => true,
    }
}

pub unsafe fn papr_rtas_common_handle_read(
    file: *mut file,
    buf: *mut core::ffi::c_char,
    size: usize,
    off: *mut loff_t,
) -> isize {
    let blob = (*file).private_data as *const papr_rtas_blob;
    if !papr_rtas_blob_has_data(blob) {
        pr_err_once!("handle without data\n");
        return -EIO;
    }
    simple_read_from_buffer(buf, size, off, (*blob).data as *const core::ffi::c_void, (*blob).len)
}

pub unsafe fn papr_rtas_common_handle_release(
    _inode: *mut inode,
    file: *mut file,
) -> i32 {
    let blob = (*file).private_data as *const papr_rtas_blob;
    papr_rtas_blob_free(blob);
    0
}

pub unsafe fn papr_rtas_common_handle_seek(
    file: *mut file,
    off: loff_t,
    whence: i32,
) -> loff_t {
    let blob = (*file).private_data as *const papr_rtas_blob;
    fixed_size_llseek(file, off, whence, (*blob).len as loff_t)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
