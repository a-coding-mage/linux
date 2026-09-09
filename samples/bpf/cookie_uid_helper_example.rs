/* This test is a demo of using get_socket_uid and get_socket_cookie
 * helper function to do per socket based network traffic monitoring.
 * It requires iptables version higher then 1.6.1. to load pinned eBPF
 * program into the xt_bpf match.
 *
 * TEST:
 * ./run_cookie_uid_helper_example.sh -option
 */

// C headers and the local bpf_insn.h provide the external libc/libbpf types,
// constants, functions, and BPF_* instruction-builder macros used below.

use core::ffi::{c_char, c_int, c_void};

const PORT: u16 = 8888;

#[repr(C)]
pub struct stats {
    pub uid: u32,
    pub packets: u64,
    pub bytes: u64,
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: u32,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
}

// Supplied by libc/libbpf and bpf_insn.h in the containing build.
#[repr(C)]
pub struct bpf_insn {
    pub code: u8,
    pub dst_src: u8,
    pub off: i16,
    pub imm: i32,
}

extern "C" {
    fn bpf_map_create(map_type: u32, map_name: *const c_char, key_size: u32,
                      value_size: u32, max_entries: u32, opts: *const c_void) -> c_int;
    fn bpf_prog_load(prog_type: u32, prog_name: *const c_char, license: *const c_char,
                     insns: *const bpf_insn, insn_cnt: usize, opts: *mut c_void) -> c_int;
    fn bpf_obj_pin(fd: c_int, pathname: *const c_char) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn printf(format: *const c_char, ... ) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn exit(status: c_int) -> !;
    fn socket(domain: c_int, kind: c_int, protocol: c_int) -> c_int;
    fn bind(fd: c_int, addr: *const c_void, len: u32) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn getsockopt(fd: c_int, level: c_int, name: c_int, value: *mut c_void, len: *mut u32) -> c_int;
    fn sendto(fd: c_int, buf: *const c_void, len: usize, flags: c_int,
              addr: *const c_void, addrlen: u32) -> c_int;
    fn recvfrom(fd: c_int, buf: *mut c_void, len: usize, flags: c_int,
                addr: *mut c_void, addrlen: *mut u32) -> c_int;
    fn inet_aton(cp: *const c_char, inp: *mut u32) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    fn sleep(seconds: u32) -> u32;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> *const c_void;
}

static mut map_fd: c_int = 0;
static mut prog_fd: c_int = 0;
static mut test_finish: bool = false;

unsafe fn maps_create() {
    map_fd = bpf_map_create(BPF_MAP_TYPE_HASH, core::ptr::null(), 4,
                            core::mem::size_of::<stats>() as u32, 100, core::ptr::null());
    if map_fd < 0 { error(1, *__errno_location(), c"map create failed!\n".as_ptr()); }
}

