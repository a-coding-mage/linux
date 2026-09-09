/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 */

// Linux and BCM63xx headers supplied by the surrounding translation unit.

static BCM6348_REGS_ENETDMAC: [u64; 4] = [
    ENETDMAC_CHANCFG_REG,
    ENETDMAC_IR_REG,
    ENETDMAC_IRMASK_REG,
    ENETDMAC_MAXBURST_REG,
];

static BCM6345_REGS_ENETDMAC: [u64; 8] = [
    ENETDMAC_CHANCFG_REG,
    ENETDMAC_IR_REG,
    ENETDMAC_IRMASK_REG,
    ENETDMAC_MAXBURST_REG,
    ENETDMA_6345_BUFALLOC_REG,
    ENETDMA_6345_RSTART_REG,
    ENETDMA_6345_FC_REG,
    ENETDMA_6345_LEN_REG,
];

pub static mut bcm63xx_regs_enetdmac: *const u64 = core::ptr::null();

unsafe fn bcm63xx_enetdmac_regs_init() {
    if BCMCPU_IS_6345() {
        bcm63xx_regs_enetdmac = BCM6345_REGS_ENETDMAC.as_ptr();
    } else {
        bcm63xx_regs_enetdmac = BCM6348_REGS_ENETDMAC.as_ptr();
    }
}

static mut shared_res: [struct_resource; 3] = [
    struct_resource { start: -1, end: -1, flags: IORESOURCE_MEM },
    struct_resource { start: -1, end: -1, flags: IORESOURCE_MEM },
    struct_resource { start: -1, end: -1, flags: IORESOURCE_MEM },
];

static mut bcm63xx_enet_shared_device: platform_device = platform_device {
    name: "bcm63xx_enet_shared",
    id: 0,
    num_resources: 3,
    resource: shared_res.as_mut_ptr(),
    dev: device::default(),
};

static mut shared_device_registered: i32 = 0;
static mut enet_dmamask: u64 = DMA_BIT_MASK(32);

static mut enet0_res: [struct_resource; 4] = [
    struct_resource { start: -1, end: -1, flags: IORESOURCE_MEM },
    struct_resource { start: -1, end: 0, flags: IORESOURCE_IRQ },
    struct_resource { start: -1, end: 0, flags: IORESOURCE_IRQ },
    struct_resource { start: -1, end: 0, flags: IORESOURCE_IRQ },
];
static mut enet0_pd: bcm63xx_enet_platform_data = bcm63xx_enet_platform_data::default();
static mut bcm63xx_enet0_device: platform_device = platform_device::default();

static mut enet1_res: [struct_resource; 4] = [
    struct_resource { start: -1, end: -1, flags: IORESOURCE_MEM },
    struct_resource { start: -1, end: 0, flags: IORESOURCE_IRQ },
    struct_resource { start: -1, end: 0, flags: IORESOURCE_IRQ },
    struct_resource { start: -1, end: 0, flags: IORESOURCE_IRQ },
];
static mut enet1_pd: bcm63xx_enet_platform_data = bcm63xx_enet_platform_data::default();
static mut bcm63xx_enet1_device: platform_device = platform_device::default();

static mut enetsw_res: [struct_resource; 3] = [
    struct_resource { start: 0, end: 0, flags: IORESOURCE_MEM },
    struct_resource { start: 0, end: 0, flags: IORESOURCE_IRQ },
    struct_resource { start: 0, end: 0, flags: IORESOURCE_IRQ },
];
static mut enetsw_pd: bcm63xx_enetsw_platform_data = bcm63xx_enetsw_platform_data::default();
static mut bcm63xx_enetsw_device: platform_device = platform_device::default();

unsafe fn register_shared() -> i32 {
    if shared_device_registered != 0 { return 0; }
    bcm63xx_enetdmac_regs_init();
    shared_res[0].start = bcm63xx_regset_address(RSET_ENETDMA);
    shared_res[0].end = shared_res[0].start;
    shared_res[0].end += if BCMCPU_IS_6345() { RSET_6345_ENETDMA_SIZE } else { RSET_ENETDMA_SIZE } - 1;
    let chan_count = if BCMCPU_IS_6328() || BCMCPU_IS_6362() || BCMCPU_IS_6368() { 32 } else if BCMCPU_IS_6345() { 8 } else { 16 };
    shared_res[1].start = bcm63xx_regset_address(RSET_ENETDMAC);
    shared_res[1].end = shared_res[1].start + RSET_ENETDMAC_SIZE(chan_count) - 1;
    shared_res[2].start = bcm63xx_regset_address(RSET_ENETDMAS);
    shared_res[2].end = shared_res[2].start + RSET_ENETDMAS_SIZE(chan_count) - 1;
    let ret = platform_device_register(&mut bcm63xx_enet_shared_device);
    if ret != 0 { return ret; }
    shared_device_registered = 1;
    0
}

