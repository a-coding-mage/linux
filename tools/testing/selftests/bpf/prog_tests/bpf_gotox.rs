// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external expectations:
// test_progs.h, linux/* protocol headers, sys/syscall.h, bpf/bpf.h,
// and "bpf_gotox.skel.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u32 = u32;
type __u64 = u64;
type size_t = usize;

const BPF_MAP_TYPE_INSN_ARRAY: c_uint = 36;
const BPF_PROG_TYPE_RAW_TRACEPOINT: c_uint = 17;
const BPF_PSEUDO_MAP_VALUE: c_int = 2;
const BPF_REG_0: c_int = 0;
const BPF_REG_1: c_int = 1;
const BPF_ADD: c_int = 0x00;
const BPF_DW: c_int = 0x18;
const BPF_JMP: c_int = 0x05;
const BPF_JA: c_int = 0x00;
const BPF_X: c_int = 0x08;
const EACCES: c_int = 13;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_gotox_bss {
    pub ret_user: __u64,
    pub in_user: __u64,
    pub pid: c_int,
}

#[repr(C)]
pub struct bpf_gotox_data {
    pub skip: bool,
}

#[repr(C)]
pub struct bpf_gotox_progs {
    pub one_map_two_jumps: *mut bpf_program,
    pub one_switch: *mut bpf_program,
    pub one_switch_non_zero_sec_off: *mut bpf_program,
    pub two_switches: *mut bpf_program,
    pub big_jump_table: *mut bpf_program,
    pub one_jump_two_maps: *mut bpf_program,
    pub use_static_global1: *mut bpf_program,
    pub use_static_global2: *mut bpf_program,
    pub use_nonstatic_global1: *mut bpf_program,
    pub use_nonstatic_global2: *mut bpf_program,
    pub simple_test_other_sec: *mut bpf_program,
    pub use_static_global_other_sec: *mut bpf_program,
    pub use_nonstatic_global_other_sec: *mut bpf_program,
    pub load_with_nonzero_offset: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_gotox {
    pub bss: *mut bpf_gotox_bss,
    pub data: *mut bpf_gotox_data,
    pub progs: bpf_gotox_progs,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: size_t,
    pub ctx_in: *mut c_void,
    pub ctx_size_in: __u32,
    pub retval: __u32,
}

#[repr(C)]
pub struct bpf_prog_info {
    pub map_ids: c_long,
    pub nr_map_ids: __u32,
}

#[repr(C)]
pub struct bpf_map_info {
    pub type_: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_insn {
    pub code: u8,
    pub dst_src: u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_insn_array_value {
    pub orig_off: __u32,
}

extern "C" {
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn test__skip();
    fn bpf_obj_get_info_by_fd(fd: c_int, info: *mut c_void, info_len: *mut __u32) -> c_int;
    fn bpf_map_get_fd_by_id(id: __u32) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn usleep(usec: c_uint) -> c_int;
    fn bpf_map_create(
        map_type: c_uint,
        map_name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_prog_load(
        prog_type: c_uint,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *mut bpf_insn,
        insn_cnt: __u32,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_map_freeze(fd: c_int) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;
    fn bpf_gotox__open() -> *mut bpf_gotox;
    fn bpf_gotox__load(skel: *mut bpf_gotox) -> c_int;
    fn bpf_gotox__destroy(skel: *mut bpf_gotox);
    fn test__start_subtest(name: *const c_char) -> bool;
}

extern "Rust" {
    fn ASSERT_OK(ret: c_int, msg: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, msg: *const c_char) -> bool;
    fn ASSERT_GE<T: PartialOrd>(a: T, b: T, msg: *const c_char) -> bool;
    fn ASSERT_EQ<T: PartialEq>(a: T, b: T, msg: *const c_char) -> bool;
    fn ASSERT_NEQ<T: PartialEq>(a: T, b: T, msg: *const c_char) -> bool;
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

fn BPF_LD_IMM64_RAW(dst: c_int, src: c_int, imm: i32) -> [bpf_insn; 2] {
    [
        bpf_insn {
            code: 0x18,
            dst_src: ((src as u8) << 4) | (dst as u8),
            off: 0,
            imm,
        },
        bpf_insn {
            code: 0,
            dst_src: 0,
            off: 0,
            imm: 0,
        },
    ]
}

fn BPF_MOV64_IMM(dst: c_int, imm: i32) -> bpf_insn {
    bpf_insn {
        code: 0xb7,
        dst_src: dst as u8,
        off: 0,
        imm,
    }
}

fn BPF_EXIT_INSN() -> bpf_insn {
    bpf_insn {
        code: 0x95,
        dst_src: 0,
        off: 0,
        imm: 0,
    }
}

fn BPF_ALU64_IMM(op: c_int, dst: c_int, imm: i32) -> bpf_insn {
    bpf_insn {
        code: (0x07 | op) as u8,
        dst_src: dst as u8,
        off: 0,
        imm,
    }
}

fn BPF_LDX_MEM(size: c_int, dst: c_int, src: c_int, off: i16) -> bpf_insn {
    bpf_insn {
        code: (0x01 | size) as u8,
        dst_src: ((src as u8) << 4) | (dst as u8),
        off,
        imm: 0,
    }
}

fn BPF_RAW_INSN(code: c_int, dst: c_int, src: c_int, off: i16, imm: i32) -> bpf_insn {
    bpf_insn {
        code: code as u8,
        dst_src: ((src as u8) << 4) | (dst as u8),
        off,
        imm,
    }
}

unsafe fn __test_run(prog: *mut bpf_program, ctx_in: *mut c_void, ctx_size_in: size_t) {
    let mut topts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        ctx_in,
        ctx_size_in: ctx_size_in as __u32,
        retval: 0,
    };
    let err: c_int;
    let prog_fd: c_int;

    prog_fd = bpf_program__fd(prog);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c_str!("test_run_opts err"));
}

unsafe fn __subtest(skel: *mut bpf_gotox, check: unsafe fn(*mut bpf_gotox)) {
    if (*(*skel).data).skip {
        test__skip();
    } else {
        check(skel);
    }
}

unsafe fn check_simple(
    skel: *mut bpf_gotox,
    prog: *mut bpf_program,
    mut ctx_in: __u64,
    expected: __u64,
) {
    (*(*skel).bss).ret_user = 0;

    __test_run(
        prog,
        &mut ctx_in as *mut __u64 as *mut c_void,
        size_of::<__u64>(),
    );

    if !ASSERT_EQ((*(*skel).bss).ret_user, expected, c_str!("skel->bss->ret_user")) {
        return;
    }
}

unsafe fn check_simple_fentry(
    skel: *mut bpf_gotox,
    _prog: *mut bpf_program,
    ctx_in: __u64,
    expected: __u64,
) {
    (*(*skel).bss).in_user = ctx_in;
    (*(*skel).bss).ret_user = 0;

    /* trigger */
    usleep(1);

    if !ASSERT_EQ((*(*skel).bss).ret_user, expected, c_str!("skel->bss->ret_user")) {
        return;
    }
}

/* validate that for two loads of the same jump table libbpf generates only one map */
unsafe fn check_one_map_two_jumps(skel: *mut bpf_gotox) {
    let mut prog_info: bpf_prog_info = zeroed();
    let mut map_info: bpf_map_info = zeroed();
    let mut len: __u32;
    let mut map_ids: [__u32; 16] = [0; 16];
    let prog_fd: c_int;
    let mut map_fd: c_int;
    let mut ret: c_int;
    let mut i: c_int;
    let mut seen = false;

    prog_info.map_ids = map_ids.as_mut_ptr() as c_long;
    prog_info.nr_map_ids = map_ids.len() as __u32;
    prog_fd = bpf_program__fd((*skel).progs.one_map_two_jumps);
    if !ASSERT_GE(prog_fd, 0, c_str!("bpf_program__fd(one_map_two_jumps)")) {
        return;
    }

    len = size_of::<bpf_prog_info>() as __u32;
    ret = bpf_obj_get_info_by_fd(
        prog_fd,
        &mut prog_info as *mut bpf_prog_info as *mut c_void,
        &mut len,
    );
    if !ASSERT_OK(ret, c_str!("bpf_obj_get_info_by_fd(prog_fd)")) {
        return;
    }

    i = 0;
    while i < prog_info.nr_map_ids as c_int {
        map_fd = bpf_map_get_fd_by_id(map_ids[i as usize]);
        if !ASSERT_GE(map_fd, 0, c_str!("bpf_map_get_fd_by_id")) {
            return;
        }

        len = size_of::<bpf_map_info>() as __u32;
        map_info = zeroed();
        ret = bpf_obj_get_info_by_fd(
            map_fd,
            &mut map_info as *mut bpf_map_info as *mut c_void,
            &mut len,
        );
        if !ASSERT_OK(ret, c_str!("bpf_obj_get_info_by_fd(map_fd)")) {
            close(map_fd);
            return;
        }

        if map_info.type_ == BPF_MAP_TYPE_INSN_ARRAY {
            if !ASSERT_EQ(seen, false, c_str!("more than one INSN_ARRAY map")) {
                close(map_fd);
                return;
            }
            seen = true;
        }
        close(map_fd);
        i += 1;
    }

    ASSERT_EQ(seen, true, c_str!("no INSN_ARRAY map"));
}

unsafe fn check_one_switch(skel: *mut bpf_gotox) {
    let in_: [__u64; 7] = [0, 1, 2, 3, 4, 5, 77];
    let out: [__u64; 7] = [2, 3, 4, 5, 7, 19, 19];
    let mut i: c_int;

    i = 0;
    while i < in_.len() as c_int {
        check_simple(skel, (*skel).progs.one_switch, in_[i as usize], out[i as usize]);
        i += 1;
    }
}

unsafe fn check_one_switch_non_zero_sec_off(skel: *mut bpf_gotox) {
    let in_: [__u64; 7] = [0, 1, 2, 3, 4, 5, 77];
    let out: [__u64; 7] = [2, 3, 4, 5, 7, 19, 19];
    let mut i: c_int;

    i = 0;
    while i < in_.len() as c_int {
        check_simple(
            skel,
            (*skel).progs.one_switch_non_zero_sec_off,
            in_[i as usize],
            out[i as usize],
        );
        i += 1;
    }
}

unsafe fn check_two_switches(skel: *mut bpf_gotox) {
    let in_: [__u64; 7] = [0, 1, 2, 3, 4, 5, 77];
    let out: [__u64; 7] = [103, 104, 107, 205, 115, 1019, 1019];
    let mut i: c_int;

    i = 0;
    while i < in_.len() as c_int {
        check_simple(skel, (*skel).progs.two_switches, in_[i as usize], out[i as usize]);
        i += 1;
    }
}

unsafe fn check_big_jump_table(skel: *mut bpf_gotox) {
    let in_: [__u64; 7] = [0, 11, 27, 31, 22, 45, 99];
    let out: [__u64; 7] = [2, 3, 4, 5, 19, 19, 19];
    let mut i: c_int;

    i = 0;
    while i < in_.len() as c_int {
        check_simple(skel, (*skel).progs.big_jump_table, in_[i as usize], out[i as usize]);
        i += 1;
    }
}

unsafe fn check_one_jump_two_maps(skel: *mut bpf_gotox) {
    let in_: [__u64; 7] = [0, 1, 2, 3, 4, 5, 77];
    let out: [__u64; 7] = [12, 15, 7, 15, 12, 15, 15];
    let mut i: c_int;

    i = 0;
    while i < in_.len() as c_int {
        check_simple(skel, (*skel).progs.one_jump_two_maps, in_[i as usize], out[i as usize]);
        i += 1;
    }
}

unsafe fn check_static_global(skel: *mut bpf_gotox) {
    let in_: [__u64; 7] = [0, 1, 2, 3, 4, 5, 77];
    let out: [__u64; 7] = [2, 3, 4, 5, 7, 19, 19];
    let mut i: c_int;

    i = 0;
    while i < in_.len() as c_int {
        check_simple(skel, (*skel).progs.use_static_global1, in_[i as usize], out[i as usize]);
        i += 1;
    }
    i = 0;
    while i < in_.len() as c_int {
        check_simple(skel, (*skel).progs.use_static_global2, in_[i as usize], out[i as usize]);
        i += 1;
    }
}

unsafe fn check_nonstatic_global(skel: *mut bpf_gotox) {
    let in_: [__u64; 7] = [0, 1, 2, 3, 4, 5, 77];
    let out: [__u64; 7] = [2, 3, 4, 5, 7, 19, 19];
    let mut i: c_int;

    i = 0;
    while i < in_.len() as c_int {
        check_simple(skel, (*skel).progs.use_nonstatic_global1, in_[i as usize], out[i as usize]);
        i += 1;
    }

    i = 0;
    while i < in_.len() as c_int {
        check_simple(skel, (*skel).progs.use_nonstatic_global2, in_[i as usize], out[i as usize]);
        i += 1;
    }
}

unsafe fn check_other_sec(skel: *mut bpf_gotox) {
    let link: *mut bpf_link;
    let in_: [__u64; 7] = [0, 1, 2, 3, 4, 5, 77];
    let out: [__u64; 7] = [2, 3, 4, 5, 7, 19, 19];
    let mut i: c_int;

    link = bpf_program__attach((*skel).progs.simple_test_other_sec);
    if !ASSERT_OK_PTR(link, c_str!("link")) {
        return;
    }

    i = 0;
    while i < in_.len() as c_int {
        check_simple_fentry(
            skel,
            (*skel).progs.simple_test_other_sec,
            in_[i as usize],
            out[i as usize],
        );
        i += 1;
    }

    bpf_link__destroy(link);
}

unsafe fn check_static_global_other_sec(skel: *mut bpf_gotox) {
    let link: *mut bpf_link;
    let in_: [__u64; 7] = [0, 1, 2, 3, 4, 5, 77];
    let out: [__u64; 7] = [2, 3, 4, 5, 7, 19, 19];
    let mut i: c_int;

    link = bpf_program__attach((*skel).progs.use_static_global_other_sec);
    if !ASSERT_OK_PTR(link, c_str!("link")) {
        return;
    }

    i = 0;
    while i < in_.len() as c_int {
        check_simple_fentry(
            skel,
            (*skel).progs.use_static_global_other_sec,
            in_[i as usize],
            out[i as usize],
        );
        i += 1;
    }

    bpf_link__destroy(link);
}

unsafe fn check_nonstatic_global_other_sec(skel: *mut bpf_gotox) {
    let link: *mut bpf_link;
    let in_: [__u64; 7] = [0, 1, 2, 3, 4, 5, 77];
    let out: [__u64; 7] = [2, 3, 4, 5, 7, 19, 19];
    let mut i: c_int;

    link = bpf_program__attach((*skel).progs.use_nonstatic_global_other_sec);
    if !ASSERT_OK_PTR(link, c_str!("link")) {
        return;
    }

    i = 0;
    while i < in_.len() as c_int {
        check_simple_fentry(
            skel,
            (*skel).progs.use_nonstatic_global_other_sec,
            in_[i as usize],
            out[i as usize],
        );
        i += 1;
    }

    bpf_link__destroy(link);
}

/*
 * The following subtests do not use skeleton rather than to check
 * if the test should be skipped.
 */

unsafe fn create_jt_map(max_entries: __u32) -> c_int {
    let map_name = c_str!("jt");
    let key_size: __u32 = 4;
    let value_size: __u32 = size_of::<bpf_insn_array_value>() as __u32;

    bpf_map_create(
        BPF_MAP_TYPE_INSN_ARRAY,
        map_name,
        key_size,
        value_size,
        max_entries,
        ptr::null(),
    )
}

unsafe fn prog_load(insns: *mut bpf_insn, insn_cnt: __u32) -> c_int {
    bpf_prog_load(
        BPF_PROG_TYPE_RAW_TRACEPOINT,
        ptr::null(),
        c_str!("GPL"),
        insns,
        insn_cnt,
        ptr::null(),
    )
}

unsafe fn __check_ldimm64_off_prog_load(max_entries: __u32, off: __u32) -> c_int {
    let ld = BPF_LD_IMM64_RAW(BPF_REG_1, BPF_PSEUDO_MAP_VALUE, 0);
    let mut insns: [bpf_insn; 4] = [
        ld[0],
        ld[1],
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    let map_fd: c_int;
    let ret: c_int;

    map_fd = create_jt_map(max_entries);
    if !ASSERT_GE(map_fd, 0, c_str!("create_jt_map")) {
        return -1;
    }
    if !ASSERT_EQ(bpf_map_freeze(map_fd), 0, c_str!("bpf_map_freeze")) {
        close(map_fd);
        return -1;
    }

    insns[0].imm = map_fd;
    insns[1].imm = off as i32;

    ret = prog_load(insns.as_mut_ptr(), insns.len() as __u32);
    close(map_fd);
    ret
}

/*
 * Check that loads from an instruction array map are only allowed with offsets
 * which are multiples of 8 and do not point to outside of the map.
 */
unsafe fn check_ldimm64_off_load(_skel: *mut bpf_gotox) {
    const MAX_ENTRIES: __u32 = 10;
    let mut prog_fd: c_int;
    let mut off: __u32;

    off = 0;
    while off < MAX_ENTRIES {
        prog_fd = __check_ldimm64_off_prog_load(MAX_ENTRIES, off * 8);
        if !ASSERT_GE(prog_fd, 0, c_str!("__check_ldimm64_off_prog_load")) {
            return;
        }
        close(prog_fd);
        off += 1;
    }

    prog_fd = __check_ldimm64_off_prog_load(MAX_ENTRIES, 7 /* not a multiple of 8 */);
    if !ASSERT_EQ(
        prog_fd,
        -EACCES,
        c_str!("__check_ldimm64_off_prog_load: should be -EACCES"),
    ) {
        close(prog_fd);
        return;
    }

    prog_fd = __check_ldimm64_off_prog_load(MAX_ENTRIES, MAX_ENTRIES * 8 /* too large */);
    if !ASSERT_EQ(
        prog_fd,
        -EACCES,
        c_str!("__check_ldimm64_off_prog_load: should be -EACCES"),
    ) {
        close(prog_fd);
        return;
    }
}

unsafe fn __check_ldimm64_gotox_prog_load(
    insns: *mut bpf_insn,
    insn_cnt: __u32,
    off1: c_int,
    off2: c_int,
    off3: c_int,
) -> c_int {
    let values: [__u32; 6] = [5, 7, 9, 11, 13, 15];
    let max_entries: __u32 = values.len() as __u32;
    let mut val: bpf_insn_array_value = zeroed();
    let map_fd: c_int;
    let ret: c_int;
    let mut i: c_int;

    map_fd = create_jt_map(max_entries);
    if !ASSERT_GE(map_fd, 0, c_str!("create_jt_map")) {
        return -1;
    }

    i = 0;
    while i < max_entries as c_int {
        val.orig_off = values[i as usize];
        if !ASSERT_EQ(
            bpf_map_update_elem(
                map_fd,
                &i as *const c_int as *const c_void,
                &val as *const bpf_insn_array_value as *const c_void,
                0,
            ),
            0,
            c_str!("bpf_map_update_elem"),
        ) {
            close(map_fd);
            return -1;
        }
        i += 1;
    }

    if !ASSERT_EQ(bpf_map_freeze(map_fd), 0, c_str!("bpf_map_freeze")) {
        close(map_fd);
        return -1;
    }

    /* r1 = &map + offset1 */
    (*insns.add(0)).imm = map_fd;
    (*insns.add(1)).imm = off1;

    /* r1 += off2 */
    (*insns.add(2)).imm = off2;

    /* r1 = *(r1 + off3) */
    (*insns.add(3)).off = off3 as i16;

    ret = prog_load(insns, insn_cnt);
    close(map_fd);
    ret
}

unsafe fn allow_offsets(
    insns: *mut bpf_insn,
    insn_cnt: __u32,
    off1: c_int,
    off2: c_int,
    off3: c_int,
) {
    let mut topts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        ctx_in: ptr::null_mut(),
        ctx_size_in: 0,
        retval: 0,
    };
    let prog_fd: c_int;
    let err: c_int;
    let mut s: [c_char; 128] = [0; 128];

    prog_fd = __check_ldimm64_gotox_prog_load(insns, insn_cnt, off1, off2, off3);
    snprintf(
        s.as_mut_ptr(),
        s.len(),
        c_str!("__check_ldimm64_gotox_prog_load(%d,%d,%d)"),
        off1,
        off2,
        off3,
    );
    if !ASSERT_GE(prog_fd, 0, s.as_ptr()) {
        return;
    }

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c_str!("test_run_opts err")) {
        close(prog_fd);
        return;
    }

    if !ASSERT_EQ(
        topts.retval,
        ((off1 + off2 + off3) / 8) as __u32,
        c_str!("test_run_opts retval"),
    ) {
        close(prog_fd);
        return;
    }

    close(prog_fd);
}

unsafe fn reject_offsets(
    insns: *mut bpf_insn,
    insn_cnt: __u32,
    off1: c_int,
    off2: c_int,
    off3: c_int,
) {
    let prog_fd: c_int;

    prog_fd = __check_ldimm64_gotox_prog_load(insns, insn_cnt, off1, off2, off3);
    if !ASSERT_EQ(
        prog_fd,
        -EACCES,
        c_str!("__check_ldimm64_gotox_prog_load"),
    ) {
        close(prog_fd);
    }
}

/*
 * Verify a bit more complex programs which include indirect jumps
 * and with jump tables loaded with a non-zero offset
 */
unsafe fn check_ldimm64_off_gotox(_skel: *mut bpf_gotox) {
    let ld = BPF_LD_IMM64_RAW(BPF_REG_1, BPF_PSEUDO_MAP_VALUE, 0);
    let mut insns: [bpf_insn; 16] = [
        /*
         * The following instructions perform an indirect jump to
         * labels below. Thus valid offsets in the map are {0,...,5}.
         * The program rewrites the offsets in the instructions below:
         *     r1 = &map + offset1
         *     r1 += offset2
         *     r1 = *(r1 + offset3)
         *     gotox r1
         */
        ld[0],
        ld[1],
        BPF_ALU64_IMM(BPF_ADD, BPF_REG_1, 0),
        BPF_LDX_MEM(BPF_DW, BPF_REG_1, BPF_REG_1, 0),
        BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_1, 0, 0, 0),
        /* case 0: */
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
        /* case 1: */
        BPF_MOV64_IMM(BPF_REG_0, 1),
        BPF_EXIT_INSN(),
        /* case 2: */
        BPF_MOV64_IMM(BPF_REG_0, 2),
        BPF_EXIT_INSN(),
        /* case 3: */
        BPF_MOV64_IMM(BPF_REG_0, 3),
        BPF_EXIT_INSN(),
        /* case 4: */
        BPF_MOV64_IMM(BPF_REG_0, 4),
        BPF_EXIT_INSN(),
        /* default: */
        BPF_MOV64_IMM(BPF_REG_0, 5),
        BPF_EXIT_INSN(),
    ];
    let mut off1: c_int;
    let mut off2: c_int;
    let mut off3: c_int;

    /* allow all combinations off1 + off2 + off3 < 6 */
    off1 = 0;
    while off1 < 6 {
        off2 = 0;
        while off1 + off2 < 6 {
            off3 = 0;
            while off1 + off2 + off3 < 6 {
                allow_offsets(
                    insns.as_mut_ptr(),
                    insns.len() as __u32,
                    off1 * 8,
                    off2 * 8,
                    off3 * 8,
                );
                off3 += 1;
            }
            off2 += 1;
        }
        off1 += 1;
    }

    /* allow for some offsets to be negative */
    allow_offsets(insns.as_mut_ptr(), insns.len() as __u32, 8 * 3, 0, -(8 * 3));
    allow_offsets(insns.as_mut_ptr(), insns.len() as __u32, 8 * 3, -(8 * 3), 0);
    allow_offsets(insns.as_mut_ptr(), insns.len() as __u32, 0, 8 * 3, -(8 * 3));
    allow_offsets(insns.as_mut_ptr(), insns.len() as __u32, 8 * 4, 0, -(8 * 2));
    allow_offsets(insns.as_mut_ptr(), insns.len() as __u32, 8 * 4, -(8 * 2), 0);
    allow_offsets(insns.as_mut_ptr(), insns.len() as __u32, 0, 8 * 4, -(8 * 2));

    /* disallow negative sums of offsets */
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, 8 * 3, 0, -(8 * 4));
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, 8 * 3, -(8 * 4), 0);
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, 0, 8 * 3, -(8 * 4));

    /* disallow the off1 to be negative in any case */
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, -8 * 1, 0, 0);
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, -8 * 1, 8 * 1, 0);
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, -8 * 1, 8 * 1, 8 * 1);

    /* reject off1 + off2 + off3 >= 6 */
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, 8 * 3, 8 * 3, 8 * 0);
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, 8 * 7, 8 * 0, 8 * 0);
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, 8 * 0, 8 * 7, 8 * 0);
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, 8 * 3, 8 * 0, 8 * 3);
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, 8 * 0, 8 * 3, 8 * 3);

    /* reject (off1 + off2) % 8 != 0, off3 % 8 != 0 */
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, 3, 3, 0);
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, 7, 0, 0);
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, 0, 7, 0);
    reject_offsets(insns.as_mut_ptr(), insns.len() as __u32, 0, 0, 7);
}

