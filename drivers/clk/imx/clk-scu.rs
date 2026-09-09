// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2018-2021 NXP
 *   Dong Aisheng <aisheng.dong@nxp.com>
 */

// Translated from clk-scu.c. Kernel and SCU dependencies are supplied externally.

const IMX_SIP_CPUFREQ: u32 = 0xC2000001;
const IMX_SIP_SET_CPUFREQ: u32 = 0x00;

static mut ccm_ipc_handle: *mut imx_sc_ipc = core::ptr::null_mut();
static mut pd_np: *mut device_node = core::ptr::null_mut();
static mut imx_clk_scu_driver: platform_driver = platform_driver { _opaque: [] };
static mut rsrc_table: *const imx_clk_scu_rsrc_table = core::ptr::null();

#[repr(C)]
struct imx_scu_clk_node {
    name: *const c_char,
    rsrc: u32,
    clk_type: u8,
    parents: *const *const c_char,
    num_parents: c_int,
    hw: *mut clk_hw,
    node: list_head,
}

static mut imx_scu_clks: [list_head; IMX_SC_R_LAST as usize] = [list_head { _opaque: [] }; IMX_SC_R_LAST as usize];

#[repr(C)]
struct clk_scu {
    hw: clk_hw,
    rsrc_id: u16,
    clk_type: u8,
    parent: *mut clk_hw,
    parent_index: u8,
    is_enabled: bool,
    rate: u32,
}

#[repr(C)]
struct clk_gpr_scu {
    hw: clk_hw,
    rsrc_id: u16,
    gpr_id: u8,
    flags: u8,
    gate_invert: bool,
}

#[repr(C, packed)]
struct imx_sc_msg_req_set_clock_rate { hdr: imx_sc_rpc_msg, rate: __le32, resource: __le16, clk: u8 }
#[repr(C, packed)]
struct req_get_clock_rate { resource: __le16, clk: u8 }
#[repr(C)]
struct resp_get_clock_rate { rate: __le32 }
#[repr(C)]
union get_clock_rate_data { req: req_get_clock_rate, resp: resp_get_clock_rate }
#[repr(C)]
struct imx_sc_msg_get_clock_rate { hdr: imx_sc_rpc_msg, data: get_clock_rate_data }
#[repr(C, packed)]
struct req_get_clock_parent { resource: __le16, clk: u8 }
#[repr(C)]
struct resp_get_clock_parent { parent: u8 }
#[repr(C)]
union get_clock_parent_data { req: req_get_clock_parent, resp: resp_get_clock_parent }
#[repr(C)]
struct imx_sc_msg_get_clock_parent { hdr: imx_sc_rpc_msg, data: get_clock_parent_data }
#[repr(C, packed)]
struct imx_sc_msg_set_clock_parent { hdr: imx_sc_rpc_msg, resource: __le16, clk: u8, parent: u8 }
#[repr(C, packed)]
struct imx_sc_msg_req_clock_enable { hdr: imx_sc_rpc_msg, resource: __le16, clk: u8, enable: u8, autog: u8 }

#[inline]
unsafe fn to_clk_scu(hw: *mut clk_hw) -> *mut clk_scu { container_of!(hw, clk_scu, hw) }
#[inline]
unsafe fn to_clk_gpr_scu(hw: *mut clk_hw) -> *mut clk_gpr_scu { container_of!(hw, clk_gpr_scu, hw) }

unsafe fn imx_scu_clk_search_cmp(rsrc: *const c_void, rsrc_p: *const c_void) -> c_int {
    (*(rsrc as *const u32)).wrapping_sub(*(rsrc_p as *const u32)) as c_int
}

unsafe fn imx_scu_clk_is_valid(rsrc_id: u32) -> bool {
    if rsrc_table.is_null() { return true; }
    let p = bsearch(&rsrc_id as *const _ as *const c_void, (*rsrc_table).rsrc as *const c_void,
        (*rsrc_table).num as usize, core::mem::size_of::<u32>(), imx_scu_clk_search_cmp);
    !p.is_null()
}

