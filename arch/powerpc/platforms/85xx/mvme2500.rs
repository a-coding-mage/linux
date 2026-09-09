// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Board setup routines for the Emerson/Artesyn MVME2500
 *
 * Copyright 2014 Elettra-Sincrotrone Trieste S.C.p.A.
 *
 * Based on earlier code by:
 *
 *	Xianghua Xiao (x.xiao@freescale.com)
 *	Tom Armistead (tom.armistead@emerson.com)
 *	Copyright 2012 Emerson
 *
 * Author Alessio Igor Bogani <alessio.bogani@elettra.eu>
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static mut ppc_md: PpcMd;
    fn mpic_alloc(
        node: *mut c_void,
        flags: c_int,
        isu_shift: c_int,
        irq_offset: c_int,
        irq_count: c_int,
        name: *const c_char,
    ) -> *mut Mpic;
    fn mpic_init(mpic: *mut Mpic);
    fn fsl_pci_assign_primary();
    fn fsl_pcibios_fixup_bus(bus: *mut c_void);
    fn fsl_pcibios_fixup_phb(phb: *mut c_void);
    fn mpic_get_irq(regs: *mut c_void) -> c_int;
    fn udbg_progress(message: *const c_char, value: c_int);
    fn mpc85xx_common_publish_devices() -> c_int;
    fn pr_info(format: *const c_char, ...);
    fn bug_on(condition: bool);
}

#[repr(C)]
pub struct Mpic {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PpcMd {
    pub progress: Option<unsafe extern "C" fn(*const c_char, c_int)>,
}

const MPIC_BIG_ENDIAN: c_int = 1 << 0;
const MPIC_SINGLE_DEST_CPU: c_int = 1 << 1;

unsafe extern "C" fn mvme2500_pic_init() {
    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU,
        0,
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const c_char,
    );
    bug_on(mpic.is_null());
    mpic_init(mpic);
}

/*
 * Setup the architecture
 */
unsafe extern "C" fn mvme2500_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(b"mvme2500_setup_arch()\0".as_ptr() as *const c_char, 0);
    }
    fsl_pci_assign_primary();
    pr_info(b"MVME2500 board from Artesyn\n\0".as_ptr() as *const c_char);
}

// Equivalent of machine_arch_initcall(mvme2500, mpc85xx_common_publish_devices).
#[allow(dead_code)]
static MVME2500_ARCH_INITCALL: unsafe extern "C" fn() -> c_int = mpc85xx_common_publish_devices;

#[repr(C)]
pub struct MachineDesc {
    pub name: *const c_char,
    pub compatible: *const c_char,
    pub setup_arch: Option<unsafe extern "C" fn()>,
    pub init_irq: Option<unsafe extern "C" fn()>,
    // These callbacks are present when CONFIG_PCI is enabled.
    #[cfg(feature = "CONFIG_PCI")]
    pub pcibios_fixup_bus: Option<unsafe extern "C" fn(*mut c_void)>,
    #[cfg(feature = "CONFIG_PCI")]
    pub pcibios_fixup_phb: Option<unsafe extern "C" fn(*mut c_void)>,
    pub get_irq: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub progress: Option<unsafe extern "C" fn(*const c_char, c_int)>,
}

#[no_mangle]
pub static mut mvme2500: MachineDesc = MachineDesc {
    name: b"MVME2500\0".as_ptr() as *const c_char,
    compatible: b"artesyn,MVME2500\0".as_ptr() as *const c_char,
    setup_arch: Some(mvme2500_setup_arch),
    init_irq: Some(mvme2500_pic_init),
    #[cfg(feature = "CONFIG_PCI")]
    pcibios_fixup_bus: Some(fsl_pcibios_fixup_bus),
    #[cfg(feature = "CONFIG_PCI")]
    pcibios_fixup_phb: Some(fsl_pcibios_fixup_phb),
    get_irq: Some(mpic_get_irq),
    progress: Some(udbg_progress),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
