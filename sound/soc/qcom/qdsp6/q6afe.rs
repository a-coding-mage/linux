// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2011-2017, The Linux Foundation. All rights reserved.
// Copyright (c) 2018, Linaro Limited

// Translated from soc/qcom/qdsp6/q6afe.c. C include dependencies are expected
// to be supplied by the surrounding translated kernel crate.

type c_int = i32;
type c_uint = u32;
type c_char = i8;
type c_void = core::ffi::c_void;
type u8_ = u8;
type u16_ = u16;
type u32_ = u32;
type uint16_t = u16;
type uint32_t = u32;

extern "C" {
    static AFE_PORT_MAX: c_int;
    static AFE_PORT_MAX_AUDIO_CHAN_CNT: usize;
    static APR_HDR_SIZE: usize;
    static APR_MSG_TYPE_SEQ_CMD: u32_;
    static APR_PKT_VER: u32_;
    static APR_BASIC_RSP_RESULT: u32_;
    static GFP_KERNEL: u32_;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ETIMEDOUT: c_int;

    static HDMI_RX: c_int;
    static SLIMBUS_0_RX: c_int;
    static SLIMBUS_1_RX: c_int;
    static SLIMBUS_2_RX: c_int;
    static SLIMBUS_3_RX: c_int;
    static SLIMBUS_4_RX: c_int;
    static SLIMBUS_5_RX: c_int;
    static SLIMBUS_6_RX: c_int;
    static SLIMBUS_0_TX: c_int;
    static SLIMBUS_1_TX: c_int;
    static SLIMBUS_2_TX: c_int;
    static SLIMBUS_3_TX: c_int;
    static SLIMBUS_4_TX: c_int;
    static SLIMBUS_5_TX: c_int;
    static SLIMBUS_6_TX: c_int;
    static PRIMARY_MI2S_RX: c_int;
    static PRIMARY_MI2S_TX: c_int;
    static SECONDARY_MI2S_RX: c_int;
    static SECONDARY_MI2S_TX: c_int;
    static TERTIARY_MI2S_RX: c_int;
    static TERTIARY_MI2S_TX: c_int;
    static QUATERNARY_MI2S_RX: c_int;
    static QUATERNARY_MI2S_TX: c_int;
    static QUINARY_MI2S_RX: c_int;
    static QUINARY_MI2S_TX: c_int;
    static SENARY_MI2S_RX: c_int;
    static SENARY_MI2S_TX: c_int;
    static PRIMARY_TDM_RX_0: c_int;
    static PRIMARY_TDM_TX_0: c_int;
    static PRIMARY_TDM_RX_1: c_int;
    static PRIMARY_TDM_TX_1: c_int;
    static PRIMARY_TDM_RX_2: c_int;
    static PRIMARY_TDM_TX_2: c_int;
    static PRIMARY_TDM_RX_3: c_int;
    static PRIMARY_TDM_TX_3: c_int;
    static PRIMARY_TDM_RX_4: c_int;
    static PRIMARY_TDM_TX_4: c_int;
    static PRIMARY_TDM_RX_5: c_int;
    static PRIMARY_TDM_TX_5: c_int;
    static PRIMARY_TDM_RX_6: c_int;
    static PRIMARY_TDM_TX_6: c_int;
    static PRIMARY_TDM_RX_7: c_int;
    static PRIMARY_TDM_TX_7: c_int;
    static SECONDARY_TDM_RX_0: c_int;
    static SECONDARY_TDM_TX_0: c_int;
    static SECONDARY_TDM_RX_1: c_int;
    static SECONDARY_TDM_TX_1: c_int;
    static SECONDARY_TDM_RX_2: c_int;
    static SECONDARY_TDM_TX_2: c_int;
    static SECONDARY_TDM_RX_3: c_int;
    static SECONDARY_TDM_TX_3: c_int;
    static SECONDARY_TDM_RX_4: c_int;
    static SECONDARY_TDM_TX_4: c_int;
    static SECONDARY_TDM_RX_5: c_int;
    static SECONDARY_TDM_TX_5: c_int;
    static SECONDARY_TDM_RX_6: c_int;
    static SECONDARY_TDM_TX_6: c_int;
    static SECONDARY_TDM_RX_7: c_int;
    static SECONDARY_TDM_TX_7: c_int;
    static TERTIARY_TDM_RX_0: c_int;
    static TERTIARY_TDM_TX_0: c_int;
    static TERTIARY_TDM_RX_1: c_int;
    static TERTIARY_TDM_TX_1: c_int;
    static TERTIARY_TDM_RX_2: c_int;
    static TERTIARY_TDM_TX_2: c_int;
    static TERTIARY_TDM_RX_3: c_int;
    static TERTIARY_TDM_TX_3: c_int;
    static TERTIARY_TDM_RX_4: c_int;
    static TERTIARY_TDM_TX_4: c_int;
    static TERTIARY_TDM_RX_5: c_int;
    static TERTIARY_TDM_TX_5: c_int;
    static TERTIARY_TDM_RX_6: c_int;
    static TERTIARY_TDM_TX_6: c_int;
    static TERTIARY_TDM_RX_7: c_int;
    static TERTIARY_TDM_TX_7: c_int;
    static QUATERNARY_TDM_RX_0: c_int;
    static QUATERNARY_TDM_TX_0: c_int;
    static QUATERNARY_TDM_RX_1: c_int;
    static QUATERNARY_TDM_TX_1: c_int;
    static QUATERNARY_TDM_RX_2: c_int;
    static QUATERNARY_TDM_TX_2: c_int;
    static QUATERNARY_TDM_RX_3: c_int;
    static QUATERNARY_TDM_TX_3: c_int;
    static QUATERNARY_TDM_RX_4: c_int;
    static QUATERNARY_TDM_TX_4: c_int;
    static QUATERNARY_TDM_RX_5: c_int;
    static QUATERNARY_TDM_TX_5: c_int;
    static QUATERNARY_TDM_RX_6: c_int;
    static QUATERNARY_TDM_TX_6: c_int;
    static QUATERNARY_TDM_RX_7: c_int;
    static QUATERNARY_TDM_TX_7: c_int;
    static QUINARY_TDM_RX_0: c_int;
    static QUINARY_TDM_TX_0: c_int;
    static QUINARY_TDM_RX_1: c_int;
    static QUINARY_TDM_TX_1: c_int;
    static QUINARY_TDM_RX_2: c_int;
    static QUINARY_TDM_TX_2: c_int;
    static QUINARY_TDM_RX_3: c_int;
    static QUINARY_TDM_TX_3: c_int;
    static QUINARY_TDM_RX_4: c_int;
    static QUINARY_TDM_TX_4: c_int;
    static QUINARY_TDM_RX_5: c_int;
    static QUINARY_TDM_TX_5: c_int;
    static QUINARY_TDM_RX_6: c_int;
    static QUINARY_TDM_TX_6: c_int;
    static QUINARY_TDM_RX_7: c_int;
    static QUINARY_TDM_TX_7: c_int;
    static DISPLAY_PORT_RX: c_int;
    static WSA_CODEC_DMA_RX_0: c_int;
    static WSA_CODEC_DMA_TX_0: c_int;
    static WSA_CODEC_DMA_RX_1: c_int;
    static WSA_CODEC_DMA_TX_1: c_int;
    static WSA_CODEC_DMA_TX_2: c_int;
    static VA_CODEC_DMA_TX_0: c_int;
    static VA_CODEC_DMA_TX_1: c_int;
    static VA_CODEC_DMA_TX_2: c_int;
    static RX_CODEC_DMA_RX_0: c_int;
    static TX_CODEC_DMA_TX_0: c_int;
    static RX_CODEC_DMA_RX_1: c_int;
    static TX_CODEC_DMA_TX_1: c_int;
    static RX_CODEC_DMA_RX_2: c_int;
    static TX_CODEC_DMA_TX_2: c_int;
    static RX_CODEC_DMA_RX_3: c_int;
    static TX_CODEC_DMA_TX_3: c_int;
    static RX_CODEC_DMA_RX_4: c_int;
    static TX_CODEC_DMA_TX_4: c_int;
    static RX_CODEC_DMA_RX_5: c_int;
    static TX_CODEC_DMA_TX_5: c_int;
    static RX_CODEC_DMA_RX_6: c_int;
    static RX_CODEC_DMA_RX_7: c_int;
    static USB_RX: c_int;
    static LPI_MI2S_RX_0: c_int;
    static LPI_MI2S_TX_0: c_int;
    static LPI_MI2S_RX_1: c_int;
    static LPI_MI2S_TX_1: c_int;
    static LPI_MI2S_RX_2: c_int;
    static LPI_MI2S_TX_2: c_int;
    static LPI_MI2S_RX_3: c_int;
    static LPI_MI2S_TX_3: c_int;
    static LPI_MI2S_RX_4: c_int;
    static LPI_MI2S_TX_4: c_int;
    static LPI_MI2S_RX_5: c_int;
    static LPI_MI2S_TX_5: c_int;
    static LPI_MI2S_RX_6: c_int;
    static LPI_MI2S_TX_6: c_int;

    static LPAIF_DIG_CLK: c_int;
    static LPAIF_BIT_CLK: c_int;
    static LPAIF_OSR_CLK: c_int;
    static Q6AFE_LPASS_CLK_ID_PRI_MI2S_IBIT: c_int;
    static Q6AFE_LPASS_CLK_ID_QUI_MI2S_OSR: c_int;
    static Q6AFE_LPASS_CLK_ID_MCLK_1: c_int;
    static Q6AFE_LPASS_CLK_ID_INT_MCLK_1: c_int;
    static Q6AFE_LPASS_CLK_ID_PRI_TDM_IBIT: c_int;
    static Q6AFE_LPASS_CLK_ID_QUIN_TDM_EBIT: c_int;
    static Q6AFE_LPASS_CLK_ID_WSA_CORE_MCLK: c_int;
    static Q6AFE_LPASS_CLK_ID_VA_CORE_2X_MCLK: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_BP_FP: c_uint;
    static SND_SOC_DAIFMT_BC_FC: c_uint;

    fn APR_HDR_FIELD(msg_type: u32_, hdr_len: u32_, ver: u32_) -> u32_;
    fn APR_HDR_LEN(len: usize) -> u32_;
    fn apr_send_pkt(apr: *mut apr_device, pkt: *mut apr_pkt) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32_) -> *mut c_void;
    fn devm_of_platform_populate(dev: *mut device) -> c_int;
    fn q6core_get_svc_api_info(svc_id: c_int, ainfo: *mut q6core_svc_api_info);
    fn kzalloc(size: usize, flags: u32_) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn hweight_long(w: c_uint) -> c_int;
    fn msecs_to_jiffies(m: c_uint) -> usize;
    fn wait_event_timeout(wait: *mut wait_queue_head_t, condition: bool, timeout: usize) -> c_int;
    fn wake_up(wait: *mut wait_queue_head_t);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn init_waitqueue_head(wait: *mut wait_queue_head_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn list_add_tail(node: *mut list_head, head: *mut list_head);
    fn list_del(node: *mut list_head);
    fn kref_init(kref: *mut kref);
    fn kref_get(kref: *mut kref);
    fn kref_put(kref: *mut kref, release: unsafe extern "C" fn(*mut kref)) -> c_int;
    fn ERR_PTR(err: isize) -> *mut q6afe_port;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)] pub struct apr_device { pub dev: device, pub svc_id: c_int }
#[repr(C)] pub struct device { pub parent: *mut device }
#[repr(C)] pub struct q6core_svc_api_info { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct kref { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] #[derive(Copy, Clone)] pub struct aprv2_ibasic_rsp_result_t { pub opcode: u32_, pub status: u32_ }
#[repr(C)] #[derive(Copy, Clone)] pub struct apr_hdr { pub hdr_field: u32_, pub pkt_size: u32_, pub src_port: u16_, pub dest_port: u16_, pub token: u32_, pub opcode: u32_ }
#[repr(C)] pub struct apr_pkt { pub hdr: apr_hdr }
#[repr(C)] pub struct apr_resp_pkt { pub hdr: apr_hdr, pub payload_size: u32_, pub payload: *const aprv2_ibasic_rsp_result_t }

#[repr(C)] pub struct q6afe_slim_cfg { pub sample_rate: u32_, pub bit_width: u16_, pub num_channels: u16_, pub data_format: u16_, pub ch_mapping: [u8_; 4] }
#[repr(C)] pub struct q6afe_tdm_cfg { pub num_channels: u32_, pub sample_rate: u32_, pub bit_width: u32_, pub data_format: u16_, pub sync_mode: u16_, pub sync_src: u16_, pub nslots_per_frame: u16_, pub slot_width: u16_, pub slot_mask: u32_, pub data_align_type: u32_, pub ch_mapping: [u16_; 32] }
#[repr(C)] pub struct q6afe_usb_cfg { pub sample_rate: u32_, pub num_channels: u16_, pub bit_width: u16_ }
#[repr(C)] pub struct q6afe_hdmi_cfg { pub datatype: u16_, pub channel_allocation: u16_, pub sample_rate: u32_, pub bit_width: u16_ }
#[repr(C)] pub struct q6afe_i2s_cfg { pub sample_rate: u32_, pub bit_width: u16_, pub fmt: c_uint, pub sd_line_mask: c_uint, pub num_channels: u16_ }
#[repr(C)] pub struct q6afe_cdc_dma_cfg { pub sample_rate: u32_, pub bit_width: u16_, pub data_format: u16_, pub num_channels: u16_, pub active_channels_mask: u16_ }

