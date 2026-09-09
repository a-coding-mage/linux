// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MPC85xx RDB Board Setup
 *
 * Copyright 2009,2012-2013 Freescale Semiconductor Inc.
 */

// C dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

extern "C" {
    fn of_machine_is_compatible(compat: *const u8) -> bool;
    fn mpic_alloc(
        node: *mut c_void,
        flags1: i32,
        flags: i32,
        irq_offset: i32,
        irq_count: i32,
        name: *const u8,
    ) -> *mut mpic;
    fn mpic_init(mpic: *mut mpic);
    fn mpc85xx_smp_init();
    fn fsl_pci_assign_primary();
    fn mpc85xx_qe_par_io_init();
    fn machine_is(machine: *const c_void) -> bool;
    fn of_find_node_by_name(from: *mut device_node, name: *const u8) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut ccsr_guts;
    fn iounmap(addr: *mut ccsr_guts);
    fn of_node_put(node: *mut device_node);
    fn setbits32(addr: *mut u32, bits: u32);
    fn mpc85xx_common_publish_devices();
    fn fsl_pcibios_fixup_bus(bus: *mut c_void);
    fn fsl_pcibios_fixup_phb(phb: *mut c_void);
    fn mpic_get_irq(regs: *mut c_void) -> i32;
    fn udbg_progress(message: *const u8, hex: u16);
}

#[repr(C)]
pub struct mpic {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ccsr_guts {
    _pad: [u8; 0xA4],
    pub pmuxcr: u32,
}

#[repr(C)]
pub struct PpcMd {
    pub progress: Option<unsafe extern "C" fn(*const u8, u16)>,
}

extern "C" {
    static mut ppc_md: PpcMd;
}

const MPIC_BIG_ENDIAN: i32 = 1 << 0;
const MPIC_SINGLE_DEST_CPU: i32 = 1 << 1;
const MPIC_NO_RESET: i32 = 1 << 2;

const MPC85XX_PMUXCR_QE_0: u32 = 0;
const MPC85XX_PMUXCR_QE_3: u32 = 0;
const MPC85XX_PMUXCR_QE_9: u32 = 0;
const MPC85XX_PMUXCR_QE_12: u32 = 0;

unsafe extern "C" fn mpc85xx_rdb_pic_init() {
    let mut mpic: *mut mpic;
    let mut flags: i32 = MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU;

    if of_machine_is_compatible(b"fsl,MPC85XXRDB-CAMP\0".as_ptr()) {
        flags |= MPIC_NO_RESET;
    }

    mpic = mpic_alloc(core::ptr::null_mut(), 0, flags, 0, 256, b" OpenPIC  \0".as_ptr());

    if mpic.is_null() {
        return;
    }

    mpic_init(mpic);
}

/* Setup the architecture */
unsafe extern "C" fn mpc85xx_rdb_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(b"mpc85xx_rdb_setup_arch()\0".as_ptr(), 0);
    }

    mpc85xx_smp_init();
    fsl_pci_assign_primary();
    mpc85xx_qe_par_io_init();

    // CONFIG_UCC_GETH || CONFIG_SERIAL_QE
    #[cfg(any(feature = "CONFIG_UCC_GETH", feature = "CONFIG_SERIAL_QE"))]
    {
        if machine_is(core::ptr::addr_of!(p1025_rdb) as *const c_void) {
            let np = of_find_node_by_name(core::ptr::null_mut(), b"global-utilities\0".as_ptr());
            if !np.is_null() {
                let guts = of_iomap(np, 0);
                if guts.is_null() {
                    // pr_err("mpc85xx-rdb: could not map global utilities register\n");
                } else {
                    setbits32(
                        core::ptr::addr_of_mut!((*guts).pmuxcr),
                        MPC85XX_PMUXCR_QE_0
                            | MPC85XX_PMUXCR_QE_3
                            | MPC85XX_PMUXCR_QE_9
                            | MPC85XX_PMUXCR_QE_12,
                    );
                    iounmap(guts);
                }
                of_node_put(np);
            }
        }
    }

    // pr_info("MPC85xx RDB board from Freescale Semiconductor\n");
}

// machine_arch_initcall(p1020_mbg_pc, mpc85xx_common_publish_devices);
// machine_arch_initcall(p1020_rdb, mpc85xx_common_publish_devices);
// machine_arch_initcall(p1020_rdb_pc, mpc85xx_common_publish_devices);
// machine_arch_initcall(p1020_rdb_pd, mpc85xx_common_publish_devices);
// machine_arch_initcall(p1020_utm_pc, mpc85xx_common_publish_devices);
// machine_arch_initcall(p1021_rdb_pc, mpc85xx_common_publish_devices);
// machine_arch_initcall(p1025_rdb, mpc85xx_common_publish_devices);
// machine_arch_initcall(p1024_rdb, mpc85xx_common_publish_devices);

#[repr(C)]
pub struct MachineDesc {
    pub name: *const u8,
    pub compatible: *const u8,
    pub setup_arch: unsafe extern "C" fn(),
    pub init_irq: unsafe extern "C" fn(),
    pub get_irq: unsafe extern "C" fn(*mut c_void) -> i32,
    pub progress: unsafe extern "C" fn(*const u8, u16),
}

macro_rules! define_machine {
    ($id:ident, $name:literal, $compatible:literal) => {
        #[no_mangle]
        pub static $id: MachineDesc = MachineDesc {
            name: concat!($name, "\0").as_ptr(),
            compatible: concat!($compatible, "\0").as_ptr(),
            setup_arch: mpc85xx_rdb_setup_arch,
            init_irq: mpc85xx_rdb_pic_init,
            get_irq: mpic_get_irq,
            progress: udbg_progress,
        };
    };
}

define_machine!(p1020_rdb, "P1020 RDB", "fsl,P1020RDB");
define_machine!(p1021_rdb_pc, "P1021 RDB-PC", "fsl,P1021RDB-PC");
define_machine!(p1025_rdb, "P1025 RDB", "fsl,P1025RDB");
define_machine!(p1020_mbg_pc, "P1020 MBG-PC", "fsl,P1020MBG-PC");
define_machine!(p1020_utm_pc, "P1020 UTM-PC", "fsl,P1020UTM-PC");
define_machine!(p1020_rdb_pc, "P1020RDB-PC", "fsl,P1020RDB-PC");
define_machine!(p1020_rdb_pd, "P1020RDB-PD", "fsl,P1020RDB-PD");
define_machine!(p1024_rdb, "P1024 RDB", "fsl,P1024RDB");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
