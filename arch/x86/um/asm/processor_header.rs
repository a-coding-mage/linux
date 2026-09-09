/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent preserved from <linux/time-internal.h>.
// Dependency intent preserved from <sysdep/faultinfo.h>.
// CONFIG_X86_32 selects processor_32.h; otherwise processor_64.h.
// Dependency intent preserved from <asm/user.h> and <asm/processor-generic.h>.

/* #define KSTK_EIP(tsk) KSTK_REG(tsk, HOST_IP) */
#[macro_export]
macro_rules! KSTK_EIP {
    ($tsk:expr) => { KSTK_REG!($tsk, HOST_IP) };
}

/* #define KSTK_ESP(tsk) KSTK_REG(tsk, HOST_SP) */
#[macro_export]
macro_rules! KSTK_ESP {
    ($tsk:expr) => { KSTK_REG!($tsk, HOST_SP) };
}

/* #define KSTK_EBP(tsk) KSTK_REG(tsk, HOST_BP) */
#[macro_export]
macro_rules! KSTK_EBP {
    ($tsk:expr) => { KSTK_REG!($tsk, HOST_BP) };
}

/*
 * #define ARCH_IS_STACKGROW(address) \
 *     (address + 65536 + 32 * sizeof(unsigned long) >= \
 *      UPT_SP(&current->thread.regs.regs))
 */
#[macro_export]
macro_rules! ARCH_IS_STACKGROW {
    ($address:expr) => {
        $address + 65536 + 32 * core::mem::size_of::<core::ffi::c_ulong>()
            >= UPT_SP!(&current.thread.regs.regs)
    };
}

/* PAUSE is a good thing to insert into busy-wait loops. */
#[inline(always)]
pub unsafe fn native_pause() {
    core::arch::asm!("pause", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn cpu_relax() {
    if time_travel_mode == TT_MODE_INFCPU || time_travel_mode == TT_MODE_EXTERNAL {
        time_travel_ndelay(1);
    } else {
        native_pause();
    }
}

#[macro_export]
macro_rules! task_pt_regs {
    ($t:expr) => { &mut ($t).thread.regs };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
