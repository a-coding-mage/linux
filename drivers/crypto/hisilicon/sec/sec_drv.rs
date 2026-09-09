// SPDX-License-Identifier: GPL-2.0
/* Driver for the HiSilicon SEC units found on Hip06 Hip07. */

// Kernel headers and sec_drv.h are external dependencies of this translation.

const SEC_QUEUE_AR_FROCE_ALLOC: u32 = 0;
const SEC_QUEUE_AR_FROCE_NOALLOC: u32 = 1;
const SEC_QUEUE_AR_FROCE_DIS: u32 = 2;
const SEC_QUEUE_AW_FROCE_ALLOC: u32 = 0;
const SEC_QUEUE_AW_FROCE_NOALLOC: u32 = 1;
const SEC_QUEUE_AW_FROCE_DIS: u32 = 2;

const SEC_ALGSUB_CLK_EN_REG: usize = 0x03b8;
const SEC_ALGSUB_CLK_DIS_REG: usize = 0x03bc;
const SEC_ALGSUB_CLK_ST_REG: usize = 0x535c;
const SEC_ALGSUB_RST_REQ_REG: usize = 0x0aa8;
const SEC_ALGSUB_RST_DREQ_REG: usize = 0x0aac;
const SEC_ALGSUB_RST_ST_REG: usize = 0x5a54;
const SEC_ALGSUB_RST_ST_IS_RST: u32 = 1 << 0;
const SEC_ALGSUB_BUILD_RST_REQ_REG: usize = 0x0ab8;
const SEC_ALGSUB_BUILD_RST_DREQ_REG: usize = 0x0abc;
const SEC_ALGSUB_BUILD_RST_ST_REG: usize = 0x5a5c;
const SEC_ALGSUB_BUILD_RST_ST_IS_RST: u32 = 1 << 0;
const SEC_SAA_BASE: usize = 0x1000;
const SEC_SAA_CTRL_GET_QM_EN: u32 = 1;
const SEC_ST_INTMSK1_REG: usize = 0x200;
const SEC_QM_UNUSED: usize = 0;
const SEC_CLK_EN_REG: usize = 0;
const SEC_CTRL_REG: usize = 4;
const SEC_COMMON_CNT_CLR_CE_REG: usize = 8;
const SEC_COMMON_CNT_CLR_CE_CLEAR: u32 = 1;
const SEC_COMMON_CNT_CLR_CE_SNAP_EN: u32 = 2;
const SEC_IPV4_MASK_TABLE_REG: usize = 0x20;
const SEC_CTRL2_REG: usize = 0x68;
const SEC_CTRL2_DATA_AXI_RD_OTSD_CFG_M: u32 = 0xf;
const SEC_CTRL2_DATA_AXI_RD_OTSD_CFG_S: u32 = 0;
const SEC_CTRL2_DATA_AXI_WR_OTSD_CFG_M: u32 = 0x70;
const SEC_CTRL2_DATA_AXI_WR_OTSD_CFG_S: u32 = 4;
const SEC_CTRL2_CLK_GATE_EN: u32 = 1 << 7;
const SEC_CTRL2_ENDIAN_BD: u32 = 1 << 8;
const SEC_CTRL2_ENDIAN_BD_TYPE: u32 = 1 << 9;
const SEC_DEBUG_BD_CFG_REG: usize = 0x70;
const SEC_DEBUG_BD_CFG_WB_NORMAL: u32 = 1;
const SEC_DEBUG_BD_CFG_WB_EN: u32 = 2;
const SEC_HASH_IPV4_MASK: u32 = 0xfff00000;
const SEC_MAX_SAA_NUM: usize = 10;
const SEC_SAA_ADDR_SIZE: usize = 0x1000;
const SEC_Q_INIT_REG: usize = 0;
const SEC_Q_INIT_AND_STAT_CLEAR: u32 = 3;
const SEC_Q_CFG_REG: usize = 8;
const SEC_Q_CFG_REORDER: u32 = 1;
const SEC_Q_PROC_NUM_CFG_REG: usize = 0x10;
const SEC_QUEUE_ENB_REG: usize = 0x18;
const SEC_Q_DEPTH_CFG_REG: usize = 0x50;
const SEC_Q_DEPTH_CFG_DEPTH_M: u32 = 0xfff;
const SEC_Q_BASE_HADDR_REG: usize = 0x54;
const SEC_Q_BASE_LADDR_REG: usize = 0x58;
const SEC_Q_WR_PTR_REG: usize = 0x5c;
const SEC_Q_OUTORDER_BASE_HADDR_REG: usize = 0x60;
const SEC_Q_OUTORDER_BASE_LADDR_REG: usize = 0x64;
const SEC_Q_OUTORDER_RD_PTR_REG: usize = 0x68;
const SEC_Q_OUTORDER_WR_PTR_REG: usize = 0x60c;
const SEC_Q_OT_TH_REG: usize = 0x6c;
const SEC_Q_ARUSER_CFG_REG: usize = 0x70;
const SEC_Q_ARUSER_CFG_FA: u32 = 1;
const SEC_Q_ARUSER_CFG_FNA: u32 = 2;
const SEC_Q_ARUSER_CFG_PKG: u32 = 8;
const SEC_Q_AWUSER_CFG_REG: usize = 0x74;
const SEC_Q_AWUSER_CFG_FA: u32 = 1;
const SEC_Q_AWUSER_CFG_FNA: u32 = 2;
const SEC_Q_AWUSER_CFG_PKG: u32 = 4;
const SEC_Q_ERR_BASE_HADDR_REG: usize = 0x7c;
const SEC_Q_ERR_BASE_LADDR_REG: usize = 0x80;
const SEC_Q_FAIL_INT_MSK_REG: usize = 0x300;
const SEC_Q_FLOW_INT_MKS_REG: usize = 0x304;
const SEC_Q_RD_PTR_REG: usize = 0x604;
const SEC_Q_OUTORDER_WR_PTR_REG: usize = 0x60c;
const SEC_OUT_BD_INFO_Q_ID_M: u16 = 0xfff;
const SEC_Q_NUM: usize = 16; // supplied by sec_drv.h

