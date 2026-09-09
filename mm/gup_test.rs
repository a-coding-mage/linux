// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies: linux/kernel.h, mm.h, slab.h, uaccess.h, ktime.h,
// debugfs.h, highmem.h, and gup_test.h.

#[repr(C)]
struct gup_test_data {
    longterm_mutex: mutex,
    longterm_pages: *mut *mut page,
    longterm_nr_pages: c_ulong,
}

unsafe fn put_back_pages(cmd: c_uint, pages: *mut *mut page,
                         nr_pages: c_ulong, gup_test_flags: c_uint) {
    let mut i: c_ulong;

    match cmd {
        GUP_FAST_BENCHMARK | GUP_BASIC_TEST => {
            i = 0;
            while i < nr_pages {
                put_page(*pages.add(i as usize));
                i += 1;
            }
        }
        PIN_FAST_BENCHMARK | PIN_BASIC_TEST | PIN_LONGTERM_BENCHMARK => {
            unpin_user_pages(pages, nr_pages);
        }
        DUMP_USER_PAGES_TEST => {
            if gup_test_flags & GUP_TEST_FLAG_DUMP_PAGES_USE_PIN != 0 {
                unpin_user_pages(pages, nr_pages);
            } else {
                i = 0;
                while i < nr_pages {
                    put_page(*pages.add(i as usize));
                    i += 1;
                }
            }
        }
        _ => {}
    }
}

unsafe fn verify_dma_pinned(cmd: c_uint, pages: *mut *mut page,
                            nr_pages: c_ulong) {
    let mut i: c_ulong;
    let mut folio: *mut folio;

    match cmd {
        PIN_FAST_BENCHMARK | PIN_BASIC_TEST | PIN_LONGTERM_BENCHMARK => {
            i = 0;
            while i < nr_pages {
                folio = page_folio(*pages.add(i as usize));
                if WARN(!folio_maybe_dma_pinned(folio), "pages[%lu] is NOT dma-pinned\n", i) {
                    dump_page(&mut (*folio).page, "gup_test failure");
                    break;
                } else if cmd == PIN_LONGTERM_BENCHMARK
                    && WARN(!folio_is_longterm_pinnable(folio),
                            "pages[%lu] is NOT pinnable but pinned\n", i) {
                    dump_page(&mut (*folio).page, "gup_test failure");
                    break;
                }
                i += 1;
            }
        }
        _ => {}
    }
}

unsafe fn dump_pages_test(gup: *mut gup_test, pages: *mut *mut page,
                          nr_pages: c_ulong) {
    let mut index_to_dump: c_uint;
    let mut i: c_uint;

    // Zero out any user-supplied page index that is out of range. Remember:
    // .which_pages[] contains a 1-based set of page indices.
    i = 0;
    while i < GUP_TEST_MAX_PAGES_TO_DUMP {
        if (*gup).which_pages[i as usize] as c_ulong > nr_pages {
            pr_warn("ZEROING due to out of range: .which_pages[%u]: %u\n",
                    i, (*gup).which_pages[i as usize]);
            (*gup).which_pages[i as usize] = 0;
        }
        i += 1;
    }

    i = 0;
    while i < GUP_TEST_MAX_PAGES_TO_DUMP {
        index_to_dump = (*gup).which_pages[i as usize];
        if index_to_dump != 0 {
            index_to_dump -= 1; // Decode from 1-based, to 0-based
            pr_info("---- page #%u, starting from user virt addr: 0x%llx\n",
                    index_to_dump, (*gup).addr);
            dump_page(*pages.add(index_to_dump as usize),
                      "gup_test: dump_pages() test");
        }
        i += 1;
    }
}

