// SPDX-License-Identifier: GPL-2.0-only
/*
 * page-types: Tool for querying page flags
 *
 * Copyright (C) 2009 Intel corporation
 *
 * Authors: Wu Fengguang <fengguang.wu@intel.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr;

type uint64_t = u64;
type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type time_t = c_long;

const MAX_PATH: usize = 256;

/*
 * pagemap kernel ABI bits
 */

const PM_ENTRY_BYTES: uint64_t = 8;
const PM_PFRAME_BITS: uint64_t = 55;
const PM_PFRAME_MASK: uint64_t = (1u64 << PM_PFRAME_BITS) - 1;
const MAX_SWAPFILES_SHIFT: uint64_t = 5;
const PM_SOFT_DIRTY: uint64_t = 1u64 << 55;
const PM_MMAP_EXCLUSIVE: uint64_t = 1u64 << 56;
const PM_FILE: uint64_t = 1u64 << 61;
const PM_SWAP: uint64_t = 1u64 << 62;
const PM_PRESENT: uint64_t = 1u64 << 63;

fn PM_PFRAME(x: uint64_t) -> uint64_t {
    x & PM_PFRAME_MASK
}

fn PM_SWAP_OFFSET(x: uint64_t) -> uint64_t {
    (x & PM_PFRAME_MASK) >> MAX_SWAPFILES_SHIFT
}

/*
 * kernel page flags
 */

const KPF_BYTES: uint64_t = 8;
const PROC_KPAGEFLAGS: &[u8] = b"/proc/kpageflags\0";
const PROC_KPAGECOUNT: &[u8] = b"/proc/kpagecount\0";
const PROC_KPAGECGROUP: &[u8] = b"/proc/kpagecgroup\0";

const SYS_KERNEL_MM_PAGE_IDLE: &[u8] = b"/sys/kernel/mm/page_idle/bitmap\0";

/* from include/uapi/linux/kernel-page-flags.h */
const KPF_LOCKED: usize = 0;
const KPF_ERROR: usize = 1;
const KPF_REFERENCED: usize = 2;
const KPF_UPTODATE: usize = 3;
const KPF_DIRTY: usize = 4;
const KPF_LRU: usize = 5;
const KPF_ACTIVE: usize = 6;
const KPF_SLAB: usize = 7;
const KPF_WRITEBACK: usize = 8;
const KPF_RECLAIM: usize = 9;
const KPF_BUDDY: usize = 10;
const KPF_MMAP: usize = 11;
const KPF_ANON: usize = 12;
const KPF_SWAPCACHE: usize = 13;
const KPF_SWAPBACKED: usize = 14;
const KPF_COMPOUND_HEAD: usize = 15;
const KPF_COMPOUND_TAIL: usize = 16;
const KPF_HUGE: usize = 17;
const KPF_UNEVICTABLE: usize = 18;
const KPF_HWPOISON: usize = 19;
const KPF_NOPAGE: usize = 20;
const KPF_KSM: usize = 21;
const KPF_THP: usize = 22;
const KPF_OFFLINE: usize = 23;
const KPF_ZERO_PAGE: usize = 24;
const KPF_IDLE: usize = 25;
const KPF_PGTABLE: usize = 26;

/* [32-] kernel hacking assistances */
const KPF_RESERVED: usize = 32;
const KPF_MLOCKED: usize = 33;
const KPF_OWNER_2: usize = 34;
const KPF_PRIVATE: usize = 35;
const KPF_PRIVATE_2: usize = 36;
const KPF_OWNER_PRIVATE: usize = 37;
const KPF_ARCH: usize = 38;
const KPF_UNCACHED: usize = 39; /* unused */
const KPF_SOFTDIRTY: usize = 40;
const KPF_ARCH_2: usize = 41;

/* [47-] take some arbitrary free slots for expanding overloaded flags
 * not part of kernel API
 */
const KPF_ANON_EXCLUSIVE: usize = 47;
const KPF_READAHEAD: usize = 48;
const KPF_SLUB_FROZEN: usize = 50;
const KPF_SLUB_DEBUG: usize = 51;
const KPF_FILE: usize = 61;
const KPF_SWAP: usize = 62;
const KPF_MMAP_EXCLUSIVE: usize = 63;

const KPF_ALL_BITS: uint64_t = !0u64;
const KPF_HACKERS_BITS: uint64_t = 0xffffu64 << 32;
const KPF_OVERLOADED_BITS: uint64_t = 0xffffu64 << 48;
fn BIT(bit: usize) -> uint64_t {
    1u64 << bit
}
fn BITS_COMPOUND() -> uint64_t {
    BIT(KPF_COMPOUND_HEAD) | BIT(KPF_COMPOUND_TAIL)
}

static page_flag_names: [Option<&'static str>; 64] = {
    let mut a = [None; 64];
    a[KPF_LOCKED] = Some("L:locked");
    a[KPF_ERROR] = Some("E:error");
    a[KPF_REFERENCED] = Some("R:referenced");
    a[KPF_UPTODATE] = Some("U:uptodate");
    a[KPF_DIRTY] = Some("D:dirty");
    a[KPF_LRU] = Some("l:lru");
    a[KPF_ACTIVE] = Some("A:active");
    a[KPF_SLAB] = Some("S:slab");
    a[KPF_WRITEBACK] = Some("W:writeback");
    a[KPF_RECLAIM] = Some("I:reclaim");
    a[KPF_BUDDY] = Some("B:buddy");
    a[KPF_MMAP] = Some("M:mmap");
    a[KPF_ANON] = Some("a:anonymous");
    a[KPF_SWAPCACHE] = Some("s:swapcache");
    a[KPF_SWAPBACKED] = Some("b:swapbacked");
    a[KPF_COMPOUND_HEAD] = Some("H:compound_head");
    a[KPF_COMPOUND_TAIL] = Some("T:compound_tail");
    a[KPF_HUGE] = Some("G:huge");
    a[KPF_UNEVICTABLE] = Some("u:unevictable");
    a[KPF_HWPOISON] = Some("X:hwpoison");
    a[KPF_NOPAGE] = Some("n:nopage");
    a[KPF_KSM] = Some("x:ksm");
    a[KPF_THP] = Some("t:thp");
    a[KPF_OFFLINE] = Some("o:offline");
    a[KPF_PGTABLE] = Some("g:pgtable");
    a[KPF_ZERO_PAGE] = Some("z:zero_page");
    a[KPF_IDLE] = Some("i:idle_page");
    a[KPF_RESERVED] = Some("r:reserved");
    a[KPF_MLOCKED] = Some("m:mlocked");
    a[KPF_OWNER_2] = Some("d:owner_2");
    a[KPF_PRIVATE] = Some("P:private");
    a[KPF_PRIVATE_2] = Some("p:private_2");
    a[KPF_OWNER_PRIVATE] = Some("O:owner_private");
    a[KPF_ARCH] = Some("h:arch");
    a[KPF_SOFTDIRTY] = Some("f:softdirty");
    a[KPF_ARCH_2] = Some("H:arch_2");
    a[KPF_ANON_EXCLUSIVE] = Some("d:anon_exclusive");
    a[KPF_READAHEAD] = Some("I:readahead");
    a[KPF_SLUB_FROZEN] = Some("A:slub_frozen");
    a[KPF_SLUB_DEBUG] = Some("E:slub_debug");
    a[KPF_FILE] = Some("F:file");
    a[KPF_SWAP] = Some("w:swap");
    a[KPF_MMAP_EXCLUSIVE] = Some("1:mmap_exclusive");
    a
};

