// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external requirements:
// <test_progs.h>, <sys/user.h>/<unistd.h>, and "arena_mem_usage.skel.h".
// PAGE_SIZE came either from sys/user.h or from getpagesize().

use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub retval: c_int,
}

#[repr(C)]
pub struct arena_mem_usage {
    pub maps: arena_mem_usage_maps,
    pub progs: arena_mem_usage_progs,
    pub bss: *mut arena_mem_usage_bss,
}

#[repr(C)]
pub struct arena_mem_usage_maps {
    pub arena: *mut bpf_map,
}

#[repr(C)]
pub struct arena_mem_usage_progs {
    pub alloc: *mut bpf_program,
    pub free_pages: *mut bpf_program,
}

#[repr(C)]
pub struct arena_mem_usage_bss {
    pub alloc_cnt: c_long,
    pub ptr: *mut c_char,
    pub free_byte_off: c_long,
    pub free_cnt: c_long,
}

unsafe extern "C" {
    fn getpagesize() -> c_int;

    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_long, expected: c_long, name: *const c_char) -> bool;

    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__initial_value(map: *mut bpf_map, sz: *mut usize) -> *mut c_char;

    fn arena_mem_usage__open_and_load() -> *mut arena_mem_usage;
    fn arena_mem_usage__destroy(obj: *mut arena_mem_usage);
}

#[inline]
unsafe fn page_size() -> c_long {
    unsafe { getpagesize() as c_long }
}

/*
 * arena_map_mem_usage() is surfaced to user space through the map's
 * /proc/<pid>/fdinfo/<fd> "memlock:" line (the same value bpftool map show
 * prints). Read it directly so the test has no external dependency.
 */
unsafe fn map_memlock(map_fd: c_int) -> c_long {
    let mut path: [c_char; 64] = [0; 64];
    let mut line: [c_char; 128] = [0; 128];
    let mut memlock: c_long = -1;
    let f: *mut FILE;

    unsafe {
        snprintf(
            path.as_mut_ptr(),
            path.len(),
            c"/proc/self/fdinfo/%d".as_ptr(),
            map_fd,
        );
        f = fopen(path.as_ptr(), c"r".as_ptr());
        if !ASSERT_OK_PTR(f as *const c_void, c"open_fdinfo".as_ptr()) {
            return -1;
        }
        while !fgets(line.as_mut_ptr(), line.len() as c_int, f).is_null() {
            if sscanf(
                line.as_ptr(),
                c"memlock:\t%ld".as_ptr(),
                &mut memlock as *mut c_long,
            ) == 1
            {
                break;
            }
        }
        fclose(f);
        ASSERT_NEQ(memlock, -1, c"parse_memlock".as_ptr());
    }
    memlock
}

unsafe fn run(prog: *mut bpf_program, name: *const c_char) -> c_int {
    let mut opts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        retval: 0,
    };
    let err = unsafe { bpf_prog_test_run_opts(bpf_program__fd(prog), &mut opts) };

    unsafe {
        if !ASSERT_OK(err, name) {
            return -1;
        }
        if !ASSERT_OK(opts.retval, name) {
            return -1;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_arena_mem_usage() {
    let skel: *mut arena_mem_usage;
    let ps: c_long = unsafe { page_size() };
    let base: *mut c_char;
    let mut sz: usize = 0;
    let fd: c_int;
    let mut i: c_int;

    unsafe {
        skel = arena_mem_usage__open_and_load();
        if !ASSERT_OK_PTR(skel as *const c_void, c"open_load".as_ptr()) {
            return;
        }
        fd = bpf_map__fd((*skel).maps.arena);

        /* Fresh arena: no data pages, and the scratch page is not counted. */
        ASSERT_EQ(map_memlock(fd), 0, c"initial".as_ptr());

        /* BPF-side allocation of 17 pages. */
        (*(*skel).bss).alloc_cnt = 17;
        if run((*skel).progs.alloc, c"alloc".as_ptr()) != 0 {
            goto_out(skel);
            return;
        }
        /*
         * A NULL ptr means bpf_arena_alloc_pages() itself failed (e.g. the host
         * is under memory pressure), not a miscount -- flag it distinctly so a
         * red CI run is not mistaken for a counting bug.
         */
        if !ASSERT_OK_PTR(
            (*(*skel).bss).ptr as *const c_void,
            c"arena_alloc_pages".as_ptr(),
        ) {
            goto_out(skel);
            return;
        }
        ASSERT_EQ(map_memlock(fd), 17 * ps, c"after_alloc".as_ptr());

        /* Free a single page (arena_free_pages page_cnt==1 path). */
        (*(*skel).bss).free_byte_off = 0;
        (*(*skel).bss).free_cnt = 1;
        if run((*skel).progs.free_pages, c"free_one".as_ptr()) != 0 {
            goto_out(skel);
            return;
        }
        ASSERT_EQ(map_memlock(fd), 16 * ps, c"after_free_one".as_ptr());

        /* Free ten pages in one call (bulk path); only the freed pages count. */
        (*(*skel).bss).free_byte_off = 1 * ps;
        (*(*skel).bss).free_cnt = 10;
        if run((*skel).progs.free_pages, c"free_bulk".as_ptr()) != 0 {
            goto_out(skel);
            return;
        }
        ASSERT_EQ(map_memlock(fd), 6 * ps, c"after_free_bulk".as_ptr());

        /* Free the remaining six -> arena empty again. */
        (*(*skel).bss).free_byte_off = 11 * ps;
        (*(*skel).bss).free_cnt = 6;
        if run((*skel).progs.free_pages, c"free_rest".as_ptr()) != 0 {
            goto_out(skel);
            return;
        }
        ASSERT_EQ(map_memlock(fd), 0, c"after_free_rest".as_ptr());

        /*
         * User-space fault-in: touching unallocated arena pages allocates them
         * through arena_vm_fault(). libbpf mmap()s the arena at map_extra during
         * load, so bpf_map__initial_value() hands back that base.
         */
        base = bpf_map__initial_value((*skel).maps.arena, &mut sz);
        if !ASSERT_OK_PTR(base as *const c_void, c"arena_base".as_ptr()) {
            goto_out(skel);
            return;
        }
        i = 0;
        while i < 8 {
            *base.offset((i as c_long * ps) as isize) = 1;
            i += 1;
        }
        ASSERT_EQ(map_memlock(fd), 8 * ps, c"after_faultin".as_ptr());

        /*
         * Free the faulted-in pages from BPF. They are mapped into the user vma
         * (elevated refcount), so this also exercises the zap path.
         */
        (*(*skel).bss).ptr = base;
        (*(*skel).bss).free_byte_off = 0;
        (*(*skel).bss).free_cnt = 8;
        if run((*skel).progs.free_pages, c"free_faulted".as_ptr()) != 0 {
            goto_out(skel);
            return;
        }
        ASSERT_EQ(map_memlock(fd), 0, c"after_free_faulted".as_ptr());

        goto_out(skel);
    }
}

unsafe fn goto_out(skel: *mut arena_mem_usage) {
    unsafe {
        arena_mem_usage__destroy(skel);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
