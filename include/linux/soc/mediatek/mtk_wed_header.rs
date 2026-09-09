// Translated from mtk_wed.h. Kernel-provided types and symbols are external dependencies.

pub const MTK_WED_TX_QUEUES: usize = 2;
pub const MTK_WED_RX_QUEUES: usize = 2;
pub const MTK_WED_RX_PAGE_QUEUES: usize = 3;
pub const WED_WO_STA_REC: u32 = 0x6;

pub const MTK_WED_RING_CONFIGURED: u32 = 1 << 0;

#[repr(C)]
pub struct mtk_wed_hw { _private: [u8; 0] }
#[repr(C)]
pub struct mtk_wdma_desc { _private: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mtk_wed_wo_cmd {
    MTK_WED_WO_CMD_WED_CFG,
    MTK_WED_WO_CMD_WED_RX_STAT,
    MTK_WED_WO_CMD_RRO_SER,
    MTK_WED_WO_CMD_DBG_INFO,
    MTK_WED_WO_CMD_DEV_INFO,
    MTK_WED_WO_CMD_BSS_INFO,
    MTK_WED_WO_CMD_STA_REC,
    MTK_WED_WO_CMD_DEV_INFO_DUMP,
    MTK_WED_WO_CMD_BSS_INFO_DUMP,
    MTK_WED_WO_CMD_STA_REC_DUMP,
    MTK_WED_WO_CMD_BA_INFO_DUMP,
    MTK_WED_WO_CMD_FBCMD_Q_DUMP,
    MTK_WED_WO_CMD_FW_LOG_CTRL,
    MTK_WED_WO_CMD_LOG_FLUSH,
    MTK_WED_WO_CMD_CHANGE_STATE,
    MTK_WED_WO_CMD_CPU_STATS_ENABLE,
    MTK_WED_WO_CMD_CPU_STATS_DUMP,
    MTK_WED_WO_CMD_EXCEPTION_INIT,
    MTK_WED_WO_CMD_PROF_CTRL,
    MTK_WED_WO_CMD_STA_BA_DUMP,
    MTK_WED_WO_CMD_BA_CTRL_DUMP,
    MTK_WED_WO_CMD_RXCNT_CTRL,
    MTK_WED_WO_CMD_RXCNT_INFO,
    MTK_WED_WO_CMD_SET_CAP,
    MTK_WED_WO_CMD_CCIF_RING_DUMP,
    MTK_WED_WO_CMD_WED_END,
}

#[repr(C, packed(4))]
pub struct mtk_wed_bm_desc { pub buf0: u32, pub token: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mtk_wed_bus_tye { MTK_WED_BUS_PCIE, MTK_WED_BUS_AXI }

#[repr(C)]
pub struct mtk_wed_ring {
    pub desc: *mut mtk_wdma_desc, pub desc_phys: dma_addr_t, pub desc_size: u32,
    pub size: i32, pub flags: u32, pub reg_base: u32, pub wpdma: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct mtk_wed_wo_rx_stats { pub wlan_idx: u16, pub tid: u16, pub rx_pkt_cnt: u32, pub rx_byte_cnt: u32, pub rx_err_cnt: u32, pub rx_drop_cnt: u32 }
#[repr(C)]
pub struct mtk_wed_buf { pub p: *mut core::ffi::c_void, pub phy_addr: dma_addr_t }

// CONFIG_NET_MEDIATEK_SOC_WED controls the conditional members in this structure.
#[repr(C)]
pub struct mtk_wed_device {
    pub ops: *const mtk_wed_ops, pub dev: *mut device, pub hw: *mut mtk_wed_hw,
    pub init_done: bool, pub running: bool, pub wdma_idx: i32, pub irq: i32, pub version: u8,
    pub rev_id: u32,
    pub tx_ring: [mtk_wed_ring; MTK_WED_TX_QUEUES], pub rx_ring: [mtk_wed_ring; MTK_WED_RX_QUEUES],
    pub txfree_ring: mtk_wed_ring, pub tx_wdma: [mtk_wed_ring; MTK_WED_TX_QUEUES],
    pub rx_wdma: [mtk_wed_ring; MTK_WED_RX_QUEUES], pub rx_rro_ring: [mtk_wed_ring; MTK_WED_RX_QUEUES],
    pub rx_page_ring: [mtk_wed_ring; MTK_WED_RX_PAGE_QUEUES], pub ind_cmd_ring: mtk_wed_ring,
    pub tx_buf_ring: mtk_wed_tx_buf_ring, pub rx_buf_ring: mtk_wed_rx_buf_ring,
    pub rro: mtk_wed_rro, pub hw_rro: mtk_wed_hw_rro,
    pub wlan: mtk_wed_wlan,
}
#[repr(C)] pub struct mtk_wed_tx_buf_ring { pub size: i32, pub pages: *mut mtk_wed_buf, pub desc: *mut mtk_wdma_desc, pub desc_phys: dma_addr_t }
#[repr(C)] pub struct mtk_wed_rx_buf_ring { pub size: i32, pub desc: *mut mtk_wed_bm_desc, pub desc_phys: dma_addr_t }
#[repr(C)] pub struct mtk_wed_rro { pub ring: mtk_wed_ring, pub miod_phys: dma_addr_t, pub fdbk_phys: dma_addr_t }
#[repr(C)] pub struct mtk_wed_hw_rro { pub size: i32, pub pages: *mut mtk_wed_buf, pub desc: *mut mtk_wed_bm_desc, pub desc_phys: dma_addr_t }
#[repr(C)] pub struct mtk_wed_wlan { pub platform_dev: *mut platform_device, pub bus_type: mtk_wed_bus_tye, pub base: *mut core::ffi::c_void, pub phy_base: u32, pub id: u32, pub wpdma_phys: u32, pub wpdma_int: u32, pub wpdma_mask: u32, pub wpdma_tx: u32, pub wpdma_txfree: u32, pub wpdma_rx_glo: u32, pub wpdma_rx: [u32; 2], pub wpdma_rx_rro: [u32; 2], pub wpdma_rx_pg: u32, pub wcid_512: bool, pub hw_rro: bool, pub msi: bool, pub hif2: bool, pub token_start: u16, pub nbuf: u32, pub rx_nbuf: u32, pub rx_npkt: u32, pub rx_size: u32, pub amsdu_max_len: u32, pub tx_tbit: [u8; 2], pub rx_tbit: [u8; 2], pub rro_rx_tbit: [u8; 2], pub rx_pg_tbit: [u8; 3], pub txfree_tbit: u8, pub amsdu_max_subframes: u8, pub init_buf: Option<unsafe extern "C" fn(*mut core::ffi::c_void,dma_addr_t,i32)->u32>, pub offload_enable: Option<unsafe extern "C" fn(*mut mtk_wed_device)->i32>, pub offload_disable: Option<unsafe extern "C" fn(*mut mtk_wed_device)>, pub init_rx_buf: Option<unsafe extern "C" fn(*mut mtk_wed_device,i32)->u32>, pub release_rx_buf: Option<unsafe extern "C" fn(*mut mtk_wed_device)>, pub update_wo_rx_stats: Option<unsafe extern "C" fn(*mut mtk_wed_device,*mut mtk_wed_wo_rx_stats)>, pub reset: Option<unsafe extern "C" fn(*mut mtk_wed_device)->i32>, pub reset_complete: Option<unsafe extern "C" fn(*mut mtk_wed_device)> }

#[repr(C)] pub struct mtk_wed_ops { pub attach: Option<unsafe extern "C" fn(*mut mtk_wed_device) -> i32>, pub tx_ring_setup: Option<unsafe extern "C" fn(*mut mtk_wed_device,i32,*mut core::ffi::c_void,bool)->i32>, pub rx_ring_setup: Option<unsafe extern "C" fn(*mut mtk_wed_device,i32,*mut core::ffi::c_void,bool)->i32>, pub txfree_ring_setup: Option<unsafe extern "C" fn(*mut mtk_wed_device,*mut core::ffi::c_void)->i32>, pub msg_update: Option<unsafe extern "C" fn(*mut mtk_wed_device,i32,*mut core::ffi::c_void,i32)->i32>, pub detach: Option<unsafe extern "C" fn(*mut mtk_wed_device)>, pub ppe_check: Option<unsafe extern "C" fn(*mut mtk_wed_device,*mut sk_buff,u32,u32)>, pub stop: Option<unsafe extern "C" fn(*mut mtk_wed_device)>, pub start: Option<unsafe extern "C" fn(*mut mtk_wed_device,u32)>, pub reset_dma: Option<unsafe extern "C" fn(*mut mtk_wed_device)>, pub reg_read: Option<unsafe extern "C" fn(*mut mtk_wed_device,u32)->u32>, pub reg_write: Option<unsafe extern "C" fn(*mut mtk_wed_device,u32,u32)>, pub irq_get: Option<unsafe extern "C" fn(*mut mtk_wed_device,u32)->u32>, pub irq_set_mask: Option<unsafe extern "C" fn(*mut mtk_wed_device,u32)>, pub setup_tc: Option<unsafe extern "C" fn(*mut mtk_wed_device,*mut net_device,tc_setup_type,*mut core::ffi::c_void)->i32>, pub start_hw_rro: Option<unsafe extern "C" fn(*mut mtk_wed_device,u32,bool)>, pub rro_rx_ring_setup: Option<unsafe extern "C" fn(*mut mtk_wed_device,i32,*mut core::ffi::c_void)>, pub msdu_pg_rx_ring_setup: Option<unsafe extern "C" fn(*mut mtk_wed_device,i32,*mut core::ffi::c_void)>, pub ind_rx_ring_setup: Option<unsafe extern "C" fn(*mut mtk_wed_device,*mut core::ffi::c_void)->i32> }

// The remaining operation-table members and inline wrappers depend on kernel build configuration.
// They are represented as external declarations to preserve the header interface.
extern "C" { pub static mut mtk_soc_wed_ops: *const mtk_wed_ops; }

pub unsafe fn mtk_wed_device_attach(dev: *mut mtk_wed_device) -> i32 {
    let mut ret = -19i32;
    #[cfg(feature = "CONFIG_NET_MEDIATEK_SOC_WED")]
    { (*dev).ops = mtk_soc_wed_ops; if !(*dev).ops.is_null() { ret = ((*(*dev).ops).attach.unwrap())(dev); } if ret != 0 { (*dev).ops = core::ptr::null(); } }
    ret
}
pub unsafe fn mtk_wed_get_rx_capa(dev: *mut mtk_wed_device) -> bool { #[cfg(feature = "CONFIG_NET_MEDIATEK_SOC_WED")] { if (*dev).version == 3 { return (*dev).wlan.hw_rro; } return (*dev).version != 1; } #[cfg(not(feature = "CONFIG_NET_MEDIATEK_SOC_WED"))] { let _ = dev; false } }
pub unsafe fn mtk_wed_is_amsdu_supported(dev: *mut mtk_wed_device) -> bool { #[cfg(feature = "CONFIG_NET_MEDIATEK_SOC_WED")] { (*dev).version == 3 } #[cfg(not(feature = "CONFIG_NET_MEDIATEK_SOC_WED"))] { let _ = dev; false } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
