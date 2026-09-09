// SPDX-License-Identifier: GPL-2.0
/*
 * Request memory topology information via diag0x310.
 *
 * Copyright IBM Corp. 2025
 */

// Dependencies supplied by the surrounding kernel translation.

const DIAG310_LEVELMIN: usize = 1;
const DIAG310_LEVELMAX: usize = 6;

#[repr(u32)]
enum Diag310Sc {
    DIAG310_SUBC_0 = 0,
    DIAG310_SUBC_1 = 1,
    DIAG310_SUBC_4 = 4,
    DIAG310_SUBC_5 = 5,
}

#[repr(u32)]
enum Diag310Retcode {
    DIAG310_RET_SUCCESS = 0x0001,
    DIAG310_RET_BUSY = 0x0101,
    DIAG310_RET_OPNOTSUPP = 0x0102,
    DIAG310_RET_SC4_INVAL = 0x0401,
    DIAG310_RET_SC4_NODATA = 0x0402,
    DIAG310_RET_SC5_INVAL = 0x0501,
    DIAG310_RET_SC5_NODATA = 0x0502,
    DIAG310_RET_SC5_ESIZE = 0x0503,
}

#[repr(C)]
union Diag310Response {
    response: u64,
}

impl Diag310Response {
    #[inline]
    unsafe fn result(&self) -> u32 {
        (self.response & 0xffff_ffff) as u32
    }

    #[inline]
    unsafe fn rc(&self) -> u16 {
        (self.response >> 48) as u16
    }
}

#[repr(C)]
union Diag310ReqSubcode {
    subcode: u64,
}

impl Diag310ReqSubcode {
    #[inline]
    fn new(sc: u64, st: u64) -> Self {
        Self { subcode: (st << 8) | sc }
    }
}

#[repr(C)]
union Diag310ReqSize {
    size: u64,
}

impl Diag310ReqSize {
    #[inline]
    fn new(page_count: u64) -> Self {
        Self { size: page_count }
    }
}

