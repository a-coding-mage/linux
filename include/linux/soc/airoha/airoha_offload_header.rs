/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2025 AIROHA Inc
 * Author: Lorenzo Bianconi <lorenzo@kernel.org>
 */

// Linux dependencies supplied by other translation units:
// sk_buff, device, regmap, spinlock_t, work_struct, airoha_foe_stats,
// dma_addr_t, gfp_t, and the integer aliases are intentionally unresolved here.

pub const PPE_CPU_REASON_HIT_UNBIND_RATE_REACHED: u32 = 0x0f;

#[repr(C)]
pub struct airoha_ppe_dev_ops {
    pub setup_tc_block_cb: Option<unsafe extern "C" fn(*mut airoha_ppe_dev, *mut core::ffi::c_void) -> i32>,
    pub check_skb: Option<unsafe extern "C" fn(*mut airoha_ppe_dev, *mut sk_buff, u16, bool)>,
}

#[repr(C)]
pub struct airoha_ppe_dev {
    pub ops: airoha_ppe_dev_ops,
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct airoha_npu_rx_dma_desc {
    pub ctrl: u32,
    pub info: u32,
    pub data: u32,
    pub addr: u32,
    pub rsv: u64,
}

pub const NPU_NUM_CORES: usize = 8;
pub const NPU_NUM_IRQ: usize = 6;
pub const NPU_RX0_DESC_NUM: usize = 512;
pub const NPU_RX1_DESC_NUM: usize = 512;

pub const NPU_RX_DMA_DESC_LAST_MASK: u32 = 1u32 << 29;
pub const NPU_RX_DMA_DESC_LEN_MASK: u32 = 0x1fff_u32 << 15;
pub const NPU_RX_DMA_DESC_CUR_LEN_MASK: u32 = 0x3fff_u32 << 1;
pub const NPU_RX_DMA_DESC_DONE_MASK: u32 = 1;
pub const NPU_RX_DMA_PKT_COUNT_MASK: u32 = 0x7_u32 << 29;
pub const NPU_RX_DMA_PKT_ID_MASK: u32 = 0x7_u32 << 26;
pub const NPU_RX_DMA_SRC_PORT_MASK: u32 = 0x1f_u32 << 21;
pub const NPU_RX_DMA_CRSN_MASK: u32 = 0x1f_u32 << 16;
pub const NPU_RX_DMA_FOE_ID_MASK: u32 = 0xffff;
pub const NPU_RX_DMA_SID_MASK: u32 = 0xffff_u32 << 16;
pub const NPU_RX_DMA_FRAG_TYPE_MASK: u32 = 0x3_u32 << 14;
pub const NPU_RX_DMA_PRIORITY_MASK: u32 = 0xf_u32 << 10;
pub const NPU_RX_DMA_RADIO_ID_MASK: u32 = 0xf_u32 << 6;
pub const NPU_RX_DMA_VAP_ID_MASK: u32 = 0xf_u32 << 2;
pub const NPU_RX_DMA_FRAME_TYPE_MASK: u32 = 0x3;

pub const NPU_TX_DMA_DESC_SCHED_MASK: u32 = 1u32 << 31;
pub const NPU_TX_DMA_DESC_LEN_MASK: u32 = 0x1fff_u32 << 18;
pub const NPU_TX_DMA_DESC_VEND_LEN_MASK: u32 = 0x1ffff_u32 << 1;
pub const NPU_TX_DMA_DESC_DONE_MASK: u32 = 1;
pub const NPU_TXWI_LEN: usize = 192;

#[repr(C, packed)]
pub struct airoha_npu_tx_dma_desc {
    pub ctrl: u32,
    pub addr: u32,
    pub rsv: u64,
    pub txwi: [u8; NPU_TXWI_LEN],
}

#[repr(u32)]
pub enum airoha_npu_wlan_set_cmd {
    WLAN_FUNC_SET_WAIT_PCIE_ADDR,
    WLAN_FUNC_SET_WAIT_DESC,
    WLAN_FUNC_SET_WAIT_NPU_INIT_DONE,
    WLAN_FUNC_SET_WAIT_TRAN_TO_CPU,
    WLAN_FUNC_SET_WAIT_BA_WIN_SIZE,
    WLAN_FUNC_SET_WAIT_DRIVER_MODEL,
    WLAN_FUNC_SET_WAIT_DEL_STA,
    WLAN_FUNC_SET_WAIT_DRAM_BA_NODE_ADDR,
    WLAN_FUNC_SET_WAIT_PKT_BUF_ADDR,
    WLAN_FUNC_SET_WAIT_IS_TEST_NOBA,
    WLAN_FUNC_SET_WAIT_FLUSHONE_TIMEOUT,
    WLAN_FUNC_SET_WAIT_FLUSHALL_TIMEOUT,
    WLAN_FUNC_SET_WAIT_IS_FORCE_TO_CPU,
    WLAN_FUNC_SET_WAIT_PCIE_STATE,
    WLAN_FUNC_SET_WAIT_PCIE_PORT_TYPE,
    WLAN_FUNC_SET_WAIT_ERROR_RETRY_TIMES,
    WLAN_FUNC_SET_WAIT_BAR_INFO,
    WLAN_FUNC_SET_WAIT_FAST_FLAG,
    WLAN_FUNC_SET_WAIT_NPU_BAND0_ONCPU,
    WLAN_FUNC_SET_WAIT_TX_RING_PCIE_ADDR,
    WLAN_FUNC_SET_WAIT_TX_DESC_HW_BASE,
    WLAN_FUNC_SET_WAIT_TX_BUF_SPACE_HW_BASE,
    WLAN_FUNC_SET_WAIT_RX_RING_FOR_TXDONE_HW_BASE,
    WLAN_FUNC_SET_WAIT_TX_PKT_BUF_ADDR,
    WLAN_FUNC_SET_WAIT_INODE_TXRX_REG_ADDR,
    WLAN_FUNC_SET_WAIT_INODE_DEBUG_FLAG,
    WLAN_FUNC_SET_WAIT_INODE_HW_CFG_INFO,
    WLAN_FUNC_SET_WAIT_INODE_STOP_ACTION,
    WLAN_FUNC_SET_WAIT_INODE_PCIE_SWAP,
    WLAN_FUNC_SET_WAIT_RATELIMIT_CTRL,
    WLAN_FUNC_SET_WAIT_HWNAT_INIT,
    WLAN_FUNC_SET_WAIT_ARHT_CHIP_INFO,
    WLAN_FUNC_SET_WAIT_TX_BUF_CHECK_ADDR,
    WLAN_FUNC_SET_WAIT_TOKEN_ID_SIZE,
}

#[repr(u32)]
pub enum airoha_npu_wlan_get_cmd {
    WLAN_FUNC_GET_WAIT_NPU_INFO,
    WLAN_FUNC_GET_WAIT_LAST_RATE,
    WLAN_FUNC_GET_WAIT_COUNTER,
    WLAN_FUNC_GET_WAIT_DBG_COUNTER,
    WLAN_FUNC_GET_WAIT_RXDESC_BASE,
    WLAN_FUNC_GET_WAIT_WCID_DBG_COUNTER,
    WLAN_FUNC_GET_WAIT_DMA_ADDR,
    WLAN_FUNC_GET_WAIT_RING_SIZE,
    WLAN_FUNC_GET_WAIT_NPU_SUPPORT_MAP,
    WLAN_FUNC_GET_WAIT_MDC_LOCK_ADDRESS,
    WLAN_FUNC_GET_WAIT_NPU_VERSION,
}

// The NPU implementation fields are present only when CONFIG_NET_AIROHA_NPU
// is built in or modular; the external Linux types are supplied elsewhere.
#[repr(C)]
pub struct airoha_npu {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub cores: [airoha_npu_core; NPU_NUM_CORES],
    pub irqs: [i32; NPU_NUM_IRQ],
    pub stats: *mut airoha_foe_stats,
    pub ops: airoha_npu_ops,
}

#[repr(C)]
pub struct airoha_npu_core {
    pub npu: *mut airoha_npu,
    pub lock: spinlock_t,
    pub wdt_work: work_struct,
}

#[repr(C)]
pub struct airoha_npu_ops {
    pub ppe_init: Option<unsafe extern "C" fn(*mut airoha_npu) -> i32>,
    pub ppe_deinit: Option<unsafe extern "C" fn(*mut airoha_npu) -> i32>,
    pub ppe_init_stats: Option<unsafe extern "C" fn(*mut airoha_npu, dma_addr_t, u32) -> i32>,
    pub ppe_flush_sram_entries: Option<unsafe extern "C" fn(*mut airoha_npu, dma_addr_t, i32) -> i32>,
    pub ppe_foe_commit_entry: Option<unsafe extern "C" fn(*mut airoha_npu, dma_addr_t, u32, u32, bool) -> i32>,
    pub wlan_init_reserved_memory: Option<unsafe extern "C" fn(*mut airoha_npu) -> i32>,
    pub wlan_send_msg: Option<unsafe extern "C" fn(*mut airoha_npu, i32, airoha_npu_wlan_set_cmd, *mut core::ffi::c_void, i32, gfp_t) -> i32>,
    pub wlan_get_msg: Option<unsafe extern "C" fn(*mut airoha_npu, i32, airoha_npu_wlan_get_cmd, *mut core::ffi::c_void, i32, gfp_t) -> i32>,
    pub wlan_get_queue_addr: Option<unsafe extern "C" fn(*mut airoha_npu, i32, bool) -> u32>,
    pub wlan_set_irq_status: Option<unsafe extern "C" fn(*mut airoha_npu, u32)>,
    pub wlan_get_irq_status: Option<unsafe extern "C" fn(*mut airoha_npu, i32) -> u32>,
    pub wlan_enable_irq: Option<unsafe extern "C" fn(*mut airoha_npu, i32)>,
    pub wlan_disable_irq: Option<unsafe extern "C" fn(*mut airoha_npu, i32)>,
}

// The C header's IS_BUILTIN/IS_MODULE configuration branches are preserved by
// these declarations; unavailable configurations use the fallback semantics.
extern "C" {
    pub fn airoha_ppe_get_dev(dev: *mut device) -> *mut airoha_ppe_dev;
    pub fn airoha_ppe_put_dev(dev: *mut airoha_ppe_dev);
    pub fn airoha_npu_get(dev: *mut device) -> *mut airoha_npu;
    pub fn airoha_npu_put(npu: *mut airoha_npu);
}

pub unsafe fn airoha_ppe_dev_setup_tc_block_cb(dev: *mut airoha_ppe_dev, type_data: *mut core::ffi::c_void) -> i32 {
    ((*dev).ops.setup_tc_block_cb.unwrap())(dev, type_data)
}

pub unsafe fn airoha_ppe_dev_check_skb(dev: *mut airoha_ppe_dev, skb: *mut sk_buff, hash: u16, rx_wlan: bool) {
    ((*dev).ops.check_skb.unwrap())(dev, skb, hash, rx_wlan)
}

pub const EOPNOTSUPP: i32 = 95;

pub unsafe fn airoha_npu_wlan_init_reserved_memory(_npu: *mut airoha_npu) -> i32 { EOPNOTSUPP }
pub unsafe fn airoha_npu_wlan_send_msg(
    _npu: *mut airoha_npu, _ifindex: i32, _cmd: airoha_npu_wlan_set_cmd,
    _data: *mut core::ffi::c_void, _data_len: i32, _gfp: gfp_t,
) -> i32 { EOPNOTSUPP }
pub unsafe fn airoha_npu_wlan_get_msg(
    _npu: *mut airoha_npu, _ifindex: i32, _cmd: airoha_npu_wlan_get_cmd,
    _data: *mut core::ffi::c_void, _data_len: i32, _gfp: gfp_t,
) -> i32 { EOPNOTSUPP }
pub unsafe fn airoha_npu_wlan_get_queue_addr(_npu: *mut airoha_npu, _qid: i32, _xmit: bool) -> u32 { 0 }
pub unsafe fn airoha_npu_wlan_set_irq_status(_npu: *mut airoha_npu, _val: u32) {}
pub unsafe fn airoha_npu_wlan_get_irq_status(_npu: *mut airoha_npu, _q: i32) -> u32 { 0 }
pub unsafe fn airoha_npu_wlan_enable_irq(_npu: *mut airoha_npu, _q: i32) {}
pub unsafe fn airoha_npu_wlan_disable_irq(_npu: *mut airoha_npu, _q: i32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
