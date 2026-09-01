// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * thp_swap_allocator_test
 *
 * The purpose of this test program is helping check if THP swpout
 * can correctly get swap slots to swap out as a whole instead of
 * being split. It randomly releases swap entries through madvise
 * DONTNEED and swapin/out on two memory areas: a memory area for
 * 64KB THP and the other area for small folios. The second memory
 * can be enabled by "-s".
 * Before running the program, we need to setup a zRAM or similar
 * swap device by:
 *  echo lzo > /sys/block/zram0/comp_algorithm
 *  echo 64M > /sys/block/zram0/disksize
 *  echo never > /sys/kernel/mm/transparent_hugepage/hugepages-2048kB/enabled
 *  echo always > /sys/kernel/mm/transparent_hugepage/hugepages-64kB/enabled
 *  mkswap /dev/zram0
 *  swapon /dev/zram0
 * The expected result should be 0% anon swpout fallback ratio w/ or
 * w/o "-s".
 *
 * Author(s): Barry Song <v-songbaohua@oppo.com>
 */

#![allow(non_camel_case_types)]

use std::ffi::c_void;
use std::ptr;

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type size_t = usize;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

const MEMSIZE_MTHP: size_t = 60 * 1024 * 1024;
const MEMSIZE_SMALLFOLIO: size_t = 4 * 1024 * 1024;
const ALIGNMENT_MTHP: size_t = 64 * 1024;
const ALIGNMENT_SMALLFOLIO: size_t = 4 * 1024;
const TOTAL_DONTNEED_MTHP: size_t = 16 * 1024 * 1024;
const TOTAL_DONTNEED_SMALLFOLIO: size_t = 1 * 1024 * 1024;
const MTHP_FOLIO_SIZE: size_t = 64 * 1024;

const MADV_NORMAL: c_int = 0;
const MADV_RANDOM: c_int = 1;
const MADV_SEQUENTIAL: c_int = 2;
const MADV_WILLNEED: c_int = 3;
const MADV_DONTNEED: c_int = 4;
const MADV_FREE: c_int = 8;
const MADV_REMOVE: c_int = 9;
const MADV_DONTFORK: c_int = 10;
const MADV_DOFORK: c_int = 11;
const MADV_MERGEABLE: c_int = 12;
const MADV_UNMERGEABLE: c_int = 13;
const MADV_HUGEPAGE: c_int = 14;
const MADV_NOHUGEPAGE: c_int = 15;
const MADV_DONTDUMP: c_int = 16;
const MADV_DODUMP: c_int = 17;
const MADV_WIPEONFORK: c_int = 18;
const MADV_KEEPONFORK: c_int = 19;
const MADV_COLD: c_int = 20;
const MADV_PAGEOUT: c_int = 21;
const MADV_POPULATE_READ: c_int = 22;
const MADV_POPULATE_WRITE: c_int = 23;
const MADV_DONTNEED_LOCKED: c_int = 24;
const MADV_COLLAPSE: c_int = 25;
const MADV_HWPOISON: c_int = 100;
const MADV_SOFT_OFFLINE: c_int = 101;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

const SWPOUT_PATH: &[u8] =
    b"/sys/kernel/mm/transparent_hugepage/hugepages-64kB/stats/swpout\0";
const SWPOUT_FALLBACK_PATH: &[u8] =
    b"/sys/kernel/mm/transparent_hugepage/hugepages-64kB/stats/swpout_fallback\0";

unsafe extern "C" {
    fn posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
    fn perror(s: *const c_char);
    fn madvise(addr: *mut c_void, length: size_t, advice: c_int) -> c_int;
    fn rand() -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);

    static mut stderr: *mut FILE;
}

unsafe fn aligned_alloc_mem(size: size_t, alignment: size_t) -> *mut c_void {
    let mut mem: *mut c_void = ptr::null_mut();

    if posix_memalign(&mut mem, alignment, size) != 0 {
        perror(c"posix_memalign".as_ptr());
        return ptr::null_mut();
    }
    mem
}