extern "C" {
    fn diag_stat_inc(stat: usize);
    fn diag310_asm(subcode: usize, size: usize, addr: *mut core::ffi::c_void) -> usize;
    fn test_bit_inv(nr: usize, addr: *const usize) -> bool;
    fn __vmalloc_node(size: usize, align: usize, gfp_mask: usize, node: i32, caller: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn vfree(addr: *mut core::ffi::c_void);
    fn copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
}

extern "C" {
    static sclp_has_diag310: bool;
}

const DIAG_STAT_X310: usize = 0;
const PAGE_SIZE: usize = 4096;
const GFP_KERNEL: usize = 0;
const __GFP_ZERO: usize = 0;
const __GFP_ACCOUNT: usize = 0;
const NUMA_NO_NODE: i32 = -1;

#[inline]
unsafe fn diag310(subcode: usize, size: usize, addr: *mut core::ffi::c_void) -> usize {
    diag_stat_inc(DIAG_STAT_X310);
    diag310_asm(subcode, size, addr)
}

unsafe fn diag310_result_to_errno(result: u32) -> i32 {
    match result {
        0x0101 => -16,  // -EBUSY
        0x0102 => -95,  // -EOPNOTSUPP
        _ => -22,       // -EINVAL
    }
}

unsafe fn diag310_get_subcode_mask(mask: *mut usize) -> i32 {
    let res = Diag310Response { response: diag310(0, 0, core::ptr::null_mut()) as u64 };
    if res.rc() != 0x0001 {
        return diag310_result_to_errno(res.rc() as u32);
    }
    *mask = res.response as usize;
    0
}

unsafe fn diag310_get_memtop_stride(stride: *mut usize) -> i32 {
    let res = Diag310Response { response: diag310(1, 0, core::ptr::null_mut()) as u64 };
    if res.rc() != 0x0001 {
        return diag310_result_to_errno(res.rc() as u32);
    }
    *stride = res.result() as usize;
    0
}

unsafe fn diag310_get_memtop_size(pages: *mut usize, level: usize) -> i32 {
    let req = Diag310ReqSubcode::new(4, level as u64);
    let res = Diag310Response { response: diag310(req.subcode as usize, 0, core::ptr::null_mut()) as u64 };
    match res.rc() {
        0x0001 => { *pages = res.result() as usize; 0 }
        0x0402 => -61, // -ENODATA
        0x0401 => -22, // -EINVAL
        rc => diag310_result_to_errno(rc as u32),
    }
}

unsafe fn diag310_store_topology_map(buf: *mut core::ffi::c_void, pages: usize, level: usize) -> i32 {
    let req_sc = Diag310ReqSubcode::new(5, level as u64);
    let req_size = Diag310ReqSize::new(pages as u64);
    let res = Diag310Response { response: diag310(req_sc.subcode as usize, req_size.size as usize, buf) as u64 };
    match res.rc() {
        0x0001 => 0,
        0x0502 => -61, // -ENODATA
        0x0503 => -75, // -EOVERFLOW
        0x0501 => -22, // -EINVAL
        rc => diag310_result_to_errno(rc as u32),
    }
}

static mut FEATURES_AVAILABLE: i32 = 0;
static mut MEMTOP_STRIDE: usize = 0;
static mut MEMTOP_PAGES: [usize; DIAG310_LEVELMAX] = [0; DIAG310_LEVELMAX];

unsafe fn diag310_check_features() -> i32 {
    if core::ptr::read_volatile(&FEATURES_AVAILABLE) != 0 { return 0; }
    if !sclp_has_diag310 { return -95; }
    let mut mask = 0usize;
    let rc = diag310_get_subcode_mask(&mut mask);
    if rc != 0 { return rc; }
    if !test_bit_inv(1, &mask) || !test_bit_inv(4, &mask) || !test_bit_inv(5, &mask) { return -95; }
    core::ptr::write_volatile(&mut FEATURES_AVAILABLE, 1);
    0
}

unsafe fn memtop_get_stride_len(res: *mut usize) -> i32 {
    let mut stride = core::ptr::read_volatile(&MEMTOP_STRIDE);
    if stride == 0 {
        let rc = diag310_get_memtop_stride(&mut stride);
        if rc != 0 { return rc; }
        core::ptr::write_volatile(&mut MEMTOP_STRIDE, stride);
    }
    *res = stride;
    0
}

unsafe fn memtop_get_page_count(res: *mut usize, level: usize) -> i32 {
    if level > DIAG310_LEVELMAX || level < DIAG310_LEVELMIN { return -22; }
    let idx = level - 1;
    let mut pages = core::ptr::read_volatile(&MEMTOP_PAGES[idx]);
    if pages == 0 {
        let rc = diag310_get_memtop_size(&mut pages, level);
        if rc != 0 { return rc; }
        core::ptr::write_volatile(&mut MEMTOP_PAGES[idx], pages);
    }
    *res = pages;
    0
}

#[no_mangle]
pub unsafe extern "C" fn diag310_memtop_stride(arg: usize) -> i64 {
    if diag310_check_features() != 0 { return diag310_check_features() as i64; }
    let mut stride = 0usize;
    let rc = memtop_get_stride_len(&mut stride);
    if rc != 0 { return rc as i64; }
    *(arg as *mut usize) = stride;
    0
}

#[no_mangle]
pub unsafe extern "C" fn diag310_memtop_len(arg: usize) -> i64 {
    let rc = diag310_check_features();
    if rc != 0 { return rc as i64; }
    let level = *(arg as *const usize);
    let mut pages = 0usize;
    let rc = memtop_get_page_count(&mut pages, level);
    if rc != 0 { return rc as i64; }
    *(arg as *mut usize) = pages.wrapping_mul(PAGE_SIZE);
    0
}

#[repr(C)]
struct Diag310Memtop {
    nesting_lvl: usize,
    address: u64,
}

#[no_mangle]
pub unsafe extern "C" fn diag310_memtop_buf(arg: usize) -> i64 {
    let udata = arg as *mut Diag310Memtop;
    let rc = diag310_check_features();
    if rc != 0 { return rc as i64; }
    let level = (*udata).nesting_lvl;
    let address = (*udata).address;
    let mut pages = 0usize;
    let rc = memtop_get_page_count(&mut pages, level);
    if rc != 0 { return rc as i64; }
    let data_size = pages.wrapping_mul(PAGE_SIZE);
    let buf = __vmalloc_node(data_size, PAGE_SIZE, GFP_KERNEL | __GFP_ZERO | __GFP_ACCOUNT,
                             NUMA_NO_NODE, core::ptr::null());
    if buf.is_null() { return -12; } // -ENOMEM
    let mut rc = diag310_store_topology_map(buf, pages, level);
    if rc == 0 && copy_to_user(address as *mut core::ffi::c_void, buf, data_size) != 0 {
        rc = -14; // -EFAULT
    }
    vfree(buf);
    rc as i64
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
