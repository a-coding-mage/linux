// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2013 Google Inc.
 * Author: Willem de Bruijn (willemb@google.com)
 *
 * A basic test of packet socket fanout behavior.
 *
 * Control:
 * - create fanout fails as expected with illegal flag combinations
 * - join   fanout fails as expected with diverging types or flags
 *
 * Datapath:
 *   Open a pair of packet sockets and a pair of INET sockets, send a known
 *   number of packets across the two INET sockets and count the number of
 *   packets enqueued onto the two packet sockets.
 *
 *   The test currently runs for
 *   - PACKET_FANOUT_HASH
 *   - PACKET_FANOUT_HASH with PACKET_FANOUT_FLAG_ROLLOVER
 *   - PACKET_FANOUT_LB
 *   - PACKET_FANOUT_CPU
 *   - PACKET_FANOUT_ROLLOVER
 *   - PACKET_FANOUT_CBPF
 *   - PACKET_FANOUT_EBPF
 *
 * Todo:
 * - functionality: PACKET_FANOUT_FLAG_DEFRAG
 */

/* _GNU_SOURCE: for sched_setaffinity in the C source. */
/* C dependencies: arpa/inet.h, errno.h, fcntl.h, linux/unistd.h,
 * linux/filter.h, linux/bpf.h, linux/if_packet.h, net/if.h,
 * net/ethernet.h, netinet/ip.h, netinet/udp.h, poll.h, sched.h,
 * stdint.h, stdio.h, stdlib.h, string.h, sys/mman.h, sys/socket.h,
 * sys/ioctl.h, sys/stat.h, sys/types.h, unistd.h, psock_lib.h,
 * kselftest.h.
 */

use std::ffi::c_void;
use std::mem;
use std::ptr;

const RING_NUM_FRAMES: i32 = 20;

const ETH_P_IP: i32 = 0x0800;
const SIOCGIFFLAGS: libc::c_ulong = 0x8913;
const SIOCSIFFLAGS: libc::c_ulong = 0x8914;
const IFF_UP: libc::c_short = 0x1;
const SOL_PACKET: libc::c_int = 263;
const PACKET_FANOUT: libc::c_int = 18;
const PACKET_RX_RING: libc::c_int = 5;
const PACKET_VERSION: libc::c_int = 10;
const PACKET_FANOUT_DATA: libc::c_int = 22;
const PACKET_FANOUT_HASH: u16 = 0;
const PACKET_FANOUT_LB: u16 = 1;
const PACKET_FANOUT_CPU: u16 = 2;
const PACKET_FANOUT_ROLLOVER: u16 = 3;
const PACKET_FANOUT_CBPF: u16 = 6;
const PACKET_FANOUT_EBPF: u16 = 7;
const PACKET_FANOUT_FLAG_ROLLOVER: u16 = 0x1000;
const PACKET_FANOUT_FLAG_UNIQUEID: u16 = 0x2000;
const PACKET_FANOUT_FLAG_DEFRAG: u16 = 0x8000;
const TPACKET_V2: libc::c_int = 1;
const TP_STATUS_USER: libc::c_uint = 1;

const BPF_LD: u16 = 0x00;
const BPF_LDX: u16 = 0x01;
const BPF_ALU: u16 = 0x04;
const BPF_JMP: u16 = 0x05;
const BPF_RET: u16 = 0x06;
const BPF_ALU64: u16 = 0x07;
const BPF_W: u16 = 0x00;
const BPF_B: u16 = 0x10;
const BPF_ABS: u16 = 0x20;
const BPF_MEM: u16 = 0x60;
const BPF_K: u16 = 0x00;
const BPF_A: u16 = 0x10;
const BPF_X: u16 = 0x08;
const BPF_JA: u16 = 0x00;
const BPF_JEQ: u16 = 0x10;
const BPF_JGE: u16 = 0x30;
const BPF_MOV: u16 = 0xb0;
const BPF_EXIT: u16 = 0x90;
const BPF_PROG_LOAD: libc::c_int = 5;
const BPF_PROG_TYPE_SOCKET_FILTER: u32 = 1;
const __NR_BPF: libc::c_long = 321;

#[repr(C)]
struct Ifreq {
    ifr_name: [libc::c_char; libc::IFNAMSIZ],
    ifr_flags: libc::c_short,
    _pad: [u8; 22],
}

#[repr(C)]
struct FanoutArgs {
    id: u16,
    type_flags: u16,
    max_num_members: u32,
}