/*
 * data structures
 */

static mut opt_raw: c_int = 0; /* for kernel developers */
static mut opt_list: c_int = 0; /* list pages (in ranges) */
static mut opt_mark_idle: c_int = 0; /* set accessed bit */
static mut opt_no_summary: c_int = 0; /* don't show summary */
static mut opt_pid: pid_t = 0; /* process to walk */
static mut opt_file: *const c_char = ptr::null(); /* file or directory path */
static mut opt_cgroup: uint64_t = 0; /* cgroup inode */
static mut opt_list_cgroup: c_int = 0; /* list page cgroup */
static mut opt_list_mapcnt: c_int = 0; /* list page map count */
static mut opt_kpageflags: *const c_char = ptr::null(); /* kpageflags file to parse */

const MAX_ADDR_RANGES: usize = 1024;
static mut nr_addr_ranges: c_int = 0;
static mut opt_offset: [c_ulong; MAX_ADDR_RANGES] = [0; MAX_ADDR_RANGES];
static mut opt_size: [c_ulong; MAX_ADDR_RANGES] = [0; MAX_ADDR_RANGES];

const MAX_VMAS: usize = 10240;
static mut nr_vmas: c_int = 0;
static mut pg_start: [c_ulong; MAX_VMAS] = [0; MAX_VMAS];
static mut pg_end: [c_ulong; MAX_VMAS] = [0; MAX_VMAS];

const MAX_BIT_FILTERS: usize = 64;
static mut nr_bit_filters: c_int = 0;
static mut opt_mask: [uint64_t; MAX_BIT_FILTERS] = [0; MAX_BIT_FILTERS];
static mut opt_bits: [uint64_t; MAX_BIT_FILTERS] = [0; MAX_BIT_FILTERS];

static mut page_size: c_int = 0;

static mut pagemap_fd: c_int = 0;
static mut kpageflags_fd: c_int = 0;
static mut kpagecount_fd: c_int = -1;
static mut kpagecgroup_fd: c_int = -1;
static mut page_idle_fd: c_int = -1;

static mut opt_hwpoison: c_int = 0;
static mut opt_unpoison: c_int = 0;

static mut hwpoison_debug_fs: *const c_char = ptr::null();
static mut hwpoison_inject_fd: c_int = 0;
static mut hwpoison_forget_fd: c_int = 0;

const HASH_SHIFT: usize = 13;
const HASH_SIZE: usize = 1 << HASH_SHIFT;
const HASH_MASK: uint64_t = (HASH_SIZE as uint64_t) - 1;
fn HASH_KEY(flags: uint64_t) -> usize {
    (flags & HASH_MASK) as usize
}

static mut total_pages: c_ulong = 0;
static mut nr_pages: [c_ulong; HASH_SIZE] = [0; HASH_SIZE];
static mut page_flags: [uint64_t; HASH_SIZE] = [0; HASH_SIZE];

/*
 * helper functions
 */

fn pages2mb(pages: c_ulong) -> c_ulong {
    unsafe { (pages * page_size as c_ulong) >> 20 }
}

unsafe fn fatal(msg: &str) -> ! {
    eprint!("{}", msg);
    libc::exit(libc::EXIT_FAILURE);
}

unsafe fn fatal_c(fmt: &str, s: *const c_char) -> ! {
    let text = if s.is_null() {
        "(null)".to_string()
    } else {
        CStr::from_ptr(s).to_string_lossy().into_owned()
    };
    fatal(&fmt.replace("%s", &text))
}

unsafe fn checked_open(pathname: *const c_char, flags: c_int) -> c_int {
    let fd = libc::open(pathname, flags);
    if fd < 0 {
        libc::perror(pathname);
        libc::exit(libc::EXIT_FAILURE);
    }
    fd
}

/*
 * pagemap/kpageflags routines
 */

unsafe fn do_u64_read(
    fd: c_int,
    name: *const c_char,
    buf: *mut uint64_t,
    index: c_ulong,
    count: c_ulong,
) -> c_ulong {
    let bytes: c_long;

    if index > c_ulong::MAX / 8 {
        fatal(&format!("index overflow: {}\n", index));
    }

    bytes = libc::pread(
        fd,
        buf as *mut c_void,
        (count * 8) as usize,
        (index * 8) as off_t,
    ) as c_long;
    if bytes < 0 {
        libc::perror(name);
        libc::exit(libc::EXIT_FAILURE);
    }
    if bytes % 8 != 0 {
        fatal(&format!("partial read: {} bytes\n", bytes));
    }

    (bytes / 8) as c_ulong
}

unsafe fn kpageflags_read(buf: *mut uint64_t, index: c_ulong, pages: c_ulong) -> c_ulong {
    do_u64_read(kpageflags_fd, opt_kpageflags, buf, index, pages)
}

unsafe fn kpagecgroup_read(buf: *mut uint64_t, index: c_ulong, pages: c_ulong) -> c_ulong {
    if kpagecgroup_fd < 0 {
        return pages;
    }
    do_u64_read(kpagecgroup_fd, opt_kpageflags, buf, index, pages)
}

unsafe fn kpagecount_read(buf: *mut uint64_t, index: c_ulong, pages: c_ulong) -> c_ulong {
    if kpagecount_fd < 0 {
        pages
    } else {
        do_u64_read(kpagecount_fd, PROC_KPAGECOUNT.as_ptr() as *const c_char, buf, index, pages)
    }
}

