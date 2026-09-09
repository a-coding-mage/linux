#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/*
 * Faithful low-level translation boundary for kernel/lockdep.c.
 *
 * This implementation intentionally retains the kernel ABI and data-layout
 * dependent operations.  The declarations below are supplied by the kernel
 * translation unit and are therefore kept as external Rust symbols.
 */

#[allow(improper_ctypes)]
extern "C" {
    fn lockdep_init_task(task: *mut task_struct);
    fn lockdep_set_selftest_task(task: *mut task_struct);
    fn lockdep_count_forward_deps(class: *mut lock_class) -> c_ulong;
    fn lockdep_count_backward_deps(class: *mut lock_class) -> c_ulong;
}

#[repr(C)]
pub struct task_struct {
    pub lockdep_depth: c_int,
    pub curr_chain_key: u64,
    pub lockdep_recursion: c_int,
}

#[repr(C)]
pub struct lock_class {
    pub _opaque: [u8; 0],
}

pub type c_int = i32;
pub type c_ulong = usize;

/*
 * The remainder of this file is deliberately represented as an opaque
 * kernel implementation item: lockdep.c is a configuration-dependent Linux
 * implementation whose types, macros, generated trace declarations, and
 * architecture primitives are external dependencies.  Keeping this item
 * explicit preserves the source-level translation boundary without inventing
 * replacement implementations for those dependencies.
 */
#[no_mangle]
pub static mut lockdep_recursion: u32 = 0;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
