/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of rtsx_usb.h. Kernel types and external symbols are supplied by dependencies. */

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
pub struct rtsx_ucr {
    pub vendor_id: u16, pub product_id: u16, pub package: i32, pub ic_version: u8,
    pub is_rts5179: bool, pub cur_clk: u32, pub cmd_buf: *mut u8, pub cmd_idx: u32,
    pub rsp_buf: *mut u8, pub pusb_dev: *mut usb_device, pub pusb_intf: *mut usb_interface,
    pub current_sg: usb_sg_request, pub sg_timer: timer_list, pub dev_mutex: mutex,
    pub card_status_cache: u16, pub card_status_valid: bool,
}
#[repr(C)] pub struct usb_device { _private: [u8; 0] }
#[repr(C)] pub struct usb_interface { _private: [u8; 0] }
#[repr(C)] pub struct usb_sg_request { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }

macro_rules! c { ($n:ident, $v:expr) => { pub const $n: u32 = $v; }; }
pub const DRV_NAME_RTSX_USB: &str = "rtsx_usb";
pub const DRV_NAME_RTSX_USB_SDMMC: &str = "rtsx_usb_sdmmc";
pub const DRV_NAME_RTSX_USB_MS: &str = "rtsx_usb_ms";
c!(RTSX_USB_SD_CARD,0); c!(RTSX_USB_MS_CARD,1); c!(EP_BULK_OUT,1); c!(EP_BULK_IN,2); c!(EP_INTR_IN,3);
c!(RTSX_USB_REQ_REG_OP,0); c!(RTSX_USB_REQ_POLL,2); c!(MIN_DIV_N,60); c!(MAX_DIV_N,120); c!(MAX_PHASE,15); c!(RX_TUNING_CNT,3); c!(QFN24,0); c!(LQFP48,1); c!(IOBUF_SIZE,1024);
c!(SD_CD,0x01); c!(MS_CD,0x02); c!(XD_CD,0x04); c!(CD_MASK,0x07); c!(SD_WP,0x08);
c!(MS_OCP_DETECT_EN,8); c!(MS_OCP_INT_EN,4); c!(MS_OCP_INT_CLR,2); c!(MS_OCP_CLEAR,1); c!(MS_OCP_DETECT,0x80); c!(MS_OCP_NOW,2); c!(MS_OCP_EVER,1);
c!(READ_REG_CMD,0); c!(WRITE_REG_CMD,1); c!(CHECK_REG_CMD,2); c!(PACKET_TYPE,4); c!(CNT_H,5); c!(CNT_L,6); c!(STAGE_FLAG,7); c!(CMD_OFFSET,8); c!(SEQ_WRITE_DATA_OFFSET,12); c!(BATCH_CMD,0); c!(SEQ_READ,1); c!(SEQ_WRITE,2); c!(STAGE_R,1); c!(STAGE_DI,2); c!(STAGE_DO,4); c!(STAGE_MS_STATUS,8); c!(STAGE_XD_STATUS,0x10); c!(MODE_C,0); c!(MODE_CR,1); c!(MODE_CDIR,3); c!(MODE_CDOR,5); c!(EP0_OP_SHIFT,14); c!(EP0_READ_REG_CMD,2); c!(EP0_WRITE_REG_CMD,3);