unsafe fn pagemap_read(buf: *mut uint64_t, index: c_ulong, pages: c_ulong) -> c_ulong {
    do_u64_read(pagemap_fd, b"/proc/pid/pagemap\0".as_ptr() as *const c_char, buf, index, pages)
}

fn pagemap_pfn(val: uint64_t) -> c_ulong {
    if val & PM_PRESENT != 0 {
        PM_PFRAME(val) as c_ulong
    } else {
        0
    }
}

fn pagemap_swap_offset(val: uint64_t) -> c_ulong {
    if val & PM_SWAP != 0 {
        PM_SWAP_OFFSET(val) as c_ulong
    } else {
        0
    }
}

/*
 * page flag names
 */

unsafe fn page_flag_name(flags: uint64_t) -> *const c_char {
    static mut BUF: [u8; 65] = [0; 65];
    let mut j: usize = 0;

    for i in 0..page_flag_names.len() {
        let present = ((flags >> i) & 1) != 0;
        if page_flag_names[i].is_none() {
            if present {
                fatal(&format!("unknown flag bit {}\n", i));
            }
            continue;
        }
        let name = page_flag_names[i].unwrap().as_bytes();
        BUF[j] = if present { name[0] } else { b'_' };
        j += 1;
    }
    BUF[j] = 0;
    BUF.as_ptr() as *const c_char
}

unsafe fn page_flag_longname(flags: uint64_t) -> *const c_char {
    static mut BUF: [u8; 1024] = [0; 1024];
    let mut out = String::new();

    for i in 0..page_flag_names.len() {
        if let Some(name) = page_flag_names[i] {
            if (flags >> i) & 1 != 0 {
                out.push_str(&name[2..]);
                out.push(',');
            }
        }
    }
    if out.ends_with(',') {
        out.pop();
    }
    let bytes = out.as_bytes();
    let n = bytes.len().min(BUF.len() - 1);
    ptr::copy_nonoverlapping(bytes.as_ptr(), BUF.as_mut_ptr(), n);
    BUF[n] = 0;
    BUF.as_ptr() as *const c_char
}

unsafe fn cstr(p: *const c_char) -> String {
    CStr::from_ptr(p).to_string_lossy().into_owned()
}

/*
 * page list and summary
 */

unsafe fn show_page_range(
    voffset: c_ulong,
    offset: c_ulong,
    size: c_ulong,
    flags: uint64_t,
    cgroup: uint64_t,
    mapcnt: uint64_t,
) {
    static mut FLAGS0: uint64_t = 0;
    static mut CGROUP0: uint64_t = 0;
    static mut MAPCNT0: uint64_t = 0;
    static mut VOFF: c_ulong = 0;
    static mut INDEX: c_ulong = 0;
    static mut COUNT: c_ulong = 0;

    if flags == FLAGS0
        && cgroup == CGROUP0
        && mapcnt == MAPCNT0
        && offset == INDEX + COUNT
        && size != 0
        && voffset == VOFF + COUNT
    {
        COUNT += size;
        return;
    }

    if COUNT != 0 {
        if opt_pid != 0 {
            print!("{:x}\t", VOFF);
        }
        if !opt_file.is_null() {
            print!("{:x}\t", VOFF);
        }
        if opt_list_cgroup != 0 {
            print!("@{}\t", CGROUP0);
        }
        if opt_list_mapcnt != 0 {
            print!("{}\t", MAPCNT0);
        }
        println!("{:x}\t{:x}\t{}", INDEX, COUNT, cstr(page_flag_name(FLAGS0)));
    }

    FLAGS0 = flags;
    CGROUP0 = cgroup;
    MAPCNT0 = mapcnt;
    INDEX = offset;
    VOFF = voffset;
    COUNT = size;
}

unsafe fn flush_page_range() {
    show_page_range(0, 0, 0, 0, 0, 0);
}

unsafe fn show_page(voffset: c_ulong, offset: c_ulong, flags: uint64_t, cgroup: uint64_t, mapcnt: uint64_t) {
    if opt_pid != 0 {
        print!("{:x}\t", voffset);
    }
    if !opt_file.is_null() {
        print!("{:x}\t", voffset);
    }
    if opt_list_cgroup != 0 {
        print!("@{}\t", cgroup);
    }
    if opt_list_mapcnt != 0 {
        print!("{}\t", mapcnt);
    }

    println!("{:x}\t{}", offset, cstr(page_flag_name(flags)));
}

unsafe fn show_summary() {
    println!("             flags\tpage-count       MB  symbolic-flags\t\t\tlong-symbolic-flags");

    for i in 0..nr_pages.len() {
        if nr_pages[i] != 0 {
            println!(
                "0x{:016x}\t{:10} {:8}  {}\t{}",
                page_flags[i],
                nr_pages[i],
                pages2mb(nr_pages[i]),
                cstr(page_flag_name(page_flags[i])),
                cstr(page_flag_longname(page_flags[i]))
            );
        }
    }

    println!("             total\t{:10} {:8}", total_pages, pages2mb(total_pages));
}

/*
 * page flag filters
 */

unsafe fn bit_mask_ok(flags: uint64_t) -> c_int {
    for i in 0..nr_bit_filters as usize {
        if opt_bits[i] == KPF_ALL_BITS {
            if (flags & opt_mask[i]) == 0 {
                return 0;
            }
        } else if (flags & opt_mask[i]) != opt_bits[i] {
            return 0;
        }
    }
    1
}

unsafe fn expand_overloaded_flags(mut flags: uint64_t, pme: uint64_t) -> uint64_t {
    /* Anonymous pages use PG_owner_2 for anon_exclusive */
    if (flags & BIT(KPF_ANON) != 0) && (flags & BIT(KPF_OWNER_2) != 0) {
        flags ^= BIT(KPF_OWNER_2) | BIT(KPF_ANON_EXCLUSIVE);
    }

    /* SLUB overloads several page flags */
    if flags & BIT(KPF_SLAB) != 0 {
        if flags & BIT(KPF_ACTIVE) != 0 {
            flags ^= BIT(KPF_ACTIVE) | BIT(KPF_SLUB_FROZEN);
        }
        if flags & BIT(KPF_ERROR) != 0 {
            flags ^= BIT(KPF_ERROR) | BIT(KPF_SLUB_DEBUG);
        }
    }

    /* PG_reclaim is overloaded as PG_readahead in the read path */
    if (flags & (BIT(KPF_RECLAIM) | BIT(KPF_WRITEBACK))) == BIT(KPF_RECLAIM) {
        flags ^= BIT(KPF_RECLAIM) | BIT(KPF_READAHEAD);
    }

    if pme & PM_SOFT_DIRTY != 0 {
        flags |= BIT(KPF_SOFTDIRTY);
    }
    if pme & PM_FILE != 0 {
        flags |= BIT(KPF_FILE);
    }
    if pme & PM_SWAP != 0 {
        flags |= BIT(KPF_SWAP);
    }
    if pme & PM_MMAP_EXCLUSIVE != 0 {
        flags |= BIT(KPF_MMAP_EXCLUSIVE);
    }

    flags
}

