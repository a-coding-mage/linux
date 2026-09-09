// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/init.h, linux/types.h, linux/audit.h,
// asm/unistd.h, asm/audit.h, asm-generic/audit_*.h

// The entries supplied by the asm-generic audit headers are external build
// inputs; the terminating ~0U entry is preserved here.
static mut dir_class: [u32; 1] = [u32::MAX];
static mut read_class: [u32; 1] = [u32::MAX];
static mut write_class: [u32; 1] = [u32::MAX];
static mut chattr_class: [u32; 1] = [u32::MAX];
static mut signal_class: [u32; 1] = [u32::MAX];

// CONFIG_IA32_EMULATION controls the 32-bit declarations and branches below.
extern "C" {
    fn ia32_classify_syscall(syscall: u32) -> i32;
    static mut ia32_write_class: u32;
    static mut ia32_read_class: u32;
    static mut ia32_dir_class: u32;
    static mut ia32_chattr_class: u32;
    static mut ia32_signal_class: u32;
    fn audit_register_class(class: u32, mask: *mut u32);
}

pub unsafe fn audit_classify_arch(arch: i32) -> i32 {
    // #ifdef CONFIG_IA32_EMULATION
    // if (arch == AUDIT_ARCH_I386)
    //     return 1;
    // #endif
    let _ = arch;
    0
}

pub unsafe fn audit_classify_syscall(abi: i32, syscall: u32) -> i32 {
    // #ifdef CONFIG_IA32_EMULATION
    // if (abi == AUDIT_ARCH_I386)
    //     return ia32_classify_syscall(syscall);
    // #endif
    let _ = abi;
    match syscall {
        __NR_open => AUDITSC_OPEN,
        __NR_openat => AUDITSC_OPENAT,
        __NR_execve | __NR_execveat => AUDITSC_EXECVE,
        __NR_openat2 => AUDITSC_OPENAT2,
        _ => AUDITSC_NATIVE,
    }
}

unsafe fn audit_classes_init() -> i32 {
    // #ifdef CONFIG_IA32_EMULATION
    // audit_register_class(AUDIT_CLASS_WRITE_32, ia32_write_class);
    // audit_register_class(AUDIT_CLASS_READ_32, ia32_read_class);
    // audit_register_class(AUDIT_CLASS_DIR_WRITE_32, ia32_dir_class);
    // audit_register_class(AUDIT_CLASS_CHATTR_32, ia32_chattr_class);
    // audit_register_class(AUDIT_CLASS_SIGNAL_32, ia32_signal_class);
    // #endif
    audit_register_class(AUDIT_CLASS_WRITE, write_class.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_READ, read_class.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_DIR_WRITE, dir_class.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_CHATTR, chattr_class.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_SIGNAL, signal_class.as_mut_ptr());
    0
}

// __initcall(audit_classes_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