#[repr(C)]
struct SockFilter {
    code: u16,
    jt: u8,
    jf: u8,
    k: u32,
}

#[repr(C)]
struct SockFprog {
    len: libc::c_ushort,
    filter: *mut SockFilter,
}

#[repr(C)]
struct BpfInsn {
    code: u8,
    dst_src: u8,
    off: i16,
    imm: i32,
}

impl BpfInsn {
    const fn new(code: u16, dst_reg: u8, src_reg: u8, off: i16, imm: i32) -> Self {
        Self {
            code: code as u8,
            dst_src: (dst_reg & 0x0f) | ((src_reg & 0x0f) << 4),
            off,
            imm,
        }
    }
}

#[repr(C)]
struct BpfAttrProgLoad {
    prog_type: u32,
    insn_cnt: u32,
    insns: u64,
    license: u64,
    log_level: u32,
    log_size: u32,
    log_buf: u64,
    kern_version: u32,
    prog_flags: u32,
    prog_name: [u8; 16],
    prog_ifindex: u32,
    expected_attach_type: u32,
}

#[repr(C)]
struct TpacketReq {
    tp_block_size: libc::c_uint,
    tp_block_nr: libc::c_uint,
    tp_frame_size: libc::c_uint,
    tp_frame_nr: libc::c_uint,
}

#[repr(C)]
struct Tpacket2Hdr {
    tp_status: libc::c_uint,
    tp_len: libc::c_uint,
    tp_snaplen: libc::c_uint,
    tp_mac: libc::c_ushort,
    tp_net: libc::c_ushort,
    tp_sec: libc::c_uint,
    tp_nsec: libc::c_uint,
    tp_vlan_tci: libc::c_ushort,
    tp_vlan_tpid: libc::c_ushort,
    tp_padding: [u8; 4],
}

unsafe extern "C" {
    fn pair_udp_setfilter(fd: libc::c_int);
    fn pair_udp_open(fds: *mut libc::c_int, port: libc::c_ushort);
    fn pair_udp_send(fds: *mut libc::c_int, num: libc::c_int);
    fn pair_udp_send_char(fds: *mut libc::c_int, num: libc::c_int, chr: libc::c_uchar);

    static PORT_BASE: libc::c_ushort;
    static DATA_LEN: libc::c_int;
    static DATA_CHAR: libc::c_uchar;
    static DATA_CHAR_1: libc::c_uchar;
}

static mut CFG_MAX_NUM_MEMBERS: u32 = 0;

fn htons(v: u16) -> u16 {
    v.to_be()
}

unsafe fn loopback_set_up_down(state_up: libc::c_int) {
    let mut ifreq: Ifreq = mem::zeroed();
    let fd: libc::c_int;
    let mut err: libc::c_int;

    fd = libc::socket(libc::AF_PACKET, libc::SOCK_RAW, 0);
    if fd < 0 {
        libc::perror(c"socket loopback".as_ptr());
        libc::exit(1);
    }
    libc::strcpy(ifreq.ifr_name.as_mut_ptr(), c"lo".as_ptr());
    err = libc::ioctl(fd, SIOCGIFFLAGS, &mut ifreq);
    if err != 0 {
        libc::perror(c"SIOCGIFFLAGS".as_ptr());
        libc::exit(1);
    }
    if state_up != ((ifreq.ifr_flags & IFF_UP) != 0) as libc::c_int {
        ifreq.ifr_flags ^= IFF_UP;
        err = libc::ioctl(fd, SIOCSIFFLAGS, &ifreq);
        if err != 0 {
            libc::perror(c"SIOCSIFFLAGS".as_ptr());
            libc::exit(1);
        }
    }
    libc::close(fd);
}

/* Open a socket in a given fanout mode.
 * @return -1 if mode is bad, a valid socket otherwise */
