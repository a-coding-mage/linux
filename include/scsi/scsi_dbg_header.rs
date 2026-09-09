/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations from the C header.
#[repr(C)]
pub struct scsi_cmnd {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scsi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scsi_sense_hdr {
    _private: [u8; 0],
}

extern "C" {
    pub fn scsi_print_command(scmd: *mut scsi_cmnd);
    pub fn __scsi_format_command(
        buffer: *mut core::ffi::c_char,
        size: usize,
        cdb: *const u8,
        cdb_len: usize,
    ) -> usize;
    pub fn scsi_print_sense_hdr(
        sdev: *const scsi_device,
        name: *const core::ffi::c_char,
        sshdr: *const scsi_sense_hdr,
    );
    pub fn scsi_print_sense(scmd: *mut scsi_cmnd);
    pub fn __scsi_print_sense(
        sdev: *const scsi_device,
        name: *const core::ffi::c_char,
        sense_buffer: *const u8,
        sense_len: core::ffi::c_int,
    );
    pub fn scsi_print_result(
        scmd: *mut scsi_cmnd,
        name: *const core::ffi::c_char,
        disposition: core::ffi::c_int,
    );
}

// CONFIG_SCSI_CONSTANTS selects external implementations in the C build.
// The declarations below correspond to the CONFIG_SCSI_CONSTANTS branch.
#[cfg(CONFIG_SCSI_CONSTANTS)]
extern "C" {
    pub fn scsi_opcode_sa_name(
        cmd: core::ffi::c_int,
        sa: core::ffi::c_int,
        cdb_name: *mut *const core::ffi::c_char,
        sa_name: *mut *const core::ffi::c_char,
    ) -> bool;
    pub fn scsi_sense_key_string(key: u8) -> *const core::ffi::c_char;
    pub fn scsi_extd_sense_format(
        asc: u8,
        ascq: u8,
        fmt: *mut *const core::ffi::c_char,
    ) -> *const core::ffi::c_char;
    pub fn scsi_mlreturn_string(result: core::ffi::c_int) -> *const core::ffi::c_char;
    pub fn scsi_hostbyte_string(result: core::ffi::c_int) -> *const core::ffi::c_char;
}

#[cfg(not(CONFIG_SCSI_CONSTANTS))]
pub unsafe fn scsi_opcode_sa_name(
    cmd: core::ffi::c_int,
    _sa: core::ffi::c_int,
    cdb_name: *mut *const core::ffi::c_char,
    sa_name: *mut *const core::ffi::c_char,
) -> bool {
    *cdb_name = core::ptr::null();
    match cmd {
        VARIABLE_LENGTH_CMD
        | MAINTENANCE_IN
        | MAINTENANCE_OUT
        | PERSISTENT_RESERVE_IN
        | PERSISTENT_RESERVE_OUT
        | SERVICE_ACTION_IN_12
        | SERVICE_ACTION_OUT_12
        | SERVICE_ACTION_BIDIRECTIONAL
        | SERVICE_ACTION_IN_16
        | SERVICE_ACTION_OUT_16
        | EXTENDED_COPY
        | RECEIVE_COPY_RESULTS => {
            *sa_name = core::ptr::null();
            true
        }
        _ => false,
    }
}

#[cfg(not(CONFIG_SCSI_CONSTANTS))]
pub unsafe fn scsi_sense_key_string(_key: u8) -> *const core::ffi::c_char {
    core::ptr::null()
}

#[cfg(not(CONFIG_SCSI_CONSTANTS))]
pub unsafe fn scsi_extd_sense_format(
    _asc: u8,
    _ascq: u8,
    fmt: *mut *const core::ffi::c_char,
) -> *const core::ffi::c_char {
    *fmt = core::ptr::null();
    core::ptr::null()
}

#[cfg(not(CONFIG_SCSI_CONSTANTS))]
pub unsafe fn scsi_mlreturn_string(_result: core::ffi::c_int) -> *const core::ffi::c_char {
    core::ptr::null()
}

#[cfg(not(CONFIG_SCSI_CONSTANTS))]
pub unsafe fn scsi_hostbyte_string(_result: core::ffi::c_int) -> *const core::ffi::c_char {
    core::ptr::null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
