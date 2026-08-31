// SPDX-License-Identifier: GPL-2.0
//
// Translated from lib/subcmd/help.c. C include dependencies are represented as
// external declarations where this file references symbols supplied elsewhere.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

#[repr(C)]
pub struct cmdname {
    pub len: usize,
    pub name: [c_char; 0],
}

#[repr(C)]
pub struct cmdnames {
    pub names: *mut *mut cmdname,
    pub cnt: c_uint,
    pub alloc: c_uint,
}

#[repr(C)]
pub struct winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

#[repr(C)]
pub struct stat {
    pub st_dev: c_ulong,
    pub st_ino: c_ulong,
    pub st_nlink: c_ulong,
    pub st_mode: c_uint,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad0: c_int,
    pub st_rdev: c_ulong,
    pub st_size: isize,
    pub st_blksize: isize,
    pub st_blocks: isize,
    pub st_atime: isize,
    pub st_atime_nsec: isize,
    pub st_mtime: isize,
    pub st_mtime_nsec: isize,
    pub st_ctime: isize,
    pub st_ctime_nsec: isize,
    pub __unused: [isize; 3],
}

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: c_ulong,
    pub d_off: isize,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

const S_IFMT: c_uint = 0o170000;
const S_IFREG: c_uint = 0o100000;
const S_IXUSR: c_uint = 0o00100;
const TIOCGWINSZ: c_ulong = 0x5413;

#[inline]
unsafe fn S_ISREG(mode: c_uint) -> bool {
    (mode & S_IFMT) == S_IFREG
}

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn getenv(name: *const c_char) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn qsort(
        base: *mut c_void,
        nmemb: usize,
        size: usize,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    );
    fn printf(format: *const c_char, ...) -> c_int;
    fn putchar(c: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn abort() -> !;

    fn astrcatf(strp: *mut *mut c_char, fmt: *const c_char, ...);
    fn astrcat(strp: *mut *mut c_char, str: *const c_char);
    fn strstarts(str: *const c_char, prefix: *const c_char) -> bool;
    fn get_argv_exec_path() -> *mut c_char;
    fn mput_char(c: c_char, num: usize);
}

unsafe fn zfree<T>(ptrp: *mut *mut T) {
    if !(*ptrp).is_null() {
        free(*ptrp as *mut c_void);
        *ptrp = ptr::null_mut();
    }
}

