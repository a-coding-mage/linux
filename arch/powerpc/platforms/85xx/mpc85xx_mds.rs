// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2006-2010, 2012-2013 Freescale Semiconductor, Inc.
 * All rights reserved.
 *
 * Author: Andy Fleming <afleming@freescale.com>
 *
 * Based on 83xx/mpc8360e_pb.c by:
 *      Li Yang <LeoLi@freescale.com>
 *      Yin Olivia <Hong-hua.Yin@freescale.com>
 *
 * Description:
 * MPC85xx MDS board specific routines.
 */

// C headers and symbols supplied by the surrounding kernel translation unit.

#[cfg(feature = "phylib")]
const MV88E1111_SCR: u32 = 0x10;
#[cfg(feature = "phylib")]
const MV88E1111_SCR_125CLK: i32 = 0x0010;

#[cfg(feature = "phylib")]
unsafe fn mpc8568_fixup_125_clock(phydev: *mut phy_device) -> i32 {
    let mut scr: i32;
    let mut err: i32;

    /* Workaround for the 125 CLK Toggle */
    scr = phy_read(phydev, MV88E1111_SCR);
    if scr < 0 { return scr; }

    err = phy_write(phydev, MV88E1111_SCR, scr & !MV88E1111_SCR_125CLK);
    if err != 0 { return err; }
    err = phy_write(phydev, MII_BMCR, BMCR_RESET);
    if err != 0 { return err; }

    scr = phy_read(phydev, MV88E1111_SCR);
    if scr < 0 { return scr; }
    err = phy_write(phydev, MV88E1111_SCR, scr | 0x0008);
    err
}

#[cfg(feature = "phylib")]
unsafe fn mpc8568_mds_phy_fixups(phydev: *mut phy_device) -> i32 {
    let mut temp: i32;
    let mut err: i32;

    /* Errata */
    err = phy_write(phydev, 29, 0x0006);
    if err != 0 { return err; }
    temp = phy_read(phydev, 30);
    if temp < 0 { return temp; }
    temp = (temp & !0x8000) | 0x4000;
    err = phy_write(phydev, 30, temp);
    if err != 0 { return err; }
    err = phy_write(phydev, 29, 0x000a);
    if err != 0 { return err; }
    temp = phy_read(phydev, 30);
    if temp < 0 { return temp; }
    temp = phy_read(phydev, 30);
    if temp < 0 { return temp; }
    temp &= !0x0020;
    err = phy_write(phydev, 30, temp);
    if err != 0 { return err; }

    /* Disable automatic MDI/MDIX selection */
    temp = phy_read(phydev, 16);
    if temp < 0 { return temp; }
    temp &= !0x0060;
    phy_write(phydev, 16, temp)
}

#[cfg(feature = "quicc_engine")]
unsafe fn mpc85xx_mds_reset_ucc_phys() {
    let mut np: *mut device_node;
    static mut bcsr_regs: *mut u8 = core::ptr::null_mut();

    /* Map BCSR area */
    np = of_find_node_by_name(core::ptr::null_mut(), "bcsr");
    if np.is_null() { return; }
    bcsr_regs = of_iomap(np, 0);
    of_node_put(np);
    if bcsr_regs.is_null() { return; }

    if machine_is(mpc8568_mds) {
        const BCSR_UCC1_GETH_EN: u8 = 0x1 << 7;
        const BCSR_UCC2_GETH_EN: u8 = 0x1 << 7;
        const BCSR_UCC1_MODE_MSK: u8 = 0x3 << 4;
        const BCSR_UCC2_MODE_MSK: u8 = 0x3 << 0;
        clrbits8(bcsr_regs.add(8), BCSR_UCC1_GETH_EN);
        clrbits8(bcsr_regs.add(9), BCSR_UCC2_GETH_EN);
        clrbits8(bcsr_regs.add(11), BCSR_UCC1_MODE_MSK | BCSR_UCC2_MODE_MSK);
        setbits8(bcsr_regs.add(8), BCSR_UCC1_GETH_EN);
        setbits8(bcsr_regs.add(9), BCSR_UCC2_GETH_EN);
    } else if machine_is(mpc8569_mds) {
        const BCSR7_UCC12_GETHnRST: u8 = 0x1 << 2;
        const BCSR8_UEM_MARVELL_RST: u8 = 0x1 << 1;
        const BCSR_UCC_RGMII: u8 = 0x1 << 6;
        const BCSR_UCC_RTBI: u8 = 0x1 << 5;
        clrbits8(bcsr_regs.add(7), BCSR7_UCC12_GETHnRST);
        setbits8(bcsr_regs.add(8), BCSR8_UEM_MARVELL_RST);
        setbits8(bcsr_regs.add(7), BCSR7_UCC12_GETHnRST);
        clrbits8(bcsr_regs.add(8), BCSR8_UEM_MARVELL_RST);
        for_each_compatible_node(np, "network", "ucc_geth") {
            let prop = of_get_property(np, "cell-index", core::ptr::null_mut());
            if prop.is_null() { continue; }
            let ucc_num = *(prop as *const u32) - 1;
            let prop = of_get_property(np, "phy-connection-type", core::ptr::null_mut());
            if prop.is_null() { continue; }
            if strcmp("rtbi", prop as *const i8) == 0 {
                clrsetbits_8(bcsr_regs.add(7 + ucc_num as usize), BCSR_UCC_RGMII, BCSR_UCC_RTBI);
            }
        }
    } else if machine_is(p1021_mds) {
        const BCSR11_ENET_MICRST: u8 = 0x1 << 5;
        clrbits8(bcsr_regs.add(11), BCSR11_ENET_MICRST);
        setbits8(bcsr_regs.add(11), BCSR11_ENET_MICRST);
    }
    iounmap(bcsr_regs);
}

