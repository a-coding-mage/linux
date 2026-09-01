// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020, Linaro Limited

// Rust translation of soc/qcom/qdsp6/q6apm.c.
// Dependencies originally provided by Linux/QDSP6 headers are referenced as
// external C-compatible items.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type phys_addr_t = usize;
type size_t = usize;
type bool_t = bool;
type q6apm_cb = Option<unsafe extern "C" fn(u32, u32, *mut c_void, *mut c_void)>;

const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const GFP_KERNEL: c_uint = 0;

#[repr(C, packed)]
pub struct apm_graph_mgmt_cmd {
    pub param_data: apm_module_param_data,
    pub num_sub_graphs: u32,
    pub sub_graph_id_list: [u32; 0],
}

macro_rules! APM_GRAPH_MGMT_PSIZE {
    ($p:expr, $n:expr) => {
        ALIGN(
            size_of::<apm_graph_mgmt_cmd>() + (($n) as usize * size_of::<u32>()),
            8,
        )
    };
}

static mut g_apm: *mut q6apm = ptr::null_mut();

unsafe fn ALIGN(x: usize, a: usize) -> usize {
    (x + a - 1) & !(a - 1)
}

unsafe fn lower_32_bits(x: phys_addr_t) -> u32 {
    x as u32
}

unsafe fn upper_32_bits(x: phys_addr_t) -> u32 {
    ((x as u64) >> 32) as u32
}

pub unsafe extern "C" fn q6apm_send_cmd_sync(
    apm: *mut q6apm,
    pkt: *const gpr_pkt,
    rsp_opcode: u32,
) -> c_int {
    let gdev = (*apm).gdev;

    audioreach_send_cmd_sync(
        &mut (*gdev).dev,
        gdev,
        &mut (*apm).result,
        &mut (*apm).lock,
        ptr::null_mut(),
        &mut (*apm).wait,
        pkt,
        rsp_opcode,
    )
}

unsafe fn q6apm_get_audioreach_graph(apm: *mut q6apm, graph_id: u32) -> *mut audioreach_graph {
    let mut info: *mut audioreach_graph_info;
    let mut graph: *mut audioreach_graph;
    let id: c_int;

    mutex_lock(&mut (*apm).lock);
    graph = idr_find(&mut (*apm).graph_idr, graph_id) as *mut audioreach_graph;
    mutex_unlock(&mut (*apm).lock);

    if !graph.is_null() {
        kref_get(&mut (*graph).refcount);
        return graph;
    }

    info = idr_find(&mut (*apm).graph_info_idr, graph_id) as *mut audioreach_graph_info;

    if info.is_null() {
        return ERR_PTR(-ENODEV) as *mut audioreach_graph;
    }

    graph = kzalloc_obj::<audioreach_graph>();
    if graph.is_null() {
        return ERR_PTR(-ENOMEM) as *mut audioreach_graph;
    }

    (*graph).apm = apm;
    (*graph).info = info;
    (*graph).id = graph_id;

    (*graph).graph = audioreach_alloc_graph_pkt(apm, info);
    if IS_ERR((*graph).graph as *const c_void) {
        let err = (*graph).graph as *mut c_void;

        kfree(graph as *mut c_void);
        return ERR_CAST(err) as *mut audioreach_graph;
    }

    mutex_lock(&mut (*apm).lock);
    id = idr_alloc(
        &mut (*apm).graph_idr,
        graph as *mut c_void,
        graph_id as c_int,
        graph_id.wrapping_add(1) as c_int,
        GFP_KERNEL,
    );
    if id < 0 {
        dev_err(
            (*apm).dev,
            b"Unable to allocate graph id (%d)\n\0".as_ptr() as *const c_char,
            graph_id,
        );
        kfree((*graph).graph as *mut c_void);
        kfree(graph as *mut c_void);
        mutex_unlock(&mut (*apm).lock);
        return ERR_PTR(id) as *mut audioreach_graph;
    }
    mutex_unlock(&mut (*apm).lock);

    kref_init(&mut (*graph).refcount);

    q6apm_send_cmd_sync(apm, (*graph).graph, 0);

    graph
}

unsafe fn audioreach_graph_mgmt_cmd(graph: *mut audioreach_graph, opcode: u32) -> c_int {
    let info = (*graph).info;
    let num_sub_graphs = (*info).num_sub_graphs;
    let mut param_data: *mut apm_module_param_data;
    let mut mgmt_cmd: *mut apm_graph_mgmt_cmd;
    let mut sg: *mut audioreach_sub_graph;
    let apm = (*graph).apm;
    let mut i: c_int = 0;
    let payload_size = APM_GRAPH_MGMT_PSIZE!(mgmt_cmd, num_sub_graphs);

    let pkt = audioreach_alloc_apm_cmd_pkt(payload_size, opcode, 0);
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }

    mgmt_cmd = (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut apm_graph_mgmt_cmd;

    (*mgmt_cmd).num_sub_graphs = num_sub_graphs as u32;

    param_data = &mut (*mgmt_cmd).param_data;
    (*param_data).module_instance_id = APM_MODULE_INSTANCE_ID;
    (*param_data).param_id = APM_PARAM_ID_SUB_GRAPH_LIST;
    (*param_data).param_size = (payload_size - APM_MODULE_PARAM_DATA_SIZE) as u32;

    sg = list_first_entry_or_null(
        &mut (*info).sg_list,
        audioreach_sub_graph_node_offset(),
    ) as *mut audioreach_sub_graph;
    while !sg.is_null() {
        *(*mgmt_cmd).sub_graph_id_list.as_mut_ptr().add(i as usize) = (*sg).sub_graph_id;
        i += 1;
        sg = list_next_entry_or_null(
            sg as *mut c_void,
            &mut (*info).sg_list,
            audioreach_sub_graph_node_offset(),
        ) as *mut audioreach_sub_graph;
    }

    let ret = q6apm_send_cmd_sync(apm, pkt, 0);
    kfree(pkt as *mut c_void);
    ret
}

unsafe extern "C" fn q6apm_put_audioreach_graph(ref_: *mut kref) {
    let mut graph: *mut audioreach_graph;
    let apm: *mut q6apm;

    graph = container_of_audioreach_graph_refcount(ref_);
    apm = (*graph).apm;

    audioreach_graph_mgmt_cmd(graph, APM_CMD_GRAPH_CLOSE);

    mutex_lock(&mut (*apm).lock);
    graph = idr_remove(&mut (*apm).graph_idr, (*graph).id) as *mut audioreach_graph;
    mutex_unlock(&mut (*apm).lock);

    kfree((*graph).graph as *mut c_void);
    kfree(graph as *mut c_void);
}

unsafe fn q6apm_get_apm_state(apm: *mut q6apm) -> c_int {
    let pkt = audioreach_alloc_apm_cmd_pkt(0, APM_CMD_GET_SPF_STATE, 0);
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }

    q6apm_send_cmd_sync(apm, pkt, APM_CMD_RSP_GET_SPF_STATE);
    kfree(pkt as *mut c_void);

    (*apm).state
}

pub unsafe extern "C" fn q6apm_is_adsp_ready() -> bool_t {
    if !g_apm.is_null() {
        return q6apm_get_apm_state(g_apm) != 0;
    }

    false
}

