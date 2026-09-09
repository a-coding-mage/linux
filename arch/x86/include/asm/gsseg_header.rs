/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[cfg(target_arch = "x86_64")]
extern "C" {
    pub fn asm_load_gs_index(selector: u16);
}

// Replace with `lkgs %di` once assembler support for the LKGS instruction is
// available.  This is the byte encoding of LKGS with the selector in DI.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn native_lkgs(selector: u32) {
    let mut sel = selector as u16;
    core::arch::asm!(
        ".byte 0xf2, 0x0f, 0x00, 0xf7",
        inout("di") sel,
        options(nostack, preserves_flags),
    );
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn native_load_gs_index(selector: u32) {
    if cpu_feature_enabled(X86_FEATURE_LKGS) {
        native_lkgs(selector);
    } else {
        let mut flags: usize;
        core::arch::asm!("", lateout("ax") flags, options(nomem, nostack, preserves_flags));
        local_irq_save(&mut flags);
        asm_load_gs_index(selector as u16);
        local_irq_restore(flags);
    }
}

#[inline(always)]
pub fn lkgs_init() {
    // CONFIG_PARAVIRT_XXL and CONFIG_X86_64 are build-time kernel conditions.
    #[cfg(all(feature = "paravirt_xxl", target_arch = "x86_64"))]
    unsafe {
        if cpu_feature_enabled(X86_FEATURE_LKGS) {
            pv_ops.cpu.load_gs_index = Some(native_lkgs);
        }
    }
}

// CONFIG_PARAVIRT_XXL is a build-time kernel condition.
#[cfg(not(feature = "paravirt_xxl"))]
#[inline(always)]
pub unsafe fn load_gs_index(selector: u32) {
    #[cfg(target_arch = "x86_64")]
    {
        native_load_gs_index(selector);
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        loadsegment!(gs, selector);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