fn well_known_flags(mut flags: uint64_t) -> uint64_t {
    /* hide flags intended only for kernel hacker */
    flags &= !KPF_HACKERS_BITS;

    /* hide non-hugeTLB compound pages */
    if (flags & BITS_COMPOUND() != 0) && (flags & BIT(KPF_HUGE) == 0) {
        flags &= !BITS_COMPOUND();
    }
    flags
}

unsafe fn kpageflags_flags(flags: uint64_t, pme: uint64_t) -> uint64_t {
    if opt_raw != 0 {
        expand_overloaded_flags(flags, pme)
    } else {
        well_known_flags(flags)
    }
}

/*
 * page actions
 */

extern "C" {
    fn debugfs__mount() -> *const c_char;
}

unsafe fn prepare_hwpoison_fd() {
    let mut buf = [0i8; MAX_PATH + 1];

    hwpoison_debug_fs = debugfs__mount();
    if hwpoison_debug_fs.is_null() {
        libc::perror(b"mount debugfs\0".as_ptr() as *const c_char);
        libc::exit(libc::EXIT_FAILURE);
    }

    if opt_hwpoison != 0 && hwpoison_inject_fd == 0 {
        libc::snprintf(
            buf.as_mut_ptr(),
            MAX_PATH,
            b"%s/hwpoison/corrupt-pfn\0".as_ptr() as *const c_char,
            hwpoison_debug_fs,
        );
        hwpoison_inject_fd = checked_open(buf.as_ptr(), libc::O_WRONLY);
    }

    if opt_unpoison != 0 && hwpoison_forget_fd == 0 {
        libc::snprintf(
            buf.as_mut_ptr(),
            MAX_PATH,
            b"%s/hwpoison/unpoison-pfn\0".as_ptr() as *const c_char,
            hwpoison_debug_fs,
        );
        hwpoison_forget_fd = checked_open(buf.as_ptr(), libc::O_WRONLY);
    }
}

unsafe fn hwpoison_page(offset: c_ulong) -> c_int {
    let s = format!("0x{:x}\n", offset);
    let len = libc::write(hwpoison_inject_fd, s.as_ptr() as *const c_void, s.len()) as c_int;
    if len < 0 {
        libc::perror(b"hwpoison inject\0".as_ptr() as *const c_char);
        return len;
    }
    0
}

unsafe fn unpoison_page(offset: c_ulong) -> c_int {
    let s = format!("0x{:x}\n", offset);
    let len = libc::write(hwpoison_forget_fd, s.as_ptr() as *const c_void, s.len()) as c_int;
    if len < 0 {
        libc::perror(b"hwpoison forget\0".as_ptr() as *const c_char);
        return len;
    }
    0
}

unsafe fn mark_page_idle(offset: c_ulong) -> c_int {
    static mut OFF: c_ulong = 0;
    static mut BUF: uint64_t = 0;

    if (offset / 64 == OFF / 64) || BUF == 0 {
        BUF |= 1u64 << (offset % 64);
        OFF = offset;
        return 0;
    }

    let len = libc::pwrite(
        page_idle_fd,
        &raw const BUF as *const c_void,
        8,
        (8 * (OFF / 64)) as off_t,
    ) as c_int;
    if len < 0 {
        libc::perror(b"mark page idle\0".as_ptr() as *const c_char);
        return len;
    }

    BUF = 1u64 << (offset % 64);
    OFF = offset;
    0
}

/*
 * page frame walker
 */

unsafe fn hash_slot(flags: uint64_t) -> size_t {
    let mut k = HASH_KEY(flags);

    /* Explicitly reserve slot 0 for flags 0: the following logic
     * cannot distinguish an unoccupied slot from slot (flags==0).
     */
    if flags == 0 {
        return 0;
    }

    /* search through the remaining (HASH_SIZE-1) slots */
    for _i in 1..page_flags.len() {
        if k == 0 || k >= page_flags.len() {
            k = 1;
        }
        if page_flags[k] == 0 {
            page_flags[k] = flags;
            return k;
        }
        if page_flags[k] == flags {
            return k;
        }
        k += 1;
    }

    fatal("hash table full: bump up HASH_SHIFT?\n");
}

unsafe fn add_page(
    voffset: c_ulong,
    offset: c_ulong,
    mut flags: uint64_t,
    cgroup: uint64_t,
    mapcnt: uint64_t,
    pme: uint64_t,
) {
    flags = kpageflags_flags(flags, pme);

    if bit_mask_ok(flags) == 0 {
        return;
    }

    if opt_cgroup != 0 && cgroup != opt_cgroup {
        return;
    }

    if opt_hwpoison != 0 {
        hwpoison_page(offset);
    }
    if opt_unpoison != 0 {
        unpoison_page(offset);
    }

    if opt_mark_idle != 0 {
        mark_page_idle(offset);
    }

    if opt_list == 1 {
        show_page_range(voffset, offset, 1, flags, cgroup, mapcnt);
    } else if opt_list == 2 {
        show_page(voffset, offset, flags, cgroup, mapcnt);
    }

    nr_pages[hash_slot(flags)] += 1;
    total_pages += 1;
}

