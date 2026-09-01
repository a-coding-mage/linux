// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/data.c. External types, constants, and functions
// are supplied by the surrounding perf sources and libc bindings.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type u64 = u64;
type mode_t = c_uint;
type uid_t = c_uint;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: c_ulong,
    pub d_off: c_long,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: mode_t,
    pub st_uid: uid_t,
    pub st_gid: uid_t,
    pub __pad0: c_int,
    pub st_rdev: u64,
    pub st_size: off_t,
}

#[repr(C)]
pub struct perf_data_file {
    pub path: *mut c_char,
    pub fd: c_int,
    pub size: u64,
    pub use_stdio: bool_,
    pub fptr: *mut FILE,
}

#[repr(C)]
pub struct perf_data_dir {
    pub version: c_int,
    pub files: *mut perf_data_file,
    pub nr: c_int,
}

#[repr(C)]
pub struct perf_data {
    pub path: *mut c_char,
    pub file: perf_data_file,
    pub dir: perf_data_dir,
    pub is_dir: bool_,
    pub is_pipe: bool_,
    pub open: bool_,
    pub force: bool_,
    pub in_place_update: bool_,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rlimit_action {
    NO_CHANGE = 0,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EMFILE: c_int = 24;
const ENOENT: c_int = 2;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const O_CLOEXEC: c_int = 0o2000000;
const S_IRUSR: mode_t = 0o400;
const S_IWUSR: mode_t = 0o200;
const S_IRWXU: mode_t = 0o700;
const S_IFMT: mode_t = 0o170000;
const S_IFDIR: mode_t = 0o040000;
const S_IFREG: mode_t = 0o100000;
const S_IFIFO: mode_t = 0o010000;
const STDIN_FILENO: c_int = 0;
const STDOUT_FILENO: c_int = 1;
const SEEK_SET: c_int = 0;
const PATH_MAX: usize = 4096;
const PERF_DIR_VERSION: c_int = 1;

unsafe extern "C" {
    static mut errno: c_int;

    fn fclose(stream: *mut FILE) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn free(ptr: *mut c_void);
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn feof(stream: *mut FILE) -> c_int;
    fn fseeko(stream: *mut FILE, offset: off_t, whence: c_int) -> c_int;
    fn ftello(stream: *mut FILE) -> off_t;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn rename(oldpath: *const c_char, newpath: *const c_char) -> c_int;
    fn mkdir(pathname: *const c_char, mode: mode_t) -> c_int;
    fn geteuid() -> uid_t;

    fn zfree(ptr: *mut *mut c_char);
    fn WARN_ON(condition: bool_) -> bool_;
    fn rlimit__increase_nofile(action: *mut rlimit_action) -> bool_;
    fn perf_data__is_single_file(data: *mut perf_data) -> bool_;
    fn perf_data__is_read(data: *mut perf_data) -> bool_;
    fn perf_data__is_write(data: *mut perf_data) -> bool_;
    fn perf_data__is_dir(data: *mut perf_data) -> bool_;
    fn perf_data__fd(data: *mut perf_data) -> c_int;
    fn rm_rf_perf_data(path: *const c_char) -> c_int;
    fn readn(fd: c_int, buf: *mut c_void, n: size_t) -> ssize_t;
    fn writen(fd: c_int, buf: *const c_void, n: size_t) -> ssize_t;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe fn S_ISREG(mode: mode_t) -> bool_ {
    (mode & S_IFMT) == S_IFREG
}

unsafe fn S_ISFIFO(mode: mode_t) -> bool_ {
    (mode & S_IFMT) == S_IFIFO
}

unsafe fn perf_data_file__close(file: *mut perf_data_file) {
    if (*file).use_stdio {
        if !(*file).fptr.is_null() {
            fclose((*file).fptr);
            (*file).fptr = core::ptr::null_mut();
        }
    } else {
        close((*file).fd);
        (*file).fd = -1;
    }
    zfree(&mut (*file).path);
}

unsafe fn close_dir(files: *mut perf_data_file, mut nr: c_int) {
    loop {
        nr -= 1;
        if nr < 0 {
            break;
        }
        perf_data_file__close(files.offset(nr as isize));
    }

    free(files as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn perf_data__close_dir(data: *mut perf_data) {
    close_dir((*data).dir.files, (*data).dir.nr);
    (*data).dir.files = core::ptr::null_mut();
    (*data).dir.nr = 0;
}

#[no_mangle]
pub unsafe extern "C" fn perf_data__create_dir(data: *mut perf_data, nr: c_int) -> c_int {
    let mut set_rlimit = rlimit_action::NO_CHANGE;
    let mut files: *mut perf_data_file = core::ptr::null_mut();
    let mut i: c_int;
    let mut ret: c_int;

    if WARN_ON(!(*data).is_dir) {
        return -EINVAL;
    }

    files = calloc(nr as size_t, core::mem::size_of::<perf_data_file>()) as *mut perf_data_file;
    if files.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < nr {
        let file = files.offset(i as isize);

        ret = asprintf(
            &mut (*file).path,
            b"%s/data.%d\0".as_ptr() as *const c_char,
            (*data).path,
            i,
        );
        if ret < 0 {
            ret = -ENOMEM;
            close_dir(files, i);
            return ret;
        }

        loop {
            ret = open(
                (*file).path,
                O_RDWR | O_CREAT | O_TRUNC,
                S_IRUSR | S_IWUSR,
            );
            if ret < 0 {
                /*
                 * If using parallel threads to collect data,
                 * perf record needs at least 6 fds per CPU.
                 * When we run out of them try to increase the limits.
                 */
                if errno == EMFILE && rlimit__increase_nofile(&mut set_rlimit) {
                    continue;
                }

                ret = -errno;
                close_dir(files, i);
                return ret;
            }
            break;
        }
        set_rlimit = rlimit_action::NO_CHANGE;

        (*file).fd = ret;
        i += 1;
    }

    (*data).dir.version = PERF_DIR_VERSION;
    (*data).dir.files = files;
    (*data).dir.nr = nr;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_data__open_dir(data: *mut perf_data) -> c_int {
    let mut files: *mut perf_data_file = core::ptr::null_mut();
    let mut dent: *mut dirent;
    let mut ret: c_int = -1;
    let dir: *mut DIR;
    let mut nr: c_int = 0;

    /*
     * Directory containing a single regular perf data file which is already
     * open, means there is nothing more to do here.
     */
    if perf_data__is_single_file(data) {
        return 0;
    }

    if WARN_ON(!(*data).is_dir) {
        return -EINVAL;
    }

    /* The version is provided by DIR_FORMAT feature. */
    if WARN_ON((*data).dir.version != PERF_DIR_VERSION) {
        return -1;
    }

    dir = opendir((*data).path);
    if dir.is_null() {
        return -EINVAL;
    }

    loop {
        dent = readdir(dir);
        if dent.is_null() {
            break;
        }

        let mut file: *mut perf_data_file;
        let mut path = [0 as c_char; PATH_MAX];
        let mut st: stat = core::mem::zeroed();

        snprintf(
            path.as_mut_ptr(),
            path.len(),
            b"%s/%s\0".as_ptr() as *const c_char,
            (*data).path,
            (*dent).d_name.as_ptr(),
        );
        if stat(path.as_ptr(), &mut st) != 0 {
            continue;
        }

        if !S_ISREG(st.st_mode)
            || strncmp(
                (*dent).d_name.as_ptr(),
                b"data.\0".as_ptr() as *const c_char,
                5,
            ) != 0
        {
            continue;
        }

        ret = -ENOMEM;

        file = realloc(
            files as *mut c_void,
            (nr as size_t + 1) * core::mem::size_of::<perf_data_file>(),
        ) as *mut perf_data_file;
        if file.is_null() {
            closedir(dir);
            close_dir(files, nr);
            return ret;
        }

        files = file;
        file = files.offset(nr as isize);
        nr += 1;

        (*file).path = strdup(path.as_ptr());
        (*file).fd = -1;
        (*file).size = st.st_size as u64;
        (*file).use_stdio = false;
        (*file).fptr = core::ptr::null_mut();

        if (*file).path.is_null() {
            closedir(dir);
            close_dir(files, nr);
            return ret;
        }

        ret = open((*file).path, O_RDONLY);
        if ret < 0 {
            ret = -errno;
            closedir(dir);
            close_dir(files, nr);
            return ret;
        }
        (*file).fd = ret;
    }

    closedir(dir);
    if files.is_null() {
        return -EINVAL;
    }

    (*data).dir.files = files;
    (*data).dir.nr = nr;
    0
}

unsafe fn check_pipe(data: *mut perf_data) -> bool_ {
    let mut st: stat = core::mem::zeroed();
    let mut is_pipe = false;
    let fd = if perf_data__is_read(data) {
        STDIN_FILENO
    } else {
        STDOUT_FILENO
    };

    if (*data).path.is_null() {
        if fstat(fd, &mut st) == 0 && S_ISFIFO(st.st_mode) {
            is_pipe = true;
        }
    } else if strcmp((*data).path, b"-\0".as_ptr() as *const c_char) == 0 {
        is_pipe = true;
    }

    if is_pipe {
        if (*data).file.use_stdio {
            let mode: *const c_char;

            mode = if perf_data__is_read(data) {
                b"r\0".as_ptr() as *const c_char
            } else {
                b"w\0".as_ptr() as *const c_char
            };
            (*data).file.fptr = fdopen(fd, mode);

            if (*data).file.fptr.is_null() {
                (*data).file.fd = fd;
                (*data).file.use_stdio = false;
            }

        /*
         * When is_pipe and data->file.fd is given, use given fd
         * instead of STDIN_FILENO or STDOUT_FILENO
         */
        } else if (*data).file.fd <= 0 {
            (*data).file.fd = fd;
        }
    }

    (*data).is_pipe = is_pipe;
    is_pipe
}

unsafe fn check_backup(data: *mut perf_data) -> c_int {
    let mut st: stat = core::mem::zeroed();

    if perf_data__is_read(data) {
        return 0;
    }

    if stat((*data).path, &mut st) == 0 && st.st_size != 0 {
        let mut oldname = [0 as c_char; PATH_MAX];
        let ret: c_int;

        snprintf(
            oldname.as_mut_ptr(),
            oldname.len(),
            b"%s.old\0".as_ptr() as *const c_char,
            (*data).path,
        );

        ret = rm_rf_perf_data(oldname.as_ptr());
        if ret != 0 {
            if ret == -2 {
                pr_err(
                    b"Can't remove old data: Unknown file found (%s)\n\0".as_ptr()
                        as *const c_char,
                    oldname.as_ptr(),
                );
            } else {
                pr_err(
                    b"Can't remove old data: %m (%s)\n\0".as_ptr() as *const c_char,
                    oldname.as_ptr(),
                );
            }
            return -1;
        }

        if rename((*data).path, oldname.as_ptr()) != 0 {
            pr_err(
                b"Can't move data: %m (%s to %s)\n\0".as_ptr() as *const c_char,
                (*data).path,
                oldname.as_ptr(),
            );
            return -1;
        }
    }

    0
}

unsafe fn is_dir(data: *mut perf_data) -> bool_ {
    let mut st: stat = core::mem::zeroed();

    if stat((*data).path, &mut st) != 0 {
        return false;
    }

    (st.st_mode & S_IFMT) == S_IFDIR
}

unsafe fn open_file_read(data: *mut perf_data) -> c_int {
    let flags = if (*data).in_place_update { O_RDWR } else { O_RDONLY };
    let mut st: stat = core::mem::zeroed();
    let fd: c_int;

    fd = open((*data).file.path, flags);
    if fd < 0 {
        let err = errno;

        pr_err(b"failed to open %s: %m\0".as_ptr() as *const c_char, (*data).file.path);
        if err == ENOENT
            && strcmp(
                (*data).file.path,
                b"perf.data\0".as_ptr() as *const c_char,
            ) == 0
        {
            pr_err(b"  (try 'perf record' first)\0".as_ptr() as *const c_char);
        }
        pr_err(b"\n\0".as_ptr() as *const c_char);
        return -err;
    }

    if fstat(fd, &mut st) < 0 {
        close(fd);
        return -1;
    }

    if !(*data).force && st.st_uid != 0 && st.st_uid != geteuid() {
        pr_err(
            b"File %s not owned by current user or root (use -f to override)\n\0".as_ptr()
                as *const c_char,
            (*data).file.path,
        );
        close(fd);
        return -1;
    }

    if st.st_size == 0 {
        pr_info(
            b"zero-sized data (%s), nothing to do!\n\0".as_ptr() as *const c_char,
            (*data).file.path,
        );
        close(fd);
        return -1;
    }

    (*data).file.size = st.st_size as u64;
    fd
}

unsafe fn open_file_write(data: *mut perf_data) -> c_int {
    let fd = open(
        (*data).file.path,
        O_CREAT | O_RDWR | O_TRUNC | O_CLOEXEC,
        S_IRUSR | S_IWUSR,
    );

    if fd < 0 {
        pr_err(
            b"failed to open %s : %m\n\0".as_ptr() as *const c_char,
            (*data).file.path,
        );
    }

    fd
}

unsafe fn open_file(data: *mut perf_data) -> c_int {
    let fd: c_int;

    fd = if perf_data__is_read(data) {
        open_file_read(data)
    } else {
        open_file_write(data)
    };

    if fd < 0 {
        zfree(&mut (*data).file.path);
        return -1;
    }

    (*data).file.fd = fd;
    0
}

unsafe fn open_file_dup(data: *mut perf_data) -> c_int {
    (*data).file.path = strdup((*data).path);
    if (*data).file.path.is_null() {
        return -ENOMEM;
    }

    open_file(data)
}

unsafe fn open_dir(data: *mut perf_data) -> c_int {
    let ret: c_int;

    /*
     * So far we open only the header, so we can read the data version and
     * layout.
     */
    if asprintf(
        &mut (*data).file.path,
        b"%s/data\0".as_ptr() as *const c_char,
        (*data).path,
    ) < 0
    {
        return -1;
    }

    if perf_data__is_write(data) && mkdir((*data).path, 0o700) < 0 {
        zfree(&mut (*data).file.path);
        return -1;
    }

    ret = open_file(data);

    /* Cleanup whatever we managed to create so far. */
    if ret != 0 && perf_data__is_write(data) {
        rm_rf_perf_data((*data).path);
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_data__open(data: *mut perf_data) -> c_int {
    let ret: c_int;

    if (*data).open {
        return 0;
    }

    if check_pipe(data) {
        (*data).open = true;
        return 0;
    }

    /* currently it allows stdio for pipe only */
    (*data).file.use_stdio = false;

    if (*data).path.is_null() {
        (*data).path = b"perf.data\0".as_ptr() as *mut c_char;
    }

    if check_backup(data) != 0 {
        return -1;
    }

    if perf_data__is_read(data) {
        (*data).is_dir = is_dir(data);
    }

    ret = if perf_data__is_dir(data) {
        open_dir(data)
    } else {
        open_file_dup(data)
    };

    if ret == 0 {
        (*data).open = true;
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_data__close(data: *mut perf_data) {
    if !(*data).open {
        return;
    }

    if perf_data__is_dir(data) {
        perf_data__close_dir(data);
    }

    perf_data_file__close(&mut (*data).file);
    (*data).open = false;
}

unsafe fn perf_data_file__read(
    file: *mut perf_data_file,
    buf: *mut c_void,
    size: size_t,
) -> ssize_t {
    if (*file).use_stdio {
        if fread(buf, size, 1, (*file).fptr) == 1 {
            return size as ssize_t;
        }
        return if feof((*file).fptr) != 0 { 0 } else { -1 };
    }
    readn((*file).fd, buf, size)
}

#[no_mangle]
pub unsafe extern "C" fn perf_data__read(
    data: *mut perf_data,
    buf: *mut c_void,
    size: size_t,
) -> ssize_t {
    perf_data_file__read(&mut (*data).file, buf, size)
}

#[no_mangle]
pub unsafe extern "C" fn perf_data_file__write(
    file: *mut perf_data_file,
    buf: *mut c_void,
    size: size_t,
) -> ssize_t {
    if (*file).use_stdio {
        if fwrite(buf, size, 1, (*file).fptr) == 1 {
            return size as ssize_t;
        }
        return -1;
    }
    writen((*file).fd, buf, size)
}

#[no_mangle]
pub unsafe extern "C" fn perf_data__write(
    data: *mut perf_data,
    buf: *mut c_void,
    size: size_t,
) -> ssize_t {
    perf_data_file__write(&mut (*data).file, buf, size)
}

#[no_mangle]
pub unsafe extern "C" fn perf_data_file__seek(
    file: *mut perf_data_file,
    offset: off_t,
    whence: c_int,
) -> off_t {
    if (*file).use_stdio {
        let res: off_t = fseeko((*file).fptr, offset, whence) as off_t;

        return if res < 0 { -1 } else { ftello((*file).fptr) };
    }
    lseek((*file).fd, offset, whence)
}

#[no_mangle]
pub unsafe extern "C" fn perf_data__seek(
    data: *mut perf_data,
    offset: off_t,
    whence: c_int,
) -> off_t {
    /* Note, a pipe fd will fail with -1 with errno of ESPIPE. */
    perf_data_file__seek(&mut (*data).file, offset, whence)
}

#[no_mangle]
pub unsafe extern "C" fn perf_data__switch(
    data: *mut perf_data,
    postfix: *const c_char,
    pos: size_t,
    at_exit: bool_,
    new_filepath: *mut *mut c_char,
) -> c_int {
    let mut ret: c_int;

    if perf_data__is_read(data) {
        return -EINVAL;
    }

    if asprintf(
        new_filepath,
        b"%s.%s\0".as_ptr() as *const c_char,
        (*data).path,
        postfix,
    ) < 0
    {
        return -ENOMEM;
    }

    /*
     * Only fire a warning, don't return error, continue fill
     * original file.
     */
    if rename((*data).path, *new_filepath) != 0 {
        pr_warning(
            b"Failed to rename %s to %s\n\0".as_ptr() as *const c_char,
            (*data).path,
            *new_filepath,
        );
    }

    if !at_exit {
        perf_data_file__close(&mut (*data).file);
        (*data).open = false;
        ret = perf_data__open(data);
        if ret < 0 {
            return ret;
        }

        if perf_data__seek(data, pos as off_t, SEEK_SET) == -1 as off_t {
            ret = -errno;
            pr_debug(
                b"Failed to seek to %zu: %m\0".as_ptr() as *const c_char,
                pos,
            );
            return ret;
        }
    }
    ret = perf_data__fd(data);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_data__size(data: *mut perf_data) -> c_ulong {
    let mut size: u64 = (*data).file.size;
    let mut i: c_int;

    if perf_data__is_single_file(data) {
        return size as c_ulong;
    }

    i = 0;
    while i < (*data).dir.nr {
        let file = (*data).dir.files.offset(i as isize);

        size = size.wrapping_add((*file).size);
        i += 1;
    }

    size as c_ulong
}

#[no_mangle]
pub unsafe extern "C" fn perf_data__make_kcore_dir(
    data: *mut perf_data,
    buf: *mut c_char,
    buf_sz: size_t,
) -> c_int {
    let ret: c_int;

    if !(*data).is_dir {
        return -1;
    }

    ret = snprintf(
        buf,
        buf_sz,
        b"%s/kcore_dir\0".as_ptr() as *const c_char,
        (*data).path,
    );
    if ret < 0 || ret as size_t >= buf_sz {
        return -1;
    }

    mkdir(buf, S_IRWXU)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
