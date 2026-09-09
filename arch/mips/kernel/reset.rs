/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001, 06 by Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2001 MIPS Technologies, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_char;

/*
 * Urgs ...  Too many MIPS machines to handle this in a generic way.
 * So handle all using function pointers to machine specific
 * functions.
 */
#[no_mangle]
pub static mut _machine_restart: Option<unsafe extern "C" fn(*mut c_char)> = None;
#[no_mangle]
pub static mut _machine_halt: Option<unsafe extern "C" fn()> = None;
#[no_mangle]
pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;

extern "C" {
    fn local_irq_disable();
    fn clear_c0_status(value: u32);
    fn cpu_wait();
    fn write_c0_compare(value: u32);
    fn do_kernel_restart(command: *mut c_char);
    fn mdelay(milliseconds: u32);
    fn do_kernel_power_off();
    fn preempt_disable();
    fn smp_send_stop();
    fn pr_emerg(format: *const c_char, ...);
}

// These symbols and constants are supplied by the MIPS architecture layer.
extern "C" {
    static cpu_has_mips_r: bool;
    static cpu_has_counter: bool;
    static cpu_wait: Option<unsafe extern "C" fn()>;
}

const ST0_IM: u32 = 0x0000_ff00;

unsafe fn machine_hang() {
    /*
     * We're hanging the system so we don't want to be interrupted anymore.
     * Any interrupt handlers that ran would at best be useless & at worst
     * go awry because the system isn't in a functional state.
     */
    local_irq_disable();

    /*
     * Mask all interrupts, giving us a better chance of remaining in the
     * low power wait state.
     */
    clear_c0_status(ST0_IM);

    loop {
        if cpu_has_mips_r {
            /*
             * We know that the wait instruction is supported so
             * make use of it directly, leaving interrupts disabled.
             */
            core::arch::asm!("wait", options(nostack));
        } else if let Some(wait) = cpu_wait {
            /*
             * Try the cpu_wait() callback. This isn't ideal since
             * it'll re-enable interrupts, but that ought to be
             * harmless given that they're all masked.
             */
            wait();
            local_irq_disable();
        } else {
            /*
             * We're going to burn some power running round the
             * loop, but we don't really have a choice. This isn't
             * a path we should expect to run for long during
             * typical use anyway.
             */
        }

        /*
         * In most modern MIPS CPUs interrupts will cause the wait
         * instruction to graduate even when disabled, and in some
         * cases even when masked. In order to prevent a timer
         * interrupt from continuously taking us out of the low power
         * wait state, we clear any pending timer interrupt here.
         */
        if cpu_has_counter {
            write_c0_compare(0);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn machine_restart(command: *mut c_char) {
    if let Some(restart) = _machine_restart {
        restart(command);
    }

    // CONFIG_SMP conditional code from the C source.
    #[cfg(feature = "CONFIG_SMP")]
    {
        preempt_disable();
        smp_send_stop();
    }
    do_kernel_restart(command);
    mdelay(1000);
    pr_emerg(b"Reboot failed -- System halted\0".as_ptr() as *const c_char);
    machine_hang();
}

#[no_mangle]
pub unsafe extern "C" fn machine_halt() {
    if let Some(halt) = _machine_halt {
        halt();
    }

    // CONFIG_SMP conditional code from the C source.
    #[cfg(feature = "CONFIG_SMP")]
    {
        preempt_disable();
        smp_send_stop();
    }
    machine_hang();
}

#[no_mangle]
pub unsafe extern "C" fn machine_power_off() {
    do_kernel_power_off();

    // CONFIG_SMP conditional code from the C source.
    #[cfg(feature = "CONFIG_SMP")]
    {
        preempt_disable();
        smp_send_stop();
    }
    machine_hang();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