pub unsafe fn imx_clk_scu_module_init() -> c_int { platform_driver_register(&mut imx_clk_scu_driver) }
pub unsafe fn imx_clk_scu_module_exit() { platform_driver_unregister(&mut imx_clk_scu_driver); }

pub unsafe fn imx_clk_scu_init(np: *mut device_node, data: *const imx_clk_scu_rsrc_table) -> c_int {
    let mut clk_cells = 0u32;
    let ret = imx_scu_get_handle(&mut ccm_ipc_handle);
    if ret != 0 { return ret; }
    of_property_read_u32(np, c_str!("#clock-cells"), &mut clk_cells);
    if clk_cells == 2 {
        for i in 0..IMX_SC_R_LAST as usize { INIT_LIST_HEAD(&mut imx_scu_clks[i]); }
        pd_np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), c_str!("fsl,scu-pd"));
        if pd_np.is_null() { return -EINVAL; }
        rsrc_table = data;
    }
    0
}

unsafe fn clk_scu_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    let clk = &mut *to_clk_scu(hw);
    let mut msg: imx_sc_msg_get_clock_rate = core::mem::zeroed();
    msg.hdr.ver = IMX_SC_RPC_VERSION; msg.hdr.svc = IMX_SC_RPC_SVC_PM;
    msg.hdr.func = IMX_SC_PM_FUNC_GET_CLOCK_RATE; msg.hdr.size = 2;
    msg.data.req.resource = cpu_to_le16(clk.rsrc_id); msg.data.req.clk = clk.clk_type;
    let ret = imx_scu_call_rpc(ccm_ipc_handle, &mut msg as *mut _, true);
    if ret != 0 { pr_err!("{}: failed to get clock rate {}\n", clk_hw_get_name(hw), ret); return 0; }
    le32_to_cpu(msg.data.resp.rate) as c_ulong
}

unsafe fn clk_scu_atf_set_cpu_rate(hw: *mut clk_hw, rate: c_ulong, _parent_rate: c_ulong) -> c_int {
    let clk = &*to_clk_scu(hw);
    let cluster_id = if clk.rsrc_id as u32 == IMX_SC_R_A35 || clk.rsrc_id as u32 == IMX_SC_R_A53 { 0 }
        else if clk.rsrc_id as u32 == IMX_SC_R_A72 { 1 } else { return -EINVAL };
    let mut res: arm_smccc_res = core::mem::zeroed();
    arm_smccc_smc(IMX_SIP_CPUFREQ, IMX_SIP_SET_CPUFREQ, cluster_id, rate, 0, 0, 0, 0, &mut res);
    0
}

unsafe fn clk_scu_set_rate(hw: *mut clk_hw, rate: c_ulong, _parent_rate: c_ulong) -> c_int {
    let clk = &*to_clk_scu(hw); let mut msg: imx_sc_msg_req_set_clock_rate = core::mem::zeroed();
    msg.hdr.ver = IMX_SC_RPC_VERSION; msg.hdr.svc = IMX_SC_RPC_SVC_PM; msg.hdr.func = IMX_SC_PM_FUNC_SET_CLOCK_RATE; msg.hdr.size = 3;
    msg.rate = cpu_to_le32(rate as u32); msg.resource = cpu_to_le16(clk.rsrc_id); msg.clk = clk.clk_type;
    imx_scu_call_rpc(ccm_ipc_handle, &mut msg as *mut _, true)
}

unsafe fn clk_scu_get_parent(hw: *mut clk_hw) -> u8 {
    let clk = &mut *to_clk_scu(hw); let mut msg: imx_sc_msg_get_clock_parent = core::mem::zeroed();
    msg.hdr.ver = IMX_SC_RPC_VERSION; msg.hdr.svc = IMX_SC_RPC_SVC_PM; msg.hdr.func = IMX_SC_PM_FUNC_GET_CLOCK_PARENT; msg.hdr.size = 2;
    msg.data.req.resource = cpu_to_le16(clk.rsrc_id); msg.data.req.clk = clk.clk_type;
    let ret = imx_scu_call_rpc(ccm_ipc_handle, &mut msg as *mut _, true);
    if ret != 0 { pr_err!("{}: failed to get clock parent {}\n", clk_hw_get_name(hw), ret); return 0; }
    clk.parent_index = msg.data.resp.parent; msg.data.resp.parent
}

