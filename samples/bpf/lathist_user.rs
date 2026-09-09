// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2013-2015 PLUMgrid, http://plumgrid.com
 * Copyright (c) 2015 BMW Car IT GmbH
 */

// C dependencies supplied by the surrounding build are intentionally left external.

const MAX_ENTRIES: usize = 20;
const MAX_CPU: usize = 4;
const MAX_STARS: usize = 40;

#[repr(C)]
struct cpu_hist {
    data: [libc::c_long; MAX_ENTRIES],
    max: libc::c_long,
}

static mut cpu_hist: [cpu_hist; MAX_CPU] = [
    cpu_hist {
        data: [0; MAX_ENTRIES],
        max: 0,
    };
    MAX_CPU
];

extern "C" {
    fn printf(format: *const libc::c_char, ...) -> libc::c_int;
    fn fprintf(stream: *mut libc::FILE, format: *const libc::c_char, ...)
        -> libc::c_int;
    fn snprintf(
        string: *mut libc::c_char,
        size: libc::size_t,
        format: *const libc::c_char,
        ...,
    ) -> libc::c_int;
    fn sleep(seconds: libc::c_uint) -> libc::c_uint;

    static mut stderr: *mut libc::FILE;

    fn bpf_map_lookup_elem(fd: libc::c_int, key: *const libc::c_void, value: *mut libc::c_void)
        -> libc::c_int;
    fn bpf_object__open_file(
        path: *const libc::c_char,
        opts: *const libc::c_void,
    ) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const libc::c_void) -> libc::c_long;
    fn bpf_object__load(obj: *mut bpf_object) -> libc::c_int;
    fn bpf_object__find_map_fd_by_name(
        obj: *mut bpf_object,
        name: *const libc::c_char,
    ) -> libc::c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link) -> libc::c_int;
    fn bpf_object__close(obj: *mut bpf_object);
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}
#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}
#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

unsafe fn stars(str_: *mut libc::c_char, val: libc::c_long, max: libc::c_long, width: libc::c_int) {
    let mut i: libc::c_int = 0;

    while i < (width * val / max) - 1 && i < width - 1 {
        *str_.offset(i as isize) = b'*' as libc::c_char;
        i += 1;
    }
    if val > max {
        *str_.offset((i - 1) as isize) = b'+' as libc::c_char;
    }
    *str_.offset(i as isize) = 0;
}

unsafe fn print_hist() {
    let mut starstr = [0 as libc::c_char; MAX_STARS];
    let mut hist: *mut cpu_hist;
    let mut i: libc::c_int;
    let mut j: libc::c_int;

    /* clear screen */
    printf(b"\x1b[2J\0".as_ptr() as *const libc::c_char);

    j = 0;
    while j < MAX_CPU as libc::c_int {
        hist = &mut cpu_hist[j as usize];

        /* ignore CPUs without data (maybe offline?) */
        if (*hist).max == 0 {
            j += 1;
            continue;
        }

        printf(b"CPU %d\n\0".as_ptr() as *const libc::c_char, j);
        printf(b"      latency        : count     distribution\n\0".as_ptr() as *const libc::c_char);
        i = 1;
        while i <= MAX_ENTRIES as libc::c_int {
            stars(starstr.as_mut_ptr(), (*hist).data[(i - 1) as usize], (*hist).max, MAX_STARS as libc::c_int);
            printf(
                b"%8ld -> %-8ld : %-8ld |%-*s|\n\0".as_ptr() as *const libc::c_char,
                (1i64 << i) >> 1,
                (1i64 << i) - 1,
                (*hist).data[(i - 1) as usize],
                MAX_STARS as libc::c_int,
                starstr.as_ptr(),
            );
            i += 1;
        }
        j += 1;
    }
}

unsafe fn get_data(fd: libc::c_int) {
    let mut key: libc::c_long = 0;
    let mut value: libc::c_long = 0;
    let mut c: libc::c_int;
    let mut i: libc::c_int;

    i = 0;
    while i < MAX_CPU as libc::c_int {
        cpu_hist[i as usize].max = 0;
        i += 1;
    }

    c = 0;
    while c < MAX_CPU as libc::c_int {
        i = 0;
        while i < MAX_ENTRIES as libc::c_int {
            key = (c as libc::c_long) * MAX_ENTRIES as libc::c_long + i as libc::c_long;
            bpf_map_lookup_elem(
                fd,
                &key as *const libc::c_long as *const libc::c_void,
                &mut value as *mut libc::c_long as *mut libc::c_void,
            );

            cpu_hist[c as usize].data[i as usize] = value;
            if value > cpu_hist[c as usize].max {
                cpu_hist[c as usize].max = value;
            }
            i += 1;
        }
        c += 1;
    }
}

unsafe fn main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut links: [*mut bpf_link; 2] = [core::ptr::null_mut(); 2];
    let mut prog: *mut bpf_program;
    let mut obj: *mut bpf_object;
    let mut filename = [0 as libc::c_char; 256];
    let mut map_fd: libc::c_int;
    let mut i: libc::c_int = 0;

    snprintf(
        filename.as_mut_ptr(),
        core::mem::size_of_val(&filename),
        b"%s_kern.o\0".as_ptr() as *const libc::c_char,
        *argv,
    );
    obj = bpf_object__open_file(filename.as_ptr(), core::ptr::null());
    if libbpf_get_error(obj as *const libc::c_void) != 0 {
        fprintf(stderr, b"ERROR: opening BPF object file failed\n\0".as_ptr() as *const libc::c_char);
        return 0;
    }

    /* load BPF program */
    if bpf_object__load(obj) != 0 {
        fprintf(stderr, b"ERROR: loading BPF object file failed\n\0".as_ptr() as *const libc::c_char);
        goto_cleanup(&mut i, &mut links, obj);
        return 0;
    }

    map_fd = bpf_object__find_map_fd_by_name(obj, b"my_lat\0".as_ptr() as *const libc::c_char);
    if map_fd < 0 {
        fprintf(stderr, b"ERROR: finding a map in obj file failed\n\0".as_ptr() as *const libc::c_char);
        goto_cleanup(&mut i, &mut links, obj);
        return 0;
    }

    // Corresponds to the libbpf bpf_object__for_each_program(prog, obj) macro.
    bpf_object__for_each_program!(prog, obj, {
        links[i as usize] = bpf_program__attach(prog);
        if libbpf_get_error(links[i as usize] as *const libc::c_void) != 0 {
            fprintf(stderr, b"ERROR: bpf_program__attach failed\n\0".as_ptr() as *const libc::c_char);
            links[i as usize] = core::ptr::null_mut();
            goto_cleanup(&mut i, &mut links, obj);
            return 0;
        }
        i += 1;
    });

    loop {
        get_data(map_fd);
        print_hist();
        sleep(5);
    }
}

unsafe fn goto_cleanup(i: &mut libc::c_int, links: &mut [*mut bpf_link; 2], obj: *mut bpf_object) {
    *i -= 1;
    while *i >= 0 {
        bpf_link__destroy(links[*i as usize]);
        *i -= 1;
    }
    bpf_object__close(obj);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
