// SPDX-License-Identifier: GPL-2.0
//! Initramfs extraction, initial RAM disk lifetime, and rootfs synchronization.
//!
//! The parser and decompressor callback are serialized by the rootfs async
//! domain (and by the existing initramfs KUnit tests). No parser reference is
//! retained while calling a decompressor, which can re-enter via flush_buffer.

#[allow(
    clippy::all,
    dead_code,
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    improper_ctypes,
    unreachable_pub,
    unsafe_op_in_unsafe_fn
)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/init_initramfs_generated.rs"
    ));
}

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
mod main_printk;
mod main_setup;

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use core::{mem, ptr};
use kernel::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use main_printk::main_printk;

const PATH_MAX: usize = bindings::PATH_MAX as usize;
const CPIO_HDRLEN: usize = bindings::CPIO_HDRLEN as usize;

// newc's 110-byte header ends two bytes short of a four-byte boundary.
const fn n_align(length: usize) -> usize {
    ((length + 1) & !3) + 2
}

type Hash = bindings::rust_initramfs_hash;
type Buffers = bindings::rust_initramfs_buffers;
#[cfg(CONFIG_INITRAMFS_PRESERVE_MTIME)]
type DirEntry = bindings::rust_initramfs_dir_entry;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Start,
    Collect,
    GotHeader,
    SkipIt,
    GotName,
    CopyFile,
    GotSymlink,
    Reset,
}

struct Parser {
    csum_present: bool,
    io_csum: u32,
    head: [*mut Hash; 32],
    hardlink_seen: bool,
    #[cfg(CONFIG_INITRAMFS_PRESERVE_MTIME)]
    dirs: *mut DirEntry,
    ino: u32,
    major: u32,
    minor: u32,
    nlink: u32,
    mode: bindings::umode_t,
    body_len: c_ulong,
    name_len: usize,
    uid: bindings::uid_t,
    gid: bindings::gid_t,
    rdev: u32,
    hdr_csum: u32,
    mtime: bindings::time64_t,
    state: State,
    next_state: State,
    victim: *mut c_char,
    byte_count: c_ulong,
    this_header: bindings::loff_t,
    next_header: bindings::loff_t,
    collected: *mut c_char,
    remains: usize,
    collect: *mut c_char,
    buffers: *mut Buffers,
    wfile: *mut bindings::file,
    wfile_pos: bindings::loff_t,
}

#[link_section = ".init.data"]
static mut PARSER: Parser = Parser {
    csum_present: false,
    io_csum: 0,
    head: [ptr::null_mut(); 32],
    hardlink_seen: false,
    #[cfg(CONFIG_INITRAMFS_PRESERVE_MTIME)]
    dirs: ptr::null_mut(),
    ino: 0,
    major: 0,
    minor: 0,
    nlink: 0,
    mode: 0,
    body_len: 0,
    name_len: 0,
    uid: 0,
    gid: 0,
    rdev: 0,
    hdr_csum: 0,
    mtime: 0,
    state: State::Start,
    next_state: State::Start,
    victim: ptr::null_mut(),
    byte_count: 0,
    this_header: 0,
    next_header: 0,
    collected: ptr::null_mut(),
    remains: 0,
    collect: ptr::null_mut(),
    buffers: ptr::null_mut(),
    wfile: ptr::null_mut(),
    wfile_pos: 0,
};

#[link_section = ".init.data"]
static mut MESSAGE: *mut c_char = ptr::null_mut();
#[link_section = ".init.data"]
static mut MY_INPTR: c_long = 0;

#[link_section = ".init.text"]
unsafe extern "C" fn error(text: *mut c_char) {
    // SAFETY: parser callbacks share the serialized extraction context.
    unsafe {
        if MESSAGE.is_null() {
            MESSAGE = text;
        }
    }
}

#[link_section = ".init.text"]
unsafe fn panic_show_mem(text: *const c_char) -> ! {
    // SAFETY: generated declarations preserve the kernel's actual ABI.
    unsafe {
        bindings::__show_mem(
            0,
            ptr::null(),
            bindings::RUST_INITRAMFS_MAX_NR_ZONES as c_int - 1,
        );
        bindings::panic(c"%s".as_ptr(), text)
    }
}

#[inline]
fn is_type(mode: bindings::umode_t, kind: u32) -> bool {
    mode as u32 & bindings::S_IFMT == kind
}

#[inline]
fn is_err<T>(pointer: *const T) -> bool {
    pointer as usize >= (-(bindings::MAX_ERRNO as isize)) as usize
}

#[link_section = ".init.text"]
unsafe fn do_utime(name: *mut c_char, mtime: bindings::time64_t) {
    #[cfg(CONFIG_INITRAMFS_PRESERVE_MTIME)]
    unsafe {
        let mut times: [bindings::timespec64; 2] = mem::zeroed();
        times[0].tv_sec = mtime;
        times[1].tv_sec = mtime;
        bindings::init_utimes(name, times.as_mut_ptr());
    }
    #[cfg(not(CONFIG_INITRAMFS_PRESERVE_MTIME))]
    let _ = (name, mtime);
}

