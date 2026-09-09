/* SPDX-License-Identifier: GPL-2.0 */

// C header guard `_NFS_FS_I` omitted; Rust items are defined once per module.

// External dependency supplied by another translated header.
#[repr(C)]
pub struct nlm_lockowner {
    _private: [u8; 0],
}

/*
 * NFS lock info
 */
#[repr(C)]
pub struct nfs_lock_info {
    pub state: u32,
    pub owner: *mut nlm_lockowner,
    // External dependency supplied by another translated header.
    pub list: list_head,
}

#[repr(C)]
pub struct nfs4_lock_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nfs4_lock_info {
    pub owner: *mut nfs4_lock_state,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
