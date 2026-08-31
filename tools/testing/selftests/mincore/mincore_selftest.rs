// SPDX-License-Identifier: GPL-2.0+
/*
 * kselftest suite for mincore().
 *
 * Copyright (C) 2020 Collabora, Ltd.
 */

// C dependencies: stdio.h, errno.h, unistd.h, stdlib.h, sys/mman.h,
// string.h, fcntl.h, kselftest.h, kselftest_harness.h.
// The kselftest TEST/ASSERT/EXPECT/SKIP/TH_LOG/TEST_HARNESS_MAIN interfaces
// are expected to be provided by the surrounding Rust test harness.

/* Default test file size: 4MB */
const MB: usize = 1usize << 20;
const FILE_SIZE: usize = 4 * MB;

unsafe extern "C" {
    static mut errno: libc::c_int;
}

/*
 * Tests the user interface. This test triggers most of the documented
 * error conditions in mincore().
 */
test!(basic_interface, {
    let mut retval: libc::c_int;
    let page_size: libc::c_int;
    let mut vec: [libc::c_uchar; 1] = [0; 1];
    let mut addr: *mut libc::c_char;

    page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as libc::c_int };

    /* Query a 0 byte sized range */
    retval = unsafe { libc::mincore(0 as *mut libc::c_void, 0, vec.as_mut_ptr()) };
    expect_eq!(0, retval);

    /* Addresses in the specified range are invalid or unmapped */
    unsafe {
        errno = 0;
    }
    retval = unsafe {
        libc::mincore(
            core::ptr::null_mut(),
            page_size as libc::size_t,
            vec.as_mut_ptr(),
        )
    };
    expect_eq!(-1, retval);
    expect_eq!(libc::ENOMEM, unsafe { errno });

    unsafe {
        errno = 0;
    }
    addr = unsafe {
        libc::mmap(
            core::ptr::null_mut(),
            page_size as libc::size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED | libc::MAP_ANONYMOUS,
            -1,
            0,
        ) as *mut libc::c_char
    };
    assert_ne!(libc::MAP_FAILED as *mut libc::c_char, addr, {
        th_log!("mmap error: %s", unsafe { libc::strerror(errno) });
    });

    /* <addr> argument is not page-aligned */
    unsafe {
        errno = 0;
    }
    retval = unsafe {
        libc::mincore(
            addr.add(1) as *mut libc::c_void,
            page_size as libc::size_t,
            vec.as_mut_ptr(),
        )
    };
    expect_eq!(-1, retval);
    expect_eq!(libc::EINVAL, unsafe { errno });

    /* <length> argument is too large */
    unsafe {
        errno = 0;
    }
    retval = unsafe { libc::mincore(addr as *mut libc::c_void, -1isize as libc::size_t, vec.as_mut_ptr()) };
    expect_eq!(-1, retval);
    expect_eq!(libc::ENOMEM, unsafe { errno });

    /* <vec> argument points to an illegal address */
    unsafe {
        errno = 0;
    }
    retval = unsafe {
        libc::mincore(
            addr as *mut libc::c_void,
            page_size as libc::size_t,
            core::ptr::null_mut(),
        )
    };
    expect_eq!(-1, retval);
    expect_eq!(libc::EFAULT, unsafe { errno });
    unsafe {
        libc::munmap(addr as *mut libc::c_void, page_size as libc::size_t);
    }
});

/*
 * Test mincore() behavior on a private anonymous page mapping.
 * Check that the page is not loaded into memory right after the mapping
 * but after accessing it (on-demand allocation).
 * Then free the page and check that it's not memory-resident.
 */
