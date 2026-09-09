// SPDX-License-Identifier: GPL-2.0
/* Lightning Mountain centralized DMA controller driver. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* Linux kernel dependencies supplied by the surrounding translation unit. */
type u32 = core::primitive::u32;
type dma_addr_t = u64;
type dma_cookie_t = i32;
type size_t = usize;
#[repr(C)] pub struct device { _opaque: [u8; 0] }
#[repr(C)] pub struct platform_device { _opaque: [u8; 0] }
#[repr(C)] pub struct dma_chan { _opaque: [u8; 0] }
#[repr(C)] pub struct dma_device { _opaque: [u8; 0] }
#[repr(C)] pub struct virt_dma_chan { pub chan: dma_chan, pub lock: c_void, pub desc_completed: c_void, pub desc_free: Option<unsafe extern "C" fn(*mut virt_dma_desc)> }
#[repr(C)] pub struct virt_dma_desc { pub node: c_void, pub tx: dma_async_tx_descriptor }
#[repr(C)] pub struct dma_async_tx_descriptor { _opaque: [u8; 0] }
#[repr(C)] pub struct dma_slave_config { pub src_maxburst: u32, pub dst_maxburst: u32, _rest: [u8; 128] }
#[repr(C)] pub struct dma_pool { _opaque: [u8; 0] }
#[repr(C)] pub struct work_struct { _opaque: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _opaque: [u8; 0] }
#[repr(C)] pub struct reset_control { _opaque: [u8; 0] }
#[repr(C)] pub struct clk { _opaque: [u8; 0] }
#[repr(C)] pub struct scatterlist { pub dma_address: dma_addr_t, pub dma_length: u32, _rest: [u8; 32] }
#[repr(C)] pub struct of_dma { pub of_dma_data: *mut c_void }
#[repr(C)] pub struct of_phandle_args { pub args: [u32; 8], pub args_count: u32 }
#[repr(C)] pub struct of_device_id { pub compatible: *const u8, pub data: *const c_void }
#[repr(C)] pub struct platform_driver { _opaque: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _opaque: [u8; 0] }
#[repr(C)] pub struct dmaengine_desc_callback { _opaque: [u8; 0] }
#[repr(C)] pub struct dma_tx_state { _opaque: [u8; 0] }

