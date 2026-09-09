/*
 * Based on linux/arch/arm/mm/nommu.c
 *
 * ARM PMSAv7 supporting functions.
 */

#[repr(C)]
struct Region {
    base: phys_addr_t,
    size: phys_addr_t,
    subreg: c_ulong,
}

static mut MEM: [Region; MPU_MAX_REGIONS as usize] = [Region { base: 0, size: 0, subreg: 0 }; MPU_MAX_REGIONS as usize];
#[cfg(CONFIG_XIP_KERNEL)]
static mut XIP: [Region; MPU_MAX_REGIONS as usize] = [Region { base: 0, size: 0, subreg: 0 }; MPU_MAX_REGIONS as usize];

static mut MPU_MIN_REGION_ORDER: c_uint = 0;
static mut MPU_MAX_REGIONS_VALUE: c_uint = 0;

extern "C" {
    type c_ulong;
    type c_uint;
    type phys_addr_t;
    static mut mpu_rgn_info: MpuRgnInfo;
    static vectors_base: phys_addr_t;
}

#[cfg(not(CONFIG_CPU_V7M))]
#[inline]
unsafe fn rgnr_write(v: u32) { write_sysreg(v, RNGNR); }
#[cfg(not(CONFIG_CPU_V7M))]
#[inline]
unsafe fn dracr_write(v: u32) { write_sysreg(v, DRACR); }
#[cfg(not(CONFIG_CPU_V7M))]
#[inline]
unsafe fn drsr_write(v: u32) { write_sysreg(v, DRSR); }
#[cfg(not(CONFIG_CPU_V7M))]
#[inline]
unsafe fn drbar_write(v: u32) { write_sysreg(v, DRBAR); }
#[cfg(not(CONFIG_CPU_V7M))]
#[inline]
unsafe fn drbar_read() -> u32 { read_sysreg(DRBAR) }
#[cfg(not(CONFIG_CPU_V7M))]
#[inline]
unsafe fn iracr_write(v: u32) { write_sysreg(v, IRACR); }
#[cfg(not(CONFIG_CPU_V7M))]
#[inline]
unsafe fn irsr_write(v: u32) { write_sysreg(v, IRSR); }
#[cfg(not(CONFIG_CPU_V7M))]
#[inline]
unsafe fn irbar_write(v: u32) { write_sysreg(v, IRBAR); }
#[cfg(not(CONFIG_CPU_V7M))]
#[inline]
unsafe fn irbar_read() -> u32 { read_sysreg(IRBAR) }

#[cfg(CONFIG_CPU_V7M)]
#[inline]
unsafe fn rgnr_write(v: u32) { writel_relaxed(v, BASEADDR_V7M_SCB + PMSAv7_RNR); }
#[cfg(CONFIG_CPU_V7M)]
#[inline]
unsafe fn dracr_write(v: u32) {
    let rsr = readl_relaxed(BASEADDR_V7M_SCB + PMSAv7_RASR) & GENMASK(15, 0);
    writel_relaxed((v << 16) | rsr, BASEADDR_V7M_SCB + PMSAv7_RASR);
}
#[cfg(CONFIG_CPU_V7M)]
#[inline]
unsafe fn drsr_write(v: u32) {
    let racr = readl_relaxed(BASEADDR_V7M_SCB + PMSAv7_RASR) & GENMASK(31, 16);
    writel_relaxed(v | racr, BASEADDR_V7M_SCB + PMSAv7_RASR);
}
#[cfg(CONFIG_CPU_V7M)]
#[inline]
unsafe fn drbar_write(v: u32) { writel_relaxed(v, BASEADDR_V7M_SCB + PMSAv7_RBAR); }
#[cfg(CONFIG_CPU_V7M)]
#[inline]
unsafe fn drbar_read() -> u32 { readl_relaxed(BASEADDR_V7M_SCB + PMSAv7_RBAR) }
#[cfg(CONFIG_CPU_V7M)]
#[inline]
unsafe fn iracr_write(_v: u32) {}
#[cfg(CONFIG_CPU_V7M)]
#[inline]
unsafe fn irsr_write(_v: u32) {}
#[cfg(CONFIG_CPU_V7M)]
#[inline]
unsafe fn irbar_write(_v: u32) {}
#[cfg(CONFIG_CPU_V7M)]
#[inline]
unsafe fn irbar_read() -> c_ulong { 0 }

