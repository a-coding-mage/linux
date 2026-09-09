// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Copyright (c) 2024-2025, NVIDIA CORPORATION & AFFILIATES
 */

// Kernel headers and generated mlx5 interfaces are supplied by other files.

#[repr(C)]
pub struct mlx5ctl_uctx {
    pub uctx: fwctl_uctx,
    pub uctx_caps: u32,
    pub uctx_uid: u32,
}

#[repr(C)]
pub struct mlx5ctl_dev {
    pub fwctl: fwctl_device,
    pub mdev: *mut mlx5_core_dev,
}

#[repr(C)]
pub struct mlx5_ifc_mbox_in_hdr_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
pub struct mlx5_ifc_mbox_out_hdr_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

pub const MLX5_UCTX_OBJECT_CAP_TOOLS_RESOURCES: u32 = 0x4;
pub const MLX5_CMD_OP_QUERY_DRIVER_VERSION: u32 = 0x10c;
pub const MLX5_CMD_OP_QUERY_OTHER_HCA_CAP: u32 = 0x10e;
pub const MLX5_CMD_OP_QUERY_RDB: u32 = 0x512;
pub const MLX5_CMD_OP_QUERY_PSV: u32 = 0x602;
pub const MLX5_CMD_OP_QUERY_DC_CNAK_TRACE: u32 = 0x716;
pub const MLX5_CMD_OP_QUERY_NVMF_BACKEND_CONTROLLER: u32 = 0x722;
pub const MLX5_CMD_OP_QUERY_NVMF_NAMESPACE_CONTEXT: u32 = 0x728;
pub const MLX5_CMD_OP_QUERY_ADJACENT_FUNCTIONS_ID: u32 = 0x730;
pub const MLX5_CMD_OP_DELEGATE_VHCA_MANAGEMENT: u32 = 0x731;
pub const MLX5_CMD_OP_QUERY_DELEGATED_VHCA: u32 = 0x732;
pub const MLX5_CMD_OP_QUERY_BURST_SIZE: u32 = 0x813;
pub const MLX5_CMD_OP_QUERY_DIAGNOSTIC_PARAMS: u32 = 0x819;
pub const MLX5_CMD_OP_SET_DIAGNOSTIC_PARAMS: u32 = 0x820;
pub const MLX5_CMD_OP_QUERY_DIAGNOSTIC_COUNTERS: u32 = 0x821;
pub const MLX5_CMD_OP_QUERY_DELAY_DROP_PARAMS: u32 = 0x911;
pub const MLX5_CMD_OP_QUERY_AFU: u32 = 0x971;
pub const MLX5_CMD_OP_QUERY_CAPI_PEC: u32 = 0x981;
pub const MLX5_CMD_OP_QUERY_UCTX: u32 = 0xa05;
pub const MLX5_CMD_OP_QUERY_UMEM: u32 = 0xa09;
pub const MLX5_CMD_OP_QUERY_NVMF_CC_RESPONSE: u32 = 0xb02;
pub const MLX5_CMD_OP_QUERY_EMULATED_FUNCTIONS_INFO: u32 = 0xb03;
pub const MLX5_CMD_OP_QUERY_REGEXP_PARAMS: u32 = 0xb05;
pub const MLX5_CMD_OP_QUERY_REGEXP_REGISTER: u32 = 0xb07;
pub const MLX5_CMD_OP_USER_QUERY_XRQ_DC_PARAMS_ENTRY: u32 = 0xb08;
pub const MLX5_CMD_OP_USER_QUERY_XRQ_ERROR_PARAMS: u32 = 0xb0a;
pub const MLX5_CMD_OP_ACCESS_REGISTER_USER: u32 = 0xb0c;
pub const MLX5_CMD_OP_QUERY_EMULATION_DEVICE_EQ_MSIX_MAPPING: u32 = 0xb0f;
pub const MLX5_CMD_OP_QUERY_MATCH_SAMPLE_INFO: u32 = 0xb13;
pub const MLX5_CMD_OP_QUERY_CRYPTO_STATE: u32 = 0xb14;
pub const MLX5_CMD_OP_QUERY_VUID: u32 = 0xb22;
pub const MLX5_CMD_OP_QUERY_DPA_PARTITION: u32 = 0xb28;
pub const MLX5_CMD_OP_QUERY_DPA_PARTITIONS: u32 = 0xb2a;
pub const MLX5_CMD_OP_POSTPONE_CONNECTED_QP_TIMEOUT: u32 = 0xb2e;
pub const MLX5_CMD_OP_QUERY_EMULATED_RESOURCES_INFO: u32 = 0xb2f;
pub const MLX5_CMD_OP_QUERY_RSV_RESOURCES: u32 = 0x8000;
pub const MLX5_CMD_OP_QUERY_MTT: u32 = 0x8001;
pub const MLX5_CMD_OP_QUERY_SCHED_QUEUE: u32 = 0x8006;