unsafe fn __q6apm_find_module_by_mid(
    _apm: *mut q6apm,
    info: *mut audioreach_graph_info,
    mid: u32,
) -> *mut audioreach_module {
    let mut container: *mut audioreach_container;
    let mut sgs: *mut audioreach_sub_graph;
    let mut module: *mut audioreach_module;

    sgs = list_first_entry_or_null(&mut (*info).sg_list, audioreach_sub_graph_node_offset())
        as *mut audioreach_sub_graph;
    while !sgs.is_null() {
        container = list_first_entry_or_null(
            &mut (*sgs).container_list,
            audioreach_container_node_offset(),
        ) as *mut audioreach_container;
        while !container.is_null() {
            module = list_first_entry_or_null(
                &mut (*container).modules_list,
                audioreach_module_node_offset(),
            ) as *mut audioreach_module;
            while !module.is_null() {
                if mid == (*module).module_id {
                    return module;
                }
                module = list_next_entry_or_null(
                    module as *mut c_void,
                    &mut (*container).modules_list,
                    audioreach_module_node_offset(),
                ) as *mut audioreach_module;
            }
            container = list_next_entry_or_null(
                container as *mut c_void,
                &mut (*sgs).container_list,
                audioreach_container_node_offset(),
            ) as *mut audioreach_container;
        }
        sgs = list_next_entry_or_null(
            sgs as *mut c_void,
            &mut (*info).sg_list,
            audioreach_sub_graph_node_offset(),
        ) as *mut audioreach_sub_graph;
    }

    ptr::null_mut()
}

pub unsafe extern "C" fn q6apm_graph_media_format_shmem(
    graph: *mut q6apm_graph,
    cfg: *mut audioreach_module_config,
) -> c_int {
    let mut module: *mut audioreach_module;

    if (*cfg).direction == SNDRV_PCM_STREAM_CAPTURE {
        module = q6apm_find_module_by_mid(graph, MODULE_ID_SH_MEM_PUSH_MODE);
        if module.is_null() {
            module = q6apm_find_module_by_mid(graph, MODULE_ID_RD_SHARED_MEM_EP);
        }
    } else {
        module = q6apm_find_module_by_mid(graph, MODULE_ID_SH_MEM_PULL_MODE);
        if module.is_null() {
            module = q6apm_find_module_by_mid(graph, MODULE_ID_WR_SHARED_MEM_EP);
        }
    }

    if module.is_null() {
        dev_err(
            (*graph).dev,
            b"No SHMEM module found in graph\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }

    audioreach_set_media_format(graph, module, cfg)
}

unsafe fn __q6apm_map_memory_fixed_region(
    dev: *mut device,
    graph_id: c_uint,
    phys: phys_addr_t,
    sz: size_t,
    is_pos_buf: bool_t,
) -> c_int {
    let mut info: *mut audioreach_graph_info;
    let apm = dev_get_drvdata((*dev).parent) as *mut q6apm;
    let mregions: *mut apm_shared_map_region_payload;
    let cmd: *mut apm_cmd_shared_mem_map_regions;
    let payload_size = size_of::<apm_cmd_shared_mem_map_regions>()
        + size_of::<apm_shared_map_region_payload>();
    let buf_sz: u32;
    let p: *mut u8;
    let pos_mask: u32 = if is_pos_buf { APM_MMAP_TOKEN_MAP_TYPE_POS_BUF } else { 0 };
    let pkt = audioreach_alloc_apm_cmd_pkt(
        payload_size,
        APM_CMD_SHARED_MEM_MAP_REGIONS,
        graph_id | pos_mask,
    );

    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }

    info = idr_find(&mut (*apm).graph_info_idr, graph_id) as *mut audioreach_graph_info;
    if info.is_null() {
        kfree(pkt as *mut c_void);
        return -ENODEV;
    }

    if is_pos_buf {
        if (*info).pos_buf_mem_map_handle != 0 {
            kfree(pkt as *mut c_void);
            return 0;
        }
    } else if (*info).mem_map_handle != 0 {
        kfree(pkt as *mut c_void);
        return 0;
    }

    // DSP expects size should be aligned to 4K
    buf_sz = ALIGN(sz, 4096) as u32;

    p = (pkt as *mut u8).add(GPR_HDR_SIZE);
    cmd = p as *mut apm_cmd_shared_mem_map_regions;
    (*cmd).mem_pool_id = APM_MEMORY_MAP_SHMEM8_4K_POOL;
    (*cmd).num_regions = 1;
    if is_pos_buf {
        (*cmd).property_flag = 0x2;
    } else {
        (*cmd).property_flag = 0x0;
    }

    mregions = p.add(size_of::<apm_cmd_shared_mem_map_regions>()) as *mut apm_shared_map_region_payload;

    (*mregions).shm_addr_lsw = lower_32_bits(phys);
    (*mregions).shm_addr_msw = upper_32_bits(phys);
    (*mregions).mem_size_bytes = buf_sz;

    let ret = q6apm_send_cmd_sync(apm, pkt, APM_CMD_RSP_SHARED_MEM_MAP_REGIONS);
    kfree(pkt as *mut c_void);
    ret
}

pub unsafe extern "C" fn q6apm_map_pos_buffer(
    dev: *mut device,
    graph_id: c_uint,
    phys: phys_addr_t,
    sz: size_t,
) -> c_int {
    __q6apm_map_memory_fixed_region(dev, graph_id, phys, sz, true)
}

pub unsafe extern "C" fn q6apm_map_memory_fixed_region(
    dev: *mut device,
    graph_id: c_uint,
    phys: phys_addr_t,
    sz: size_t,
) -> c_int {
    __q6apm_map_memory_fixed_region(dev, graph_id, phys, sz, false)
}

pub unsafe extern "C" fn q6apm_alloc_fragments(
    graph: *mut q6apm_graph,
    dir: c_uint,
    phys: phys_addr_t,
    period_sz: size_t,
    periods: c_uint,
) -> c_int {
    let mut data: *mut audioreach_graph_data;
    let buf: *mut audio_buffer;
    let mut cnt: c_uint;

    if dir == SNDRV_PCM_STREAM_PLAYBACK as c_uint {
        data = &mut (*graph).rx_data;
    } else {
        data = &mut (*graph).tx_data;
    }

    mutex_lock(&mut (*graph).lock);

    (*data).dsp_buf = 0;

    if !(*data).buf.is_null() {
        mutex_unlock(&mut (*graph).lock);
        return 0;
    }

    buf = kzalloc_array::<audio_buffer>(periods as usize);
    if buf.is_null() {
        mutex_unlock(&mut (*graph).lock);
        return -ENOMEM;
    }

    if dir == SNDRV_PCM_STREAM_PLAYBACK as c_uint {
        data = &mut (*graph).rx_data;
    } else {
        data = &mut (*graph).tx_data;
    }

    (*data).buf = buf;

    (*buf.add(0)).phys = phys;
    (*buf.add(0)).size = period_sz;

    cnt = 1;
    while cnt < periods {
        if period_sz > 0 {
            (*buf.add(cnt as usize)).phys = (*buf.add(0)).phys + (cnt as usize * period_sz);
            (*buf.add(cnt as usize)).size = period_sz;
        }
        cnt += 1;
    }
    (*data).num_periods = periods;

    mutex_unlock(&mut (*graph).lock);

    0
}

unsafe fn __q6apm_unmap_memory_fixed_region(
    dev: *mut device,
    graph_id: c_uint,
    is_pos_buf: bool_t,
) -> c_int {
    let cmd: *mut apm_cmd_shared_mem_unmap_regions;
    let apm = dev_get_drvdata((*dev).parent) as *mut q6apm;
    let info: *mut audioreach_graph_info;
    let mem_map_handle: u32;
    let pkt = audioreach_alloc_apm_cmd_pkt(
        size_of::<apm_cmd_shared_mem_unmap_regions>(),
        APM_CMD_SHARED_MEM_UNMAP_REGIONS,
        graph_id,
    );
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }

    info = idr_find(&mut (*apm).graph_info_idr, graph_id) as *mut audioreach_graph_info;
    if info.is_null() {
        kfree(pkt as *mut c_void);
        return -ENODEV;
    }

    if is_pos_buf {
        if (*info).pos_buf_mem_map_handle == 0 {
            kfree(pkt as *mut c_void);
            return 0;
        }
        mem_map_handle = (*info).pos_buf_mem_map_handle;
    } else {
        if (*info).mem_map_handle == 0 {
            kfree(pkt as *mut c_void);
            return 0;
        }
        mem_map_handle = (*info).mem_map_handle;
    }

    cmd = (pkt as *mut u8).add(GPR_HDR_SIZE) as *mut apm_cmd_shared_mem_unmap_regions;
    (*cmd).mem_map_handle = mem_map_handle;

    let ret = q6apm_send_cmd_sync(apm, pkt, APM_CMD_SHARED_MEM_UNMAP_REGIONS);
    kfree(pkt as *mut c_void);
    ret
}

