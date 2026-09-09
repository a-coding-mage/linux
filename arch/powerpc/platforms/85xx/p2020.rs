// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Freescale P2020 board Setup
 *
 * Copyright 2007,2009,2012-2013 Freescale Semiconductor Inc.
 * Copyright 2022-2023 Pali Rohár <pali@kernel.org>
 */

// C headers omitted; the referenced kernel and architecture symbols are
// supplied by external dependencies.

#[repr(C)]
pub struct Mpic {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

extern "C" {
    static MPIC_BIG_ENDIAN: ::core::ffi::c_int;
    static MPIC_SINGLE_DEST_CPU: ::core::ffi::c_int;

    fn mpic_alloc(
        node: *mut ::core::ffi::c_void,
        isu_size: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        irq_count: ::core::ffi::c_int,
        name: *const ::core::ffi::c_char,
    ) -> *mut Mpic;
    fn mpic_init(mpic: *mut Mpic);
    fn mpc85xx_8259_init();
    fn swiotlb_detect_4g();
    fn fsl_pci_assign_primary();
    fn uli_init();
    fn mpc85xx_smp_init();
    fn mpc85xx_qe_par_io_init();
    fn of_find_node_by_path(path: *const ::core::ffi::c_char) -> *mut DeviceNode;
    fn of_node_put(node: *mut DeviceNode);
    fn fsl_pcibios_fixup_bus(bus: *mut ::core::ffi::c_void);
    fn fsl_pcibios_fixup_phb(phb: *mut ::core::ffi::c_void);
    fn mpic_get_irq(regs: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn udbg_progress(message: *const ::core::ffi::c_char, hex: ::core::ffi::c_uint);
    fn mpc85xx_common_publish_devices();
}

unsafe extern "C" fn p2020_pic_init() {
    let mut mpic: *mut Mpic;
    let flags: ::core::ffi::c_int =
        MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU;

    mpic = mpic_alloc(
        ::core::ptr::null_mut(),
        0,
        flags,
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const ::core::ffi::c_char,
    );

    // Equivalent to WARN_ON(!mpic): warning machinery is supplied externally.
    if mpic.is_null() {
        return;
    }

    mpic_init(mpic);
    mpc85xx_8259_init();
}

/*
 * Setup the architecture
 */
unsafe extern "C" fn p2020_setup_arch() {
    swiotlb_detect_4g();
    fsl_pci_assign_primary();
    uli_init();
    mpc85xx_smp_init();
    mpc85xx_qe_par_io_init();
}

/*
 * Called very early, device-tree isn't unflattened
 */
unsafe extern "C" fn p2020_probe() -> ::core::ffi::c_int {
    let p2020_cpu: *mut DeviceNode;

    /*
     * There is no common compatible string for all P2020 boards.
     * The only common thing is "PowerPC,P2020@0" cpu node.
     * So check for P2020 board via this cpu node.
     */
    p2020_cpu = of_find_node_by_path(b"/cpus/PowerPC,P2020@0\0".as_ptr() as *const ::core::ffi::c_char);
    of_node_put(p2020_cpu);

    if !p2020_cpu.is_null() { 1 } else { 0 }
}

// machine_arch_initcall(p2020, mpc85xx_common_publish_devices);
// define_machine(p2020) {
//     .name = "Freescale P2020",
//     .probe = p2020_probe,
//     .setup_arch = p2020_setup_arch,
//     .init_IRQ = p2020_pic_init,
// #ifdef CONFIG_PCI
//     .pcibios_fixup_bus = fsl_pcibios_fixup_bus,
//     .pcibios_fixup_phb = fsl_pcibios_fixup_phb,
// #endif
//     .get_irq = mpic_get_irq,
//     .progress = udbg_progress,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
