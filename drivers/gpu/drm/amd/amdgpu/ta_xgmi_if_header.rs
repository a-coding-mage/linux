/*
 * Copyright 2018-2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

/* Responses have bit 31 set */
pub const RSP_ID_MASK: u32 = 1u32 << 31;
#[inline]
pub const fn RSP_ID(cmd_id: u32) -> u32 { cmd_id | RSP_ID_MASK }

pub const EXTEND_PEER_LINK_INFO_CMD_FLAG: i32 = 1;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ta_command_xgmi {
    TA_COMMAND_XGMI__INITIALIZE = 0x00,
    TA_COMMAND_XGMI__GET_NODE_ID = 0x01,
    TA_COMMAND_XGMI__GET_HIVE_ID = 0x02,
    TA_COMMAND_XGMI__GET_TOPOLOGY_INFO = 0x03,
    TA_COMMAND_XGMI__SET_TOPOLOGY_INFO = 0x04,
    TA_COMMAND_XGMI__GET_PEER_LINKS = 0x0B,
    TA_COMMAND_XGMI__GET_EXTEND_PEER_LINKS = 0x0C,
}

pub const TA_XGMI__MAX_CONNECTED_NODES: usize = 64;
pub const TA_XGMI__MAX_INTERNAL_STATE: usize = 32;
pub const TA_XGMI__MAX_INTERNAL_STATE_BUFFER: usize = 128;
pub const TA_XGMI__MAX_PORT_NUM: usize = 8;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ta_xgmi_status {
    TA_XGMI_STATUS__SUCCESS = 0x00,
    TA_XGMI_STATUS__GENERIC_FAILURE = 0x01,
    TA_XGMI_STATUS__NULL_POINTER = 0x02,
    TA_XGMI_STATUS__INVALID_PARAMETER = 0x03,
    TA_XGMI_STATUS__NOT_INITIALIZED = 0x04,
    TA_XGMI_STATUS__INVALID_NODE_NUM = 0x05,
    TA_XGMI_STATUS__INVALID_NODE_ID = 0x06,
    TA_XGMI_STATUS__INVALID_TOPOLOGY = 0x07,
    TA_XGMI_STATUS__FAILED_ID_GEN = 0x08,
    TA_XGMI_STATUS__FAILED_TOPOLOGY_INIT = 0x09,
    TA_XGMI_STATUS__SET_SHARING_ERROR = 0x0A,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ta_xgmi_assigned_sdma_engine {
    TA_XGMI_ASSIGNED_SDMA_ENGINE__NOT_ASSIGNED = -1,
    TA_XGMI_ASSIGNED_SDMA_ENGINE__SDMA0 = 0,
    TA_XGMI_ASSIGNED_SDMA_ENGINE__SDMA1 = 1,
    TA_XGMI_ASSIGNED_SDMA_ENGINE__SDMA2 = 2,
    TA_XGMI_ASSIGNED_SDMA_ENGINE__SDMA3 = 3,
    TA_XGMI_ASSIGNED_SDMA_ENGINE__SDMA4 = 4,
    TA_XGMI_ASSIGNED_SDMA_ENGINE__SDMA5 = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_xgmi_node_info { pub node_id: u64, pub num_hops: u8, pub is_sharing_enabled: u8, pub sdma_engine: ta_xgmi_assigned_sdma_engine }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_xgmi_peer_link_info { pub node_id: u64, pub num_links: u8 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgmi_connected_port_num { pub dst_xgmi_port_num: u8, pub src_xgmi_port_num: u8 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_xgmi_extend_peer_link_info { pub node_id: u64, pub num_links: u8, pub port_num: [xgmi_connected_port_num; TA_XGMI__MAX_PORT_NUM] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_xgmi_cmd_initialize_output { pub status: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_xgmi_cmd_get_node_id_output { pub node_id: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_xgmi_cmd_get_hive_id_output { pub hive_id: u64 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_xgmi_cmd_get_topology_info_input { pub num_nodes: u32, pub nodes: [ta_xgmi_node_info; TA_XGMI__MAX_CONNECTED_NODES] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_xgmi_cmd_get_topology_info_output { pub num_nodes: u32, pub nodes: [ta_xgmi_node_info; TA_XGMI__MAX_CONNECTED_NODES] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_xgmi_cmd_set_topology_info_input { pub num_nodes: u32, pub nodes: [ta_xgmi_node_info; TA_XGMI__MAX_CONNECTED_NODES] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_xgmi_cmd_get_peer_link_info { pub num_nodes: u32, pub nodes: [ta_xgmi_peer_link_info; TA_XGMI__MAX_CONNECTED_NODES] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_xgmi_cmd_get_extend_peer_link_info { pub num_nodes: u32, pub nodes: [ta_xgmi_extend_peer_link_info; TA_XGMI__MAX_CONNECTED_NODES] }

#[repr(C)]
#[derive(Copy, Clone)]
pub union ta_xgmi_cmd_input { pub get_topology_info: ta_xgmi_cmd_get_topology_info_input, pub set_topology_info: ta_xgmi_cmd_set_topology_info_input }

#[repr(C)]
#[derive(Copy, Clone)]
pub union ta_xgmi_cmd_output {
    pub initialize: ta_xgmi_cmd_initialize_output,
    pub get_node_id: ta_xgmi_cmd_get_node_id_output,
    pub get_hive_id: ta_xgmi_cmd_get_hive_id_output,
    pub get_topology_info: ta_xgmi_cmd_get_topology_info_output,
    pub get_link_info: ta_xgmi_cmd_get_peer_link_info,
    pub get_extend_link_info: ta_xgmi_cmd_get_extend_peer_link_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_xgmi_shared_memory {
    pub cmd_id: u32,
    pub resp_id: u32,
    pub xgmi_status: ta_xgmi_status,
    /* if the number of xgmi link record is more than 128, driver will set the
     * flag 0 to get the first 128 of the link records and will set to 1, to get
     * the second set
     */
    pub flag_extend_link_record: u8,
    /* bit0: port_num info support flag for GET_EXTEND_PEER_LINKS commmand */
    pub caps_flag: u8,
    pub reserved: [u8; 2],
    pub xgmi_in_message: ta_xgmi_cmd_input,
    pub xgmi_out_message: ta_xgmi_cmd_output,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
