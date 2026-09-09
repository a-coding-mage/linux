/* Translated from imx-ipu-v3.h. */

pub struct ipu_soc;

#[repr(C)]
pub enum ipuv3_type { IPUV3EX, IPUV3M, IPUV3H }

pub const IPU_PIX_FMT_GBR24: u32 = (b'G' as u32) | ((b'B' as u32) << 8) |
    ((b'R' as u32) << 16) | ((b'3' as u32) << 24);

#[repr(C)]
pub struct ipu_di_signal_cfg {
    pub data_pol: u32,
    pub clk_pol: u32,
    pub enable_pol: u32,
    pub mode: videomode,
    pub bus_format: u32,
    pub v_to_h_sync: u32,
    pub clkflags: c_ulong,
    pub hsync_pin: u8,
    pub vsync_pin: u8,
}
pub const IPU_DI_CLKMODE_SYNC: c_ulong = 1 << 0;
pub const IPU_DI_CLKMODE_EXT: c_ulong = 1 << 1;

#[repr(C)]
pub enum ipu_csi_dest { IPU_CSI_DEST_IDMAC, IPU_CSI_DEST_IC, IPU_CSI_DEST_VDIC }
pub const IPU_ROT_BIT_VFLIP: u32 = 1 << 0;
pub const IPU_ROT_BIT_HFLIP: u32 = 1 << 1;
pub const IPU_ROT_BIT_90: u32 = 1 << 2;
#[repr(C)]
pub enum ipu_rotate_mode {
    IPU_ROTATE_NONE = 0, IPU_ROTATE_VERT_FLIP = IPU_ROT_BIT_VFLIP as isize,
    IPU_ROTATE_HORIZ_FLIP = IPU_ROT_BIT_HFLIP as isize,
    IPU_ROTATE_180 = (IPU_ROT_BIT_VFLIP | IPU_ROT_BIT_HFLIP) as isize,
    IPU_ROTATE_90_RIGHT = IPU_ROT_BIT_90 as isize,
    IPU_ROTATE_90_RIGHT_VFLIP = (IPU_ROT_BIT_90 | IPU_ROT_BIT_VFLIP) as isize,
    IPU_ROTATE_90_RIGHT_HFLIP = (IPU_ROT_BIT_90 | IPU_ROT_BIT_HFLIP) as isize,
    IPU_ROTATE_90_LEFT = (IPU_ROT_BIT_90 | IPU_ROT_BIT_VFLIP | IPU_ROT_BIT_HFLIP) as isize,
}
#[inline] pub fn ipu_rot_mode_is_irt(m: u32) -> bool { (m & IPU_ROT_BIT_90) != 0 }

#[repr(C)] pub enum ipu_color_space { IPUV3_COLORSPACE_RGB, IPUV3_COLORSPACE_YUV, IPUV3_COLORSPACE_UNKNOWN }
#[repr(C)] pub enum ipu_motion_sel { MOTION_NONE = 0, LOW_MOTION, MED_MOTION, HIGH_MOTION }
pub struct ipuv3_channel;
#[repr(C)] pub enum ipu_channel_irq { IPU_IRQ_EOF = 0, IPU_IRQ_NFACK = 64, IPU_IRQ_NFB4EOF = 128, IPU_IRQ_EOS = 192 }

