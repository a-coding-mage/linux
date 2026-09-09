// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Corenet based SoC DS Setup
 *
 * Maintained by Kumar Gala (see MAINTAINERS for contact information)
 *
 * Copyright 2009-2011 Freescale Semiconductor Inc.
 */

// C dependencies supplied by the surrounding kernel translation unit.

unsafe extern "C" {
    static mut ppc_md: MachineDescription;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;

    fn mpic_alloc(
        node: *mut core::ffi::c_void,
        flags_a: u32,
        flags: u32,
        flags_b: u32,
        nr_irqs: u32,
        name: *const u8,
    ) -> *mut Mpic;
    fn mpic_init(mpic: *mut Mpic);
    fn mpc85xx_smp_init();
    fn swiotlb_detect_4g();
    fn of_platform_bus_probe(
        node: *mut core::ffi::c_void,
        matches: *const OfDeviceId,
        parent: *mut core::ffi::c_void,
    ) -> i32;
    fn of_machine_compatible_match(boards: *const *const u8) -> i32;
    fn of_machine_is_compatible(compat: *const u8) -> i32;
    fn snprintf(dst: *mut u8, size: usize, format: *const u8, ...) -> i32;
    fn ehv_pic_init();
    fn ehv_pic_get_irq() -> i32;
    fn fsl_hv_restart();
    fn fsl_hv_halt();
    fn mpic_get_irq() -> i32;
    fn mpic_get_coreint_irq() -> i32;
    fn udbg_progress();
    fn e500_idle();
    fn fsl_pcibios_fixup_bus();
    fn fsl_pcibios_fixup_phb();
}

#[repr(C)]
struct Mpic;

#[repr(C)]
struct OfDeviceId {
    name: *const u8,
    compatible: *const u8,
}

#[repr(C)]
struct MachineDescription {
    name: *const u8,
    probe: Option<unsafe extern "C" fn() -> i32>,
    setup_arch: Option<unsafe extern "C" fn()>,
    init_IRQ: Option<unsafe extern "C" fn()>,
    pcibios_fixup_bus: Option<unsafe extern "C" fn()>,
    pcibios_fixup_phb: Option<unsafe extern "C" fn()>,
    get_irq: Option<unsafe extern "C" fn() -> i32>,
    progress: Option<unsafe extern "C" fn()>,
    power_save: Option<unsafe extern "C" fn()>,
    ..
}

const MPIC_BIG_ENDIAN: u32 = 0;
const MPIC_SINGLE_DEST_CPU: u32 = 0;
const MPIC_NO_RESET: u32 = 0;
const MPIC_ENABLE_COREINT: u32 = 0;

unsafe fn corenet_gen_pic_init() {
    let mut flags: u32 = MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU | MPIC_NO_RESET;

    // !IS_ENABLED(CONFIG_HOTPLUG_CPU) && !IS_ENABLED(CONFIG_KEXEC_CORE)
    #[cfg(not(any(feature = "CONFIG_HOTPLUG_CPU", feature = "CONFIG_KEXEC_CORE")))]
    {
        flags |= MPIC_ENABLE_COREINT;
    }

    let mpic = mpic_alloc(core::ptr::null_mut(), 0, flags, 0, 512, b" OpenPIC  \0".as_ptr());
    assert!(!mpic.is_null());

    mpic_init(mpic);
}

/*
 * Setup the architecture
 */
unsafe fn corenet_gen_setup_arch() {
    mpc85xx_smp_init();

    swiotlb_detect_4g();

    // pr_info!("%s board\\n", ppc_md.name);
}

static OF_DEVICE_IDS: &[OfDeviceId] = &[
    OfDeviceId { name: core::ptr::null(), compatible: b"simple-bus\0".as_ptr() },
    OfDeviceId { name: core::ptr::null(), compatible: b"mdio-mux-gpio\0".as_ptr() },
    OfDeviceId { name: core::ptr::null(), compatible: b"fsl,fpga-ngpixis\0".as_ptr() },
    OfDeviceId { name: core::ptr::null(), compatible: b"fsl,fpga-qixis\0".as_ptr() },
    OfDeviceId { name: core::ptr::null(), compatible: b"fsl,srio\0".as_ptr() },
    OfDeviceId { name: core::ptr::null(), compatible: b"fsl,p4080-pcie\0".as_ptr() },
    OfDeviceId { name: core::ptr::null(), compatible: b"fsl,qoriq-pcie-v2.2\0".as_ptr() },
    OfDeviceId { name: core::ptr::null(), compatible: b"fsl,qoriq-pcie-v2.3\0".as_ptr() },
    OfDeviceId { name: core::ptr::null(), compatible: b"fsl,qoriq-pcie-v2.4\0".as_ptr() },
    OfDeviceId { name: core::ptr::null(), compatible: b"fsl,qoriq-pcie-v3.0\0".as_ptr() },
    OfDeviceId { name: core::ptr::null(), compatible: b"fsl,qe\0".as_ptr() },
    // The following two are for the Freescale hypervisor.
    OfDeviceId { name: b"hypervisor\0".as_ptr(), compatible: core::ptr::null() },
    OfDeviceId { name: b"handles\0".as_ptr(), compatible: core::ptr::null() },
    OfDeviceId { name: core::ptr::null(), compatible: core::ptr::null() },
];