test!(check_anonymous_locked_pages, {
    let mut vec: [libc::c_uchar; 1] = [0; 1];
    let addr: *mut libc::c_char;
    let mut retval: libc::c_int;
    let page_size: libc::c_int;

    page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as libc::c_int };

    /* Map one page and check it's not memory-resident */
    unsafe {
        errno = 0;
    }
    addr = unsafe {
        libc::mmap(
            core::ptr::null_mut(),
            page_size as libc::size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        ) as *mut libc::c_char
    };
    assert_ne!(libc::MAP_FAILED as *mut libc::c_char, addr, {
        th_log!("mmap error: %s", unsafe { libc::strerror(errno) });
    });
    retval = unsafe { libc::mincore(addr as *mut libc::c_void, page_size as libc::size_t, vec.as_mut_ptr()) };
    assert_eq!(0, retval);
    assert_eq!(0, vec[0], {
        th_log!("Page found in memory before use");
    });

    /* Touch the page and check again. It should now be in memory */
    unsafe {
        *addr = 1;
        libc::mlock(addr as *const libc::c_void, page_size as libc::size_t);
    }
    retval = unsafe { libc::mincore(addr as *mut libc::c_void, page_size as libc::size_t, vec.as_mut_ptr()) };
    assert_eq!(0, retval);
    assert_eq!(1, vec[0], {
        th_log!("Page not found in memory after use");
    });

    /*
     * It shouldn't be memory-resident after unlocking it and
     * marking it as unneeded.
     */
    unsafe {
        libc::munlock(addr as *const libc::c_void, page_size as libc::size_t);
        libc::madvise(addr as *mut libc::c_void, page_size as libc::size_t, libc::MADV_DONTNEED);
    }
    retval = unsafe { libc::mincore(addr as *mut libc::c_void, page_size as libc::size_t, vec.as_mut_ptr()) };
    assert_eq!(0, retval);
    assert_eq!(0, vec[0], {
        th_log!("Page in memory after being zapped");
    });
    unsafe {
        libc::munmap(addr as *mut libc::c_void, page_size as libc::size_t);
    }
});

/*
 * Check mincore() behavior on huge pages.
 * This test will be skipped if the mapping fails (ie. if there are no
 * huge pages available).
 *
 * Make sure the system has at least one free huge page, check
 * "HugePages_Free" in /proc/meminfo.
 * Increment /sys/kernel/mm/hugepages/hugepages-2048kB/nr_hugepages if
 * needed.
 */
test!(check_huge_pages, {
    let mut vec: [libc::c_uchar; 1] = [0; 1];
    let addr: *mut libc::c_char;
    let mut retval: libc::c_int;
    let page_size: libc::c_int;

    page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as libc::c_int };

    unsafe {
        errno = 0;
    }
    addr = unsafe {
        libc::mmap(
            core::ptr::null_mut(),
            page_size as libc::size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_HUGETLB,
            -1,
            0,
        ) as *mut libc::c_char
    };
    if addr == libc::MAP_FAILED as *mut libc::c_char {
        if unsafe { errno } == libc::ENOMEM || unsafe { errno } == libc::EINVAL {
            skip_return!("No huge pages available or CONFIG_HUGETLB_PAGE disabled.");
        } else {
            th_log!("mmap error: %s", unsafe { libc::strerror(errno) });
        }
    }
    retval = unsafe { libc::mincore(addr as *mut libc::c_void, page_size as libc::size_t, vec.as_mut_ptr()) };
    assert_eq!(0, retval);
    assert_eq!(0, vec[0], {
        th_log!("Page found in memory before use");
    });

    unsafe {
        *addr = 1;
        libc::mlock(addr as *const libc::c_void, page_size as libc::size_t);
    }
    retval = unsafe { libc::mincore(addr as *mut libc::c_void, page_size as libc::size_t, vec.as_mut_ptr()) };
    assert_eq!(0, retval);
    assert_eq!(1, vec[0], {
        th_log!("Page not found in memory after use");
    });

    unsafe {
        libc::munlock(addr as *const libc::c_void, page_size as libc::size_t);
        libc::munmap(addr as *mut libc::c_void, page_size as libc::size_t);
    }
});