pub const IPUV3_CHANNEL_CSI0: u32 = 0; pub const IPUV3_CHANNEL_CSI1: u32 = 1;
pub const IPUV3_CHANNEL_CSI2: u32 = 2; pub const IPUV3_CHANNEL_CSI3: u32 = 3;
pub const IPUV3_CHANNEL_VDI_MEM_IC_VF: u32 = 5; pub const IPUV3_CHANNEL_CSI_DIRECT: u32 = 6;
pub const IPUV3_CHANNEL_CSI_VDI_PREV: u32 = 7; pub const IPUV3_CHANNEL_MEM_VDI_PREV: u32 = 8;
pub const IPUV3_CHANNEL_MEM_VDI_CUR: u32 = 9; pub const IPUV3_CHANNEL_MEM_VDI_NEXT: u32 = 10;
pub const IPUV3_CHANNEL_MEM_IC_PP: u32 = 11; pub const IPUV3_CHANNEL_MEM_IC_PRP_VF: u32 = 12;
pub const IPUV3_CHANNEL_VDI_MEM_RECENT: u32 = 13; pub const IPUV3_CHANNEL_G_MEM_IC_PRP_VF: u32 = 14;
pub const IPUV3_CHANNEL_G_MEM_IC_PP: u32 = 15; pub const IPUV3_CHANNEL_G_MEM_IC_PRP_VF_ALPHA: u32 = 17;
pub const IPUV3_CHANNEL_G_MEM_IC_PP_ALPHA: u32 = 18; pub const IPUV3_CHANNEL_MEM_VDI_PLANE1_COMB_ALPHA: u32 = 19;
pub const IPUV3_CHANNEL_IC_PRP_ENC_MEM: u32 = 20; pub const IPUV3_CHANNEL_IC_PRP_VF_MEM: u32 = 21;
pub const IPUV3_CHANNEL_IC_PP_MEM: u32 = 22; pub const IPUV3_CHANNEL_MEM_BG_SYNC: u32 = 23;
pub const IPUV3_CHANNEL_MEM_BG_ASYNC: u32 = 24; pub const IPUV3_CHANNEL_MEM_VDI_PLANE1_COMB: u32 = 25;
pub const IPUV3_CHANNEL_MEM_VDI_PLANE3_COMB: u32 = 26; pub const IPUV3_CHANNEL_MEM_FG_SYNC: u32 = 27;
pub const IPUV3_CHANNEL_MEM_DC_SYNC: u32 = 28; pub const IPUV3_CHANNEL_MEM_FG_ASYNC: u32 = 29;
pub const IPUV3_CHANNEL_MEM_FG_SYNC_ALPHA: u32 = 31; pub const IPUV3_CHANNEL_MEM_FG_ASYNC_ALPHA: u32 = 33;
pub const IPUV3_CHANNEL_DC_MEM_READ: u32 = 40; pub const IPUV3_CHANNEL_MEM_DC_ASYNC: u32 = 41;
pub const IPUV3_CHANNEL_MEM_DC_COMMAND: u32 = 42; pub const IPUV3_CHANNEL_MEM_DC_COMMAND2: u32 = 43;
pub const IPUV3_CHANNEL_MEM_DC_OUTPUT_MASK: u32 = 44; pub const IPUV3_CHANNEL_MEM_ROT_ENC: u32 = 45;
pub const IPUV3_CHANNEL_MEM_ROT_VF: u32 = 46; pub const IPUV3_CHANNEL_MEM_ROT_PP: u32 = 47;
pub const IPUV3_CHANNEL_ROT_ENC_MEM: u32 = 48; pub const IPUV3_CHANNEL_ROT_VF_MEM: u32 = 49;
pub const IPUV3_CHANNEL_ROT_PP_MEM: u32 = 50; pub const IPUV3_CHANNEL_MEM_BG_SYNC_ALPHA: u32 = 51;
pub const IPUV3_CHANNEL_MEM_BG_ASYNC_ALPHA: u32 = 52; pub const IPUV3_NUM_CHANNELS: u32 = 64;

#[inline] pub fn ipu_channel_alpha_channel(ch_num: i32) -> i32 { match ch_num {
    x if x == IPUV3_CHANNEL_G_MEM_IC_PRP_VF as i32 => IPUV3_CHANNEL_G_MEM_IC_PRP_VF_ALPHA as i32,
    x if x == IPUV3_CHANNEL_G_MEM_IC_PP as i32 => IPUV3_CHANNEL_G_MEM_IC_PP_ALPHA as i32,
    x if x == IPUV3_CHANNEL_MEM_FG_SYNC as i32 => IPUV3_CHANNEL_MEM_FG_SYNC_ALPHA as i32,
    x if x == IPUV3_CHANNEL_MEM_FG_ASYNC as i32 => IPUV3_CHANNEL_MEM_FG_ASYNC_ALPHA as i32,
    x if x == IPUV3_CHANNEL_MEM_BG_SYNC as i32 => IPUV3_CHANNEL_MEM_BG_SYNC_ALPHA as i32,
    x if x == IPUV3_CHANNEL_MEM_BG_ASYNC as i32 => IPUV3_CHANNEL_MEM_BG_ASYNC_ALPHA as i32,
    x if x == IPUV3_CHANNEL_MEM_VDI_PLANE1_COMB as i32 => IPUV3_CHANNEL_MEM_VDI_PLANE1_COMB_ALPHA as i32,
    _ => -22,
} }

extern "C" {
    pub fn ipu_map_irq(ipu: *mut ipu_soc, irq: i32) -> i32;
    pub fn ipu_idmac_channel_irq(ipu: *mut ipu_soc, channel: *mut ipuv3_channel, irq: ipu_channel_irq) -> i32;
}

