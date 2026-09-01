// SPDX-License-Identifier: GPL-2.0

/* Test cases that can't load programs using libbpf and need direct
 * BPF syscall access
 *
 * C dependencies removed from executable Rust:
 * <sys/syscall.h>, <bpf/libbpf.h>, <bpf/btf.h>, "test_progs.h",
 * "test_btf.h", and "bpf/libbpf_internal.h".
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, size_of_val, zeroed};

static mut r#log: [c_char; 16 * 1024] = [0; 16 * 1024];

#[repr(C)]
struct test_btf {
    hdr: btf_header,
    types: [__u32; 15],
    strings: [c_char; 128],
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut env: test_env;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn close(fd: c_int) -> c_int;

    fn bpf_btf_load(
        btf_data: *const c_void,
        btf_size: usize,
        opts: *const bpf_btf_load_opts,
    ) -> c_int;
    fn sys_bpf_prog_load(attr: *mut bpf_attr, size: usize, log_level: c_int) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
}

/* Check that verifier rejects BPF program containing relocation
 * pointing to non-existent BTF type.
 */
unsafe fn test_bad_local_id() {
    let mut raw_btf = test_btf {
        hdr: btf_header {
            magic: BTF_MAGIC,
            version: BTF_VERSION,
            flags: 0,
            hdr_len: size_of::<btf_header>() as __u32,
            type_off: 0,
            type_len: size_of::<[__u32; 15]>() as __u32,
            str_off: (core::mem::offset_of!(test_btf, strings)
                - core::mem::offset_of!(test_btf, types)) as __u32,
            str_len: size_of::<[c_char; 128]>() as __u32,
        },
        types: {
            let mut types = [0 as __u32; 15];
            types[0] = BTF_PTR_ENC(0); /* [1] void*  */
            types[1] = BTF_TYPE_INT_ENC(1, BTF_INT_SIGNED, 0, 32, 4); /* [2] int    */
            types[2] = BTF_FUNC_PROTO_ENC(2, 1); /* [3] int (*)(void*) */
            types[3] = BTF_FUNC_PROTO_ARG_ENC(8, 1);
            types[4] = BTF_FUNC_ENC(8, 3); /* [4] FUNC 'foo' type_id=2   */
            types
        },
        strings: {
            let mut strings = [0 as c_char; 128];
            let bytes = b"\0int\0 0\0foo\0";
            let mut i = 0;
            while i < bytes.len() {
                strings[i] = bytes[i] as c_char;
                i += 1;
            }
            strings
        },
    };
    let log_level: __u32 = 1 | 2 | 4;
    let mut opts = bpf_btf_load_opts {
        sz: size_of::<bpf_btf_load_opts>(),
        log_buf: r#log.as_mut_ptr(),
        log_size: size_of_val(&r#log) as __u32,
        log_level,
        ..zeroed()
    };
    let mut insns = [
        BPF_ALU64_IMM(BPF_MOV, BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    let mut funcs = [bpf_func_info {
        insn_off: 0,
        type_id: 4,
    }];
    let mut relos = [bpf_core_relo {
        insn_off: 0,        /* patch first instruction (r0 = 0) */
        type_id: 100500,    /* !!! this type id does not exist */
        access_str_off: 6,  /* offset of "0" */
        kind: BPF_CORE_TYPE_ID_LOCAL,
    }];
    let mut attr: bpf_attr = zeroed();
    let mut saved_errno: c_int;
    let mut prog_fd: c_int = -1;
    let mut btf_fd: c_int = -1;

    btf_fd = bpf_btf_load(
        (&mut raw_btf as *mut test_btf).cast::<c_void>(),
        size_of::<test_btf>(),
        &mut opts,
    );
    saved_errno = errno;
    if btf_fd < 0 || env.verbosity > VERBOSE_NORMAL {
        printf(c"-------- BTF load log start --------\n".as_ptr());
        printf(c"%s".as_ptr(), r#log.as_ptr());
        printf(c"-------- BTF load log end ----------\n".as_ptr());
    }
    if btf_fd < 0 {
        PRINT_FAIL(
            c"bpf_btf_load() failed, errno=%d\n".as_ptr(),
            saved_errno,
        );
        return;
    }

    r#log[0] = 0;
    memset(
        (&mut attr as *mut bpf_attr).cast::<c_void>(),
        0,
        size_of::<bpf_attr>(),
    );
    attr.prog_btf_fd = btf_fd as __u32;
    attr.prog_type = BPF_TRACE_RAW_TP;
    attr.license = c"GPL".as_ptr() as __u64;
    attr.insns = (&mut insns as *mut [bpf_insn; 2]) as __u64;
    attr.insn_cnt = (size_of::<[bpf_insn; 2]>() / size_of::<bpf_insn>()) as __u32;
    attr.log_buf = r#log.as_mut_ptr() as __u64;
    attr.log_size = size_of_val(&r#log) as __u32;
    attr.log_level = log_level;
    attr.func_info = (&mut funcs as *mut [bpf_func_info; 1]) as __u64;
    attr.func_info_cnt = (size_of::<[bpf_func_info; 1]>() / size_of::<bpf_func_info>()) as __u32;
    attr.func_info_rec_size = size_of::<bpf_func_info>() as __u32;
    attr.core_relos = (&mut relos as *mut [bpf_core_relo; 1]) as __u64;
    attr.core_relo_cnt = (size_of::<[bpf_core_relo; 1]>() / size_of::<bpf_core_relo>()) as __u32;
    attr.core_relo_rec_size = size_of::<bpf_core_relo>() as __u32;
    prog_fd = sys_bpf_prog_load(&mut attr, size_of::<bpf_attr>(), 1);
    saved_errno = errno;
    if prog_fd < 0 || env.verbosity > VERBOSE_NORMAL {
        printf(c"-------- program load log start --------\n".as_ptr());
        printf(c"%s".as_ptr(), r#log.as_ptr());
        printf(c"-------- program load log end ----------\n".as_ptr());
    }
    if prog_fd >= 0 {
        PRINT_FAIL(c"sys_bpf_prog_load() expected to fail\n".as_ptr());
    } else {
        ASSERT_HAS_SUBSTR(
            r#log.as_ptr(),
            c"relo #0: bad type id 100500".as_ptr(),
            c"program load log".as_ptr(),
        );
    }

    close(prog_fd);
    close(btf_fd);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_core_reloc_raw() {
    if test__start_subtest(c"bad_local_id".as_ptr()) {
        test_bad_local_id();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
