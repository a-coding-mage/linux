/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h and linux/uidgid.h.

#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}

extern "C" {
    pub static mut nop_mnt_idmap: mnt_idmap;
    pub static mut invalid_mnt_idmap: mnt_idmap;
    pub static mut init_user_ns: user_namespace;
}

#[repr(C)]
pub struct vfsuid_t {
    pub val: uid_t,
}

#[repr(C)]
pub struct vfsgid_t {
    pub val: gid_t,
}

// C static assertions require the corresponding external kuid_t/kgid_t layouts.

#[inline]
pub unsafe fn is_valid_mnt_idmap(idmap: *const mnt_idmap) -> bool {
    idmap != core::ptr::addr_of!(nop_mnt_idmap) && idmap != core::ptr::addr_of!(invalid_mnt_idmap)
}

#[cfg(CONFIG_MULTIUSER)]
#[inline]
pub unsafe fn __vfsuid_val(uid: vfsuid_t) -> uid_t { uid.val }

#[cfg(not(CONFIG_MULTIUSER))]
#[inline]
pub unsafe fn __vfsuid_val(_uid: vfsuid_t) -> uid_t { 0 }

#[cfg(CONFIG_MULTIUSER)]
#[inline]
pub unsafe fn __vfsgid_val(gid: vfsgid_t) -> gid_t { gid.val }

#[cfg(not(CONFIG_MULTIUSER))]
#[inline]
pub unsafe fn __vfsgid_val(_gid: vfsgid_t) -> gid_t { 0 }

#[inline]
pub unsafe fn vfsuid_valid(uid: vfsuid_t) -> bool { __vfsuid_val(uid) != (!0 as uid_t) }

#[inline]
pub unsafe fn vfsgid_valid(gid: vfsgid_t) -> bool { __vfsgid_val(gid) != (!0 as gid_t) }

#[inline]
pub unsafe fn vfsuid_eq(left: vfsuid_t, right: vfsuid_t) -> bool {
    vfsuid_valid(left) && __vfsuid_val(left) == __vfsuid_val(right)
}

#[inline]
pub unsafe fn vfsgid_eq(left: vfsgid_t, right: vfsgid_t) -> bool {
    vfsgid_valid(left) && __vfsgid_val(left) == __vfsgid_val(right)
}

#[inline]
pub unsafe fn vfsuid_eq_kuid(vfsuid: vfsuid_t, kuid: kuid_t) -> bool {
    vfsuid_valid(vfsuid) && __vfsuid_val(vfsuid) == __kuid_val(kuid)
}

#[inline]
pub unsafe fn vfsgid_eq_kgid(vfsgid: vfsgid_t, kgid: kgid_t) -> bool {
    vfsgid_valid(vfsgid) && __vfsgid_val(vfsgid) == __kgid_val(kgid)
}

#[macro_export]
macro_rules! VFSUIDT_INIT { ($val:expr) => { $crate::vfsuid_t { val: unsafe { $crate::__kuid_val($val) } } }; }
#[macro_export]
macro_rules! VFSGIDT_INIT { ($val:expr) => { $crate::vfsgid_t { val: unsafe { $crate::__kgid_val($val) } } }; }
#[macro_export]
macro_rules! INVALID_VFSUID { () => { $crate::VFSUIDT_INIT!($crate::INVALID_UID) }; }
#[macro_export]
macro_rules! INVALID_VFSGID { () => { $crate::VFSGIDT_INIT!($crate::INVALID_GID) }; }
#[macro_export]
macro_rules! AS_KUIDT { ($val:expr) => { $crate::kuid_t { val: unsafe { $crate::__vfsuid_val($val) } } }; }
#[macro_export]
macro_rules! AS_KGIDT { ($val:expr) => { $crate::kgid_t { val: unsafe { $crate::__vfsgid_val($val) } } }; }

extern "C" {
    pub fn vfsgid_in_group_p(vfsgid: vfsgid_t) -> bool;
    pub fn mnt_idmap_get(idmap: *mut mnt_idmap) -> *mut mnt_idmap;
    pub fn mnt_idmap_put(idmap: *mut mnt_idmap);
    pub fn make_vfsuid(idmap: *mut mnt_idmap, fs_userns: *mut user_namespace, kuid: kuid_t) -> vfsuid_t;
    pub fn make_vfsgid(idmap: *mut mnt_idmap, fs_userns: *mut user_namespace, kgid: kgid_t) -> vfsgid_t;
    pub fn from_vfsuid(idmap: *mut mnt_idmap, fs_userns: *mut user_namespace, vfsuid: vfsuid_t) -> kuid_t;
    pub fn from_vfsgid(idmap: *mut mnt_idmap, fs_userns: *mut user_namespace, vfsgid: vfsgid_t) -> kgid_t;
}

#[inline]
pub unsafe fn vfsuid_has_fsmapping(idmap: *mut mnt_idmap, fs_userns: *mut user_namespace, vfsuid: vfsuid_t) -> bool {
    uid_valid(from_vfsuid(idmap, fs_userns, vfsuid))
}

#[inline]
pub unsafe fn vfsuid_has_mapping(userns: *mut user_namespace, vfsuid: vfsuid_t) -> bool {
    from_kuid(userns, AS_KUIDT!(vfsuid)) != (!0 as uid_t)
}

#[inline]
pub unsafe fn vfsuid_into_kuid(vfsuid: vfsuid_t) -> kuid_t { AS_KUIDT!(vfsuid) }

#[inline]
pub unsafe fn vfsgid_has_fsmapping(idmap: *mut mnt_idmap, fs_userns: *mut user_namespace, vfsgid: vfsgid_t) -> bool {
    gid_valid(from_vfsgid(idmap, fs_userns, vfsgid))
}

#[inline]
pub unsafe fn vfsgid_has_mapping(userns: *mut user_namespace, vfsgid: vfsgid_t) -> bool {
    from_kgid(userns, AS_KGIDT!(vfsgid)) != (!0 as gid_t)
}

#[inline]
pub unsafe fn vfsgid_into_kgid(vfsgid: vfsgid_t) -> kgid_t { AS_KGIDT!(vfsgid) }

#[inline]
pub unsafe fn mapped_fsuid(idmap: *mut mnt_idmap, fs_userns: *mut user_namespace) -> kuid_t {
    from_vfsuid(idmap, fs_userns, VFSUIDT_INIT!(current_fsuid()))
}

#[inline]
pub unsafe fn mapped_fsgid(idmap: *mut mnt_idmap, fs_userns: *mut user_namespace) -> kgid_t {
    from_vfsgid(idmap, fs_userns, VFSGIDT_INIT!(current_fsgid()))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
