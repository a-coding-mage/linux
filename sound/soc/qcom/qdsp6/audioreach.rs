// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020, Linaro Limited

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type uint32_t = u32;
type phys_addr_t = u64;

const APM_SUB_GRAPH_CFG_NPROP: u32 = 3;

const fn ALIGN(x: usize, a: usize) -> usize {
    (x + a - 1) & !(a - 1)
}

const fn BIT(n: i32) -> u32 {
    1u32 << (n as u32)
}

const fn lower_32_bits(x: phys_addr_t) -> u32 {
    x as u32
}

const fn upper_32_bits(x: phys_addr_t) -> u32 {
    (x >> 32) as u32
}

#[repr(C, packed)]
pub struct apm_sub_graph_data {
    pub sub_graph_cfg: apm_sub_graph_cfg,
    pub perf_data: apm_prop_data,
    pub perf: apm_sg_prop_id_perf_mode,
    pub dir_data: apm_prop_data,
    pub dir: apm_sg_prop_id_direction,
    pub sid_data: apm_prop_data,
    pub sid: apm_sg_prop_id_scenario_id,
}

#[repr(C, packed)]
pub struct apm_sub_graph_params {
    pub param_data: apm_module_param_data,
    pub num_sub_graphs: uint32_t,
    pub sg_cfg: [apm_sub_graph_data; 0],
}

fn APM_SUB_GRAPH_PSIZE(n: usize) -> usize {
    ALIGN(size_of::<apm_sub_graph_params>() + n * size_of::<apm_sub_graph_data>(), 8)
}

#[repr(C, packed)]
pub struct apm_container_obj {
    pub container_cfg: apm_container_cfg,
    /* Capability ID list */
    pub cap_data: apm_prop_data,
    pub num_capability_id: uint32_t,
    pub capability_id: uint32_t,
    /* Container graph Position */
    pub pos_data: apm_prop_data,
    pub pos: apm_cont_prop_id_graph_pos,
    /* Container Stack size */
    pub stack_data: apm_prop_data,
    pub stack: apm_cont_prop_id_stack_size,
    /* Container proc domain id */
    pub domain_data: apm_prop_data,
    pub domain: apm_cont_prop_id_domain,
}

#[repr(C, packed)]
pub struct apm_container_params {
    pub param_data: apm_module_param_data,
    pub num_containers: uint32_t,
    pub cont_obj: [apm_container_obj; 0],
}

fn APM_CONTAINER_PSIZE(n: usize) -> usize {
    ALIGN(size_of::<apm_container_params>() + n * size_of::<apm_container_obj>(), 8)
}

#[repr(C, packed)]
pub struct apm_mod_list_obj {
    /* Modules list cfg */
    pub sub_graph_id: uint32_t,
    pub container_id: uint32_t,
    pub num_modules: uint32_t,
    pub mod_cfg: [apm_module_obj; 0],
}

fn APM_MOD_LIST_OBJ_PSIZE(n: usize) -> usize {
    size_of::<apm_mod_list_obj>() + n * size_of::<apm_module_obj>()
}

#[repr(C, packed)]
pub struct apm_module_list_params {
    pub param_data: apm_module_param_data,
    pub num_modules_list: uint32_t,
    /* Module list config array */
    pub mod_list_obj: [apm_mod_list_obj; 0],
}

#[repr(C, packed)]
pub struct apm_mod_prop_obj {
    pub instance_id: u32,
    pub num_props: u32,
    pub prop_data_1: apm_prop_data,
    pub prop_id_port: apm_module_prop_id_port_info,
}

#[repr(C, packed)]
pub struct apm_prop_list_params {
    pub param_data: apm_module_param_data,
    pub num_modules_prop_cfg: u32,
    pub mod_prop_obj: [apm_mod_prop_obj; 0],
}

fn APM_MOD_PROP_PSIZE(n: usize) -> usize {
    ALIGN(size_of::<apm_prop_list_params>() + n * size_of::<apm_mod_prop_obj>(), 8)
}

#[repr(C, packed)]
pub struct apm_mod_conn_list_params {
    pub param_data: apm_module_param_data,
    pub num_connections: u32,
    pub conn_obj: [apm_module_conn_obj; 0],
}

fn APM_MOD_CONN_PSIZE(n: usize) -> usize {
    ALIGN(size_of::<apm_mod_conn_list_params>() + n * size_of::<apm_module_conn_obj>(), 8)
}

#[repr(C, packed)]
pub struct apm_graph_open_params {
    pub cmd_header: *mut apm_cmd_header,
    pub sg_data: *mut apm_sub_graph_params,
    pub cont_data: *mut apm_container_params,
    pub mod_list_data: *mut apm_module_list_params,
    pub mod_prop_data: *mut apm_prop_list_params,
    pub mod_conn_list_data: *mut apm_mod_conn_list_params,
}

#[repr(C, packed)]
pub struct apm_pcm_module_media_fmt_cmd {
    pub param_data: apm_module_param_data,
    pub header: param_id_pcm_output_format_cfg,
    pub media_cfg: payload_pcm_output_format_cfg,
}

#[repr(C, packed)]
pub struct apm_rd_shmem_module_config_cmd {
    pub param_data: apm_module_param_data,
    pub cfg: param_id_rd_sh_mem_cfg,
}

#[repr(C, packed)]
pub struct apm_sh_module_media_fmt_cmd {
    pub header: media_format,
    pub cfg: payload_media_fmt_pcm,
}

fn APM_SHMEM_FMT_CFG_PSIZE(ch: usize) -> usize {
    ALIGN(size_of::<apm_sh_module_media_fmt_cmd>() + ch * size_of::<u8>(), 8)
}

/* num of channels as argument */
fn APM_PCM_MODULE_FMT_CMD_PSIZE(ch: usize) -> usize {
    ALIGN(size_of::<apm_pcm_module_media_fmt_cmd>() + ch * size_of::<u8>(), 8)
}

fn APM_PCM_OUT_FMT_CFG_PSIZE(n: usize) -> usize {
    ALIGN(size_of::<payload_pcm_output_format_cfg>() + n * size_of::<u8>(), 4)
}

#[repr(C, packed)]
pub struct apm_i2s_module_intf_cfg {
    pub param_data: apm_module_param_data,
    pub cfg: param_id_i2s_intf_cfg,
}

fn APM_I2S_INTF_CFG_PSIZE() -> usize {
    ALIGN(size_of::<apm_i2s_module_intf_cfg>(), 8)
}

#[repr(C, packed)]
pub struct apm_audio_if_module_intf_cfg {
    pub param_data: apm_module_param_data,
    pub cfg: param_id_audio_if_intf_cfg,
}

fn APM_AUDIO_IF_INTF_CFG_PSIZE() -> usize {
    ALIGN(size_of::<apm_audio_if_module_intf_cfg>(), 8)
}

#[repr(C, packed)]
pub struct apm_module_hw_ep_mf_cfg {
    pub param_data: apm_module_param_data,
    pub mf: param_id_hw_ep_mf,
}

fn APM_HW_EP_CFG_PSIZE() -> usize {
    ALIGN(size_of::<apm_module_hw_ep_mf_cfg>(), 8)
}

fn APM_MFC_CFG_PSIZE(n: usize) -> usize {
    ALIGN(size_of::<param_id_mfc_media_format>() + n * size_of::<u8>(), 4)
}

#[repr(C, packed)]
pub struct apm_module_frame_size_factor_cfg {
    pub param_data: apm_module_param_data,
    pub frame_size_factor: uint32_t,
}

fn APM_FS_CFG_PSIZE() -> usize {
    ALIGN(size_of::<apm_module_frame_size_factor_cfg>(), 8)
}

#[repr(C, packed)]
pub struct apm_module_hw_ep_frame_duration_cfg {
    pub param_data: apm_module_param_data,
    pub frame_duration: param_id_hw_ep_frame_duration,
}

fn APM_HW_EP_FRAME_DURATION_PSIZE() -> usize {
    ALIGN(size_of::<apm_module_hw_ep_frame_duration_cfg>(), 8)
}

#[repr(C, packed)]
pub struct apm_module_hw_ep_power_mode_cfg {
    pub param_data: apm_module_param_data,
    pub power_mode: param_id_hw_ep_power_mode_cfg,
}

fn APM_HW_EP_PMODE_CFG_PSIZE() -> usize {
    ALIGN(size_of::<apm_module_hw_ep_power_mode_cfg>(), 8)
}

#[repr(C, packed)]
pub struct apm_module_hw_ep_dma_data_align_cfg {
    pub param_data: apm_module_param_data,
    pub align: param_id_hw_ep_dma_data_align,
}

fn APM_HW_EP_DALIGN_CFG_PSIZE() -> usize {
    ALIGN(size_of::<apm_module_hw_ep_dma_data_align_cfg>(), 8)
}

#[repr(C, packed)]
pub struct apm_gain_module_cfg {
    pub param_data: apm_module_param_data,
    pub gain_cfg: param_id_gain_cfg,
}

fn APM_GAIN_CFG_PSIZE() -> usize {
    ALIGN(size_of::<apm_gain_module_cfg>(), 8)
}

#[repr(C, packed)]
pub struct apm_codec_dma_module_intf_cfg {
    pub param_data: apm_module_param_data,
    pub cfg: param_id_codec_dma_intf_cfg,
}

fn APM_CDMA_INTF_CFG_PSIZE() -> usize {
    ALIGN(size_of::<apm_codec_dma_module_intf_cfg>(), 8)
}

#[repr(C, packed)]
pub struct apm_display_port_module_intf_cfg {
    pub param_data: apm_module_param_data,
    pub cfg: param_id_display_port_intf_cfg,
}

fn APM_DP_INTF_CFG_PSIZE() -> usize {
    ALIGN(size_of::<apm_display_port_module_intf_cfg>(), 8)
}

#[repr(C, packed)]
pub struct apm_module_sp_vi_op_mode_cfg {
    pub param_data: apm_module_param_data,
    pub cfg: param_id_sp_vi_op_mode_cfg,
}

fn APM_SP_VI_OP_MODE_CFG_PSIZE(ch: usize) -> usize {
    ALIGN(size_of::<apm_module_sp_vi_op_mode_cfg>() + ch * size_of::<uint32_t>(), 8)
}

