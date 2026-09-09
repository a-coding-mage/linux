/* SPDX-License-Identifier: GPL-2.0-only */

/* C header dependencies: linux/kernel.h, linux/jump_label.h,
 * linux/percpu-defs.h, and linux/prandom.h. */

/* The following declarations are supplied by the surrounding kernel. */
#[repr(C)]
pub struct RndState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct StaticKeyMaybe {
    _private: [u8; 0],
}

extern "C" {
    pub static mut randomize_kstack_offset: StaticKeyMaybe;
    pub static mut kstack_rnd_state: RndState;
    pub fn prandom_u32_state(state: *mut RndState) -> u32;
    pub fn static_branch_maybe(key: bool, branch: *const StaticKeyMaybe) -> bool;
    pub fn lockdep_assert_irqs_disabled();
    pub fn raw_cpu_ptr(state: *mut RndState) -> *mut RndState;
    pub fn get_cpu_var(state: *mut RndState) -> *mut RndState;
    pub fn put_cpu_var(state: *mut RndState);
    /* Compiler builtin used by __kstack_alloca; provided by the build target. */
    pub fn __kstack_alloca(size: usize) -> *mut u8;
}

/* DECLARE_STATIC_KEY_MAYBE(CONFIG_RANDOMIZE_KSTACK_OFFSET_DEFAULT,
 *                          randomize_kstack_offset); */
pub const CONFIG_RANDOMIZE_KSTACK_OFFSET_DEFAULT: bool = false;

/* CONFIG_RANDOMIZE_KSTACK_OFFSET */

#[inline(always)]
pub unsafe fn get_kstack_offset() -> u32 {
    let state: *mut RndState = get_cpu_var(&raw mut kstack_rnd_state);
    let rnd: u32 = prandom_u32_state(state);
    put_cpu_var(&raw mut kstack_rnd_state);
    rnd
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub const fn kstack_offset_max(x: u32) -> u32 {
    x & 0b1111110000
}

#[cfg(not(target_pointer_width = "64"))]
#[inline(always)]
pub const fn kstack_offset_max(x: u32) -> u32 {
    x & 0b1111111100
}

/**
 * add_random_kstack_offset - Increase stack utilization by a random offset.
 *
 * This should be used in the syscall entry path after user registers have been
 * stored to the stack. Preemption may be enabled. For testing the resulting
 * entropy, please see: tools/testing/selftests/lkdtm/stack-entropy.sh
 */
#[macro_export]
macro_rules! add_random_kstack_offset {
    () => {{
        unsafe {
            if $crate::static_branch_maybe(
                CONFIG_RANDOMIZE_KSTACK_OFFSET_DEFAULT,
                &$crate::randomize_kstack_offset,
            ) {
                let offset: u32 = $crate::get_kstack_offset();
                let ptr: *mut u8 = $crate::__kstack_alloca(
                    $crate::kstack_offset_max(offset) as usize,
                );
                /* Keep allocation even after "ptr" loses scope. */
                core::arch::asm!("", in("reg") ptr, options(nostack, preserves_flags));
            }
        }
    }};
}

/**
 * add_random_kstack_offset_irqsoff - Increase stack utilization by a random offset.
 *
 * This should be used in the syscall entry path after user registers have been
 * stored to the stack. Interrupts must be still disabled.
 */
#[macro_export]
macro_rules! add_random_kstack_offset_irqsoff {
    () => {{
        unsafe {
            $crate::lockdep_assert_irqs_disabled();
            if $crate::static_branch_maybe(
                CONFIG_RANDOMIZE_KSTACK_OFFSET_DEFAULT,
                &$crate::randomize_kstack_offset,
            ) {
                let offset: u32 = $crate::prandom_u32_state(
                    $crate::raw_cpu_ptr(&raw mut $crate::kstack_rnd_state),
                );
                let ptr: *mut u8 = $crate::__kstack_alloca(
                    $crate::kstack_offset_max(offset) as usize,
                );
                /* Keep allocation even after "ptr" loses scope. */
                core::arch::asm!("", in("reg") ptr, options(nostack, preserves_flags));
            }
        }
    }};
}

/* When CONFIG_RANDOMIZE_KSTACK_OFFSET is disabled, both C macros are no-ops. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
