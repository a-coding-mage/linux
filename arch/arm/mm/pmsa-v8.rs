/*
 * Based on linux/arch/arm/pmsa-v7.c
 *
 * ARM PMSAv8 supporting functions.
 */

// C dependencies: linux/memblock.h, linux/range.h, asm/cp15.h,
// asm/cputype.h, asm/mpu.h, asm/page.h, asm/sections.h, and mm.h.

#[cfg(not(feature = "cpu_v7m"))]
#[inline]
unsafe fn prlar_read() -> u32 {
    read_sysreg(PRLAR)
}

#[cfg(not(feature = "cpu_v7m"))]
#[inline]
unsafe fn prbar_read() -> u32 {
    read_sysreg(PRBAR)
}

#[cfg(not(feature = "cpu_v7m"))]
#[inline]
unsafe fn prsel_write(v: u32) {
    write_sysreg(v, PRSEL);
}

#[cfg(not(feature = "cpu_v7m"))]
#[inline]
unsafe fn prbar_write(v: u32) {
    write_sysreg(v, PRBAR);
}

#[cfg(not(feature = "cpu_v7m"))]
#[inline]
unsafe fn prlar_write(v: u32) {
    write_sysreg(v, PRLAR);
}

#[cfg(feature = "cpu_v7m")]
#[inline]
unsafe fn prlar_read() -> u32 {
    readl_relaxed(BASEADDR_V7M_SCB + PMSAV8_RLAR)
}

#[cfg(feature = "cpu_v7m")]
#[inline]
unsafe fn prbar_read() -> u32 {
    readl_relaxed(BASEADDR_V7M_SCB + PMSAV8_RBAR)
}

#[cfg(feature = "cpu_v7m")]
#[inline]
unsafe fn prsel_write(v: u32) {
    writel_relaxed(v, BASEADDR_V7M_SCB + PMSAV8_RNR);
}

#[cfg(feature = "cpu_v7m")]
#[inline]
unsafe fn prbar_write(v: u32) {
    writel_relaxed(v, BASEADDR_V7M_SCB + PMSAV8_RBAR);
}

#[cfg(feature = "cpu_v7m")]
#[inline]
unsafe fn prlar_write(v: u32) {
    writel_relaxed(v, BASEADDR_V7M_SCB + PMSAV8_RLAR);
}

static mut IO: [range; MPU_MAX_REGIONS] = [range { start: 0, end: 0 }; MPU_MAX_REGIONS];
static mut MEM: [range; MPU_MAX_REGIONS] = [range { start: 0, end: 0 }; MPU_MAX_REGIONS];

static mut mpu_max_regions: c_uint = 0;

#[inline]
unsafe fn is_region_fixed(number: c_int) -> bool {
    match number {
        PMSAv8_XIP_REGION | PMSAv8_KERNEL_REGION => true,
        _ => false,
    }
}

pub unsafe fn pmsav8_adjust_lowmem_bounds() {
    let mut mem_end: phys_addr_t = 0;
    let mut reg_start: phys_addr_t;
    let mut reg_end: phys_addr_t;
    let mut first = true;
    let mut i: u64 = 0;

    for_each_mem_range!(i, &mut reg_start, &mut reg_end) {
        if first {
            let phys_offset: phys_addr_t = PHYS_OFFSET;

            /* Initially only use memory continuous from PHYS_OFFSET */
            if reg_start != phys_offset {
                panic!("First memory bank must be contiguous from PHYS_OFFSET");
            }
            mem_end = reg_end;
            first = false;
        } else {
            /*
             * memblock auto merges contiguous blocks, remove
             * all blocks afterwards in one go (we can't remove
             * blocks separately while iterating)
             */
            pr_notice!("Ignoring RAM after %pa, memory at %pa ignored\n", &mem_end, &reg_start);
            memblock_remove(reg_start, 0u64.wrapping_sub(reg_start));
            break;
        }
    }
}

unsafe fn __mpu_max_regions() -> c_int {
    static mut max_regions: c_int = 0;
    let mut mpuir: u32;

    if max_regions != 0 {
        return max_regions;
    }

    mpuir = read_cpuid_mputype();
    max_regions = ((mpuir & MPUIR_DREGION_SZMASK) >> MPUIR_DREGION) as c_int;
    max_regions
}

unsafe fn __pmsav8_setup_region(number: c_uint, bar: u32, lar: u32) -> c_int {
    if number > mpu_max_regions || number >= MPU_MAX_REGIONS {
        return -ENOENT;
    }

    dsb();
    prsel_write(number);
    isb();
    prbar_write(bar);
    prlar_write(lar);

    mpu_rgn_info.rgns[number as usize].prbar = bar;
    mpu_rgn_info.rgns[number as usize].prlar = lar;
    mpu_rgn_info.used += 1;
    0
}

unsafe fn pmsav8_setup_ram(number: c_uint, start: phys_addr_t, end: phys_addr_t) -> c_int {
    if is_region_fixed(number as c_int) {
        return -EINVAL;
    }

    let mut bar = start as u32;
    let mut lar = ((end - 1) & !(PMSAv8_MINALIGN - 1)) as u32;
    bar |= PMSAv8_AP_PL1RW_PL0RW | PMSAv8_RGN_SHARED;
    lar |= PMSAv8_LAR_IDX(PMSAv8_RGN_NORMAL) | PMSAv8_LAR_EN;
    __pmsav8_setup_region(number, bar, lar)
}

