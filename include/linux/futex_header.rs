/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/sched.h, linux/ktime.h, linux/mm_types.h, uapi/linux/futex.h

use core::ffi::c_int;

// struct inode;
// struct task_struct;

pub const FUT_OFF_INODE: u32 = 1; // We set bit 0 if key has a reference on inode
pub const FUT_OFF_MMSHARED: u32 = 2; // We set bit 1 if key has a reference on mm

#[repr(C)]
pub struct FutexKeyShared {
    pub i_seq: u64,
    pub pgoff: usize,
    pub offset: u32,
}

#[repr(C)]
pub union FutexKeyPrivateMm {
    pub mm: *mut MmStruct,
    pub __tmp: u64,
}

#[repr(C)]
pub struct FutexKeyPrivate {
    pub mm: FutexKeyPrivateMm,
    pub address: usize,
    pub offset: u32,
}

#[repr(C)]
pub struct FutexKeyBoth {
    pub ptr: u64,
    pub word: usize,
    pub offset: u32,
    pub node: u32, // NOT hashed!
}

#[repr(C)]
pub union FutexKey {
    pub shared: FutexKeyShared,
    pub private: FutexKeyPrivate,
    pub both: FutexKeyBoth,
}

pub const FUTEX_KEY_INIT: FutexKey = FutexKey {
    both: FutexKeyBoth {
        ptr: 0,
        word: 0,
        offset: 0,
        node: 0,
    },
};

// The following declarations are enabled by CONFIG_FUTEX in the kernel.
#[cfg(feature = "CONFIG_FUTEX")]
pub const FUTEX_STATE_OK: c_int = 0;
#[cfg(feature = "CONFIG_FUTEX")]
pub const FUTEX_STATE_EXITING: c_int = 1;
#[cfg(feature = "CONFIG_FUTEX")]
pub const FUTEX_STATE_DEAD: c_int = 2;

#[cfg(feature = "CONFIG_FUTEX")]
pub unsafe fn futex_init_task(tsk: *mut TaskStruct) {
    memset(
        core::ptr::addr_of_mut!((*tsk).futex) as *mut u8,
        0,
        core::mem::size_of_val(&(*tsk).futex),
    );
    init_list_head(core::ptr::addr_of_mut!((*tsk).futex.pi_state_list));
    (*tsk).futex.state = FUTEX_STATE_OK;
    mutex_init(core::ptr::addr_of_mut!((*tsk).futex.exit_mutex));
}

extern "C" {
    pub fn futex_exit_recursive(tsk: *mut TaskStruct);
    pub fn futex_exit_exec_release(tsk: *mut TaskStruct);
    pub fn futex_exec_done(tsk: *mut TaskStruct);
    pub fn do_futex(
        uaddr: *mut u32,
        op: c_int,
        val: u32,
        timeout: *mut KtimeT,
        uaddr2: *mut u32,
        val2: u32,
        val3: u32,
    ) -> isize;
    pub fn futex_hash_prctl(arg2: usize, arg3: usize, arg4: usize) -> c_int;
}

#[cfg(feature = "CONFIG_FUTEX_PRIVATE_HASH")]
extern "C" {
    pub fn futex_hash_allocate_default() -> c_int;
    pub fn futex_hash_free(mm: *mut MmStruct) -> c_int;
}

#[cfg(not(feature = "CONFIG_FUTEX_PRIVATE_HASH"))]
pub unsafe fn futex_hash_allocate_default() -> c_int { 0 }
#[cfg(not(feature = "CONFIG_FUTEX_PRIVATE_HASH"))]
pub unsafe fn futex_hash_free(_mm: *mut MmStruct) -> c_int { 0 }