const KPAGEFLAGS_BATCH: usize = 64 << 10; /* 64k pages */
unsafe fn walk_pfn(voffset: c_ulong, mut index: c_ulong, mut count: c_ulong, pme: uint64_t) {
    let mut buf = [0u64; KPAGEFLAGS_BATCH];
    let mut cgi = [0u64; KPAGEFLAGS_BATCH];
    let mut cnt = [0u64; KPAGEFLAGS_BATCH];

    /*
     * kpagecgroup_read() reads only if kpagecgroup were opened, but
     * /proc/kpagecgroup might even not exist, so it's better to fill
     * them with zeros here.
     */
    if count == 1 {
        cgi[0] = 0;
    } else {
        cgi.fill(0);
    }

    while count != 0 {
        let batch = count.min(KPAGEFLAGS_BATCH as c_ulong);
        let pages = kpageflags_read(buf.as_mut_ptr(), index, batch);
        if pages == 0 {
            break;
        }

        if kpagecgroup_read(cgi.as_mut_ptr(), index, pages) != pages {
            fatal("kpagecgroup returned fewer pages than expected");
        }

        if kpagecount_read(cnt.as_mut_ptr(), index, pages) != pages {
            fatal("kpagecount returned fewer pages than expected");
        }

        for i in 0..pages as usize {
            add_page(voffset + i as c_ulong, index + i as c_ulong, buf[i], cgi[i], cnt[i], pme);
        }

        index += pages;
        count -= pages;
    }
}

unsafe fn walk_swap(voffset: c_ulong, pme: uint64_t) {
    let flags = kpageflags_flags(0, pme);

    if bit_mask_ok(flags) == 0 {
        return;
    }

    if opt_cgroup != 0 {
        return;
    }

    if opt_list == 1 {
        show_page_range(voffset, pagemap_swap_offset(pme), 1, flags, 0, 0);
    } else if opt_list == 2 {
        show_page(voffset, pagemap_swap_offset(pme), flags, 0, 0);
    }

    nr_pages[hash_slot(flags)] += 1;
    total_pages += 1;
}

const PAGEMAP_BATCH: usize = 64 << 10;
unsafe fn walk_vma(mut index: c_ulong, mut count: c_ulong) {
    let mut buf = [0u64; PAGEMAP_BATCH];

    while count != 0 {
        let batch = count.min(PAGEMAP_BATCH as c_ulong);
        let pages = pagemap_read(buf.as_mut_ptr(), index, batch);
        if pages == 0 {
            break;
        }

        for i in 0..pages as usize {
            let pfn = pagemap_pfn(buf[i]);
            if pfn != 0 {
                walk_pfn(index + i as c_ulong, pfn, 1, buf[i]);
            }
            if buf[i] & PM_SWAP != 0 {
                walk_swap(index + i as c_ulong, buf[i]);
            }
        }

        index += pages;
        count -= pages;
    }
}

unsafe fn walk_task(mut index: c_ulong, count: c_ulong) {
    let end = index + count;
    let mut i: usize = 0;

    while index < end {
        while pg_end[i] <= index {
            i += 1;
            if i >= nr_vmas as usize {
                return;
            }
        }
        if pg_start[i] >= end {
            return;
        }

        let start = pg_start[i].max(index);
        index = pg_end[i].min(end);

        assert!(start < index);
        walk_vma(start, index - start);
    }
}

unsafe fn add_addr_range(offset: c_ulong, size: c_ulong) {
    if nr_addr_ranges as usize >= MAX_ADDR_RANGES {
        fatal("too many addr ranges\n");
    }

    opt_offset[nr_addr_ranges as usize] = offset;
    opt_size[nr_addr_ranges as usize] = size.min(c_ulong::MAX - offset);
    nr_addr_ranges += 1;
}

unsafe fn walk_addr_ranges() {
    kpageflags_fd = checked_open(opt_kpageflags, libc::O_RDONLY);

    if nr_addr_ranges == 0 {
        add_addr_range(0, c_ulong::MAX);
    }

    for i in 0..nr_addr_ranges as usize {
        if opt_pid == 0 {
            walk_pfn(opt_offset[i], opt_offset[i], opt_size[i], 0);
        } else {
            walk_task(opt_offset[i], opt_size[i]);
        }
    }

    if opt_mark_idle != 0 {
        mark_page_idle(0);
    }

    libc::close(kpageflags_fd);
}

/*
 * user interface
 */

fn page_flag_type(flag: uint64_t) -> &'static str {
    if flag & KPF_HACKERS_BITS != 0 {
        "(r)"
    } else if flag & KPF_OVERLOADED_BITS != 0 {
        "(o)"
    } else {
        "   "
    }
}

fn usage() {
    println!(
"page-types [options]
            -r|--raw                   Raw mode, for kernel developers
            -d|--describe flags        Describe flags
            -a|--addr    addr-spec     Walk a range of pages
            -b|--bits    bits-spec     Walk pages with specified bits
            -c|--cgroup  path|@inode   Walk pages within memory cgroup
            -p|--pid     pid           Walk process address space
            -f|--file    filename      Walk file address space
            -i|--mark-idle             Mark pages idle
            -l|--list                  Show page details in ranges
            -L|--list-each             Show page details one by one
            -C|--list-cgroup           Show cgroup inode for pages
            -M|--list-mapcnt           Show page map count
            -N|--no-summary            Don't show summary info
            -X|--hwpoison              hwpoison pages
            -x|--unpoison              unpoison pages
            -F|--kpageflags filename   kpageflags file to parse
            -h|--help                  Show this usage message
flags:
            0x10                       bitfield format, e.g.
            anon                       bit-name, e.g.
            0x10,anon                  comma-separated list, e.g.
addr-spec:
            N                          one page at offset N (unit: pages)
            N+M                        pages range from N to N+M-1
            N,M                        pages range from N to M-1
            N,                         pages range from N to end
            ,M                         pages range from 0 to M-1
bits-spec:
            bit1,bit2                  (flags & (bit1|bit2)) != 0
            bit1,bit2=bit1             (flags & (bit1|bit2)) == bit1
            bit1,~bit2                 (flags & (bit1|bit2)) == bit1
            =bit1,bit2                 flags == (bit1|bit2)
bit-names:"
    );

    let mut j = 0usize;
    for i in 0..page_flag_names.len() {
        if let Some(name) = page_flag_names[i] {
            print!("{:>16}{}", &name[2..], page_flag_type(1u64 << i));
            j += 1;
            if j > 3 {
                j = 0;
                println!();
            }
        }
    }
    println!("\n                                   (r) raw mode bits  (o) overloaded bits");
}

unsafe fn parse_number(str_: *const c_char) -> c_ulong {
    let n = libc::strtoll(str_, ptr::null_mut(), 0) as c_ulong;

    if n == 0 && *str_ != b'0' as c_char {
        fatal(&format!("invalid name or number: {}\n", cstr(str_)));
    }

    n
}