unsafe fn try_split_region(base: phys_addr_t, size: phys_addr_t, region: *mut Region) -> bool {
    let abase = base & !(size - 1);
    let asize = base + size - abase;
    let mut p2size = 1 << __fls(asize);
    if p2size != asize { p2size *= 2; }
    let bdiff = base - abase;
    let sdiff = p2size - asize;
    let subreg = p2size / PMSAv7_NR_SUBREGS;
    if bdiff % subreg != 0 || sdiff % subreg != 0 { return false; }
    let bslots = bdiff / subreg;
    let sslots = sdiff / subreg;
    if bslots != 0 || sslots != 0 {
        if subreg < PMSAv7_MIN_SUBREG_SIZE || bslots + sslots > PMSAv7_NR_SUBREGS { return false; }
        for i in 0..bslots { _set_bit(i, &mut (*region).subreg); }
        for i in 1..=sslots { _set_bit(PMSAv7_NR_SUBREGS - i, &mut (*region).subreg); }
    }
    (*region).base = abase;
    (*region).size = p2size;
    true
}

unsafe fn allocate_region(mut base: phys_addr_t, mut size: phys_addr_t, limit: c_uint, regions: *mut Region) -> c_int {
    let mut count = 0;
    let mut diff = size;
    let mut attempts = MPU_MAX_REGIONS;
    while diff != 0 {
        if try_split_region(base, size, regions.add(count as usize)) {
            count += 1; base += size; diff -= size; size = diff;
        } else {
            let asize = (base - 1) ^ base;
            let p2size = (1 << __fls(diff)) - 1;
            size = if asize < p2size { asize + 1 } else { p2size + 1 };
        }
        if count as c_uint > limit || attempts == 0 { break; }
        attempts -= 1;
    }
    count
}

unsafe fn __mpu_max_regions() -> c_int {
    let mpuir = read_cpuid_mputype();
    let mut dregions = (mpuir & MPUIR_DREGION_SZMASK) >> MPUIR_DREGION;
    let mut iregions = dregions;
    if (mpuir & MPUIR_nU) != 0 { iregions = (mpuir & MPUIR_IREGION_SZMASK) >> MPUIR_IREGION; }
    min(dregions, iregions) as c_int
}

unsafe fn mpu_iside_independent() -> bool { (read_cpuid_mputype() & MPUIR_nU) != 0 }

unsafe fn __mpu_min_region_order() -> c_int {
    rgnr_write(PMSAv7_PROBE_REGION); isb();
    drbar_write(0xFFFF_FFFC); let mut drbar_result = drbar_read(); let mut irbar_result = drbar_result; drbar_write(0);
    if mpu_iside_independent() { irbar_write(0xFFFF_FFFC); irbar_result = irbar_read(); irbar_write(0); }
    isb(); __ffs(max(drbar_result, irbar_result)) as c_int
}

