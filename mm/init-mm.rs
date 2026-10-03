// SPDX-License-Identifier: GPL-2.0
//! Rust-owned bootstrap memory descriptor and its initial address bounds.
//!
//! Native headers supply the complete configured layout. Bindgen's FAM
//! generic is specialized only for this static object's CPU/CID bitmap;
//! dynamically allocated mm_struct retains its ordinary unsized tail.

#![allow(non_upper_case_globals, unused_unsafe, unused_mut, unused_macros)]

#[cfg(not(any(CONFIG_X86_64, CONFIG_ARM64)))]
compile_error!("Rust initial mm supports x86_64 and arm64 architecture initializers");

#[allow(
    clippy::all, dead_code, missing_docs, non_camel_case_types, non_snake_case,
    non_upper_case_globals, improper_ctypes, unsafe_op_in_unsafe_fn, unreachable_pub
)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/init_mm_generated.rs"));
}
use bindings::*;
use core::mem::{align_of, offset_of, size_of, zeroed};

include!("init_mm_initializers.rs");

// MM_STRUCT_FLEXIBLE_ARRAY_INIT supplies precisely these zero bytes. The
// compiler-generated generic preserves the actual flexible-array offset,
// including configurations where mm_struct has cache-line tail padding.
type InitialMm = mm_struct<[kernel::ffi::c_char; RUST_INIT_MM_FLEX_BYTES as usize]>;

macro_rules! assert_layout {
    ($ty:ty, $size:ident, $align:ident) => {
        assert!(size_of::<$ty>() == $size as usize);
        assert!(align_of::<$ty>() == $align as usize);
    };
}
macro_rules! assert_mm_offset {
    ($field:ident, $offset:ident) => {
        assert!(offset_of!(mm_struct, __bindgen_anon_1.$field) == $offset as usize);
        assert!(offset_of!(InitialMm, __bindgen_anon_1.$field) == $offset as usize);
    };
}
const _: () = {
    assert_layout!(mm_struct, RUST_INIT_MM_BASE_SIZE, RUST_INIT_MM_BASE_ALIGN);
    assert_layout!(vm_operations_struct, RUST_INIT_MM_VM_OPS_SIZE, RUST_INIT_MM_VM_OPS_ALIGN);
    assert_layout!(raw_spinlock_t, RUST_INIT_MM_RAW_LOCK_SIZE, RUST_INIT_MM_RAW_LOCK_ALIGN);
    assert_layout!(spinlock_t, RUST_INIT_MM_SPIN_LOCK_SIZE, RUST_INIT_MM_SPIN_LOCK_ALIGN);
    assert_layout!(rw_semaphore, RUST_INIT_MM_RWSEM_SIZE, RUST_INIT_MM_RWSEM_ALIGN);
    assert_layout!(mutex, RUST_INIT_MM_MUTEX_SIZE, RUST_INIT_MM_MUTEX_ALIGN);
    assert_layout!(seqcount_t, RUST_INIT_MM_SEQCOUNT_SIZE, RUST_INIT_MM_SEQCOUNT_ALIGN);
    assert_layout!(maple_tree, RUST_INIT_MM_MAPLE_SIZE, RUST_INIT_MM_MAPLE_ALIGN);
    assert!(align_of::<InitialMm>() == align_of::<mm_struct>());
    assert!(offset_of!(mm_struct, flexible_array) == RUST_INIT_MM_flexible_array_OFFSET as usize);
    assert!(offset_of!(InitialMm, flexible_array) == RUST_INIT_MM_flexible_array_OFFSET as usize);
    assert!(size_of::<InitialMm>() >= RUST_INIT_MM_flexible_array_OFFSET as usize + RUST_INIT_MM_FLEX_BYTES as usize);
    assert!(offset_of!(mm_struct, __bindgen_anon_1.__bindgen_anon_1.mm_count) == RUST_INIT_MM_mm_count_OFFSET as usize);
    assert!(offset_of!(InitialMm, __bindgen_anon_1.__bindgen_anon_1.mm_count) == RUST_INIT_MM_mm_count_OFFSET as usize);
    assert_mm_offset!(mm_mt, RUST_INIT_MM_mm_mt_OFFSET);
    assert_mm_offset!(pgd, RUST_INIT_MM_pgd_OFFSET);
    assert_mm_offset!(mm_users, RUST_INIT_MM_mm_users_OFFSET);
    assert_mm_offset!(write_protect_seq, RUST_INIT_MM_write_protect_seq_OFFSET);
    assert_mm_offset!(mmap_lock, RUST_INIT_MM_mmap_lock_OFFSET);
    assert_mm_offset!(page_table_lock, RUST_INIT_MM_page_table_lock_OFFSET);
    assert_mm_offset!(arg_lock, RUST_INIT_MM_arg_lock_OFFSET);
    assert_mm_offset!(mmlist, RUST_INIT_MM_mmlist_OFFSET);
    assert_mm_offset!(context, RUST_INIT_MM_context_OFFSET);
    assert_mm_offset!(start_code, RUST_INIT_MM_start_code_OFFSET);
    assert_mm_offset!(end_code, RUST_INIT_MM_end_code_OFFSET);
    assert_mm_offset!(end_data, RUST_INIT_MM_end_data_OFFSET);
    assert_mm_offset!(brk, RUST_INIT_MM_brk_OFFSET);
    assert!(RUST_INIT_MM_MAPLE_LOCK_OFFSET == 0);
    assert!(size_of::<maple_tree>() >= size_of::<spinlock_t>());
    assert!(align_of::<maple_tree>() >= align_of::<spinlock_t>());
};
#[cfg(CONFIG_PER_VMA_LOCK)]
const _: () = {
    assert_mm_offset!(vma_writer_wait, RUST_INIT_MM_vma_writer_wait_OFFSET);
    assert_mm_offset!(mm_lock_seq, RUST_INIT_MM_mm_lock_seq_OFFSET);
};
#[cfg(CONFIG_SCHED_MM_CID)]
const _: () = { assert_mm_offset!(mm_cid, RUST_INIT_MM_mm_cid_OFFSET); };
#[cfg(not(CONFIG_PREEMPT_RT))]
const _: () = {
    assert!(RUST_INIT_MM_SPIN_RAW_OFFSET == 0);
    assert!(size_of::<spinlock_t>() == size_of::<raw_spinlock_t>());
    assert!(align_of::<spinlock_t>() == align_of::<raw_spinlock_t>());
};

