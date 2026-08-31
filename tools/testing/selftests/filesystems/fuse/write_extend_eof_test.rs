// SPDX-License-Identifier: GPL-2.0
/*
 * Regression test for the fuse write-extend partial-EOF-page zeroing bug.
 *
 * A buffered write that extends i_size past a non-page-aligned EOF must zero
 * the tail of the old last page.  If an application has mmap'd that page and
 * stored into the post-EOF region (undefined until the file grows), the
 * now-in-bounds tail must read back as zero, not as the stale stored bytes.
 *
 * The bug is exposed on a non-writeback_cache server that keeps the page cache
 * across the write (FOPEN_KEEP_CACHE without FOPEN_DIRECT_IO).  This test is a
 * raw /dev/fuse server in that mode; the backing data is always zero in the
 * hole, so any non-zero byte a read sees is stale page-cache data.
 *
 * Requires root to mount fuse.
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

const FUSE_ROOT_ID: u64 = 1;
const FILE_INO: u64 = 2;
const MAX_WRITE: usize = 128 * 1024;
const BACKING_SIZE: usize = 4 * 1024 * 1024;
const POLLUTE: c_int = 0xee;

/* Server-side state, shared with the responder thread. */
#[repr(C)]
struct server {
    fd: c_int,
    backing: [u8; BACKING_SIZE], /* authoritative bytes */
    size: u64,
}

unsafe fn reply(fd: c_int, unique: u64, error: c_int, data: *mut c_void, len: usize) {
    let mut oh = fuse_out_header {
        len: (size_of::<fuse_out_header>() + if !data.is_null() { len } else { 0 }) as u32,
        error,
        unique,
    };
    let mut iov = [
        iovec {
            iov_base: &mut oh as *mut fuse_out_header as *mut c_void,
            iov_len: size_of::<fuse_out_header>(),
        },
        iovec {
            iov_base: data,
            iov_len: len,
        },
    ];

    /* Errors here are teardown races (device closed on unmount); ignore. */
    if writev(fd, iov.as_mut_ptr(), if !data.is_null() { 2 } else { 1 }) < 0 {
        return;
    }
}

unsafe fn fill_attr(a: *mut fuse_attr, ino: u64, mode: u32, size: u64) {
    memset(
        a as *mut c_void,
        0,
        size_of::<fuse_attr>(),
    );
    (*a).ino = ino;
    (*a).mode = mode;
    (*a).nlink = 1;
    (*a).size = size;
    (*a).blksize = sysconf(_SC_PAGESIZE) as u32;
}

