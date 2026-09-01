// SPDX-License-Identifier: GPL-2.0
//
// Translated from testing/selftests/bpf/test_tag.c.
// C include dependencies: stdint, stdio, stdlib, ctype, time, errno, unistd,
// string, sched, limits, assert, sys/socket, linux/filter, linux/bpf,
// linux/if_alg, bpf/bpf, ../../../include/linux/filter.h, testing_helpers.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, size_of_val};
use core::ptr;

const BPF_MAXINSNS: usize = 4096;
const BPF_REG_10: c_int = 10;
const BPF_MOV: c_int = 0xb0;
const BPF_ALU64: u8 = 0x07;
const BPF_K: u8 = 0x00;
const BPF_EXIT: u8 = 0x90;
const BPF_JMP: u8 = 0x05;
const BPF_DW: u8 = 0x18;
const BPF_LD: u8 = 0x00;
const BPF_PSEUDO_MAP_FD: u8 = 1;

const BPF_PROG_TYPE_SCHED_CLS: c_int = 3;
const BPF_MAP_TYPE_HASH: c_int = 1;
const BPF_F_NO_PREALLOC: u32 = 1;
const LIBBPF_STRICT_ALL: c_int = 0x7fffffff;

const AF_ALG: c_int = 38;
const SOCK_SEQPACKET: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_insn {
    code: u8,
    regs: u8,
    off: i16,
    imm: i32,
}

impl bpf_insn {
    const fn zeroed() -> Self {
        Self {
            code: 0,
            regs: 0,
            off: 0,
            imm: 0,
        }
    }
}

#[repr(C)]
struct bpf_map_create_opts {
    sz: usize,
    btf_fd: u32,
    btf_key_type_id: u32,
    btf_value_type_id: u32,
    btf_vmlinux_value_type_id: u32,
    inner_map_fd: u32,
    map_flags: u32,
    map_extra: u64,
    numa_node: u32,
    map_ifindex: u32,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_alg {
    salg_family: u16,
    salg_type: [u8; 14],
    salg_feat: u32,
    salg_mask: u32,
    salg_name: [u8; 64],
}

type FILE = c_void;

static mut prog: [bpf_insn; BPF_MAXINSNS] = [bpf_insn::zeroed(); BPF_MAXINSNS];

unsafe extern "C" {
    fn srand(seed: c_uint);
    fn rand() -> c_int;
    fn time(tloc: *mut c_long) -> c_long;
    fn tolower(c: c_int) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: u32) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut u32) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn sched_yield() -> c_int;

    fn libbpf_set_strict_mode(mode: c_int) -> c_int;
    fn bpf_map_create(
        map_type: c_int,
        map_name: *const c_char,
        key_size: u32,
        value_size: u32,
        max_entries: u32,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
    fn bpf_test_load_program(
        prog_type: c_int,
        insns: *const bpf_insn,
        insn_cnt: c_int,
        license: *const c_char,
        kern_version: u32,
        log_buf: *mut c_char,
        log_buf_sz: u32,
    ) -> c_int;
}

const fn bpf_class(code: u8) -> u8 {
    code & 0x07
}

const fn bpf_src(code: u8) -> u8 {
    code & 0x08
}

const fn bpf_opcode(code: c_int) -> u8 {
    (code & 0xf0) as u8
}

const fn bpf_alu64_imm(op: c_int, dst: c_int, imm: c_int) -> bpf_insn {
    bpf_insn {
        code: BPF_ALU64 | bpf_opcode(op) | BPF_K,
        regs: (dst as u8) & 0x0f,
        off: 0,
        imm,
    }
}

const fn bpf_exit_insn() -> bpf_insn {
    bpf_insn {
        code: BPF_JMP | BPF_EXIT,
        regs: 0,
        off: 0,
        imm: 0,
    }
}

const fn bpf_ld_map_fd(dst: c_int, fd: c_int) -> [bpf_insn; 2] {
    [
        bpf_insn {
            code: BPF_LD | BPF_DW | BPF_K,
            regs: ((BPF_PSEUDO_MAP_FD << 4) | ((dst as u8) & 0x0f)),
            off: 0,
            imm: fd,
        },
        bpf_insn {
            code: 0,
            regs: 0,
            off: 0,
            imm: 0,
        },
    ]
}

