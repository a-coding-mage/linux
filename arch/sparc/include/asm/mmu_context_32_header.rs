/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: __SPARC_MMU_CONTEXT_H */
/* C assembler guard: declarations are omitted when __ASSEMBLER__ is defined. */

/* Dependency: <asm-generic/mm_hooks.h> */

/* Initialize a new mmu context.  This is invoked when a new
 * address space instance (unique or shared) is instantiated.
 */
/* C macro: #define init_new_context init_new_context */
extern "C" {
    pub fn init_new_context(
        tsk: *mut task_struct,
        mm: *mut mm_struct,
    ) -> core::ffi::c_int;
}

/* Destroy a dead context.  This occurs when mmput drops the
 * mm_users count to zero, the mmaps have been released, and
 * all the page tables have been flushed.  Our job is to destroy
 * any remaining processor-specific state.
 */
/* C macro: #define destroy_context destroy_context */
extern "C" {
    pub fn destroy_context(mm: *mut mm_struct);
}

/* Switch the current MM context. */
extern "C" {
    pub fn switch_mm(
        old_mm: *mut mm_struct,
        mm: *mut mm_struct,
        tsk: *mut task_struct,
    );
}

/* Activate a new MM instance for the current task. */
#[macro_export]
macro_rules! activate_mm {
    ($active_mm:expr, $mm:expr) => {
        $crate::switch_mm(($active_mm), ($mm), core::ptr::null_mut())
    };
}

/* Dependency: <asm-generic/mmu_context.h> */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
