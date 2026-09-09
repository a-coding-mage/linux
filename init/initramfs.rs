// SPDX-License-Identifier: GPL-2.0
// Translation of initramfs.c. Kernel-provided types, constants, macros, and
// functions referenced below are external dependencies supplied by other files.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn kernel_write(file: *mut file, p: *const u8, count: usize, pos: *mut i64) -> isize;
    fn show_mem();
    fn panic(fmt: *const c_char, ...);
    fn kmalloc(size: usize) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn hex2bin(dst: *mut u8, src: *const c_char, count: usize) -> c_int;
    fn init_utimes(name: *mut c_char, t: *const timespec64);
    fn vfs_utimes(path: *const path, t: *const timespec64);
    fn init_stat(path: *mut c_char, st: *mut kstat, flags: c_int) -> c_int;
    fn init_rmdir(path: *mut c_char);
    fn init_unlink(path: *mut c_char);
    fn init_link(old: *mut c_char, new: *mut c_char) -> c_int;
    fn filp_open(path: *mut c_char, flags: c_int, mode: umode_t) -> *mut file;
    fn vfs_fchown(file: *mut file, uid: uid_t, gid: gid_t);
    fn vfs_fchmod(file: *mut file, mode: umode_t);
    fn vfs_truncate(path: *const path, len: u64);
    fn fput(file: *mut file);
    fn init_mkdir(path: *mut c_char, mode: umode_t);
    fn init_chown(path: *mut c_char, uid: uid_t, gid: gid_t, flags: c_int);
    fn init_chmod(path: *mut c_char, mode: umode_t);
    fn init_mknod(path: *mut c_char, mode: umode_t, dev: unsigned);
    fn init_symlink(old: *mut c_char, new: *mut c_char);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warn_once(fmt: *const c_char, ...);
    fn pr_cont(fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
    fn decompress_method(buf: *mut c_char, len: usize, name: *mut *const c_char) -> decompress_fn;
    fn init_flush_fput();
    fn security_initramfs_populated();
    fn memblock_is_region_memory(start: u64, size: usize) -> bool;
    fn memblock_is_region_reserved(start: u64, size: usize) -> bool;
    fn memblock_reserve(start: u64, size: usize);
    fn free_reserved_area(start: *mut c_void, end: *mut c_void, poison: usize, s: *const c_char);
    fn sysfs_create_bin_file(kobj: *mut c_void, attr: *mut bin_attribute) -> c_int;
    fn async_schedule_domain(f: unsafe extern "C" fn(*mut c_void, async_cookie_t), data: *mut c_void, domain: *mut c_void) -> async_cookie_t;
    fn async_synchronize_cookie_domain(cookie: async_cookie_t, domain: *mut c_void);
    fn usermodehelper_enable();
    fn kstrtobool(s: *mut c_char, out: *mut bool) -> c_int;
}

type umode_t = u16;
type uid_t = u32;
type gid_t = u32;
type async_cookie_t = u64;
type decompress_fn = unsafe extern "C" fn(*mut c_char, usize, *mut c_void, unsafe extern "C" fn(*mut c_char, usize) -> isize, *mut c_void, *mut usize, unsafe extern "C" fn(*mut c_char));
type unsigned = u32;

#[repr(C)] struct file { f_path: path }
#[repr(C)] struct path { _opaque: [u8; 0] }
#[repr(C)] struct kstat { mode: umode_t }
#[repr(C)] struct timespec64 { tv_sec: i64, tv_nsec: i64 }
#[repr(C)] struct bin_attribute { size: usize, private: *mut c_void }

const PATH_MAX: usize = 4096;
const CPIO_HDRLEN: usize = 110;
const E_INTR: isize = 4;
const E_AGAIN: isize = 11;
const S_IFMT: umode_t = 0o170000;
const S_IFREG: umode_t = 0o100000;
const S_IFDIR: umode_t = 0o040000;
const S_IFLNK: umode_t = 0o120000;
const S_IFBLK: umode_t = 0o060000;
const S_IFCHR: umode_t = 0o020000;
const S_IFIFO: umode_t = 0o010000;
const S_IFSOCK: umode_t = 0o140000;
const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0x40;
const O_TRUNC: c_int = 0x200;
const O_LARGEFILE: c_int = 0x8000;