unsafe fn parse_pid(str_: *const c_char) {
    let mut buf = [0i8; 5000];

    opt_pid = parse_number(str_) as pid_t;

    libc::sprintf(buf.as_mut_ptr(), b"/proc/%d/pagemap\0".as_ptr() as *const c_char, opt_pid);
    pagemap_fd = checked_open(buf.as_ptr(), libc::O_RDONLY);

    libc::sprintf(buf.as_mut_ptr(), b"/proc/%d/maps\0".as_ptr() as *const c_char, opt_pid);
    let file = libc::fopen(buf.as_ptr(), b"r\0".as_ptr() as *const c_char);
    if file.is_null() {
        libc::perror(buf.as_ptr());
        libc::exit(libc::EXIT_FAILURE);
    }

    while !libc::fgets(buf.as_mut_ptr(), buf.len() as c_int, file).is_null() {
        let mut vm_start: c_ulong = 0;
        let mut vm_end: c_ulong = 0;
        let mut pgoff: u64 = 0;
        let mut major: c_int = 0;
        let mut minor: c_int = 0;
        let mut r: c_char = 0;
        let mut w: c_char = 0;
        let mut x: c_char = 0;
        let mut s: c_char = 0;
        let mut ino: c_ulong = 0;

        let n = libc::sscanf(
            buf.as_ptr(),
            b"%lx-%lx %c%c%c%c %llx %x:%x %lu\0".as_ptr() as *const c_char,
            &mut vm_start,
            &mut vm_end,
            &mut r,
            &mut w,
            &mut x,
            &mut s,
            &mut pgoff,
            &mut major,
            &mut minor,
            &mut ino,
        );
        if n < 10 {
            eprintln!("unexpected line: {}", cstr(buf.as_ptr()));
            continue;
        }
        pg_start[nr_vmas as usize] = vm_start / page_size as c_ulong;
        pg_end[nr_vmas as usize] = vm_end / page_size as c_ulong;
        nr_vmas += 1;
        if nr_vmas as usize >= MAX_VMAS {
            eprintln!("too many VMAs");
            break;
        }
    }
    libc::fclose(file);
}

unsafe fn show_file(name: *const c_char, st: *const libc::stat) {
    let size = (*st).st_size as u64;
    let now = libc::time(ptr::null_mut());

    println!(
        "{}\tInode: {}\tSize: {} ({} pages)",
        cstr(name),
        (*st).st_ino as c_uint,
        size,
        (size + page_size as u64 - 1) / page_size as u64
    );

    let atime_tm = libc::localtime(&(*st).st_atime);
    let mtime_tm = libc::localtime(&(*st).st_mtime);
    let mut atime = [0i8; 64];
    let mut mtime = [0i8; 64];
    libc::strftime(atime.as_mut_ptr(), atime.len(), b"%c\0".as_ptr() as *const c_char, atime_tm);
    libc::strftime(mtime.as_mut_ptr(), mtime.len(), b"%c\0".as_ptr() as *const c_char, mtime_tm);

    println!(
        "Modify: {} ({} seconds ago)\nAccess: {} ({} seconds ago)",
        cstr(mtime.as_ptr()),
        now - (*st).st_mtime,
        cstr(atime.as_ptr()),
        now - (*st).st_atime
    );
}

static mut sigbus_jmp: libc::sigjmp_buf = unsafe { mem::zeroed() };
static mut sigbus_addr: *mut c_void = ptr::null_mut();

unsafe extern "C" fn sigbus_handler(sig: c_int, info: *mut libc::siginfo_t, ucontex: *mut c_void) {
    let _ = sig;
    let _ = ucontex;
    sigbus_addr = if !info.is_null() {
        (*info).si_addr()
    } else {
        ptr::null_mut()
    };
    libc::siglongjmp(&raw mut sigbus_jmp, 1);
}

static mut sigbus_action: libc::sigaction = unsafe { mem::zeroed() };
static mut st: libc::stat = unsafe { mem::zeroed() };

unsafe fn walk_file_range(name: *const c_char, fd: c_int, mut off: c_ulong, mut end: c_ulong) {
    let mut vec = [0u8; PAGEMAP_BATCH];
    let mut buf = [0u64; PAGEMAP_BATCH];
    let mut flags: uint64_t = 0;
    let mut cgroup: uint64_t = 0;
    let mut mapcnt: uint64_t = 0;
    let mut first = 1;

    while off < end {
        let mut nr_pages = (end - off + page_size as c_ulong - 1) / page_size as c_ulong;
        if nr_pages > PAGEMAP_BATCH as c_ulong {
            nr_pages = PAGEMAP_BATCH as c_ulong;
        }
        let len = nr_pages * page_size as c_ulong;

        let ptr = libc::mmap(
            ptr::null_mut(),
            len as usize,
            libc::PROT_READ,
            libc::MAP_SHARED,
            fd,
            off as off_t,
        );
        if ptr == libc::MAP_FAILED {
            fatal_c("mmap failed: %s", name);
        }

        /* determine cached pages */
        if libc::mincore(ptr, len as usize, vec.as_mut_ptr()) != 0 {
            fatal_c("mincore failed: %s", name);
        }

        /* turn off readahead */
        if libc::madvise(ptr, len as usize, libc::MADV_RANDOM) != 0 {
            fatal_c("madvise failed: %s", name);
        }

        if libc::sigsetjmp(&raw mut sigbus_jmp, 1) != 0 {
            end = off + if !sigbus_addr.is_null() {
                (sigbus_addr as usize - ptr as usize) as c_ulong
            } else {
                0
            };
            eprintln!("got sigbus at offset {}: {}", end as i64, cstr(name));
        } else {
            /* populate ptes */
            for i in 0..nr_pages as usize {
                if vec[i] & 1 != 0 {
                    ptr::read_volatile((ptr as *const u8).add(i * page_size as usize) as *const c_int);
                }
            }
        }

        /* turn off harvesting reference bits */
        if libc::madvise(ptr, len as usize, libc::MADV_SEQUENTIAL) != 0 {
            fatal_c("madvise failed: %s", name);
        }

        if pagemap_read(buf.as_mut_ptr(), ptr as c_ulong / page_size as c_ulong, nr_pages) != nr_pages {
            fatal("cannot read pagemap");
        }

        libc::munmap(ptr, len as usize);

        for i in 0..nr_pages as usize {
            let pfn = pagemap_pfn(buf[i]);
            if pfn == 0 {
                continue;
            }
            if kpageflags_read(&mut flags, pfn, 1) == 0 {
                continue;
            }
            if kpagecgroup_read(&mut cgroup, pfn, 1) == 0 {
                fatal("kpagecgroup_read failed");
            }
            if kpagecount_read(&mut mapcnt, pfn, 1) == 0 {
                fatal("kpagecount_read failed");
            }
            if first != 0 && opt_list != 0 {
                first = 0;
                flush_page_range();
            }
            add_page(off / page_size as c_ulong + i as c_ulong, pfn, flags, cgroup, mapcnt, buf[i]);
        }

        off += len;
    }
}