#[cfg(feature = "quicc_engine")]
unsafe fn mpc85xx_mds_qe_init() {
    let mut np: *mut device_node;
    mpc85xx_qe_par_io_init();
    mpc85xx_mds_reset_ucc_phys();
    if machine_is(p1021_mds) {
        let mut guts: *mut ccsr_guts;
        np = of_find_node_by_name(core::ptr::null_mut(), "global-utilities");
        if !np.is_null() {
            guts = of_iomap(np, 0);
            if guts.is_null() { pr_err!("mpc85xx-rdb: could not map global utilities register\n"); }
            else {
                setbits32(&mut (*guts).pmuxcr, MPC85xx_PMUXCR_QE(0) | MPC85xx_PMUXCR_QE(3) | MPC85xx_PMUXCR_QE(9) | MPC85xx_PMUXCR_QE(12));
                iounmap(guts);
            }
            of_node_put(np);
        }
    }
}

#[cfg(not(feature = "quicc_engine"))]
unsafe fn mpc85xx_mds_qe_init() {}

unsafe fn mpc85xx_mds_setup_arch() {
    if !ppc_md.progress.is_none() { ppc_md.progress.unwrap()("mpc85xx_mds_setup_arch()", 0); }
    mpc85xx_smp_init();
    mpc85xx_mds_qe_init();
    fsl_pci_assign_primary();
    swiotlb_detect_4g();
}

#[cfg(feature = "phylib")]
unsafe fn board_fixups() -> i32 {
    let mut phy_id = [0i8; 20];
    let compstrs = ["fsl,gianfar-mdio", "fsl,ucc-mdio"];
    for compstr in compstrs {
        let mdio = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), compstr);
        let mut res = core::mem::MaybeUninit::<resource>::uninit();
        of_address_to_resource(mdio, 0, res.as_mut_ptr());
        let res = res.assume_init();
        snprintf(phy_id.as_mut_ptr(), phy_id.len(), "%llx:%02x", res.start as u64, 1);
        phy_register_fixup_for_id(phy_id.as_ptr(), Some(mpc8568_fixup_125_clock));
        phy_register_fixup_for_id(phy_id.as_ptr(), Some(mpc8568_mds_phy_fixups));
        snprintf(phy_id.as_mut_ptr(), phy_id.len(), "%llx:%02x", res.start as u64, 7);
        phy_register_fixup_for_id(phy_id.as_ptr(), Some(mpc8568_mds_phy_fixups));
        of_node_put(mdio);
    }
    0
}

unsafe fn mpc85xx_publish_devices() -> i32 { mpc85xx_common_publish_devices() }
unsafe fn mpc85xx_mds_pic_init() {
    let mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU, 0, 256, " OpenPIC  ");
    BUG_ON!(mpic.is_null());
    mpic_init(mpic);
}

// The following define_machine declarations are translated as the platform
// descriptors expected by the surrounding PowerPC machine-definition layer.
define_machine!(mpc8568_mds, "MPC8568 MDS", "MPC85xxMDS", mpc85xx_mds_setup_arch, mpc85xx_mds_pic_init, mpic_get_irq, udbg_progress);
define_machine!(mpc8569_mds, "MPC8569 MDS", "fsl,MPC8569EMDS", mpc85xx_mds_setup_arch, mpc85xx_mds_pic_init, mpic_get_irq, udbg_progress);
define_machine!(p1021_mds, "P1021 MDS", "fsl,P1021MDS", mpc85xx_mds_setup_arch, mpc85xx_mds_pic_init, mpic_get_irq, udbg_progress);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