#[link_section = ".init.text"]
unsafe fn do_utime_path(path: *const bindings::path, mtime: bindings::time64_t) {
    #[cfg(CONFIG_INITRAMFS_PRESERVE_MTIME)]
    unsafe {
        let mut times: [bindings::timespec64; 2] = mem::zeroed();
        times[0].tv_sec = mtime;
        times[1].tv_sec = mtime;
        bindings::vfs_utimes(path, times.as_mut_ptr());
    }
    #[cfg(not(CONFIG_INITRAMFS_PRESERVE_MTIME))]
    let _ = (path, mtime);
}

#[link_section = ".init.text"]
unsafe fn clean_path(name: *const c_char, mode: bindings::umode_t) {
    unsafe {
        let mut stat = mem::MaybeUninit::<bindings::kstat>::uninit();
        if bindings::init_stat(
            name,
            stat.as_mut_ptr(),
            bindings::AT_SYMLINK_NOFOLLOW as c_int,
        ) == 0
        {
            let old_mode = (*stat.as_ptr()).mode;
            if (old_mode ^ mode) as u32 & bindings::S_IFMT != 0 {
                if is_type(old_mode, bindings::S_IFDIR) {
                    bindings::init_rmdir(name);
                } else {
                    bindings::init_unlink(name);
                }
            }
        }
    }
}

impl Parser {
    #[link_section = ".init.text"]
    unsafe fn find_link(&mut self) -> *mut c_char {
        unsafe {
            // C does the initial arithmetic as int, then promotes to unsigned
            // long. Wrapping applies only to the deliberately modular hash.
            let major = self.major as c_int;
            let minor = self.minor as c_int;
            let ino = self.ino as c_int;
            let mut hash = ino.wrapping_add(minor).wrapping_add(major.wrapping_shl(3)) as c_ulong;
            hash = hash.wrapping_add(hash >> 5);
            let mut link = self.head.as_mut_ptr().add((hash & 31) as usize);
            while !(*link).is_null() {
                let node = *link;
                if (*node).ino == ino
                    && (*node).minor == minor
                    && (*node).major == major
                    && ((*node).mode ^ self.mode) as u32 & bindings::S_IFMT == 0
                {
                    return ptr::addr_of_mut!((*node).name).cast();
                }
                link = ptr::addr_of_mut!((*node).next);
            }
            let node = bindings::rust_initramfs_alloc_hash().cast::<Hash>();
            if node.is_null() {
                panic_show_mem(c"can't allocate link hash entry".as_ptr());
            }
            ptr::addr_of_mut!((*node).major).write(major);
            ptr::addr_of_mut!((*node).minor).write(minor);
            ptr::addr_of_mut!((*node).ino).write(ino);
            ptr::addr_of_mut!((*node).mode).write(self.mode);
            ptr::addr_of_mut!((*node).next).write(ptr::null_mut());
            // do_name has verified the trailing NUL and PATH_MAX bound.
            ptr::copy_nonoverlapping(
                self.collected,
                ptr::addr_of_mut!((*node).name).cast(),
                self.name_len,
            );
            link.write(node);
            self.hardlink_seen = true;
            ptr::null_mut()
        }
    }

    #[link_section = ".init.text"]
    unsafe fn free_hash(&mut self) {
        unsafe {
            if self.hardlink_seen {
                for bucket in &mut self.head {
                    while !bucket.is_null() {
                        let node = *bucket;
                        *bucket = (*node).next;
                        bindings::kfree(node.cast());
                    }
                }
            }
            self.hardlink_seen = false;
        }
    }

    #[link_section = ".init.text"]
    unsafe fn dir_add(&mut self) {
        #[cfg(CONFIG_INITRAMFS_PRESERVE_MTIME)]
        unsafe {
            // The typed flexible allocation uses the checked name_len.
            let entry = bindings::rust_initramfs_alloc_dir(self.name_len).cast::<DirEntry>();
            if entry.is_null() {
                panic_show_mem(c"can't allocate dir_entry buffer".as_ptr());
            }
            ptr::addr_of_mut!((*entry).next).write(self.dirs);
            ptr::addr_of_mut!((*entry).mtime).write(self.mtime);
            ptr::copy_nonoverlapping(
                self.collected,
                ptr::addr_of_mut!((*entry).name).cast(),
                self.name_len,
            );
            // list_add in C restores directories in reverse creation order.
            self.dirs = entry;
        }
    }

    #[link_section = ".init.text"]
    unsafe fn dir_utime(&mut self) {
        #[cfg(CONFIG_INITRAMFS_PRESERVE_MTIME)]
        unsafe {
            while !self.dirs.is_null() {
                let entry = self.dirs;
                self.dirs = (*entry).next;
                do_utime(ptr::addr_of_mut!((*entry).name).cast(), (*entry).mtime);
                bindings::kfree(entry.cast());
            }
        }
    }