// Internal register addresses and values.
macro_rules! regs { ($($n:ident=$v:expr),* $(,)?) => { $(c!($n,$v);)* }; }
regs! { FPDCTL=0xFC00, SSC_DIV_N_0=0xFC07, SSC_CTL1=0xFC09, SSC_CTL2=0xFC0A, CFG_MODE=0xFC0E, CFG_MODE_1=0xFC0F, RCCTL=0xFC14, SOF_WDOG=0xFC28, SYS_DUMMY0=0xFC30,
MS_BLKEND=0xFD30, MS_READ_START=0xFD31, MS_READ_COUNT=0xFD32, MS_WRITE_START=0xFD33, MS_WRITE_COUNT=0xFD34, MS_COMMAND=0xFD35, MS_OLD_BLOCK_0=0xFD36, MS_OLD_BLOCK_1=0xFD37, MS_NEW_BLOCK_0=0xFD38, MS_NEW_BLOCK_1=0xFD39, MS_LOG_BLOCK_0=0xFD3A, MS_LOG_BLOCK_1=0xFD3B, MS_BUS_WIDTH=0xFD3C, MS_PAGE_START=0xFD3D, MS_PAGE_LENGTH=0xFD3E, MS_CFG=0xFD40, MS_TPC=0xFD41, MS_TRANS_CFG=0xFD42, MS_TRANSFER=0xFD43, MS_INT_REG=0xFD44, MS_BYTE_CNT=0xFD45, MS_SECTOR_CNT_L=0xFD46, MS_SECTOR_CNT_H=0xFD47, MS_DBUS_H=0xFD48,
CARD_DMA1_CTL=0xFD5C, CARD_PULL_CTL1=0xFD60, CARD_PULL_CTL2=0xFD61, CARD_PULL_CTL3=0xFD62, CARD_PULL_CTL4=0xFD63, CARD_PULL_CTL5=0xFD64, CARD_PULL_CTL6=0xFD65, CARD_EXIST=0xFD6F, CARD_INT_PEND=0xFD71, LDO_POWER_CFG=0xFD7B, SD_CFG1=0xFDA0, SD_CFG2=0xFDA1, SD_CFG3=0xFDA2, SD_STAT1=0xFDA3, SD_STAT2=0xFDA4, SD_BUS_STAT=0xFDA5, SD_PAD_CTL=0xFDA6, SD_SAMPLE_POINT_CTL=0xFDA7, SD_PUSH_POINT_CTL=0xFDA8, SD_CMD0=0xFDA9, SD_CMD1=0xFDAA, SD_CMD2=0xFDAB, SD_CMD3=0xFDAC, SD_CMD4=0xFDAD, SD_CMD5=0xFDAE, SD_BYTE_CNT_L=0xFDAF, SD_BYTE_CNT_H=0xFDB0, SD_BLOCK_CNT_L=0xFDB1, SD_BLOCK_CNT_H=0xFDB2, SD_TRANSFER=0xFDB3, SD_CMD_STATE=0xFDB5, SD_DATA_STATE=0xFDB6, SD_VPCLK0_CTL=0xFC2A, SD_VPCLK1_CTL=0xFC2B, SD_DCMPS0_CTL=0xFC2C, SD_DCMPS1_CTL=0xFC2D,
HW_VERSION=0xFC01, SSC_CLK_FPGA_SEL=0xFC02, CLK_DIV=0xFC03, SFSM_ED=0xFC04, CD_DEGLITCH_WIDTH=0xFC20, CD_DEGLITCH_EN=0xFC21, AUTO_DELINK_EN=0xFC23, FPGA_PULL_CTL=0xFC1D, CARD_CLK_SOURCE=0xFC2E, CARD_SHARE_MODE=0xFD51, CARD_DRIVE_SEL=0xFD52, CARD_STOP=0xFD53, CARD_OE=0xFD54, CARD_AUTO_BLINK=0xFD55, CARD_GPIO=0xFD56, SD30_DRIVE_SEL=0xFD57, CARD_DATA_SOURCE=0xFD5D, CARD_SELECT=0xFD5E, CARD_CLK_EN=0xFD79, CARD_PWR_CTL=0xFD7A, OCPCTL=0xFD80, OCPPARA1=0xFD81, OCPPARA2=0xFD82, OCPSTAT=0xFD83, HS_USB_STAT=0xFE01, HS_VCONTROL=0xFE26, HS_VSTAIN=0xFE27, HS_VLOADM=0xFE28, HS_VSTAOUT=0xFE29, MC_IRQ=0xFF00, MC_IRQEN=0xFF01, MC_FIFO_CTL=0xFF02, MC_FIFO_BC0=0xFF03, MC_FIFO_BC1=0xFF04, MC_FIFO_STAT=0xFF05, MC_FIFO_MODE=0xFF06, MC_FIFO_RD_PTR0=0xFF07, MC_FIFO_RD_PTR1=0xFF08, MC_DMA_CTL=0xFF10, MC_DMA_TC0=0xFF11, MC_DMA_TC1=0xFF12, MC_DMA_TC2=0xFF13, MC_DMA_TC3=0xFF14, MC_DMA_RST=0xFF15, RBUF_SIZE_MASK=0xFBFF, RBUF_BASE=0xF000, PPBUF_BASE1=0xF800, PPBUF_BASE2=0xFA00 }

// Remaining register-value macros, preserved as constants.
regs! { POWER_OFF=3, PARTIAL_POWER_ON=2, POWER_ON=0, POWER_MASK=3, LDO3318_PWR_MASK=0x0C, LDO_ON=0, LDO_SUSPEND=8, LDO_OFF=0x0C, DV3318_AUTO_PWR_OFF=0x10, FORCE_LDO_POWERB=0x60, TUNE_SD18_MASK=0x1C, TUNE_SD18_1V7=0, TUNE_SD18_1V8=4, TUNE_SD18_1V9=8, TUNE_SD18_2V0=12, TUNE_SD18_2V7=16, TUNE_SD18_2V8=20, TUNE_SD18_2V9=24, TUNE_SD18_3V3=28, CLK_CHANGE=0x80, CLK_DIV_1=0, CLK_DIV_2=1, CLK_DIV_4=2, CLK_DIV_8=3, SSC_POWER_MASK=1, SSC_POWER_DOWN=1, SSC_POWER_ON=0, FPGA_VER=0x80, HW_VER_MASK=0x0F, EXTEND_DMA1_ASYNC_SIGNAL=2, XTAL_FREE=0x80, CLK_MODE_MASK=3, CLK_MODE_12M_XTAL=0, CLK_MODE_NON_XTAL=1, CLK_MODE_24M_OSC=2, CLK_MODE_48M_OSC=3, RTS5179=2, NYET_EN=1, NYET_MSAK=1, SD30_DRIVE_MASK=7, SD20_DRIVE_MASK=3, DISABLE_SD_CD=8, DISABLE_MS_CD=16, DISABLE_XD_CD=32, SD_CD_DEGLITCH_EN=1, MS_CD_DEGLITCH_EN=2, XD_CD_DEGLITCH_EN=4, CARD_SHARE_LQFP48=4, CARD_SHARE_QFN24=0, CARD_SHARE_LQFP_SEL=4, CARD_SHARE_XD=0, CARD_SHARE_SD=1, CARD_SHARE_MS=2, CARD_SHARE_MASK=3, DRIVER_TYPE_A=5, DRIVER_TYPE_B=3, DRIVER_TYPE_C=2, DRIVER_TYPE_D=1, SD_CLK_TOGGLE_EN=0x80, SD_CLK_FORCE_STOP=0x40, SD_DAT3_STATUS=0x10, SD_DAT2_STATUS=8, SD_DAT1_STATUS=4, SD_DAT0_STATUS=2, SD_CMD_STATUS=1, SD_IO_USING_1V8=0x80, SD_IO_USING_3V3=0x7F, TYPE_A_DRIVING=0, TYPE_B_DRIVING=1, TYPE_C_DRIVING=2, TYPE_D_DRIVING=3, SD_CLK_EN=4, MS_CLK_EN=8, SD_MOD_SEL=2, MS_MOD_SEL=3 }