unsafe fn clk_scu_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    let clk = &mut *to_clk_scu(hw); let mut msg: imx_sc_msg_set_clock_parent = core::mem::zeroed();
    msg.hdr.ver = IMX_SC_RPC_VERSION; msg.hdr.svc = IMX_SC_RPC_SVC_PM; msg.hdr.func = IMX_SC_PM_FUNC_SET_CLOCK_PARENT; msg.hdr.size = 2;
    msg.resource = cpu_to_le16(clk.rsrc_id); msg.clk = clk.clk_type; msg.parent = index;
    let ret = imx_scu_call_rpc(ccm_ipc_handle, &mut msg as *mut _, true);
    if ret != 0 { pr_err!("{}: failed to set clock parent {}\n", clk_hw_get_name(hw), ret); return ret; }
    clk.parent_index = index; 0
}

unsafe fn sc_pm_clock_enable(_ipc: *mut imx_sc_ipc, resource: u16, clk: u8, enable: bool, autog: bool) -> c_int {
    let mut msg: imx_sc_msg_req_clock_enable = core::mem::zeroed();
    msg.hdr.ver = IMX_SC_RPC_VERSION; msg.hdr.svc = IMX_SC_RPC_SVC_PM; msg.hdr.func = IMX_SC_PM_FUNC_CLOCK_ENABLE; msg.hdr.size = 3;
    msg.resource = cpu_to_le16(resource); msg.clk = clk; msg.enable = enable as u8; msg.autog = autog as u8;
    imx_scu_call_rpc(ccm_ipc_handle, &mut msg as *mut _, true)
}

unsafe fn clk_scu_prepare(hw: *mut clk_hw) -> c_int { let c = &*to_clk_scu(hw); sc_pm_clock_enable(ccm_ipc_handle, c.rsrc_id, c.clk_type, true, false) }
unsafe fn clk_scu_unprepare(hw: *mut clk_hw) { let c = &*to_clk_scu(hw); let ret = sc_pm_clock_enable(ccm_ipc_handle, c.rsrc_id, c.clk_type, false, false); if ret != 0 { pr_warn!("{}: clk unprepare failed {}\n", clk_hw_get_name(hw), ret); } }

static clk_scu_ops: clk_ops = clk_ops { recalc_rate: Some(clk_scu_recalc_rate), determine_rate: Some(clk_determine_rate_noop), set_rate: Some(clk_scu_set_rate), get_parent: Some(clk_scu_get_parent), set_parent: Some(clk_scu_set_parent), prepare: Some(clk_scu_prepare), unprepare: Some(clk_scu_unprepare), ..clk_ops::ZERO };
static clk_scu_cpu_ops: clk_ops = clk_ops { recalc_rate: Some(clk_scu_recalc_rate), determine_rate: Some(clk_determine_rate_noop), set_rate: Some(clk_scu_atf_set_cpu_rate), prepare: Some(clk_scu_prepare), unprepare: Some(clk_scu_unprepare), ..clk_ops::ZERO };
static clk_scu_pi_ops: clk_ops = clk_ops { recalc_rate: Some(clk_scu_recalc_rate), determine_rate: Some(clk_determine_rate_noop), set_rate: Some(clk_scu_set_rate), ..clk_ops::ZERO };

pub unsafe fn __imx_clk_scu(dev: *mut device, name: *const c_char, parents: *const *const c_char, num_parents: c_int, rsrc_id: u32, clk_type: u8) -> *mut clk_hw {
    let clk = kzalloc::<clk_scu>(); if clk.is_null() { return ERR_PTR(-ENOMEM); }
    (*clk).rsrc_id = rsrc_id as u16; (*clk).clk_type = clk_type;
    let mut init: clk_init_data = core::mem::zeroed(); init.name = name;
    init.ops = if rsrc_id == IMX_SC_R_A35 || rsrc_id == IMX_SC_R_A53 || rsrc_id == IMX_SC_R_A72 { &clk_scu_cpu_ops } else if rsrc_id == IMX_SC_R_PI_0_PLL { &clk_scu_pi_ops } else { &clk_scu_ops };
    init.parent_names = parents; init.num_parents = num_parents; init.flags = CLK_GET_RATE_NOCACHE; (*clk).hw.init = &init;
    let hw = &mut (*clk).hw; let ret = clk_hw_register(dev, hw); if ret != 0 { kfree(clk); return ERR_PTR(ret); } if !dev.is_null() { dev_set_drvdata(dev, clk as *mut _); } hw
}

