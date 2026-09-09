// Translated from: <xen/arm/page.h>
// Translated from: <asm/mmu.h>

extern "C" {
    fn arm64_kernel_unmapped_at_el0() -> bool;
}

#[inline]
fn xen_kernel_unmapped_at_usr() -> bool {
    unsafe { arm64_kernel_unmapped_at_el0() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
