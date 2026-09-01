// SPDX-License-Identifier: GPL-2.0
/*
 * test-all.c: Try to build all the main testcases at once.
 *
 * A well-configured system will have all the prereqs installed, so we can speed
 * up auto-detection on such systems.
 */

use std::os::raw::{c_char, c_int};

/*
 * Quirk: Python headers cannot be in arbitrary places, so keep this testcase at
 * the top:
 */
/*
 * C source includes the following files after redefining main to the listed
 * symbols:
 *
 * test-libpython.c                  -> main_test_libpython
 * test-hello.c                      -> main_test_hello
 * test-libelf.c                     -> main_test_libelf
 * test-gettid.c                     -> main_test_gettid
 * test-glibc.c                      -> main_test_glibc
 * test-libdw.c                      -> main_test_libdw
 * test-eventfd.c                    -> main_test_eventfd
 * test-libelf-getphdrnum.c          -> main_test_libelf_getphdrnum
 * test-libelf-gelf_getnote.c        -> main_test_libelf_gelf_getnote
 * test-libelf-getshdrstrndx.c       -> main_test_libelf_getshdrstrndx
 * test-libelf-zstd.c                -> main_test_libelf_zstd
 * test-libslang.c                   -> main_test_libslang
 * test-backtrace.c                  -> main_test_backtrace
 * test-libnuma.c                    -> main_test_libnuma
 * test-numa_num_possible_cpus.c     -> main_test_numa_num_possible_cpus
 * test-timerfd.c                    -> main_test_timerfd
 * test-stackprotector-all.c         -> main_test_stackprotector_all
 * test-zlib.c                       -> main_test_zlib
 * test-pthread-attr-setaffinity-np.c -> main_test_pthread_attr_setaffinity_np
 * test-pthread-barrier.c            -> main_test_pthread_barrier
 * test-scandirat.c                  -> main_test_scandirat
 * test-sched_getcpu.c               -> main_test_sched_getcpu
 * test-lzma.c                       -> main_test_lzma
 * test-bpf.c                        -> main_test_bpf
 * test-sdt.c                        -> main_test_sdt
 * test-setns.c                      -> main_test_setns
 * test-libaio.c                     -> main_test_libaio
 * test-reallocarray.c               -> main_test_reallocarray
 * test-libzstd.c                    -> main_test_libzstd
 * test-libtraceevent.c              -> main_test_libtraceevent
 * test-libopenssl.c                 -> main_test_libopenssl
 */
unsafe extern "C" {
    fn main_test_libpython() -> c_int;
    fn main_test_hello() -> c_int;
    fn main_test_libelf() -> c_int;
    fn main_test_gettid() -> c_int;
    fn main_test_glibc() -> c_int;
    fn main_test_libdw() -> c_int;
    fn main_test_eventfd() -> c_int;
    fn main_test_libelf_getphdrnum() -> c_int;
    fn main_test_libelf_gelf_getnote() -> c_int;
    fn main_test_libelf_getshdrstrndx() -> c_int;
    fn main_test_libelf_zstd() -> c_int;
    fn main_test_libslang() -> c_int;
    fn main_test_backtrace() -> c_int;
    fn main_test_libnuma() -> c_int;
    fn main_test_numa_num_possible_cpus() -> c_int;
    fn main_test_timerfd() -> c_int;
    fn main_test_stackprotector_all() -> c_int;
    fn main_test_zlib() -> c_int;
    fn main_test_pthread_attr_setaffinity_np() -> c_int;
    fn main_test_pthread_barrier() -> c_int;
    fn main_test_scandirat() -> c_int;
    fn main_test_sched_getcpu() -> c_int;
    fn main_test_lzma() -> c_int;
    fn main_test_bpf() -> c_int;
    fn main_test_sdt() -> c_int;
    fn main_test_setns() -> c_int;
    fn main_test_libaio() -> c_int;
    fn main_test_reallocarray() -> c_int;
    fn main_test_libzstd() -> c_int;
    fn main_test_libtraceevent() -> c_int;
    fn main_test_libopenssl() -> c_int;
}

/*
 * Disable babeltrace2-ctf-writer check for test-all, because the requested
 * library version is not released yet in most distributions. Will
 * reenable later.
 *
 * Disabled C preprocessor block:
 * test-babeltrace2-ctf-writer.c -> main_test_babeltrace2_ctf_writer
 */

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    unsafe {
        main_test_libpython();
        main_test_hello();
        main_test_libelf();
        main_test_gettid();
        main_test_glibc();
        main_test_libdw();
        main_test_eventfd();
        main_test_libelf_getphdrnum();
        main_test_libelf_gelf_getnote();
        main_test_libelf_getshdrstrndx();
        main_test_libslang();
        main_test_backtrace();
        main_test_libnuma();
        main_test_numa_num_possible_cpus();
        main_test_timerfd();
        main_test_stackprotector_all();
        main_test_zlib();
        main_test_pthread_attr_setaffinity_np();
        main_test_pthread_barrier();
        main_test_lzma();
        main_test_bpf();
        main_test_scandirat();
        main_test_sched_getcpu();
        main_test_sdt();
        main_test_setns();
        main_test_libaio();
        main_test_reallocarray();
        main_test_libzstd();
        main_test_libtraceevent();
        main_test_libopenssl();
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
