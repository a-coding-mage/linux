/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

pub const MAX_COMMAND_SIZE: usize = 16;

#[repr(C)]
pub struct scsi_data_buffer {
    pub table: sg_table,
    pub length: c_uint,
}

#[repr(C)]
pub struct scsi_pointer {
    pub ptr: *mut c_char,
    pub this_residual: c_int,
    pub buffer: *mut scatterlist,
    pub buffers_residual: c_int,
    pub dma_handle: dma_addr_t,
    pub Status: c_int,
    pub Message: c_int,
    pub have_data_in: c_int,
    pub sent_command: c_int,
    pub phase: c_int,
}

pub const SCMD_TAGGED: c_int = 1 << 0;
pub const SCMD_INITIALIZED: c_int = 1 << 1;
pub const SCMD_LAST: c_int = 1 << 2;
pub const SCMD_FORCE_EH_SUCCESS: c_int = 1 << 3;
pub const SCMD_FAIL_IF_RECOVERING: c_int = 1 << 4;
pub const SCMD_PRESERVED_FLAGS: c_int = SCMD_INITIALIZED | SCMD_FAIL_IF_RECOVERING;

pub const SCMD_STATE_COMPLETE: c_ulong = 0;
pub const SCMD_STATE_INFLIGHT: c_ulong = 1;

#[repr(u8)]
pub enum scsi_cmnd_submitter {
    SUBMITTED_BY_BLOCK_LAYER = 0,
    SUBMITTED_BY_SCSI_ERROR_HANDLER = 1,
    SUBMITTED_BY_SCSI_RESET_IOCTL = 2,
}

#[repr(C)]
pub struct scsi_cmnd {
    pub device: *mut scsi_device,
    pub eh_entry: list_head,
    pub abort_work: delayed_work,
    pub rcu: rcu_head,
    pub eh_eflags: c_int,
    pub budget_token: c_int,
    pub jiffies_at_alloc: c_ulong,
    pub retries: c_int,
    pub allowed: c_int,
    pub prot_op: c_uchar,
    pub prot_type: c_uchar,
    pub prot_flags: c_uchar,
    pub submitter: scsi_cmnd_submitter,
    pub cmd_len: c_ushort,
    pub sc_data_direction: dma_data_direction,
    pub cmnd: [c_uchar; 32],
    pub sdb: scsi_data_buffer,
    pub prot_sdb: *mut scsi_data_buffer,
    pub underflow: c_uint,
    pub transfersize: c_uint,
    pub resid_len: c_uint,
    pub sense_len: c_uint,
    pub sense_buffer: *mut c_uchar,
    pub flags: c_int,
    pub state: c_ulong,
    pub extra_len: c_uint,
    pub host_scribble: *mut c_uchar,
    pub result: c_int,
}

#[inline]
pub unsafe fn scsi_cmd_to_rq(scmd: *mut scsi_cmnd) -> *mut request {
    blk_mq_rq_from_pdu(scmd.cast())
}

#[inline]
pub unsafe fn scsi_cmd_priv(cmd: *mut scsi_cmnd) -> *mut c_void {
    cmd.add(1).cast()
}

extern "C" {
    pub fn scsi_done(cmd: *mut scsi_cmnd);
    pub fn scsi_done_direct(cmd: *mut scsi_cmnd);
    pub fn scsi_finish_command(cmd: *mut scsi_cmnd);
    pub fn scsi_kmap_atomic_sg(sg: *mut scatterlist, sg_count: c_int,
                               offset: *mut size_t, len: *mut size_t) -> *mut c_void;
    pub fn scsi_kunmap_atomic_sg(virt: *mut c_void);
    pub fn scsi_alloc_sgtables(cmd: *mut scsi_cmnd) -> blk_status_t;
    pub fn scsi_free_sgtables(cmd: *mut scsi_cmnd);
    pub fn scsi_dma_map(cmd: *mut scsi_cmnd) -> c_int;
    pub fn scsi_dma_unmap(cmd: *mut scsi_cmnd);
    pub fn scsi_build_sense(scmd: *mut scsi_cmnd, desc: c_int, key: u8, asc: u8, ascq: u8);
    pub fn scsi_alloc_request(q: *mut request_queue, opf: blk_opf_t,
                              flags: blk_mq_req_flags_t) -> *mut request;
}

#[inline]
pub unsafe fn scsi_sg_count(cmd: *mut scsi_cmnd) -> c_uint { (*cmd).sdb.table.nents }
#[inline]
pub unsafe fn scsi_sglist(cmd: *mut scsi_cmnd) -> *mut scatterlist { (*cmd).sdb.table.sgl }
#[inline]
pub unsafe fn scsi_bufflen(cmd: *mut scsi_cmnd) -> c_uint { (*cmd).sdb.length }
#[inline]
pub unsafe fn scsi_set_resid(cmd: *mut scsi_cmnd, resid: c_uint) { (*cmd).resid_len = resid; }
#[inline]
pub unsafe fn scsi_get_resid(cmd: *mut scsi_cmnd) -> c_uint { (*cmd).resid_len }

#[macro_export]
macro_rules! scsi_for_each_sg { ($cmd:expr, $sg:expr, $nseg:expr, $__i:expr) => { for_each_sg!(scsi_sglist($cmd), $sg, $nseg, $__i) }; }

