/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Functions used by both the SCSI initiator code and the SCSI target code.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

/* From the standard INQUIRY data description in SPC-6. */
pub const INQUIRY_VENDOR_OFFSET: usize = 8;
pub const INQUIRY_VENDOR_LEN: usize = 8;
pub const INQUIRY_MODEL_OFFSET: usize = 16;
pub const INQUIRY_MODEL_LEN: usize = 16;
pub const INQUIRY_REVISION_OFFSET: usize = 32;
pub const INQUIRY_REVISION_LEN: usize = 4;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum scsi_pr_type {
    SCSI_PR_WRITE_EXCLUSIVE = 0x01,
    SCSI_PR_EXCLUSIVE_ACCESS = 0x03,
    SCSI_PR_WRITE_EXCLUSIVE_REG_ONLY = 0x05,
    SCSI_PR_EXCLUSIVE_ACCESS_REG_ONLY = 0x06,
    SCSI_PR_WRITE_EXCLUSIVE_ALL_REGS = 0x07,
    SCSI_PR_EXCLUSIVE_ACCESS_ALL_REGS = 0x08,
}

unsafe extern "C" {
    pub fn block_pr_type_to_scsi(type_: pr_type) -> scsi_pr_type;
    pub fn scsi_pr_type_to_block(type_: scsi_pr_type) -> pr_type;
}

#[inline]
pub unsafe fn scsi_varlen_cdb_length(hdr: *const core::ffi::c_void) -> u32 {
    (*(hdr as *const scsi_varlen_cdb_hdr)).additional_cdb_length as u32 + 8
}

unsafe extern "C" {
    pub static scsi_command_size_tbl: [u8; 8];
}

#[macro_export]
macro_rules! COMMAND_SIZE {
    ($opcode:expr) => {
        $crate::scsi_command_size_tbl[((($opcode) >> 5) & 7) as usize]
    };
}

#[inline]
pub unsafe fn scsi_command_size(cmnd: *const u8) -> u32 {
    if *cmnd == VARIABLE_LENGTH_CMD {
        scsi_varlen_cdb_length(cmnd as *const core::ffi::c_void)
    } else {
        COMMAND_SIZE!(*cmnd) as u32
    }
}

#[inline]
pub unsafe fn scsi_command_control(cmnd: *const u8) -> u8 {
    if *cmnd == VARIABLE_LENGTH_CMD {
        *cmnd.add(1)
    } else {
        *cmnd.add((COMMAND_SIZE!(*cmnd) as usize).wrapping_sub(1))
    }
}

/* Returns a human-readable name for the device */
unsafe extern "C" {
    pub fn scsi_device_type(type_: u32) -> *const core::ffi::c_char;
    pub fn int_to_scsilun(value: u64, lun: *mut scsi_lun);
    pub fn scsilun_to_int(lun: *mut scsi_lun) -> u64;
}

/*
 * This is a slightly modified SCSI sense "descriptor" format header.
 * The addition is to allow the 0x70 and 0x71 response codes. The idea
 * is to place the salient data from either "fixed" or "descriptor" sense
 * format into one structure to ease application processing.
 *
 * The original sense buffer should be kept around for those cases in
 * which more information is required (e.g. the LBA of a MEDIUM ERROR).
 */
#[repr(C)]
pub struct scsi_sense_hdr { /* See SPC-3 section 4.5 */
    pub response_code: u8, /* permit: 0x0, 0x70, 0x71, 0x72, 0x73 */
    pub sense_key: u8,
    pub asc: u8,
    pub ascq: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub additional_length: u8, /* always 0 for fixed sense format */
}

#[inline]
pub unsafe fn scsi_sense_valid(sshdr: *const scsi_sense_hdr) -> bool {
    if sshdr.is_null() {
        return false;
    }
    ((*sshdr).response_code & 0x70) == 0x70
}

unsafe extern "C" {
    pub fn scsi_normalize_sense(
        sense_buffer: *const u8,
        sb_len: i32,
        sshdr: *mut scsi_sense_hdr,
    ) -> bool;
    pub fn scsi_build_sense_buffer(desc: i32, buf: *mut u8, key: u8, asc: u8, ascq: u8);
    pub fn scsi_set_sense_information(buf: *mut u8, buf_len: i32, info: u64) -> i32;
    pub fn scsi_set_sense_field_pointer(
        buf: *mut u8,
        buf_len: i32,
        fp: u16,
        bp: u8,
        cd: bool,
    ) -> i32;
    pub fn scsi_sense_desc_find(
        sense_buffer: *const u8,
        sb_len: i32,
        desc_type: i32,
    ) -> *const u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