#[repr(C, packed)]
pub struct apm_module_sp_vi_ex_mode_cfg {
    pub param_data: apm_module_param_data,
    pub cfg: param_id_sp_vi_ex_mode_cfg,
}

fn APM_SP_VI_EX_MODE_CFG_PSIZE() -> usize {
    ALIGN(size_of::<apm_module_sp_vi_ex_mode_cfg>(), 8)
}

#[repr(C, packed)]
pub struct apm_module_sp_vi_channel_map_cfg {
    pub param_data: apm_module_param_data,
    pub cfg: param_id_sp_vi_channel_map_cfg,
}

fn APM_SP_VI_CH_MAP_CFG_PSIZE(ch: usize) -> usize {
    ALIGN(size_of::<apm_module_sp_vi_channel_map_cfg>() + ch * size_of::<uint32_t>(), 8)
}

unsafe fn ptr_add<T>(p: *mut c_void, bytes: usize) -> *mut T {
    (p as *mut u8).add(bytes) as *mut T
}

unsafe fn __audioreach_alloc_pkt(
    payload_size: i32,
    opcode: uint32_t,
    token: uint32_t,
    src_port: uint32_t,
    dest_port: uint32_t,
    has_cmd_hdr: bool,
) -> *mut c_void {
    let mut pkt: *mut gpr_pkt;
    let mut p: *mut c_void;
    let mut pkt_size: i32 = GPR_HDR_SIZE as i32 + payload_size;

    if has_cmd_hdr {
        pkt_size += APM_CMD_HDR_SIZE as i32;
    }

    p = kzalloc(pkt_size as usize, GFP_KERNEL);
    if p.is_null() {
        return ERR_PTR(-ENOMEM) as *mut c_void;
    }

    pkt = p as *mut gpr_pkt;
    (*pkt).hdr.version = GPR_PKT_VER;
    (*pkt).hdr.hdr_size = GPR_PKT_HEADER_WORD_SIZE;
    (*pkt).hdr.pkt_size = pkt_size as _;
    (*pkt).hdr.dest_port = dest_port;
    (*pkt).hdr.src_port = src_port;
    (*pkt).hdr.dest_domain = GPR_DOMAIN_ID_ADSP;
    (*pkt).hdr.src_domain = GPR_DOMAIN_ID_APPS;
    (*pkt).hdr.token = token;
    (*pkt).hdr.opcode = opcode;

    if has_cmd_hdr {
        let cmd_header: *mut apm_cmd_header;
        p = ptr_add::<c_void>(p, GPR_HDR_SIZE);
        cmd_header = p as *mut apm_cmd_header;
        (*cmd_header).payload_size = payload_size as _;
    }

    pkt as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_alloc_pkt(
    payload_size: i32,
    opcode: uint32_t,
    token: uint32_t,
    src_port: uint32_t,
    dest_port: uint32_t,
) -> *mut c_void {
    __audioreach_alloc_pkt(payload_size, opcode, token, src_port, dest_port, false)
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_alloc_apm_pkt(
    pkt_size: i32,
    opcode: uint32_t,
    token: uint32_t,
    src_port: uint32_t,
) -> *mut c_void {
    __audioreach_alloc_pkt(
        pkt_size,
        opcode,
        token,
        src_port,
        APM_MODULE_INSTANCE_ID,
        false,
    )
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_alloc_cmd_pkt(
    payload_size: i32,
    opcode: uint32_t,
    token: uint32_t,
    src_port: uint32_t,
    dest_port: uint32_t,
) -> *mut c_void {
    __audioreach_alloc_pkt(payload_size, opcode, token, src_port, dest_port, true)
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_alloc_apm_cmd_pkt(
    pkt_size: i32,
    opcode: uint32_t,
    token: uint32_t,
) -> *mut c_void {
    __audioreach_alloc_pkt(
        pkt_size,
        opcode,
        token,
        GPR_APM_MODULE_IID,
        APM_MODULE_INSTANCE_ID,
        true,
    )
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_set_default_channel_mapping(
    ch_map: *mut u8,
    num_channels: i32,
) {
    if num_channels == 1 {
        *ch_map.add(0) = PCM_CHANNEL_FL as u8;
    } else if num_channels == 2 {
        *ch_map.add(0) = PCM_CHANNEL_FL as u8;
        *ch_map.add(1) = PCM_CHANNEL_FR as u8;
    } else if num_channels == 4 {
        *ch_map.add(0) = PCM_CHANNEL_FL as u8;
        *ch_map.add(1) = PCM_CHANNEL_FR as u8;
        *ch_map.add(2) = PCM_CHANNEL_LS as u8;
        *ch_map.add(3) = PCM_CHANNEL_RS as u8;
    }
}

unsafe fn apm_populate_container_config(
    cfg: *mut apm_container_obj,
    cont: *const audioreach_container,
) {
    /* Container Config */
    (*cfg).container_cfg.container_id = (*cont).container_id;
    (*cfg).container_cfg.num_prop = 4;

    /* Capability list */
    (*cfg).cap_data.prop_id = APM_CONTAINER_PROP_ID_CAPABILITY_LIST;
    (*cfg).cap_data.prop_size = APM_CONTAINER_PROP_ID_CAPABILITY_SIZE;
    (*cfg).num_capability_id = 1;
    (*cfg).capability_id = (*cont).capability_id;

    /* Graph Position */
    (*cfg).pos_data.prop_id = APM_CONTAINER_PROP_ID_GRAPH_POS;
    (*cfg).pos_data.prop_size = size_of::<apm_cont_prop_id_graph_pos>() as _;
    (*cfg).pos.graph_pos = (*cont).graph_pos;

    /* Stack size */
    (*cfg).stack_data.prop_id = APM_CONTAINER_PROP_ID_STACK_SIZE;
    (*cfg).stack_data.prop_size = size_of::<apm_cont_prop_id_stack_size>() as _;
    (*cfg).stack.stack_size = (*cont).stack_size;

    /* Proc domain */
    (*cfg).domain_data.prop_id = APM_CONTAINER_PROP_ID_PROC_DOMAIN;
    (*cfg).domain_data.prop_size = size_of::<apm_cont_prop_id_domain>() as _;
    (*cfg).domain.proc_domain = (*cont).proc_domain;
}

unsafe fn apm_populate_sub_graph_config(
    cfg: *mut apm_sub_graph_data,
    sg: *const audioreach_sub_graph,
) {
    (*cfg).sub_graph_cfg.sub_graph_id = (*sg).sub_graph_id;
    (*cfg).sub_graph_cfg.num_sub_graph_prop = APM_SUB_GRAPH_CFG_NPROP;

    /* Perf Mode */
    (*cfg).perf_data.prop_id = APM_SUB_GRAPH_PROP_ID_PERF_MODE;
    (*cfg).perf_data.prop_size = APM_SG_PROP_ID_PERF_MODE_SIZE;
    (*cfg).perf.perf_mode = (*sg).perf_mode;

    /* Direction */
    (*cfg).dir_data.prop_id = APM_SUB_GRAPH_PROP_ID_DIRECTION;
    (*cfg).dir_data.prop_size = APM_SG_PROP_ID_DIR_SIZE;
    (*cfg).dir.direction = (*sg).direction;

    /* Scenario ID */
    (*cfg).sid_data.prop_id = APM_SUB_GRAPH_PROP_ID_SCENARIO_ID;
    (*cfg).sid_data.prop_size = APM_SG_PROP_ID_SID_SIZE;
    (*cfg).sid.scenario_id = (*sg).scenario_id;
}

unsafe fn apm_populate_module_prop_obj(
    obj: *mut apm_mod_prop_obj,
    module: *const audioreach_module,
) {
    (*obj).instance_id = (*module).instance_id;
    (*obj).num_props = 1;
    (*obj).prop_data_1.prop_id = APM_MODULE_PROP_ID_PORT_INFO;
    (*obj).prop_data_1.prop_size = APM_MODULE_PROP_ID_PORT_INFO_SZ;
    (*obj).prop_id_port.max_ip_port = (*module).max_ip_port;
    (*obj).prop_id_port.max_op_port = (*module).max_op_port;
}

unsafe fn apm_populate_module_list_obj(
    obj: *mut apm_mod_list_obj,
    container: *const audioreach_container,
    sub_graph_id: i32,
) {
    let mut i: isize = 0;

    (*obj).sub_graph_id = sub_graph_id as _;
    (*obj).container_id = (*container).container_id;
    (*obj).num_modules = (*container).num_modules;
    list_for_each_entry_audioreach_module(module, &(*container).modules_list, |module| {
        (*obj).mod_cfg.as_ptr().offset(i).cast_mut().write(apm_module_obj {
            module_id: (*module).module_id,
            instance_id: (*module).instance_id,
        });
        i += 1;
    });
}

unsafe fn audioreach_populate_graph(
    _apm: *mut q6apm,
    info: *const audioreach_graph_info,
    open: *mut apm_graph_open_params,
    sg_list: *const list_head,
    _num_sub_graphs: i32,
) {
    let mc_data = (*open).mod_conn_list_data;
    let ml_data = (*open).mod_list_data;
    let mp_data = (*open).mod_prop_data;
    let c_data = (*open).cont_data;
    let sg_data = (*open).sg_data;
    let mut ncontainer: isize = 0;
    let mut nmodule: isize = 0;
    let mut nconn: isize = 0;
    let mut mlobj = (*ml_data).mod_list_obj.as_mut_ptr();
    let mut i: isize = 0;

    if (*info).dst_mod_inst_id != 0 && (*info).src_mod_inst_id != 0 {
        let conn_obj = (*mc_data).conn_obj.as_mut_ptr().offset(nconn);
        (*conn_obj).src_mod_inst_id = (*info).src_mod_inst_id;
        (*conn_obj).src_mod_op_port_id = (*info).src_mod_op_port_id;
        (*conn_obj).dst_mod_inst_id = (*info).dst_mod_inst_id;
        (*conn_obj).dst_mod_ip_port_id = (*info).dst_mod_ip_port_id;
        nconn += 1;
    }

    list_for_each_entry_audioreach_sub_graph(sg_list, |sg| {
        let sg_cfg = (*sg_data).sg_cfg.as_mut_ptr().offset(i);
        i += 1;
        apm_populate_sub_graph_config(sg_cfg, sg);

        list_for_each_entry_audioreach_container(&(*sg).container_list, |container| {
            let cobj = (*c_data).cont_obj.as_mut_ptr().offset(ncontainer);
            apm_populate_container_config(cobj, container);
            apm_populate_module_list_obj(mlobj, container, (*sg).sub_graph_id as i32);

            list_for_each_entry_audioreach_module(module, &(*container).modules_list, |module| {
                let module_prop_obj = (*mp_data).mod_prop_obj.as_mut_ptr().offset(nmodule);
                nmodule += 1;
                apm_populate_module_prop_obj(module_prop_obj, module);

                if (*module).max_op_port == 0 {
                    return;
                }

                let mut pn = 0;
                while pn < (*module).max_op_port as i32 {
                    if (*module).dst_mod_inst_id[pn as usize] != 0 {
                        let conn_obj = (*mc_data).conn_obj.as_mut_ptr().offset(nconn);
                        (*conn_obj).src_mod_inst_id = (*module).instance_id;
                        (*conn_obj).src_mod_op_port_id = (*module).src_mod_op_port_id[pn as usize];
                        (*conn_obj).dst_mod_inst_id = (*module).dst_mod_inst_id[pn as usize];
                        (*conn_obj).dst_mod_ip_port_id = (*module).dst_mod_ip_port_id[pn as usize];
                        nconn += 1;
                    }
                    pn += 1;
                }
            });
            mlobj = (mlobj as *mut u8).add(APM_MOD_LIST_OBJ_PSIZE((*container).num_modules as usize))
                as *mut apm_mod_list_obj;
            ncontainer += 1;
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_alloc_graph_pkt(
    apm: *mut q6apm,
    info: *const audioreach_graph_info,
) -> *mut c_void {
    let mut ml_sz: i32 = 0;
    let sg_list: *const list_head = &(*info).sg_list;
    let mut num_connections: i32 = 0;
    let mut num_containers: i32 = 0;
    let mut num_sub_graphs: i32 = 0;
    let mut num_modules: i32 = 0;

    /* add FE-BE connections */
    if (*info).dst_mod_inst_id != 0 && (*info).src_mod_inst_id != 0 {
        num_connections += 1;
    }

    list_for_each_entry_audioreach_sub_graph(sg_list, |sgs| {
        num_sub_graphs += 1;
        list_for_each_entry_audioreach_container(&(*sgs).container_list, |container| {
            num_containers += 1;
            num_modules += (*container).num_modules as i32;
            ml_sz = ml_sz
                + size_of::<apm_module_list_params>() as i32
                + APM_MOD_LIST_OBJ_PSIZE((*container).num_modules as usize) as i32;

            list_for_each_entry_audioreach_module(module, &(*container).modules_list, |module| {
                num_connections += (*module).num_connections as i32;
            });
        });
    });

    let num_modules_list = num_containers;
    let sg_sz = APM_SUB_GRAPH_PSIZE(num_sub_graphs as usize) as i32;
    let cont_sz = APM_CONTAINER_PSIZE(num_containers as usize) as i32;
    ml_sz = ALIGN(ml_sz as usize, 8) as i32;
    let mp_sz = APM_MOD_PROP_PSIZE(num_modules as usize) as i32;
    let mc_sz = APM_MOD_CONN_PSIZE(num_connections as usize) as i32;

    let payload_size = sg_sz + cont_sz + ml_sz + mp_sz + mc_sz;
    let pkt = audioreach_alloc_apm_cmd_pkt(payload_size, APM_CMD_GRAPH_OPEN, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return pkt as *mut c_void;
    }

    let mut p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    let mut params: apm_graph_open_params = core::mem::zeroed();

    /* SubGraph */
    params.sg_data = p as *mut apm_sub_graph_params;
    let mut param_data = &mut (*params.sg_data).param_data as *mut apm_module_param_data;
    (*param_data).module_instance_id = APM_MODULE_INSTANCE_ID;
    (*param_data).param_id = APM_PARAM_ID_SUB_GRAPH_CONFIG;
    (*param_data).param_size = (sg_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*params.sg_data).num_sub_graphs = num_sub_graphs as _;
    p = ptr_add::<c_void>(p, sg_sz as usize);

    /* Container */
    params.cont_data = p as *mut apm_container_params;
    param_data = &mut (*params.cont_data).param_data;
    (*param_data).module_instance_id = APM_MODULE_INSTANCE_ID;
    (*param_data).param_id = APM_PARAM_ID_CONTAINER_CONFIG;
    (*param_data).param_size = (cont_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*params.cont_data).num_containers = num_containers as _;
    p = ptr_add::<c_void>(p, cont_sz as usize);

    /* Module List*/
    params.mod_list_data = p as *mut apm_module_list_params;
    param_data = &mut (*params.mod_list_data).param_data;
    (*param_data).module_instance_id = APM_MODULE_INSTANCE_ID;
    (*param_data).param_id = APM_PARAM_ID_MODULE_LIST;
    (*param_data).param_size = (ml_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*params.mod_list_data).num_modules_list = num_modules_list as _;
    p = ptr_add::<c_void>(p, ml_sz as usize);

    /* Module Properties */
    params.mod_prop_data = p as *mut apm_prop_list_params;
    param_data = &mut (*params.mod_prop_data).param_data;
    (*param_data).module_instance_id = APM_MODULE_INSTANCE_ID;
    (*param_data).param_id = APM_PARAM_ID_MODULE_PROP;
    (*param_data).param_size = (mp_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*params.mod_prop_data).num_modules_prop_cfg = num_modules as _;
    p = ptr_add::<c_void>(p, mp_sz as usize);

    /* Module Connections */
    params.mod_conn_list_data = p as *mut apm_mod_conn_list_params;
    param_data = &mut (*params.mod_conn_list_data).param_data;
    (*param_data).module_instance_id = APM_MODULE_INSTANCE_ID;
    (*param_data).param_id = APM_PARAM_ID_MODULE_CONN;
    (*param_data).param_size = (mc_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*params.mod_conn_list_data).num_connections = num_connections as _;
    p = ptr_add::<c_void>(p, mc_sz as usize);

    audioreach_populate_graph(apm, info, &mut params, sg_list, num_sub_graphs);
    pkt as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_send_cmd_sync(
    dev: *mut device,
    gdev: *mut gpr_device_t,
    result: *mut gpr_ibasic_rsp_result_t,
    cmd_lock: *mut mutex,
    port: *mut gpr_port_t,
    cmd_wait: *mut wait_queue_head_t,
    pkt: *const gpr_pkt,
    rsp_opcode: uint32_t,
) -> i32 {
    let hdr = &(*pkt).hdr as *const gpr_hdr;
    let mut rc: i32;

    mutex_lock(cmd_lock);
    (*result).opcode = 0;
    (*result).status = 0;

    if !port.is_null() {
        rc = gpr_send_port_pkt(port, pkt);
    } else if !gdev.is_null() {
        rc = gpr_send_pkt(gdev, pkt);
    } else {
        rc = -EINVAL;
    }

    if rc < 0 {
        mutex_unlock(cmd_lock);
        return rc;
    }

    if rsp_opcode != 0 {
        rc = wait_event_timeout(
            cmd_wait,
            (*result).opcode == (*hdr).opcode || (*result).opcode == rsp_opcode,
            5 * HZ,
        );
    } else {
        rc = wait_event_timeout(cmd_wait, (*result).opcode == (*hdr).opcode, 5 * HZ);
    }

    if rc == 0 {
        dev_err(dev, c"CMD timeout for [%x] opcode\n".as_ptr(), (*hdr).opcode);
        rc = -ETIMEDOUT;
    } else if (*result).status > 0 {
        dev_err(
            dev,
            c"DSP returned error[%x] %x\n".as_ptr(),
            (*hdr).opcode,
            (*result).status,
        );
        rc = -EINVAL;
    } else {
        /* DSP successfully finished the command */
        rc = 0;
    }

    mutex_unlock(cmd_lock);
    rc
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_graph_send_cmd_sync(
    graph: *mut q6apm_graph,
    pkt: *const gpr_pkt,
    rsp_opcode: uint32_t,
) -> i32 {
    audioreach_send_cmd_sync(
        (*graph).dev,
        ptr::null_mut(),
        &mut (*graph).result,
        &mut (*graph).lock,
        (*graph).port,
        &mut (*graph).cmd_wait,
        pkt,
        rsp_opcode,
    )
}

unsafe fn fill_hw_ep(
    hw_cfg: *mut apm_module_hw_ep_mf_cfg,
    module: *const audioreach_module,
    cfg: *const audioreach_module_config,
    ep_sz: i32,
) {
    let param_data = &mut (*hw_cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_HW_EP_MF_CFG;
    param_data.param_size = (ep_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*hw_cfg).mf.sample_rate = (*cfg).sample_rate;
    (*hw_cfg).mf.bit_width = (*cfg).bit_width;
    (*hw_cfg).mf.num_channels = (*cfg).num_channels;
    (*hw_cfg).mf.data_format = (*module).data_format;
}

unsafe fn audioreach_display_port_set_media_format(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    cfg: *const audioreach_module_config,
) -> i32 {
    let ic_sz = APM_DP_INTF_CFG_PSIZE() as i32;
    let ep_sz = APM_HW_EP_CFG_PSIZE() as i32;
    let fs_sz = APM_FS_CFG_PSIZE() as i32;
    let size = ic_sz + ep_sz + fs_sz;
    let pkt = audioreach_alloc_apm_cmd_pkt(size, APM_CMD_SET_CFG, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let mut p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    let hw_cfg = p as *mut apm_module_hw_ep_mf_cfg;
    fill_hw_ep(hw_cfg, module, cfg, ep_sz);
    p = ptr_add::<c_void>(p, ep_sz as usize);

    let fs_cfg = p as *mut apm_module_frame_size_factor_cfg;
    let param_data = &mut (*fs_cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_HW_EP_FRAME_SIZE_FACTOR;
    param_data.param_size = (fs_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*fs_cfg).frame_size_factor = 1;
    p = ptr_add::<c_void>(p, fs_sz as usize);

    let intf_cfg = p as *mut apm_display_port_module_intf_cfg;
    let param_data = &mut (*intf_cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_DISPLAY_PORT_INTF_CFG;
    param_data.param_size = (ic_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*intf_cfg).cfg.channel_allocation = (*cfg).channel_allocation;
    (*intf_cfg).cfg.mst_idx = 0;
    (*intf_cfg).cfg.dptx_idx = (*cfg).dp_idx;
    q6apm_send_cmd_sync((*graph).apm, pkt, 0)
}

/* LPASS Codec DMA port Module Media Format Setup */
unsafe fn audioreach_codec_dma_set_media_format(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    cfg: *const audioreach_module_config,
) -> i32 {
    let ic_sz = APM_CDMA_INTF_CFG_PSIZE() as i32;
    let ep_sz = APM_HW_EP_CFG_PSIZE() as i32;
    let fs_sz = APM_FS_CFG_PSIZE() as i32;
    let pm_sz = APM_HW_EP_PMODE_CFG_PSIZE() as i32;
    let size = ic_sz + ep_sz + fs_sz + pm_sz;
    let pkt = audioreach_alloc_apm_cmd_pkt(size, APM_CMD_SET_CFG, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let mut p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    fill_hw_ep(p as *mut apm_module_hw_ep_mf_cfg, module, cfg, ep_sz);
    p = ptr_add::<c_void>(p, ep_sz as usize);

    let fs_cfg = p as *mut apm_module_frame_size_factor_cfg;
    let param_data = &mut (*fs_cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_HW_EP_FRAME_SIZE_FACTOR;
    param_data.param_size = (fs_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*fs_cfg).frame_size_factor = 1;
    p = ptr_add::<c_void>(p, fs_sz as usize);

    let intf_cfg = p as *mut apm_codec_dma_module_intf_cfg;
    let param_data = &mut (*intf_cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_CODEC_DMA_INTF_CFG;
    param_data.param_size = (ic_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*intf_cfg).cfg.lpaif_type = (*module).hw_interface_type;
    (*intf_cfg).cfg.intf_index = (*module).hw_interface_idx;
    (*intf_cfg).cfg.active_channels_mask = 0;
    /* Convert the physical channel mapping into a bit field */
    let mut i = 0;
    while i < AR_PCM_MAX_NUM_CHANNEL {
        if (*cfg).channel_map[i as usize] != 0 {
            (*intf_cfg).cfg.active_channels_mask |= BIT(i);
        }
        i += 1;
    }
    p = ptr_add::<c_void>(p, ic_sz as usize);

    let pm_cfg = p as *mut apm_module_hw_ep_power_mode_cfg;
    let param_data = &mut (*pm_cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_HW_EP_POWER_MODE_CFG;
    param_data.param_size = (pm_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*pm_cfg).power_mode.power_mode = 0;
    q6apm_send_cmd_sync((*graph).apm, pkt, 0)
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_send_u32_param(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    param_id: uint32_t,
    param_val: uint32_t,
) -> i32 {
    let payload_size = size_of::<uint32_t>() + APM_MODULE_PARAM_DATA_SIZE;
    let pkt = audioreach_alloc_apm_cmd_pkt(payload_size as i32, APM_CMD_SET_CFG, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return -ENOMEM;
    }
    let mut p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    let param_data = p as *mut apm_module_param_data;
    (*param_data).module_instance_id = (*module).instance_id;
    (*param_data).error_code = 0;
    (*param_data).param_id = param_id;
    (*param_data).param_size = size_of::<uint32_t>() as _;
    p = ptr_add::<c_void>(p, APM_MODULE_PARAM_DATA_SIZE);
    *(p as *mut uint32_t) = param_val;
    q6apm_send_cmd_sync((*graph).apm, pkt, 0)
}

unsafe fn audioreach_sal_limiter_enable(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    enable: bool,
) -> i32 {
    audioreach_send_u32_param(graph, module, PARAM_ID_SAL_LIMITER_ENABLE, enable as uint32_t)
}

unsafe fn audioreach_sal_set_media_format(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    cfg: *const audioreach_module_config,
) -> i32 {
    audioreach_send_u32_param(graph, module, PARAM_ID_SAL_OUTPUT_CFG, (*cfg).bit_width)
}

unsafe fn audioreach_module_enable(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    enable: bool,
) -> i32 {
    audioreach_send_u32_param(graph, module, PARAM_ID_MODULE_ENABLE, enable as uint32_t)
}

unsafe fn audioreach_gapless_set_media_format(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    _cfg: *const audioreach_module_config,
) -> i32 {
    audioreach_send_u32_param(graph, module, PARAM_ID_EARLY_EOS_DELAY, EARLY_EOS_DELAY_MS)
}

unsafe fn audioreach_set_module_config(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    _cfg: *const audioreach_module_config,
) -> i32 {
    let size = le32_to_cpu((*(*module).data).size) as i32;
    let pkt = audioreach_alloc_apm_cmd_pkt(size, APM_CMD_SET_CFG, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    memcpy(p, (*(*module).data).data as *const c_void, size as usize);
    q6apm_send_cmd_sync((*graph).apm, pkt, 0)
}

unsafe fn audioreach_mfc_set_media_format(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    cfg: *const audioreach_module_config,
) -> i32 {
    let num_channels = (*cfg).num_channels;
    let payload_size = APM_MFC_CFG_PSIZE(num_channels as usize) + APM_MODULE_PARAM_DATA_SIZE;
    let pkt = audioreach_alloc_apm_cmd_pkt(payload_size as i32, APM_CMD_SET_CFG, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let mut p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    let param_data = p as *mut apm_module_param_data;
    (*param_data).module_instance_id = (*module).instance_id;
    (*param_data).error_code = 0;
    (*param_data).param_id = PARAM_ID_MFC_OUTPUT_MEDIA_FORMAT;
    (*param_data).param_size = APM_MFC_CFG_PSIZE(num_channels as usize) as _;
    p = ptr_add::<c_void>(p, APM_MODULE_PARAM_DATA_SIZE);
    let media_format = p as *mut param_id_mfc_media_format;
    (*media_format).sample_rate = (*cfg).sample_rate;
    (*media_format).bit_width = (*cfg).bit_width;
    (*media_format).num_channels = (*cfg).num_channels;
    /* Convert the physical mapping to a logical mapping of the channels */
    let mut i = 0;
    let mut j = 0;
    while i < AR_PCM_MAX_NUM_CHANNEL && j < (*cfg).num_channels as i32 {
        if (*cfg).channel_map[i as usize] != 0 {
            (*media_format).channel_mapping[j as usize] = (*cfg).channel_map[i as usize];
            j += 1;
        }
        i += 1;
    }
    q6apm_send_cmd_sync((*graph).apm, pkt, 0)
}

unsafe fn audioreach_set_compr_media_format(
    media_fmt_hdr: *mut media_format,
    mut p: *mut c_void,
    mcfg: *const audioreach_module_config,
) -> i32 {
    match (*mcfg).fmt {
        SND_AUDIOCODEC_MP3 => {
            (*media_fmt_hdr).data_format = DATA_FORMAT_RAW_COMPRESSED;
            (*media_fmt_hdr).fmt_id = MEDIA_FMT_ID_MP3;
            (*media_fmt_hdr).payload_size = 0;
            p = ptr_add::<c_void>(p, size_of::<media_format>());
            let mp3_cfg = p as *mut payload_media_fmt_pcm;
            (*mp3_cfg).sample_rate = (*mcfg).sample_rate;
            (*mp3_cfg).bit_width = (*mcfg).bit_width;
            (*mp3_cfg).alignment = PCM_LSB_ALIGNED;
            (*mp3_cfg).bits_per_sample = (*mcfg).bit_width;
            (*mp3_cfg).q_factor = (*mcfg).bit_width - 1;
            (*mp3_cfg).endianness = PCM_LITTLE_ENDIAN;
            (*mp3_cfg).num_channels = (*mcfg).num_channels;
        }
        SND_AUDIOCODEC_AAC => {
            (*media_fmt_hdr).data_format = DATA_FORMAT_RAW_COMPRESSED;
            (*media_fmt_hdr).fmt_id = MEDIA_FMT_ID_AAC;
            (*media_fmt_hdr).payload_size = size_of::<payload_media_fmt_aac_t>() as _;
            p = ptr_add::<c_void>(p, size_of::<media_format>());
            let aac_cfg = p as *mut payload_media_fmt_aac_t;
            (*aac_cfg).aac_fmt_flag = 0;
            (*aac_cfg).audio_obj_type = 5;
            (*aac_cfg).num_channels = (*mcfg).num_channels;
            (*aac_cfg).total_size_of_PCE_bits = 0;
            (*aac_cfg).sample_rate = (*mcfg).sample_rate;
        }
        SND_AUDIOCODEC_FLAC => {
            (*media_fmt_hdr).data_format = DATA_FORMAT_RAW_COMPRESSED;
            (*media_fmt_hdr).fmt_id = MEDIA_FMT_ID_FLAC;
            (*media_fmt_hdr).payload_size = size_of::<payload_media_fmt_flac_t>() as _;
            p = ptr_add::<c_void>(p, size_of::<media_format>());
            let flac_cfg = p as *mut payload_media_fmt_flac_t;
            (*flac_cfg).sample_size = (*mcfg).codec.options.flac_d.sample_size;
            (*flac_cfg).num_channels = (*mcfg).num_channels;
            (*flac_cfg).min_blk_size = (*mcfg).codec.options.flac_d.min_blk_size;
            (*flac_cfg).max_blk_size = (*mcfg).codec.options.flac_d.max_blk_size;
            (*flac_cfg).sample_rate = (*mcfg).sample_rate;
            (*flac_cfg).min_frame_size = (*mcfg).codec.options.flac_d.min_frame_size;
            (*flac_cfg).max_frame_size = (*mcfg).codec.options.flac_d.max_frame_size;
        }
        SND_AUDIOCODEC_OPUS_RAW => {
            (*media_fmt_hdr).data_format = DATA_FORMAT_RAW_COMPRESSED;
            (*media_fmt_hdr).fmt_id = MEDIA_FMT_ID_OPUS;
            (*media_fmt_hdr).payload_size = size_of::<payload_media_fmt_opus_t>() as _;
            p = ptr_add::<c_void>(p, size_of::<media_format>());
            let opus_cfg = p as *mut payload_media_fmt_opus_t;
            /* raw opus packets prepended with 4 bytes of length */
            (*opus_cfg).bitstream_format = 1;
            /*
             * payload_type:
             * 0 -- read metadata from opus stream;
             * 1 -- metadata is provided by filling in the struct here.
             */
            (*opus_cfg).payload_type = 1;
            (*opus_cfg).version = (*mcfg).codec.options.opus_d.version;
            (*opus_cfg).num_channels = (*mcfg).codec.options.opus_d.num_channels;
            (*opus_cfg).pre_skip = (*mcfg).codec.options.opus_d.pre_skip;
            (*opus_cfg).sample_rate = (*mcfg).codec.options.opus_d.sample_rate;
            (*opus_cfg).output_gain = (*mcfg).codec.options.opus_d.output_gain;
            (*opus_cfg).mapping_family = (*mcfg).codec.options.opus_d.mapping_family;
            (*opus_cfg).stream_count = (*mcfg).codec.options.opus_d.chan_map.stream_count;
            (*opus_cfg).coupled_count = (*mcfg).codec.options.opus_d.chan_map.coupled_count;
            memcpy(
                (*opus_cfg).channel_mapping.as_mut_ptr() as *mut c_void,
                (*mcfg).codec.options.opus_d.chan_map.channel_map.as_ptr() as *const c_void,
                size_of_val(&(*opus_cfg).channel_mapping),
            );
            (*opus_cfg).reserved[0] = 0;
            (*opus_cfg).reserved[1] = 0;
            (*opus_cfg).reserved[2] = 0;
        }
        _ => return -EINVAL,
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_compr_set_param(
    graph: *mut q6apm_graph,
    mcfg: *const audioreach_module_config,
) -> i32 {
    let iid = (*graph).shm_iid;
    let payload_size = size_of::<apm_sh_module_media_fmt_cmd>() as i32;
    let pkt = audioreach_alloc_cmd_pkt(
        payload_size,
        DATA_CMD_WR_SH_MEM_EP_MEDIA_FORMAT,
        0,
        (*(*graph).port).id,
        iid as _,
    ) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return -ENOMEM;
    }
    let p = (pkt as *mut u8).add(GPR_HDR_SIZE) as *mut c_void;
    let rc = audioreach_set_compr_media_format(p as *mut media_format, p, mcfg);
    if rc != 0 {
        return rc;
    }
    gpr_send_port_pkt((*graph).port, pkt)
}

unsafe fn audioreach_i2s_set_media_format(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    cfg: *const audioreach_module_config,
) -> i32 {
    let ic_sz = APM_I2S_INTF_CFG_PSIZE() as i32;
    let ep_sz = APM_HW_EP_CFG_PSIZE() as i32;
    let fs_sz = APM_FS_CFG_PSIZE() as i32;
    let size = ic_sz + ep_sz + fs_sz;
    let pkt = audioreach_alloc_apm_cmd_pkt(size, APM_CMD_SET_CFG, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let mut p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    let intf_cfg = p as *mut apm_i2s_module_intf_cfg;
    let param_data = &mut (*intf_cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_I2S_INTF_CFG;
    param_data.param_size = (ic_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*intf_cfg).cfg.lpaif_type = (*module).hw_interface_type;
    (*intf_cfg).cfg.intf_idx = (*module).hw_interface_idx;
    (*intf_cfg).cfg.sd_line_idx = (*module).sd_line_idx;

    match (*cfg).fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => (*intf_cfg).cfg.ws_src = CONFIG_I2S_WS_SRC_INTERNAL,
        SND_SOC_DAIFMT_BC_FC => {
            /* CPU is slave */
            (*intf_cfg).cfg.ws_src = CONFIG_I2S_WS_SRC_EXTERNAL;
        }
        _ => {}
    }

    p = ptr_add::<c_void>(p, ic_sz as usize);
    fill_hw_ep(p as *mut apm_module_hw_ep_mf_cfg, module, cfg, ep_sz);
    p = ptr_add::<c_void>(p, ep_sz as usize);
    let fs_cfg = p as *mut apm_module_frame_size_factor_cfg;
    let param_data = &mut (*fs_cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_HW_EP_FRAME_SIZE_FACTOR;
    param_data.param_size = (fs_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*fs_cfg).frame_size_factor = 1;
    q6apm_send_cmd_sync((*graph).apm, pkt, 0)
}

unsafe fn audioreach_audio_if_set_media_format(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    cfg: *const audioreach_module_config,
) -> i32 {
    let ic_sz = APM_AUDIO_IF_INTF_CFG_PSIZE() as i32;
    let ep_sz = APM_HW_EP_CFG_PSIZE() as i32;
    let fd_sz = APM_HW_EP_FRAME_DURATION_PSIZE() as i32;
    let size = ic_sz + ep_sz + fd_sz;
    let slot_mask: u32 = if (*cfg).slot_mask != 0 { (*cfg).slot_mask } else { (*module).slot_mask };
    let nslots_per_frame: u16 = if (*cfg).nslots_per_frame != 0 {
        (*cfg).nslots_per_frame as u16
    } else {
        (*module).nslots_per_frame
    };
    let slot_width: u16 = if (*cfg).slot_width != 0 {
        (*cfg).slot_width as u16
    } else {
        (*module).slot_width
    };
    let pkt = audioreach_alloc_apm_cmd_pkt(size, APM_CMD_SET_CFG, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let mut p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    let intf_cfg = p as *mut apm_audio_if_module_intf_cfg;
    let param_data = &mut (*intf_cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_AUDIO_IF_INTF_CFG;
    param_data.param_size = (ic_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*intf_cfg).cfg.qaif_type = (*module).qaif_type;
    (*intf_cfg).cfg.intf_idx = (*module).hw_interface_idx as u16;
    (*intf_cfg).cfg.intf_mode = (*module).intf_mode;
    (*intf_cfg).cfg.ctrl_data_out_enable = (*module).ctrl_data_out_enable;
    (*intf_cfg).cfg.active_slot_mask = slot_mask;
    (*intf_cfg).cfg.nslots_per_frame = nslots_per_frame;
    (*intf_cfg).cfg.slot_width = slot_width;
    (*intf_cfg).cfg.active_lane_mask = (*module).active_lane_mask;
    (*intf_cfg).cfg.frame_sync_rate = (*module).frame_sync_rate;
    (*intf_cfg).cfg.frame_sync_src = (*module).sync_src;
    (*intf_cfg).cfg.frame_sync_mode = (*module).sync_mode;
    (*intf_cfg).cfg.invert_frame_sync_pulse = (*module).ctrl_invert_sync_pulse;
    (*intf_cfg).cfg.frame_sync_data_delay = (*module).ctrl_sync_data_delay;
    (*intf_cfg).cfg.bit_clk_type = (*module).bit_clk_type;
    (*intf_cfg).cfg.inv_int_bit_clk = (*module).inv_int_bit_clk;
    (*intf_cfg).cfg.inv_ext_bit_clk = (*module).inv_ext_bit_clk;

    p = ptr_add::<c_void>(p, ic_sz as usize);
    fill_hw_ep(p as *mut apm_module_hw_ep_mf_cfg, module, cfg, ep_sz);
    p = ptr_add::<c_void>(p, ep_sz as usize);

    let fd_cfg = p as *mut apm_module_hw_ep_frame_duration_cfg;
    let param_data = &mut (*fd_cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_HW_EP_FRAME_DURATION;
    param_data.param_size = (fd_sz - APM_MODULE_PARAM_DATA_SIZE as i32) as _;
    (*fd_cfg).frame_duration.frame_duration_in_us = AUDIO_IF_FRAME_DURATION_US;
    (*fd_cfg).frame_duration.allow_frame_duration_normalization =
        AUDIO_IF_FRAME_DURATION_NORMALIZATION_ENABLE;
    (*fd_cfg).frame_duration.min_normalized_frame_dur_us = AUDIO_IF_FRAME_DURATION_MIN_US;
    (*fd_cfg).frame_duration.max_normalized_frame_dur_us = AUDIO_IF_FRAME_DURATION_MAX_US;
    q6apm_send_cmd_sync((*graph).apm, pkt, 0)
}

unsafe fn audioreach_logging_set_media_format(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
) -> i32 {
    let size = size_of::<data_logging_config>() + APM_MODULE_PARAM_DATA_SIZE;
    let pkt = audioreach_alloc_apm_cmd_pkt(size as i32, APM_CMD_SET_CFG, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let mut p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    let param_data = p as *mut apm_module_param_data;
    (*param_data).module_instance_id = (*module).instance_id;
    (*param_data).error_code = 0;
    (*param_data).param_id = PARAM_ID_DATA_LOGGING_CONFIG;
    (*param_data).param_size = (size - APM_MODULE_PARAM_DATA_SIZE) as _;
    p = ptr_add::<c_void>(p, APM_MODULE_PARAM_DATA_SIZE);
    let cfg = p as *mut data_logging_config;
    (*cfg).log_code = (*module).log_code;
    (*cfg).log_tap_point_id = (*module).log_tap_point_id;
    (*cfg).mode = (*module).log_mode;
    q6apm_send_cmd_sync((*graph).apm, pkt, 0)
}

unsafe fn audioreach_pcm_set_media_format(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    mcfg: *const audioreach_module_config,
) -> i32 {
    let num_channels = (*mcfg).num_channels;
    if num_channels > 4 {
        dev_err((*graph).dev, c"Error: Invalid channels (%d)!\n".as_ptr(), num_channels);
        return -EINVAL;
    }
    let payload_size = APM_PCM_MODULE_FMT_CMD_PSIZE(num_channels as usize);
    let pkt = audioreach_alloc_apm_cmd_pkt(payload_size as i32, APM_CMD_SET_CFG, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let cfg = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE)
        as *mut apm_pcm_module_media_fmt_cmd;
    let param_data = &mut (*cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_PCM_OUTPUT_FORMAT_CFG;
    param_data.param_size = (payload_size - APM_MODULE_PARAM_DATA_SIZE) as _;
    (*cfg).header.data_format = DATA_FORMAT_FIXED_POINT;
    (*cfg).header.fmt_id = MEDIA_FMT_ID_PCM;
    (*cfg).header.payload_size = APM_PCM_OUT_FMT_CFG_PSIZE(num_channels as usize) as _;
    let media_cfg = &mut (*cfg).media_cfg as *mut payload_pcm_output_format_cfg;
    (*media_cfg).alignment = PCM_LSB_ALIGNED;
    (*media_cfg).bit_width = (*mcfg).bit_width;
    (*media_cfg).endianness = PCM_LITTLE_ENDIAN;
    (*media_cfg).interleaved = (*module).interleave_type;
    (*media_cfg).num_channels = (*mcfg).num_channels;
    (*media_cfg).q_factor = (*mcfg).bit_width - 1;
    (*media_cfg).bits_per_sample = (*mcfg).bit_width;
    /* Convert the physical mapping to a logical mapping of the channels */
    let mut i = 0;
    let mut j = 0;
    while i < AR_PCM_MAX_NUM_CHANNEL && j < (*mcfg).num_channels as i32 {
        if (*mcfg).channel_map[i as usize] != 0 {
            (*media_cfg).channel_mapping[j as usize] = (*mcfg).channel_map[i as usize];
            j += 1;
        }
        i += 1;
    }
    q6apm_send_cmd_sync((*graph).apm, pkt, 0)
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_shmem_register_event(
    graph: *mut q6apm_graph,
    bytes: i32,
    num_levels: i32,
) -> i32 {
    if num_levels <= 0 || bytes <= 0 {
        return -EINVAL;
    }
    let payload_size = size_of::<apm_module_register_events>()
        + size_of::<event_cfg_sh_mem_pull_push_mode_watermark_t>()
        + num_levels as usize * size_of::<uint32_t>();
    let pkt = audioreach_alloc_cmd_pkt(
        payload_size as i32,
        APM_CMD_REGISTER_MODULE_EVENTS,
        0,
        (*(*graph).port).id,
        (*graph).shm_iid as _,
    ) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let mut p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    let event = p as *mut apm_module_register_events;
    (*event).module_instance_id = (*graph).shm_iid;
    (*event).event_id = EVENT_ID_SH_MEM_PULL_PUSH_MODE_WATERMARK;
    (*event).is_register = 1;
    (*event).event_config_payload_size =
        (size_of::<event_cfg_sh_mem_pull_push_mode_watermark_t>()
            + num_levels as usize * size_of::<uint32_t>()) as _;
    p = ptr_add::<c_void>(p, size_of::<apm_module_register_events>());
    let level = p as *mut event_cfg_sh_mem_pull_push_mode_watermark_t;
    (*level).num_water_mark_levels = num_levels as _;
    let mut i = 0;
    while i < num_levels {
        (*level).level[i as usize] = ((i + 1) * bytes) as _;
        i += 1;
    }
    audioreach_graph_send_cmd_sync(graph, pkt, 0)
}

unsafe fn audioreach_shmem_set_media_format(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    mcfg: *const audioreach_module_config,
) -> i32 {
    let num_channels = (*mcfg).num_channels;
    if num_channels > 4 {
        dev_err((*graph).dev, c"Error: Invalid channels (%d)!\n".as_ptr(), num_channels);
        return -EINVAL;
    }
    let payload_size = APM_SHMEM_FMT_CFG_PSIZE(num_channels as usize) + APM_MODULE_PARAM_DATA_SIZE;
    let pkt = audioreach_alloc_cmd_pkt(
        payload_size as i32,
        APM_CMD_SET_CFG,
        0,
        (*(*graph).port).id,
        (*module).instance_id,
    ) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let mut p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    let param_data = p as *mut apm_module_param_data;
    (*param_data).module_instance_id = (*module).instance_id;
    (*param_data).error_code = 0;
    (*param_data).param_id = PARAM_ID_MEDIA_FORMAT;
    (*param_data).param_size = (payload_size - APM_MODULE_PARAM_DATA_SIZE) as _;
    p = ptr_add::<c_void>(p, APM_MODULE_PARAM_DATA_SIZE);
    let header = p as *mut media_format;
    if (*mcfg).fmt == SND_AUDIOCODEC_PCM {
        (*header).data_format = DATA_FORMAT_FIXED_POINT;
        (*header).fmt_id = MEDIA_FMT_ID_PCM;
        (*header).payload_size = (payload_size - size_of::<media_format>()) as _;
        p = ptr_add::<c_void>(p, size_of::<media_format>());
        let cfg = p as *mut payload_media_fmt_pcm;
        (*cfg).sample_rate = (*mcfg).sample_rate;
        (*cfg).bit_width = (*mcfg).bit_width;
        (*cfg).alignment = PCM_LSB_ALIGNED;
        (*cfg).bits_per_sample = (*mcfg).bit_width;
        (*cfg).q_factor = (*mcfg).bit_width - 1;
        (*cfg).endianness = PCM_LITTLE_ENDIAN;
        (*cfg).num_channels = (*mcfg).num_channels;
        /* Convert the physical mapping to a logical mapping of the channels */
        let mut i = 0;
        let mut j = 0;
        while i < AR_PCM_MAX_NUM_CHANNEL && j < (*cfg).num_channels as i32 {
            if (*mcfg).channel_map[i as usize] != 0 {
                (*cfg).channel_mapping[j as usize] = (*mcfg).channel_map[i as usize];
                j += 1;
            }
            i += 1;
        }
    } else {
        let rc = audioreach_set_compr_media_format(header, p, mcfg);
        if rc != 0 {
            return rc;
        }
    }
    audioreach_graph_send_cmd_sync(graph, pkt, 0)
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_gain_set_vol_ctrl(
    apm: *mut q6apm,
    module: *const audioreach_module,
    vol: i32,
) -> i32 {
    let size = size_of::<param_id_vol_ctrl_master_gain>() + APM_MODULE_PARAM_DATA_SIZE;
    let pkt = audioreach_alloc_apm_cmd_pkt(size as i32, APM_CMD_SET_CFG, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let mut p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    let param_data = p as *mut apm_module_param_data;
    (*param_data).module_instance_id = (*module).instance_id;
    (*param_data).error_code = 0;
    (*param_data).param_id = PARAM_ID_VOL_CTRL_MASTER_GAIN;
    (*param_data).param_size = (size - APM_MODULE_PARAM_DATA_SIZE) as _;
    p = ptr_add::<c_void>(p, APM_MODULE_PARAM_DATA_SIZE);
    let cfg = p as *mut param_id_vol_ctrl_master_gain;
    (*cfg).master_gain = vol;
    q6apm_send_cmd_sync(apm, pkt, 0)
}

unsafe fn audioreach_gain_set(graph: *mut q6apm_graph, module: *const audioreach_module) -> i32 {
    let size = APM_GAIN_CFG_PSIZE();
    let pkt = audioreach_alloc_apm_cmd_pkt(size as i32, APM_CMD_SET_CFG, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let cfg = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut apm_gain_module_cfg;
    let param_data = &mut (*cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = APM_PARAM_ID_GAIN;
    param_data.param_size = (size - APM_MODULE_PARAM_DATA_SIZE) as _;
    (*cfg).gain_cfg.gain = (*module).gain;
    q6apm_send_cmd_sync((*graph).apm, pkt, 0)
}

unsafe fn audioreach_speaker_protection(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    operation_mode: uint32_t,
) -> i32 {
    audioreach_send_u32_param(graph, module, PARAM_ID_SP_OP_MODE, operation_mode)
}

unsafe fn audioreach_speaker_protection_vi(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    mcfg: *const audioreach_module_config,
) -> i32 {
    let num_channels = (*mcfg).num_channels;
    if num_channels > 2 {
        dev_err((*graph).dev, c"Error: Invalid channels (%d)!\n".as_ptr(), num_channels);
        return -EINVAL;
    }
    let op_sz = APM_SP_VI_OP_MODE_CFG_PSIZE(num_channels as usize);
    /* Channel mapping for Isense and Vsense, thus twice number of speakers. */
    let cm_sz = APM_SP_VI_CH_MAP_CFG_PSIZE((num_channels * 2) as usize);
    let ex_sz = APM_SP_VI_EX_MODE_CFG_PSIZE();
    let payload_size = op_sz + cm_sz + ex_sz;
    let pkt = audioreach_alloc_apm_cmd_pkt(payload_size as i32, APM_CMD_SET_CFG, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let mut p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    let op_cfg = p as *mut apm_module_sp_vi_op_mode_cfg;
    let param_data = &mut (*op_cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_SP_VI_OP_MODE_CFG;
    param_data.param_size = (op_sz - APM_MODULE_PARAM_DATA_SIZE) as _;
    (*op_cfg).cfg.num_channels = num_channels;
    (*op_cfg).cfg.operation_mode = PARAM_ID_SP_VI_OP_MODE_NORMAL;
    p = ptr_add::<c_void>(p, op_sz);

    let cm_cfg = p as *mut apm_module_sp_vi_channel_map_cfg;
    let param_data = &mut (*cm_cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_SP_VI_CHANNEL_MAP_CFG;
    param_data.param_size = (cm_sz - APM_MODULE_PARAM_DATA_SIZE) as _;
    (*cm_cfg).cfg.num_channels = num_channels * 2;
    /* Convert the physical mapping to a logical mapping of the channels */
    let mut i = 0;
    let mut j = 0;
    while i < AR_PCM_MAX_NUM_CHANNEL && j < num_channels as i32 {
        if (*mcfg).channel_map[i as usize] != 0 {
            /*
             * Map speakers into Vsense and then Isense of each channel.
             * E.g. for PCM_CHANNEL_FL and PCM_CHANNEL_FR to:
             * [1, 2, 3, 4]
             */
            (*cm_cfg).cfg.channel_mapping[(2 * j) as usize] =
                ((*mcfg).channel_map[i as usize] - 1) * 2 + 1;
            (*cm_cfg).cfg.channel_mapping[(2 * j + 1) as usize] =
                ((*mcfg).channel_map[i as usize] - 1) * 2 + 2;
            j += 1;
        }
        i += 1;
    }
    p = ptr_add::<c_void>(p, cm_sz);

    let ex_cfg = p as *mut apm_module_sp_vi_ex_mode_cfg;
    let param_data = &mut (*ex_cfg).param_data;
    param_data.module_instance_id = (*module).instance_id;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_SP_VI_EX_MODE_CFG;
    param_data.param_size = (ex_sz - APM_MODULE_PARAM_DATA_SIZE) as _;
    (*ex_cfg).cfg.factory_mode = 0;
    let rc = q6apm_send_cmd_sync((*graph).apm, pkt, 0);
    kfree(pkt as *mut c_void);
    rc
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_set_media_format(
    graph: *mut q6apm_graph,
    module: *const audioreach_module,
    cfg: *const audioreach_module_config,
) -> i32 {
    let rc: i32;
    match (*module).module_id {
        MODULE_ID_DATA_LOGGING => {
            rc = audioreach_module_enable(graph, module, true);
            if rc == 0 {
                return audioreach_logging_set_media_format(graph, module);
            }
        }
        MODULE_ID_PCM_DEC | MODULE_ID_PCM_ENC | MODULE_ID_PCM_CNV
        | MODULE_ID_PLACEHOLDER_DECODER | MODULE_ID_PLACEHOLDER_ENCODER => {
            rc = audioreach_pcm_set_media_format(graph, module, cfg);
        }
        MODULE_ID_DISPLAY_PORT_SINK => rc = audioreach_display_port_set_media_format(graph, module, cfg),
        MODULE_ID_SMECNS_V2 => rc = audioreach_set_module_config(graph, module, cfg),
        MODULE_ID_I2S_SOURCE | MODULE_ID_I2S_SINK => rc = audioreach_i2s_set_media_format(graph, module, cfg),
        MODULE_ID_WR_SHARED_MEM_EP | MODULE_ID_SH_MEM_PULL_MODE => {
            rc = audioreach_shmem_set_media_format(graph, module, cfg)
        }
        MODULE_ID_GAIN => rc = audioreach_gain_set(graph, module),
        MODULE_ID_CODEC_DMA_SINK | MODULE_ID_CODEC_DMA_SOURCE => {
            rc = audioreach_codec_dma_set_media_format(graph, module, cfg)
        }
        MODULE_ID_SAL => {
            rc = audioreach_sal_set_media_format(graph, module, cfg);
            if rc == 0 {
                return audioreach_sal_limiter_enable(graph, module, true);
            }
        }
        MODULE_ID_MFC => rc = audioreach_mfc_set_media_format(graph, module, cfg),
        MODULE_ID_GAPLESS => rc = audioreach_gapless_set_media_format(graph, module, cfg),
        MODULE_ID_SPEAKER_PROTECTION => {
            rc = audioreach_speaker_protection(graph, module, PARAM_ID_SP_OP_MODE_NORMAL);
            if rc == 0 {
                return audioreach_module_enable(graph, module, true);
            }
        }
        MODULE_ID_SPEAKER_PROTECTION_VI => {
            rc = audioreach_speaker_protection_vi(graph, module, cfg);
            if rc == 0 {
                return audioreach_module_enable(graph, module, true);
            }
        }
        MODULE_ID_AUDIO_IF_SOURCE | MODULE_ID_AUDIO_IF_SINK => {
            rc = audioreach_audio_if_set_media_format(graph, module, cfg)
        }
        _ => rc = 0,
    }
    rc
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_graph_free_buf(graph: *mut q6apm_graph) {
    mutex_lock(&mut (*graph).lock);
    let mut port = &mut (*graph).rx_data as *mut audioreach_graph_data;
    (*port).num_periods = 0;
    kfree((*port).buf as *mut c_void);
    (*port).buf = ptr::null_mut();

    port = &mut (*graph).tx_data;
    (*port).num_periods = 0;
    kfree((*port).buf as *mut c_void);
    (*port).buf = ptr::null_mut();
    mutex_unlock(&mut (*graph).lock);
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_setup_push_pull(
    graph: *mut q6apm_graph,
    bphys: phys_addr_t,
    pphys: phys_addr_t,
    mem_map_handle: uint32_t,
    pos_buf_mem_map_handle: uint32_t,
    size: uint32_t,
) -> i32 {
    let payload_size = size_of::<param_id_sh_mem_pull_push_mode_cfg>() + APM_MODULE_PARAM_DATA_SIZE;
    let pkt = audioreach_alloc_apm_cmd_pkt(payload_size as i32, APM_CMD_SET_CFG, 0) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let mut p = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut c_void;
    let param_data = p as *mut apm_module_param_data;
    (*param_data).module_instance_id = (*graph).shm_iid;
    (*param_data).error_code = 0;
    (*param_data).param_id = PARAM_ID_SH_MEM_PULL_PUSH_MODE_CFG;
    (*param_data).param_size = (payload_size - APM_MODULE_PARAM_DATA_SIZE) as _;
    p = ptr_add::<c_void>(p, APM_MODULE_PARAM_DATA_SIZE);
    let cfg = p as *mut param_id_sh_mem_pull_push_mode_cfg;
    (*cfg).shared_circ_buf_addr_lsw = lower_32_bits(bphys);
    (*cfg).shared_circ_buf_addr_msw = upper_32_bits(bphys);
    (*cfg).shared_circ_buf_size = size;
    (*cfg).circ_buf_mem_map_handle = mem_map_handle;
    (*cfg).shared_pos_buf_addr_lsw = lower_32_bits(pphys);
    (*cfg).shared_pos_buf_addr_msw = upper_32_bits(pphys);
    (*cfg).pos_buf_mem_map_handle = pos_buf_mem_map_handle;
    q6apm_send_cmd_sync((*graph).apm, pkt, 0)
}

#[no_mangle]
pub unsafe extern "C" fn audioreach_shared_memory_send_eos(graph: *mut q6apm_graph) -> i32 {
    let iid = (*graph).shm_iid;
    let pkt = audioreach_alloc_cmd_pkt(
        size_of::<data_cmd_wr_sh_mem_ep_eos>() as i32,
        DATA_CMD_WR_SH_MEM_EP_EOS,
        0,
        (*(*graph).port).id,
        iid as _,
    ) as *mut gpr_pkt;
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }
    let eos = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE)
        as *mut data_cmd_wr_sh_mem_ep_eos;
    (*eos).policy = WR_SH_MEM_EP_EOS_POLICY_LAST;
    gpr_send_port_pkt((*graph).port, pkt)
}

// External declarations, constants, and layout definitions are supplied by the
// Rust translations of the original C headers included by audioreach.c.
// The list iteration helpers below represent Linux list_for_each_entry over the
// corresponding container member and intentionally preserve that dependency.
extern "C" {
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn ERR_PTR(err: i32) -> *mut c_void;
    fn IS_ERR(p: *const c_void) -> bool;
    fn PTR_ERR(p: *const c_void) -> i32;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn le32_to_cpu(v: u32) -> u32;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn gpr_send_port_pkt(port: *mut gpr_port_t, pkt: *const gpr_pkt) -> i32;
    fn gpr_send_pkt(gdev: *mut gpr_device_t, pkt: *const gpr_pkt) -> i32;
    fn q6apm_send_cmd_sync(apm: *mut q6apm, pkt: *const gpr_pkt, rsp_opcode: u32) -> i32;
    fn wait_event_timeout(wait: *mut wait_queue_head_t, condition: bool, timeout: i32) -> i32;
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn list_for_each_entry_audioreach_sub_graph<F: FnMut(*mut audioreach_sub_graph)>(
        head: *const list_head,
        f: F,
    );
    fn list_for_each_entry_audioreach_container<F: FnMut(*mut audioreach_container)>(
        head: *const list_head,
        f: F,
    );
    fn list_for_each_entry_audioreach_module<F: FnMut(*mut audioreach_module)>(
        head: *const list_head,
        f: F,
    );
}

extern "C" {
    static GFP_KERNEL: u32;
    static ENOMEM: i32;
    static EINVAL: i32;
    static ETIMEDOUT: i32;
    static HZ: i32;
    static GPR_HDR_SIZE: usize;
    static APM_CMD_HDR_SIZE: usize;
    static APM_MODULE_PARAM_DATA_SIZE: usize;
    static GPR_PKT_VER: u32;
    static GPR_PKT_HEADER_WORD_SIZE: u32;
    static GPR_DOMAIN_ID_ADSP: u32;
    static GPR_DOMAIN_ID_APPS: u32;
    static APM_MODULE_INSTANCE_ID: u32;
    static GPR_APM_MODULE_IID: u32;
    static PCM_CHANNEL_FL: u32;
    static PCM_CHANNEL_FR: u32;
    static PCM_CHANNEL_LS: u32;
    static PCM_CHANNEL_RS: u32;
    static APM_CONTAINER_PROP_ID_CAPABILITY_LIST: u32;
    static APM_CONTAINER_PROP_ID_CAPABILITY_SIZE: u32;
    static APM_CONTAINER_PROP_ID_GRAPH_POS: u32;
    static APM_CONTAINER_PROP_ID_STACK_SIZE: u32;
    static APM_CONTAINER_PROP_ID_PROC_DOMAIN: u32;
    static APM_SUB_GRAPH_PROP_ID_PERF_MODE: u32;
    static APM_SG_PROP_ID_PERF_MODE_SIZE: u32;
    static APM_SUB_GRAPH_PROP_ID_DIRECTION: u32;
    static APM_SG_PROP_ID_DIR_SIZE: u32;
    static APM_SUB_GRAPH_PROP_ID_SCENARIO_ID: u32;
    static APM_SG_PROP_ID_SID_SIZE: u32;
    static APM_MODULE_PROP_ID_PORT_INFO: u32;
    static APM_MODULE_PROP_ID_PORT_INFO_SZ: u32;
    static APM_CMD_GRAPH_OPEN: u32;
    static APM_PARAM_ID_SUB_GRAPH_CONFIG: u32;
    static APM_PARAM_ID_CONTAINER_CONFIG: u32;
    static APM_PARAM_ID_MODULE_LIST: u32;
    static APM_PARAM_ID_MODULE_PROP: u32;
    static APM_PARAM_ID_MODULE_CONN: u32;
    static APM_CMD_SET_CFG: u32;
    static PARAM_ID_HW_EP_MF_CFG: u32;
    static PARAM_ID_HW_EP_FRAME_SIZE_FACTOR: u32;
    static PARAM_ID_DISPLAY_PORT_INTF_CFG: u32;
    static PARAM_ID_CODEC_DMA_INTF_CFG: u32;
    static PARAM_ID_HW_EP_POWER_MODE_CFG: u32;
    static AR_PCM_MAX_NUM_CHANNEL: i32;
    static PARAM_ID_SAL_LIMITER_ENABLE: u32;
    static PARAM_ID_SAL_OUTPUT_CFG: u32;
    static PARAM_ID_MODULE_ENABLE: u32;
    static PARAM_ID_EARLY_EOS_DELAY: u32;
    static EARLY_EOS_DELAY_MS: u32;
    static PARAM_ID_MFC_OUTPUT_MEDIA_FORMAT: u32;
    static SND_AUDIOCODEC_MP3: u32;
    static SND_AUDIOCODEC_AAC: u32;
    static SND_AUDIOCODEC_FLAC: u32;
    static SND_AUDIOCODEC_OPUS_RAW: u32;
    static SND_AUDIOCODEC_PCM: u32;
    static DATA_FORMAT_RAW_COMPRESSED: u32;
    static DATA_FORMAT_FIXED_POINT: u32;
    static MEDIA_FMT_ID_MP3: u32;
    static MEDIA_FMT_ID_AAC: u32;
    static MEDIA_FMT_ID_FLAC: u32;
    static MEDIA_FMT_ID_OPUS: u32;
    static MEDIA_FMT_ID_PCM: u32;
    static PCM_LSB_ALIGNED: u32;
    static PCM_LITTLE_ENDIAN: u32;
    static DATA_CMD_WR_SH_MEM_EP_MEDIA_FORMAT: u32;
    static PARAM_ID_I2S_INTF_CFG: u32;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: u32;
    static SND_SOC_DAIFMT_BP_FP: u32;
    static SND_SOC_DAIFMT_BC_FC: u32;
    static CONFIG_I2S_WS_SRC_INTERNAL: u32;
    static CONFIG_I2S_WS_SRC_EXTERNAL: u32;
    static PARAM_ID_AUDIO_IF_INTF_CFG: u32;
    static AUDIO_IF_FRAME_DURATION_US: u32;
    static AUDIO_IF_FRAME_DURATION_NORMALIZATION_ENABLE: u32;
    static AUDIO_IF_FRAME_DURATION_MIN_US: u32;
    static AUDIO_IF_FRAME_DURATION_MAX_US: u32;
    static PARAM_ID_HW_EP_FRAME_DURATION: u32;
    static PARAM_ID_DATA_LOGGING_CONFIG: u32;
    static PARAM_ID_PCM_OUTPUT_FORMAT_CFG: u32;
    static APM_CMD_REGISTER_MODULE_EVENTS: u32;
    static EVENT_ID_SH_MEM_PULL_PUSH_MODE_WATERMARK: u32;
    static PARAM_ID_MEDIA_FORMAT: u32;
    static PARAM_ID_VOL_CTRL_MASTER_GAIN: u32;
    static APM_PARAM_ID_GAIN: u32;
    static PARAM_ID_SP_OP_MODE: u32;
    static PARAM_ID_SP_OP_MODE_NORMAL: u32;
    static PARAM_ID_SP_VI_OP_MODE_CFG: u32;
    static PARAM_ID_SP_VI_OP_MODE_NORMAL: u32;
    static PARAM_ID_SP_VI_CHANNEL_MAP_CFG: u32;
    static PARAM_ID_SP_VI_EX_MODE_CFG: u32;
    static MODULE_ID_DATA_LOGGING: u32;
    static MODULE_ID_PCM_DEC: u32;
    static MODULE_ID_PCM_ENC: u32;
    static MODULE_ID_PCM_CNV: u32;
    static MODULE_ID_PLACEHOLDER_DECODER: u32;
    static MODULE_ID_PLACEHOLDER_ENCODER: u32;
    static MODULE_ID_DISPLAY_PORT_SINK: u32;
    static MODULE_ID_SMECNS_V2: u32;
    static MODULE_ID_I2S_SOURCE: u32;
    static MODULE_ID_I2S_SINK: u32;
    static MODULE_ID_WR_SHARED_MEM_EP: u32;
    static MODULE_ID_SH_MEM_PULL_MODE: u32;
    static MODULE_ID_GAIN: u32;
    static MODULE_ID_CODEC_DMA_SINK: u32;
    static MODULE_ID_CODEC_DMA_SOURCE: u32;
    static MODULE_ID_SAL: u32;
    static MODULE_ID_MFC: u32;
    static MODULE_ID_GAPLESS: u32;
    static MODULE_ID_SPEAKER_PROTECTION: u32;
    static MODULE_ID_SPEAKER_PROTECTION_VI: u32;
    static MODULE_ID_AUDIO_IF_SOURCE: u32;
    static MODULE_ID_AUDIO_IF_SINK: u32;
    static PARAM_ID_SH_MEM_PULL_PUSH_MODE_CFG: u32;
    static DATA_CMD_WR_SH_MEM_EP_EOS: u32;
    static WR_SH_MEM_EP_EOS_POLICY_LAST: u32;
}

// Header-provided C structs referenced by field in this translation.
// Their concrete Rust definitions are expected from the translated dependencies.
type device = crate::device;
type mutex = crate::mutex;
type wait_queue_head_t = crate::wait_queue_head_t;
type list_head = crate::list_head;
type gpr_device_t = crate::gpr_device_t;
type gpr_port_t = crate::gpr_port_t;
type q6apm = crate::q6apm;
type q6apm_graph = crate::q6apm_graph;
type gpr_hdr = crate::gpr_hdr;
type gpr_pkt = crate::gpr_pkt;
type gpr_ibasic_rsp_result_t = crate::gpr_ibasic_rsp_result_t;
type apm_cmd_header = crate::apm_cmd_header;
type apm_sub_graph_cfg = crate::apm_sub_graph_cfg;
type apm_prop_data = crate::apm_prop_data;
type apm_sg_prop_id_perf_mode = crate::apm_sg_prop_id_perf_mode;
type apm_sg_prop_id_direction = crate::apm_sg_prop_id_direction;
type apm_sg_prop_id_scenario_id = crate::apm_sg_prop_id_scenario_id;
type apm_module_param_data = crate::apm_module_param_data;
type apm_container_cfg = crate::apm_container_cfg;
type apm_cont_prop_id_graph_pos = crate::apm_cont_prop_id_graph_pos;
type apm_cont_prop_id_stack_size = crate::apm_cont_prop_id_stack_size;
type apm_cont_prop_id_domain = crate::apm_cont_prop_id_domain;
type apm_module_obj = crate::apm_module_obj;
type apm_module_prop_id_port_info = crate::apm_module_prop_id_port_info;
type apm_module_conn_obj = crate::apm_module_conn_obj;
type param_id_pcm_output_format_cfg = crate::param_id_pcm_output_format_cfg;
type payload_pcm_output_format_cfg = crate::payload_pcm_output_format_cfg;
type param_id_rd_sh_mem_cfg = crate::param_id_rd_sh_mem_cfg;
type media_format = crate::media_format;
type payload_media_fmt_pcm = crate::payload_media_fmt_pcm;
type param_id_i2s_intf_cfg = crate::param_id_i2s_intf_cfg;
type param_id_audio_if_intf_cfg = crate::param_id_audio_if_intf_cfg;
type param_id_hw_ep_mf = crate::param_id_hw_ep_mf;
type param_id_mfc_media_format = crate::param_id_mfc_media_format;
type param_id_hw_ep_frame_duration = crate::param_id_hw_ep_frame_duration;
type param_id_hw_ep_power_mode_cfg = crate::param_id_hw_ep_power_mode_cfg;
type param_id_hw_ep_dma_data_align = crate::param_id_hw_ep_dma_data_align;
type param_id_gain_cfg = crate::param_id_gain_cfg;
type param_id_codec_dma_intf_cfg = crate::param_id_codec_dma_intf_cfg;
type param_id_display_port_intf_cfg = crate::param_id_display_port_intf_cfg;
type param_id_sp_vi_op_mode_cfg = crate::param_id_sp_vi_op_mode_cfg;
type param_id_sp_vi_ex_mode_cfg = crate::param_id_sp_vi_ex_mode_cfg;
type param_id_sp_vi_channel_map_cfg = crate::param_id_sp_vi_channel_map_cfg;
type audioreach_container = crate::audioreach_container;
type audioreach_sub_graph = crate::audioreach_sub_graph;
type audioreach_module = crate::audioreach_module;
type audioreach_graph_info = crate::audioreach_graph_info;
type audioreach_module_config = crate::audioreach_module_config;
type payload_media_fmt_aac_t = crate::payload_media_fmt_aac_t;
type payload_media_fmt_flac_t = crate::payload_media_fmt_flac_t;
type payload_media_fmt_opus_t = crate::payload_media_fmt_opus_t;
type data_logging_config = crate::data_logging_config;
type apm_module_register_events = crate::apm_module_register_events;
type event_cfg_sh_mem_pull_push_mode_watermark_t =
    crate::event_cfg_sh_mem_pull_push_mode_watermark_t;
type param_id_vol_ctrl_master_gain = crate::param_id_vol_ctrl_master_gain;
type audioreach_graph_data = crate::audioreach_graph_data;
type param_id_sh_mem_pull_push_mode_cfg = crate::param_id_sh_mem_pull_push_mode_cfg;
type data_cmd_wr_sh_mem_ep_eos = crate::data_cmd_wr_sh_mem_ep_eos;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
