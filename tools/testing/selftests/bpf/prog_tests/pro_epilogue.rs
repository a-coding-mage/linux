// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C includes translated as external Rust dependencies:
// <test_progs.h>
// "pro_epilogue.skel.h"
// "epilogue_tailcall.skel.h"
// "pro_epilogue_goto_start.skel.h"
// "epilogue_exit.skel.h"
// "pro_epilogue_with_kfunc.skel.h"

#[repr(C)]
struct st_ops_args {
    a: __u64,
}

unsafe fn test_tailcall() {
    let mut topts: bpf_test_run_opts = core::mem::zeroed();
    let mut skel: *mut epilogue_tailcall;
    let mut args: st_ops_args = core::mem::zeroed();
    let err: i32;
    let prog_fd: i32;

    skel = epilogue_tailcall__open_and_load();
    if !ASSERT_OK_PTR(skel, "epilogue_tailcall__open_and_load") {
        return;
    }

    topts.ctx_in = &mut args as *mut st_ops_args as *mut _;
    topts.ctx_size_in = core::mem::size_of_val(&args) as _;

    (*skel).links.epilogue_tailcall =
        bpf_map__attach_struct_ops((*skel).maps.epilogue_tailcall);
    if !ASSERT_OK_PTR((*skel).links.epilogue_tailcall, "attach_struct_ops") {
        epilogue_tailcall__destroy(skel);
        return;
    }

    /* Both test_epilogue_tailcall and test_epilogue_subprog are
     * patched with epilogue. When syscall_epilogue_tailcall()
     * is run, test_epilogue_tailcall() is triggered.
     * It executes a tail call and control is transferred to
     * test_epilogue_subprog(). Only test_epilogue_subprog()
     * does args->a += 1, thus final args.a value of 10001
     * guarantees that only the epilogue of the
     * test_epilogue_subprog is executed.
     */
    core::ptr::write_bytes(
        &mut args as *mut st_ops_args as *mut u8,
        0,
        core::mem::size_of_val(&args),
    );
    prog_fd = bpf_program__fd((*skel).progs.syscall_epilogue_tailcall);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    ASSERT_EQ(args.a, 10001, "args.a");
    ASSERT_EQ(topts.retval, 10001 * 2, "topts.retval");

    epilogue_tailcall__destroy(skel);
}

pub unsafe fn test_pro_epilogue() {
    RUN_TESTS(pro_epilogue);
    RUN_TESTS(pro_epilogue_goto_start);
    RUN_TESTS(epilogue_exit);
    RUN_TESTS(pro_epilogue_with_kfunc);
    if test__start_subtest("tailcall") {
        test_tailcall();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