    #[link_section = ".init.text"]
    unsafe fn parse_header(&mut self) -> c_int {
        unsafe {
            let mut header = [0u32; 13];
            let result = bindings::hex2bin(
                header.as_mut_ptr().cast(),
                self.collected.add(6),
                mem::size_of_val(&header),
            );
            if result != 0 {
                error(c"damaged header".as_ptr().cast_mut());
                return result;
            }
            self.ino = u32::from_be(header[0]);
            self.mode = u32::from_be(header[1]) as bindings::umode_t;
            self.uid = u32::from_be(header[2]);
            self.gid = u32::from_be(header[3]);
            self.nlink = u32::from_be(header[4]);
            self.mtime = u32::from_be(header[5]) as bindings::time64_t;
            self.body_len = u32::from_be(header[6]) as c_ulong;
            self.major = u32::from_be(header[7]);
            self.minor = u32::from_be(header[8]);
            // MKDEV then new_encode_dev, including dev_t's unsigned truncation.
            let dev: bindings::dev_t = u32::from_be(header[9]).wrapping_shl(bindings::MINORBITS)
                | u32::from_be(header[10]);
            let major = dev >> bindings::MINORBITS;
            let minor = dev & bindings::MINORMASK;
            self.rdev = (minor & 0xff) | (major << 8) | ((minor & !0xff) << 12);
            self.name_len = u32::from_be(header[11]) as usize;
            self.hdr_csum = u32::from_be(header[12]);
            0
        }
    }

    #[inline]
    unsafe fn eat(&mut self, count: u32) {
        unsafe {
            self.victim = self.victim.add(count as usize);
        }
        self.this_header += count as bindings::loff_t;
        self.byte_count -= count as c_ulong;
    }

    #[link_section = ".init.text"]
    unsafe fn read_into(&mut self, buffer: *mut c_char, size: usize, next: State) {
        unsafe {
            if self.byte_count >= size as c_ulong {
                self.collected = self.victim;
                self.eat(size as u32);
                self.state = next;
            } else {
                self.collect = buffer;
                self.collected = buffer;
                self.remains = size;
                self.next_state = next;
                self.state = State::Collect;
            }
        }
    }

    #[link_section = ".init.text"]
    unsafe fn do_start(&mut self) -> bool {
        unsafe {
            self.read_into(
                ptr::addr_of_mut!((*self.buffers).header).cast(),
                CPIO_HDRLEN,
                State::GotHeader,
            );
        }
        false
    }

    #[link_section = ".init.text"]
    unsafe fn do_collect(&mut self) -> bool {
        unsafe {
            let count = self.remains.min(self.byte_count as usize);
            ptr::copy_nonoverlapping(self.victim, self.collect, count);
            self.eat(count as u32);
            self.collect = self.collect.add(count);
            self.remains -= count;
            if self.remains != 0 {
                return true;
            }
            self.state = self.next_state;
            false
        }
    }

    #[link_section = ".init.text"]
    unsafe fn do_header(&mut self) -> bool {
        unsafe {
            let magic = core::slice::from_raw_parts(self.collected.cast::<u8>(), 6);
            if magic == b"070701" {
                self.csum_present = false;
            } else if magic == b"070702" {
                self.csum_present = true;
            } else {
                error(
                    if magic == b"070707" {
                        c"incorrect cpio method used: use -H newc option"
                    } else {
                        c"no cpio magic"
                    }
                    .as_ptr()
                    .cast_mut(),
                );
                return true;
            }
            if self.parse_header() != 0 {
                return true;
            }
            // The fields are at most u32; calculate in loff_t rather than
            // overflow a 32-bit usize before validating the pathname length.
            self.next_header =
                self.this_header + (((self.name_len as i64 + 1) & !3) + 2) + self.body_len as i64;
            self.next_header = (self.next_header + 3) & !3;
            self.state = State::SkipIt;
            if self.name_len == 0 || self.name_len > PATH_MAX {
                return false;
            }
            if is_type(self.mode, bindings::S_IFLNK) {
                if self.body_len > PATH_MAX as c_ulong {
                    return false;
                }
                self.collect = ptr::addr_of_mut!((*self.buffers).symlink).cast();
                self.collected = self.collect;
                self.remains = n_align(self.name_len) + self.body_len as usize;
                self.next_state = State::GotSymlink;
                self.state = State::Collect;
                return false;
            }
            if is_type(self.mode, bindings::S_IFREG) || self.body_len == 0 {
                self.read_into(
                    ptr::addr_of_mut!((*self.buffers).name).cast(),
                    n_align(self.name_len),
                    State::GotName,
                );
            }
            false
        }
    }

    #[link_section = ".init.text"]
    unsafe fn do_skip(&mut self) -> bool {
        unsafe {
            if self.this_header + (self.byte_count as i64) < self.next_header {
                self.eat(self.byte_count as u32);
                true
            } else {
                self.eat((self.next_header - self.this_header) as u32);
                self.state = self.next_state;
                false
            }
        }
    }

