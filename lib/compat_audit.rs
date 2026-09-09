// SPDX-License-Identifier: GPL-2.0

// The C initializers below are populated by the corresponding kernel audit
// headers.  Those generated/header-provided entries remain external to this
// translation; the terminating sentinel is preserved here.
#[no_mangle]
pub static mut compat_dir_class: [u32; 1] = [u32::MAX];

#[no_mangle]
pub static mut compat_read_class: [u32; 1] = [u32::MAX];

#[no_mangle]
pub static mut compat_write_class: [u32; 1] = [u32::MAX];

#[no_mangle]
pub static mut compat_chattr_class: [u32; 1] = [u32::MAX];

#[no_mangle]
pub static mut compat_signal_class: [u32; 1] = [u32::MAX];

// `abi` is retained to preserve the C interface and behavior; the original
// implementation does not inspect it.
#[no_mangle]
pub unsafe extern "C" fn audit_classify_compat_syscall(
    abi: i32,
    syscall: u32,
) -> i32 {
    let _ = abi;
    match syscall {
        // These conditional cases correspond to the C preprocessor checks
        // for the platform's available syscall-number definitions.
        #[cfg(feature = "__NR_open")]
        __NR_open => AUDITSC_OPEN,
        #[cfg(feature = "__NR_openat")]
        __NR_openat => AUDITSC_OPENAT,
        #[cfg(feature = "__NR_socketcall")]
        __NR_socketcall => AUDITSC_SOCKETCALL,
        __NR_execve => AUDITSC_EXECVE,
        #[cfg(feature = "__NR_openat2")]
        __NR_openat2 => AUDITSC_OPENAT2,
        _ => AUDITSC_COMPAT,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