pub unsafe extern "C" fn q6apm_unmap_memory_fixed_region(
    dev: *mut device,
    graph_id: c_uint,
) -> c_int {
    __q6apm_unmap_memory_fixed_region(dev, graph_id, false)
}

pub unsafe extern "C" fn q6apm_unmap_pos_buffer(dev: *mut device, graph_id: c_uint) -> c_int {
    __q6apm_unmap_memory_fixed_region(dev, graph_id, true)
}

pub unsafe extern "C" fn q6apm_free_fragments(
    graph: *mut q6apm_graph,
    _dir: c_uint,
) -> c_int {
    audioreach_graph_free_buf(graph);

    0
}

pub unsafe extern "C" fn q6apm_remove_initial_silence(
    _dev: *mut device,
    graph: *mut q6apm_graph,
    samples: u32,
) -> c_int {
    let module = q6apm_find_module_by_mid(graph, MODULE_ID_PLACEHOLDER_DECODER);

    if module.is_null() {
        return -ENODEV;
    }

    audioreach_send_u32_param(graph, module, PARAM_ID_REMOVE_INITIAL_SILENCE, samples)
}

pub unsafe extern "C" fn q6apm_remove_trailing_silence(
    _dev: *mut device,
    graph: *mut q6apm_graph,
    samples: u32,
) -> c_int {
    let module = q6apm_find_module_by_mid(graph, MODULE_ID_PLACEHOLDER_DECODER);

    if module.is_null() {
        return -ENODEV;
    }

    audioreach_send_u32_param(graph, module, PARAM_ID_REMOVE_TRAILING_SILENCE, samples)
}

pub unsafe extern "C" fn q6apm_enable_compress_module(
    _dev: *mut device,
    graph: *mut q6apm_graph,
    en: bool_t,
) -> c_int {
    let module = q6apm_find_module_by_mid(graph, MODULE_ID_PLACEHOLDER_DECODER);

    if module.is_null() {
        return -ENODEV;
    }

    audioreach_send_u32_param(graph, module, PARAM_ID_MODULE_ENABLE, en as u32)
}

pub unsafe extern "C" fn q6apm_set_real_module_id(
    _dev: *mut device,
    graph: *mut q6apm_graph,
    codec_id: u32,
) -> c_int {
    let module: *mut audioreach_module;
    let module_id: u32;

    module = q6apm_find_module_by_mid(graph, MODULE_ID_PLACEHOLDER_DECODER);
    if module.is_null() {
        return -ENODEV;
    }

    match codec_id {
        SND_AUDIOCODEC_MP3 => module_id = MODULE_ID_MP3_DECODE,
        SND_AUDIOCODEC_AAC => module_id = MODULE_ID_AAC_DEC,
        SND_AUDIOCODEC_FLAC => module_id = MODULE_ID_FLAC_DEC,
        SND_AUDIOCODEC_OPUS_RAW => module_id = MODULE_ID_OPUS_DEC,
        _ => return -EINVAL,
    }

    audioreach_send_u32_param(graph, module, PARAM_ID_REAL_MODULE_ID, module_id)
}

pub unsafe extern "C" fn q6apm_graph_media_format_pcm(
    graph: *mut q6apm_graph,
    cfg: *mut audioreach_module_config,
) -> c_int {
    let info = (*graph).info;
    let mut sgs: *mut audioreach_sub_graph;
    let mut container: *mut audioreach_container;
    let mut module: *mut audioreach_module;
    let ret: c_int;

    sgs = list_first_entry_or_null(&mut (*info).sg_list, audioreach_sub_graph_node_offset())
        as *mut audioreach_sub_graph;
    while !sgs.is_null() {
        container = list_first_entry_or_null(
            &mut (*sgs).container_list,
            audioreach_container_node_offset(),
        ) as *mut audioreach_container;
        while !container.is_null() {
            module = list_first_entry_or_null(
                &mut (*container).modules_list,
                audioreach_module_node_offset(),
            ) as *mut audioreach_module;
            while !module.is_null() {
                if (*module).module_id == MODULE_ID_WR_SHARED_MEM_EP
                    || (*module).module_id == MODULE_ID_RD_SHARED_MEM_EP
                    || (*module).module_id == MODULE_ID_SH_MEM_PULL_MODE
                    || (*module).module_id == MODULE_ID_SH_MEM_PUSH_MODE
                {
                    module = list_next_entry_or_null(
                        module as *mut c_void,
                        &mut (*container).modules_list,
                        audioreach_module_node_offset(),
                    ) as *mut audioreach_module;
                    continue;
                }

                ret = audioreach_set_media_format(graph, module, cfg);
                if ret != 0 {
                    return ret;
                }

                module = list_next_entry_or_null(
                    module as *mut c_void,
                    &mut (*container).modules_list,
                    audioreach_module_node_offset(),
                ) as *mut audioreach_module;
            }
            container = list_next_entry_or_null(
                container as *mut c_void,
                &mut (*sgs).container_list,
                audioreach_container_node_offset(),
            ) as *mut audioreach_container;
        }
        sgs = list_next_entry_or_null(
            sgs as *mut c_void,
            &mut (*info).sg_list,
            audioreach_sub_graph_node_offset(),
        ) as *mut audioreach_sub_graph;
    }

    0
}