    #[link_section = ".init.text"]
    unsafe fn do_reset(&mut self) -> bool {
        unsafe {
            while self.byte_count != 0 && self.victim.read() == 0 {
                self.eat(1);
            }
            if self.byte_count != 0 && self.this_header & 3 != 0 {
                error(c"broken padding".as_ptr().cast_mut());
            }
            true
        }
    }

    #[link_section = ".init.text"]
    unsafe fn maybe_link(&mut self) -> c_int {
        unsafe {
            if self.nlink >= 2 {
                let old = self.find_link();
                if !old.is_null() {
                    clean_path(self.collected, 0);
                    return if bindings::init_link(old, self.collected) < 0 {
                        -1
                    } else {
                        1
                    };
                }
            }
            0
        }
    }

    #[link_section = ".init.text"]
    unsafe fn do_name(&mut self) -> bool {
        unsafe {
            self.state = State::SkipIt;
            self.next_state = State::Reset;
            if self.collected.add(self.name_len - 1).read() != 0 {
                main_printk!(
                    "do_name",
                    b"\x013initramfs name without nulterm: %.*s\n\0",
                    self.name_len as c_int,
                    self.collected
                );
                error(c"malformed archive".as_ptr().cast_mut());
                return true;
            }
            if bindings::strcmp(self.collected, c"TRAILER!!!".as_ptr()) == 0 {
                self.free_hash();
                return false;
            }
            clean_path(self.collected, self.mode);
            if is_type(self.mode, bindings::S_IFREG) {
                let linked = self.maybe_link();
                if linked >= 0 {
                    let mut flags = bindings::O_WRONLY | bindings::O_CREAT | bindings::O_LARGEFILE;
                    if linked != 1 {
                        flags |= bindings::O_TRUNC;
                    }
                    let file = bindings::filp_open(self.collected, flags as c_int, self.mode);
                    if is_err(file) {
                        return false;
                    }
                    self.wfile = file;
                    self.wfile_pos = 0;
                    self.io_csum = 0;
                    bindings::vfs_fchown(file, self.uid, self.gid);
                    bindings::vfs_fchmod(file, self.mode);
                    if self.body_len != 0 {
                        bindings::vfs_truncate(
                            ptr::addr_of!((*file).f_path),
                            self.body_len as bindings::loff_t,
                        );
                    }
                    self.state = State::CopyFile;
                }
            } else if is_type(self.mode, bindings::S_IFDIR) {
                bindings::init_mkdir(self.collected, self.mode);
                bindings::init_chown(self.collected, self.uid, self.gid, 0);
                bindings::init_chmod(self.collected, self.mode);
                self.dir_add();
            } else if is_type(self.mode, bindings::S_IFBLK)
                || is_type(self.mode, bindings::S_IFCHR)
                || is_type(self.mode, bindings::S_IFIFO)
                || is_type(self.mode, bindings::S_IFSOCK)
            {
                if self.maybe_link() == 0 {
                    bindings::init_mknod(self.collected, self.mode, self.rdev);
                    bindings::init_chown(self.collected, self.uid, self.gid, 0);
                    bindings::init_chmod(self.collected, self.mode);
                    do_utime(self.collected, self.mtime);
                }
            }
            false
        }
    }

    #[link_section = ".init.text"]
    unsafe fn xwrite(
        &mut self,
        file: *mut bindings::file,
        mut data: *const u8,
        mut count: usize,
        pos: *mut bindings::loff_t,
    ) -> isize {
        unsafe {
            let mut written = 0isize;
            while count != 0 {
                let result = bindings::kernel_write(file, data.cast(), count, pos);
                if result < 0 {
                    if result == -(bindings::EINTR as isize)
                        || result == -(bindings::EAGAIN as isize)
                    {
                        continue;
                    }
                    return if written != 0 { written } else { result };
                }
                if result == 0 {
                    break;
                }
                if self.csum_present {
                    for index in 0..result as usize {
                        self.io_csum = self.io_csum.wrapping_add(data.add(index).read() as u32);
                    }
                }
                data = data.add(result as usize);
                count -= result as usize;
                written += result;
            }
            written
        }
    }

    #[link_section = ".init.text"]
    unsafe fn do_copy(&mut self) -> bool {
        unsafe {
            let count = self.byte_count.min(self.body_len);
            let mut position = self.wfile_pos;
            let written = self.xwrite(
                self.wfile,
                self.victim.cast(),
                count as usize,
                &mut position,
            );
            self.wfile_pos = position;
            if written != count as isize {
                error(c"write error".as_ptr().cast_mut());
            }
            if self.byte_count >= self.body_len {
                do_utime_path(ptr::addr_of!((*self.wfile).f_path), self.mtime);
                bindings::fput(self.wfile);
                self.wfile = ptr::null_mut();
                if self.csum_present && self.io_csum != self.hdr_csum {
                    error(c"bad data checksum".as_ptr().cast_mut());
                }
                self.eat(self.body_len as u32);
                self.state = State::SkipIt;
                false
            } else {
                self.body_len -= count;
                self.eat(count as u32);
                true
            }
        }
    }

