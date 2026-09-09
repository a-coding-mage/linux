/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux Ceph OSD client layer:
// ceph_entity_name, ceph_entity_addr, ceph_osd_client, ceph_object_id,
// ceph_object_locator, and ceph_osd_request.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ceph_cls_lock_type {
    CEPH_CLS_LOCK_NONE = 0,
    CEPH_CLS_LOCK_EXCLUSIVE = 1,
    CEPH_CLS_LOCK_SHARED = 2,
}

#[repr(C)]
pub struct ceph_locker_id {
    pub name: ceph_entity_name,
    pub cookie: *mut ::std::os::raw::c_char,
}

#[repr(C)]
pub struct ceph_locker_info {
    pub addr: ceph_entity_addr,
}

#[repr(C)]
pub struct ceph_locker {
    pub id: ceph_locker_id,
    pub info: ceph_locker_info,
}

extern "C" {
    pub fn ceph_cls_lock(
        osdc: *mut ceph_osd_client,
        oid: *mut ceph_object_id,
        oloc: *mut ceph_object_locator,
        lock_name: *mut ::std::os::raw::c_char,
        type_: u8,
        cookie: *mut ::std::os::raw::c_char,
        tag: *mut ::std::os::raw::c_char,
        desc: *mut ::std::os::raw::c_char,
        flags: u8,
    ) -> ::std::os::raw::c_int;

    pub fn ceph_cls_unlock(
        osdc: *mut ceph_osd_client,
        oid: *mut ceph_object_id,
        oloc: *mut ceph_object_locator,
        lock_name: *mut ::std::os::raw::c_char,
        cookie: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn ceph_cls_break_lock(
        osdc: *mut ceph_osd_client,
        oid: *mut ceph_object_id,
        oloc: *mut ceph_object_locator,
        lock_name: *mut ::std::os::raw::c_char,
        cookie: *mut ::std::os::raw::c_char,
        locker: *mut ceph_entity_name,
    ) -> ::std::os::raw::c_int;

    pub fn ceph_cls_set_cookie(
        osdc: *mut ceph_osd_client,
        oid: *mut ceph_object_id,
        oloc: *mut ceph_object_locator,
        lock_name: *mut ::std::os::raw::c_char,
        type_: u8,
        old_cookie: *mut ::std::os::raw::c_char,
        tag: *mut ::std::os::raw::c_char,
        new_cookie: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn ceph_free_lockers(lockers: *mut ceph_locker, num_lockers: u32);

    pub fn ceph_cls_lock_info(
        osdc: *mut ceph_osd_client,
        oid: *mut ceph_object_id,
        oloc: *mut ceph_object_locator,
        lock_name: *mut ::std::os::raw::c_char,
        type_: *mut u8,
        tag: *mut *mut ::std::os::raw::c_char,
        lockers: *mut *mut ceph_locker,
        num_lockers: *mut u32,
    ) -> ::std::os::raw::c_int;

    pub fn ceph_cls_assert_locked(
        req: *mut ceph_osd_request,
        which: ::std::os::raw::c_int,
        lock_name: *mut ::std::os::raw::c_char,
        type_: u8,
        cookie: *mut ::std::os::raw::c_char,
        tag: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
