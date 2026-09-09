/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent: SWAP_CLUSTER_MAX is supplied by linux/swap.h. */

#[repr(C)]
pub union swap_iocb_union {
    pub iocb: core::mem::ManuallyDrop<kiocb>,
    pub bio: core::mem::ManuallyDrop<bio>,
}

#[repr(C)]
pub struct swap_iocb {
    pub union_: swap_iocb_union,
    pub bvecs: [bio_vec; SWAP_CLUSTER_MAX],
    pub nr_bvecs: core::ffi::c_int,
    pub len: core::ffi::c_int,
}

#[repr(C)]
pub struct swap_io_ctx {
    pub sio: *mut swap_iocb,
    pub sis: *mut swap_info_struct,
}

/*
 * SWAP_OPS_F_REQUIRE_NOFS:
 *	When set, all reclaim operations must operated as GFS_NOFS and not
 *	just GFP_NOIO, as GFP_NOIO allocations could recourse into the
 *	file system backing this swap file.
 */
pub const SWAP_OPS_F_REQUIRE_NOFS: core::ffi::c_uint = 1u32 << 0;

#[repr(C)]
pub struct swap_ops {
    pub flags: core::ffi::c_uint,
    pub can_merge: Option<
        unsafe extern "C" fn(
            folio: *mut folio,
            prev_folio: *mut folio,
            prev_folio_size: usize,
            rw: core::ffi::c_int,
        ) -> bool,
    >,
    pub submit_write: Option<unsafe extern "C" fn(ctx: *mut swap_io_ctx)>,
    pub submit_read: Option<unsafe extern "C" fn(ctx: *mut swap_io_ctx)>,
}

unsafe extern "C" {
    pub fn swap_fs_prepare_rw(
        ctx: *mut swap_io_ctx,
        rw: core::ffi::c_int,
        iter: *mut iov_iter,
    );
    pub fn swap_fs_can_merge(
        folio: *mut folio,
        prev_folio: *mut folio,
        prev_folio_size: usize,
        rw: core::ffi::c_int,
    ) -> bool;
    pub fn swap_fs_activate(
        sis: *mut swap_info_struct,
        ops: *const swap_ops,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