unsafe fn sock_fanout_open(typeflags: u16, group_id: u16) -> libc::c_int {
    let mut addr: libc::sockaddr_ll = mem::zeroed();
    let mut args: FanoutArgs = mem::zeroed();
    let fd: libc::c_int;
    let mut val: libc::c_int = 0;
    let err: libc::c_int;

    fd = libc::socket(libc::PF_PACKET, libc::SOCK_RAW, 0);
    if fd < 0 {
        libc::perror(c"socket packet".as_ptr());
        libc::exit(1);
    }

    pair_udp_setfilter(fd);

    addr.sll_family = libc::AF_PACKET as libc::c_ushort;
    addr.sll_protocol = htons(ETH_P_IP as u16);
    addr.sll_ifindex = libc::if_nametoindex(c"lo".as_ptr()) as libc::c_int;
    if addr.sll_ifindex == 0 {
        libc::perror(c"if_nametoindex".as_ptr());
        libc::exit(1);
    }
    if libc::bind(
        fd,
        &addr as *const _ as *const libc::sockaddr,
        mem::size_of_val(&addr) as libc::socklen_t,
    ) != 0
    {
        libc::perror(c"bind packet".as_ptr());
        libc::exit(1);
    }

    if CFG_MAX_NUM_MEMBERS != 0 {
        args.id = group_id;
        args.type_flags = typeflags;
        args.max_num_members = CFG_MAX_NUM_MEMBERS;
        err = libc::setsockopt(
            fd,
            SOL_PACKET,
            PACKET_FANOUT,
            &args as *const _ as *const c_void,
            mem::size_of_val(&args) as libc::socklen_t,
        );
    } else {
        val = (((typeflags as libc::c_int) << 16) | group_id as libc::c_int) as libc::c_int;
        err = libc::setsockopt(
            fd,
            SOL_PACKET,
            PACKET_FANOUT,
            &val as *const _ as *const c_void,
            mem::size_of_val(&val) as libc::socklen_t,
        );
    }
    if err != 0 {
        if libc::close(fd) != 0 {
            libc::perror(c"close packet".as_ptr());
            libc::exit(1);
        }
        return -1;
    }

    fd
}

unsafe fn sock_fanout_set_cbpf(fd: libc::c_int) {
    let mut bpf_filter = [
        SockFilter { code: BPF_LD | BPF_B | BPF_ABS, jt: 0, jf: 0, k: 80 }, /* ldb [80] */
        SockFilter { code: BPF_RET | BPF_A, jt: 0, jf: 0, k: 0 },           /* ret A */
    ];
    let mut bpf_prog = SockFprog {
        filter: ptr::null_mut(),
        len: 0,
    };

    bpf_prog.filter = bpf_filter.as_mut_ptr();
    bpf_prog.len = bpf_filter.len() as libc::c_ushort;

    if libc::setsockopt(
        fd,
        SOL_PACKET,
        PACKET_FANOUT_DATA,
        &bpf_prog as *const _ as *const c_void,
        mem::size_of_val(&bpf_prog) as libc::socklen_t,
    ) != 0
    {
        libc::perror(c"fanout data cbpf".as_ptr());
        libc::exit(1);
    }
}

unsafe fn sock_fanout_getopts(fd: libc::c_int, typeflags: *mut u16, group_id: *mut u16) {
    let mut sockopt: libc::c_int = 0;
    let mut sockopt_len: libc::socklen_t = mem::size_of_val(&sockopt) as libc::socklen_t;

    if libc::getsockopt(
        fd,
        SOL_PACKET,
        PACKET_FANOUT,
        &mut sockopt as *mut _ as *mut c_void,
        &mut sockopt_len,
    ) != 0
    {
        libc::perror(c"failed to getsockopt".as_ptr());
        libc::exit(1);
    }
    *typeflags = (sockopt >> 16) as u16;
    *group_id = (sockopt & 0xfffff) as u16;
}