/* AFE CMDs */
const AFE_PORT_CMD_DEVICE_START: u32_ = 0x000100E5;
const AFE_PORT_CMD_DEVICE_STOP: u32_ = 0x000100E6;
const AFE_PORT_CMD_SET_PARAM_V2: u32_ = 0x000100EF;
const AFE_SVC_CMD_SET_PARAM: u32_ = 0x000100f3;
const AFE_PORT_CMDRSP_GET_PARAM_V2: u32_ = 0x00010106;
const AFE_PARAM_ID_HDMI_CONFIG: c_int = 0x00010210;
const AFE_MODULE_AUDIO_DEV_INTERFACE: c_int = 0x0001020C;
const AFE_MODULE_TDM: c_int = 0x0001028A;
const AFE_PARAM_ID_CDC_SLIMBUS_SLAVE_CFG: u32_ = 0x00010235;
const AFE_PARAM_ID_USB_AUDIO_DEV_PARAMS: c_int = 0x000102A5;
const AFE_PARAM_ID_USB_AUDIO_DEV_LPCM_FMT: c_int = 0x000102AA;
const AFE_PARAM_ID_LPAIF_CLK_CONFIG: c_int = 0x00010238;
const AFE_PARAM_ID_INT_DIGITAL_CDC_CLK_CONFIG: c_int = 0x00010239;
const AFE_PARAM_ID_SLIMBUS_CONFIG: c_int = 0x00010212;
const AFE_PARAM_ID_I2S_CONFIG: c_int = 0x0001020D;
const AFE_PARAM_ID_TDM_CONFIG: c_int = 0x0001029D;
const AFE_PARAM_ID_PORT_SLOT_MAPPING_CONFIG: c_int = 0x00010297;
const AFE_PARAM_ID_CODEC_DMA_CONFIG: c_int = 0x000102B8;
const AFE_PARAM_ID_USB_AUDIO_CONFIG: c_int = 0x000102A4;
const AFE_CMD_REMOTE_LPASS_CORE_HW_VOTE_REQUEST: u32_ = 0x000100f4;
const AFE_CMD_RSP_REMOTE_LPASS_CORE_HW_VOTE_REQUEST: u32_ = 0x000100f5;
const AFE_CMD_REMOTE_LPASS_CORE_HW_DEVOTE_REQUEST: u32_ = 0x000100f6;

/* I2S config specific */
const AFE_API_VERSION_I2S_CONFIG: u32_ = 0x1;
const AFE_PORT_I2S_SD0: u16_ = 0x1;
const AFE_PORT_I2S_SD1: u16_ = 0x2;
const AFE_PORT_I2S_SD2: u16_ = 0x3;
const AFE_PORT_I2S_SD3: u16_ = 0x4;
const AFE_PORT_I2S_SD0_MASK: c_uint = 1 << 0x0;
const AFE_PORT_I2S_SD1_MASK: c_uint = 1 << 0x1;
const AFE_PORT_I2S_SD2_MASK: c_uint = 1 << 0x2;
const AFE_PORT_I2S_SD3_MASK: c_uint = 1 << 0x3;
const AFE_PORT_I2S_SD0_1_MASK: c_uint = 0x3;
const AFE_PORT_I2S_SD2_3_MASK: c_uint = 0xc;
const AFE_PORT_I2S_SD0_1_2_MASK: c_uint = 0x7;
const AFE_PORT_I2S_SD0_1_2_3_MASK: c_uint = 0xf;
const AFE_PORT_I2S_QUAD01: u16_ = 0x5;
const AFE_PORT_I2S_QUAD23: u16_ = 0x6;
const AFE_PORT_I2S_6CHS: u16_ = 0x7;
const AFE_PORT_I2S_8CHS: u16_ = 0x8;
const AFE_PORT_I2S_MONO: u16_ = 0x0;
const AFE_PORT_I2S_STEREO: u16_ = 0x1;
const AFE_PORT_CONFIG_I2S_WS_SRC_EXTERNAL: u16_ = 0x0;
const AFE_PORT_CONFIG_I2S_WS_SRC_INTERNAL: u16_ = 0x1;
const AFE_LINEAR_PCM_DATA: u16_ = 0x0;

const AFE_API_MINOR_VERSION_USB_AUDIO_CONFIG: u32_ = 0x1;
const AFE_API_VERSION_HDMI_CONFIG: u32_ = 0x1;
const AFE_PORT_ID_MULTICHAN_HDMI_RX: c_int = 0x100E;
const AFE_PORT_ID_HDMI_OVER_DP_RX: c_int = 0x6020;
const AFE_PORT_ID_USB_RX: c_int = 0x7000;
const AFE_API_VERSION_SLIMBUS_CONFIG: u32_ = 0x1;
const AFE_API_VERSION_CLOCK_SET: u32_ = 1;
const Q6AFE_LPASS_CLK_CONFIG_API_VERSION: u32_ = 0x1;
const AFE_MODULE_CLOCK_SET: c_int = 0x0001028F;
const AFE_PARAM_ID_CLOCK_SET: c_int = 0x00010290;

const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_0_RX: c_int = 0x4000;
const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_0_TX: c_int = 0x4001;
const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_1_RX: c_int = 0x4002;
const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_1_TX: c_int = 0x4003;
const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_2_RX: c_int = 0x4004;
const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_2_TX: c_int = 0x4005;
const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_3_RX: c_int = 0x4006;
const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_3_TX: c_int = 0x4007;
const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_4_RX: c_int = 0x4008;
const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_4_TX: c_int = 0x4009;
const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_5_RX: c_int = 0x400a;
const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_5_TX: c_int = 0x400b;
const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_6_RX: c_int = 0x400c;
const AFE_PORT_ID_SLIMBUS_MULTI_CHAN_6_TX: c_int = 0x400d;
const AFE_PORT_ID_PRIMARY_MI2S_RX: c_int = 0x1000;
const AFE_PORT_ID_PRIMARY_MI2S_TX: c_int = 0x1001;
const AFE_PORT_ID_SECONDARY_MI2S_RX: c_int = 0x1002;
const AFE_PORT_ID_SECONDARY_MI2S_TX: c_int = 0x1003;
const AFE_PORT_ID_TERTIARY_MI2S_RX: c_int = 0x1004;
const AFE_PORT_ID_TERTIARY_MI2S_TX: c_int = 0x1005;
const AFE_PORT_ID_QUATERNARY_MI2S_RX: c_int = 0x1006;
const AFE_PORT_ID_QUATERNARY_MI2S_TX: c_int = 0x1007;
const AFE_PORT_ID_QUINARY_MI2S_RX: c_int = 0x1016;
const AFE_PORT_ID_QUINARY_MI2S_TX: c_int = 0x1017;
const AFE_PORT_ID_SENARY_MI2S_RX: c_int = 0x1018;
const AFE_PORT_ID_SENARY_MI2S_TX: c_int = 0x1019;
const AFE_PORT_ID_INT0_MI2S_RX: c_int = 0x102e;
const AFE_PORT_ID_INT0_MI2S_TX: c_int = 0x102f;
const AFE_PORT_ID_INT1_MI2S_RX: c_int = 0x1030;
const AFE_PORT_ID_INT1_MI2S_TX: c_int = 0x1031;
const AFE_PORT_ID_INT2_MI2S_RX: c_int = 0x1032;
const AFE_PORT_ID_INT2_MI2S_TX: c_int = 0x1033;
const AFE_PORT_ID_INT3_MI2S_RX: c_int = 0x1034;
const AFE_PORT_ID_INT3_MI2S_TX: c_int = 0x1035;
const AFE_PORT_ID_INT4_MI2S_RX: c_int = 0x1036;
const AFE_PORT_ID_INT4_MI2S_TX: c_int = 0x1037;
const AFE_PORT_ID_INT5_MI2S_RX: c_int = 0x1038;
const AFE_PORT_ID_INT5_MI2S_TX: c_int = 0x1039;
const AFE_PORT_ID_INT6_MI2S_RX: c_int = 0x103a;
const AFE_PORT_ID_INT6_MI2S_TX: c_int = 0x103b;

/* Start of the range of port IDs for TDM devices. */
const AFE_PORT_ID_TDM_PORT_RANGE_START: c_int = 0x9000;
/* End of the range of port IDs for TDM devices. */
const AFE_PORT_ID_TDM_PORT_RANGE_END: c_int = AFE_PORT_ID_TDM_PORT_RANGE_START + 0x50 - 1;
/* Size of the range of port IDs for TDM ports. */
const AFE_PORT_ID_TDM_PORT_RANGE_SIZE: c_int = AFE_PORT_ID_TDM_PORT_RANGE_END - AFE_PORT_ID_TDM_PORT_RANGE_START + 1;

const AFE_PORT_ID_PRIMARY_TDM_RX: c_int = AFE_PORT_ID_TDM_PORT_RANGE_START + 0x00;
const AFE_PORT_ID_PRIMARY_TDM_RX_1: c_int = AFE_PORT_ID_PRIMARY_TDM_RX + 0x02;
const AFE_PORT_ID_PRIMARY_TDM_RX_2: c_int = AFE_PORT_ID_PRIMARY_TDM_RX + 0x04;
const AFE_PORT_ID_PRIMARY_TDM_RX_3: c_int = AFE_PORT_ID_PRIMARY_TDM_RX + 0x06;
const AFE_PORT_ID_PRIMARY_TDM_RX_4: c_int = AFE_PORT_ID_PRIMARY_TDM_RX + 0x08;
const AFE_PORT_ID_PRIMARY_TDM_RX_5: c_int = AFE_PORT_ID_PRIMARY_TDM_RX + 0x0A;
const AFE_PORT_ID_PRIMARY_TDM_RX_6: c_int = AFE_PORT_ID_PRIMARY_TDM_RX + 0x0C;
const AFE_PORT_ID_PRIMARY_TDM_RX_7: c_int = AFE_PORT_ID_PRIMARY_TDM_RX + 0x0E;
const AFE_PORT_ID_PRIMARY_TDM_TX: c_int = AFE_PORT_ID_TDM_PORT_RANGE_START + 0x01;
const AFE_PORT_ID_PRIMARY_TDM_TX_1: c_int = AFE_PORT_ID_PRIMARY_TDM_TX + 0x02;
const AFE_PORT_ID_PRIMARY_TDM_TX_2: c_int = AFE_PORT_ID_PRIMARY_TDM_TX + 0x04;
const AFE_PORT_ID_PRIMARY_TDM_TX_3: c_int = AFE_PORT_ID_PRIMARY_TDM_TX + 0x06;
const AFE_PORT_ID_PRIMARY_TDM_TX_4: c_int = AFE_PORT_ID_PRIMARY_TDM_TX + 0x08;
const AFE_PORT_ID_PRIMARY_TDM_TX_5: c_int = AFE_PORT_ID_PRIMARY_TDM_TX + 0x0A;
const AFE_PORT_ID_PRIMARY_TDM_TX_6: c_int = AFE_PORT_ID_PRIMARY_TDM_TX + 0x0C;
const AFE_PORT_ID_PRIMARY_TDM_TX_7: c_int = AFE_PORT_ID_PRIMARY_TDM_TX + 0x0E;

const AFE_PORT_ID_SECONDARY_TDM_RX: c_int = AFE_PORT_ID_TDM_PORT_RANGE_START + 0x10;
const AFE_PORT_ID_SECONDARY_TDM_RX_1: c_int = AFE_PORT_ID_SECONDARY_TDM_RX + 0x02;
const AFE_PORT_ID_SECONDARY_TDM_RX_2: c_int = AFE_PORT_ID_SECONDARY_TDM_RX + 0x04;
const AFE_PORT_ID_SECONDARY_TDM_RX_3: c_int = AFE_PORT_ID_SECONDARY_TDM_RX + 0x06;
const AFE_PORT_ID_SECONDARY_TDM_RX_4: c_int = AFE_PORT_ID_SECONDARY_TDM_RX + 0x08;
const AFE_PORT_ID_SECONDARY_TDM_RX_5: c_int = AFE_PORT_ID_SECONDARY_TDM_RX + 0x0A;
const AFE_PORT_ID_SECONDARY_TDM_RX_6: c_int = AFE_PORT_ID_SECONDARY_TDM_RX + 0x0C;
const AFE_PORT_ID_SECONDARY_TDM_RX_7: c_int = AFE_PORT_ID_SECONDARY_TDM_RX + 0x0E;
const AFE_PORT_ID_SECONDARY_TDM_TX: c_int = AFE_PORT_ID_TDM_PORT_RANGE_START + 0x11;
const AFE_PORT_ID_SECONDARY_TDM_TX_1: c_int = AFE_PORT_ID_SECONDARY_TDM_TX + 0x02;
const AFE_PORT_ID_SECONDARY_TDM_TX_2: c_int = AFE_PORT_ID_SECONDARY_TDM_TX + 0x04;
const AFE_PORT_ID_SECONDARY_TDM_TX_3: c_int = AFE_PORT_ID_SECONDARY_TDM_TX + 0x06;
const AFE_PORT_ID_SECONDARY_TDM_TX_4: c_int = AFE_PORT_ID_SECONDARY_TDM_TX + 0x08;
const AFE_PORT_ID_SECONDARY_TDM_TX_5: c_int = AFE_PORT_ID_SECONDARY_TDM_TX + 0x0A;
const AFE_PORT_ID_SECONDARY_TDM_TX_6: c_int = AFE_PORT_ID_SECONDARY_TDM_TX + 0x0C;
const AFE_PORT_ID_SECONDARY_TDM_TX_7: c_int = AFE_PORT_ID_SECONDARY_TDM_TX + 0x0E;

