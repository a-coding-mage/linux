/* SPDX-License-Identifier: GPL-2.0 */

// Translated from scsi_eh.h. Declarations supplied by included headers remain
// external dependencies of this translation.

use core::ffi::{c_int, c_uint};

pub struct scsi_device;
pub struct Scsi_Host;

unsafe extern "C" {
    pub fn scsi_eh_finish_cmd(scmd: *mut scsi_cmnd, done_q: *mut list_head);
    pub fn scsi_eh_flush_done_q(done_q: *mut list_head);
    pub fn scsi_report_bus_reset(host: *mut Scsi_Host, channel: c_int);
    pub fn scsi_report_device_reset(
        host: *mut Scsi_Host,
        channel: c_int,
        target: c_int,
    );
    pub fn scsi_block_when_processing_errors(sdev: *mut scsi_device) -> bool;
    pub fn scsi_command_normalize_sense(
        cmd: *const scsi_cmnd,
        sshdr: *mut scsi_sense_hdr,
    ) -> bool;
    pub fn scsi_check_sense(scmd: *mut scsi_cmnd) -> scsi_disposition;
    pub fn scsi_get_sense_info_fld(
        sense_buffer: *const u8,
        sb_len: c_int,
        info_out: *mut u64,
    ) -> bool;
    pub fn scsi_ioctl_reset(sdev: *mut scsi_device, flags: *mut c_int) -> c_int;
    pub fn scsi_eh_prep_cmnd(
        scmd: *mut scsi_cmnd,
        ses: *mut scsi_eh_save,
        cmnd: *mut u8,
        cmnd_size: c_int,
        sense_bytes: c_uint,
    );
    pub fn scsi_eh_restore_cmnd(scmd: *mut scsi_cmnd, ses: *mut scsi_eh_save);
}

#[inline]
pub unsafe fn scsi_sense_is_deferred(sshdr: *const scsi_sense_hdr) -> bool {
    let response_code = (*sshdr).response_code;
    (response_code >= 0x70) && ((response_code & 1) != 0)
}

#[repr(C)]
pub struct scsi_eh_save {
    /* saved state */
    pub result: c_int,
    pub resid_len: c_uint,
    pub eh_eflags: c_int,
    pub data_direction: dma_data_direction,
    pub underflow: c_uint,
    pub cmd_len: u8,
    pub prot_op: u8,
    pub cmnd: [u8; 32],
    pub sdb: scsi_data_buffer,
    pub sense_sgl: scatterlist,

    /* struct request fields */
    // CONFIG_BLK_INLINE_ENCRYPTION controls these C fields at build time.
    #[cfg(CONFIG_BLK_INLINE_ENCRYPTION)]
    pub rq_crypt_ctx: *mut bio_crypt_ctx,
    #[cfg(CONFIG_BLK_INLINE_ENCRYPTION)]
    pub rq_crypt_keyslot: *mut blk_crypto_keyslot,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