/*
 * This emulates the behavior of native libc and Java heap,
 * as well as process exit and munmap. It helps generate mTHP
 * and ensures that iterations can proceed with mTHP, as we
 * currently don't support large folios swap-in.
 */
unsafe fn random_madvise_dontneed(
    mem: *mut c_void,
    mem_size: size_t,
    align_size: size_t,
    total_dontneed_size: size_t,
) {
    let num_pages: size_t = total_dontneed_size / align_size;
    let mut i: size_t;
    let mut offset: size_t;
    let mut addr: *mut c_void;

    i = 0;
    while i < num_pages {
        offset = (rand() as size_t % (mem_size / align_size)) * align_size;
        addr = (mem as *mut c_char).add(offset) as *mut c_void;
        if madvise(addr, align_size, MADV_DONTNEED) != 0 {
            perror(c"madvise dontneed".as_ptr());
        }

        memset(addr, 0x11, align_size);
        i += 1;
    }
}

unsafe fn random_swapin(
    mem: *mut c_void,
    mem_size: size_t,
    align_size: size_t,
    total_swapin_size: size_t,
) {
    let num_pages: size_t = total_swapin_size / align_size;
    let mut i: size_t;
    let mut offset: size_t;
    let mut addr: *mut c_void;

    i = 0;
    while i < num_pages {
        offset = (rand() as size_t % (mem_size / align_size)) * align_size;
        addr = (mem as *mut c_char).add(offset) as *mut c_void;
        memset(addr, 0x11, align_size);
        i += 1;
    }
}

