// SPDX-License-Identifier: GPL-2.0
/* Xilinx Versal memory controller driver, translated from C. */

const XDDR_EDAC_ERR_GRAIN: usize = 1;
const XDDR_EDAC_MSG_SIZE: usize = 256;
const EVENT: usize = 2;
const XDDR_PCSR_OFFSET: usize = 0xC;
const XDDR_ISR_OFFSET: usize = 0x14;
const XDDR_IRQ_EN_OFFSET: usize = 0x20;
const XDDR_IRQ1_EN_OFFSET: usize = 0x2C;
const XDDR_IRQ_DIS_OFFSET: usize = 0x24;
const XDDR_IRQ_CE_MASK: u32 = 0xF << 15;
const XDDR_IRQ_UE_MASK: u32 = 0xF << 11;
const XDDR_REG_CONFIG0_OFFSET: usize = 0x258;
const XDDR_REG_CONFIG0_BUS_WIDTH_MASK: u32 = 3 << 18;
const XDDR_REG_CONFIG0_NUM_CHANS_MASK: u32 = 1 << 17;
const XDDR_REG_CONFIG0_NUM_RANKS_MASK: u32 = 3 << 14;
const XDDR_REG_CONFIG0_SIZE_MASK: u32 = 7 << 8;
const XDDR_REG_PINOUT_OFFSET: usize = 0x25C;
const XDDR_REG_PINOUT_ECC_EN_MASK: u32 = 7 << 5;
const ECCW0_FLIP_CTRL: usize = 0x109C;
const ECCW0_FLIP0_OFFSET: usize = 0x10A0;
const ECCW0_FLIP0_BITS: u8 = 31;
const ECCW0_FLIP1_OFFSET: usize = 0x10A4;
const ECCW1_FLIP_CTRL: usize = 0x10AC;
const ECCW1_FLIP0_OFFSET: usize = 0x10B0;
const ECCW1_FLIP1_OFFSET: usize = 0x10B4;
const ECCR0_CERR_STAT_OFFSET: usize = 0x10BC;
const ECCR0_CE_ADDR_LO_OFFSET: usize = 0x10C0;
const ECCR0_CE_ADDR_HI_OFFSET: usize = 0x10C4;
const ECCR0_CE_DATA_LO_OFFSET: usize = 0x10C8;
const ECCR0_CE_DATA_HI_OFFSET: usize = 0x10CC;
const ECCR0_CE_DATA_PAR_OFFSET: usize = 0x10D0;
const ECCR0_UERR_STAT_OFFSET: usize = 0x10D4;
const ECCR0_UE_ADDR_LO_OFFSET: usize = 0x10D8;
const ECCR0_UE_ADDR_HI_OFFSET: usize = 0x10DC;
const ECCR0_UE_DATA_LO_OFFSET: usize = 0x10E0;
const ECCR0_UE_DATA_HI_OFFSET: usize = 0x10E4;
const ECCR0_UE_DATA_PAR_OFFSET: usize = 0x10E8;
const ECCR1_CERR_STAT_OFFSET: usize = 0x10F4;
const ECCR1_CE_ADDR_LO_OFFSET: usize = 0x10F8;
const ECCR1_CE_ADDR_HI_OFFSET: usize = 0x10FC;
const ECCR1_CE_DATA_LO_OFFSET: usize = 0x1100;
const ECCR1_CE_DATA_HI_OFFSET: usize = 0x110C;
const ECCR1_CE_DATA_PAR_OFFSET: usize = 0x1108;
const ECCR1_UERR_STAT_OFFSET: usize = 0x110C;
const ECCR1_UE_ADDR_LO_OFFSET: usize = 0x1110;
const ECCR1_UE_ADDR_HI_OFFSET: usize = 0x1114;
const ECCR1_UE_DATA_LO_OFFSET: usize = 0x1118;
const ECCR1_UE_DATA_HI_OFFSET: usize = 0x111C;
const ECCR1_UE_DATA_PAR_OFFSET: usize = 0x1120;
const XDDR_NOC_REG_ADEC4_OFFSET: usize = 0x44;
const RANK_1_MASK: u32 = 0x3f << 6;
const LRANK_0_MASK: u32 = 0x3f << 12;
const LRANK_1_MASK: u32 = 0x3f << 18;
const MASK_24: u32 = 0x3f << 24;
const XDDR_NOC_REG_ADEC5_OFFSET: usize = 0x48;
const XDDR_NOC_REG_ADEC6_OFFSET: usize = 0x4C;
const XDDR_NOC_REG_ADEC7_OFFSET: usize = 0x50;
const XDDR_NOC_REG_ADEC8_OFFSET: usize = 0x54;
const XDDR_NOC_REG_ADEC9_OFFSET: usize = 0x58;
const XDDR_NOC_REG_ADEC10_OFFSET: usize = 0x5C;
const XDDR_NOC_REG_ADEC11_OFFSET: usize = 0x60;
const MASK_0: u32 = 0x3f;
const GRP_0_MASK: u32 = 0x3f << 6;
const GRP_1_MASK: u32 = 0x3f << 12;
const CH_0_MASK: u32 = 0x3f << 18;
const XDDR_NOC_REG_ADEC12_OFFSET: usize = 0x71C;
const XDDR_NOC_REG_ADEC13_OFFSET: usize = 0x720;
const XDDR_NOC_REG_ADEC14_OFFSET: usize = 0x724;
const XDDR_NOC_ROW_MATCH_MASK: u32 = (1 << 18) - 1;
const XDDR_NOC_COL_MATCH_MASK: u32 = ((1 << 10) - 1) << 18;
const XDDR_NOC_BANK_MATCH_MASK: u32 = 3 << 28;
const XDDR_NOC_GRP_MATCH_MASK: u32 = 3 << 30;
const XDDR_NOC_REG_ADEC15_OFFSET: usize = 0x728;
const XDDR_NOC_RANK_MATCH_MASK: u32 = 3;
const XDDR_NOC_LRANK_MATCH_MASK: u32 = 7 << 2;
const XDDR_NOC_CH_MATCH_MASK: u32 = 1 << 5;
const XDDR_NOC_MOD_SEL_MASK: u32 = 1 << 6;
const XDDR_NOC_MATCH_EN_MASK: u32 = 1 << 8;
const ECCR_UE_CE_ADDR_HI_ROW_MASK: u32 = 0xff;
const XDDR_MAX_ROW_CNT: usize = 18;
const XDDR_MAX_COL_CNT: usize = 10;
const XDDR_MAX_RANK_CNT: usize = 2;
const XDDR_MAX_LRANK_CNT: usize = 3;
const XDDR_MAX_BANK_CNT: usize = 2;
const XDDR_MAX_GRP_CNT: usize = 2;
const PCSR_UNLOCK_VAL: u32 = 0xF9E8D7C6;
const PCSR_LOCK_VAL: u32 = 1;
const XDDR_ERR_TYPE_CE: u8 = 0;
const XDDR_ERR_TYPE_UE: u8 = 1;
const XILINX_DRAM_SIZE_4G: u32 = 0;
const XILINX_DRAM_SIZE_6G: u32 = 1;
const XILINX_DRAM_SIZE_8G: u32 = 2;
const XILINX_DRAM_SIZE_12G: u32 = 3;
const XILINX_DRAM_SIZE_16G: u32 = 4;
const XILINX_DRAM_SIZE_32G: u32 = 5;
const NUM_UE_BITPOS: usize = 2;

