// Translated from fds_example.c. Required Linux, libbpf, and local header
// declarations are supplied by the surrounding build environment.

use std::ffi::{c_char, c_int, c_void};

const BPF_F_PIN: u32 = 1 << 0;
const BPF_F_GET: u32 = 1 << 1;
const BPF_F_PIN_GET: u32 = BPF_F_PIN | BPF_F_GET;

const BPF_F_KEY: u32 = 1 << 2;
const BPF_F_VAL: u32 = 1 << 3;
const BPF_F_KEY_VAL: u32 = BPF_F_KEY | BPF_F_VAL;

const BPF_M_UNSPEC: c_int = 0;
const BPF_M_MAP: c_int = 1;
const BPF_M_PROG: c_int = 2;

const BPF_LOG_BUF_SIZE: usize = 65536;

#[repr(C)]
pub struct bpf_insn {
    pub code: u8,
    pub regs: u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
pub struct bpf_object;
#[repr(C)]
pub struct bpf_program;

extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;

    fn printf(format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    fn assert_failed(expr: *const c_char, file: *const c_char, line: c_int, function: *const c_char) -> !;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u32;
    fn setsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: usize) -> c_int;

    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> c_int;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__next_program(obj: *mut bpf_object, prev: *mut bpf_program) -> *mut bpf_program;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_load(prog_type: u32, name: *const c_char, license: *const c_char,
                     insns: *const bpf_insn, insn_cnt: usize, opts: *mut c_void) -> c_int;
    fn bpf_map_create(map_type: u32, name: *const c_char, key_size: u32,
                      value_size: u32, max_entries: u32, opts: *const c_void) -> c_int;
    fn bpf_obj_pin(fd: c_int, pathname: *const c_char) -> c_int;
    fn bpf_obj_get(pathname: *const c_char) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn open_raw_sock(name: *const c_char) -> c_int;
}

static mut BPF_LOG_BUF: [c_char; BPF_LOG_BUF_SIZE] = [0; BPF_LOG_BUF_SIZE];

unsafe fn usage() {
    printf(b"Usage: fds_example [...]\0".as_ptr() as *const c_char);
    printf(b"       -F <file>   File to pin/get object\n\0".as_ptr() as *const c_char);
    printf(b"       -P          |- pin object\n\0".as_ptr() as *const c_char);
    printf(b"       -G          `- get object\n\0".as_ptr() as *const c_char);
    printf(b"       -m          eBPF map mode\n\0".as_ptr() as *const c_char);
    printf(b"       -k <key>    |- map key\n\0".as_ptr() as *const c_char);
    printf(b"       -v <value>  `- map value\n\0".as_ptr() as *const c_char);
    printf(b"       -p          eBPF prog mode\n\0".as_ptr() as *const c_char);
    printf(b"       -o <object> `- object file\n\0".as_ptr() as *const c_char);
    printf(b"       -h          Display this help.\n\0".as_ptr() as *const c_char);
}

unsafe fn bpf_prog_create(object: *const c_char) -> c_int {
    let insns = [bpf_insn { code: 0xb7, regs: 0, off: 0, imm: 1 }, bpf_insn { code: 0x95, regs: 0, off: 0, imm: 0 }];
    if !object.is_null() {
        let obj = bpf_object__open_file(object, std::ptr::null());
        assert!(libbpf_get_error(obj as *const c_void) == 0);
        let err = bpf_object__load(obj);
        assert!(err == 0);
        return bpf_program__fd(bpf_object__next_program(obj, std::ptr::null_mut()));
    }
    bpf_prog_load(1, std::ptr::null(), b"GPL\0".as_ptr() as *const c_char,
                  insns.as_ptr(), insns.len(), std::ptr::null_mut())
}

unsafe fn bpf_do_map(file: *const c_char, flags: u32, mut key: u32, mut value: u32) -> c_int {
    let fd;
    let ret;
    if flags & BPF_F_PIN != 0 {
        fd = bpf_map_create(2, std::ptr::null(), 4, 4, 1024, std::ptr::null());
        assert!(fd > 0);
        ret = bpf_obj_pin(fd, file);
        assert!(ret == 0);
    } else {
        fd = bpf_obj_get(file);
        assert!(fd > 0);
    }
    if flags & BPF_F_KEY_VAL == BPF_F_KEY_VAL {
        ret = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, 0);
        assert!(ret == 0);
    } else if flags & BPF_F_KEY != 0 {
        ret = bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, &mut value as *mut _ as *mut c_void);
        assert!(ret == 0);
    }
    0
}

unsafe fn bpf_do_prog(file: *const c_char, flags: u32, object: *const c_char) -> c_int {
    let fd = if flags & BPF_F_PIN != 0 { let f = bpf_prog_create(object); assert!(f > 0); assert!(bpf_obj_pin(f, file) == 0); f } else { let f = bpf_obj_get(file); assert!(f > 0); f };
    let sock = open_raw_sock(b"lo\0".as_ptr() as *const c_char);
    assert!(sock > 0);
    let ret = setsockopt(sock, 1, 50, &fd as *const _ as *const c_void, std::mem::size_of::<c_int>());
    assert!(ret == 0);
    0
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut file: *const c_char = std::ptr::null();
    let mut object: *const c_char = std::ptr::null();
    let mut key = 0u32;
    let mut value = 0u32;
    let mut flags = 0u32;
    let mut mode = BPF_M_UNSPEC;
    loop {
        let opt = getopt(argc, argv, b"F:PGmk:v:po:\0".as_ptr() as *const c_char);
        if opt == -1 { break; }
        match opt as u8 {
            b'F' => file = optarg,
            b'P' => flags |= BPF_F_PIN,
            b'G' => flags |= BPF_F_GET,
            b'm' => mode = BPF_M_MAP,
            b'k' => { key = strtoul(optarg, std::ptr::null_mut(), 0); flags |= BPF_F_KEY; },
            b'v' => { value = strtoul(optarg, std::ptr::null_mut(), 0); flags |= BPF_F_VAL; },
            b'p' => mode = BPF_M_PROG,
            b'o' => object = optarg,
            _ => { usage(); return -1; }
        }
    }
    if flags & BPF_F_PIN_GET == 0 || file.is_null() { usage(); return -1; }
    match mode {
        BPF_M_MAP => bpf_do_map(file, flags, key, value),
        BPF_M_PROG => bpf_do_prog(file, flags, object),
        _ => { usage(); -1 }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