pub const IPU_IRQ_DP_SF_START: i32 = 450; pub const IPU_IRQ_DP_SF_END: i32 = 451;
pub const IPU_IRQ_BG_SF_END: i32 = IPU_IRQ_DP_SF_END;
pub const IPU_IRQ_DC_FC_0: i32 = 456; pub const IPU_IRQ_DC_FC_1: i32 = 457;
pub const IPU_IRQ_DC_FC_2: i32 = 458; pub const IPU_IRQ_DC_FC_3: i32 = 459;
pub const IPU_IRQ_DC_FC_4: i32 = 460; pub const IPU_IRQ_DC_FC_6: i32 = 461;
pub const IPU_IRQ_VSYNC_PRE_0: i32 = 462; pub const IPU_IRQ_VSYNC_PRE_1: i32 = 463;

/* Remaining declarations retain the header's external ABI. */
pub type c_ulong = usize;
pub type dma_addr_t = usize;
pub struct videomode;
pub struct fb_bitfield; pub struct v4l2_pix_format; pub struct v4l2_rect;
pub struct device_node; pub struct v4l2_mbus_config; pub struct v4l2_mbus_framefmt;
pub struct ipu_dc; pub struct ipu_di; pub struct dmfc_channel; pub struct ipu_dp;
pub struct ipu_csi; pub struct ipu_ic; pub struct ipu_vdi; pub struct ipu_smfc;
pub type v4l2_std_id = u64;
pub type drm_color_encoding = u32; pub type drm_color_range = u32;
pub type v4l2_ycbcr_encoding = u32; pub type v4l2_quantization = u32;

#[repr(C)] pub struct ipu_rgb { pub red: fb_bitfield, pub green: fb_bitfield, pub blue: fb_bitfield, pub transp: fb_bitfield, pub bits_per_pixel: i32 }
#[repr(C)] pub struct ipu_image { pub pix: v4l2_pix_format, pub rect: v4l2_rect, pub phys0: dma_addr_t, pub phys1: dma_addr_t, pub u_offset: u32, pub v_offset: u32 }
#[repr(C)] pub enum ipu_ic_task { IC_TASK_ENCODER, IC_TASK_VIEWFINDER, IC_TASK_POST_PROCESSOR, IC_NUM_TASKS }
#[repr(C)] pub struct ipu_ic_colorspace { pub enc: v4l2_ycbcr_encoding, pub quant: v4l2_quantization, pub cs: ipu_color_space }
#[inline] pub unsafe fn ipu_ic_fill_colorspace(p: *mut ipu_ic_colorspace, enc: v4l2_ycbcr_encoding, quant: v4l2_quantization, cs: ipu_color_space) { (*p).enc=enc; (*p).quant=quant; (*p).cs=cs; }
#[repr(C)] pub struct ipu_ic_csc_params { pub coeff: [[i16;3];3], pub offset: [i16;3], pub scale: u8, pub sat: bool }
#[repr(C)] pub struct ipu_ic_csc { pub in_cs: ipu_ic_colorspace, pub out_cs: ipu_ic_colorspace, pub params: ipu_ic_csc_params }
#[repr(C)] pub struct ipu_client_platformdata { pub csi: i32, pub di: i32, pub dc: i32, pub dp: i32, pub dma: [i32;2], pub of_node: *mut device_node }
pub const IPU_DP_FLOW_SYNC_BG: u32 = 0;
pub const IPU_DP_FLOW_SYNC_FG: u32 = 1;
pub const IPU_DP_FLOW_ASYNC0_BG: u32 = 2;
pub const IPU_DP_FLOW_ASYNC0_FG: u32 = 3;
pub const IPU_DP_FLOW_ASYNC1_BG: u32 = 4;
pub const IPU_DP_FLOW_ASYNC1_FG: u32 = 5;

