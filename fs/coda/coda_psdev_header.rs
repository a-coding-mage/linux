/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel/Coda translation.

pub const CODA_PSDEV_MAJOR: i32 = 67;
pub const MAX_CODADEVS: i32 = 5; // how many do we allow

pub struct kstatfs;

/* messages between coda filesystem in kernel and Venus */
#[repr(C)]
pub struct upc_req {
    pub uc_chain: list_head,
    pub uc_data: *mut core::ffi::c_char,
    pub uc_flags: u16,
    pub uc_inSize: u16,  /* Size is at most 5000 bytes */
    pub uc_outSize: u16,
    pub uc_opcode: u16,  /* copied from data to save lookup */
    pub uc_unique: i32,
    pub uc_sleep: wait_queue_head_t,   /* process' wait queue */
}

pub const CODA_REQ_ASYNC: i32 = 0x1;
pub const CODA_REQ_READ: i32 = 0x2;
pub const CODA_REQ_WRITE: i32 = 0x4;
pub const CODA_REQ_ABORT: i32 = 0x8;

/* communication pending/processing queues */
#[repr(C)]
pub struct venus_comm {
    pub vc_seq: core::ffi::c_ulong,
    pub vc_waitq: wait_queue_head_t, /* Venus wait queue */
    pub vc_pending: list_head,
    pub vc_processing: list_head,
    pub vc_inuse: i32,
    pub vc_sb: *mut super_block,
    pub vc_mutex: mutex,
}

#[inline]
pub unsafe fn coda_vcp(sb: *mut super_block) -> *mut venus_comm {
    (*sb).s_fs_info as *mut venus_comm
}

/* upcalls */
unsafe extern "C" {
    pub fn venus_rootfid(sb: *mut super_block, fidp: *mut CodaFid) -> i32;
    pub fn venus_getattr(
        sb: *mut super_block,
        fid: *mut CodaFid,
        attr: *mut coda_vattr,
    ) -> i32;
    pub fn venus_setattr(
        sb: *mut super_block,
        fid: *mut CodaFid,
        attr: *mut coda_vattr,
    ) -> i32;
    pub fn venus_lookup(
        sb: *mut super_block,
        fid: *mut CodaFid,
        name: *const core::ffi::c_char,
        length: i32,
        type_: *mut i32,
        resfid: *mut CodaFid,
    ) -> i32;
    pub fn venus_close(
        sb: *mut super_block,
        fid: *mut CodaFid,
        flags: i32,
        uid: kuid_t,
    ) -> i32;
    pub fn venus_open(
        sb: *mut super_block,
        fid: *mut CodaFid,
        flags: i32,
        f: *mut *mut file,
    ) -> i32;
    pub fn venus_mkdir(
        sb: *mut super_block,
        dirfid: *mut CodaFid,
        name: *const core::ffi::c_char,
        length: i32,
        newfid: *mut CodaFid,
        attrs: *mut coda_vattr,
    ) -> i32;
    pub fn venus_create(
        sb: *mut super_block,
        dirfid: *mut CodaFid,
        name: *const core::ffi::c_char,
        length: i32,
        excl: i32,
        mode: i32,
        newfid: *mut CodaFid,
        attrs: *mut coda_vattr,
    ) -> i32;
    pub fn venus_rmdir(
        sb: *mut super_block,
        dirfid: *mut CodaFid,
        name: *const core::ffi::c_char,
        length: i32,
    ) -> i32;
    pub fn venus_remove(
        sb: *mut super_block,
        dirfid: *mut CodaFid,
        name: *const core::ffi::c_char,
        length: i32,
    ) -> i32;
    pub fn venus_readlink(
        sb: *mut super_block,
        fid: *mut CodaFid,
        buffer: *mut core::ffi::c_char,
        length: *mut i32,
    ) -> i32;
    pub fn venus_rename(
        sb: *mut super_block,
        new_fid: *mut CodaFid,
        old_fid: *mut CodaFid,
        old_length: usize,
        new_length: usize,
        old_name: *const core::ffi::c_char,
        new_name: *const core::ffi::c_char,
    ) -> i32;
    pub fn venus_link(
        sb: *mut super_block,
        fid: *mut CodaFid,
        dirfid: *mut CodaFid,
        name: *const core::ffi::c_char,
        len: i32,
    ) -> i32;
    pub fn venus_symlink(
        sb: *mut super_block,
        fid: *mut CodaFid,
        name: *const core::ffi::c_char,
        len: i32,
        symname: *const core::ffi::c_char,
        symlen: i32,
    ) -> i32;
    pub fn venus_access(sb: *mut super_block, fid: *mut CodaFid, mask: i32) -> i32;
    pub fn venus_pioctl(
        sb: *mut super_block,
        fid: *mut CodaFid,
        cmd: u32,
        data: *mut PioctlData,
    ) -> i32;
    pub fn coda_downcall(
        vcp: *mut venus_comm,
        opcode: i32,
        out: *mut outputArgs,
        nbytes: usize,
    ) -> i32;
    pub fn venus_fsync(sb: *mut super_block, fid: *mut CodaFid) -> i32;
    pub fn venus_statfs(dentry: *mut dentry, sfs: *mut kstatfs) -> i32;
    pub fn venus_access_intent(
        sb: *mut super_block,
        fid: *mut CodaFid,
        access_intent_supported: *mut bool,
        count: usize,
        ppos: loff_t,
        type_: i32,
    ) -> i32;

    /* Statistics */
    // C declaration: extern struct venus_comm coda_comms[];
    pub static mut coda_comms: [venus_comm; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
