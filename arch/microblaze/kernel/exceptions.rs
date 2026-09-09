/*
 * HW exception handling
 *
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008 PetaLogix
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file COPYING in the main directory of this
 * archive for more details.
 */

/*
 * This file handles the architecture-dependent parts of hardware exceptions
 */

// C headers and architecture dependencies are supplied by the surrounding kernel.

const MICROBLAZE_ILL_OPCODE_EXCEPTION: u32 = 0x02;
const MICROBLAZE_IBUS_EXCEPTION: u32 = 0x03;
const MICROBLAZE_DBUS_EXCEPTION: u32 = 0x04;
const MICROBLAZE_DIV_ZERO_EXCEPTION: u32 = 0x05;
const MICROBLAZE_FPU_EXCEPTION: u32 = 0x06;
const MICROBLAZE_PRIVILEGED_EXCEPTION: u32 = 0x07;

static mut die_lock: DEFINE_SPINLOCK = DEFINE_SPINLOCK::new();

pub unsafe fn die(str_: *const core::ffi::c_char, fp: *mut pt_regs, err: libc::c_long) {
    console_verbose();
    spin_lock_irq(&raw mut die_lock);
    pr_warn!("Oops: %s, sig: %ld\n", str_, err);
    show_regs(fp);
    spin_unlock_irq(&raw mut die_lock);
    /* make_task_dead() should take care of panic'ing from an interrupt
     * context so we don't handle it here
     */
    make_task_dead(err);
}

/* for user application debugging */
pub unsafe extern "C" fn sw_exception(regs: *mut pt_regs) {
    _exception(SIGTRAP, regs, TRAP_BRKPT, (*regs).r16);
    flush_dcache_range((*regs).r16, (*regs).r16.wrapping_add(0x4));
    flush_icache_range((*regs).r16, (*regs).r16.wrapping_add(0x4));
}

pub unsafe fn _exception(
    signr: libc::c_int,
    regs: *mut pt_regs,
    code: libc::c_int,
    addr: libc::c_ulong,
) {
    if kernel_mode(regs) {
        die(c"Exception in kernel mode".as_ptr(), regs, signr as libc::c_long);
    }

    force_sig_fault(signr, code, addr as *mut core::ffi::c_void);
}

pub unsafe extern "C" fn full_exception(
    regs: *mut pt_regs,
    type_: libc::c_uint,
    mut fsr: libc::c_int,
    mut addr: libc::c_int,
) {
    addr = (*regs).pc as libc::c_int;

    #[cfg(any())]
    pr_warn!(
        "Exception %02x in %s mode, FSR=%08x PC=%08x ESR=%08x\n",
        type_,
        if user_mode(regs) { "user" } else { "kernel" },
        fsr,
        (*regs).pc as libc::c_uint,
        (*regs).esr as libc::c_uint
    );

    match type_ & 0x1f {
        MICROBLAZE_ILL_OPCODE_EXCEPTION => {
            if user_mode(regs) {
                pr_debug!("Illegal opcode exception in user mode\n");
                _exception(SIGILL, regs, ILL_ILLOPC, addr as libc::c_ulong);
                return;
            }
            pr_warn!("Illegal opcode exception in kernel mode.\n");
            die(c"opcode exception".as_ptr(), regs, SIGBUS as libc::c_long);
        }
        MICROBLAZE_IBUS_EXCEPTION | MICROBLAZE_DBUS_EXCEPTION => {
            let data = type_ == MICROBLAZE_DBUS_EXCEPTION;
            if user_mode(regs) {
                if data { pr_debug!("Data bus error exception in user mode\n"); }
                else { pr_debug!("Instruction bus error exception in user mode\n"); }
                _exception(SIGBUS, regs, BUS_ADRERR, addr as libc::c_ulong);
                return;
            }
            if data { pr_warn!("Data bus error exception in kernel mode.\n"); }
            else { pr_warn!("Instruction bus error exception in kernel mode.\n"); }
            die(c"bus exception".as_ptr(), regs, SIGBUS as libc::c_long);
        }
        MICROBLAZE_DIV_ZERO_EXCEPTION => {
            if user_mode(regs) {
                pr_debug!("Divide by zero exception in user mode\n");
                _exception(SIGFPE, regs, FPE_INTDIV, addr as libc::c_ulong);
                return;
            }
            pr_warn!("Divide by zero exception in kernel mode.\n");
            die(c"Divide by zero exception".as_ptr(), regs, SIGBUS as libc::c_long);
        }
        MICROBLAZE_FPU_EXCEPTION => {
            pr_debug!("FPU exception\n");
            /* IEEE FP exception */
            /* I removed fsr variable and use code var for storing fsr */
            if fsr & FSR_IO != 0 { fsr = FPE_FLTINV; }
            else if fsr & FSR_OF != 0 { fsr = FPE_FLTOVF; }
            else if fsr & FSR_UF != 0 { fsr = FPE_FLTUND; }
            else if fsr & FSR_DZ != 0 { fsr = FPE_FLTDIV; }
            else if fsr & FSR_DO != 0 { fsr = FPE_FLTRES; }
            _exception(SIGFPE, regs, fsr, addr as libc::c_ulong);
        }
        MICROBLAZE_PRIVILEGED_EXCEPTION => {
            pr_debug!("Privileged exception\n");
            _exception(SIGILL, regs, ILL_PRVOPC, addr as libc::c_ulong);
        }
        _ => {
            /* FIXME what to do in unexpected exception */
            pr_warn!(
                "Unexpected exception %02x PC=%08x in %s mode\n",
                type_, addr as libc::c_uint,
                if kernel_mode(regs) { "kernel" } else { "user" }
            );
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