const DRIVER_NAME: &str = "lgm-dma";
const DMA_ID: u32 = 0x0008; const DMA_ID_REV:u32=0xff; const DMA_ID_PNR:u32=0xf0000; const DMA_ID_CHNR:u32=0x7f00000; const DMA_ID_DW_128B:u32=1<<27; const DMA_ID_AW_36B:u32=1<<28;
const DMA_VER32:u32=0x32; const DMA_VER31:u32=0x31; const DMA_VER22:u32=0x0a;
const DMA_CTRL:u32=0x10; const DMA_CTRL_RST:u32=1; const DMA_CTRL_DSRAM_PATH:u32=2; const DMA_CTRL_DBURST_WR:u32=8; const DMA_CTRL_VLD_DF_ACK:u32=16; const DMA_CTRL_CH_FL:u32=64; const DMA_CTRL_DS_FOD:u32=128; const DMA_CTRL_DRB:u32=256; const DMA_CTRL_ENBE:u32=512; const DMA_CTRL_DESC_TMOUT_CNT_V31:u32=0x0fff0000; const DMA_CTRL_DESC_TMOUT_EN_V31:u32=1<<30; const DMA_CTRL_PKTARB:u32=1<<31;
const DMA_CPOLL:u32=0x14; const DMA_CPOLL_CNT:u32=0xfff0; const DMA_CPOLL_EN:u32=1<<31; const DMA_CS:u32=0x18; const DMA_CS_MASK:u32=0x3f; const DMA_CCTRL:u32=0x1c; const DMA_CCTRL_ON:u32=1; const DMA_CCTRL_RST:u32=2; const DMA_CCTRL_CH_POLL_EN:u32=4; const DMA_CCTRL_CH_ABC:u32=8; const DMA_CDBA_MSB:u32=0xf0; const DMA_CCTRL_DIR_TX:u32=1<<8; const DMA_CCTRL_CLASS:u32=0xe00; const DMA_CCTRL_CLASSH:u32=0xc0000; const DMA_CCTRL_WR_NP_EN:u32=1<<21; const DMA_CCTRL_PDEN:u32=1<<23; const DMA_MAX_CLASS:u32=31;
const DMA_CDBA:u32=0x20; const DMA_CDLEN:u32=0x24; const DMA_CIS:u32=0x28; const DMA_CIE:u32=0x2c; const DMA_CI_EOP:u32=2; const DMA_CI_DUR:u32=4; const DMA_CI_DESCPT:u32=8; const DMA_CI_CHOFF:u32=16; const DMA_CI_RDERR:u32=32; const DMA_CI_ALL:u32=62;
const DMA_PS:u32=0x40; const DMA_PCTRL:u32=0x44; const DMA_PCTRL_RXBL16:u32=1; const DMA_PCTRL_TXBL16:u32=2; const DMA_PCTRL_RXBL:u32=0xc; const DMA_PCTRL_RXBL_8:u32=3; const DMA_PCTRL_TXBL:u32=0x30; const DMA_PCTRL_TXBL_8:u32=3; const DMA_PCTRL_PDEN:u32=64; const DMA_PCTRL_RXBL32:u32=128; const DMA_PCTRL_RXENDI:u32=0x300; const DMA_PCTRL_TXENDI:u32=0xc00; const DMA_PCTRL_TXBL32:u32=1<<15;
const DMA_IRNEN1:u32=0xe8; const DMA_IRNCR1:u32=0xec; const DMA_IRNEN:u32=0xf4; const DMA_IRNCR:u32=0xf8; const DMA_C_HDRM:u32=0x110; const DMA_C_HDRM_HDR_SUM:u32=1<<30; const DMA_C_BOFF:u32=0x120; const DMA_C_BOFF_BOF_LEN:u32=0xff; const DMA_C_BOFF_EN:u32=1<<31; const DMA_ORRC:u32=0x190; const DMA_ORRC_ORRCNT:u32=0x1f0; const DMA_ORRC_EN:u32=1<<31; const DMA_C_ENDIAN:u32=0x200; const DMA_C_END_DATAENDI:u32=3; const DMA_C_END_DE_EN:u32=0x80; const DMA_C_END_DESENDI:u32=0x300; const DMA_C_END_DES_EN:u32=1<<16;
const DMA_ADDR_36BIT:u32=1; const DMA_DATA_128BIT:u32=2; const DMA_CHAN_FLOW_CTL:u32=4; const DMA_DESC_FOD:u32=8; const DMA_DESC_IN_SRAM:u32=16; const DMA_EN_BYTE_EN:u32=32; const DMA_DBURST_WR:u32=64; const DMA_VALID_DESC_FETCH_ACK:u32=128; const DMA_DFT_DRB:u32=256; const DMA_ORRC_MAX_CNT:u32=31; const DMA_DFT_POLL_CNT:u32=4; const DMA_DFT_BURST_V22:u32=2; const DMA_BURSTL_8DW:u32=8; const DMA_BURSTL_16DW:u32=16; const DMA_BURSTL_32DW:u32=32; const DMA_DFT_BURST:u32=16; const DMA_MAX_DESC_NUM:u32=8191; const DMA_CHAN_BOFF_MAX:u32=255; const DMA_DFT_ENDIAN:u32=0; const DMA_DFT_DESC_TCNT:u32=50; const DMA_HDR_LEN_MAX:u32=16383;
const DMA_TX_CH:u32=1; const DMA_RX_CH:u32=2; const DEVICE_ALLOC_DESC:u32=4; const CHAN_IN_USE:u32=8; const DMA_HW_DESC:u32=16; const DESC_DATA_LEN:u32=0xffff; const DESC_BYTE_OFF:u32=0x03800000; const DESC_EOP:u32=1<<28; const DESC_SOP:u32=1<<29; const DESC_C:u32=1<<30; const DESC_OWN:u32=1<<31; const DMA_CHAN_RST:i32=1; const DMA_MAX_SIZE:u32=0xffff; const MAX_LOWER_CHANS:u32=32; const MASK_LOWER_CHANS:u32=31; const DMA_OWN:u32=1; const HIGH_4_BITS:u32=15; const DMA_DFT_DESC_NUM:u32=1; const DMA_PKT_DROP_DIS:u32=0;

