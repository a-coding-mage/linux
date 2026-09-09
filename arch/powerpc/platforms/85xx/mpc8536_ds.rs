// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MPC8536 DS Board Setup
 *
 * Copyright 2008 Freescale Semiconductor, Inc.
 */

// C dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn mpic_alloc(
        node: *mut core::ffi::c_void,
        flags: i32,
        irq_offset: i32,
        irq_count: i32,
        nr_irqs: i32,
        name: *const core::ffi::c_char,
    ) -> *mut mpic;
    fn mpic_init(mpic: *mut mpic);
    fn fsl_pci_assign_primary();
    fn swiotlb_detect_4g();
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn fsl_pcibios_fixup_bus(bus: *mut core::ffi::c_void);
    fn fsl_pcibios_fixup_phb(phb: *mut core::ffi::c_void);
    fn mpic_get_irq(regs: *mut core::ffi::c_void) -> i32;
    fn udbg_progress(message: *const core::ffi::c_char, status: u16);
    fn mpc85xx_common_publish_devices() -> i32;
}

#[repr(C)]
pub struct mpic {
    _private: [u8; 0],
}

extern "C" {
    static mut ppc_md: ppc_machine_desc;
}

#[repr(C)]
pub struct ppc_machine_desc {
    pub progress: Option<unsafe extern "C" fn(*const core::ffi::c_char, u16)>,
}

#[inline]
unsafe fn bug_on(condition: bool) {
    if condition {
        core::intrinsics::abort();
    }
}

unsafe extern "C" fn mpc8536_ds_pic_init() {
    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        1, // MPIC_BIG_ENDIAN
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const core::ffi::c_char,
    );
    bug_on(mpic.is_null());
    mpic_init(mpic);
}

/*
 * Setup the architecture
 */
unsafe extern "C" fn mpc8536_ds_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(
            b"mpc8536_ds_setup_arch()\0".as_ptr() as *const core::ffi::c_char,
            0,
        );
    }

    fsl_pci_assign_primary();

    swiotlb_detect_4g();

    printk(b"MPC8536 DS board from Freescale Semiconductor\n\0".as_ptr()
        as *const core::ffi::c_char);
}

// machine_arch_initcall(mpc8536_ds, mpc85xx_common_publish_devices);
#[used]
#[cfg_attr(target_os = "linux", link_section = ".initcall.arch")]
static MPC8536_DS_ARCH_INITCALL: unsafe extern "C" fn() -> i32 =
    mpc85xx_common_publish_devices;

// define_machine(mpc8536_ds)
#[repr(C)]
pub struct MachineDesc {
    pub name: *const core::ffi::c_char,
    pub compatible: *const core::ffi::c_char,
    pub setup_arch: Option<unsafe extern "C" fn()>,
    pub init_irq: Option<unsafe extern "C" fn()>,
    pub pcibios_fixup_bus: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub pcibios_fixup_phb: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub get_irq: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub progress: Option<unsafe extern "C" fn(*const core::ffi::c_char, u16)>,
}

#[used]
#[no_mangle]
pub static mut mpc8536_ds: MachineDesc = MachineDesc {
    name: b"MPC8536 DS\0".as_ptr() as *const core::ffi::c_char,
    compatible: b"fsl,mpc8536ds\0".as_ptr() as *const core::ffi::c_char,
    setup_arch: Some(mpc8536_ds_setup_arch),
    init_irq: Some(mpc8536_ds_pic_init),
    // CONFIG_PCI conditional fields are retained unconditionally as part of
    // the machine descriptor layout; the build configuration may omit them.
    pcibios_fixup_bus: Some(fsl_pcibios_fixup_bus),
    pcibios_fixup_phb: Some(fsl_pcibios_fixup_phb),
    get_irq: Some(mpic_get_irq),
    progress: Some(udbg_progress),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
