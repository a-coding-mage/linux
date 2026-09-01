// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type u8 = u8;
type u64 = u64;
type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;

const BUFSZ: usize = 1024;
const READLEN: usize = 128;
const PATH_MAX: usize = 4096;
const UINT_MAX: c_uint = c_uint::MAX;
const ULLONG_MAX: u64 = u64::MAX;
const KMOD_DECOMP_LEN: usize = 4096;

const PERF_RECORD_MISC_KERNEL: u8 = 1;
const PERF_RECORD_MISC_HYPERVISOR: u8 = 3;
const PERF_RECORD_MISC_GUEST_KERNEL: u8 = 4;
const PERF_RECORD_SAMPLE: c_uint = 9;
const PERF_RECORD_THROTTLE: c_uint = 5;
const PERF_RECORD_UNTHROTTLE: c_uint = 6;
const PERF_RECORD_MAX: c_uint = 22;
const DSO_BINARY_TYPE__KALLSYMS: c_int = 5;

#[repr(C)]
struct rb_node {
    rb_parent_color: c_ulong,
    rb_right: *mut rb_node,
    rb_left: *mut rb_node,
}

#[repr(C)]
struct rb_root {
    rb_node: *mut rb_node,
}

#[repr(C)]
struct tested_section {
    rb_node: rb_node,
    addr: u64,
    path: *mut c_char,
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct utsname {
    sysname: [c_char; 65],
    nodename: [c_char; 65],
    release: [c_char; 65],
    version: [c_char; 65],
    machine: [c_char; 65],
    domainname: [c_char; 65],
}

#[repr(C)]
struct rb_root_holder {
    _private: [u8; 0],
}

#[repr(C)]
struct machine {
    _private: [u8; 0],
}

#[repr(C)]
struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
struct evsel {
    core: perf_evsel,
}

#[repr(C)]
struct perf_evsel {
    attr: perf_event_attr,
}

#[repr(C)]
struct perf_event_attr {
    _prefix: [u8; 0],
    comm: c_uint,
    disabled: c_uint,
    enable_on_exec: c_uint,
}

#[repr(C)]
union perf_event {
    header: perf_event_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct perf_event_header {
    type_: c_uint,
    misc: u16,
    size: u16,
}

#[repr(C)]
struct perf_sample {
    _prefix: [u8; 0],
    ip: u64,
    pid: pid_t,
    tid: pid_t,
    cpumode: u8,
}

#[repr(C)]
struct thread {
    _private: [u8; 0],
}

#[repr(C)]
struct addr_location {
    map: *mut map,
    addr: u64,
}

#[repr(C)]
struct map {
    _private: [u8; 0],
}

#[repr(C)]
struct dso {
    _private: [u8; 0],
}

#[repr(C)]
struct mmap {
    core: perf_mmap,
}

#[repr(C)]
struct perf_mmap {
    _private: [u8; 0],
}

#[repr(C)]
struct evlist_core {
    nr_mmaps: c_int,
}

#[repr(C)]
struct record_opts_target {
    uses_mmap: bool,
}

#[repr(C)]
struct record_opts {
    mmap_pages: c_uint,
    user_freq: c_uint,
    user_interval: u64,
    freq: c_uint,
    target: record_opts_target,
}

#[repr(C)]
struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
struct symbol_conf_t {
    kallsyms_name: *const c_char,
}

#[repr(C)]
struct test_suite {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut test_objdump_path: *const c_char;
    static mut verbose: c_int;
    static mut symbol_conf: symbol_conf_t;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn feof(stream: *mut FILE) -> c_int;
    fn popen(command: *const c_char, type_: *const c_char) -> *mut FILE;
    fn pclose(stream: *mut FILE) -> c_int;
    fn uname(buf: *mut utsname) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn getpid() -> pid_t;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>);

    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, rb_link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);

