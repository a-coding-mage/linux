/* SPDX-License-Identifier: GPL-2.0 */
/*
 * GCC stack protector support.
 *
 * Stack protector works by putting a predefined pattern at the start of
 * the stack frame and verifying that it hasn't been overwritten when
 * returning from the function.  The pattern is called the stack canary
 * and is a unique value for each task.
 */

/* The C header guard and include directives are omitted. */

/* CONFIG_STACKPROTECTOR */

/* Dependencies supplied by the surrounding kernel translation. */
#[cfg(CONFIG_STACKPROTECTOR)]
extern "C" {
    pub static mut current: *mut task_struct;
    pub fn get_random_canary() -> usize;
}

#[repr(C)]
pub struct task_struct {
    pub stack_canary: usize,
}

/* DECLARE_PER_CPU_CACHE_HOT(unsigned long, __stack_chk_guard); */
#[cfg(CONFIG_STACKPROTECTOR)]
extern "C" {
    pub static mut __stack_chk_guard: usize;
}

/*
 * Initialize the stackprotector canary value.
 *
 * NOTE: this must only be called from functions that never return
 * and it must always be inlined.
 *
 * In addition, it should be called from a compilation unit for which
 * stack protector is disabled. Alternatively, the caller should not end
 * with a function call which gets tail-call optimized as that would
 * lead to checking a modified canary value.
 */
#[inline(always)]
#[cfg(CONFIG_STACKPROTECTOR)]
pub unsafe fn boot_init_stack_canary() {
    let canary: usize = get_random_canary();

    (*current).stack_canary = canary;
    /* this_cpu_write(__stack_chk_guard, canary); */
    __stack_chk_guard = canary;
}

#[cfg(CONFIG_STACKPROTECTOR)]
pub unsafe fn cpu_init_stack_canary(cpu: i32, idle: *mut task_struct) {
    /* per_cpu(__stack_chk_guard, cpu) = idle->stack_canary; */
    let _ = cpu;
    __stack_chk_guard = (*idle).stack_canary;
}

/* STACKPROTECTOR disabled: dummy boot_init_stack_canary() is defined in
 * linux/stackprotector.h. */
#[cfg(not(CONFIG_STACKPROTECTOR))]
pub unsafe fn cpu_init_stack_canary(_cpu: i32, _idle: *mut task_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
