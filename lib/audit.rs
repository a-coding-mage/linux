// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel headers are intentionally external.

static mut DIR_CLASS: [u32; 1] = [u32::MAX];
static mut READ_CLASS: [u32; 1] = [u32::MAX];
static mut WRITE_CLASS: [u32; 1] = [u32::MAX];
static mut CHATTR_CLASS: [u32; 1] = [u32::MAX];
static mut SIGNAL_CLASS: [u32; 1] = [u32::MAX];

extern "C" {
    fn audit_is_compat(arch: i32) -> i32;
    fn audit_classify_compat_syscall(abi: i32, syscall: u32) -> i32;
    fn audit_register_class(class: u32, map: *mut u32);
}

#[no_mangle]
pub unsafe extern "C" fn audit_classify_arch(arch: i32) -> i32 {
    if audit_is_compat(arch) != 0 {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn audit_classify_syscall(abi: i32, syscall: u32) -> i32 {
    if audit_is_compat(abi) != 0 {
        return audit_classify_compat_syscall(abi, syscall);
    }

    match syscall {
        #[cfg(__NR_open)]
        __NR_open => AUDITSC_OPEN,
        #[cfg(__NR_openat)]
        __NR_openat => AUDITSC_OPENAT,
        #[cfg(__NR_socketcall)]
        __NR_socketcall => AUDITSC_SOCKETCALL,
        #[cfg(__NR_execveat)]
        __NR_execveat => AUDITSC_EXECVE,
        __NR_execve => AUDITSC_EXECVE,
        #[cfg(__NR_openat2)]
        __NR_openat2 => AUDITSC_OPENAT2,
        _ => AUDITSC_NATIVE,
    }
}

unsafe extern "C" fn audit_classes_init() -> i32 {
    // CONFIG_AUDIT_COMPAT_GENERIC: register the corresponding compat classes
    // when that build-time configuration is enabled.
    #[cfg(CONFIG_AUDIT_COMPAT_GENERIC)]
    {
        audit_register_class(AUDIT_CLASS_WRITE_32, compat_write_class);
        audit_register_class(AUDIT_CLASS_READ_32, compat_read_class);
        audit_register_class(AUDIT_CLASS_DIR_WRITE_32, compat_dir_class);
        audit_register_class(AUDIT_CLASS_CHATTR_32, compat_chattr_class);
        audit_register_class(AUDIT_CLASS_SIGNAL_32, compat_signal_class);
    }

    audit_register_class(AUDIT_CLASS_WRITE, WRITE_CLASS.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_READ, READ_CLASS.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_DIR_WRITE, DIR_CLASS.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_CHATTR, CHATTR_CLASS.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_SIGNAL, SIGNAL_CLASS.as_mut_ptr());
    0
}

// __initcall(audit_classes_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