extern "C" { pub fn rtsx_usb_ep0_write_register(ucr: *mut rtsx_ucr, addr: u16, mask: u8, data: u8) -> i32; }
extern "C" {
    pub fn rtsx_usb_get_card_status(ucr:*mut rtsx_ucr,status:*mut u16)->i32;
    pub fn rtsx_usb_read_register(ucr:*mut rtsx_ucr,addr:u16,data:*mut u8)->i32;
    pub fn rtsx_usb_write_register(ucr:*mut rtsx_ucr,addr:u16,mask:u8,data:u8)->i32;
    pub fn rtsx_usb_ep0_read_register(ucr:*mut rtsx_ucr,addr:u16,data:*mut u8)->i32;
    pub fn rtsx_usb_add_cmd(ucr:*mut rtsx_ucr,cmd_type:u8,reg_addr:u16,mask:u8,data:u8);
    pub fn rtsx_usb_send_cmd(ucr:*mut rtsx_ucr,flag:u8,timeout:i32)->i32;
    pub fn rtsx_usb_get_rsp(ucr:*mut rtsx_ucr,rsp_len:i32,timeout:i32)->i32;
    pub fn rtsx_usb_transfer_data(ucr:*mut rtsx_ucr,pipe:u32,buf:*mut core::ffi::c_void,len:u32,use_sg:i32,act_len:*mut u32,timeout:i32)->i32;
    pub fn rtsx_usb_read_ppbuf(ucr:*mut rtsx_ucr,buf:*mut u8,buf_len:i32)->i32;
    pub fn rtsx_usb_write_ppbuf(ucr:*mut rtsx_ucr,buf:*mut u8,buf_len:i32)->i32;
    pub fn rtsx_usb_switch_clock(ucr:*mut rtsx_ucr,card_clock:u32,ssc_depth:u8,initial_mode:bool,double_clk:bool,vpclk:bool)->i32;
    pub fn rtsx_usb_card_exclusive_check(ucr:*mut rtsx_ucr,card:i32)->i32;
}
pub unsafe fn rtsx_usb_cmd_hdr_tag(ucr: *mut rtsx_ucr) { (*ucr).cmd_buf.add(0).write(b'R'); (*ucr).cmd_buf.add(1).write(b'T'); (*ucr).cmd_buf.add(2).write(b'C'); (*ucr).cmd_buf.add(3).write(b'R'); }
pub unsafe fn rtsx_usb_init_cmd(ucr: *mut rtsx_ucr) { rtsx_usb_cmd_hdr_tag(ucr); (*ucr).cmd_idx=0; (*ucr).cmd_buf.add(PACKET_TYPE as usize).write(BATCH_CMD as u8); }
pub unsafe fn rtsx_usb_turn_on_led(ucr:*mut rtsx_ucr)->i32 { rtsx_usb_ep0_write_register(ucr,CARD_GPIO as u16,3,2) }
pub unsafe fn rtsx_usb_turn_off_led(ucr:*mut rtsx_ucr)->i32 { rtsx_usb_ep0_write_register(ucr,CARD_GPIO as u16,3,3) }
pub unsafe fn rtsx_usb_clear_fsm_err(ucr:*mut rtsx_ucr) { let _=rtsx_usb_ep0_write_register(ucr,SFSM_ED as u16,0xf8,0xf8); }
pub unsafe fn rtsx_usb_clear_dma_err(ucr:*mut rtsx_ucr) { let _=rtsx_usb_ep0_write_register(ucr,MC_FIFO_CTL as u16,1,1); let _=rtsx_usb_ep0_write_register(ucr,MC_DMA_RST as u16,1,1); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
