// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/exceptions.c.
// C dependencies: test_progs.h, network_helpers.h, exceptions*.skel.h.

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const BPF_ANY: u64 = 0;
const EINVAL: c_int = 22;

static mut LOG_BUF: [c_char; 1024 * 1024] = [0; 1024 * 1024];

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *const c_void,
    pub data_size_in: u32,
    pub repeat: u32,
    pub retval: u32,
}

#[repr(C)]
pub struct bpf_object_open_opts {
    pub sz: usize,
    pub kernel_log_buf: *mut c_char,
    pub kernel_log_size: usize,
    pub kernel_log_level: u32,
}

#[repr(C)]
pub struct exceptions_maps {
    pub jmp_table: *mut bpf_map,
}

#[repr(C)]
pub struct exceptions_progs {
    pub exception_tail_call_target: *mut bpf_program,
    pub exception_throw_always_1: *mut bpf_program,
    pub exception_throw_always_2: *mut bpf_program,
    pub exception_throw_unwind_1: *mut bpf_program,
    pub exception_throw_unwind_2: *mut bpf_program,
    pub exception_throw_default: *mut bpf_program,
    pub exception_throw_default_value: *mut bpf_program,
    pub exception_tail_call: *mut bpf_program,
    pub exception_ext: *mut bpf_program,
    pub exception_ext_mod_cb_runtime: *mut bpf_program,
    pub exception_throw_subprog: *mut bpf_program,
    pub exception_assert_nz_gfunc: *mut bpf_program,
    pub exception_assert_zero_gfunc: *mut bpf_program,
    pub exception_assert_neg_gfunc: *mut bpf_program,
    pub exception_assert_pos_gfunc: *mut bpf_program,
    pub exception_assert_negeq_gfunc: *mut bpf_program,
    pub exception_assert_poseq_gfunc: *mut bpf_program,
    pub exception_assert_nz_gfunc_with: *mut bpf_program,
    pub exception_assert_zero_gfunc_with: *mut bpf_program,
    pub exception_assert_neg_gfunc_with: *mut bpf_program,
    pub exception_assert_pos_gfunc_with: *mut bpf_program,
    pub exception_assert_negeq_gfunc_with: *mut bpf_program,
    pub exception_assert_poseq_gfunc_with: *mut bpf_program,
    pub exception_bad_assert_nz_gfunc: *mut bpf_program,
    pub exception_bad_assert_zero_gfunc: *mut bpf_program,
    pub exception_bad_assert_neg_gfunc: *mut bpf_program,
    pub exception_bad_assert_pos_gfunc: *mut bpf_program,
    pub exception_bad_assert_negeq_gfunc: *mut bpf_program,
    pub exception_bad_assert_poseq_gfunc: *mut bpf_program,
    pub exception_bad_assert_nz_gfunc_with: *mut bpf_program,
    pub exception_bad_assert_zero_gfunc_with: *mut bpf_program,
    pub exception_bad_assert_neg_gfunc_with: *mut bpf_program,
    pub exception_bad_assert_pos_gfunc_with: *mut bpf_program,
    pub exception_bad_assert_negeq_gfunc_with: *mut bpf_program,
    pub exception_bad_assert_poseq_gfunc_with: *mut bpf_program,
    pub exception_assert_range: *mut bpf_program,
    pub exception_assert_range_with: *mut bpf_program,
    pub exception_bad_assert_range: *mut bpf_program,
    pub exception_bad_assert_range_with: *mut bpf_program,
    pub exception_throw_from_void_global: *mut bpf_program,
    pub exception_throw_stack_arg: *mut bpf_program,
    pub exception_throw_after_stack_arg: *mut bpf_program,
    pub exception_throw_subprog_stack_arg: *mut bpf_program,
    pub exception_throw_subprog_after_stack_arg: *mut bpf_program,
}

#[repr(C)]
pub struct exceptions_rodata {
    pub has_stack_arg: bool,
}

#[repr(C)]
pub struct exceptions {
    pub maps: exceptions_maps,
    pub progs: exceptions_progs,
    pub rodata: *mut exceptions_rodata,
}

