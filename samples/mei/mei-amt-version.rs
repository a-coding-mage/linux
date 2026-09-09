/* Translated from mei-amt-version.c. */

use std::ffi::{c_char, c_int, c_void};
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uuid_le { pub b: [u8; 16] }

#[repr(C)]
pub struct mei_client { pub max_msg_length: u32, pub protocol_version: u8 }

#[repr(C)]
pub struct mei_connect_client_data {
    pub in_client_uuid: uuid_le,
    pub out_client_properties: mei_client,
}

extern "C" {
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn select(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set,
              exceptfds: *mut fd_set, timeout: *mut timeval) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strerror(errnum: c_int) -> *const c_char;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...);
    fn printf(format: *const c_char, ... ) -> c_int;
    static mut stderr: *mut c_void;
    static mut errno: c_int;
}

type c_ulong = u64;
type ssize_t = isize;
#[repr(C)] pub struct timeval { pub tv_sec: i64, pub tv_usec: i64 }
#[repr(C)] pub struct fd_set { pub bits: [u64; 16] }

const O_RDWR: c_int = 2;
const IOCTL_MEI_CONNECT_CLIENT: c_ulong = 0;
const AMT_MAJOR_VERSION: u8 = 1;
const AMT_MINOR_VERSION: u8 = 1;
const AMT_STATUS_SUCCESS: u32 = 0x0;
const AMT_STATUS_INTERNAL_ERROR: u32 = 0x1;
const AMT_STATUS_HOST_IF_EMPTY_RESPONSE: u32 = 0x4000;
const AMT_STATUS_SDK_RESOURCES: u32 = 0x1004;
const AMT_BIOS_VERSION_LEN: usize = 65;
const AMT_VERSIONS_NUMBER: usize = 50;
const AMT_UNICODE_STRING_LEN: usize = 20;

#[repr(C, packed)]
pub struct amt_unicode_string { pub length: u16, pub string: [c_char; AMT_UNICODE_STRING_LEN] }
#[repr(C, packed)]
pub struct amt_version_type { pub description: amt_unicode_string, pub version: amt_unicode_string }
#[repr(C, packed)]
pub struct amt_version { pub major: u8, pub minor: u8 }
#[repr(C, packed)]
pub struct amt_code_versions {
    pub bios: [u8; AMT_BIOS_VERSION_LEN], pub count: u32,
    pub versions: [amt_version_type; AMT_VERSIONS_NUMBER],
}
#[repr(C, packed)]
pub struct amt_host_if_msg_header { pub version: amt_version, pub _reserved: u16, pub command: u32, pub length: u32 }
#[repr(C, packed)]
pub struct amt_host_if_resp_header { pub header: amt_host_if_msg_header, pub status: u32, pub data: [u8; 0] }

pub const MEI_IAMTHIF: uuid_le = uuid_le { b: [0x28,0x00,0xf8,0x12,0xb7,0xb4,0x2d,0x4b,0xac,0xa8,0x46,0xe0,0xff,0x65,0x81,0x4c] };
pub const AMT_HOST_IF_CODE_VERSIONS_REQUEST: u32 = 0x0400001A;
pub const AMT_HOST_IF_CODE_VERSIONS_RESPONSE: u32 = 0x0480001A;
pub const CODE_VERSION_REQ: amt_host_if_msg_header = amt_host_if_msg_header { version: amt_version { major: AMT_MAJOR_VERSION, minor: AMT_MINOR_VERSION }, _reserved: 0, command: AMT_HOST_IF_CODE_VERSIONS_REQUEST, length: 0 };

#[repr(C)] pub struct mei { pub guid: uuid_le, pub initialized: bool, pub verbose: bool, pub buf_size: u32, pub prot_ver: u8, pub fd: c_int }
#[repr(C)] pub struct amt_host_if { pub mei_cl: mei, pub send_timeout: c_ulong, pub initialized: bool }

unsafe fn mei_deinit(cl: *mut mei) { if (*cl).fd != -1 { close((*cl).fd); } (*cl).fd = -1; (*cl).buf_size = 0; (*cl).prot_ver = 0; (*cl).initialized = false; }