unsafe fn check_ldimm64_off_gotox_llvm(skel: *mut bpf_gotox) {
    let in_: [__u64; 5] = [0, 1, 2, 3, 4];
    let out: [__u64; 5] = [1, 1, 5, 1, 1];
    let mut i: c_int;

    i = 0;
    while i < in_.len() as c_int {
        check_simple(
            skel,
            (*skel).progs.load_with_nonzero_offset,
            in_[i as usize],
            out[i as usize],
        );
        i += 1;
    }
}

pub unsafe extern "C" fn test_bpf_gotox() {
    let skel: *mut bpf_gotox;
    let ret: c_int;

    skel = bpf_gotox__open();
    if !ASSERT_NEQ(skel, ptr::null_mut(), c_str!("bpf_gotox__open")) {
        return;
    }

    ret = bpf_gotox__load(skel);
    if !ASSERT_OK(ret, c_str!("bpf_gotox__load")) {
        return;
    }

    (*(*skel).bss).pid = getpid();

    if test__start_subtest(c_str!("one-switch")) {
        __subtest(skel, check_one_switch);
    }

    if test__start_subtest(c_str!("one-switch-non-zero-sec-offset")) {
        __subtest(skel, check_one_switch_non_zero_sec_off);
    }

    if test__start_subtest(c_str!("two-switches")) {
        __subtest(skel, check_two_switches);
    }

    if test__start_subtest(c_str!("big-jump-table")) {
        __subtest(skel, check_big_jump_table);
    }

    if test__start_subtest(c_str!("static-global")) {
        __subtest(skel, check_static_global);
    }

    if test__start_subtest(c_str!("nonstatic-global")) {
        __subtest(skel, check_nonstatic_global);
    }

    if test__start_subtest(c_str!("other-sec")) {
        __subtest(skel, check_other_sec);
    }

    if test__start_subtest(c_str!("static-global-other-sec")) {
        __subtest(skel, check_static_global_other_sec);
    }

    if test__start_subtest(c_str!("nonstatic-global-other-sec")) {
        __subtest(skel, check_nonstatic_global_other_sec);
    }

    if test__start_subtest(c_str!("one-jump-two-maps")) {
        __subtest(skel, check_one_jump_two_maps);
    }

    if test__start_subtest(c_str!("one-map-two-jumps")) {
        __subtest(skel, check_one_map_two_jumps);
    }

    if test__start_subtest(c_str!("check-ldimm64-off")) {
        __subtest(skel, check_ldimm64_off_load);
    }

    if test__start_subtest(c_str!("check-ldimm64-off-gotox")) {
        __subtest(skel, check_ldimm64_off_gotox);
    }

    if test__start_subtest(c_str!("check-ldimm64-off-gotox-llvm")) {
        __subtest(skel, check_ldimm64_off_gotox_llvm);
    }

    bpf_gotox__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
