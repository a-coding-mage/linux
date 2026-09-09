// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from the Linux ref_tracker header.

// Required external dependencies are supplied by other translated headers.

#[repr(C)]
pub struct ref_tracker {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct ref_tracker_dir {
    #[cfg(feature = "CONFIG_REF_TRACKER")]
    pub lock: spinlock_t,
    #[cfg(feature = "CONFIG_REF_TRACKER")]
    pub quarantine_avail: ::core::ffi::c_uint,
    #[cfg(feature = "CONFIG_REF_TRACKER")]
    pub untracked: refcount_t,
    #[cfg(feature = "CONFIG_REF_TRACKER")]
    pub no_tracker: refcount_t,
    #[cfg(feature = "CONFIG_REF_TRACKER")]
    pub dead: bool,
    #[cfg(feature = "CONFIG_REF_TRACKER")]
    pub list: list_head, // List of active trackers
    #[cfg(feature = "CONFIG_REF_TRACKER")]
    pub quarantine: list_head, // List of dead trackers
    #[cfg(feature = "CONFIG_REF_TRACKER")]
    pub class: *const ::core::ffi::c_char, // object classname
}

#[cfg(feature = "CONFIG_REF_TRACKER")]
#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub fn ref_tracker_dir_debugfs(dir: *mut ref_tracker_dir);
    pub fn ref_tracker_dir_symlink(
        dir: *mut ref_tracker_dir,
        fmt: *const ::core::ffi::c_char,
        ...,
    );
}

#[cfg(feature = "CONFIG_REF_TRACKER")]
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn ref_tracker_dir_debugfs(_dir: *mut ref_tracker_dir) {}

#[cfg(feature = "CONFIG_REF_TRACKER")]
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe extern "C" fn ref_tracker_dir_symlink(
    _dir: *mut ref_tracker_dir,
    _fmt: *const ::core::ffi::c_char,
    ...,
) {
}

#[cfg(feature = "CONFIG_REF_TRACKER")]
#[inline]
pub unsafe fn ref_tracker_dir_init(
    dir: *mut ref_tracker_dir,
    quarantine_count: ::core::ffi::c_uint,
    class: *const ::core::ffi::c_char,
) {
    INIT_LIST_HEAD(::core::ptr::addr_of_mut!((*dir).list));
    INIT_LIST_HEAD(::core::ptr::addr_of_mut!((*dir).quarantine));
    spin_lock_init(::core::ptr::addr_of_mut!((*dir).lock));
    (*dir).quarantine_avail = quarantine_count;
    (*dir).dead = false;
    refcount_set(::core::ptr::addr_of_mut!((*dir).untracked), 1);
    refcount_set(::core::ptr::addr_of_mut!((*dir).no_tracker), 1);
    (*dir).class = class;
    ref_tracker_dir_debugfs(dir);
    stack_depot_init();
}

#[cfg(feature = "CONFIG_REF_TRACKER")]
extern "C" {
    pub fn ref_tracker_dir_exit(dir: *mut ref_tracker_dir);
    pub fn ref_tracker_dir_print_locked(
        dir: *mut ref_tracker_dir,
        display_limit: ::core::ffi::c_uint,
    );
    pub fn ref_tracker_dir_print(
        dir: *mut ref_tracker_dir,
        display_limit: ::core::ffi::c_uint,
    );
    pub fn ref_tracker_dir_snprint(
        dir: *mut ref_tracker_dir,
        buf: *mut ::core::ffi::c_char,
        size: usize,
    ) -> ::core::ffi::c_int;
    pub fn ref_tracker_alloc(
        dir: *mut ref_tracker_dir,
        trackerp: *mut *mut ref_tracker,
        gfp: gfp_t,
    ) -> ::core::ffi::c_int;
    pub fn ref_tracker_free(
        dir: *mut ref_tracker_dir,
        trackerp: *mut *mut ref_tracker,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_REF_TRACKER"))]
#[inline]
pub unsafe fn ref_tracker_dir_init(
    _dir: *mut ref_tracker_dir,
    _quarantine_count: ::core::ffi::c_uint,
    _class: *const ::core::ffi::c_char,
) {
}

#[cfg(not(feature = "CONFIG_REF_TRACKER"))]
#[inline]
pub unsafe fn ref_tracker_dir_debugfs(_dir: *mut ref_tracker_dir) {}

#[cfg(not(feature = "CONFIG_REF_TRACKER"))]
#[inline]
pub unsafe extern "C" fn ref_tracker_dir_symlink(
    _dir: *mut ref_tracker_dir,
    _fmt: *const ::core::ffi::c_char,
    ...,
) {
}

#[cfg(not(feature = "CONFIG_REF_TRACKER"))]
#[inline]
pub unsafe fn ref_tracker_dir_exit(_dir: *mut ref_tracker_dir) {}

#[cfg(not(feature = "CONFIG_REF_TRACKER"))]
#[inline]
pub unsafe fn ref_tracker_dir_print_locked(
    _dir: *mut ref_tracker_dir,
    _display_limit: ::core::ffi::c_uint,
) {
}

#[cfg(not(feature = "CONFIG_REF_TRACKER"))]
#[inline]
pub unsafe fn ref_tracker_dir_print(
    _dir: *mut ref_tracker_dir,
    _display_limit: ::core::ffi::c_uint,
) {
}

#[cfg(not(feature = "CONFIG_REF_TRACKER"))]
#[inline]
pub unsafe fn ref_tracker_dir_snprint(
    _dir: *mut ref_tracker_dir,
    _buf: *mut ::core::ffi::c_char,
    _size: usize,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_REF_TRACKER"))]
#[inline]
pub unsafe fn ref_tracker_alloc(
    _dir: *mut ref_tracker_dir,
    _trackerp: *mut *mut ref_tracker,
    _gfp: gfp_t,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_REF_TRACKER"))]
#[inline]
pub unsafe fn ref_tracker_free(
    _dir: *mut ref_tracker_dir,
    _trackerp: *mut *mut ref_tracker,
) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
