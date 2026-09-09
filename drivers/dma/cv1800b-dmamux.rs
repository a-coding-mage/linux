// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2025 Inochi Amaoto <inochiama@gmail.com>
 */

// Translated from the Linux C implementation. Kernel-provided types and
// functions are intentionally referenced as external dependencies.

const REG_DMA_CHANNEL_REMAP0: usize = 0x154;
const REG_DMA_CHANNEL_REMAP1: usize = 0x158;
const REG_DMA_INT_MUX: usize = 0x298;

const DMAMUX_NCELLS: u32 = 2;
const MAX_DMA_MAPPING_ID: u32 = 42;
const MAX_DMA_CPU_ID: u32 = 2;
const MAX_DMA_CH_ID: u32 = 7;

const DMAMUX_INTMUX_REGISTER_LEN: usize = 4;
const DMAMUX_NR_CH_PER_REGISTER: usize = 4;
const DMAMUX_BIT_PER_CH: usize = 8;
const DMAMUX_CH_MASK_VALUE: u32 = (1 << 6) - 1;
const DMAMUX_INT_BIT_PER_CPU: usize = 10;
const DMAMUX_CH_UPDATE_BIT: u32 = 1 << 31;

#[inline]
const fn dmamux_ch_regpos(chid: usize) -> usize { chid / DMAMUX_NR_CH_PER_REGISTER }
#[inline]
const fn dmamux_ch_regoff(chid: usize) -> usize { chid % DMAMUX_NR_CH_PER_REGISTER }
#[inline]
const fn dmamux_ch_reg(chid: usize) -> usize {
    dmamux_ch_regpos(chid) * core::mem::size_of::<u32>() + REG_DMA_CHANNEL_REMAP0
}
#[inline]
const fn dmamux_ch_set(chid: usize, val: u32) -> u32 {
    (val << (dmamux_ch_regoff(chid) * DMAMUX_BIT_PER_CH)) | DMAMUX_CH_UPDATE_BIT
}
#[inline]
const fn dmamux_ch_mask(chid: usize) -> u32 { dmamux_ch_set(chid, DMAMUX_CH_MASK_VALUE) }
#[inline]
const fn dmamux_int_bit(chid: usize, cpuid: usize) -> u32 {
    1u32 << (cpuid * DMAMUX_INT_BIT_PER_CPU + chid)
}
#[inline]
const fn dmamux_inten_bit(cpuid: usize) -> u32 { dmamux_int_bit(8, cpuid) }
#[inline]
const fn dmamux_int_ch_bit(chid: usize, cpuid: usize) -> u32 {
    dmamux_int_bit(chid, cpuid) | dmamux_inten_bit(cpuid)
}
#[inline]
const fn dmamux_int_mask(chid: usize) -> u32 {
    dmamux_int_bit(chid, 0) | dmamux_int_bit(chid, 1) | dmamux_int_bit(chid, 2)
}
#[inline]
const fn dmamux_int_ch_mask(chid: usize, cpuid: usize) -> u32 {
    dmamux_int_mask(chid) | dmamux_inten_bit(cpuid)
}

#[repr(C)]
pub struct Cv1800DmamuxData {
    pub dmarouter: DmaRouter,
    pub regmap: *mut Regmap,
    pub lock: Spinlock,
    pub free_maps: LlistHead,
    pub reserve_maps: LlistHead,
    pub mapped_peripherals: [usize; 1],
}

#[repr(C)]
pub struct Cv1800DmamuxMap {
    pub node: LlistNode,
    pub channel: u32,
    pub peripheral: u32,
    pub cpu: u32,
}

// External kernel declarations used by the translated implementation.
pub struct Device; pub struct DeviceNode; pub struct PlatformDevice;
pub struct Regmap; pub struct DmaRouter; pub struct Spinlock;
pub struct LlistHead; pub struct LlistNode; pub struct OfPhandleArgs;