unsafe fn sock_fanout_set_ebpf(fd: libc::c_int) {
    static mut LOG_BUF: [libc::c_char; 65536] = [0; 65536];

    let len_off: libc::c_int = 0; /* __builtin_offsetof(struct __sk_buff, len) */
    let prog = [
        BpfInsn::new(BPF_ALU64 | BPF_MOV | BPF_X, 6, 1, 0, 0),
        BpfInsn::new(BPF_LDX | BPF_W | BPF_MEM, 0, 6, len_off as i16, 0),
        BpfInsn::new(BPF_JMP | BPF_JGE | BPF_K, 0, 0, 1, DATA_LEN),
        BpfInsn::new(BPF_JMP | BPF_JA | BPF_K, 0, 0, 4, 0),
        BpfInsn::new(BPF_LD | BPF_B | BPF_ABS, 0, 0, 0, 0x50),
        BpfInsn::new(BPF_JMP | BPF_JEQ | BPF_K, 0, 0, 2, DATA_CHAR as i32),
        BpfInsn::new(BPF_JMP | BPF_JEQ | BPF_K, 0, 0, 1, DATA_CHAR_1 as i32),
        BpfInsn::new(BPF_ALU | BPF_MOV | BPF_K, 0, 0, 0, 0),
        BpfInsn::new(BPF_JMP | BPF_EXIT, 0, 0, 0, 0),
    ];
    let mut attr: BpfAttrProgLoad = mem::zeroed();
    let pfd: libc::c_int;

    attr.prog_type = BPF_PROG_TYPE_SOCKET_FILTER;
    attr.insns = prog.as_ptr() as u64;
    attr.insn_cnt = prog.len() as u32;
    attr.license = c"GPL".as_ptr() as u64;
    attr.log_buf = LOG_BUF.as_mut_ptr() as u64;
    attr.log_size = mem::size_of_val(&LOG_BUF) as u32;
    attr.log_level = 1;

    pfd = libc::syscall(
        __NR_BPF,
        BPF_PROG_LOAD,
        &attr as *const _,
        mem::size_of_val(&attr),
    ) as libc::c_int;
    if pfd < 0 {
        libc::perror(c"bpf".as_ptr());
        libc::fprintf(
            libc::stderr,
            c"bpf verifier:\n%s\n".as_ptr(),
            LOG_BUF.as_ptr(),
        );
        libc::exit(1);
    }

    if libc::setsockopt(
        fd,
        SOL_PACKET,
        PACKET_FANOUT_DATA,
        &pfd as *const _ as *const c_void,
        mem::size_of_val(&pfd) as libc::socklen_t,
    ) != 0
    {
        libc::perror(c"fanout data ebpf".as_ptr());
        libc::exit(1);
    }

    if libc::close(pfd) != 0 {
        libc::perror(c"close ebpf".as_ptr());
        libc::exit(1);
    }
}

unsafe fn sock_fanout_open_ring(fd: libc::c_int) -> *mut libc::c_char {
    let mut req = TpacketReq {
        tp_block_size: libc::getpagesize() as libc::c_uint,
        tp_frame_size: libc::getpagesize() as libc::c_uint,
        tp_block_nr: RING_NUM_FRAMES as libc::c_uint,
        tp_frame_nr: RING_NUM_FRAMES as libc::c_uint,
    };
    let mut ring: *mut libc::c_char;
    let val: libc::c_int = TPACKET_V2;

    if libc::setsockopt(
        fd,
        SOL_PACKET,
        PACKET_VERSION,
        &val as *const _ as *const c_void,
        mem::size_of_val(&val) as libc::socklen_t,
    ) != 0
    {
        libc::perror(c"packetsock ring setsockopt version".as_ptr());
        libc::exit(1);
    }
    if libc::setsockopt(
        fd,
        SOL_PACKET,
        PACKET_RX_RING,
        &req as *const _ as *const c_void,
        mem::size_of_val(&req) as libc::socklen_t,
    ) != 0
    {
        libc::perror(c"packetsock ring setsockopt".as_ptr());
        libc::exit(1);
    }

    ring = libc::mmap(
        ptr::null_mut(),
        (req.tp_block_size * req.tp_block_nr) as libc::size_t,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_SHARED,
        fd,
        0,
    ) as *mut libc::c_char;
    if ring == libc::MAP_FAILED as *mut libc::c_char {
        libc::perror(c"packetsock ring mmap".as_ptr());
        libc::exit(1);
    }

    ring
}

unsafe fn sock_fanout_read_ring(_fd: libc::c_int, ring: *mut c_void) -> libc::c_int {
    let mut header = ring as *mut Tpacket2Hdr;
    let mut count: libc::c_int = 0;

    while count < RING_NUM_FRAMES && ((*header).tp_status & TP_STATUS_USER) != 0 {
        count += 1;
        header = (ring as *mut u8).add((count * libc::getpagesize()) as usize) as *mut Tpacket2Hdr;
    }

    count
}

unsafe fn sock_fanout_read(
    fds: *mut libc::c_int,
    rings: *mut *mut libc::c_char,
    expect: *const libc::c_int,
) -> libc::c_int {
    let mut ret = [0 as libc::c_int; 2];

    ret[0] = sock_fanout_read_ring(*fds.add(0), *rings.add(0) as *mut c_void);
    ret[1] = sock_fanout_read_ring(*fds.add(1), *rings.add(1) as *mut c_void);

    libc::fprintf(
        libc::stderr,
        c"info: count=%d,%d, expect=%d,%d\n".as_ptr(),
        ret[0],
        ret[1],
        *expect.add(0),
        *expect.add(1),
    );

    if !((ret[0] == *expect.add(0) && ret[1] == *expect.add(1))
        || (ret[0] == *expect.add(1) && ret[1] == *expect.add(0)))
    {
        libc::fprintf(libc::stderr, c"warning: incorrect queue lengths\n".as_ptr());
        return 1;
    }

    0
}