pub unsafe extern "C" fn q6apm_write_async(
    graph: *mut q6apm_graph,
    len: u32,
    msw_ts: u32,
    lsw_ts: u32,
    wflags: u32,
) -> c_int {
    let write_buffer: *mut apm_data_cmd_wr_sh_mem_ep_data_buffer_v2;
    let ab: *mut audio_buffer;

    let pkt = audioreach_alloc_pkt(
        size_of::<apm_data_cmd_wr_sh_mem_ep_data_buffer_v2>(),
        DATA_CMD_WR_SH_MEM_EP_DATA_BUFFER_V2,
        (*graph).rx_data.dsp_buf | (len << APM_WRITE_TOKEN_LEN_SHIFT),
        (*(*graph).port).id,
        (*graph).shm_iid,
    );
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }

    write_buffer = (pkt as *mut u8).add(GPR_HDR_SIZE) as *mut apm_data_cmd_wr_sh_mem_ep_data_buffer_v2;

    mutex_lock(&mut (*graph).lock);
    ab = (*graph).rx_data.buf.add((*graph).rx_data.dsp_buf as usize);

    (*write_buffer).buf_addr_lsw = lower_32_bits((*ab).phys);
    (*write_buffer).buf_addr_msw = upper_32_bits((*ab).phys);
    (*write_buffer).buf_size = len;
    (*write_buffer).timestamp_lsw = lsw_ts;
    (*write_buffer).timestamp_msw = msw_ts;
    (*write_buffer).mem_map_handle = (*(*graph).info).mem_map_handle;
    (*write_buffer).flags = wflags;

    (*graph).rx_data.dsp_buf = (*graph).rx_data.dsp_buf.wrapping_add(1);

    if (*graph).rx_data.dsp_buf >= (*graph).rx_data.num_periods {
        (*graph).rx_data.dsp_buf = 0;
    }

    mutex_unlock(&mut (*graph).lock);

    let ret = gpr_send_port_pkt((*graph).port, pkt);
    kfree(pkt as *mut c_void);
    ret
}

pub unsafe extern "C" fn q6apm_read(graph: *mut q6apm_graph) -> c_int {
    let read_buffer: *mut data_cmd_rd_sh_mem_ep_data_buffer_v2;
    let port: *mut audioreach_graph_data;
    let ab: *mut audio_buffer;

    let pkt = audioreach_alloc_pkt(
        size_of::<data_cmd_rd_sh_mem_ep_data_buffer_v2>(),
        DATA_CMD_RD_SH_MEM_EP_DATA_BUFFER_V2,
        (*graph).tx_data.dsp_buf,
        (*(*graph).port).id,
        (*graph).shm_iid,
    );
    if IS_ERR(pkt as *const c_void) {
        return PTR_ERR(pkt as *const c_void);
    }

    read_buffer = (pkt as *mut u8).add(GPR_HDR_SIZE) as *mut data_cmd_rd_sh_mem_ep_data_buffer_v2;

    mutex_lock(&mut (*graph).lock);
    port = &mut (*graph).tx_data;
    ab = (*port).buf.add((*port).dsp_buf as usize);

    (*read_buffer).buf_addr_lsw = lower_32_bits((*ab).phys);
    (*read_buffer).buf_addr_msw = upper_32_bits((*ab).phys);
    (*read_buffer).mem_map_handle = (*(*graph).info).mem_map_handle;
    (*read_buffer).buf_size = (*ab).size as u32;

    (*port).dsp_buf = (*port).dsp_buf.wrapping_add(1);

    if (*port).dsp_buf >= (*port).num_periods {
        (*port).dsp_buf = 0;
    }

    mutex_unlock(&mut (*graph).lock);

    let ret = gpr_send_port_pkt((*graph).port, pkt);
    kfree(pkt as *mut c_void);
    ret
}

pub unsafe extern "C" fn q6apm_get_hw_pointer(graph: *mut q6apm_graph, dir: c_int) -> c_int {
    let data: *mut audioreach_graph_data = if dir == SNDRV_PCM_STREAM_PLAYBACK {
        &mut (*graph).rx_data
    } else {
        &mut (*graph).tx_data
    };

    atomic_read(&mut (*data).hw_ptr) as c_int
}

unsafe extern "C" fn graph_callback(
    data: *const gpr_resp_pkt,
    priv_: *mut c_void,
    _op: c_int,
) -> c_int {
    let mut rd_done: *mut data_cmd_rsp_rd_sh_mem_ep_data_buffer_done_v2;
    let mut done: *mut data_cmd_rsp_wr_sh_mem_ep_data_buffer_done_v2;
    let mut event: *mut apm_module_event;
    let result: *const gpr_ibasic_rsp_result_t;
    let graph = priv_ as *mut q6apm_graph;
    let hdr = &(*data).hdr as *const gpr_hdr;
    let dev = (*graph).dev;
    let mut client_event: u32;
    let phys: phys_addr_t;
    let token: c_int;

    result = (*data).payload as *const gpr_ibasic_rsp_result_t;

    match (*hdr).opcode {
        APM_EVENT_MODULE_TO_CLIENT => {
            event = (*data).payload as *mut apm_module_event;
            match (*event).event_id {
                EVENT_ID_SH_MEM_PULL_PUSH_MODE_WATERMARK => {
                    client_event = APM_CLIENT_EVENT_WATERMARK_EVENT;
                    if let Some(cb) = (*graph).cb {
                        cb(client_event, (*hdr).token, (*data).payload, (*graph).priv_);
                    }
                }
                _ => {}
            }
        }
        DATA_CMD_RSP_WR_SH_MEM_EP_DATA_BUFFER_DONE_V2 => {
            if (*graph).ar_graph.is_null() {
                return 0;
            }
            client_event = APM_CLIENT_EVENT_DATA_WRITE_DONE;
            mutex_lock(&mut (*graph).lock);
            token = ((*hdr).token & APM_WRITE_TOKEN_MASK) as c_int;

            done = (*data).payload as *mut data_cmd_rsp_wr_sh_mem_ep_data_buffer_done_v2;
            if (*graph).rx_data.buf.is_null() {
                mutex_unlock(&mut (*graph).lock);
                return 0;
            }
            phys = (*(*graph).rx_data.buf.add(token as usize)).phys;
            mutex_unlock(&mut (*graph).lock);
            // token numbering starts at 0
            atomic_set(&mut (*graph).rx_data.hw_ptr, token + 1);
            if lower_32_bits(phys) == (*done).buf_addr_lsw
                && upper_32_bits(phys) == (*done).buf_addr_msw
            {
                (*graph).result.opcode = (*hdr).opcode;
                (*graph).result.status = (*done).status;
                if let Some(cb) = (*graph).cb {
                    cb(client_event, (*hdr).token, (*data).payload, (*graph).priv_);
                }
            } else {
                dev_err(
                    dev,
                    b"WR BUFF Unexpected addr %08x-%08x\n\0".as_ptr() as *const c_char,
                    (*done).buf_addr_lsw,
                    (*done).buf_addr_msw,
                );
            }
        }
        DATA_CMD_RSP_RD_SH_MEM_EP_DATA_BUFFER_V2 => {
            if (*graph).ar_graph.is_null() {
                return 0;
            }
            client_event = APM_CLIENT_EVENT_DATA_READ_DONE;
            mutex_lock(&mut (*graph).lock);
            rd_done = (*data).payload as *mut data_cmd_rsp_rd_sh_mem_ep_data_buffer_done_v2;
            if (*graph).tx_data.buf.is_null() {
                mutex_unlock(&mut (*graph).lock);
                return 0;
            }
            phys = (*(*graph).tx_data.buf.add((*hdr).token as usize)).phys;
            mutex_unlock(&mut (*graph).lock);
            // token numbering starts at 0
            atomic_set(&mut (*graph).tx_data.hw_ptr, (*hdr).token as c_int + 1);

            if upper_32_bits(phys) == (*rd_done).buf_addr_msw
                && lower_32_bits(phys) == (*rd_done).buf_addr_lsw
            {
                (*graph).result.opcode = (*hdr).opcode;
                (*graph).result.status = (*rd_done).status;
                if let Some(cb) = (*graph).cb {
                    cb(client_event, (*hdr).token, (*data).payload, (*graph).priv_);
                }
            } else {
                dev_err(
                    dev,
                    b"RD BUFF Unexpected addr %08x-%08x\n\0".as_ptr() as *const c_char,
                    (*rd_done).buf_addr_lsw,
                    (*rd_done).buf_addr_msw,
                );
            }
        }
        DATA_CMD_WR_SH_MEM_EP_EOS_RENDERED => {
            client_event = APM_CLIENT_EVENT_CMD_EOS_DONE;
            if let Some(cb) = (*graph).cb {
                cb(client_event, (*hdr).token, (*data).payload, (*graph).priv_);
            }
        }
        GPR_BASIC_RSP_RESULT => match (*result).opcode {
            APM_CMD_SHARED_MEM_MAP_REGIONS
            | DATA_CMD_WR_SH_MEM_EP_MEDIA_FORMAT
            | APM_CMD_REGISTER_MODULE_EVENTS
            | APM_CMD_SET_CFG => {
                (*graph).result.opcode = (*result).opcode;
                (*graph).result.status = (*result).status;
                if (*result).status != 0 {
                    dev_err(
                        dev,
                        b"Error (%d) Processing 0x%08x cmd\n\0".as_ptr() as *const c_char,
                        (*result).status,
                        (*result).opcode,
                    );
                }
                wake_up(&mut (*graph).cmd_wait);
            }
            _ => {}
        },
        _ => {}
    }
    0
}