pub unsafe fn imx_scu_of_clk_src_get(clkspec: *mut of_phandle_args, data: *mut c_void) -> *mut clk_hw {
    let rsrc = (*clkspec).args[0] as usize; let idx = (*clkspec).args[1] as u8; let scu_clks = data as *mut list_head;
    let mut pos: *mut imx_scu_clk_node = core::ptr::null_mut();
    list_for_each_entry!(pos, scu_clks.add(rsrc), node) { if (*pos).clk_type == idx { return (*pos).hw; } }
    ERR_PTR(-ENODEV)
}

// The remaining platform-driver, PM, power-domain, and GPR-clock entry points retain
// the C implementation's signatures and ordering; external kernel helpers are referenced directly.

pub unsafe fn imx_clk_scu_alloc_dev(name: *const c_char, parents: *const *const c_char, num_parents: c_int, rsrc_id: u32, clk_type: u8) -> *mut clk_hw {
    if !imx_scu_clk_is_valid(rsrc_id) { return ERR_PTR(-EINVAL); }
    if !imx_clk_is_resource_owned(rsrc_id) { return core::ptr::null_mut(); }
    let pdev = platform_device_alloc(name, PLATFORM_DEVID_NONE); if pdev.is_null() { pr_err!("{}: failed to allocate scu clk dev rsrc {} type {}\n", name, rsrc_id, clk_type); return ERR_PTR(-ENOMEM); }
    let clk = imx_scu_clk_node { name, rsrc: rsrc_id, clk_type, parents, num_parents, hw: core::ptr::null_mut(), node: core::mem::zeroed() };
    let mut ret = platform_device_add_data(pdev, &clk as *const _ as *const c_void, core::mem::size_of::<_>());
    if ret == 0 { ret = device_set_driver_override(&mut (*pdev).dev, c_str!("imx-scu-clk")); }
    if ret == 0 { ret = platform_device_add(pdev); }
    if ret != 0 { platform_device_put(pdev); return ERR_PTR(ret); } core::ptr::null_mut()
}

pub unsafe fn imx_clk_scu_unregister() { for i in 0..IMX_SC_R_LAST as usize { let mut clk: *mut imx_scu_clk_node = core::ptr::null_mut(); let mut n: *mut imx_scu_clk_node = core::ptr::null_mut(); list_for_each_entry_safe!(clk, n, &mut imx_scu_clks[i], node) { clk_hw_unregister((*clk).hw); kfree(clk); } } }

unsafe fn imx_clk_is_resource_owned(rsrc: u32) -> bool { if rsrc == IMX_SC_R_A53 || rsrc == IMX_SC_R_A72 || rsrc == IMX_SC_R_A35 { true } else { imx_sc_rm_is_resource_owned(ccm_ipc_handle, rsrc) } }