unsafe fn mlx5ctl_alloc_uid(mcdev: *mut mlx5ctl_dev, cap: u32) -> i32 {
    let mut out = [0u32; MLX5_ST_SZ_DW(create_uctx_out) as usize];
    let mut input = [0u32; MLX5_ST_SZ_DW(create_uctx_in) as usize];
    let uctx = MLX5_ADDR_OF(create_uctx_in, input.as_mut_ptr(), uctx);
    mlx5ctl_dbg(mcdev, "%s: caps 0x%x\n", __func__, cap);
    MLX5_SET(create_uctx_in, input.as_mut_ptr(), opcode, MLX5_CMD_OP_CREATE_UCTX);
    MLX5_SET(uctx, uctx, cap, cap);
    let ret = mlx5_cmd_exec((*mcdev).mdev, input.as_mut_ptr(), core::mem::size_of_val(&input), out.as_mut_ptr(), core::mem::size_of_val(&out));
    if ret != 0 { return ret; }
    let uid: u16 = MLX5_GET(create_uctx_out, out.as_ptr(), uid);
    mlx5ctl_dbg(mcdev, "allocated uid %u with caps 0x%x\n", uid, cap);
    uid as i32
}

unsafe fn mlx5ctl_release_uid(mcdev: *mut mlx5ctl_dev, uid: u16) {
    let mut input = [0u32; MLX5_ST_SZ_DW(destroy_uctx_in) as usize];
    let mdev = (*mcdev).mdev;
    MLX5_SET(destroy_uctx_in, input.as_mut_ptr(), opcode, MLX5_CMD_OP_DESTROY_UCTX);
    MLX5_SET(destroy_uctx_in, input.as_mut_ptr(), uid, uid);
    let ret = mlx5_cmd_exec_in(mdev, destroy_uctx, input.as_mut_ptr());
    mlx5ctl_dbg(mcdev, "released uid %u %pe\n", uid, ERR_PTR(ret));
}

unsafe fn mlx5ctl_open_uctx(uctx: *mut fwctl_uctx) -> i32 {
    let mfd = container_of!(uctx, mlx5ctl_uctx, uctx);
    let mcdev = container_of!((*uctx).fwctl, mlx5ctl_dev, fwctl);
    if (MLX5_CAP_GEN((*mcdev).mdev, uctx_cap) & MLX5_UCTX_OBJECT_CAP_TOOLS_RESOURCES) != 0 {
        (*mfd).uctx_caps |= MLX5_UCTX_OBJECT_CAP_TOOLS_RESOURCES;
    }
    let uid = mlx5ctl_alloc_uid(mcdev, (*mfd).uctx_caps);
    if uid < 0 { return uid; }
    (*mfd).uctx_uid = uid as u32;
    0
}

unsafe fn mlx5ctl_close_uctx(uctx: *mut fwctl_uctx) {
    let mcdev = container_of!((*uctx).fwctl, mlx5ctl_dev, fwctl);
    let mfd = container_of!(uctx, mlx5ctl_uctx, uctx);
    mlx5ctl_release_uid(mcdev, (*mfd).uctx_uid as u16);
}

