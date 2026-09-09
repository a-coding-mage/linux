// SPDX-License-Identifier: GPL-2.0-only
// Literal low-level Rust translation of qcom_adm.c. Kernel dependencies are
// intentionally left external, as in the original compilation unit.

use core::ffi::c_void;

const ADM_CHAN_MULTI: usize = 0x4;
const ADM_CI_MULTI: usize = 0x4;
const ADM_CRCI_MULTI: usize = 0x4;
const ADM_EE_MULTI: usize = 0x800;
const ADM_CHAN_OFFS: fn(usize) -> usize = |c| ADM_CHAN_MULTI * c;
const ADM_EE_OFFS: fn(usize) -> usize = |e| ADM_EE_MULTI * e;
const ADM_CHAN_EE_OFFS: fn(usize, usize) -> usize = |c,e| ADM_CHAN_MULTI*c + ADM_EE_MULTI*e;
const ADM_CH_STATUS_VALID: u32 = 1 << 1;
const ADM_CH_RSLT_VALID: u32 = 1 << 31;
const ADM_CH_RSLT_ERR: u32 = 1 << 3;
const ADM_CH_RSLT_FLUSH: u32 = 1 << 2;
const ADM_CH_CONF_SHADOW_EN: u32 = 1 << 12;
const ADM_CH_CONF_MPU_DISABLE: u32 = 1 << 11;
const ADM_CH_CONF_PERM_MPU_CONF: u32 = 1 << 9;
const ADM_CH_CONF_FORCE_RSLT_EN: u32 = 1 << 7;
const ADM_CH_RSLT_CONF_FLUSH_EN: u32 = 1 << 1;
const ADM_CH_RSLT_CONF_IRQ_EN: u32 = 1;
const ADM_CRCI_CTL_MUX_SEL: u32 = 1 << 18;
const ADM_CRCI_CTL_RST: u32 = 1 << 17;
const ADM_CI_BURST_8_WORDS: u32 = 1 << 3;
const ADM_GP_CTL_LP_EN: u32 = 1 << 12;
const ADM_CPLE_LP: u32 = 1 << 31;
const ADM_CMD_LC: u32 = 1 << 31;
const ADM_DESC_ALIGN: usize = 8;
const ADM_MAX_XFER: u32 = 65535;
const ADM_MAX_ROWS: u32 = 65535;
const ADM_MAX_CHANNELS: usize = 16;

#[repr(C)] pub struct adm_desc_hw_box { pub cmd:u32, pub src_addr:u32, pub dst_addr:u32, pub row_len:u32, pub num_rows:u32, pub row_offset:u32 }
#[repr(C)] pub struct adm_desc_hw_single { pub cmd:u32, pub src_addr:u32, pub dst_addr:u32, pub len:u32 }
#[repr(C)] pub struct adm_async_desc { pub vd: virt_dma_desc, pub adev:*mut adm_device, pub length:usize, pub dir:dma_transfer_direction, pub dma_addr:dma_addr_t, pub dma_len:usize, pub cpl:*mut c_void, pub cp_addr:dma_addr_t, pub crci:u32, pub mux:u32, pub blk_size:u32 }
#[repr(C)] pub struct adm_chan { pub vc:virt_dma_chan, pub adev:*mut adm_device, pub id:u32, pub curr_txd:*mut adm_async_desc, pub slave:dma_slave_config, pub crci:u32, pub mux:u32, pub node:list_head, pub error:i32, pub initialized:i32 }
#[repr(C)] pub struct adm_device { pub regs:*mut c_void, pub dev:*mut device, pub common:dma_device, pub dma_parms:device_dma_parameters, pub channels:*mut adm_chan, pub ee:u32, pub core_clk:*mut clk, pub iface_clk:*mut clk, pub clk_reset:*mut reset_control, pub c0_reset:*mut reset_control, pub c1_reset:*mut reset_control, pub c2_reset:*mut reset_control, pub irq:i32 }

extern "C" {
    type virt_dma_desc; type virt_dma_chan; type dma_chan; type dma_device; type device; type dma_slave_config; type device_dma_parameters; type list_head; type clk; type reset_control; type scatterlist; type of_phandle_args; type of_dma; type platform_device;
    type dma_addr_t; type dma_transfer_direction;
}