// SAFETY: This local binding contains function pointers and, with USERFAULTFD,
// a pointer to immutable operations. Reading it cannot mutate their targets;
// invoking a callback or dereferencing a pointer still requires unsafe code.
unsafe impl Sync for vm_operations_struct {}

/// Empty immutable VMA operations table, as in the original C definition.
#[no_mangle]
pub static vma_dummy_vm_ops: vm_operations_struct = unsafe { zeroed() };

/// Initial address space, with static storage for the full NR_CPUS bitmap
/// and the optional two SCHED_MM_CID bitmaps.
#[no_mangle]
pub static mut init_mm: InitialMm = {
    // SAFETY: Native C designated initialization zero-fills unnamed fields.
    // Generated C scalars, pointers, nullable callbacks and bitfields all
    // admit those zero values. This also initializes every bitmap byte.
    let mut mm: InitialMm = unsafe { zeroed() };
    let m = &mut mm.__bindgen_anon_1;
    // MTREE_INIT_EXT(mm_mt, MM_MT_FLAGS, init_mm.mmap_lock)
    #[cfg(CONFIG_LOCKDEP)]
    {
        m.mm_mt.__bindgen_anon_1.ma_external_lock =
            unsafe { &raw mut init_mm.__bindgen_anon_1.mmap_lock.dep_map };
    }
    #[cfg(not(CONFIG_LOCKDEP))]
    {
        // Native MTREE_INIT stringifies (mm_mt).ma_lock, including parens.
        let lock = spin_lock!("(mm_mt).ma_lock");
        unsafe { (&raw mut m.mm_mt).cast::<spinlock_t>().write(lock) };
    }
    m.mm_mt.ma_flags = RUST_INIT_MM_MT_FLAGS as _;
    #[cfg(CONFIG_X86_64)]
    { m.pgd = unsafe { (&raw mut init_top_pgt).cast() }; }
    #[cfg(CONFIG_ARM64)]
    { m.pgd = unsafe { (&raw mut swapper_pg_dir).cast() }; }
    m.mm_users.counter = 2;
    m.__bindgen_anon_1.mm_count.counter = 1;
    m.write_protect_seq = seqcount!("init_mm.write_protect_seq");
    // MMAP_LOCK_INITIALIZER passes (init_mm).mmap_lock, with parentheses.
    m.mmap_lock = rwsem!(init_mm.__bindgen_anon_1.mmap_lock, "(init_mm).mmap_lock");
    m.page_table_lock = spin_lock!("init_mm.page_table_lock");
    m.arg_lock = spin_lock!("init_mm.arg_lock");
    let list = unsafe { &raw mut init_mm.__bindgen_anon_1.mmlist };
    m.mmlist = list_head { next: list, prev: list };
    #[cfg(CONFIG_PER_VMA_LOCK)]
    {
        // __RCUWAIT_INITIALIZER has exactly one member: task = NULL.
        m.vma_writer_wait.task = core::ptr::null_mut();
        m.mm_lock_seq = seqcount!("init_mm.mm_lock_seq");
    }
    #[cfg(CONFIG_SCHED_MM_CID)]
    { m.mm_cid.lock = raw_spin_lock!("init_mm.mm_cid.lock"); }
    #[cfg(CONFIG_X86_64)]
    {
        // asm/x86/mmu.h INIT_MM_CONTEXT; arm64's macro is absent/empty.
        m.context.ctx_id = 1;
        m.context.lock = mutex!(init_mm.__bindgen_anon_1.context.lock, "init_mm.context.lock");
    }
    mm
};

/// Set the initial kernel image and break bounds during architecture setup.
///
/// # Safety
/// The caller must serialize these writes against all accesses to init_mm,
/// as architecture startup does before concurrent users can observe it.
#[no_mangle]
pub unsafe extern "C" fn setup_initial_init_mm(
    start_code: *mut kernel::ffi::c_void,
    end_code: *mut kernel::ffi::c_void,
    end_data: *mut kernel::ffi::c_void,
    brk: *mut kernel::ffi::c_void,
) {
    // SAFETY: Early architecture initialization provides exclusive access.
    unsafe {
        init_mm.__bindgen_anon_1.start_code = start_code as kernel::ffi::c_ulong;
        init_mm.__bindgen_anon_1.end_code = end_code as kernel::ffi::c_ulong;
        init_mm.__bindgen_anon_1.end_data = end_data as kernel::ffi::c_ulong;
        init_mm.__bindgen_anon_1.brk = brk as kernel::ffi::c_ulong;
    }
}