#[inline]
pub unsafe fn scsi_sg_copy_from_buffer(cmd: *mut scsi_cmnd, buf: *const c_void, buflen: c_int) -> c_int {
    sg_copy_from_buffer(scsi_sglist(cmd), scsi_sg_count(cmd), buf, buflen)
}
#[inline]
pub unsafe fn scsi_sg_copy_to_buffer(cmd: *mut scsi_cmnd, buf: *mut c_void, buflen: c_int) -> c_int {
    sg_copy_to_buffer(scsi_sglist(cmd), scsi_sg_count(cmd), buf, buflen)
}
#[inline]
pub unsafe fn scsi_get_sector(scmd: *mut scsi_cmnd) -> sector_t { blk_rq_pos(scsi_cmd_to_rq(scmd)) }
#[inline]
pub unsafe fn scsi_get_lba(scmd: *mut scsi_cmnd) -> sector_t {
    let shift = ilog2((*(*scmd).device).sector_size) - SECTOR_SHIFT;
    blk_rq_pos(scsi_cmd_to_rq(scmd)) >> shift
}
#[inline]
pub unsafe fn scsi_logical_block_count(scmd: *mut scsi_cmnd) -> c_uint {
    let shift = ilog2((*(*scmd).device).sector_size);
    blk_rq_bytes(scsi_cmd_to_rq(scmd)) >> shift
}

#[repr(C)]
pub enum scsi_prot_operations { SCSI_PROT_NORMAL = 0, SCSI_PROT_READ_INSERT, SCSI_PROT_WRITE_STRIP, SCSI_PROT_READ_STRIP, SCSI_PROT_WRITE_INSERT, SCSI_PROT_READ_PASS, SCSI_PROT_WRITE_PASS }
#[inline] pub unsafe fn scsi_set_prot_op(scmd: *mut scsi_cmnd, op: c_uchar) { (*scmd).prot_op = op; }
#[inline] pub unsafe fn scsi_get_prot_op(scmd: *mut scsi_cmnd) -> c_uchar { (*scmd).prot_op }

pub const SCSI_PROT_TRANSFER_PI: c_int = 1 << 0;
pub const SCSI_PROT_GUARD_CHECK: c_int = 1 << 1;
pub const SCSI_PROT_REF_CHECK: c_int = 1 << 2;
pub const SCSI_PROT_REF_INCREMENT: c_int = 1 << 3;
pub const SCSI_PROT_IP_CHECKSUM: c_int = 1 << 4;
#[repr(C)] pub enum scsi_prot_target_type { SCSI_PROT_DIF_TYPE0 = 0, SCSI_PROT_DIF_TYPE1, SCSI_PROT_DIF_TYPE2, SCSI_PROT_DIF_TYPE3 }
#[inline] pub unsafe fn scsi_set_prot_type(scmd: *mut scsi_cmnd, ty: c_uchar) { (*scmd).prot_type = ty; }
#[inline] pub unsafe fn scsi_get_prot_type(scmd: *mut scsi_cmnd) -> c_uchar { (*scmd).prot_type }
#[inline] pub unsafe fn scsi_prot_ref_tag(scmd: *mut scsi_cmnd) -> u32 { t10_pi_ref_tag(blk_mq_rq_from_pdu(scmd.cast())) }
#[inline] pub unsafe fn scsi_prot_interval(scmd: *mut scsi_cmnd) -> c_uint { (*(*scmd).device).sector_size }
#[inline] pub unsafe fn scsi_prot_sg_count(cmd: *mut scsi_cmnd) -> c_uint { if !(*cmd).prot_sdb.is_null() { (*(*cmd).prot_sdb).table.nents } else { 0 } }
#[inline] pub unsafe fn scsi_prot_sglist(cmd: *mut scsi_cmnd) -> *mut scatterlist { if !(*cmd).prot_sdb.is_null() { (*(*cmd).prot_sdb).table.sgl } else { core::ptr::null_mut() } }
#[inline] pub unsafe fn scsi_prot(cmd: *mut scsi_cmnd) -> *mut scsi_data_buffer { (*cmd).prot_sdb }
#[macro_export] macro_rules! scsi_for_each_prot_sg { ($cmd:expr, $sg:expr, $nseg:expr, $__i:expr) => { for_each_sg!(scsi_prot_sglist($cmd), $sg, $nseg, $__i) }; }

#[inline] pub unsafe fn set_status_byte(cmd: *mut scsi_cmnd, status: c_char) { (*cmd).result = ((*cmd).result & 0xffffff00) | status as c_int; }
#[inline] pub unsafe fn get_status_byte(cmd: *mut scsi_cmnd) -> u8 { ((*cmd).result & 0xff) as u8 }
#[inline] pub unsafe fn set_host_byte(cmd: *mut scsi_cmnd, status: c_char) { (*cmd).result = ((*cmd).result & 0xff00ffff) | ((status as c_int) << 16); }
#[inline] pub unsafe fn get_host_byte(cmd: *mut scsi_cmnd) -> u8 { (((*cmd).result >> 16) & 0xff) as u8 }

#[inline]
pub unsafe fn scsi_msg_to_host_byte(cmd: *mut scsi_cmnd, msg: u8) {
    match msg { COMMAND_COMPLETE => {}, ABORT_TASK_SET => set_host_byte(cmd, DID_ABORT), TARGET_RESET => set_host_byte(cmd, DID_RESET), _ => set_host_byte(cmd, DID_ERROR) }
}
#[inline]
pub unsafe fn scsi_transfer_length(scmd: *mut scsi_cmnd) -> c_uint {
    let mut xfer_len = (*scmd).sdb.length;
    let prot_interval = scsi_prot_interval(scmd);
    if (*scmd).prot_flags & SCSI_PROT_TRANSFER_PI as c_uchar != 0 { xfer_len += (xfer_len >> ilog2(prot_interval)) * 8; }
    xfer_len
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