unsafe fn read_stat(path: *const c_char) -> c_ulong {
    let mut file: *mut FILE;
    let mut value: c_ulong = 0;

    file = fopen(path, c"r".as_ptr());
    if file.is_null() {
        perror(c"fopen".as_ptr());
        return 0;
    }

    if fscanf(file, c"%lu".as_ptr(), &mut value as *mut c_ulong) != 1 {
        perror(c"fscanf".as_ptr());
        fclose(file);
        return 0;
    }

    fclose(file);
    value
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut use_small_folio: c_int = 0;
    let mut aligned_swapin: c_int = 0;
    let mut mem1: *mut c_void = ptr::null_mut();
    let mut mem2: *mut c_void = ptr::null_mut();
    let mut i: c_int;

    i = 1;
    while i < argc {
        if std::ffi::CStr::from_ptr(*argv.add(i as usize)) == c"-s" {
            use_small_folio = 1;
        } else if std::ffi::CStr::from_ptr(*argv.add(i as usize)) == c"-a" {
            aligned_swapin = 1;
        }
        i += 1;
    }

    mem1 = aligned_alloc_mem(MEMSIZE_MTHP, ALIGNMENT_MTHP);
    if mem1.is_null() {
        fprintf(
            stderr,
            c"Failed to allocate large folios memory\n".as_ptr(),
        );
        return EXIT_FAILURE;
    }

    if madvise(mem1, MEMSIZE_MTHP, MADV_HUGEPAGE) != 0 {
        perror(c"madvise hugepage for mem1".as_ptr());
        free(mem1);
        return EXIT_FAILURE;
    }

    if use_small_folio != 0 {
        mem2 = aligned_alloc_mem(MEMSIZE_SMALLFOLIO, ALIGNMENT_SMALLFOLIO);
        if mem2.is_null() {
            fprintf(
                stderr,
                c"Failed to allocate small folios memory\n".as_ptr(),
            );
            free(mem1);
            return EXIT_FAILURE;
        }

        if madvise(mem2, MEMSIZE_SMALLFOLIO, MADV_NOHUGEPAGE) != 0 {
            perror(c"madvise nohugepage for mem2".as_ptr());
            free(mem1);
            free(mem2);
            return EXIT_FAILURE;
        }
    }

    /* warm-up phase to occupy the swapfile */
    memset(mem1, 0x11, MEMSIZE_MTHP);
    madvise(mem1, MEMSIZE_MTHP, MADV_PAGEOUT);
    if use_small_folio != 0 {
        memset(mem2, 0x11, MEMSIZE_SMALLFOLIO);
        madvise(mem2, MEMSIZE_SMALLFOLIO, MADV_PAGEOUT);
    }

    /* iterations with newly created mTHP, swap-in, and swap-out */
    i = 0;
    while i < 100 {
        let initial_swpout: c_ulong;
        let initial_swpout_fallback: c_ulong;
        let final_swpout: c_ulong;
        let final_swpout_fallback: c_ulong;
        let swpout_inc: c_ulong;
        let swpout_fallback_inc: c_ulong;
        let fallback_percentage: f64;

        initial_swpout = read_stat(SWPOUT_PATH.as_ptr() as *const c_char);
        initial_swpout_fallback = read_stat(SWPOUT_FALLBACK_PATH.as_ptr() as *const c_char);

        /*
         * The following setup creates a 1:1 ratio of mTHP to small folios
         * since large folio swap-in isn't supported yet. Once we support
         * mTHP swap-in, we'll likely need to reduce MEMSIZE_MTHP and
         * increase MEMSIZE_SMALLFOLIO to maintain the ratio.
         */
        random_swapin(
            mem1,
            MEMSIZE_MTHP,
            if aligned_swapin != 0 {
                ALIGNMENT_MTHP
            } else {
                ALIGNMENT_SMALLFOLIO
            },
            TOTAL_DONTNEED_MTHP,
        );
        random_madvise_dontneed(mem1, MEMSIZE_MTHP, ALIGNMENT_MTHP, TOTAL_DONTNEED_MTHP);

        if use_small_folio != 0 {
            random_swapin(
                mem2,
                MEMSIZE_SMALLFOLIO,
                ALIGNMENT_SMALLFOLIO,
                TOTAL_DONTNEED_SMALLFOLIO,
            );
        }

        if madvise(mem1, MEMSIZE_MTHP, MADV_PAGEOUT) != 0 {
            perror(c"madvise pageout for mem1".as_ptr());
            free(mem1);
            if !mem2.is_null() {
                free(mem2);
            }
            return EXIT_FAILURE;
        }

        if use_small_folio != 0 {
            if madvise(mem2, MEMSIZE_SMALLFOLIO, MADV_PAGEOUT) != 0 {
                perror(c"madvise pageout for mem2".as_ptr());
                free(mem1);
                free(mem2);
                return EXIT_FAILURE;
            }
        }

        final_swpout = read_stat(SWPOUT_PATH.as_ptr() as *const c_char);
        final_swpout_fallback = read_stat(SWPOUT_FALLBACK_PATH.as_ptr() as *const c_char);

        swpout_inc = final_swpout.wrapping_sub(initial_swpout);
        swpout_fallback_inc = final_swpout_fallback.wrapping_sub(initial_swpout_fallback);

        fallback_percentage =
            swpout_fallback_inc as f64 / (swpout_fallback_inc + swpout_inc) as f64 * 100.0;

        printf(
            c"Iteration %d: swpout inc: %lu, swpout fallback inc: %lu, Fallback percentage: %.2f%%\n"
                .as_ptr(),
            i + 1,
            swpout_inc,
            swpout_fallback_inc,
            fallback_percentage,
        );
        i += 1;
    }

    free(mem1);
    if !mem2.is_null() {
        free(mem2);
    }

    EXIT_SUCCESS
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| {
            std::ffi::CString::new(arg)
                .expect("argument contains interior NUL")
                .into_raw()
        })
        .collect();
    let argc = args.len() as c_int;
    let ret = unsafe { c_main(argc, args.as_mut_ptr()) };

    for arg in args {
        unsafe {
            let _ = std::ffi::CString::from_raw(arg);
        }
    }

    std::process::exit(ret);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
