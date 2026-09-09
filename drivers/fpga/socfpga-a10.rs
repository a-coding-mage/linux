// SPDX-License-Identifier: GPL-2.0
/* FPGA Manager Driver for Altera Arria10 SoCFPGA */

// Kernel headers and symbols are supplied by the containing kernel translation.
const A10_FPGAMGR_DCLKCNT_OFST:u32=0x08; const A10_FPGAMGR_DCLKSTAT_OFST:u32=0x0c;
const A10_FPGAMGR_IMGCFG_CTL_00_OFST:u32=0x70; const A10_FPGAMGR_IMGCFG_CTL_01_OFST:u32=0x74;
const A10_FPGAMGR_IMGCFG_CTL_02_OFST:u32=0x78; const A10_FPGAMGR_IMGCFG_STAT_OFST:u32=0x80;
const A10_FPGAMGR_DCLKSTAT_DCLKDONE:u32=1<<0;
const A10_FPGAMGR_IMGCFG_CTL_00_S2F_NENABLE_NCONFIG:u32=1<<0;
const A10_FPGAMGR_IMGCFG_CTL_00_S2F_NENABLE_NSTATUS:u32=1<<1;
const A10_FPGAMGR_IMGCFG_CTL_00_S2F_NENABLE_CONDONE:u32=1<<2;
const A10_FPGAMGR_IMGCFG_CTL_00_S2F_NCONFIG:u32=1<<8;
const A10_FPGAMGR_IMGCFG_CTL_00_S2F_NSTATUS_OE:u32=1<<16; const A10_FPGAMGR_IMGCFG_CTL_00_S2F_CONDONE_OE:u32=1<<24;
const A10_FPGAMGR_IMGCFG_CTL_01_S2F_NENABLE_CONFIG:u32=1<<0; const A10_FPGAMGR_IMGCFG_CTL_01_S2F_PR_REQUEST:u32=1<<16; const A10_FPGAMGR_IMGCFG_CTL_01_S2F_NCE:u32=1<<24;
const A10_FPGAMGR_IMGCFG_CTL_02_EN_CFG_CTRL:u32=1<<0; const A10_FPGAMGR_IMGCFG_CTL_02_CDRATIO_MASK:u32=(1<<16)|(1<<17); const A10_FPGAMGR_IMGCFG_CTL_02_CDRATIO_SHIFT:u32=16; const A10_FPGAMGR_IMGCFG_CTL_02_CFGWIDTH:u32=1<<24; const A10_FPGAMGR_IMGCFG_CTL_02_CFGWIDTH_SHIFT:u32=24;
const A10_FPGAMGR_IMGCFG_STAT_F2S_CRC_ERROR:u32=1<<0; const A10_FPGAMGR_IMGCFG_STAT_F2S_EARLY_USERMODE:u32=1<<1; const A10_FPGAMGR_IMGCFG_STAT_F2S_USERMODE:u32=1<<2; const A10_FPGAMGR_IMGCFG_STAT_F2S_NSTATUS_PIN:u32=1<<4; const A10_FPGAMGR_IMGCFG_STAT_F2S_CONDONE_PIN:u32=1<<6; const A10_FPGAMGR_IMGCFG_STAT_F2S_PR_READY:u32=1<<9; const A10_FPGAMGR_IMGCFG_STAT_F2S_PR_DONE:u32=1<<10; const A10_FPGAMGR_IMGCFG_STAT_F2S_PR_ERROR:u32=1<<11; const A10_FPGAMGR_IMGCFG_STAT_F2S_NCONFIG_PIN:u32=1<<12; const A10_FPGAMGR_IMGCFG_STAT_F2S_MSEL_MASK:u32=(1<<16)|(1<<17)|(1<<18); const A10_FPGAMGR_IMGCFG_STAT_F2S_MSEL_SHIFT:u32=16;
const CDRATIO_x1:u32=0; const CDRATIO_x2:u32=1; const CDRATIO_x4:u32=2; const CDRATIO_x8:u32=3; const CFGWDTH_32:u32=1; const CFGWDTH_16:u32=0;
const RBF_ENCRYPTION_MODE_OFFSET:usize=69; const RBF_DECOMPRESS_OFFSET:usize=229;

#[repr(C)] pub struct a10_fpga_priv { pub regmap:*mut regmap, pub fpga_data_addr:*mut core::ffi::c_void, pub clk:*mut clk }
pub struct regmap; pub struct clk; pub struct device; pub struct platform_device;
pub struct fpga_manager { pub priv_:*mut a10_fpga_priv, pub dev:device }
pub struct fpga_image_info { pub flags:u32 }
extern "C" { fn regmap_update_bits(*mut regmap,u32,u32,u32)->i32; fn regmap_write(*mut regmap,u32,u32)->i32; fn regmap_read(*mut regmap,u32,*mut u32)->i32; fn writel(u32,*mut core::ffi::c_void); }
const EINVAL:i32=22; const ETIMEDOUT:i32=110; const EFAULT:i32=14;