const AFE_PORT_ID_TERTIARY_TDM_RX: c_int = AFE_PORT_ID_TDM_PORT_RANGE_START + 0x20;
const AFE_PORT_ID_TERTIARY_TDM_RX_1: c_int = AFE_PORT_ID_TERTIARY_TDM_RX + 0x02;
const AFE_PORT_ID_TERTIARY_TDM_RX_2: c_int = AFE_PORT_ID_TERTIARY_TDM_RX + 0x04;
const AFE_PORT_ID_TERTIARY_TDM_RX_3: c_int = AFE_PORT_ID_TERTIARY_TDM_RX + 0x06;
const AFE_PORT_ID_TERTIARY_TDM_RX_4: c_int = AFE_PORT_ID_TERTIARY_TDM_RX + 0x08;
const AFE_PORT_ID_TERTIARY_TDM_RX_5: c_int = AFE_PORT_ID_TERTIARY_TDM_RX + 0x0A;
const AFE_PORT_ID_TERTIARY_TDM_RX_6: c_int = AFE_PORT_ID_TERTIARY_TDM_RX + 0x0C;
const AFE_PORT_ID_TERTIARY_TDM_RX_7: c_int = AFE_PORT_ID_TERTIARY_TDM_RX + 0x0E;
const AFE_PORT_ID_TERTIARY_TDM_TX: c_int = AFE_PORT_ID_TDM_PORT_RANGE_START + 0x21;
const AFE_PORT_ID_TERTIARY_TDM_TX_1: c_int = AFE_PORT_ID_TERTIARY_TDM_TX + 0x02;
const AFE_PORT_ID_TERTIARY_TDM_TX_2: c_int = AFE_PORT_ID_TERTIARY_TDM_TX + 0x04;
const AFE_PORT_ID_TERTIARY_TDM_TX_3: c_int = AFE_PORT_ID_TERTIARY_TDM_TX + 0x06;
const AFE_PORT_ID_TERTIARY_TDM_TX_4: c_int = AFE_PORT_ID_TERTIARY_TDM_TX + 0x08;
const AFE_PORT_ID_TERTIARY_TDM_TX_5: c_int = AFE_PORT_ID_TERTIARY_TDM_TX + 0x0A;
const AFE_PORT_ID_TERTIARY_TDM_TX_6: c_int = AFE_PORT_ID_TERTIARY_TDM_TX + 0x0C;
const AFE_PORT_ID_TERTIARY_TDM_TX_7: c_int = AFE_PORT_ID_TERTIARY_TDM_TX + 0x0E;

const AFE_PORT_ID_QUATERNARY_TDM_RX: c_int = AFE_PORT_ID_TDM_PORT_RANGE_START + 0x30;
const AFE_PORT_ID_QUATERNARY_TDM_RX_1: c_int = AFE_PORT_ID_QUATERNARY_TDM_RX + 0x02;
const AFE_PORT_ID_QUATERNARY_TDM_RX_2: c_int = AFE_PORT_ID_QUATERNARY_TDM_RX + 0x04;
const AFE_PORT_ID_QUATERNARY_TDM_RX_3: c_int = AFE_PORT_ID_QUATERNARY_TDM_RX + 0x06;
const AFE_PORT_ID_QUATERNARY_TDM_RX_4: c_int = AFE_PORT_ID_QUATERNARY_TDM_RX + 0x08;
const AFE_PORT_ID_QUATERNARY_TDM_RX_5: c_int = AFE_PORT_ID_QUATERNARY_TDM_RX + 0x0A;
const AFE_PORT_ID_QUATERNARY_TDM_RX_6: c_int = AFE_PORT_ID_QUATERNARY_TDM_RX + 0x0C;
const AFE_PORT_ID_QUATERNARY_TDM_RX_7: c_int = AFE_PORT_ID_QUATERNARY_TDM_RX + 0x0E;
const AFE_PORT_ID_QUATERNARY_TDM_TX: c_int = AFE_PORT_ID_TDM_PORT_RANGE_START + 0x31;
const AFE_PORT_ID_QUATERNARY_TDM_TX_1: c_int = AFE_PORT_ID_QUATERNARY_TDM_TX + 0x02;
const AFE_PORT_ID_QUATERNARY_TDM_TX_2: c_int = AFE_PORT_ID_QUATERNARY_TDM_TX + 0x04;
const AFE_PORT_ID_QUATERNARY_TDM_TX_3: c_int = AFE_PORT_ID_QUATERNARY_TDM_TX + 0x06;
const AFE_PORT_ID_QUATERNARY_TDM_TX_4: c_int = AFE_PORT_ID_QUATERNARY_TDM_TX + 0x08;
const AFE_PORT_ID_QUATERNARY_TDM_TX_5: c_int = AFE_PORT_ID_QUATERNARY_TDM_TX + 0x0A;
const AFE_PORT_ID_QUATERNARY_TDM_TX_6: c_int = AFE_PORT_ID_QUATERNARY_TDM_TX + 0x0C;
const AFE_PORT_ID_QUATERNARY_TDM_TX_7: c_int = AFE_PORT_ID_QUATERNARY_TDM_TX + 0x0E;

const AFE_PORT_ID_QUINARY_TDM_RX: c_int = AFE_PORT_ID_TDM_PORT_RANGE_START + 0x40;
const AFE_PORT_ID_QUINARY_TDM_RX_1: c_int = AFE_PORT_ID_QUINARY_TDM_RX + 0x02;
const AFE_PORT_ID_QUINARY_TDM_RX_2: c_int = AFE_PORT_ID_QUINARY_TDM_RX + 0x04;
const AFE_PORT_ID_QUINARY_TDM_RX_3: c_int = AFE_PORT_ID_QUINARY_TDM_RX + 0x06;
const AFE_PORT_ID_QUINARY_TDM_RX_4: c_int = AFE_PORT_ID_QUINARY_TDM_RX + 0x08;
const AFE_PORT_ID_QUINARY_TDM_RX_5: c_int = AFE_PORT_ID_QUINARY_TDM_RX + 0x0A;
const AFE_PORT_ID_QUINARY_TDM_RX_6: c_int = AFE_PORT_ID_QUINARY_TDM_RX + 0x0C;
const AFE_PORT_ID_QUINARY_TDM_RX_7: c_int = AFE_PORT_ID_QUINARY_TDM_RX + 0x0E;
const AFE_PORT_ID_QUINARY_TDM_TX: c_int = AFE_PORT_ID_TDM_PORT_RANGE_START + 0x41;
const AFE_PORT_ID_QUINARY_TDM_TX_1: c_int = AFE_PORT_ID_QUINARY_TDM_TX + 0x02;
const AFE_PORT_ID_QUINARY_TDM_TX_2: c_int = AFE_PORT_ID_QUINARY_TDM_TX + 0x04;
const AFE_PORT_ID_QUINARY_TDM_TX_3: c_int = AFE_PORT_ID_QUINARY_TDM_TX + 0x06;
const AFE_PORT_ID_QUINARY_TDM_TX_4: c_int = AFE_PORT_ID_QUINARY_TDM_TX + 0x08;
const AFE_PORT_ID_QUINARY_TDM_TX_5: c_int = AFE_PORT_ID_QUINARY_TDM_TX + 0x0A;
const AFE_PORT_ID_QUINARY_TDM_TX_6: c_int = AFE_PORT_ID_QUINARY_TDM_TX + 0x0C;
const AFE_PORT_ID_QUINARY_TDM_TX_7: c_int = AFE_PORT_ID_QUINARY_TDM_TX + 0x0E;

/* AFE WSA/VA/RX/TX Codec DMA ports. */
const AFE_PORT_ID_WSA_CODEC_DMA_RX_0: c_int = 0xB000;
const AFE_PORT_ID_WSA_CODEC_DMA_TX_0: c_int = 0xB001;
const AFE_PORT_ID_WSA_CODEC_DMA_RX_1: c_int = 0xB002;
const AFE_PORT_ID_WSA_CODEC_DMA_TX_1: c_int = 0xB003;
const AFE_PORT_ID_WSA_CODEC_DMA_TX_2: c_int = 0xB005;
const AFE_PORT_ID_VA_CODEC_DMA_TX_0: c_int = 0xB021;
const AFE_PORT_ID_VA_CODEC_DMA_TX_1: c_int = 0xB023;
const AFE_PORT_ID_VA_CODEC_DMA_TX_2: c_int = 0xB025;
const AFE_PORT_ID_RX_CODEC_DMA_RX_0: c_int = 0xB030;
const AFE_PORT_ID_TX_CODEC_DMA_TX_0: c_int = 0xB031;
const AFE_PORT_ID_RX_CODEC_DMA_RX_1: c_int = 0xB032;
const AFE_PORT_ID_TX_CODEC_DMA_TX_1: c_int = 0xB033;
const AFE_PORT_ID_RX_CODEC_DMA_RX_2: c_int = 0xB034;
const AFE_PORT_ID_TX_CODEC_DMA_TX_2: c_int = 0xB035;
const AFE_PORT_ID_RX_CODEC_DMA_RX_3: c_int = 0xB036;
const AFE_PORT_ID_TX_CODEC_DMA_TX_3: c_int = 0xB037;
const AFE_PORT_ID_RX_CODEC_DMA_RX_4: c_int = 0xB038;
const AFE_PORT_ID_TX_CODEC_DMA_TX_4: c_int = 0xB039;
const AFE_PORT_ID_RX_CODEC_DMA_RX_5: c_int = 0xB03A;
const AFE_PORT_ID_TX_CODEC_DMA_TX_5: c_int = 0xB03B;
const AFE_PORT_ID_RX_CODEC_DMA_RX_6: c_int = 0xB03C;
const AFE_PORT_ID_RX_CODEC_DMA_RX_7: c_int = 0xB03E;

const Q6AFE_LPASS_MODE_CLK1_VALID: u16_ = 1;
const Q6AFE_LPASS_MODE_CLK2_VALID: u16_ = 2;
const Q6AFE_LPASS_CLK_SRC_INTERNAL: u16_ = 1;
const Q6AFE_LPASS_CLK_ROOT_DEFAULT: u16_ = 0;
const AFE_API_VERSION_TDM_CONFIG: u32_ = 1;
const AFE_API_VERSION_SLOT_MAPPING_CONFIG: u32_ = 1;
const AFE_API_VERSION_CODEC_DMA_CONFIG: u32_ = 1;
const TIMEOUT_MS: c_uint = 3000;
const AFE_CMD_RESP_AVAIL: c_int = 0;
const AFE_CMD_RESP_NONE: c_int = 1;
const AFE_CLK_TOKEN: u32_ = 1024;
const AFE_PARAM_ID_USB_AUDIO_SVC_INTERVAL: c_int = 0x000102B7;

#[repr(C)]
pub struct q6afe {
    apr: *mut apr_device,
    dev: *mut device,
    ainfo: q6core_svc_api_info,
    lock: mutex,
    result: aprv2_ibasic_rsp_result_t,
    wait: wait_queue_head_t,
    port_list: list_head,
    port_list_lock: spinlock_t,
}

#[repr(C, packed)] pub struct afe_port_cmd_device_start { port_id: u16_, reserved: u16_ }
#[repr(C, packed)] pub struct afe_port_cmd_device_stop { port_id: u16_, reserved: u16_ /* Reserved for 32-bit alignment. This field must be set to 0. */ }
#[repr(C, packed)] pub struct afe_port_param_data_v2 { module_id: u32_, param_id: u32_, param_size: u16_, reserved: u16_ }
#[repr(C, packed)] pub struct afe_svc_cmd_set_param { payload_size: uint32_t, payload_address_lsw: uint32_t, payload_address_msw: uint32_t, mem_map_handle: uint32_t }
#[repr(C, packed)] pub struct afe_port_cmd_set_param_v2 { port_id: u16_, payload_size: u16_, payload_address_lsw: u32_, payload_address_msw: u32_, mem_map_handle: u32_ }
#[repr(C, packed)] pub struct afe_param_id_hdmi_multi_chan_audio_cfg { hdmi_cfg_minor_version: u32_, datatype: u16_, channel_allocation: u16_, sample_rate: u32_, bit_width: u16_, reserved: u16_ }
#[repr(C, packed)] pub struct afe_param_id_slimbus_cfg { sb_cfg_minor_version: u32_, slimbus_dev_id: u16_, bit_width: u16_, data_format: u16_, num_channels: u16_, shared_ch_mapping: [u8_; 32], sample_rate: u32_ }
#[repr(C, packed)] pub struct afe_clk_cfg { i2s_cfg_minor_version: u32_, clk_val1: u32_, clk_val2: u32_, clk_src: u16_, clk_root: u16_, clk_set_mode: u16_, reserved: u16_ }
#[repr(C, packed)] pub struct afe_digital_clk_cfg { i2s_cfg_minor_version: u32_, clk_val: u32_, clk_root: u16_, reserved: u16_ }
#[repr(C, packed)] pub struct afe_param_id_i2s_cfg { i2s_cfg_minor_version: u32_, bit_width: u16_, channel_mode: u16_, mono_stereo: u16_, ws_src: u16_, sample_rate: u32_, data_format: u16_, reserved: u16_ }
#[repr(C, packed)] pub struct afe_param_id_tdm_cfg { tdm_cfg_minor_version: u32_, num_channels: u32_, sample_rate: u32_, bit_width: u32_, data_format: u16_, sync_mode: u16_, sync_src: u16_, nslots_per_frame: u16_, ctrl_data_out_enable: u16_, ctrl_invert_sync_pulse: u16_, ctrl_sync_data_delay: u16_, slot_width: u16_, slot_mask: u32_ }
#[repr(C, packed)] pub struct afe_param_id_cdc_dma_cfg { cdc_dma_cfg_minor_version: u32_, sample_rate: u32_, bit_width: u16_, data_format: u16_, num_channels: u16_, active_channels_mask: u16_ }
#[repr(C, packed)] pub struct afe_param_id_usb_cfg { cfg_minor_version: u32_, sample_rate: u32_, bit_width: u16_, num_channels: u16_, data_format: u16_, reserved: u16_, dev_token: u32_, endian: u32_, service_interval: u32_ }
#[repr(C, packed)] pub struct afe_param_id_usb_audio_dev_params { cfg_minor_version: u32_, dev_token: u32_ }
#[repr(C, packed)] pub struct afe_param_id_usb_audio_dev_lpcm_fmt { cfg_minor_version: u32_, endian: u32_ }
#[repr(C, packed)] pub struct afe_param_id_usb_audio_svc_interval { cfg_minor_version: u32_, svc_interval: u32_ }

