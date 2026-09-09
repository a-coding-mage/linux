/* SPDX-License-Identifier: GPL-2.0 */

// Translated dependencies:
// #include <linux/ns_common.h>
// #include <uapi/linux/utsname.h>

use core::ffi::c_ulong;

// Declaration supplied by another translation unit.
pub enum UserNamespace {}

extern "C" {
    pub static mut init_user_ns: UserNamespace;
}

#[repr(C)]
pub struct UtsNamespace {
    pub name: NewUtsname,
    pub user_ns: *mut UserNamespace,
    pub ucounts: *mut Ucounts,
    pub ns: NsCommon,
}

// __randomize_layout

extern "C" {
    pub static mut init_uts_ns: UtsNamespace;
}

#[cfg(CONFIG_UTS_NS)]
#[inline]
pub unsafe fn to_uts_ns(ns: *mut NsCommon) -> *mut UtsNamespace {
    // Equivalent of container_of(ns, struct uts_namespace, ns).
    container_of!(ns, UtsNamespace, ns)
}

#[cfg(CONFIG_UTS_NS)]
#[inline]
pub unsafe fn get_uts_ns(ns: *mut UtsNamespace) {
    ns_ref_inc(ns);
}

#[cfg(CONFIG_UTS_NS)]
extern "C" {
    pub fn copy_utsname(
        flags: u64,
        user_ns: *mut UserNamespace,
        old_ns: *mut UtsNamespace,
    ) -> *mut UtsNamespace;
    pub fn free_uts_ns(ns: *mut UtsNamespace);
}

#[cfg(CONFIG_UTS_NS)]
#[inline]
pub unsafe fn put_uts_ns(ns: *mut UtsNamespace) {
    if ns_ref_put(ns) {
        free_uts_ns(ns);
    }
}

#[cfg(CONFIG_UTS_NS)]
extern "C" {
    pub fn uts_ns_init();
}

#[cfg(not(CONFIG_UTS_NS))]
#[inline]
pub unsafe fn get_uts_ns(_ns: *mut UtsNamespace) {}

#[cfg(not(CONFIG_UTS_NS))]
#[inline]
pub unsafe fn put_uts_ns(_ns: *mut UtsNamespace) {}

#[cfg(not(CONFIG_UTS_NS))]
#[inline]
pub unsafe fn copy_utsname(
    flags: u64,
    _user_ns: *mut UserNamespace,
    old_ns: *mut UtsNamespace,
) -> *mut UtsNamespace {
    if flags & CLONE_NEWUTS != 0 {
        return ERR_PTR(-EINVAL);
    }

    old_ns
}

#[cfg(not(CONFIG_UTS_NS))]
#[inline]
pub unsafe fn uts_ns_init() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