/* Test that creating/joining a fanout group fails for unbound socket without
 * a specified protocol
 */
unsafe fn test_unbound_fanout() {
    let mut val: libc::c_int;
    let fd0: libc::c_int;
    let fd1: libc::c_int;
    let mut err: libc::c_int;

    libc::fprintf(libc::stderr, c"test: unbound fanout\n".as_ptr());
    fd0 = libc::socket(libc::PF_PACKET, libc::SOCK_RAW, 0);
    if fd0 < 0 {
        libc::perror(c"socket packet".as_ptr());
        libc::exit(1);
    }
    /* Try to create a new fanout group. Should fail. */
    val = ((PACKET_FANOUT_HASH as libc::c_int) << 16) | 1;
    err = libc::setsockopt(
        fd0,
        SOL_PACKET,
        PACKET_FANOUT,
        &val as *const _ as *const c_void,
        mem::size_of_val(&val) as libc::socklen_t,
    );
    if err == 0 {
        libc::fprintf(libc::stderr, c"ERROR: unbound socket fanout create\n".as_ptr());
        libc::exit(1);
    }
    fd1 = sock_fanout_open(PACKET_FANOUT_HASH, 1);
    if fd1 == -1 {
        libc::fprintf(libc::stderr, c"ERROR: failed to open HASH socket\n".as_ptr());
        libc::exit(1);
    }
    /* Try to join an existing fanout group. Should fail. */
    err = libc::setsockopt(
        fd0,
        SOL_PACKET,
        PACKET_FANOUT,
        &val as *const _ as *const c_void,
        mem::size_of_val(&val) as libc::socklen_t,
    );
    if err == 0 {
        libc::fprintf(libc::stderr, c"ERROR: unbound socket fanout join\n".as_ptr());
        libc::exit(1);
    }
    libc::close(fd0);
    libc::close(fd1);
}

/* Test illegal mode + flag combination */
unsafe fn test_control_single() {
    libc::fprintf(libc::stderr, c"test: control single socket\n".as_ptr());

    if sock_fanout_open(PACKET_FANOUT_ROLLOVER | PACKET_FANOUT_FLAG_ROLLOVER, 0) != -1 {
        libc::fprintf(libc::stderr, c"ERROR: opened socket with dual rollover\n".as_ptr());
        libc::exit(1);
    }
}

/* Test illegal group with different modes or flags */
unsafe fn test_control_group(toggle: libc::c_int) {
    let mut fds = [0 as libc::c_int; 2];

    if toggle != 0 {
        libc::fprintf(
            libc::stderr,
            c"test: control multiple sockets with link down toggle\n".as_ptr(),
        );
    } else {
        libc::fprintf(libc::stderr, c"test: control multiple sockets\n".as_ptr());
    }

    fds[0] = sock_fanout_open(PACKET_FANOUT_HASH, 0);
    if fds[0] == -1 {
        libc::fprintf(libc::stderr, c"ERROR: failed to open HASH socket\n".as_ptr());
        libc::exit(1);
    }
    if toggle != 0 {
        loopback_set_up_down(0);
    }
    if sock_fanout_open(PACKET_FANOUT_HASH | PACKET_FANOUT_FLAG_DEFRAG, 0) != -1 {
        libc::fprintf(libc::stderr, c"ERROR: joined group with wrong flag defrag\n".as_ptr());
        libc::exit(1);
    }
    if sock_fanout_open(PACKET_FANOUT_HASH | PACKET_FANOUT_FLAG_ROLLOVER, 0) != -1 {
        libc::fprintf(libc::stderr, c"ERROR: joined group with wrong flag ro\n".as_ptr());
        libc::exit(1);
    }
    if sock_fanout_open(PACKET_FANOUT_CPU, 0) != -1 {
        libc::fprintf(libc::stderr, c"ERROR: joined group with wrong mode\n".as_ptr());
        libc::exit(1);
    }
    fds[1] = sock_fanout_open(PACKET_FANOUT_HASH, 0);
    if fds[1] == -1 {
        libc::fprintf(libc::stderr, c"ERROR: failed to join group\n".as_ptr());
        libc::exit(1);
    }
    if toggle != 0 {
        loopback_set_up_down(1);
    }
    if libc::close(fds[1]) != 0 || libc::close(fds[0]) != 0 {
        libc::fprintf(libc::stderr, c"ERROR: closing sockets\n".as_ptr());
        libc::exit(1);
    }
}