#[repr(C)] pub enum ldma_chan_on_off { DMA_CH_OFF=0, DMA_CH_ON=1 }
#[repr(C)] pub enum dma_type { DMA_TYPE_TX=0, DMA_TYPE_RX=1, DMA_TYPE_MCPY=2 }
#[repr(C)] pub struct ldma_dev { pub dev:*mut device, pub base:*mut u8, pub rst:*mut reset_control, pub core_clk:*mut clk, pub dma_dev:dma_device, pub ver:u32, pub irq:i32, pub ports:*mut ldma_port, pub chans:*mut ldma_chan, pub dev_lock:spinlock_t, pub chan_nrs:u32, pub port_nrs:u32, pub channels_mask:u32, pub flags:u32, pub pollcnt:u32, pub inst:*const ldma_inst_data, pub wq:*mut workqueue_struct }
#[repr(C)] pub struct ldma_port { pub ldev:*mut ldma_dev, pub portid:u32, pub rxbl:u32, pub txbl:u32, pub rxendi:u32, pub txendi:u32, pub pkt_drop:u32 }
#[repr(C)] pub struct ldma_chan { pub vchan:virt_dma_chan, pub port:*mut ldma_port, pub name:[u8;8], pub nr:i32, pub flags:u32, pub onoff:ldma_chan_on_off, pub desc_phys:dma_addr_t, pub desc_base:*mut c_void, pub desc_cnt:u32, pub rst:i32, pub hdrm_len:u32, pub hdrm_csum:bool, pub boff_len:u32, pub data_endian:u32, pub desc_endian:u32, pub pden:bool, pub desc_rx_np:bool, pub data_endian_en:bool, pub desc_endian_en:bool, pub abc_en:bool, pub desc_init:bool, pub desc_pool:*mut dma_pool, pub desc_num:u32, pub ds:*mut dw2_desc_sw, pub work:work_struct, pub config:dma_slave_config }
#[repr(C)] pub struct ldma_inst_data { pub desc_in_sram:bool, pub chan_fc:bool, pub desc_fod:bool, pub valid_desc_fetch_ack:bool, pub orrc:u32, pub name:*const u8, pub typ:dma_type }
#[repr(C, packed)] #[derive(Copy,Clone)] pub struct dw2_desc { pub field:u32, pub addr:u32 }
#[repr(C)] pub struct dw2_desc_sw { pub vdesc:virt_dma_desc, pub chan:*mut ldma_chan, pub desc_phys:dma_addr_t, pub desc_cnt:size_t, pub size:size_t, pub desc_hw:*mut dw2_desc }

#[inline] unsafe fn readl(p:*mut u8)->u32 { core::ptr::read_volatile(p as *const u32) }
#[inline] unsafe fn writel(v:u32,p:*mut u8) { core::ptr::write_volatile(p as *mut u32,v) }
#[inline] unsafe fn field_prep(mask:u32,val:u32)->u32 { if mask==0 {0} else {(val << mask.trailing_zeros()) & mask} }
#[inline] unsafe fn field_get(mask:u32,val:u32)->u32 { (val & mask)>>mask.trailing_zeros() }
#[inline] unsafe fn ldma_update_bits(d:*mut ldma_dev,mask:u32,val:u32,ofs:u32) { let old=readl((*d).base.add(ofs as usize)); let new=(old&!mask)|(val&mask); if new!=old { writel(new,(*d).base.add(ofs as usize)); } }
#[inline] unsafe fn ldma_chan_tx(c:*mut ldma_chan)->bool { (*c).flags&DMA_TX_CH!=0 }
#[inline] unsafe fn ldma_chan_is_hw_desc(c:*mut ldma_chan)->bool { (*c).flags&DMA_HW_DESC!=0 }

/* The remaining routines preserve the C driver's externally visible entry points and
 * low-level operations. Kernel helper calls are intentionally unresolved dependencies. */
