// SPDX-License-Identifier: GPL-2.0

// Translated from C. External declarations below correspond to symbols and
// macros supplied by test_progs.h, bpf/bpf.h, errno.h, sys/mman.h, and unistd.h.

const ARENA_PAGES: u32 = 32;

static mut log_buf: [libc::c_char; 16384] = [0; 16384];

#[repr(C)]
pub struct bpf_insn {
    pub code: u8,
    pub dst_reg_src_reg: u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
pub struct bpf_map_create_opts {
    pub map_flags: u32,
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub log_buf: *mut libc::c_char,
    pub log_size: u32,
    pub log_level: u32,
}

unsafe extern "C" {
    static mut errno: libc::c_int;

    static BPF_REG_0: u32;
    static BPF_REG_1: u32;
    static BPF_PSEUDO_MAP_VALUE: u32;
    static BPF_F_MMAPABLE: u32;
    static BPF_MAP_TYPE_ARENA: u32;
    static BPF_PROG_TYPE_RAW_TRACEPOINT: u32;
    static EOPNOTSUPP: libc::c_int;
    static PROT_READ: libc::c_int;
    static PROT_WRITE: libc::c_int;
    static MAP_SHARED: libc::c_int;
    static MAP_FAILED: *mut libc::c_void;

    fn getpagesize() -> libc::c_int;
    fn bpf_map_create(
        map_type: u32,
        map_name: *const libc::c_char,
        key_size: u32,
        value_size: u32,
        max_entries: u32,
        opts: *const bpf_map_create_opts,
    ) -> libc::c_int;
    fn mmap(
        addr: *mut libc::c_void,
        length: libc::size_t,
        prot: libc::c_int,
        flags: libc::c_int,
        fd: libc::c_int,
        offset: libc::off_t,
    ) -> *mut libc::c_void;
    fn bpf_prog_load(
        prog_type: u32,
        prog_name: *const libc::c_char,
        license: *const libc::c_char,
        insns: *const bpf_insn,
        insn_cnt: libc::size_t,
        opts: *const bpf_prog_load_opts,
    ) -> libc::c_int;
    fn snprintf(
        str_: *mut libc::c_char,
        size: libc::size_t,
        format: *const libc::c_char,
        ...
    ) -> libc::c_int;
    fn munmap(addr: *mut libc::c_void, length: libc::size_t) -> libc::c_int;
    fn close(fd: libc::c_int) -> libc::c_int;

    fn test__skip();
    fn test__start_subtest(name: *const libc::c_char) -> bool;
    fn ASSERT_GE(a: libc::c_int, b: libc::c_int, name: *const libc::c_char) -> bool;
    fn ASSERT_NEQ(
        a: *mut libc::c_void,
        b: *mut libc::c_void,
        name: *const libc::c_char,
    ) -> bool;
    fn ASSERT_LT(a: libc::c_int, b: libc::c_int, name: *const libc::c_char) -> bool;
    fn ASSERT_HAS_SUBSTR(
        str_: *const libc::c_char,
        substr: *const libc::c_char,
        name: *const libc::c_char,
    ) -> bool;

    fn BPF_LD_IMM64_RAW(dst: u32, src: u32, imm: i32) -> bpf_insn;
    fn BPF_MOV64_IMM(dst: u32, imm: i32) -> bpf_insn;
    fn BPF_EXIT_INSN() -> bpf_insn;
}

unsafe fn test_arena_direct_value_one_past_end() {
    let mut expected: [libc::c_char; 128] = [0; 128];
    let arena_sz: u32 = ARENA_PAGES.wrapping_mul(getpagesize() as u32);
    let mut insns: [bpf_insn; 3] = [
        BPF_LD_IMM64_RAW(BPF_REG_1, BPF_PSEUDO_MAP_VALUE, 0),
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    let mut map_opts: bpf_map_create_opts = core::mem::zeroed();
    let mut prog_opts: bpf_prog_load_opts = core::mem::zeroed();
    let arena: *mut libc::c_void;
    let map_fd: libc::c_int;
    let prog_fd: libc::c_int;

    map_opts.map_flags = BPF_F_MMAPABLE;
    prog_opts.log_buf = log_buf.as_mut_ptr();
    prog_opts.log_size = core::mem::size_of_val(&log_buf) as u32;
    prog_opts.log_level = 1;

    map_fd = bpf_map_create(
        BPF_MAP_TYPE_ARENA,
        c"arena_direct_value".as_ptr(),
        0,
        0,
        ARENA_PAGES,
        &map_opts,
    );
    if map_fd < 0 {
        if errno == EOPNOTSUPP {
            test__skip();
            return;
        }
        ASSERT_GE(map_fd, 0, c"bpf_map_create".as_ptr());
        return;
    }

    arena = mmap(
        core::ptr::null_mut(),
        arena_sz as libc::size_t,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        map_fd,
        0,
    );
    if !ASSERT_NEQ(arena, MAP_FAILED, c"arena_mmap".as_ptr()) {
        close(map_fd);
        return;
    }

    insns[0].imm = map_fd;
    insns[1].imm = arena_sz as i32;

    prog_fd = bpf_prog_load(
        BPF_PROG_TYPE_RAW_TRACEPOINT,
        c"arena_direct_value".as_ptr(),
        c"GPL".as_ptr(),
        insns.as_ptr(),
        insns.len(),
        &prog_opts,
    );
    if !ASSERT_LT(prog_fd, 0, c"prog_load".as_ptr()) {
        close(prog_fd);
        if arena != MAP_FAILED {
            munmap(arena, arena_sz as libc::size_t);
        }
        close(map_fd);
        return;
    }

    snprintf(
        expected.as_mut_ptr(),
        core::mem::size_of_val(&expected) as libc::size_t,
        c"invalid access to map value pointer, value_size=0 off=%u".as_ptr(),
        arena_sz,
    );
    ASSERT_HAS_SUBSTR(log_buf.as_ptr(), expected.as_ptr(), c"verifier_log".as_ptr());

    if arena != MAP_FAILED {
        munmap(arena, arena_sz as libc::size_t);
    }
    close(map_fd);
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_arena_direct_value() {
    if test__start_subtest(c"one_past_end".as_ptr()) {
        test_arena_direct_value_one_past_end();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
