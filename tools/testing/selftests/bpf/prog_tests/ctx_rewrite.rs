// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const MAX_PROG_TEXT_SZ: usize = 32 * 1024;
const REG_EXTENDED: c_int = 1;
const USHRT_MAX: c_int = 65535;

type u32 = c_uint;
type __u32 = c_uint;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_member {
    pub name_off: __u32,
    pub type_: __u32,
    pub offset: __u32,
}

#[repr(C)]
pub struct regex_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmatch_t {
    pub rm_so: isize,
    pub rm_eo: isize,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_insn {
    _private: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_load_opts {
    pub sz: usize,
    pub log_buf: *mut c_char,
    pub log_size: __u32,
    pub log_level: __u32,
    pub expected_attach_type: bpf_attach_type,
}

type bpf_prog_type = c_int;
type bpf_attach_type = c_int;

const BPF_PROG_TYPE_SCHED_CLS: bpf_prog_type = 3;
const BPF_PROG_TYPE_CGROUP_SOCK: bpf_prog_type = 9;
const BPF_PROG_TYPE_SOCK_OPS: bpf_prog_type = 13;
const BPF_PROG_TYPE_CGROUP_SYSCTL: bpf_prog_type = 23;
const BPF_PROG_TYPE_CGROUP_SOCKOPT: bpf_prog_type = 25;
const BPF_CGROUP_GETSOCKOPT: bpf_attach_type = 15;
const BPF_CGROUP_SETSOCKOPT: bpf_attach_type = 16;

const BPF_DW: c_int = 0x18;
const BPF_W: c_int = 0x00;
const BPF_H: c_int = 0x08;
const BPF_B: c_int = 0x10;
const BPF_REG_0: c_int = 0;
const BPF_REG_1: c_int = 1;
const BPF_REG_2: c_int = 2;

#[repr(C)]
struct st_value {
    use_: bool,
    value: c_int,
}

#[repr(C)]
struct test_case {
    name: *mut c_char,
    prog_type: bpf_prog_type,
    expected_attach_type: bpf_attach_type,
    field_offset: c_int,
    field_sz: c_int,
    st_value: st_value,
    read: *mut c_char,
    write: *mut c_char,
    write_st: *mut c_char,
    write_stx: *mut c_char,
}

#[repr(C)]
struct prog_info {
    prog_kind: *mut c_char,
    prog_type: bpf_prog_type,
    expected_attach_type: bpf_attach_type,
    prog: *mut bpf_insn,
    prog_len: u32,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut errno: c_int;

    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn isspace(c: c_int) -> c_int;
    fn isalnum(c: c_int) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fmemopen(buf: *mut c_void, size: usize, mode: *const c_char) -> *mut FILE;

    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: usize,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
    fn regerror(
        errcode: c_int,
        preg: *const regex_t,
        errbuf: *mut c_char,
        errbuf_size: usize,
    ) -> usize;
    fn regfree(preg: *mut regex_t);

    fn btf__type_by_id(btf: *mut btf, type_id: c_int) -> *const btf_type;
    fn btf__name_by_offset(btf: *mut btf, offset: __u32) -> *const c_char;
    fn btf__find_by_name(btf: *mut btf, name: *const c_char) -> c_int;
    fn btf__load_vmlinux_btf() -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf_is_struct(t: *const btf_type) -> bool;
    fn btf_is_union(t: *const btf_type) -> bool;
    fn btf_members(t: *const btf_type) -> *const btf_member;
    fn btf_vlen(t: *const btf_type) -> __u32;

    fn bpf_prog_load(
        prog_type: bpf_prog_type,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: usize,
        opts: *const bpf_prog_load_opts,
    ) -> c_int;
    fn get_xlated_program(fd: c_int, buf: *mut *mut bpf_insn, cnt: *mut __u32) -> c_int;
    fn disasm_insn(insn: *mut bpf_insn, buf: *mut c_char, buf_sz: usize) -> *mut bpf_insn;

    fn test__fail();
    fn test__start_subtest(name: *const c_char) -> bool;
    fn test__end_subtest();
    fn print_fail(format: *const c_char, ...);
}

// Includes supply offsetof(), BPF instruction construction macros, ASSERT_TRUE,
// PRINT_FAIL, max/min helpers, and kernel UAPI layouts in the original C file.
unsafe fn assert_true(cond: bool, _msg: *const c_char) {
    if !cond {
        test__fail();
    }
}

unsafe fn bpf_ldx_mem(_sz: c_int, _dst: c_int, _src: c_int, _off: c_int) -> bpf_insn {
    todo!("external BPF_LDX_MEM macro")
}

unsafe fn bpf_stx_mem(_sz: c_int, _dst: c_int, _src: c_int, _off: c_int) -> bpf_insn {
    todo!("external BPF_STX_MEM macro")
}

unsafe fn bpf_st_mem(_sz: c_int, _dst: c_int, _off: c_int, _imm: c_int) -> bpf_insn {
    todo!("external BPF_ST_MEM macro")
}

unsafe fn bpf_mov64_imm(_dst: c_int, _imm: c_int) -> bpf_insn {
    todo!("external BPF_MOV64_IMM macro")
}

unsafe fn bpf_exit_insn() -> bpf_insn {
    todo!("external BPF_EXIT_INSN macro")
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *mut c_char
    };
}

// Test cases translated from the C initializer. Field offsets and sizes depend
// on external kernel layouts and offsetof/sizeof; preserve that dependency.
unsafe fn test_cases() -> Vec<test_case> {
    vec![
        // Sign extension on s390 changes the pattern. Original C includes this
        // case only for defined(__x86_64__) || defined(__aarch64__).
        test_case { name: c!("SCHED_CLS.tstamp"), prog_type: BPF_PROG_TYPE_SCHED_CLS, expected_attach_type: 0, field_offset: todo!("offsetof(struct __sk_buff, tstamp)"), field_sz: todo!("sizeof(struct __sk_buff.tstamp)"), st_value: st_value { use_: false, value: 0 }, read: c!("r12 = *(u8 *)($ctx + sk_buff::__mono_tc_offset);if w12 & 0x4 goto pc+1;goto pc+4;if w12 & 0x3 goto pc+1;goto pc+2;$dst = 0;goto pc+1;$dst = *(u64 *)($ctx + sk_buff::tstamp);"), write: c!("r12 = *(u8 *)($ctx + sk_buff::__mono_tc_offset);if w12 & 0x4 goto pc+1;goto pc+2;w12 &= -4;*(u8 *)($ctx + sk_buff::__mono_tc_offset) = r12;*(u64 *)($ctx + sk_buff::tstamp) = $src;"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("SCHED_CLS.priority"), prog_type: BPF_PROG_TYPE_SCHED_CLS, expected_attach_type: 0, field_offset: todo!("offsetof(struct __sk_buff, priority)"), field_sz: todo!("sizeof(struct __sk_buff.priority)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u32 *)($ctx + sk_buff::priority);"), write: c!("*(u32 *)($ctx + sk_buff::priority) = $src;"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("SCHED_CLS.mark"), prog_type: BPF_PROG_TYPE_SCHED_CLS, expected_attach_type: 0, field_offset: todo!("offsetof(struct __sk_buff, mark)"), field_sz: todo!("sizeof(struct __sk_buff.mark)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u32 *)($ctx + sk_buff::mark);"), write: c!("*(u32 *)($ctx + sk_buff::mark) = $src;"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("SCHED_CLS.cb[0]"), prog_type: BPF_PROG_TYPE_SCHED_CLS, expected_attach_type: 0, field_offset: todo!("offsetof(struct __sk_buff, cb[0])"), field_sz: todo!("sizeof(struct __sk_buff.cb[0])"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u32 *)($ctx + $(sk_buff::cb + qdisc_skb_cb::data));"), write: c!("*(u32 *)($ctx + $(sk_buff::cb + qdisc_skb_cb::data)) = $src;"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("SCHED_CLS.tc_classid"), prog_type: BPF_PROG_TYPE_SCHED_CLS, expected_attach_type: 0, field_offset: todo!("offsetof(struct __sk_buff, tc_classid)"), field_sz: todo!("sizeof(struct __sk_buff.tc_classid)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u16 *)($ctx + $(sk_buff::cb + qdisc_skb_cb::tc_classid));"), write: c!("*(u16 *)($ctx + $(sk_buff::cb + qdisc_skb_cb::tc_classid)) = $src;"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("SCHED_CLS.tc_index"), prog_type: BPF_PROG_TYPE_SCHED_CLS, expected_attach_type: 0, field_offset: todo!("offsetof(struct __sk_buff, tc_index)"), field_sz: todo!("sizeof(struct __sk_buff.tc_index)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u16 *)($ctx + sk_buff::tc_index);"), write: c!("*(u16 *)($ctx + sk_buff::tc_index) = $src;"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("SCHED_CLS.queue_mapping"), prog_type: BPF_PROG_TYPE_SCHED_CLS, expected_attach_type: 0, field_offset: todo!("offsetof(struct __sk_buff, queue_mapping)"), field_sz: todo!("sizeof(struct __sk_buff.queue_mapping)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u16 *)($ctx + sk_buff::queue_mapping);"), write: ptr::null_mut(), write_st: c!("*(u16 *)($ctx + sk_buff::queue_mapping) = $src;"), write_stx: c!("if $src >= 0xffff goto pc+1;*(u16 *)($ctx + sk_buff::queue_mapping) = $src;") },
        test_case { name: c!("SCHED_CLS.queue_mapping.ushrt_max"), prog_type: BPF_PROG_TYPE_SCHED_CLS, expected_attach_type: 0, field_offset: todo!("offsetof(struct __sk_buff, queue_mapping)"), field_sz: todo!("sizeof(struct __sk_buff.queue_mapping)"), st_value: st_value { use_: true, value: USHRT_MAX }, read: ptr::null_mut(), write: ptr::null_mut(), write_st: c!("goto pc+0;"), write_stx: ptr::null_mut() },
        test_case { name: c!("CGROUP_SOCK.bound_dev_if"), prog_type: BPF_PROG_TYPE_CGROUP_SOCK, expected_attach_type: 0, field_offset: todo!("offsetof(struct bpf_sock, bound_dev_if)"), field_sz: todo!("sizeof(struct bpf_sock.bound_dev_if)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u32 *)($ctx + sock_common::skc_bound_dev_if);"), write: c!("*(u32 *)($ctx + sock_common::skc_bound_dev_if) = $src;"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("CGROUP_SOCK.mark"), prog_type: BPF_PROG_TYPE_CGROUP_SOCK, expected_attach_type: 0, field_offset: todo!("offsetof(struct bpf_sock, mark)"), field_sz: todo!("sizeof(struct bpf_sock.mark)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u32 *)($ctx + sock::sk_mark);"), write: c!("*(u32 *)($ctx + sock::sk_mark) = $src;"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("CGROUP_SOCK.priority"), prog_type: BPF_PROG_TYPE_CGROUP_SOCK, expected_attach_type: 0, field_offset: todo!("offsetof(struct bpf_sock, priority)"), field_sz: todo!("sizeof(struct bpf_sock.priority)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u32 *)($ctx + sock::sk_priority);"), write: c!("*(u32 *)($ctx + sock::sk_priority) = $src;"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("SOCK_OPS.replylong[0]"), prog_type: BPF_PROG_TYPE_SOCK_OPS, expected_attach_type: 0, field_offset: todo!("offsetof(struct bpf_sock_ops, replylong[0])"), field_sz: todo!("sizeof(struct bpf_sock_ops.replylong[0])"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u32 *)($ctx + bpf_sock_ops_kern::replylong);"), write: c!("*(u32 *)($ctx + bpf_sock_ops_kern::replylong) = $src;"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("CGROUP_SYSCTL.file_pos"), prog_type: BPF_PROG_TYPE_CGROUP_SYSCTL, expected_attach_type: 0, field_offset: todo!("offsetof(struct bpf_sysctl, file_pos)"), field_sz: todo!("sizeof(struct bpf_sysctl.file_pos)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u64 *)($ctx + bpf_sysctl_kern::ppos);$dst = *(u32 *)($dst +0);"), write: c!("*(u64 *)($ctx + bpf_sysctl_kern::tmp_reg) = r9;r9 = *(u64 *)($ctx + bpf_sysctl_kern::ppos);*(u32 *)(r9 +0) = $src;r9 = *(u64 *)($ctx + bpf_sysctl_kern::tmp_reg);"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("CGROUP_SOCKOPT.sk"), prog_type: BPF_PROG_TYPE_CGROUP_SOCKOPT, expected_attach_type: BPF_CGROUP_GETSOCKOPT, field_offset: todo!("offsetof(struct bpf_sockopt, sk)"), field_sz: todo!("sizeof(struct bpf_sockopt.sk)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u64 *)($ctx + bpf_sockopt_kern::sk);"), write: ptr::null_mut(), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("CGROUP_SOCKOPT.level"), prog_type: BPF_PROG_TYPE_CGROUP_SOCKOPT, expected_attach_type: BPF_CGROUP_SETSOCKOPT, field_offset: todo!("offsetof(struct bpf_sockopt, level)"), field_sz: todo!("sizeof(struct bpf_sockopt.level)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u32 *)($ctx + bpf_sockopt_kern::level);"), write: c!("*(u32 *)($ctx + bpf_sockopt_kern::level) = $src;"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("CGROUP_SOCKOPT.optname"), prog_type: BPF_PROG_TYPE_CGROUP_SOCKOPT, expected_attach_type: BPF_CGROUP_SETSOCKOPT, field_offset: todo!("offsetof(struct bpf_sockopt, optname)"), field_sz: todo!("sizeof(struct bpf_sockopt.optname)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u32 *)($ctx + bpf_sockopt_kern::optname);"), write: c!("*(u32 *)($ctx + bpf_sockopt_kern::optname) = $src;"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("CGROUP_SOCKOPT.optlen"), prog_type: BPF_PROG_TYPE_CGROUP_SOCKOPT, expected_attach_type: BPF_CGROUP_SETSOCKOPT, field_offset: todo!("offsetof(struct bpf_sockopt, optlen)"), field_sz: todo!("sizeof(struct bpf_sockopt.optlen)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u32 *)($ctx + bpf_sockopt_kern::optlen);"), write: c!("*(u32 *)($ctx + bpf_sockopt_kern::optlen) = $src;"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("CGROUP_SOCKOPT.retval"), prog_type: BPF_PROG_TYPE_CGROUP_SOCKOPT, expected_attach_type: BPF_CGROUP_GETSOCKOPT, field_offset: todo!("offsetof(struct bpf_sockopt, retval)"), field_sz: todo!("sizeof(struct bpf_sockopt.retval)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u64 *)($ctx + bpf_sockopt_kern::current_task);$dst = *(u64 *)($dst + task_struct::bpf_ctx);$dst = *(u32 *)($dst + bpf_cg_run_ctx::retval);"), write: c!("*(u64 *)($ctx + bpf_sockopt_kern::tmp_reg) = r9;r9 = *(u64 *)($ctx + bpf_sockopt_kern::current_task);r9 = *(u64 *)(r9 + task_struct::bpf_ctx);*(u32 *)(r9 + bpf_cg_run_ctx::retval) = $src;r9 = *(u64 *)($ctx + bpf_sockopt_kern::tmp_reg);"), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("CGROUP_SOCKOPT.optval"), prog_type: BPF_PROG_TYPE_CGROUP_SOCKOPT, expected_attach_type: BPF_CGROUP_GETSOCKOPT, field_offset: todo!("offsetof(struct bpf_sockopt, optval)"), field_sz: todo!("sizeof(struct bpf_sockopt.optval)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u64 *)($ctx + bpf_sockopt_kern::optval);"), write: ptr::null_mut(), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
        test_case { name: c!("CGROUP_SOCKOPT.optval_end"), prog_type: BPF_PROG_TYPE_CGROUP_SOCKOPT, expected_attach_type: BPF_CGROUP_GETSOCKOPT, field_offset: todo!("offsetof(struct bpf_sockopt, optval_end)"), field_sz: todo!("sizeof(struct bpf_sockopt.optval_end)"), st_value: st_value { use_: false, value: 0 }, read: c!("$dst = *(u64 *)($ctx + bpf_sockopt_kern::optval_end);"), write: ptr::null_mut(), write_st: ptr::null_mut(), write_stx: ptr::null_mut() },
    ]
}