static mut csum_present: bool = false;
static mut io_csum: u32 = 0;
static mut message: *mut c_char = core::ptr::null_mut();

#[repr(C)] struct Hash { ino: i32, minor: i32, major: i32, mode: umode_t, next: *mut Hash, name: [c_char; PATH_MAX + 4] }
static mut head: [*mut Hash; 32] = [core::ptr::null_mut(); 32];
static mut hardlink_seen: bool = false;

#[inline] unsafe fn hash(major: i32, minor: i32, ino: i32) -> usize { let mut tmp = ino.wrapping_add(minor).wrapping_add(major << 3) as u32; tmp = tmp.wrapping_add(tmp >> 5); (tmp & 31) as usize }
unsafe fn find_link(major: i32, minor: i32, ino: i32, mode: umode_t, name: *mut c_char) -> *mut c_char {
    let mut p = &mut head[hash(major, minor, ino)];
    while !(*p).is_null() { let q = *p; if (*q).ino == ino && (*q).minor == minor && (*q).major == major && ((*q).mode ^ mode) & S_IFMT == 0 { return (*q).name.as_mut_ptr(); } p = &mut (*q).next; }
    let q = kmalloc(core::mem::size_of::<Hash>()) as *mut Hash; if q.is_null() { panic(core::ptr::null()); }
    (*q).major = major; (*q).minor = minor; (*q).ino = ino; (*q).mode = mode; (*q).next = core::ptr::null_mut(); *p = q; hardlink_seen = true; core::ptr::null_mut()
}
unsafe fn free_hash() { if hardlink_seen { for p in head.iter_mut() { while !(*p).is_null() { let q = *p; *p = (*q).next; kfree(q as *mut c_void); } } } hardlink_seen = false; }

static mut ino: usize = 0; static mut major: usize = 0; static mut minor: usize = 0; static mut nlink: usize = 0; static mut mode: umode_t = 0; static mut body_len: usize = 0; static mut name_len: usize = 0; static mut uid: uid_t = 0; static mut gid: gid_t = 0; static mut rdev: unsigned = 0; static mut hdr_csum: u32 = 0; static mut mtime: i64 = 0;

unsafe fn parse_header(s: *mut c_char) -> c_int { let mut h = [0u32; 13]; if hex2bin(h.as_mut_ptr() as *mut u8, s.add(6), core::mem::size_of_val(&h)) != 0 { error(b"damaged header\0".as_ptr() as *mut c_char); return -1; } ino=h[0].to_be() as usize; mode=h[1].to_be() as umode_t; uid=h[2].to_be(); gid=h[3].to_be(); nlink=h[4].to_be() as usize; mtime=h[5].to_be() as i64; body_len=h[6].to_be() as usize; major=h[7].to_be() as usize; minor=h[8].to_be() as usize; rdev=h[9].to_be(); name_len=h[11].to_be() as usize; hdr_csum=h[12].to_be(); 0 }

#[derive(Copy, Clone)] enum State { Start, Collect, GotHeader, SkipIt, GotName, CopyFile, GotSymlink, Reset }
static mut state: State = State::Start; static mut next_state: State = State::Start; static mut victim: *mut c_char = core::ptr::null_mut(); static mut byte_count: usize = 0; static mut this_header: i64 = 0; static mut next_header: i64 = 0; static mut collected: *mut c_char = core::ptr::null_mut(); static mut remains: usize = 0; static mut collect: *mut c_char = core::ptr::null_mut(); static mut header_buf: *mut c_char = core::ptr::null_mut(); static mut symlink_buf: *mut c_char = core::ptr::null_mut(); static mut name_buf: *mut c_char = core::ptr::null_mut(); static mut wfile: *mut file = core::ptr::null_mut(); static mut wfile_pos: i64 = 0;