unsafe fn walk_file(name: *const c_char, st_: *const libc::stat) {
    let fd = checked_open(name, libc::O_RDONLY | libc::O_NOATIME | libc::O_NOFOLLOW);

    if nr_addr_ranges == 0 {
        add_addr_range(0, (*st_).st_size as c_ulong / page_size as c_ulong);
    }

    for i in 0..nr_addr_ranges as usize {
        walk_file_range(
            name,
            fd,
            opt_offset[i] * page_size as c_ulong,
            (opt_offset[i] + opt_size[i]) * page_size as c_ulong,
        );
    }

    libc::close(fd);
}

#[no_mangle]
pub unsafe extern "C" fn walk_tree(
    name: *const c_char,
    st_: *const libc::stat,
    type_: c_int,
    f: *mut libc::FTW,
) -> c_int {
    let _ = f;
    match type_ {
        libc::FTW_F => {
            if ((*st_).st_mode & libc::S_IFMT) == libc::S_IFREG {
                walk_file(name, st_);
            }
        }
        libc::FTW_DNR => {
            eprintln!("cannot read dir: {}", cstr(name));
        }
        _ => {}
    }
    0
}

unsafe fn walk_page_cache() {
    kpageflags_fd = checked_open(opt_kpageflags, libc::O_RDONLY);
    pagemap_fd = checked_open(b"/proc/self/pagemap\0".as_ptr() as *const c_char, libc::O_RDONLY);
    sigbus_action.sa_sigaction = sigbus_handler as usize;
    sigbus_action.sa_flags = libc::SA_SIGINFO;
    libc::sigaction(libc::SIGBUS, &raw const sigbus_action, ptr::null_mut());

    if libc::stat(opt_file, &raw mut st) != 0 {
        fatal_c("stat failed: %s\n", opt_file);
    }

    if (st.st_mode & libc::S_IFMT) == libc::S_IFREG {
        walk_file(opt_file, &raw const st);
    } else if (st.st_mode & libc::S_IFMT) == libc::S_IFDIR {
        /* do not follow symlinks and mountpoints */
        if libc::nftw(opt_file, Some(walk_tree), 64, libc::FTW_MOUNT | libc::FTW_PHYS) < 0 {
            fatal_c("nftw failed: %s\n", opt_file);
        }
    } else {
        fatal_c("unhandled file type: %s\n", opt_file);
    }

    libc::close(kpageflags_fd);
    libc::close(pagemap_fd);
    libc::signal(libc::SIGBUS, libc::SIG_DFL);
}

unsafe fn parse_file(name: *const c_char) {
    opt_file = name;
}

unsafe fn parse_cgroup(path: *const c_char) {
    if *path == b'@' as c_char {
        opt_cgroup = parse_number(path.add(1)) as uint64_t;
        return;
    }

    let mut st_: libc::stat = mem::zeroed();

    if libc::stat(path, &mut st_) != 0 {
        fatal(&format!("stat failed: {}: {}\n", cstr(path), std::io::Error::last_os_error()));
    }

    if (st_.st_mode & libc::S_IFMT) != libc::S_IFDIR {
        fatal(&format!("cgroup supposed to be a directory: {}\n", cstr(path)));
    }

    opt_cgroup = st_.st_ino as uint64_t;
}

unsafe fn parse_addr_range(optarg: *const c_char) {
    let comma = libc::strchr(optarg, b',' as c_int);
    let plus = libc::strchr(optarg, b'+' as c_int);
    let p = if !comma.is_null() { comma } else { plus };
    let offset: c_ulong;
    let mut size: c_ulong;

    if p == optarg {
        offset = 0;
        size = parse_number(p.add(1));
    } else if !p.is_null() {
        offset = parse_number(optarg);
        if *p.add(1) == 0 {
            size = c_ulong::MAX;
        } else {
            size = parse_number(p.add(1));
            if *p == b',' as c_char {
                if size < offset {
                    fatal(&format!("invalid range: {},{}\n", offset, size));
                }
                size -= offset;
            }
        }
    } else {
        offset = parse_number(optarg);
        size = 1;
    }

    add_addr_range(offset, size);
}

unsafe fn add_bits_filter(mask: uint64_t, bits: uint64_t) {
    if nr_bit_filters as usize >= MAX_BIT_FILTERS {
        fatal("too much bit filters\n");
    }

    opt_mask[nr_bit_filters as usize] = mask;
    opt_bits[nr_bit_filters as usize] = bits;
    nr_bit_filters += 1;
}

unsafe fn parse_flag_name(str_: *const c_char, len: c_int) -> uint64_t {
    if *str_ == 0 || len == 0 {
        return 0;
    }

    if len <= 8 && libc::strncmp(str_, b"compound\0".as_ptr() as *const c_char, len as usize) == 0 {
        return BITS_COMPOUND();
    }

    for i in 0..page_flag_names.len() {
        if let Some(name) = page_flag_names[i] {
            let cname = CString::new(&name[2..]).unwrap();
            if libc::strncmp(str_, cname.as_ptr(), len as usize) == 0 {
                return 1u64 << i;
            }
        }
    }

    parse_number(str_) as uint64_t
}

unsafe fn parse_flag_names(mut str_: *const c_char, all: c_int) -> uint64_t {
    let mut p = str_;
    let mut flags: uint64_t = 0;

    loop {
        if *p == b',' as c_char || *p == b'=' as c_char || *p == 0 {
            if *str_ != b'~' as c_char || (*str_ == b'~' as c_char && all != 0 && {
                str_ = str_.add(1);
                *str_ != 0
            }) {
                flags |= parse_flag_name(str_, p.offset_from(str_) as c_int);
            }
            if *p != b',' as c_char {
                break;
            }
            str_ = p.add(1);
        }
        p = p.add(1);
    }

    flags
}