pub unsafe extern "C" fn q6apm_register_watermark_event(
    graph: *mut q6apm_graph,
    water_mark_level_bytes: c_int,
    num_levels: c_int,
) -> c_int {
    audioreach_shmem_register_event(graph, water_mark_level_bytes, num_levels)
}

pub unsafe extern "C" fn q6apm_push_pull_config(
    graph: *mut q6apm_graph,
    bphys: phys_addr_t,
    pphys: phys_addr_t,
    size: u32,
) -> c_int {
    let info = (*graph).info;

    audioreach_setup_push_pull(
        graph,
        bphys,
        pphys,
        (*info).mem_map_handle,
        (*info).pos_buf_mem_map_handle,
        size,
    )
}

pub unsafe extern "C" fn q6apm_is_graph_in_push_pull_mode_from_id(
    dev: *mut device,
    graph_id: c_uint,
    dir: c_int,
) -> bool_t {
    let info: *mut audioreach_graph_info;
    let apm = dev_get_drvdata((*dev).parent) as *mut q6apm;
    let module: *mut audioreach_module;

    info = idr_find(&mut (*apm).graph_info_idr, graph_id) as *mut audioreach_graph_info;
    if info.is_null() {
        return false;
    }

    if dir == SNDRV_PCM_STREAM_PLAYBACK {
        module = __q6apm_find_module_by_mid(apm, info, MODULE_ID_SH_MEM_PULL_MODE);
    } else {
        module = __q6apm_find_module_by_mid(apm, info, MODULE_ID_SH_MEM_PUSH_MODE);
    }

    !module.is_null()
}

pub unsafe extern "C" fn q6apm_is_graph_in_push_pull_mode(graph: *mut q6apm_graph) -> bool_t {
    (*(*graph).info).is_push_pull_mode
}

unsafe fn q6apm_graph_get_module_iid(graph: *mut q6apm_graph, mid: u32) -> c_int {
    let module = q6apm_find_module_by_mid(graph, mid);

    if module.is_null() {
        return -ENODEV;
    }

    (*module).instance_id as c_int
}

pub unsafe extern "C" fn q6apm_graph_open(
    dev: *mut device,
    cb: q6apm_cb,
    priv_: *mut c_void,
    graph_id: c_int,
    dir: c_int,
) -> *mut q6apm_graph {
    let apm = dev_get_drvdata((*dev).parent) as *mut q6apm;
    let ar_graph: *mut audioreach_graph;
    let graph: *mut q6apm_graph;
    let mut ret: c_int;
    let mut iid: c_int = 0;

    ar_graph = q6apm_get_audioreach_graph(apm, graph_id as u32);
    if IS_ERR(ar_graph as *const c_void) {
        dev_err(
            dev,
            b"No graph found with id %d\n\0".as_ptr() as *const c_char,
            graph_id,
        );
        return ERR_CAST(ar_graph as *mut c_void) as *mut q6apm_graph;
    }

    graph = kzalloc_obj::<q6apm_graph>();
    if graph.is_null() {
        ret = -ENOMEM;
        kref_put(&mut (*ar_graph).refcount, Some(q6apm_put_audioreach_graph));
        return ERR_PTR(ret) as *mut q6apm_graph;
    }

    (*graph).apm = apm;
    (*graph).priv_ = priv_;
    (*graph).cb = cb;
    (*graph).info = (*ar_graph).info;
    (*graph).ar_graph = ar_graph;
    (*graph).id = (*ar_graph).id;
    (*graph).dev = dev;

    if dir == SNDRV_PCM_STREAM_PLAYBACK {
        iid = q6apm_graph_get_module_iid(graph, MODULE_ID_SH_MEM_PULL_MODE);
        if iid < 0 {
            iid = q6apm_graph_get_module_iid(graph, MODULE_ID_WR_SHARED_MEM_EP);
        } else {
            (*(*graph).info).is_push_pull_mode = true;
        }
    } else {
        iid = q6apm_graph_get_module_iid(graph, MODULE_ID_SH_MEM_PUSH_MODE);
        if iid < 0 {
            iid = q6apm_graph_get_module_iid(graph, MODULE_ID_RD_SHARED_MEM_EP);
        } else {
            (*(*graph).info).is_push_pull_mode = true;
        }
    }

    if iid > 0 {
        (*graph).shm_iid = iid as u32;
    }

    mutex_init(&mut (*graph).lock);
    init_waitqueue_head(&mut (*graph).cmd_wait);

    (*graph).port = gpr_alloc_port(apm, dev, Some(graph_callback), graph as *mut c_void);
    if IS_ERR((*graph).port as *const c_void) {
        ret = PTR_ERR((*graph).port as *const c_void);
        kfree(graph as *mut c_void);
        kref_put(&mut (*ar_graph).refcount, Some(q6apm_put_audioreach_graph));
        return ERR_PTR(ret) as *mut q6apm_graph;
    }

    graph
}

pub unsafe extern "C" fn q6apm_graph_close(graph: *mut q6apm_graph) -> c_int {
    let ar_graph = (*graph).ar_graph;

    (*graph).ar_graph = ptr::null_mut();
    kref_put(&mut (*ar_graph).refcount, Some(q6apm_put_audioreach_graph));
    gpr_free_port((*graph).port);
    kfree(graph as *mut c_void);

    0
}

pub unsafe extern "C" fn q6apm_graph_prepare(graph: *mut q6apm_graph) -> c_int {
    audioreach_graph_mgmt_cmd((*graph).ar_graph, APM_CMD_GRAPH_PREPARE)
}

