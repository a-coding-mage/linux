// SPDX-License-Identifier: GPL-2.0-only
// Rust translation of linux/arch/arm/common/sa1111.c.
// Kernel-provided types, constants, macros, and functions are intentionally
// referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn sa1110_mb_enable();
    fn sa1110_mb_disable();
}

const SA1111_IRQ_NR: u32 = 55;
const IRQ_GPAIN0: u32 = 0; const IRQ_GPBIN0: u32 = 4; const IRQ_GPCIN0: u32 = 10;
const SSPROR: u32 = 26; const AUDXMTDMADONEA: u32 = 32;
const IRQ_S0_READY_NINT: u32 = 49; const IRQ_S1_BVD1_STSCHG: u32 = 54;
const IRQ_USBPWR: u32 = 43; const IRQ_HCIM: u32 = 44; const IRQ_HCIBUFFACC: u32 = 45;
const IRQ_HCIRMTWKP: u32 = 46; const IRQ_NHCIMFCIR: u32 = 47;
const IRQ_USB_PORT_RESUME: u32 = 48; const IRQ_S0_CD_VALID: u32 = 51;
const IRQ_S0_BVD1_STSCHG: u32 = 53; const IRQ_S1_READY_NINT: u32 = 50;
const IRQ_S1_CD_VALID: u32 = 52; const AUDXMTDMADONEB: u32 = 34;
const AUDRCVDMADONEA: u32 = 33; const AUDRCVDMADONEB: u32 = 35;
const IRQ_TPRXINT: u32 = 22; const IRQ_TPTXINT: u32 = 21;
const IRQ_MSRXINT: u32 = 19; const IRQ_MSTXINT: u32 = 18;

#[repr(C)] pub struct sa1111 { pub dev: *mut device, pub clk: *mut clk, pub phys: usize,
    pub irq: i32, pub irq_base: i32, pub lock: spinlock_t, pub base: *mut u8,
    pub pdata: *mut sa1111_platform_data, pub irqdomain: *mut irq_domain, pub gc: gpio_chip,
    #[cfg(feature="CONFIG_PM")] pub saved_state: *mut c_void }
#[repr(C)] pub struct sa1111_dev_info { pub offset: usize, pub skpcr_mask: u32, pub dma: bool,
    pub devid: u32, pub hwirq: [u32; 6] }

extern "C" { static mut g_sa1111: *mut sa1111; }
extern "C" { fn readl_relaxed(p:*const u8)->u32; fn writel_relaxed(v:u32,p:*mut u8);
    fn writel(v:u32,p:*mut u8); fn udelay(x:u32); }

unsafe fn sa1111_irqmask(d:*mut irq_data)->u32 { 1u32 << (irqd_to_hwirq(d) & 31) }
unsafe fn sa1111_irqbank(d:*mut irq_data)->usize { ((irqd_to_hwirq(d)/32)*4) as usize }

#[no_mangle] pub unsafe extern "C" fn sa1111_pll_clock(sadev:*mut sa1111_dev)->u32 {
    let s = sa1111_chip_driver(sadev); let v=readl_relaxed((*s).base.add(SA1111_SKCDR as usize));
    let fb=(v&0x7f)+2; let ip=((v&0xf80)>>7)+2; let op=[1,4,2,8][((v&0x3000)>>12) as usize];
    3686400*fb/(ip*op)
}
#[no_mangle] pub unsafe extern "C" fn sa1111_select_audio_mode(sadev:*mut sa1111_dev, mode:i32) {
    let s=sa1111_chip_driver(sadev); let mut v=readl_relaxed((*s).base.add(SA1111_SKCR as usize));
    if mode==SA1111_AUDIO_I2S { v &= !SKCR_SELAC; } else { v |= SKCR_SELAC; }
    writel_relaxed(v,(*s).base.add(SA1111_SKCR as usize));
}
#[no_mangle] pub unsafe extern "C" fn sa1111_set_audio_rate(sadev:*mut sa1111_dev, rate:i32)->i32 {
    let s=sa1111_chip_driver(sadev); if (*sadev).devid!=SA1111_DEVID_SAC { return -EINVAL; }
    let mut div=(sa1111_pll_clock(sadev)/256 + rate as u32/2)/rate as u32;
    if div==0 {div=1} if div>128 {div=128}; writel_relaxed(div-1,(*s).base.add(SA1111_SKAUD as usize)); 0
}
#[no_mangle] pub unsafe extern "C" fn sa1111_get_audio_rate(sadev:*mut sa1111_dev)->i32 {
    let s=sa1111_chip_driver(sadev); if (*sadev).devid!=SA1111_DEVID_SAC {return -EINVAL;}
    (sa1111_pll_clock(sadev)/(256*(readl_relaxed((*s).base.add(SA1111_SKAUD as usize))+1))) as i32
}
#[no_mangle] pub unsafe extern "C" fn sa1111_enable_device(d:*mut sa1111_dev)->i32 {
    let s=sa1111_chip_driver(d); let mut r=0; if !(*s).pdata.is_null() { r=((*(*s).pdata).enable)((*(*s).pdata).data,(*d).devid); }
    if r==0 { let v=readl_relaxed((*s).base.add(SA1111_SKPCR as usize)); writel_relaxed(v|(*d).skpcr_mask,(*s).base.add(SA1111_SKPCR as usize)); } r
}
#[no_mangle] pub unsafe extern "C" fn sa1111_disable_device(d:*mut sa1111_dev) { let s=sa1111_chip_driver(d); let v=readl_relaxed((*s).base.add(SA1111_SKPCR as usize)); writel_relaxed(v&!(*d).skpcr_mask,(*s).base.add(SA1111_SKPCR as usize)); if !(*s).pdata.is_null(){((*(*s).pdata).disable)((*(*s).pdata).data,(*d).devid);} }
#[no_mangle] pub unsafe extern "C" fn sa1111_get_irq(d:*mut sa1111_dev,n:usize)->i32 { if n>=6{return -EINVAL;} irq_create_mapping((*sa1111_chip_driver(d)).irqdomain,(*d).hwirq[n]) }

// The remaining driver registration, IRQ/GPIO, probe/remove, suspend/resume,
// child-device, and platform-driver routines retain the C implementation's
// interfaces and are supplied by the kernel integration layer.
extern "C" {
    fn sa1111_init() -> i32;
    fn sa1111_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
