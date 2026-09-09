/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 * Copyright (C) 2009 Florian Fainelli <florian@openwrt.org>
 */

// Linux and BCM63xx headers from the original implementation provide the
// constants, macros, types, and external functions referenced below.

pub static mut bcm63xx_regs_base: *const ::core::ffi::c_ulong = ::core::ptr::null();
pub static mut bcm63xx_irqs: *const ::core::ffi::c_int = ::core::ptr::null();
pub static mut bcm63xx_cpu_id: u16 = 0;

static mut bcm63xx_cpu_rev: u8 = 0;
static mut bcm63xx_cpu_freq: u32 = 0;
static mut bcm63xx_memory_size: u32 = 0;

static bcm3368_regs_base: [::core::ffi::c_ulong; 0] = [__GEN_CPU_REGS_TABLE!(3368)];
static bcm3368_irqs: [::core::ffi::c_int; 0] = [__GEN_CPU_IRQ_TABLE!(3368)];
static bcm6328_regs_base: [::core::ffi::c_ulong; 0] = [__GEN_CPU_REGS_TABLE!(6328)];
static bcm6328_irqs: [::core::ffi::c_int; 0] = [__GEN_CPU_IRQ_TABLE!(6328)];
static bcm6338_regs_base: [::core::ffi::c_ulong; 0] = [__GEN_CPU_REGS_TABLE!(6338)];
static bcm6338_irqs: [::core::ffi::c_int; 0] = [__GEN_CPU_IRQ_TABLE!(6338)];
static bcm6345_regs_base: [::core::ffi::c_ulong; 0] = [__GEN_CPU_REGS_TABLE!(6345)];
static bcm6345_irqs: [::core::ffi::c_int; 0] = [__GEN_CPU_IRQ_TABLE!(6345)];
static bcm6348_regs_base: [::core::ffi::c_ulong; 0] = [__GEN_CPU_REGS_TABLE!(6348)];
static bcm6348_irqs: [::core::ffi::c_int; 0] = [__GEN_CPU_IRQ_TABLE!(6348)];
static bcm6358_regs_base: [::core::ffi::c_ulong; 0] = [__GEN_CPU_REGS_TABLE!(6358)];
static bcm6358_irqs: [::core::ffi::c_int; 0] = [__GEN_CPU_IRQ_TABLE!(6358)];
static bcm6362_regs_base: [::core::ffi::c_ulong; 0] = [__GEN_CPU_REGS_TABLE!(6362)];
static bcm6362_irqs: [::core::ffi::c_int; 0] = [__GEN_CPU_IRQ_TABLE!(6362)];
static bcm6368_regs_base: [::core::ffi::c_ulong; 0] = [__GEN_CPU_REGS_TABLE!(6368)];
static bcm6368_irqs: [::core::ffi::c_int; 0] = [__GEN_CPU_IRQ_TABLE!(6368)];

pub unsafe fn bcm63xx_get_cpu_rev() -> u8 { bcm63xx_cpu_rev }
pub unsafe fn bcm63xx_get_cpu_freq() -> u32 { bcm63xx_cpu_freq }
pub unsafe fn bcm63xx_get_memory_size() -> u32 { bcm63xx_memory_size }