unsafe fn bpf_gen_imm_prog(insns: c_uint, _fd_map: c_int) {
    let mut i: c_int;

    unsafe {
        srand(time(ptr::null_mut()) as c_uint);
    }
    i = 0;
    while i < insns as c_int {
        unsafe {
            prog[i as usize] = bpf_alu64_imm(BPF_MOV, i % BPF_REG_10, rand());
        }
        i += 1;
    }
    unsafe {
        prog[(i - 1) as usize] = bpf_exit_insn();
    }
}

unsafe fn bpf_gen_map_prog(insns: c_uint, fd_map: c_int) {
    let mut i: c_int = 0;
    let mut j: c_int = 0;

    while i + 1 < insns as c_int {
        let tmp = bpf_ld_map_fd(j % BPF_REG_10, fd_map);
        j += 1;

        unsafe {
            memcpy(
                prog.as_mut_ptr().add(i as usize) as *mut c_void,
                tmp.as_ptr() as *const c_void,
                size_of_val(&tmp),
            );
        }
        i += 2;
    }
    if insns % 2 == 0 {
        unsafe {
            prog[(insns - 2) as usize] = bpf_alu64_imm(BPF_MOV, i % BPF_REG_10, 42);
        }
    }
    unsafe {
        prog[(insns - 1) as usize] = bpf_exit_insn();
    }
}

unsafe fn bpf_try_load_prog(
    insns: c_int,
    fd_map: c_int,
    bpf_filler: unsafe fn(c_uint, c_int),
) -> c_int {
    let fd_prog: c_int;

    unsafe {
        bpf_filler(insns as c_uint, fd_map);
        fd_prog = bpf_test_load_program(
            BPF_PROG_TYPE_SCHED_CLS,
            prog.as_ptr(),
            insns,
            c"".as_ptr(),
            0,
            ptr::null_mut(),
            0,
        );
    }
    assert!(fd_prog > 0);
    if fd_map > 0 {
        unsafe {
            bpf_filler(insns as c_uint, 0);
        }
    }
    fd_prog
}

unsafe fn __hex2bin(mut ch: c_char) -> c_int {
    if ch >= b'0' as c_char && ch <= b'9' as c_char {
        return (ch - b'0' as c_char) as c_int;
    }
    unsafe {
        ch = tolower(ch as c_int) as c_char;
    }
    if ch >= b'a' as c_char && ch <= b'f' as c_char {
        return (ch - b'a' as c_char + 10) as c_int;
    }
    -1
}

unsafe fn hex2bin(mut dst: *mut u8, mut src: *const c_char, mut count: usize) -> c_int {
    while count != 0 {
        count -= 1;
        let hi: c_int;
        let lo: c_int;

        unsafe {
            hi = __hex2bin(*src);
            src = src.add(1);
            lo = __hex2bin(*src);
            src = src.add(1);
        }

        if hi < 0 || lo < 0 {
            return -1;
        }
        unsafe {
            *dst = ((hi << 4) | lo) as u8;
            dst = dst.add(1);
        }
    }
    0
}

unsafe fn tag_from_fdinfo(fd_prog: c_int, tag: *mut u8, len: u32) {
    const PREFIX: &[u8] = b"prog_tag:\t\0";
    const PREFIX_LEN: usize = PREFIX.len() - 1;
    let mut buff = [0 as c_char; 256];
    let mut ret: c_int = -1;
    let fp: *mut FILE;

    unsafe {
        snprintf(
            buff.as_mut_ptr(),
            size_of_val(&buff),
            c"/proc/%d/fdinfo/%d".as_ptr(),
            getpid(),
            fd_prog,
        );
        fp = fopen(buff.as_ptr(), c"r".as_ptr());
    }
    assert!(!fp.is_null());

    unsafe {
        while !fgets(buff.as_mut_ptr(), size_of_val(&buff) as c_int, fp).is_null() {
            if strncmp(buff.as_ptr(), PREFIX.as_ptr() as *const c_char, PREFIX_LEN) != 0 {
                continue;
            }
            ret = hex2bin(tag, buff.as_ptr().add(PREFIX_LEN), len as usize);
            break;
        }

        fclose(fp);
    }
    assert!(ret == 0);
}