static mut ident_regex: *mut regex_t = ptr::null_mut();
static mut field_regex: *mut regex_t = ptr::null_mut();

unsafe fn skip_space(mut str_: *mut c_char) -> *mut c_char {
    while *str_ != 0 && isspace(*str_ as c_int) != 0 {
        str_ = str_.add(1);
    }
    str_
}

unsafe fn skip_space_and_semi(mut str_: *mut c_char) -> *mut c_char {
    while *str_ != 0 && (isspace(*str_ as c_int) != 0 || *str_ == b';' as c_char) {
        str_ = str_.add(1);
    }
    str_
}

unsafe fn match_str(mut str_: *mut c_char, mut prefix: *mut c_char) -> *mut c_char {
    while *str_ != 0 && *prefix != 0 && *str_ == *prefix {
        str_ = str_.add(1);
        prefix = prefix.add(1);
    }
    if *prefix != 0 {
        return ptr::null_mut();
    }
    str_
}

unsafe fn match_number(str_: *mut c_char, num: c_int) -> *mut c_char {
    let mut next: *mut c_char = ptr::null_mut();
    let snum = strtol(str_, &mut next, 10);
    if next.offset_from(str_) == 0 || num != snum {
        return ptr::null_mut();
    }
    next
}

unsafe fn find_field_offset_aux(btf: *mut btf, btf_id: c_int, field_name: *mut c_char, off: c_int) -> c_int {
    let type_ = btf__type_by_id(btf, btf_id);
    let mut m: *const btf_member;
    let mnum: __u32;

    if type_.is_null() {
        print_fail(c!("Can't find btf_type for id %d\n"), btf_id);
        return -1;
    }

    if !btf_is_struct(type_) && !btf_is_union(type_) {
        print_fail(c!("BTF id %d is not struct or union\n"), btf_id);
        return -1;
    }

    m = btf_members(type_);
    mnum = btf_vlen(type_);

    for _i in 0..mnum {
        let mname = btf__name_by_offset(btf, (*m).name_off);

        if strcmp(mname, c!("")) == 0 {
            let msize = find_field_offset_aux(btf, (*m).type_ as c_int, field_name, off + (*m).offset as c_int);
            if msize >= 0 {
                return msize;
            }
        }

        if strcmp(mname, field_name) != 0 {
            m = m.add(1);
            continue;
        }

        return (off + (*m).offset as c_int) / 8;
    }

    -1
}

