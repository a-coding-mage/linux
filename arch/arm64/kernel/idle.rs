// SPDX-License-Identifier: GPL-2.0-only
/*
 * Low-level idle sequences
 */

use core::arch::asm;
use core::ffi::{c_char, c_int};

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct arm_cpuidle_irq_context {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
    fn arm_cpuidle_save_irq_context(context: *mut arm_cpuidle_irq_context);
    fn arm_cpuidle_restore_irq_context(context: *mut arm_cpuidle_irq_context);
}

enum Idle {
    ARM64_IDLE_WFI,
    ARM64_IDLE_YIELD,
    ARM64_IDLE_NOP,
}

static mut idle: i32 = 0; // ARM64_IDLE_WFI

unsafe fn setup_idle(arg: *mut c_char) -> c_int
{
    if arg.is_null() {
        return -1;
    } else if strcmp(arg, c"wfi".as_ptr()) == 0 {
        idle = 0; // ARM64_IDLE_WFI
    } else if strcmp(arg, c"yield".as_ptr()) == 0 {
        idle = 1; // ARM64_IDLE_YIELD
    } else if strcmp(arg, c"nop".as_ptr()) == 0 {
        idle = 2; // ARM64_IDLE_NOP
    } else {
        return -1;
    }

    0
}

// early_param("idle", setup_idle);

/*
 *	cpu_do_idle()
 *
 *	Idle the processor (wait for interrupt).
 *
 *	If the CPU supports priority masking we must do additional work to
 *	ensure that interrupts are not masked at the PMR (because the core will
 *	not wake up if we block the wake up signal in the interrupt controller).
 */
pub unsafe fn cpu_do_idle()
{
    let mut context = core::mem::MaybeUninit::<arm_cpuidle_irq_context>::uninit();

    arm_cpuidle_save_irq_context(context.as_mut_ptr());

    if idle == 0 { // likely(idle == ARM64_IDLE_WFI)
        asm!("dsb sy", options(nostack, preserves_flags));
        asm!("wfi", options(nostack, preserves_flags));
    } else if idle == 1 {
        asm!("dsb sy", options(nostack, preserves_flags));
        asm!("yield", options(nostack, preserves_flags));
    }

    arm_cpuidle_restore_irq_context(context.as_mut_ptr());
}

/*
 * This is our default idle handler.
 */
pub unsafe fn arch_cpu_idle()
{
    /*
     * This should do all the clock switching and wait for interrupt
     * tricks
     */
    cpu_do_idle();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