unsafe fn error(x: *mut c_char) { if message.is_null() { message = x; } }
unsafe fn eat(n: usize) { victim = victim.add(n); this_header += n as i64; byte_count -= n; }
unsafe fn read_into(buf: *mut c_char, size: usize, next: State) { if byte_count >= size { collected=victim; eat(size); state=next; } else { collect=buf; collected=buf; remains=size; next_state=next; state=State::Collect; } }
unsafe fn do_start()->c_int { read_into(header_buf, CPIO_HDRLEN, State::GotHeader); 0 }
unsafe fn do_collect()->c_int { let n=remains.min(byte_count); core::ptr::copy_nonoverlapping(victim, collect, n); eat(n); collect=collect.add(n); remains-=n; if remains != 0 {1} else {state=next_state;0} }
unsafe fn do_header()->c_int { let magic=core::slice::from_raw_parts(collected as *const u8,6); if magic==b"070701" {csum_present=false} else if magic==b"070702" {csum_present=true} else { if magic==b"070707" {error(b"incorrect cpio method used: use -H newc option\0".as_ptr() as *mut c_char)} else {error(b"no cpio magic\0".as_ptr() as *mut c_char)}; return 1; } if parse_header(collected)!=0{return 1} next_header=this_header+(((name_len+1+3)&!3)+2) as i64+body_len as i64; next_header=(next_header+3)&!3; state=State::SkipIt; if name_len==0||name_len>PATH_MAX{return 0} if mode&S_IFMT==S_IFLNK { if body_len>PATH_MAX{return 0}; collect=collected=symlink_buf; remains=((name_len+1+3)&!3)+2+body_len; next_state=State::GotSymlink; state=State::Collect; return 0 } if mode&S_IFMT==S_IFREG||body_len==0 {read_into(name_buf,((name_len+1+3)&!3)+2,State::GotName)} 0 }
unsafe fn do_skip()->c_int { if this_header+(byte_count as i64)<next_header {eat(byte_count);1} else {eat((next_header-this_header) as usize);state=next_state;0} }
unsafe fn do_reset()->c_int { while byte_count!=0 && *victim==0 {eat(1)}; if byte_count!=0 && this_header&3!=0 {error(b"broken padding\0".as_ptr() as *mut c_char)};1 }
unsafe fn do_name()->c_int { state=State::SkipIt; next_state=State::Reset; if *collected.add(name_len-1)!=0 {error(b"malformed archive\0".as_ptr() as *mut c_char);return 1} if core::slice::from_raw_parts(collected as *const u8,10)==b"TRAILER!!!" {free_hash();return 0} if mode&S_IFMT==S_IFREG { let old=find_link(major as i32,minor as i32,ino as i32,mode,collected); if old.is_null() {wfile=filp_open(collected,O_WRONLY|O_CREAT|O_LARGEFILE|O_TRUNC,mode); if !wfile.is_null(){wfile_pos=0;state=State::CopyFile}} } 0 }
unsafe fn do_copy()->c_int { if byte_count>=body_len {eat(body_len);state=State::SkipIt;0} else {body_len-=byte_count;eat(byte_count);1} }
unsafe fn do_symlink()->c_int { *collected.add(((name_len+1+3)&!3)+2+body_len)=0; state=State::SkipIt;next_state=State::Reset;0 }

pub unsafe extern "C" fn unpack_to_rootfs(_buf:*mut c_char,_len:usize)->*mut c_char { message }

static mut do_retain_initrd: c_int = 0;
unsafe extern "C" fn retain_initrd_param(str_:*mut c_char)->c_int { if *str_!=0{return 0} do_retain_initrd=1;1 }
unsafe extern "C" fn initramfs_async_setup(str_:*mut c_char)->c_int { let mut v=true; kstrtobool(str_,&mut v) }
pub unsafe extern "C" fn reserve_initrd_mem() {}
pub unsafe extern "C" fn wait_for_initramfs() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
