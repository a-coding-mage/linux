// SPDX-License-Identifier: GPL-2.0
/*
 * Request power readings for resources in a computing environment via
 * diag 0x324. diag 0x324 stores the power readings in the power information
 * block (pib).
 *
 * Copyright IBM Corp. 2024
 */

// Linux and s390 headers provide the types, constants, helpers, workqueue,
// user-access, allocator, and diagnostic interfaces referenced below.

#[repr(u32)]
enum Subcode {
    DIAG324_SUBC_0 = 0,
    DIAG324_SUBC_1 = 1,
    DIAG324_SUBC_2 = 2,
}

const DIAG324_RET_SUCCESS: u16 = 0x0001;
const DIAG324_RET_SUBC_NOTAVAIL: u16 = 0x0103;
const DIAG324_RET_INSUFFICIENT_SIZE: u16 = 0x0104;
const DIAG324_RET_READING_UNAVAILABLE: u16 = 0x0105;

#[repr(C)]
union Diag324Response {
    response: u64,
    sc0: Diag324ResponseSc0,
    sc1: Diag324ResponseSc1,
    sc2: Diag324ResponseSc2,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Diag324ResponseSc0 {
    response: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Diag324ResponseSc1 {
    response: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Diag324ResponseSc2 {
    response: u64,
}

#[repr(C)]
union Diag324Request {
    request: u64,
    sc2: Diag324RequestSc2,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Diag324RequestSc2 {
    request: u64,
}

#[repr(C, packed)]
struct Pib {
    _reserved0: u32,
    _reserved1: u64,
    intv: u64,
    r: [u8; 0],
}

#[repr(C)]
struct Pibdata {
    pib: *mut Pib,
    expire: KtimeT,
    sequence: u64,
    len: usize,
    rc: i32,
}

type KtimeT = i64;

extern "C" {
    static mut pibmutex: Mutex;
    static mut pibdata: Pibdata;
    static mut pibwork: DelayedWork;
    static mut system_percpu_wq: *mut WorkqueueStruct;
    static sclp: Sclp;

    fn diag_stat_inc(stat: i32);
    fn vmalloc(size: usize) -> *mut core::ffi::c_void;
    fn vfree(addr: *mut core::ffi::c_void);
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn ktime_get() -> KtimeT;
    fn ktime_add_ns(kt: KtimeT, ns: i64) -> KtimeT;
    fn ktime_before(a: KtimeT, b: KtimeT) -> bool;
    fn ktime_after(a: KtimeT, b: KtimeT) -> bool;
    fn tod_to_ns(tod: u64) -> i64;
    fn nsecs_to_jiffies(nsecs: i64) -> u64;
    fn mutex_lock(mutex: *mut Mutex);
    fn mutex_unlock(mutex: *mut Mutex);
    fn mod_delayed_work(wq: *mut WorkqueueStruct, work: *mut DelayedWork, delay: u64);
    fn get_user(dst: *mut u64, src: *const u64) -> i32;
    fn put_user<T>(value: T, dst: *mut T) -> i32;
    fn copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> i32;
    fn test_bit_inv(nr: u32, addr: *const usize) -> bool;
}

#[repr(C)] struct Mutex;
#[repr(C)] struct DelayedWork;
#[repr(C)] struct WorkStruct;
#[repr(C)] struct WorkqueueStruct;
#[repr(C)] struct Sclp { has_diag324: bool }
#[repr(C)] struct Diag324Pib { address: u64, sequence: u64 }

const PIBWORK_DELAY: i64 = 5 * 1_000_000_000;
const EOPNOTSUPP: i32 = 95;
const EFAULT: i32 = 14;
const ENOMEM: i32 = 12;
const ENOENT: i32 = 2;
const EMSGSIZE: i32 = 90;
const EBUSY: i32 = 16;
const EINVAL: i32 = 22;

unsafe fn response_rc(response: u64) -> u16 { (response & 0xffff) as u16 }
unsafe fn response_sc0_installed(response: u64) -> usize { (response >> 32) as usize }
unsafe fn response_sc1_pib_len(response: u64) -> usize { ((response >> 16) & 0xffff) as usize }

unsafe fn diag324(subcode: usize, addr: *mut core::ffi::c_void) -> usize {
    diag_stat_inc(0x324);
    let _ = addr;
    // The inline s390 `diag` instruction updates the register pair and returns
    // its odd register; the architecture-specific implementation supplies it.
    core::hint::unreachable_unchecked()
}

unsafe extern "C" fn pibwork_handler(_work: *mut WorkStruct) {
    let data = &mut pibdata;
    let timedout = ktime_add_ns(data.expire, PIBWORK_DELAY);
    if ktime_before(ktime_get(), timedout) {
        mod_delayed_work(system_percpu_wq, &mut pibwork, nsecs_to_jiffies(PIBWORK_DELAY));
        return;
    }
    vfree(data.pib as *mut core::ffi::c_void);
    data.pib = core::ptr::null_mut();
}

unsafe fn pib_update(data: &mut Pibdata) {
    let req = ((DIAG324_SUBC_2 as u64) | ((data.len as u64) << 16)) as usize;
    memset(data.pib as *mut core::ffi::c_void, 0, data.len);
    let response = diag324(req, data.pib as *mut core::ffi::c_void) as u64;
    data.rc = match response_rc(response) {
        DIAG324_RET_SUCCESS => 0,
        DIAG324_RET_SUBC_NOTAVAIL => -ENOENT,
        DIAG324_RET_INSUFFICIENT_SIZE => -EMSGSIZE,
        DIAG324_RET_READING_UNAVAILABLE => -EBUSY,
        _ => -EINVAL,
    };
}

pub unsafe extern "C" fn diag324_pibbuf(arg: usize) -> i64 {
    let udata = arg as *mut Diag324Pib;
    let data = &mut pibdata;
    static mut FIRST: bool = true;
    if data.len == 0 { return -(EOPNOTSUPP as i64); }
    let mut address = 0u64;
    if get_user(&mut address, &(*udata).address) != 0 { return -(EFAULT as i64); }
    mutex_lock(&mut pibmutex);
    let mut rc = -ENOMEM;
    if data.pib.is_null() { data.pib = vmalloc(data.len) as *mut Pib; }
    if data.pib.is_null() { mutex_unlock(&mut pibmutex); return rc as i64; }
    if FIRST || ktime_after(ktime_get(), data.expire) {
        pib_update(data);
        data.sequence = data.sequence.wrapping_add(1);
        data.expire = ktime_add_ns(ktime_get(), tod_to_ns((*data.pib).intv));
        mod_delayed_work(system_percpu_wq, &mut pibwork, nsecs_to_jiffies(PIBWORK_DELAY));
        FIRST = false;
    }
    rc = data.rc;
    if rc != 0 && rc != -EBUSY { mutex_unlock(&mut pibmutex); return rc as i64; }
    rc = copy_to_user(address as *mut core::ffi::c_void, data.pib as *const core::ffi::c_void, (*data.pib).len as usize);
    rc |= put_user(data.sequence, &mut (*udata).sequence);
    if rc != 0 { rc = -EFAULT; }
    mutex_unlock(&mut pibmutex);
    rc as i64
}

pub unsafe extern "C" fn diag324_piblen(arg: usize) -> i64 {
    if pibdata.len == 0 { return -(EOPNOTSUPP as i64); }
    if put_user(pibdata.len, arg as *mut usize) != 0 { return -(EFAULT as i64); }
    0
}

unsafe extern "C" fn diag324_init() -> i32 {
    if !sclp.has_diag324 { return -EOPNOTSUPP; }
    let response = diag324(DIAG324_SUBC_0 as usize, core::ptr::null_mut()) as u64;
    if response_rc(response) != DIAG324_RET_SUCCESS { return -EOPNOTSUPP; }
    let installed = response_sc0_installed(response);
    if !test_bit_inv(DIAG324_SUBC_1 as u32, &installed) || !test_bit_inv(DIAG324_SUBC_2 as u32, &installed) { return -EOPNOTSUPP; }
    let response = diag324(DIAG324_SUBC_1 as usize, core::ptr::null_mut()) as u64;
    if response_rc(response) != DIAG324_RET_SUCCESS { return -EOPNOTSUPP; }
    pibdata.len = response_sc1_pib_len(response);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
