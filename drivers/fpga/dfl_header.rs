/* SPDX-License-Identifier: GPL-2.0 */
/* Driver Header File for FPGA Device Feature List (DFL) Support */

/* C dependencies supplied by the surrounding kernel translation. */

pub const MAX_DFL_FPGA_PORT_NUM: i32 = 4;
pub const MAX_DFL_FEATURE_DEV_NUM: i32 = MAX_DFL_FPGA_PORT_NUM + 1;

pub const FEATURE_ID_FIU_HEADER: u64 = 0xfe;
pub const FEATURE_ID_AFU: u64 = 0xff;

pub const FME_FEATURE_ID_HEADER: u64 = FEATURE_ID_FIU_HEADER;
pub const FME_FEATURE_ID_THERMAL_MGMT: u64 = 0x1;
pub const FME_FEATURE_ID_POWER_MGMT: u64 = 0x2;
pub const FME_FEATURE_ID_GLOBAL_IPERF: u64 = 0x3;
pub const FME_FEATURE_ID_GLOBAL_ERR: u64 = 0x4;
pub const FME_FEATURE_ID_PR_MGMT: u64 = 0x5;
pub const FME_FEATURE_ID_HSSI: u64 = 0x6;
pub const FME_FEATURE_ID_GLOBAL_DPERF: u64 = 0x7;

pub const PORT_FEATURE_ID_HEADER: u64 = FEATURE_ID_FIU_HEADER;
pub const PORT_FEATURE_ID_AFU: u64 = FEATURE_ID_AFU;
pub const PORT_FEATURE_ID_ERROR: u64 = 0x10;
pub const PORT_FEATURE_ID_UMSG: u64 = 0x11;
pub const PORT_FEATURE_ID_UINT: u64 = 0x12;
pub const PORT_FEATURE_ID_STP: u64 = 0x13;

pub const DFH: u64 = 0x0;
pub const GUID_L: u64 = 0x8;
pub const GUID_H: u64 = 0x10;
pub const NEXT_AFU: u64 = 0x18;
pub const DFH_SIZE: u64 = 0x8;

pub const DFH_ID: u64 = 0xfff;
pub const DFH_ID_FIU_FME: u64 = 0;
pub const DFH_ID_FIU_PORT: u64 = 1;
pub const DFH_REVISION: u64 = 0xf000;
pub const DFH_NEXT_HDR_OFST: u64 = 0x0000_00ff_ffff_0000;
pub const DFH_EOL: u64 = 1u64 << 40;
pub const DFH_VERSION: u64 = 0x0ff0_0000_0000_0000;
pub const DFH_TYPE: u64 = 0xf000_0000_0000_0000;
pub const DFH_TYPE_AFU: u64 = 1;
pub const DFH_TYPE_PRIVATE: u64 = 3;
pub const DFH_TYPE_FIU: u64 = 4;

pub const DFHv1_CSR_ADDR: u64 = 0x18;
pub const DFHv1_CSR_SIZE_GRP: u64 = 0x20;
pub const DFHv1_PARAM_HDR: u64 = 0x28;
pub const DFHv1_CSR_ADDR_REL: u64 = 1;
pub const DFHv1_CSR_ADDR_MASK: u64 = 0xffff_ffff_ffff_fffe;
pub const DFHv1_CSR_SIZE_GRP_INSTANCE_ID: u64 = 0xffff;
pub const DFHv1_CSR_SIZE_GRP_GROUPING_ID: u64 = 0x7fff_0000;
pub const DFHv1_CSR_SIZE_GRP_HAS_PARAMS: u64 = 1u64 << 31;
pub const DFHv1_CSR_SIZE_GRP_SIZE: u64 = 0xffff_ffff_0000_0000;
pub const DFHv1_PARAM_HDR_ID: u64 = 0xffff;
pub const DFHv1_PARAM_HDR_VER: u64 = 0xffff_0000;
pub const DFHv1_PARAM_HDR_NEXT_OFFSET: u64 = 0xffff_fff8_0000_0000;
pub const DFHv1_PARAM_HDR_NEXT_EOP: u64 = 1u64 << 32;
pub const DFHv1_PARAM_DATA: u64 = 0x08;
pub const DFHv1_PARAM_ID_MSI_X: u64 = 0x1;
pub const DFHv1_PARAM_MSI_X_NUMV: u64 = 0xffff_ffff_0000_0000;
pub const DFHv1_PARAM_MSI_X_STARTV: u64 = 0xffff_ffff;
pub const NEXT_AFU_NEXT_DFH_OFST: u64 = 0x00ff_ffff;

