// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2020 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

pub type bool_ = bool;
pub type size_t = usize;
pub type dma_addr_t = u64;
pub type u8 = u8;
pub type u16 = u16;
pub type u32 = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_device {
    pub dev: *mut device,
}

#[repr(C)]
pub struct dma_chan {
    pub device: *mut dma_device,
    pub chan_id: c_int,
}

#[repr(C)]
pub struct dma_slave_config {
    pub src_addr_width: c_int,
    pub dst_addr_width: c_int,
    pub src_maxburst: c_int,
    pub dst_maxburst: c_int,
}

#[repr(C)]
pub struct dma_async_tx_descriptor {
    _private: [u8; 0],
}

pub type dma_cap_mask_t = c_ulong;
pub type dma_status = c_int;

#[repr(C)]
pub struct dw_dma_chip {
    pub regs: *mut c_void,
    pub dev: *mut device,
    pub irq: c_int,
}

#[repr(C)]
pub struct resource {
    pub start: u32,
    pub end: u32,
    pub child: *mut resource,
    pub sibling: *mut resource,
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_spec {
    pub pll_shutdown: unsafe extern "C" fn(*mut catpt_dev, bool_),
    pub dram_mask: c_ulong,
    pub iram_mask: c_ulong,
    pub d3srampgd_bit: u32,
    pub d3pgd_bit: u32,
    pub core_id: u8,
}

#[repr(C)]
pub struct catpt_fw_config {
    pub fw_info: *mut c_char,
}

#[repr(C)]
pub struct catpt_ipc {
    pub config: catpt_fw_config,
}

#[repr(C)]
pub struct catpt_dev {
    pub dev: *mut device,
    pub irq: c_int,
    pub dmac: *mut dw_dma_chip,
    pub dram: resource,
    pub iram: resource,
    pub spec: *mut catpt_spec,
    pub lpe_ba: *mut u8,
    pub clk_mutex: c_void,
    pub stream_list: list_head,
    pub ipc: catpt_ipc,
}

#[repr(C)]
pub struct catpt_stream_runtime {
    pub prepared: bool_,
    pub node: list_head,
}

#[repr(C)]
struct catpt_dump_section_hdr {
    magic: u16,
    core_id: u8,
    section_id: u8,
    size: u32,
}

const CATPT_DMA_DEVID: c_int = 1;
const CATPT_DMA_DSP_ADDR_MASK: dma_addr_t = GENMASK(31, 20) as dma_addr_t;

const CATPT_DUMP_MAGIC: u16 = 0xcd42;
const CATPT_DUMP_SECTION_ID_FILE: u8 = 0x00;
const CATPT_DUMP_SECTION_ID_IRAM: u8 = 0x01;
const CATPT_DUMP_SECTION_ID_DRAM: u8 = 0x02;
const CATPT_DUMP_SECTION_ID_REGS: u8 = 0x03;
const CATPT_DUMP_HASH_SIZE: usize = 20;

extern "C" {
    static DMA_MEMCPY: c_int;
    static DMA_SLAVE_BUSWIDTH_4_BYTES: c_int;
    static DMA_CTRL_ACK: c_int;
    static DMA_COMPLETE: dma_status;
    static GFP_KERNEL: c_int;
    static ENODEV: c_int;
    static EIO: c_int;
    static EPROTO: c_int;
    static ENOMEM: c_int;
    static HMDC: c_int;
    static VDRTCTL0: c_int;
    static VDRTCTL2: c_int;
    static CS1: c_int;
    static CS2: c_int;
    static ISC: c_int;
    static ISD: c_int;
    static IMC: c_int;
    static IMD: c_int;
    static IPCC: c_int;
    static IPCD: c_int;
    static CLKCTL: c_int;
    static LTRC: c_int;
    static SSCR0: c_int;
    static SSCR1: c_int;
    static SSSR: c_int;
    static SSITR: c_int;
    static SSDR: c_int;
    static SSTO: c_int;
    static SSPSP: c_int;
    static SSTSA: c_int;
    static SSRSA: c_int;
    static SSTSS: c_int;
    static SSCR2: c_int;
    static SSPSP2: c_int;
    static PMCS: c_int;
    static CATPT_HMDC_DEFAULT: u32;
    static CATPT_MEMBLOCK_SIZE: u32;
    static CATPT_VDRTCTL2_DCLCGE: u32;
    static CATPT_CS_STALL: u32;
    static CATPT_CS_RST: u32;
    static LPT_VDRTCTL0_APLLSE: u32;
    static WPT_VDRTCTL2_APLLSE: u32;
    static CATPT_CS_LPCS: u32;
    static CATPT_ISD_DCPWM: u32;
    static CATPT_CLKCTL_CFCIP: u32;
    static CATPT_CS_DCS_HIGH: u32;
    static CATPT_CS_DCS: u32;
    static CATPT_CS_DEFAULT: u32;
    static CATPT_ISC_DEFAULT: u32;
    static CATPT_ISD_DEFAULT: u32;
    static CATPT_IMC_DEFAULT: u32;
    static CATPT_IMD_DEFAULT: u32;
    static CATPT_IPCC_DEFAULT: u32;
    static CATPT_IPCD_DEFAULT: u32;
    static CATPT_CLKCTL_DEFAULT: u32;
    static CATPT_CS2_DEFAULT: u32;
    static CATPT_LTRC_DEFAULT: u32;
    static CATPT_SSP_COUNT: c_int;
    static CATPT_SSC0_DEFAULT: u32;
    static CATPT_SSC1_DEFAULT: u32;
    static CATPT_SSS_DEFAULT: u32;
    static CATPT_SSIT_DEFAULT: u32;
    static CATPT_SSD_DEFAULT: u32;
    static CATPT_SSTO_DEFAULT: u32;
    static CATPT_SSPSP_DEFAULT: u32;
    static CATPT_SSTSA_DEFAULT: u32;
    static CATPT_SSRSA_DEFAULT: u32;
    static CATPT_SSTSS_DEFAULT: u32;
    static CATPT_SSCR2_DEFAULT: u32;
    static CATPT_SSPSP2_DEFAULT: u32;
    static CATPT_VDRTCTL2_CGEALL: u32;
    static CATPT_VDRTCTL2_DTCGE: u32;
    static PCI_PM_CTRL_STATE_MASK: u32;
    static PCI_D3hot: u32;
    static PCI_D0: u32;
    static CATPT_CLKCTL_SMOS: u32;
    static CATPT_IMC_IPCDB: u32;
    static CATPT_IMC_IPCCD: u32;
    static CATPT_SHIM_REGS_SIZE: usize;
    static CATPT_DMA_COUNT: c_int;
    static CATPT_DMA_REGS_SIZE: usize;
    static CATPT_SSP_REGS_SIZE: usize;
    static FW_INFO_SIZE_MAX: usize;

    fn GENMASK(h: c_int, l: c_int) -> c_ulong;
    fn ERR_PTR(err: c_int) -> *mut dma_chan;
    fn dma_cap_zero(mask: *mut dma_cap_mask_t);
    fn dma_cap_set(cap: c_int, mask: *mut dma_cap_mask_t);
    fn dma_request_channel(
        mask: dma_cap_mask_t,
        filter: unsafe extern "C" fn(*mut dma_chan, *mut c_void) -> bool_,
        param: *mut c_void,
    ) -> *mut dma_chan;
    fn dmaengine_slave_config(chan: *mut dma_chan, config: *mut dma_slave_config) -> c_int;
    fn dma_release_channel(chan: *mut dma_chan);
    fn dmaengine_prep_dma_memcpy(
        chan: *mut dma_chan,
        dst_addr: dma_addr_t,
        src_addr: dma_addr_t,
        size: size_t,
        flags: c_int,
    ) -> *mut dma_async_tx_descriptor;
    fn dmaengine_submit(desc: *mut dma_async_tx_descriptor) -> c_int;
    fn dma_submit_error(cookie: c_int) -> c_int;
    fn dma_wait_for_async_tx(desc: *mut dma_async_tx_descriptor) -> dma_status;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_int) -> *mut c_void;
    fn dw_dma_probe(chip: *mut dw_dma_chip) -> c_int;
    fn dw_dma_remove(chip: *mut dw_dma_chip);
    fn catpt_dma_addr(cdev: *mut catpt_dev, id: c_int) -> *mut c_void;
    fn catpt_iram_addr(cdev: *mut catpt_dev) -> *mut c_void;
    fn catpt_dram_addr(cdev: *mut catpt_dev) -> *mut c_void;
    fn catpt_shim_addr(cdev: *mut catpt_dev) -> *mut c_void;
    fn catpt_ssp_addr(cdev: *mut catpt_dev, id: c_int) -> *mut c_void;
    fn catpt_updatel_shim(cdev: *mut catpt_dev, reg: c_int, mask: u32, val: u32);
    fn catpt_updatel_pci(cdev: *mut catpt_dev, reg: c_int, mask: u32, val: u32);
    fn catpt_readl_pci(cdev: *mut catpt_dev, reg: c_int) -> u32;
    fn catpt_readl_shim(cdev: *mut catpt_dev, reg: c_int) -> u32;
    fn catpt_writel_shim(cdev: *mut catpt_dev, reg: c_int, val: u32);
    fn catpt_writel_ssp(cdev: *mut catpt_dev, id: c_int, reg: c_int, val: u32);
    fn catpt_readl_poll_shim(
        cdev: *mut catpt_dev,
        reg_id: c_int,
        reg: u32,
        condition: bool_,
        delay_us: c_int,
        timeout_us: c_int,
    ) -> c_int;
    fn __ffs(word: c_ulong) -> c_ulong;
    fn fls_long(word: c_ulong) -> c_int;
    fn test_bit(nr: c_ulong, addr: *const c_ulong) -> bool_;
    fn memcpy_fromio(to: *mut c_void, from: *const c_void, count: size_t);
    fn udelay(usecs: c_ulong);
    fn resource_size(res: *const resource) -> size_t;
    fn vzalloc(size: size_t) -> *mut u8;
    fn strnchr(s: *const c_char, count: size_t, c: c_int) -> *mut c_char;
    fn dev_coredumpv(dev: *mut device, data: *mut c_void, datalen: size_t, gfp: c_int);
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn catpt_first_stream_runtime(head: *mut list_head) -> *mut catpt_stream_runtime;
    fn catpt_next_stream_runtime(
        head: *mut list_head,
        stream: *mut catpt_stream_runtime,
    ) -> *mut catpt_stream_runtime;
    fn CATPT_HMDC_HDDA(devid: c_int, chan_id: c_int) -> u32;
    fn CATPT_CS_SBCS(id: c_int) -> u32;
}