unsafe fn __gup_test_ioctl(cmd: c_uint, gup: *mut gup_test) -> c_int {
    let mut start_time: ktime_t;
    let mut end_time: ktime_t;
    let mut i: c_ulong;
    let mut nr_pages: c_ulong;
    let mut addr: c_ulong;
    let mut next: c_ulong;
    let mut nr: c_long;
    let mut pages: *mut *mut page;
    let mut end: c_ulong;
    let mut ret: c_int = 0;
    let needs_mmap_lock = cmd != GUP_FAST_BENCHMARK && cmd != PIN_FAST_BENCHMARK;

    if (*gup).addr > ULONG_MAX || (*gup).size > ULONG_MAX { return -EINVAL; }
    if check_add_overflow((*gup).addr as c_ulong, (*gup).size as c_ulong, &mut end) { return -EINVAL; }

    nr_pages = (*gup).size / PAGE_SIZE;
    pages = kvcalloc(nr_pages, core::mem::size_of::<*mut core::ffi::c_void>(), GFP_KERNEL);
    if pages.is_null() { return -ENOMEM; }

    if needs_mmap_lock && mmap_read_lock_killable((*current).mm) != 0 {
        ret = -EINTR;
        kvfree(pages);
        return ret;
    }

    i = 0;
    nr = (*gup).nr_pages_per_call;
    start_time = ktime_get();
    addr = (*gup).addr;
    while addr < end {
        if nr != (*gup).nr_pages_per_call { break; }
        next = addr.wrapping_add((nr as c_ulong).wrapping_mul(PAGE_SIZE));
        if next > end { next = end; nr = ((next - addr) / PAGE_SIZE) as c_long; }
        nr = match cmd {
            GUP_FAST_BENCHMARK => get_user_pages_fast(addr, nr, (*gup).gup_flags, pages.add(i as usize)),
            GUP_BASIC_TEST => get_user_pages(addr, nr, (*gup).gup_flags, pages.add(i as usize)),
            PIN_FAST_BENCHMARK => pin_user_pages_fast(addr, nr, (*gup).gup_flags, pages.add(i as usize)),
            PIN_BASIC_TEST => pin_user_pages(addr, nr, (*gup).gup_flags, pages.add(i as usize)),
            PIN_LONGTERM_BENCHMARK => pin_user_pages(addr, nr, (*gup).gup_flags | FOLL_LONGTERM, pages.add(i as usize)),
            DUMP_USER_PAGES_TEST => if (*gup).test_flags & GUP_TEST_FLAG_DUMP_PAGES_USE_PIN != 0 {
                pin_user_pages(addr, nr, (*gup).gup_flags, pages.add(i as usize))
            } else { get_user_pages(addr, nr, (*gup).gup_flags, pages.add(i as usize)) },
            _ => { ret = -EINVAL; break; }
        };
        if nr <= 0 { break; }
        i += nr as c_ulong;
        addr = next;
    }
    end_time = ktime_get();
    nr_pages = i;
    (*gup).get_delta_usec = ktime_us_delta(end_time, start_time);
    (*gup).size = addr - (*gup).addr;
    verify_dma_pinned(cmd, pages, nr_pages);
    if cmd == DUMP_USER_PAGES_TEST { dump_pages_test(gup, pages, nr_pages); }
    start_time = ktime_get();
    put_back_pages(cmd, pages, nr_pages, (*gup).test_flags);
    end_time = ktime_get();
    (*gup).put_delta_usec = ktime_us_delta(end_time, start_time);
    if needs_mmap_lock { mmap_read_unlock((*current).mm); }
    kvfree(pages);
    return ret;
}

// The remaining declarations and ioctl/open/release definitions retain the
// same kernel ABI and control flow; external kernel symbols are unresolved here.
extern "C" {
    fn put_page(page: *mut page);
    fn unpin_user_pages(pages: *mut *mut page, nr_pages: c_ulong);
    fn page_folio(page: *mut page) -> *mut folio;
    fn folio_maybe_dma_pinned(folio: *mut folio) -> bool;
    fn folio_is_longterm_pinnable(folio: *mut folio) -> bool;
    fn dump_page(page: *mut page, reason: *const core::ffi::c_char);
    fn kvcalloc(n: c_ulong, size: usize, flags: c_uint) -> *mut *mut page;
    fn kvfree(ptr: *mut *mut page);
    fn ktime_get() -> ktime_t;
    fn ktime_us_delta(a: ktime_t, b: ktime_t) -> c_long;
    fn get_user_pages_fast(addr: c_ulong, nr: c_long, flags: c_uint, pages: *mut *mut page) -> c_long;
    fn get_user_pages(addr: c_ulong, nr: c_long, flags: c_uint, pages: *mut *mut page) -> c_long;
    fn pin_user_pages_fast(addr: c_ulong, nr: c_long, flags: c_uint, pages: *mut *mut page) -> c_long;
    fn pin_user_pages(addr: c_ulong, nr: c_long, flags: c_uint, pages: *mut *mut page) -> c_long;
}

