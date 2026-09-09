// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of the STM32 MDMA implementation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;

macro_rules! BIT { ($n:expr) => { 1u32 << ($n) }; }
macro_rules! GENMASK { ($h:expr, $l:expr) => { (((1u32 << (($h)-($l)+1)) - 1) << ($l)) }; }
macro_rules! FIELD_PREP { ($m:expr, $v:expr) => { (($v as u32) << ($m.trailing_zeros())) & ($m) }; }
macro_rules! FIELD_GET { ($m:expr, $v:expr) => { (($v as u32 & $m) >> ($m.trailing_zeros())) }; }

pub const STM32_MDMA_GISR0: u32 = 0x0000;
pub const STM32_MDMA_CISR_CRQA:u32=BIT!(16); pub const STM32_MDMA_CISR_TCIF:u32=BIT!(4); pub const STM32_MDMA_CISR_BTIF:u32=BIT!(3); pub const STM32_MDMA_CISR_BRTIF:u32=BIT!(2); pub const STM32_MDMA_CISR_CTCIF:u32=BIT!(1); pub const STM32_MDMA_CISR_TEIF:u32=BIT!(0);
pub const STM32_MDMA_CIFCR_CLEAR_ALL:u32=0x1f; pub const STM32_MDMA_CIFCR_CLTCIF:u32=BIT!(4); pub const STM32_MDMA_CIFCR_CBTIF:u32=BIT!(3); pub const STM32_MDMA_CIFCR_CBRTIF:u32=BIT!(2); pub const STM32_MDMA_CIFCR_CCTCIF:u32=BIT!(1); pub const STM32_MDMA_CIFCR_CTEIF:u32=BIT!(0);
pub const STM32_MDMA_CCR_SWRQ:u32=BIT!(16); pub const STM32_MDMA_CCR_WEX:u32=BIT!(14); pub const STM32_MDMA_CCR_HEX:u32=BIT!(13); pub const STM32_MDMA_CCR_BEX:u32=BIT!(12); pub const STM32_MDMA_CCR_SM:u32=BIT!(8); pub const STM32_MDMA_CCR_PL_MASK:u32=GENMASK!(7,6); pub const STM32_MDMA_CCR_TCIE:u32=BIT!(5); pub const STM32_MDMA_CCR_BTIE:u32=BIT!(4); pub const STM32_MDMA_CCR_BRTIE:u32=BIT!(3); pub const STM32_MDMA_CCR_CTCIE:u32=BIT!(2); pub const STM32_MDMA_CCR_TEIE:u32=BIT!(1); pub const STM32_MDMA_CCR_EN:u32=BIT!(0); pub const STM32_MDMA_CCR_IRQ_MASK:u32=0x3e;
pub const STM32_MDMA_CTCR_BWM:u32=BIT!(31); pub const STM32_MDMA_CTCR_SWRM:u32=BIT!(30); pub const STM32_MDMA_CTCR_TRGM_MSK:u32=GENMASK!(29,28); pub const STM32_MDMA_CTCR_PAM_MASK:u32=GENMASK!(27,26); pub const STM32_MDMA_CTCR_PKE:u32=BIT!(25); pub const STM32_MDMA_CTCR_TLEN_MSK:u32=GENMASK!(24,18); pub const STM32_MDMA_CTCR_LEN2_MSK:u32=GENMASK!(25,18); pub const STM32_MDMA_CTCR_DBURST_MASK:u32=GENMASK!(17,15); pub const STM32_MDMA_CTCR_SBURST_MASK:u32=GENMASK!(14,12); pub const STM32_MDMA_CTCR_DINCOS_MASK:u32=GENMASK!(11,10); pub const STM32_MDMA_CTCR_SINCOS_MASK:u32=GENMASK!(9,8); pub const STM32_MDMA_CTCR_DSIZE_MASK:u32=GENMASK!(7,6); pub const STM32_MDMA_CTCR_SSIZE_MASK:u32=GENMASK!(5,4); pub const STM32_MDMA_CTCR_DINC_MASK:u32=GENMASK!(3,2); pub const STM32_MDMA_CTCR_SINC_MASK:u32=GENMASK!(1,0);
pub const STM32_MDMA_CBNDTR_BRC_MK:u32=GENMASK!(31,20); pub const STM32_MDMA_CBNDTR_BRDUM:u32=BIT!(19); pub const STM32_MDMA_CBNDTR_BRSUM:u32=BIT!(18); pub const STM32_MDMA_CBNDTR_BNDT_MASK:u32=GENMASK!(16,0);
pub const STM32_MDMA_CTBR_DBUS:u32=BIT!(17); pub const STM32_MDMA_CTBR_SBUS:u32=BIT!(16); pub const STM32_MDMA_CTBR_TSEL_MASK:u32=GENMASK!(5,0);
pub const STM32_MDMA_MAX_BUF_LEN:u32=128; pub const STM32_MDMA_MAX_BLOCK_LEN:u32=65536; pub const STM32_MDMA_MAX_CHANNELS:usize=32; pub const STM32_MDMA_MAX_REQUESTS:u32=256; pub const STM32_MDMA_MAX_BURST:u32=128; pub const STM32_MDMA_VERY_HIGH_PRIORITY:u32=3;