unsafe fn find_field_offset(btf: *mut btf, pattern: *mut c_char, matches: *mut regmatch_t) -> c_int {
    let type_sz = (*matches.add(1)).rm_eo - (*matches.add(1)).rm_so;
    let field_sz = (*matches.add(2)).rm_eo - (*matches.add(2)).rm_so;
    let type_ = pattern.offset((*matches.add(1)).rm_so);
    let field = pattern.offset((*matches.add(2)).rm_so);
    let mut field_str = [0 as c_char; 128];
    let mut type_str = [0 as c_char; 128];
    let btf_id: c_int;
    let field_offset: c_int;

    if type_sz as usize >= size_of_val(&type_str) {
        print_fail(c!("Malformed pattern: type ident is too long: %d\n"), type_sz as c_int);
        return -1;
    }

    if field_sz as usize >= size_of_val(&field_str) {
        print_fail(c!("Malformed pattern: field ident is too long: %d\n"), field_sz as c_int);
        return -1;
    }

    memcpy(type_str.as_mut_ptr() as *mut c_void, type_ as *const c_void, type_sz as usize);
    type_str[type_sz as usize] = 0;
    memcpy(field_str.as_mut_ptr() as *mut c_void, field as *const c_void, field_sz as usize);
    field_str[field_sz as usize] = 0;
    btf_id = btf__find_by_name(btf, type_str.as_ptr());
    if btf_id < 0 {
        print_fail(c!("No BTF info for type %s\n"), type_str.as_ptr());
        return -1;
    }

    field_offset = find_field_offset_aux(btf, btf_id, field_str.as_mut_ptr(), 0);
    if field_offset < 0 {
        print_fail(c!("No BTF info for field %s::%s\n"), type_str.as_ptr(), field_str.as_ptr());
        return -1;
    }

    field_offset
}