pub const FME_HDR_DFH: u64 = DFH;
pub const FME_HDR_GUID_L: u64 = GUID_L;
pub const FME_HDR_GUID_H: u64 = GUID_H;
pub const FME_HDR_NEXT_AFU: u64 = NEXT_AFU;
pub const FME_HDR_CAP: u64 = 0x30;
#[inline]
pub const fn FME_HDR_PORT_OFST(n: u64) -> u64 { 0x38 + n * 0x8 }
pub const FME_PORT_OFST_BAR_SKIP: u64 = 7;
pub const FME_HDR_BITSTREAM_ID: u64 = 0x60;
pub const FME_HDR_BITSTREAM_MD: u64 = 0x68;
pub const FME_CAP_FABRIC_VERID: u64 = 0xff;
pub const FME_CAP_SOCKET_ID: u64 = 1u64 << 8;
pub const FME_CAP_PCIE0_LINK_AVL: u64 = 1u64 << 12;
pub const FME_CAP_PCIE1_LINK_AVL: u64 = 1u64 << 13;
pub const FME_CAP_COHR_LINK_AVL: u64 = 1u64 << 14;
pub const FME_CAP_IOMMU_AVL: u64 = 1u64 << 16;
pub const FME_CAP_NUM_PORTS: u64 = 0x000e_0000;
pub const FME_CAP_ADDR_WIDTH: u64 = 0x3f00_0000;
pub const FME_CAP_CACHE_SIZE: u64 = 0x000f_ff00_0000_0000;
pub const FME_CAP_CACHE_ASSOC: u64 = 0x00f0_0000_0000_0000;
pub const FME_PORT_OFST_DFH_OFST: u64 = 0x00ff_ffff;
pub const FME_PORT_OFST_BAR_ID: u64 = 0x0000_0007_0000_0000;
pub const FME_PORT_OFST_ACC_CTRL: u64 = 1u64 << 55;
pub const FME_PORT_OFST_ACC_PF: u64 = 0;
pub const FME_PORT_OFST_ACC_VF: u64 = 1;
pub const FME_PORT_OFST_IMP: u64 = 1u64 << 60;
pub const FME_ERROR_CAP: u64 = 0x70;
pub const FME_ERROR_CAP_SUPP_INT: u64 = 1;
pub const FME_ERROR_CAP_INT_VECT: u64 = 0x1ffe;

pub const PORT_HDR_DFH: u64 = DFH;
pub const PORT_HDR_GUID_L: u64 = GUID_L;
pub const PORT_HDR_GUID_H: u64 = GUID_H;
pub const PORT_HDR_NEXT_AFU: u64 = NEXT_AFU;
pub const PORT_HDR_CAP: u64 = 0x30;
pub const PORT_HDR_CTRL: u64 = 0x38;
pub const PORT_HDR_STS: u64 = 0x40;
pub const PORT_HDR_USRCLK_CMD0: u64 = 0x50;
pub const PORT_HDR_USRCLK_CMD1: u64 = 0x58;
pub const PORT_HDR_USRCLK_STS0: u64 = 0x60;
pub const PORT_HDR_USRCLK_STS1: u64 = 0x68;
pub const PORT_CAP_PORT_NUM: u64 = 0x3;
pub const PORT_CAP_MMIO_SIZE: u64 = 0x00ff_ff00;
pub const PORT_CAP_SUPP_INT_NUM: u64 = 0x000f_0000_0000;
pub const PORT_CTRL_SFTRST: u64 = 1;
pub const PORT_CTRL_LATENCY: u64 = 1u64 << 2;
pub const PORT_CTRL_SFTRST_ACK: u64 = 1u64 << 4;
pub const PORT_STS_AP2_EVT: u64 = 1u64 << 13;
pub const PORT_STS_AP1_EVT: u64 = 1u64 << 12;
pub const PORT_STS_PWR_STATE: u64 = 0xf00;
pub const PORT_STS_PWR_STATE_NORM: u64 = 0;
pub const PORT_STS_PWR_STATE_AP1: u64 = 1;
pub const PORT_STS_PWR_STATE_AP2: u64 = 2;
pub const PORT_STS_PWR_STATE_AP6: u64 = 6;
pub const PORT_ERROR_CAP: u64 = 0x38;
pub const PORT_ERROR_CAP_SUPP_INT: u64 = 1;
pub const PORT_ERROR_CAP_INT_VECT: u64 = 0x1ffe;
pub const PORT_UINT_CAP: u64 = 0x8;
pub const PORT_UINT_CAP_INT_NUM: u64 = 0xfff;
pub const PORT_UINT_CAP_FST_VECT: u64 = 0x00fff_000;

#[repr(C)]
pub struct dfl_feature_dev_data;

#[repr(C)]
pub struct dfl_fpga_port_ops {
    pub name: *const core::ffi::c_char,
    pub owner: *mut module,
    pub node: list_head,
    pub get_id: Option<unsafe extern "C" fn(*mut dfl_feature_dev_data) -> i32>,
    pub enable_set: Option<unsafe extern "C" fn(*mut dfl_feature_dev_data, bool) -> i32>,
}