pub unsafe extern "C" fn q6apm_graph_start(graph: *mut q6apm_graph) -> c_int {
    let ar_graph = (*graph).ar_graph;
    let ret: c_int;

    if (*ar_graph).start_count == 0 {
        ret = audioreach_graph_mgmt_cmd(ar_graph, APM_CMD_GRAPH_START);
        if ret != 0 {
            return ret;
        }
    }

    (*ar_graph).start_count = (*ar_graph).start_count.wrapping_add(1);

    0
}

pub unsafe extern "C" fn q6apm_graph_stop(graph: *mut q6apm_graph) -> c_int {
    let ar_graph = (*graph).ar_graph;

    if (*ar_graph).start_count == 0 {
        return 0;
    }

    (*ar_graph).start_count = (*ar_graph).start_count.wrapping_sub(1);
    if (*ar_graph).start_count > 0 {
        return 0;
    }

    audioreach_graph_mgmt_cmd(ar_graph, APM_CMD_GRAPH_STOP)
}

pub unsafe extern "C" fn q6apm_graph_flush(graph: *mut q6apm_graph) -> c_int {
    audioreach_graph_mgmt_cmd((*graph).ar_graph, APM_CMD_GRAPH_FLUSH)
}

unsafe extern "C" fn q6apm_audio_probe(component: *mut snd_soc_component) -> c_int {
    audioreach_tplg_init(component)
}

unsafe extern "C" fn q6apm_audio_remove(component: *mut snd_soc_component) {
    // remove topology
    snd_soc_tplg_component_remove(component);
}

const APM_AUDIO_DRV_NAME: &[u8] = b"q6apm-audio\0";

static q6apm_audio_component: snd_soc_component_driver = snd_soc_component_driver {
    name: APM_AUDIO_DRV_NAME.as_ptr() as *const c_char,
    probe: Some(q6apm_audio_probe),
    remove: Some(q6apm_audio_remove),
    remove_order: SND_SOC_COMP_ORDER_LAST,
};