fn size_of_val<T>(v: &T) -> usize {
    size_of::<T>()
}

unsafe fn compile_regex(pat: *mut c_char) -> *mut regex_t {
    let re: *mut regex_t;
    let err: c_int;

    re = malloc(size_of::<regex_t>()) as *mut regex_t;
    if re.is_null() {
        print_fail(c!("Can't alloc regex\n"));
        return ptr::null_mut();
    }

    err = regcomp(re, pat, REG_EXTENDED);
    if err != 0 {
        let mut errbuf = [0 as c_char; 512];

        regerror(err, re, errbuf.as_mut_ptr(), size_of_val(&errbuf));
        print_fail(c!("Can't compile regex: %s\n"), errbuf.as_ptr());
        free(re as *mut c_void);
        return ptr::null_mut();
    }

    re
}

unsafe fn free_regex(re: *mut regex_t) {
    if re.is_null() {
        return;
    }

    regfree(re);
    free(re as *mut c_void);
}

unsafe fn max_line_len(mut str_: *mut c_char) -> u32 {
    let mut max_line: u32 = 0;
    let mut next = str_;

    while !next.is_null() {
        next = strchr(str_, b'\n' as c_int);
        if !next.is_null() {
            max_line = core::cmp::max(max_line, next.offset_from(str_) as u32);
            str_ = next.add(1);
        } else {
            max_line = core::cmp::max(max_line, strlen(str_) as u32);
        }
    }

    core::cmp::min(max_line, 60u32)
}

