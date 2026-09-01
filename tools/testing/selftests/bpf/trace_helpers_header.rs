/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent from C header: <bpf/libbpf.h> */

#[cfg(target_arch = "x86_64")]
pub const SYS_PREFIX: &str = "__x64_";
#[cfg(target_arch = "s390x")]
pub const SYS_PREFIX: &str = "__s390x_";
#[cfg(target_arch = "aarch64")]
pub const SYS_PREFIX: &str = "__arm64_";
#[cfg(target_arch = "riscv64")]
pub const SYS_PREFIX: &str = "__riscv_";
#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "s390x",
    target_arch = "aarch64",
    target_arch = "riscv64"
)))]
pub const SYS_PREFIX: &str = "";

#[macro_export]
macro_rules! __ALIGN_MASK {
    ($x:expr, $mask:expr) => {
        (($x).wrapping_add($mask)) & !($mask)
    };
}

#[macro_export]
macro_rules! ALIGN {
    ($x:expr, $a:expr) => {
        $crate::__ALIGN_MASK!($x, (($a) as _).wrapping_sub(1))
    };
}

#[repr(C)]
pub struct ksym {
    pub addr: ::std::os::raw::c_long,
    pub name: *mut ::std::os::raw::c_char,
}

#[repr(C)]
pub struct ksyms {
    pub syms: *mut ksym,
    pub sym_cap: usize,
    pub sym_cnt: usize,
    pub filtered_syms: *mut *mut ::std::os::raw::c_char,
    pub filtered_cnt: usize,
}

pub type ksym_cmp_t = Option<
    unsafe extern "C" fn(
        p1: *const ::std::os::raw::c_void,
        p2: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type ksym_search_cmp_t = Option<
    unsafe extern "C" fn(
        p1: *const ::std::os::raw::c_void,
        p2: *const ksym,
    ) -> ::std::os::raw::c_int,
>;

unsafe extern "C" {
    pub fn load_kallsyms() -> ::std::os::raw::c_int;
    pub fn ksym_search(key: ::std::os::raw::c_long) -> *mut ksym;
    pub fn ksym_get_addr(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;

    pub fn load_kallsyms_local() -> *mut ksyms;
    pub fn ksym_search_local(ksyms: *mut ksyms, key: ::std::os::raw::c_long) -> *mut ksym;
    pub fn ksym_get_addr_local(
        ksyms: *mut ksyms,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
    pub fn free_kallsyms_local(ksyms: *mut ksyms);

    pub fn load_kallsyms_custom_local(cmp_cb: ksym_cmp_t) -> *mut ksyms;
    pub fn search_kallsyms_custom_local(
        ksyms: *mut ksyms,
        p1: *const ::std::os::raw::c_void,
        cmp_cb: ksym_search_cmp_t,
    ) -> *mut ksym;

    /* open kallsyms and find addresses on the fly, faster than load + search. */
    pub fn kallsyms_find(
        sym: *const ::std::os::raw::c_char,
        addr: *mut ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;

    pub fn read_trace_pipe();
    pub fn read_trace_pipe_iter(
        cb: Option<
            unsafe extern "C" fn(str: *const ::std::os::raw::c_char, data: *mut ::std::os::raw::c_void),
        >,
        data: *mut ::std::os::raw::c_void,
        iter: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn get_uprobe_offset(addr: *const ::std::os::raw::c_void) -> isize;
    pub fn get_rel_offset(addr: usize) -> isize;

    pub fn read_build_id(
        path: *const ::std::os::raw::c_char,
        build_id: *mut ::std::os::raw::c_char,
        size: usize,
    ) -> ::std::os::raw::c_int;

    pub fn bpf_get_ksyms(ksymsp: *mut *mut ksyms, kernel: bool) -> ::std::os::raw::c_int;
    pub fn bpf_get_addrs(
        addrsp: *mut *mut ::std::os::raw::c_ulong,
        cntp: *mut usize,
        kernel: bool,
    ) -> ::std::os::raw::c_int;

    pub fn is_unsafe_function(name: *const ::std::os::raw::c_char) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