unsafe fn tag_from_alg(mut insns: c_int, tag: *mut u8, len: u32) {
    static ALG: sockaddr_alg = sockaddr_alg {
        salg_family: AF_ALG as u16,
        salg_type: *b"hash\0\0\0\0\0\0\0\0\0\0",
        salg_feat: 0,
        salg_mask: 0,
        salg_name: *b"sha256\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    };
    let fd_base: c_int;
    let fd_alg: c_int;
    let mut ret: c_int;
    let mut size: isize;

    unsafe {
        fd_base = socket(AF_ALG, SOCK_SEQPACKET, 0);
    }
    assert!(fd_base > 0);

    unsafe {
        ret = bind(
            fd_base,
            &ALG as *const sockaddr_alg as *const sockaddr,
            size_of::<sockaddr_alg>() as u32,
        );
    }
    assert!(ret == 0);

    unsafe {
        fd_alg = accept(fd_base, ptr::null_mut(), ptr::null_mut());
    }
    assert!(fd_alg > 0);

    insns *= size_of::<bpf_insn>() as c_int;
    unsafe {
        size = write(fd_alg, prog.as_ptr() as *const c_void, insns as usize);
    }
    assert!(size == insns as isize);

    unsafe {
        size = read(fd_alg, tag as *mut c_void, len as usize);
    }
    assert!(size == len as isize);

    unsafe {
        close(fd_alg);
        close(fd_base);
    }
}

unsafe fn tag_dump(prefix: *const c_char, tag: *mut u8, len: u32) {
    let mut i: c_int;

    unsafe {
        printf(c"%s".as_ptr(), prefix);
    }
    i = 0;
    while i < len as c_int {
        unsafe {
            printf(c"%02x".as_ptr(), *tag.add(i as usize) as c_int);
        }
        i += 1;
    }
    unsafe {
        printf(c"\n".as_ptr());
    }
}

unsafe fn tag_exit_report(
    insns: c_int,
    fd_map: c_int,
    ftag: *mut u8,
    atag: *mut u8,
    len: u32,
) -> ! {
    unsafe {
        printf(
            c"Program tag mismatch for %d insns%s!\n".as_ptr(),
            insns,
            if fd_map < 0 {
                c"".as_ptr()
            } else {
                c" with map".as_ptr()
            },
        );

        tag_dump(c"  fdinfo result: ".as_ptr(), ftag, len);
        tag_dump(c"  af_alg result: ".as_ptr(), atag, len);
        exit(1);
    }
}

unsafe fn do_test(
    tests: *mut u32,
    start_insns: c_int,
    fd_map: c_int,
    bpf_filler: unsafe fn(c_uint, c_int),
) {
    let mut i: c_int;
    let mut fd_prog: c_int;

    i = start_insns;
    while i <= BPF_MAXINSNS as c_int {
        let mut ftag = [0u8; 8];
        let mut atag = [0u8; 8];

        unsafe {
            fd_prog = bpf_try_load_prog(i, fd_map, bpf_filler);
            tag_from_fdinfo(fd_prog, ftag.as_mut_ptr(), size_of_val(&ftag) as u32);
            tag_from_alg(i, atag.as_mut_ptr(), size_of_val(&atag) as u32);
            if memcmp(
                ftag.as_ptr() as *const c_void,
                atag.as_ptr() as *const c_void,
                size_of_val(&ftag),
            ) != 0
            {
                tag_exit_report(
                    i,
                    fd_map,
                    ftag.as_mut_ptr(),
                    atag.as_mut_ptr(),
                    size_of_val(&ftag) as u32,
                );
            }

            close(fd_prog);
            sched_yield();
            *tests += 1;
        }
        i += 1;
    }
}

fn main() {
    let opts = bpf_map_create_opts {
        sz: size_of::<bpf_map_create_opts>(),
        btf_fd: 0,
        btf_key_type_id: 0,
        btf_value_type_id: 0,
        btf_vmlinux_value_type_id: 0,
        inner_map_fd: 0,
        map_flags: BPF_F_NO_PREALLOC,
        map_extra: 0,
        numa_node: 0,
        map_ifindex: 0,
    };
    let mut tests: u32 = 0;
    let mut i: c_int;
    let fd_map: c_int;

    /* Use libbpf 1.0 API mode */
    unsafe {
        libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    }

    unsafe {
        fd_map = bpf_map_create(
            BPF_MAP_TYPE_HASH,
            ptr::null(),
            size_of::<c_int>() as u32,
            size_of::<c_int>() as u32,
            1,
            &opts,
        );
    }
    assert!(fd_map > 0);

    i = 0;
    while i < 5 {
        unsafe {
            do_test(&mut tests, 2, -1, bpf_gen_imm_prog);
            do_test(&mut tests, 3, fd_map, bpf_gen_map_prog);
        }
        i += 1;
    }

    unsafe {
        printf(c"test_tag: OK (%u tests)\n".as_ptr(), tests);
        close(fd_map);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