extern "C" {
    pub fn dfl_fpga_port_ops_add(ops: *mut dfl_fpga_port_ops);
    pub fn dfl_fpga_port_ops_del(ops: *mut dfl_fpga_port_ops);
    pub fn dfl_fpga_port_ops_get(fdata: *mut dfl_feature_dev_data) -> *mut dfl_fpga_port_ops;
    pub fn dfl_fpga_port_ops_put(ops: *mut dfl_fpga_port_ops);
    pub fn dfl_fpga_check_port_id(fdata: *mut dfl_feature_dev_data, pport_id: *mut core::ffi::c_void) -> i32;
}

#[repr(C)] pub struct dfl_feature_id { pub id: u16 }
#[repr(C)] pub struct dfl_feature_driver { pub id_table: *const dfl_feature_id, pub ops: *const dfl_feature_ops }
#[repr(C)] pub struct dfl_feature_irq_ctx { pub irq: i32, pub trigger: *mut eventfd_ctx, pub name: *mut core::ffi::c_char }

#[repr(C)]
pub struct dfl_feature {
    pub dev: *mut platform_device, pub id: u16, pub revision: u8, pub resource_index: i32,
    pub ioaddr: *mut core::ffi::c_void, pub irq_ctx: *mut dfl_feature_irq_ctx,
    pub nr_irqs: u32, pub ops: *const dfl_feature_ops, pub ddev: *mut dfl_device,
    pub priv_: *mut core::ffi::c_void, pub dfh_version: u8, pub param_size: u32,
    pub params: *mut core::ffi::c_void,
}
pub const FEATURE_DEV_ID_UNUSED: i32 = -1;

#[repr(C)]
pub struct dfl_feature_dev_data {
    pub node: list_head, pub lock: mutex, pub dev: *mut platform_device, pub r#type: dfl_id_type,
    pub pdev_id: i32, pub pdev_name: *const core::ffi::c_char, pub dfl_cdev: *mut dfl_fpga_cdev,
    pub id: i32, pub disable_count: u32, pub excl_open: bool, pub open_count: i32,
    pub private: *mut core::ffi::c_void, pub num: i32, pub features: *mut dfl_feature,
    pub resource_num: i32, pub resources: *mut resource,
}

#[repr(C)] pub struct dfl_feature_platform_data { pub cdev: cdev, pub fdata: *mut dfl_feature_dev_data }

pub unsafe fn dfl_feature_dev_use_begin(fdata: *mut dfl_feature_dev_data, excl: bool) -> i32 {
    if (*fdata).excl_open { return -EBUSY; }
    if excl { if (*fdata).open_count != 0 { return -EBUSY; } (*fdata).excl_open = true; }
    (*fdata).open_count += 1; 0
}
pub unsafe fn dfl_feature_dev_use_end(fdata: *mut dfl_feature_dev_data) {
    (*fdata).excl_open = false;
    if WARN_ON((*fdata).open_count <= 0) { return; }
    (*fdata).open_count -= 1;
}
pub unsafe fn dfl_feature_dev_use_count(fdata: *mut dfl_feature_dev_data) -> i32 { (*fdata).open_count }
pub unsafe fn dfl_fpga_fdata_set_private(fdata: *mut dfl_feature_dev_data, private: *mut core::ffi::c_void) { (*fdata).private = private; }
pub unsafe fn dfl_fpga_fdata_get_private(fdata: *mut dfl_feature_dev_data) -> *mut core::ffi::c_void { (*fdata).private }

#[repr(C)]
pub struct dfl_feature_ops {
    pub init: Option<unsafe extern "C" fn(*mut platform_device, *mut dfl_feature) -> i32>,
    pub uinit: Option<unsafe extern "C" fn(*mut platform_device, *mut dfl_feature)>,
    pub ioctl: Option<unsafe extern "C" fn(*mut platform_device, *mut dfl_feature, u32, usize) -> isize>,
}
pub const DFL_FPGA_FEATURE_DEV_FME: &[u8] = b"dfl-fme\0";
pub const DFL_FPGA_FEATURE_DEV_PORT: &[u8] = b"dfl-port\0";

extern "C" {
    pub fn dfl_fpga_dev_feature_uinit(pdev: *mut platform_device);
    pub fn dfl_fpga_dev_feature_init(pdev: *mut platform_device, feature_drvs: *mut dfl_feature_driver) -> i32;
    pub fn dfl_fpga_dev_ops_register(pdev: *mut platform_device, fops: *const file_operations, owner: *mut module) -> i32;
    pub fn dfl_fpga_dev_ops_unregister(pdev: *mut platform_device);
}