unsafe fn alloc_grow_names(cmds: *mut cmdnames, nr: c_uint) {
    if nr <= (*cmds).alloc {
        return;
    }

    let mut alloc = (*cmds).alloc;
    if alloc == 0 {
        alloc = 16;
    }
    while alloc < nr {
        alloc = alloc.wrapping_mul(2);
    }

    let old_size = ((*cmds).alloc as usize).wrapping_mul(mem::size_of::<*mut cmdname>());
    let new_size = (alloc as usize).wrapping_mul(mem::size_of::<*mut cmdname>());
    let new_names = malloc(new_size) as *mut *mut cmdname;
    if new_names.is_null() {
        abort();
    }
    if !(*cmds).names.is_null() && old_size != 0 {
        memcpy(
            new_names as *mut c_void,
            (*cmds).names as *const c_void,
            old_size,
        );
        free((*cmds).names as *mut c_void);
    }
    (*cmds).names = new_names;
    (*cmds).alloc = alloc;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_cmdname(cmds: *mut cmdnames, name: *const c_char, len: usize) {
    let ent = malloc(
        mem::size_of::<cmdname>()
            .wrapping_add(len)
            .wrapping_add(1),
    ) as *mut cmdname;
    if ent.is_null() {
        return;
    }

    (*ent).len = len;
    memcpy((*ent).name.as_mut_ptr() as *mut c_void, name as *const c_void, len);
    *(*ent).name.as_mut_ptr().add(len) = 0;

    alloc_grow_names(cmds, (*cmds).cnt.wrapping_add(1));
    *(*cmds).names.add((*cmds).cnt as usize) = ent;
    (*cmds).cnt = (*cmds).cnt.wrapping_add(1);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clean_cmdnames(cmds: *mut cmdnames) {
    let mut i: c_uint;

    i = 0;
    while i < (*cmds).cnt {
        zfree((*cmds).names.add(i as usize));
        i = i.wrapping_add(1);
    }
    zfree(&mut (*cmds).names);
    (*cmds).cnt = 0;
    (*cmds).alloc = 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmdname_compare(a_: *const c_void, b_: *const c_void) -> c_int {
    let a = *(a_ as *const *mut cmdname);
    let b = *(b_ as *const *mut cmdname);
    strcmp((*a).name.as_ptr(), (*b).name.as_ptr())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uniq(cmds: *mut cmdnames) {
    let mut i: c_uint;
    let mut j: c_uint;

    if (*cmds).cnt == 0 {
        return;
    }

    i = 1;
    while i < (*cmds).cnt {
        if strcmp(
            (**(*cmds).names.add(i as usize)).name.as_ptr(),
            (**(*cmds).names.add(i.wrapping_sub(1) as usize)).name.as_ptr(),
        ) == 0
        {
            zfree((*cmds).names.add(i.wrapping_sub(1) as usize));
        }
        i = i.wrapping_add(1);
    }
    i = 0;
    j = 0;
    while i < (*cmds).cnt {
        if !(*(*cmds).names.add(i as usize)).is_null() {
            if i == j {
                j = j.wrapping_add(1);
            } else {
                *(*cmds).names.add(j as usize) = *(*cmds).names.add(i as usize);
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    (*cmds).cnt = j;
    while j < i {
        *(*cmds).names.add(j as usize) = ptr::null_mut();
        j = j.wrapping_add(1);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exclude_cmds(cmds: *mut cmdnames, excludes: *mut cmdnames) {
    let mut ci: usize;
    let mut cj: usize;
    let mut ei: usize;
    let mut cmp: c_int;

    if (*excludes).cnt == 0 {
        return;
    }

    ci = 0;
    cj = 0;
    ei = 0;
    while ci < (*cmds).cnt as usize && ei < (*excludes).cnt as usize {
        cmp = strcmp(
            (**(*cmds).names.add(ci)).name.as_ptr(),
            (**(*excludes).names.add(ei)).name.as_ptr(),
        );
        if cmp < 0 {
            if ci == cj {
                ci += 1;
                cj += 1;
            } else {
                *(*cmds).names.add(cj) = *(*cmds).names.add(ci);
                cj += 1;
                *(*cmds).names.add(ci) = ptr::null_mut();
                ci += 1;
            }
        } else if cmp == 0 {
            zfree((*cmds).names.add(ci));
            ci += 1;
            ei += 1;
        } else if cmp > 0 {
            ei += 1;
        }
    }
    while ci < (*cmds).cnt as usize {
        if ci != cj {
            *(*cmds).names.add(cj) = *(*cmds).names.add(ci);
            *(*cmds).names.add(ci) = ptr::null_mut();
        }
        ci += 1;
        cj += 1;
    }
    ci = cj;
    while ci < (*cmds).cnt as usize {
        if !(*(*cmds).names.add(ci)).is_null() {
            abort();
        }
        ci += 1;
    }
    (*cmds).cnt = cj as c_uint;
}

unsafe fn get_term_dimensions(ws: *mut winsize) {
    let mut s = getenv(c"LINES".as_ptr());

    if !s.is_null() {
        (*ws).ws_row = atoi(s) as u16;
        s = getenv(c"COLUMNS".as_ptr());
        if !s.is_null() {
            (*ws).ws_col = atoi(s) as u16;
            if (*ws).ws_row != 0 && (*ws).ws_col != 0 {
                return;
            }
        }
    }
    // Original C condition: #ifdef TIOCGWINSZ.
    if ioctl(1, TIOCGWINSZ, ws) == 0 && (*ws).ws_row != 0 && (*ws).ws_col != 0 {
        return;
    }
    (*ws).ws_row = 25;
    (*ws).ws_col = 80;
}

unsafe fn pretty_print_string_list(cmds: *mut cmdnames, longest: c_int) {
    let mut cols: c_int = 1;
    let rows: c_int;
    let space: c_int = longest + 1; /* min 1 SP between words */
    let mut win = mem::MaybeUninit::<winsize>::uninit();
    let max_cols: c_int;
    let mut i: c_int;
    let mut j: c_int;

    get_term_dimensions(win.as_mut_ptr());
    let win = win.assume_init();
    max_cols = win.ws_col as c_int - 1; /* don't print *on* the edge */

    if space < max_cols {
        cols = max_cols / space;
    }
    rows = ((*cmds).cnt as c_int + cols - 1) / cols;

    i = 0;
    while i < rows {
        printf(c"  ".as_ptr());

        j = 0;
        while j < cols {
            let n: c_uint = (j * rows + i) as c_uint;
            let mut size: c_uint = space as c_uint;

            if n >= (*cmds).cnt {
                break;
            }
            if j == cols - 1 || n.wrapping_add(rows as c_uint) >= (*cmds).cnt {
                size = 1;
            }
            printf(
                c"%-*s".as_ptr(),
                size,
                (**(*cmds).names.add(n as usize)).name.as_ptr(),
            );
            j += 1;
        }
        putchar('\n' as c_int);
        i += 1;
    }
}

unsafe fn is_executable(name: *const c_char) -> c_int {
    let mut st = mem::MaybeUninit::<stat>::uninit();

    if stat(name, st.as_mut_ptr()) != 0 || !S_ISREG((*st.as_ptr()).st_mode) {
        return 0;
    }

    ((*st.as_ptr()).st_mode & S_IXUSR) as c_int
}

unsafe fn has_extension(filename: *const c_char, ext: *const c_char) -> c_int {
    let len = strlen(filename);
    let extlen = strlen(ext);

    (len > extlen
        && memcmp(
            filename.add(len - extlen) as *const c_void,
            ext as *const c_void,
            extlen,
        ) == 0) as c_int
}

unsafe fn list_commands_in_dir(cmds: *mut cmdnames, path: *const c_char, mut prefix: *const c_char) {
    let prefix_len: c_int;
    let dir = opendir(path);
    let mut de: *mut dirent;
    let mut buf: *mut c_char = ptr::null_mut();

    if dir.is_null() {
        return;
    }
    if prefix.is_null() {
        prefix = c"perf-".as_ptr();
    }
    prefix_len = strlen(prefix) as c_int;

    astrcatf(&mut buf, c"%s/".as_ptr(), path);

    loop {
        de = readdir(dir);
        if de.is_null() {
            break;
        }
        let mut entlen: c_int;

        if !strstarts((*de).d_name.as_ptr(), prefix) {
            continue;
        }

        astrcat(&mut buf, (*de).d_name.as_ptr());
        if is_executable(buf) == 0 {
            continue;
        }

        entlen = strlen((*de).d_name.as_ptr()) as c_int - prefix_len;
        if has_extension((*de).d_name.as_ptr(), c".exe".as_ptr()) != 0 {
            entlen -= 4;
        }

        add_cmdname(
            cmds,
            (*de).d_name.as_ptr().add(prefix_len as usize),
            entlen as usize,
        );
    }
    closedir(dir);
    free(buf as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_command_list(
    prefix: *const c_char,
    main_cmds: *mut cmdnames,
    other_cmds: *mut cmdnames,
) {
    let env_path = getenv(c"PATH".as_ptr());
    let exec_path = get_argv_exec_path();

    if !exec_path.is_null() {
        list_commands_in_dir(main_cmds, exec_path, prefix);
        qsort(
            (*main_cmds).names as *mut c_void,
            (*main_cmds).cnt as usize,
            mem::size_of::<*mut cmdname>(),
            Some(cmdname_compare),
        );
        uniq(main_cmds);
    }

    if !env_path.is_null() {
        let paths: *mut c_char;
        let mut path: *mut c_char;
        let mut colon: *mut c_char;
        path = strdup(env_path);
        paths = path;
        loop {
            colon = strchr(path, ':' as c_int);
            if !colon.is_null() {
                *colon = 0;
            }
            if exec_path.is_null() || strcmp(path, exec_path) != 0 {
                list_commands_in_dir(other_cmds, path, prefix);
            }

            if colon.is_null() {
                break;
            }
            path = colon.add(1);
        }
        free(paths as *mut c_void);

        qsort(
            (*other_cmds).names as *mut c_void,
            (*other_cmds).cnt as usize,
            mem::size_of::<*mut cmdname>(),
            Some(cmdname_compare),
        );
        uniq(other_cmds);
    }
    free(exec_path as *mut c_void);
    exclude_cmds(other_cmds, main_cmds);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn list_commands(
    title: *const c_char,
    main_cmds: *mut cmdnames,
    other_cmds: *mut cmdnames,
) {
    let mut i: c_uint;
    let mut longest: c_uint = 0;

    i = 0;
    while i < (*main_cmds).cnt {
        if longest < (**(*main_cmds).names.add(i as usize)).len as c_uint {
            longest = (**(*main_cmds).names.add(i as usize)).len as c_uint;
        }
        i = i.wrapping_add(1);
    }
    i = 0;
    while i < (*other_cmds).cnt {
        if longest < (**(*other_cmds).names.add(i as usize)).len as c_uint {
            longest = (**(*other_cmds).names.add(i as usize)).len as c_uint;
        }
        i = i.wrapping_add(1);
    }

    if (*main_cmds).cnt != 0 {
        let exec_path = get_argv_exec_path();
        printf(c"available %s in '%s'\n".as_ptr(), title, exec_path);
        printf(c"----------------".as_ptr());
        mput_char('-' as c_char, strlen(title).wrapping_add(strlen(exec_path)));
        putchar('\n' as c_int);
        pretty_print_string_list(main_cmds, longest as c_int);
        putchar('\n' as c_int);
        free(exec_path as *mut c_void);
    }

    if (*other_cmds).cnt != 0 {
        printf(c"%s available from elsewhere on your $PATH\n".as_ptr(), title);
        printf(c"---------------------------------------".as_ptr());
        mput_char('-' as c_char, strlen(title));
        putchar('\n' as c_int);
        pretty_print_string_list(other_cmds, longest as c_int);
        putchar('\n' as c_int);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_in_cmdlist(c: *mut cmdnames, s: *const c_char) -> c_int {
    let mut i: c_uint;

    i = 0;
    while i < (*c).cnt {
        if strcmp(s, (**(*c).names.add(i as usize)).name.as_ptr()) == 0 {
            return 1;
        }
        i = i.wrapping_add(1);
    }
    0
}
