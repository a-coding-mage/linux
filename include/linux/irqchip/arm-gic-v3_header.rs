/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2013, 2014 ARM Limited, All Rights Reserved. */

// Direct translation of linux/irqchip/arm-gic-v3.h.  External kernel types,
// constants, and architecture helpers are supplied by the surrounding crate.

pub const GICD_CTLR: u32 = 0x0000; pub const GICD_TYPER: u32 = 0x0004;
pub const GICD_IIDR: u32 = 0x0008; pub const GICD_TYPER2: u32 = 0x000c;
pub const GICD_STATUSR: u32 = 0x0010; pub const GICD_SETSPI_NSR: u32 = 0x0040;
pub const GICD_CLRSPI_NSR: u32 = 0x0048; pub const GICD_SETSPI_SR: u32 = 0x0050;
pub const GICD_CLRSPI_SR: u32 = 0x0058; pub const GICD_IGROUPR: u32 = 0x0080;
pub const GICD_ISENABLER: u32 = 0x0100; pub const GICD_ICENABLER: u32 = 0x0180;
pub const GICD_ISPENDR: u32 = 0x0200; pub const GICD_ICPENDR: u32 = 0x0280;
pub const GICD_ISACTIVER: u32 = 0x0300; pub const GICD_ICACTIVER: u32 = 0x0380;
pub const GICD_IPRIORITYR: u32 = 0x0400; pub const GICD_ICFGR: u32 = 0x0c00;
pub const GICD_IGRPMODR: u32 = 0x0d00; pub const GICD_NSACR: u32 = 0x0e00;
pub const GICD_IGROUPRnE: u32 = 0x1000; pub const GICD_ISENABLERnE: u32 = 0x1200;
pub const GICD_ICENABLERnE: u32 = 0x1400; pub const GICD_ISPENDRnE: u32 = 0x1600;
pub const GICD_ICPENDRnE: u32 = 0x1800; pub const GICD_ISACTIVERnE: u32 = 0x1a00;
pub const GICD_ICACTIVERnE: u32 = 0x1c00; pub const GICD_IPRIORITYRnE: u32 = 0x2000;
pub const GICD_ICFGRnE: u32 = 0x3000; pub const GICD_IROUTER: u32 = 0x6000;
pub const GICD_IROUTERnE: u32 = 0x8000; pub const GICD_IDREGS: u32 = 0xffd0;
pub const GICD_PIDR2: u32 = 0xffe8; pub const ESPI_BASE_INTID: u32 = 4096;
pub const GICD_ITARGETSR: u32 = 0x0800; pub const GICD_SGIR: u32 = 0x0f00;
pub const GICD_CPENDSGIR: u32 = 0x0f10; pub const GICD_SPENDSGIR: u32 = 0x0f20;
pub const GICD_CTLR_RWP: u32 = 1<<31; pub const GICD_CTLR_nASSGIreq: u32 = 1<<8;
pub const GICD_CTLR_DS: u32 = 1<<6; pub const GICD_CTLR_ARE_NS: u32 = 1<<4;
pub const GICD_CTLR_ENABLE_G1A: u32 = 1<<1; pub const GICD_CTLR_ENABLE_G1: u32 = 1;
pub const GICD_CTLR_ENABLE_SS_G1: u32 = 1<<1; pub const GICD_CTLR_ENABLE_SS_G0: u32 = 1;
pub const GICD_TYPER_RSS: u32=1<<26; pub const GICD_TYPER_LPIS:u32=1<<17;
pub const GICD_TYPER_MBIS:u32=1<<16; pub const GICD_TYPER_ESPI:u32=1<<8;
pub const GICD_TYPER2_nASSGIcap:u32=1<<8; pub const GICD_TYPER2_VIL:u32=1<<7;
pub const GICD_TYPER2_VID:u32=0x1f; pub const GICD_IROUTER_SPI_MODE_ONE:u32=0;
pub const GICD_IROUTER_SPI_MODE_ANY:u32=1<<31; pub const GIC_PIDR2_ARCH_MASK:u32=0xf0;
pub const GIC_PIDR2_ARCH_GICv3:u32=0x30; pub const GIC_PIDR2_ARCH_GICv4:u32=0x40;
pub const GIC_V3_DIST_SIZE:u32=0x10000; pub const GIC_PAGE_SIZE_4K:u64=0;
pub const GIC_PAGE_SIZE_16K:u64=1; pub const GIC_PAGE_SIZE_64K:u64=2; pub const GIC_PAGE_SIZE_MASK:u64=3;
pub const GICR_CTLR:u32=GICD_CTLR; pub const GICR_IIDR:u32=4; pub const GICR_TYPER:u32=8;
pub const GICR_STATUSR:u32=GICD_STATUSR; pub const GICR_WAKER:u32=0x14;
pub const GICR_SETLPIR:u32=0x40; pub const GICR_CLRLPIR:u32=0x48; pub const GICR_PROPBASER:u32=0x70;
pub const GICR_PENDBASER:u32=0x78; pub const GICR_INVLPIR:u32=0xa0; pub const GICR_INVALLR:u32=0xb0;
pub const GICR_SYNCR:u32=0xc0; pub const GICR_IDREGS:u32=GICD_IDREGS; pub const GICR_PIDR2:u32=GICD_PIDR2;
pub const GICR_CTLR_ENABLE_LPIS:u64=1; pub const GICR_CTLR_CES:u64=1<<1; pub const GICR_CTLR_IR:u64=1<<2; pub const GICR_CTLR_RWP:u64=1<<3;
pub const EPPI_BASE_INTID:u32=1056; pub const GICR_WAKER_ProcessorSleep:u32=1<<1; pub const GICR_WAKER_ChildrenAsleep:u32=1<<2;
pub const GIC_V3_REDIST_SIZE:u32=0x20000; pub const LPI_PROP_GROUP1:u32=1<<1; pub const LPI_PROP_ENABLED:u32=1;
pub const GICR_VPROPBASER:u32=0x70; pub const GICR_VPENDBASER:u32=0x78; pub const GICR_VSGIR:u32=0x80; pub const GICR_VSGIPENDR:u32=0x88;
pub const GICR_VSGIPENDR_BUSY:u32=1<<31; pub const GICR_VSGIPENDR_PENDING:u32=0xffff;
pub const GITS_CTLR:u32=0; pub const GITS_IIDR:u32=4; pub const GITS_TYPER:u32=8; pub const GITS_MPIDR:u32=0x18;
pub const GITS_CBASER:u32=0x80; pub const GITS_CWRITER:u32=0x88; pub const GITS_CREADR:u32=0x90; pub const GITS_BASER:u32=0x100;
pub const GITS_IDREGS_BASE:u32=0xffd0; pub const GITS_PIDR0:u32=0xffe0; pub const GITS_PIDR1:u32=0xffe4;
pub const GITS_PIDR2:u32=GICR_PIDR2; pub const GITS_PIDR4:u32=0xffd0; pub const GITS_CIDR0:u32=0xfff0; pub const GITS_CIDR1:u32=0xfff4; pub const GITS_CIDR2:u32=0xfff8; pub const GITS_CIDR3:u32=0xfffc;
pub const GITS_TRANSLATER:u32=0x10040; pub const GITS_SGIR:u32=0x20020; pub const GITS_CTLR_ENABLE:u32=1; pub const GITS_CTLR_ImDe:u32=2; pub const GITS_CTLR_QUIESCENT:u32=1<<31;
pub const GITS_BASER_NR_REGS:u32=8; pub const GITS_BASER_VALID:u64=1<<63; pub const GITS_BASER_INDIRECT:u64=1<<62;
pub const GITS_BASER_TYPE_NONE:u32=0; pub const GITS_BASER_TYPE_DEVICE:u32=1; pub const GITS_BASER_TYPE_VCPU:u32=2; pub const GITS_BASER_TYPE_RESERVED3:u32=3; pub const GITS_BASER_TYPE_COLLECTION:u32=4; pub const GITS_BASER_TYPE_RESERVED5:u32=5; pub const GITS_BASER_TYPE_RESERVED6:u32=6; pub const GITS_BASER_TYPE_RESERVED7:u32=7;
pub const GITS_LVL1_ENTRY_SIZE:u64=8;