#[repr(C)]
pub union afe_port_config {
    hdmi_multi_ch: core::mem::ManuallyDrop<afe_param_id_hdmi_multi_chan_audio_cfg>,
    slim_cfg: core::mem::ManuallyDrop<afe_param_id_slimbus_cfg>,
    i2s_cfg: core::mem::ManuallyDrop<afe_param_id_i2s_cfg>,
    tdm_cfg: core::mem::ManuallyDrop<afe_param_id_tdm_cfg>,
    dma_cfg: core::mem::ManuallyDrop<afe_param_id_cdc_dma_cfg>,
    usb_cfg: core::mem::ManuallyDrop<afe_param_id_usb_cfg>,
}

#[repr(C)] pub struct afe_clk_set { clk_set_minor_version: uint32_t, clk_id: uint32_t, clk_freq_in_hz: uint32_t, clk_attri: uint16_t, clk_root: uint16_t, enable: uint32_t }
#[repr(C, packed)] pub struct afe_param_id_slot_mapping_cfg { minor_version: u32_, num_channels: u16_, bitwidth: u16_, data_align_type: u32_, ch_mapping: [u16_; 32] }

#[repr(C)]
pub struct q6afe_port {
    wait: wait_queue_head_t,
    port_cfg: afe_port_config,
    scfg: *mut afe_param_id_slot_mapping_cfg,
    result: aprv2_ibasic_rsp_result_t,
    token: c_int,
    id: c_int,
    cfg_type: c_int,
    afe: *mut q6afe,
    refcount: kref,
    node: list_head,
}

#[repr(C, packed)] pub struct afe_cmd_remote_lpass_core_hw_vote_request { hw_block_id: uint32_t, client_name: [c_char; 8] }
#[repr(C, packed)] pub struct afe_cmd_remote_lpass_core_hw_devote_request { hw_block_id: uint32_t, client_handle: uint32_t }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afe_port_map { port_id: c_int, token: c_int, is_rx: c_int, is_dig_pcm: c_int }

/*
 * Mapping between Virtual Port IDs to DSP AFE Port ID.
 * This static is filled with the same designated-initializer entries as the C
 * source by q6afe_init_port_maps(), because the indexes come from external
 * dt-binding constants in this isolated translation.
 */
static mut port_maps: *mut afe_port_map = core::ptr::null_mut();

unsafe fn set_port_map(index: c_int, port_id: c_int, token: c_int, is_rx: c_int, is_dig_pcm: c_int) {
    *port_maps.add(index as usize) = afe_port_map { port_id, token, is_rx, is_dig_pcm };
}