unsafe fn print_match_error(
    out: *mut FILE,
    pattern_origin: *mut c_char,
    text_origin: *mut c_char,
    pattern_pos: *mut c_char,
    text_pos: *mut c_char,
) {
    let mut pattern = pattern_origin;
    let mut text = text_origin;
    let middle = max_line_len(text) as c_int + 2;

    fprintf(out, c!("Can't match disassembly(left) with pattern(right):\n"));
    while *pattern != 0 || *text != 0 {
        let mut column: c_int = 0;
        let mut mark1: c_int = -1;
        let mut mark2: c_int = -1;

        while *text != 0 && *text != b'\n' as c_char {
            if text == text_pos {
                mark1 = column;
            }
            fputc(*text as c_int, out);
            text = text.add(1);
            column += 1;
        }
        if text == text_pos {
            mark1 = column;
        }

        while column < middle {
            fputc(b' ' as c_int, out);
            column += 1;
        }
        fputs(c!(";  "), out);
        column += 3;

        while *pattern != 0 && *pattern != b';' as c_char {
            if pattern == pattern_pos {
                mark2 = column;
            }
            fputc(*pattern as c_int, out);
            pattern = pattern.add(1);
            column += 1;
        }
        if pattern == pattern_pos {
            mark2 = column;
        }

        fputc(b'\n' as c_int, out);
        if *pattern != 0 {
            pattern = pattern.add(1);
        }
        if *text != 0 {
            text = text.add(1);
        }

        if mark1 > 0 || mark2 > 0 {
            column = 0;
            while column <= core::cmp::max(mark1, mark2) {
                if column == mark1 || column == mark2 {
                    fputc(b'^' as c_int, out);
                } else {
                    fputc(b' ' as c_int, out);
                }
                column += 1;
            }
            fputc(b'\n' as c_int, out);
        }
    }
}

