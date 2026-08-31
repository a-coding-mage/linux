// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * probe-file.c : operate ftrace k/uprobe events files
 *
 * Written by Masami Hiramatsu <masami.hiramatsu.pt@hitachi.com>
 */

use core::ffi::{c_char, c_int, c_long, c_ulonglong, c_void};
use core::mem;
use core::ptr;

/* 4096 - 2 ('\n' + '\0') */
const MAX_CMDLEN: usize = 4094;

const E2BIG: c_int = 7;
const EACCES: c_int = 13;
const EEXIST: c_int = 17;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const ENOTSUP: c_int = 95;

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_CREAT: c_int = 0o100;
const O_APPEND: c_int = 0o2000;
const SEEK_SET: c_int = 0;

const PATH_MAX: usize = 4096;
const STRERR_BUFSIZE: usize = 128;
const SBUILD_ID_SIZE: usize = 64;
const PF_FL_UPROBE: c_int = 1;
const PF_FL_RW: c_int = 2;
const DSO__NAME_KALLSYMS: *const c_char = b"[kernel.kallsyms]\0".as_ptr() as *const c_char;
const PROBE_TYPE_X: probe_type = 0;
const PROBE_TYPE_END: probe_type = 1;

type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type probe_type = c_int;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strfilter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct str_node {
    pub s: *mut c_char,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct probe_point {
    pub symbol: *mut c_char,
}

#[repr(C)]
pub struct probe_trace_event {
    pub event: *mut c_char,
    pub group: *mut c_char,
    pub point: probe_point,
}

#[repr(C)]
pub struct perf_probe_event {
    pub event: *mut c_char,
    pub group: *mut c_char,
    pub sdt: bool,
}

#[repr(C)]
pub struct probe_cache_entry {
    pub node: list_head,
    pub tevlist: *mut strlist,
    pub pev: perf_probe_event,
    pub spev: *mut c_char,
    pub sdt: bool,
}

#[repr(C)]
pub struct probe_cache {
    pub entries: list_head,
    pub fd: c_int,
}

#[repr(C)]
pub struct nscookie {
    _private: [u8; 0],
}

#[repr(C)]
pub struct probe_conf_t {
    pub max_probes: c_int,
}

#[repr(C)]
pub struct stat {
    pub st_size: off_t,
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
pub struct strbuf {
    _private: [u8; 0],
}

#[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
const SDT_NOTE_IDX_LOC: usize = 0;
#[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
const SDT_NOTE_IDX_REFCTR: usize = 2;
#[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
const EM_HOST: c_int = 0;
#[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
const SDT_ARG_VALID: c_int = 1;

#[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
#[repr(C)]
pub union sdt_addr {
    pub a32: [u32; 3],
    pub a64: [u64; 3],
}

#[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
#[repr(C)]
pub struct sdt_note {
    pub note_list: list_head,
    pub bit32: bool,
    pub addr: sdt_addr,
    pub name: *mut c_char,
    pub provider: *mut c_char,
    pub args: *mut c_char,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut probe_event_dry_run: bool;
    static mut probe_conf: probe_conf_t;

    fn debugfs__configured() -> bool;
    fn tracefs__configured() -> bool;
    fn tracing_path_mount() -> *const c_char;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *mut c_char;
    fn e_snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn pr_warning(format: *const c_char, ...);
    fn pr_debug(format: *const c_char, ...);
    fn pr_debug4(format: *const c_char, ...);
    fn pr_err(format: *const c_char, ...);
    fn pr_info(format: *const c_char, ...);

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn dup(oldfd: c_int) -> c_int;
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    fn feof(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn writev(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn ftruncate(fd: c_int, length: off_t) -> c_int;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;

    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strlcpy(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);

    fn strlist__new(list: *const c_char, dupstr: *const c_void) -> *mut strlist;
    fn strlist__delete(slist: *mut strlist);
    fn strlist__add(slist: *mut strlist, str_: *const c_char) -> c_int;
    fn strlist__nr_entries(slist: *mut strlist) -> c_int;
    fn strlist__first(slist: *mut strlist) -> *mut str_node;
    fn strlist__next(node: *mut str_node) -> *mut str_node;

    fn parse_probe_trace_command(cmd: *const c_char, tev: *mut probe_trace_event) -> c_int;
    fn clear_probe_trace_event(tev: *mut probe_trace_event);
    fn synthesize_probe_trace_command(tev: *mut probe_trace_event) -> *mut c_char;
    fn synthesize_perf_probe_command(pev: *mut perf_probe_event) -> *mut c_char;
    fn clear_perf_probe_event(pev: *mut perf_probe_event);
    fn perf_probe_event__copy(dst: *mut perf_probe_event, src: *mut perf_probe_event) -> c_int;
    fn parse_perf_probe_command(cmd: *const c_char, pev: *mut perf_probe_event) -> c_int;

    fn build_id_cache__cached(sbuildid: *const c_char) -> bool;
    fn build_id_cache__linkname(sbuildid: *const c_char, a: *const c_void, b: c_int) -> *mut c_char;
    fn sysfs__snprintf_build_id(root: *const c_char, sbuildid: *mut c_char, size: size_t) -> c_int;
    fn nsinfo__mountns_enter(nsi: *mut nsinfo, nsc: *mut nscookie);
    fn nsinfo__mountns_exit(nsc: *mut nscookie);
    fn filename__snprintf_build_id(filename: *const c_char, sbuildid: *mut c_char, size: size_t) -> c_int;
    fn build_id_cache__add_s(sbuildid: *const c_char, name: *const c_char, nsi: *mut nsinfo, is_kallsyms: bool, a: *const c_void) -> c_int;
    fn build_id_cache__cachedir(sbuildid: *const c_char, name: *const c_char, nsi: *mut nsinfo, is_kallsyms: bool, a: bool) -> *mut c_char;
    fn build_id_cache__list_all(validonly: bool) -> *mut strlist;
    fn build_id_cache__origname(sbuildid: *const c_char) -> *mut c_char;

    fn strfilter__compare(filter: *mut strfilter, str_: *const c_char) -> bool;
    fn strfilter__string(filter: *mut strfilter) -> *mut c_char;
    fn strglobmatch(str_: *const c_char, pat: *const c_char) -> bool;

    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;
    fn list_add_tail(new_: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn BUG_ON(condition: bool);
    fn probe_cache_entry_from_node(node: *mut list_head) -> *mut probe_cache_entry;

    fn strbuf_init(sb: *mut strbuf, hint: size_t) -> c_int;
    fn strbuf_addf(sb: *mut strbuf, fmt: *const c_char, ...) -> c_int;
    fn strbuf_detach(sb: *mut strbuf, sz: *mut size_t) -> *mut c_char;
    fn strbuf_release(sb: *mut strbuf);

    #[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
    fn perf_sdt_arg_parse_op(em: c_int, op: *const c_char, new_op: *mut *mut c_char) -> c_int;
    #[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
    fn argv_split(str_: *const c_char, argcp: *mut c_int) -> *mut *mut c_char;
    #[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
    fn argv_free(argv: *mut *mut c_char);
    #[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
    fn get_sdt_note_list(head: *mut list_head, pathname: *const c_char) -> c_int;
    #[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
    fn cleanup_sdt_note_list(head: *mut list_head);
    #[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
    fn sdt_note_from_node(node: *mut list_head) -> *mut sdt_note;
}

unsafe fn strlist_for_each(mut sl: *mut strlist, mut f: impl FnMut(*mut str_node) -> bool) {
    let mut ent = strlist__first(sl);
    while !ent.is_null() {
        if !f(ent) {
            break;
        }
        ent = strlist__next(ent);
    }
}

unsafe fn for_each_probe_cache_entry(mut pcache: *mut probe_cache, mut f: impl FnMut(*mut probe_cache_entry) -> bool) {
    let head = &mut (*pcache).entries as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let entry = probe_cache_entry_from_node(pos);
        pos = (*pos).next;
        if !f(entry) {
            break;
        }
    }
}

unsafe fn print_common_warning(err: c_int, readwrite: bool) -> bool {
    if err == -EACCES {
        pr_warning(
            b"No permission to %s tracefs.\nPlease %s\n\0".as_ptr() as *const c_char,
            if readwrite { b"write\0".as_ptr() } else { b"read\0".as_ptr() },
            if readwrite {
                b"run this command again with sudo.\0".as_ptr()
            } else {
                b"try 'sudo mount -o remount,mode=755 /sys/kernel/tracing/'\0".as_ptr()
            },
        );
    } else {
        return false;
    }
    true
}

unsafe fn print_configure_probe_event(kerr: c_int, uerr: c_int) -> bool {
    let config: *const c_char;
    let file: *const c_char;

    if kerr == -ENOENT && uerr == -ENOENT {
        file = b"{k,u}probe_events\0".as_ptr() as *const c_char;
        config = b"CONFIG_KPROBE_EVENTS=y and CONFIG_UPROBE_EVENTS=y\0".as_ptr() as *const c_char;
    } else if kerr == -ENOENT {
        file = b"kprobe_events\0".as_ptr() as *const c_char;
        config = b"CONFIG_KPROBE_EVENTS=y\0".as_ptr() as *const c_char;
    } else if uerr == -ENOENT {
        file = b"uprobe_events\0".as_ptr() as *const c_char;
        config = b"CONFIG_UPROBE_EVENTS=y\0".as_ptr() as *const c_char;
    } else {
        return false;
    }

    if !debugfs__configured() && !tracefs__configured() {
        pr_warning(b"Debugfs or tracefs is not mounted\nPlease try 'sudo mount -t tracefs nodev /sys/kernel/tracing/'\n\0".as_ptr() as *const c_char);
    } else {
        pr_warning(
            b"%s/%s does not exist.\nPlease rebuild kernel with %s.\n\0".as_ptr() as *const c_char,
            tracing_path_mount(),
            file,
            config,
        );
    }

    true
}

unsafe fn print_open_warning(err: c_int, uprobe: bool, readwrite: bool) {
    let mut sbuf = [0 as c_char; STRERR_BUFSIZE];

    if print_common_warning(err, readwrite) {
        return;
    }
    if print_configure_probe_event(if uprobe { 0 } else { err }, if uprobe { err } else { 0 }) {
        return;
    }
    pr_warning(
        b"Failed to open %s/%cprobe_events: %s\n\0".as_ptr() as *const c_char,
        tracing_path_mount(),
        if uprobe { 'u' as c_int } else { 'k' as c_int },
        str_error_r(-err, sbuf.as_mut_ptr(), sbuf.len()),
    );
}

unsafe fn print_both_open_warning(kerr: c_int, uerr: c_int, readwrite: bool) {
    let mut sbuf = [0 as c_char; STRERR_BUFSIZE];

    if kerr == uerr && print_common_warning(kerr, readwrite) {
        return;
    }
    if print_configure_probe_event(kerr, uerr) {
        return;
    }
    if kerr < 0 {
        pr_warning(
            b"Failed to open %s/kprobe_events: %s.\n\0".as_ptr() as *const c_char,
            tracing_path_mount(),
            str_error_r(-kerr, sbuf.as_mut_ptr(), sbuf.len()),
        );
    }
    if uerr < 0 {
        pr_warning(
            b"Failed to open %s/uprobe_events: %s.\n\0".as_ptr() as *const c_char,
            tracing_path_mount(),
            str_error_r(-uerr, sbuf.as_mut_ptr(), sbuf.len()),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn open_trace_file(trace_file: *const c_char, readwrite: bool) -> c_int {
    let mut buf = [0 as c_char; PATH_MAX];
    let mut ret = e_snprintf(buf.as_mut_ptr(), PATH_MAX, b"%s/%s\0".as_ptr() as *const c_char, tracing_path_mount(), trace_file);
    if ret >= 0 {
        pr_debug(b"Opening %s write=%d\n\0".as_ptr() as *const c_char, buf.as_ptr(), readwrite as c_int);
        if readwrite && !probe_event_dry_run {
            ret = open(buf.as_ptr(), O_RDWR | O_APPEND, 0);
        } else {
            ret = open(buf.as_ptr(), O_RDONLY, 0);
        }
        if ret < 0 {
            ret = -errno;
        }
    }
    ret
}

unsafe fn open_kprobe_events(readwrite: bool) -> c_int {
    open_trace_file(b"kprobe_events\0".as_ptr() as *const c_char, readwrite)
}

unsafe fn open_uprobe_events(readwrite: bool) -> c_int {
    open_trace_file(b"uprobe_events\0".as_ptr() as *const c_char, readwrite)
}

#[no_mangle]
pub unsafe extern "C" fn probe_file__open(flag: c_int) -> c_int {
    let fd = if flag & PF_FL_UPROBE != 0 {
        open_uprobe_events(flag & PF_FL_RW != 0)
    } else {
        open_kprobe_events(flag & PF_FL_RW != 0)
    };
    if fd < 0 {
        print_open_warning(fd, flag & PF_FL_UPROBE != 0, flag & PF_FL_RW != 0);
    }
    fd
}

#[no_mangle]
pub unsafe extern "C" fn probe_file__open_both(kfd: *mut c_int, ufd: *mut c_int, flag: c_int) -> c_int {
    if kfd.is_null() || ufd.is_null() {
        return -EINVAL;
    }
    *kfd = open_kprobe_events(flag & PF_FL_RW != 0);
    *ufd = open_uprobe_events(flag & PF_FL_RW != 0);
    if *kfd < 0 && *ufd < 0 {
        print_both_open_warning(*kfd, *ufd, flag & PF_FL_RW != 0);
        return *kfd;
    }
    0
}

/* Get raw string list of current kprobe_events  or uprobe_events */
#[no_mangle]
pub unsafe extern "C" fn probe_file__get_rawlist(fd: c_int) -> *mut strlist {
    let mut ret: c_int;
    let mut idx: c_int;
    let fddup: c_int;
    let fp: *mut FILE;
    let mut buf = [0 as c_char; MAX_CMDLEN];
    let mut p: *mut c_char;
    let sl: *mut strlist;

    if fd < 0 {
        return ptr::null_mut();
    }
    sl = strlist__new(ptr::null(), ptr::null());
    if sl.is_null() {
        return ptr::null_mut();
    }
    fddup = dup(fd);
    if fddup < 0 {
        strlist__delete(sl);
        return ptr::null_mut();
    }
    fp = fdopen(fddup, b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        close(fddup);
        strlist__delete(sl);
        return ptr::null_mut();
    }
    while feof(fp) == 0 {
        p = fgets(buf.as_mut_ptr(), MAX_CMDLEN as c_int, fp);
        if p.is_null() {
            break;
        }
        idx = strlen(p) as c_int - 1;
        if *p.offset(idx as isize) == '\n' as c_char {
            *p.offset(idx as isize) = 0;
        }
        if buf[0] == '#' as c_char {
            continue;
        }
        ret = strlist__add(sl, buf.as_ptr());
        if ret < 0 {
            pr_debug(b"strlist__add failed (%d)\n\0".as_ptr() as *const c_char, ret);
            fclose(fp);
            strlist__delete(sl);
            return ptr::null_mut();
        }
    }
    fclose(fp);
    sl
}

unsafe fn __probe_file__get_namelist(fd: c_int, include_group: bool) -> *mut strlist {
    let mut buf = [0 as c_char; 128];
    let sl: *mut strlist;
    let rawlist: *mut strlist;
    let mut tev: probe_trace_event = mem::zeroed();
    let mut ret: c_int = 0;

    memset(&mut tev as *mut _ as *mut c_void, 0, mem::size_of_val(&tev));
    rawlist = probe_file__get_rawlist(fd);
    if rawlist.is_null() {
        return ptr::null_mut();
    }
    sl = strlist__new(ptr::null(), ptr::null());
    strlist_for_each(rawlist, |ent| {
        ret = parse_probe_trace_command((*ent).s, &mut tev);
        if ret < 0 {
            return false;
        }
        if include_group {
            ret = e_snprintf(buf.as_mut_ptr(), 128, b"%s:%s\0".as_ptr() as *const c_char, tev.group, tev.event);
            if ret >= 0 {
                ret = strlist__add(sl, buf.as_ptr());
            }
        } else {
            ret = strlist__add(sl, tev.event);
        }
        clear_probe_trace_event(&mut tev);
        /* Skip if there is same name multi-probe event in the list */
        if ret == -EEXIST {
            ret = 0;
        }
        ret >= 0
    });
    strlist__delete(rawlist);

    if ret < 0 {
        strlist__delete(sl);
        return ptr::null_mut();
    }
    sl
}

/* Get current perf-probe event names */
#[no_mangle]
pub unsafe extern "C" fn probe_file__get_namelist(fd: c_int) -> *mut strlist {
    __probe_file__get_namelist(fd, false)
}

#[no_mangle]
pub unsafe extern "C" fn probe_file__add_event(fd: c_int, tev: *mut probe_trace_event) -> c_int {
    let mut ret = 0;
    let buf = synthesize_probe_trace_command(tev);
    let mut sbuf = [0 as c_char; STRERR_BUFSIZE];

    if buf.is_null() {
        pr_debug(b"Failed to synthesize probe trace event.\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    pr_debug(b"Writing event: %s\n\0".as_ptr() as *const c_char, buf);
    if !probe_event_dry_run {
        if write(fd, buf as *const c_void, strlen(buf)) < strlen(buf) as ssize_t {
            ret = -errno;
            pr_warning(b"Failed to write event: %s\n\0".as_ptr() as *const c_char, str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()));
        }
    }
    free(buf as *mut c_void);
    ret
}

unsafe fn __del_trace_probe_event(fd: c_int, ent: *mut str_node) -> c_int {
    let mut buf = [0 as c_char; 128];
    let mut ret = e_snprintf(buf.as_mut_ptr(), 128, b"-:%s\0".as_ptr() as *const c_char, (*ent).s);
    if ret < 0 {
        pr_warning(b"Failed to delete event: %s\n\0".as_ptr() as *const c_char, str_error_r(-ret, buf.as_mut_ptr(), buf.len()));
        return ret;
    }
    let p = strchr(buf.as_mut_ptr().offset(2), ':' as c_int);
    if p.is_null() {
        pr_debug(b"Internal error: %s should have ':' but not.\n\0".as_ptr() as *const c_char, (*ent).s);
        ret = -ENOTSUP;
        pr_warning(b"Failed to delete event: %s\n\0".as_ptr() as *const c_char, str_error_r(-ret, buf.as_mut_ptr(), buf.len()));
        return ret;
    }
    *p = '/' as c_char;
    pr_debug(b"Writing event: %s\n\0".as_ptr() as *const c_char, buf.as_ptr());
    ret = write(fd, buf.as_ptr() as *const c_void, strlen(buf.as_ptr())) as c_int;
    if ret < 0 {
        ret = -errno;
        pr_warning(b"Failed to delete event: %s\n\0".as_ptr() as *const c_char, str_error_r(-ret, buf.as_mut_ptr(), buf.len()));
        return ret;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn probe_file__get_events(fd: c_int, filter: *mut strfilter, plist: *mut strlist) -> c_int {
    let namelist: *mut strlist;
    let mut ret = -ENOENT;

    if plist.is_null() {
        return -EINVAL;
    }
    namelist = __probe_file__get_namelist(fd, true);
    if namelist.is_null() {
        return -ENOENT;
    }
    strlist_for_each(namelist, |ent| {
        let p = strchr((*ent).s, ':' as c_int);
        if (!p.is_null() && strfilter__compare(filter, p.offset(1))) || strfilter__compare(filter, (*ent).s) {
            ret = strlist__add(plist, (*ent).s);
            if ret == -ENOMEM {
                pr_err(b"strlist__add failed with -ENOMEM\n\0".as_ptr() as *const c_char);
                return false;
            }
            ret = 0;
        }
        true
    });
    strlist__delete(namelist);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn probe_file__del_strlist(fd: c_int, namelist: *mut strlist) -> c_int {
    let mut ret = 0;
    strlist_for_each(namelist, |ent| {
        ret = __del_trace_probe_event(fd, ent);
        ret >= 0
    });
    ret
}

/* Caller must ensure to remove this entry from list */
unsafe fn probe_cache_entry__delete(entry: *mut probe_cache_entry) {
    if !entry.is_null() {
        BUG_ON(list_empty(&(*entry).node) == 0);
        strlist__delete((*entry).tevlist);
        clear_perf_probe_event(&mut (*entry).pev);
        zfree(&mut (*entry).spev as *mut _ as *mut *mut c_void);
        free(entry as *mut c_void);
    }
}

unsafe fn probe_cache_entry__new(pev: *mut perf_probe_event) -> *mut probe_cache_entry {
    let entry = zalloc(mem::size_of::<probe_cache_entry>()) as *mut probe_cache_entry;
    if !entry.is_null() {
        INIT_LIST_HEAD(&mut (*entry).node);
        (*entry).tevlist = strlist__new(ptr::null(), ptr::null());
        if (*entry).tevlist.is_null() {
            let mut e = entry as *mut c_void;
            zfree(&mut e);
        } else if !pev.is_null() {
            (*entry).spev = synthesize_perf_probe_command(pev);
            if (*entry).spev.is_null() || perf_probe_event__copy(&mut (*entry).pev, pev) < 0 {
                probe_cache_entry__delete(entry);
                return ptr::null_mut();
            }
        }
    }
    entry
}

#[no_mangle]
pub unsafe extern "C" fn probe_cache_entry__get_event(entry: *mut probe_cache_entry, tevs: *mut *mut probe_trace_event) -> c_int {
    let mut ret = strlist__nr_entries((*entry).tevlist);
    if ret > probe_conf.max_probes {
        return -E2BIG;
    }
    *tevs = calloc(ret as size_t, mem::size_of::<probe_trace_event>()) as *mut probe_trace_event;
    if (*tevs).is_null() {
        return -ENOMEM;
    }
    let mut i = 0;
    strlist_for_each((*entry).tevlist, |node| {
        let tev = (*tevs).offset(i as isize);
        i += 1;
        ret = parse_probe_trace_command((*node).s, tev);
        ret >= 0
    });
    i
}

/* For the kernel probe caches, pass target = NULL or DSO__NAME_KALLSYMS */
unsafe fn probe_cache__open(pcache: *mut probe_cache, mut target: *const c_char, nsi: *mut nsinfo) -> c_int {
    let mut cpath = [0 as c_char; PATH_MAX];
    let mut sbuildid = [0 as c_char; SBUILD_ID_SIZE];
    let mut dir_name: *mut c_char = ptr::null_mut();
    let mut is_kallsyms = false;
    let mut nsc: nscookie = mem::zeroed();
    let ret: c_int;

    if !target.is_null() && build_id_cache__cached(target) {
        strlcpy(sbuildid.as_mut_ptr(), target, SBUILD_ID_SIZE);
        dir_name = build_id_cache__linkname(sbuildid.as_ptr(), ptr::null(), 0);
    } else {
        if target.is_null() || strcmp(target, DSO__NAME_KALLSYMS) == 0 {
            target = DSO__NAME_KALLSYMS;
            is_kallsyms = true;
            ret = sysfs__snprintf_build_id(b"/\0".as_ptr() as *const c_char, sbuildid.as_mut_ptr(), sbuildid.len());
        } else {
            nsinfo__mountns_enter(nsi, &mut nsc);
            ret = filename__snprintf_build_id(target, sbuildid.as_mut_ptr(), sbuildid.len());
            nsinfo__mountns_exit(&mut nsc);
        }
        if ret < 0 {
            pr_debug(b"Failed to get build-id from %s.\n\0".as_ptr() as *const c_char, target);
            return ret;
        }
        /* If we have no buildid cache, make it */
        if !build_id_cache__cached(sbuildid.as_ptr()) {
            let add_ret = build_id_cache__add_s(sbuildid.as_ptr(), target, nsi, is_kallsyms, ptr::null());
            if add_ret < 0 {
                pr_debug(b"Failed to add build-id cache: %s\n\0".as_ptr() as *const c_char, target);
                return add_ret;
            }
        }
        dir_name = build_id_cache__cachedir(sbuildid.as_ptr(), target, nsi, is_kallsyms, false);
    }
    if dir_name.is_null() {
        pr_debug(b"Failed to get cache from %s\n\0".as_ptr() as *const c_char, target);
        return -ENOMEM;
    }
    snprintf(cpath.as_mut_ptr(), PATH_MAX, b"%s/probes\0".as_ptr() as *const c_char, dir_name);
    let fd = open(cpath.as_ptr(), O_CREAT | O_RDWR, 0o644);
    if fd < 0 {
        pr_debug(b"Failed to open cache(%d): %s\n\0".as_ptr() as *const c_char, fd, cpath.as_ptr());
    }
    free(dir_name as *mut c_void);
    (*pcache).fd = fd;
    fd
}

unsafe fn probe_cache__load(pcache: *mut probe_cache) -> c_int {
    let mut entry: *mut probe_cache_entry = ptr::null_mut();
    let mut buf = [0 as c_char; MAX_CMDLEN];
    let mut ret = 0;
    let fddup = dup((*pcache).fd);
    if fddup < 0 {
        return -errno;
    }
    let fp = fdopen(fddup, b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        close(fddup);
        return -EINVAL;
    }
    while feof(fp) == 0 {
        if fgets(buf.as_mut_ptr(), MAX_CMDLEN as c_int, fp).is_null() {
            break;
        }
        let p = strchr(buf.as_mut_ptr(), '\n' as c_int);
        if !p.is_null() {
            *p = 0;
        }
        /* #perf_probe_event or %sdt_event */
        if buf[0] == '#' as c_char || buf[0] == '%' as c_char {
            entry = probe_cache_entry__new(ptr::null_mut());
            if entry.is_null() {
                ret = -ENOMEM;
                break;
            }
            if buf[0] == '%' as c_char {
                (*entry).sdt = true;
            }
            (*entry).spev = strdup(buf.as_ptr().offset(1));
            if !(*entry).spev.is_null() {
                ret = parse_perf_probe_command(buf.as_ptr().offset(1), &mut (*entry).pev);
            } else {
                ret = -ENOMEM;
            }
            if ret < 0 {
                probe_cache_entry__delete(entry);
                break;
            }
            list_add_tail(&mut (*entry).node, &mut (*pcache).entries);
        } else {
            if entry.is_null() {
                ret = -EINVAL;
                break;
            }
            ret = strlist__add((*entry).tevlist, buf.as_ptr());
            if ret == -ENOMEM {
                pr_err(b"strlist__add failed with -ENOMEM\n\0".as_ptr() as *const c_char);
                break;
            }
        }
    }
    fclose(fp);
    ret
}

unsafe fn probe_cache__alloc() -> *mut probe_cache {
    let pcache = zalloc(mem::size_of::<probe_cache>()) as *mut probe_cache;
    if !pcache.is_null() {
        INIT_LIST_HEAD(&mut (*pcache).entries);
        (*pcache).fd = -EINVAL;
    }
    pcache
}

#[no_mangle]
pub unsafe extern "C" fn probe_cache__purge(pcache: *mut probe_cache) {
    let head = &mut (*pcache).entries as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let next = (*pos).next;
        let entry = probe_cache_entry_from_node(pos);
        list_del_init(&mut (*entry).node);
        probe_cache_entry__delete(entry);
        pos = next;
    }
}

#[no_mangle]
pub unsafe extern "C" fn probe_cache__delete(pcache: *mut probe_cache) {
    if pcache.is_null() {
        return;
    }
    probe_cache__purge(pcache);
    if (*pcache).fd > 0 {
        close((*pcache).fd);
    }
    free(pcache as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn probe_cache__new(target: *const c_char, nsi: *mut nsinfo) -> *mut probe_cache {
    let pcache = probe_cache__alloc();
    if pcache.is_null() {
        return ptr::null_mut();
    }
    let mut ret = probe_cache__open(pcache, target, nsi);
    if ret < 0 {
        pr_debug(b"Cache open error: %d\n\0".as_ptr() as *const c_char, ret);
        probe_cache__delete(pcache);
        return ptr::null_mut();
    }
    ret = probe_cache__load(pcache);
    if ret < 0 {
        pr_debug(b"Cache read error: %d\n\0".as_ptr() as *const c_char, ret);
        probe_cache__delete(pcache);
        return ptr::null_mut();
    }
    pcache
}

unsafe fn streql(a: *const c_char, b: *const c_char) -> bool {
    if a == b {
        return true;
    }
    if a.is_null() || b.is_null() {
        return false;
    }
    strcmp(a, b) == 0
}

#[no_mangle]
pub unsafe extern "C" fn probe_cache__find(pcache: *mut probe_cache, pev: *mut perf_probe_event) -> *mut probe_cache_entry {
    let cmd = synthesize_perf_probe_command(pev);
    if cmd.is_null() {
        return ptr::null_mut();
    }
    let mut found: *mut probe_cache_entry = ptr::null_mut();
    for_each_probe_cache_entry(pcache, |entry| {
        if (*pev).sdt {
            if !(*entry).pev.event.is_null()
                && streql((*entry).pev.event, (*pev).event)
                && ((*pev).group.is_null() || streql((*entry).pev.group, (*pev).group))
            {
                found = entry;
                return false;
            }
            return true;
        }
        /* Hit if same event name or same command-string */
        if (!(*pev).event.is_null()
            && streql((*entry).pev.group, (*pev).group)
            && streql((*entry).pev.event, (*pev).event))
            || strcmp((*entry).spev, cmd) == 0
        {
            found = entry;
            return false;
        }
        true
    });
    free(cmd as *mut c_void);
    found
}

#[no_mangle]
pub unsafe extern "C" fn probe_cache__find_by_name(pcache: *mut probe_cache, group: *const c_char, event: *const c_char) -> *mut probe_cache_entry {
    let mut found: *mut probe_cache_entry = ptr::null_mut();
    for_each_probe_cache_entry(pcache, |entry| {
        /* Hit if same event name or same command-string */
        if streql((*entry).pev.group, group) && streql((*entry).pev.event, event) {
            found = entry;
            return false;
        }
        true
    });
    found
}

#[no_mangle]
pub unsafe extern "C" fn probe_cache__add_entry(pcache: *mut probe_cache, pev: *mut perf_probe_event, tevs: *mut probe_trace_event, ntevs: c_int) -> c_int {
    let mut entry: *mut probe_cache_entry = ptr::null_mut();
    let mut ret = 0;

    if pcache.is_null() || pev.is_null() || tevs.is_null() || ntevs <= 0 {
        ret = -EINVAL;
        pr_debug(b"Failed to add probe caches\n\0".as_ptr() as *const c_char);
        return ret;
    }
    /* Remove old cache entry */
    entry = probe_cache__find(pcache, pev);
    if !entry.is_null() {
        list_del_init(&mut (*entry).node);
        probe_cache_entry__delete(entry);
    }
    ret = -ENOMEM;
    entry = probe_cache_entry__new(pev);
    if entry.is_null() {
        pr_debug(b"Failed to add probe caches\n\0".as_ptr() as *const c_char);
        return ret;
    }
    for i in 0..ntevs {
        let tev = tevs.offset(i as isize);
        if (*tev).point.symbol.is_null() {
            continue;
        }
        let command = synthesize_probe_trace_command(tev);
        if command.is_null() {
            pr_debug(b"Failed to add probe caches\n\0".as_ptr() as *const c_char);
            probe_cache_entry__delete(entry);
            return ret;
        }
        ret = strlist__add((*entry).tevlist, command);
        if ret == -ENOMEM {
            pr_err(b"strlist__add failed with -ENOMEM\n\0".as_ptr() as *const c_char);
            free(command as *mut c_void);
            pr_debug(b"Failed to add probe caches\n\0".as_ptr() as *const c_char);
            probe_cache_entry__delete(entry);
            return ret;
        }
        free(command as *mut c_void);
    }
    list_add_tail(&mut (*entry).node, &mut (*pcache).entries);
    pr_debug(b"Added probe cache: %d\n\0".as_ptr() as *const c_char, ntevs);
    0
}

#[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
unsafe fn sdt_note__get_addr(note: *mut sdt_note) -> c_ulonglong {
    if (*note).bit32 {
        (*note).addr.a32[SDT_NOTE_IDX_LOC] as c_ulonglong
    } else {
        (*note).addr.a64[SDT_NOTE_IDX_LOC] as c_ulonglong
    }
}

#[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
unsafe fn sdt_note__get_ref_ctr_offset(note: *mut sdt_note) -> c_ulonglong {
    if (*note).bit32 {
        (*note).addr.a32[SDT_NOTE_IDX_REFCTR] as c_ulonglong
    } else {
        (*note).addr.a64[SDT_NOTE_IDX_REFCTR] as c_ulonglong
    }
}

#[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
static type_to_suffix: [*const c_char; 17] = [
    b":s64\0".as_ptr() as *const c_char, b"\0".as_ptr() as *const c_char, b"\0".as_ptr() as *const c_char,
    b"\0".as_ptr() as *const c_char, b":s32\0".as_ptr() as *const c_char, b"\0".as_ptr() as *const c_char,
    b":s16\0".as_ptr() as *const c_char, b":s8\0".as_ptr() as *const c_char, b"\0".as_ptr() as *const c_char,
    b":u8\0".as_ptr() as *const c_char, b":u16\0".as_ptr() as *const c_char, b"\0".as_ptr() as *const c_char,
    b":u32\0".as_ptr() as *const c_char, b"\0".as_ptr() as *const c_char, b"\0".as_ptr() as *const c_char,
    b"\0".as_ptr() as *const c_char, b":u64\0".as_ptr() as *const c_char,
];

#[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
unsafe fn sdt_arg_parse_size(n_ptr: *mut c_char, suffix: *mut *const c_char) -> c_int {
    let type_idx = strtol(n_ptr, ptr::null_mut(), 10);
    if type_idx < -8 || type_idx > 8 {
        pr_debug4(b"Failed to get a valid sdt type\n\0".as_ptr() as *const c_char);
        return -1;
    }
    *suffix = type_to_suffix[(type_idx + 8) as usize];
    0
}

#[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
unsafe fn synthesize_sdt_probe_arg(buf: *mut strbuf, i: c_int, arg: *const c_char) -> c_int {
    let mut desc = strdup(arg);
    let mut new_op: *mut c_char = ptr::null_mut();
    let mut suffix: *const c_char = b"\0".as_ptr() as *const c_char;
    let mut ret = -1;

    if desc.is_null() {
        pr_debug4(b"Allocation error\n\0".as_ptr() as *const c_char);
        return ret;
    }
    let mut op = strchr(desc, '@' as c_int);
    if !op.is_null() {
        *op = 0;
        op = op.offset(1);
        if sdt_arg_parse_size(desc, &mut suffix) != 0 {
            free(desc as *mut c_void);
            free(new_op as *mut c_void);
            return ret;
        }
    } else {
        op = desc;
    }
    ret = perf_sdt_arg_parse_op(EM_HOST, op, &mut new_op);
    if ret < 0 {
        free(desc as *mut c_void);
        free(new_op as *mut c_void);
        return ret;
    }
    if ret == SDT_ARG_VALID {
        ret = strbuf_addf(buf, b" arg%d=%s%s\0".as_ptr() as *const c_char, i + 1, new_op, suffix);
        if ret < 0 {
            free(desc as *mut c_void);
            free(new_op as *mut c_void);
            return ret;
        }
    }
    ret = 0;
    free(desc as *mut c_void);
    free(new_op as *mut c_void);
    ret
}

#[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
unsafe fn synthesize_sdt_probe_command(note: *mut sdt_note, pathname: *const c_char, sdtgrp: *const c_char) -> *mut c_char {
    let mut buf: strbuf = mem::zeroed();
    let mut ret: *mut c_char = ptr::null_mut();
    let mut args_count = 0;
    let mut arg_idx = 0;
    if strbuf_init(&mut buf, 32) < 0 {
        return ptr::null_mut();
    }
    let mut err = strbuf_addf(&mut buf, b"p:%s/%s %s:0x%llx\0".as_ptr() as *const c_char, sdtgrp, (*note).name, pathname, sdt_note__get_addr(note));
    let ref_ctr_offset = sdt_note__get_ref_ctr_offset(note);
    if ref_ctr_offset != 0 && err >= 0 {
        err = strbuf_addf(&mut buf, b"(0x%llx)\0".as_ptr() as *const c_char, ref_ctr_offset);
    }
    if err < 0 {
        strbuf_release(&mut buf);
        return ret;
    }
    if (*note).args.is_null() {
        ret = strbuf_detach(&mut buf, ptr::null_mut());
        strbuf_release(&mut buf);
        return ret;
    }
    let args = argv_split((*note).args, &mut args_count);
    if args.is_null() {
        strbuf_release(&mut buf);
        return ret;
    }
    let mut i = 0;
    while i < args_count {
        let mut arg: *mut c_char = ptr::null_mut();
        if !strstr(*args.offset(i as isize), b"[sp,\0".as_ptr() as *const c_char).is_null() && i + 1 < args_count {
            err = asprintf(&mut arg, b"%s %s\0".as_ptr() as *const c_char, *args.offset(i as isize), *args.offset((i + 1) as isize));
            i += 2;
        } else {
            err = asprintf(&mut arg, b"%s\0".as_ptr() as *const c_char, *args.offset(i as isize));
            i += 1;
        }
        /* Failed to allocate memory */
        if err < 0 {
            argv_free(args);
            strbuf_release(&mut buf);
            return ret;
        }
        if synthesize_sdt_probe_arg(&mut buf, arg_idx, arg) < 0 {
            free(arg as *mut c_void);
            argv_free(args);
            strbuf_release(&mut buf);
            return ret;
        }
        free(arg as *mut c_void);
        arg_idx += 1;
    }
    argv_free(args);
    ret = strbuf_detach(&mut buf, ptr::null_mut());
    strbuf_release(&mut buf);
    ret
}

#[cfg(HAVE_GELF_GETNOTE_SUPPORT)]
#[no_mangle]
pub unsafe extern "C" fn probe_cache__scan_sdt(pcache: *mut probe_cache, pathname: *const c_char) -> c_int {
    let mut entry: *mut probe_cache_entry = ptr::null_mut();
    let mut sdtlist: list_head = mem::zeroed();
    let mut sdtgrp = [0 as c_char; 64];
    INIT_LIST_HEAD(&mut sdtlist);
    let mut ret = get_sdt_note_list(&mut sdtlist, pathname);
    if ret < 0 {
        pr_debug4(b"Failed to get sdt note: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    let head = &mut sdtlist as *mut list_head;
    let mut pos = sdtlist.next;
    while pos != head {
        let note = sdt_note_from_node(pos);
        pos = (*pos).next;
        ret = snprintf(sdtgrp.as_mut_ptr(), 64, b"sdt_%s\0".as_ptr() as *const c_char, (*note).provider);
        if ret < 0 {
            break;
        }
        /* Try to find same-name entry */
        entry = probe_cache__find_by_name(pcache, sdtgrp.as_ptr(), (*note).name);
        if entry.is_null() {
            entry = probe_cache_entry__new(ptr::null_mut());
            if entry.is_null() {
                ret = -ENOMEM;
                break;
            }
            (*entry).sdt = true;
            ret = asprintf(&mut (*entry).spev, b"%s:%s=%s\0".as_ptr() as *const c_char, sdtgrp.as_ptr(), (*note).name, (*note).name);
            if ret < 0 {
                break;
            }
            (*entry).pev.event = strdup((*note).name);
            (*entry).pev.group = strdup(sdtgrp.as_ptr());
            list_add_tail(&mut (*entry).node, &mut (*pcache).entries);
        }
        let buf = synthesize_sdt_probe_command(note, pathname, sdtgrp.as_ptr());
        if buf.is_null() {
            ret = -ENOMEM;
            break;
        }
        ret = strlist__add((*entry).tevlist, buf);
        free(buf as *mut c_void);
        entry = ptr::null_mut();
        if ret == -ENOMEM {
            pr_err(b"strlist__add failed with -ENOMEM\n\0".as_ptr() as *const c_char);
            break;
        }
    }
    if !entry.is_null() {
        list_del_init(&mut (*entry).node);
        probe_cache_entry__delete(entry);
    }
    cleanup_sdt_note_list(&mut sdtlist);
    ret
}

unsafe fn probe_cache_entry__write(entry: *mut probe_cache_entry, fd: c_int) -> c_int {
    let mut st: stat = mem::zeroed();
    let mut iov: [iovec; 3] = mem::zeroed();
    let prefix = if (*entry).sdt { b"%\0".as_ptr() } else { b"#\0".as_ptr() } as *const c_char;
    let mut ret = fstat(fd, &mut st);
    if ret < 0 {
        return ret;
    }
    pr_debug(b"Writing cache: %s%s\n\0".as_ptr() as *const c_char, prefix, (*entry).spev);
    iov[0].iov_base = prefix as *mut c_void;
    iov[0].iov_len = 1;
    iov[1].iov_base = (*entry).spev as *mut c_void;
    iov[1].iov_len = strlen((*entry).spev);
    iov[2].iov_base = b"\n\0".as_ptr() as *mut c_void;
    iov[2].iov_len = 1;
    ret = writev(fd, iov.as_ptr(), 3) as c_int;
    if ret < iov[1].iov_len as c_int + 2 {
        if ret > 0 {
            ret = -1;
        }
        if ftruncate(fd, st.st_size) < 0 {
            ret = -2;
        }
        return ret;
    }
    strlist_for_each((*entry).tevlist, |snode| {
        iov[0].iov_base = (*snode).s as *mut c_void;
        iov[0].iov_len = strlen((*snode).s);
        iov[1].iov_base = b"\n\0".as_ptr() as *mut c_void;
        iov[1].iov_len = 1;
        ret = writev(fd, iov.as_ptr(), 2) as c_int;
        ret >= iov[0].iov_len as c_int + 1
    });
    if ret < 0 || ret < iov[0].iov_len as c_int + 1 {
        /* Rollback to avoid cache file corruption */
        if ret > 0 {
            ret = -1;
        }
        if ftruncate(fd, st.st_size) < 0 {
            ret = -2;
        }
        return ret;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn probe_cache__commit(pcache: *mut probe_cache) -> c_int {
    let mut ret = lseek((*pcache).fd, 0, SEEK_SET) as c_int;
    if ret < 0 {
        return ret;
    }
    ret = ftruncate((*pcache).fd, 0);
    if ret < 0 {
        return ret;
    }
    for_each_probe_cache_entry(pcache, |entry| {
        ret = probe_cache_entry__write(entry, (*pcache).fd);
        pr_debug(b"Cache committed: %d\n\0".as_ptr() as *const c_char, ret);
        ret >= 0
    });
    ret
}

unsafe fn probe_cache_entry__compare(entry: *mut probe_cache_entry, filter: *mut strfilter) -> bool {
    let mut buf = [0 as c_char; 128];
    let mut ptr_ = (*entry).spev;
    if !(*entry).pev.event.is_null() {
        snprintf(buf.as_mut_ptr(), 128, b"%s:%s\0".as_ptr() as *const c_char, (*entry).pev.group, (*entry).pev.event);
        ptr_ = buf.as_mut_ptr();
    }
    strfilter__compare(filter, ptr_)
}

#[no_mangle]
pub unsafe extern "C" fn probe_cache__filter_purge(pcache: *mut probe_cache, filter: *mut strfilter) -> c_int {
    let head = &mut (*pcache).entries as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let next = (*pos).next;
        let entry = probe_cache_entry_from_node(pos);
        if probe_cache_entry__compare(entry, filter) {
            pr_info(b"Removed cached event: %s\n\0".as_ptr() as *const c_char, (*entry).spev);
            list_del_init(&mut (*entry).node);
            probe_cache_entry__delete(entry);
        }
        pos = next;
    }
    0
}

unsafe fn probe_cache__show_entries(pcache: *mut probe_cache, filter: *mut strfilter) -> c_int {
    for_each_probe_cache_entry(pcache, |entry| {
        if probe_cache_entry__compare(entry, filter) {
            printf(b"%s\n\0".as_ptr() as *const c_char, (*entry).spev);
        }
        true
    });
    0
}

/* Show all cached probes */
#[no_mangle]
pub unsafe extern "C" fn probe_cache__show_all_caches(filter: *mut strfilter) -> c_int {
    let mut buf = strfilter__string(filter);
    pr_debug(b"list cache with filter: %s\n\0".as_ptr() as *const c_char, buf);
    free(buf as *mut c_void);
    let bidlist = build_id_cache__list_all(true);
    if bidlist.is_null() {
        pr_debug(b"Failed to get buildids: %d\n\0".as_ptr() as *const c_char, errno);
        return -EINVAL;
    }
    strlist_for_each(bidlist, |nd| {
        let pcache = probe_cache__new((*nd).s, ptr::null_mut());
        if pcache.is_null() {
            return true;
        }
        if list_empty(&(*pcache).entries) == 0 {
            buf = build_id_cache__origname((*nd).s);
            printf(b"%s (%s):\n\0".as_ptr() as *const c_char, buf, (*nd).s);
            free(buf as *mut c_void);
            probe_cache__show_entries(pcache, filter);
        }
        probe_cache__delete(pcache);
        true
    });
    strlist__delete(bidlist);
    0
}

type ftrace_readme = c_int;
const FTRACE_README_PROBE_TYPE_X: ftrace_readme = 0;
const FTRACE_README_KRETPROBE_OFFSET: ftrace_readme = 1;
const FTRACE_README_UPROBE_REF_CTR: ftrace_readme = 2;
const FTRACE_README_USER_ACCESS: ftrace_readme = 3;
const FTRACE_README_MULTIPROBE_EVENT: ftrace_readme = 4;
const FTRACE_README_IMMEDIATE_VALUE: ftrace_readme = 5;
const FTRACE_README_END: ftrace_readme = 6;

#[repr(C)]
struct ftrace_readme_entry {
    pattern: *const c_char,
    avail: bool,
}

static mut ftrace_readme_table: [ftrace_readme_entry; 6] = [
    ftrace_readme_entry { pattern: b"*type: * x8/16/32/64,*\0".as_ptr() as *const c_char, avail: false },
    ftrace_readme_entry { pattern: b"*place (kretprobe): *\0".as_ptr() as *const c_char, avail: false },
    ftrace_readme_entry { pattern: b"*ref_ctr_offset*\0".as_ptr() as *const c_char, avail: false },
    ftrace_readme_entry { pattern: b"*u]<offset>*\0".as_ptr() as *const c_char, avail: false },
    ftrace_readme_entry { pattern: b"*Create/append/*\0".as_ptr() as *const c_char, avail: false },
    ftrace_readme_entry { pattern: b"*\\imm-value,*\0".as_ptr() as *const c_char, avail: false },
];

unsafe fn scan_ftrace_readme(type_: ftrace_readme) -> bool {
    let mut buf: *mut c_char = ptr::null_mut();
    let mut len: size_t = 0;
    let mut ret = false;
    static mut SCANNED: bool = false;

    if SCANNED {
        if type_ >= FTRACE_README_END {
            return false;
        }
        return ftrace_readme_table[type_ as usize].avail;
    }
    let fd = open_trace_file(b"README\0".as_ptr() as *const c_char, false);
    if fd < 0 {
        return ret;
    }
    let fp = fdopen(fd, b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        close(fd);
        return ret;
    }
    while getline(&mut buf, &mut len, fp) > 0 {
        let mut i = 0;
        while i < FTRACE_README_END {
            if !ftrace_readme_table[i as usize].avail {
                ftrace_readme_table[i as usize].avail =
                    strglobmatch(buf, ftrace_readme_table[i as usize].pattern);
            }
            i += 1;
        }
    }
    SCANNED = true;
    fclose(fp);
    free(buf as *mut c_void);

    if type_ >= FTRACE_README_END {
        return false;
    }
    ret = ftrace_readme_table[type_ as usize].avail;
    ret
}

#[no_mangle]
pub unsafe extern "C" fn probe_type_is_available(type_: probe_type) -> bool {
    if type_ >= PROBE_TYPE_END {
        false
    } else if type_ == PROBE_TYPE_X {
        scan_ftrace_readme(FTRACE_README_PROBE_TYPE_X)
    } else {
        true
    }
}

#[no_mangle]
pub unsafe extern "C" fn kretprobe_offset_is_supported() -> bool {
    scan_ftrace_readme(FTRACE_README_KRETPROBE_OFFSET)
}

#[no_mangle]
pub unsafe extern "C" fn uprobe_ref_ctr_is_supported() -> bool {
    scan_ftrace_readme(FTRACE_README_UPROBE_REF_CTR)
}

#[no_mangle]
pub unsafe extern "C" fn user_access_is_supported() -> bool {
    scan_ftrace_readme(FTRACE_README_USER_ACCESS)
}

#[no_mangle]
pub unsafe extern "C" fn multiprobe_event_is_supported() -> bool {
    scan_ftrace_readme(FTRACE_README_MULTIPROBE_EVENT)
}

#[no_mangle]
pub unsafe extern "C" fn immediate_value_is_supported() -> bool {
    scan_ftrace_readme(FTRACE_README_IMMEDIATE_VALUE)
}