/*
 * Test mincore() behavior on a file-backed page.
 * No pages should be loaded into memory right after the mapping. Then,
 * accessing any address in the mapping range should load the page
 * containing the address and a number of subsequent pages (readahead).
 *
 * The actual readahead settings depend on the test environment, so we
 * can't make a lot of assumptions about that. This test covers the most
 * general cases.
 */
test!(check_file_mmap, {
    let vec: *mut libc::c_uchar;
    let mut vec_size: libc::c_int;
    let addr: *mut libc::c_char;
    let mut retval: libc::c_int;
    let page_size: libc::c_int;
    let fd: libc::c_int;
    let mut i: libc::c_int;
    let mut ra_pages: libc::c_int = 0;

    page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as libc::c_int };
    vec_size = (FILE_SIZE / page_size as usize) as libc::c_int;
    if FILE_SIZE % page_size as usize != 0 {
        vec_size += 1;
    }

    vec = unsafe { libc::calloc(vec_size as libc::size_t, core::mem::size_of::<libc::c_uchar>()) as *mut libc::c_uchar };
    assert_ne!(core::ptr::null_mut(), vec, {
        th_log!("Can't allocate array");
    });

    unsafe {
        errno = 0;
    }
    fd = unsafe { libc::open(c".".as_ptr(), libc::O_TMPFILE | libc::O_RDWR, 0o600) };
    if fd < 0 {
        assert_eq!(unsafe { errno }, libc::EOPNOTSUPP, {
            th_log!("Can't create temporary file: %s", unsafe {
                libc::strerror(errno)
            });
        });
        skip_goto!(out_free, "O_TMPFILE not supported by filesystem.");
    }
    unsafe {
        errno = 0;
    }
    retval = unsafe { libc::fallocate(fd, 0, 0, FILE_SIZE as libc::off_t) };
    if retval != 0 {
        assert_eq!(unsafe { errno }, libc::EOPNOTSUPP, {
            th_log!("Error allocating space for the temporary file: %s", unsafe {
                libc::strerror(errno)
            });
        });
        skip_goto!(out_close, "fallocate not supported by filesystem.");
    }

    /*
     * Map the whole file, the pages shouldn't be fetched yet.
     */
    unsafe {
        errno = 0;
    }
    addr = unsafe {
        libc::mmap(
            core::ptr::null_mut(),
            FILE_SIZE as libc::size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        ) as *mut libc::c_char
    };
    assert_ne!(libc::MAP_FAILED as *mut libc::c_char, addr, {
        th_log!("mmap error: %s", unsafe { libc::strerror(errno) });
    });
    retval = unsafe { libc::mincore(addr as *mut libc::c_void, FILE_SIZE as libc::size_t, vec) };
    assert_eq!(0, retval);
    i = 0;
    while i < vec_size {
        assert_eq!(0, unsafe { *vec.add(i as usize) }, {
            th_log!("Unexpected page in memory");
        });
        i += 1;
    }

    /*
     * Touch a page in the middle of the mapping. We expect some
     * surrounding pages (the readahead window) to be populated too.
     * Depending on the page size and readahead setting, the pages may
     * land before the faulted page rather than after it.
     */
    unsafe {
        *addr.add(FILE_SIZE / 2) = 1;
    }
    retval = unsafe { libc::mincore(addr as *mut libc::c_void, FILE_SIZE as libc::size_t, vec) };
    assert_eq!(0, retval);
    assert_eq!(1, unsafe { *vec.add(FILE_SIZE / 2 / page_size as usize) }, {
        th_log!("Page not found in memory after use");
    });

    i = (FILE_SIZE / 2 / page_size as usize) as libc::c_int - 1;
    while i >= 0 && unsafe { *vec.add(i as usize) } != 0 {
        ra_pages += 1;
        i -= 1;
    }

    i = (FILE_SIZE / 2 / page_size as usize) as libc::c_int + 1;
    while i < vec_size && unsafe { *vec.add(i as usize) } != 0 {
        ra_pages += 1;
        i += 1;
    }
    expect_gt!(ra_pages, 0, {
        th_log!("No read-ahead pages found in memory");
    });

    /*
     * End of the readahead window. The rest of the pages shouldn't
     * be in memory.
     */
    if i < vec_size {
        while i < vec_size && unsafe { *vec.add(i as usize) } == 0 {
            i += 1;
        }
        expect_eq!(vec_size, i, {
            th_log!("Unexpected page in memory beyond readahead window");
        });
    }

    unsafe {
        libc::munmap(addr as *mut libc::c_void, FILE_SIZE as libc::size_t);
    }
out_close:
    unsafe {
        libc::close(fd);
    }
out_free:
    unsafe {
        libc::free(vec as *mut libc::c_void);
    }
});

