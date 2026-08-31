// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
// C dependencies:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>, "bpf_misc.h",
// "bpf_experimental.h", <bpf_arena_common.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;

type u64 = u64;

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_ARENA: u32 = 33;
const BPF_F_MMAPABLE: u32 = 1024;
const BPF_MAX_LOOPS: i32 = 8_388_608;
const BPF_STDOUT: bpf_stream_id = 1;
const BPF_STDERR: bpf_stream_id = 2;
const ENOSPC: i32 = 28;
const _STR: &[u8] = b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\0";

#[repr(C)]
pub struct bpf_res_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_arena {
    pub user_vm_start: u64,
}

#[repr(C)]
pub struct bpf_insn {
    pub code: u8,
    pub dst_src_reg: u8,
    pub off: i16,
    pub imm: i32,
}

impl bpf_insn {
    pub const fn new(code: u8, dst_reg: u8, src_reg: u8, off: i16, imm: i32) -> Self {
        Self {
            code,
            dst_src_reg: (dst_reg & 0x0f) | ((src_reg & 0x0f) << 4),
            off,
            imm,
        }
    }
}

#[repr(C)]
pub struct arr_elem {
    pub lock: bpf_res_spin_lock,
}

#[repr(C)]
pub struct elem {
    pub timer: bpf_timer,
}

#[repr(C)]
pub struct arrmap_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[repr(C)]
pub struct arena_def {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
}

#[repr(C)]
pub struct array_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut arrmap: arrmap_def = arrmap_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<arr_elem>() as u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut arena: arena_def = arena_def {
    type_: BPF_MAP_TYPE_ARENA,
    map_flags: BPF_F_MMAPABLE,
    max_entries: 1, /* number of pages */
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut array: array_def = array_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<elem>() as u32,
};

#[no_mangle]
pub static mut size: i32 = 0;
#[no_mangle]
pub static mut fault_addr: u64 = 0;
#[no_mangle]
pub static mut arena_ptr: *mut c_void = core::ptr::null_mut();

// private(STREAM) struct bpf_spin_lock block;
#[no_mangle]
#[link_section = ".bss.STREAM"]
pub static mut block: bpf_spin_lock = bpf_spin_lock { _private: [] };

pub type bpf_stream_id = u32;

extern "C" {
    static mut can_loop: i32;

    fn bpf_stream_printk(stream_id: bpf_stream_id, fmt: *const u8) -> i32;
    fn bpf_stream_print_stack(stream_id: bpf_stream_id) -> i32;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_res_spin_lock(lock: *mut bpf_res_spin_lock) -> i32;
    fn bpf_res_spin_unlock(lock: *mut bpf_res_spin_lock);
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_addr_space_cast(addr: u64, off: i32, as_id: i32) -> *mut c_void;
    fn bpf_timer_init(timer: *mut bpf_timer, map: *mut c_void, clockid: u64) -> i32;
    fn bpf_timer_set_callback(
        timer: *mut bpf_timer,
        callback: unsafe extern "C" fn(*mut c_void, *mut i32, *mut bpf_timer) -> i32,
    ) -> i32;
    fn bpf_timer_start(timer: *mut bpf_timer, nsecs: u64, flags: u64) -> i32;
}

#[inline(always)]
unsafe fn barrier_var<T>(_: *mut T) {}

// SEC("syscall")
// __success __retval(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_exhaust(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    /* Use global variable for loop convergence. */
    size = 0;
    for _ in 0..BPF_MAX_LOOPS {
        if bpf_stream_printk(BPF_STDOUT, _STR.as_ptr()) == -ENOSPC && size == 99954 {
            return 0;
        }
        size += core::mem::size_of_val(_STR) as i32 - 1;
    }
    1
}

// SEC("syscall")
// __arch_x86_64 __arch_arm64 __arch_s390x __arch_riscv64 __arch_loongarch
// __success __retval(0)
// __stderr("ERROR: Timeout detected for may_goto instruction")
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_cond_break(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    while core::ptr::read_volatile(core::ptr::addr_of!(can_loop)) != 0 {}
    0
}

