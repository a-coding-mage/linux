// Translated from audit.c.
// The C include files below supplied architecture-specific class entries;
// those entries remain external build-time dependencies in this translation.

unsafe extern "C" {
    fn audit_register_class(class: u32, list: *mut u32);
}

// Values supplied by <asm/unistd.h> and <linux/audit.h>.
unsafe extern "C" {
    static __NR_open: u32;
    static __NR_openat: u32;
    static __NR_execve: u32;
    static __NR_openat2: u32;
    static AUDITSC_OPEN: i32;
    static AUDITSC_OPENAT: i32;
    static AUDITSC_EXECVE: i32;
    static AUDITSC_OPENAT2: i32;
    static AUDITSC_NATIVE: i32;
    static AUDIT_CLASS_WRITE: u32;
    static AUDIT_CLASS_READ: u32;
    static AUDIT_CLASS_DIR_WRITE: u32;
    static AUDIT_CLASS_CHATTR: u32;
    static AUDIT_CLASS_SIGNAL: u32;
}

// Entries supplied by <asm-generic/audit_dir_write.h>, terminated by ~0U.
static mut dir_class: [u32; 1] = [u32::MAX];

// Entries supplied by <asm-generic/audit_read.h>, terminated by ~0U.
static mut read_class: [u32; 1] = [u32::MAX];

// Entries supplied by <asm-generic/audit_write.h>, terminated by ~0U.
static mut write_class: [u32; 1] = [u32::MAX];

// Entries supplied by <asm-generic/audit_change_attr.h>, terminated by ~0U.
static mut chattr_class: [u32; 1] = [u32::MAX];

// Entries supplied by <asm-generic/audit_signal.h>, terminated by ~0U.
static mut signal_class: [u32; 1] = [u32::MAX];

pub unsafe fn audit_classify_arch(arch: i32) -> i32 {
    let _ = arch;
    0
}

pub unsafe fn audit_classify_syscall(abi: i32, syscall: u32) -> i32 {
    let _ = abi;
    if syscall == __NR_open {
        AUDITSC_OPEN
    } else if syscall == __NR_openat {
        AUDITSC_OPENAT
    } else if syscall == __NR_execve {
        AUDITSC_EXECVE
    } else if syscall == __NR_openat2 {
        AUDITSC_OPENAT2
    } else {
        AUDITSC_NATIVE
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