#[repr(C)]
pub struct exceptions_ext_progs {
    pub pfentry: *mut bpf_program,
    pub throwing_fentry: *mut bpf_program,
    pub pfexit: *mut bpf_program,
    pub throwing_fexit: *mut bpf_program,
    pub throwing_exception_cb_extension: *mut bpf_program,
    pub throwing_extension: *mut bpf_program,
    pub pfmod_ret: *mut bpf_program,
    pub extension: *mut bpf_program,
}

#[repr(C)]
pub struct exceptions_ext {
    pub progs: exceptions_ext_progs,
}

unsafe extern "C" {
    static pkt_v4: [u8; 0];

    fn test__start_subtest(name: *const c_char) -> bool;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__set_attach_target(
        prog: *mut bpf_program,
        attach_prog_fd: c_int,
        attach_func_name: *const c_char,
    ) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn printf(fmt: *const c_char, ...) -> c_int;

    fn exceptions__open() -> *mut exceptions;
    fn exceptions__load(obj: *mut exceptions) -> c_int;
    fn exceptions__destroy(obj: *mut exceptions);

    fn exceptions_ext__open_opts(opts: *const bpf_object_open_opts) -> *mut exceptions_ext;
    fn exceptions_ext__load(obj: *mut exceptions_ext) -> c_int;
    fn exceptions_ext__destroy(obj: *mut exceptions_ext);

    fn RUN_TESTS(name: *const c_char);
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: u32, right: u32, name: *const c_char) -> bool;
}

unsafe fn test_exceptions_failure() {
    RUN_TESTS(c"exceptions_fail".as_ptr());
}

unsafe fn run_success(
    skel: *mut exceptions,
    ropts: *mut bpf_test_run_opts,
    prog: *mut bpf_program,
    prog_name: *const c_char,
    prog_run_ret_msg: *const c_char,
    prog_run_retval_msg: *const c_char,
    return_val: u32,
) {
    if !test__start_subtest(prog_name) {
        return;
    }
    let ret = bpf_prog_test_run_opts(bpf_program__fd(prog), ropts);
    ASSERT_OK(ret, prog_run_ret_msg);
    ASSERT_EQ((*ropts).retval, return_val, prog_run_retval_msg);
}

unsafe fn run_ext<F, G>(
    skel: *mut exceptions,
    eskelp: *mut *mut exceptions_ext,
    load_ret: c_int,
    attach_err: bool,
    mut expr: F,
    msg: *const c_char,
    mut after_link: G,
) -> bool
where
    F: FnMut(*mut exceptions_ext, &mut *mut bpf_program) -> bool,
    G: FnMut(*mut exceptions),
{
    let o = bpf_object_open_opts {
        sz: size_of::<bpf_object_open_opts>(),
        kernel_log_buf: LOG_BUF.as_mut_ptr(),
        kernel_log_size: size_of::<[c_char; 1024 * 1024]>(),
        kernel_log_level: 2,
    };
    exceptions_ext__destroy(*eskelp);
    *eskelp = exceptions_ext__open_opts(&o);
    let mut prog: *mut bpf_program = ptr::null_mut();
    let mut link: *mut bpf_link = ptr::null_mut();
    if !ASSERT_OK_PTR(*eskelp as *const c_void, c"exceptions_ext__open".as_ptr()) {
        return false;
    }
    if !expr(*eskelp, &mut prog) {
        return false;
    }
    ASSERT_OK_PTR(
        bpf_program__name(prog) as *const c_void,
        bpf_program__name(prog),
    );
    if !ASSERT_EQ(
        exceptions_ext__load(*eskelp) as u32,
        load_ret as u32,
        c"exceptions_ext__load".as_ptr(),
    ) {
        printf(c"%s\n".as_ptr(), LOG_BUF.as_ptr());
        return false;
    }
    if load_ret != 0 {
        if !ASSERT_OK_PTR(strstr(LOG_BUF.as_ptr(), msg) as *const c_void, c"strstr".as_ptr()) {
            printf(c"%s\n".as_ptr(), LOG_BUF.as_ptr());
            return false;
        }
    }
    if load_ret == 0 && attach_err {
        link = bpf_program__attach(prog);
        if !ASSERT_ERR_PTR(link as *const c_void, c"attach err".as_ptr()) {
            return false;
        }
    } else if load_ret == 0 {
        link = bpf_program__attach(prog);
        if !ASSERT_OK_PTR(link as *const c_void, c"attach ok".as_ptr()) {
            return false;
        }
        after_link(skel);
        bpf_link__destroy(link);
    }
    true
}

