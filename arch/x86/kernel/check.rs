// SPDX-License-Identifier: GPL-2.0

// Kernel dependencies supplied by the surrounding build are intentionally not
// expanded here.

const MAX_SCAN_AREAS: usize = 8;

static mut MEMORY_CORRUPTION_CHECK: i32 = -1;
static mut CORRUPTION_CHECK_SIZE: u32 = 64 * 1024;
static mut CORRUPTION_CHECK_PERIOD: u32 = 60;

#[repr(C)]
struct ScanArea {
    addr: u64,
    size: u64,
}

static mut SCAN_AREAS: [ScanArea; MAX_SCAN_AREAS] = [ScanArea { addr: 0, size: 0 }; MAX_SCAN_AREAS];
static mut NUM_SCAN_AREAS: i32 = 0;

unsafe extern "C" {
    fn kstrtoul(arg: *const core::ffi::c_char, base: u32, val: *mut usize) -> isize;
    fn memparse(arg: *const core::ffi::c_char, end: *mut *mut core::ffi::c_char) -> u32;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn __va(addr: u64) -> *mut core::ffi::c_void;
    fn __pa(addr: *const core::ffi::c_void) -> u64;
    fn memblock_reserve(addr: u64, size: u64);
    fn memset(dst: *mut core::ffi::c_void, value: i32, size: u64) -> *mut core::ffi::c_void;
    fn round_up(value: u64, alignment: u64) -> u64;
    fn round_down(value: u64, alignment: u64) -> u64;
    fn clamp_t(value: u64, min: u64, max: u64) -> u64;
    fn for_each_free_mem_range(
        index: *mut u64,
        nid: i32,
        flags: u32,
        start: *mut u64,
        end: *mut u64,
        type_: *mut core::ffi::c_void,
    );
    fn schedule_delayed_work(work: *mut WorkStruct, delay: u64) -> i32;
    fn round_jiffies_relative(jiffies: u64) -> u64;
    fn warn_once(condition: bool, fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
struct WorkStruct {
    _private: [u8; 0],
}

static mut BIOS_CHECK_WORK: WorkStruct = WorkStruct { _private: [] };

unsafe fn set_corruption_check(arg: *mut core::ffi::c_char) -> i32 {
    if arg.is_null() {
        pr_err(c"memory_corruption_check config string not provided\n".as_ptr());
        return -22;
    }
    let mut val = 0usize;
    let ret = kstrtoul(arg, 10, &mut val);
    if ret != 0 {
        return ret as i32;
    }
    MEMORY_CORRUPTION_CHECK = val as i32;
    0
}

unsafe fn set_corruption_check_period(arg: *mut core::ffi::c_char) -> i32 {
    if arg.is_null() {
        pr_err(c"memory_corruption_check_period config string not provided\n".as_ptr());
        return -22;
    }
    let mut val = 0usize;
    let ret = kstrtoul(arg, 10, &mut val);
    if ret != 0 {
        return ret as i32;
    }
    CORRUPTION_CHECK_PERIOD = val as u32;
    0
}

unsafe fn set_corruption_check_size(arg: *mut core::ffi::c_char) -> i32 {
    if arg.is_null() {
        pr_err(c"memory_corruption_check_size config string not provided\n".as_ptr());
        return -22;
    }
    let mut end = core::ptr::null_mut();
    let size = memparse(arg, &mut end);
    if *end == 0 {
        CORRUPTION_CHECK_SIZE = size;
    }
    if size == CORRUPTION_CHECK_SIZE { 0 } else { -22 }
}

pub unsafe fn setup_bios_corruption_check() {
    let mut start: u64;
    let mut end: u64;
    let mut i = 0u64;
    if MEMORY_CORRUPTION_CHECK == -1 {
        // CONFIG_X86_BOOTPARAM_MEMORY_CORRUPTION_CHECK selects 1; otherwise 0.
        MEMORY_CORRUPTION_CHECK = 0;
    }
    if CORRUPTION_CHECK_SIZE == 0 {
        MEMORY_CORRUPTION_CHECK = 0;
    }
    if MEMORY_CORRUPTION_CHECK == 0 { return; }
    CORRUPTION_CHECK_SIZE = round_up(CORRUPTION_CHECK_SIZE as u64, 4096) as u32;
    loop {
        for_each_free_mem_range(&mut i, -1, 0, &mut start, &mut end, core::ptr::null_mut());
        start = clamp_t(round_up(start, 4096), 4096, CORRUPTION_CHECK_SIZE as u64);
        end = clamp_t(round_down(end, 4096), 4096, CORRUPTION_CHECK_SIZE as u64);
        if start < end {
            memblock_reserve(start, end - start);
            SCAN_AREAS[NUM_SCAN_AREAS as usize] = ScanArea { addr: start, size: end - start };
            memset(__va(start), 0, end - start);
            NUM_SCAN_AREAS += 1;
            if NUM_SCAN_AREAS >= MAX_SCAN_AREAS as i32 { break; }
        }
        // The kernel iterator advances through the free-memory ranges.
        if i == u64::MAX { break; }
    }
    if NUM_SCAN_AREAS != 0 { pr_info(c"Scanning %d areas for low memory corruption\n".as_ptr(), NUM_SCAN_AREAS); }
}

unsafe fn check_for_bios_corruption() {
    if MEMORY_CORRUPTION_CHECK == 0 { return; }
    let mut corruption = false;
    for i in 0..NUM_SCAN_AREAS {
        let mut addr = __va(SCAN_AREAS[i as usize].addr) as *mut usize;
        let mut size = SCAN_AREAS[i as usize].size as usize;
        while size != 0 {
            if *addr != 0 {
                pr_err(c"Corrupted low memory at %p (%lx phys) = %08lx\n".as_ptr(), addr, __pa(addr.cast()), *addr);
                corruption = true;
                *addr = 0;
            }
            addr = addr.add(1);
            size -= core::mem::size_of::<usize>();
        }
    }
    warn_once(corruption, c"Memory corruption detected in low memory\n".as_ptr());
}

unsafe fn check_corruption(_dummy: *mut WorkStruct) {
    check_for_bios_corruption();
    schedule_delayed_work(&raw mut BIOS_CHECK_WORK, round_jiffies_relative(CORRUPTION_CHECK_PERIOD as u64 * 100));
}

unsafe fn start_periodic_check_for_corruption() -> i32 {
    if NUM_SCAN_AREAS == 0 || MEMORY_CORRUPTION_CHECK == 0 || CORRUPTION_CHECK_PERIOD == 0 { return 0; }
    pr_info(c"Scanning for low memory corruption every %d seconds\n".as_ptr(), CORRUPTION_CHECK_PERIOD);
    schedule_delayed_work(&raw mut BIOS_CHECK_WORK, 0);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
