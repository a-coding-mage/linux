// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
 */

// Linux dependencies and build-time module machinery are supplied externally.

/* Registers Offset */
const AL_MC_ECC_CFG: usize = 0x70;
const AL_MC_ECC_CLEAR: usize = 0x7c;
const AL_MC_ECC_ERR_COUNT: usize = 0x80;
const AL_MC_ECC_CE_ADDR0: usize = 0x84;
const AL_MC_ECC_CE_ADDR1: usize = 0x88;
const AL_MC_ECC_UE_ADDR0: usize = 0xa4;
const AL_MC_ECC_UE_ADDR1: usize = 0xa8;
const AL_MC_ECC_CE_SYND0: usize = 0x8c;
const AL_MC_ECC_CE_SYND1: usize = 0x90;
const AL_MC_ECC_CE_SYND2: usize = 0x94;
const AL_MC_ECC_UE_SYND0: usize = 0xac;
const AL_MC_ECC_UE_SYND1: usize = 0xb0;
const AL_MC_ECC_UE_SYND2: usize = 0xb4;

/* Registers Fields */
const AL_MC_ECC_CFG_SCRUB_DISABLED: u32 = 1 << 4;
const AL_MC_ECC_CLEAR_UE_COUNT: u32 = 1 << 3;
const AL_MC_ECC_CLEAR_CE_COUNT: u32 = 1 << 2;
const AL_MC_ECC_CLEAR_UE_ERR: u32 = 1 << 1;
const AL_MC_ECC_CLEAR_CE_ERR: u32 = 1 << 0;
const AL_MC_ECC_ERR_COUNT_UE: u32 = 0xffff0000;
const AL_MC_ECC_ERR_COUNT_CE: u32 = 0x0000ffff;
const AL_MC_ECC_CE_ADDR0_RANK: u32 = 0x03000000;
const AL_MC_ECC_CE_ADDR0_ROW: u32 = 0x0003ffff;
const AL_MC_ECC_CE_ADDR1_BG: u32 = 0x03000000;
const AL_MC_ECC_CE_ADDR1_BANK: u32 = 0x00070000;
const AL_MC_ECC_CE_ADDR1_COLUMN: u32 = 0x00000fff;
const AL_MC_ECC_UE_ADDR0_RANK: u32 = 0x03000000;
const AL_MC_ECC_UE_ADDR0_ROW: u32 = 0x0003ffff;
const AL_MC_ECC_UE_ADDR1_BG: u32 = 0x03000000;
const AL_MC_ECC_UE_ADDR1_BANK: u32 = 0x00070000;
const AL_MC_ECC_UE_ADDR1_COLUMN: u32 = 0x00000fff;

const DRV_NAME: &str = "al_mc_edac";
const AL_MC_EDAC_MSG_MAX: usize = 256;

#[repr(C)]
struct AlMcEdac {
    mmio_base: *mut core::ffi::c_void,
    lock: Spinlock,
    irq_ce: i32,
    irq_ue: i32,
}

unsafe fn prepare_msg(
    message: *mut i8, buffer_size: usize, typ: HwEventMcErrType,
    rank: u8, row: u32, bg: u8, bank: u8, column: u16,
    syn0: u32, syn1: u32, syn2: u32,
) {
    snprintf(message, buffer_size,
        b"%s rank=0x%x row=0x%x bg=0x%x bank=0x%x col=0x%x syn0: 0x%x syn1: 0x%x syn2: 0x%x\0".as_ptr() as *const i8,
        if typ == HW_EVENT_ERR_UNCORRECTED { b"UE\0".as_ptr() } else { b"CE\0".as_ptr() },
        rank as core::ffi::c_uint, row, bg as core::ffi::c_uint, bank as core::ffi::c_uint,
        column as core::ffi::c_uint, syn0, syn1, syn2);
}