/* Test illegal max_num_members values */
unsafe fn test_control_group_max_num_members() {
    let mut fds = [0 as libc::c_int; 3];

    libc::fprintf(
        libc::stderr,
        c"test: control multiple sockets, max_num_members\n".as_ptr(),
    );

    /* expected failure on greater than PACKET_FANOUT_MAX */
    CFG_MAX_NUM_MEMBERS = (1 << 16) + 1;
    if sock_fanout_open(PACKET_FANOUT_HASH, 0) != -1 {
        libc::fprintf(libc::stderr, c"ERROR: max_num_members > PACKET_FANOUT_MAX\n".as_ptr());
        libc::exit(1);
    }

    CFG_MAX_NUM_MEMBERS = 256;
    fds[0] = sock_fanout_open(PACKET_FANOUT_HASH, 0);
    if fds[0] == -1 {
        libc::fprintf(libc::stderr, c"ERROR: failed open\n".as_ptr());
        libc::exit(1);
    }

    /* expected failure on joining group with different max_num_members */
    CFG_MAX_NUM_MEMBERS = 257;
    if sock_fanout_open(PACKET_FANOUT_HASH, 0) != -1 {
        libc::fprintf(libc::stderr, c"ERROR: set different max_num_members\n".as_ptr());
        libc::exit(1);
    }

    /* success on joining group with same max_num_members */
    CFG_MAX_NUM_MEMBERS = 256;
    fds[1] = sock_fanout_open(PACKET_FANOUT_HASH, 0);
    if fds[1] == -1 {
        libc::fprintf(libc::stderr, c"ERROR: failed to join group\n".as_ptr());
        libc::exit(1);
    }

    /* success on joining group with max_num_members unspecified */
    CFG_MAX_NUM_MEMBERS = 0;
    fds[2] = sock_fanout_open(PACKET_FANOUT_HASH, 0);
    if fds[2] == -1 {
        libc::fprintf(libc::stderr, c"ERROR: failed to join group\n".as_ptr());
        libc::exit(1);
    }

    if libc::close(fds[2]) != 0 || libc::close(fds[1]) != 0 || libc::close(fds[0]) != 0 {
        libc::fprintf(libc::stderr, c"ERROR: closing sockets\n".as_ptr());
        libc::exit(1);
    }
}

/* Test creating a unique fanout group ids */
unsafe fn test_unique_fanout_group_ids() {
    let mut fds = [0 as libc::c_int; 3];
    let mut typeflags: u16 = 0;
    let mut first_group_id: u16 = 0;
    let mut second_group_id: u16 = 0;

    libc::fprintf(libc::stderr, c"test: unique ids\n".as_ptr());

    fds[0] = sock_fanout_open(PACKET_FANOUT_HASH | PACKET_FANOUT_FLAG_UNIQUEID, 0);
    if fds[0] == -1 {
        libc::fprintf(
            libc::stderr,
            c"ERROR: failed to create a unique id group.\n".as_ptr(),
        );
        libc::exit(1);
    }

    sock_fanout_getopts(fds[0], &mut typeflags, &mut first_group_id);
    if typeflags != PACKET_FANOUT_HASH {
        libc::fprintf(
            libc::stderr,
            c"ERROR: unexpected typeflags %x\n".as_ptr(),
            typeflags as libc::c_int,
        );
        libc::exit(1);
    }

    if sock_fanout_open(PACKET_FANOUT_CPU, first_group_id) != -1 {
        libc::fprintf(libc::stderr, c"ERROR: joined group with wrong type.\n".as_ptr());
        libc::exit(1);
    }

    fds[1] = sock_fanout_open(PACKET_FANOUT_HASH, first_group_id);
    if fds[1] == -1 {
        libc::fprintf(
            libc::stderr,
            c"ERROR: failed to join previously created group.\n".as_ptr(),
        );
        libc::exit(1);
    }

    fds[2] = sock_fanout_open(PACKET_FANOUT_HASH | PACKET_FANOUT_FLAG_UNIQUEID, 0);
    if fds[2] == -1 {
        libc::fprintf(
            libc::stderr,
            c"ERROR: failed to create a second unique id group.\n".as_ptr(),
        );
        libc::exit(1);
    }

    sock_fanout_getopts(fds[2], &mut typeflags, &mut second_group_id);
    if sock_fanout_open(PACKET_FANOUT_HASH | PACKET_FANOUT_FLAG_UNIQUEID, second_group_id) != -1 {
        libc::fprintf(
            libc::stderr,
            c"ERROR: specified a group id when requesting unique id\n".as_ptr(),
        );
        libc::exit(1);
    }

    if libc::close(fds[0]) != 0 || libc::close(fds[1]) != 0 || libc::close(fds[2]) != 0 {
        libc::fprintf(libc::stderr, c"ERROR: closing sockets\n".as_ptr());
        libc::exit(1);
    }
}

