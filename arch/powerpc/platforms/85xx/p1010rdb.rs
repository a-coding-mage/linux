// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * P1010RDB Board Setup
 *
 * Copyright 2011 Freescale Semiconductor Inc.
 */

// External declarations supplied by the Linux PowerPC platform code.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct Mpic {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PpcMd {
    pub progress: Option<unsafe extern "C" fn(*const c_char, c_int)>,
}

#[repr(C)]
pub struct MachineDesc {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn() -> c_int>,
    pub setup_arch: Option<unsafe extern "C" fn()>,
    pub init_irq: Option<unsafe extern "C" fn()>,
    pub pcibios_fixup_bus: Option<unsafe extern "C" fn(*mut c_void)>,
    pub pcibios_fixup_phb: Option<unsafe extern "C" fn(*mut c_void)>,
    pub get_irq: Option<unsafe extern "C" fn() -> c_int>,
    pub progress: Option<unsafe extern "C" fn(*const c_char, c_int)>,
}

unsafe extern "C" {
    static mut ppc_md: PpcMd;

    fn mpic_alloc(
        node: *mut c_void,
        flags: c_int,
        mode: c_int,
        first_irq: c_int,
        nr_irqs: c_int,
        name: *const c_char,
    ) -> *mut Mpic;
    fn mpic_init(mpic: *mut Mpic);
    fn fsl_pci_assign_primary();
    fn printk(format: *const c_char, ...);
    fn of_machine_is_compatible(compat: *const c_char) -> c_int;
    fn fsl_pcibios_fixup_bus(bus: *mut c_void);
    fn fsl_pcibios_fixup_phb(phb: *mut c_void);
    fn mpic_get_irq() -> c_int;
    fn udbg_progress(message: *const c_char, value: c_int);
}

// External constants supplied by the included platform headers.
const MPIC_BIG_ENDIAN: c_int = 0x0001;
const MPIC_SINGLE_DEST_CPU: c_int = 0x0002;

unsafe extern "C" {
    fn bug_on(condition: bool);
}

unsafe extern "C" fn p1010_rdb_pic_init() {
    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU,
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
unsafe extern "C" fn p1010_rdb_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(b"p1010_rdb_setup_arch()\0".as_ptr() as *const c_char, 0);
    }

    fsl_pci_assign_primary();

    printk(b"P1010 RDB board from Freescale Semiconductor\n\0".as_ptr() as *const c_char);
}

// machine_arch_initcall(p1010_rdb, mpc85xx_common_publish_devices);

/*
 * Called very early, device-tree isn't unflattened
 */
unsafe extern "C" fn p1010_rdb_probe() -> c_int {
    if of_machine_is_compatible(b"fsl,P1010RDB\0".as_ptr() as *const c_char) != 0 {
        return 1;
    }
    if of_machine_is_compatible(b"fsl,P1010RDB-PB\0".as_ptr() as *const c_char) != 0 {
        return 1;
    }
    0
}

// define_machine(p1010_rdb)
#[no_mangle]
pub static mut p1010_rdb: MachineDesc = MachineDesc {
    name: b"P1010 RDB\0".as_ptr() as *const c_char,
    probe: Some(p1010_rdb_probe),
    setup_arch: Some(p1010_rdb_setup_arch),
    init_irq: Some(p1010_rdb_pic_init),
    // CONFIG_PCI conditional fields are retained here as the source-level
    // equivalents; the surrounding build decides whether PCI is available.
    pcibios_fixup_bus: Some(fsl_pcibios_fixup_bus),
    pcibios_fixup_phb: Some(fsl_pcibios_fixup_phb),
    get_irq: Some(mpic_get_irq),
    progress: Some(udbg_progress),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