unsafe fn handle_ce(mci: *mut MemCtlInfo) -> i32 {
    let al_mc = (*mci).pvt_info as *mut AlMcEdac;
    let mut msg = [0i8; AL_MC_EDAC_MSG_MAX];
    let eccerrcnt = readl_relaxed((*al_mc).mmio_base.add(AL_MC_ECC_ERR_COUNT));
    let ce_count = ((eccerrcnt & AL_MC_ECC_ERR_COUNT_CE) >> 0) as u16;
    if ce_count == 0 { return 0; }
    let ecccaddr0 = readl_relaxed((*al_mc).mmio_base.add(AL_MC_ECC_CE_ADDR0));
    let ecccaddr1 = readl_relaxed((*al_mc).mmio_base.add(AL_MC_ECC_CE_ADDR1));
    let ecccsyn0 = readl_relaxed((*al_mc).mmio_base.add(AL_MC_ECC_CE_SYND0));
    let ecccsyn1 = readl_relaxed((*al_mc).mmio_base.add(AL_MC_ECC_CE_SYND1));
    let ecccsyn2 = readl_relaxed((*al_mc).mmio_base.add(AL_MC_ECC_CE_SYND2));
    writel_relaxed(AL_MC_ECC_CLEAR_CE_COUNT | AL_MC_ECC_CLEAR_CE_ERR, (*al_mc).mmio_base.add(AL_MC_ECC_CLEAR));
    let rank = ((ecccaddr0 & AL_MC_ECC_CE_ADDR0_RANK) >> 24) as u8;
    let row = ecccaddr0 & AL_MC_ECC_CE_ADDR0_ROW;
    let bg = ((ecccaddr1 & AL_MC_ECC_CE_ADDR1_BG) >> 24) as u8;
    let bank = ((ecccaddr1 & AL_MC_ECC_CE_ADDR1_BANK) >> 16) as u8;
    let column = (ecccaddr1 & AL_MC_ECC_CE_ADDR1_COLUMN) as u16;
    prepare_msg(msg.as_mut_ptr(), msg.len(), HW_EVENT_ERR_CORRECTED, rank, row, bg, bank, column, ecccsyn0, ecccsyn1, ecccsyn2);
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*al_mc).lock, &mut flags);
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, ce_count as u32, 0, 0, 0, 0, 0, -1, (*mci).ctl_name, msg.as_mut_ptr());
    spin_unlock_irqrestore(&mut (*al_mc).lock, flags);
    ce_count as i32
}

unsafe fn handle_ue(mci: *mut MemCtlInfo) -> i32 {
    let al_mc = (*mci).pvt_info as *mut AlMcEdac;
    let mut msg = [0i8; AL_MC_EDAC_MSG_MAX];
    let eccerrcnt = readl_relaxed((*al_mc).mmio_base.add(AL_MC_ECC_ERR_COUNT));
    let ue_count = ((eccerrcnt & AL_MC_ECC_ERR_COUNT_UE) >> 16) as u16;
    if ue_count == 0 { return 0; }
    let eccuaddr0 = readl_relaxed((*al_mc).mmio_base.add(AL_MC_ECC_UE_ADDR0));
    let eccuaddr1 = readl_relaxed((*al_mc).mmio_base.add(AL_MC_ECC_UE_ADDR1));
    let eccusyn0 = readl_relaxed((*al_mc).mmio_base.add(AL_MC_ECC_UE_SYND0));
    let eccusyn1 = readl_relaxed((*al_mc).mmio_base.add(AL_MC_ECC_UE_SYND1));
    let eccusyn2 = readl_relaxed((*al_mc).mmio_base.add(AL_MC_ECC_UE_SYND2));
    writel_relaxed(AL_MC_ECC_CLEAR_UE_COUNT | AL_MC_ECC_CLEAR_UE_ERR, (*al_mc).mmio_base.add(AL_MC_ECC_CLEAR));
    let rank = ((eccuaddr0 & AL_MC_ECC_UE_ADDR0_RANK) >> 24) as u8;
    let row = eccuaddr0 & AL_MC_ECC_UE_ADDR0_ROW;
    let bg = ((eccuaddr1 & AL_MC_ECC_UE_ADDR1_BG) >> 24) as u8;
    let bank = ((eccuaddr1 & AL_MC_ECC_UE_ADDR1_BANK) >> 16) as u8;
    let column = (eccuaddr1 & AL_MC_ECC_UE_ADDR1_COLUMN) as u16;
    prepare_msg(msg.as_mut_ptr(), msg.len(), HW_EVENT_ERR_UNCORRECTED, rank, row, bg, bank, column, eccusyn0, eccusyn1, eccusyn2);
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*al_mc).lock, &mut flags);
    edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, ue_count as u32, 0, 0, 0, 0, 0, -1, (*mci).ctl_name, msg.as_mut_ptr());
    spin_unlock_irqrestore(&mut (*al_mc).lock, flags);
    ue_count as i32
}