    #[link_section = ".init.text"]
    unsafe fn do_symlink(&mut self) -> bool {
        unsafe {
            if self.collected.add(self.name_len - 1).read() != 0 {
                main_printk!(
                    "do_symlink",
                    b"\x013initramfs symlink without nulterm: %.*s\n\0",
                    self.name_len as c_int,
                    self.collected
                );
                error(c"malformed archive".as_ptr().cast_mut());
                return true;
            }
            self.collected
                .add(n_align(self.name_len) + self.body_len as usize)
                .write(0);
            clean_path(self.collected, 0);
            bindings::init_symlink(self.collected.add(n_align(self.name_len)), self.collected);
            bindings::init_chown(
                self.collected,
                self.uid,
                self.gid,
                bindings::AT_SYMLINK_NOFOLLOW as c_int,
            );
            do_utime(self.collected, self.mtime);
            self.state = State::SkipIt;
            self.next_state = State::Reset;
            false
        }
    }

    #[link_section = ".init.text"]
    unsafe fn write_buffer(&mut self, buffer: *mut c_char, length: c_ulong) -> c_long {
        unsafe {
            self.byte_count = length;
            self.victim = buffer;
            loop {
                let stop = match self.state {
                    State::Start => self.do_start(),
                    State::Collect => self.do_collect(),
                    State::GotHeader => self.do_header(),
                    State::SkipIt => self.do_skip(),
                    State::GotName => self.do_name(),
                    State::CopyFile => self.do_copy(),
                    State::GotSymlink => self.do_symlink(),
                    State::Reset => self.do_reset(),
                };
                if stop {
                    return (length - self.byte_count) as c_long;
                }
            }
        }
    }
}

#[link_section = ".init.text"]
unsafe extern "C" fn flush_buffer(buffer: *mut c_void, mut length: c_ulong) -> c_long {
    unsafe {
        if !MESSAGE.is_null() {
            return -1;
        }
        let original = length as c_long;
        let mut buffer = buffer.cast::<c_char>();
        let parser = &mut *ptr::addr_of_mut!(PARSER);
        loop {
            let written = parser.write_buffer(buffer, length) as c_ulong;
            if written >= length || !MESSAGE.is_null() {
                break;
            }
            let byte = buffer.add(written as usize).read();
            if byte == b'0' as c_char || byte == 0 {
                buffer = buffer.add(written as usize);
                length -= written;
                parser.state = if byte == b'0' as c_char {
                    State::Start
                } else {
                    State::Reset
                };
            } else {
                error(c"junk within compressed archive".as_ptr().cast_mut());
                break;
            }
        }
        original
    }
}

/// Extract an uncompressed or concatenated/compressed newc archive.
///
/// # Safety
/// The caller serializes extraction during init and supplies `length` readable
/// bytes. Filesystem operations require the init filesystem context. This C ABI
/// is retained for init/initramfs_test.c, with its original tests unchanged.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn unpack_to_rootfs(
    mut buffer: *mut c_char,
    mut length: c_ulong,
) -> *mut c_char {
    unsafe {
        let buffers = bindings::rust_initramfs_alloc_buffers().cast::<Buffers>();
        if buffers.is_null() {
            panic_show_mem(c"can't allocate buffers".as_ptr());
        }
        PARSER.buffers = buffers;
        PARSER.state = State::Start;
        PARSER.this_header = 0;
        MESSAGE = ptr::null_mut();
        while MESSAGE.is_null() && length != 0 {
            let saved_offset = PARSER.this_header;
            if buffer.read() == b'0' as c_char && PARSER.this_header & 3 == 0 {
                PARSER.state = State::Start;
                let written =
                    (&mut *ptr::addr_of_mut!(PARSER)).write_buffer(buffer, length) as c_ulong;
                buffer = buffer.add(written as usize);
                length -= written;
                continue;
            }
            if buffer.read() == 0 {
                buffer = buffer.add(1);
                length -= 1;
                PARSER.this_header += 1;
                continue;
            }
            PARSER.this_header = 0;
            let mut name = ptr::null();
            let decompress =
                bindings::decompress_method(buffer.cast(), length as c_long, &mut name);
            bindings::rust_initramfs_detected_compression(name);
            // A decompressor writes its input-consumption count synchronously.
            // No Rust reference into PARSER is live across this callback edge.
            if let Some(decompress) = decompress {
                if decompress(
                    buffer.cast(),
                    length as c_long,
                    None,
                    Some(flush_buffer),
                    ptr::null_mut(),
                    ptr::addr_of_mut!(MY_INPTR),
                    Some(error),
                ) != 0
                {
                    error(c"decompressor failed".as_ptr().cast_mut());
                }
            } else if !name.is_null() {
                main_printk!(
                    "unpack_to_rootfs",
                    b"\x013compression method %s not configured\n\0",
                    name
                );
                error(c"decompressor failed".as_ptr().cast_mut());
            } else {
                error(
                    c"invalid magic at start of compressed archive"
                        .as_ptr()
                        .cast_mut(),
                );
            }
            if PARSER.state != State::Reset {
                error(c"junk at the end of compressed archive".as_ptr().cast_mut());
            }
            // Never perform pointer arithmetic using an invalid decompressor
            // result on a failed stream; all configured decoders follow this ABI.
            if MY_INPTR < 0 || MY_INPTR as c_ulong > length {
                error(c"decompressor failed".as_ptr().cast_mut());
                break;
            }
            PARSER.this_header = saved_offset + MY_INPTR as i64;
            buffer = buffer.add(MY_INPTR as usize);
            length -= MY_INPTR as c_ulong;
        }
        let parser = &mut *ptr::addr_of_mut!(PARSER);
        parser.dir_utime();
        parser.free_hash();
        // Match the C lifetime on interrupted or short archives: an unfinished
        // CopyFile is not closed by unpack_to_rootfs cleanup.
        bindings::kfree(buffers.cast());
        parser.buffers = ptr::null_mut();
        MESSAGE
    }
}

