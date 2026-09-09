// SPDX-License-Identifier: GPL-2.0-only
/*
 * crash.c - kernel crash support code.
 * Copyright (C) 2002-2004 Eric Biederman  <ebiederm@xmission.com>
 */

// Kernel headers and architecture-specific headers are supplied by the surrounding build.

#[repr(C)]
pub struct Resource {
    pub name: *const core::ffi::c_char,
    pub start: u64,
    pub end: u64,
    pub flags: u64,
    pub desc: u32,
}

#[repr(C)]
pub struct Range { pub start: u64, pub end: u64 }

pub static mut crashk_res: Resource = Resource { name: b"Crash kernel\0".as_ptr() as *const _, start: 0, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM, desc: IORES_DESC_CRASH_KERNEL };
pub static mut crashk_low_res: Resource = Resource { name: b"Crash kernel\0".as_ptr() as *const _, start: 0, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM, desc: IORES_DESC_CRASH_KERNEL };

const SUFFIX_HIGH: usize = 0;
const SUFFIX_LOW: usize = 1;
const SUFFIX_CMA: usize = 2;
const SUFFIX_NULL: usize = 3;
static mut suffix_tbl: [*mut core::ffi::c_char; 4] = [b",high\0".as_ptr() as *mut _, b",low\0".as_ptr() as *mut _, b",cma\0".as_ptr() as *mut _, core::ptr::null_mut()];

unsafe fn parse_crashkernel_mem(mut cmdline: *mut u8, system_ram: u64, crash_size: *mut u64, crash_base: *mut u64) -> i32 {
    let mut total_mem = roundup(system_ram, SZ_128M);
    let mut cur = cmdline;
    loop {
        let mut tmp = core::ptr::null_mut();
        let start = memparse(cur, &mut tmp);
        if cur == tmp { pr_warn!("crashkernel: Memory value expected\n"); return -EINVAL; }
        cur = tmp;
        if *cur != b'-' { pr_warn!("crashkernel: '-' expected\n"); return -EINVAL; }
        cur = cur.add(1);
        let mut end = u64::MAX;
        if *cur != b':' {
            end = memparse(cur, &mut tmp);
            if cur == tmp { pr_warn!("crashkernel: Memory value expected\n"); return -EINVAL; }
            cur = tmp;
            if end <= start { pr_warn!("crashkernel: end <= start\n"); return -EINVAL; }
        }
        if *cur != b':' { pr_warn!("crashkernel: ':' expected\n"); return -EINVAL; }
        cur = cur.add(1);
        let size = memparse(cur, &mut tmp);
        if cur == tmp { pr_warn!("crashkernel: Memory value expected\n"); return -EINVAL; }
        cur = tmp;
        if size >= total_mem { pr_warn!("crashkernel: invalid size\n"); return -EINVAL; }
        if total_mem >= start && total_mem < end { *crash_size = size; break; }
        if *cur != b',' { break; }
        cur = cur.add(1);
    }
    if *crash_size > 0 {
        while *cur != 0 && *cur != b' ' && *cur != b'@' { cur = cur.add(1); }
        if *cur == b'@' {
            cur = cur.add(1); let mut tmp = core::ptr::null_mut();
            *crash_base = memparse(cur, &mut tmp);
            if cur == tmp { pr_warn!("crashkernel: Memory value expected after '@'\n"); return -EINVAL; }
        }
    } else { pr_info!("crashkernel size resulted in zero bytes\n"); }
    0
}

unsafe fn parse_crashkernel_simple(cmdline: *mut u8, crash_size: *mut u64, crash_base: *mut u64) -> i32 {
    let mut cur = cmdline;
    *crash_size = memparse(cmdline, &mut cur);
    if cmdline == cur { pr_warn!("crashkernel: memory value expected\n"); return -EINVAL; }
    if *cur == b'@' { let mut p = cur.add(1); *crash_base = memparse(p, &mut cur); }
    else if *cur != b' ' && *cur != 0 { pr_warn!("crashkernel: unrecognized char: %c\n", *cur); return -EINVAL; }
    0
}

