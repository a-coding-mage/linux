/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <asm/ldt.h>

#[repr(C)]
pub struct task_struct;

/* misc architecture specific prototypes */

unsafe extern "C" {
    pub fn syscall_init();

    #[cfg(CONFIG_X86_64)]
    pub fn entry_SYSCALL_64();
    #[cfg(CONFIG_X86_64)]
    pub fn entry_SYSCALL_64_safe_stack();
    #[cfg(CONFIG_X86_64)]
    pub fn entry_SYSRETQ_unsafe_stack();
    #[cfg(CONFIG_X86_64)]
    pub fn entry_SYSRETQ_end();

    #[cfg(CONFIG_X86_32)]
    pub fn entry_INT80_32();
    #[cfg(CONFIG_X86_32)]
    pub fn entry_SYSENTER_32();
    #[cfg(CONFIG_X86_32)]
    pub fn __begin_SYSENTER_singlestep_region();
    #[cfg(CONFIG_X86_32)]
    pub fn __end_SYSENTER_singlestep_region();

    #[cfg(CONFIG_IA32_EMULATION)]
    pub fn entry_SYSENTER_compat();
    #[cfg(CONFIG_IA32_EMULATION)]
    pub fn __end_entry_SYSENTER_compat();
    #[cfg(CONFIG_IA32_EMULATION)]
    pub fn entry_SYSCALL_compat();
    #[cfg(CONFIG_IA32_EMULATION)]
    pub fn entry_SYSCALL_compat_safe_stack();
    #[cfg(CONFIG_IA32_EMULATION)]
    pub fn entry_SYSRETL_compat_unsafe_stack();
    #[cfg(CONFIG_IA32_EMULATION)]
    pub fn entry_SYSRETL_compat_end();

    pub fn x86_configure_nx();

    pub static mut reboot_force: i32;

    pub fn do_arch_prctl_64(
        task: *mut task_struct,
        option: i32,
        arg2: usize,
    ) -> isize;
}

#[cfg(not(CONFIG_IA32_EMULATION))]
pub const entry_SYSCALL_compat: Option<unsafe extern "C" fn()> = None;
#[cfg(not(CONFIG_IA32_EMULATION))]
pub const entry_SYSENTER_compat: Option<unsafe extern "C" fn()> = None;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