unsafe extern "C" fn server_thread(arg: *mut c_void) -> *mut c_void {
    let s = arg as *mut server;
    static mut BUF: [c_char; MAX_WRITE + 4096] = [0; MAX_WRITE + 4096];

    loop {
        let n = read(
            (*s).fd,
            core::ptr::addr_of_mut!(BUF) as *mut c_void,
            size_of_val(&BUF),
        );
        let ih = core::ptr::addr_of_mut!(BUF) as *mut fuse_in_header;

        if n < 0 {
            if *__errno_location() == EINTR || *__errno_location() == EAGAIN {
                continue;
            }
            return ptr::null_mut(); /* device closed on unmount */
        }
        if n < size_of::<fuse_in_header>() as isize {
            continue;
        }

        match (*ih).opcode {
            FUSE_INIT => {
                let in_ = ih.add(1) as *mut fuse_init_in;
                let mut out: fuse_init_out = core::mem::zeroed();

                /* No FUSE_WRITEBACK_CACHE: the exposed configuration. */
                out.major = FUSE_KERNEL_VERSION;
                out.minor = FUSE_KERNEL_MINOR_VERSION;
                out.max_readahead = (*in_).max_readahead;
                out.max_write = MAX_WRITE as u32;
                out.max_background = 16;
                out.congestion_threshold = 12;
                out.flags = FUSE_MAX_PAGES;
                out.max_pages = (MAX_WRITE as c_long / sysconf(_SC_PAGESIZE)) as u16;
                reply(
                    (*s).fd,
                    (*ih).unique,
                    0,
                    &mut out as *mut fuse_init_out as *mut c_void,
                    size_of::<fuse_init_out>(),
                );
            }
            FUSE_GETATTR => {
                let mut out: fuse_attr_out = core::mem::zeroed();
                let root = (*ih).nodeid == FUSE_ROOT_ID;

                out.attr_valid = 3600;
                fill_attr(
                    &mut out.attr,
                    (*ih).nodeid,
                    if root { S_IFDIR | 0o755 } else { S_IFREG | 0o644 },
                    if root { 0 } else { (*s).size },
                );
                reply(
                    (*s).fd,
                    (*ih).unique,
                    0,
                    &mut out as *mut fuse_attr_out as *mut c_void,
                    size_of::<fuse_attr_out>(),
                );
            }
            FUSE_LOOKUP => {
                let mut out: fuse_entry_out = core::mem::zeroed();

                out.nodeid = FILE_INO;
                out.attr_valid = 3600;
                out.entry_valid = 3600;
                fill_attr(&mut out.attr, FILE_INO, S_IFREG | 0o644, (*s).size);
                reply(
                    (*s).fd,
                    (*ih).unique,
                    0,
                    &mut out as *mut fuse_entry_out as *mut c_void,
                    size_of::<fuse_entry_out>(),
                );
            }
            FUSE_OPEN | FUSE_OPENDIR => {
                let mut out: fuse_open_out = core::mem::zeroed();

                /* Keep the cache across the write, but not direct I/O. */
                out.open_flags = FOPEN_KEEP_CACHE;
                reply(
                    (*s).fd,
                    (*ih).unique,
                    0,
                    &mut out as *mut fuse_open_out as *mut c_void,
                    size_of::<fuse_open_out>(),
                );
            }
            FUSE_READ => {
                let in_ = ih.add(1) as *mut fuse_read_in;
                let off = (*in_).offset;
                let mut size = (*in_).size;

                if off >= BACKING_SIZE as u64 {
                    size = 0;
                } else if off + size as u64 > BACKING_SIZE as u64 {
                    size = (BACKING_SIZE as u64 - off) as u32;
                }
                reply(
                    (*s).fd,
                    (*ih).unique,
                    0,
                    (*s).backing.as_mut_ptr().add(off as usize) as *mut c_void,
                    size as usize,
                );
            }
            FUSE_WRITE => {
                let in_ = ih.add(1) as *mut fuse_write_in;
                let mut out: fuse_write_out = core::mem::zeroed();
                let off = (*in_).offset;
                let size = (*in_).size;

                if off < BACKING_SIZE as u64 {
                    let mut c = size;

                    if off + c as u64 > BACKING_SIZE as u64 {
                        c = (BACKING_SIZE as u64 - off) as u32;
                    }
                    memcpy(
                        (*s).backing.as_mut_ptr().add(off as usize) as *mut c_void,
                        in_.add(1) as *const c_void,
                        c as usize,
                    );
                    if off + c as u64 > (*s).size {
                        (*s).size = off + c as u64;
                    }
                }
                out.size = size;
                reply(
                    (*s).fd,
                    (*ih).unique,
                    0,
                    &mut out as *mut fuse_write_out as *mut c_void,
                    size_of::<fuse_write_out>(),
                );
            }
            FUSE_SETATTR => {
                let in_ = ih.add(1) as *mut fuse_setattr_in;
                let mut out: fuse_attr_out = core::mem::zeroed();

                if ((*in_).valid & FATTR_SIZE as u32) != 0 && (*in_).size <= BACKING_SIZE as u64 {
                    if (*in_).size > (*s).size {
                        memset(
                            (*s).backing.as_mut_ptr().add((*s).size as usize) as *mut c_void,
                            0,
                            ((*in_).size - (*s).size) as usize,
                        );
                    }
                    (*s).size = (*in_).size;
                }
                out.attr_valid = 3600;
                fill_attr(&mut out.attr, (*ih).nodeid, S_IFREG | 0o644, (*s).size);
                reply(
                    (*s).fd,
                    (*ih).unique,
                    0,
                    &mut out as *mut fuse_attr_out as *mut c_void,
                    size_of::<fuse_attr_out>(),
                );
            }
            FUSE_FALLOCATE => {
                let in_ = ih.add(1) as *mut fuse_fallocate_in;
                let end = (*in_).offset + (*in_).length;

                /* Only plain (size-extending) fallocate is used here. */
                if ((*in_).mode & FALLOC_FL_KEEP_SIZE as u32) == 0
                    && end <= BACKING_SIZE as u64
                    && end > (*s).size
                {
                    memset(
                        (*s).backing.as_mut_ptr().add((*s).size as usize) as *mut c_void,
                        0,
                        (end - (*s).size) as usize,
                    );
                    (*s).size = end;
                }
                reply((*s).fd, (*ih).unique, 0, ptr::null_mut(), 0);
            }
            FUSE_FLUSH | FUSE_RELEASE | FUSE_RELEASEDIR | FUSE_FSYNC | FUSE_ACCESS => {
                reply((*s).fd, (*ih).unique, 0, ptr::null_mut(), 0);
            }
            FUSE_FORGET => {}
            _ => {
                reply((*s).fd, (*ih).unique, -EOPNOTSUPP, ptr::null_mut(), 0);
            }
        }
    }
}