unsafe fn parse_crashkernel_suffix(cmdline: *mut u8, crash_size: *mut u64, suffix: *const core::ffi::c_char) -> i32 {
    let mut cur = cmdline;
    *crash_size = memparse(cmdline, &mut cur);
    if cmdline == cur { pr_warn!("crashkernel: memory value expected\n"); return -EINVAL; }
    let len = strlen(suffix); if strncmp(cur, suffix, len) != 0 { pr_warn!("crashkernel: unrecognized char: %c\n", *cur); return -EINVAL; }
    cur = cur.add(len); if *cur != b' ' && *cur != 0 { pr_warn!("crashkernel: unrecognized char: %c\n", *cur); return -EINVAL; } 0
}

unsafe fn get_last_crashkernel(mut cmdline: *mut u8, name: *const u8, suffix: *const core::ffi::c_char) -> *mut u8 {
    let mut p = strstr(cmdline, name); let mut result = core::ptr::null_mut();
    while !p.is_null() { let mut end_p = strchr(p, b' '); if end_p.is_null() { end_p = p.add(strlen(p)); }
        if suffix.is_null() { let mut i = 0; while !suffix_tbl[i].is_null() { let q = end_p.sub(strlen(suffix_tbl[i])); if strncmp(q, suffix_tbl[i], strlen(suffix_tbl[i])) == 0 { p = strstr(p.add(1), name); continue; } i += 1; } result = p; }
        else { let q = end_p.sub(strlen(suffix)); if strncmp(q, suffix, strlen(suffix)) == 0 { result = p; } }
        p = strstr(p.add(1), name);
    } result
}

unsafe fn __parse_crashkernel(cmdline: *mut u8, system_ram: u64, crash_size: *mut u64, crash_base: *mut u64, suffix: *const core::ffi::c_char) -> i32 {
    BUG_ON!(crash_size.is_null() || crash_base.is_null()); *crash_size = 0; *crash_base = 0;
    let mut p = get_last_crashkernel(cmdline, b"crashkernel=\0".as_ptr(), suffix); if p.is_null() { return -ENOENT; }
    p = p.add(strlen(b"crashkernel=\0".as_ptr()));
    if !suffix.is_null() { return parse_crashkernel_suffix(p, crash_size, suffix); }
    let colon = strchr(p, b':'); let space = strchr(p, b' ');
    if !colon.is_null() && (space.is_null() || colon < space) { parse_crashkernel_mem(p, system_ram, crash_size, crash_base) } else { parse_crashkernel_simple(p, crash_size, crash_base) }
}

pub unsafe fn parse_crashkernel(cmdline: *mut u8, system_ram: u64, crash_size: *mut u64, crash_base: *mut u64, low_size: *mut u64, cma_size: *mut u64, high: *mut bool) -> i32 {
    let mut ret = __parse_crashkernel(cmdline, system_ram, crash_size, crash_base, core::ptr::null());
    // CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION conditionally enables high/low/CMA parsing.
    if !high.is_null() && ret == -ENOENT { ret = __parse_crashkernel(cmdline, 0, crash_size, crash_base, suffix_tbl[SUFFIX_HIGH]); if ret != 0 || *crash_size == 0 { return -EINVAL; } ret = __parse_crashkernel(cmdline, 0, low_size, crash_base, suffix_tbl[SUFFIX_LOW]); if ret == -ENOENT { *low_size = DEFAULT_CRASH_KERNEL_LOW_SIZE; ret = 0; } else if ret != 0 { return ret; } *high = true; }
    if !cma_size.is_null() { let mut cma_base = 0; __parse_crashkernel(cmdline, 0, cma_size, &mut cma_base, suffix_tbl[SUFFIX_CMA]); }
    if *crash_size == 0 || *crash_size >= system_ram { ret = -EINVAL; } ret
}

unsafe fn parse_crashkernel_dummy(_arg: *mut u8) -> i32 { 0 }
// early_param("crashkernel", parse_crashkernel_dummy);

// The remaining reservation functions retain the kernel's external allocation/resource APIs.
// CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION and CRASHKERNEL_CMA gate their definitions.
extern "C" {
    fn memparse(s: *mut u8, endp: *mut *mut u8) -> u64;
    fn roundup(x: u64, y: u64) -> u64;
    fn strlen(s: *const core::ffi::c_char) -> usize;
    fn strncmp(a: *const u8, b: *const core::ffi::c_char, n: usize) -> i32;
    fn strstr(a: *mut u8, b: *const u8) -> *mut u8;
    fn strchr(a: *mut u8, c: u8) -> *mut u8;
}