    fn isxdigit(c: c_int) -> c_int;
    fn isspace(c: c_int) -> c_int;
    fn hex(c: c_char) -> c_int;
    fn host_is_bigendian() -> bool;
    fn pr_debug(fmt: *const c_char, ...);

    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn thread__find_map(thread: *mut thread, cpumode: u8, addr: u64, al: *mut addr_location) -> bool;
    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__symtab_type(dso: *mut dso) -> c_int;
    fn dso__is_kcore(dso: *mut dso) -> bool;
    fn dso__is_kmod(dso: *mut dso) -> bool;
    fn dso__text_end(dso: *mut dso) -> u64;
    fn dso__data_read_offset(dso: *mut dso, machine: *mut machine, offset: u64, buf: *mut u8, len: size_t) -> size_t;
    fn dso__needs_decompress(dso: *mut dso) -> bool;
    fn dso__decompress_kmodule_path(dso: *mut dso, name: *const c_char, out: *mut c_char, len: size_t) -> c_int;
    fn dso__is_vmlinux(dso: *mut dso) -> bool;
    fn map__start(map: *mut map) -> u64;
    fn map__end(map: *mut map) -> u64;
    fn map__load(map: *mut map) -> c_int;
    fn map__rip_2objdump(map: *mut map, addr: u64) -> u64;
    fn thread__maps(thread: *mut thread) -> *mut c_void;
    fn maps__machine(maps: *mut c_void) -> *mut machine;
    fn thread__put(thread: *mut thread);
    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn evlist__parse_sample(evlist: *mut evlist, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn machine__findnew_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn machine__process_event(machine: *mut machine, event: *mut perf_event, sample: *mut c_void) -> c_int;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__mmap(evlist: *mut evlist) -> *mut mmap;
    fn perf_mmap__read_init(md: *mut perf_mmap) -> c_int;
    fn perf_mmap__read_event(md: *mut perf_mmap) -> *mut perf_event;
    fn perf_mmap__consume(md: *mut perf_mmap);
    fn perf_mmap__read_done(md: *mut perf_mmap);
    fn perf_env__init(env: *mut perf_env);
    fn perf_env__exit(env: *mut perf_env);
    fn machine__new_host(env: *mut perf_env) -> *mut machine;
    fn machine__create_kernel_maps(machine: *mut machine) -> c_int;
    fn machine__kernel_map(machine: *mut machine) -> *mut map;
    fn machine__delete(machine: *mut machine);
    fn thread_map__new_by_tid(pid: pid_t) -> *mut perf_thread_map;
    fn perf_event__synthesize_thread_map(tool: *mut c_void, threads: *mut perf_thread_map, process: *mut c_void, machine: *mut machine, needs_mmap: bool, mmap_data: bool) -> c_int;
    static perf_event__process: c_void;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_evlist__set_maps(core: *mut evlist_core, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map);
    fn evlist__new() -> *mut evlist;
    fn parse_event(evlist: *mut evlist, str_: *const c_char) -> c_int;
    fn evlist__config(evlist: *mut evlist, opts: *mut record_opts, callchain_param: *mut c_void);
    fn evlist__open(evlist: *mut evlist) -> c_int;
    fn evlist__strerror_open(evlist: *mut evlist, err: c_int, buf: *mut c_char, size: size_t);
    fn evlist__put(evlist: *mut evlist);
    fn evlist__do_mmap(evlist: *mut evlist, pages: c_uint) -> c_int;
    fn evlist__enable(evlist: *mut evlist);
    fn evlist__disable(evlist: *mut evlist);
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn __errno_location() -> *mut c_int;
}

const TEST_CODE_READING_OK: c_int = 0;
const TEST_CODE_READING_NO_VMLINUX: c_int = 1;
const TEST_CODE_READING_NO_KCORE: c_int = 2;
const TEST_CODE_READING_NO_ACCESS: c_int = 3;
const TEST_CODE_READING_NO_KERNEL_OBJ: c_int = 4;

unsafe fn rb_empty_root(root: *const rb_root) -> bool {
    unsafe { (*root).rb_node.is_null() }
}

unsafe fn tested_section_from_node(node: *mut rb_node) -> *mut tested_section {
    node as *mut tested_section
}

unsafe fn tested_code_insert_or_exists(path: *const c_char, addr: u64, tested_sections: *mut rb_root) -> bool {
    let mut node = unsafe { &mut (*tested_sections).rb_node as *mut *mut rb_node };
    let mut parent: *mut rb_node = ptr::null_mut();
    let mut data: *mut tested_section;

    unsafe {
        while !(*node).is_null() {
            let mut cmp: c_int;

            parent = *node;
            data = tested_section_from_node(*node);
            cmp = strcmp(path, (*data).path);
            if cmp == 0 {
                if addr < (*data).addr {
                    cmp = -1;
                } else if addr > (*data).addr {
                    cmp = 1;
                } else {
                    return true; /* already tested */
                }
            }

            if cmp < 0 {
                node = &mut (**node).rb_left as *mut *mut rb_node;
            } else {
                node = &mut (**node).rb_right as *mut *mut rb_node;
            }
        }

        data = zalloc(mem::size_of::<tested_section>()) as *mut tested_section;
        if data.is_null() {
            return true;
        }

        (*data).addr = addr;
        (*data).path = strdup(path);
        if (*data).path.is_null() {
            free(data as *mut c_void);
            return true;
        }
        rb_link_node(&mut (*data).rb_node, parent, node);
        rb_insert_color(&mut (*data).rb_node, tested_sections);
        false
    }
}

unsafe fn tested_sections__free(root: *mut rb_root) {
    unsafe {
        while !rb_empty_root(root) {
            let node = rb_first(root);
            let ts = tested_section_from_node(node);

            rb_erase(node, root);
            free((*ts).path as *mut c_void);
            free(ts as *mut c_void);
        }
    }
}

unsafe fn read_objdump_chunk(line: *mut *const c_char, buf: *mut *mut u8, buf_len: *mut size_t) -> size_t {
    let mut bytes_read: size_t = 0;
    let mut chunk_start = unsafe { *buf };

    /* Read bytes */
    unsafe {
        while *buf_len > 0 {
            let c1: c_char;
            let c2: c_char;

            /* Get 2 hex digits */
            c1 = **line;
            *line = (*line).add(1);
            if isxdigit(c1 as c_int) == 0 {
                break;
            }
            c2 = **line;
            *line = (*line).add(1);
            if isxdigit(c2 as c_int) == 0 {
                break;
            }

            /* Store byte and advance buf */
            **buf = ((hex(c1) << 4) | hex(c2)) as u8;
            *buf = (*buf).add(1);
            *buf_len -= 1;
            bytes_read += 1;

            /* End of chunk? */
            if isspace(**line as c_int) != 0 {
                break;
            }
        }

        /*
         * objdump will display raw insn as LE if code endian
         * is LE and bytes_per_chunk > 1. In that case reverse
         * the chunk we just read.
         *
         * see disassemble_bytes() at binutils/objdump.c for details
         * how objdump chooses display endian)
         */
        if bytes_read > 1 && !host_is_bigendian() {
            let mut chunk_end = chunk_start.add(bytes_read - 1);
            let mut tmp: u8;

            while chunk_start < chunk_end {
                tmp = *chunk_start;
                *chunk_start = *chunk_end;
                *chunk_end = tmp;
                chunk_start = chunk_start.add(1);
                chunk_end = chunk_end.sub(1);
            }
        }
    }

    bytes_read
}

unsafe fn read_objdump_line(line: *const c_char, buf: *mut u8, mut buf_len: size_t) -> size_t {
    let mut p: *const c_char;
    let mut ret: size_t;
    let mut bytes_read: size_t = 0;
    let mut buf = buf;

    /* Skip to a colon */
    unsafe {
        p = strchr(line, ':' as c_int) as *const c_char;
        if p.is_null() {
            return 0;
        }
        p = p.add(1);

        /* Skip initial spaces */
        while *p != 0 {
            if isspace(*p as c_int) == 0 {
                break;
            }
            p = p.add(1);
        }

        loop {
            ret = read_objdump_chunk(&mut p, &mut buf, &mut buf_len);
            bytes_read += ret;
            p = p.add(1);
            if ret == 0 {
                break;
            }
        }
    }

    /* return number of successfully read bytes */
    bytes_read
}

unsafe fn read_objdump_output(f: *mut FILE, buf: *mut c_void, len: *mut size_t, start_addr: u64) -> c_int {
    let mut line: *mut c_char = ptr::null_mut();
    let mut line_len: size_t = 0;
    let mut off_last: size_t = 0;
    let mut ret: ssize_t;
    let mut err: c_int = 0;
    let mut addr: u64 = 0;
    let mut last_addr: u64 = start_addr;

    unsafe {
        while off_last < *len {
            let off: size_t;
            let read_bytes: size_t;
            let written_bytes: size_t;
            let mut tmp = [0u8; BUFSZ];

            ret = getline(&mut line, &mut line_len, f);
            if feof(f) != 0 {
                break;
            }
            if ret < 0 {
                pr_debug(b"getline failed\n\0".as_ptr() as *const c_char);
                err = -1;
                break;
            }

            /* read objdump data into temporary buffer */
            read_bytes = read_objdump_line(line, tmp.as_mut_ptr(), tmp.len());
            if read_bytes == 0 {
                continue;
            }

            if sscanf(line, b"%llx\0".as_ptr() as *const c_char, &mut addr as *mut u64) != 1 {
                continue;
            }
            if addr < last_addr {
                pr_debug(b"addr going backwards, read beyond section?\n\0".as_ptr() as *const c_char);
                break;
            }
            last_addr = addr;

            /* copy it from temporary buffer to 'buf' according
             * to address on current objdump line */
            off = (addr - start_addr) as size_t;
            if off >= *len {
                break;
            }
            written_bytes = core::cmp::min(read_bytes, *len - off);
            memcpy((buf as *mut u8).add(off) as *mut c_void, tmp.as_ptr() as *const c_void, written_bytes);
            off_last = off + written_bytes;
        }

        /* len returns number of bytes that could not be read */
        *len -= off_last;

        free(line as *mut c_void);
    }

    err
}

/*
 * Only gets GNU objdump version. Returns 0 for llvm-objdump.
 */
unsafe fn objdump_version() -> c_int {
    let mut line_len: size_t = 0;
    let mut cmd = [0 as c_char; PATH_MAX * 2];
    let fmt: *const c_char;
    let mut f: *mut FILE;
    let mut ret: c_int;

    let mut version_tmp: c_int;
    let mut version_num: c_int = 0;
    let mut version: *mut c_char = ptr::null_mut();
    let mut token: *mut c_char;
    let mut line: *mut c_char = ptr::null_mut();

    unsafe {
        fmt = b"%s --version\0".as_ptr() as *const c_char;
        ret = snprintf(cmd.as_mut_ptr(), cmd.len(), fmt, test_objdump_path);
        if ret <= 0 || ret as size_t >= cmd.len() {
            return -1;
        }
        /* Ignore objdump errors */
        strcat(cmd.as_mut_ptr(), b" 2>/dev/null\0".as_ptr() as *const c_char);
        f = popen(cmd.as_ptr(), b"r\0".as_ptr() as *const c_char);
        if f.is_null() {
            pr_debug(b"popen failed\n\0".as_ptr() as *const c_char);
            return -1;
        }
        /* Get first line of objdump --version output */
        ret = getline(&mut line, &mut line_len, f) as c_int;
        pclose(f);
        if ret < 0 {
            pr_debug(b"getline failed\n\0".as_ptr() as *const c_char);
            return -1;
        }

        token = strsep(&mut line, b" \0".as_ptr() as *const c_char);
        if !token.is_null() && strcmp(token, b"GNU\0".as_ptr() as *const c_char) == 0 {
            // version is last part of first line of objdump --version output.
            loop {
                token = strsep(&mut line, b" \0".as_ptr() as *const c_char);
                if token.is_null() {
                    break;
                }
                version = token;
            }

            // Convert version into a format we can compare with
            token = strsep(&mut version, b".\0".as_ptr() as *const c_char);
            version_num = atoi(token);
            if version_num != 0 {
                version_num *= 10000;
            }

            token = strsep(&mut version, b".\0".as_ptr() as *const c_char);
            version_tmp = atoi(token);
            if !token.is_null() {
                version_num += version_tmp * 100;
            }

            token = strsep(&mut version, b".\0".as_ptr() as *const c_char);
            version_tmp = atoi(token);
            if !token.is_null() {
                version_num += version_tmp;
            }
        }
    }

    version_num
}

unsafe fn read_via_objdump(filename: *const c_char, addr: u64, buf: *mut c_void, mut len: size_t) -> c_int {
    let mut stop_address: u64 = addr + len as u64;
    let mut uname_buf: utsname = unsafe { mem::zeroed() };
    let mut cmd = [0 as c_char; PATH_MAX * 2];
    let fmt: *const c_char;
    let mut f: *mut FILE;
    let mut ret: c_int;

    unsafe {
        ret = uname(&mut uname_buf);
        if ret != 0 {
            pr_debug(b"uname failed\n\0".as_ptr() as *const c_char);
            return -1;
        }

        if strncmp(uname_buf.machine.as_ptr(), b"riscv\0".as_ptr() as *const c_char, 5) == 0 {
            let version = objdump_version();

            /* Default to this workaround if version parsing fails */
            if version < 0 || version > 24100 {
                /*
                 * Starting at riscv objdump version 2.41, dumping in
                 * the middle of an instruction is not supported. riscv
                 * instructions are aligned along 2-byte intervals and
                 * can be either 2-bytes or 4-bytes. This makes it
                 * possible that the stop-address lands in the middle of
                 * a 4-byte instruction. Increase the stop_address by
                 * two to ensure an instruction is not cut in half, but
                 * leave the len as-is so only the expected number of
                 * bytes are collected.
                 */
                stop_address += 2;
            }
        }

        fmt = b"%s -z -d --start-address=0x%llx --stop-address=0x%llx %s\0".as_ptr() as *const c_char;
        ret = snprintf(cmd.as_mut_ptr(), cmd.len(), fmt, test_objdump_path, addr, stop_address, filename);
        if ret <= 0 || ret as size_t >= cmd.len() {
            return -1;
        }

        pr_debug(b"Objdump command is: %s\n\0".as_ptr() as *const c_char, cmd.as_ptr());

        /* Ignore objdump errors */
        strcat(cmd.as_mut_ptr(), b" 2>/dev/null\0".as_ptr() as *const c_char);

        f = popen(cmd.as_ptr(), b"r\0".as_ptr() as *const c_char);
        if f.is_null() {
            pr_debug(b"popen failed\n\0".as_ptr() as *const c_char);
            return -1;
        }

        ret = read_objdump_output(f, buf, &mut len, addr);
        if len != 0 {
            pr_debug(b"objdump read too few bytes: %zd\n\0".as_ptr() as *const c_char, len);
            if ret == 0 {
                ret = len as c_int;
            }
        }

        pclose(f);
    }

    ret
}

unsafe fn dump_buf(buf: *mut u8, len: size_t) {
    unsafe {
        for i in 0..len {
            pr_debug(b"0x%02x \0".as_ptr() as *const c_char, *buf.add(i) as c_int);
            if i % 16 == 15 {
                pr_debug(b"\n\0".as_ptr() as *const c_char);
            }
        }
        pr_debug(b"\n\0".as_ptr() as *const c_char);
    }
}

unsafe fn read_object_code(addr: u64, mut len: size_t, cpumode: u8, thread: *mut thread, tested_sections: *mut rb_root) -> c_int {
    let mut al: addr_location = unsafe { mem::zeroed() };
    let mut buf1 = [0u8; BUFSZ];
    let mut buf2 = [0u8; BUFSZ];
    let ret_len: size_t;
    let objdump_addr: u64;
    let skip_addr: u64;
    let mut objdump_name: *const c_char;
    let mut decomp_name = [0 as c_char; KMOD_DECOMP_LEN];
    let mut decomp = false;
    let mut ret: c_int;
    let mut err: c_int = 0;
    let dso: *mut dso;

    unsafe {
        pr_debug(b"Reading object code for memory address: %#llx\n\0".as_ptr() as *const c_char, addr);

        addr_location__init(&mut al);
        if !thread__find_map(thread, cpumode, addr, &mut al) || map__dso(al.map).is_null() {
            if cpumode == PERF_RECORD_MISC_HYPERVISOR {
                pr_debug(b"Hypervisor address can not be resolved - skipping\n\0".as_ptr() as *const c_char);
                goto_out(&mut al);
                return err;
            }

            pr_debug(b"thread__find_map failed\n\0".as_ptr() as *const c_char);
            err = -1;
            goto_out(&mut al);
            return err;
        }
        dso = map__dso(al.map);
        pr_debug(b"File is: %s\n\0".as_ptr() as *const c_char, dso__long_name(dso));

        if dso__symtab_type(dso) == DSO_BINARY_TYPE__KALLSYMS && !dso__is_kcore(dso) {
            pr_debug(b"Unexpected kernel address - skipping\n\0".as_ptr() as *const c_char);
            goto_out(&mut al);
            return err;
        }

        /*
         * Don't retest the same addresses. objdump struggles with kcore - try
         * each map only once even if the address is different.
         */
        skip_addr = if dso__is_kcore(dso) { map__start(al.map) } else { al.addr };
        if tested_code_insert_or_exists(dso__long_name(dso), skip_addr, tested_sections) {
            pr_debug(b"Already tested %s @ %#llx - skipping\n\0".as_ptr() as *const c_char, dso__long_name(dso), skip_addr);
            goto_out(&mut al);
            return err;
        }

        pr_debug(b"On file address is: %#llx\n\0".as_ptr() as *const c_char, al.addr);

        if len > BUFSZ {
            len = BUFSZ;
        }

        /* Do not go off the map */
        if addr + len as u64 > map__end(al.map) {
            len = (map__end(al.map) - addr) as size_t;
        }

        /*
         * Some architectures (ex: powerpc) have stubs (trampolines) in kernel
         * modules to manage long jumps. Check if the ip offset falls in stubs
         * sections for kernel modules. And skip module address after text end
         */
        if dso__is_kmod(dso) && al.addr > dso__text_end(dso) {
            pr_debug(b"skipping the module address %#llx after text end\n\0".as_ptr() as *const c_char, al.addr);
            goto_out(&mut al);
            return err;
        }

        /* Read the object code using perf */
        ret_len = dso__data_read_offset(dso, maps__machine(thread__maps(thread)), al.addr, buf1.as_mut_ptr(), len);
        if ret_len != len {
            pr_debug(b"dso__data_read_offset failed\n\0".as_ptr() as *const c_char);
            err = -1;
            goto_out(&mut al);
            return err;
        }

        /*
         * Converting addresses for use by objdump requires more information.
         * map__load() does that.  See map__rip_2objdump() for details.
         */
        if map__load(al.map) != 0 {
            err = -1;
            goto_out(&mut al);
            return err;
        }

        objdump_name = dso__long_name(dso);
        if dso__needs_decompress(dso) {
            if dso__decompress_kmodule_path(dso, objdump_name, decomp_name.as_mut_ptr(), decomp_name.len()) < 0 {
                pr_debug(b"decompression failed\n\0".as_ptr() as *const c_char);
                err = -1;
                goto_out(&mut al);
                return err;
            }

            /* empty pathname means file wasn't actually compressed */
            if decomp_name[0] != 0 {
                decomp = true;
                objdump_name = decomp_name.as_ptr();
            }
        }

        /* Read the object code using objdump */
        objdump_addr = map__rip_2objdump(al.map, al.addr);
        ret = read_via_objdump(objdump_name, objdump_addr, buf2.as_mut_ptr() as *mut c_void, len);

        if decomp {
            unlink(objdump_name);
        }

        if ret > 0 {
            /*
             * The kernel maps are inaccurate - assume objdump is right in
             * that case.
             */
            if cpumode == PERF_RECORD_MISC_KERNEL || cpumode == PERF_RECORD_MISC_GUEST_KERNEL {
                len -= ret as size_t;
                if len != 0 {
                    pr_debug(b"Reducing len to %zu\n\0".as_ptr() as *const c_char, len);
                } else if dso__is_kcore(dso) {
                    /*
                     * objdump cannot handle very large segments
                     * that may be found in kcore.
                     */
                    pr_debug(b"objdump failed for kcore\0".as_ptr() as *const c_char);
                    pr_debug(b" - skipping\n\0".as_ptr() as *const c_char);
                } else {
                    err = -1;
                }
                goto_out(&mut al);
                return err;
            }
        }
        if ret < 0 {
            pr_debug(b"read_via_objdump failed\n\0".as_ptr() as *const c_char);
            err = -1;
            goto_out(&mut al);
            return err;
        }

        /* The results should be identical */
        if memcmp(buf1.as_ptr() as *const c_void, buf2.as_ptr() as *const c_void, len) != 0 {
            pr_debug(b"Bytes read differ from those read by objdump\n\0".as_ptr() as *const c_char);
            pr_debug(b"buf1 (dso):\n\0".as_ptr() as *const c_char);
            dump_buf(buf1.as_mut_ptr(), len);
            pr_debug(b"buf2 (objdump):\n\0".as_ptr() as *const c_char);
            dump_buf(buf2.as_mut_ptr(), len);
            err = -1;
            goto_out(&mut al);
            return err;
        }
        pr_debug(b"Bytes read match those read by objdump\n\0".as_ptr() as *const c_char);
        goto_out(&mut al);
    }

    err
}

unsafe fn goto_out(al: *mut addr_location) {
    unsafe {
        addr_location__exit(al);
    }
}

unsafe fn process_sample_event(machine: *mut machine, evlist: *mut evlist, event: *mut perf_event, tested_sections: *mut rb_root) -> c_int {
    let mut sample: perf_sample = unsafe { mem::zeroed() };
    let mut thread: *mut thread;
    let mut ret: c_int;

    unsafe {
        perf_sample__init(&mut sample, false);
        ret = evlist__parse_sample(evlist, event, &mut sample);
        if ret != 0 {
            pr_debug(b"evlist__parse_sample failed\n\0".as_ptr() as *const c_char);
            ret = -1;
            perf_sample__exit(&mut sample);
            return ret;
        }

        thread = machine__findnew_thread(machine, sample.pid, sample.tid);
        if thread.is_null() {
            pr_debug(b"machine__findnew_thread failed\n\0".as_ptr() as *const c_char);
            ret = -1;
            perf_sample__exit(&mut sample);
            return ret;
        }

        ret = read_object_code(sample.ip, READLEN, sample.cpumode, thread, tested_sections);
        thread__put(thread);
        perf_sample__exit(&mut sample);
    }
    ret
}

unsafe fn process_event(machine: *mut machine, evlist: *mut evlist, event: *mut perf_event, tested_sections: *mut rb_root) -> c_int {
    unsafe {
        if (*event).header.type_ == PERF_RECORD_SAMPLE {
            return process_sample_event(machine, evlist, event, tested_sections);
        }

        if (*event).header.type_ == PERF_RECORD_THROTTLE || (*event).header.type_ == PERF_RECORD_UNTHROTTLE {
            return 0;
        }

        if (*event).header.type_ < PERF_RECORD_MAX {
            let ret: c_int;

            ret = machine__process_event(machine, event, ptr::null_mut());
            if ret < 0 {
                pr_debug(b"machine__process_event failed, event type %u\n\0".as_ptr() as *const c_char, (*event).header.type_);
            }
            return ret;
        }
    }

    0
}

unsafe fn process_events(machine: *mut machine, evlist: *mut evlist, tested_sections: *mut rb_root) -> c_int {
    let mut event: *mut perf_event;
    let mut md: *mut mmap;
    let mut i: c_int;
    let mut ret: c_int;

    unsafe {
        i = 0;
        while i < (*evlist__core(evlist)).nr_mmaps {
            md = evlist__mmap(evlist).add(i as usize);
            if perf_mmap__read_init(&mut (*md).core) < 0 {
                i += 1;
                continue;
            }

            loop {
                event = perf_mmap__read_event(&mut (*md).core);
                if event.is_null() {
                    break;
                }
                ret = process_event(machine, evlist, event, tested_sections);
                perf_mmap__consume(&mut (*md).core);
                if ret < 0 {
                    return ret;
                }
            }
            perf_mmap__read_done(&mut (*md).core);
            i += 1;
        }
    }
    0
}

unsafe extern "C" fn comp(a: *const c_void, b: *const c_void) -> c_int {
    unsafe { *(a as *const c_int) - *(b as *const c_int) }
}

unsafe fn do_sort_something() {
    let mut buf = [0 as c_int; 40960];

    unsafe {
        for i in 0..buf.len() {
            buf[i] = (buf.len() - i - 1) as c_int;
        }

        qsort(buf.as_mut_ptr() as *mut c_void, buf.len(), mem::size_of::<c_int>(), Some(comp));

        for i in 0..buf.len() {
            if buf[i] != i as c_int {
                pr_debug(b"qsort failed\n\0".as_ptr() as *const c_char);
                break;
            }
        }
    }
}

unsafe fn sort_something() {
    unsafe {
        for _i in 0..10 {
            do_sort_something();
        }
    }
}

unsafe fn syscall_something() {
    let mut pipefd = [0 as c_int; 2];

    unsafe {
        for _i in 0..1000 {
            if pipe(pipefd.as_mut_ptr()) < 0 {
                pr_debug(b"pipe failed\n\0".as_ptr() as *const c_char);
                break;
            }
            close(pipefd[1]);
            close(pipefd[0]);
        }
    }
}

unsafe fn fs_something() {
    let test_file_name = b"temp-perf-code-reading-test-file--\0";
    let mut f: *mut FILE;

    unsafe {
        for _i in 0..1000 {
            f = fopen(test_file_name.as_ptr() as *const c_char, b"w+\0".as_ptr() as *const c_char);
            if !f.is_null() {
                fclose(f);
                unlink(test_file_name.as_ptr() as *const c_char);
            }
        }
    }
}

unsafe fn do_something() {
    unsafe {
        fs_something();
        sort_something();
        syscall_something();
    }
}

unsafe fn do_test_code_reading(try_kcore: bool) -> c_int {
    let machine: *mut machine;
    let mut thread: *mut thread = ptr::null_mut();
    let mut opts = record_opts {
        mmap_pages: UINT_MAX,
        user_freq: UINT_MAX,
        user_interval: ULLONG_MAX,
        freq: 500,
        target: record_opts_target {
            uses_mmap: true,
        },
    };
    let mut tested_sections = rb_root { rb_node: ptr::null_mut() };
    let mut threads: *mut perf_thread_map = ptr::null_mut();
    let mut cpus: *mut perf_cpu_map = ptr::null_mut();
    let mut evlist: *mut evlist = ptr::null_mut();
    let mut evsel: *mut evsel = ptr::null_mut();
    let mut err: c_int = -1;
    let mut ret: c_int;
    let pid: pid_t;
    let map: *mut map;
    let have_vmlinux: bool;
    let have_kcore: bool;
    let dso: *mut dso;
    let events: [*const c_char; 5] = [
        b"cpu-cycles\0".as_ptr() as *const c_char,
        b"cpu-cycles:u\0".as_ptr() as *const c_char,
        b"cpu-clock\0".as_ptr() as *const c_char,
        b"cpu-clock:u\0".as_ptr() as *const c_char,
        ptr::null(),
    ];
    let mut evidx: usize = 0;
    let mut host_env: perf_env = unsafe { mem::zeroed() };

    unsafe {
        pid = getpid();

        perf_env__init(&mut host_env);
        machine = machine__new_host(&mut host_env);

        ret = machine__create_kernel_maps(machine);
        if ret < 0 {
            pr_debug(b"machine__create_kernel_maps failed\n\0".as_ptr() as *const c_char);
            goto_do_test_out_err(evlist, cpus, threads, machine, &mut host_env, &mut tested_sections);
            return err;
        }

        /* Force the use of kallsyms instead of vmlinux to try kcore */
        if try_kcore {
            symbol_conf.kallsyms_name = b"/proc/kallsyms\0".as_ptr() as *const c_char;
        }

        /* Load kernel map */
        map = machine__kernel_map(machine);
        ret = map__load(map);
        if ret < 0 {
            pr_debug(b"map__load failed\n\0".as_ptr() as *const c_char);
            goto_do_test_out_err(evlist, cpus, threads, machine, &mut host_env, &mut tested_sections);
            return err;
        }
        dso = map__dso(map);
        have_vmlinux = dso__is_vmlinux(dso);
        have_kcore = dso__is_kcore(dso);

        /* 2nd time through we just try kcore */
        if try_kcore && !have_kcore {
            return TEST_CODE_READING_NO_KCORE;
        }

        /* No point getting kernel events if there is no kernel object */
        if !have_vmlinux && !have_kcore {
            evidx += 1;
        }

        threads = thread_map__new_by_tid(pid);
        if threads.is_null() {
            pr_debug(b"thread_map__new_by_tid failed\n\0".as_ptr() as *const c_char);
            goto_do_test_out_err(evlist, cpus, threads, machine, &mut host_env, &mut tested_sections);
            return err;
        }

        ret = perf_event__synthesize_thread_map(ptr::null_mut(), threads, &perf_event__process as *const c_void as *mut c_void, machine, true, false);
        if ret < 0 {
            pr_debug(b"perf_event__synthesize_thread_map failed\n\0".as_ptr() as *const c_char);
            goto_do_test_out_err(evlist, cpus, threads, machine, &mut host_env, &mut tested_sections);
            return err;
        }

        thread = machine__findnew_thread(machine, pid, pid);
        if thread.is_null() {
            pr_debug(b"machine__findnew_thread failed\n\0".as_ptr() as *const c_char);
            goto_do_test_out_put(thread, evlist, cpus, threads, machine, &mut host_env, &mut tested_sections);
            return err;
        }

        cpus = perf_cpu_map__new_online_cpus();
        if cpus.is_null() {
            pr_debug(b"perf_cpu_map__new failed\n\0".as_ptr() as *const c_char);
            goto_do_test_out_put(thread, evlist, cpus, threads, machine, &mut host_env, &mut tested_sections);
            return err;
        }

        while !events[evidx].is_null() {
            let str_: *const c_char;

            evlist = evlist__new();
            if evlist.is_null() {
                pr_debug(b"evlist__new failed\n\0".as_ptr() as *const c_char);
                goto_do_test_out_put(thread, evlist, cpus, threads, machine, &mut host_env, &mut tested_sections);
                return err;
            }

            perf_evlist__set_maps(evlist__core(evlist), cpus, threads);

            str_ = events[evidx];
            pr_debug(b"Parsing event '%s'\n\0".as_ptr() as *const c_char, str_);
            ret = parse_event(evlist, str_);
            if ret < 0 {
                pr_debug(b"parse_events failed\n\0".as_ptr() as *const c_char);
                goto_do_test_out_put(thread, evlist, cpus, threads, machine, &mut host_env, &mut tested_sections);
                return err;
            }

            evlist__config(evlist, &mut opts, ptr::null_mut());

            /* Translation of evlist__for_each_entry(evlist, evsel). */
            while !evsel.is_null() {
                (*evsel).core.attr.comm = 1;
                (*evsel).core.attr.disabled = 1;
                (*evsel).core.attr.enable_on_exec = 0;
                break;
            }

            ret = evlist__open(evlist);
            if ret < 0 {
                evidx += 1;

                if events[evidx].is_null() && verbose > 0 {
                    let mut errbuf = [0 as c_char; 512];
                    evlist__strerror_open(evlist, *__errno_location(), errbuf.as_mut_ptr(), errbuf.len());
                    pr_debug(b"perf_evlist__open() failed!\n%s\n\0".as_ptr() as *const c_char, errbuf.as_ptr());
                }

                perf_evlist__set_maps(evlist__core(evlist), ptr::null_mut(), ptr::null_mut());
                evlist__put(evlist);
                evlist = ptr::null_mut();
                continue;
            }
            break;
        }

        if events[evidx].is_null() {
            goto_do_test_out_put(thread, evlist, cpus, threads, machine, &mut host_env, &mut tested_sections);
            return err;
        }

        ret = evlist__do_mmap(evlist, UINT_MAX);
        if ret < 0 {
            pr_debug(b"evlist__mmap failed\n\0".as_ptr() as *const c_char);
            goto_do_test_out_put(thread, evlist, cpus, threads, machine, &mut host_env, &mut tested_sections);
            return err;
        }

        evlist__enable(evlist);

        do_something();

        evlist__disable(evlist);

        ret = process_events(machine, evlist, &mut tested_sections);
        if ret < 0 {
            goto_do_test_out_put(thread, evlist, cpus, threads, machine, &mut host_env, &mut tested_sections);
            return err;
        }

        if !have_vmlinux && !have_kcore && !try_kcore {
            err = TEST_CODE_READING_NO_KERNEL_OBJ;
        } else if !have_vmlinux && !try_kcore {
            err = TEST_CODE_READING_NO_VMLINUX;
        } else if !strstr(events[evidx], b":u\0".as_ptr() as *const c_char).is_null() {
            err = TEST_CODE_READING_NO_ACCESS;
        } else {
            err = TEST_CODE_READING_OK;
        }
        goto_do_test_out_put(thread, evlist, cpus, threads, machine, &mut host_env, &mut tested_sections);
    }

    err
}

unsafe fn goto_do_test_out_put(thread: *mut thread, evlist: *mut evlist, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map, machine: *mut machine, host_env: *mut perf_env, tested_sections: *mut rb_root) {
    unsafe {
        thread__put(thread);
        goto_do_test_out_err(evlist, cpus, threads, machine, host_env, tested_sections);
    }
}

unsafe fn goto_do_test_out_err(evlist: *mut evlist, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map, machine: *mut machine, host_env: *mut perf_env, tested_sections: *mut rb_root) {
    unsafe {
        evlist__put(evlist);
        perf_cpu_map__put(cpus);
        perf_thread_map__put(threads);
        machine__delete(machine);
        perf_env__exit(host_env);
        tested_sections__free(tested_sections);
    }
}

unsafe fn test__code_reading(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut ret: c_int;

    unsafe {
        ret = do_test_code_reading(false);
        if ret == 0 {
            ret = do_test_code_reading(true);
        }

        match ret {
            TEST_CODE_READING_OK => 0,
            TEST_CODE_READING_NO_VMLINUX => {
                pr_debug(b"no vmlinux\n\0".as_ptr() as *const c_char);
                0
            }
            TEST_CODE_READING_NO_KCORE => {
                pr_debug(b"no kcore\n\0".as_ptr() as *const c_char);
                0
            }
            TEST_CODE_READING_NO_ACCESS => {
                pr_debug(b"no access\n\0".as_ptr() as *const c_char);
                0
            }
            TEST_CODE_READING_NO_KERNEL_OBJ => {
                pr_debug(b"no kernel obj\n\0".as_ptr() as *const c_char);
                0
            }
            _ => -1,
        }
    }
}

/* DEFINE_SUITE("Object code reading", code_reading); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
