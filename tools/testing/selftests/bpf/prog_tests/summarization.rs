// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external Rust dependencies:
// "bpf/libbpf.h", "summarization_freplace.skel.h",
// "summarization.skel.h", <test_progs.h>

use core::ffi::{c_char, c_int, c_void};

const VERBOSE_SUPER: c_int = 2;
const VERBOSE_VERY: c_int = 3;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct test_env {
    pub verbosity: c_int,
}

#[repr(C)]
pub struct bpf_object_open_opts {
    pub sz: usize,
    pub object_name: *const c_char,
    pub relaxed_maps: bool,
    pub pin_root_path: *const c_char,
    pub kconfig: *const c_char,
    pub btf_custom_path: *const c_char,
    pub kernel_log_buf: *mut c_char,
    pub kernel_log_size: usize,
    pub kernel_log_level: c_int,
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct summarization {
    pub obj: *mut bpf_object,
}

#[repr(C)]
pub struct summarization_freplace {
    pub obj: *mut bpf_object,
}

#[repr(C)]
struct main_entry {
    main: *const c_char,
    to_be_replaced: *const c_char,
    has_side_effect: bool,
}

#[repr(C)]
struct replacement_entry {
    func: *const c_char,
    has_side_effect: bool,
    err_msg: *const c_char,
}

unsafe extern "C" {
    static mut env: test_env;
    static mut stdout: *mut c_void;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__set_autoattach(prog: *mut bpf_program, autoattach: bool);
    fn bpf_program__set_attach_target(
        prog: *mut bpf_program,
        attach_prog_fd: c_int,
        attach_func_name: *const c_char,
    ) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;

    fn summarization__open_opts(opts: *const bpf_object_open_opts) -> *mut summarization;
    fn summarization__load(obj: *mut summarization) -> c_int;
    fn summarization__destroy(obj: *mut summarization);

    fn summarization_freplace__open_opts(
        opts: *const bpf_object_open_opts,
    ) -> *mut summarization_freplace;
    fn summarization_freplace__load(obj: *mut summarization_freplace) -> c_int;
    fn summarization_freplace__destroy(obj: *mut summarization_freplace);

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_HAS_SUBSTR(str_: *const c_char, substr: *const c_char, name: *const c_char)
        -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe fn print_verifier_log(log: *const c_char) {
    if env.verbosity >= VERBOSE_VERY {
        fprintf(
            stdout,
            c"VERIFIER LOG:\n=============\n%s=============\n".as_ptr(),
            log,
        );
    }
}

unsafe fn test_aux(
    main_prog_name: *const c_char,
    to_be_replaced: *const c_char,
    replacement: *const c_char,
    expect_load: bool,
    err_msg: *const c_char,
) {
    let mut freplace: *mut summarization_freplace = core::ptr::null_mut();
    let mut freplace_prog: *mut bpf_program = core::ptr::null_mut();
    let mut main_prog: *mut bpf_program = core::ptr::null_mut();
    let mut opts: bpf_object_open_opts = core::mem::zeroed();
    opts.sz = core::mem::size_of::<bpf_object_open_opts>();
    let mut main: *mut summarization = core::ptr::null_mut();
    let mut log = [0 as c_char; 16 * 1024];
    let mut err: c_int;

    opts.kernel_log_buf = log.as_mut_ptr();
    opts.kernel_log_size = core::mem::size_of_val(&log);
    if env.verbosity >= VERBOSE_SUPER {
        opts.kernel_log_level = 1 | 2 | 4;
    }
    main = summarization__open_opts(&opts);
    if !ASSERT_OK_PTR(main as *const c_void, c"summarization__open".as_ptr()) {
        goto_out(freplace, main);
        return;
    }
    main_prog = bpf_object__find_program_by_name((*main).obj, main_prog_name);
    if !ASSERT_OK_PTR(main_prog as *const c_void, c"main_prog".as_ptr()) {
        goto_out(freplace, main);
        return;
    }
    bpf_program__set_autoload(main_prog, true);
    err = summarization__load(main);
    print_verifier_log(log.as_ptr());
    if !ASSERT_OK(err, c"summarization__load".as_ptr()) {
        goto_out(freplace, main);
        return;
    }
    freplace = summarization_freplace__open_opts(&opts);
    if !ASSERT_OK_PTR(
        freplace as *const c_void,
        c"summarization_freplace__open".as_ptr(),
    ) {
        goto_out(freplace, main);
        return;
    }
    freplace_prog = bpf_object__find_program_by_name((*freplace).obj, replacement);
    if !ASSERT_OK_PTR(freplace_prog as *const c_void, c"freplace_prog".as_ptr()) {
        goto_out(freplace, main);
        return;
    }
    bpf_program__set_autoload(freplace_prog, true);
    bpf_program__set_autoattach(freplace_prog, true);
    bpf_program__set_attach_target(freplace_prog, bpf_program__fd(main_prog), to_be_replaced);
    err = summarization_freplace__load(freplace);
    print_verifier_log(log.as_ptr());

    /* The might_sleep extension doesn't work yet as sleepable calls are not
     * allowed, but preserve the check in case it's supported later and then
     * this particular combination can be enabled.
     */
    if strcmp(c"might_sleep".as_ptr(), replacement) == 0 && err != 0 {
        ASSERT_HAS_SUBSTR(
            log.as_ptr(),
            c"sleepable helper bpf_copy_from_user#".as_ptr(),
            c"error log".as_ptr(),
        );
        ASSERT_EQ(err, -EINVAL, c"err".as_ptr());
        test__skip();
        goto_out(freplace, main);
        return;
    }

    if expect_load {
        ASSERT_OK(err, c"summarization_freplace__load".as_ptr());
    } else {
        ASSERT_ERR(err, c"summarization_freplace__load".as_ptr());
        ASSERT_HAS_SUBSTR(log.as_ptr(), err_msg, c"error log".as_ptr());
    }

    goto_out(freplace, main);
}

unsafe fn goto_out(freplace: *mut summarization_freplace, main: *mut summarization) {
    summarization_freplace__destroy(freplace);
    summarization__destroy(main);
}

/* There are two global subprograms in both summarization.skel.h:
 * - one changes packet data;
 * - another does not.
 * It is ok to freplace subprograms that change packet data with those
 * that either do or do not. It is only ok to freplace subprograms
 * that do not change packet data with those that do not as well.
 * The below tests check outcomes for each combination of such freplace.
 * Also test a case when main subprogram itself is replaced and is a single
 * subprogram in a program.
 *
 * This holds for might_sleep programs. It is ok to replace might_sleep with
 * might_sleep and with does_not_sleep, but does_not_sleep cannot be replaced
 * with might_sleep.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_summarization_freplace() {
    let mains: [[main_entry; 4]; 2] = [
        [
            main_entry {
                main: c"main_changes_with_subprogs".as_ptr(),
                to_be_replaced: c"changes_pkt_data".as_ptr(),
                has_side_effect: true,
            },
            main_entry {
                main: c"main_changes_with_subprogs".as_ptr(),
                to_be_replaced: c"does_not_change_pkt_data".as_ptr(),
                has_side_effect: false,
            },
            main_entry {
                main: c"main_changes".as_ptr(),
                to_be_replaced: c"main_changes".as_ptr(),
                has_side_effect: true,
            },
            main_entry {
                main: c"main_does_not_change".as_ptr(),
                to_be_replaced: c"main_does_not_change".as_ptr(),
                has_side_effect: false,
            },
        ],
        [
            main_entry {
                main: c"main_might_sleep_with_subprogs".as_ptr(),
                to_be_replaced: c"might_sleep".as_ptr(),
                has_side_effect: true,
            },
            main_entry {
                main: c"main_might_sleep_with_subprogs".as_ptr(),
                to_be_replaced: c"does_not_sleep".as_ptr(),
                has_side_effect: false,
            },
            main_entry {
                main: c"main_might_sleep".as_ptr(),
                to_be_replaced: c"main_might_sleep".as_ptr(),
                has_side_effect: true,
            },
            main_entry {
                main: c"main_does_not_sleep".as_ptr(),
                to_be_replaced: c"main_does_not_sleep".as_ptr(),
                has_side_effect: false,
            },
        ],
    ];
    let pkt_err: *const c_char = c"Extension program changes packet data".as_ptr();
    let slp_err: *const c_char = c"Extension program may sleep".as_ptr();
    let replacements: [[replacement_entry; 2]; 2] = [
        [
            replacement_entry {
                func: c"changes_pkt_data".as_ptr(),
                has_side_effect: true,
                err_msg: pkt_err,
            },
            replacement_entry {
                func: c"does_not_change_pkt_data".as_ptr(),
                has_side_effect: false,
                err_msg: pkt_err,
            },
        ],
        [
            replacement_entry {
                func: c"might_sleep".as_ptr(),
                has_side_effect: true,
                err_msg: slp_err,
            },
            replacement_entry {
                func: c"does_not_sleep".as_ptr(),
                has_side_effect: false,
                err_msg: slp_err,
            },
        ],
    ];
    let mut buf = [0 as c_char; 64];

    for t in 0..2 {
        for i in 0..mains.len() {
            for j in 0..replacements.len() {
                snprintf(
                    buf.as_mut_ptr(),
                    core::mem::size_of_val(&buf),
                    c"%s_with_%s".as_ptr(),
                    mains[t][i].to_be_replaced,
                    replacements[t][j].func,
                );
                if !test__start_subtest(buf.as_ptr()) {
                    continue;
                }
                test_aux(
                    mains[t][i].main,
                    mains[t][i].to_be_replaced,
                    replacements[t][j].func,
                    mains[t][i].has_side_effect || !replacements[t][j].has_side_effect,
                    replacements[t][j].err_msg,
                );
            }
        }
    }
}
