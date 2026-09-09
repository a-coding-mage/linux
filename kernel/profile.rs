// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/kernel/profile.c
 * Simple profiling. Manages a direct-mapped profile hit count buffer.
 *
 * C headers and kernel-provided symbols are intentionally left as external
 * dependencies of this translation.
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct profile_hit {
    pub pc: u32,
    pub hits: u32,
}

// PROFILE_GRPSHIFT = 3; PROFILE_GRPSZ = 1 << PROFILE_GRPSHIFT;
// NR_PROFILE_HIT = PAGE_SIZE / sizeof(struct profile_hit);
// NR_PROFILE_GRP = NR_PROFILE_HIT / PROFILE_GRPSZ;

// Kernel-provided types and functions.
type AtomicT = c_int;
type PtRegs = c_void;
type SizeT = usize;
type SsizeT = isize;
type LoffT = i64;

extern "C" {
    static mut _stext: u8;
    static mut _etext: u8;
    static mut SCHED_PROFILING: c_int;
    static mut KVM_PROFILING: c_int;
    static mut CPU_PROFILING: c_int;
    static BITS_PER_LONG: usize;

    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn get_option(s: *mut *mut c_char, val: *mut c_int) -> c_int;
    fn pr_info(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn kzalloc(size: usize, flags: usize) -> *mut AtomicT;
    fn alloc_pages_exact(size: usize, flags: usize) -> *mut AtomicT;
    fn vzalloc(size: usize) -> *mut AtomicT;
    fn atomic_add(i: u32, v: *mut AtomicT);
    fn get_irq_regs() -> *mut PtRegs;
    fn user_mode(regs: *mut PtRegs) -> c_int;
    fn profile_pc(regs: *mut PtRegs) -> *mut c_void;
    fn profile_hit(type_: c_int, pc: *mut c_void);
    fn put_user(value: u8, dst: *mut c_char) -> c_int;
    fn copy_to_user(dst: *mut c_char, src: *const c_void, count: usize) -> c_int;
    fn copy_from_user(dst: *mut u32, src: *const c_char, count: usize) -> c_int;
    fn memset(dst: *mut c_void, value: c_int, count: usize) -> *mut c_void;
}

static mut prof_buffer: *mut AtomicT = core::ptr::null_mut();
static mut prof_len: usize = 0;
static mut prof_shift: u16 = 0;
#[no_mangle]
pub static mut prof_on: c_int = 0;

pub unsafe extern "C" fn profile_setup(mut str_: *mut c_char) -> c_int {
    let schedstr = b"schedule\0";
    let kvmstr = b"kvm\0";
    let mut select: *const c_char = core::ptr::null();
    let mut par: c_int = 0;

    if strncmp(str_, schedstr.as_ptr() as *const c_char, strlen(schedstr.as_ptr() as *const c_char)) == 0 {
        prof_on = SCHED_PROFILING;
        select = schedstr.as_ptr() as *const c_char;
    } else if strncmp(str_, kvmstr.as_ptr() as *const c_char, strlen(kvmstr.as_ptr() as *const c_char)) == 0 {
        prof_on = KVM_PROFILING;
        select = kvmstr.as_ptr() as *const c_char;
    } else if get_option(&mut str_, &mut par) != 0 {
        prof_shift = par.clamp(0, (BITS_PER_LONG - 1) as c_int) as u16;
        prof_on = CPU_PROFILING;
        pr_info(b"kernel profiling enabled (shift: %u)\n\0".as_ptr() as *const c_char, prof_shift as c_int);
    }

    if !select.is_null() {
        let n = strlen(select);
        if *str_.add(n) as u8 == b',' { str_ = str_.add(n + 1); }
        if get_option(&mut str_, &mut par) != 0 {
            prof_shift = par.clamp(0, (BITS_PER_LONG - 1) as c_int) as u16;
        }
        pr_info(b"kernel %s profiling enabled (shift: %u)\n\0".as_ptr() as *const c_char, select, prof_shift as c_int);
    }
    1
}

pub unsafe extern "C" fn profile_init() -> c_int {
    if prof_on == 0 { return 0; }
    prof_len = ((_etext as usize).wrapping_sub(&_stext as *const u8 as usize)) >> prof_shift;
    if prof_len == 0 { prof_on = 0; return -22; }
    let buffer_bytes = prof_len * core::mem::size_of::<AtomicT>();
    prof_buffer = kzalloc(buffer_bytes, 0);
    if !prof_buffer.is_null() { return 0; }
    prof_buffer = alloc_pages_exact(buffer_bytes, 0);
    if !prof_buffer.is_null() { return 0; }
    prof_buffer = vzalloc(buffer_bytes);
    if !prof_buffer.is_null() { return 0; }
    -12
}

unsafe fn do_profile_hits(_type_: c_int, pc_: *mut c_void, nr_hits: u32) {
    let pc = ((pc_ as usize).wrapping_sub(&_stext as *const u8 as usize)) >> prof_shift;
    if pc < prof_len { atomic_add(nr_hits, prof_buffer.add(pc)); }
}

#[no_mangle]
pub unsafe extern "C" fn profile_hits(type_: c_int, pc: *mut c_void, nr_hits: u32) {
    if prof_on != type_ || prof_buffer.is_null() { return; }
    do_profile_hits(type_, pc, nr_hits);
}

#[no_mangle]
pub unsafe extern "C" fn profile_tick(type_: c_int) {
    let regs = get_irq_regs();
    if user_mode(regs) == 0 { profile_hit(type_, profile_pc(regs)); }
}

// The following /proc/profile implementation is conditional on CONFIG_PROC_FS.
#[cfg(feature = "CONFIG_PROC_FS")]
mod proc_profile {
    use super::*;
    pub unsafe extern "C" fn setup_profiling_timer(_mult: u32) -> c_int { -22 }
    pub unsafe extern "C" fn write_profile(buf: *const c_char, count: SizeT) -> SsizeT {
        // CONFIG_SMP conditional: a four-byte write changes the profiling timer.
        if count == core::mem::size_of::<c_int>() {
            let mut multiplier = 0u32;
            if copy_from_user(&mut multiplier, buf, core::mem::size_of::<c_int>()) != 0 { return -14; }
            if setup_profiling_timer(multiplier) != 0 { return -22; }
        }
        memset(prof_buffer as *mut c_void, 0, prof_len * core::mem::size_of::<AtomicT>());
        count as SsizeT
    }
    pub unsafe extern "C" fn read_profile(buf: *mut c_char, count: SizeT, ppos: *mut LoffT) -> SsizeT {
        let mut p = *ppos as usize;
        let total = (prof_len + 1) * core::mem::size_of::<u32>();
        if p >= total { return 0; }
        let count = core::cmp::min(count, total - p);
        let sample_step = 1usize << prof_shift;
        let mut read = 0usize;
        while p < core::mem::size_of::<u32>() && read < count {
            let byte = *((&sample_step as *const usize as *const u8).add(p));
            if put_user(byte, buf.add(read) as *mut c_char) != 0 { return -14; }
            p += 1; read += 1;
        }
        let pnt = (prof_buffer as *mut u8).sub(core::mem::size_of::<AtomicT>()).add(p);
        if copy_to_user(buf.add(read), pnt as *const c_void, count - read) != 0 { return -14; }
        read += count - read;
        *ppos += read as LoffT;
        read as SsizeT
    }
    pub unsafe extern "C" fn create_proc_profile() -> c_int { if prof_on == 0 { 0 } else { 0 } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