pub const GITS_CMD_MAPD:u8=8; pub const GITS_CMD_MAPC:u8=9; pub const GITS_CMD_MAPTI:u8=0xa; pub const GITS_CMD_MAPI:u8=0xb; pub const GITS_CMD_MOVI:u8=1; pub const GITS_CMD_DISCARD:u8=0xf; pub const GITS_CMD_INV:u8=0xc; pub const GITS_CMD_MOVALL:u8=0xe; pub const GITS_CMD_INVALL:u8=0xd; pub const GITS_CMD_INT:u8=3; pub const GITS_CMD_CLEAR:u8=4; pub const GITS_CMD_SYNC:u8=5;
pub const fn GITS_CMD_GICv4(x:u8)->u8{x|0x20} pub const GITS_CMD_VMOVP:u8=GITS_CMD_GICv4(2); pub const GITS_CMD_VSGI:u8=GITS_CMD_GICv4(3); pub const GITS_CMD_INVDB:u8=GITS_CMD_GICv4(0xe);
pub const GIC_IRQ_TYPE_LPI:u32=0xa110c8ed; pub const ICC_SRE_EL1_DIB:u32=1<<2; pub const ICC_SRE_EL1_DFB:u32=1<<1; pub const ICC_SRE_EL1_SRE:u32=1;
pub const ICC_IAR1_EL1_SPURIOUS:u32=0x3ff; pub const ICC_SRE_EL2_SRE:u32=1; pub const ICC_SRE_EL2_ENABLE:u32=1<<3;

