// SPDX-License-Identifier: GPL-2.0-only
/*
 * EISA specific code
 */

// Declarations supplied by the surrounding kernel translation.
extern "C" {
    fn xen_pv_domain() -> bool;
    fn xen_initial_domain() -> bool;
    fn cc_platform_has(attr: i32) -> bool;
    fn memremap(addr: usize, size: usize, flags: usize) -> *mut u32;
    fn memunmap(addr: *mut u32);
    static mut EISA_bus: i32;
}

// These are preprocessor constants in the C source and are supplied by the
// corresponding translated headers.
extern "C" {
    static MEMREMAP_WB: usize;
    static CC_ATTR_GUEST_SEV_SNP: i32;
}

unsafe fn eisa_bus_probe() -> i32 {
    let mut p: *mut u32;

    if (xen_pv_domain() && !xen_initial_domain()) || cc_platform_has(CC_ATTR_GUEST_SEV_SNP) {
        return 0;
    }

    p = memremap(0x0FFFD9, 4, MEMREMAP_WB);
    if !p.is_null()
        && *p == ('E' as u32)
            .wrapping_add(('I' as u32) << 8)
            .wrapping_add(('S' as u32) << 16)
            .wrapping_add(('A' as u32) << 24)
    {
        EISA_bus = 1;
    }
    memunmap(p);
    0
}

// Equivalent to: subsys_initcall(eisa_bus_probe);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
