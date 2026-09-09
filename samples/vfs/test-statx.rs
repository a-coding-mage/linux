// SPDX-License-Identifier: GPL-2.0-or-later
/* Test the statx() system call. */

use std::ffi::{c_char, c_int, c_uint, c_void, CString};
use std::ptr;

const AT_STATX_SYNC_TYPE: c_int = 0x6000;
const AT_STATX_SYNC_AS_STAT: c_int = 0x0000;
const AT_STATX_FORCE_SYNC: c_int = 0x2000;
const AT_STATX_DONT_SYNC: c_int = 0x4000;
const __NR_STATX: c_long = -1;
type c_long = isize;

extern "C" {
    fn syscall(number: c_long, ...) -> c_long;
    fn localtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm;
    fn strftime(s: *mut c_char, max: usize, format: *const c_char, tm: *const Tm) -> usize;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn printf(format: *const c_char, ...);
    fn sprintf(s: *mut c_char, format: *const c_char, ...);
    fn fwrite(ptr: *const c_void, size: usize, nmemb: usize, stream: *mut c_void) -> usize;
    fn putchar(c: c_int) -> c_int;
    static mut stdout: *mut c_void;
}

#[repr(C)] struct Tm { _data: [u8; 64] }
#[repr(C)] pub struct StatxTimestamp { pub tv_sec: i64, pub tv_nsec: u32, pub __reserved: i32 }
#[repr(C)] pub struct Statx {
    pub stx_mask: u32, pub stx_blksize: u32, pub stx_attributes: u64,
    pub stx_nlink: u32, pub stx_uid: u32, pub stx_gid: u32, pub stx_mode: u16,
    pub __spare0: [u16; 1], pub stx_ino: u64, pub stx_size: u64, pub stx_blocks: u64,
    pub stx_attributes_mask: u64, pub stx_atime: StatxTimestamp, pub stx_btime: StatxTimestamp,
    pub stx_ctime: StatxTimestamp, pub stx_mtime: StatxTimestamp,
    pub stx_rdev_major: u32, pub stx_rdev_minor: u32, pub stx_dev_major: u32, pub stx_dev_minor: u32,
    pub __spare2: [u64; 14],
}

const STATX_TYPE: u32 = 0x0001; const STATX_MODE: u32 = 0x0002; const STATX_NLINK: u32 = 0x0004;
const STATX_UID: u32 = 0x0008; const STATX_GID: u32 = 0x0010; const STATX_ATIME: u32 = 0x0020;
const STATX_MTIME: u32 = 0x0040; const STATX_CTIME: u32 = 0x0080; const STATX_INO: u32 = 0x0100;
const STATX_SIZE: u32 = 0x0200; const STATX_BLOCKS: u32 = 0x0400; const STATX_BASIC_STATS: u32 = 0x07ff;
const STATX_BTIME: u32 = 0x0800;
const S_IFMT: u16 = 0o170000; const S_IFIFO: u16 = 0o010000; const S_IFCHR: u16 = 0o020000;
const S_IFDIR: u16 = 0o040000; const S_IFBLK: u16 = 0o060000; const S_IFREG: u16 = 0o100000;
const S_IFLNK: u16 = 0o120000; const S_IFSOCK: u16 = 0o140000;
const S_IRUSR: u16 = 0o400; const S_IWUSR: u16 = 0o200; const S_IXUSR: u16 = 0o100;
const S_IRGRP: u16 = 0o040; const S_IWGRP: u16 = 0o020; const S_IXGRP: u16 = 0o010;
const S_IROTH: u16 = 0o004; const S_IWOTH: u16 = 0o002; const S_IXOTH: u16 = 0o001;
const AT_SYMLINK_NOFOLLOW: c_int = 0x100; const AT_NO_AUTOMOUNT: c_int = 0x800;
const AT_FDCWD: c_int = -100;

unsafe fn statx(dfd: c_int, filename: *const c_char, flags: c_uint, mask: c_uint, buffer: *mut Statx) -> c_long {
    syscall(__NR_STATX, dfd, filename, flags, mask, buffer)
}