#[cfg(not(feature = "CONFIG_FUTEX"))]
pub unsafe fn futex_init_task(_tsk: *mut TaskStruct) {}
#[cfg(not(feature = "CONFIG_FUTEX"))]
pub unsafe fn futex_exit_recursive(_tsk: *mut TaskStruct) {}
#[cfg(not(feature = "CONFIG_FUTEX"))]
pub unsafe fn futex_exit_exec_release(_tsk: *mut TaskStruct) {}
#[cfg(not(feature = "CONFIG_FUTEX"))]
pub unsafe fn futex_exec_done(_tsk: *mut TaskStruct) {}
#[cfg(not(feature = "CONFIG_FUTEX"))]
pub unsafe fn do_futex(
    _uaddr: *mut u32, _op: c_int, _val: u32, _timeout: *mut KtimeT,
    _uaddr2: *mut u32, _val2: u32, _val3: u32,
) -> isize { -EINVAL as isize }
#[cfg(not(feature = "CONFIG_FUTEX"))]
pub unsafe fn futex_hash_prctl(_arg2: usize, _arg3: usize, _arg4: usize) -> c_int { -EINVAL }
#[cfg(not(feature = "CONFIG_FUTEX"))]
pub unsafe fn futex_hash_allocate_default() -> c_int { 0 }
#[cfg(not(feature = "CONFIG_FUTEX"))]
pub unsafe fn futex_hash_free(_mm: *mut MmStruct) -> c_int { 0 }

#[cfg(feature = "CONFIG_FUTEX_ROBUST_UNLOCK")]
extern "C" {
    pub fn futex_reset_cs_ranges(fd: *mut FutexMmData);
    pub fn __futex_fixup_robust_unlock(regs: *mut PtRegs, csr: *mut FutexUnlockCsRange);
}

#[cfg(feature = "CONFIG_FUTEX_ROBUST_UNLOCK")]
pub unsafe fn futex_within_robust_unlock(
    regs: *mut PtRegs,
    csr: *mut FutexUnlockCsRange,
) -> bool {
    let ip = instruction_pointer(regs);
    ip >= (*csr).start_ip && ip < (*csr).start_ip + (*csr).len
}

#[cfg(feature = "CONFIG_FUTEX_ROBUST_UNLOCK")]
pub unsafe fn futex_fixup_robust_unlock(regs: *mut PtRegs) {
    if !(*current).rseq.event.user_irq {
        return;
    }
    let mut csr = (*(*current).mm).futex.unlock.cs_ranges;
    for _r in 0..FUTEX_ROBUST_MAX_CS_RANGES {
        if futex_within_robust_unlock(regs, csr) {
            __futex_fixup_robust_unlock(regs, csr);
            return;
        }
        csr = csr.add(1);
    }
}

#[cfg(feature = "CONFIG_FUTEX_ROBUST_UNLOCK")]
pub unsafe fn futex_set_vdso_cs_range(
    fd: *mut FutexMmData, idx: u32, start: usize, end: usize, sz32: bool,
) {
    (*fd).unlock.cs_ranges.add(idx as usize).write(FutexUnlockCsRange {
        start_ip: start,
        len: end.wrapping_sub(start),
        pop_size32: sz32,
    });
}

#[cfg(not(feature = "CONFIG_FUTEX_ROBUST_UNLOCK"))]
pub unsafe fn futex_fixup_robust_unlock(_regs: *mut PtRegs) {}

#[cfg(any(feature = "CONFIG_FUTEX_PRIVATE_HASH", feature = "CONFIG_FUTEX_ROBUST_UNLOCK"))]
extern "C" { pub fn futex_mm_init(mm: *mut MmStruct); }
#[cfg(not(any(feature = "CONFIG_FUTEX_PRIVATE_HASH", feature = "CONFIG_FUTEX_ROBUST_UNLOCK")))]
pub unsafe fn futex_mm_init(_mm: *mut MmStruct) {}

// External types, constants, globals, and helpers are supplied by included kernel headers.
extern "C" {
    static mut current: *mut TaskStruct;
    fn memset(dest: *mut u8, value: c_int, count: usize) -> *mut u8;
    fn init_list_head(list: *mut core::ffi::c_void);
    fn mutex_init(lock: *mut core::ffi::c_void);
    fn instruction_pointer(regs: *mut PtRegs) -> usize;
}

// Opaque declarations corresponding to included kernel types.
pub enum MmStruct {}
pub enum TaskStruct {}
pub enum KtimeT {}
pub enum PtRegs {}
pub enum FutexMmData {}
pub enum FutexUnlockCsRange {}
extern "C" { static EINVAL: c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