/* External declarations from the remaining header sections. */
extern "C" {
    pub fn ipu_get_num(ipu: *mut ipu_soc) -> i32;
    pub fn ipu_set_csi_src_mux(ipu: *mut ipu_soc, csi_id: i32, mipi_csi2: bool);
    pub fn ipu_set_ic_src_mux(ipu: *mut ipu_soc, csi_id: i32, vdi: bool);
    pub fn ipu_dump(ipu: *mut ipu_soc);
    pub fn ipu_idmac_get(ipu: *mut ipu_soc, channel: u32) -> *mut ipuv3_channel;
    pub fn ipu_idmac_put(channel: *mut ipuv3_channel);
    pub fn ipu_idmac_enable_channel(channel: *mut ipuv3_channel) -> i32;
    pub fn ipu_idmac_disable_channel(channel: *mut ipuv3_channel) -> i32;
    pub fn ipu_idmac_enable_watermark(channel: *mut ipuv3_channel, enable: bool);
    pub fn ipu_idmac_lock_enable(channel: *mut ipuv3_channel, num_bursts: i32) -> i32;
    pub fn ipu_idmac_wait_busy(channel: *mut ipuv3_channel, ms: i32) -> i32;
    pub fn ipu_idmac_set_double_buffer(channel: *mut ipuv3_channel, doublebuffer: bool);
    pub fn ipu_idmac_get_current_buffer(channel: *mut ipuv3_channel) -> i32;
    pub fn ipu_idmac_buffer_is_ready(channel: *mut ipuv3_channel, buf_num: u32) -> bool;
    pub fn ipu_idmac_select_buffer(channel: *mut ipuv3_channel, buf_num: u32);
    pub fn ipu_idmac_clear_buffer(channel: *mut ipuv3_channel, buf_num: u32);
    pub fn ipu_fsu_link(ipu: *mut ipu_soc, src_ch: i32, sink_ch: i32) -> i32;
    pub fn ipu_fsu_unlink(ipu: *mut ipu_soc, src_ch: i32, sink_ch: i32) -> i32;
    pub fn ipu_idmac_link(src: *mut ipuv3_channel, sink: *mut ipuv3_channel) -> i32;
    pub fn ipu_idmac_unlink(src: *mut ipuv3_channel, sink: *mut ipuv3_channel) -> i32;
    pub fn ipu_cpmem_zero(ch: *mut ipuv3_channel);
    pub fn ipu_cpmem_set_resolution(ch: *mut ipuv3_channel, xres: i32, yres: i32);
    pub fn ipu_cpmem_set_stride(ch: *mut ipuv3_channel, stride: i32);
    pub fn ipu_cpmem_set_buffer(ch: *mut ipuv3_channel, bufnum: i32, buf: dma_addr_t);
    pub fn ipu_cpmem_set_rotation(ch: *mut ipuv3_channel, rot: ipu_rotate_mode);
    pub fn ipu_cpmem_set_fmt(ch: *mut ipuv3_channel, drm_fourcc: u32) -> i32;
    pub fn ipu_cpmem_set_image(ch: *mut ipuv3_channel, image: *mut ipu_image) -> i32;
    pub fn ipu_dc_get(ipu: *mut ipu_soc, channel: i32) -> *mut ipu_dc;
    pub fn ipu_dc_put(dc: *mut ipu_dc);
    pub fn ipu_dc_enable(ipu: *mut ipu_soc);
    pub fn ipu_dc_disable(ipu: *mut ipu_soc);
    pub fn ipu_di_get(ipu: *mut ipu_soc, disp: i32) -> *mut ipu_di;
    pub fn ipu_di_put(di: *mut ipu_di);
    pub fn ipu_di_disable(di: *mut ipu_di) -> i32;
    pub fn ipu_di_enable(di: *mut ipu_di) -> i32;
    pub fn ipu_dmfc_get(ipu: *mut ipu_soc, ch: i32) -> *mut dmfc_channel;
    pub fn ipu_dmfc_put(dmfc: *mut dmfc_channel);
    pub fn ipu_dp_get(ipu: *mut ipu_soc, flow: u32) -> *mut ipu_dp;
    pub fn ipu_dp_put(dp: *mut ipu_dp);
    pub fn ipu_dp_enable(ipu: *mut ipu_soc) -> i32;
    pub fn ipu_dp_disable(ipu: *mut ipu_soc);
    pub fn ipu_prg_max_active_channels() -> i32;
    pub fn ipu_prg_present(ipu: *mut ipu_soc) -> bool;
    pub fn ipu_prg_enable(ipu: *mut ipu_soc) -> i32;
    pub fn ipu_prg_disable(ipu: *mut ipu_soc);
    pub fn ipu_csi_get(ipu: *mut ipu_soc, id: i32) -> *mut ipu_csi;
    pub fn ipu_csi_put(csi: *mut ipu_csi);
    pub fn ipu_csi_enable(csi: *mut ipu_csi) -> i32;
    pub fn ipu_csi_disable(csi: *mut ipu_csi) -> i32;
    pub fn ipu_ic_enable(ic: *mut ipu_ic) -> i32;
    pub fn ipu_ic_disable(ic: *mut ipu_ic) -> i32;
    pub fn ipu_ic_put(ic: *mut ipu_ic);
    pub fn ipu_vdi_get(ipu: *mut ipu_soc) -> *mut ipu_vdi;
    pub fn ipu_vdi_put(vdi: *mut ipu_vdi);
    pub fn ipu_smfc_get(ipu: *mut ipu_soc, chno: u32) -> *mut ipu_smfc;
    pub fn ipu_smfc_put(smfc: *mut ipu_smfc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