unsafe fn q6afe_init_port_maps() {
    if !port_maps.is_null() { return; }
    port_maps = kzalloc((AFE_PORT_MAX as usize) * core::mem::size_of::<afe_port_map>(), GFP_KERNEL) as *mut afe_port_map;
    if port_maps.is_null() { return; }
    set_port_map(HDMI_RX, AFE_PORT_ID_MULTICHAN_HDMI_RX, HDMI_RX, 1, 1);
    set_port_map(SLIMBUS_0_RX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_0_RX, SLIMBUS_0_RX, 1, 1);
    set_port_map(SLIMBUS_1_RX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_1_RX, SLIMBUS_1_RX, 1, 1);
    set_port_map(SLIMBUS_2_RX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_2_RX, SLIMBUS_2_RX, 1, 1);
    set_port_map(SLIMBUS_3_RX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_3_RX, SLIMBUS_3_RX, 1, 1);
    set_port_map(SLIMBUS_4_RX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_4_RX, SLIMBUS_4_RX, 1, 1);
    set_port_map(SLIMBUS_5_RX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_5_RX, SLIMBUS_5_RX, 1, 1);
    set_port_map(SLIMBUS_6_RX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_6_RX, SLIMBUS_6_RX, 1, 1);
    set_port_map(SLIMBUS_0_TX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_0_TX, SLIMBUS_0_TX, 0, 1);
    set_port_map(SLIMBUS_1_TX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_1_TX, SLIMBUS_1_TX, 0, 1);
    set_port_map(SLIMBUS_2_TX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_2_TX, SLIMBUS_2_TX, 0, 1);
    set_port_map(SLIMBUS_3_TX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_3_TX, SLIMBUS_3_TX, 0, 1);
    set_port_map(SLIMBUS_4_TX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_4_TX, SLIMBUS_4_TX, 0, 1);
    set_port_map(SLIMBUS_5_TX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_5_TX, SLIMBUS_5_TX, 0, 1);
    set_port_map(SLIMBUS_6_TX, AFE_PORT_ID_SLIMBUS_MULTI_CHAN_6_TX, SLIMBUS_6_TX, 0, 1);
    set_port_map(PRIMARY_MI2S_RX, AFE_PORT_ID_PRIMARY_MI2S_RX, PRIMARY_MI2S_RX, 1, 1);
    set_port_map(PRIMARY_MI2S_TX, AFE_PORT_ID_PRIMARY_MI2S_TX, PRIMARY_MI2S_RX, 0, 1);
    set_port_map(SECONDARY_MI2S_RX, AFE_PORT_ID_SECONDARY_MI2S_RX, SECONDARY_MI2S_RX, 1, 1);
    set_port_map(SECONDARY_MI2S_TX, AFE_PORT_ID_SECONDARY_MI2S_TX, SECONDARY_MI2S_TX, 0, 1);
    set_port_map(TERTIARY_MI2S_RX, AFE_PORT_ID_TERTIARY_MI2S_RX, TERTIARY_MI2S_RX, 1, 1);
    set_port_map(TERTIARY_MI2S_TX, AFE_PORT_ID_TERTIARY_MI2S_TX, TERTIARY_MI2S_TX, 0, 1);
    set_port_map(QUATERNARY_MI2S_RX, AFE_PORT_ID_QUATERNARY_MI2S_RX, QUATERNARY_MI2S_RX, 1, 1);
    set_port_map(QUATERNARY_MI2S_TX, AFE_PORT_ID_QUATERNARY_MI2S_TX, QUATERNARY_MI2S_TX, 0, 1);
    set_port_map(QUINARY_MI2S_RX, AFE_PORT_ID_QUINARY_MI2S_RX, QUINARY_MI2S_RX, 1, 1);
    set_port_map(QUINARY_MI2S_TX, AFE_PORT_ID_QUINARY_MI2S_TX, QUINARY_MI2S_TX, 0, 1);
    set_port_map(SENARY_MI2S_RX, AFE_PORT_ID_SENARY_MI2S_RX, SENARY_MI2S_RX, 1, 1);
    set_port_map(SENARY_MI2S_TX, AFE_PORT_ID_SENARY_MI2S_TX, SENARY_MI2S_TX, 0, 1);
    let tdm = [
        (PRIMARY_TDM_RX_0, AFE_PORT_ID_PRIMARY_TDM_RX, PRIMARY_TDM_RX_0, 1), (PRIMARY_TDM_TX_0, AFE_PORT_ID_PRIMARY_TDM_TX, PRIMARY_TDM_TX_0, 0),
        (PRIMARY_TDM_RX_1, AFE_PORT_ID_PRIMARY_TDM_RX_1, PRIMARY_TDM_RX_1, 1), (PRIMARY_TDM_TX_1, AFE_PORT_ID_PRIMARY_TDM_TX_1, PRIMARY_TDM_TX_1, 0),
        (PRIMARY_TDM_RX_2, AFE_PORT_ID_PRIMARY_TDM_RX_2, PRIMARY_TDM_RX_2, 1), (PRIMARY_TDM_TX_2, AFE_PORT_ID_PRIMARY_TDM_TX_2, PRIMARY_TDM_TX_2, 0),
        (PRIMARY_TDM_RX_3, AFE_PORT_ID_PRIMARY_TDM_RX_3, PRIMARY_TDM_RX_3, 1), (PRIMARY_TDM_TX_3, AFE_PORT_ID_PRIMARY_TDM_TX_3, PRIMARY_TDM_TX_3, 0),
        (PRIMARY_TDM_RX_4, AFE_PORT_ID_PRIMARY_TDM_RX_4, PRIMARY_TDM_RX_4, 1), (PRIMARY_TDM_TX_4, AFE_PORT_ID_PRIMARY_TDM_TX_4, PRIMARY_TDM_TX_4, 0),
        (PRIMARY_TDM_RX_5, AFE_PORT_ID_PRIMARY_TDM_RX_5, PRIMARY_TDM_RX_5, 1), (PRIMARY_TDM_TX_5, AFE_PORT_ID_PRIMARY_TDM_TX_5, PRIMARY_TDM_TX_5, 0),
        (PRIMARY_TDM_RX_6, AFE_PORT_ID_PRIMARY_TDM_RX_6, PRIMARY_TDM_RX_6, 1), (PRIMARY_TDM_TX_6, AFE_PORT_ID_PRIMARY_TDM_TX_6, PRIMARY_TDM_TX_6, 0),
        (PRIMARY_TDM_RX_7, AFE_PORT_ID_PRIMARY_TDM_RX_7, PRIMARY_TDM_RX_7, 1), (PRIMARY_TDM_TX_7, AFE_PORT_ID_PRIMARY_TDM_TX_7, PRIMARY_TDM_TX_7, 0),
        (SECONDARY_TDM_RX_0, AFE_PORT_ID_SECONDARY_TDM_RX, SECONDARY_TDM_RX_0, 1), (SECONDARY_TDM_TX_0, AFE_PORT_ID_SECONDARY_TDM_TX, SECONDARY_TDM_TX_0, 0),
        (SECONDARY_TDM_RX_1, AFE_PORT_ID_SECONDARY_TDM_RX_1, SECONDARY_TDM_RX_1, 1), (SECONDARY_TDM_TX_1, AFE_PORT_ID_SECONDARY_TDM_TX_1, SECONDARY_TDM_TX_1, 0),
        (SECONDARY_TDM_RX_2, AFE_PORT_ID_SECONDARY_TDM_RX_2, SECONDARY_TDM_RX_2, 1), (SECONDARY_TDM_TX_2, AFE_PORT_ID_SECONDARY_TDM_TX_2, SECONDARY_TDM_TX_2, 0),
        (SECONDARY_TDM_RX_3, AFE_PORT_ID_SECONDARY_TDM_RX_3, SECONDARY_TDM_RX_3, 1), (SECONDARY_TDM_TX_3, AFE_PORT_ID_SECONDARY_TDM_TX_3, SECONDARY_TDM_TX_3, 0),
        (SECONDARY_TDM_RX_4, AFE_PORT_ID_SECONDARY_TDM_RX_4, SECONDARY_TDM_RX_4, 1), (SECONDARY_TDM_TX_4, AFE_PORT_ID_SECONDARY_TDM_TX_4, SECONDARY_TDM_TX_4, 0),
        (SECONDARY_TDM_RX_5, AFE_PORT_ID_SECONDARY_TDM_RX_5, SECONDARY_TDM_RX_5, 1), (SECONDARY_TDM_TX_5, AFE_PORT_ID_SECONDARY_TDM_TX_5, SECONDARY_TDM_TX_5, 0),
        (SECONDARY_TDM_RX_6, AFE_PORT_ID_SECONDARY_TDM_RX_6, SECONDARY_TDM_RX_6, 1), (SECONDARY_TDM_TX_6, AFE_PORT_ID_SECONDARY_TDM_TX_6, SECONDARY_TDM_TX_6, 0),
        (SECONDARY_TDM_RX_7, AFE_PORT_ID_SECONDARY_TDM_RX_7, SECONDARY_TDM_RX_7, 1), (SECONDARY_TDM_TX_7, AFE_PORT_ID_SECONDARY_TDM_TX_7, SECONDARY_TDM_TX_7, 0),
        (TERTIARY_TDM_RX_0, AFE_PORT_ID_TERTIARY_TDM_RX, TERTIARY_TDM_RX_0, 1), (TERTIARY_TDM_TX_0, AFE_PORT_ID_TERTIARY_TDM_TX, TERTIARY_TDM_TX_0, 0),
        (TERTIARY_TDM_RX_1, AFE_PORT_ID_TERTIARY_TDM_RX_1, TERTIARY_TDM_RX_1, 1), (TERTIARY_TDM_TX_1, AFE_PORT_ID_TERTIARY_TDM_TX_1, TERTIARY_TDM_TX_1, 0),
        (TERTIARY_TDM_RX_2, AFE_PORT_ID_TERTIARY_TDM_RX_2, TERTIARY_TDM_RX_2, 1), (TERTIARY_TDM_TX_2, AFE_PORT_ID_TERTIARY_TDM_TX_2, TERTIARY_TDM_TX_2, 0),
        (TERTIARY_TDM_RX_3, AFE_PORT_ID_TERTIARY_TDM_RX_3, TERTIARY_TDM_RX_3, 1), (TERTIARY_TDM_TX_3, AFE_PORT_ID_TERTIARY_TDM_TX_3, TERTIARY_TDM_TX_3, 0),
        (TERTIARY_TDM_RX_4, AFE_PORT_ID_TERTIARY_TDM_RX_4, TERTIARY_TDM_RX_4, 1), (TERTIARY_TDM_TX_4, AFE_PORT_ID_TERTIARY_TDM_TX_4, TERTIARY_TDM_TX_4, 0),
        (TERTIARY_TDM_RX_5, AFE_PORT_ID_TERTIARY_TDM_RX_5, TERTIARY_TDM_RX_5, 1), (TERTIARY_TDM_TX_5, AFE_PORT_ID_TERTIARY_TDM_TX_5, TERTIARY_TDM_TX_5, 0),
        (TERTIARY_TDM_RX_6, AFE_PORT_ID_TERTIARY_TDM_RX_6, TERTIARY_TDM_RX_6, 1), (TERTIARY_TDM_TX_6, AFE_PORT_ID_TERTIARY_TDM_TX_6, TERTIARY_TDM_TX_6, 0),
        (TERTIARY_TDM_RX_7, AFE_PORT_ID_TERTIARY_TDM_RX_7, TERTIARY_TDM_RX_7, 1), (TERTIARY_TDM_TX_7, AFE_PORT_ID_TERTIARY_TDM_TX_7, TERTIARY_TDM_TX_7, 0),
        (QUATERNARY_TDM_RX_0, AFE_PORT_ID_QUATERNARY_TDM_RX, QUATERNARY_TDM_RX_0, 1), (QUATERNARY_TDM_TX_0, AFE_PORT_ID_QUATERNARY_TDM_TX, QUATERNARY_TDM_TX_0, 0),
        (QUATERNARY_TDM_RX_1, AFE_PORT_ID_QUATERNARY_TDM_RX_1, QUATERNARY_TDM_RX_1, 1), (QUATERNARY_TDM_TX_1, AFE_PORT_ID_QUATERNARY_TDM_TX_1, QUATERNARY_TDM_TX_1, 0),
        (QUATERNARY_TDM_RX_2, AFE_PORT_ID_QUATERNARY_TDM_RX_2, QUATERNARY_TDM_RX_2, 1), (QUATERNARY_TDM_TX_2, AFE_PORT_ID_QUATERNARY_TDM_TX_2, QUATERNARY_TDM_TX_2, 0),
        (QUATERNARY_TDM_RX_3, AFE_PORT_ID_QUATERNARY_TDM_RX_3, QUATERNARY_TDM_RX_3, 1), (QUATERNARY_TDM_TX_3, AFE_PORT_ID_QUATERNARY_TDM_TX_3, QUATERNARY_TDM_TX_3, 0),
        (QUATERNARY_TDM_RX_4, AFE_PORT_ID_QUATERNARY_TDM_RX_4, QUATERNARY_TDM_RX_4, 1), (QUATERNARY_TDM_TX_4, AFE_PORT_ID_QUATERNARY_TDM_TX_4, QUATERNARY_TDM_TX_4, 0),
        (QUATERNARY_TDM_RX_5, AFE_PORT_ID_QUATERNARY_TDM_RX_5, QUATERNARY_TDM_RX_5, 1), (QUATERNARY_TDM_TX_5, AFE_PORT_ID_QUATERNARY_TDM_TX_5, QUATERNARY_TDM_TX_5, 0),
        (QUATERNARY_TDM_RX_6, AFE_PORT_ID_QUATERNARY_TDM_RX_6, QUATERNARY_TDM_RX_6, 1), (QUATERNARY_TDM_TX_6, AFE_PORT_ID_QUATERNARY_TDM_TX_6, QUATERNARY_TDM_TX_6, 0),
        (QUATERNARY_TDM_RX_7, AFE_PORT_ID_QUATERNARY_TDM_RX_7, QUATERNARY_TDM_RX_7, 1), (QUATERNARY_TDM_TX_7, AFE_PORT_ID_QUATERNARY_TDM_TX_7, QUATERNARY_TDM_TX_7, 0),
        (QUINARY_TDM_RX_0, AFE_PORT_ID_QUINARY_TDM_RX, QUINARY_TDM_RX_0, 1), (QUINARY_TDM_TX_0, AFE_PORT_ID_QUINARY_TDM_TX, QUINARY_TDM_TX_0, 0),
        (QUINARY_TDM_RX_1, AFE_PORT_ID_QUINARY_TDM_RX_1, QUINARY_TDM_RX_1, 1), (QUINARY_TDM_TX_1, AFE_PORT_ID_QUINARY_TDM_TX_1, QUINARY_TDM_TX_1, 0),
        (QUINARY_TDM_RX_2, AFE_PORT_ID_QUINARY_TDM_RX_2, QUINARY_TDM_RX_2, 1), (QUINARY_TDM_TX_2, AFE_PORT_ID_QUINARY_TDM_TX_2, QUINARY_TDM_TX_2, 0),
        (QUINARY_TDM_RX_3, AFE_PORT_ID_QUINARY_TDM_RX_3, QUINARY_TDM_RX_3, 1), (QUINARY_TDM_TX_3, AFE_PORT_ID_QUINARY_TDM_TX_3, QUINARY_TDM_TX_3, 0),
        (QUINARY_TDM_RX_4, AFE_PORT_ID_QUINARY_TDM_RX_4, QUINARY_TDM_RX_4, 1), (QUINARY_TDM_TX_4, AFE_PORT_ID_QUINARY_TDM_TX_4, QUINARY_TDM_TX_4, 0),
        (QUINARY_TDM_RX_5, AFE_PORT_ID_QUINARY_TDM_RX_5, QUINARY_TDM_RX_5, 1), (QUINARY_TDM_TX_5, AFE_PORT_ID_QUINARY_TDM_TX_5, QUINARY_TDM_TX_5, 0),
        (QUINARY_TDM_RX_6, AFE_PORT_ID_QUINARY_TDM_RX_6, QUINARY_TDM_RX_6, 1), (QUINARY_TDM_TX_6, AFE_PORT_ID_QUINARY_TDM_TX_6, QUINARY_TDM_TX_6, 0),
        (QUINARY_TDM_RX_7, AFE_PORT_ID_QUINARY_TDM_RX_7, QUINARY_TDM_RX_7, 1), (QUINARY_TDM_TX_7, AFE_PORT_ID_QUINARY_TDM_TX_7, QUINARY_TDM_TX_7, 0),
    ];
    for &(idx, pid, tok, rx) in &tdm { set_port_map(idx, pid, tok, rx, 1); }
    set_port_map(DISPLAY_PORT_RX, AFE_PORT_ID_HDMI_OVER_DP_RX, DISPLAY_PORT_RX, 1, 1);
    set_port_map(WSA_CODEC_DMA_RX_0, AFE_PORT_ID_WSA_CODEC_DMA_RX_0, WSA_CODEC_DMA_RX_0, 1, 1);
    set_port_map(WSA_CODEC_DMA_TX_0, AFE_PORT_ID_WSA_CODEC_DMA_TX_0, WSA_CODEC_DMA_TX_0, 0, 1);
    set_port_map(WSA_CODEC_DMA_RX_1, AFE_PORT_ID_WSA_CODEC_DMA_RX_1, WSA_CODEC_DMA_RX_1, 1, 1);
    set_port_map(WSA_CODEC_DMA_TX_1, AFE_PORT_ID_WSA_CODEC_DMA_TX_1, WSA_CODEC_DMA_TX_1, 0, 1);
    set_port_map(WSA_CODEC_DMA_TX_2, AFE_PORT_ID_WSA_CODEC_DMA_TX_2, WSA_CODEC_DMA_TX_2, 0, 1);
    set_port_map(VA_CODEC_DMA_TX_0, AFE_PORT_ID_VA_CODEC_DMA_TX_0, VA_CODEC_DMA_TX_0, 0, 1);
    set_port_map(VA_CODEC_DMA_TX_1, AFE_PORT_ID_VA_CODEC_DMA_TX_1, VA_CODEC_DMA_TX_1, 0, 1);
    set_port_map(VA_CODEC_DMA_TX_2, AFE_PORT_ID_VA_CODEC_DMA_TX_2, VA_CODEC_DMA_TX_2, 0, 1);
    set_port_map(RX_CODEC_DMA_RX_0, AFE_PORT_ID_RX_CODEC_DMA_RX_0, RX_CODEC_DMA_RX_0, 1, 1);
    set_port_map(TX_CODEC_DMA_TX_0, AFE_PORT_ID_TX_CODEC_DMA_TX_0, TX_CODEC_DMA_TX_0, 0, 1);
    set_port_map(RX_CODEC_DMA_RX_1, AFE_PORT_ID_RX_CODEC_DMA_RX_1, RX_CODEC_DMA_RX_1, 1, 1);
    set_port_map(TX_CODEC_DMA_TX_1, AFE_PORT_ID_TX_CODEC_DMA_TX_1, TX_CODEC_DMA_TX_1, 0, 1);
    set_port_map(RX_CODEC_DMA_RX_2, AFE_PORT_ID_RX_CODEC_DMA_RX_2, RX_CODEC_DMA_RX_2, 1, 1);
    set_port_map(TX_CODEC_DMA_TX_2, AFE_PORT_ID_TX_CODEC_DMA_TX_2, TX_CODEC_DMA_TX_2, 0, 1);
    set_port_map(RX_CODEC_DMA_RX_3, AFE_PORT_ID_RX_CODEC_DMA_RX_3, RX_CODEC_DMA_RX_3, 1, 1);
    set_port_map(TX_CODEC_DMA_TX_3, AFE_PORT_ID_TX_CODEC_DMA_TX_3, TX_CODEC_DMA_TX_3, 0, 1);
    set_port_map(RX_CODEC_DMA_RX_4, AFE_PORT_ID_RX_CODEC_DMA_RX_4, RX_CODEC_DMA_RX_4, 1, 1);
    set_port_map(TX_CODEC_DMA_TX_4, AFE_PORT_ID_TX_CODEC_DMA_TX_4, TX_CODEC_DMA_TX_4, 0, 1);
    set_port_map(RX_CODEC_DMA_RX_5, AFE_PORT_ID_RX_CODEC_DMA_RX_5, RX_CODEC_DMA_RX_5, 1, 1);
    set_port_map(TX_CODEC_DMA_TX_5, AFE_PORT_ID_TX_CODEC_DMA_TX_5, TX_CODEC_DMA_TX_5, 0, 1);
    set_port_map(RX_CODEC_DMA_RX_6, AFE_PORT_ID_RX_CODEC_DMA_RX_6, RX_CODEC_DMA_RX_6, 1, 1);
    set_port_map(RX_CODEC_DMA_RX_7, AFE_PORT_ID_RX_CODEC_DMA_RX_7, RX_CODEC_DMA_RX_7, 1, 1);
    set_port_map(USB_RX, AFE_PORT_ID_USB_RX, USB_RX, 1, 1);
    set_port_map(LPI_MI2S_RX_0, AFE_PORT_ID_INT0_MI2S_RX, LPI_MI2S_RX_0, 1, 1);
    set_port_map(LPI_MI2S_TX_0, AFE_PORT_ID_INT0_MI2S_TX, LPI_MI2S_TX_0, 0, 1);
    set_port_map(LPI_MI2S_RX_1, AFE_PORT_ID_INT1_MI2S_RX, LPI_MI2S_RX_1, 1, 1);
    set_port_map(LPI_MI2S_TX_1, AFE_PORT_ID_INT1_MI2S_TX, LPI_MI2S_TX_1, 0, 1);
    set_port_map(LPI_MI2S_RX_2, AFE_PORT_ID_INT2_MI2S_RX, LPI_MI2S_RX_2, 1, 1);
    set_port_map(LPI_MI2S_TX_2, AFE_PORT_ID_INT2_MI2S_TX, LPI_MI2S_TX_2, 0, 1);
    set_port_map(LPI_MI2S_RX_3, AFE_PORT_ID_INT3_MI2S_RX, LPI_MI2S_RX_3, 1, 1);
    set_port_map(LPI_MI2S_TX_3, AFE_PORT_ID_INT3_MI2S_TX, LPI_MI2S_TX_3, 0, 1);
    set_port_map(LPI_MI2S_RX_4, AFE_PORT_ID_INT4_MI2S_RX, LPI_MI2S_RX_4, 1, 1);
    set_port_map(LPI_MI2S_TX_4, AFE_PORT_ID_INT4_MI2S_TX, LPI_MI2S_TX_4, 0, 1);
    set_port_map(LPI_MI2S_RX_5, AFE_PORT_ID_INT5_MI2S_RX, LPI_MI2S_RX_5, 1, 1);
    set_port_map(LPI_MI2S_TX_5, AFE_PORT_ID_INT5_MI2S_TX, LPI_MI2S_TX_5, 0, 1);
    set_port_map(LPI_MI2S_RX_6, AFE_PORT_ID_INT6_MI2S_RX, LPI_MI2S_RX_6, 1, 1);
    set_port_map(LPI_MI2S_TX_6, AFE_PORT_ID_INT6_MI2S_TX, LPI_MI2S_TX_6, 0, 1);
}

unsafe extern "C" fn q6afe_port_free(ref_: *mut kref) {
    let port = (ref_ as *mut u8).sub(core::mem::offset_of!(q6afe_port, refcount)) as *mut q6afe_port;
    let afe = (*port).afe;
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*afe).port_list_lock, &mut flags);
    list_del(&mut (*port).node);
    spin_unlock_irqrestore(&mut (*afe).port_list_lock, flags);
    kfree((*port).scfg as *mut c_void);
    kfree(port as *mut c_void);
}