unsafe fn parse_bits_mask(optarg: *const c_char) {
    let p = libc::strchr(optarg, b'=' as c_int);
    let mask: uint64_t;
    let bits: uint64_t;

    if p == optarg {
        mask = KPF_ALL_BITS;
        bits = parse_flag_names(p.add(1), 0);
    } else if !p.is_null() {
        mask = parse_flag_names(optarg, 0);
        bits = parse_flag_names(p.add(1), 0);
    } else if !libc::strchr(optarg, b'~' as c_int).is_null() {
        mask = parse_flag_names(optarg, 1);
        bits = parse_flag_names(optarg, 0);
    } else {
        mask = parse_flag_names(optarg, 0);
        bits = KPF_ALL_BITS;
    }

    add_bits_filter(mask, bits);
}

unsafe fn parse_kpageflags(name: *const c_char) {
    opt_kpageflags = name;
}

unsafe fn describe_flags(optarg: *const c_char) {
    let flags = parse_flag_names(optarg, 0);

    println!(
        "0x{:016x}\t{}\t{}",
        flags,
        cstr(page_flag_name(flags)),
        cstr(page_flag_longname(flags))
    );
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

extern "C" {
    static mut optarg: *mut c_char;
    fn getopt_long(argc: c_int, argv: *const *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int;
}

const no_argument: c_int = 0;
const required_argument: c_int = 1;

static opts: [option; 18] = [
    option { name: b"raw\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'r' as c_int },
    option { name: b"pid\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: b'p' as c_int },
    option { name: b"file\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: b'f' as c_int },
    option { name: b"addr\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: b'a' as c_int },
    option { name: b"bits\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: b'b' as c_int },
    option { name: b"cgroup\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: b'c' as c_int },
    option { name: b"describe\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: b'd' as c_int },
    option { name: b"mark-idle\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'i' as c_int },
    option { name: b"list\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'l' as c_int },
    option { name: b"list-each\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'L' as c_int },
    option { name: b"list-cgroup\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'C' as c_int },
    option { name: b"list-mapcnt\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'M' as c_int },
    option { name: b"no-summary\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'N' as c_int },
    option { name: b"hwpoison\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'X' as c_int },
    option { name: b"unpoison\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'x' as c_int },
    option { name: b"kpageflags\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: b'F' as c_int },
    option { name: b"help\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'h' as c_int },
    option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
];

unsafe fn real_main(argc: c_int, argv: *const *mut c_char) -> c_int {
    page_size = libc::getpagesize();

    loop {
        let c = getopt_long(
            argc,
            argv,
            b"rp:f:a:b:d:c:CilLMNXxF:h\0".as_ptr() as *const c_char,
            opts.as_ptr(),
            ptr::null_mut(),
        );
        if c == -1 {
            break;
        }
        match c {
            x if x == b'r' as c_int => opt_raw = 1,
            x if x == b'p' as c_int => parse_pid(optarg),
            x if x == b'f' as c_int => parse_file(optarg),
            x if x == b'a' as c_int => parse_addr_range(optarg),
            x if x == b'b' as c_int => parse_bits_mask(optarg),
            x if x == b'c' as c_int => parse_cgroup(optarg),
            x if x == b'C' as c_int => opt_list_cgroup = 1,
            x if x == b'd' as c_int => {
                describe_flags(optarg);
                libc::exit(0);
            }
            x if x == b'i' as c_int => opt_mark_idle = 1,
            x if x == b'l' as c_int => opt_list = 1,
            x if x == b'L' as c_int => opt_list = 2,
            x if x == b'M' as c_int => opt_list_mapcnt = 1,
            x if x == b'N' as c_int => opt_no_summary = 1,
            x if x == b'X' as c_int => {
                opt_hwpoison = 1;
                prepare_hwpoison_fd();
            }
            x if x == b'x' as c_int => {
                opt_unpoison = 1;
                prepare_hwpoison_fd();
            }
            x if x == b'F' as c_int => parse_kpageflags(optarg),
            x if x == b'h' as c_int => {
                usage();
                libc::exit(0);
            }
            _ => {
                usage();
                libc::exit(1);
            }
        }
    }

    if opt_kpageflags.is_null() {
        opt_kpageflags = PROC_KPAGEFLAGS.as_ptr() as *const c_char;
    }

    if opt_cgroup != 0 || opt_list_cgroup != 0 {
        kpagecgroup_fd = checked_open(PROC_KPAGECGROUP.as_ptr() as *const c_char, libc::O_RDONLY);
    }

    if opt_list != 0 && opt_list_mapcnt != 0 {
        kpagecount_fd = checked_open(PROC_KPAGECOUNT.as_ptr() as *const c_char, libc::O_RDONLY);
    }

    if opt_mark_idle != 0 {
        page_idle_fd = checked_open(SYS_KERNEL_MM_PAGE_IDLE.as_ptr() as *const c_char, libc::O_RDWR);
    }

    if opt_list != 0 && opt_pid != 0 {
        print!("voffset\t");
    }
    if opt_list != 0 && !opt_file.is_null() {
        print!("foffset\t");
    }
    if opt_list != 0 && opt_list_cgroup != 0 {
        print!("cgroup\t");
    }
    if opt_list != 0 && opt_list_mapcnt != 0 {
        print!("map-cnt\t");
    }

    if opt_list == 1 {
        println!("offset\tlen\tflags");
    }
    if opt_list == 2 {
        println!("offset\tflags");
    }

    if !opt_file.is_null() {
        walk_page_cache();
    } else {
        walk_addr_ranges();
    }

    if opt_list == 1 {
        flush_page_range();
    }

    if opt_no_summary != 0 {
        return 0;
    }

    if opt_list != 0 {
        println!("\n");
    }

    if !opt_file.is_null() {
        show_file(opt_file, &raw const st);
        println!();
    }

    show_summary();

    if opt_list_mapcnt != 0 {
        libc::close(kpagecount_fd);
    }

    if page_idle_fd >= 0 {
        libc::close(page_idle_fd);
    }

    0
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| CString::new(arg).unwrap().into_raw())
        .collect();
    args.push(ptr::null_mut());
    let code = unsafe { real_main((args.len() - 1) as c_int, args.as_ptr()) };
    unsafe {
        for arg in args.into_iter().take_while(|p| !p.is_null()) {
            drop(CString::from_raw(arg));
        }
        libc::exit(code);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
