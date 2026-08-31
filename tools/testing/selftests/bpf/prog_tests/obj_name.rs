// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include <test_progs.h>

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::{size_of, size_of_val, zeroed};

#[repr(C)]
struct Test {
    name: *const c_char,
    success: c_int,
    expected_errno: c_int,
}

extern "C" {
    static mut errno: c_int;

    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn bzero(s: *mut c_void, n: usize);
    fn syscall(number: c_long, ...) -> c_long;
    fn close(fd: c_int) -> c_int;
}

pub unsafe fn test_obj_name() {
    let tests = [
        Test {
            name: b"\0".as_ptr() as *const c_char,
            success: 1,
            expected_errno: 0,
        },
        Test {
            name: b"_123456789ABCDE\0".as_ptr() as *const c_char,
            success: 1,
            expected_errno: 0,
        },
        Test {
            name: b"_123456789ABCDEF\0".as_ptr() as *const c_char,
            success: 0,
            expected_errno: EINVAL,
        },
        Test {
            name: b"_123456789ABCD\n\0".as_ptr() as *const c_char,
            success: 0,
            expected_errno: EINVAL,
        },
    ];
    let mut prog: [bpf_insn; 2] = [
        BPF_ALU64_IMM!(BPF_MOV, BPF_REG_0, 0),
        BPF_EXIT_INSN!(),
    ];
    let mut duration: __u32 = 0;
    let mut i: c_int;

    i = 0;
    while i < tests.len() as c_int {
        let name_len: usize = strlen(tests[i as usize].name) + 1;
        let mut attr: bpf_attr = zeroed();
        let mut ncopy: usize;
        let mut fd: c_int;

        /* test different attr.prog_name during BPF_PROG_LOAD */
        ncopy = if name_len < size_of_val(&attr.prog_name) {
            name_len
        } else {
            size_of_val(&attr.prog_name)
        };
        bzero(
            &mut attr as *mut bpf_attr as *mut c_void,
            size_of::<bpf_attr>(),
        );
        attr.prog_type = BPF_PROG_TYPE_SCHED_CLS;
        attr.insn_cnt = 2;
        attr.insns = ptr_to_u64(prog.as_mut_ptr() as *mut c_void);
        attr.license = ptr_to_u64(b"\0".as_ptr() as *const c_void);
        memcpy(
            attr.prog_name.as_mut_ptr() as *mut c_void,
            tests[i as usize].name as *const c_void,
            ncopy,
        );

        fd = syscall(
            __NR_bpf as c_long,
            BPF_PROG_LOAD,
            &mut attr as *mut bpf_attr,
            size_of::<bpf_attr>(),
        ) as c_int;
        CHECK!(
            (tests[i as usize].success != 0 && fd < 0)
                || (tests[i as usize].success == 0 && fd >= 0)
                || (tests[i as usize].success == 0
                    && errno != tests[i as usize].expected_errno),
            "check-bpf-prog-name",
            "fd %d(%d) errno %d(%d)\n",
            fd,
            tests[i as usize].success,
            errno,
            tests[i as usize].expected_errno
        );

        if fd >= 0 {
            close(fd);
        }

        /* test different attr.map_name during BPF_MAP_CREATE */
        ncopy = if name_len < size_of_val(&attr.map_name) {
            name_len
        } else {
            size_of_val(&attr.map_name)
        };
        bzero(
            &mut attr as *mut bpf_attr as *mut c_void,
            size_of::<bpf_attr>(),
        );
        attr.map_type = BPF_MAP_TYPE_ARRAY;
        attr.key_size = 4;
        attr.value_size = 4;
        attr.max_entries = 1;
        attr.map_flags = 0;
        memcpy(
            attr.map_name.as_mut_ptr() as *mut c_void,
            tests[i as usize].name as *const c_void,
            ncopy,
        );
        fd = syscall(
            __NR_bpf as c_long,
            BPF_MAP_CREATE,
            &mut attr as *mut bpf_attr,
            size_of::<bpf_attr>(),
        ) as c_int;
        CHECK!(
            (tests[i as usize].success != 0 && fd < 0)
                || (tests[i as usize].success == 0 && fd >= 0)
                || (tests[i as usize].success == 0
                    && errno != tests[i as usize].expected_errno),
            "check-bpf-map-name",
            "fd %d(%d) errno %d(%d)\n",
            fd,
            tests[i as usize].success,
            errno,
            tests[i as usize].expected_errno
        );

        if fd >= 0 {
            close(fd);
        }

        i += 1;
    }
}