pub unsafe fn bcm63xx_enet_register(unit: i32, pd: *const bcm63xx_enet_platform_data) -> i32 {
    if unit > 1 || (unit == 1 && (BCMCPU_IS_6338() || BCMCPU_IS_6345())) { return -ENODEV; }
    let ret = register_shared(); if ret != 0 { return ret; }
    let (pdev, res, dpd) = if unit == 0 { (&mut bcm63xx_enet0_device, &mut enet0_res, &mut enet0_pd) } else { (&mut bcm63xx_enet1_device, &mut enet1_res, &mut enet1_pd) };
    res[0].start = bcm63xx_regset_address(if unit == 0 { RSET_ENET0 } else { RSET_ENET1 });
    res[0].end = res[0].start + RSET_ENET_SIZE - 1;
    res[1].start = bcm63xx_get_irq_number(if unit == 0 { IRQ_ENET0 } else { IRQ_ENET1 });
    res[2].start = bcm63xx_get_irq_number(if unit == 0 { IRQ_ENET0_RXDMA } else { IRQ_ENET1_RXDMA });
    res[3].start = bcm63xx_get_irq_number(if unit == 0 { IRQ_ENET0_TXDMA } else { IRQ_ENET1_TXDMA });
    core::ptr::copy_nonoverlapping(pd, dpd, 1);
    if dpd.use_internal_phy { if unit == 1 { return -ENODEV; } dpd.phy_id = 1; dpd.has_phy_interrupt = 1; dpd.phy_interrupt = bcm63xx_get_irq_number(IRQ_ENET_PHY); }
    dpd.dma_chan_en_mask = ENETDMAC_CHANCFG_EN_MASK; dpd.dma_chan_int_mask = ENETDMAC_IR_PKTDONE_MASK;
    if BCMCPU_IS_6345() { dpd.dma_chan_en_mask |= ENETDMAC_CHANCFG_CHAINING_MASK | ENETDMAC_CHANCFG_WRAP_EN_MASK | ENETDMAC_CHANCFG_FLOWC_EN_MASK; dpd.dma_chan_int_mask |= ENETDMA_IR_BUFDONE_MASK | ENETDMA_IR_NOTOWNER_MASK; dpd.dma_chan_width = ENETDMA_6345_CHAN_WIDTH; dpd.dma_desc_shift = ENETDMA_6345_DESC_SHIFT; } else { dpd.dma_has_sram = true; dpd.dma_chan_width = ENETDMA_CHAN_WIDTH; }
    if unit == 0 { dpd.rx_chan = 0; dpd.tx_chan = 1; } else { dpd.rx_chan = 2; dpd.tx_chan = 3; }
    platform_device_register(pdev)
}

pub unsafe fn bcm63xx_enetsw_register(pd: *const bcm63xx_enetsw_platform_data) -> i32 {
    if !BCMCPU_IS_6328() && !BCMCPU_IS_6362() && !BCMCPU_IS_6368() { return -ENODEV; }
    let ret = register_shared(); if ret != 0 { return ret; }
    enetsw_res[0].start = bcm63xx_regset_address(RSET_ENETSW); enetsw_res[0].end = enetsw_res[0].start + RSET_ENETSW_SIZE - 1;
    enetsw_res[1].start = bcm63xx_get_irq_number(IRQ_ENETSW_RXDMA0); enetsw_res[2].start = bcm63xx_get_irq_number(IRQ_ENETSW_TXDMA0); if enetsw_res[2].start == 0 { enetsw_res[2].start = -1; }
    core::ptr::copy_nonoverlapping(pd, &mut enetsw_pd, 1);
    if BCMCPU_IS_6328() { enetsw_pd.num_ports = ENETSW_PORTS_6328; } else { enetsw_pd.num_ports = ENETSW_PORTS_6368; }
    enetsw_pd.dma_has_sram = true; enetsw_pd.dma_chan_width = ENETDMA_CHAN_WIDTH; enetsw_pd.dma_chan_en_mask = ENETDMAC_CHANCFG_EN_MASK; enetsw_pd.dma_chan_int_mask = ENETDMAC_IR_PKTDONE_MASK;
    platform_device_register(&mut bcm63xx_enetsw_device)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