unsafe fn set_attach_target(
    prog: *mut bpf_program,
    target_prog: *mut bpf_program,
    target: *const c_char,
) -> bool {
    if !ASSERT_OK(
        bpf_program__set_attach_target(prog, bpf_program__fd(target_prog), target),
        c"set_attach_target".as_ptr(),
    ) {
        return false;
    }
    true
}

unsafe fn test_exceptions_success() {
    let mut ropts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: pkt_v4.as_ptr() as *const c_void,
        data_size_in: size_of_val(&pkt_v4) as u32,
        repeat: 1,
        retval: 0,
    };
    let mut eskel: *mut exceptions_ext = ptr::null_mut();
    let skel: *mut exceptions;
    let mut ret: c_int;

    skel = exceptions__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"exceptions__open".as_ptr()) {
        return;
    }

    'done: loop {
        ret = exceptions__load(skel);
        if !ASSERT_OK(ret, c"exceptions__load".as_ptr()) {
            break 'done;
        }

        let key: c_int = 0;
        let value: c_int = bpf_program__fd((*skel).progs.exception_tail_call_target);
        if !ASSERT_OK(
            bpf_map_update_elem(
                bpf_map__fd((*skel).maps.jmp_table),
                &key as *const c_int as *const c_void,
                &value as *const c_int as *const c_void,
                BPF_ANY,
            ),
            c"bpf_map_update_elem jmp_table".as_ptr(),
        ) {
            break 'done;
        }

        macro_rules! run_success {
            ($prog:ident, $return_val:expr) => {
                run_success(
                    skel,
                    &mut ropts,
                    (*skel).progs.$prog,
                    concat!(stringify!($prog), "\0").as_ptr() as *const c_char,
                    concat!(stringify!($prog), " prog run ret\0").as_ptr() as *const c_char,
                    concat!(stringify!($prog), " prog run retval\0").as_ptr() as *const c_char,
                    $return_val,
                )
            };
        }

        run_success!(exception_throw_always_1, 64);
        run_success!(exception_throw_always_2, 32);
        run_success!(exception_throw_unwind_1, 16);
        run_success!(exception_throw_unwind_2, 32);
        run_success!(exception_throw_default, 0);
        run_success!(exception_throw_default_value, 5);
        run_success!(exception_tail_call, 24);
        run_success!(exception_ext, 0);
        run_success!(exception_ext_mod_cb_runtime, 35);
        run_success!(exception_throw_subprog, 1);
        run_success!(exception_assert_nz_gfunc, 1);
        run_success!(exception_assert_zero_gfunc, 1);
        run_success!(exception_assert_neg_gfunc, 1);
        run_success!(exception_assert_pos_gfunc, 1);
        run_success!(exception_assert_negeq_gfunc, 1);
        run_success!(exception_assert_poseq_gfunc, 1);
        run_success!(exception_assert_nz_gfunc_with, 1);
        run_success!(exception_assert_zero_gfunc_with, 1);
        run_success!(exception_assert_neg_gfunc_with, 1);
        run_success!(exception_assert_pos_gfunc_with, 1);
        run_success!(exception_assert_negeq_gfunc_with, 1);
        run_success!(exception_assert_poseq_gfunc_with, 1);
        run_success!(exception_bad_assert_nz_gfunc, 0);
        run_success!(exception_bad_assert_zero_gfunc, 0);
        run_success!(exception_bad_assert_neg_gfunc, 0);
        run_success!(exception_bad_assert_pos_gfunc, 0);
        run_success!(exception_bad_assert_negeq_gfunc, 0);
        run_success!(exception_bad_assert_poseq_gfunc, 0);
        run_success!(exception_bad_assert_nz_gfunc_with, 100);
        run_success!(exception_bad_assert_zero_gfunc_with, 105);
        run_success!(exception_bad_assert_neg_gfunc_with, 200);
        run_success!(exception_bad_assert_pos_gfunc_with, 0);
        run_success!(exception_bad_assert_negeq_gfunc_with, 101);
        run_success!(exception_bad_assert_poseq_gfunc_with, 99);
        run_success!(exception_assert_range, 1);
        run_success!(exception_assert_range_with, 1);
        run_success!(exception_bad_assert_range, 0);
        run_success!(exception_bad_assert_range_with, 10);
        run_success!(exception_throw_from_void_global, 11);

        if (*(*skel).rodata).has_stack_arg {
            run_success!(exception_throw_stack_arg, 56);
            run_success!(exception_throw_after_stack_arg, 56);
            run_success!(exception_throw_subprog_stack_arg, 56);
            run_success!(exception_throw_subprog_after_stack_arg, 56);
        }

        macro_rules! run_ext_case {
            ($subtest:expr, $load_ret:expr, $attach_err:expr, $prog_field:ident, $target_prog:ident, $target:expr, $msg:expr, $after_link:expr) => {
                if test__start_subtest($subtest.as_ptr()) {
                    if !run_ext(
                        skel,
                        &mut eskel,
                        $load_ret,
                        $attach_err,
                        |eskel, prog| {
                            *prog = (*eskel).progs.$prog_field;
                            bpf_program__set_autoload(*prog, true);
                            set_attach_target(*prog, (*skel).progs.$target_prog, $target.as_ptr())
                        },
                        $msg.as_ptr(),
                        $after_link,
                    ) {
                        break 'done;
                    }
                }
            };
        }

        run_ext_case!(c"non-throwing fentry -> exception_cb", -EINVAL, true, pfentry, exception_ext_mod_cb_runtime, c"exception_cb_mod", c"Tracing programs cannot attach to exception callback", |_| {});
        run_ext_case!(c"throwing fentry -> exception_cb", -EINVAL, true, throwing_fentry, exception_ext_mod_cb_runtime, c"exception_cb_mod", c"Tracing programs cannot attach to exception callback", |_| {});
        run_ext_case!(c"non-throwing fexit -> exception_cb", -EINVAL, true, pfexit, exception_ext_mod_cb_runtime, c"exception_cb_mod", c"Tracing programs cannot attach to exception callback", |_| {});
        run_ext_case!(c"throwing fexit -> exception_cb", -EINVAL, true, throwing_fexit, exception_ext_mod_cb_runtime, c"exception_cb_mod", c"Tracing programs cannot attach to exception callback", |_| {});
        run_ext_case!(c"throwing extension (with custom cb) -> exception_cb", -EINVAL, true, throwing_exception_cb_extension, exception_ext_mod_cb_runtime, c"exception_cb_mod", c"Extension programs cannot attach to exception callback", |_| {});
        run_ext_case!(c"throwing extension -> global func in exception_cb", 0, false, throwing_exception_cb_extension, exception_ext_mod_cb_runtime, c"exception_cb_mod_global", c"", |skel| {
            run_success(skel, &mut ropts, (*skel).progs.exception_ext_mod_cb_runtime, c"exception_ext_mod_cb_runtime".as_ptr(), c"exception_ext_mod_cb_runtime prog run ret".as_ptr(), c"exception_ext_mod_cb_runtime prog run retval".as_ptr(), 131);
        });
        run_ext_case!(c"throwing extension (with custom cb) -> global func in exception_cb", 0, false, throwing_extension, exception_ext, c"exception_ext_global", c"", |skel| {
            run_success(skel, &mut ropts, (*skel).progs.exception_ext, c"exception_ext".as_ptr(), c"exception_ext prog run ret".as_ptr(), c"exception_ext prog run retval".as_ptr(), 128);
        });

        /* non-throwing fentry -> non-throwing subprog : OK */
        run_ext_case!(c"non-throwing fentry -> non-throwing subprog", 0, false, pfentry, exception_throw_subprog, c"subprog", c"", |_| {});
        /* throwing fentry -> non-throwing subprog : OK */
        run_ext_case!(c"throwing fentry -> non-throwing subprog", 0, false, throwing_fentry, exception_throw_subprog, c"subprog", c"", |_| {});
        /* non-throwing fentry -> throwing subprog : OK */
        run_ext_case!(c"non-throwing fentry -> throwing subprog", 0, false, pfentry, exception_throw_subprog, c"throwing_subprog", c"", |_| {});
        /* throwing fentry -> throwing subprog : OK */
        run_ext_case!(c"throwing fentry -> throwing subprog", 0, false, throwing_fentry, exception_throw_subprog, c"throwing_subprog", c"", |_| {});
        /* non-throwing fexit -> non-throwing subprog : OK */
        run_ext_case!(c"non-throwing fexit -> non-throwing subprog", 0, false, pfexit, exception_throw_subprog, c"subprog", c"", |_| {});
        /* throwing fexit -> non-throwing subprog : OK */
        run_ext_case!(c"throwing fexit -> non-throwing subprog", 0, false, throwing_fexit, exception_throw_subprog, c"subprog", c"", |_| {});
        /* non-throwing fexit -> throwing subprog : OK */
        run_ext_case!(c"non-throwing fexit -> throwing subprog", 0, false, pfexit, exception_throw_subprog, c"throwing_subprog", c"", |_| {});
        /* throwing fexit -> throwing subprog : OK */
        run_ext_case!(c"throwing fexit -> throwing subprog", 0, false, throwing_fexit, exception_throw_subprog, c"throwing_subprog", c"", |_| {});

        /* fmod_ret not allowed for subprog - Check so we remember to handle its
         * throwing specification compatibility with target when supported.
         */
        run_ext_case!(c"non-throwing fmod_ret -> non-throwing subprog", -EINVAL, true, pfmod_ret, exception_throw_subprog, c"subprog", c"can't modify return codes of BPF program", |_| {});

        /* fmod_ret not allowed for subprog - Check so we remember to handle its
         * throwing specification compatibility with target when supported.
         */
        run_ext_case!(c"non-throwing fmod_ret -> non-throwing global subprog", -EINVAL, true, pfmod_ret, exception_throw_subprog, c"global_subprog", c"can't modify return codes of BPF program", |_| {});

        /* non-throwing extension -> non-throwing subprog : BAD (!global) */
        run_ext_case!(c"non-throwing extension -> non-throwing subprog", -EINVAL, true, extension, exception_throw_subprog, c"subprog", c"subprog() is not a global function", |_| {});
        /* non-throwing extension -> throwing subprog : BAD (!global) */
        run_ext_case!(c"non-throwing extension -> throwing subprog", -EINVAL, true, extension, exception_throw_subprog, c"throwing_subprog", c"throwing_subprog() is not a global function", |_| {});
        /* non-throwing extension -> non-throwing global subprog : OK */
        run_ext_case!(c"non-throwing extension -> non-throwing subprog", 0, false, extension, exception_throw_subprog, c"global_subprog", c"", |_| {});
        /* non-throwing extension -> throwing global subprog : OK */
        run_ext_case!(c"non-throwing extension -> throwing global subprog", 0, false, extension, exception_throw_subprog, c"throwing_global_subprog", c"", |_| {});
        /* throwing extension -> throwing global subprog : OK */
        run_ext_case!(c"throwing extension -> throwing global subprog", 0, false, throwing_extension, exception_throw_subprog, c"throwing_global_subprog", c"", |_| {});
        /* throwing extension -> non-throwing global subprog : OK */
        run_ext_case!(c"throwing extension -> non-throwing global subprog", 0, false, throwing_extension, exception_throw_subprog, c"global_subprog", c"", |_| {});
        /* non-throwing extension -> main subprog : OK */
        run_ext_case!(c"non-throwing extension -> main subprog", 0, false, extension, exception_throw_subprog, c"exception_throw_subprog", c"", |_| {});
        /* throwing extension -> main subprog : OK */
        run_ext_case!(c"throwing extension -> main subprog", 0, false, throwing_extension, exception_throw_subprog, c"exception_throw_subprog", c"", |_| {});

        break 'done;
    }

    exceptions_ext__destroy(eskel);
    exceptions__destroy(skel);
}

unsafe fn test_exceptions_assertions() {
    RUN_TESTS(c"exceptions_assert".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn test_exceptions() {
    test_exceptions_success();
    test_exceptions_failure();
    test_exceptions_assertions();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
