/* SPDX-License-Identifier: GPL-2.0 */

/*
 * A set of types for the internal kernel types representing uids and gids.
 * The C header's external type and constant dependencies are supplied by
 * other translated headers.
 */

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub struct user_namespace;
extern "C" {
    pub static mut init_user_ns: user_namespace;
}

pub struct uid_gid_map;

#[macro_export]
macro_rules! KUIDT_INIT {
    ($value:expr) => {
        kuid_t { val: $value }
    };
}

#[macro_export]
macro_rules! KGIDT_INIT {
    ($value:expr) => {
        kgid_t { val: $value }
    };
}

#[cfg(CONFIG_MULTIUSER)]
#[inline]
pub fn __kuid_val(uid: kuid_t) -> uid_t {
    uid.val
}

#[cfg(not(CONFIG_MULTIUSER))]
#[inline]
pub fn __kuid_val(_uid: kuid_t) -> uid_t {
    0
}

#[cfg(CONFIG_MULTIUSER)]
#[inline]
pub fn __kgid_val(gid: kgid_t) -> gid_t {
    gid.val
}

#[cfg(not(CONFIG_MULTIUSER))]
#[inline]
pub fn __kgid_val(_gid: kgid_t) -> gid_t {
    0
}

pub const GLOBAL_ROOT_UID: kuid_t = KUIDT_INIT!(0);
pub const GLOBAL_ROOT_GID: kgid_t = KGIDT_INIT!(0);
pub const INVALID_UID: kuid_t = KUIDT_INIT!(-1);
pub const INVALID_GID: kgid_t = KGIDT_INIT!(-1);

#[inline]
pub fn uid_eq(left: kuid_t, right: kuid_t) -> bool { __kuid_val(left) == __kuid_val(right) }
#[inline]
pub fn gid_eq(left: kgid_t, right: kgid_t) -> bool { __kgid_val(left) == __kgid_val(right) }
#[inline]
pub fn uid_gt(left: kuid_t, right: kuid_t) -> bool { __kuid_val(left) > __kuid_val(right) }
#[inline]
pub fn gid_gt(left: kgid_t, right: kgid_t) -> bool { __kgid_val(left) > __kgid_val(right) }
#[inline]
pub fn uid_gte(left: kuid_t, right: kuid_t) -> bool { __kuid_val(left) >= __kuid_val(right) }
#[inline]
pub fn gid_gte(left: kgid_t, right: kgid_t) -> bool { __kgid_val(left) >= __kgid_val(right) }
#[inline]
pub fn uid_lt(left: kuid_t, right: kuid_t) -> bool { __kuid_val(left) < __kuid_val(right) }
#[inline]
pub fn gid_lt(left: kgid_t, right: kgid_t) -> bool { __kgid_val(left) < __kgid_val(right) }
#[inline]
pub fn uid_lte(left: kuid_t, right: kuid_t) -> bool { __kuid_val(left) <= __kuid_val(right) }
#[inline]
pub fn gid_lte(left: kgid_t, right: kgid_t) -> bool { __kgid_val(left) <= __kgid_val(right) }
#[inline]
pub fn uid_valid(uid: kuid_t) -> bool { __kuid_val(uid) != (-1i32 as uid_t) }
#[inline]
pub fn gid_valid(gid: kgid_t) -> bool { __kgid_val(gid) != (-1i32 as gid_t) }

#[cfg(CONFIG_USER_NS)]
extern "C" {
    pub fn make_kuid(from: *mut user_namespace, uid: uid_t) -> kuid_t;
    pub fn make_kgid(from: *mut user_namespace, gid: gid_t) -> kgid_t;
    pub fn from_kuid(to: *mut user_namespace, uid: kuid_t) -> uid_t;
    pub fn from_kgid(to: *mut user_namespace, uid: kgid_t) -> gid_t;
    pub fn from_kuid_munged(to: *mut user_namespace, uid: kuid_t) -> uid_t;
    pub fn from_kgid_munged(to: *mut user_namespace, gid: kgid_t) -> gid_t;
    pub fn map_id_down(map: *mut uid_gid_map, id: u32) -> u32;
    pub fn map_id_up(map: *mut uid_gid_map, id: u32) -> u32;
    pub fn map_id_range_up(map: *mut uid_gid_map, id: u32, count: u32) -> u32;
}

#[cfg(CONFIG_USER_NS)]
#[inline]
pub fn kuid_has_mapping(ns: *mut user_namespace, uid: kuid_t) -> bool {
    unsafe { from_kuid(ns, uid) != (-1i32 as uid_t) }
}
#[cfg(CONFIG_USER_NS)]
#[inline]
pub fn kgid_has_mapping(ns: *mut user_namespace, gid: kgid_t) -> bool {
    unsafe { from_kgid(ns, gid) != (-1i32 as gid_t) }
}

#[cfg(not(CONFIG_USER_NS))]
#[inline]
pub fn make_kuid(_from: *mut user_namespace, uid: uid_t) -> kuid_t { KUIDT_INIT!(uid) }
#[cfg(not(CONFIG_USER_NS))]
#[inline]
pub fn make_kgid(_from: *mut user_namespace, gid: gid_t) -> kgid_t { KGIDT_INIT!(gid) }
#[cfg(not(CONFIG_USER_NS))]
#[inline]
pub fn from_kuid(_to: *mut user_namespace, kuid: kuid_t) -> uid_t { __kuid_val(kuid) }
#[cfg(not(CONFIG_USER_NS))]
#[inline]
pub fn from_kgid(_to: *mut user_namespace, kgid: kgid_t) -> gid_t { __kgid_val(kgid) }
#[cfg(not(CONFIG_USER_NS))]
#[inline]
pub fn from_kuid_munged(to: *mut user_namespace, kuid: kuid_t) -> uid_t {
    let mut uid = from_kuid(to, kuid);
    if uid == (-1i32 as uid_t) { uid = overflowuid; }
    uid
}
#[cfg(not(CONFIG_USER_NS))]
#[inline]
pub fn from_kgid_munged(to: *mut user_namespace, kgid: kgid_t) -> gid_t {
    let mut gid = from_kgid(to, kgid);
    if gid == (-1i32 as gid_t) { gid = overflowgid; }
    gid
}
#[cfg(not(CONFIG_USER_NS))]
#[inline]
pub fn kuid_has_mapping(_ns: *mut user_namespace, uid: kuid_t) -> bool { uid_valid(uid) }
#[cfg(not(CONFIG_USER_NS))]
#[inline]
pub fn kgid_has_mapping(_ns: *mut user_namespace, gid: kgid_t) -> bool { gid_valid(gid) }
#[cfg(not(CONFIG_USER_NS))]
#[inline]
pub fn map_id_down(_map: *mut uid_gid_map, id: u32) -> u32 { id }
#[cfg(not(CONFIG_USER_NS))]
#[inline]
pub fn map_id_range_up(_map: *mut uid_gid_map, id: u32, _count: u32) -> u32 { id }
#[cfg(not(CONFIG_USER_NS))]
#[inline]
pub fn map_id_up(_map: *mut uid_gid_map, id: u32) -> u32 { id }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