unsafe fn mpu_setup_region(number: c_uint, start: phys_addr_t, size_order: c_uint, properties: c_uint, subregions: c_uint, need_flush: bool) -> c_int {
    if number > MPU_MAX_REGIONS_VALUE || number >= MPU_MAX_REGIONS { return -ENOENT; }
    if size_order > 32 || size_order < MPU_MIN_REGION_ORDER { return -ENOMEM; }
    let size_data = ((size_order - 1) << PMSAv7_RSR_SZ) | (1 << PMSAv7_RSR_EN) | (subregions << PMSAv7_RSR_SD);
    if need_flush { flush_cache_all(); }
    dsb(); rgnr_write(number); isb(); drbar_write(start); dracr_write(properties); isb(); drsr_write(size_data);
    if mpu_iside_independent() { irbar_write(start); iracr_write(properties); isb(); irsr_write(size_data); }
    isb();
    (*mpu_rgn_info.rgns.add(number as usize)).dracr = properties;
    (*mpu_rgn_info.rgns.add(number as usize)).drbar = start;
    (*mpu_rgn_info.rgns.add(number as usize)).drsr = size_data;
    mpu_rgn_info.used += 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pmsav7_adjust_lowmem_bounds() {
    MPU_MIN_REGION_ORDER = __mpu_min_region_order() as c_uint;
    MPU_MAX_REGIONS_VALUE = __mpu_max_regions() as c_uint;
    let mut mem_max_regions = min(MPU_MAX_REGIONS, MPU_MAX_REGIONS_VALUE) - 1;
    #[cfg(not(CONFIG_CPU_V7M))] { mem_max_regions -= 1; }
    #[cfg(CONFIG_XIP_KERNEL)] {
        let num = allocate_region(CONFIG_XIP_PHYS_ADDR, __pa(_exiprom) - CONFIG_XIP_PHYS_ADDR, mem_max_regions, XIP.as_mut_ptr());
        mem_max_regions -= num as c_uint;
    }
    let mut first = true;
    let mut specified_mem_size: phys_addr_t = 0;
    let mut mem_start: phys_addr_t = 0;
    let mut mem_end: phys_addr_t = 0;
    let mut reg_start: phys_addr_t = 0;
    let mut reg_end: phys_addr_t = 0;
    let mut i: u64 = 0;
    for_each_mem_range(i, &mut reg_start, &mut reg_end) {
        if first {
            if reg_start != PHYS_OFFSET { panic!("First memory bank must be contiguous from PHYS_OFFSET"); }
            mem_start = reg_start; mem_end = reg_end; specified_mem_size = mem_end - mem_start; first = false;
        } else { memblock_remove(reg_start, 0 - reg_start); break; }
    }
    memset(MEM.as_mut_ptr() as *mut u8, 0, core::mem::size_of_val(&MEM));
    let num = allocate_region(mem_start, specified_mem_size, mem_max_regions, MEM.as_mut_ptr());
    let mut total_mem_size: phys_addr_t = 0;
    for i in 0..num as usize {
        let subreg = MEM[i].size / PMSAv7_NR_SUBREGS;
        total_mem_size += MEM[i].size - subreg * hweight_long(MEM[i].subreg);
    }
    if total_mem_size != specified_mem_size { memblock_remove(mem_start + total_mem_size, specified_mem_size - total_mem_size); }
}

#[no_mangle]
pub unsafe extern "C" fn pmsav7_setup() {
    let mut region = 0;
    let mut err = 0;
    err |= mpu_setup_region(region, 0, 32, PMSAv7_ACR_XN | PMSAv7_RGN_STRONGLY_ORDERED | PMSAv7_AP_PL1RW_PL0RW, 0, false);
    region += 1;
    #[cfg(CONFIG_XIP_KERNEL)]
    for i in 0..MPU_MAX_REGIONS as usize {
        if XIP[i].size == 0 { continue; }
        let need_flush = region == PMSAv7_RAM_REGION;
        err |= mpu_setup_region(region, XIP[i].base, ilog2(XIP[i].size), PMSAv7_AP_PL1RO_PL0NA | PMSAv7_RGN_NORMAL, XIP[i].subreg, need_flush);
        region += 1;
    }
    for i in 0..MPU_MAX_REGIONS as usize {
        if MEM[i].size == 0 { continue; }
        err |= mpu_setup_region(region, MEM[i].base, ilog2(MEM[i].size), PMSAv7_AP_PL1RW_PL0RW | PMSAv7_RGN_NORMAL, MEM[i].subreg, false);
        region += 1;
    }
    #[cfg(not(CONFIG_CPU_V7M))]
    { err |= mpu_setup_region(region, vectors_base, ilog2(2 * PAGE_SIZE), PMSAv7_AP_PL1RW_PL0NA | PMSAv7_RGN_NORMAL, 0, false); }
    if err != 0 { panic!("MPU region initialization failure! {}", err); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
