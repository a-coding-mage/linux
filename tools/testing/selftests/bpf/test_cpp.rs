/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */
/* Dependencies in the original source:
 * <iostream>, <unistd.h>, <linux/bpf.h>, <linux/btf.h>,
 * <bpf/libbpf.h>, <bpf/bpf.h>, <bpf/btf.h>,
 * "test_core_extern.skel.h", "struct_ops_module.skel.h"
 *
 * The original C++ source defines _Bool as bool before including skeleton
 * headers when _Bool is unavailable.
 */

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

const EBUSY: c_int = 16;
const BPF_STATS_RUN_TIME: c_int = 0;

#[repr(C)]
pub struct bpf_object_open_opts {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct btf_dump_opts {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct btf_dump {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct test_core_extern_data {
    pub kern_ver: c_int,
    pub int_val: c_int,
    pub ushort_val: u16,
}

#[repr(C)]
pub struct test_core_extern_kconfig {
    pub CONFIG_BPF_SYSCALL: bool,
}

#[repr(C)]
pub struct test_core_extern_progs {
    pub handle_sys_enter: *mut bpf_program,
}

#[repr(C)]
pub struct test_core_extern_links {
    pub handle_sys_enter: *mut bpf_link,
}

#[repr(C)]
pub struct test_core_extern {
    pub data: *mut test_core_extern_data,
    pub kconfig: *mut test_core_extern_kconfig,
    pub progs: test_core_extern_progs,
    pub links: test_core_extern_links,
}

#[repr(C)]
pub struct struct_ops_module {
    _unused: [u8; 0],
}

unsafe extern "C" {
    static mut stderr: *mut c_void;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn libbpf_get_error(ptr: *const c_void) -> isize;
    fn libbpf_set_print(
        print_fn: Option<
            unsafe extern "C" fn(
                level: c_int,
                format: *const c_char,
                args: *mut c_void,
            ) -> c_int,
        >,
    );

    fn bpf_prog_get_fd_by_id(id: u32) -> c_int;
    fn bpf_enable_stats(r#type: c_int) -> c_int;

    fn btf__new(data: *const c_void, size: usize) -> *mut btf;
    fn btf_dump__new(
        btf: *mut btf,
        printf_fn: unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void),
        ctx: *mut c_void,
        opts: *const btf_dump_opts,
    ) -> *mut btf_dump;

    fn bpf_program__name(prog: *const bpf_program) -> *const c_char;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_program__attach(prog: *const bpf_program) -> *mut bpf_link;

    fn test_core_extern__open(opts: *const bpf_object_open_opts) -> *mut test_core_extern;
    fn test_core_extern__load(skel: *mut test_core_extern) -> c_int;
    fn test_core_extern__attach(skel: *mut test_core_extern) -> c_int;
    fn test_core_extern__detach(skel: *mut test_core_extern);
    fn test_core_extern__destroy(skel: *mut test_core_extern);
    fn test_core_extern__open_and_load() -> *mut test_core_extern;

    fn struct_ops_module__open_and_load() -> *mut struct_ops_module;
    fn struct_ops_module__destroy(skel: *mut struct_ops_module);
}

trait SkeletonOps {
    unsafe fn open(opts: *const bpf_object_open_opts) -> *mut Self;
    unsafe fn load(skel: *mut Self) -> c_int;
    unsafe fn attach(skel: *mut Self) -> c_int;
    unsafe fn detach(skel: *mut Self);
    unsafe fn destroy(skel: *mut Self);
}

impl SkeletonOps for test_core_extern {
    unsafe fn open(opts: *const bpf_object_open_opts) -> *mut Self {
        unsafe { test_core_extern__open(opts) }
    }

    unsafe fn load(skel: *mut Self) -> c_int {
        unsafe { test_core_extern__load(skel) }
    }

    unsafe fn attach(skel: *mut Self) -> c_int {
        unsafe { test_core_extern__attach(skel) }
    }

    unsafe fn detach(skel: *mut Self) {
        unsafe { test_core_extern__detach(skel) }
    }

    unsafe fn destroy(skel: *mut Self) {
        unsafe { test_core_extern__destroy(skel) }
    }
}

struct Skeleton<T: SkeletonOps> {
    skel: *mut T,
}

impl<T: SkeletonOps> Skeleton<T> {
    fn new() -> Self {
        Self {
            skel: ptr::null_mut(),
        }
    }

