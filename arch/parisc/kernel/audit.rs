// SPDX-License-Identifier: GPL-2.0
//
// Translated from the C implementation.  The entries originally supplied by
// asm-generic audit headers remain external build-time dependencies.

// The following arrays correspond to the C arrays populated by
// <asm-generic/audit_*.h>, followed by ~0U.
static mut DIR_CLASS: [u32; 1] = [u32::MAX];
static mut READ_CLASS: [u32; 1] = [u32::MAX];
static mut WRITE_CLASS: [u32; 1] = [u32::MAX];
static mut CHATTR_CLASS: [u32; 1] = [u32::MAX];
static mut SIGNAL_CLASS: [u32; 1] = [u32::MAX];

// CONFIG_COMPAT conditionally supplies these declarations and registrations.
extern "C" {
    fn audit_register_class(class: u32, mask: *mut u32);
}

pub unsafe fn audit_classify_arch(arch: i32) -> i32 {
    // #ifdef CONFIG_COMPAT
    if arch == AUDIT_ARCH_PARISC {
        return 1;
    }
    // #endif
    0
}

pub unsafe fn audit_classify_syscall(abi: i32, syscall: u32) -> i32 {
    match syscall {
        __NR_open => AUDITSC_OPEN,
        __NR_openat => AUDITSC_OPENAT,
        __NR_execve => AUDITSC_EXECVE,
        __NR_openat2 => AUDITSC_OPENAT2,
        _ => {
            // #ifdef CONFIG_COMPAT
            if abi == AUDIT_ARCH_PARISC {
                return AUDITSC_COMPAT;
            }
            // #endif
            AUDITSC_NATIVE
        }
    }
}

unsafe fn audit_classes_init() -> i32 {
    // #ifdef CONFIG_COMPAT
    extern "C" {
        static mut parisc32_dir_class: u32;
        static mut parisc32_write_class: u32;
        static mut parisc32_read_class: u32;
        static mut parisc32_chattr_class: u32;
        static mut parisc32_signal_class: u32;
    }
    audit_register_class(AUDIT_CLASS_WRITE_32, &mut parisc32_write_class);
    audit_register_class(AUDIT_CLASS_READ_32, &mut parisc32_read_class);
    audit_register_class(AUDIT_CLASS_DIR_WRITE_32, &mut parisc32_dir_class);
    audit_register_class(AUDIT_CLASS_CHATTR_32, &mut parisc32_chattr_class);
    audit_register_class(AUDIT_CLASS_SIGNAL_32, &mut parisc32_signal_class);
    // #endif
    audit_register_class(AUDIT_CLASS_WRITE, WRITE_CLASS.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_READ, READ_CLASS.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_DIR_WRITE, DIR_CLASS.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_CHATTR, CHATTR_CLASS.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_SIGNAL, SIGNAL_CLASS.as_mut_ptr());
    0
}

// __initcall(audit_classes_init);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
