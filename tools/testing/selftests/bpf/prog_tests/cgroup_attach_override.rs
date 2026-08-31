// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C includes:
// <test_progs.h>
// "cgroup_helpers.h"

const FOO: &[u8] = b"/foo\0";
const BAR: &[u8] = b"/foo/bar/\0";
const PING_CMD: &[u8] = b"ping -q -c1 -w1 127.0.0.1 > /dev/null\0";

static mut BPF_LOG_BUF: [::std::os::raw::c_char; BPF_LOG_BUF_SIZE as usize] =
    [0; BPF_LOG_BUF_SIZE as usize];

unsafe fn prog_load(verdict: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    let mut prog = [
        BPF_MOV64_IMM(BPF_REG_0, verdict), /* r0 = verdict */
        BPF_EXIT_INSN(),
    ];
    let insns_cnt: usize = prog.len();

    bpf_test_load_program(
        BPF_PROG_TYPE_CGROUP_SKB,
        prog.as_mut_ptr(),
        insns_cnt,
        b"GPL\0".as_ptr() as *const ::std::os::raw::c_char,
        0,
        BPF_LOG_BUF.as_mut_ptr(),
        BPF_LOG_BUF_SIZE,
    )
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_cgroup_attach_override() {
    let mut drop_prog: ::std::os::raw::c_int = -1;
    let mut allow_prog: ::std::os::raw::c_int = -1;
    let mut foo: ::std::os::raw::c_int = -1;
    let mut bar: ::std::os::raw::c_int = -1;
    let mut duration: __u32 = 0;

    'err: {
        allow_prog = prog_load(1);
        if CHECK(
            allow_prog < 0,
            b"prog_load_allow\0".as_ptr() as *const ::std::os::raw::c_char,
            b"verifier output:\n%s\n-------\n\0".as_ptr() as *const ::std::os::raw::c_char,
            BPF_LOG_BUF.as_ptr(),
        ) {
            break 'err;
        }

        drop_prog = prog_load(0);
        if CHECK(
            drop_prog < 0,
            b"prog_load_drop\0".as_ptr() as *const ::std::os::raw::c_char,
            b"verifier output:\n%s\n-------\n\0".as_ptr() as *const ::std::os::raw::c_char,
            BPF_LOG_BUF.as_ptr(),
        ) {
            break 'err;
        }

        foo = test__join_cgroup(FOO.as_ptr() as *const ::std::os::raw::c_char);
        if CHECK(
            foo < 0,
            b"cgroup_join_foo\0".as_ptr() as *const ::std::os::raw::c_char,
            b"cgroup setup failed\n\0".as_ptr() as *const ::std::os::raw::c_char,
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_attach(
                drop_prog,
                foo,
                BPF_CGROUP_INET_EGRESS,
                BPF_F_ALLOW_OVERRIDE,
            ) != 0,
            b"prog_attach_drop_foo_override\0".as_ptr() as *const ::std::os::raw::c_char,
            b"attach prog to %s failed, errno=%d\n\0".as_ptr() as *const ::std::os::raw::c_char,
            FOO.as_ptr(),
            errno,
        ) {
            break 'err;
        }

        if CHECK(
            system(PING_CMD.as_ptr() as *const ::std::os::raw::c_char) == 0,
            b"ping_fail\0".as_ptr() as *const ::std::os::raw::c_char,
            b"ping unexpectedly succeeded\n\0".as_ptr() as *const ::std::os::raw::c_char,
        ) {
            break 'err;
        }

        bar = test__join_cgroup(BAR.as_ptr() as *const ::std::os::raw::c_char);
        if CHECK(
            bar < 0,
            b"cgroup_join_bar\0".as_ptr() as *const ::std::os::raw::c_char,
            b"cgroup setup failed\n\0".as_ptr() as *const ::std::os::raw::c_char,
        ) {
            break 'err;
        }

        if CHECK(
            system(PING_CMD.as_ptr() as *const ::std::os::raw::c_char) == 0,
            b"ping_fail\0".as_ptr() as *const ::std::os::raw::c_char,
            b"ping unexpectedly succeeded\n\0".as_ptr() as *const ::std::os::raw::c_char,
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_attach(
                allow_prog,
                bar,
                BPF_CGROUP_INET_EGRESS,
                BPF_F_ALLOW_OVERRIDE,
            ) != 0,
            b"prog_attach_allow_bar_override\0".as_ptr() as *const ::std::os::raw::c_char,
            b"attach prog to %s failed, errno=%d\n\0".as_ptr() as *const ::std::os::raw::c_char,
            BAR.as_ptr(),
            errno,
        ) {
            break 'err;
        }

        if CHECK(
            system(PING_CMD.as_ptr() as *const ::std::os::raw::c_char) != 0,
            b"ping_ok\0".as_ptr() as *const ::std::os::raw::c_char,
            b"ping failed\n\0".as_ptr() as *const ::std::os::raw::c_char,
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_detach(bar, BPF_CGROUP_INET_EGRESS) != 0,
            b"prog_detach_bar\0".as_ptr() as *const ::std::os::raw::c_char,
            b"detach prog from %s failed, errno=%d\n\0".as_ptr() as *const ::std::os::raw::c_char,
            BAR.as_ptr(),
            errno,
        ) {
            break 'err;
        }

        if CHECK(
            system(PING_CMD.as_ptr() as *const ::std::os::raw::c_char) == 0,
            b"ping_fail\0".as_ptr() as *const ::std::os::raw::c_char,
            b"ping unexpectedly succeeded\n\0".as_ptr() as *const ::std::os::raw::c_char,
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_attach(
                allow_prog,
                bar,
                BPF_CGROUP_INET_EGRESS,
                BPF_F_ALLOW_OVERRIDE,
            ) != 0,
            b"prog_attach_allow_bar_override\0".as_ptr() as *const ::std::os::raw::c_char,
            b"attach prog to %s failed, errno=%d\n\0".as_ptr() as *const ::std::os::raw::c_char,
            BAR.as_ptr(),
            errno,
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_detach(foo, BPF_CGROUP_INET_EGRESS) != 0,
            b"prog_detach_foo\0".as_ptr() as *const ::std::os::raw::c_char,
            b"detach prog from %s failed, errno=%d\n\0".as_ptr() as *const ::std::os::raw::c_char,
            FOO.as_ptr(),
            errno,
        ) {
            break 'err;
        }

        if CHECK(
            system(PING_CMD.as_ptr() as *const ::std::os::raw::c_char) != 0,
            b"ping_ok\0".as_ptr() as *const ::std::os::raw::c_char,
            b"ping failed\n\0".as_ptr() as *const ::std::os::raw::c_char,
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_attach(
                allow_prog,
                bar,
                BPF_CGROUP_INET_EGRESS,
                BPF_F_ALLOW_OVERRIDE,
            ) != 0,
            b"prog_attach_allow_bar_override\0".as_ptr() as *const ::std::os::raw::c_char,
            b"attach prog to %s failed, errno=%d\n\0".as_ptr() as *const ::std::os::raw::c_char,
            BAR.as_ptr(),
            errno,
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_attach(allow_prog, bar, BPF_CGROUP_INET_EGRESS, 0) == 0,
            b"fail_prog_attach_allow_bar_none\0".as_ptr() as *const ::std::os::raw::c_char,
            b"attach prog to %s unexpectedly succeeded\n\0".as_ptr()
                as *const ::std::os::raw::c_char,
            BAR.as_ptr(),
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_detach(bar, BPF_CGROUP_INET_EGRESS) != 0,
            b"prog_detach_bar\0".as_ptr() as *const ::std::os::raw::c_char,
            b"detach prog from %s failed, errno=%d\n\0".as_ptr() as *const ::std::os::raw::c_char,
            BAR.as_ptr(),
            errno,
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_detach(foo, BPF_CGROUP_INET_EGRESS) == 0,
            b"fail_prog_detach_foo\0".as_ptr() as *const ::std::os::raw::c_char,
            b"double detach from %s unexpectedly succeeded\n\0".as_ptr()
                as *const ::std::os::raw::c_char,
            FOO.as_ptr(),
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_attach(allow_prog, foo, BPF_CGROUP_INET_EGRESS, 0) != 0,
            b"prog_attach_allow_foo_none\0".as_ptr() as *const ::std::os::raw::c_char,
            b"attach prog to %s failed, errno=%d\n\0".as_ptr() as *const ::std::os::raw::c_char,
            FOO.as_ptr(),
            errno,
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_attach(allow_prog, bar, BPF_CGROUP_INET_EGRESS, 0) == 0,
            b"fail_prog_attach_allow_bar_none\0".as_ptr() as *const ::std::os::raw::c_char,
            b"attach prog to %s unexpectedly succeeded\n\0".as_ptr()
                as *const ::std::os::raw::c_char,
            BAR.as_ptr(),
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_attach(
                allow_prog,
                bar,
                BPF_CGROUP_INET_EGRESS,
                BPF_F_ALLOW_OVERRIDE,
            ) == 0,
            b"fail_prog_attach_allow_bar_override\0".as_ptr() as *const ::std::os::raw::c_char,
            b"attach prog to %s unexpectedly succeeded\n\0".as_ptr()
                as *const ::std::os::raw::c_char,
            BAR.as_ptr(),
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_attach(
                allow_prog,
                foo,
                BPF_CGROUP_INET_EGRESS,
                BPF_F_ALLOW_OVERRIDE,
            ) == 0,
            b"fail_prog_attach_allow_foo_override\0".as_ptr() as *const ::std::os::raw::c_char,
            b"attach prog to %s unexpectedly succeeded\n\0".as_ptr()
                as *const ::std::os::raw::c_char,
            FOO.as_ptr(),
        ) {
            break 'err;
        }

        if CHECK(
            bpf_prog_attach(drop_prog, foo, BPF_CGROUP_INET_EGRESS, 0) != 0,
            b"prog_attach_drop_foo_none\0".as_ptr() as *const ::std::os::raw::c_char,
            b"attach prog to %s failed, errno=%d\n\0".as_ptr() as *const ::std::os::raw::c_char,
            FOO.as_ptr(),
            errno,
        ) {
            break 'err;
        }
    }

    close(foo);
    close(bar);
    close(allow_prog);
    close(drop_prog);
}