unsafe fn mlx5ctl_info(uctx: *mut fwctl_uctx, length: *mut usize) -> *mut core::ffi::c_void {
    let mfd = container_of!(uctx, mlx5ctl_uctx, uctx);
    let info = kzalloc_obj::<fwctl_info_mlx5>();
    if info.is_null() { return ERR_PTR(-ENOMEM); }
    (*info).uid = (*mfd).uctx_uid;
    (*info).uctx_caps = (*mfd).uctx_caps;
    *length = core::mem::size_of::<fwctl_info_mlx5>();
    info.cast()
}

unsafe fn mlx5ctl_validate_rpc(input: *const core::ffi::c_void, scope: fwctl_rpc_scope) -> bool {
    let opcode: u16 = MLX5_GET(mbox_in_hdr, input, opcode);
    let op_mod: u16 = MLX5_GET(mbox_in_hdr, input, op_mod);
    match opcode {
        MLX5_CMD_OP_MODIFY_CONG_STATUS | MLX5_CMD_OP_POSTPONE_CONNECTED_QP_TIMEOUT |
        MLX5_CMD_OP_QUERY_ADAPTER | MLX5_CMD_OP_QUERY_ESW_FUNCTIONS | MLX5_CMD_OP_QUERY_HCA_CAP |
        MLX5_CMD_OP_QUERY_HCA_VPORT_CONTEXT | MLX5_CMD_OP_QUERY_OTHER_HCA_CAP |
        MLX5_CMD_OP_QUERY_ROCE_ADDRESS | MLX5_CMD_OPCODE_QUERY_VUID |
        MLX5_CMD_OP_DELEGATE_VHCA_MANAGEMENT | MLX5_CMD_OP_SET_HCA_CAP => true,
        MLX5_CMD_OP_FPGA_QUERY_QP_COUNTERS | MLX5_CMD_OP_FPGA_QUERY_QP | MLX5_CMD_OP_NOP |
        MLX5_CMD_OP_QUERY_AFU | MLX5_CMD_OP_QUERY_BURST_SIZE | MLX5_CMD_OP_QUERY_CAPI_PEC |
        MLX5_CMD_OP_QUERY_CONG_PARAMS | MLX5_CMD_OP_QUERY_CONG_STATISTICS | MLX5_CMD_OP_QUERY_CONG_STATUS |
        MLX5_CMD_OP_QUERY_CQ | MLX5_CMD_OP_QUERY_CRYPTO_STATE | MLX5_CMD_OP_QUERY_DC_CNAK_TRACE |
        MLX5_CMD_OP_QUERY_DCT | MLX5_CMD_OP_QUERY_DELAY_DROP_PARAMS | MLX5_CMD_OP_QUERY_DIAGNOSTIC_COUNTERS |
        MLX5_CMD_OP_QUERY_DIAGNOSTIC_PARAMS | MLX5_CMD_OP_QUERY_DPA_PARTITION | MLX5_CMD_OP_QUERY_DPA_PARTITIONS |
        MLX5_CMD_OP_QUERY_DRIVER_VERSION | MLX5_CMD_OP_QUERY_EMULATED_FUNCTIONS_INFO |
        MLX5_CMD_OP_QUERY_EMULATED_RESOURCES_INFO | MLX5_CMD_OP_QUERY_EMULATION_DEVICE_EQ_MSIX_MAPPING |
        MLX5_CMD_OP_QUERY_EQ | MLX5_CMD_OP_QUERY_ESW_VPORT_CONTEXT | MLX5_CMD_OP_QUERY_FLOW_COUNTER |
        MLX5_CMD_OP_QUERY_FLOW_GROUP | MLX5_CMD_OP_QUERY_FLOW_TABLE_ENTRY | MLX5_CMD_OP_QUERY_FLOW_TABLE |
        MLX5_CMD_OP_QUERY_GENERAL_OBJECT | MLX5_CMD_OP_QUERY_HCA_VPORT_GID | MLX5_CMD_OP_QUERY_HCA_VPORT_PKEY |
        MLX5_CMD_OP_QUERY_ISSI | MLX5_CMD_OP_QUERY_L2_TABLE_ENTRY | MLX5_CMD_OP_QUERY_LAG |
        MLX5_CMD_OP_QUERY_MAD_DEMUX | MLX5_CMD_OP_QUERY_MATCH_SAMPLE_INFO | MLX5_CMD_OP_QUERY_MKEY |
        MLX5_CMD_OP_QUERY_MODIFY_HEADER_CONTEXT | MLX5_CMD_OP_QUERY_MTT | MLX5_CMD_OP_QUERY_NIC_VPORT_CONTEXT |
        MLX5_CMD_OP_QUERY_NVMF_BACKEND_CONTROLLER | MLX5_CMD_OP_QUERY_NVMF_CC_RESPONSE |
        MLX5_CMD_OP_QUERY_NVMF_NAMESPACE_CONTEXT | MLX5_CMD_OP_QUERY_PACKET_REFORMAT_CONTEXT |
        MLX5_CMD_OP_QUERY_PAGES | MLX5_CMD_OP_QUERY_PSV | MLX5_CMD_OP_QUERY_Q_COUNTER |
        MLX5_CMD_OP_QUERY_QP | MLX5_CMD_OP_QUERY_RATE_LIMIT | MLX5_CMD_OP_QUERY_RDB |
        MLX5_CMD_OP_QUERY_REGEXP_PARAMS | MLX5_CMD_OP_QUERY_REGEXP_REGISTER | MLX5_CMD_OP_QUERY_RMP |
        MLX5_CMD_OP_QUERY_RQ | MLX5_CMD_OP_QUERY_RQT | MLX5_CMD_OP_QUERY_RSV_RESOURCES |
        MLX5_CMD_OP_QUERY_SCHED_QUEUE | MLX5_CMD_OP_QUERY_SCHEDULING_ELEMENT | MLX5_CMD_OP_QUERY_SF_PARTITION |
        MLX5_CMD_OP_QUERY_SPECIAL_CONTEXTS | MLX5_CMD_OP_QUERY_SQ | MLX5_CMD_OP_QUERY_SRQ |
        MLX5_CMD_OP_QUERY_TIR | MLX5_CMD_OP_QUERY_TIS | MLX5_CMD_OP_QUERY_UCTX | MLX5_CMD_OP_QUERY_UMEM |
        MLX5_CMD_OP_QUERY_VHCA_MIGRATION_STATE | MLX5_CMD_OP_QUERY_VHCA_STATE | MLX5_CMD_OP_QUERY_VNIC_ENV |
        MLX5_CMD_OP_QUERY_VPORT_COUNTER | MLX5_CMD_OP_QUERY_VPORT_STATE | MLX5_CMD_OP_QUERY_WOL_ROL |
        MLX5_CMD_OP_QUERY_XRC_SRQ | MLX5_CMD_OP_QUERY_XRQ_DC_PARAMS_ENTRY | MLX5_CMD_OP_QUERY_XRQ_ERROR_PARAMS |
        MLX5_CMD_OP_QUERY_XRQ | MLX5_CMD_OP_USER_QUERY_XRQ_DC_PARAMS_ENTRY |
        MLX5_CMD_OP_USER_QUERY_XRQ_ERROR_PARAMS | MLX5_CMD_OP_QUERY_ADJACENT_FUNCTIONS_ID |
        MLX5_CMD_OP_QUERY_DELEGATED_VHCA => scope >= FWCTL_RPC_DEBUG_READ_ONLY,
        MLX5_CMD_OP_SET_DIAGNOSTIC_PARAMS => scope >= FWCTL_RPC_DEBUG_WRITE,
        MLX5_CMD_OP_ACCESS_REG | MLX5_CMD_OP_ACCESS_REGISTER_USER => {
            if op_mod == 0 { true } else { scope >= FWCTL_RPC_DEBUG_READ_ONLY }
        }
        _ => false,
    }
}

