// SPDX-License-Identifier: GPL-2.0-only
/* FPGA Manager Driver for Xilinx Zynq, translated from zynq-fpga.c. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const SLCR_FPGA_RST_CTRL_OFFSET: u32 = 0x240;
const SLCR_LVL_SHFTR_EN_OFFSET: u32 = 0x900;
const CTRL_OFFSET: u32 = 0x00;
const LOCK_OFFSET: u32 = 0x04;
const INT_STS_OFFSET: u32 = 0x0c;
const INT_MASK_OFFSET: u32 = 0x10;
const STATUS_OFFSET: u32 = 0x14;
const DMA_SRC_ADDR_OFFSET: u32 = 0x18;
const DMA_DST_ADDR_OFFSET: u32 = 0x1c;
const DMA_SRC_LEN_OFFSET: u32 = 0x20;
const DMA_DEST_LEN_OFFSET: u32 = 0x24;
const UNLOCK_OFFSET: u32 = 0x34;
const MCTRL_OFFSET: u32 = 0x80;

const CTRL_PCFG_PROG_B_MASK: u32 = 1 << 30;
const CTRL_PCAP_PR_MASK: u32 = 1 << 27;
const CTRL_PCAP_MODE_MASK: u32 = 1 << 26;
const CTRL_PCAP_RATE_EN_MASK: u32 = 1 << 25;
const CTRL_SEC_EN_MASK: u32 = 1 << 7;
const MCTRL_PCAP_LPBK_MASK: u32 = 1 << 4;
const STATUS_DMA_Q_F: u32 = 1 << 31;
const STATUS_DMA_Q_E: u32 = 1 << 30;
const STATUS_PCFG_INIT_MASK: u32 = 1 << 4;
const IXR_DMA_DONE_MASK: u32 = 1 << 13;
const IXR_D_P_DONE_MASK: u32 = 1 << 12;
const IXR_PCFG_DONE_MASK: u32 = 1 << 2;
const IXR_ERROR_FLAGS_MASK: u32 = 0x00f0c860;
const IXR_ALL_MASK: u32 = 0xf8f7f87f;
const DMA_INVALID_ADDRESS: u32 = 0xffff_ffff;
const UNLOCK_MASK: u32 = 0x757bdf0d;
const INIT_POLL_TIMEOUT: u32 = 2_500_000;
const INIT_POLL_DELAY: u32 = 20;
const DMA_SRC_LAST_TRANSFER: u32 = 1;
const DMA_TIMEOUT_MS: u32 = 5000;
const LVL_SHFTR_DISABLE_ALL_MASK: u32 = 0;
const LVL_SHFTR_ENABLE_PS_TO_PL: u32 = 0xa;
const LVL_SHFTR_ENABLE_PL_TO_PS: u32 = 0xf;
const FPGA_RST_ALL_MASK: u32 = 0xf;
const FPGA_RST_NONE_MASK: u32 = 0;

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { pub next: *mut scatterlist, pub offset: u32, pub length: u32 }
#[repr(C)] pub struct sg_table { pub sgl: *mut scatterlist, pub nents: u32 }
#[repr(C)] pub struct device { pub parent: *mut device }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct fpga_image_info { pub flags: u32 }
#[repr(C)] pub struct fpga_manager { pub priv_: *mut c_void, pub dev: device }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct fpga_manager_ops {
    pub initial_header_size: usize,
    pub state: Option<unsafe extern "C" fn(*mut fpga_manager) -> fpga_mgr_states>,
    pub write_init: Option<unsafe extern "C" fn(*mut fpga_manager, *mut fpga_image_info, *const c_char, usize) -> c_int>,
    pub write_sg: Option<unsafe extern "C" fn(*mut fpga_manager, *mut sg_table) -> c_int>,
    pub write_complete: Option<unsafe extern "C" fn(*mut fpga_manager, *mut fpga_image_info) -> c_int>,
}
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char }
#[repr(C)] pub struct platform_driver_driver { pub name: *const c_char, pub of_match_table: *const of_device_id }
#[repr(C)] pub struct platform_driver { pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut platform_device)>, pub driver: platform_driver_driver }
#[repr(C)] pub enum fpga_mgr_states { FPGA_MGR_STATE_UNKNOWN = 0, FPGA_MGR_STATE_OPERATING = 1 }
#[repr(C)] pub struct zynq_fpga_priv {
    irq: c_int, clk: *mut clk, io_base: *mut u8, slcr: *mut regmap,
    dma_lock: spinlock_t, dma_elm: c_uint, dma_nelms: c_uint,
    cur_sg: *mut scatterlist, dma_done: completion,
}

extern "C" {
    fn readl(addr: *const u8) -> u32; fn writel(val: u32, addr: *mut u8);
    fn clk_enable(c: *mut clk) -> c_int; fn clk_disable(c: *mut clk);
    fn clk_prepare_enable(c: *mut clk) -> c_int; fn clk_disable_unprepare(c: *mut clk); fn clk_unprepare(c: *mut clk);
    fn regmap_write(m: *mut regmap, off: u32, val: u32) -> c_int;
    fn sg_dma_address(sg: *mut scatterlist) -> u32; fn sg_dma_len(sg: *mut scatterlist) -> u32;
    fn dma_map_sgtable(d: *mut device, t: *mut sg_table, dir: c_int, attrs: c_ulong) -> c_int;
    fn dma_unmap_sgtable(d: *mut device, t: *mut sg_table, dir: c_int, attrs: c_ulong);
    fn spin_lock(l: *mut spinlock_t); fn spin_unlock(l: *mut spinlock_t);
    fn spin_lock_irqsave(l: *mut spinlock_t, flags: *mut c_ulong); fn spin_unlock_irqrestore(l: *mut spinlock_t, flags: c_ulong);
    fn complete(c: *mut completion); fn reinit_completion(c: *mut completion); fn wait_for_completion_timeout(c: *mut completion, j: c_ulong) -> c_ulong;
    fn msecs_to_jiffies(ms: c_ulong) -> c_ulong;
    fn platform_get_drvdata(p: *mut platform_device) -> *mut fpga_manager; fn platform_set_drvdata(p: *mut platform_device, d: *mut fpga_manager);
    fn fpga_mgr_register(d: *mut device, n: *const c_char, o: *const fpga_manager_ops, p: *mut zynq_fpga_priv) -> *mut fpga_manager;
    fn fpga_mgr_unregister(m: *mut fpga_manager); fn devm_request_irq(d: *mut device, irq: c_int, f: unsafe extern "C" fn(c_int,*mut c_void)->c_int, flags: c_ulong, n: *const c_char, data: *mut zynq_fpga_priv) -> c_int;
    fn devm_platform_ioremap_resource(p: *mut platform_device, n: c_uint) -> *mut u8; fn platform_get_irq(p: *mut platform_device, n: c_uint) -> c_int;
    fn syscon_regmap_lookup_by_phandle(n: *mut c_void, p: *const c_char) -> *mut regmap;
    fn devm_clk_get(d: *mut device, n: *const c_char) -> *mut clk; fn spin_lock_init(l: *mut spinlock_t); fn init_completion(c: *mut completion);
}

#[inline] unsafe fn zynq_fpga_write(p: *mut zynq_fpga_priv, off: u32, val: u32) { writel(val, (*p).io_base.add(off as usize)); }
#[inline] unsafe fn zynq_fpga_read(p: *const zynq_fpga_priv, off: u32) -> u32 { readl((*p).io_base.add(off as usize)) }
#[inline] unsafe fn zynq_fpga_set_irq(p: *mut zynq_fpga_priv, enable: u32) { zynq_fpga_write(p, INT_MASK_OFFSET, !enable); }

unsafe fn zynq_step_dma(p: *mut zynq_fpga_priv) {
    let first = (*p).dma_elm == 0;
    while !(*p).cur_sg.is_null() {
        if zynq_fpga_read(p, STATUS_OFFSET) & STATUS_DMA_Q_F != 0 { break; }
        let sg = (*p).cur_sg; let mut addr = sg_dma_address(sg); let len = sg_dma_len(sg);
        if (*p).dma_elm + 1 == (*p).dma_nelms { addr |= DMA_SRC_LAST_TRANSFER; (*p).cur_sg = core::ptr::null_mut(); }
        else { (*p).cur_sg = (*sg).next; (*p).dma_elm += 1; }
        zynq_fpga_write(p, DMA_SRC_ADDR_OFFSET, addr); zynq_fpga_write(p, DMA_DST_ADDR_OFFSET, DMA_INVALID_ADDRESS);
        zynq_fpga_write(p, DMA_SRC_LEN_OFFSET, len / 4); zynq_fpga_write(p, DMA_DEST_LEN_OFFSET, 0);
    }
    if first && !(*p).cur_sg.is_null() { zynq_fpga_set_irq(p, IXR_DMA_DONE_MASK | IXR_ERROR_FLAGS_MASK); }
    else if (*p).cur_sg.is_null() { zynq_fpga_set_irq(p, IXR_D_P_DONE_MASK | IXR_ERROR_FLAGS_MASK); }
}

unsafe extern "C" fn zynq_fpga_isr(_irq: c_int, data: *mut c_void) -> c_int {
    let p = data as *mut zynq_fpga_priv; spin_lock(&mut (*p).dma_lock);
    let s = zynq_fpga_read(p, INT_STS_OFFSET);
    if s & IXR_ERROR_FLAGS_MASK == 0 && s & IXR_DMA_DONE_MASK != 0 && !(*p).cur_sg.is_null() {
        zynq_fpga_write(p, INT_STS_OFFSET, IXR_DMA_DONE_MASK); zynq_step_dma(p); spin_unlock(&mut (*p).dma_lock); return 1;
    }
    spin_unlock(&mut (*p).dma_lock); zynq_fpga_set_irq(p, 0); complete(&mut (*p).dma_done); 1
}

unsafe fn zynq_fpga_has_sync(mut buf: *const u8, mut count: usize) -> bool { while count >= 4 { if *buf == 0x66 && *buf.add(1) == 0x55 && *buf.add(2) == 0x99 && *buf.add(3) == 0xaa { return true; } buf = buf.add(4); count -= 4; } false }

const FPGA_MGR_ENCRYPTED_BITSTREAM: u32 = 1 << 0;
const FPGA_MGR_PARTIAL_RECONFIG: u32 = 1 << 1;
const DMA_TO_DEVICE: c_int = 1;

unsafe extern "C" fn zynq_fpga_ops_write_init(m: *mut fpga_manager, info: *mut fpga_image_info, buf: *const c_char, count: usize) -> c_int {
    let p = (*m).priv_ as *mut zynq_fpga_priv; let encrypted = (*info).flags & FPGA_MGR_ENCRYPTED_BITSTREAM != 0;
    let mut e = clk_enable((*p).clk); if e != 0 { return e; }
    if encrypted && zynq_fpga_read(p, CTRL_OFFSET) & CTRL_SEC_EN_MASK == 0 { e = -22; }
    if e == 0 && (*info).flags & FPGA_MGR_PARTIAL_RECONFIG == 0 {
        if !zynq_fpga_has_sync(buf as *const u8, count) { e = -22; }
        if e == 0 { regmap_write((*p).slcr, SLCR_FPGA_RST_CTRL_OFFSET, FPGA_RST_ALL_MASK); regmap_write((*p).slcr, SLCR_LVL_SHFTR_EN_OFFSET, LVL_SHFTR_DISABLE_ALL_MASK); regmap_write((*p).slcr, SLCR_LVL_SHFTR_EN_OFFSET, LVL_SHFTR_ENABLE_PS_TO_PL); }
        let mut ctrl = zynq_fpga_read(p, CTRL_OFFSET) | CTRL_PCFG_PROG_B_MASK;
        if e == 0 { zynq_fpga_write(p, CTRL_OFFSET, ctrl); e = 0; }
        if e == 0 { ctrl &= !CTRL_PCFG_PROG_B_MASK; zynq_fpga_write(p, CTRL_OFFSET, ctrl); ctrl |= CTRL_PCFG_PROG_B_MASK; zynq_fpga_write(p, CTRL_OFFSET, ctrl); }
    }
    if e == 0 { let ctrl = zynq_fpga_read(p, CTRL_OFFSET); let v = CTRL_PCAP_PR_MASK | CTRL_PCAP_MODE_MASK | ctrl | if encrypted { CTRL_PCAP_RATE_EN_MASK } else { 0 }; zynq_fpga_write(p, CTRL_OFFSET, v); let s = zynq_fpga_read(p, STATUS_OFFSET); if s & STATUS_DMA_Q_F != 0 || s & STATUS_DMA_Q_E != STATUS_DMA_Q_E { e = -16; } }
    if e == 0 { let ctrl = zynq_fpga_read(p, MCTRL_OFFSET); zynq_fpga_write(p, MCTRL_OFFSET, ctrl & !MCTRL_PCAP_LPBK_MASK); } clk_disable((*p).clk); e
}

unsafe extern "C" fn zynq_fpga_ops_write(m: *mut fpga_manager, sgt: *mut sg_table) -> c_int {
    let p = (*m).priv_ as *mut zynq_fpga_priv; let mut sg = (*sgt).sgl; let mut i = 0; while i < (*sgt).nents { if (*sg).offset % 8 != 0 || (*sg).length % 4 != 0 { return -22; } sg = (*sg).next; i += 1; }
    let mut e = dma_map_sgtable((*m).dev.parent, sgt, DMA_TO_DEVICE, 0); if e != 0 { return e; } (*p).dma_nelms = (*sgt).nents; e = clk_enable((*p).clk); if e != 0 { dma_unmap_sgtable((*m).dev.parent, sgt, DMA_TO_DEVICE, 0); return e; }
    zynq_fpga_write(p, INT_STS_OFFSET, IXR_ALL_MASK); reinit_completion(&mut (*p).dma_done); let mut flags = 0; spin_lock_irqsave(&mut (*p).dma_lock, &mut flags); (*p).dma_elm = 0; (*p).cur_sg = (*sgt).sgl; zynq_step_dma(p); spin_unlock_irqrestore(&mut (*p).dma_lock, flags); let left = wait_for_completion_timeout(&mut (*p).dma_done, msecs_to_jiffies(DMA_TIMEOUT_MS as c_ulong)); spin_lock_irqsave(&mut (*p).dma_lock, &mut flags); zynq_fpga_set_irq(p, 0); (*p).cur_sg = core::ptr::null_mut(); spin_unlock_irqrestore(&mut (*p).dma_lock, flags); let s = zynq_fpga_read(p, INT_STS_OFFSET); zynq_fpga_write(p, INT_STS_OFFSET, IXR_ALL_MASK); if s & IXR_ERROR_FLAGS_MASK != 0 { e = -5; } else if !(*p).cur_sg.is_null() || s & IXR_D_P_DONE_MASK != IXR_D_P_DONE_MASK || left == 0 { e = -5; } else { e = 0; } clk_disable((*p).clk); dma_unmap_sgtable((*m).dev.parent, sgt, DMA_TO_DEVICE, 0); e
}

unsafe extern "C" fn zynq_fpga_ops_write_complete(m: *mut fpga_manager, info: *mut fpga_image_info) -> c_int { let p = (*m).priv_ as *mut zynq_fpga_priv; let e = clk_enable((*p).clk); if e != 0 { return e; } let s = zynq_fpga_read(p, INT_STS_OFFSET); zynq_fpga_write(p, CTRL_OFFSET, zynq_fpga_read(p, CTRL_OFFSET) & !CTRL_PCAP_PR_MASK); clk_disable((*p).clk); if s & IXR_PCFG_DONE_MASK == 0 { return -110; } if (*info).flags & FPGA_MGR_PARTIAL_RECONFIG == 0 { regmap_write((*p).slcr, SLCR_LVL_SHFTR_EN_OFFSET, LVL_SHFTR_ENABLE_PL_TO_PS); regmap_write((*p).slcr, SLCR_FPGA_RST_CTRL_OFFSET, FPGA_RST_NONE_MASK); } 0 }
unsafe extern "C" fn zynq_fpga_ops_state(m: *mut fpga_manager) -> fpga_mgr_states { let p = (*m).priv_ as *mut zynq_fpga_priv; if clk_enable((*p).clk) != 0 { return fpga_mgr_states::FPGA_MGR_STATE_UNKNOWN; } let s = zynq_fpga_read(p, INT_STS_OFFSET); clk_disable((*p).clk); if s & IXR_PCFG_DONE_MASK != 0 { fpga_mgr_states::FPGA_MGR_STATE_OPERATING } else { fpga_mgr_states::FPGA_MGR_STATE_UNKNOWN } }
#[no_mangle] pub static zynq_fpga_ops: fpga_manager_ops = fpga_manager_ops { initial_header_size: 128, state: Some(zynq_fpga_ops_state), write_init: Some(zynq_fpga_ops_write_init), write_sg: Some(zynq_fpga_ops_write), write_complete: Some(zynq_fpga_ops_write_complete) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