#[link_section = ".init.data"]
static mut DO_RETAIN_INITRD: bool = false;
#[link_section = ".init.data"]
static mut INITRAMFS_ASYNC: bool = true;

#[link_section = ".init.text"]
unsafe extern "C" fn retain_initrd_param(argument: *mut c_char) -> c_int {
    unsafe {
        if argument.read() != 0 {
            return 0;
        }
        DO_RETAIN_INITRD = true;
    }
    1
}
#[cfg(CONFIG_ARCH_HAS_KEEPINITRD)]
#[link_section = ".init.text"]
unsafe extern "C" fn keepinitrd_setup(_argument: *mut c_char) -> c_int {
    unsafe {
        DO_RETAIN_INITRD = true;
    }
    1
}
#[link_section = ".init.text"]
unsafe extern "C" fn initramfs_async_setup(argument: *mut c_char) -> c_int {
    unsafe { (bindings::kstrtobool(argument, ptr::addr_of_mut!(INITRAMFS_ASYNC)) == 0) as c_int }
}
main_setup::setup_param!(
    "retain_initrd",
    RETAIN_INITRD_SETUP,
    Some(retain_initrd_param),
    0
);
#[cfg(CONFIG_ARCH_HAS_KEEPINITRD)]
main_setup::setup_param!("keepinitrd", KEEPINITRD_SETUP, Some(keepinitrd_setup), 0);
main_setup::setup_param!(
    "initramfs_async=",
    INITRAMFS_ASYNC_SETUP,
    Some(initramfs_async_setup),
    0
);

/// Reserve every physical page intersecting the bootloader's RAM disk.
///
/// # Safety
/// Called during serialized architecture memory setup, before init memory is
/// released. The authoritative address state belongs to do_mounts_initrd.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn reserve_initrd_mem() {
    unsafe {
        bindings::initrd_start = 0;
        bindings::initrd_end = 0;
        if bindings::phys_initrd_size == 0 {
            return;
        }
        let page_size = bindings::RUST_INITRAMFS_PAGE_SIZE as c_ulong;
        let start = bindings::phys_initrd_start & !((page_size - 1) as bindings::phys_addr_t);
        // These unsigned calculations mirror round_down/round_up in C.
        let size = bindings::phys_initrd_size
            .wrapping_add((bindings::phys_initrd_start - start) as c_ulong);
        let size = size.wrapping_add(page_size - 1) & !(page_size - 1);
        let valid = if !bindings::memblock_is_region_memory(start, size as bindings::phys_addr_t) {
            main_printk!(
                "reserve_initrd_mem",
                b"\x013INITRD: 0x%08llx+0x%08lx is not a memory region\0",
                start as u64,
                size
            );
            false
        } else if bindings::memblock_is_region_reserved(start, size as bindings::phys_addr_t) {
            main_printk!(
                "reserve_initrd_mem",
                b"\x013INITRD: 0x%08llx+0x%08lx overlaps in-use memory region\n\0",
                start as u64,
                size
            );
            false
        } else {
            true
        };
        if !valid {
            main_printk!("reserve_initrd_mem", b"\x01c - disabling initrd\n\0");
            return;
        }
        bindings::__memblock_reserve(
            start,
            size as bindings::phys_addr_t,
            bindings::NUMA_NO_NODE,
            0,
        );
        bindings::initrd_start =
            bindings::rust_initramfs_phys_to_virt(bindings::phys_initrd_start) as c_ulong;
        bindings::initrd_end = bindings::initrd_start.wrapping_add(bindings::phys_initrd_size);
        bindings::initrd_below_start_ok = 1;
    }
}

/// Generic weak initial RAM disk release; architectures may override it.
///
/// # Safety
/// The range must refer to reserved initrd pages no longer needed by readers.
#[no_mangle]
#[linkage = "weak"]
#[link_section = ".init.text"]
pub unsafe extern "C" fn free_initrd_mem(start: c_ulong, end: c_ulong) {
    unsafe {
        bindings::free_reserved_area(
            start as *mut c_void,
            end as *mut c_void,
            bindings::POISON_FREE_INITMEM as c_int,
            c"initrd".as_ptr(),
        );
    }
}

