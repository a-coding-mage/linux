/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <linux/compiler.h> and <linux/threads.h>.
// Dependency intent from <shared/smp.h>.

#[cfg(not(__ASSEMBLER__))]
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[cfg(not(__ASSEMBLER__))]
extern "C" {
    pub static mut cpu_tasks: [*mut task_struct; NR_CPUS];
    pub fn uml_curr_cpu() -> usize;
}

#[cfg(not(__ASSEMBLER__))]
#[inline(always)]
pub unsafe fn get_current() -> *mut task_struct {
    cpu_tasks[uml_curr_cpu()]
}

#[cfg(not(__ASSEMBLER__))]
#[macro_export]
macro_rules! current {
    () => {
        $crate::get_current()
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
