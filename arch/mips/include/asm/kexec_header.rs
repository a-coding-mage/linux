/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * kexec.h for kexec
 * Created by <nschichan@corp.free.fr> on Thu Oct 12 14:59:34 2006
 */

// C header guard: _MIPS_KEXEC
// Dependency supplied by asm/stacktrace.h.

/* Maximum physical address we can use pages from */
pub const KEXEC_SOURCE_MEMORY_LIMIT: c_ulong = !0;
/* Maximum address we can reach in physical address mode */
pub const KEXEC_DESTINATION_MEMORY_LIMIT: c_ulong = !0;
/* Maximum address we can use for the control code buffer */
pub const KEXEC_CONTROL_MEMORY_LIMIT: c_ulong = !0;
/* Reserve 3*4096 bytes for board-specific info */
pub const KEXEC_CONTROL_PAGE_SIZE: usize = 4096 + 3 * 4096;

/* The native architecture */
pub const KEXEC_ARCH: u32 = KEXEC_ARCH_MIPS;
pub const MAX_NOTE_BYTES: usize = 1024;

pub unsafe fn crash_setup_regs(newregs: *mut pt_regs, oldregs: *mut pt_regs) {
    if !oldregs.is_null() {
        memcpy(
            newregs.cast::<c_void>(),
            oldregs.cast::<c_void>(),
            core::mem::size_of::<pt_regs>(),
        );
    } else {
        prepare_frametrace(newregs);
    }
}

/* CONFIG_KEXEC_CORE conditional declarations from the C header. */
#[cfg(CONFIG_KEXEC_CORE)]
extern "C" {
    pub static mut kexec_args: [c_ulong; 4];
    pub static mut _machine_kexec_prepare:
        Option<unsafe extern "C" fn(*mut kimage) -> c_int>;
    pub static mut _machine_kexec_shutdown: Option<unsafe extern "C" fn()>;
    pub static mut _machine_crash_shutdown:
        Option<unsafe extern "C" fn(*mut pt_regs)>;
    pub fn default_machine_crash_shutdown(regs: *mut pt_regs);
    pub fn kexec_nonboot_cpu_jump();
    pub fn kexec_reboot();

    /* CONFIG_SMP conditional declarations from the C header. */
    #[cfg(CONFIG_SMP)]
    pub static kexec_smp_wait: [c_uchar; 0];
    #[cfg(CONFIG_SMP)]
    pub static mut secondary_kexec_args: [c_ulong; 4];
    #[cfg(CONFIG_SMP)]
    pub static mut kexec_ready_to_reboot: atomic_t;
    #[cfg(CONFIG_SMP)]
    pub static mut _crash_smp_send_stop: Option<unsafe extern "C" fn()>;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