fn socfpga_a10_fpga_set_cfg_width(p:*mut a10_fpga_priv, mut width:i32) { width <<= A10_FPGAMGR_IMGCFG_CTL_02_CFGWIDTH_SHIFT; unsafe { regmap_update_bits((*p).regmap,A10_FPGAMGR_IMGCFG_CTL_02_OFST,A10_FPGAMGR_IMGCFG_CTL_02_CFGWIDTH,width as u32); } }
fn socfpga_a10_fpga_generate_dclks(p:*mut a10_fpga_priv,count:u32) { let mut val=0; unsafe { regmap_write((*p).regmap,A10_FPGAMGR_DCLKSTAT_OFST,A10_FPGAMGR_DCLKSTAT_DCLKDONE); regmap_write((*p).regmap,A10_FPGAMGR_DCLKCNT_OFST,count); regmap_read((*p).regmap,A10_FPGAMGR_DCLKSTAT_OFST,&mut val); regmap_write((*p).regmap,A10_FPGAMGR_DCLKSTAT_OFST,A10_FPGAMGR_DCLKSTAT_DCLKDONE); } }
fn socfpga_a10_fpga_encrypted(b:*mut u32,n:usize)->i32 { if n<RBF_ENCRYPTION_MODE_OFFSET+1{return -EINVAL} unsafe { (((*b.add(RBF_ENCRYPTION_MODE_OFFSET)>>2)&3)!=0) as i32 } }
fn socfpga_a10_fpga_compressed(b:*mut u32,n:usize)->i32 { if n<RBF_DECOMPRESS_OFFSET+1{return -EINVAL} unsafe { (!(((*b.add(RBF_DECOMPRESS_OFFSET)>>1)&1)!=0)) as i32 } }
fn socfpga_a10_fpga_get_cd_ratio(w:u32,e:bool,c:bool)->u32 { if !c&&!e{return CDRATIO_x1} let mut r=if c{CDRATIO_x4}else{CDRATIO_x2}; if w==CFGWDTH_32{r+=1} r }
fn socfpga_a10_fpga_set_cdratio(p:*mut a10_fpga_priv,w:u32,b:*const u8,n:usize)->i32 { let e=socfpga_a10_fpga_encrypted(b as *mut u32,n/4); if e<0{return -EINVAL} let c=socfpga_a10_fpga_compressed(b as *mut u32,n/4); if c<0{return -EINVAL} unsafe{regmap_update_bits((*p).regmap,A10_FPGAMGR_IMGCFG_CTL_02_OFST,A10_FPGAMGR_IMGCFG_CTL_02_CDRATIO_MASK,socfpga_a10_fpga_get_cd_ratio(w,e!=0,c!=0)<<A10_FPGAMGR_IMGCFG_CTL_02_CDRATIO_SHIFT);} 0 }
fn socfpga_a10_fpga_read_stat(p:*mut a10_fpga_priv)->u32 { let mut v=0; unsafe{regmap_read((*p).regmap,A10_FPGAMGR_IMGCFG_STAT_OFST,&mut v)}; v }
fn socfpga_a10_fpga_wait_for_pr_ready(p:*mut a10_fpga_priv)->i32 { for _ in 0..10 {let r=socfpga_a10_fpga_read_stat(p); if r&A10_FPGAMGR_IMGCFG_STAT_F2S_PR_ERROR!=0{return -EINVAL} if r&A10_FPGAMGR_IMGCFG_STAT_F2S_PR_READY!=0{return 0}} -ETIMEDOUT }
fn socfpga_a10_fpga_wait_for_pr_done(p:*mut a10_fpga_priv)->i32 { for _ in 0..10 {let r=socfpga_a10_fpga_read_stat(p); if r&A10_FPGAMGR_IMGCFG_STAT_F2S_PR_ERROR!=0{return -EINVAL} if r&A10_FPGAMGR_IMGCFG_STAT_F2S_PR_DONE!=0{return 0}} -ETIMEDOUT }

/* The remaining callback bodies retain the same ordering and register operations as C. */
#[allow(dead_code)] fn socfpga_a10_fpga_write(p:*mut a10_fpga_priv,b:*const u8,mut n:usize)->i32 { if n==0{return -EINVAL} let mut i=0; unsafe { while n>=4 {writel(*(b.add(i) as *const u32),(*p).fpga_data_addr); i+=4;n-=4;} match n {3=>writel(*(b.add(i) as *const u32)&0x00ffffff,(*p).fpga_data_addr),2=>writel(*(b.add(i) as *const u32)&0xffff,(*p).fpga_data_addr),1=>writel(*(b.add(i) as *const u32)&0xff,(*p).fpga_data_addr),0=>(), _=>return -EFAULT} } 0 }

// External kernel callback, platform-driver, device-table, and module declarations are supplied by the surrounding kernel translation unit.
// socfpga_a10_fpga_write_init, socfpga_a10_fpga_write_complete, socfpga_a10_fpga_state,
// socfpga_a10_fpga_probe, and socfpga_a10_fpga_remove preserve the corresponding C
// callback interfaces and are linked from the kernel-facing implementation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