unsafe extern "C" fn catpt_dma_filter(chan: *mut dma_chan, param: *mut c_void) -> bool_ {
    param == (*(*chan).device).dev as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn catpt_dma_request_config_chan(cdev: *mut catpt_dev) -> *mut dma_chan {
    let mut config: dma_slave_config = core::mem::zeroed();
    let mut mask: dma_cap_mask_t = 0;
    let ret: c_int;

    dma_cap_zero(&mut mask);
    dma_cap_set(DMA_MEMCPY, &mut mask);

    let chan = dma_request_channel(mask, catpt_dma_filter, (*cdev).dev as *mut c_void);
    if chan.is_null() {
        dev_err((*cdev).dev, c"request channel failed\n".as_ptr());
        return ERR_PTR(-ENODEV);
    }

    config.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    config.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    config.src_maxburst = 16;
    config.dst_maxburst = 16;

    ret = dmaengine_slave_config(chan, &mut config);
    if ret != 0 {
        dev_err((*cdev).dev, c"slave config failed: %d\n".as_ptr(), ret);
        dma_release_channel(chan);
        return ERR_PTR(ret);
    }

    chan
}

unsafe extern "C" fn catpt_dma_memcpy(
    cdev: *mut catpt_dev,
    chan: *mut dma_chan,
    dst_addr: dma_addr_t,
    src_addr: dma_addr_t,
    size: size_t,
) -> c_int {
    let desc: *mut dma_async_tx_descriptor;
    let status: dma_status;
    let mut ret: c_int;

    desc = dmaengine_prep_dma_memcpy(chan, dst_addr, src_addr, size, DMA_CTRL_ACK);
    if desc.is_null() {
        dev_err((*cdev).dev, c"prep dma memcpy failed\n".as_ptr());
        return -EIO;
    }

    /* enable demand mode for dma channel */
    catpt_updatel_shim(
        cdev,
        HMDC,
        CATPT_HMDC_HDDA(CATPT_DMA_DEVID, (*chan).chan_id),
        CATPT_HMDC_HDDA(CATPT_DMA_DEVID, (*chan).chan_id),
    );

    ret = dma_submit_error(dmaengine_submit(desc));
    if ret != 0 {
        dev_err((*cdev).dev, c"submit tx failed: %d\n".as_ptr(), ret);
    } else {
        status = dma_wait_for_async_tx(desc);
        ret = if status == DMA_COMPLETE { 0 } else { -EPROTO };
    }

    /* regardless of status, disable access to HOST memory in demand mode */
    catpt_updatel_shim(
        cdev,
        HMDC,
        CATPT_HMDC_HDDA(CATPT_DMA_DEVID, (*chan).chan_id),
        0,
    );

    ret
}

#[no_mangle]
pub unsafe extern "C" fn catpt_dma_memcpy_todsp(
    cdev: *mut catpt_dev,
    chan: *mut dma_chan,
    dst_addr: dma_addr_t,
    src_addr: dma_addr_t,
    size: size_t,
) -> c_int {
    catpt_dma_memcpy(
        cdev,
        chan,
        dst_addr | CATPT_DMA_DSP_ADDR_MASK,
        src_addr,
        size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn catpt_dma_memcpy_fromdsp(
    cdev: *mut catpt_dev,
    chan: *mut dma_chan,
    dst_addr: dma_addr_t,
    src_addr: dma_addr_t,
    size: size_t,
) -> c_int {
    catpt_dma_memcpy(
        cdev,
        chan,
        dst_addr,
        src_addr | CATPT_DMA_DSP_ADDR_MASK,
        size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn catpt_dmac_probe(cdev: *mut catpt_dev) -> c_int {
    let dmac: *mut dw_dma_chip;
    let ret: c_int;

    dmac = devm_kzalloc((*cdev).dev, size_of::<dw_dma_chip>(), GFP_KERNEL) as *mut dw_dma_chip;
    if dmac.is_null() {
        return -ENOMEM;
    }

    (*dmac).regs = catpt_dma_addr(cdev, CATPT_DMA_DEVID);
    (*dmac).dev = (*cdev).dev;
    (*dmac).irq = (*cdev).irq;

    /*
     * Caller is responsible for putting device in D0 to allow
     * for I/O and memory access before probing DW.
     */
    ret = dw_dma_probe(dmac);
    if ret != 0 {
        return ret;
    }

    (*cdev).dmac = dmac;
    0
}

#[no_mangle]
pub unsafe extern "C" fn catpt_dmac_remove(cdev: *mut catpt_dev) {
    /*
     * As do_dma_remove() juggles with pm_runtime_get_xxx() and
     * pm_runtime_put_xxx() while both ADSP and DW 'devices' are part of
     * the same module, caller makes sure pm_runtime_disable() is invoked
     * before removing DW to prevent postmortem resume and suspend.
     */
    dw_dma_remove((*cdev).dmac);
}

unsafe extern "C" fn catpt_dsp_set_srampge(
    cdev: *mut catpt_dev,
    sram: *mut resource,
    mask: c_ulong,
    new: c_ulong,
) {
    let old: c_ulong;
    let mut off: u32 = (*sram).start;
    let mut b: c_ulong = __ffs(mask);

    old = catpt_readl_pci(cdev, VDRTCTL0) as c_ulong & mask;
    dev_dbg(
        (*cdev).dev,
        c"SRAMPGE [0x%08lx] 0x%08lx -> 0x%08lx".as_ptr(),
        mask,
        old,
        new,
    );

    if old == new {
        return;
    }

    catpt_updatel_pci(cdev, VDRTCTL0, mask as u32, new as u32);
    /* wait for SRAM power gating to propagate */
    udelay(60);

    /*
     * Dummy read as the very first access after block enable
     * to prevent byte loss in future operations.
     */
    while b < fls_long(mask) as c_ulong {
        if (new & (1usize.wrapping_shl(b as u32) as c_ulong)) == 0 {
            let mut buf: [u8; 4] = [0; 4];

            /* newly enabled: new bit=0 while old bit=1 */
            if test_bit(b, &old) {
                dev_dbg(
                    (*cdev).dev,
                    c"sanitize block %ld: off 0x%08x\n".as_ptr(),
                    b.wrapping_sub(__ffs(mask)),
                    off,
                );
                memcpy_fromio(
                    buf.as_mut_ptr() as *mut c_void,
                    (*cdev).lpe_ba.add(off as usize) as *const c_void,
                    size_of::<[u8; 4]>(),
                );
            }
            off = off.wrapping_add(CATPT_MEMBLOCK_SIZE);
        }
        b = b.wrapping_add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn catpt_dsp_update_srampge(
    cdev: *mut catpt_dev,
    sram: *mut resource,
    mask: c_ulong,
) {
    let mut res: *mut resource;
    let mut new: c_ulong = 0;

    /* flag all busy blocks */
    res = (*sram).child;
    while !res.is_null() {
        let h: u32;
        let l: u32;

        h = ((*res).end.wrapping_sub((*sram).start)) / CATPT_MEMBLOCK_SIZE;
        l = ((*res).start.wrapping_sub((*sram).start)) / CATPT_MEMBLOCK_SIZE;
        new |= GENMASK(h as c_int, l as c_int);
        res = (*res).sibling;
    }

    /* offset value given mask's start and invert it as ON=b0 */
    new = !(new.wrapping_shl(__ffs(mask) as u32)) & mask;

    /* disable core clock gating */
    catpt_updatel_pci(cdev, VDRTCTL2, CATPT_VDRTCTL2_DCLCGE, 0);

    catpt_dsp_set_srampge(cdev, sram, mask, new);

    /* enable core clock gating */
    catpt_updatel_pci(
        cdev,
        VDRTCTL2,
        CATPT_VDRTCTL2_DCLCGE,
        CATPT_VDRTCTL2_DCLCGE,
    );
}

#[no_mangle]
pub unsafe extern "C" fn catpt_dsp_stall(cdev: *mut catpt_dev, stall: bool_) -> c_int {
    let reg: u32 = 0;
    let val: u32;

    val = if stall { CATPT_CS_STALL } else { 0 };
    catpt_updatel_shim(cdev, CS1, CATPT_CS_STALL, val);

    catpt_readl_poll_shim(
        cdev,
        CS1,
        reg,
        (reg & CATPT_CS_STALL) == val,
        500,
        10000,
    )
}

unsafe extern "C" fn catpt_dsp_reset(cdev: *mut catpt_dev, reset: bool_) -> c_int {
    let reg: u32 = 0;
    let val: u32;

    val = if reset { CATPT_CS_RST } else { 0 };
    catpt_updatel_shim(cdev, CS1, CATPT_CS_RST, val);

    catpt_readl_poll_shim(cdev, CS1, reg, (reg & CATPT_CS_RST) == val, 500, 10000)
}

#[no_mangle]
pub unsafe extern "C" fn lpt_dsp_pll_shutdown(cdev: *mut catpt_dev, enable: bool_) {
    let val: u32;

    val = if enable { LPT_VDRTCTL0_APLLSE } else { 0 };
    catpt_updatel_pci(cdev, VDRTCTL0, LPT_VDRTCTL0_APLLSE, val);
}

#[no_mangle]
pub unsafe extern "C" fn wpt_dsp_pll_shutdown(cdev: *mut catpt_dev, enable: bool_) {
    let val: u32;

    val = if enable { WPT_VDRTCTL2_APLLSE } else { 0 };
    catpt_updatel_pci(cdev, VDRTCTL2, WPT_VDRTCTL2_APLLSE, val);
}

unsafe extern "C" fn catpt_dsp_select_lpclock(
    cdev: *mut catpt_dev,
    lp: bool_,
    waiti: bool_,
) -> c_int {
    let mask: u32;
    let mut reg: u32;
    let mut val: u32;
    let mut ret: c_int;

    mutex_lock(&mut (*cdev).clk_mutex);

    val = if lp { CATPT_CS_LPCS } else { 0 };
    reg = catpt_readl_shim(cdev, CS1) & CATPT_CS_LPCS;
    dev_dbg(
        (*cdev).dev,
        c"LPCS [0x%08lx] 0x%08x -> 0x%08x".as_ptr(),
        CATPT_CS_LPCS as c_ulong,
        reg,
        val,
    );

    if reg == val {
        mutex_unlock(&mut (*cdev).clk_mutex);
        return 0;
    }

    if waiti {
        /* wait for DSP to signal WAIT state */
        ret = catpt_readl_poll_shim(
            cdev,
            ISD,
            reg,
            (reg & CATPT_ISD_DCPWM) != 0,
            500,
            10000,
        );
        if ret != 0 {
            dev_warn((*cdev).dev, c"await WAITI timeout\n".as_ptr());
            /* no signal - only high clock selection allowed */
            if lp {
                mutex_unlock(&mut (*cdev).clk_mutex);
                return 0;
            }
        }
    }

    ret = catpt_readl_poll_shim(
        cdev,
        CLKCTL,
        reg,
        !(reg & CATPT_CLKCTL_CFCIP != 0),
        500,
        10000,
    );
    if ret != 0 {
        dev_warn((*cdev).dev, c"clock change still in progress\n".as_ptr());
    }

    /* default to DSP core & audio fabric high clock */
    val |= CATPT_CS_DCS_HIGH;
    mask = CATPT_CS_LPCS | CATPT_CS_DCS;
    catpt_updatel_shim(cdev, CS1, mask, val);

    ret = catpt_readl_poll_shim(
        cdev,
        CLKCTL,
        reg,
        !(reg & CATPT_CLKCTL_CFCIP != 0),
        500,
        10000,
    );
    if ret != 0 {
        dev_warn((*cdev).dev, c"clock change still in progress\n".as_ptr());
    }

    /* update PLL accordingly */
    ((*(*cdev).spec).pll_shutdown)(cdev, lp);

    mutex_unlock(&mut (*cdev).clk_mutex);
    0
}

#[no_mangle]
pub unsafe extern "C" fn catpt_dsp_update_lpclock(cdev: *mut catpt_dev) -> c_int {
    let mut stream: *mut catpt_stream_runtime;

    stream = catpt_first_stream_runtime(&mut (*cdev).stream_list);
    while !stream.is_null() {
        if (*stream).prepared {
            return catpt_dsp_select_lpclock(cdev, false, true);
        }
        stream = catpt_next_stream_runtime(&mut (*cdev).stream_list, stream);
    }

    catpt_dsp_select_lpclock(cdev, true, true)
}

/* bring registers to their defaults as HW won't reset itself */
unsafe extern "C" fn catpt_dsp_set_regs_defaults(cdev: *mut catpt_dev) {
    let mut i: c_int;

    catpt_writel_shim(cdev, CS1, CATPT_CS_DEFAULT);
    catpt_writel_shim(cdev, ISC, CATPT_ISC_DEFAULT);
    catpt_writel_shim(cdev, ISD, CATPT_ISD_DEFAULT);
    catpt_writel_shim(cdev, IMC, CATPT_IMC_DEFAULT);
    catpt_writel_shim(cdev, IMD, CATPT_IMD_DEFAULT);
    catpt_writel_shim(cdev, IPCC, CATPT_IPCC_DEFAULT);
    catpt_writel_shim(cdev, IPCD, CATPT_IPCD_DEFAULT);
    catpt_writel_shim(cdev, CLKCTL, CATPT_CLKCTL_DEFAULT);
    catpt_writel_shim(cdev, CS2, CATPT_CS2_DEFAULT);
    catpt_writel_shim(cdev, LTRC, CATPT_LTRC_DEFAULT);
    catpt_writel_shim(cdev, HMDC, CATPT_HMDC_DEFAULT);

    i = 0;
    while i < CATPT_SSP_COUNT {
        catpt_writel_ssp(cdev, i, SSCR0, CATPT_SSC0_DEFAULT);
        catpt_writel_ssp(cdev, i, SSCR1, CATPT_SSC1_DEFAULT);
        catpt_writel_ssp(cdev, i, SSSR, CATPT_SSS_DEFAULT);
        catpt_writel_ssp(cdev, i, SSITR, CATPT_SSIT_DEFAULT);
        catpt_writel_ssp(cdev, i, SSDR, CATPT_SSD_DEFAULT);
        catpt_writel_ssp(cdev, i, SSTO, CATPT_SSTO_DEFAULT);
        catpt_writel_ssp(cdev, i, SSPSP, CATPT_SSPSP_DEFAULT);
        catpt_writel_ssp(cdev, i, SSTSA, CATPT_SSTSA_DEFAULT);
        catpt_writel_ssp(cdev, i, SSRSA, CATPT_SSRSA_DEFAULT);
        catpt_writel_ssp(cdev, i, SSTSS, CATPT_SSTSS_DEFAULT);
        catpt_writel_ssp(cdev, i, SSCR2, CATPT_SSCR2_DEFAULT);
        catpt_writel_ssp(cdev, i, SSPSP2, CATPT_SSPSP2_DEFAULT);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn catpt_dsp_power_down(cdev: *mut catpt_dev) -> c_int {
    let mut mask: u32;
    let val: u32;

    /* disable core clock gating */
    catpt_updatel_pci(cdev, VDRTCTL2, CATPT_VDRTCTL2_DCLCGE, 0);

    catpt_dsp_reset(cdev, true);
    /* set 24Mhz clock for both SSPs */
    catpt_updatel_shim(
        cdev,
        CS1,
        CATPT_CS_SBCS(0) | CATPT_CS_SBCS(1),
        CATPT_CS_SBCS(0) | CATPT_CS_SBCS(1),
    );
    catpt_dsp_select_lpclock(cdev, true, false);
    /* disable MCLK */
    catpt_updatel_shim(cdev, CLKCTL, CATPT_CLKCTL_SMOS, 0);

    catpt_dsp_set_regs_defaults(cdev);

    /* switch clock gating */
    mask = CATPT_VDRTCTL2_CGEALL & !CATPT_VDRTCTL2_DCLCGE;
    val = mask & !CATPT_VDRTCTL2_DTCGE;
    catpt_updatel_pci(cdev, VDRTCTL2, mask, val);
    /* enable DTCGE separatelly */
    catpt_updatel_pci(cdev, VDRTCTL2, CATPT_VDRTCTL2_DTCGE, CATPT_VDRTCTL2_DTCGE);

    /* SRAM power gating all */
    catpt_dsp_set_srampge(
        cdev,
        &mut (*cdev).dram,
        (*(*cdev).spec).dram_mask,
        (*(*cdev).spec).dram_mask,
    );
    catpt_dsp_set_srampge(
        cdev,
        &mut (*cdev).iram,
        (*(*cdev).spec).iram_mask,
        (*(*cdev).spec).iram_mask,
    );
    mask = (*(*cdev).spec).d3srampgd_bit | (*(*cdev).spec).d3pgd_bit;
    catpt_updatel_pci(cdev, VDRTCTL0, mask, (*(*cdev).spec).d3pgd_bit);

    catpt_updatel_pci(cdev, PMCS, PCI_PM_CTRL_STATE_MASK, PCI_D3hot as u32);
    /* give hw time to drop off */
    udelay(50);

    /* enable core clock gating */
    catpt_updatel_pci(
        cdev,
        VDRTCTL2,
        CATPT_VDRTCTL2_DCLCGE,
        CATPT_VDRTCTL2_DCLCGE,
    );
    udelay(50);

    0
}

#[no_mangle]
pub unsafe extern "C" fn catpt_dsp_power_up(cdev: *mut catpt_dev) -> c_int {
    let mut mask: u32;
    let val: u32;

    /* disable core clock gating */
    catpt_updatel_pci(cdev, VDRTCTL2, CATPT_VDRTCTL2_DCLCGE, 0);

    /* switch clock gating */
    mask = CATPT_VDRTCTL2_CGEALL & !CATPT_VDRTCTL2_DCLCGE;
    val = mask & !CATPT_VDRTCTL2_DTCGE;
    catpt_updatel_pci(cdev, VDRTCTL2, mask, val);

    catpt_updatel_pci(cdev, PMCS, PCI_PM_CTRL_STATE_MASK, PCI_D0 as u32);

    /* SRAM power gating none */
    mask = (*(*cdev).spec).d3srampgd_bit | (*(*cdev).spec).d3pgd_bit;
    catpt_updatel_pci(cdev, VDRTCTL0, mask, mask);
    catpt_dsp_set_srampge(cdev, &mut (*cdev).dram, (*(*cdev).spec).dram_mask, 0);
    catpt_dsp_set_srampge(cdev, &mut (*cdev).iram, (*(*cdev).spec).iram_mask, 0);

    catpt_dsp_set_regs_defaults(cdev);

    /* restore MCLK */
    catpt_updatel_shim(cdev, CLKCTL, CATPT_CLKCTL_SMOS, CATPT_CLKCTL_SMOS);
    catpt_dsp_select_lpclock(cdev, false, false);
    /* set 24Mhz clock for both SSPs */
    catpt_updatel_shim(
        cdev,
        CS1,
        CATPT_CS_SBCS(0) | CATPT_CS_SBCS(1),
        CATPT_CS_SBCS(0) | CATPT_CS_SBCS(1),
    );
    catpt_dsp_reset(cdev, false);

    /* enable core clock gating */
    catpt_updatel_pci(
        cdev,
        VDRTCTL2,
        CATPT_VDRTCTL2_DCLCGE,
        CATPT_VDRTCTL2_DCLCGE,
    );

    /* generate int deassert msg to fix inversed int logic */
    catpt_updatel_shim(cdev, IMC, CATPT_IMC_IPCDB | CATPT_IMC_IPCCD, 0);

    0
}

#[no_mangle]
pub unsafe extern "C" fn catpt_coredump(cdev: *mut catpt_dev) -> c_int {
    let mut hdr: *mut catpt_dump_section_hdr;
    let mut dump_size: size_t;
    let mut regs_size: size_t;
    let dump: *mut u8;
    let mut pos: *mut u8;
    let eof: *const c_char;
    let mut info: *mut c_char;
    let mut i: c_int;

    regs_size = CATPT_SHIM_REGS_SIZE;
    regs_size = regs_size.wrapping_add((CATPT_DMA_COUNT as usize).wrapping_mul(CATPT_DMA_REGS_SIZE));
    regs_size = regs_size.wrapping_add((CATPT_SSP_COUNT as usize).wrapping_mul(CATPT_SSP_REGS_SIZE));
    dump_size = resource_size(&(*cdev).dram);
    dump_size = dump_size.wrapping_add(resource_size(&(*cdev).iram));
    dump_size = dump_size.wrapping_add(regs_size);
    /* account for header of each section and hash chunk */
    dump_size = dump_size.wrapping_add(4usize.wrapping_mul(size_of::<catpt_dump_section_hdr>()));
    dump_size = dump_size.wrapping_add(CATPT_DUMP_HASH_SIZE);

    dump = vzalloc(dump_size);
    if dump.is_null() {
        return -ENOMEM;
    }

    pos = dump;

    hdr = pos as *mut catpt_dump_section_hdr;
    (*hdr).magic = CATPT_DUMP_MAGIC;
    (*hdr).core_id = (*(*cdev).spec).core_id;
    (*hdr).section_id = CATPT_DUMP_SECTION_ID_FILE;
    (*hdr).size = dump_size.wrapping_sub(size_of::<catpt_dump_section_hdr>()) as u32;
    pos = pos.add(size_of::<catpt_dump_section_hdr>());

    info = (*cdev).ipc.config.fw_info;
    eof = info.add(FW_INFO_SIZE_MAX) as *const c_char;
    /* navigate to fifth info segment (fw hash) */
    i = 0;
    while i < 4 && (info as *const c_char) < eof {
        /* info segments are separated by space each */
        info = strnchr(
            info as *const c_char,
            eof.offset_from(info as *const c_char) as size_t,
            b' ' as c_int,
        );
        if info.is_null() {
            break;
        }
        i += 1;
        info = info.add(1);
    }

    if i == 4 && !info.is_null() {
        let len = core::cmp::min(
            eof.offset_from(info as *const c_char) as u32,
            CATPT_DUMP_HASH_SIZE as u32,
        ) as usize;
        ptr::copy_nonoverlapping(info as *const u8, pos, len);
    }
    pos = pos.add(CATPT_DUMP_HASH_SIZE);

    hdr = pos as *mut catpt_dump_section_hdr;
    (*hdr).magic = CATPT_DUMP_MAGIC;
    (*hdr).core_id = (*(*cdev).spec).core_id;
    (*hdr).section_id = CATPT_DUMP_SECTION_ID_IRAM;
    (*hdr).size = resource_size(&(*cdev).iram) as u32;
    pos = pos.add(size_of::<catpt_dump_section_hdr>());

    memcpy_fromio(pos as *mut c_void, catpt_iram_addr(cdev), (*hdr).size as size_t);
    pos = pos.add((*hdr).size as usize);

    hdr = pos as *mut catpt_dump_section_hdr;
    (*hdr).magic = CATPT_DUMP_MAGIC;
    (*hdr).core_id = (*(*cdev).spec).core_id;
    (*hdr).section_id = CATPT_DUMP_SECTION_ID_DRAM;
    (*hdr).size = resource_size(&(*cdev).dram) as u32;
    pos = pos.add(size_of::<catpt_dump_section_hdr>());

    memcpy_fromio(pos as *mut c_void, catpt_dram_addr(cdev), (*hdr).size as size_t);
    pos = pos.add((*hdr).size as usize);

    hdr = pos as *mut catpt_dump_section_hdr;
    (*hdr).magic = CATPT_DUMP_MAGIC;
    (*hdr).core_id = (*(*cdev).spec).core_id;
    (*hdr).section_id = CATPT_DUMP_SECTION_ID_REGS;
    (*hdr).size = regs_size as u32;
    pos = pos.add(size_of::<catpt_dump_section_hdr>());

    memcpy_fromio(pos as *mut c_void, catpt_shim_addr(cdev), CATPT_SHIM_REGS_SIZE);
    pos = pos.add(CATPT_SHIM_REGS_SIZE);

    i = 0;
    while i < CATPT_SSP_COUNT {
        memcpy_fromio(
            pos as *mut c_void,
            catpt_ssp_addr(cdev, i),
            CATPT_SSP_REGS_SIZE,
        );
        pos = pos.add(CATPT_SSP_REGS_SIZE);
        i += 1;
    }
    i = 0;
    while i < CATPT_DMA_COUNT {
        memcpy_fromio(
            pos as *mut c_void,
            catpt_dma_addr(cdev, i),
            CATPT_DMA_REGS_SIZE,
        );
        pos = pos.add(CATPT_DMA_REGS_SIZE);
        i += 1;
    }

    dev_coredumpv((*cdev).dev, dump as *mut c_void, dump_size, GFP_KERNEL);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