unsafe fn pmsav8_setup_io(number: c_uint, start: phys_addr_t, end: phys_addr_t) -> c_int {
    if is_region_fixed(number as c_int) {
        return -EINVAL;
    }

    let mut bar = start as u32;
    let mut lar = ((end - 1) & !(PMSAv8_MINALIGN - 1)) as u32;
    bar |= PMSAv8_AP_PL1RW_PL0RW | PMSAv8_RGN_SHARED | PMSAv8_BAR_XN;
    lar |= PMSAv8_LAR_IDX(PMSAv8_RGN_DEVICE_nGnRnE) | PMSAv8_LAR_EN;
    __pmsav8_setup_region(number, bar, lar)
}

unsafe fn pmsav8_setup_fixed(number: c_uint, start: phys_addr_t, end: phys_addr_t) -> c_int {
    if !is_region_fixed(number as c_int) {
        return -EINVAL;
    }

    let bar = (start as u32) | PMSAv8_AP_PL1RW_PL0NA | PMSAv8_RGN_SHARED;
    let lar = (((end - 1) & !(PMSAv8_MINALIGN - 1)) as u32)
        | PMSAv8_LAR_IDX(PMSAv8_RGN_NORMAL) | PMSAv8_LAR_EN;

    prsel_write(number);
    isb();
    if prbar_read() != bar || prlar_read() != lar {
        return -EINVAL;
    }

    /* Reserved region was set up early, we just need a record for secondaries */
    mpu_rgn_info.rgns[number as usize].prbar = bar;
    mpu_rgn_info.rgns[number as usize].prlar = lar;
    mpu_rgn_info.used += 1;
    0
}

#[cfg(not(feature = "cpu_v7m"))]
unsafe fn pmsav8_setup_vector(number: c_uint, start: phys_addr_t, end: phys_addr_t) -> c_int {
    if number as c_int == PMSAv8_KERNEL_REGION {
        return -EINVAL;
    }

    let bar = (start as u32) | PMSAv8_AP_PL1RW_PL0NA | PMSAv8_RGN_SHARED;
    let lar = (((end - 1) & !(PMSAv8_MINALIGN - 1)) as u32)
        | PMSAv8_LAR_IDX(PMSAv8_RGN_NORMAL) | PMSAv8_LAR_EN;
    __pmsav8_setup_region(number, bar, lar)
}

pub unsafe fn pmsav8_setup() {
    let mut i: c_int;
    let mut err: c_int = 0;
    let mut region: c_int = PMSAv8_KERNEL_REGION;

    mpu_max_regions = __mpu_max_regions() as c_uint;

    add_range!(MEM, ARRAY_SIZE!(MEM), 0, memblock.memory.regions[0].base,
        memblock.memory.regions[0].base + memblock.memory.regions[0].size);
    add_range!(IO, ARRAY_SIZE!(IO), 0, 0, 0xffffffff);

    subtract_range!(MEM, ARRAY_SIZE!(MEM), __pa(KERNEL_START), __pa(KERNEL_END));
    subtract_range!(IO, ARRAY_SIZE!(IO), __pa(KERNEL_START), __pa(KERNEL_END));

    // #ifdef CONFIG_XIP_KERNEL
    #[cfg(feature = "xip_kernel")]
    {
        subtract_range!(MEM, ARRAY_SIZE!(MEM), CONFIG_XIP_PHYS_ADDR, __pa(_exiprom));
        subtract_range!(IO, ARRAY_SIZE!(IO), CONFIG_XIP_PHYS_ADDR, __pa(_exiprom));
    }

    #[cfg(not(feature = "cpu_v7m"))]
    {
        subtract_range!(MEM, ARRAY_SIZE!(MEM), vectors_base, vectors_base + 2 * PAGE_SIZE);
        subtract_range!(IO, ARRAY_SIZE!(IO), vectors_base, vectors_base + 2 * PAGE_SIZE);
    }

    i = 0;
    while i < ARRAY_SIZE!(MEM) as c_int {
        subtract_range!(IO, ARRAY_SIZE!(IO), MEM[i as usize].start, MEM[i as usize].end);
        i += 1;
    }

    // #ifdef CONFIG_XIP_KERNEL
    #[cfg(feature = "xip_kernel")]
    {
        err |= pmsav8_setup_fixed(PMSAv8_XIP_REGION, CONFIG_XIP_PHYS_ADDR, __pa(_exiprom));
    }
    err |= pmsav8_setup_fixed(region as c_uint, __pa(KERNEL_START), __pa(KERNEL_END));
    region += 1;

    i = 0;
    while i < ARRAY_SIZE!(IO) as c_int {
        if IO[i as usize].end != 0 {
            err |= pmsav8_setup_io(region as c_uint, IO[i as usize].start, IO[i as usize].end);
            region += 1;
        }
        i += 1;
    }

    i = 0;
    while i < ARRAY_SIZE!(MEM) as c_int {
        if MEM[i as usize].end != 0 {
            err |= pmsav8_setup_ram(region as c_uint, MEM[i as usize].start, MEM[i as usize].end);
            region += 1;
        }
        i += 1;
    }

    #[cfg(not(feature = "cpu_v7m"))]
    {
        err |= pmsav8_setup_vector(region as c_uint, vectors_base, vectors_base + 2 * PAGE_SIZE);
    }

    if err != 0 {
        pr_warn!("MPU region initialization failure! {}", err);
    } else {
        pr_info!("Using ARM PMSAv8 Compliant MPU. Used {} of {} regions\n",
            mpu_rgn_info.used, mpu_max_regions);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
