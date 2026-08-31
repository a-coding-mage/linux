/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/util/build-id.h.
// C includes removed: "machine.h", "tool.h", <linux/types.h>.

pub const BUILD_ID_SIZE: usize = 20; /* SHA-1 length in bytes */
pub const BUILD_ID_MIN_SIZE: usize = 16; /* MD5/UUID/GUID length in bytes */
pub const SBUILD_ID_SIZE: usize = BUILD_ID_SIZE * 2 + 1;
pub const SBUILD_ID_MIN_SIZE: usize = BUILD_ID_MIN_SIZE * 2 + 1;

pub type u8 = ::std::os::raw::c_uchar;
pub type size_t = usize;

#[repr(C)]
pub struct build_id {
    pub data: [u8; BUILD_ID_SIZE],
    pub size: u8,
}

#[repr(C)]
pub struct dso {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct feat_fd {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct nsinfo {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_tool {
    _unused: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct strlist {
    _unused: [u8; 0],
}

// machine__dso_t is supplied by machine.h.
pub type machine__dso_t = Option<unsafe extern "C" fn()>;

unsafe extern "C" {
    pub fn build_id__init(bid: *mut build_id, data: *const u8, size: size_t);
    pub fn build_id__snprintf(
        build_id: *const build_id,
        bf: *mut ::std::os::raw::c_char,
        bf_size: size_t,
    ) -> ::std::os::raw::c_int;
    pub fn build_id__is_defined(bid: *const build_id) -> bool;
    pub fn sysfs__snprintf_build_id(
        root_dir: *const ::std::os::raw::c_char,
        sbuild_id: *mut ::std::os::raw::c_char,
        sbuild_id_size: size_t,
    ) -> ::std::os::raw::c_int;
    pub fn filename__snprintf_build_id(
        pathname: *const ::std::os::raw::c_char,
        sbuild_id: *mut ::std::os::raw::c_char,
        sbuild_id_size: size_t,
    ) -> ::std::os::raw::c_int;
    pub fn build_id_cache__kallsyms_path(
        sbuild_id: *const ::std::os::raw::c_char,
        bf: *mut ::std::os::raw::c_char,
        size: size_t,
    ) -> *mut ::std::os::raw::c_char;

    pub fn dso__build_id_filename(
        dso: *const dso,
        bf: *mut ::std::os::raw::c_char,
        size: size_t,
        is_debug: bool,
    ) -> *mut ::std::os::raw::c_char;
    pub fn __dso__build_id_filename(
        dso: *const dso,
        bf: *mut ::std::os::raw::c_char,
        size: size_t,
        is_debug: bool,
        is_kallsyms: bool,
    ) -> *mut ::std::os::raw::c_char;

    pub fn build_id__mark_dso_hit(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> ::std::os::raw::c_int;

    pub fn perf_session__read_build_ids(session: *mut perf_session, with_hits: bool) -> bool;
    pub fn perf_session__write_buildid_table(
        session: *mut perf_session,
        fd: *mut feat_fd,
    ) -> ::std::os::raw::c_int;
    pub fn perf_session__cache_build_ids(session: *mut perf_session) -> ::std::os::raw::c_int;
    pub fn __perf_session__cache_build_ids(
        session: *mut perf_session,
        fn_: machine__dso_t,
        priv_: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn build_id_cache__origname(
        sbuild_id: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn build_id_cache__linkname(
        sbuild_id: *const ::std::os::raw::c_char,
        bf: *mut ::std::os::raw::c_char,
        size: size_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn build_id_cache__cachedir(
        sbuild_id: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        nsi: *mut nsinfo,
        is_kallsyms: bool,
        is_vdso: bool,
    ) -> *mut ::std::os::raw::c_char;

    pub fn build_id_cache__list_all(validonly: bool) -> *mut strlist;
    pub fn build_id_cache__complement(
        incomplete_sbuild_id: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn build_id_cache__list_build_ids(
        pathname: *const ::std::os::raw::c_char,
        nsi: *mut nsinfo,
        result: *mut *mut strlist,
    ) -> ::std::os::raw::c_int;
    pub fn build_id_cache__cached(sbuild_id: *const ::std::os::raw::c_char) -> bool;
    pub fn build_id_cache__add(
        sbuild_id: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        realname: *const ::std::os::raw::c_char,
        nsi: *mut nsinfo,
        is_kallsyms: bool,
        is_vdso: bool,
        proper_name: *const ::std::os::raw::c_char,
        root_dir: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn __build_id_cache__add_s(
        sbuild_id: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        nsi: *mut nsinfo,
        is_kallsyms: bool,
        is_vdso: bool,
        proper_name: *const ::std::os::raw::c_char,
        root_dir: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn build_id_cache__remove_s(
        sbuild_id: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub static mut buildid_dir: [::std::os::raw::c_char; 0];

    pub fn set_buildid_dir(dir: *const ::std::os::raw::c_char);
    pub fn disable_buildid_cache();
}

#[inline]
pub unsafe fn build_id_cache__add_s(
    sbuild_id: *const ::std::os::raw::c_char,
    name: *const ::std::os::raw::c_char,
    nsi: *mut nsinfo,
    is_kallsyms: bool,
    is_vdso: bool,
) -> ::std::os::raw::c_int {
    unsafe {
        __build_id_cache__add_s(
            sbuild_id,
            name,
            nsi,
            is_kallsyms,
            is_vdso,
            ::std::ptr::null(),
            ::std::ptr::null(),
        )
    }
}