unsafe fn q6afe_find_port(afe: *mut q6afe, token: c_int) -> *mut q6afe_port {
    let mut ret: *mut q6afe_port = core::ptr::null_mut();
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*afe).port_list_lock, &mut flags);
    let mut pos = (*afe).port_list.next;
    while pos != &mut (*afe).port_list {
        let p = (pos as *mut u8).sub(core::mem::offset_of!(q6afe_port, node)) as *mut q6afe_port;
        if (*p).token == token {
            ret = p;
            kref_get(&mut (*p).refcount);
            break;
        }
        pos = (*pos).next;
    }
    spin_unlock_irqrestore(&mut (*afe).port_list_lock, flags);
    ret
}

unsafe extern "C" fn q6afe_callback(adev: *mut apr_device, data: *const apr_resp_pkt) -> c_int {
    let afe = dev_get_drvdata(&mut (*adev).dev) as *mut q6afe;
    let hdr = &(*data).hdr as *const apr_hdr;
    if (*data).payload_size == 0 { return 0; }
    let res = (*data).payload;
    match (*hdr).opcode {
        APR_BASIC_RSP_RESULT => {
            if (*res).status != 0 {
                dev_err((*afe).dev, b"cmd = 0x%x returned error = 0x%x\n\0".as_ptr() as *const c_char, (*res).opcode, (*res).status);
            }
            match (*res).opcode {
                AFE_PORT_CMD_SET_PARAM_V2 | AFE_PORT_CMD_DEVICE_STOP | AFE_PORT_CMD_DEVICE_START | AFE_SVC_CMD_SET_PARAM => {
                    let port = q6afe_find_port(afe, (*hdr).token as c_int);
                    if !port.is_null() {
                        (*port).result = *res;
                        wake_up(&mut (*port).wait);
                        kref_put(&mut (*port).refcount, q6afe_port_free);
                    } else if (*hdr).token == AFE_CLK_TOKEN {
                        (*afe).result = *res;
                        wake_up(&mut (*afe).wait);
                    }
                }
                _ => dev_err((*afe).dev, b"Unknown cmd 0x%x\n\0".as_ptr() as *const c_char, (*res).opcode),
            }
        }
        AFE_CMD_RSP_REMOTE_LPASS_CORE_HW_VOTE_REQUEST => {
            (*afe).result.opcode = (*hdr).opcode;
            (*afe).result.status = (*res).status;
            wake_up(&mut (*afe).wait);
        }
        _ => {}
    }
    0
}

/**
 * q6afe_get_port_id() - Get port id from a given port index
 *
 * @index: port index
 *
 * Return: Will be an negative on error or valid port_id on success
 */
#[no_mangle]
pub unsafe extern "C" fn q6afe_get_port_id(index: c_int) -> c_int {
    q6afe_init_port_maps();
    if index < 0 || index >= AFE_PORT_MAX { return -EINVAL; }
    (*port_maps.add(index as usize)).port_id
}

unsafe fn afe_apr_send_pkt(afe: *mut q6afe, pkt: *mut apr_pkt, port: *mut q6afe_port, rsp_opcode: uint32_t) -> c_int {
    let wait: *mut wait_queue_head_t;
    let result: *mut aprv2_ibasic_rsp_result_t;
    let mut ret: c_int;
    mutex_lock(&mut (*afe).lock);
    if !port.is_null() {
        wait = &mut (*port).wait;
        result = &mut (*port).result;
    } else {
        result = &mut (*afe).result;
        wait = &mut (*afe).wait;
    }
    (*result).opcode = 0;
    (*result).status = 0;
    ret = apr_send_pkt((*afe).apr, pkt);
    if ret < 0 {
        dev_err((*afe).dev, b"packet not transmitted (%d)\n\0".as_ptr() as *const c_char, ret);
        ret = -EINVAL;
    } else {
        ret = wait_event_timeout(wait, (*result).opcode == rsp_opcode, msecs_to_jiffies(TIMEOUT_MS));
        if ret == 0 {
            ret = -ETIMEDOUT;
        } else if (*result).status > 0 {
            dev_err((*afe).dev, b"DSP returned error[%x]\n\0".as_ptr() as *const c_char, (*result).status);
            ret = -EINVAL;
        } else {
            ret = 0;
        }
    }
    mutex_unlock(&mut (*afe).lock);
    ret
}

unsafe fn q6afe_set_param(afe: *mut q6afe, port: *mut q6afe_port, data: *mut c_void, param_id: c_int, module_id: c_int, psize: c_int, token: c_int) -> c_int {
    let pkt_size = APR_HDR_SIZE + core::mem::size_of::<afe_svc_cmd_set_param>() + core::mem::size_of::<afe_port_param_data_v2>() + psize as usize;
    let p = kzalloc(pkt_size, GFP_KERNEL);
    if p.is_null() { return -ENOMEM; }
    let pkt = p as *mut apr_pkt;
    let param = (p as *mut u8).add(APR_HDR_SIZE) as *mut afe_svc_cmd_set_param;
    let pdata = (p as *mut u8).add(APR_HDR_SIZE + core::mem::size_of::<afe_svc_cmd_set_param>()) as *mut afe_port_param_data_v2;
    let pl = (p as *mut u8).add(APR_HDR_SIZE + core::mem::size_of::<afe_svc_cmd_set_param>() + core::mem::size_of::<afe_port_param_data_v2>()) as *mut c_void;
    memcpy(pl, data, psize as usize);
    (*pkt).hdr.hdr_field = APR_HDR_FIELD(APR_MSG_TYPE_SEQ_CMD, APR_HDR_LEN(APR_HDR_SIZE), APR_PKT_VER);
    (*pkt).hdr.pkt_size = pkt_size as u32_;
    (*pkt).hdr.src_port = 0;
    (*pkt).hdr.dest_port = 0;
    (*pkt).hdr.token = token as u32_;
    (*pkt).hdr.opcode = AFE_SVC_CMD_SET_PARAM;
    (*param).payload_size = (core::mem::size_of::<afe_port_param_data_v2>() + psize as usize) as uint32_t;
    (*param).payload_address_lsw = 0;
    (*param).payload_address_msw = 0;
    (*param).mem_map_handle = 0;
    (*pdata).module_id = module_id as u32_;
    (*pdata).param_id = param_id as u32_;
    (*pdata).param_size = psize as u16_;
    let ret = afe_apr_send_pkt(afe, pkt, port, AFE_SVC_CMD_SET_PARAM);
    if ret != 0 { dev_err((*afe).dev, b"AFE set params failed %d\n\0".as_ptr() as *const c_char, ret); }
    kfree(p);
    ret
}

unsafe fn q6afe_port_set_param(port: *mut q6afe_port, data: *mut c_void, param_id: c_int, module_id: c_int, psize: c_int) -> c_int {
    q6afe_set_param((*port).afe, port, data, param_id, module_id, psize, (*port).token)
}

unsafe fn q6afe_port_set_param_v2(port: *mut q6afe_port, data: *mut c_void, param_id: c_int, module_id: c_int, psize: c_int) -> c_int {
    let afe = (*port).afe;
    let port_id = (*port).id as u16_;
    let pkt_size = APR_HDR_SIZE + core::mem::size_of::<afe_port_cmd_set_param_v2>() + core::mem::size_of::<afe_port_param_data_v2>() + psize as usize;
    let p = kzalloc(pkt_size, GFP_KERNEL);
    if p.is_null() { return -ENOMEM; }
    let pkt = p as *mut apr_pkt;
    let param = (p as *mut u8).add(APR_HDR_SIZE) as *mut afe_port_cmd_set_param_v2;
    let pdata = (p as *mut u8).add(APR_HDR_SIZE + core::mem::size_of::<afe_port_cmd_set_param_v2>()) as *mut afe_port_param_data_v2;
    let pl = (p as *mut u8).add(APR_HDR_SIZE + core::mem::size_of::<afe_port_cmd_set_param_v2>() + core::mem::size_of::<afe_port_param_data_v2>()) as *mut c_void;
    memcpy(pl, data, psize as usize);
    (*pkt).hdr.hdr_field = APR_HDR_FIELD(APR_MSG_TYPE_SEQ_CMD, APR_HDR_LEN(APR_HDR_SIZE), APR_PKT_VER);
    (*pkt).hdr.pkt_size = pkt_size as u32_;
    (*pkt).hdr.src_port = 0;
    (*pkt).hdr.dest_port = 0;
    (*pkt).hdr.token = (*port).token as u32_;
    (*pkt).hdr.opcode = AFE_PORT_CMD_SET_PARAM_V2;
    (*param).port_id = port_id;
    (*param).payload_size = (core::mem::size_of::<afe_port_param_data_v2>() + psize as usize) as u16_;
    (*param).payload_address_lsw = 0;
    (*param).payload_address_msw = 0;
    (*param).mem_map_handle = 0;
    (*pdata).module_id = module_id as u32_;
    (*pdata).param_id = param_id as u32_;
    (*pdata).param_size = psize as u16_;
    let ret = afe_apr_send_pkt(afe, pkt, port, AFE_PORT_CMD_SET_PARAM_V2);
    if ret != 0 { dev_err((*afe).dev, b"AFE enable for port 0x%x failed %d\n\0".as_ptr() as *const c_char, port_id as c_int, ret); }
    kfree(p);
    ret
}

