// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netstat_counter {
    pub val: u64,
    pub name: *mut c_char,
}

#[repr(C)]
pub struct netstat {
    pub header_name: *mut c_char,
    pub next: *mut netstat,
    pub counters_nr: usize,
    pub counters: *mut netstat_counter,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn fclose(stream: *mut FILE) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn free(ptr: *mut c_void);
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut usize, stream: *mut FILE) -> isize;
    fn malloc(size: usize) -> *mut c_void;
    fn reallocarray(ptr: *mut c_void, nmemb: usize, size: usize) -> *mut c_void;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strndup(s: *const c_char, n: usize) -> *mut c_char;

    fn test_error(fmt: *const c_char, ...) -> !;
    fn test_print(fmt: *const c_char, ...);
}

#[inline]
fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

#[inline]
fn unlikely(v: bool) -> bool {
    v
}

unsafe fn lookup_type(mut ns: *mut netstat, type_: *const c_char, len: usize) -> *mut netstat {
    unsafe {
        while !ns.is_null() {
            let cmp = max(len, strlen((*ns).header_name));

            if strncmp((*ns).header_name, type_, cmp) == 0 {
                return ns;
            }
            ns = (*ns).next;
        }
        core::ptr::null_mut()
    }
}

unsafe fn lookup_get(ns: *mut netstat, type_: *const c_char, len: usize) -> *mut netstat {
    unsafe {
        let mut ret: *mut netstat;

        ret = lookup_type(ns, type_, len);
        if !ret.is_null() {
            return ret;
        }

        ret = malloc(core::mem::size_of::<netstat>()) as *mut netstat;
        if ret.is_null() {
            test_error(c"malloc()".as_ptr());
        }

        (*ret).header_name = strndup(type_, len);
        if (*ret).header_name.is_null() {
            test_error(c"strndup()".as_ptr());
        }
        (*ret).next = ns;
        (*ret).counters_nr = 0;
        (*ret).counters = core::ptr::null_mut();

        ret
    }
}

unsafe fn lookup_get_column(ns: *mut netstat, line: *const c_char) -> *mut netstat {
    unsafe {
        let column: *mut c_char;

        column = strchr(line, ':' as c_int);
        if column.is_null() {
            test_error(c"can't parse netstat file".as_ptr());
        }

        lookup_get(ns, line, column.offset_from(line) as usize)
    }
}

unsafe fn netstat_read_type(fnetstat: *mut FILE, dest: *mut *mut netstat, line: *mut c_char) {
    unsafe {
        let type_ = lookup_get_column(*dest, line);
        let mut pos = line as *const c_char;
        let mut i: usize;
        let mut nr_elems: usize = 0;
        let mut tmp: c_char = 0;

        while {
            pos = strchr(pos, ' ' as c_int) as *const c_char;
            !pos.is_null()
        } {
            nr_elems += 1;
            pos = pos.add(1);
        }

        *dest = type_;
        (*type_).counters = reallocarray(
            (*type_).counters as *mut c_void,
            (*type_).counters_nr + nr_elems,
            core::mem::size_of::<netstat_counter>(),
        ) as *mut netstat_counter;
        if (*type_).counters.is_null() {
            test_error(c"reallocarray()".as_ptr());
        }

        pos = strchr(line, ' ' as c_int).add(1) as *const c_char;

        if fscanf(fnetstat, c"%[^ :]".as_ptr(), (*type_).header_name) == -1 {
            test_error(c"fscanf(%s)".as_ptr(), (*type_).header_name);
        }
        if fread(
            &mut tmp as *mut c_char as *mut c_void,
            1,
            1,
            fnetstat,
        ) != 1
            || tmp != ':' as c_char
        {
            test_error(c"Unexpected netstat format (%c)".as_ptr(), tmp as c_int);
        }

        i = (*type_).counters_nr;
        while i < (*type_).counters_nr + nr_elems {
            let nc = (*type_).counters.add(i);
            let mut new_pos = strchr(pos, ' ' as c_int) as *const c_char;
            let mut fmt = c" %lu".as_ptr();

            if new_pos.is_null() {
                new_pos = strchr(pos, '\n' as c_int) as *const c_char;
            }

            (*nc).name = strndup(pos, new_pos.offset_from(pos) as usize);
            if (*nc).name.is_null() {
                test_error(c"strndup()".as_ptr());
            }

            if unlikely(strcmp((*nc).name, c"MaxConn".as_ptr()) == 0) {
                fmt = c" %ld".as_ptr(); /* MaxConn is signed, RFC 2012 */
            }
            if fscanf(fnetstat, fmt, &mut (*nc).val as *mut u64) != 1 {
                test_error(c"fscanf(%s)".as_ptr(), (*nc).name);
            }
            pos = new_pos.add(1);
            i += 1;
        }
        (*type_).counters_nr += nr_elems;

        if fread(
            &mut tmp as *mut c_char as *mut c_void,
            1,
            1,
            fnetstat,
        ) != 1
            || tmp != '\n' as c_char
        {
            test_error(c"Unexpected netstat format".as_ptr());
        }
    }
}