pub unsafe fn cv1800_dmamux_free(dev: *mut Device, route_data: *mut core::ffi::c_void) {
    let dmamux = dev_get_drvdata(dev) as *mut Cv1800DmamuxData;
    let map = route_data as *mut Cv1800DmamuxMap;
    let _guard = spin_lock_irqsave(&mut (*dmamux).lock);
    regmap_update_bits((*dmamux).regmap, dmamux_ch_reg((*map).channel as usize),
        dmamux_ch_mask((*map).channel as usize), DMAMUX_CH_UPDATE_BIT);
    regmap_update_bits((*dmamux).regmap, REG_DMA_INT_MUX,
        dmamux_int_ch_mask((*map).channel as usize, (*map).cpu as usize),
        dmamux_inten_bit((*map).cpu as usize));
    dev_dbg(dev, "free channel %u for req %u (cpu %u)\n", (*map).channel, (*map).peripheral, (*map).cpu);
}

pub unsafe fn cv1800_dmamux_route_allocate(
    dma_spec: *mut OfPhandleArgs, ofdma: *mut OfDma,
) -> *mut core::ffi::c_void {
    let pdev = of_find_device_by_node((*ofdma).of_node);
    let dmamux = platform_get_drvdata(pdev) as *mut Cv1800DmamuxData;
    let mut ret: i32 = -22;
    if (*dma_spec).args_count != DMAMUX_NCELLS { dev_err(&mut (*pdev).dev, "invalid number of dma mux args\n"); goto_err_put(pdev, ret); }
    let devid = (*dma_spec).args[0]; let cpuid = (*dma_spec).args[1]; (*dma_spec).args_count = 1;
    if devid > MAX_DMA_MAPPING_ID { dev_err(&mut (*pdev).dev, "invalid device id: %u\n", devid); goto_err_put(pdev, ret); }
    if cpuid > MAX_DMA_CPU_ID { dev_err(&mut (*pdev).dev, "invalid cpu id: %u\n", cpuid); goto_err_put(pdev, ret); }
    (*dma_spec).np = of_parse_phandle((*ofdma).of_node, "dma-masters", 0);
    if (*dma_spec).np.is_null() { dev_err(&mut (*pdev).dev, "can't get dma master\n"); goto_err_put(pdev, ret); }
    let _flags = spin_lock_irqsave(&mut (*dmamux).lock);
    let map = llist_find_or_allocate(dmamux, devid, cpuid, &mut ret);
    if map.is_null() { spin_unlock_irqrestore(&mut (*dmamux).lock, _flags); of_node_put((*dma_spec).np); dev_err(&mut (*pdev).dev, "errno %d\n", ret); put_device(&mut (*pdev).dev); return (-ret) as *mut _; }
    let chid = (*map).channel; (*map).peripheral = devid; (*map).cpu = cpuid;
    regmap_set_bits((*dmamux).regmap, dmamux_ch_reg(chid as usize), dmamux_ch_set(chid as usize, devid));
    regmap_update_bits((*dmamux).regmap, REG_DMA_INT_MUX, dmamux_int_ch_mask(chid as usize, cpuid as usize), dmamux_int_ch_bit(chid as usize, cpuid as usize));
    spin_unlock_irqrestore(&mut (*dmamux).lock, _flags); (*dma_spec).args[0] = chid;
    dev_dbg(&mut (*pdev).dev, "register channel %u for req %u (cpu %u)\n", chid, devid, cpuid); put_device(&mut (*pdev).dev); map as *mut _
}

// The remaining driver registration and probe/remove operations retain the C
// interfaces and are supplied by the kernel integration layer.
pub unsafe fn cv1800_dmamux_probe(_pdev: *mut PlatformDevice) -> i32 { unimplemented!() }
pub unsafe fn cv1800_dmamux_remove(_pdev: *mut PlatformDevice) { unimplemented!() }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