unsafe fn print_time(field: *const c_char, ts: *const StatxTimestamp) {
    let mut tm = Tm { _data: [0; 64] }; let mut buffer = [0i8; 100];
    let tim = (*ts).tv_sec;
    if localtime_r(&tim, &mut tm).is_null() { perror(b"localtime_r\0".as_ptr() as _); exit(1); }
    let len = strftime(buffer.as_mut_ptr(), 100, b"%F %T\0".as_ptr() as _, &tm);
    if len == 0 { perror(b"strftime\0".as_ptr() as _); exit(1); }
    printf(b"%s\0".as_ptr() as _, field); fwrite(buffer.as_ptr() as _, 1, len, stdout);
    printf(b".%09u\0".as_ptr() as _, (*ts).tv_nsec);
    let len = strftime(buffer.as_mut_ptr(), 100, b"%z\0".as_ptr() as _, &tm);
    if len == 0 { perror(b"strftime2\0".as_ptr() as _); exit(1); }
    fwrite(buffer.as_ptr() as _, 1, len, stdout); printf(b"\n\0".as_ptr() as _);
}

unsafe fn dump_statx(stx: *const Statx) {
    let mut ft = b'?'; let mut buffer = [0i8; 256];
    printf(b"results=%x\n \0".as_ptr() as _, (*stx).stx_mask);
    if (*stx).stx_mask & STATX_SIZE != 0 { printf(b" Size: %-15llu\0".as_ptr() as _, (*stx).stx_size); }
    if (*stx).stx_mask & STATX_BLOCKS != 0 { printf(b" Blocks: %-10llu\0".as_ptr() as _, (*stx).stx_blocks); }
    printf(b" IO Block: %-6llu\0".as_ptr() as _, (*stx).stx_blksize as u64);
    if (*stx).stx_mask & STATX_TYPE != 0 { match (*stx).stx_mode & S_IFMT {
        S_IFIFO => { printf(b"  FIFO\n\0".as_ptr() as _); ft=b'p'; }, S_IFCHR => { printf(b"  character special file\n\0".as_ptr() as _); ft=b'c'; },
        S_IFDIR => { printf(b"  directory\n\0".as_ptr() as _); ft=b'd'; }, S_IFBLK => { printf(b"  block special file\n\0".as_ptr() as _); ft=b'b'; },
        S_IFREG => { printf(b"  regular file\n\0".as_ptr() as _); ft=b'-'; }, S_IFLNK => { printf(b"  symbolic link\n\0".as_ptr() as _); ft=b'l'; },
        S_IFSOCK => { printf(b"  socket\n\0".as_ptr() as _); ft=b's'; }, m => printf(b" unknown type (%o)\n\0".as_ptr() as _, m),
    }} else { printf(b" no type\n\0".as_ptr() as _); }
    sprintf(buffer.as_mut_ptr(), b"%02x:%02x\0".as_ptr() as _, (*stx).stx_dev_major, (*stx).stx_dev_minor);
    printf(b"Device: %-15s\0".as_ptr() as _, buffer.as_ptr());
    if (*stx).stx_mask & STATX_INO != 0 { printf(b" Inode: %-11llu\0".as_ptr() as _, (*stx).stx_ino); }
    if (*stx).stx_mask & STATX_NLINK != 0 { printf(b" Links: %-5u\0".as_ptr() as _, (*stx).stx_nlink); }
    if (*stx).stx_mask & STATX_TYPE != 0 && ((*stx).stx_mode & S_IFMT == S_IFBLK || (*stx).stx_mode & S_IFMT == S_IFCHR) { printf(b" Device type: %u,%u\0".as_ptr() as _, (*stx).stx_rdev_major, (*stx).stx_rdev_minor); }
    printf(b"\n\0".as_ptr() as _);
    if (*stx).stx_mask & STATX_MODE != 0 { printf(b"Access: (%04o/%c%c%c%c%c%c%c%c%c%c)  \0".as_ptr() as _, (*stx).stx_mode & 0o7777, ft, if (*stx).stx_mode&S_IRUSR!=0 {b'r'}else{b'-'}, if (*stx).stx_mode&S_IWUSR!=0 {b'w'}else{b'-'}, if (*stx).stx_mode&S_IXUSR!=0 {b'x'}else{b'-'}, if (*stx).stx_mode&S_IRGRP!=0 {b'r'}else{b'-'}, if (*stx).stx_mode&S_IWGRP!=0 {b'w'}else{b'-'}, if (*stx).stx_mode&S_IXGRP!=0 {b'x'}else{b'-'}, if (*stx).stx_mode&S_IROTH!=0 {b'r'}else{b'-'}, if (*stx).stx_mode&S_IWOTH!=0 {b'w'}else{b'-'}, if (*stx).stx_mode&S_IXOTH!=0 {b'x'}else{b'-'}); }
    if (*stx).stx_mask & STATX_UID != 0 { printf(b"Uid: %5d   \0".as_ptr() as _, (*stx).stx_uid); } if (*stx).stx_mask & STATX_GID != 0 { printf(b"Gid: %5d\n\0".as_ptr() as _, (*stx).stx_gid); }
    if (*stx).stx_mask & STATX_ATIME != 0 { print_time(b"Access: \0".as_ptr() as _, &(*stx).stx_atime); } if (*stx).stx_mask & STATX_MTIME != 0 { print_time(b"Modify: \0".as_ptr() as _, &(*stx).stx_mtime); } if (*stx).stx_mask & STATX_CTIME != 0 { print_time(b"Change: \0".as_ptr() as _, &(*stx).stx_ctime); } if (*stx).stx_mask & STATX_BTIME != 0 { print_time(b" Birth: \0".as_ptr() as _, &(*stx).stx_btime); }
}