unsafe fn mlx5ctl_fw_rpc(uctx: *mut fwctl_uctx, scope: fwctl_rpc_scope, rpc_in: *mut core::ffi::c_void, in_len: usize, out_len: *mut usize) -> *mut core::ffi::c_void {
    let mcdev = container_of!((*uctx).fwctl, mlx5ctl_dev, fwctl);
    let mfd = container_of!(uctx, mlx5ctl_uctx, uctx);
    if in_len < MLX5_ST_SZ_BYTES(mbox_in_hdr) || *out_len < MLX5_ST_SZ_BYTES(mbox_out_hdr) { return ERR_PTR(-EMSGSIZE); }
    if !mlx5ctl_validate_rpc(rpc_in, scope) { return ERR_PTR(-EBADMSG); }
    let rpc_out = if *out_len <= in_len { rpc_in } else { kvzalloc(*out_len, GFP_KERNEL) };
    if rpc_out.is_null() { return ERR_PTR(-ENOMEM); }
    MLX5_SET(mbox_in_hdr, rpc_in, uid, (*mfd).uctx_uid);
    let ret = mlx5_cmd_do((*mcdev).mdev, rpc_in, in_len, rpc_out, *out_len);
    if ret != 0 && ret != -EREMOTEIO {
        if rpc_out != rpc_in { kvfree(rpc_out); }
        return ERR_PTR(ret);
    }
    rpc_out
}