#[link_section = ".init.text"]
unsafe fn kexec_free_initrd() -> bool {
    #[cfg(CONFIG_CRASH_RESERVE)]
    unsafe {
        let start = bindings::rust_initramfs_phys_to_virt(
            bindings::crashk_res.start as bindings::phys_addr_t,
        ) as c_ulong;
        let end = bindings::rust_initramfs_phys_to_virt(
            bindings::crashk_res.end as bindings::phys_addr_t,
        ) as c_ulong;
        if bindings::initrd_start >= end || bindings::initrd_end <= start {
            return false;
        }
        ptr::write_bytes(
            bindings::initrd_start as *mut u8,
            0,
            (bindings::initrd_end - bindings::initrd_start) as usize,
        );
        if bindings::initrd_start < start {
            bindings::free_initrd_mem(bindings::initrd_start, start);
        }
        if bindings::initrd_end > end {
            bindings::free_initrd_mem(end, bindings::initrd_end);
        }
        true
    }
    #[cfg(not(CONFIG_CRASH_RESERVE))]
    {
        false
    }
}

#[cfg(CONFIG_BLK_DEV_RAM = "y")]
#[link_section = ".init.text"]
unsafe fn populate_initrd_image(error: *const c_char) {
    unsafe {
        main_printk!(
            "populate_initrd_image",
            b"\x016rootfs image is not initramfs (%s); looks like an initrd\n\0",
            error
        );
        let file = bindings::filp_open(
            c"/initrd.image".as_ptr(),
            (bindings::O_WRONLY | bindings::O_CREAT | bindings::O_LARGEFILE) as c_int,
            0o700,
        );
        if is_err(file) {
            return;
        }
        let mut position = 0;
        let length = bindings::initrd_end - bindings::initrd_start;
        let written = (&mut *ptr::addr_of_mut!(PARSER)).xwrite(
            file,
            bindings::initrd_start as *const u8,
            length as usize,
            &mut position,
        );
        if written != length as isize {
            main_printk!(
                "populate_initrd_image",
                b"\x013/initrd.image: incomplete write (%zd != %ld)\n\0",
                written,
                length
            );
        }
        bindings::fput(file);
    }
}

#[link_section = ".init.text"]
unsafe fn unpack_initramfs(_cookie: bindings::async_cookie_t) {
    unsafe {
        let error = unpack_to_rootfs(
            ptr::addr_of_mut!(bindings::__initramfs_start).cast(),
            bindings::__initramfs_size,
        );
        if !error.is_null() {
            panic_show_mem(error);
        }
        if bindings::initrd_start == 0 || cfg!(CONFIG_INITRAMFS_FORCE) {
            return;
        }
        #[cfg(CONFIG_BLK_DEV_RAM)]
        main_printk!(
            "unpack_initramfs",
            b"\x016Trying to unpack rootfs image as initramfs...\n\0"
        );
        #[cfg(not(CONFIG_BLK_DEV_RAM))]
        main_printk!("unpack_initramfs", b"\x016Unpacking initramfs...\n\0");
        let error = unpack_to_rootfs(
            bindings::initrd_start as *mut c_char,
            bindings::initrd_end - bindings::initrd_start,
        );
        if !error.is_null() {
            #[cfg(CONFIG_BLK_DEV_RAM = "y")]
            populate_initrd_image(error);
            #[cfg(not(CONFIG_BLK_DEV_RAM = "y"))]
            main_printk!(
                "unpack_initramfs",
                b"\x010Initramfs unpacking failed: %s\n\0",
                error
            );
        }
    }
}

// Static storage must survive async completion and retained-initrd sysfs reads.
#[cfg(CONFIG_SYSFS)]
static mut BIN_ATTR_INITRD: bindings::bin_attribute = {
    let mut attribute: bindings::bin_attribute = unsafe { mem::zeroed() };
    attribute.attr.name = c"initrd".as_ptr();
    attribute.attr.mode = 0o440;
    attribute.read = Some(bindings::sysfs_bin_attr_simple_read);
    attribute
};

struct InitFsGuard(*mut bindings::fs_struct);
impl Drop for InitFsGuard {
    #[link_section = ".init.text"]
    fn drop(&mut self) {
        // SAFETY: this guard lives only in its creating async worker. The
        // helper restores the current task's previous filesystem context.
        unsafe {
            bindings::rust_initramfs_revert_init_fs(self.0);
        }
    }
}