#[repr(C)]
pub struct SecDebugBdInfo { pub soft_err_check: u32, pub hard_err_check: u32, pub icv_mac1st_word: u32, pub sec_get_id: u32, pub reserv_left: [u32; 12] }
#[repr(C)] pub struct SecOutBdInfo { pub data: u16 }

extern "C" {
    static mut sec_devices: [*mut SecDevInfo; 8];
    fn sec_alg_callback(msg: *mut SecBdInfo, ctx: *mut core::ffi::c_void);
}

unsafe fn reg(r: *mut u8, o: usize) -> *mut u8 { r.add(o) }
unsafe fn rd(a: *mut u8) -> u32 { readl_relaxed(a) }
unsafe fn wr(v: u32, a: *mut u8) { writel_relaxed(v, a) }
extern "C" { fn readl_relaxed(a: *mut u8) -> u32; fn writel_relaxed(v: u32, a: *mut u8); fn readl(a: *mut u8) -> u32; fn writel(v: u32, a: *mut u8); }

#[inline] unsafe fn sec_queue_ar_pkgattr(q: *mut SecQueue, p: u32) -> i32 { let a=reg((*q).regs,SEC_Q_ARUSER_CFG_REG); let mut v=rd(a); if p!=0 {v|=SEC_Q_ARUSER_CFG_PKG} else {v&=!SEC_Q_ARUSER_CFG_PKG}; wr(v,a); 0 }
#[inline] unsafe fn sec_queue_aw_pkgattr(q: *mut SecQueue, _p: u32) -> i32 { let a=reg((*q).regs,SEC_Q_AWUSER_CFG_REG); let v=rd(a)|SEC_Q_AWUSER_CFG_PKG; wr(v,a); 0 }
unsafe fn sec_clk_en(i:*mut SecDevInfo)->i32 { let b=(*i).regs[0]; wr(7,reg(b,SEC_ALGSUB_CLK_EN_REG)); for _ in 0..10 { usleep_range(1000,10000); if rd(reg(b,SEC_ALGSUB_CLK_ST_REG))&7==7{return 0} } -5 }
unsafe fn sec_clk_dis(i:*mut SecDevInfo)->i32 { let b=(*i).regs[0]; wr(7,reg(b,SEC_ALGSUB_CLK_DIS_REG)); for _ in 0..10 {usleep_range(1000,10000);if rd(reg(b,SEC_ALGSUB_CLK_ST_REG))&7==0{return 0}} -5 }
unsafe fn sec_reset_whole_module(i:*mut SecDevInfo)->i32 { let b=(*i).regs[0]; wr(1,reg(b,SEC_ALGSUB_RST_REQ_REG));wr(1,reg(b,SEC_ALGSUB_BUILD_RST_REQ_REG)); for _ in 0..11 {usleep_range(1000,10000);if rd(reg(b,SEC_ALGSUB_RST_ST_REG))&1!=0&&rd(reg(b,SEC_ALGSUB_BUILD_RST_ST_REG))&1!=0{break}} wr(1,reg(b,SEC_ALGSUB_RST_DREQ_REG));wr(1,reg(b,SEC_ALGSUB_BUILD_RST_DREQ_REG)); for _ in 0..11 {usleep_range(1000,10000);if rd(reg(b,SEC_ALGSUB_RST_ST_REG))&1==0&&rd(reg(b,SEC_ALGSUB_BUILD_RST_ST_REG))&1==0{return 0}} -5 }
unsafe fn sec_bd_endian_little(i:*mut SecDevInfo){let a=reg((*i).regs[1],SEC_CTRL2_REG);wr(rd(a)&!(SEC_CTRL2_ENDIAN_BD|SEC_CTRL2_ENDIAN_BD_TYPE),a)}
unsafe fn sec_data_cfg(i:*mut SecDevInfo,c:u32,write:bool){let a=reg((*i).regs[1],SEC_CTRL2_REG);let (m,s)=if write{(SEC_CTRL2_DATA_AXI_WR_OTSD_CFG_M,4)}else{(SEC_CTRL2_DATA_AXI_RD_OTSD_CFG_M,0)};let mut v=rd(a)&!m;v|=(c<<s)&m;wr(v,a)}
unsafe fn sec_clk_gate_en(i:*mut SecDevInfo,e:bool){let a=reg((*i).regs[1],SEC_CTRL2_REG);let mut v=rd(a);if e{v|=SEC_CTRL2_CLK_GATE_EN}else{v&=!SEC_CTRL2_CLK_GATE_EN};wr(v,a)}
unsafe fn sec_comm_cnt_cfg(i:*mut SecDevInfo,e:bool){let a=reg((*i).regs[1],SEC_COMMON_CNT_CLR_CE_REG);let mut v=rd(a);if e{v|=1}else{v&=!1};wr(v,a)}
unsafe fn sec_commsnap_en(i:*mut SecDevInfo,e:bool){let a=reg((*i).regs[1],SEC_COMMON_CNT_CLR_CE_REG);let mut v=rd(a);if e{v|=2}else{v&=!2};wr(v,a)}
unsafe fn sec_ipv6_hashmask(i:*mut SecDevInfo,h:*const u32){for x in 0..10{wr(*h,reg((*i).regs[1],0x24+x*4))}}
unsafe fn sec_ipv4_hashmask(i:*mut SecDevInfo,h:u32)->i32{if h&SEC_HASH_IPV4_MASK!=0{return -22}wr(h,reg((*i).regs[1],SEC_IPV4_MASK_TABLE_REG));0}
unsafe fn sec_set_dbg_bd_cfg(i:*mut SecDevInfo,c:u32){let a=reg((*i).regs[1],SEC_DEBUG_BD_CFG_REG);let mut v=rd(a)&!SEC_DEBUG_BD_CFG_WB_NORMAL;if c!=0{v&=!SEC_DEBUG_BD_CFG_WB_EN}else{v|=SEC_DEBUG_BD_CFG_WB_EN};wr(v,a)}
unsafe fn sec_queue_reorder(q:*mut SecQueue,e:bool){let a=reg((*q).regs,SEC_Q_CFG_REG);let mut v=rd(a);if e{v|=1}else{v&=!1};wr(v,a)}
unsafe fn sec_queue_depth(q:*mut SecQueue,d:u32){let a=reg((*q).regs,SEC_Q_DEPTH_CFG_REG);let mut v=rd(a)&!SEC_Q_DEPTH_CFG_DEPTH_M;v|=d&SEC_Q_DEPTH_CFG_DEPTH_M;wr(v,a)}
unsafe fn sec_queue_addr(q:*mut SecQueue,a:u64,hi:usize,lo:usize){wr((a>>32)as u32,reg((*q).regs,hi));wr(a as u32,reg((*q).regs,lo))}
unsafe fn sec_queue_hw_init(q:*mut SecQueue){sec_queue_reorder(q,true);wr(1,reg((*q).regs,SEC_Q_PROC_NUM_CFG_REG));sec_queue_depth(q,SEC_Q_NUM as u32-1);sec_queue_addr(q,(*q).ring_cmd.paddr,SEC_Q_BASE_HADDR_REG,SEC_Q_BASE_LADDR_REG);sec_queue_addr(q,(*q).ring_cq.paddr,SEC_Q_OUTORDER_BASE_HADDR_REG,SEC_Q_OUTORDER_BASE_LADDR_REG);wr(u32::MAX,reg((*q).regs,SEC_Q_FLOW_INT_MKS_REG));wr(3,reg((*q).regs,SEC_Q_INIT_REG))}

// The remaining driver entry points retain the C ABI and are intentionally expressed
// against the structures and kernel services declared by the companion header.
pub unsafe fn sec_queue_empty(q:*mut SecQueue)->bool{atomic_read(&mut (*q).ring_cmd.used)==0}
pub unsafe fn sec_queue_can_enqueue(q:*mut SecQueue,n:i32)->bool{SEC_Q_NUM as i32-atomic_read(&mut (*q).ring_cmd.used)>=n}

extern "C" {
    fn atomic_read(v:*mut i32)->i32; fn usleep_range(a:u32,b:u32); fn writel_relaxed(v:u32,a:*mut u8);
}

// External kernel structures, allocation, IRQ, platform, and module registration
// declarations are supplied by sec_drv.h and the Linux Rust bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