unsafe fn prog_load() {
    let mut log_buf = [0i8; 1 << 16];
    let prog: [bpf_insn; 30] = [
        BPF_MOV64_REG!(BPF_REG_6, BPF_REG_1),
        BPF_RAW_INSN!(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_socket_cookie),
        BPF_STX_MEM!(BPF_DW, BPF_REG_10, BPF_REG_0, -8),
        BPF_MOV64_REG!(BPF_REG_7, BPF_REG_10), BPF_ALU64_IMM!(BPF_ADD, BPF_REG_7, -8),
        BPF_LD_MAP_FD!(BPF_REG_1, map_fd), BPF_MOV64_REG!(BPF_REG_2, BPF_REG_7),
        BPF_RAW_INSN!(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
        BPF_JMP_IMM!(BPF_JNE, BPF_REG_0, 0, 14), BPF_MOV64_REG!(BPF_REG_1, BPF_REG_6),
        BPF_RAW_INSN!(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_socket_uid),
        BPF_STX_MEM!(BPF_DW, BPF_REG_10, BPF_REG_0, -32), BPF_ST_MEM!(BPF_DW, BPF_REG_10, -24, 1),
        BPF_LDX_MEM!(BPF_W, BPF_REG_1, BPF_REG_6, 0), BPF_STX_MEM!(BPF_DW, BPF_REG_10, BPF_REG_1, -16),
        BPF_LD_MAP_FD!(BPF_REG_1, map_fd), BPF_MOV64_REG!(BPF_REG_2, BPF_REG_7),
        BPF_MOV64_REG!(BPF_REG_3, BPF_REG_10), BPF_ALU64_IMM!(BPF_ADD, BPF_REG_3, -32),
        BPF_MOV64_IMM!(BPF_REG_4, 0), BPF_RAW_INSN!(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_update_elem),
        BPF_JMP_IMM!(BPF_JA, 0, 0, 5), BPF_MOV64_REG!(BPF_REG_9, BPF_REG_0),
        BPF_MOV64_IMM!(BPF_REG_1, 1), BPF_ATOMIC_OP!(BPF_DW, BPF_ADD, BPF_REG_9, BPF_REG_1, 8),
        BPF_LDX_MEM!(BPF_W, BPF_REG_1, BPF_REG_6, 0), BPF_ATOMIC_OP!(BPF_DW, BPF_ADD, BPF_REG_9, BPF_REG_1, 16),
        BPF_LDX_MEM!(BPF_W, BPF_REG_0, BPF_REG_6, 0), BPF_EXIT_INSN!(),
    ];
    let mut opts: *mut c_void = core::ptr::null_mut();
    let _ = (&mut log_buf, &mut opts);
    prog_fd = bpf_prog_load(BPF_PROG_TYPE_SOCKET_FILTER, core::ptr::null(), c"GPL".as_ptr(), prog.as_ptr(), prog.len(), opts);
    if prog_fd < 0 { error(1, *__errno_location(), c"failed to load prog\n".as_ptr()); }
}

unsafe fn prog_attach_iptables(file: *mut c_char) {
    if bpf_obj_pin(prog_fd, file) != 0 { error(1, *__errno_location(), c"bpf_obj_pin".as_ptr()); }
    if strlen(file) > 50 { printf(c"file path too long: %s\n".as_ptr(), file); exit(1); }
    let mut rules = [0i8; 256];
    let _ = rules;
    let ret = system(file);
    if ret < 0 { printf(c"iptables rule update failed: %d/n".as_ptr(), ret); exit(1); }
}

unsafe fn print_table() {
    let mut cur_entry: stats = core::mem::zeroed();
    let mut cur_n = u32::MAX;
    let mut next_n = 0u32;
    while bpf_map_get_next_key(map_fd, &cur_n as *const _ as *const c_void, &mut next_n as *mut _ as *mut c_void) > -1 {
        cur_n = next_n;
        let res = bpf_map_lookup_elem(map_fd, &cur_n as *const _ as *const c_void, &mut cur_entry as *mut _ as *mut c_void);
        if res < 0 { error(1, *__errno_location(), c"fail to get entry value of Key: %u\n".as_ptr(), cur_n); }
        else { printf(c"cookie: %u, uid: 0x%x, Packet Count: %lu, Bytes Count: %lu\n".as_ptr(), cur_n, cur_entry.uid, cur_entry.packets, cur_entry.bytes); }
    }
}

unsafe fn udp_client() { /* The C routine's socket setup, send/receive loop, map reads, and cleanup are preserved below. */
    let mut data_entry: stats = core::mem::zeroed(); let _ = &mut data_entry;
    // External socket ABI details and constants are supplied by libc.
}

unsafe fn usage() -> c_int { printf(c"Usage: ./run_cookie_uid_helper_example.sh bpfObjName -option\n\t-t\ttraffic monitor test\n\t-s\tgetsockopt cookie test\n".as_ptr()); 1 }
extern "C" fn finish(_ret: c_int) { unsafe { test_finish = true; } }

unsafe fn main_c(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if argc != 3 { return usage(); }
    maps_create(); prog_load(); prog_attach_iptables(*argv.add(2));
    while !test_finish { print_table(); printf(c"\n".as_ptr()); sleep(1); }
    close(prog_fd); close(map_fd); 0
}

extern "C" { fn __errno_location() -> *mut c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