unsafe fn detect_cpu_clock() -> u32 {
    let cpu_id: u16 = bcm63xx_get_cpu_id();
    match cpu_id {
        BCM3368_CPU_ID => 300000000,
        BCM6328_CPU_ID => {
            let tmp = bcm_misc_readl(MISC_STRAPBUS_6328_REG);
            let fcvo = (tmp & STRAPBUS_6328_FCVO_MASK) >> STRAPBUS_6328_FCVO_SHIFT;
            match fcvo { 0x12 | 0x14 | 0x19 => 160000000, 0x1c => 192000000,
                0x13 | 0x15 => 200000000, 0x1a => 384000000, 0x16 => 400000000,
                _ => 320000000 }
        }
        BCM6338_CPU_ID => 240000000,
        BCM6345_CPU_ID => 140000000,
        BCM6348_CPU_ID => {
            let tmp = bcm_perf_readl(PERF_MIPSPLLCTL_REG);
            let n1 = ((tmp & MIPSPLLCTL_N1_MASK) >> MIPSPLLCTL_N1_SHIFT) + 1;
            let n2 = ((tmp & MIPSPLLCTL_N2_MASK) >> MIPSPLLCTL_N2_SHIFT) + 2;
            let m1 = ((tmp & MIPSPLLCTL_M1CPU_MASK) >> MIPSPLLCTL_M1CPU_SHIFT) + 1;
            (16 * 1000000 * n1 * n2) / m1
        }
        BCM6358_CPU_ID => {
            let tmp = bcm_ddr_readl(DDR_DMIPSPLLCFG_REG);
            let n1 = (tmp & DMIPSPLLCFG_N1_MASK) >> DMIPSPLLCFG_N1_SHIFT;
            let n2 = (tmp & DMIPSPLLCFG_N2_MASK) >> DMIPSPLLCFG_N2_SHIFT;
            let m1 = (tmp & DMIPSPLLCFG_M1_MASK) >> DMIPSPLLCFG_M1_SHIFT;
            (16 * 1000000 * n1 * n2) / m1
        }
        BCM6362_CPU_ID => {
            let tmp = bcm_misc_readl(MISC_STRAPBUS_6362_REG);
            let fcvo = (tmp & STRAPBUS_6362_FCVO_MASK) >> STRAPBUS_6362_FCVO_SHIFT;
            match fcvo { 0x03 | 0x0b | 0x13 | 0x1b => 240000000,
                0x04 | 0x0c | 0x14 | 0x1c => 160000000,
                0x05 | 0x0e | 0x16 | 0x1e | 0x1f => 400000000, 0x06 => 440000000,
                0x07 | 0x17 => 384000000, 0x15 | 0x1d => 200000000, _ => 320000000 }
        }
        BCM6368_CPU_ID => {
            let tmp = bcm_ddr_readl(DDR_DMIPSPLLCFG_6368_REG);
            let p1 = (tmp & DMIPSPLLCFG_6368_P1_MASK) >> DMIPSPLLCFG_6368_P1_SHIFT;
            let p2 = (tmp & DMIPSPLLCFG_6368_P2_MASK) >> DMIPSPLLCFG_6368_P2_SHIFT;
            let ndiv = (tmp & DMIPSPLLCFG_6368_NDIV_MASK) >> DMIPSPLLCFG_6368_NDIV_SHIFT;
            let tmp = bcm_ddr_readl(DDR_DMIPSPLLDIV_6368_REG);
            let m1 = (tmp & DMIPSPLLDIV_6368_MDIV_MASK) >> DMIPSPLLDIV_6368_MDIV_SHIFT;
            ((64 * 1000000) / p1 * p2 * ndiv) / m1
        }
        _ => panic!("Failed to detect clock for CPU with id=%04X\n", cpu_id),
    }
}

unsafe fn detect_memory_size() -> u32 {
    let mut cols = 0; let mut rows = 0; let mut is_32bits = 0; let mut banks = 0;
    let mut val: u32;
    if BCMCPU_IS_6328!() || BCMCPU_IS_6362!() { return bcm_ddr_readl(DDR_CSEND_REG) << 24; }
    if BCMCPU_IS_6345!() { val = bcm_sdram_readl(SDRAM_MBASE_REG); return val * 8 * 1024 * 1024; }
    if BCMCPU_IS_6338!() || BCMCPU_IS_6348!() {
        val = bcm_sdram_readl(SDRAM_CFG_REG); rows = (val & SDRAM_CFG_ROW_MASK) >> SDRAM_CFG_ROW_SHIFT;
        cols = (val & SDRAM_CFG_COL_MASK) >> SDRAM_CFG_COL_SHIFT; is_32bits = if val & SDRAM_CFG_32B_MASK != 0 { 1 } else { 0 };
        banks = if val & SDRAM_CFG_BANK_MASK != 0 { 2 } else { 1 };
    }
    if BCMCPU_IS_3368!() || BCMCPU_IS_6358!() || BCMCPU_IS_6368!() {
        val = bcm_memc_readl(MEMC_CFG_REG); rows = (val & MEMC_CFG_ROW_MASK) >> MEMC_CFG_ROW_SHIFT;
        cols = (val & MEMC_CFG_COL_MASK) >> MEMC_CFG_COL_SHIFT; is_32bits = if val & MEMC_CFG_32B_MASK != 0 { 0 } else { 1 }; banks = 2;
    }
    rows += 11; cols += 8; 1 << (cols + rows + (is_32bits + 1) + banks)
}