static SNMP6_NAME: &[u8] = b"Snmp6\0";

unsafe fn snmp6_read(fnetstat: *mut FILE, dest: *mut *mut netstat) {
    unsafe {
        let type_ = lookup_get(*dest, SNMP6_NAME.as_ptr() as *const c_char, strlen(SNMP6_NAME.as_ptr() as *const c_char));
        let mut counter_name: *mut c_char = core::ptr::null_mut();
        let mut i: usize;

        i = (*type_).counters_nr;
        loop {
            let nc: *mut netstat_counter;
            let mut counter: u64 = 0;

            if fscanf(fnetstat, c"%ms".as_ptr(), &mut counter_name as *mut *mut c_char) == -1 {
                break;
            }
            if fscanf(fnetstat, c"%lu".as_ptr(), &mut counter as *mut u64) == -1 {
                test_error(c"Unexpected snmp6 format".as_ptr());
            }
            (*type_).counters = reallocarray(
                (*type_).counters as *mut c_void,
                i + 1,
                core::mem::size_of::<netstat_counter>(),
            ) as *mut netstat_counter;
            if (*type_).counters.is_null() {
                test_error(c"reallocarray()".as_ptr());
            }
            nc = (*type_).counters.add(i);
            (*nc).name = counter_name;
            (*nc).val = counter;
            i += 1;
        }
        (*type_).counters_nr = i;
        *dest = type_;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn netstat_read() -> *mut netstat {
    unsafe {
        let mut ret: *mut netstat = core::ptr::null_mut();
        let mut line_sz: usize = 0;
        let mut line: *mut c_char = core::ptr::null_mut();
        let mut fnetstat: *mut FILE;

        /*
         * Opening thread-self instead of /proc/net/... as the latter
         * points to /proc/self/net/ which instantiates thread-leader's
         * net-ns, see:
         * commit 155134fef2b6 ("Revert "proc: Point /proc/{mounts,net} at..")
         */
        errno = 0;
        fnetstat = fopen(c"/proc/thread-self/net/netstat".as_ptr(), c"r".as_ptr());
        if fnetstat.is_null() {
            test_error(c"failed to open /proc/net/netstat".as_ptr());
        }

        while getline(&mut line as *mut *mut c_char, &mut line_sz as *mut usize, fnetstat) != -1 {
            netstat_read_type(fnetstat, &mut ret as *mut *mut netstat, line);
        }
        fclose(fnetstat);

        errno = 0;
        fnetstat = fopen(c"/proc/thread-self/net/snmp".as_ptr(), c"r".as_ptr());
        if fnetstat.is_null() {
            test_error(c"failed to open /proc/net/snmp".as_ptr());
        }

        while getline(&mut line as *mut *mut c_char, &mut line_sz as *mut usize, fnetstat) != -1 {
            netstat_read_type(fnetstat, &mut ret as *mut *mut netstat, line);
        }
        fclose(fnetstat);

        errno = 0;
        fnetstat = fopen(c"/proc/thread-self/net/snmp6".as_ptr(), c"r".as_ptr());
        if fnetstat.is_null() {
            test_error(c"failed to open /proc/net/snmp6".as_ptr());
        }

        snmp6_read(fnetstat, &mut ret as *mut *mut netstat);
        fclose(fnetstat);

        free(line as *mut c_void);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn netstat_free(mut ns: *mut netstat) {
    unsafe {
        while !ns.is_null() {
            let prev = ns;
            let mut i: usize;

            free((*ns).header_name as *mut c_void);
            i = 0;
            while i < (*ns).counters_nr {
                free((*(*ns).counters.add(i)).name as *mut c_void);
                i += 1;
            }
            free((*ns).counters as *mut c_void);
            ns = (*ns).next;
            free(prev as *mut c_void);
        }
    }
}

#[inline]
unsafe fn __netstat_print_diff(a: u64, nsb: *mut netstat, i: usize) {
    unsafe {
        if unlikely(strcmp((*nsb).header_name, c"MaxConn".as_ptr()) == 0) {
            test_print(
                c"%8s %25s: %ld => %ld".as_ptr(),
                (*nsb).header_name,
                (*(*nsb).counters.add(i)).name,
                a as i64,
                (*(*nsb).counters.add(i)).val as i64,
            );
            return;
        }

        test_print(
            c"%8s %25s: %lu => %lu".as_ptr(),
            (*nsb).header_name,
            (*(*nsb).counters.add(i)).name,
            a,
            (*(*nsb).counters.add(i)).val,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn netstat_print_diff(mut nsa: *mut netstat, mut nsb: *mut netstat) {
    unsafe {
        let mut i: usize;
        let mut j: usize;

        while !nsb.is_null() {
            if unlikely(strcmp((*nsb).header_name, (*nsa).header_name) != 0) {
                i = 0;
                while i < (*nsb).counters_nr {
                    __netstat_print_diff(0, nsb, i);
                    i += 1;
                }
                nsb = (*nsb).next;
                continue;
            }

            if (*nsb).counters_nr < (*nsa).counters_nr {
                test_error(c"Unexpected: some counters disappeared!".as_ptr());
            }

            j = 0;
            i = 0;
            while i < (*nsb).counters_nr {
                if strcmp((*(*nsb).counters.add(i)).name, (*(*nsa).counters.add(j)).name) != 0 {
                    __netstat_print_diff(0, nsb, i);
                    i += 1;
                    continue;
                }

                if (*(*nsa).counters.add(j)).val == (*(*nsb).counters.add(i)).val {
                    j += 1;
                    i += 1;
                    continue;
                }

                __netstat_print_diff((*(*nsa).counters.add(j)).val, nsb, i);
                j += 1;
                i += 1;
            }
            if j != (*nsa).counters_nr {
                test_error(c"Unexpected: some counters disappeared!".as_ptr());
            }

            nsb = (*nsb).next;
            nsa = (*nsa).next;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn netstat_get(
    mut ns: *mut netstat,
    name: *const c_char,
    not_found: *mut bool,
) -> u64 {
    unsafe {
        if !not_found.is_null() {
            *not_found = false;
        }

        while !ns.is_null() {
            let mut i: usize;

            i = 0;
            while i < (*ns).counters_nr {
                if strcmp(name, (*(*ns).counters.add(i)).name) == 0 {
                    return (*(*ns).counters.add(i)).val;
                }
                i += 1;
            }

            ns = (*ns).next;
        }

        if !not_found.is_null() {
            *not_found = true;
        }
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
