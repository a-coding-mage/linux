// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2010-2011, 2013 Freescale Semiconductor, Inc.
 *
 * Author: Roy Zang <tie-fei.zang@freescale.com>
 *
 * Description:
 * P1023 RDB Board Setup
 */

// Linux and architecture headers from the original source provide these
// declarations and types.

use core::ffi::c_void;

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Mpic {
    _private: [u8; 0],
}

extern "C" {
    static mut ppc_md: PpcMd;
    fn of_find_node_by_name(from: *mut DeviceNode, name: *const u8) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: i32) -> *mut u8;
    fn of_node_put(node: *mut DeviceNode);
    fn iounmap(addr: *mut u8);
    fn clrbits8(addr: *mut u8, clear: u8);
    fn setbits8(addr: *mut u8, set: u8);
    fn mpc85xx_smp_init();
    fn fsl_pci_assign_primary();
    fn mpic_alloc(
        node: *mut c_void,
        flags: u32,
        flags2: u32,
        isu_size: u32,
        irq_count: u32,
        name: *const u8,
    ) -> *mut Mpic;
    fn mpic_init(mpic: *mut Mpic);
    fn mpic_get_irq() -> i32;
    fn udbg_progress(message: *const u8, value: u32);
    fn mpc85xx_common_publish_devices() -> i32;
    fn fsl_pcibios_fixup_bus(bus: *mut c_void);
    fn fsl_pcibios_fixup_phb(phb: *mut c_void);
    fn printk(format: *const u8, ...);
    fn bug_on(condition: bool);
}

#[repr(C)]
pub struct PpcMd {
    pub progress: Option<unsafe extern "C" fn(*const u8, u32)>,
}

const MPIC_BIG_ENDIAN: u32 = 1 << 0;
const MPIC_SINGLE_DEST_CPU: u32 = 1 << 1;

/* ************************************************************************
 *
 * Setup the architecture
 *
 */
unsafe extern "C" fn p1023_rdb_setup_arch() {
    let mut np: *mut DeviceNode;

    if let Some(progress) = ppc_md.progress {
        progress(b"p1023_rdb_setup_arch()\0".as_ptr(), 0);
    }

    /* Map BCSR area */
    np = of_find_node_by_name(core::ptr::null_mut(), b"bcsr\0".as_ptr());
    if !np.is_null() {
        static mut BCSR_REGS: *mut u8 = core::ptr::null_mut();

        BCSR_REGS = of_iomap(np, 0);
        of_node_put(np);

        if BCSR_REGS.is_null() {
            printk(b"BCSR: Failed to map bcsr register space\n\0".as_ptr());
            return;
        } else {
            const BCSR15_I2C_BUS0_SEG_CLR: u8 = 0x07;
            const BCSR15_I2C_BUS0_SEG2: u8 = 0x02;

            /*
             * Note: Accessing exclusively i2c devices.
             *
             * The i2c controller selects initially ID EEPROM in the u-boot;
             * but if menu configuration selects RTC support in the kernel,
             * the i2c controller switches to select RTC chip in the kernel.
             */
            // CONFIG_RTC_CLASS controls this build-time block in the original.
            #[cfg(feature = "CONFIG_RTC_CLASS")]
            {
                /* Enable RTC chip on the segment #2 of i2c */
                clrbits8(BCSR_REGS.add(15), BCSR15_I2C_BUS0_SEG_CLR);
                setbits8(BCSR_REGS.add(15), BCSR15_I2C_BUS0_SEG2);
            }

            iounmap(BCSR_REGS);
        }
    }

    mpc85xx_smp_init();

    fsl_pci_assign_primary();
}

// machine_arch_initcall(p1023_rdb, mpc85xx_common_publish_devices);
#[allow(dead_code)]
static P1023_RDB_MACHINE_ARCH_INITCALL: unsafe extern "C" fn() -> i32 =
    mpc85xx_common_publish_devices;

unsafe extern "C" fn p1023_rdb_pic_init() {
    let mpic: *mut Mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU,
        0,
        256,
        b" OpenPIC  \0".as_ptr(),
    );

    bug_on(mpic.is_null());

    mpic_init(mpic);
}

// Equivalent of define_machine(p1023_rdb) from the original source.
#[repr(C)]
pub struct MachineDesc {
    pub name: *const u8,
    pub compatible: *const u8,
    pub setup_arch: unsafe extern "C" fn(),
    pub init_irq: unsafe extern "C" fn(),
    pub get_irq: unsafe extern "C" fn() -> i32,
    pub progress: unsafe extern "C" fn(*const u8, u32),
    // CONFIG_PCI-controlled callbacks are preserved below.
    pub pcibios_fixup_bus: Option<unsafe extern "C" fn(*mut c_void)>,
    pub pcibios_fixup_phb: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[no_mangle]
pub static mut P1023_RDB: MachineDesc = MachineDesc {
    name: b"P1023 RDB\0".as_ptr(),
    compatible: b"fsl,P1023RDB\0".as_ptr(),
    setup_arch: p1023_rdb_setup_arch,
    init_irq: p1023_rdb_pic_init,
    get_irq: mpic_get_irq,
    progress: udbg_progress,
    pcibios_fixup_bus: Some(fsl_pcibios_fixup_bus),
    pcibios_fixup_phb: Some(fsl_pcibios_fixup_phb),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