pub const fn GICD_TYPER_ID_BITS(typer:u64)->u64{((typer>>19)&0x1f)+1}
pub const fn GICD_TYPER_NUM_LPIS(typer:u64)->u64{((typer>>11)&0x1f)+1}
pub const fn GICD_TYPER_SPIS(typer:u64)->u64{((typer&0x1f)+1)*32}
pub const fn GICD_TYPER_ESPIS(typer:u64)->u64{if typer&(1<<8)!=0{GICD_TYPER_SPIS(typer>>27)}else{0}}
pub const fn GICR_TYPER_CPU_NUMBER(r:u64)->u64{(r>>8)&0xffff}
pub const fn GICR_TYPER_NR_PPIS(r:u64)->u32{let n=((r>>27)&0x1f) as u32;if n==1||n==2{16+n*32}else{16}}
pub const fn GITS_TYPER_HCC(r:u64)->u64{(r>>24)&0xff} pub const fn GITS_IIDR_REV(r:u64)->u64{(r>>12)&0xf}
pub const fn GITS_BASER_TYPE(r:u64)->u64{(r>>56)&7} pub const fn GITS_BASER_ENTRY_SIZE(r:u64)->u64{((r>>48)&0x1f)+1} pub const fn GITS_BASER_NR_PAGES(r:u64)->u64{(r&0xff)+1}
pub const fn GITS_CMD_VINVALL()->u8{GITS_CMD_GICv4(GITS_CMD_INVALL)} pub const fn GITS_CMD_VMAPP()->u8{GITS_CMD_GICv4(GITS_CMD_MAPC)} pub const fn GITS_CMD_VMAPTI()->u8{GITS_CMD_GICv4(GITS_CMD_MAPTI)} pub const fn GITS_CMD_VMOVI()->u8{GITS_CMD_GICv4(GITS_CMD_MOVI)} pub const fn GITS_CMD_VSYNC()->u8{GITS_CMD_GICv4(GITS_CMD_SYNC)}