unsafe fn corenet_gen_publish_devices() -> i32 {
    of_platform_bus_probe(core::ptr::null_mut(), OF_DEVICE_IDS.as_ptr(), core::ptr::null_mut())
}

// machine_arch_initcall(corenet_generic, corenet_gen_publish_devices);

static BOARDS: &[Option<&[u8]>] = &[
    Some(b"fsl,P2041RDB\0"), Some(b"fsl,P3041DS\0"), Some(b"fsl,OCA4080\0"),
    Some(b"fsl,P4080DS\0"), Some(b"fsl,P5020DS\0"), Some(b"fsl,P5040DS\0"),
    Some(b"fsl,T2080QDS\0"), Some(b"fsl,T2080RDB\0"), Some(b"fsl,T2081QDS\0"),
    Some(b"fsl,T4240QDS\0"), Some(b"fsl,T4240RDB\0"), Some(b"fsl,B4860QDS\0"),
    Some(b"fsl,B4420QDS\0"), Some(b"fsl,B4220QDS\0"), Some(b"fsl,T1023RDB\0"),
    Some(b"fsl,T1024QDS\0"), Some(b"fsl,T1024RDB\0"), Some(b"fsl,T1040D4RDB\0"),
    Some(b"fsl,T1042D4RDB\0"), Some(b"fsl,T1040QDS\0"), Some(b"fsl,T1042QDS\0"),
    Some(b"fsl,T1040RDB\0"), Some(b"fsl,T1042RDB\0"), Some(b"fsl,T1042RDB_PI\0"),
    Some(b"keymile,kmcent2\0"), Some(b"keymile,kmcoge4\0"), Some(b"varisys,CYRUS\0"), None,
];

/*
 * Called very early, device-tree isn't unflattened
 */
unsafe fn corenet_generic_probe() -> i32 {
    let mut hv_compat = [0u8; 24];

    if of_machine_compatible_match(BOARDS.as_ptr() as *const *const u8) != 0 {
        return 1;
    }

    /* Check if we're running under the Freescale hypervisor */
    for board in BOARDS.iter().flatten() {
        snprintf(hv_compat.as_mut_ptr(), hv_compat.len(), b"%s-hv\0".as_ptr(), board.as_ptr());
        if of_machine_is_compatible(hv_compat.as_ptr()) != 0 {
            ppc_md.init_IRQ = Some(ehv_pic_init);
            ppc_md.get_irq = Some(ehv_pic_get_irq);
            ppc_md.restart = Some(fsl_hv_restart);
            pm_power_off = Some(fsl_hv_halt);
            ppc_md.halt = Some(fsl_hv_halt);

            // CONFIG_SMP: disable timebase synchronization under the hypervisor.
            return 1;
        }
    }

    0
}

// define_machine(corenet_generic)
#[allow(non_upper_case_globals)]
static mut corenet_generic: MachineDescription = MachineDescription {
    name: b"CoreNet Generic\0".as_ptr(),
    probe: Some(corenet_generic_probe),
    setup_arch: Some(corenet_gen_setup_arch),
    init_IRQ: Some(corenet_gen_pic_init),
    // CONFIG_PCI
    pcibios_fixup_bus: Some(fsl_pcibios_fixup_bus),
    pcibios_fixup_phb: Some(fsl_pcibios_fixup_phb),
    // CONFIG_HOTPLUG_CPU || CONFIG_KEXEC_CORE selects mpic_get_irq.
    get_irq: Some(mpic_get_coreint_irq),
    progress: Some(udbg_progress),
    power_save: Some(e500_idle),
    ..unsafe { core::mem::zeroed() }
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
