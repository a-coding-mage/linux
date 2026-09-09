/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * Return codes for sequence based RTAS calls.
 * Not listed under PAPR+ v2.13 7.2.8: "Return Codes".
 * But defined in the specific section of each RTAS call.
 */
pub const RTAS_SEQ_COMPLETE: i32 = 0; // All data has been retrieved.
pub const RTAS_SEQ_MORE_DATA: i32 = 1; // More data is available
pub const RTAS_SEQ_START_OVER: i32 = -4; // Data changed, restart call sequence.

/*
 * Internal "blob" APIs for accumulating RTAS call results into
 * an immutable buffer to be attached to a file descriptor.
 */
#[repr(C)]
pub struct papr_rtas_blob {
    pub data: *const core::ffi::c_char,
    pub len: usize,
}

/**
 * struct papr_sequence - State for managing a sequence of RTAS calls.
 * @error:  Shall be zero as long as the sequence has not encountered an error,
 *          -ve errno otherwise. Use papr_rtas_sequence_set_err() to update.
 * @params: Parameter block to pass to rtas_*() calls.
 * @begin: Work area allocation and initialize the needed parameter
 *         values passed to RTAS call
 * @end: Free the allocated work area
 * @work: Obtain data with RTAS call and invoke it until the sequence is
 *        completed.
 *
 */
#[repr(C)]
pub struct papr_rtas_sequence {
    pub error: i32,
    pub params: *mut core::ffi::c_void,
    pub begin: Option<unsafe extern "C" fn(seq: *mut papr_rtas_sequence)>,
    pub end: Option<unsafe extern "C" fn(seq: *mut papr_rtas_sequence)>,
    pub work: Option<
        unsafe extern "C" fn(
            seq: *mut papr_rtas_sequence,
            len: *mut usize,
        ) -> *const core::ffi::c_char,
    >,
}

/* Types supplied by the surrounding kernel interfaces. */
#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn papr_rtas_blob_has_data(blob: *const papr_rtas_blob) -> bool;
    pub fn papr_rtas_blob_free(blob: *const papr_rtas_blob);
    pub fn papr_rtas_sequence_set_err(seq: *mut papr_rtas_sequence, err: i32) -> i32;
    pub fn papr_rtas_retrieve(seq: *mut papr_rtas_sequence) -> *const papr_rtas_blob;
    pub fn papr_rtas_setup_file_interface(
        seq: *mut papr_rtas_sequence,
        fops: *const file_operations,
        name: *mut core::ffi::c_char,
    ) -> isize;
    pub fn papr_rtas_sequence_should_stop(
        seq: *const papr_rtas_sequence,
        status: i32,
        init_state: bool,
    ) -> bool;
    pub fn papr_rtas_common_handle_read(
        file: *mut file,
        buf: *mut core::ffi::c_char,
        size: usize,
        off: *mut i64,
    ) -> isize;
    pub fn papr_rtas_common_handle_release(inode: *mut inode, file: *mut file) -> i32;
    pub fn papr_rtas_common_handle_seek(file: *mut file, off: i64, whence: i32) -> i64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