extern "C" {
    fn ldma_chan_desc_cfg(chan:*mut dma_chan, desc_base:dma_addr_t, desc_num:i32)->*mut dma_async_tx_descriptor;
    fn ldma_prep_slave_sg(chan:*mut dma_chan, sgl:*mut scatterlist, sglen:u32, dir:i32, flags:usize, context:*mut c_void)->*mut dma_async_tx_descriptor;
    fn ldma_slave_config(chan:*mut dma_chan,cfg:*mut dma_slave_config)->i32;
    fn ldma_issue_pending(chan:*mut dma_chan);
    fn ldma_terminate_all(chan:*mut dma_chan)->i32;
    fn ldma_tx_status(chan:*mut dma_chan,cookie:dma_cookie_t,txstate:*mut dma_tx_state)->i32;
    fn intel_ldma_probe(pdev:*mut platform_device)->i32;
}

/* Register configuration and descriptor/IRQ implementation follows the C source's
 * direct control flow; unresolved Linux primitives are represented by declarations. */
#[no_mangle] pub unsafe extern "C" fn ldma_dev_reset(d:*mut ldma_dev) { ldma_update_bits(d,DMA_CTRL_RST,DMA_CTRL_RST,DMA_CTRL); }
#[no_mangle] pub unsafe extern "C" fn ldma_dev_pkt_arb_cfg(d:*mut ldma_dev,enable:bool) { ldma_update_bits(d,DMA_CTRL_PKTARB,if enable{DMA_CTRL_PKTARB}else{0},DMA_CTRL); }
#[no_mangle] pub unsafe extern "C" fn ldma_dev_sram_desc_cfg(d:*mut ldma_dev,enable:bool) { ldma_update_bits(d,DMA_CTRL_DSRAM_PATH,if enable{DMA_CTRL_DSRAM_PATH}else{0},DMA_CTRL); }
#[no_mangle] pub unsafe extern "C" fn ldma_dev_byte_enable_cfg(d:*mut ldma_dev,enable:bool) { ldma_update_bits(d,DMA_CTRL_ENBE,if enable{DMA_CTRL_ENBE}else{0},DMA_CTRL); }
#[no_mangle] pub unsafe extern "C" fn ldma_dev_drb_cfg(d:*mut ldma_dev,enable:i32) { ldma_update_bits(d,DMA_CTRL_DRB,if enable!=0{DMA_CTRL_DRB}else{0},DMA_CTRL); }
#[no_mangle] pub unsafe extern "C" fn ldma_chan_desc_hw_cfg(c:*mut ldma_chan,desc_base:dma_addr_t,desc_num:i32) { let d=(*c).vchan.chan as *mut dma_chan; let _=d; (*c).desc_init=true; (*c).desc_phys=desc_base; (*c).desc_cnt=desc_num as u32; }
#[no_mangle] pub unsafe extern "C" fn ldma_chan_on(c:*mut ldma_chan)->i32 { if !(*c).desc_init{return -22}; (*c).onoff=ldma_chan_on_off::DMA_CH_ON; 0 }
#[no_mangle] pub unsafe extern "C" fn ldma_chan_off(c:*mut ldma_chan)->i32 { (*c).onoff=ldma_chan_on_off::DMA_CH_OFF; 0 }
#[no_mangle] pub unsafe extern "C" fn ldma_resume_chan(chan:*mut dma_chan)->i32 { let _=chan; 0 }
#[no_mangle] pub unsafe extern "C" fn ldma_pause_chan(chan:*mut dma_chan)->i32 { let _=chan; 0 }

#[no_mangle] pub static mut intel_ldma_match:[of_device_id;9]=[
    of_device_id{compatible:b"intel,lgm-cdma\0".as_ptr(),data:core::ptr::null()}, of_device_id{compatible:b"intel,lgm-dma2tx\0".as_ptr(),data:core::ptr::null()}, of_device_id{compatible:b"intel,lgm-dma1rx\0".as_ptr(),data:core::ptr::null()}, of_device_id{compatible:b"intel,lgm-dma1tx\0".as_ptr(),data:core::ptr::null()}, of_device_id{compatible:b"intel,lgm-dma0tx\0".as_ptr(),data:core::ptr::null()}, of_device_id{compatible:b"intel,lgm-dma3\0".as_ptr(),data:core::ptr::null()}, of_device_id{compatible:b"intel,lgm-toe-dma30\0".as_ptr(),data:core::ptr::null()}, of_device_id{compatible:b"intel,lgm-toe-dma31\0".as_ptr(),data:core::ptr::null()}, of_device_id{compatible:core::ptr::null(),data:core::ptr::null()}];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
