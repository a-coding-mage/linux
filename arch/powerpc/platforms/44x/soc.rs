// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IBM/AMCC PPC4xx SoC setup code
 *
 * Copyright 2008 DENX Software Engineering, Stefan Roese <sr@denx.de>
 *
 * L2 cache routines cloned from arch/ppc/syslib/ibm440gx_common.c which is:
 *   Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
 *   Copyright (c) 2003 - 2006 Zultys Technologies
 */

static mut dcrbase_l2c: u32 = 0;

/* L2-cache */

/* Issue L2C diagnostic command */
#[inline]
unsafe fn l2c_diag(addr: u32) -> u32 {
    mtdcr(dcrbase_l2c + DCRN_L2C0_ADDR, addr);
    mtdcr(dcrbase_l2c + DCRN_L2C0_CMD, L2C_CMD_DIAG);
    while (mfdcr(dcrbase_l2c + DCRN_L2C0_SR) & L2C_SR_CC) == 0 {}
    mfdcr(dcrbase_l2c + DCRN_L2C0_DATA)
}

unsafe fn l2c_error_handler(_irq: i32, _dev: *mut core::ffi::c_void) -> irqreturn_t {
    let sr = mfdcr(dcrbase_l2c + DCRN_L2C0_SR);

    if sr & L2C_SR_CPE != 0 {
        /* Read cache trapped address */
        let addr = l2c_diag(0x42000000);
        printk(KERN_EMERG, "L2C: Cache Parity Error, addr[16:26] = 0x%08x\n", addr);
    }
    if sr & L2C_SR_TPE != 0 {
        /* Read tag trapped address */
        let addr = l2c_diag(0x82000000) >> 16;
        printk(KERN_EMERG, "L2C: Tag Parity Error, addr[16:26] = 0x%08x\n", addr);
    }

    /* Clear parity errors */
    if sr & (L2C_SR_CPE | L2C_SR_TPE) != 0 {
        mtdcr(dcrbase_l2c + DCRN_L2C0_ADDR, 0);
        mtdcr(dcrbase_l2c + DCRN_L2C0_CMD, L2C_CMD_CCP | L2C_CMD_CTE);
    } else {
        printk(KERN_EMERG, "L2C: LRU error\n");
    }

    IRQ_HANDLED
}