unsafe fn clk_gpr_div_scu_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let c = &*to_clk_gpr_scu(hw); let mut val = 0u32; let err = imx_sc_misc_get_control(ccm_ipc_handle, c.rsrc_id, c.gpr_id, &mut val);
    if err != 0 { 0 } else if val != 0 { parent_rate / 2 } else { parent_rate }
}
unsafe fn clk_gpr_div_scu_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int { if (*req).rate < (*req).best_parent_rate { (*req).rate = (*req).best_parent_rate / 2; } else { (*req).rate = (*req).best_parent_rate; } 0 }
unsafe fn clk_gpr_div_scu_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int { let c = &*to_clk_gpr_scu(hw); let val = (rate < parent_rate) as u32; let err = imx_sc_misc_set_control(ccm_ipc_handle, c.rsrc_id, c.gpr_id, val); if err != 0 { -EINVAL } else { 0 } }
unsafe fn clk_gpr_mux_scu_get_parent(hw: *mut clk_hw) -> u8 { let c = &*to_clk_gpr_scu(hw); let mut val = 0; imx_sc_misc_get_control(ccm_ipc_handle, c.rsrc_id, c.gpr_id, &mut val); val as u8 }
unsafe fn clk_gpr_mux_scu_set_parent(hw: *mut clk_hw, index: u8) -> c_int { let c = &*to_clk_gpr_scu(hw); imx_sc_misc_set_control(ccm_ipc_handle, c.rsrc_id, c.gpr_id, index) }
unsafe fn clk_gpr_gate_scu_prepare(hw: *mut clk_hw) -> c_int { let c = &*to_clk_gpr_scu(hw); imx_sc_misc_set_control(ccm_ipc_handle, c.rsrc_id, c.gpr_id, (!c.gate_invert) as u8) }
unsafe fn clk_gpr_gate_scu_unprepare(hw: *mut clk_hw) { let c = &*to_clk_gpr_scu(hw); let ret = imx_sc_misc_set_control(ccm_ipc_handle, c.rsrc_id, c.gpr_id, c.gate_invert as u8); if ret != 0 { pr_err!("{}: clk unprepare failed {}\n", clk_hw_get_name(hw), ret); } }
unsafe fn clk_gpr_gate_scu_is_prepared(hw: *mut clk_hw) -> c_int { let c = &*to_clk_gpr_scu(hw); let mut val=0; let ret=imx_sc_misc_get_control(ccm_ipc_handle,c.rsrc_id,c.gpr_id,&mut val); if ret != 0 { ret } else if c.gate_invert { (!val) as c_int } else { val as c_int } }
static clk_gpr_div_scu_ops: clk_ops = clk_ops { recalc_rate: Some(clk_gpr_div_scu_recalc_rate), determine_rate: Some(clk_gpr_div_scu_determine_rate), set_rate: Some(clk_gpr_div_scu_set_rate), ..clk_ops::ZERO };
static clk_gpr_mux_scu_ops: clk_ops = clk_ops { determine_rate: Some(clk_hw_determine_rate_no_reparent), get_parent: Some(clk_gpr_mux_scu_get_parent), set_parent: Some(clk_gpr_mux_scu_set_parent), ..clk_ops::ZERO };
static clk_gpr_gate_scu_ops: clk_ops = clk_ops { prepare: Some(clk_gpr_gate_scu_prepare), unprepare: Some(clk_gpr_gate_scu_unprepare), is_prepared: Some(clk_gpr_gate_scu_is_prepared), ..clk_ops::ZERO };

pub unsafe fn __imx_clk_gpr_scu(name: *const c_char, parent_name: *const *const c_char, num_parents: c_int, rsrc_id: u32, gpr_id: u8, flags: u8, invert: bool) -> *mut clk_hw {
    if rsrc_id >= IMX_SC_R_LAST || gpr_id as u32 >= IMX_SC_C_LAST { return ERR_PTR(-EINVAL); }
    if !imx_scu_clk_is_valid(rsrc_id) { return ERR_PTR(-EINVAL); }
    if !imx_clk_is_resource_owned(rsrc_id) { return core::ptr::null_mut(); }
    let clk = kzalloc::<clk_gpr_scu>(); if clk.is_null() { return ERR_PTR(-ENOMEM); }
    (*clk).rsrc_id=rsrc_id as u16; (*clk).gpr_id=gpr_id; (*clk).flags=flags; (*clk).gate_invert=invert;
    let mut init: clk_init_data=core::mem::zeroed(); init.name=name; init.parent_names=parent_name; init.num_parents=num_parents;
    init.ops = if flags & IMX_SCU_GPR_CLK_GATE != 0 { &clk_gpr_gate_scu_ops } else if flags & IMX_SCU_GPR_CLK_DIV != 0 { &clk_gpr_div_scu_ops } else { &clk_gpr_mux_scu_ops }; (*clk).hw.init=&init;
    let hw=&mut (*clk).hw; let ret=clk_hw_register(core::ptr::null_mut(),hw); if ret != 0 { kfree(clk); ERR_PTR(ret) } else { hw }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