unsafe fn match_pattern(
    btf: *mut btf,
    mut pattern: *mut c_char,
    mut text: *mut c_char,
    reg_map: *mut [*mut c_char; 2],
) -> bool {
    let pattern_origin = pattern;
    let text_origin = text;
    let mut matches = [regmatch_t { rm_so: 0, rm_eo: 0 }; 3];

    'continue_loop: loop {
        while *pattern != 0 {
            if *text == 0 {
                break 'continue_loop;
            }

            if isspace(*pattern as c_int) != 0 || *pattern == b';' as c_char {
                if isspace(*text as c_int) == 0
                    && text != text_origin
                    && isalnum(*text.offset(-1) as c_int) != 0
                {
                    break 'continue_loop;
                }
                pattern = skip_space_and_semi(pattern);
                text = skip_space(text);
                continue;
            }

            let mut i = 0usize;
            while !(*reg_map.add(i))[0].is_null() {
                let pattern_next = match_str(pattern, (*reg_map.add(i))[0]);
                if pattern_next.is_null() {
                    i += 1;
                    continue;
                }

                let text_next = match_str(text, (*reg_map.add(i))[1]);
                if text_next.is_null() {
                    break 'continue_loop;
                }

                pattern = pattern_next;
                text = text_next;
                continue 'continue_loop;
            }

            if strncmp(pattern, c!("$("), 2) == 0 {
                let group_start = pattern;
                let text_next: *mut c_char;
                let mut acc_offset = 0;

                pattern = pattern.add(2);

                loop {
                    pattern = skip_space(pattern);
                    if *pattern == 0 {
                        print_fail(c!("Unexpected end of pattern\n"));
                        break 'continue_loop;
                    }

                    if *pattern == b')' as c_char {
                        pattern = pattern.add(1);
                        break;
                    }

                    if *pattern == b'+' as c_char {
                        pattern = pattern.add(1);
                        continue;
                    }

                    printf(c!("pattern: %s\n"), pattern);
                    if regexec(field_regex, pattern, 3, matches.as_mut_ptr(), 0) != 0 {
                        print_fail(c!("Field reference expected\n"));
                        break 'continue_loop;
                    }

                    let field_offset = find_field_offset(btf, pattern, matches.as_mut_ptr());
                    if field_offset < 0 {
                        break 'continue_loop;
                    }

                    pattern = pattern.offset(matches[0].rm_eo);
                    acc_offset += field_offset;
                }

                text_next = match_number(text, acc_offset);
                if text_next.is_null() {
                    print_fail(
                        c!("No match for group offset %.*s (%d)\n"),
                        pattern.offset_from(group_start) as c_int,
                        group_start,
                        acc_offset,
                    );
                    break 'continue_loop;
                }
                text = text_next;
            }

            if regexec(field_regex, pattern, 3, matches.as_mut_ptr(), 0) == 0 {
                let field_offset = find_field_offset(btf, pattern, matches.as_mut_ptr());
                if field_offset < 0 {
                    break 'continue_loop;
                }

                let text_next = match_number(text, field_offset);
                if text_next.is_null() {
                    print_fail(c!("No match for field offset %.*s (%d)\n"), matches[0].rm_eo as c_int, pattern, field_offset);
                    break 'continue_loop;
                }

                pattern = pattern.offset(matches[0].rm_eo);
                text = text_next;
                continue;
            }

            if regexec(ident_regex, pattern, 1, matches.as_mut_ptr(), 0) == 0 {
                if strncmp(pattern, text, matches[0].rm_eo as usize) != 0 {
                    break 'continue_loop;
                }

                pattern = pattern.offset(matches[0].rm_eo);
                text = text.offset(matches[0].rm_eo);
                continue;
            }

            if *pattern != *text {
                break 'continue_loop;
            }

            pattern = pattern.add(1);
            text = text.add(1);
        }

        return true;
    }

    test__fail();
    print_match_error(stdout, pattern_origin, text_origin, pattern, text);
    false
}

