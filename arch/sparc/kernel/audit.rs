// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and architecture headers are
// referenced below but are not implemented in this translation unit.

extern "C" {
    fn sparc32_classify_syscall(syscall: u32) -> i32;
    static sparc32_write_class: u32;
    static sparc32_read_class: u32;
    static sparc32_dir_class: u32;
    static sparc32_chattr_class: u32;
    static sparc32_signal_class: u32;

    fn audit_register_class(class: u32, entries: *const u32);
}

// Entries supplied by asm-generic/audit_dir_write.h.
static mut dir_class: [u32; 1] = [u32::MAX];

// Entries supplied by asm-generic/audit_read.h.
static mut read_class: [u32; 1] = [u32::MAX];

// Entries supplied by asm-generic/audit_write.h.
static mut write_class: [u32; 1] = [u32::MAX];

// Entries supplied by asm-generic/audit_change_attr.h.
static mut chattr_class: [u32; 1] = [u32::MAX];

// Entries supplied by asm-generic/audit_signal.h.
static mut signal_class: [u32; 1] = [u32::MAX];

pub unsafe fn audit_classify_arch(arch: i32) -> i32 {
    // CONFIG_COMPAT
    if arch == AUDIT_ARCH_SPARC {
        return 1;
    }
    0
}

pub unsafe fn audit_classify_syscall(abi: i32, syscall: u32) -> i32 {
    // CONFIG_COMPAT
    if abi == AUDIT_ARCH_SPARC {
        return sparc32_classify_syscall(syscall);
    }
    match syscall {
        __NR_open => AUDITSC_OPEN,
        __NR_openat => AUDITSC_OPENAT,
        __NR_socketcall => AUDITSC_SOCKETCALL,
        __NR_execve => AUDITSC_EXECVE,
        __NR_openat2 => AUDITSC_OPENAT2,
        _ => AUDITSC_NATIVE,
    }
}

unsafe extern "C" fn audit_classes_init() -> i32 {
    // CONFIG_COMPAT
    audit_register_class(AUDIT_CLASS_WRITE_32, &sparc32_write_class);
    audit_register_class(AUDIT_CLASS_READ_32, &sparc32_read_class);
    audit_register_class(AUDIT_CLASS_DIR_WRITE_32, &sparc32_dir_class);
    audit_register_class(AUDIT_CLASS_CHATTR_32, &sparc32_chattr_class);
    audit_register_class(AUDIT_CLASS_SIGNAL_32, &sparc32_signal_class);

    audit_register_class(AUDIT_CLASS_WRITE, write_class.as_ptr());
    audit_register_class(AUDIT_CLASS_READ, read_class.as_ptr());
    audit_register_class(AUDIT_CLASS_DIR_WRITE, dir_class.as_ptr());
    audit_register_class(AUDIT_CLASS_CHATTR, chattr_class.as_ptr());
    audit_register_class(AUDIT_CLASS_SIGNAL, signal_class.as_ptr());
    0
}

// __initcall(audit_classes_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