unsafe fn al_mc_edac_check(mci: *mut MemCtlInfo) {
    let al_mc = (*mci).pvt_info as *mut AlMcEdac;
    if (*al_mc).irq_ue <= 0 { handle_ue(mci); }
    if (*al_mc).irq_ce <= 0 { handle_ce(mci); }
}

unsafe fn al_mc_edac_irq_handler_ue(_irq: i32, info: *mut core::ffi::c_void) -> IrqReturn {
    let mci = platform_get_drvdata(info as *mut PlatformDevice);
    if handle_ue(mci) != 0 { IRQ_HANDLED } else { IRQ_NONE }
}

unsafe fn al_mc_edac_irq_handler_ce(_irq: i32, info: *mut core::ffi::c_void) -> IrqReturn {
    let mci = platform_get_drvdata(info as *mut PlatformDevice);
    if handle_ce(mci) != 0 { IRQ_HANDLED } else { IRQ_NONE }
}

unsafe fn get_scrub_mode(mmio_base: *mut core::ffi::c_void) -> ScrubType {
    if (readl(mmio_base.add(AL_MC_ECC_CFG)) & AL_MC_ECC_CFG_SCRUB_DISABLED) != 0 { SCRUB_NONE } else { SCRUB_HW_SRC }
}

unsafe fn devm_al_mc_edac_free(data: *mut core::ffi::c_void) { edac_mc_free(data as *mut MemCtlInfo); }
unsafe fn devm_al_mc_edac_del(data: *mut core::ffi::c_void) { edac_mc_del_mc(data); }

// The remaining probe, platform-driver registration, and module metadata map directly
// to the corresponding external kernel APIs and structures.
unsafe fn al_mc_edac_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut layers = [EdacMcLayer { typ: EDAC_MC_LAYER_CHIP_SELECT, size: 1, is_virt_csrow: false }];
    let mmio_base = devm_platform_ioremap_resource(pdev, 0);
    if is_err(mmio_base) { return ptr_err(mmio_base); }
    let mci = edac_mc_alloc(0, layers.len(), layers.as_mut_ptr(), core::mem::size_of::<AlMcEdac>());
    if mci.is_null() { return -12; }
    platform_set_drvdata(pdev, mci);
    let al_mc = (*mci).pvt_info as *mut AlMcEdac;
    (*al_mc).mmio_base = mmio_base;
    (*al_mc).irq_ue = of_irq_get_byname((*pdev).dev.of_node, b"ue\0".as_ptr() as *const i8);
    (*al_mc).irq_ce = of_irq_get_byname((*pdev).dev.of_node, b"ce\0".as_ptr() as *const i8);
    if (*al_mc).irq_ue <= 0 || (*al_mc).irq_ce <= 0 { edac_op_state = EDAC_OPSTATE_POLL; (*mci).edac_check = Some(al_mc_edac_check); } else { edac_op_state = EDAC_OPSTATE_INT; }
    spin_lock_init(&mut (*al_mc).lock);
    (*mci).mtype_cap = MEM_FLAG_DDR3 | MEM_FLAG_DDR4;
    (*mci).edac_ctl_cap = EDAC_FLAG_NONE | EDAC_FLAG_SECDED;
    (*mci).edac_cap = EDAC_FLAG_SECDED;
    (*mci).mod_name = DRV_NAME.as_ptr() as *const i8;
    (*mci).ctl_name = b"al_mc\0".as_ptr() as *const i8;
    (*mci).pdev = &mut (*pdev).dev;
    (*mci).scrub_mode = get_scrub_mode(mmio_base);
    edac_mc_add_mc(mci)
}

// External kernel types, constants, and functions referenced above are supplied by
// the surrounding repository translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