unsafe fn ppc4xx_l2c_probe() -> i32 {
    let np: *mut device_node;
    let mut r: u32;
    let mut flags: unsigned_long;
    let irq: i32;
    let dcrreg: *const u32;
    let dcrbase_isram: u32;
    let mut len: i32 = 0;
    let prop: *const u32;
    let l2_size: u32;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,l2-cache");
    if np.is_null() { return 0; }

    /* Get l2 cache size */
    prop = of_get_property(np, "cache-size", core::ptr::null_mut());
    if prop.is_null() {
        printk(KERN_ERR, "%pOF: Can't get cache-size!\n", np);
        of_node_put(np);
        return -ENODEV;
    }
    l2_size = *prop;

    /* Map DCRs */
    dcrreg = of_get_property(np, "dcr-reg", &mut len);
    if dcrreg.is_null() || len != 4 * core::mem::size_of::<u32>() as i32 {
        printk(KERN_ERR, "%pOF: Can't get DCR register base !", np);
        of_node_put(np);
        return -ENODEV;
    }
    dcrbase_isram = *dcrreg;
    dcrbase_l2c = *dcrreg.add(2);

    /* Get and map irq number from device tree */
    irq = irq_of_parse_and_map(np, 0);
    if irq == 0 {
        printk(KERN_ERR, "irq_of_parse_and_map failed\n");
        of_node_put(np);
        return -ENODEV;
    }

    /* Install error handler */
    if request_irq(irq, l2c_error_handler, 0, "L2C", core::ptr::null_mut()) < 0 {
        printk(KERN_ERR, "Cannot install L2C error handler, cache is not enabled\n");
        of_node_put(np);
        return -ENODEV;
    }

    local_irq_save(&mut flags);
    core::arch::asm!("sync", options(nostack, preserves_flags));

    /* Disable SRAM */
    mtdcr(dcrbase_isram + DCRN_SRAM0_DPC, mfdcr(dcrbase_isram + DCRN_SRAM0_DPC) & !SRAM_DPC_ENABLE);
    mtdcr(dcrbase_isram + DCRN_SRAM0_SB0CR, mfdcr(dcrbase_isram + DCRN_SRAM0_SB0CR) & !SRAM_SBCR_BU_MASK);
    mtdcr(dcrbase_isram + DCRN_SRAM0_SB1CR, mfdcr(dcrbase_isram + DCRN_SRAM0_SB1CR) & !SRAM_SBCR_BU_MASK);
    mtdcr(dcrbase_isram + DCRN_SRAM0_SB2CR, mfdcr(dcrbase_isram + DCRN_SRAM0_SB2CR) & !SRAM_SBCR_BU_MASK);
    mtdcr(dcrbase_isram + DCRN_SRAM0_SB3CR, mfdcr(dcrbase_isram + DCRN_SRAM0_SB3CR) & !SRAM_SBCR_BU_MASK);

    /* Enable L2_MODE without ICU/DCU */
    r = mfdcr(dcrbase_l2c + DCRN_L2C0_CFG) & !(L2C_CFG_ICU | L2C_CFG_DCU | L2C_CFG_SS_MASK);
    r |= L2C_CFG_L2M | L2C_CFG_SS_256;
    mtdcr(dcrbase_l2c + DCRN_L2C0_CFG, r);
    mtdcr(dcrbase_l2c + DCRN_L2C0_ADDR, 0);

    /* Hardware Clear Command */
    mtdcr(dcrbase_l2c + DCRN_L2C0_CMD, L2C_CMD_HCC);
    while (mfdcr(dcrbase_l2c + DCRN_L2C0_SR) & L2C_SR_CC == 0) {}
    /* Clear Cache Parity and Tag Errors */
    mtdcr(dcrbase_l2c + DCRN_L2C0_CMD, L2C_CMD_CCP | L2C_CMD_CTE);

    /* Enable 64G snoop region starting at 0 */
    r = mfdcr(dcrbase_l2c + DCRN_L2C0_SNP0) & !(L2C_SNP_BA_MASK | L2C_SNP_SSR_MASK);
    r |= L2C_SNP_SSR_32G | L2C_SNP_ESR;
    mtdcr(dcrbase_l2c + DCRN_L2C0_SNP0, r);
    r = mfdcr(dcrbase_l2c + DCRN_L2C0_SNP1) & !(L2C_SNP_BA_MASK | L2C_SNP_SSR_MASK);
    r |= 0x80000000 | L2C_SNP_SSR_32G | L2C_SNP_ESR;
    mtdcr(dcrbase_l2c + DCRN_L2C0_SNP1, r);

    core::arch::asm!("sync", options(nostack, preserves_flags));
    /* Enable ICU/DCU ports */
    r = mfdcr(dcrbase_l2c + DCRN_L2C0_CFG);
    r &= !(L2C_CFG_DCW_MASK | L2C_CFG_PMUX_MASK | L2C_CFG_PMIM | L2C_CFG_TPEI | L2C_CFG_CPEI | L2C_CFG_NAM | L2C_CFG_NBRM);
    r |= L2C_CFG_ICU | L2C_CFG_DCU | L2C_CFG_TPC | L2C_CFG_CPC | L2C_CFG_FRAN | L2C_CFG_CPIM | L2C_CFG_TPIM | L2C_CFG_LIM | L2C_CFG_SMCM;
    if of_device_is_compatible(np, "ibm,l2-cache-460ex") || of_device_is_compatible(np, "ibm,l2-cache-460gt") { r |= L2C_CFG_RDBW; }
    mtdcr(dcrbase_l2c + DCRN_L2C0_CFG, r);
    core::arch::asm!("sync; isync", options(nostack, preserves_flags));
    local_irq_restore(flags);
    printk(KERN_INFO, "%dk L2-cache enabled\n", l2_size >> 10);
    of_node_put(np);
    0
}

/* Build-time arch_initcall registration is supplied by the surrounding kernel environment. */
arch_initcall!(ppc4xx_l2c_probe);

/* Apply a system reset. Alternatively a board specific value may be provided via the "reset-type" property in the cpu node. */
unsafe fn ppc4xx_reset_system(_cmd: *mut i8) {
    let np = of_get_cpu_node(0, core::ptr::null_mut());
    let mut reset_type = DBCR0_RST_SYSTEM;
    if !np.is_null() {
        let prop = of_get_property(np, "reset-type", core::ptr::null_mut());
        /* Check if property exists and if it is in range: 1 core, 2 chip, 3 system reset. */
        if !prop.is_null() && *prop >= 1 && *prop <= 3 { reset_type = *prop << 28; }
    }
    mtspr(SPRN_DBCR0, mfspr(SPRN_DBCR0) | reset_type);
    loop {}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