    unsafe fn open(&mut self, opts: *const bpf_object_open_opts) -> c_int {
        let err: c_int;

        if !self.skel.is_null() {
            return -EBUSY;
        }

        self.skel = unsafe { T::open(opts) };
        err = unsafe { libbpf_get_error(self.skel as *const c_void) as c_int };
        if err != 0 {
            self.skel = ptr::null_mut();
            return err;
        }

        0
    }

    unsafe fn load(&mut self) -> c_int {
        unsafe { T::load(self.skel) }
    }

    unsafe fn attach(&mut self) -> c_int {
        unsafe { T::attach(self.skel) }
    }

    unsafe fn detach(&mut self) {
        unsafe { T::detach(self.skel) }
    }

    fn get(&self) -> *const T {
        self.skel
    }

    fn get_mut(&mut self) -> *mut T {
        self.skel
    }
}

impl<T: SkeletonOps> Drop for Skeleton<T> {
    fn drop(&mut self) {
        if !self.skel.is_null() {
            unsafe { T::destroy(self.skel) };
        }
    }
}

unsafe extern "C" fn dump_printf(_ctx: *mut c_void, _fmt: *const c_char, _args: *mut c_void) {}

fn try_skeleton_template() {
    let mut skel: Skeleton<test_core_extern> = Skeleton::new();
    let mut prog_name: String;
    let mut err: c_int;
    let opts: bpf_object_open_opts = unsafe { std::mem::zeroed() };

    unsafe {
        err = skel.open(&opts);
        if err != 0 {
            fprintf(
                stderr,
                c"Skeleton open failed: %d\n".as_ptr(),
                err,
            );
            return;
        }

        (*(*skel.get_mut()).data).kern_ver = 123;
        (*(*skel.get_mut()).data).int_val = (*(*skel.get_mut()).data).ushort_val as c_int;

        err = skel.load();
        if err != 0 {
            fprintf(
                stderr,
                c"Skeleton load failed: %d\n".as_ptr(),
                err,
            );
            return;
        }

        if !(*(*skel.get()).kconfig).CONFIG_BPF_SYSCALL {
            fprintf(
                stderr,
                c"Seems like CONFIG_BPF_SYSCALL isn't set?!\n".as_ptr(),
            );
        }

        err = skel.attach();
        if err != 0 {
            fprintf(
                stderr,
                c"Skeleton attach failed: %d\n".as_ptr(),
                err,
            );
            return;
        }

        prog_name = CStr::from_ptr(bpf_program__name((*skel.get()).progs.handle_sys_enter))
            .to_string_lossy()
            .into_owned();
        if prog_name != "handle_sys_enter" {
            fprintf(
                stderr,
                c"Unexpected program name: %s\n".as_ptr(),
                prog_name.as_ptr() as *const c_char,
            );
        }

        bpf_link__destroy((*skel.get_mut()).links.handle_sys_enter);
        (*skel.get_mut()).links.handle_sys_enter =
            bpf_program__attach((*skel.get()).progs.handle_sys_enter);

        skel.detach();
    }

    /* destructor will destroy underlying skeleton */
}

fn main() {
    let opts: btf_dump_opts = unsafe { std::mem::zeroed() };
    let skel: *mut test_core_extern;
    let skel2: *mut struct_ops_module;
    let btf: *mut btf;
    let fd: c_int;

    try_skeleton_template();

    unsafe {
        /* libbpf.h */
        libbpf_set_print(None);

        /* bpf.h */
        bpf_prog_get_fd_by_id(0);

        /* btf.h */
        btf = btf__new(ptr::null(), 0);
        if libbpf_get_error(btf as *const c_void) == 0 {
            btf_dump__new(btf, dump_printf, ptr::null_mut(), &opts);
        }

        /* BPF skeleton */
        skel = test_core_extern__open_and_load();
        test_core_extern__destroy(skel);

        skel2 = struct_ops_module__open_and_load();
        struct_ops_module__destroy(skel2);

        fd = bpf_enable_stats(BPF_STATS_RUN_TIME);
        if fd < 0 {
            println!("FAILED to enable stats: {}", fd);
        } else {
            close(fd);
        }
    }

    println!("DONE!");
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