// SEC("syscall")
// __success __retval(0)
// __stderr("ERROR: AA or ABBA deadlock detected for bpf_res_spin_lock")
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_deadlock(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut lock: *mut bpf_res_spin_lock;
    let mut nlock: *mut bpf_res_spin_lock;
    let key: i32 = 0;

    lock = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(arrmap).cast::<c_void>(),
        core::ptr::addr_of!(key).cast::<c_void>(),
    )
    .cast::<bpf_res_spin_lock>();
    if lock.is_null() {
        return 1;
    }
    nlock = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(arrmap).cast::<c_void>(),
        core::ptr::addr_of!(key).cast::<c_void>(),
    )
    .cast::<bpf_res_spin_lock>();
    if nlock.is_null() {
        return 1;
    }
    if bpf_res_spin_lock(lock) != 0 {
        return 1;
    }
    if bpf_res_spin_lock(nlock) != 0 {
        bpf_res_spin_unlock(lock);
        return 0;
    }
    bpf_res_spin_unlock(nlock);
    bpf_res_spin_unlock(lock);
    1
}

// SEC("syscall")
// __success __retval(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_syscall(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    bpf_stream_printk(BPF_STDOUT, b"foo\0".as_ptr());
    0
}

// SEC("syscall")
// __arch_x86_64 __arch_arm64
// __success __retval(0)
// __stderr("ERROR: Arena WRITE access at unmapped address 0x{{.*}}")
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_arena_write_fault(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut ptr: *mut bpf_arena = core::ptr::addr_of_mut!(arena).cast::<bpf_arena>();
    let user_vm_start: u64;

    /* Prevent GCC bounds warning: casting &arena to struct bpf_arena *
     * triggers bounds checking since the map definition is smaller than struct
     * bpf_arena. barrier_var() makes the pointer opaque to GCC, preventing the
     * bounds analysis
     */
    barrier_var(ptr);
    user_vm_start = (*ptr).user_vm_start;
    fault_addr = user_vm_start + 0x7fff;
    bpf_addr_space_cast(user_vm_start, 0, 1);
    asm!(
        "r1 = {0}",
        "r2 = 1",
        "*(u32 *)(r1 + 0x7fff) = r2",
        in(reg) user_vm_start,
        lateout("r1") _,
        lateout("r2") _,
    );
    0
}

// SEC("syscall")
// __arch_x86_64 __arch_arm64
// __success __retval(0)
// __stderr("ERROR: Arena READ access at unmapped address 0x{{.*}}")
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_arena_read_fault(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut ptr: *mut bpf_arena = core::ptr::addr_of_mut!(arena).cast::<bpf_arena>();
    let user_vm_start: u64;

    /* Prevent GCC bounds warning: casting &arena to struct bpf_arena *
     * triggers bounds checking since the map definition is smaller than struct
     * bpf_arena. barrier_var() makes the pointer opaque to GCC, preventing the
     * bounds analysis
     */
    barrier_var(ptr);
    user_vm_start = (*ptr).user_vm_start;
    fault_addr = user_vm_start + 0x7fff;
    bpf_addr_space_cast(user_vm_start, 0, 1);
    asm!(
        "r1 = {0}",
        "r1 = *(u32 *)(r1 + 0x7fff)",
        in(reg) user_vm_start,
        lateout("r1") _,
    );
    0
}