#[cfg(feature = "generic_crashkernel_reservation")]
pub unsafe fn reserve_crashkernel_generic(mut crash_size: u64, mut crash_base: u64, mut crash_low_size: u64, high: bool) {
    let mut search_end = CRASH_ADDR_LOW_MAX; let mut search_base = 0; let fixed_base = crash_base != 0;
    if fixed_base { search_base = crash_base; search_end = crash_base + crash_size; }
    else if high { search_base = CRASH_ADDR_LOW_MAX; search_end = CRASH_ADDR_HIGH_MAX; }
    loop {
        crash_base = memblock_phys_alloc_range(crash_size, CRASH_ALIGN, search_base, search_end);
        if crash_base == 0 {
            if fixed_base { pr_warn!("crashkernel reservation failed - memory is in use.\n"); return; }
            if !high && search_end == CRASH_ADDR_LOW_MAX { search_end = CRASH_ADDR_HIGH_MAX; search_base = CRASH_ADDR_LOW_MAX; crash_low_size = DEFAULT_CRASH_KERNEL_LOW_SIZE; continue; }
            if high && search_end == CRASH_ADDR_HIGH_MAX { search_end = CRASH_ADDR_LOW_MAX; search_base = 0; if search_end != CRASH_ADDR_HIGH_MAX { continue; } }
            pr_warn!("cannot allocate crashkernel (size:0x%llx)\n", crash_size); return;
        }
        if crash_base >= CRASH_ADDR_LOW_MAX && crash_low_size != 0 && reserve_crashkernel_low(crash_low_size) != 0 { memblock_phys_free(crash_base, crash_size); return; }
        pr_info!("crashkernel reserved: 0x%016llx - 0x%016llx (%lld MB)\n", crash_base, crash_base + crash_size, crash_size >> 20);
        kmemleak_ignore_phys(crash_base); if crashk_low_res.end != 0 { kmemleak_ignore_phys(crashk_low_res.start); }
        crashk_res.start = crash_base; crashk_res.end = crash_base + crash_size - 1; return;
    }
}

#[cfg(feature = "generic_crashkernel_reservation")]
unsafe fn reserve_crashkernel_low(low_size: u64) -> i32 {
    let low_base = memblock_phys_alloc_range(low_size, CRASH_ALIGN, 0, CRASH_ADDR_LOW_MAX);
    if low_base == 0 { pr_err!("cannot allocate crashkernel low memory (size:0x%llx).\n", low_size); return -ENOMEM; }
    pr_info!("crashkernel low memory reserved: 0x%08llx - 0x%08llx (%lld MB)\n", low_base, low_base + low_size, low_size >> 20);
    crashk_low_res.start = low_base; crashk_low_res.end = low_base + low_size - 1; 0
}

pub unsafe fn reserve_crashkernel_cma(cma_size: u64) {
    #[cfg(feature = "crashkernel_cma")]
    { let mut request_size = roundup(cma_size, PAGE_SIZE); let mut reserved_size = 0; if cma_size == 0 { return; } while cma_size > reserved_size && crashk_cma_cnt < CRASHKERNEL_CMA_RANGES_MAX { let mut res = core::ptr::null_mut(); if cma_declare_contiguous(0, request_size, 0, 0, 0, false, b"crashkernel\0".as_ptr() as *const _, &mut res) != 0 { if request_size <= PAGE_SIZE { break; } request_size = roundup(request_size / 2, PAGE_SIZE); continue; } crashk_cma_ranges[crashk_cma_cnt].start = cma_get_base(res); crashk_cma_ranges[crashk_cma_cnt].end = crashk_cma_ranges[crashk_cma_cnt].start + cma_get_size(res) - 1; crashk_cma_cnt += 1; reserved_size += request_size; } if cma_size > reserved_size { pr_warn!("crashkernel CMA reservation failed: %lld MB requested, %lld MB reserved in %d ranges\n", cma_size >> 20, reserved_size >> 20, crashk_cma_cnt); } else { pr_info!("crashkernel CMA reserved: %lld MB in %d ranges\n", reserved_size >> 20, crashk_cma_cnt); } }
    #[cfg(not(feature = "crashkernel_cma"))]
    { if cma_size != 0 { pr_warn!("crashkernel CMA reservation not supported\n"); } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