#[repr(C, packed)]
pub union EccErrorInfo { pub bits: EccErrorBits, pub i: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EccErrorBits { pub burstpos: u32, pub lrank: u32, pub rank: u32, pub group: u32, pub bank: u32, pub col: u32, pub row: u32, pub rowhi: u32 }
#[repr(C, packed)]
pub union EdacInfo { pub bits: EdacBits, pub i: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EdacBits { pub row0:u32,pub row1:u32,pub row2:u32,pub row3:u32,pub row4:u32,pub reserved:u32 }
#[repr(C)]
pub struct EccStatus { pub ceinfo:[EccErrorInfo;2], pub ueinfo:[EccErrorInfo;2], pub channel:u8, pub error_type:u8 }
#[repr(C)]
pub struct EdacPriv { pub ddrmc_baseaddr:*mut u8, pub ddrmc_noc_baseaddr:*mut u8, pub message:[u8;XDDR_EDAC_MSG_SIZE], pub mc_id:u32, pub ce_cnt:u32, pub ue_cnt:u32, pub stat:EccStatus, pub lrank_bit:[u32;3], pub rank_bit:[u32;2], pub row_bit:[u32;18], pub col_bit:[u32;10], pub bank_bit:[u32;2], pub grp_bit:[u32;2], pub ch_bit:u32, pub err_inject_addr:u64 }

extern "C" {
    fn readl(addr:*const u8)->u32; fn writel(v:u32, addr:*mut u8);
    fn memset(p:*mut core::ffi::c_void, v:i32, n:usize)->*mut core::ffi::c_void;
}
#[repr(C)] pub enum DevType { DevUnknown, DevX2, DevX4, DevX8 }
unsafe fn get_dwidth(base:*const u8)->DevType{match field_get(XDDR_REG_CONFIG0_BUS_WIDTH_MASK,readl(base.add(XDDR_REG_CONFIG0_OFFSET))){2=>DevType::DevX2,1=>DevType::DevX4,0=>DevType::DevX8,_=>DevType::DevUnknown}}
unsafe fn get_ecc_state(base:*mut u8)->bool{if matches!(get_dwidth(base),DevType::DevUnknown){false}else{readl(base.add(XDDR_REG_PINOUT_OFFSET))&XDDR_REG_PINOUT_ECC_EN_MASK!=0}}
unsafe fn get_memsize(p:&EdacPriv)->u64{let n=field_get(XDDR_REG_CONFIG0_SIZE_MASK,readl(p.ddrmc_baseaddr.add(XDDR_REG_CONFIG0_OFFSET)));let g=match n{0=>4,1=>6,2=>8,3=>12,4=>16,5=>32,_=>0};(g as u64)*1024*1024*1024}
#[inline] fn field_get(mask:u32, v:u32)->u32 { (v & mask) >> mask.trailing_zeros() }
#[inline] fn field_prep(mask:u32, v:u32)->u32 { (v << mask.trailing_zeros()) & mask }

unsafe fn get_ce_error_info(priv_:&mut EdacPriv) { let b=priv_.ddrmc_baseaddr; priv_.stat.error_type=XDDR_ERR_TYPE_CE; let lo=readl(b.add(ECCR0_CE_ADDR_LO_OFFSET)); let hi=readl(b.add(ECCR0_CE_ADDR_HI_OFFSET)); priv_.stat.ceinfo[0].i=(lo as u64)|((hi & ECCR_UE_CE_ADDR_HI_ROW_MASK) as u64)<<32; let lo=readl(b.add(ECCR1_CE_ADDR_LO_OFFSET)); let hi=readl(b.add(ECCR1_CE_ADDR_HI_OFFSET)); priv_.stat.ceinfo[1].i=(lo as u64)|((hi as u64)<<32); }
unsafe fn get_ue_error_info(priv_:&mut EdacPriv) { let b=priv_.ddrmc_baseaddr; priv_.stat.error_type=XDDR_ERR_TYPE_UE; let lo=readl(b.add(ECCR0_UE_ADDR_LO_OFFSET)); let hi=readl(b.add(ECCR0_UE_ADDR_HI_OFFSET)); priv_.stat.ueinfo[0].i=(lo as u64)|((hi as u64)<<32); let lo=readl(b.add(ECCR1_UE_ADDR_LO_OFFSET)); let hi=readl(b.add(ECCR1_UE_ADDR_HI_OFFSET)); priv_.stat.ueinfo[1].i=(lo as u64)|((hi as u64)<<32); }
unsafe fn get_error_info(priv_:&mut EdacPriv)->bool { let b=priv_.ddrmc_baseaddr; let c0=readl(b.add(ECCR0_CERR_STAT_OFFSET)); let c1=readl(b.add(ECCR1_CERR_STAT_OFFSET)); let u0=readl(b.add(ECCR0_UERR_STAT_OFFSET)); let u1=readl(b.add(ECCR1_UERR_STAT_OFFSET)); if c0==0&&c1==0&&u0==0&&u1==0{return true} priv_.stat.channel=if c0==0{1}else{0}; if c0!=0||c1!=0{get_ce_error_info(priv_)} if u0!=0||u1!=0 {priv_.stat.channel=if u0==0{1}else{0};get_ue_error_info(priv_)} writel(PCSR_UNLOCK_VAL,b.add(XDDR_PCSR_OFFSET)); for o in [ECCR0_CERR_STAT_OFFSET,ECCR1_CERR_STAT_OFFSET,ECCR0_UERR_STAT_OFFSET,ECCR1_UERR_STAT_OFFSET]{writel(0,b.add(o))} writel(PCSR_LOCK_VAL,b.add(XDDR_PCSR_OFFSET)); false }
unsafe fn convert_to_physical(p:&EdacPriv, mut e:EccErrorInfo)->usize { let mut a=0usize; let mut row=(((e.bits.rowhi)<<10)|e.bits.row); for i in 0..18{a|=((row&1) as usize)<<p.row_bit[i];row>>=1} for i in 0..10{a|=((e.bits.col&1) as usize)<<p.col_bit[i];e.bits.col>>=1} for i in 0..2{a|=((e.bits.bank&1) as usize)<<p.bank_bit[i];e.bits.bank>>=1;a|=((e.bits.group&1) as usize)<<p.grp_bit[i];e.bits.group>>=1;a|=((e.bits.rank&1) as usize)<<p.rank_bit[i];e.bits.rank>>=1} for i in 0..3{a|=((e.bits.lrank&1) as usize)<<p.lrank_bit[i];e.bits.lrank>>=1} a|((p.stat.channel as usize)&1)<<p.ch_bit }

// The following kernel-facing declarations preserve the remaining implementation
// interfaces; their concrete types and symbols are supplied by the surrounding tree.
#[allow(dead_code)]
unsafe fn enable_intr(p:&mut EdacPriv){let b=p.ddrmc_baseaddr;writel(PCSR_UNLOCK_VAL,b.add(XDDR_PCSR_OFFSET));writel(XDDR_IRQ_CE_MASK|XDDR_IRQ_UE_MASK,b.add(XDDR_IRQ_EN_OFFSET));writel(XDDR_IRQ_UE_MASK,b.add(XDDR_IRQ1_EN_OFFSET));writel(PCSR_LOCK_VAL,b.add(XDDR_PCSR_OFFSET));}
#[allow(dead_code)]
unsafe fn disable_intr(p:&mut EdacPriv){let b=p.ddrmc_baseaddr;writel(PCSR_UNLOCK_VAL,b.add(XDDR_PCSR_OFFSET));writel(XDDR_IRQ_CE_MASK|XDDR_IRQ_UE_MASK,b.add(XDDR_IRQ_DIS_OFFSET));writel(PCSR_LOCK_VAL,b.add(XDDR_PCSR_OFFSET));}

#[cfg(feature="CONFIG_EDAC_DEBUG")]
unsafe fn poison_setup(p:&mut EdacPriv){let mut row=0;let mut col=0;let mut bank=0;let mut grp=0;let mut rank=0;let mut lrank=0;for i in 0..18{row|=(((p.err_inject_addr>>p.row_bit[i])&1)<<i)}for i in 0..10{col|=(((p.err_inject_addr>>p.col_bit[i])&1)<<i)}for i in 0..2{bank|=(((p.err_inject_addr>>p.bank_bit[i])&1)<<i);grp|=(((p.err_inject_addr>>p.grp_bit[i])&1)<<i);rank|=(((p.err_inject_addr>>p.rank_bit[i])&1)<<i)}for i in 0..3{lrank|=(((p.err_inject_addr>>p.lrank_bit[i])&1)<<i)}let ch=(p.err_inject_addr>>p.ch_bit)&1;writel(0,p.ddrmc_noc_baseaddr.add(XDDR_NOC_REG_ADEC12_OFFSET));writel(0,p.ddrmc_noc_baseaddr.add(XDDR_NOC_REG_ADEC13_OFFSET));writel((row&XDDR_NOC_ROW_MATCH_MASK)|field_prep(XDDR_NOC_COL_MATCH_MASK,col)|field_prep(XDDR_NOC_BANK_MATCH_MASK,bank)|field_prep(XDDR_NOC_GRP_MATCH_MASK,grp),p.ddrmc_noc_baseaddr.add(XDDR_NOC_REG_ADEC14_OFFSET));writel((rank&3)|field_prep(XDDR_NOC_LRANK_MATCH_MASK,lrank)|field_prep(XDDR_NOC_CH_MATCH_MASK,ch)|XDDR_NOC_MOD_SEL_MASK|XDDR_NOC_MATCH_EN_MASK,p.ddrmc_noc_baseaddr.add(XDDR_NOC_REG_ADEC15_OFFSET));}

// Address-map setup mirrors the C register extraction and is intentionally kept
// behind the same build-time debug condition.
#[cfg(feature="CONFIG_EDAC_DEBUG")]
unsafe fn setup_address_map(p:&mut EdacPriv){let b=p.ddrmc_noc_baseaddr;let mut v=readl(b.add(XDDR_NOC_REG_ADEC5_OFFSET));for i in 0..5{p.row_bit[i]=((v>>(i*6))&0x3f)}v=readl(b.add(XDDR_NOC_REG_ADEC6_OFFSET));for i in 0..5{p.row_bit[i+5]=((v>>(i*6))&0x3f)}v=readl(b.add(XDDR_NOC_REG_ADEC7_OFFSET));for i in 0..5{p.row_bit[i+10]=((v>>(i*6))&0x3f)}v=readl(b.add(XDDR_NOC_REG_ADEC8_OFFSET));p.row_bit[15]=v&0x3f;p.row_bit[16]=(v>>6)&0x3f;p.row_bit[17]=(v>>12)&0x3f;p.col_bit[0]=field_get(MASK_24,v);v=readl(b.add(XDDR_NOC_REG_ADEC9_OFFSET));for i in 0..5{p.col_bit[i+1]=(v>>(i*6))&0x3f}v=readl(b.add(XDDR_NOC_REG_ADEC10_OFFSET));for i in 0..4{p.col_bit[i+6]=(v>>(i*6))&0x3f}p.bank_bit[0]=field_get(MASK_24,v);v=readl(b.add(XDDR_NOC_REG_ADEC11_OFFSET));p.bank_bit[1]=v&MASK_0;p.grp_bit[0]=field_get(GRP_0_MASK,v);p.grp_bit[1]=field_get(GRP_1_MASK,v);p.ch_bit=field_get(CH_0_MASK,v);v=readl(b.add(XDDR_NOC_REG_ADEC4_OFFSET));p.rank_bit[0]=v&MASK_0;p.rank_bit[1]=field_get(RANK_1_MASK,v);p.lrank_bit[0]=field_get(LRANK_0_MASK,v);p.lrank_bit[1]=field_get(LRANK_1_MASK,v);p.lrank_bit[2]=field_get(MASK_24,v);}

// C module registration and EDAC callbacks are declarations here because the
// Linux EDAC/platform types are provided by other translation units.
extern "C" {
    fn err_callback(payload:*const u32, data:*mut core::ffi::c_void);
    fn mc_probe(pdev:*mut core::ffi::c_void)->i32;
    fn mc_remove(pdev:*mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