// SEC("syscall")
// __arch_x86_64 __arch_arm64
// __success __retval(0)
// __stderr("ERROR: Arena READ access at unmapped address 0x{{.*}}")
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_arena_load_acquire_fault(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    static load_acquire_insn: bpf_insn = bpf_insn::new(
        0xc3,   /* BPF_STX | BPF_ATOMIC | BPF_W */
        0,      /* BPF_REG_0 */
        1,      /* BPF_REG_1 */
        0x7fff,
        0x100,  /* BPF_LOAD_ACQ */
    );
    let mut ptr: *mut bpf_arena = core::ptr::addr_of_mut!(arena).cast::<bpf_arena>();
    let user_vm_start: u64;
    let val: u64;

    /*
     * Prevent GCC bounds warning: casting &arena to struct bpf_arena *
     * triggers bounds checking since the map definition is smaller than
     * struct bpf_arena. barrier_var() makes the pointer opaque to GCC,
     * preventing the bounds analysis.
     */
    barrier_var(ptr);
    user_vm_start = (*ptr).user_vm_start;
    fault_addr = user_vm_start + 0x7fff;
    bpf_addr_space_cast(user_vm_start, 0, 1);
    asm!(
        "r1 = {user_vm_start}",
        "r0 = 1",
        ".8byte {load_acquire_insn}", /* r0 = load_acquire((u32 *)(r1 + 0x7fff)) */
        "{val} = r0",
        user_vm_start = in(reg) user_vm_start,
        load_acquire_insn = const unsafe { core::mem::transmute::<bpf_insn, u64>(load_acquire_insn) },
        val = lateout(reg) val,
        lateout("r0") _,
        lateout("r1") _,
    );
    val as i32
}

// SEC("syscall")
// __arch_x86_64 __arch_arm64
// __success __retval(0)
// __stderr("ERROR: Arena WRITE access at unmapped address 0x{{.*}}")
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_arena_xchg_fault(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    static xchg_insn: bpf_insn = bpf_insn::new(
        0xc3,   /* BPF_STX | BPF_ATOMIC | BPF_W */
        1,      /* BPF_REG_1 */
        2,      /* BPF_REG_2 */
        0x7fff,
        0xe1,   /* BPF_XCHG */
    );
    let mut ptr: *mut bpf_arena = core::ptr::addr_of_mut!(arena).cast::<bpf_arena>();
    let user_vm_start: u64;
    let val: u64;

    /*
     * Prevent GCC bounds warning: casting &arena to struct bpf_arena *
     * triggers bounds checking since the map definition is smaller than
     * struct bpf_arena. barrier_var() makes the pointer opaque to GCC,
     * preventing the bounds analysis.
     */
    barrier_var(ptr);
    user_vm_start = (*ptr).user_vm_start;
    fault_addr = user_vm_start + 0x7fff;
    bpf_addr_space_cast(user_vm_start, 0, 1);
    /*
     * A read-modify-write carrying BPF_FETCH writes to memory, so the fault
     * has to be reported as a WRITE from the dst_reg address, but it also
     * reads the old value into src_reg, so the exception handler has to
     * clear src_reg. Poison it up front, the returned value must be 0.
     */
    asm!(
        "r1 = {user_vm_start}",
        "r2 = 1",
        ".8byte {xchg_insn}", /* r2 = xchg((u32 *)(r1 + 0x7fff), r2) */
        "{val} = r2",
        user_vm_start = in(reg) user_vm_start,
        xchg_insn = const unsafe { core::mem::transmute::<bpf_insn, u64>(xchg_insn) },
        val = lateout(reg) val,
        lateout("r1") _,
        lateout("r2") _,
    );
    val as i32
}