pub unsafe fn dfl_fpga_inode_to_feature_dev_data(inode: *mut inode) -> *mut dfl_feature_dev_data {
    let pdata = container_of((*inode).i_cdev, core::marker::PhantomData::<dfl_feature_platform_data>);
    (*pdata).fdata
}
pub unsafe fn dfl_get_feature_by_id(fdata: *mut dfl_feature_dev_data, id: u16) -> *mut dfl_feature {
    let mut feature = (*fdata).features;
    for _ in 0..(*fdata).num { if (*feature).id == id { return feature; } feature = feature.add(1); }
    core::ptr::null_mut()
}
pub unsafe fn dfl_get_feature_ioaddr_by_id(fdata: *mut dfl_feature_dev_data, id: u16) -> *mut core::ffi::c_void {
    let feature = dfl_get_feature_by_id(fdata, id);
    if !feature.is_null() && !(*feature).ioaddr.is_null() { return (*feature).ioaddr; }
    WARN_ON(true); core::ptr::null_mut()
}
pub unsafe fn to_dfl_feature_dev_data(dev: *mut device) -> *mut dfl_feature_dev_data {
    let pdata = dev_get_platdata(dev); (*pdata).fdata
}
pub unsafe fn dfl_fpga_fdata_to_parent(fdata: *mut dfl_feature_dev_data) -> *mut device {
    (*(*(*fdata).dev).dev).parent.unwrap().parent.unwrap()
}
pub unsafe fn dfl_feature_is_fme(base: *mut core::ffi::c_void) -> bool {
    let v = readq(base.add(DFH as usize)); ((v & DFH_TYPE) >> 60) == DFH_TYPE_FIU && (v & DFH_ID) == DFH_ID_FIU_FME
}
pub unsafe fn dfl_feature_is_port(base: *mut core::ffi::c_void) -> bool {
    let v = readq(base.add(DFH as usize)); ((v & DFH_TYPE) >> 60) == DFH_TYPE_FIU && (v & DFH_ID) == DFH_ID_FIU_PORT
}
pub unsafe fn dfl_feature_revision(base: *mut core::ffi::c_void) -> u8 { ((readq(base.add(DFH as usize)) & DFH_REVISION) >> 12) as u8 }

#[repr(C)] pub struct dfl_fpga_enum_info { pub dev: *mut device, pub dfls: list_head, pub nr_irqs: u32, pub irq_table: *mut i32 }
#[repr(C)] pub struct dfl_fpga_enum_dfl { pub start: resource_size_t, pub len: resource_size_t, pub node: list_head }
extern "C" {
    pub fn dfl_fpga_enum_info_alloc(dev: *mut device) -> *mut dfl_fpga_enum_info;
    pub fn dfl_fpga_enum_info_add_dfl(info: *mut dfl_fpga_enum_info, start: resource_size_t, len: resource_size_t) -> i32;
    pub fn dfl_fpga_enum_info_add_irq(info: *mut dfl_fpga_enum_info, nr_irqs: u32, irq_table: *mut i32) -> i32;
    pub fn dfl_fpga_enum_info_free(info: *mut dfl_fpga_enum_info);
}
#[repr(C)] pub struct dfl_fpga_cdev { pub parent: *mut device, pub region: *mut fpga_region, pub fme_dev: *mut device, pub lock: mutex, pub port_dev_list: list_head, pub released_port_num: i32 }
extern "C" {
    pub fn dfl_fpga_feature_devs_enumerate(info: *mut dfl_fpga_enum_info) -> *mut dfl_fpga_cdev;
    pub fn dfl_fpga_feature_devs_remove(cdev: *mut dfl_fpga_cdev);
    pub fn __dfl_fpga_cdev_find_port_data(cdev: *mut dfl_fpga_cdev, data: *mut core::ffi::c_void, r#match: Option<unsafe extern "C" fn(*mut dfl_feature_dev_data, *mut core::ffi::c_void) -> i32>) -> *mut dfl_feature_dev_data;
    pub fn dfl_fpga_cdev_release_port(cdev: *mut dfl_fpga_cdev, port_id: i32) -> i32;
    pub fn dfl_fpga_cdev_assign_port(cdev: *mut dfl_fpga_cdev, port_id: i32) -> i32;
    pub fn dfl_fpga_cdev_config_ports_pf(cdev: *mut dfl_fpga_cdev);
    pub fn dfl_fpga_cdev_config_ports_vf(cdev: *mut dfl_fpga_cdev, num_vf: i32) -> i32;
    pub fn dfl_fpga_set_irq_triggers(feature: *mut dfl_feature, start: u32, count: u32, fds: *mut i32) -> i32;
    pub fn dfl_feature_ioctl_get_num_irqs(pdev: *mut platform_device, feature: *mut dfl_feature, arg: usize) -> isize;
    pub fn dfl_feature_ioctl_set_irq(pdev: *mut platform_device, feature: *mut dfl_feature, arg: usize) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