unsafe fn test_datapath(
    typeflags: u16,
    port_off: libc::c_int,
    expect1: *const libc::c_int,
    expect2: *const libc::c_int,
) -> libc::c_int {
    let expect0 = [0 as libc::c_int, 0 as libc::c_int];
    let mut rings = [ptr::null_mut::<libc::c_char>(); 2];
    let type_ = (typeflags & 0xFF) as u8;
    let mut fds = [0 as libc::c_int; 2];
    let mut fds_udp = [[0 as libc::c_int; 2]; 2];
    let mut ret: libc::c_int;

    libc::fprintf(
        libc::stderr,
        c"\ntest: datapath 0x%hx ports %hu,%hu\n".as_ptr(),
        typeflags as libc::c_int,
        PORT_BASE as libc::c_int,
        (PORT_BASE as libc::c_int + port_off) as libc::c_ushort as libc::c_int,
    );

    fds[0] = sock_fanout_open(typeflags, 0);
    fds[1] = sock_fanout_open(typeflags, 0);
    if fds[0] == -1 || fds[1] == -1 {
        libc::fprintf(libc::stderr, c"ERROR: failed open\n".as_ptr());
        libc::exit(1);
    }
    if type_ == PACKET_FANOUT_CBPF as u8 {
        sock_fanout_set_cbpf(fds[0]);
    } else if type_ == PACKET_FANOUT_EBPF as u8 {
        sock_fanout_set_ebpf(fds[0]);
    }

    rings[0] = sock_fanout_open_ring(fds[0]);
    rings[1] = sock_fanout_open_ring(fds[1]);
    pair_udp_open(fds_udp[0].as_mut_ptr(), PORT_BASE);
    pair_udp_open(
        fds_udp[1].as_mut_ptr(),
        (PORT_BASE as libc::c_int + port_off) as libc::c_ushort,
    );
    sock_fanout_read(fds.as_mut_ptr(), rings.as_mut_ptr(), expect0.as_ptr());

    /* Send data, but not enough to overflow a queue */
    pair_udp_send(fds_udp[0].as_mut_ptr(), 15);
    pair_udp_send_char(fds_udp[1].as_mut_ptr(), 5, DATA_CHAR_1);
    ret = sock_fanout_read(fds.as_mut_ptr(), rings.as_mut_ptr(), expect1);

    /* Send more data, overflow the queue */
    pair_udp_send_char(fds_udp[0].as_mut_ptr(), 15, DATA_CHAR_1);
    /* TODO: ensure consistent order between expect1 and expect2 */
    ret |= sock_fanout_read(fds.as_mut_ptr(), rings.as_mut_ptr(), expect2);

    if libc::munmap(
        rings[1] as *mut c_void,
        (RING_NUM_FRAMES * libc::getpagesize()) as libc::size_t,
    ) != 0
        || libc::munmap(
            rings[0] as *mut c_void,
            (RING_NUM_FRAMES * libc::getpagesize()) as libc::size_t,
        ) != 0
    {
        libc::fprintf(libc::stderr, c"close rings\n".as_ptr());
        libc::exit(1);
    }
    if libc::close(fds_udp[1][1]) != 0
        || libc::close(fds_udp[1][0]) != 0
        || libc::close(fds_udp[0][1]) != 0
        || libc::close(fds_udp[0][0]) != 0
        || libc::close(fds[1]) != 0
        || libc::close(fds[0]) != 0
    {
        libc::fprintf(libc::stderr, c"close datapath\n".as_ptr());
        libc::exit(1);
    }

    ret
}