// SEC("syscall")
// __arch_x86_64 __arch_arm64
// __success __retval(0)
// __stderr("ERROR: Arena WRITE access at unmapped address 0x{{.*}}")
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_arena_cmpxchg_fault(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    static cmpxchg_insn: bpf_insn = bpf_insn::new(
        0xc3,   /* BPF_STX | BPF_ATOMIC | BPF_W */
        1,      /* BPF_REG_1 */
        2,      /* BPF_REG_2 */
        0x7fff,
        0xf1,   /* BPF_CMPXCHG */
    );
    let mut ptr: *mut bpf_arena = core::ptr::addr_of_mut!(arena).cast::<bpf_arena>();
    let user_vm_start: u64;
    let val: u64;

    /*
     * Prevent GCC bounds warning: casting &arena to struct bpf_arena *
     * triggers bounds checking since the map definition is smaller than
     * struct bpf_arena. barrier_var() makes the pointer opaque to GCC,
     * preventing the bounds analysis.
     */
    barrier_var(ptr);
    user_vm_start = (*ptr).user_vm_start;
    fault_addr = user_vm_start + 0x7fff;
    bpf_addr_space_cast(user_vm_start, 0, 1);
    /*
     * Same as the exchange above, except that a BPF_CMPXCHG reads the old
     * value into r0 rather than into src_reg, so r0 is the register the
     * exception handler has to clear. It doubles as the compare value, but
     * the comparison never happens since the access faults first.
     */
    asm!(
        "r1 = {user_vm_start}",
        "r0 = 1",
        "r2 = 2",
        ".8byte {cmpxchg_insn}", /* r0 = cmpxchg((u32 *)(r1 + 0x7fff), r0, r2) */
        "{val} = r0",
        user_vm_start = in(reg) user_vm_start,
        cmpxchg_insn = const unsafe { core::mem::transmute::<bpf_insn, u64>(cmpxchg_insn) },
        val = lateout(reg) val,
        lateout("r0") _,
        lateout("r1") _,
        lateout("r2") _,
    );
    val as i32
}

#[inline(never)]
unsafe extern "C" fn subprog() {
    let addr: *mut i32 = 0xdeadbeefu64 as *mut i32;

    arena_ptr = core::ptr::addr_of_mut!(arena).cast::<c_void>();
    *addr = 1;
}

// SEC("syscall")
// __arch_x86_64 __arch_arm64
// __success __retval(0)
// __stderr("ERROR: Arena WRITE access at unmapped address 0x{{.*}}")
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_arena_subprog_fault(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    subprog();
    0
}

#[inline(never)]
unsafe extern "C" fn timer_cb(map: *mut c_void, key: *mut i32, timer: *mut bpf_timer) -> i32 {
    let _ = map;
    let _ = key;
    let _ = timer;
    let addr: *mut i32 = 0xdeadbeefu64 as *mut i32;

    arena_ptr = core::ptr::addr_of_mut!(arena).cast::<c_void>();
    *addr = 1;
    0
}

// SEC("syscall")
// __arch_x86_64 __arch_arm64
// __success __retval(0)
// __stderr("ERROR: Arena WRITE access at unmapped address 0x{{.*}}")
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_arena_callback_fault(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let arr_timer: *mut bpf_timer;
    let key: i32 = 0;

    arr_timer = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(array).cast::<c_void>(),
        core::ptr::addr_of!(key).cast::<c_void>(),
    )
    .cast::<bpf_timer>();
    if arr_timer.is_null() {
        return 0;
    }
    bpf_timer_init(arr_timer, core::ptr::addr_of_mut!(array).cast::<c_void>(), 1);
    bpf_timer_set_callback(arr_timer, timer_cb);
    bpf_timer_start(arr_timer, 0, 0);
    0
}

// SEC("syscall")
// __arch_x86_64 __arch_arm64
// __success __retval(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_print_stack_kfunc(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    bpf_stream_print_stack(BPF_STDERR)
}

// SEC("syscall")
// __success __retval(-2)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_print_stack_invalid_id(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    /* Try to pass an invalid stream ID. */
    bpf_stream_print_stack(0xbadcafe as bpf_stream_id)
}

// SEC("syscall")
// __arch_x86_64 __arch_arm64
// __success __retval(0)
// __stdout(_STR)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn stream_print_kfuncs_locked(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut ret: i32;

    bpf_spin_lock(core::ptr::addr_of_mut!(block));

    ret = bpf_stream_printk(BPF_STDOUT, _STR.as_ptr());
    if ret != 0 {
        bpf_spin_unlock(core::ptr::addr_of_mut!(block));
        return ret;
    }

    ret = bpf_stream_print_stack(BPF_STDERR);

    bpf_spin_unlock(core::ptr::addr_of_mut!(block));

    ret
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
