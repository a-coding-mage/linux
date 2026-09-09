// SPDX-License-Identifier: GPL-2.0
// Dependencies correspond to: <linux/init.h>, <linux/types.h>,
// <linux/audit.h>, and <asm/unistd.h>.

// The entries supplied by <asm-generic/audit_dir_write.h> are retained as a
// build-time dependency; the terminating all-ones entry is local to this file.
static mut dir_class: [u32; 1] = [u32::MAX];

// The entries supplied by <asm-generic/audit_read.h> are retained as a
// build-time dependency; the terminating all-ones entry is local to this file.
static mut read_class: [u32; 1] = [u32::MAX];

// The entries supplied by <asm-generic/audit_write.h> are retained as a
// build-time dependency; the terminating all-ones entry is local to this file.
static mut write_class: [u32; 1] = [u32::MAX];

// The entries supplied by <asm-generic/audit_change_attr.h> are retained as a
// build-time dependency; the terminating all-ones entry is local to this file.
static mut chattr_class: [u32; 1] = [u32::MAX];

// The entries supplied by <asm-generic/audit_signal.h> are retained as a
// build-time dependency; the terminating all-ones entry is local to this file.
static mut signal_class: [u32; 1] = [u32::MAX];

extern "C" {
    fn audit_register_class(class: u32, list: *mut u32);
}

pub unsafe fn audit_classify_arch(_arch: i32) -> i32 {
    0
}

pub unsafe fn audit_classify_syscall(abi: i32, syscall: u32) -> i32 {
    let _ = abi;
    match syscall {
        __NR_open => AUDITSC_OPEN,
        __NR_openat => AUDITSC_OPENAT,
        __NR_socketcall => AUDITSC_SOCKETCALL,
        __NR_execve => AUDITSC_EXECVE,
        __NR_openat2 => AUDITSC_OPENAT2,
        _ => AUDITSC_NATIVE,
    }
}

unsafe fn audit_classes_init() -> i32 {
    audit_register_class(AUDIT_CLASS_WRITE, write_class.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_READ, read_class.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_DIR_WRITE, dir_class.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_CHATTR, chattr_class.as_mut_ptr());
    audit_register_class(AUDIT_CLASS_SIGNAL, signal_class.as_mut_ptr());
    0
}

// __initcall(audit_classes_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