pub const GITS_SGIR_VPEID:u64=0xffff_0000_0000; pub const GITS_SGIR_VINTID:u64=0xf;
pub const GITS_TYPER_PLPIS:u64=1; pub const GITS_TYPER_VLPIS:u64=2; pub const GITS_TYPER_PTA:u64=1<<19; pub const GITS_TYPER_VMOVP:u64=1<<37; pub const GITS_TYPER_VMAPP:u64=1<<40;
pub const GITS_CBASER_VALID:u64=1<<63; pub const GITS_BASER_PAGE_SIZE_SHIFT:u32=8; pub const GITS_BASER_PAGES_MAX:u32=256; pub const GITS_BASER_PAGES_SHIFT:u32=0;
pub const ICC_CTLR_EL1_EOImode_SHIFT:u32=1; pub const ICC_CTLR_EL1_EOImode_drop_dir:u32=0; pub const ICC_CTLR_EL1_EOImode_drop:u32=1<<1;
pub const ICC_CTLR_EL1_CBPR_MASK:u32=1; pub const ICC_CTLR_EL1_PMHE_MASK:u32=1<<6; pub const ICC_CTLR_EL1_PRI_BITS_MASK:u32=7<<8; pub const ICC_CTLR_EL1_ID_BITS_MASK:u32=7<<11; pub const ICC_CTLR_EL1_RSS:u32=1<<18; pub const ICC_CTLR_EL1_ExtRange:u32=1<<19;
pub const ICC_SGI1R_TARGET_LIST_MASK:u64=0xffff; pub const ICC_SGI1R_AFFINITY_1_MASK:u64=0xff<<16; pub const ICC_SGI1R_SGI_ID_MASK:u64=0xf<<24; pub const ICC_SGI1R_AFFINITY_2_MASK:u64=0xff<<32; pub const ICC_SGI1R_IRQ_ROUTING_MODE_BIT:u32=40; pub const ICC_SGI1R_RS_MASK:u64=0xf<<44; pub const ICC_SGI1R_AFFINITY_3_MASK:u64=0xff<<48;
pub const GICH_LR_VIRTUALID:u64=0x3ff; pub const GICH_LR_PHYSID_CPUID_SHIFT:u32=10; pub const GICH_LR_PHYSID_CPUID:u64=7<<10;

pub const fn GIC_ENCODE_SZ(n:u64,w:u32)->u64{(n.wrapping_sub(1)) & ((1u64<<w)-1)}
pub const fn GITS_CBASER_ADDRESS(x:u64)->u64{x & 0x000f_ffff_ffff_f000}
pub const fn GITS_BASER_ENTRY_SIZE_MASK()->u64{0x1f<<48}
pub const fn GITS_BASER_PHYS_52_to_48(p:u64)->u64{(p&0x0000_ffff_ffff_0000)|(((p>>48)&0xf)<<12)}
pub const fn GITS_BASER_ADDR_48_to_52(p:u64)->u64{(p&0x0000_ffff_ffff_0000)|(((p>>12)&0xf)<<48)}

#[repr(C)]
pub struct rdists {
    pub rdist: *mut core::ffi::c_void,
    pub prop_table_pa: u64,
    pub prop_table_va: *mut core::ffi::c_void,
    pub flags: u64,
    pub gicd_typer: u32,
    pub gicd_typer2: u32,
    pub cpuhp_memreserve_state: i32,
    pub has_vlpis: bool,
    pub has_rvpeid: bool,
    pub has_direct_lpi: bool,
    pub has_vpend_valid_dirty: bool,
}

pub struct irq_domain; pub struct fwnode_handle;
extern "C" { pub fn its_lpi_memreserve_init() -> i32; pub fn its_cpu_init() -> i32; pub fn its_init(handle:*mut fwnode_handle, rdists:*mut rdists, domain:*mut irq_domain, irq_prio:u8)->i32; pub fn mbi_init(fwnode:*mut fwnode_handle,parent:*mut irq_domain)->i32; }

extern "C" { fn gic_read_sre()->u32; fn gic_write_sre(val:u32); }
#[inline] pub unsafe fn gic_enable_sre()->bool { let mut val=gic_read_sre(); if val&ICC_SRE_EL1_SRE!=0{return true;} val|=ICC_SRE_EL1_SRE; gic_write_sre(val); val=gic_read_sre(); val&ICC_SRE_EL1_SRE!=0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