unsafe fn match_program(
    btf: *mut btf,
    pinfo: *mut prog_info,
    pattern: *mut c_char,
    reg_map: *mut [*mut c_char; 2],
    skip_first_insn: bool,
) {
    let mut buf: *mut bpf_insn = ptr::null_mut();
    let mut err: c_int = 0;
    let mut prog_fd: c_int = 0;
    let mut prog_out: *mut FILE = ptr::null_mut();
    let mut insn_buf = [0 as c_char; 64];
    let mut text: *mut c_char = ptr::null_mut();
    let mut cnt: __u32 = 0;

    text = calloc(MAX_PROG_TEXT_SZ, 1) as *mut c_char;
    if text.is_null() {
        print_fail(c!("Can't allocate %d bytes\n"), MAX_PROG_TEXT_SZ as c_int);
        return;
    }

    // TODO: log level
    let mut opts: bpf_prog_load_opts = zeroed();
    opts.sz = size_of::<bpf_prog_load_opts>();
    opts.log_buf = text;
    opts.log_size = MAX_PROG_TEXT_SZ as __u32;
    opts.log_level = 1 | 2 | 4;
    opts.expected_attach_type = (*pinfo).expected_attach_type;

    prog_fd = bpf_prog_load((*pinfo).prog_type, ptr::null(), c!("GPL"), (*pinfo).prog, (*pinfo).prog_len as usize, &opts);
    if prog_fd < 0 {
        print_fail(c!("Can't load program, errno %d (%s), verifier log:\n%s\n"), errno, strerror(errno), text);
        free(text as *mut c_void);
        return;
    }

    memset(text as *mut c_void, 0, MAX_PROG_TEXT_SZ);

    err = get_xlated_program(prog_fd, &mut buf, &mut cnt);
    if err != 0 {
        print_fail(c!("Can't load back BPF program\n"));
        close(prog_fd);
        free(buf as *mut c_void);
        free(text as *mut c_void);
        return;
    }

    prog_out = fmemopen(text as *mut c_void, MAX_PROG_TEXT_SZ - 1, c!("w"));
    if prog_out.is_null() {
        print_fail(c!("Can't open memory stream\n"));
        close(prog_fd);
        free(buf as *mut c_void);
        free(text as *mut c_void);
        return;
    }

    let insn_end = buf.add(cnt as usize);
    let mut insn = buf.add(if skip_first_insn { 1 } else { 0 });
    while insn < insn_end {
        insn = disasm_insn(insn, insn_buf.as_mut_ptr(), size_of_val(&insn_buf));
        fprintf(prog_out, c!("%s\n"), insn_buf.as_ptr());
    }
    fclose(prog_out);

    assert_true(match_pattern(btf, pattern, text, reg_map), (*pinfo).prog_kind);

    if prog_fd != 0 {
        close(prog_fd);
    }
    free(buf as *mut c_void);
    free(text as *mut c_void);
}

