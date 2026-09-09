// SPDX-License-Identifier: GPL-2.0
// Equivalent of: #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Linux kernel and generated skeleton dependencies are supplied externally.
// The selected iterator skeleton header depends on target byte order:
// iterators/iterators.lskel-little-endian.h or iterators/iterators.lskel-big-endian.h

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_preload_info {
    pub link_name: [c_char; 64],
    pub link: *mut bpf_link,
}

#[repr(C)]
pub struct iterators_bpf_links {
    pub dump_bpf_map_fd: c_int,
    pub dump_bpf_prog_fd: c_int,
}

#[repr(C)]
pub struct iterators_bpf {
    pub links: iterators_bpf_links,
}

#[repr(C)]
pub struct bpf_preload_ops {
    pub preload: Option<unsafe extern "C" fn(*mut bpf_preload_info) -> c_int>,
    pub owner: *mut c_void,
}

extern "C" {
    static mut bpf_preload_ops: *mut bpf_preload_ops;

    fn bpf_link_put(link: *mut bpf_link);
    fn iterators_bpf__destroy(skel: *mut iterators_bpf);
    fn iterators_bpf__open() -> *mut iterators_bpf;
    fn iterators_bpf__load(skel: *mut iterators_bpf) -> c_int;
    fn iterators_bpf__attach(skel: *mut iterators_bpf) -> c_int;
    fn bpf_link_get_from_fd(fd: c_int) -> *mut bpf_link;
    fn close_fd(fd: c_int);
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    static mut THIS_MODULE: c_void;
}

const ENOMEM: c_int = 12;

static mut maps_link: *mut bpf_link = core::ptr::null_mut();
static mut progs_link: *mut bpf_link = core::ptr::null_mut();
static mut skel: *mut iterators_bpf = core::ptr::null_mut();

unsafe fn is_err_or_null<T>(ptr: *mut T) -> bool {
    ptr.is_null() || (ptr as isize) >= -4095
}

unsafe fn ptr_err<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

unsafe fn free_links_and_skel() {
    if !is_err_or_null(maps_link) {
        bpf_link_put(maps_link);
    }
    if !is_err_or_null(progs_link) {
        bpf_link_put(progs_link);
    }
    iterators_bpf__destroy(skel);
}

unsafe extern "C" fn preload(obj: *mut bpf_preload_info) -> c_int {
    let maps_name = b"maps.debug\0";
    strscpy((*obj.add(0)).link_name.as_mut_ptr(), maps_name.as_ptr() as *const c_char,
            core::mem::size_of_val(&(*obj.add(0)).link_name));
    (*obj.add(0)).link = maps_link;
    let progs_name = b"progs.debug\0";
    strscpy((*obj.add(1)).link_name.as_mut_ptr(), progs_name.as_ptr() as *const c_char,
            core::mem::size_of_val(&(*obj.add(1)).link_name));
    (*obj.add(1)).link = progs_link;
    0
}

static mut ops: bpf_preload_ops = bpf_preload_ops {
    preload: Some(preload),
    owner: unsafe { &raw mut THIS_MODULE },
};

unsafe fn load_skel() -> c_int {
    let mut err: c_int;

    skel = iterators_bpf__open();
    if skel.is_null() {
        return -ENOMEM;
    }
    err = iterators_bpf__load(skel);
    if err != 0 {
        return out_load_skel(err);
    }
    err = iterators_bpf__attach(skel);
    if err != 0 {
        return out_load_skel(err);
    }
    maps_link = bpf_link_get_from_fd((*skel).links.dump_bpf_map_fd);
    if is_err(maps_link) {
        err = ptr_err(maps_link);
        return out_load_skel(err);
    }
    progs_link = bpf_link_get_from_fd((*skel).links.dump_bpf_prog_fd);
    if is_err(progs_link) {
        err = ptr_err(progs_link);
        return out_load_skel(err);
    }
    // Avoid taking over stdin/stdout/stderr of init process. Zeroing out
    // makes skel_closenz() a no-op later in iterators_bpf__destroy().
    close_fd((*skel).links.dump_bpf_map_fd);
    (*skel).links.dump_bpf_map_fd = 0;
    close_fd((*skel).links.dump_bpf_prog_fd);
    (*skel).links.dump_bpf_prog_fd = 0;
    0
}

unsafe fn is_err<T>(ptr: *mut T) -> bool {
    (ptr as isize) >= -4095
}

unsafe fn out_load_skel(err: c_int) -> c_int {
    free_links_and_skel();
    err
}

unsafe extern "C" fn load() -> c_int {
    let err = load_skel();
    if err != 0 {
        return err;
    }
    bpf_preload_ops = &raw mut ops;
    err
}

unsafe extern "C" fn fini() {
    bpf_preload_ops = core::ptr::null_mut();
    free_links_and_skel();
}

// late_initcall(load);
// module_exit(fini);
// MODULE_IMPORT_NS("BPF_INTERNAL");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Embedded BPF programs for introspection in bpffs");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