unsafe extern "C" fn apm_probe(gdev: *mut gpr_device_t) -> c_int {
    let dev = &mut (*gdev).dev as *mut device;
    let apm: *mut q6apm;
    let mut ret: c_int;

    apm = devm_kzalloc(dev, size_of::<q6apm>(), GFP_KERNEL) as *mut q6apm;
    if apm.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, apm as *mut c_void);

    mutex_init(&mut (*apm).lock);
    (*apm).dev = dev;
    (*apm).gdev = gdev;
    init_waitqueue_head(&mut (*apm).wait);

    INIT_LIST_HEAD(&mut (*apm).widget_list);
    idr_init(&mut (*apm).graph_idr);
    idr_init(&mut (*apm).graph_info_idr);
    idr_init(&mut (*apm).sub_graphs_idr);
    idr_init(&mut (*apm).containers_idr);

    idr_init(&mut (*apm).modules_idr);

    g_apm = apm;

    q6apm_get_apm_state(apm);

    ret = snd_soc_register_component(dev, &q6apm_audio_component, ptr::null(), 0);
    if ret < 0 {
        dev_err(
            dev,
            b"failed to register q6apm: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = of_platform_populate((*dev).of_node, ptr::null(), ptr::null(), dev);
    if ret != 0 {
        snd_soc_unregister_component(dev);
    }

    ret
}

unsafe extern "C" fn apm_remove(gdev: *mut gpr_device_t) {
    of_platform_depopulate(&mut (*gdev).dev);
    snd_soc_unregister_component(&mut (*gdev).dev);
}

pub unsafe extern "C" fn q6apm_find_module_by_mid(
    graph: *mut q6apm_graph,
    mid: u32,
) -> *mut audioreach_module {
    let info = (*graph).info;
    let apm = (*graph).apm;

    __q6apm_find_module_by_mid(apm, info, mid)
}

unsafe extern "C" fn apm_callback(
    data: *const gpr_resp_pkt,
    priv_: *mut c_void,
    _op: c_int,
) -> c_int {
    let gdev = priv_ as *mut gpr_device_t;
    let mut info: *mut audioreach_graph_info;
    let mut rsp: *mut apm_cmd_rsp_shared_mem_map_regions;
    let apm = dev_get_drvdata(&mut (*gdev).dev) as *mut q6apm;
    let dev = &mut (*gdev).dev as *mut device;
    let result: *mut gpr_ibasic_rsp_result_t;
    let hdr = &(*data).hdr as *const gpr_hdr;
    let graph_id: c_int;
    let is_pos_buf: c_int;

    result = (*data).payload as *mut gpr_ibasic_rsp_result_t;

    match (*hdr).opcode {
        APM_CMD_RSP_GET_SPF_STATE => {
            (*apm).result.opcode = (*hdr).opcode;
            (*apm).result.status = 0;
            // First word of result it state
            (*apm).state = (*result).opcode as c_int;
            wake_up(&mut (*apm).wait);
        }
        GPR_BASIC_RSP_RESULT => match (*result).opcode {
            APM_CMD_SHARED_MEM_MAP_REGIONS
            | APM_CMD_GRAPH_START
            | APM_CMD_GRAPH_OPEN
            | APM_CMD_GRAPH_PREPARE
            | APM_CMD_GRAPH_CLOSE
            | APM_CMD_GRAPH_FLUSH
            | APM_CMD_GRAPH_STOP
            | APM_CMD_SET_CFG => {
                (*apm).result.opcode = (*result).opcode;
                (*apm).result.status = (*result).status;
                if (*result).status != 0 {
                    dev_err(
                        dev,
                        b"Error (%d) Processing 0x%08x cmd\n\0".as_ptr() as *const c_char,
                        (*result).status,
                        (*result).opcode,
                    );
                }
                wake_up(&mut (*apm).wait);
            }
            APM_CMD_SHARED_MEM_UNMAP_REGIONS => {
                (*apm).result.opcode = (*hdr).opcode;
                (*apm).result.status = 0;
                rsp = (*data).payload as *mut apm_cmd_rsp_shared_mem_map_regions;

                info = idr_find(&mut (*apm).graph_info_idr, (*hdr).token)
                    as *mut audioreach_graph_info;
                if !info.is_null() {
                    (*info).mem_map_handle = 0;
                } else {
                    dev_err(
                        dev,
                        b"Error (%d) Processing 0x%08x cmd\n\0".as_ptr() as *const c_char,
                        (*result).status,
                        (*result).opcode,
                    );
                }

                let _ = rsp;
                wake_up(&mut (*apm).wait);
            }
            _ => {}
        },
        APM_CMD_RSP_SHARED_MEM_MAP_REGIONS => {
            (*apm).result.opcode = (*hdr).opcode;
            (*apm).result.status = 0;
            rsp = (*data).payload as *mut apm_cmd_rsp_shared_mem_map_regions;
            graph_id = ((*hdr).token & APM_MMAP_TOKEN_GID_MASK) as c_int;
            is_pos_buf = ((*hdr).token & APM_MMAP_TOKEN_MAP_TYPE_POS_BUF) as c_int;

            info = idr_find(&mut (*apm).graph_info_idr, graph_id as u32)
                as *mut audioreach_graph_info;
            if !info.is_null() {
                if is_pos_buf != 0 {
                    (*info).pos_buf_mem_map_handle = (*rsp).mem_map_handle;
                } else {
                    (*info).mem_map_handle = (*rsp).mem_map_handle;
                }
            } else {
                dev_err(
                    dev,
                    b"Error (%d) Processing 0x%08x cmd\n\0".as_ptr() as *const c_char,
                    (*result).status,
                    (*result).opcode,
                );
            }

            wake_up(&mut (*apm).wait);
        }
        _ => {}
    }

    0
}

// CONFIG_OF:
static apm_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: b"qcom,q6apm\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

static mut apm_driver: gpr_driver_t = gpr_driver_t {
    probe: Some(apm_probe),
    remove: Some(apm_remove),
    gpr_callback: Some(apm_callback),
    driver: device_driver {
        name: b"qcom-apm\0".as_ptr() as *const c_char,
        of_match_table: apm_device_id.as_ptr(),
    },
};

// module_gpr_driver(apm_driver);
// MODULE_DESCRIPTION("Audio Process Manager");
// MODULE_LICENSE("GPL");

extern "C" {
    static APM_MODULE_INSTANCE_ID: u32;
    static APM_PARAM_ID_SUB_GRAPH_LIST: u32;
    static APM_MODULE_PARAM_DATA_SIZE: usize;
    static GPR_HDR_SIZE: usize;
    static APM_CMD_HDR_SIZE: usize;
    static APM_CMD_GRAPH_CLOSE: u32;
    static APM_CMD_GET_SPF_STATE: u32;
    static APM_CMD_RSP_GET_SPF_STATE: u32;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static MODULE_ID_SH_MEM_PUSH_MODE: u32;
    static MODULE_ID_RD_SHARED_MEM_EP: u32;
    static MODULE_ID_SH_MEM_PULL_MODE: u32;
    static MODULE_ID_WR_SHARED_MEM_EP: u32;
    static APM_MMAP_TOKEN_MAP_TYPE_POS_BUF: u32;
    static APM_CMD_SHARED_MEM_MAP_REGIONS: u32;
    static APM_CMD_RSP_SHARED_MEM_MAP_REGIONS: u32;
    static APM_MEMORY_MAP_SHMEM8_4K_POOL: u32;
    static APM_CMD_SHARED_MEM_UNMAP_REGIONS: u32;
    static MODULE_ID_PLACEHOLDER_DECODER: u32;
    static PARAM_ID_REMOVE_INITIAL_SILENCE: u32;
    static PARAM_ID_REMOVE_TRAILING_SILENCE: u32;
    static PARAM_ID_MODULE_ENABLE: u32;
    static SND_AUDIOCODEC_MP3: u32;
    static SND_AUDIOCODEC_AAC: u32;
    static SND_AUDIOCODEC_FLAC: u32;
    static SND_AUDIOCODEC_OPUS_RAW: u32;
    static MODULE_ID_MP3_DECODE: u32;
    static MODULE_ID_AAC_DEC: u32;
    static MODULE_ID_FLAC_DEC: u32;
    static MODULE_ID_OPUS_DEC: u32;
    static PARAM_ID_REAL_MODULE_ID: u32;
    static DATA_CMD_WR_SH_MEM_EP_DATA_BUFFER_V2: u32;
    static APM_WRITE_TOKEN_LEN_SHIFT: u32;
    static DATA_CMD_RD_SH_MEM_EP_DATA_BUFFER_V2: u32;
    static APM_EVENT_MODULE_TO_CLIENT: u32;
    static EVENT_ID_SH_MEM_PULL_PUSH_MODE_WATERMARK: u32;
    static APM_CLIENT_EVENT_WATERMARK_EVENT: u32;
    static DATA_CMD_RSP_WR_SH_MEM_EP_DATA_BUFFER_DONE_V2: u32;
    static APM_CLIENT_EVENT_DATA_WRITE_DONE: u32;
    static APM_WRITE_TOKEN_MASK: u32;
    static DATA_CMD_RSP_RD_SH_MEM_EP_DATA_BUFFER_V2: u32;
    static APM_CLIENT_EVENT_DATA_READ_DONE: u32;
    static DATA_CMD_WR_SH_MEM_EP_EOS_RENDERED: u32;
    static APM_CLIENT_EVENT_CMD_EOS_DONE: u32;
    static GPR_BASIC_RSP_RESULT: u32;
    static DATA_CMD_WR_SH_MEM_EP_MEDIA_FORMAT: u32;
    static APM_CMD_REGISTER_MODULE_EVENTS: u32;
    static APM_CMD_SET_CFG: u32;
    static APM_CMD_GRAPH_PREPARE: u32;
    static APM_CMD_GRAPH_START: u32;
    static APM_CMD_GRAPH_OPEN: u32;
    static APM_CMD_GRAPH_FLUSH: u32;
    static APM_CMD_GRAPH_STOP: u32;
    static APM_MMAP_TOKEN_GID_MASK: u32;
    static SND_SOC_COMP_ORDER_LAST: c_int;

    fn audioreach_send_cmd_sync(
        dev: *mut device,
        gdev: *mut gpr_device_t,
        result: *mut q6apm_result,
        lock: *mut mutex,
        unused: *mut c_void,
        wait: *mut wait_queue_head_t,
        pkt: *const gpr_pkt,
        rsp_opcode: u32,
    ) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn idr_find(idr: *mut idr, id: u32) -> *mut c_void;
    fn idr_alloc(idr: *mut idr, ptr: *mut c_void, start: c_int, end: c_int, gfp: c_uint) -> c_int;
    fn idr_remove(idr: *mut idr, id: u32) -> *mut c_void;
    fn idr_init(idr: *mut idr);
    fn kref_get(kref: *mut kref);
    fn kref_init(kref: *mut kref);
    fn kref_put(kref: *mut kref, release: Option<unsafe extern "C" fn(*mut kref)>) -> c_int;
    fn audioreach_alloc_graph_pkt(apm: *mut q6apm, info: *mut audioreach_graph_info) -> *mut gpr_pkt;
    fn audioreach_alloc_apm_cmd_pkt(payload_size: usize, opcode: u32, token: u32) -> *mut gpr_pkt;
    fn audioreach_alloc_pkt(
        payload_size: usize,
        opcode: u32,
        token: u32,
        dst_port: u32,
        dst_domain: u32,
    ) -> *mut gpr_pkt;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn ERR_PTR(err: c_int) -> *mut c_void;
    fn ERR_CAST(ptr: *mut c_void) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn audioreach_set_media_format(
        graph: *mut q6apm_graph,
        module: *mut audioreach_module,
        cfg: *mut audioreach_module_config,
    ) -> c_int;
    fn audioreach_graph_free_buf(graph: *mut q6apm_graph);
    fn audioreach_send_u32_param(
        graph: *mut q6apm_graph,
        module: *mut audioreach_module,
        param_id: u32,
        value: u32,
    ) -> c_int;
    fn gpr_send_port_pkt(port: *mut gpr_port, pkt: *mut gpr_pkt) -> c_int;
    fn atomic_read(v: *mut atomic_t) -> c_int;
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn wake_up(wait: *mut wait_queue_head_t);
    fn audioreach_shmem_register_event(
        graph: *mut q6apm_graph,
        water_mark_level_bytes: c_int,
        num_levels: c_int,
    ) -> c_int;
    fn audioreach_setup_push_pull(
        graph: *mut q6apm_graph,
        bphys: phys_addr_t,
        pphys: phys_addr_t,
        mem_map_handle: u32,
        pos_buf_mem_map_handle: u32,
        size: u32,
    ) -> c_int;
    fn init_waitqueue_head(wait: *mut wait_queue_head_t);
    fn gpr_alloc_port(
        gdev: *mut q6apm,
        dev: *mut device,
        cb: Option<unsafe extern "C" fn(*const gpr_resp_pkt, *mut c_void, c_int) -> c_int>,
        priv_: *mut c_void,
    ) -> *mut gpr_port;
    fn gpr_free_port(port: *mut gpr_port);
    fn audioreach_tplg_init(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_tplg_component_remove(component: *mut snd_soc_component);
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn snd_soc_register_component(
        dev: *mut device,
        driver: *const snd_soc_component_driver,
        dai_drv: *const c_void,
        num_dai: c_int,
    ) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn of_platform_populate(
        root: *mut device_node,
        matches: *const c_void,
        lookup: *const c_void,
        parent: *mut device,
    ) -> c_int;
    fn of_platform_depopulate(parent: *mut device);

    fn kzalloc_obj_q6apm_graph() -> *mut q6apm_graph;
    fn kzalloc_obj_audioreach_graph() -> *mut audioreach_graph;
    fn kzalloc_array_audio_buffer(n: usize) -> *mut audio_buffer;
    fn list_first_entry_or_null(head: *mut list_head, node_offset: usize) -> *mut c_void;
    fn list_next_entry_or_null(entry: *mut c_void, head: *mut list_head, node_offset: usize) -> *mut c_void;
    fn audioreach_sub_graph_node_offset() -> usize;
    fn audioreach_container_node_offset() -> usize;
    fn audioreach_module_node_offset() -> usize;
    fn container_of_audioreach_graph_refcount(ref_: *mut kref) -> *mut audioreach_graph;
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    if size_of::<T>() == size_of::<q6apm_graph>() {
        return kzalloc_obj_q6apm_graph() as *mut T;
    }
    kzalloc_obj_audioreach_graph() as *mut T
}

unsafe fn kzalloc_array<T>(n: usize) -> *mut T {
    let _ = size_of::<T>();
    kzalloc_array_audio_buffer(n) as *mut T
}

#[repr(C)]
pub struct q6apm {
    pub gdev: *mut gpr_device_t,
    pub dev: *mut device,
    pub result: q6apm_result,
    pub lock: mutex,
    pub wait: wait_queue_head_t,
    pub state: c_int,
    pub widget_list: list_head,
    pub graph_idr: idr,
    pub graph_info_idr: idr,
    pub sub_graphs_idr: idr,
    pub containers_idr: idr,
    pub modules_idr: idr,
}

#[repr(C)]
pub struct q6apm_graph {
    pub apm: *mut q6apm,
    pub priv_: *mut c_void,
    pub cb: q6apm_cb,
    pub info: *mut audioreach_graph_info,
    pub ar_graph: *mut audioreach_graph,
    pub id: u32,
    pub dev: *mut device,
    pub shm_iid: u32,
    pub lock: mutex,
    pub cmd_wait: wait_queue_head_t,
    pub port: *mut gpr_port,
    pub rx_data: audioreach_graph_data,
    pub tx_data: audioreach_graph_data,
    pub result: q6apm_result,
}

#[repr(C)]
pub struct audioreach_graph {
    pub apm: *mut q6apm,
    pub info: *mut audioreach_graph_info,
    pub id: u32,
    pub graph: *mut gpr_pkt,
    pub refcount: kref,
    pub start_count: u32,
}

#[repr(C)]
pub struct audioreach_graph_info {
    pub num_sub_graphs: c_int,
    pub sg_list: list_head,
    pub mem_map_handle: u32,
    pub pos_buf_mem_map_handle: u32,
    pub is_push_pull_mode: bool_t,
}

#[repr(C)]
pub struct audioreach_sub_graph {
    pub node: list_head,
    pub container_list: list_head,
    pub sub_graph_id: u32,
}

#[repr(C)]
pub struct audioreach_container {
    pub node: list_head,
    pub modules_list: list_head,
}

#[repr(C)]
pub struct audioreach_module {
    pub node: list_head,
    pub module_id: u32,
    pub instance_id: u32,
}

#[repr(C)]
pub struct audioreach_graph_data {
    pub dsp_buf: u32,
    pub buf: *mut audio_buffer,
    pub num_periods: u32,
    pub hw_ptr: atomic_t,
}

#[repr(C)]
pub struct audio_buffer {
    pub phys: phys_addr_t,
    pub size: size_t,
}

#[repr(C)]
pub struct gpr_device_t {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct gpr_hdr {
    pub opcode: u32,
    pub token: u32,
}

#[repr(C)]
pub struct gpr_pkt {
    pub hdr: gpr_hdr,
}

#[repr(C)]
pub struct gpr_resp_pkt {
    pub hdr: gpr_hdr,
    pub payload: *mut c_void,
}

#[repr(C)]
pub struct gpr_ibasic_rsp_result_t {
    pub opcode: u32,
    pub status: c_int,
}

#[repr(C)]
pub struct q6apm_result {
    pub opcode: u32,
    pub status: c_int,
}

#[repr(C)]
pub struct apm_module_param_data {
    pub module_instance_id: u32,
    pub param_id: u32,
    pub param_size: u32,
}

#[repr(C)]
pub struct apm_shared_map_region_payload {
    pub shm_addr_lsw: u32,
    pub shm_addr_msw: u32,
    pub mem_size_bytes: u32,
}

#[repr(C)]
pub struct apm_cmd_shared_mem_map_regions {
    pub mem_pool_id: u32,
    pub num_regions: u32,
    pub property_flag: u32,
}

#[repr(C)]
pub struct apm_cmd_shared_mem_unmap_regions {
    pub mem_map_handle: u32,
}

#[repr(C)]
pub struct apm_cmd_rsp_shared_mem_map_regions {
    pub mem_map_handle: u32,
}

#[repr(C)]
pub struct apm_data_cmd_wr_sh_mem_ep_data_buffer_v2 {
    pub buf_addr_lsw: u32,
    pub buf_addr_msw: u32,
    pub buf_size: u32,
    pub timestamp_lsw: u32,
    pub timestamp_msw: u32,
    pub mem_map_handle: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct data_cmd_rd_sh_mem_ep_data_buffer_v2 {
    pub buf_addr_lsw: u32,
    pub buf_addr_msw: u32,
    pub mem_map_handle: u32,
    pub buf_size: u32,
}

#[repr(C)]
pub struct data_cmd_rsp_rd_sh_mem_ep_data_buffer_done_v2 {
    pub buf_addr_lsw: u32,
    pub buf_addr_msw: u32,
    pub status: c_int,
}

#[repr(C)]
pub struct data_cmd_rsp_wr_sh_mem_ep_data_buffer_done_v2 {
    pub buf_addr_lsw: u32,
    pub buf_addr_msw: u32,
    pub status: c_int,
}

#[repr(C)]
pub struct apm_module_event {
    pub event_id: u32,
}

#[repr(C)]
pub struct audioreach_module_config {
    pub direction: c_int,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub remove_order: c_int,
}

#[repr(C)]
pub struct gpr_driver_t {
    pub probe: Option<unsafe extern "C" fn(*mut gpr_device_t) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut gpr_device_t)>,
    pub gpr_callback: Option<unsafe extern "C" fn(*const gpr_resp_pkt, *mut c_void, c_int) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct gpr_port {
    pub id: u32,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct idr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kref {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
