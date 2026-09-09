// SPDX-License-Identifier: GPL-2.0
/*
 * x86 FPU bug checks:
 */

/* Dependency declarations supplied by the surrounding kernel translation. */
extern "C" {
    static mut boot_cpu_data: CpuData;
    fn boot_cpu_has(feature: i32) -> bool;
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
    fn set_cpu_bug(cpu: *mut CpuData, bug: i32);
    fn pr_warn(format: *const u8, ...);
}

#[repr(C)]
pub struct CpuData {
    _private: [u8; 0],
}

pub const X86_FEATURE_FPU: i32 = 0;
pub const X86_BUG_FDIV: i32 = 0;

/*
 * Boot time CPU/FPU FDIV bug detection code:
 */

static mut x: f64 = 4195835.0;
static mut y: f64 = 3145727.0;

/*
 * This used to check for exceptions..
 * However, it turns out that to support that,
 * the XMM trap handlers basically had to
 * be buggy. So let's have a correct XMM trap
 * handler, and forget about printing out
 * some status at boot.
 *
 * We should really only care about bugs here
 * anyway. Not features.
 */
pub unsafe fn fpu__init_check_bugs() {
    let mut fdiv_bug: i32;

    /* kernel_fpu_begin/end() relies on patched alternative instructions. */
    if !boot_cpu_has(X86_FEATURE_FPU) {
        return;
    }

    kernel_fpu_begin();

    /*
     * trap_init() enabled FXSR and company _before_ testing for FP
     * problems here.
     *
     * Test for the divl bug: http://en.wikipedia.org/wiki/Fdiv_bug
     */
    core::arch::asm!(
        "fninit",
        "fldl ({x})",
        "fdivl ({y})",
        "fmull ({y})",
        "fldl ({x})",
        "fsubp %st, %st(1)",
        "fistpl ({bug})",
        "fwait",
        "fninit",
        x = in(reg) &x,
        y = in(reg) &y,
        bug = in(reg) &mut fdiv_bug,
        options(nostack),
    );

    kernel_fpu_end();

    if fdiv_bug != 0 {
        set_cpu_bug(&mut boot_cpu_data, X86_BUG_FDIV);
        pr_warn(b"Hmm, FPU with FDIV bug\n\0".as_ptr());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
