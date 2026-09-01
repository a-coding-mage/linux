/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shared generic netlink helpers for the acct selftests.
 */

// Depends on C headers <stdbool.h> and <linux/netlink.h> for `struct nlattr`.

#[allow(non_upper_case_globals)]
pub const NLA_ALIGNTO: i32 = 4;

#[inline]
pub const fn NLA_ALIGN(len: i32) -> i32 {
    (len + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
}

#[allow(non_upper_case_globals)]
pub const NLA_HDRLEN: i32 = NLA_ALIGN(core::mem::size_of::<nlattr>() as i32);

/* Fail an individual test case instead of hanging the whole binary. */
#[allow(non_upper_case_globals)]
pub const ACCT_RCV_TIMEOUT_SEC: i32 = 2;

#[inline]
pub unsafe fn nla_data(na: *const nlattr) -> *mut core::ffi::c_void {
    (na as *mut u8).add(NLA_HDRLEN as usize) as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn nla_ok(na: *const nlattr, remaining: i32) -> bool {
    remaining >= core::mem::size_of::<nlattr>() as i32
        && (*na).nla_len as usize >= core::mem::size_of::<nlattr>()
        && ((*na).nla_len as i32) <= remaining
}

#[inline]
pub unsafe fn nla_next(na: *const nlattr, remaining: *mut i32) -> *mut nlattr {
    let aligned_len: i32 = NLA_ALIGN((*na).nla_len as i32);

    *remaining -= aligned_len;
    (na as *mut u8).add(aligned_len as usize) as *mut nlattr
}

unsafe extern "C" {
    pub fn netlink_open() -> i32;
    pub fn send_request(fd: i32, buf: *mut core::ffi::c_void, len: usize) -> i32;
    pub fn get_family_id(fd: i32, name: *const core::ffi::c_char) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