/*
 * Test mincore() behavior on a page backed by a tmpfs file.  This test
 * performs the same steps as the previous one.
 */
test!(check_tmpfs_mmap, {
    let vec: *mut libc::c_uchar;
    let mut vec_size: libc::c_int;
    let addr: *mut libc::c_char;
    let mut retval: libc::c_int;
    let page_size: libc::c_int;
    let fd: libc::c_int;
    let mut i: libc::c_int;

    page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as libc::c_int };
    vec_size = (FILE_SIZE / page_size as usize) as libc::c_int;
    if FILE_SIZE % page_size as usize != 0 {
        vec_size += 1;
    }

    vec = unsafe { libc::calloc(vec_size as libc::size_t, core::mem::size_of::<libc::c_uchar>()) as *mut libc::c_uchar };
    assert_ne!(core::ptr::null_mut(), vec, {
        th_log!("Can't allocate array");
    });

    unsafe {
        errno = 0;
    }
    fd = unsafe { libc::open(c"/dev/shm".as_ptr(), libc::O_TMPFILE | libc::O_RDWR, 0o600) };
    assert_ne!(-1, fd, {
        th_log!("Can't create temporary file: %s", unsafe {
            libc::strerror(errno)
        });
    });
    unsafe {
        errno = 0;
    }
    retval = unsafe { libc::fallocate(fd, 0, 0, FILE_SIZE as libc::off_t) };
    assert_eq!(0, retval, {
        th_log!("Error allocating space for the temporary file: %s", unsafe {
            libc::strerror(errno)
        });
    });

    /*
     * Map the whole file, the pages shouldn't be fetched yet.
     */
    unsafe {
        errno = 0;
    }
    addr = unsafe {
        libc::mmap(
            core::ptr::null_mut(),
            FILE_SIZE as libc::size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        ) as *mut libc::c_char
    };
    assert_ne!(libc::MAP_FAILED as *mut libc::c_char, addr, {
        th_log!("mmap error: %s", unsafe { libc::strerror(errno) });
    });
    retval = unsafe { libc::mincore(addr as *mut libc::c_void, FILE_SIZE as libc::size_t, vec) };
    assert_eq!(0, retval);
    i = 0;
    while i < vec_size {
        assert_eq!(0, unsafe { *vec.add(i as usize) }, {
            th_log!("Unexpected page in memory");
        });
        i += 1;
    }

    /*
     * Touch a page in the middle of the mapping.
     */
    unsafe {
        *addr.add(FILE_SIZE / 2) = 1;
    }
    retval = unsafe { libc::mincore(addr as *mut libc::c_void, FILE_SIZE as libc::size_t, vec) };
    assert_eq!(0, retval);
    assert_eq!(1, unsafe { *vec.add(FILE_SIZE / 2 / page_size as usize) }, {
        th_log!("Page not found in memory after use");
    });

    unsafe {
        libc::munmap(addr as *mut libc::c_void, FILE_SIZE as libc::size_t);
        libc::close(fd);
        libc::free(vec as *mut libc::c_void);
    }
});

test_harness_main!();