unsafe fn run_one_testcase(btf: *mut btf, test: *mut test_case) {
    let mut pinfo: prog_info = zeroed();
    let bpf_sz: c_int;

    if !test__start_subtest((*test).name) {
        return;
    }

    match (*test).field_sz {
        8 => bpf_sz = BPF_DW,
        4 => bpf_sz = BPF_W,
        2 => bpf_sz = BPF_H,
        1 => bpf_sz = BPF_B,
        _ => {
            print_fail(c!("Unexpected field size: %d, want 8,4,2 or 1\n"), (*test).field_sz);
            return;
        }
    }

    pinfo.prog_type = (*test).prog_type;
    pinfo.expected_attach_type = (*test).expected_attach_type;

    if !(*test).read.is_null() {
        let mut ldx_prog = [
            bpf_ldx_mem(bpf_sz, BPF_REG_2, BPF_REG_1, (*test).field_offset),
            bpf_mov64_imm(BPF_REG_0, 0),
            bpf_exit_insn(),
        ];
        let mut reg_map = [[c!("$ctx"), c!("r1")], [c!("$dst"), c!("r2")], [ptr::null_mut(), ptr::null_mut()]];

        pinfo.prog_kind = c!("LDX");
        pinfo.prog = ldx_prog.as_mut_ptr();
        pinfo.prog_len = ldx_prog.len() as u32;
        match_program(btf, &mut pinfo, (*test).read, reg_map.as_mut_ptr(), false);
    }

    if !(*test).write.is_null() || !(*test).write_st.is_null() || !(*test).write_stx.is_null() {
        let mut stx_prog = [
            bpf_mov64_imm(BPF_REG_2, 0),
            bpf_stx_mem(bpf_sz, BPF_REG_1, BPF_REG_2, (*test).field_offset),
            bpf_mov64_imm(BPF_REG_0, 0),
            bpf_exit_insn(),
        ];
        let mut stx_reg_map = [[c!("$ctx"), c!("r1")], [c!("$src"), c!("r2")], [ptr::null_mut(), ptr::null_mut()]];
        let st_imm = if (*test).st_value.use_ { (*test).st_value.value } else { 42 };
        let mut st_prog = [
            bpf_st_mem(bpf_sz, BPF_REG_1, (*test).field_offset, st_imm),
            bpf_mov64_imm(BPF_REG_0, 0),
            bpf_exit_insn(),
        ];
        let mut st_reg_map = [[c!("$ctx"), c!("r1")], [c!("$src"), c!("42")], [ptr::null_mut(), ptr::null_mut()]];

        if !(*test).write.is_null() || !(*test).write_stx.is_null() {
            let pattern = if !(*test).write_stx.is_null() { (*test).write_stx } else { (*test).write };

            pinfo.prog_kind = c!("STX");
            pinfo.prog = stx_prog.as_mut_ptr();
            pinfo.prog_len = stx_prog.len() as u32;
            match_program(btf, &mut pinfo, pattern, stx_reg_map.as_mut_ptr(), true);
        }

        if !(*test).write.is_null() || !(*test).write_st.is_null() {
            let pattern = if !(*test).write_st.is_null() { (*test).write_st } else { (*test).write };

            pinfo.prog_kind = c!("ST");
            pinfo.prog = st_prog.as_mut_ptr();
            pinfo.prog_len = st_prog.len() as u32;
            match_program(btf, &mut pinfo, pattern, st_reg_map.as_mut_ptr(), false);
        }
    }

    test__end_subtest();
}

#[no_mangle]
pub unsafe extern "C" fn test_ctx_rewrite() {
    let mut btf: *mut btf = ptr::null_mut();

    field_regex = compile_regex(c!("^([[:alpha:]_][[:alnum:]_]+)::([[:alpha:]_][[:alnum:]_]+)"));
    ident_regex = compile_regex(c!("^[[:alpha:]_][[:alnum:]_]+"));
    if field_regex.is_null() || ident_regex.is_null() {
        return;
    }

    btf = btf__load_vmlinux_btf();
    if btf.is_null() {
        print_fail(c!("Can't load vmlinux BTF, errno %d (%s)\n"), errno, strerror(errno));
        btf__free(btf);
        free_regex(field_regex);
        free_regex(ident_regex);
        return;
    }

    let mut cases = test_cases();
    for i in 0..cases.len() {
        run_one_testcase(btf, &mut cases[i]);
    }

    btf__free(btf);
    free_regex(field_regex);
    free_regex(ident_regex);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
