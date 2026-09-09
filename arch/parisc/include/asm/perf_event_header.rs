// Translated from the PA-RISC perf event header.
// The original dependency `<asm/psw.h>` supplies `KERNEL_PSW`.

/// Fetch the caller's register state into `regs`.
///
/// This preserves the original C macro's direct field accesses and inline
/// assembly. The type of `regs` is supplied by the including translation unit.
#[macro_export]
macro_rules! perf_arch_fetch_caller_regs {
    ($regs:expr, $__ip:expr) => {{
        unsafe {
            ($regs).gr[0] = KERNEL_PSW;
            ($regs).iaoq[0] = $__ip;
            core::arch::asm!(
                "copy %sp, {out}",
                out(reg) ($regs).gr[30],
                options(nostack)
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