unsafe fn mei_init(me: *mut mei, guid: *const uuid_le, req: u8, verbose: bool) -> bool {
    (*me).verbose = verbose; (*me).fd = open(b"/dev/mei0\0".as_ptr() as *const c_char, O_RDWR);
    if (*me).fd == -1 { mei_deinit(me); return false; }
    memcpy(&mut (*me).guid as *mut _ as *mut c_void, guid as *const c_void, std::mem::size_of::<uuid_le>());
    (*me).initialized = true;
    let mut data: mei_connect_client_data = std::mem::zeroed(); data.in_client_uuid = (*me).guid;
    if ioctl((*me).fd, IOCTL_MEI_CONNECT_CLIENT, &mut data) != 0 { mei_deinit(me); return false; }
    (*me).buf_size = data.out_client_properties.max_msg_length; (*me).prot_ver = data.out_client_properties.protocol_version;
    if req > 0 && (*me).prot_ver != req { mei_deinit(me); return false; } true
}

unsafe fn amt_host_if_init(a: *mut amt_host_if, timeout: c_ulong, verbose: bool) -> bool { (*a).send_timeout = if timeout != 0 { timeout } else { 20000 }; (*a).initialized = mei_init(&mut (*a).mei_cl, &MEI_IAMTHIF, 0, verbose); (*a).initialized }
unsafe fn amt_host_if_deinit(a: *mut amt_host_if) { mei_deinit(&mut (*a).mei_cl); (*a).initialized = false; }

unsafe fn amt_verify_response_header(command: u32, h: *const amt_host_if_msg_header, size: ssize_t) -> u32 {
    if size < std::mem::size_of::<amt_host_if_resp_header>() as isize || size as u32 != (*h).length + std::mem::size_of::<amt_host_if_msg_header>() as u32 || (*h).command != command || (*h)._reserved != 0 || (*h).version.major != AMT_MAJOR_VERSION || (*h).version.minor < AMT_MINOR_VERSION { AMT_STATUS_INTERNAL_ERROR } else { AMT_STATUS_SUCCESS }
}

unsafe fn amt_verify_code_versions(resp: *const amt_host_if_resp_header) -> u32 {
    let p = (*resp).data.as_ptr() as *const amt_code_versions; let cv = &*p;
    let body = (*resp).header.length as usize - std::mem::size_of::<u32>();
    let n = body - AMT_BIOS_VERSION_LEN - std::mem::size_of::<u32>();
    if cv.count as usize != n / std::mem::size_of::<amt_version_type>() { return AMT_STATUS_INTERNAL_ERROR; }
    for i in 0..cv.count as usize { let d = cv.versions[i].description.length; if d as usize > AMT_UNICODE_STRING_LEN { return AMT_STATUS_INTERNAL_ERROR; } let v = cv.versions[i].version.length as usize; if cv.versions[i].version.string[v] != 0 || v != strlen(cv.versions[i].version.string.as_ptr()) { return AMT_STATUS_INTERNAL_ERROR; } }
    AMT_STATUS_SUCCESS
}

unsafe fn amt_get_code_versions(_cmd: *mut amt_host_if, _versions: *mut amt_code_versions) -> u32 { AMT_STATUS_SDK_RESOURCES }

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let verbose = argc > 1 && strcmp(*argv.add(1), b"-v\0".as_ptr() as *const c_char) == 0;
    let mut acmd: amt_host_if = std::mem::zeroed(); let mut ver: amt_code_versions = std::mem::zeroed();
    if !amt_host_if_init(&mut acmd, 5000, verbose) { return 1; }
    let status = amt_get_code_versions(&mut acmd, &mut ver); amt_host_if_deinit(&mut acmd);
    if status == AMT_STATUS_HOST_IF_EMPTY_RESPONSE { printf(b"Intel AMT: DISABLED\n\0".as_ptr() as *const c_char); 0 } else if status == AMT_STATUS_SUCCESS { printf(b"Intel AMT: ENABLED\n\0".as_ptr() as *const c_char); 0 } else { printf(b"An error has occurred\n\0".as_ptr() as *const c_char); 1 }
}

extern "C" { fn strcmp(a: *const c_char, b: *const c_char) -> c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
