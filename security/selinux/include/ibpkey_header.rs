/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * pkey table
 *
 * SELinux must keep a mapping of pkeys to labels/SIDs.  This
 * mapping is maintained as part of the normal policy but a fast cache is
 * needed to reduce the lookup overhead.
 */

/*
 * (c) Mellanox Technologies, 2016
 */

// C dependencies: <linux/types.h> and "flask.h".

// CONFIG_SECURITY_INFINIBAND selects the external implementation.
#[cfg(CONFIG_SECURITY_INFINIBAND)]
unsafe extern "C" {
    pub fn sel_ib_pkey_flush();
    pub fn sel_ib_pkey_sid(subnet_prefix: u64, pkey: u16, sid: *mut u32) -> i32;
}

#[cfg(not(CONFIG_SECURITY_INFINIBAND))]
pub unsafe fn sel_ib_pkey_flush() {
    return;
}

#[cfg(not(CONFIG_SECURITY_INFINIBAND))]
pub unsafe fn sel_ib_pkey_sid(_subnet_prefix: u64, _pkey: u16, sid: *mut u32) -> i32 {
    // SECINITSID_UNLABELED is provided by "flask.h".
    unsafe {
        *sid = SECINITSID_UNLABELED;
    }
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