unsafe fn set_cpuaffinity(cpuid: libc::c_int) -> libc::c_int {
    let mut mask: libc::cpu_set_t = mem::zeroed();

    libc::CPU_ZERO(&mut mask);
    libc::CPU_SET(cpuid as usize, &mut mask);
    if libc::sched_setaffinity(0, mem::size_of_val(&mask), &mask) != 0 {
        if *libc::__errno_location() != libc::EINVAL {
            libc::fprintf(libc::stderr, c"setaffinity %d\n".as_ptr(), cpuid);
            libc::exit(1);
        }
        return 1;
    }

    0
}

unsafe fn c_main(_argc: libc::c_int, _argv: *mut *mut libc::c_char) -> libc::c_int {
    let expect_hash = [[15, 5], [20, 5]];
    let expect_hash_rb = [[15, 5], [20, 15]];
    let expect_lb = [[10, 10], [18, 17]];
    let expect_rb = [[15, 5], [20, 15]];
    let expect_cpu0 = [[20, 0], [20, 0]];
    let expect_cpu1 = [[0, 20], [0, 20]];
    let expect_bpf = [[15, 5], [15, 20]];
    let expect_uniqueid = [[20, 20], [20, 20]];
    let mut port_off: libc::c_int = 2;
    let mut tries: libc::c_int = 20;
    let mut ret: libc::c_int;

    test_unbound_fanout();
    test_control_single();
    test_control_group(0);
    test_control_group(1);
    test_control_group_max_num_members();
    test_unique_fanout_group_ids();

    /* PACKET_FANOUT_MAX */
    CFG_MAX_NUM_MEMBERS = 1 << 16;
    /* find a set of ports that do not collide onto the same socket */
    ret = test_datapath(
        PACKET_FANOUT_HASH,
        port_off,
        expect_hash[0].as_ptr(),
        expect_hash[1].as_ptr(),
    );
    while ret != 0 {
        libc::fprintf(
            libc::stderr,
            c"info: trying alternate ports (%d)\n".as_ptr(),
            tries,
        );
        port_off += 1;
        ret = test_datapath(
            PACKET_FANOUT_HASH,
            port_off,
            expect_hash[0].as_ptr(),
            expect_hash[1].as_ptr(),
        );
        tries -= 1;
        if tries == 0 {
            libc::fprintf(libc::stderr, c"too many collisions\n".as_ptr());
            return 1;
        }
    }

    ret |= test_datapath(
        PACKET_FANOUT_HASH | PACKET_FANOUT_FLAG_ROLLOVER,
        port_off,
        expect_hash_rb[0].as_ptr(),
        expect_hash_rb[1].as_ptr(),
    );
    ret |= test_datapath(
        PACKET_FANOUT_LB,
        port_off,
        expect_lb[0].as_ptr(),
        expect_lb[1].as_ptr(),
    );
    ret |= test_datapath(
        PACKET_FANOUT_ROLLOVER,
        port_off,
        expect_rb[0].as_ptr(),
        expect_rb[1].as_ptr(),
    );

    ret |= test_datapath(
        PACKET_FANOUT_CBPF,
        port_off,
        expect_bpf[0].as_ptr(),
        expect_bpf[1].as_ptr(),
    );
    ret |= test_datapath(
        PACKET_FANOUT_EBPF,
        port_off,
        expect_bpf[0].as_ptr(),
        expect_bpf[1].as_ptr(),
    );

    set_cpuaffinity(0);
    ret |= test_datapath(
        PACKET_FANOUT_CPU,
        port_off,
        expect_cpu0[0].as_ptr(),
        expect_cpu0[1].as_ptr(),
    );
    if set_cpuaffinity(1) == 0 {
        /* TODO: test that choice alternates with previous */
        ret |= test_datapath(
            PACKET_FANOUT_CPU,
            port_off,
            expect_cpu1[0].as_ptr(),
            expect_cpu1[1].as_ptr(),
        );
    }

    ret |= test_datapath(
        PACKET_FANOUT_FLAG_UNIQUEID,
        port_off,
        expect_uniqueid[0].as_ptr(),
        expect_uniqueid[1].as_ptr(),
    );

    if ret != 0 {
        return 1;
    }

    libc::printf(c"OK. All tests passed\n".as_ptr());
    0
}

fn main() {
    unsafe {
        let code = c_main(0, ptr::null_mut());
        if code != 0 {
            std::process::exit(code);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