FIXTURE!(fuse {
    srv: *mut server,
    thread: pthread_t,
    dir: [c_char; 64],
    page: c_long,      /* runtime page size */
    eof: off_t,        /* mid-page EOF, page-relative */
    fd: c_int,         /* open test file */
    map: *mut c_char,  /* mmap of the EOF page */
    mounted: c_int,
});

FIXTURE_SETUP!(fuse, {
    let mut opts: [c_char; 128] = [0; 128];
    let mut t: pthread_t = core::mem::zeroed();

    if geteuid() != 0 {
        SKIP!(return, "need root to mount fuse");
    }

    self_.page = sysconf(_SC_PAGESIZE);
    self_.fd = -1;
    self_.map = MAP_FAILED as *mut c_char;

    self_.srv = mmap(
        ptr::null_mut(),
        size_of_val(&*self_.srv),
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut server;
    ASSERT_NE!(MAP_FAILED, self_.srv as *mut c_void);

    (*self_.srv).fd = open(c"/dev/fuse".as_ptr(), O_RDWR);
    ASSERT_GE!((*self_.srv).fd, 0);

    strcpy(self_.dir.as_mut_ptr(), c"/tmp/fuse_weof_XXXXXX".as_ptr());
    ASSERT_NE!(ptr::null_mut::<c_char>(), mkdtemp(self_.dir.as_mut_ptr()));

    snprintf(
        opts.as_mut_ptr(),
        opts.len(),
        c"fd=%d,rootmode=40000,user_id=0,group_id=0".as_ptr(),
        (*self_.srv).fd,
    );
    ASSERT_EQ!(
        0,
        mount(
            c"fuse".as_ptr() as *const c_void,
            self_.dir.as_ptr() as *const c_void,
            c"fuse".as_ptr() as *const c_void,
            0,
            opts.as_ptr() as *const c_void,
        )
    );
    self_.mounted = 1;

    ASSERT_EQ!(
        0,
        pthread_create(&mut t, ptr::null(), Some(server_thread), self_.srv as *mut c_void)
    );
    self_.thread = t;
});

FIXTURE_TEARDOWN!(fuse, {
    if self_.map != MAP_FAILED as *mut c_char {
        munmap(self_.map as *mut c_void, self_.page as usize);
    }
    if self_.fd >= 0 {
        close(self_.fd);
    }
    if self_.mounted != 0 {
        umount2(self_.dir.as_ptr(), MNT_DETACH);
    }
    if !self_.srv.is_null() && self_.srv != MAP_FAILED as *mut server {
        if (*self_.srv).fd > 0 {
            close((*self_.srv).fd);
        }
        munmap(self_.srv as *mut c_void, size_of_val(&*self_.srv));
    }
    if self_.dir[0] != 0 {
        rmdir(self_.dir.as_ptr());
    }
});

/*
 * Create the test file with a mid-page EOF and mmap-store POLLUTE into its
 * post-EOF tail (a legal store, undefined until the file grows).  Leaves the
 * file open and the EOF page mapped in the fixture for the caller to extend.
 */
unsafe fn pollute_eof_tail(_metadata: *mut __test_metadata, self_: *mut FIXTURE_DATA_fuse) {
    let eof: off_t = 2 * (*self_).page + (*self_).page / 4;
    let mut path: [c_char; 128] = [0; 128];
    let mut buf: *mut c_char;

    snprintf(path.as_mut_ptr(), path.len(), c"%s/file".as_ptr(), (*self_).dir.as_ptr());
    (*self_).fd = open(path.as_ptr(), O_RDWR | O_CREAT | O_TRUNC, 0o644);
    ASSERT_GE!((*self_).fd, 0);
    (*self_).eof = eof;

    buf = malloc(eof as usize) as *mut c_char;
    ASSERT_NE!(ptr::null_mut::<c_char>(), buf);
    memset(buf as *mut c_void, b'A' as c_int, eof as usize);
    ASSERT_EQ!(eof, pwrite((*self_).fd, buf as *const c_void, eof as usize, 0));
    free(buf as *mut c_void);

    (*self_).map = mmap(
        ptr::null_mut(),
        (*self_).page as usize,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        (*self_).fd,
        (*self_).eof & !((*self_).page - 1),
    ) as *mut c_char;
    ASSERT_NE!(MAP_FAILED, (*self_).map as *mut c_void);
    memset(
        (*self_).map.add((eof & ((*self_).page - 1)) as usize) as *mut c_void,
        POLLUTE,
        ((*self_).page - (eof & ((*self_).page - 1))) as usize,
    );
}

/* Assert the old post-EOF tail [eof, end of its page) now reads back as zero. */
unsafe fn assert_tail_zeroed(_metadata: *mut __test_metadata, self_: *mut FIXTURE_DATA_fuse) {
    let base: off_t = (*self_).eof & !((*self_).page - 1);
    let tail = malloc((*self_).page as usize) as *mut c_char;
    let mut i: c_int;

    ASSERT_NE!(ptr::null_mut::<c_char>(), tail);
    ASSERT_EQ!(
        (*self_).page,
        pread((*self_).fd, tail as *mut c_void, (*self_).page as usize, base)
    );
    i = ((*self_).eof & ((*self_).page - 1)) as c_int;
    while i < (*self_).page as c_int {
        ASSERT_EQ!(0, *tail.add(i as usize));
        i += 1;
    }
    free(tail as *mut c_void);
}

/* Basic: pollute the post-EOF tail, extend past it by a later write. */
TEST_F!(fuse, write_extend, {
    pollute_eof_tail(_metadata, self_);
    ASSERT_EQ!(
        4,
        pwrite(
            self_.fd,
            c"data".as_ptr() as *const c_void,
            4,
            5 * self_.page + self_.page / 3,
        )
    );
    assert_tail_zeroed(_metadata, self_);
});

/* Extend via ftruncate() rather than a write. */
TEST_F!(fuse, ftruncate_extend, {
    pollute_eof_tail(_metadata, self_);
    ASSERT_EQ!(0, ftruncate(self_.fd, 8 * self_.page));
    assert_tail_zeroed(_metadata, self_);
});

/* Extend via fallocate() starting at the old EOF. */
TEST_F!(fuse, fallocate_extend, {
    pollute_eof_tail(_metadata, self_);
    ASSERT_EQ!(0, fallocate(self_.fd, 0, self_.eof, 4 * self_.page));
    assert_tail_zeroed(_metadata, self_);
});

/* A write landing inside the old EOF page must not clobber its own data. */
TEST_F!(fuse, extend_into_eof_page_preserves_data, {
    let base: off_t;
    let wr: off_t;
    let buf: *mut c_char;
    let rd: *mut c_char;
    let mut i: c_int;

    pollute_eof_tail(_metadata, self_);
    base = self_.eof & !(self_.page - 1);
    wr = base + 3 * self_.page / 4;     /* starts in the EOF page */

    buf = malloc((2 * self_.page) as usize) as *mut c_char;
    ASSERT_NE!(ptr::null_mut::<c_char>(), buf);
    memset(buf as *mut c_void, b'B' as c_int, (2 * self_.page) as usize);
    ASSERT_EQ!(
        2 * self_.page,
        pwrite(self_.fd, buf as *const c_void, (2 * self_.page) as usize, wr)
    );
    free(buf as *mut c_void);

    rd = malloc(self_.page as usize) as *mut c_char;
    ASSERT_NE!(ptr::null_mut::<c_char>(), rd);
    ASSERT_EQ!(
        self_.page,
        pread(self_.fd, rd as *mut c_void, self_.page as usize, base)
    );
    /* [eof, wr) is hole -> zero; [wr, page) is written data -> 'B'. */
    i = (self_.eof & (self_.page - 1)) as c_int;
    while i < (wr - base) as c_int {
        ASSERT_EQ!(0, *rd.add(i as usize));
        i += 1;
    }
    i = (wr - base) as c_int;
    while i < self_.page as c_int {
        ASSERT_EQ!(b'B' as c_char, *rd.add(i as usize));
        i += 1;
    }
    free(rd as *mut c_void);
});

TEST_HARNESS_MAIN!();