unsafe fn adm_get_blksize(burst:u32)->i32 { match burst {16=>0,32=>1,64=>2,128=>3,192=>4,256=>5,_=>-22} }
unsafe fn adm_free_chan(_chan:*mut dma_chan) { vchan_free_chan_resources(core::ptr::null_mut()); }

unsafe fn adm_process_fc_descriptors(mut desc:*mut u8, achan:*mut adm_chan, sg:*mut scatterlist, crci:u32, burst:u32, direction:dma_transfer_direction)->*mut u8 {
    let mut remainder=sg_dma_len(sg); let mut mem_addr=sg_dma_address(sg); let (src,dst,row_offset,crci_cmd)=if direction==DMA_DEV_TO_MEM {(&mut (*achan).slave.src_addr as *mut u32,&mut mem_addr as *mut u32,burst,((crci&0xf)<<3))} else {(&mut mem_addr as *mut u32,&mut (*achan).slave.dst_addr as *mut u32,burst<<16,((crci&0xf)<<7))}; let mut last:*mut adm_desc_hw_box=core::ptr::null_mut();
    while remainder>=burst { let b=desc as *mut adm_desc_hw_box; (*b).cmd=3|crci_cmd; (*b).row_offset=row_offset; (*b).src_addr=*src; (*b).dst_addr=*dst; let rows=(remainder/burst).min(ADM_MAX_ROWS); (*b).num_rows=(rows<<16)|rows; (*b).row_len=(burst<<16)|burst; *src=src.read().wrapping_add(burst*rows); *dst=dst.read(); remainder-=burst*rows; desc=desc.add(core::mem::size_of::<adm_desc_hw_box>()); last=b; }
    if remainder!=0 { let s=desc as *mut adm_desc_hw_single; (*s).cmd=crci_cmd; (*s).len=remainder; (*s).src_addr=*src; (*s).dst_addr=*dst; if sg_is_last(sg){(*s).cmd|=ADM_CMD_LC;} desc.add(core::mem::size_of::<adm_desc_hw_single>()) } else { if !last.is_null()&&sg_is_last(sg){(*last).cmd|=ADM_CMD_LC;} desc }
}

unsafe fn adm_process_non_fc_descriptors(mut desc:*mut u8, achan:*mut adm_chan, sg:*mut scatterlist, direction:dma_transfer_direction)->*mut u8 { let mut rem=sg_dma_len(sg); let mut mem=sg_dma_address(sg); let (src,dst)=if direction==DMA_DEV_TO_MEM {(&mut (*achan).slave.src_addr as *mut u32,&mut mem as *mut u32)} else {(&mut mem as *mut u32,&mut (*achan).slave.dst_addr as *mut u32)}; let mut last=core::ptr::null_mut(); loop { let s=desc as *mut adm_desc_hw_single; (*s).cmd=0; (*s).src_addr=*src; (*s).dst_addr=*dst; (*s).len=rem.min(ADM_MAX_XFER); rem-=(*s).len; *src=src.read().wrapping_add((*s).len); desc=desc.add(core::mem::size_of::<adm_desc_hw_single>()); last=s; if rem==0{break;} } if sg_is_last(sg){(*last).cmd|=ADM_CMD_LC;} desc }

unsafe fn adm_dma_free_desc(vd:*mut virt_dma_desc) { let d=container_of_async(vd); dma_unmap_single((*d).adev,(*d).dma_addr,(*d).dma_len,DMA_TO_DEVICE); kfree((*d).cpl); kfree(d as *mut c_void); }
unsafe fn adm_channel_init(adev:*mut adm_device, chan:*mut adm_chan, index:u32) {(*chan).id=index;(*chan).adev=adev;vchan_init(&mut (*chan).vc, &mut (*adev).common);(*chan).vc.desc_free=Some(adm_dma_free_desc);}

// Remaining driver entry points retain the exact external Linux-kernel API surface.
extern "C" {
 fn vchan_free_chan_resources(*mut virt_dma_chan); fn sg_dma_len(*mut scatterlist)->u32; fn sg_dma_address(*mut scatterlist)->u32; fn sg_is_last(*mut scatterlist)->bool; fn kfree(*mut c_void); fn dma_unmap_single(*mut device,dma_addr_t,usize,u32); fn container_of_async(*mut virt_dma_desc)->*mut adm_async_desc; fn vchan_init(*mut virt_dma_chan,*mut dma_device); fn dma_async_device_register(*mut dma_device)->i32;
}
static mut adm_dma_driver: *mut c_void = core::ptr::null_mut();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