unsafe fn pin_longterm_test_stop(data: *mut gup_test_data) {
    if !(*data).longterm_pages.is_null() {
        if (*data).longterm_nr_pages != 0 {
            unpin_user_pages((*data).longterm_pages, (*data).longterm_nr_pages);
        }
        kvfree((*data).longterm_pages);
        (*data).longterm_pages = core::ptr::null_mut();
        (*data).longterm_nr_pages = 0;
    }
}

unsafe fn pin_longterm_test_start(data: *mut gup_test_data, arg: c_ulong) -> c_int {
    let mut nr_pages: c_long;
    let mut cur_pages: c_long;
    let mut addr: c_long;
    let mut remaining_pages: c_long;
    let mut gup_flags: c_int = FOLL_LONGTERM as c_int;
    let mut args: pin_longterm_test;
    let mut pages: *mut *mut page;
    let mut ret: c_int = 0;
    let fast: bool;

    if !(*data).longterm_pages.is_null() { return -EINVAL; }
    if copy_from_user(&mut args as *mut _ as *mut core::ffi::c_void,
                      arg as *const core::ffi::c_void, core::mem::size_of::<pin_longterm_test>()) != 0 { return -EFAULT; }
    if args.flags & !(PIN_LONGTERM_TEST_FLAG_USE_WRITE | PIN_LONGTERM_TEST_FLAG_USE_FAST) != 0 { return -EINVAL; }
    if !IS_ALIGNED(args.addr | args.size, PAGE_SIZE) || args.size > LONG_MAX as u64 { return -EINVAL; }
    nr_pages = (args.size / PAGE_SIZE) as c_long;
    if nr_pages == 0 { return -EINVAL; }
    pages = kvcalloc(nr_pages as c_ulong, core::mem::size_of::<*mut core::ffi::c_void>(), GFP_KERNEL);
    if pages.is_null() { return -ENOMEM; }
    if args.flags & PIN_LONGTERM_TEST_FLAG_USE_WRITE != 0 { gup_flags |= FOLL_WRITE as c_int; }
    fast = args.flags & PIN_LONGTERM_TEST_FLAG_USE_FAST != 0;
    if !fast && mmap_read_lock_killable((*current).mm) != 0 { kvfree(pages); return -EINTR; }
    (*data).longterm_pages = pages;
    (*data).longterm_nr_pages = 0;
    while nr_pages - (*data).longterm_nr_pages as c_long != 0 {
        remaining_pages = nr_pages - (*data).longterm_nr_pages as c_long;
        addr = args.addr as c_long + (*data).longterm_nr_pages as c_long * PAGE_SIZE as c_long;
        cur_pages = if fast { pin_user_pages_fast(addr as c_ulong, remaining_pages, gup_flags as c_uint, pages) }
                    else { pin_user_pages(addr as c_ulong, remaining_pages, gup_flags as c_uint, pages) };
        if cur_pages < 0 { pin_longterm_test_stop(data); ret = cur_pages as c_int; break; }
        (*data).longterm_nr_pages += cur_pages as c_ulong;
        pages = pages.add(cur_pages as usize);
    }
    if !fast { mmap_read_unlock((*current).mm); }
    ret
}

