/* include/video/s1d13xxxfb.h
 *
 * (c) 2004 Simtec Electronics
 * (c) 2005 Thibaut VARENE <varenet@parisc-linux.org>
 *
 * Header file for Epson S1D13XXX driver code
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file COPYING in the main directory of this archive for
 * more details.
 */

pub const S1D_PALETTE_SIZE: usize = 256;
pub const S1D_FBID: &str = "S1D13xxx";
pub const S1D_DEVICENAME: &str = "s1d13xxxfb";

/* S1DREG_REV_CODE register = prod_id (6 bits) + revision (2 bits) */
pub const S1D13505_PROD_ID: u8 = 0x3;
pub const S1D13506_PROD_ID: u8 = 0x4;
pub const S1D13806_PROD_ID: u8 = 0x7;

/* Register definitions (tested on s1d13896). */
pub const S1DREG_REV_CODE: u16 = 0x0000;
pub const S1DREG_MISC: u16 = 0x0001;
pub const S1DREG_GPIO_CNF0: u16 = 0x0004;
pub const S1DREG_GPIO_CNF1: u16 = 0x0005;
pub const S1DREG_GPIO_CTL0: u16 = 0x0008;
pub const S1DREG_GPIO_CTL1: u16 = 0x0009;
pub const S1DREG_CNF_STATUS: u16 = 0x000C;
pub const S1DREG_CLK_CNF: u16 = 0x0010;
pub const S1DREG_LCD_CLK_CNF: u16 = 0x0014;
pub const S1DREG_CRT_CLK_CNF: u16 = 0x0018;
pub const S1DREG_MPLUG_CLK_CNF: u16 = 0x001C;
pub const S1DREG_CPU2MEM_WST_SEL: u16 = 0x001E;
pub const S1DREG_MEM_CNF: u16 = 0x0020;
pub const S1DREG_SDRAM_REF_RATE: u16 = 0x0021;
pub const S1DREG_SDRAM_TC0: u16 = 0x002A;
pub const S1DREG_SDRAM_TC1: u16 = 0x002B;
pub const S1DREG_PANEL_TYPE: u16 = 0x0030;
pub const S1DREG_MOD_RATE: u16 = 0x0031;
pub const S1DREG_LCD_DISP_HWIDTH: u16 = 0x0032;
pub const S1DREG_LCD_NDISP_HPER: u16 = 0x0034;
pub const S1DREG_TFT_FPLINE_START: u16 = 0x0035;
pub const S1DREG_TFT_FPLINE_PWIDTH: u16 = 0x0036;
pub const S1DREG_LCD_DISP_VHEIGHT0: u16 = 0x0038;
pub const S1DREG_LCD_DISP_VHEIGHT1: u16 = 0x0039;
pub const S1DREG_LCD_NDISP_VPER: u16 = 0x003A;
pub const S1DREG_TFT_FPFRAME_START: u16 = 0x003B;
pub const S1DREG_TFT_FPFRAME_PWIDTH: u16 = 0x003C;
pub const S1DREG_LCD_DISP_MODE: u16 = 0x0040;
pub const S1DREG_LCD_MISC: u16 = 0x0041;
pub const S1DREG_LCD_DISP_START0: u16 = 0x0042;
pub const S1DREG_LCD_DISP_START1: u16 = 0x0043;
pub const S1DREG_LCD_DISP_START2: u16 = 0x0044;
pub const S1DREG_LCD_MEM_OFF0: u16 = 0x0046;
pub const S1DREG_LCD_MEM_OFF1: u16 = 0x0047;
pub const S1DREG_LCD_PIX_PAN: u16 = 0x0048;
pub const S1DREG_LCD_DISP_FIFO_HTC: u16 = 0x004A;
pub const S1DREG_LCD_DISP_FIFO_LTC: u16 = 0x004B;
pub const S1DREG_CRT_DISP_HWIDTH: u16 = 0x0050;
pub const S1DREG_CRT_NDISP_HPER: u16 = 0x0052;
pub const S1DREG_CRT_HRTC_START: u16 = 0x0053;
pub const S1DREG_CRT_HRTC_PWIDTH: u16 = 0x0054;
pub const S1DREG_CRT_DISP_VHEIGHT0: u16 = 0x0056;
pub const S1DREG_CRT_DISP_VHEIGHT1: u16 = 0x0057;
pub const S1DREG_CRT_NDISP_VPER: u16 = 0x0058;
pub const S1DREG_CRT_VRTC_START: u16 = 0x0059;
pub const S1DREG_CRT_VRTC_PWIDTH: u16 = 0x005A;
pub const S1DREG_TV_OUT_CTL: u16 = 0x005B;
pub const S1DREG_CRT_DISP_MODE: u16 = 0x0060;
pub const S1DREG_CRT_DISP_START0: u16 = 0x0062;
pub const S1DREG_CRT_DISP_START1: u16 = 0x0063;
pub const S1DREG_CRT_DISP_START2: u16 = 0x0064;
pub const S1DREG_CRT_MEM_OFF0: u16 = 0x0066;
pub const S1DREG_CRT_MEM_OFF1: u16 = 0x0067;
pub const S1DREG_CRT_PIX_PAN: u16 = 0x0068;
pub const S1DREG_CRT_DISP_FIFO_HTC: u16 = 0x006A;
pub const S1DREG_CRT_DISP_FIFO_LTC: u16 = 0x006B;
pub const S1DREG_LCD_CUR_CTL: u16 = 0x0070;
pub const S1DREG_LCD_CUR_START: u16 = 0x0071;
pub const S1DREG_LCD_CUR_XPOS0: u16 = 0x0072;
pub const S1DREG_LCD_CUR_XPOS1: u16 = 0x0073;
pub const S1DREG_LCD_CUR_YPOS0: u16 = 0x0074;
pub const S1DREG_LCD_CUR_YPOS1: u16 = 0x0075;
pub const S1DREG_LCD_CUR_BCTL0: u16 = 0x0076;
pub const S1DREG_LCD_CUR_GCTL0: u16 = 0x0077;
pub const S1DREG_LCD_CUR_RCTL0: u16 = 0x0078;
pub const S1DREG_LCD_CUR_BCTL1: u16 = 0x007A;
pub const S1DREG_LCD_CUR_GCTL1: u16 = 0x007B;
pub const S1DREG_LCD_CUR_RCTL1: u16 = 0x007C;
pub const S1DREG_LCD_CUR_FIFO_HTC: u16 = 0x007E;
pub const S1DREG_CRT_CUR_CTL: u16 = 0x0080;
pub const S1DREG_CRT_CUR_START: u16 = 0x0081;
pub const S1DREG_CRT_CUR_XPOS0: u16 = 0x0082;
pub const S1DREG_CRT_CUR_XPOS1: u16 = 0x0083;
pub const S1DREG_CRT_CUR_YPOS0: u16 = 0x0084;
pub const S1DREG_CRT_CUR_YPOS1: u16 = 0x0085;
pub const S1DREG_CRT_CUR_BCTL0: u16 = 0x0086;
pub const S1DREG_CRT_CUR_GCTL0: u16 = 0x0087;
pub const S1DREG_CRT_CUR_RCTL0: u16 = 0x0088;
pub const S1DREG_CRT_CUR_BCTL1: u16 = 0x008A;
pub const S1DREG_CRT_CUR_GCTL1: u16 = 0x008B;
pub const S1DREG_CRT_CUR_RCTL1: u16 = 0x008C;
pub const S1DREG_CRT_CUR_FIFO_HTC: u16 = 0x008E;
pub const S1DREG_BBLT_CTL0: u16 = 0x0100;
pub const S1DREG_BBLT_CTL1: u16 = 0x0101;
pub const S1DREG_BBLT_CC_EXP: u16 = 0x0102;
pub const S1DREG_BBLT_OP: u16 = 0x0103;
pub const S1DREG_BBLT_SRC_START0: u16 = 0x0104;
pub const S1DREG_BBLT_SRC_START1: u16 = 0x0105;
pub const S1DREG_BBLT_SRC_START2: u16 = 0x0106;
pub const S1DREG_BBLT_DST_START0: u16 = 0x0108;
pub const S1DREG_BBLT_DST_START1: u16 = 0x0109;
pub const S1DREG_BBLT_DST_START2: u16 = 0x010A;
pub const S1DREG_BBLT_MEM_OFF0: u16 = 0x010C;
pub const S1DREG_BBLT_MEM_OFF1: u16 = 0x010D;
pub const S1DREG_BBLT_WIDTH0: u16 = 0x0110;
pub const S1DREG_BBLT_WIDTH1: u16 = 0x0111;
pub const S1DREG_BBLT_HEIGHT0: u16 = 0x0112;
pub const S1DREG_BBLT_HEIGHT1: u16 = 0x0113;
pub const S1DREG_BBLT_BGC0: u16 = 0x0114;
pub const S1DREG_BBLT_BGC1: u16 = 0x0115;
pub const S1DREG_BBLT_FGC0: u16 = 0x0118;
pub const S1DREG_BBLT_FGC1: u16 = 0x0119;
pub const S1DREG_LKUP_MODE: u16 = 0x01E0;
pub const S1DREG_LKUP_ADDR: u16 = 0x01E2;
pub const S1DREG_LKUP_DATA: u16 = 0x01E4;
pub const S1DREG_PS_CNF: u16 = 0x01F0;
pub const S1DREG_PS_STATUS: u16 = 0x01F1;
pub const S1DREG_CPU2MEM_WDOGT: u16 = 0x01F4;
pub const S1DREG_COM_DISP_MODE: u16 = 0x01FC;
pub const S1DREG_DELAYOFF: u16 = 0xFFFE;
pub const S1DREG_DELAYON: u16 = 0xFFFF;
pub const BBLT_SOLID_FILL: u8 = 0x0c;

/* Note: all above defines should go in separate header files
   when implementing other S1D13xxx chip support. */

#[repr(C)]
pub struct s1d13xxxfb_regval {
    pub addr: u16,
    pub value: u8,
}

#[repr(C)]
pub struct s1d13xxxfb_par {
    pub regs: *mut core::ffi::c_void,
    pub display: u8,
    pub prod_id: u8,
    pub revision: u8,
    pub pseudo_palette: [core::ffi::c_uint; 16],
    #[cfg(feature = "CONFIG_PM")]
    pub regs_save: *mut core::ffi::c_void,
    #[cfg(feature = "CONFIG_PM")]
    pub disp_save: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct s1d13xxxfb_pdata {
    pub initregs: *const s1d13xxxfb_regval,
    pub initregssize: core::ffi::c_uint,
    pub platform_init_video: Option<unsafe extern "C" fn()>,
    #[cfg(feature = "CONFIG_PM")]
    pub platform_suspend_video: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    #[cfg(feature = "CONFIG_PM")]
    pub platform_resume_video: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