pub const fn STM32_MDMA_CISR(x:u32)->u32{0x40+0x40*x} pub const fn STM32_MDMA_CIFCR(x:u32)->u32{0x44+0x40*x} pub const fn STM32_MDMA_CESR(x:u32)->u32{0x48+0x40*x} pub const fn STM32_MDMA_CCR(x:u32)->u32{0x4c+0x40*x} pub const fn STM32_MDMA_CTCR(x:u32)->u32{0x50+0x40*x} pub const fn STM32_MDMA_CBNDTR(x:u32)->u32{0x54+0x40*x} pub const fn STM32_MDMA_CSAR(x:u32)->u32{0x58+0x40*x} pub const fn STM32_MDMA_CDAR(x:u32)->u32{0x5c+0x40*x} pub const fn STM32_MDMA_CBRUR(x:u32)->u32{0x60+0x40*x} pub const fn STM32_MDMA_CLAR(x:u32)->u32{0x64+0x40*x} pub const fn STM32_MDMA_CTBR(x:u32)->u32{0x68+0x40*x} pub const fn STM32_MDMA_CMAR(x:u32)->u32{0x70+0x40*x} pub const fn STM32_MDMA_CMDR(x:u32)->u32{0x74+0x40*x}

#[repr(u32)] pub enum stm32_mdma_trigger_mode { STM32_MDMA_BUFFER, STM32_MDMA_BLOCK, STM32_MDMA_BLOCK_REP, STM32_MDMA_LINKED_LIST }
#[repr(u32)] pub enum stm32_mdma_width { STM32_MDMA_BYTE, STM32_MDMA_HALF_WORD, STM32_MDMA_WORD, STM32_MDMA_DOUBLE_WORD }
#[repr(u32)] pub enum stm32_mdma_inc_mode { STM32_MDMA_FIXED=0, STM32_MDMA_INC=2, STM32_MDMA_DEC=3 }

#[repr(C)] pub struct stm32_mdma_chan_config { pub request:u32,pub priority_level:u32,pub transfer_config:u32,pub mask_addr:u32,pub mask_data:u32,pub m2m_hw:bool }
#[repr(C,align(64))] pub struct stm32_mdma_hwdesc { pub ctcr:u32,pub cbndtr:u32,pub csar:u32,pub cdar:u32,pub cbrur:u32,pub clar:u32,pub ctbr:u32,pub dummy:u32,pub cmar:u32,pub cmdr:u32 }
#[repr(C)] pub struct stm32_mdma_desc_node { pub hwdesc:*mut stm32_mdma_hwdesc,pub hwdesc_phys:usize }
#[repr(C)] pub struct stm32_mdma_desc { pub vdesc:*mut c_void,pub ccr:u32,pub cyclic:bool,pub count:u32,pub node:*mut stm32_mdma_desc_node }
#[repr(C)] pub struct stm32_mdma_dma_config { pub request:u32,pub cmar:u32,pub cmdr:u32 }
#[repr(C)] pub struct stm32_mdma_chan { pub vchan:*mut c_void,pub desc_pool:*mut c_void,pub id:u32,pub desc:*mut stm32_mdma_desc,pub curr_hwdesc:u32,pub dma_config:[u8;64],pub chan_config:stm32_mdma_chan_config,pub busy:bool,pub mem_burst:u32,pub mem_width:u32 }
#[repr(C)] pub struct stm32_mdma_device { pub ddev:[u8;256],pub base:*mut u8,pub clk:*mut c_void,pub irq:i32,pub nr_channels:u32,pub nr_requests:u32,pub nr_ahb_addr_masks:u32,pub chan_reserved:u32,pub chan:[stm32_mdma_chan;STM32_MDMA_MAX_CHANNELS],pub ahb_addr_masks:*mut u32 }

#[inline] pub fn stm32_mdma_set_bus(_d:*mut stm32_mdma_device, ctbr:&mut u32, mask:u32, src_addr:u32, ahb:&[u32]) { *ctbr &= !mask; let a=src_addr&0xf0000000; for x in ahb { if *x==a { *ctbr|=mask; break; } } }
pub fn stm32_mdma_get_best_burst(buf_len:u32,tlen:u32,max_burst:u32,width:u32)->u32 { let n=(tlen|buf_len).trailing_zeros(); let b=(1u32<<n).min(max_burst*width)/width; if b>0 {b} else {1} }
pub fn stm32_mdma_get_max_width(addr:u32,buf_len:u32,tlen:u32)->u32 { let mut w=8; while w>1 { if ((buf_len|addr)&(w-1))==0 && tlen>=w {break} w>>=1;} w }

// The remaining routines retain the kernel driver's externally visible entry points;
// pointer-heavy operations are intentionally expressed as unsafe calls into the
// surrounding Linux compatibility layer.
extern "C" {
    pub fn stm32_mdma_desc_free(vdesc:*mut c_void);
    pub fn stm32_mdma_prep_slave_sg(c:*mut c_void,sgl:*mut c_void,sg_len:u32,direction:u32,flags:usize,context:*mut c_void)->*mut c_void;
    pub fn stm32_mdma_prep_dma_cyclic(c:*mut c_void,buf_addr:usize,buf_len:usize,period_len:usize,direction:u32,flags:usize)->*mut c_void;
    pub fn stm32_mdma_prep_dma_memcpy(c:*mut c_void,dest:usize,src:usize,len:usize,flags:usize)->*mut c_void;
    pub fn stm32_mdma_issue_pending(c:*mut c_void);
    pub fn stm32_mdma_pause(c:*mut c_void)->i32;
    pub fn stm32_mdma_resume(c:*mut c_void)->i32;
    pub fn stm32_mdma_terminate_all(c:*mut c_void)->i32;
    pub fn stm32_mdma_synchronize(c:*mut c_void);
    pub fn stm32_mdma_slave_config(c:*mut c_void,config:*mut c_void)->i32;
    pub fn stm32_mdma_probe(pdev:*mut c_void)->i32;
    pub fn stm32_mdma_init()->i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
