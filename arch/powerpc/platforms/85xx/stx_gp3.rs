// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Based on MPC8560 ADS and arch/ppc stx_gp3 ports
 *
 * Maintained by Kumar Gala (see MAINTAINERS for contact information)
 *
 * Copyright 2008 Freescale Semiconductor Inc.
 *
 * Dan Malek <dan@embeddededge.com>
 * Copyright 2004 Embedded Edge, LLC
 *
 * Copied from mpc8560_ads.c
 * Copyright 2002, 2003 Motorola Inc.
 *
 * Ported to 2.6, Matt Porter <mporter@kernel.crashing.org>
 * Copyright 2004-2005 MontaVista Software, Inc.
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/stddef.h, linux/kernel.h, linux/pci.h, linux/kdev_t.h,
// linux/delay.h, linux/seq_file.h, linux/of.h, asm/time.h,
// asm/machdep.h, asm/pci-bridge.h, asm/mpic.h, mm/mmu_decl.h,
// asm/udbg.h, sysdev/fsl_soc.h, sysdev/fsl_pci.h, and mpc85xx.h.
// CONFIG_CPM2 conditionally supplies asm/cpm2.h.

extern "C" {
    static mut ppc_md: PpcMachineDescription;

    fn mpic_alloc(
        node: *mut core::ffi::c_void,
        flags: u32,
        reg_type: u32,
        offset: u32,
        irq_count: u32,
        name: *const core::ffi::c_char,
    ) -> *mut Mpic;
    fn mpic_init(mpic: *mut Mpic);
    fn mpc85xx_cpm2_pic_init();
    fn fsl_pci_assign_primary();
    fn cpm2_reset();
    fn mfspr(spr: u32) -> u32;
    fn seq_printf(m: *mut SeqFile, format: *const core::ffi::c_char, ...);
    fn mpic_get_irq() -> u32;
    fn udbg_progress(message: *const core::ffi::c_char, value: u32);
    fn bug_on(condition: bool);
}

#[repr(C)]
struct Mpic;

#[repr(C)]
struct SeqFile;

#[repr(C)]
struct PpcMachineDescription {
    progress: Option<unsafe extern "C" fn(*const core::ffi::c_char, u32)>,
}

const MPIC_BIG_ENDIAN: u32 = 0;
const SPRN_PVR: u32 = 0;
const SPRN_SVR: u32 = 0;
const SPRN_HID1: u32 = 0;

unsafe fn stx_gp3_pic_init() {
    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        MPIC_BIG_ENDIAN,
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const core::ffi::c_char,
    );
    bug_on(mpic.is_null());
    mpic_init(mpic);

    mpc85xx_cpm2_pic_init();
}

/*
 * Setup the architecture
 */
unsafe fn stx_gp3_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(
            b"stx_gp3_setup_arch()\0".as_ptr() as *const core::ffi::c_char,
            0,
        );
    }

    fsl_pci_assign_primary();

    // #ifdef CONFIG_CPM2
    cpm2_reset();
    // #endif
}

unsafe fn stx_gp3_show_cpuinfo(m: *mut SeqFile) {
    let pvid: u32;
    let svid: u32;
    let phid1: u32;

    pvid = mfspr(SPRN_PVR);
    svid = mfspr(SPRN_SVR);

    seq_printf(m, b"Vendor\t\t: RPC Electronics STx\n\0".as_ptr() as *const core::ffi::c_char);
    seq_printf(m, b"PVR\t\t: 0x%x\n\0".as_ptr() as *const core::ffi::c_char, pvid);
    seq_printf(m, b"SVR\t\t: 0x%x\n\0".as_ptr() as *const core::ffi::c_char, svid);

    /* Display cpu Pll setting */
    phid1 = mfspr(SPRN_HID1);
    seq_printf(
        m,
        b"PLL setting\t: 0x%x\n\0".as_ptr() as *const core::ffi::c_char,
        (phid1 >> 24) & 0x3f,
    );
}

// machine_arch_initcall(stx_gp3, mpc85xx_common_publish_devices);
// define_machine(stx_gp3) {
//     .name = "STX GP3",
//     .compatible = "stx,gp3-8560",
//     .setup_arch = stx_gp3_setup_arch,
//     .init_IRQ = stx_gp3_pic_init,
//     .show_cpuinfo = stx_gp3_show_cpuinfo,
//     .get_irq = mpic_get_irq,
//     .progress = udbg_progress,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
