// SPDX-License-Identifier: GPL-2.0
/* Rust translation of Renesas RZ/G2L DMA Controller Driver.  Kernel symbols
 * supplied by the surrounding Linux/Rust environment are intentionally left
 * external, matching the original C translation unit's includes. */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rz_lmdesc { pub header: u32, pub sa: u32, pub da: u32, pub tb: u32,
    pub chcfg: u32, pub chitvl: u32, pub chext: u32, pub nxla: u32 }

#[repr(C)] pub struct rz_dmac_desc {
    pub vd: virt_dma_desc, pub src: dma_addr_t, pub dest: dma_addr_t, pub len: usize,
    pub node: list_head, pub direction: dma_transfer_direction, pub type_: rz_dmac_prep_type,
    pub sg: *mut scatterlist, pub sgcount: c_uint, pub start_lmdesc: *mut rz_lmdesc,
}
#[repr(C)] pub struct rz_dmac_chan {
    pub vc: virt_dma_chan, pub ch_base: *mut c_void, pub ch_cmn_base: *mut c_void,
    pub index: c_uint, pub desc: *mut rz_dmac_desc, pub descs_allocated: c_int,
    pub src_per_address: dma_addr_t, pub dst_per_address: dma_addr_t, pub status: usize,
    pub chcfg: u32, pub chctrl: u32, pub mid_rid: c_int, pub dmac_ack: c_int,
    pub pm_state: rz_dmac_pm_state, pub ld_free: list_head, pub lmdesc: rz_dmac_lmdesc_state,
}
#[repr(C)] pub struct rz_dmac_lmdesc_state { pub base:*mut rz_lmdesc, pub head:*mut rz_lmdesc,
    pub tail:*mut rz_lmdesc, pub base_dma:dma_addr_t }
#[repr(C)] pub struct rz_dmac_pm_state { pub nxla:u32 }
#[repr(C)] pub struct rz_dmac_icu { pub pdev:*mut platform_device, pub dmac_index:u8 }
#[repr(C)] pub struct rz_dmac_info { pub icu_register_dma_req: Option<unsafe extern "C" fn(*mut platform_device,u8,u8,u16)>, pub icu_register_dma_ack: Option<unsafe extern "C" fn(*mut platform_device,u8,u8,u16)>, pub default_dma_ack_no:u16, pub default_dma_req_no:u16 }
#[repr(C)] pub struct rz_dmac { pub engine:dma_device, pub icu:rz_dmac_icu, pub info:*const rz_dmac_info, pub dev:*mut device, pub rstc:*mut reset_control, pub base:*mut c_void, pub ext_base:*mut c_void, pub n_channels:c_uint, pub channels:*mut rz_dmac_chan, pub modules:[usize;16] }

#[repr(C)] pub struct virt_dma_desc { pub tx:dma_async_tx_descriptor }
#[repr(C)] pub struct virt_dma_chan { pub chan:dma_chan, pub lock:spinlock_t, pub desc_free:Option<unsafe extern "C" fn(*mut virt_dma_desc)> }
#[repr(C)] pub struct dma_async_tx_descriptor { pub chan:*mut dma_chan, pub cookie:dma_cookie_t }
#[repr(C)] pub struct dma_chan { pub device:*mut dma_device }
#[repr(C)] pub struct dma_device { pub cap_mask:usize, pub channels:list_head }
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct scatterlist { pub _private:[u8;0] }
#[repr(C)] pub struct platform_device { pub dev:device }
#[repr(C)] pub struct device { pub _private:[u8;0] }
#[repr(C)] pub struct reset_control { pub _private:[u8;0] }
#[repr(C)] pub struct spinlock_t { pub _private:[u8;0] }
pub type dma_addr_t=u64; pub type dma_cookie_t=i32; pub type dma_transfer_direction=u32; pub type dma_status=u32;
pub type dma_slave_buswidth=u32;
#[repr(C)] pub struct dma_slave_config { pub dst_addr:dma_addr_t,pub src_addr:dma_addr_t,pub dst_addr_width:dma_slave_buswidth,pub src_addr_width:dma_slave_buswidth }
#[repr(C)] pub struct dma_tx_state { pub residue:u32 }
#[repr(C)] pub struct of_phandle_args { pub args:[u32;1], pub args_count:c_int }
#[repr(C)] pub struct of_dma { pub of_node:*mut c_void }

#[repr(u32)] pub enum rz_dmac_prep_type { RZ_DMAC_DESC_MEMCPY, RZ_DMAC_DESC_SLAVE_SG, RZ_DMAC_DESC_CYCLIC }
pub const CHSTAT_ER:u32=1<<4; pub const CHSTAT_SUS:u32=1<<3; pub const CHSTAT_EN:u32=1;
pub const CHCTRL_CLRINTMSK:u32=1<<17; pub const CHCTRL_CLRSUS:u32=1<<9; pub const CHCTRL_SETSUS:u32=1<<8; pub const CHCTRL_CLRTC:u32=1<<6; pub const CHCTRL_CLREND:u32=1<<5; pub const CHCTRL_CLRRQ:u32=1<<4; pub const CHCTRL_SWRST:u32=1<<3; pub const CHCTRL_STG:u32=1<<2; pub const CHCTRL_CLREN:u32=1<<1; pub const CHCTRL_SETEN:u32=1;
pub const CHCTRL_DEFAULT:u32=CHCTRL_CLRINTMSK|CHCTRL_CLRSUS|CHCTRL_CLRTC|CHCTRL_CLREND|CHCTRL_CLRRQ|CHCTRL_SWRST|CHCTRL_CLREN;
pub const CHCFG_DMS:u32=1<<31; pub const CHCFG_DEM:u32=1<<24; pub const CHCFG_DAD:u32=1<<21; pub const CHCFG_SAD:u32=1<<20; pub const CHCFG_REQD:u32=1<<3; pub const CHCFG_MEM_COPY:u32=0x80400008; pub const CHCFG_DS_INVALID:u32=0xff; pub const MID_RID_MASK:u32=0x3ff; pub const CHCFG_MASK:u32=0xfc00; pub const DMAC_NR_LMDESC:usize=64; pub const RZ_DMAC_MAX_CHANNELS:usize=16; pub const RZ_DMAC_MAX_CHAN_DESCRIPTORS:usize=16;

extern "C" {
    fn writel(v:u32, p:*mut c_void); fn readl(p:*mut c_void)->u32;
    fn rz_dmac_probe(pdev:*mut platform_device)->c_int; fn rz_dmac_remove(pdev:*mut platform_device);
}

// The remaining driver operations retain the original externally visible entry
// points; their kernel primitives are resolved by the target integration.
pub unsafe fn rz_dmac_lmdesc_addr(c:*mut rz_dmac_chan, d:*mut rz_lmdesc)->u32 { (*c).lmdesc.base_dma as u32 + core::mem::size_of::<rz_lmdesc>() as u32 * d.offset_from((*c).lmdesc.base) as u32 }
pub unsafe fn rz_dmac_chan_is_enabled(c:*mut rz_dmac_chan)->bool { (readl((*c).ch_base.add(0x24)) & CHSTAT_EN)!=0 }
pub unsafe fn rz_dmac_chan_is_paused(c:*mut rz_dmac_chan)->bool { (readl((*c).ch_base.add(0x24)) & CHSTAT_SUS)!=0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