unsafe fn dump_hex(data: *const u64, from: c_int, to: c_int) { let from=(from/8) as usize; let to=((to+7)/8) as usize; let mut print_offset=true; let mut col=0; for offset in from..to { if print_offset { printf(b"%04x: \0".as_ptr() as _, offset*8); print_offset=false; } printf(b"%016llx\0".as_ptr() as _, *data.add(offset)); col+=1; if col&3==0 {printf(b"\n\0".as_ptr() as _); print_offset=true;} else {printf(b" \0".as_ptr() as _);} } if !print_offset {printf(b"\n\0".as_ptr() as _);} }

fn main() {
    let args: Vec<CString> = std::env::args_os().map(|s| CString::new(s.as_encoded_bytes()).unwrap()).collect();
    let mut atflag = AT_SYMLINK_NOFOLLOW; let mut mask = STATX_BASIC_STATS | STATX_BTIME; let mut raw = false;
    let mut stx: Statx;
    unsafe {
        for arg in args.iter().skip(1) {
            let s = arg.as_c_str();
            if s.to_bytes() == b"-F" { atflag &= !AT_STATX_SYNC_TYPE; atflag |= AT_STATX_FORCE_SYNC; continue; }
            if s.to_bytes() == b"-D" { atflag &= !AT_STATX_SYNC_TYPE; atflag |= AT_STATX_DONT_SYNC; continue; }
            if s.to_bytes() == b"-L" { atflag &= !AT_SYMLINK_NOFOLLOW; continue; }
            if s.to_bytes() == b"-O" { mask &= !STATX_BASIC_STATS; continue; }
            if s.to_bytes() == b"-A" { atflag |= AT_NO_AUTOMOUNT; continue; }
            if s.to_bytes() == b"-R" { raw = true; continue; }
            stx = std::mem::zeroed();
            let ret = statx(AT_FDCWD, s.as_ptr(), atflag as c_uint, mask, &mut stx);
            printf(b"statx(%s) = %d\n\0".as_ptr() as _, s.as_ptr(), ret as c_int);
            if ret < 0 { perror(s.as_ptr()); exit(1); }
            if raw { dump_hex(&stx as *const _ as *const u64, 0, std::mem::size_of::<Statx>() as c_int); }
            dump_statx(&stx);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
