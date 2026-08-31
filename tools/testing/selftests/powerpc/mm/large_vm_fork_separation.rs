// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2019, Michael Ellerman, IBM Corp.
//
// Test that allocating memory beyond the memory limit and then forking is
// handled correctly, ie. the child is able to access the mappings beyond the
// memory limit and the child's writes are not visible to the parent.

use libc::{
    c_char, c_int, c_long, c_void, exit, fork, getpid, mmap, perror, pipe, printf, read, sysconf,
    waitpid, write, MAP_ANONYMOUS, MAP_FAILED, MAP_FIXED, MAP_PRIVATE, PROT_READ, PROT_WRITE,
    _SC_PAGESIZE,
};

// From "utils.h".
unsafe extern "C" {
    fn barrier();
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

// If MAP_FIXED_NOREPLACE is not supplied by the libc bindings, the original C
// falls back to MAP_FIXED ("Should be safe" above 512TB). Keep the Linux value
// here for the direct translation.
const MAP_FIXED_NOREPLACE: c_int = 0x100000;

unsafe fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn fail_if(condition: bool) {
    if condition {
        exit(1);
    }
}

unsafe extern "C" fn test() -> c_int {
    let mut p2c: [c_int; 2] = [0; 2];
    let mut c2p: [c_int; 2] = [0; 2];
    let mut rc: c_int;
    let mut status: c_int = 0;
    let mut c: c_int = 0;
    let mut p: *mut c_int;
    let page_size: c_long;
    let mut pid: libc::pid_t;

    page_size = sysconf(_SC_PAGESIZE);
    if page_size != 65536 {
        return 0;
    }

    // Create a mapping at 512TB to allocate an extended_id
    p = mmap(
        ((512_u64) << 40) as *mut c_void,
        page_size as usize,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED_NOREPLACE,
        -1,
        0,
    ) as *mut c_int;
    if p as *mut c_void == MAP_FAILED {
        perror(c"mmap".as_ptr());
        printf(
            c"Error: couldn't mmap(), confirm kernel has 4TB support?\n".as_ptr(),
        );
        return 1;
    }

    printf(c"parent writing %p = 1\n".as_ptr(), p);
    *p = 1;

    fail_if(pipe(p2c.as_mut_ptr()) == -1 || pipe(c2p.as_mut_ptr()) == -1);

    pid = fork();
    if pid == 0 {
        fail_if(read(p2c[0], &mut c as *mut c_int as *mut c_void, 1) != 1);

        pid = getpid();
        printf(c"child writing  %p = %d\n".as_ptr(), p, pid);
        *p = pid;

        fail_if(write(c2p[1], &c as *const c_int as *const c_void, 1) != 1);
        fail_if(read(p2c[0], &mut c as *mut c_int as *mut c_void, 1) != 1);
        exit(0);
    }

    c = 0;
    fail_if(write(p2c[1], &c as *const c_int as *const c_void, 1) != 1);
    fail_if(read(c2p[0], &mut c as *mut c_int as *mut c_void, 1) != 1);

    // Prevent compiler optimisation
    barrier();

    rc = 0;
    printf(c"parent reading %p = %d\n".as_ptr(), p, *p);
    if *p != 1 {
        printf(c"Error: BUG! parent saw child's write! *p = %d\n".as_ptr(), *p);
        rc = 1;
    }

    fail_if(write(p2c[1], &c as *const c_int as *const c_void, 1) != 1);
    fail_if(waitpid(pid, &mut status as *mut c_int, 0) == -1);
    fail_if(!wifexited(status) || wexitstatus(status) != 0);

    if rc == 0 {
        printf(c"success: test completed OK\n".as_ptr());
    }

    rc
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            test,
            c"large_vm_fork_separation".as_ptr(),
        ));
    }
}