pub static mlx5ctl_ops: fwctl_ops = fwctl_ops {
    device_type: FWCTL_DEVICE_TYPE_MLX5,
    uctx_size: core::mem::size_of::<mlx5ctl_uctx>(),
    open_uctx: mlx5ctl_open_uctx,
    close_uctx: mlx5ctl_close_uctx,
    info: mlx5ctl_info,
    fw_rpc: mlx5ctl_fw_rpc,
};

unsafe fn mlx5ctl_probe(adev: *mut auxiliary_device, _id: *const auxiliary_device_id) -> i32 {
    let madev = container_of!(adev, mlx5_adev, adev);
    let mdev = (*madev).mdev;
    let mcdev = fwctl_alloc_device(&(*mdev).pdev.dev, &mlx5ctl_ops, mlx5ctl_dev, fwctl);
    if mcdev.is_null() { return -ENOMEM; }
    (*mcdev).mdev = mdev;
    let ret = fwctl_register(&mut (*mcdev).fwctl);
    if ret != 0 { return ret; }
    auxiliary_set_drvdata(adev, mcdev);
    0
}

unsafe fn mlx5ctl_remove(adev: *mut auxiliary_device) {
    let mcdev = auxiliary_get_drvdata(adev);
    fwctl_unregister(&mut (*mcdev).fwctl);
    fwctl_put(&mut (*mcdev).fwctl);
}

pub static mlx5ctl_id_table: [auxiliary_device_id; 2] = [
    auxiliary_device_id { name: MLX5_ADEV_NAME ".fwctl" },
    auxiliary_device_id { name: "" },
];

pub static mut mlx5ctl_driver: auxiliary_driver = auxiliary_driver {
    name: "mlx5_fwctl",
    probe: mlx5ctl_probe,
    remove: mlx5ctl_remove,
    id_table: mlx5ctl_id_table.as_ptr(),
};

// module_auxiliary_driver(mlx5ctl_driver);
// MODULE_IMPORT_NS("FWCTL");
// MODULE_DESCRIPTION("mlx5 ConnectX fwctl driver");
// MODULE_AUTHOR("Saeed Mahameed <saeedm@nvidia.com>");
// MODULE_LICENSE("Dual BSD/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