#[link_section = ".init.text"]
unsafe extern "C" fn do_populate_rootfs(_unused: *mut c_void, cookie: bindings::async_cookie_t) {
    unsafe {
        {
            let _fs = InitFsGuard(bindings::rust_initramfs_override_init_fs());
            unpack_initramfs(cookie);
            #[cfg(CONFIG_SECURITY)]
            bindings::security_initramfs_populated();
        }
        if !DO_RETAIN_INITRD && bindings::initrd_start != 0 && !kexec_free_initrd() {
            bindings::free_initrd_mem(bindings::initrd_start, bindings::initrd_end);
        } else if DO_RETAIN_INITRD && bindings::initrd_start != 0 {
            #[cfg(CONFIG_SYSFS)]
            {
                BIN_ATTR_INITRD.size = (bindings::initrd_end - bindings::initrd_start) as usize;
                BIN_ATTR_INITRD.private = bindings::initrd_start as *mut c_void;
                if bindings::sysfs_create_bin_file(
                    bindings::firmware_kobj,
                    ptr::addr_of!(BIN_ATTR_INITRD),
                ) != 0
                {
                    main_printk!(
                        "do_populate_rootfs",
                        b"\x013Failed to create initrd sysfs file\0"
                    );
                }
            }
        }
        bindings::initrd_start = 0;
        bindings::initrd_end = 0;
        bindings::flush_delayed_fput();
        bindings::task_work_run();
    }
}

static mut INITRAMFS_DOMAIN: bindings::async_domain = {
    // ASYNC_DOMAIN_EXCLUSIVE: a self-linked list and registered == 0.
    let mut domain: bindings::async_domain = unsafe { mem::zeroed() };
    domain.pending.next = ptr::addr_of_mut!(INITRAMFS_DOMAIN.pending);
    domain.pending.prev = ptr::addr_of_mut!(INITRAMFS_DOMAIN.pending);
    domain
};
static INITRAMFS_COOKIE: AtomicU64 = AtomicU64::new(0);
#[link_section = ".data.once"]
static WAIT_WARNED: AtomicBool = AtomicBool::new(false);

/// Wait for the initial RAM filesystem population and deferred file releases.
///
/// # Safety
/// Must be called from a sleepable context, outside the population worker.
#[no_mangle]
pub unsafe extern "C" fn wait_for_initramfs() {
    let cookie = INITRAMFS_COOKIE.load(Ordering::Acquire);
    if cookie == 0 {
        if !WAIT_WARNED.swap(true, Ordering::Relaxed) {
            unsafe {
                main_printk!(
                    "wait_for_initramfs",
                    b"\x014wait_for_initramfs() called before rootfs_initcalls\n\0"
                );
            }
        }
        return;
    }
    unsafe {
        bindings::async_synchronize_cookie_domain(
            cookie.wrapping_add(1),
            ptr::addr_of_mut!(INITRAMFS_DOMAIN),
        );
    }
}
ffi_export::export_symbol!(wait_for_initramfs, wait_for_initramfs, "GPL", "");

#[link_section = ".init.text"]
#[cfg_attr(
    not(all(CONFIG_LTO_CLANG, CONFIG_HAVE_ARCH_PREL32_RELOCATIONS)),
    linkage = "internal"
)]
#[cfg_attr(
    all(CONFIG_LTO_CLANG, CONFIG_HAVE_ARCH_PREL32_RELOCATIONS),
    export_name = "__initstub__kmod_initramfs__0_798_populate_rootfsrootfs"
)]
unsafe extern "C" fn populate_rootfs() -> c_int {
    unsafe {
        let cookie = bindings::async_schedule_node_domain(
            Some(do_populate_rootfs),
            ptr::null_mut(),
            bindings::NUMA_NO_NODE,
            ptr::addr_of_mut!(INITRAMFS_DOMAIN),
        );
        INITRAMFS_COOKIE.store(cookie, Ordering::Release);
        bindings::__usermodehelper_set_disable_depth(bindings::umh_disable_depth_UMH_ENABLED);
        if !INITRAMFS_ASYNC {
            wait_for_initramfs();
        }
    }
    0
}

#[cfg(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS)]
#[used]
#[link_section = ".discard.addressable"]
static ADDRESSABLE: unsafe extern "C" fn() -> c_int = populate_rootfs;
#[cfg(all(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS, not(CONFIG_LTO_CLANG)))]
core::arch::global_asm!(
    ".pushsection .initcallrootfs.init,\"a\"",
    "__initcall__kmod_initramfs__0_798_populate_rootfsrootfs:",
    ".long {callback} - .", ".popsection", callback = sym populate_rootfs,
);
#[cfg(all(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS, CONFIG_LTO_CLANG))]
core::arch::global_asm!(
    ".pushsection .initcallrootfs.init..kmod_initramfs__0_798_populate_rootfs,\"a\"",
    "__initcall__kmod_initramfs__0_798_populate_rootfsrootfs:",
    ".long {callback} - .", ".popsection", callback = sym populate_rootfs,
);
#[cfg(not(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS))]
#[used]
#[linkage = "internal"]
#[export_name = "__initcall__kmod_initramfs__0_798_populate_rootfsrootfs"]
#[cfg_attr(not(CONFIG_LTO_CLANG), link_section = ".initcallrootfs.init")]
#[cfg_attr(
    CONFIG_LTO_CLANG,
    link_section = ".initcallrootfs.init..kmod_initramfs__0_798_populate_rootfs"
)]
static mut INITCALL: unsafe extern "C" fn() -> c_int = populate_rootfs;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
// RECONSTRUCTION-BASE: 0008179a1ee0b082fa3ead118187fc6f3569a9f2
