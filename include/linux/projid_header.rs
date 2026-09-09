/* SPDX-License-Identifier: GPL-2.0 */

/*
 * A set of types for the internal kernel types representing project ids.
 *
 * The types defined in this header allow distinguishing which project ids in
 * the kernel are values used by userspace and which project id values are the
 * internal kernel values. With the addition of user namespaces the values
 * can be different. Using the type system makes it possible for the compiler
 * to detect when we overlook these differences.
 */

#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut init_user_ns: user_namespace;
}

pub type projid_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kprojid_t {
    pub val: projid_t,
}

#[inline]
pub const unsafe fn __kprojid_val(projid: kprojid_t) -> projid_t {
    projid.val
}

#[inline]
pub const fn kprojidt_init(value: projid_t) -> kprojid_t {
    kprojid_t { val: value }
}

pub const INVALID_PROJID: kprojid_t = kprojidt_init(u32::MAX);
pub const OVERFLOW_PROJID: projid_t = 65534;

#[inline]
pub unsafe fn projid_eq(left: kprojid_t, right: kprojid_t) -> bool {
    __kprojid_val(left) == __kprojid_val(right)
}

#[inline]
pub unsafe fn projid_lt(left: kprojid_t, right: kprojid_t) -> bool {
    __kprojid_val(left) < __kprojid_val(right)
}

#[inline]
pub unsafe fn projid_valid(projid: kprojid_t) -> bool {
    !projid_eq(projid, INVALID_PROJID)
}

/* CONFIG_USER_NS selects the declaration-based implementation below. */
#[cfg(feature = "CONFIG_USER_NS")]
unsafe extern "C" {
    pub fn make_kprojid(from: *mut user_namespace, projid: projid_t) -> kprojid_t;
    pub fn from_kprojid(to: *mut user_namespace, projid: kprojid_t) -> projid_t;
    pub fn from_kprojid_munged(to: *mut user_namespace, projid: kprojid_t) -> projid_t;
}

#[cfg(feature = "CONFIG_USER_NS")]
#[inline]
pub unsafe fn kprojid_has_mapping(ns: *mut user_namespace, projid: kprojid_t) -> bool {
    from_kprojid(ns, projid) != u32::MAX
}

#[cfg(not(feature = "CONFIG_USER_NS"))]
#[inline]
pub unsafe fn make_kprojid(_from: *mut user_namespace, projid: projid_t) -> kprojid_t {
    kprojidt_init(projid)
}

#[cfg(not(feature = "CONFIG_USER_NS"))]
#[inline]
pub unsafe fn from_kprojid(_to: *mut user_namespace, kprojid: kprojid_t) -> projid_t {
    __kprojid_val(kprojid)
}

#[cfg(not(feature = "CONFIG_USER_NS"))]
#[inline]
pub unsafe fn from_kprojid_munged(to: *mut user_namespace, kprojid: kprojid_t) -> projid_t {
    let mut projid = from_kprojid(to, kprojid);
    if projid == u32::MAX {
        projid = OVERFLOW_PROJID;
    }
    projid
}

#[cfg(not(feature = "CONFIG_USER_NS"))]
#[inline]
pub unsafe fn kprojid_has_mapping(_ns: *mut user_namespace, _projid: kprojid_t) -> bool {
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