unsafe fn q6afe_port_set_lpass_clock(port: *mut q6afe_port, cfg: *mut afe_clk_cfg) -> c_int {
    q6afe_port_set_param_v2(port, cfg as *mut c_void, AFE_PARAM_ID_LPAIF_CLK_CONFIG, AFE_MODULE_AUDIO_DEV_INTERFACE, core::mem::size_of::<afe_clk_cfg>() as c_int)
}
unsafe fn q6afe_set_lpass_clock_v2(port: *mut q6afe_port, cfg: *mut afe_clk_set) -> c_int {
    q6afe_port_set_param(port, cfg as *mut c_void, AFE_PARAM_ID_CLOCK_SET, AFE_MODULE_CLOCK_SET, core::mem::size_of::<afe_clk_set>() as c_int)
}
unsafe fn q6afe_set_digital_codec_core_clock(port: *mut q6afe_port, cfg: *mut afe_digital_clk_cfg) -> c_int {
    q6afe_port_set_param_v2(port, cfg as *mut c_void, AFE_PARAM_ID_INT_DIGITAL_CDC_CLK_CONFIG, AFE_MODULE_AUDIO_DEV_INTERFACE, core::mem::size_of::<afe_digital_clk_cfg>() as c_int)
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_set_lpass_clock(dev: *mut device, clk_id: c_int, attri: c_int, clk_root: c_int, freq: c_uint) -> c_int {
    let afe = dev_get_drvdata((*dev).parent) as *mut q6afe;
    let mut cset: afe_clk_set = core::mem::zeroed();
    cset.clk_set_minor_version = AFE_API_VERSION_CLOCK_SET;
    cset.clk_id = clk_id as uint32_t;
    cset.clk_freq_in_hz = freq;
    cset.clk_attri = attri as uint16_t;
    cset.clk_root = clk_root as uint16_t;
    cset.enable = (freq != 0) as uint32_t;
    q6afe_set_param(afe, core::ptr::null_mut(), &mut cset as *mut _ as *mut c_void, AFE_PARAM_ID_CLOCK_SET, AFE_MODULE_CLOCK_SET, core::mem::size_of::<afe_clk_set>() as c_int, AFE_CLK_TOKEN as c_int)
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_port_set_sysclk(port: *mut q6afe_port, clk_id: c_int, clk_src: c_int, clk_root: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let mut ccfg: afe_clk_cfg = core::mem::zeroed();
    let mut cset: afe_clk_set = core::mem::zeroed();
    let mut dcfg: afe_digital_clk_cfg = core::mem::zeroed();
    if clk_id == LPAIF_DIG_CLK {
        dcfg.i2s_cfg_minor_version = AFE_API_VERSION_I2S_CONFIG;
        dcfg.clk_val = freq;
        dcfg.clk_root = clk_root as u16_;
        q6afe_set_digital_codec_core_clock(port, &mut dcfg)
    } else if clk_id == LPAIF_BIT_CLK {
        ccfg.i2s_cfg_minor_version = AFE_API_VERSION_I2S_CONFIG;
        ccfg.clk_val1 = freq;
        ccfg.clk_src = clk_src as u16_;
        ccfg.clk_root = clk_root as u16_;
        ccfg.clk_set_mode = Q6AFE_LPASS_MODE_CLK1_VALID;
        q6afe_port_set_lpass_clock(port, &mut ccfg)
    } else if clk_id == LPAIF_OSR_CLK {
        ccfg.i2s_cfg_minor_version = AFE_API_VERSION_I2S_CONFIG;
        ccfg.clk_val2 = freq;
        ccfg.clk_src = clk_src as u16_;
        ccfg.clk_root = clk_root as u16_;
        ccfg.clk_set_mode = Q6AFE_LPASS_MODE_CLK2_VALID;
        q6afe_port_set_lpass_clock(port, &mut ccfg)
    } else if (clk_id >= Q6AFE_LPASS_CLK_ID_PRI_MI2S_IBIT && clk_id <= Q6AFE_LPASS_CLK_ID_QUI_MI2S_OSR)
        || (clk_id >= Q6AFE_LPASS_CLK_ID_MCLK_1 && clk_id <= Q6AFE_LPASS_CLK_ID_INT_MCLK_1)
        || (clk_id >= Q6AFE_LPASS_CLK_ID_PRI_TDM_IBIT && clk_id <= Q6AFE_LPASS_CLK_ID_QUIN_TDM_EBIT)
        || (clk_id >= Q6AFE_LPASS_CLK_ID_WSA_CORE_MCLK && clk_id <= Q6AFE_LPASS_CLK_ID_VA_CORE_2X_MCLK) {
        cset.clk_set_minor_version = AFE_API_VERSION_CLOCK_SET;
        cset.clk_id = clk_id as uint32_t;
        cset.clk_freq_in_hz = freq;
        cset.clk_attri = clk_src as uint16_t;
        cset.clk_root = clk_root as uint16_t;
        cset.enable = (freq != 0) as uint32_t;
        q6afe_set_lpass_clock_v2(port, &mut cset)
    } else {
        -EINVAL
    }
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_port_stop(port: *mut q6afe_port) -> c_int {
    let afe = (*port).afe;
    let port_id = (*port).id;
    let index = (*port).token;
    if index < 0 || index >= AFE_PORT_MAX {
        dev_err((*afe).dev, b"AFE port index[%d] invalid!\n\0".as_ptr() as *const c_char, index);
        return -EINVAL;
    }
    let pkt_size = APR_HDR_SIZE + core::mem::size_of::<afe_port_cmd_device_stop>();
    let p = kzalloc(pkt_size, GFP_KERNEL);
    if p.is_null() { return -ENOMEM; }
    let pkt = p as *mut apr_pkt;
    let stop = (p as *mut u8).add(APR_HDR_SIZE) as *mut afe_port_cmd_device_stop;
    (*pkt).hdr.hdr_field = APR_HDR_FIELD(APR_MSG_TYPE_SEQ_CMD, APR_HDR_LEN(APR_HDR_SIZE), APR_PKT_VER);
    (*pkt).hdr.pkt_size = pkt_size as u32_;
    (*pkt).hdr.src_port = 0;
    (*pkt).hdr.dest_port = 0;
    (*pkt).hdr.token = index as u32_;
    (*pkt).hdr.opcode = AFE_PORT_CMD_DEVICE_STOP;
    (*stop).port_id = port_id as u16_;
    (*stop).reserved = 0;
    let ret = afe_apr_send_pkt(afe, pkt, port, AFE_PORT_CMD_DEVICE_STOP);
    if ret != 0 { dev_err((*afe).dev, b"AFE close failed %d\n\0".as_ptr() as *const c_char, ret); }
    kfree(p);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_slim_port_prepare(port: *mut q6afe_port, cfg: *mut q6afe_slim_cfg) {
    (*(*port).port_cfg.slim_cfg).sb_cfg_minor_version = AFE_API_VERSION_SLIMBUS_CONFIG;
    (*(*port).port_cfg.slim_cfg).sample_rate = (*cfg).sample_rate;
    (*(*port).port_cfg.slim_cfg).bit_width = (*cfg).bit_width;
    (*(*port).port_cfg.slim_cfg).num_channels = (*cfg).num_channels;
    (*(*port).port_cfg.slim_cfg).data_format = (*cfg).data_format;
    (*(*port).port_cfg.slim_cfg).shared_ch_mapping[0] = (*cfg).ch_mapping[0];
    (*(*port).port_cfg.slim_cfg).shared_ch_mapping[1] = (*cfg).ch_mapping[1];
    (*(*port).port_cfg.slim_cfg).shared_ch_mapping[2] = (*cfg).ch_mapping[2];
    (*(*port).port_cfg.slim_cfg).shared_ch_mapping[3] = (*cfg).ch_mapping[3];
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_tdm_port_prepare(port: *mut q6afe_port, cfg: *mut q6afe_tdm_cfg) {
    (*(*port).port_cfg.tdm_cfg).tdm_cfg_minor_version = AFE_API_VERSION_TDM_CONFIG;
    (*(*port).port_cfg.tdm_cfg).num_channels = (*cfg).num_channels;
    (*(*port).port_cfg.tdm_cfg).sample_rate = (*cfg).sample_rate;
    (*(*port).port_cfg.tdm_cfg).bit_width = (*cfg).bit_width;
    (*(*port).port_cfg.tdm_cfg).data_format = (*cfg).data_format;
    (*(*port).port_cfg.tdm_cfg).sync_mode = (*cfg).sync_mode;
    (*(*port).port_cfg.tdm_cfg).sync_src = (*cfg).sync_src;
    (*(*port).port_cfg.tdm_cfg).nslots_per_frame = (*cfg).nslots_per_frame;
    (*(*port).port_cfg.tdm_cfg).slot_width = (*cfg).slot_width;
    (*(*port).port_cfg.tdm_cfg).slot_mask = (*cfg).slot_mask;
    (*port).scfg = kzalloc(core::mem::size_of::<afe_param_id_slot_mapping_cfg>(), GFP_KERNEL) as *mut afe_param_id_slot_mapping_cfg;
    if (*port).scfg.is_null() { return; }
    (*(*port).scfg).minor_version = AFE_API_VERSION_SLOT_MAPPING_CONFIG;
    (*(*port).scfg).num_channels = (*cfg).num_channels as u16_;
    (*(*port).scfg).bitwidth = (*cfg).bit_width as u16_;
    (*(*port).scfg).data_align_type = (*cfg).data_align_type;
    memcpy((*(*port).scfg).ch_mapping.as_mut_ptr() as *mut c_void, (*cfg).ch_mapping.as_ptr() as *const c_void, core::mem::size_of::<u16_>() * 32);
}

#[no_mangle]
pub unsafe extern "C" fn afe_port_send_usb_dev_param(port: *mut q6afe_port, cardidx: c_int, pcmidx: c_int) -> c_int {
    let mut usb_dev: afe_param_id_usb_audio_dev_params = core::mem::zeroed();
    usb_dev.cfg_minor_version = AFE_API_MINOR_VERSION_USB_AUDIO_CONFIG;
    usb_dev.dev_token = ((cardidx << 16) | (pcmidx << 8)) as u32_;
    let ret = q6afe_port_set_param_v2(port, &mut usb_dev as *mut _ as *mut c_void, AFE_PARAM_ID_USB_AUDIO_DEV_PARAMS, AFE_MODULE_AUDIO_DEV_INTERFACE, core::mem::size_of::<afe_param_id_usb_audio_dev_params>() as c_int);
    if ret != 0 { dev_err((*(*port).afe).dev, b"%s: AFE device param cmd failed %d\n\0".as_ptr() as *const c_char, b"afe_port_send_usb_dev_param\0".as_ptr() as *const c_char, ret); }
    ret
}

unsafe fn afe_port_send_usb_params(port: *mut q6afe_port, _cfg: *mut q6afe_usb_cfg) -> c_int {
    let mut lpcm_fmt: afe_param_id_usb_audio_dev_lpcm_fmt = core::mem::zeroed();
    let mut svc_int: afe_param_id_usb_audio_svc_interval = core::mem::zeroed();
    lpcm_fmt.cfg_minor_version = AFE_API_MINOR_VERSION_USB_AUDIO_CONFIG;
    lpcm_fmt.endian = (*(*port).port_cfg.usb_cfg).endian;
    let mut ret = q6afe_port_set_param_v2(port, &mut lpcm_fmt as *mut _ as *mut c_void, AFE_PARAM_ID_USB_AUDIO_DEV_LPCM_FMT, AFE_MODULE_AUDIO_DEV_INTERFACE, core::mem::size_of::<afe_param_id_usb_audio_dev_lpcm_fmt>() as c_int);
    if ret != 0 {
        dev_err((*(*port).afe).dev, b"%s: AFE device param cmd LPCM_FMT failed %d\n\0".as_ptr() as *const c_char, b"afe_port_send_usb_params\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    svc_int.cfg_minor_version = AFE_API_MINOR_VERSION_USB_AUDIO_CONFIG;
    svc_int.svc_interval = (*(*port).port_cfg.usb_cfg).service_interval;
    ret = q6afe_port_set_param_v2(port, &mut svc_int as *mut _ as *mut c_void, AFE_PARAM_ID_USB_AUDIO_SVC_INTERVAL, AFE_MODULE_AUDIO_DEV_INTERFACE, core::mem::size_of::<afe_param_id_usb_audio_svc_interval>() as c_int);
    if ret != 0 { dev_err((*(*port).afe).dev, b"%s: AFE device param cmd svc_interval failed %d\n\0".as_ptr() as *const c_char, b"afe_port_send_usb_params\0".as_ptr() as *const c_char, ret); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_usb_port_prepare(port: *mut q6afe_port, cfg: *mut q6afe_usb_cfg) {
    (*(*port).port_cfg.usb_cfg).cfg_minor_version = AFE_API_MINOR_VERSION_USB_AUDIO_CONFIG;
    (*(*port).port_cfg.usb_cfg).sample_rate = (*cfg).sample_rate;
    (*(*port).port_cfg.usb_cfg).num_channels = (*cfg).num_channels;
    (*(*port).port_cfg.usb_cfg).bit_width = (*cfg).bit_width;
    afe_port_send_usb_params(port, cfg);
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_hdmi_port_prepare(port: *mut q6afe_port, cfg: *mut q6afe_hdmi_cfg) {
    (*(*port).port_cfg.hdmi_multi_ch).hdmi_cfg_minor_version = AFE_API_VERSION_HDMI_CONFIG;
    (*(*port).port_cfg.hdmi_multi_ch).datatype = (*cfg).datatype;
    (*(*port).port_cfg.hdmi_multi_ch).channel_allocation = (*cfg).channel_allocation;
    (*(*port).port_cfg.hdmi_multi_ch).sample_rate = (*cfg).sample_rate;
    (*(*port).port_cfg.hdmi_multi_ch).bit_width = (*cfg).bit_width;
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_i2s_port_prepare(port: *mut q6afe_port, cfg: *mut q6afe_i2s_cfg) -> c_int {
    let dev = (*(*port).afe).dev;
    (*(*port).port_cfg.i2s_cfg).i2s_cfg_minor_version = AFE_API_VERSION_I2S_CONFIG;
    (*(*port).port_cfg.i2s_cfg).sample_rate = (*cfg).sample_rate;
    (*(*port).port_cfg.i2s_cfg).bit_width = (*cfg).bit_width;
    (*(*port).port_cfg.i2s_cfg).data_format = AFE_LINEAR_PCM_DATA;
    match (*cfg).fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_BP_FP => (*(*port).port_cfg.i2s_cfg).ws_src = AFE_PORT_CONFIG_I2S_WS_SRC_INTERNAL,
        x if x == SND_SOC_DAIFMT_BC_FC => (*(*port).port_cfg.i2s_cfg).ws_src = AFE_PORT_CONFIG_I2S_WS_SRC_EXTERNAL,
        _ => {}
    }
    match hweight_long((*cfg).sd_line_mask) {
        0 => { dev_err(dev, b"no line is assigned\n\0".as_ptr() as *const c_char); return -EINVAL; }
        1 => match (*cfg).sd_line_mask {
            AFE_PORT_I2S_SD0_MASK => (*(*port).port_cfg.i2s_cfg).channel_mode = AFE_PORT_I2S_SD0,
            AFE_PORT_I2S_SD1_MASK => (*(*port).port_cfg.i2s_cfg).channel_mode = AFE_PORT_I2S_SD1,
            AFE_PORT_I2S_SD2_MASK => (*(*port).port_cfg.i2s_cfg).channel_mode = AFE_PORT_I2S_SD2,
            AFE_PORT_I2S_SD3_MASK => (*(*port).port_cfg.i2s_cfg).channel_mode = AFE_PORT_I2S_SD3,
            _ => { dev_err(dev, b"Invalid SD lines\n\0".as_ptr() as *const c_char); return -EINVAL; }
        },
        2 => match (*cfg).sd_line_mask {
            AFE_PORT_I2S_SD0_1_MASK => (*(*port).port_cfg.i2s_cfg).channel_mode = AFE_PORT_I2S_QUAD01,
            AFE_PORT_I2S_SD2_3_MASK => (*(*port).port_cfg.i2s_cfg).channel_mode = AFE_PORT_I2S_QUAD23,
            _ => { dev_err(dev, b"Invalid SD lines\n\0".as_ptr() as *const c_char); return -EINVAL; }
        },
        3 => match (*cfg).sd_line_mask {
            AFE_PORT_I2S_SD0_1_2_MASK => (*(*port).port_cfg.i2s_cfg).channel_mode = AFE_PORT_I2S_6CHS,
            _ => { dev_err(dev, b"Invalid SD lines\n\0".as_ptr() as *const c_char); return -EINVAL; }
        },
        4 => match (*cfg).sd_line_mask {
            AFE_PORT_I2S_SD0_1_2_3_MASK => (*(*port).port_cfg.i2s_cfg).channel_mode = AFE_PORT_I2S_8CHS,
            _ => { dev_err(dev, b"Invalid SD lines\n\0".as_ptr() as *const c_char); return -EINVAL; }
        },
        _ => { dev_err(dev, b"Invalid SD lines\n\0".as_ptr() as *const c_char); return -EINVAL; }
    }
    match (*cfg).num_channels {
        1 | 2 => {
            match (*(*port).port_cfg.i2s_cfg).channel_mode {
                AFE_PORT_I2S_QUAD01 | AFE_PORT_I2S_6CHS | AFE_PORT_I2S_8CHS => (*(*port).port_cfg.i2s_cfg).channel_mode = AFE_PORT_I2S_SD0,
                AFE_PORT_I2S_QUAD23 => (*(*port).port_cfg.i2s_cfg).channel_mode = AFE_PORT_I2S_SD2,
                _ => {}
            }
            (*(*port).port_cfg.i2s_cfg).mono_stereo = if (*cfg).num_channels == 2 { AFE_PORT_I2S_STEREO } else { AFE_PORT_I2S_MONO };
        }
        3 | 4 => if (*(*port).port_cfg.i2s_cfg).channel_mode < AFE_PORT_I2S_QUAD01 { dev_err(dev, b"Invalid Channel mode\n\0".as_ptr() as *const c_char); return -EINVAL; },
        5 | 6 => if (*(*port).port_cfg.i2s_cfg).channel_mode < AFE_PORT_I2S_6CHS { dev_err(dev, b"Invalid Channel mode\n\0".as_ptr() as *const c_char); return -EINVAL; },
        7 | 8 => if (*(*port).port_cfg.i2s_cfg).channel_mode < AFE_PORT_I2S_8CHS { dev_err(dev, b"Invalid Channel mode\n\0".as_ptr() as *const c_char); return -EINVAL; },
        _ => {}
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_cdc_dma_port_prepare(port: *mut q6afe_port, cfg: *mut q6afe_cdc_dma_cfg) {
    (*(*port).port_cfg.dma_cfg).cdc_dma_cfg_minor_version = AFE_API_VERSION_CODEC_DMA_CONFIG;
    (*(*port).port_cfg.dma_cfg).sample_rate = (*cfg).sample_rate;
    (*(*port).port_cfg.dma_cfg).bit_width = (*cfg).bit_width;
    (*(*port).port_cfg.dma_cfg).data_format = (*cfg).data_format;
    (*(*port).port_cfg.dma_cfg).num_channels = (*cfg).num_channels;
    if (*cfg).active_channels_mask == 0 {
        (*(*port).port_cfg.dma_cfg).active_channels_mask = ((1u32 << (*cfg).num_channels) - 1) as u16_;
    }
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_port_start(port: *mut q6afe_port) -> c_int {
    let afe = (*port).afe;
    let port_id = (*port).id;
    let mut ret = q6afe_port_set_param_v2(port, &mut (*port).port_cfg as *mut _ as *mut c_void, (*port).cfg_type, AFE_MODULE_AUDIO_DEV_INTERFACE, core::mem::size_of::<afe_port_config>() as c_int);
    if ret != 0 {
        dev_err((*afe).dev, b"AFE enable for port 0x%x failed %d\n\0".as_ptr() as *const c_char, port_id, ret);
        return ret;
    }
    if !(*port).scfg.is_null() {
        ret = q6afe_port_set_param_v2(port, (*port).scfg as *mut c_void, AFE_PARAM_ID_PORT_SLOT_MAPPING_CONFIG, AFE_MODULE_TDM, core::mem::size_of::<afe_param_id_slot_mapping_cfg>() as c_int);
        if ret != 0 {
            dev_err((*afe).dev, b"AFE enable for port 0x%x failed %d\n\0".as_ptr() as *const c_char, port_id, ret);
            return ret;
        }
    }
    let pkt_size = APR_HDR_SIZE + core::mem::size_of::<afe_port_cmd_device_start>();
    let p = kzalloc(pkt_size, GFP_KERNEL);
    if p.is_null() { return -ENOMEM; }
    let pkt = p as *mut apr_pkt;
    let start = (p as *mut u8).add(APR_HDR_SIZE) as *mut afe_port_cmd_device_start;
    (*pkt).hdr.hdr_field = APR_HDR_FIELD(APR_MSG_TYPE_SEQ_CMD, APR_HDR_LEN(APR_HDR_SIZE), APR_PKT_VER);
    (*pkt).hdr.pkt_size = pkt_size as u32_;
    (*pkt).hdr.src_port = 0;
    (*pkt).hdr.dest_port = 0;
    (*pkt).hdr.token = (*port).token as u32_;
    (*pkt).hdr.opcode = AFE_PORT_CMD_DEVICE_START;
    (*start).port_id = port_id as u16_;
    ret = afe_apr_send_pkt(afe, pkt, port, AFE_PORT_CMD_DEVICE_START);
    if ret != 0 { dev_err((*afe).dev, b"AFE enable for port 0x%x failed %d\n\0".as_ptr() as *const c_char, port_id, ret); }
    kfree(p);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_port_get_from_id(dev: *mut device, id: c_int) -> *mut q6afe_port {
    q6afe_init_port_maps();
    let afe = dev_get_drvdata((*dev).parent) as *mut q6afe;
    if id < 0 || id >= AFE_PORT_MAX {
        dev_err(dev, b"AFE port token[%d] invalid!\n\0".as_ptr() as *const c_char, id);
        return ERR_PTR(-EINVAL as isize);
    }
    let mut port = q6afe_find_port(afe, id);
    if !port.is_null() {
        dev_err(dev, b"AFE Port already open\n\0".as_ptr() as *const c_char);
        return port;
    }
    let port_id = (*port_maps.add(id as usize)).port_id;
    let cfg_type = if port_id == AFE_PORT_ID_MULTICHAN_HDMI_RX || port_id == AFE_PORT_ID_HDMI_OVER_DP_RX {
        AFE_PARAM_ID_HDMI_CONFIG
    } else if (port_id >= AFE_PORT_ID_SLIMBUS_MULTI_CHAN_0_RX && port_id <= AFE_PORT_ID_SLIMBUS_MULTI_CHAN_6_TX) {
        AFE_PARAM_ID_SLIMBUS_CONFIG
    } else if matches!(port_id, AFE_PORT_ID_PRIMARY_MI2S_RX | AFE_PORT_ID_PRIMARY_MI2S_TX | AFE_PORT_ID_SECONDARY_MI2S_RX | AFE_PORT_ID_SECONDARY_MI2S_TX | AFE_PORT_ID_TERTIARY_MI2S_RX | AFE_PORT_ID_TERTIARY_MI2S_TX | AFE_PORT_ID_QUATERNARY_MI2S_RX | AFE_PORT_ID_QUATERNARY_MI2S_TX | AFE_PORT_ID_QUINARY_MI2S_RX | AFE_PORT_ID_QUINARY_MI2S_TX | AFE_PORT_ID_SENARY_MI2S_RX | AFE_PORT_ID_SENARY_MI2S_TX | AFE_PORT_ID_INT0_MI2S_RX | AFE_PORT_ID_INT0_MI2S_TX | AFE_PORT_ID_INT1_MI2S_RX | AFE_PORT_ID_INT1_MI2S_TX | AFE_PORT_ID_INT2_MI2S_RX | AFE_PORT_ID_INT2_MI2S_TX | AFE_PORT_ID_INT3_MI2S_RX | AFE_PORT_ID_INT3_MI2S_TX | AFE_PORT_ID_INT4_MI2S_RX | AFE_PORT_ID_INT4_MI2S_TX | AFE_PORT_ID_INT5_MI2S_RX | AFE_PORT_ID_INT5_MI2S_TX | AFE_PORT_ID_INT6_MI2S_RX | AFE_PORT_ID_INT6_MI2S_TX) {
        AFE_PARAM_ID_I2S_CONFIG
    } else if port_id >= AFE_PORT_ID_PRIMARY_TDM_RX && port_id <= AFE_PORT_ID_QUINARY_TDM_TX_7 {
        AFE_PARAM_ID_TDM_CONFIG
    } else if port_id >= AFE_PORT_ID_WSA_CODEC_DMA_RX_0 && port_id <= AFE_PORT_ID_RX_CODEC_DMA_RX_7 {
        AFE_PARAM_ID_CODEC_DMA_CONFIG
    } else if port_id == AFE_PORT_ID_USB_RX {
        AFE_PARAM_ID_USB_AUDIO_CONFIG
    } else {
        dev_err(dev, b"Invalid port id 0x%x\n\0".as_ptr() as *const c_char, port_id);
        return ERR_PTR(-EINVAL as isize);
    };
    port = kzalloc(core::mem::size_of::<q6afe_port>(), GFP_KERNEL) as *mut q6afe_port;
    if port.is_null() { return ERR_PTR(-ENOMEM as isize); }
    init_waitqueue_head(&mut (*port).wait);
    (*port).token = id;
    (*port).id = port_id;
    (*port).afe = afe;
    (*port).cfg_type = cfg_type;
    kref_init(&mut (*port).refcount);
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*afe).port_list_lock, &mut flags);
    list_add_tail(&mut (*port).node, &mut (*afe).port_list);
    spin_unlock_irqrestore(&mut (*afe).port_list_lock, flags);
    port
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_port_put(port: *mut q6afe_port) {
    kref_put(&mut (*port).refcount, q6afe_port_free);
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_unvote_lpass_core_hw(dev: *mut device, hw_block_id: uint32_t, client_handle: uint32_t) -> c_int {
    let afe = dev_get_drvdata((*dev).parent) as *mut q6afe;
    let pkt_size = APR_HDR_SIZE + core::mem::size_of::<afe_cmd_remote_lpass_core_hw_devote_request>();
    let p = kzalloc(pkt_size, GFP_KERNEL);
    if p.is_null() { return -ENOMEM; }
    let pkt = p as *mut apr_pkt;
    let vote_cfg = (p as *mut u8).add(APR_HDR_SIZE) as *mut afe_cmd_remote_lpass_core_hw_devote_request;
    (*pkt).hdr.hdr_field = APR_HDR_FIELD(APR_MSG_TYPE_SEQ_CMD, APR_HDR_LEN(APR_HDR_SIZE), APR_PKT_VER);
    (*pkt).hdr.pkt_size = pkt_size as u32_;
    (*pkt).hdr.src_port = 0;
    (*pkt).hdr.dest_port = 0;
    (*pkt).hdr.token = hw_block_id;
    (*pkt).hdr.opcode = AFE_CMD_REMOTE_LPASS_CORE_HW_DEVOTE_REQUEST;
    (*vote_cfg).hw_block_id = hw_block_id;
    (*vote_cfg).client_handle = client_handle;
    let ret = apr_send_pkt((*afe).apr, pkt);
    if ret < 0 { dev_err((*afe).dev, b"AFE failed to unvote (%d)\n\0".as_ptr() as *const c_char, hw_block_id); }
    kfree(p);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn q6afe_vote_lpass_core_hw(dev: *mut device, hw_block_id: uint32_t, client_name: *const c_char, _client_handle: *mut uint32_t) -> c_int {
    let afe = dev_get_drvdata((*dev).parent) as *mut q6afe;
    let pkt_size = APR_HDR_SIZE + core::mem::size_of::<afe_cmd_remote_lpass_core_hw_vote_request>();
    let p = kzalloc(pkt_size, GFP_KERNEL);
    if p.is_null() { return -ENOMEM; }
    let pkt = p as *mut apr_pkt;
    let vote_cfg = (p as *mut u8).add(APR_HDR_SIZE) as *mut afe_cmd_remote_lpass_core_hw_vote_request;
    (*pkt).hdr.hdr_field = APR_HDR_FIELD(APR_MSG_TYPE_SEQ_CMD, APR_HDR_LEN(APR_HDR_SIZE), APR_PKT_VER);
    (*pkt).hdr.pkt_size = pkt_size as u32_;
    (*pkt).hdr.src_port = 0;
    (*pkt).hdr.dest_port = 0;
    (*pkt).hdr.token = hw_block_id;
    (*pkt).hdr.opcode = AFE_CMD_REMOTE_LPASS_CORE_HW_VOTE_REQUEST;
    (*vote_cfg).hw_block_id = hw_block_id;
    strscpy((*vote_cfg).client_name.as_mut_ptr(), client_name, (*vote_cfg).client_name.len());
    let ret = afe_apr_send_pkt(afe, pkt, core::ptr::null_mut(), AFE_CMD_RSP_REMOTE_LPASS_CORE_HW_VOTE_REQUEST);
    if ret != 0 { dev_err((*afe).dev, b"AFE failed to vote (%d)\n\0".as_ptr() as *const c_char, hw_block_id); }
    kfree(p);
    ret
}

unsafe extern "C" fn q6afe_probe(adev: *mut apr_device) -> c_int {
    let dev = &mut (*adev).dev as *mut device;
    let afe = devm_kzalloc(dev, core::mem::size_of::<q6afe>(), GFP_KERNEL) as *mut q6afe;
    if afe.is_null() { return -ENOMEM; }
    q6core_get_svc_api_info((*adev).svc_id, &mut (*afe).ainfo);
    (*afe).apr = adev;
    mutex_init(&mut (*afe).lock);
    init_waitqueue_head(&mut (*afe).wait);
    (*afe).dev = dev;
    INIT_LIST_HEAD(&mut (*afe).port_list);
    spin_lock_init(&mut (*afe).port_list_lock);
    dev_set_drvdata(dev, afe as *mut c_void);
    q6afe_init_port_maps();
    devm_of_platform_populate(dev)
}

/* CONFIG_OF:
 * static const struct of_device_id q6afe_device_id[] = {
 *     { .compatible = "qcom,q6afe" },
 *     {},
 * };
 * MODULE_DEVICE_TABLE(of, q6afe_device_id);
 */
#[repr(C)] pub struct apr_driver { probe: Option<unsafe extern "C" fn(*mut apr_device) -> c_int>, callback: Option<unsafe extern "C" fn(*mut apr_device, *const apr_resp_pkt) -> c_int> }
#[no_mangle]
pub static mut qcom_q6afe_driver: apr_driver = apr_driver { probe: Some(q6afe_probe), callback: Some(q6afe_callback) };

/* module_apr_driver(qcom_q6afe_driver);
 * MODULE_DESCRIPTION("Q6 Audio Front End");
 * MODULE_LICENSE("GPL v2");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
