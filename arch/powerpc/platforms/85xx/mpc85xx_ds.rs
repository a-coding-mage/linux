// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MPC85xx DS Board Setup
 *
 * Author Xianghua Xiao (x.xiao@freescale.com)
 * Roy Zang <tie-fei.zang@freescale.com>
 * 	- Add PCI/PCI Exprees support
 * Copyright 2007 Freescale Semiconductor Inc.
 */

// C header dependencies are supplied by the surrounding kernel translation.

extern "C" {
    fn of_machine_is_compatible(compat: *const core::ffi::c_char) -> bool;
    fn mpic_alloc(
        node: *mut core::ffi::c_void,
        flags: i32,
        flags2: i32,
        offset: i32,
        irq_count: i32,
        name: *const core::ffi::c_char,
    ) -> *mut Mpic;
    fn mpic_init(mpic: *mut Mpic);
    fn mpc85xx_8259_init();
    fn swiotlb_detect_4g();
    fn fsl_pci_assign_primary();
    fn uli_init();
    fn mpc85xx_smp_init();
    fn fsl_pcibios_fixup_bus(bus: *mut core::ffi::c_void);
    fn fsl_pcibios_fixup_phb(phb: *mut core::ffi::c_void);
    fn mpic_get_irq(regs: *mut core::ffi::c_void) -> i32;
    fn udbg_progress(message: *const core::ffi::c_char, value: u16);
}

#[repr(C)]
pub struct Mpic {
    _private: [u8; 0],
}

// These constants correspond to MPIC_BIG_ENDIAN, MPIC_SINGLE_DEST_CPU,
// and MPIC_NO_RESET supplied by asm/mpic.h.
const MPIC_BIG_ENDIAN: i32 = 1 << 0;
const MPIC_SINGLE_DEST_CPU: i32 = 1 << 1;
const MPIC_NO_RESET: i32 = 1 << 2;

// ppc_md and pr_info are supplied by the machine-dependent kernel layer.
extern "C" {
    static mut ppc_md: PpcMd;
}

#[repr(C)]
pub struct PpcMd {
    pub progress: Option<unsafe extern "C" fn(*const core::ffi::c_char, u16)>,
}

unsafe fn mpc85xx_ds_pic_init() {
    let mut flags: i32 = MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU;

    if of_machine_is_compatible(b"fsl,MPC8572DS-CAMP\0".as_ptr() as *const core::ffi::c_char) {
        flags |= MPIC_NO_RESET;
    }

    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        flags,
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const core::ffi::c_char,
    );

    if mpic.is_null() {
        // WARN_ON(!mpic)
        return;
    }

    mpic_init(mpic);
    mpc85xx_8259_init();
}

/*
 * Setup the architecture
 */
unsafe fn mpc85xx_ds_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(
            b"mpc85xx_ds_setup_arch()\0".as_ptr() as *const core::ffi::c_char,
            0,
        );
    }

    swiotlb_detect_4g();
    fsl_pci_assign_primary();
    uli_init();
    mpc85xx_smp_init();

    // pr_info("MPC85xx DS board from Freescale Semiconductor\n");
}

// machine_arch_initcall(mpc8544_ds, mpc85xx_common_publish_devices);
// machine_arch_initcall(mpc8572_ds, mpc85xx_common_publish_devices);

// The define_machine! records below preserve the two C machine descriptions.
// CONFIG_PCI conditionals are retained as conditional field intent.
#[repr(C)]
pub struct MachineDescription {
    pub name: *const core::ffi::c_char,
    pub compatible: *const core::ffi::c_char,
    pub setup_arch: unsafe fn(),
    pub init_irq: unsafe fn(),
    pub pcibios_fixup_bus: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub pcibios_fixup_phb: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub get_irq: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
    pub progress: unsafe extern "C" fn(*const core::ffi::c_char, u16),
}

#[no_mangle]
pub static mpc8544_ds: MachineDescription = MachineDescription {
    name: b"MPC8544 DS\0".as_ptr() as *const core::ffi::c_char,
    compatible: b"MPC8544DS\0".as_ptr() as *const core::ffi::c_char,
    setup_arch: mpc85xx_ds_setup_arch,
    init_irq: mpc85xx_ds_pic_init,
    pcibios_fixup_bus: Some(fsl_pcibios_fixup_bus),
    pcibios_fixup_phb: Some(fsl_pcibios_fixup_phb),
    get_irq: mpic_get_irq,
    progress: udbg_progress,
};

#[no_mangle]
pub static mpc8572_ds: MachineDescription = MachineDescription {
    name: b"MPC8572 DS\0".as_ptr() as *const core::ffi::c_char,
    compatible: b"fsl,MPC8572DS\0".as_ptr() as *const core::ffi::c_char,
    setup_arch: mpc85xx_ds_setup_arch,
    init_irq: mpc85xx_ds_pic_init,
    pcibios_fixup_bus: Some(fsl_pcibios_fixup_bus),
    pcibios_fixup_phb: Some(fsl_pcibios_fixup_phb),
    get_irq: mpic_get_irq,
    progress: udbg_progress,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
