/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of nci_core.h. */

#[repr(u32)]
pub enum nci_flag {
    NCI_INIT,
    NCI_UP,
    NCI_DATA_EXCHANGE,
    NCI_DATA_EXCHANGE_TO,
    NCI_UNREG,
}

#[repr(u32)]
pub enum nci_state {
    NCI_IDLE,
    NCI_DISCOVERY,
    NCI_W4_ALL_DISCOVERIES,
    NCI_W4_HOST_SELECT,
    NCI_POLL_ACTIVE,
    NCI_LISTEN_ACTIVE,
    NCI_LISTEN_SLEEP,
}

pub const NCI_RESET_TIMEOUT: u32 = 5000;
pub const NCI_INIT_TIMEOUT: u32 = 5000;
pub const NCI_SET_CONFIG_TIMEOUT: u32 = 5000;
pub const NCI_RF_DISC_TIMEOUT: u32 = 5000;
pub const NCI_RF_DISC_SELECT_TIMEOUT: u32 = 5000;
pub const NCI_RF_DEACTIVATE_TIMEOUT: u32 = 30000;
pub const NCI_CMD_TIMEOUT: u32 = 5000;
pub const NCI_DATA_TIMEOUT: u32 = 3000;

#[repr(C)]
pub struct nci_driver_ops {
    pub opcode: __u16,
    pub rsp: Option<unsafe extern "C" fn(*mut nci_dev, *mut sk_buff) -> ::core::ffi::c_int>,
    pub ntf: Option<unsafe extern "C" fn(*mut nci_dev, *mut sk_buff) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct nci_ops {
    pub init: Option<unsafe extern "C" fn(*mut nci_dev) -> ::core::ffi::c_int>,
    pub open: Option<unsafe extern "C" fn(*mut nci_dev) -> ::core::ffi::c_int>,
    pub close: Option<unsafe extern "C" fn(*mut nci_dev) -> ::core::ffi::c_int>,
    pub send: Option<unsafe extern "C" fn(*mut nci_dev, *mut sk_buff) -> ::core::ffi::c_int>,
    pub setup: Option<unsafe extern "C" fn(*mut nci_dev) -> ::core::ffi::c_int>,
    pub post_setup: Option<unsafe extern "C" fn(*mut nci_dev) -> ::core::ffi::c_int>,
    pub fw_download: Option<unsafe extern "C" fn(*mut nci_dev, *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub get_rfprotocol: Option<unsafe extern "C" fn(*mut nci_dev, __u8) -> __u32>,
    pub discover_se: Option<unsafe extern "C" fn(*mut nci_dev) -> ::core::ffi::c_int>,
    pub disable_se: Option<unsafe extern "C" fn(*mut nci_dev, u32) -> ::core::ffi::c_int>,
    pub enable_se: Option<unsafe extern "C" fn(*mut nci_dev, u32) -> ::core::ffi::c_int>,
    pub se_io: Option<unsafe extern "C" fn(*mut nci_dev, u32, *mut u8, usize, se_io_cb_t, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub hci_load_session: Option<unsafe extern "C" fn(*mut nci_dev) -> ::core::ffi::c_int>,
    pub hci_event_received: Option<unsafe extern "C" fn(*mut nci_dev, u8, u8, *mut sk_buff)>,
    pub hci_cmd_received: Option<unsafe extern "C" fn(*mut nci_dev, u8, u8, *mut sk_buff)>,
    pub prop_ops: *const nci_driver_ops,
    pub n_prop_ops: usize,
    pub core_ops: *const nci_driver_ops,
    pub n_core_ops: usize,
}

pub const NCI_MAX_SUPPORTED_RF_INTERFACES: usize = 4;
pub const NCI_MAX_DISCOVERED_TARGETS: usize = 10;
pub const NCI_MAX_NUM_NFCEE: u32 = 255;
pub const NCI_MAX_CONN_ID: u32 = 7;
pub const NCI_MAX_PROPRIETARY_CMD: usize = 64;

#[repr(C)]
pub struct nci_conn_info {
    pub list: list_head,
    pub dest_params: *mut dest_spec_params,
    pub dest_type: __u8,
    pub conn_id: __u8,
    pub max_pkt_payload_len: __u8,
    pub credits_cnt: atomic_t,
    pub initial_num_credits: __u8,
    pub data_exchange_cb: data_exchange_cb_t,
    pub data_exchange_cb_context: *mut ::core::ffi::c_void,
    pub rx_skb: *mut sk_buff,
}

pub const NCI_INVALID_CONN_ID: u8 = 0x80;
pub const NCI_HCI_ANY_OPEN_PIPE: u8 = 0x03;
pub const NCI_HCI_ADMIN_GATE: u8 = 0x00;
pub const NCI_HCI_LOOPBACK_GATE: u8 = 0x04;
pub const NCI_HCI_IDENTITY_MGMT_GATE: u8 = 0x05;
pub const NCI_HCI_LINK_MGMT_GATE: u8 = 0x06;
pub const NCI_HCI_LINK_MGMT_PIPE: u8 = 0x00;
pub const NCI_HCI_ADMIN_PIPE: u8 = 0x01;
pub const NCI_HCI_ANY_OK: u8 = 0x00;
pub const NCI_HCI_ANY_E_NOT_CONNECTED: u8 = 0x01;
pub const NCI_HCI_ANY_E_CMD_PAR_UNKNOWN: u8 = 0x02;
pub const NCI_HCI_ANY_E_NOK: u8 = 0x03;
pub const NCI_HCI_ANY_E_PIPES_FULL: u8 = 0x04;
pub const NCI_HCI_ANY_E_REG_PAR_UNKNOWN: u8 = 0x05;
pub const NCI_HCI_ANY_E_PIPE_NOT_OPENED: u8 = 0x06;
pub const NCI_HCI_ANY_E_CMD_NOT_SUPPORTED: u8 = 0x07;
pub const NCI_HCI_ANY_E_INHIBITED: u8 = 0x08;
pub const NCI_HCI_ANY_E_TIMEOUT: u8 = 0x09;
pub const NCI_HCI_ANY_E_REG_ACCESS_DENIED: u8 = 0x0a;
pub const NCI_HCI_ANY_E_PIPE_ACCESS_DENIED: u8 = 0x0b;
pub const NCI_HCI_DO_NOT_OPEN_PIPE: u8 = 0x81;
pub const NCI_HCI_INVALID_PIPE: u8 = 0x80;
pub const NCI_HCI_INVALID_GATE: u8 = 0xff;
pub const NCI_HCI_INVALID_HOST: u8 = 0x80;
pub const NCI_HCI_MAX_CUSTOM_GATES: usize = 50;
pub const NCI_HCI_MAX_PIPES: usize = 128;

#[repr(C, packed)]
pub struct nci_hci_gate { pub gate: u8, pub pipe: u8, pub dest_host: u8 }
#[repr(C, packed)]
pub struct nci_hci_pipe { pub gate: u8, pub host: u8 }
#[repr(C)]
pub struct nci_hci_init_data {
    pub gate_count: u8,
    pub gates: [nci_hci_gate; NCI_HCI_MAX_CUSTOM_GATES],
    pub session_id: [::core::ffi::c_char; 9],
}
pub const NCI_HCI_MAX_GATES: usize = 256;

#[repr(C)]
pub struct nci_hci_dev {
    pub nfcee_id: u8, pub ndev: *mut nci_dev, pub conn_info: *mut nci_conn_info,
    pub init_data: nci_hci_init_data, pub pipes: [nci_hci_pipe; NCI_HCI_MAX_PIPES],
    pub gate2pipe: [u8; NCI_HCI_MAX_GATES], pub expected_pipes: ::core::ffi::c_int,
    pub count_pipes: ::core::ffi::c_int, pub rx_hcp_frags: sk_buff_head,
    pub msg_rx_work: work_struct, pub msg_rx_queue: sk_buff_head,
}

#[repr(C)]
pub struct nci_dev {
    pub nfc_dev: *mut nfc_dev, pub ops: *const nci_ops, pub hci_dev: *mut nci_hci_dev,
    pub tx_headroom: ::core::ffi::c_int, pub tx_tailroom: ::core::ffi::c_int,
    pub state: atomic_t, pub flags: ::core::ffi::c_ulong, pub cmd_cnt: atomic_t,
    pub cur_conn_id: __u8, pub conn_info_list: list_head, pub rf_conn_info: *mut nci_conn_info,
    pub cmd_timer: timer_list, pub data_timer: timer_list,
    pub cmd_wq: *mut workqueue_struct, pub cmd_work: work_struct,
    pub rx_wq: *mut workqueue_struct, pub rx_work: work_struct,
    pub tx_wq: *mut workqueue_struct, pub tx_work: work_struct,
    pub cmd_q: sk_buff_head, pub rx_q: sk_buff_head, pub tx_q: sk_buff_head,
    pub req_lock: mutex, pub req_completion: completion, pub req_status: __u32, pub req_result: __u32,
    pub driver_data: *mut ::core::ffi::c_void, pub poll_prots: __u32, pub target_active_prot: __u32,
    pub targets: [nfc_target; NCI_MAX_DISCOVERED_TARGETS], pub n_targets: ::core::ffi::c_int,
    pub nci_ver: __u8, pub nfcc_features: __u32, pub num_supported_rf_interfaces: __u8,
    pub supported_rf_interfaces: [__u8; NCI_MAX_SUPPORTED_RF_INTERFACES], pub max_logical_connections: __u8,
    pub max_routing_table_size: __u16, pub max_ctrl_pkt_payload_len: __u8, pub max_size_for_large_params: __u16,
    pub manufact_id: __u8, pub manufact_specific_info: __u32, pub cur_params: dest_spec_params,
    pub cur_dest_type: __u8, pub rx_data_reassembly: *mut sk_buff, pub remote_gb: [__u8; NFC_MAX_GT_LEN],
    pub remote_gb_len: __u8, pub target_ats: [__u8; NFC_ATS_MAXSIZE], pub target_ats_len: __u8,
}

extern "C" {
    pub fn nci_allocate_device(ops: *const nci_ops, supported_protocols: __u32, tx_headroom: ::core::ffi::c_int, tx_tailroom: ::core::ffi::c_int) -> *mut nci_dev;
    pub fn nci_free_device(ndev: *mut nci_dev); pub fn nci_register_device(ndev: *mut nci_dev) -> ::core::ffi::c_int; pub fn nci_unregister_device(ndev: *mut nci_dev);
    pub fn nci_request(ndev: *mut nci_dev, req: Option<unsafe extern "C" fn(*mut nci_dev, *const ::core::ffi::c_void)>, opt: *const ::core::ffi::c_void, timeout: __u32) -> ::core::ffi::c_int;
    pub fn nci_prop_cmd(ndev: *mut nci_dev, oid: __u8, len: usize, payload: *const __u8) -> ::core::ffi::c_int;
    pub fn nci_core_cmd(ndev: *mut nci_dev, opcode: __u16, len: usize, payload: *const __u8) -> ::core::ffi::c_int;
    pub fn nci_core_reset(ndev: *mut nci_dev) -> ::core::ffi::c_int; pub fn nci_core_init(ndev: *mut nci_dev) -> ::core::ffi::c_int;
    pub fn nci_recv_frame(ndev: *mut nci_dev, skb: *mut sk_buff) -> ::core::ffi::c_int; pub fn nci_send_frame(ndev: *mut nci_dev, skb: *mut sk_buff) -> ::core::ffi::c_int;
    pub fn nci_set_config(ndev: *mut nci_dev, id: __u8, len: usize, val: *const __u8) -> ::core::ffi::c_int;
    pub fn nci_nfcee_discover(ndev: *mut nci_dev, action: u8) -> ::core::ffi::c_int; pub fn nci_nfcee_mode_set(ndev: *mut nci_dev, nfcee_id: u8, nfcee_mode: u8) -> ::core::ffi::c_int;
    pub fn nci_core_conn_create(ndev: *mut nci_dev, destination_type: u8, number_destination_params: u8, params_len: usize, params: *const core_conn_create_dest_spec_params) -> ::core::ffi::c_int;
    pub fn nci_core_conn_close(ndev: *mut nci_dev, conn_id: u8) -> ::core::ffi::c_int;
    pub fn nci_nfcc_loopback(ndev: *mut nci_dev, data: *const ::core::ffi::c_void, data_len: usize, resp: *mut *mut sk_buff) -> ::core::ffi::c_int;
    pub fn nci_hci_allocate(ndev: *mut nci_dev) -> *mut nci_hci_dev; pub fn nci_hci_deallocate(ndev: *mut nci_dev);
    pub fn nci_hci_send_event(ndev: *mut nci_dev, gate: u8, event: u8, param: *const u8, param_len: usize) -> ::core::ffi::c_int;
    pub fn nci_hci_send_cmd(ndev: *mut nci_dev, gate: u8, cmd: u8, param: *const u8, param_len: usize, skb: *mut *mut sk_buff) -> ::core::ffi::c_int;
    pub fn nci_hci_open_pipe(ndev: *mut nci_dev, pipe: u8) -> ::core::ffi::c_int; pub fn nci_hci_connect_gate(ndev: *mut nci_dev, dest_host: u8, dest_gate: u8, pipe: u8) -> ::core::ffi::c_int;
    pub fn nci_hci_set_param(ndev: *mut nci_dev, gate: u8, idx: u8, param: *const u8, param_len: usize) -> ::core::ffi::c_int;
    pub fn nci_hci_get_param(ndev: *mut nci_dev, gate: u8, idx: u8, skb: *mut *mut sk_buff) -> ::core::ffi::c_int;
    pub fn nci_hci_clear_all_pipes(ndev: *mut nci_dev) -> ::core::ffi::c_int; pub fn nci_hci_dev_session_init(ndev: *mut nci_dev) -> ::core::ffi::c_int;
    pub fn nci_rsp_packet(ndev: *mut nci_dev, skb: *mut sk_buff); pub fn nci_ntf_packet(ndev: *mut nci_dev, skb: *mut sk_buff);
    pub fn nci_prop_rsp_packet(ndev: *mut nci_dev, opcode: __u16, skb: *mut sk_buff) -> ::core::ffi::c_int; pub fn nci_prop_ntf_packet(ndev: *mut nci_dev, opcode: __u16, skb: *mut sk_buff) -> ::core::ffi::c_int;
    pub fn nci_core_rsp_packet(ndev: *mut nci_dev, opcode: __u16, skb: *mut sk_buff) -> ::core::ffi::c_int; pub fn nci_core_ntf_packet(ndev: *mut nci_dev, opcode: __u16, skb: *mut sk_buff) -> ::core::ffi::c_int;
    pub fn nci_rx_data_packet(ndev: *mut nci_dev, skb: *mut sk_buff); pub fn nci_send_cmd(ndev: *mut nci_dev, opcode: __u16, plen: __u8, payload: *const ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn nci_send_data(ndev: *mut nci_dev, conn_id: __u8, skb: *mut sk_buff) -> ::core::ffi::c_int; pub fn nci_conn_max_data_pkt_payload_size(ndev: *mut nci_dev, conn_id: __u8) -> ::core::ffi::c_int;
    pub fn nci_data_exchange_complete(ndev: *mut nci_dev, skb: *mut sk_buff, conn_id: __u8, err: ::core::ffi::c_int); pub fn nci_hci_data_received_cb(context: *mut ::core::ffi::c_void, skb: *mut sk_buff, err: ::core::ffi::c_int);
    pub fn nci_clear_target_list(ndev: *mut nci_dev); pub fn nci_req_complete(ndev: *mut nci_dev, result: ::core::ffi::c_int);
    pub fn nci_get_conn_info_by_conn_id(ndev: *mut nci_dev, conn_id: ::core::ffi::c_int) -> *mut nci_conn_info;
    pub fn nci_get_conn_info_by_dest_type_params(ndev: *mut nci_dev, dest_type: u8, params: *const dest_spec_params) -> ::core::ffi::c_int;
    pub fn nci_to_errno(code: __u8) -> ::core::ffi::c_int;
}

extern "C" {
    fn alloc_skb(len: ::core::ffi::c_uint, how: gfp_t) -> *mut sk_buff;
    fn skb_reserve(skb: *mut sk_buff, len: ::core::ffi::c_uint);
    fn nfc_set_parent_dev(nfc_dev: *mut nfc_dev, dev: *mut device);
    fn nfc_set_vendor_cmds(nfc_dev: *mut nfc_dev, cmds: *const nfc_vendor_cmd, n_cmds: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

pub unsafe fn nci_skb_alloc(ndev: *mut nci_dev, len: ::core::ffi::c_uint, how: gfp_t) -> *mut sk_buff {
    let skb = alloc_skb(len + (*ndev).tx_headroom as ::core::ffi::c_uint + (*ndev).tx_tailroom as ::core::ffi::c_uint, how);
    if !skb.is_null() { skb_reserve(skb, (*ndev).tx_headroom as ::core::ffi::c_uint); }
    skb
}

pub unsafe fn nci_set_parent_dev(ndev: *mut nci_dev, dev: *mut device) {
    nfc_set_parent_dev((*ndev).nfc_dev, dev);
}

pub unsafe fn nci_set_drvdata(ndev: *mut nci_dev, data: *mut ::core::ffi::c_void) {
    (*ndev).driver_data = data;
}

pub unsafe fn nci_get_drvdata(ndev: *mut nci_dev) -> *mut ::core::ffi::c_void {
    (*ndev).driver_data
}

pub unsafe fn nci_set_vendor_cmds(ndev: *mut nci_dev, cmds: *const nfc_vendor_cmd, n_cmds: ::core::ffi::c_int) -> ::core::ffi::c_int {
    nfc_set_vendor_cmds((*ndev).nfc_dev, cmds, n_cmds)
}

pub const NCI_REQ_DONE: u32 = 0; pub const NCI_REQ_PEND: u32 = 1; pub const NCI_REQ_CANCELED: u32 = 2;
pub const NCI_SPI_CRC_DISABLED: u8 = 0x00; pub const NCI_SPI_CRC_ENABLED: u8 = 0x01;

#[repr(C)]
pub struct nci_spi { pub ndev: *mut nci_dev, pub spi: *mut spi_device, pub xfer_udelay: ::core::ffi::c_uint, pub xfer_speed_hz: ::core::ffi::c_uint, pub acknowledge_mode: u8, pub req_completion: completion, pub req_result: u8 }
extern "C" { pub fn nci_spi_allocate_spi(spi: *mut spi_device, acknowledge_mode: u8, delay: ::core::ffi::c_uint, ndev: *mut nci_dev) -> *mut nci_spi; pub fn nci_spi_send(nspi: *mut nci_spi, write_handshake_completion: *mut completion, skb: *mut sk_buff) -> ::core::ffi::c_int; pub fn nci_spi_read(nspi: *mut nci_spi) -> *mut sk_buff; }

// _IOW('U', 0, char *) is provided by the platform ioctl definitions.
pub const NCIUARTSETDRIVER: u32 = 0;
#[repr(u32)] pub enum nci_uart_driver { NCI_UART_DRIVER_MARVELL = 0, NCI_UART_DRIVER_MAX }
#[repr(C)] pub struct nci_uart_ops {
    pub open: Option<unsafe extern "C" fn(*mut nci_uart) -> ::core::ffi::c_int>, pub close: Option<unsafe extern "C" fn(*mut nci_uart)>, pub recv: Option<unsafe extern "C" fn(*mut nci_uart, *mut sk_buff) -> ::core::ffi::c_int>, pub send: Option<unsafe extern "C" fn(*mut nci_uart, *mut sk_buff) -> ::core::ffi::c_int>, pub tx_start: Option<unsafe extern "C" fn(*mut nci_uart)>, pub tx_done: Option<unsafe extern "C" fn(*mut nci_uart)>,
}
#[repr(C)] pub struct nci_uart {
    pub owner: *mut module, pub ops: nci_uart_ops, pub name: *const ::core::ffi::c_char, pub driver: nci_uart_driver,
    pub ndev: *mut nci_dev, pub rx_lock: spinlock_t, pub write_work: work_struct, pub tty: *mut tty_struct,
    pub tx_state: ::core::ffi::c_ulong, pub tx_q: sk_buff_head, pub tx_skb: *mut sk_buff, pub rx_skb: *mut sk_buff,
    pub rx_packet_len: ::core::ffi::c_int, pub drv_data: *mut ::core::ffi::c_void,
}
extern "C" { pub fn nci_uart_register(nu: *mut nci_uart) -> ::core::ffi::c_int; pub fn nci_uart_unregister(nu: *mut nci_uart); pub fn nci_uart_set_config(nu: *mut nci_uart, baudrate: ::core::ffi::c_int, flow_ctrl: ::core::ffi::c_int); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