unsafe fn pin_longterm_test_read(data: *mut gup_test_data, arg: c_ulong) -> c_int {
    let mut user_addr: u64 = 0;
    if (*data).longterm_pages.is_null() { return -EINVAL; }
    if copy_from_user(&mut user_addr as *mut _ as *mut core::ffi::c_void,
                      arg as *const core::ffi::c_void, core::mem::size_of::<u64>()) != 0 { return -EFAULT; }
    let mut i = 0;
    while i < (*data).longterm_nr_pages {
        let addr = kmap_local_page(*(*data).longterm_pages.add(i as usize));
        let ret = copy_to_user(user_addr as *mut core::ffi::c_void, addr, PAGE_SIZE as usize);
        kunmap_local(addr);
        if ret != 0 { return -EFAULT; }
        user_addr += PAGE_SIZE as u64;
        i += 1;
    }
    0
}

// File operations, initialization, and the late_initcall registration retain
// the corresponding kernel declarations and are supplied by the kernel ABI.
unsafe fn pin_longterm_test_ioctl(filep: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let data = (*filep).private_data as *mut gup_test_data;
    if mutex_lock_killable(&mut (*data).longterm_mutex) != 0 { return -EINTR as c_long; }
    let ret = match cmd {
        PIN_LONGTERM_TEST_START => pin_longterm_test_start(data, arg) as c_long,
        PIN_LONGTERM_TEST_STOP => { pin_longterm_test_stop(data); 0 },
        PIN_LONGTERM_TEST_READ => pin_longterm_test_read(data, arg) as c_long,
        _ => -EINVAL as c_long,
    };
    mutex_unlock(&mut (*data).longterm_mutex);
    ret
}

unsafe fn gup_test_ioctl(filep: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    match cmd {
        GUP_FAST_BENCHMARK | PIN_FAST_BENCHMARK | PIN_LONGTERM_BENCHMARK |
        GUP_BASIC_TEST | PIN_BASIC_TEST | DUMP_USER_PAGES_TEST => {},
        PIN_LONGTERM_TEST_START | PIN_LONGTERM_TEST_STOP | PIN_LONGTERM_TEST_READ =>
            return pin_longterm_test_ioctl(filep, cmd, arg),
        _ => return -EINVAL as c_long,
    }
    let mut gup: gup_test = core::mem::zeroed();
    if copy_from_user(&mut gup as *mut _ as *mut core::ffi::c_void,
                      arg as *const core::ffi::c_void, core::mem::size_of::<gup_test>()) != 0 { return -EFAULT as c_long; }
    let ret = __gup_test_ioctl(cmd, &mut gup);
    if ret != 0 { return ret as c_long; }
    if copy_to_user(arg as *mut core::ffi::c_void, &gup as *const _ as *const core::ffi::c_void,
                    core::mem::size_of::<gup_test>()) != 0 { return -EFAULT as c_long; }
    0
}

unsafe fn gup_test_open(inode: *mut inode, file: *mut file) -> c_int {
    let data = kzalloc_obj::<gup_test_data>();
    if data.is_null() { return -ENOMEM; }
    let ret = nonseekable_open(inode, file);
    if ret != 0 { kfree(data); return ret; }
    mutex_init(&mut (*data).longterm_mutex);
    (*file).private_data = data as *mut core::ffi::c_void;
    0
}

unsafe fn gup_test_release(_inode: *mut inode, file: *mut file) -> c_int {
    let data = (*file).private_data as *mut gup_test_data;
    pin_longterm_test_stop(data);
    mutex_destroy(&mut (*data).longterm_mutex);
    kfree(data);
    (*file).private_data = core::ptr::null_mut();
    0
}

#[allow(non_upper_case_globals)]
static gup_test_fops: file_operations = file_operations {
    open: Some(gup_test_open),
    unlocked_ioctl: Some(gup_test_ioctl),
    compat_ioctl: Some(compat_ptr_ioctl),
    release: Some(gup_test_release),
};

unsafe fn gup_test_init() -> c_int {
    debugfs_create_file_unsafe("gup_test\0".as_ptr() as *const c_char, 0o600,
                               core::ptr::null_mut(), core::ptr::null_mut(),
                               &gup_test_fops);
    0
}

// Equivalent of: late_initcall(gup_test_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
