/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers are intentionally
// left as external Rust names.

/* Arrange for legacy / undefined architecture-specific flags to be ignored. */
pub const MAP_32BIT: _ = 0;
pub const MAP_ABOVE4G: _ = 0;
pub const MAP_HUGE_2MB: _ = 0;
pub const MAP_HUGE_1GB: _ = 0;
pub const MAP_UNINITIALIZED: _ = 0;
pub const MAP_SYNC: _ = 0;

/*
 * The historical set of flags that all mmap implementations implicitly
 * support when a ->mmap_validate() op is not provided in file_operations.
 * MAP_EXECUTABLE and MAP_DENYWRITE are completely ignored throughout the
 * kernel.
 */
pub const LEGACY_MAP_MASK: _ = MAP_SHARED
    | MAP_PRIVATE
    | MAP_FIXED
    | MAP_ANONYMOUS
    | MAP_DENYWRITE
    | MAP_EXECUTABLE
    | MAP_UNINITIALIZED
    | MAP_GROWSDOWN
    | MAP_LOCKED
    | MAP_NORESERVE
    | MAP_POPULATE
    | MAP_NONBLOCK
    | MAP_STACK
    | MAP_HUGETLB
    | MAP_32BIT
    | MAP_ABOVE4G
    | MAP_HUGE_2MB
    | MAP_HUGE_1GB;

unsafe extern "C" {
    pub static mut sysctl_overcommit_memory: i32;
    pub static mut vm_committed_as: percpu_counter;

    // CONFIG_SMP: these declarations are present only in SMP builds.
    pub static mut vm_committed_as_batch: i32;
    pub fn mm_compute_batch(overcommit_policy: i32);

    pub fn vm_memory_committed() -> libc::c_ulong;
    pub fn percpu_counter_add_batch(
        fbc: *mut percpu_counter,
        amount: libc::c_long,
        batch: i32,
    );
}

// CONFIG_SMP disabled: vm_committed_as_batch is the constant zero and
// mm_compute_batch is an empty inline function.
#[inline]
pub unsafe fn mm_compute_batch_no_smp(_overcommit_policy: i32) {}

#[inline]
pub unsafe fn vm_acct_memory(pages: libc::c_long) {
    percpu_counter_add_batch(
        core::ptr::addr_of_mut!(vm_committed_as),
        pages,
        vm_committed_as_batch,
    );
}

#[inline]
pub unsafe fn vm_unacct_memory(pages: libc::c_long) {
    vm_acct_memory(pages.wrapping_neg());
}

/* Architecture-specific overrides may replace these definitions. */
#[inline]
pub const fn arch_calc_vm_prot_bits(_prot: libc::c_ulong, _pkey: libc::c_ulong) -> libc::c_ulong {
    0
}

#[inline]
pub const fn arch_calc_vm_flag_bits(_file: *mut file, _flags: libc::c_ulong) -> libc::c_ulong {
    0
}

/* PROT_GROWSDOWN and PROT_GROWSUP have already been masked out. */
#[inline]
pub const fn arch_validate_prot(prot: libc::c_ulong, _addr: libc::c_ulong) -> bool {
    (prot & !(PROT_READ | PROT_WRITE | PROT_EXEC | PROT_SEM)) == 0
}

#[inline]
pub const fn arch_validate_flags(_flags: libc::c_ulong) -> bool {
    true
}

/* Equivalent to: (x & bit1) ? bit2 : 0; bit1 and bit2 must be single bits. */
#[inline]
pub const fn _calc_vm_trans(x: libc::c_ulong, bit1: libc::c_ulong, bit2: libc::c_ulong) -> libc::c_ulong {
    if bit1 == 0 || bit2 == 0 {
        0
    } else if bit1 <= bit2 {
        (x & bit1).wrapping_mul(bit2 / bit1)
    } else {
        (x & bit1) / (bit1 / bit2)
    }
}

#[inline]
pub fn calc_vm_prot_bits(prot: libc::c_ulong, pkey: libc::c_ulong) -> vm_flags_t {
    (_calc_vm_trans(prot, PROT_READ, VM_READ)
        | _calc_vm_trans(prot, PROT_WRITE, VM_WRITE)
        | _calc_vm_trans(prot, PROT_EXEC, VM_EXEC)
        | arch_calc_vm_prot_bits(prot, pkey)) as vm_flags_t
}

#[inline]
pub unsafe fn calc_vm_flag_bits(file: *mut file, flags: libc::c_ulong) -> vm_flags_t {
    (_calc_vm_trans(flags, MAP_GROWSDOWN, VM_GROWSDOWN)
        | _calc_vm_trans(flags, MAP_LOCKED, VM_LOCKED)
        | _calc_vm_trans(flags, MAP_SYNC, VM_SYNC)
        // CONFIG_TRANSPARENT_HUGEPAGE: also include MAP_STACK -> VM_NOHUGEPAGE.
        | _calc_vm_trans(flags, MAP_STACK, VM_NOHUGEPAGE)
        | arch_calc_vm_flag_bits(file, flags)) as vm_flags_t
}

unsafe extern "C" {
    pub fn vm_commit_limit() -> libc::c_ulong;
}

#[inline]
pub const fn arch_memory_deny_write_exec_supported() -> bool {
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