pub unsafe fn bcm63xx_cpu_init() {
    let mut chipid_reg = 0;
    let cpu = smp_processor_id();
    match current_cpu_type() {
        CPU_BMIPS3300 => { if read_c0_prid() & PRID_IMP_MASK != PRID_IMP_BMIPS3300_ALT { __cpu_name[cpu] = "Broadcom BCM6338"; } chipid_reg = BCM_6345_PERF_BASE; }
        CPU_BMIPS32 => chipid_reg = BCM_6345_PERF_BASE,
        CPU_BMIPS4350 => chipid_reg = match read_c0_prid() & PRID_REV_MASK { 0x04 => BCM_3368_PERF_BASE, 0x10 => BCM_6345_PERF_BASE, _ => BCM_6368_PERF_BASE },
        _ => {}
    }
    if chipid_reg == 0 { panic!("unsupported Broadcom CPU"); }
    let tmp = bcm_readl(chipid_reg);
    bcm63xx_cpu_id = ((tmp & REV_CHIPID_MASK) >> REV_CHIPID_SHIFT) as u16;
    bcm63xx_cpu_rev = ((tmp & REV_REVID_MASK) >> REV_REVID_SHIFT) as u8;
    match bcm63xx_cpu_id {
        BCM3368_CPU_ID => { bcm63xx_regs_base = bcm3368_regs_base.as_ptr(); bcm63xx_irqs = bcm3368_irqs.as_ptr(); }
        BCM6328_CPU_ID => { bcm63xx_regs_base = bcm6328_regs_base.as_ptr(); bcm63xx_irqs = bcm6328_irqs.as_ptr(); }
        BCM6338_CPU_ID => { bcm63xx_regs_base = bcm6338_regs_base.as_ptr(); bcm63xx_irqs = bcm6338_irqs.as_ptr(); }
        BCM6345_CPU_ID => { bcm63xx_regs_base = bcm6345_regs_base.as_ptr(); bcm63xx_irqs = bcm6345_irqs.as_ptr(); }
        BCM6348_CPU_ID => { bcm63xx_regs_base = bcm6348_regs_base.as_ptr(); bcm63xx_irqs = bcm6348_irqs.as_ptr(); }
        BCM6358_CPU_ID => { bcm63xx_regs_base = bcm6358_regs_base.as_ptr(); bcm63xx_irqs = bcm6358_irqs.as_ptr(); }
        BCM6362_CPU_ID => { bcm63xx_regs_base = bcm6362_regs_base.as_ptr(); bcm63xx_irqs = bcm6362_irqs.as_ptr(); }
        BCM6368_CPU_ID => { bcm63xx_regs_base = bcm6368_regs_base.as_ptr(); bcm63xx_irqs = bcm6368_irqs.as_ptr(); }
        _ => panic!("unsupported broadcom CPU %x", bcm63xx_cpu_id),
    }
    bcm63xx_cpu_freq = detect_cpu_clock(); bcm63xx_memory_size = detect_memory_size();
    pr_info!("Detected Broadcom 0x%04x CPU revision %02x\n", bcm63xx_cpu_id, bcm63xx_cpu_rev);
    pr_info!("CPU frequency is %u MHz\n", bcm63xx_cpu_freq / 1000000);
    pr_info!("%uMB of RAM installed\n", bcm63xx_memory_size >> 20);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
