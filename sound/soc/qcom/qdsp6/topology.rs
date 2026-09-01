// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020, Linaro Limited

// Translated from C implementation source. External kernel/ALSA/QDSP6 types,
// constants, helpers, and list/IDR primitives are expected from surrounding
// bindings corresponding to the original includes:
// linux/cleanup.h, sound/soc.h, sound/soc-dapm.h, sound/pcm.h,
// sound/control.h, sound/asound.h, linux/firmware.h, sound/soc-topology.h,
// sound/soc-dpcm.h, uapi/sound/snd_ar_tokens.h, linux/kernel.h,
// linux/wait.h, q6apm.h, audioreach.h.

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

type uint32_t = u32;

extern "C" {
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn idr_find(idr: *mut c_void, id: u32) -> *mut c_void;
    fn idr_alloc_u32(idr: *mut c_void, ptr: *mut c_void, nextid: *mut u32, max: u32, gfp: c_int) -> c_int;
    fn idr_alloc_cyclic(idr: *mut c_void, ptr: *mut c_void, start: c_int, end: c_int, gfp: c_int) -> c_int;
    fn idr_remove(idr: *mut c_void, id: u32);
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn kasprintf(flags: c_int, fmt: *const c_char, ...) -> *mut c_char;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn release_firmware(fw: *const firmware);
    fn snd_soc_tplg_component_load(component: *mut snd_soc_component, ops: *const snd_soc_tplg_ops, fw: *const firmware) -> c_int;
    fn snd_soc_tplg_widget_bind_event(
        w: *mut snd_soc_dapm_widget,
        events: *const snd_soc_tplg_widget_events,
        count: usize,
        event_type: u16,
    ) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_kcontrol_to_widget(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_widget;
    fn snd_soc_dapm_mixer_update_power(
        dapm: *mut snd_soc_dapm_context,
        kcontrol: *mut snd_kcontrol,
        connect: c_int,
        update: *mut c_void,
    );
    fn snd_soc_info_volsw(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn of_get_compatible_child(node: *mut device_node, compat: *const c_char) -> *mut device_node;
    fn audioreach_gain_set_vol_ctrl(apm: *mut q6apm, mod_: *mut audioreach_module, gain: c_int) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

#[repr(C)]
pub struct snd_ar_control {
    pub graph_id: u32,            /* Graph ID */
    pub sgid: u32,                /* Sub Graph ID */
    pub module_instance_id: u32,  /* Connected Module Instance ID */
    pub w: *mut snd_soc_dapm_widget,
    pub node: list_head,
    pub scomp: *mut snd_soc_component,
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(core::mem::size_of::<T>(), GFP_KERNEL) as *mut T
}

unsafe fn kzalloc_flex<T>(base_size: usize, data_size: usize) -> *mut T {
    kzalloc(base_size + data_size, GFP_KERNEL) as *mut T
}

unsafe fn le32_to_cpu(v: u32) -> u32 {
    u32::from_le(v)
}

unsafe fn le16_to_cpu(v: u16) -> u16 {
    u16::from_le(v)
}

unsafe fn err_ptr<T>(err: c_int) -> *mut T {
    err as isize as *mut T
}

unsafe fn ptr_err<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}

unsafe fn is_err<T>(ptr: *const T) -> bool {
    (ptr as usize) >= (-4095isize as usize)
}

unsafe fn is_err_or_null<T>(ptr: *const T) -> bool {
    ptr.is_null() || is_err(ptr)
}

unsafe fn err_cast<T, U>(ptr: *mut T) -> *mut U {
    ptr as *mut U
}

unsafe fn audioreach_tplg_alloc_graph_info(
    apm: *mut q6apm,
    mut graph_id: uint32_t,
    found: *mut bool,
) -> *mut audioreach_graph_info {
    let mut info: *mut audioreach_graph_info;
    let ret: c_int;

    mutex_lock(&mut (*apm).lock as *mut _ as *mut c_void);
    info = idr_find(&mut (*apm).graph_info_idr as *mut _ as *mut c_void, graph_id) as *mut audioreach_graph_info;
    mutex_unlock(&mut (*apm).lock as *mut _ as *mut c_void);

    if !info.is_null() {
        *found = true;
        return info;
    }

    *found = false;
    info = kzalloc_obj::<audioreach_graph_info>();
    if info.is_null() {
        return err_ptr(-ENOMEM);
    }

    INIT_LIST_HEAD(&mut (*info).sg_list);

    mutex_lock(&mut (*apm).lock as *mut _ as *mut c_void);
    ret = idr_alloc_u32(
        &mut (*apm).graph_info_idr as *mut _ as *mut c_void,
        info as *mut c_void,
        &mut graph_id,
        graph_id,
        GFP_KERNEL,
    );
    mutex_unlock(&mut (*apm).lock as *mut _ as *mut c_void);

    if ret < 0 {
        dev_err((*apm).dev, b"Failed to allocate Graph ID (%x)\n\0".as_ptr() as *const c_char, graph_id);
        kfree(info as *mut c_void);
        return err_ptr(ret);
    }

    (*info).id = graph_id;

    info
}

unsafe fn audioreach_tplg_add_sub_graph(sg: *mut audioreach_sub_graph, info: *mut audioreach_graph_info) {
    list_add_tail(&mut (*sg).node, &mut (*info).sg_list);
    (*sg).info = info;
    (*info).num_sub_graphs += 1;
}

unsafe fn audioreach_tplg_alloc_sub_graph(
    apm: *mut q6apm,
    mut sub_graph_id: uint32_t,
    found: *mut bool,
) -> *mut audioreach_sub_graph {
    let mut sg: *mut audioreach_sub_graph;
    let ret: c_int;

    if sub_graph_id == 0 {
        return err_ptr(-EINVAL);
    }

    /* Find if there is already a matching sub-graph */
    mutex_lock(&mut (*apm).lock as *mut _ as *mut c_void);
    sg = idr_find(&mut (*apm).sub_graphs_idr as *mut _ as *mut c_void, sub_graph_id) as *mut audioreach_sub_graph;
    mutex_unlock(&mut (*apm).lock as *mut _ as *mut c_void);

    if !sg.is_null() {
        *found = true;
        return sg;
    }

    *found = false;
    sg = kzalloc_obj::<audioreach_sub_graph>();
    if sg.is_null() {
        return err_ptr(-ENOMEM);
    }

    INIT_LIST_HEAD(&mut (*sg).container_list);

    mutex_lock(&mut (*apm).lock as *mut _ as *mut c_void);
    ret = idr_alloc_u32(
        &mut (*apm).sub_graphs_idr as *mut _ as *mut c_void,
        sg as *mut c_void,
        &mut sub_graph_id,
        sub_graph_id,
        GFP_KERNEL,
    );
    mutex_unlock(&mut (*apm).lock as *mut _ as *mut c_void);

    if ret < 0 {
        dev_err((*apm).dev, b"Failed to allocate Sub-Graph Instance ID (%x)\n\0".as_ptr() as *const c_char, sub_graph_id);
        kfree(sg as *mut c_void);
        return err_ptr(ret);
    }

    (*sg).sub_graph_id = sub_graph_id;

    sg
}

unsafe fn audioreach_tplg_alloc_container(
    apm: *mut q6apm,
    sg: *mut audioreach_sub_graph,
    mut container_id: uint32_t,
    found: *mut bool,
) -> *mut audioreach_container {
    let mut cont: *mut audioreach_container;
    let ret: c_int;

    if container_id == 0 {
        return err_ptr(-EINVAL);
    }

    mutex_lock(&mut (*apm).lock as *mut _ as *mut c_void);
    cont = idr_find(&mut (*apm).containers_idr as *mut _ as *mut c_void, container_id) as *mut audioreach_container;
    mutex_unlock(&mut (*apm).lock as *mut _ as *mut c_void);

    if !cont.is_null() {
        *found = true;
        return cont;
    }
    *found = false;

    cont = kzalloc_obj::<audioreach_container>();
    if cont.is_null() {
        return err_ptr(-ENOMEM);
    }

    INIT_LIST_HEAD(&mut (*cont).modules_list);

    mutex_lock(&mut (*apm).lock as *mut _ as *mut c_void);
    ret = idr_alloc_u32(
        &mut (*apm).containers_idr as *mut _ as *mut c_void,
        cont as *mut c_void,
        &mut container_id,
        container_id,
        GFP_KERNEL,
    );
    mutex_unlock(&mut (*apm).lock as *mut _ as *mut c_void);

    if ret < 0 {
        dev_err((*apm).dev, b"Failed to allocate Container Instance ID (%x)\n\0".as_ptr() as *const c_char, container_id);
        kfree(cont as *mut c_void);
        return err_ptr(ret);
    }

    (*cont).container_id = container_id;
    (*cont).sub_graph = sg;
    /* add to container list */
    list_add_tail(&mut (*cont).node, &mut (*sg).container_list);
    (*sg).num_containers += 1;

    cont
}

unsafe fn audioreach_tplg_alloc_module(
    apm: *mut q6apm,
    cont: *mut audioreach_container,
    w: *mut snd_soc_dapm_widget,
    mut module_id: uint32_t,
    found: *mut bool,
) -> *mut audioreach_module {
    let mut mod_: *mut audioreach_module;
    let ret: c_int;

    mutex_lock(&mut (*apm).lock as *mut _ as *mut c_void);
    mod_ = idr_find(&mut (*apm).modules_idr as *mut _ as *mut c_void, module_id) as *mut audioreach_module;
    mutex_unlock(&mut (*apm).lock as *mut _ as *mut c_void);

    if !mod_.is_null() {
        *found = true;
        return mod_;
    }
    *found = false;
    mod_ = kzalloc_obj::<audioreach_module>();
    if mod_.is_null() {
        return err_ptr(-ENOMEM);
    }

    mutex_lock(&mut (*apm).lock as *mut _ as *mut c_void);
    if module_id == 0 {
        /* alloc module id dynamically */
        ret = idr_alloc_cyclic(
            &mut (*apm).modules_idr as *mut _ as *mut c_void,
            mod_ as *mut c_void,
            AR_MODULE_DYNAMIC_INSTANCE_ID_START,
            AR_MODULE_DYNAMIC_INSTANCE_ID_END,
            GFP_KERNEL,
        );
    } else {
        ret = idr_alloc_u32(
            &mut (*apm).modules_idr as *mut _ as *mut c_void,
            mod_ as *mut c_void,
            &mut module_id,
            module_id,
            GFP_KERNEL,
        );
    }
    mutex_unlock(&mut (*apm).lock as *mut _ as *mut c_void);

    if ret < 0 {
        dev_err((*apm).dev, b"Failed to allocate Module Instance ID (%x)\n\0".as_ptr() as *const c_char, module_id);
        kfree(mod_ as *mut c_void);
        return err_ptr(ret);
    }

    (*mod_).instance_id = module_id;
    /* add to module list */
    list_add_tail(&mut (*mod_).node, &mut (*cont).modules_list);
    (*mod_).container = cont;
    (*mod_).widget = w;
    (*cont).num_modules += 1;

    mod_
}

unsafe fn audioreach_get_array_by_token(
    private: *const snd_soc_tplg_private,
    wanted: u32,
) -> *const snd_soc_tplg_vendor_array {
    let mut array: *const snd_soc_tplg_vendor_array = ptr::null();
    let mut found = false;
    let mut sz: c_int = 0;

    while !found && sz < le32_to_cpu((*private).size) as c_int {
        let mut elem: *const snd_soc_tplg_vendor_value_elem;
        let mut tkn_count: c_int = 0;

        array = ((*private).array as *const u8).offset(sz as isize) as *const snd_soc_tplg_vendor_array;
        elem = (*array).value.as_ptr();
        sz += le32_to_cpu((*array).size) as c_int;
        while !found && tkn_count <= le32_to_cpu((*array).num_elems) as c_int - 1 {
            match le32_to_cpu((*elem).token) {
                token if token == wanted => found = true,
                _ => {}
            }
            tkn_count += 1;
            elem = elem.add(1);
        }
    }

    if found {
        array
    } else {
        ptr::null()
    }
}

unsafe fn audioreach_get_sg_array(private: *const snd_soc_tplg_private) -> *const snd_soc_tplg_vendor_array {
    audioreach_get_array_by_token(private, AR_TKN_U32_SUB_GRAPH_INSTANCE_ID)
}

unsafe fn audioreach_get_cont_array(private: *const snd_soc_tplg_private) -> *const snd_soc_tplg_vendor_array {
    audioreach_get_array_by_token(private, AR_TKN_U32_CONTAINER_INSTANCE_ID)
}

unsafe fn audioreach_get_module_array(private: *const snd_soc_tplg_private) -> *const snd_soc_tplg_vendor_array {
    audioreach_get_array_by_token(private, AR_TKN_U32_MODULE_INSTANCE_ID)
}

unsafe fn audioreach_get_module_priv_data(private: *const snd_soc_tplg_private) -> *mut audioreach_module_priv_data {
    let mut sz: c_int = 0;

    while sz < le32_to_cpu((*private).size) as c_int {
        let mod_array = ((*private).array as *const u8).offset(sz as isize) as *const snd_soc_tplg_vendor_array;

        if le32_to_cpu((*mod_array).type_) == SND_SOC_AR_TPLG_MODULE_CFG_TYPE {
            let size = le32_to_cpu((*mod_array).size) as usize;
            let pdata = kzalloc_flex::<audioreach_module_priv_data>(
                core::mem::size_of::<audioreach_module_priv_data>(),
                size,
            );
            if pdata.is_null() {
                return err_ptr(-ENOMEM);
            }

            memcpy(
                pdata as *mut c_void,
                ((*private).data as *const u8).offset(sz as isize) as *const c_void,
                core::mem::size_of::<audioreach_module_priv_data>() + size,
            );
            return pdata;
        }

        sz += le32_to_cpu((*mod_array).size) as c_int;
    }

    ptr::null_mut()
}

unsafe fn audioreach_parse_sg_tokens(
    apm: *mut q6apm,
    private: *const snd_soc_tplg_private,
) -> *mut audioreach_sub_graph {
    let mut sg_elem: *const snd_soc_tplg_vendor_value_elem;
    let sg_array: *const snd_soc_tplg_vendor_array;
    let mut info: *mut audioreach_graph_info = ptr::null_mut();
    let mut graph_id: c_int;
    let mut sub_graph_id: c_int;
    let mut tkn_count: c_int = 0;
    let mut sg: *mut audioreach_sub_graph = ptr::null_mut();
    let mut found = false;

    sg_array = audioreach_get_sg_array(private);
    sg_elem = (*sg_array).value.as_ptr();

    while tkn_count <= le32_to_cpu((*sg_array).num_elems) as c_int - 1 {
        match le32_to_cpu((*sg_elem).token) {
            AR_TKN_U32_SUB_GRAPH_INSTANCE_ID => {
                sub_graph_id = le32_to_cpu((*sg_elem).value) as c_int;
                sg = audioreach_tplg_alloc_sub_graph(apm, sub_graph_id as u32, &mut found);
                if is_err(sg) {
                    return sg;
                } else if found {
                    /* Already parsed data for this sub-graph */
                    return sg;
                }
            }
            AR_TKN_DAI_INDEX => {
                /* Sub graph is associated with predefined graph */
                graph_id = le32_to_cpu((*sg_elem).value) as c_int;
                info = audioreach_tplg_alloc_graph_info(apm, graph_id as u32, &mut found);
                if is_err(info) {
                    return err_cast(info);
                }
            }
            AR_TKN_U32_SUB_GRAPH_PERF_MODE => (*sg).perf_mode = le32_to_cpu((*sg_elem).value),
            AR_TKN_U32_SUB_GRAPH_DIRECTION => (*sg).direction = le32_to_cpu((*sg_elem).value),
            AR_TKN_U32_SUB_GRAPH_SCENARIO_ID => (*sg).scenario_id = le32_to_cpu((*sg_elem).value),
            _ => {
                dev_err((*apm).dev, b"Not a valid token %d for graph\n\0".as_ptr() as *const c_char, (*sg_elem).token);
            }
        }
        tkn_count += 1;
        sg_elem = sg_elem.add(1);
    }

    /* Sub graph is associated with predefined graph */
    if !info.is_null() {
        audioreach_tplg_add_sub_graph(sg, info);
    }

    sg
}

unsafe fn audioreach_parse_cont_tokens(
    apm: *mut q6apm,
    sg: *mut audioreach_sub_graph,
    private: *const snd_soc_tplg_private,
) -> *mut audioreach_container {
    let mut cont_elem: *const snd_soc_tplg_vendor_value_elem;
    let cont_array: *const snd_soc_tplg_vendor_array;
    let mut cont: *mut audioreach_container = ptr::null_mut();
    let mut container_id: c_int;
    let mut tkn_count: c_int = 0;
    let mut found = false;

    cont_array = audioreach_get_cont_array(private);
    cont_elem = (*cont_array).value.as_ptr();

    while tkn_count <= le32_to_cpu((*cont_array).num_elems) as c_int - 1 {
        match le32_to_cpu((*cont_elem).token) {
            AR_TKN_U32_CONTAINER_INSTANCE_ID => {
                container_id = le32_to_cpu((*cont_elem).value) as c_int;
                cont = audioreach_tplg_alloc_container(apm, sg, container_id as u32, &mut found);
                if is_err(cont) || found {
                    /* Error or Already parsed container data */
                    return cont;
                }
            }
            AR_TKN_U32_CONTAINER_CAPABILITY_ID => (*cont).capability_id = le32_to_cpu((*cont_elem).value),
            AR_TKN_U32_CONTAINER_STACK_SIZE => (*cont).stack_size = le32_to_cpu((*cont_elem).value),
            AR_TKN_U32_CONTAINER_GRAPH_POS => (*cont).graph_pos = le32_to_cpu((*cont_elem).value),
            AR_TKN_U32_CONTAINER_PROC_DOMAIN => (*cont).proc_domain = le32_to_cpu((*cont_elem).value),
            _ => {
                dev_err((*apm).dev, b"Not a valid token %d for graph\n\0".as_ptr() as *const c_char, (*cont_elem).token);
            }
        }
        tkn_count += 1;
        cont_elem = cont_elem.add(1);
    }

    cont
}

unsafe fn audioreach_parse_common_tokens(
    apm: *mut q6apm,
    cont: *mut audioreach_container,
    private: *const snd_soc_tplg_private,
    w: *mut snd_soc_dapm_widget,
) -> *mut audioreach_module {
    let mut max_ip_port: uint32_t = 0;
    let mut max_op_port: uint32_t = 0;
    let mut src_mod_op_port_id: [uint32_t; AR_MAX_MOD_LINKS as usize] = [0; AR_MAX_MOD_LINKS as usize];
    let mut dst_mod_inst_id: [uint32_t; AR_MAX_MOD_LINKS as usize] = [0; AR_MAX_MOD_LINKS as usize];
    let mut dst_mod_ip_port_id: [uint32_t; AR_MAX_MOD_LINKS as usize] = [0; AR_MAX_MOD_LINKS as usize];
    let mut src_mod_inst_id: uint32_t = 0;

    let mut module_id: c_int = 0;
    let mut instance_id: c_int = 0;
    let mut tkn_count: c_int = 0;
    let mut mod_elem: *const snd_soc_tplg_vendor_value_elem;
    let mod_array: *const snd_soc_tplg_vendor_array;
    let mut mod_: *mut audioreach_module = ptr::null_mut();
    let mut token: uint32_t;
    let mut found = false;
    let max_tokens: c_int;

    mod_array = audioreach_get_module_array(private);
    mod_elem = (*mod_array).value.as_ptr();
    max_tokens = le32_to_cpu((*mod_array).num_elems) as c_int;
    while tkn_count <= max_tokens - 1 {
        token = le32_to_cpu((*mod_elem).token);
        match token {
            /* common module info */
            AR_TKN_U32_MODULE_ID => module_id = le32_to_cpu((*mod_elem).value) as c_int,
            AR_TKN_U32_MODULE_INSTANCE_ID => {
                instance_id = le32_to_cpu((*mod_elem).value) as c_int;
                mod_ = audioreach_tplg_alloc_module(apm, cont, w, instance_id as u32, &mut found);
                if is_err(mod_) {
                    return mod_;
                } else if found {
                    dev_err((*apm).dev, b"Duplicate Module Instance ID 0x%08x found\n\0".as_ptr() as *const c_char, instance_id);
                    return err_ptr(-EINVAL);
                }
            }
            AR_TKN_U32_MODULE_MAX_IP_PORTS => max_ip_port = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_MAX_OP_PORTS => max_op_port = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_SRC_INSTANCE_ID => src_mod_inst_id = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_SRC_OP_PORT_ID => src_mod_op_port_id[0] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_SRC_OP_PORT_ID1 => src_mod_op_port_id[1] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_SRC_OP_PORT_ID2 => src_mod_op_port_id[2] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_SRC_OP_PORT_ID3 => src_mod_op_port_id[3] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_SRC_OP_PORT_ID4 => src_mod_op_port_id[4] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_SRC_OP_PORT_ID5 => src_mod_op_port_id[5] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_SRC_OP_PORT_ID6 => src_mod_op_port_id[6] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_SRC_OP_PORT_ID7 => src_mod_op_port_id[7] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_INSTANCE_ID => dst_mod_inst_id[0] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_INSTANCE_ID1 => dst_mod_inst_id[1] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_INSTANCE_ID2 => dst_mod_inst_id[2] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_INSTANCE_ID3 => dst_mod_inst_id[3] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_INSTANCE_ID4 => dst_mod_inst_id[4] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_INSTANCE_ID5 => dst_mod_inst_id[5] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_INSTANCE_ID6 => dst_mod_inst_id[6] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_INSTANCE_ID7 => dst_mod_inst_id[7] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_IN_PORT_ID => dst_mod_ip_port_id[0] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_IN_PORT_ID1 => dst_mod_ip_port_id[1] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_IN_PORT_ID2 => dst_mod_ip_port_id[2] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_IN_PORT_ID3 => dst_mod_ip_port_id[3] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_IN_PORT_ID4 => dst_mod_ip_port_id[4] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_IN_PORT_ID5 => dst_mod_ip_port_id[5] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_IN_PORT_ID6 => dst_mod_ip_port_id[6] = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_DST_IN_PORT_ID7 => dst_mod_ip_port_id[7] = le32_to_cpu((*mod_elem).value),
            _ => {}
        }
        tkn_count += 1;
        mod_elem = mod_elem.add(1);
    }

    if !mod_.is_null() {
        let mut pn: c_int;
        let mut id: c_int = 0;

        (*mod_).module_id = module_id as u32;
        (*mod_).max_ip_port = max_ip_port;
        (*mod_).max_op_port = max_op_port;
        (*mod_).src_mod_inst_id = src_mod_inst_id;
        pn = 0;
        while pn < (*mod_).max_op_port as c_int {
            if src_mod_op_port_id[pn as usize] != 0
                && dst_mod_inst_id[pn as usize] != 0
                && dst_mod_ip_port_id[pn as usize] != 0
            {
                (*mod_).src_mod_op_port_id[id as usize] = src_mod_op_port_id[pn as usize];
                (*mod_).dst_mod_inst_id[id as usize] = dst_mod_inst_id[pn as usize];
                (*mod_).dst_mod_ip_port_id[id as usize] = dst_mod_ip_port_id[pn as usize];
                id += 1;
                (*mod_).num_connections = id as u32;
            }
            pn += 1;
        }
    }

    mod_
}

unsafe fn audioreach_widget_load_module_common(
    component: *mut snd_soc_component,
    _index: c_int,
    w: *mut snd_soc_dapm_widget,
    tplg_w: *const snd_soc_tplg_dapm_widget,
) -> c_int {
    let apm = dev_get_drvdata((*component).dev) as *mut q6apm;
    let cont: *mut audioreach_container;
    let sg: *mut audioreach_sub_graph;
    let mod_: *mut audioreach_module;
    let dobj: *mut snd_soc_dobj;

    sg = audioreach_parse_sg_tokens(apm, &(*tplg_w).priv);
    if is_err(sg) {
        return ptr_err(sg);
    }

    cont = audioreach_parse_cont_tokens(apm, sg, &(*tplg_w).priv);
    if is_err(cont) {
        return ptr_err(cont);
    }

    mod_ = audioreach_parse_common_tokens(apm, cont, &(*tplg_w).priv, w);
    if is_err_or_null(mod_) {
        return if !mod_.is_null() { ptr_err(mod_) } else { -ENODEV };
    }

    (*mod_).data = audioreach_get_module_priv_data(&(*tplg_w).priv);

    dobj = &mut (*w).dobj;
    (*dobj).private = mod_ as *mut c_void;

    0
}

unsafe fn audioreach_widget_load_enc_dec_cnv(
    component: *mut snd_soc_component,
    index: c_int,
    w: *mut snd_soc_dapm_widget,
    tplg_w: *const snd_soc_tplg_dapm_widget,
) -> c_int {
    let mut mod_elem: *const snd_soc_tplg_vendor_value_elem;
    let mod_array: *const snd_soc_tplg_vendor_array;
    let mod_: *mut audioreach_module;
    let dobj: *mut snd_soc_dobj;
    let mut tkn_count: c_int = 0;
    let ret: c_int;

    ret = audioreach_widget_load_module_common(component, index, w, tplg_w);
    if ret != 0 {
        return ret;
    }

    dobj = &mut (*w).dobj;
    mod_ = (*dobj).private as *mut audioreach_module;
    mod_array = audioreach_get_module_array(&(*tplg_w).priv);
    mod_elem = (*mod_array).value.as_ptr();

    while tkn_count <= le32_to_cpu((*mod_array).num_elems) as c_int - 1 {
        match le32_to_cpu((*mod_elem).token) {
            AR_TKN_U32_MODULE_FMT_INTERLEAVE => (*mod_).interleave_type = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_FMT_SAMPLE_RATE => (*mod_).rate = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_FMT_BIT_DEPTH => (*mod_).bit_depth = le32_to_cpu((*mod_elem).value),
            _ => {}
        }
        tkn_count += 1;
        mod_elem = mod_elem.add(1);
    }

    0
}

unsafe fn audioreach_widget_log_module_load(
    mod_: *mut audioreach_module,
    mod_array: *const snd_soc_tplg_vendor_array,
) -> c_int {
    let mut mod_elem: *const snd_soc_tplg_vendor_value_elem = (*mod_array).value.as_ptr();
    let mut tkn_count: c_int = 0;

    while tkn_count <= le32_to_cpu((*mod_array).num_elems) as c_int - 1 {
        match le32_to_cpu((*mod_elem).token) {
            AR_TKN_U32_MODULE_LOG_CODE => (*mod_).log_code = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_LOG_TAP_POINT_ID => (*mod_).log_tap_point_id = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_LOG_MODE => (*mod_).log_mode = le32_to_cpu((*mod_elem).value),
            _ => {}
        }
        tkn_count += 1;
        mod_elem = mod_elem.add(1);
    }

    0
}

unsafe fn audioreach_widget_dma_module_load(
    mod_: *mut audioreach_module,
    mod_array: *const snd_soc_tplg_vendor_array,
) -> c_int {
    let mut mod_elem: *const snd_soc_tplg_vendor_value_elem = (*mod_array).value.as_ptr();
    let mut tkn_count: c_int = 0;

    while tkn_count <= le32_to_cpu((*mod_array).num_elems) as c_int - 1 {
        match le32_to_cpu((*mod_elem).token) {
            AR_TKN_U32_MODULE_HW_IF_IDX => (*mod_).hw_interface_idx = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_FMT_DATA => (*mod_).data_format = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_HW_IF_TYPE => (*mod_).hw_interface_type = le32_to_cpu((*mod_elem).value),
            _ => {}
        }
        tkn_count += 1;
        mod_elem = mod_elem.add(1);
    }

    0
}

unsafe fn audioreach_widget_i2s_module_load(
    mod_: *mut audioreach_module,
    mod_array: *const snd_soc_tplg_vendor_array,
) -> c_int {
    let mut mod_elem: *const snd_soc_tplg_vendor_value_elem = (*mod_array).value.as_ptr();
    let mut tkn_count: c_int = 0;

    while tkn_count <= le32_to_cpu((*mod_array).num_elems) as c_int - 1 {
        match le32_to_cpu((*mod_elem).token) {
            AR_TKN_U32_MODULE_HW_IF_IDX => (*mod_).hw_interface_idx = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_FMT_DATA => (*mod_).data_format = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_HW_IF_TYPE => (*mod_).hw_interface_type = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_SD_LINE_IDX => (*mod_).sd_line_idx = le32_to_cpu((*mod_elem).value),
            AR_TKN_U32_MODULE_WS_SRC => (*mod_).ws_src = le32_to_cpu((*mod_elem).value),
            _ => {}
        }
        tkn_count += 1;
        mod_elem = mod_elem.add(1);
    }

    0
}

unsafe fn audioreach_widget_audio_if_module_load(
    mod_: *mut audioreach_module,
    mod_array: *const snd_soc_tplg_vendor_array,
) -> c_int {
    let mut mod_elem: *const snd_soc_tplg_vendor_value_elem = (*mod_array).value.as_ptr();
    let mut tkn_count: c_int = 0;
    let mut val: u32;

    while tkn_count < le32_to_cpu((*mod_array).num_elems) as c_int {
        val = le32_to_cpu((*mod_elem).value);
        match le32_to_cpu((*mod_elem).token) {
            AR_TKN_U32_MODULE_HW_IF_IDX => (*mod_).hw_interface_idx = val,
            AR_TKN_U32_MODULE_FMT_DATA => (*mod_).data_format = val,
            AR_TKN_U16_MODULE_SYNC_SRC => {
                if val > U16_MAX as u32 { return -EINVAL; }
                (*mod_).sync_src = val as u16;
            }
            AR_TKN_U16_MODULE_CTRL_DATA_OUT_ENABLE => {
                if val > U16_MAX as u32 { return -EINVAL; }
                (*mod_).ctrl_data_out_enable = val as u16;
            }
            AR_TKN_U32_MODULE_SLOT_MASK => (*mod_).slot_mask = val,
            AR_TKN_U16_MODULE_NSLOTS_PER_FRAME => {
                if val > U16_MAX as u32 { return -EINVAL; }
                (*mod_).nslots_per_frame = val as u16;
            }
            AR_TKN_U16_MODULE_SLOT_WIDTH => {
                if val > U16_MAX as u32 { return -EINVAL; }
                (*mod_).slot_width = val as u16;
            }
            AR_TKN_U16_MODULE_INTF_MODE => {
                if val > U16_MAX as u32 { return -EINVAL; }
                (*mod_).intf_mode = val as u16;
            }
            AR_TKN_U16_MODULE_SYNC_MODE => {
                if val > U16_MAX as u32 { return -EINVAL; }
                (*mod_).sync_mode = val as u16;
            }
            AR_TKN_U16_MODULE_CTRL_INVERT_SYNC_PULSE => {
                if val > U16_MAX as u32 { return -EINVAL; }
                (*mod_).ctrl_invert_sync_pulse = val as u16;
            }
            AR_TKN_U16_MODULE_CTRL_SYNC_DATA_DELAY => {
                if val > U16_MAX as u32 { return -EINVAL; }
                (*mod_).ctrl_sync_data_delay = val as u16;
            }
            AR_TKN_U16_MODULE_QAIF_TYPE => {
                if val > U16_MAX as u32 { return -EINVAL; }
                (*mod_).qaif_type = val as u16;
            }
            AR_TKN_U32_MODULE_ACTIVE_LANE_MASK => (*mod_).active_lane_mask = val,
            AR_TKN_U32_MODULE_FRAME_SYNC_RATE => (*mod_).frame_sync_rate = val,
            AR_TKN_U16_MODULE_BIT_CLK_TYPE => {
                if val > U16_MAX as u32 { return -EINVAL; }
                (*mod_).bit_clk_type = val as u16;
            }
            AR_TKN_U8_MODULE_INV_INT_BIT_CLK => {
                if val > U8_MAX as u32 { return -EINVAL; }
                (*mod_).inv_int_bit_clk = val as u8;
            }
            AR_TKN_U8_MODULE_INV_EXT_BIT_CLK => {
                if val > U8_MAX as u32 { return -EINVAL; }
                (*mod_).inv_ext_bit_clk = val as u8;
            }
            _ => {}
        }
        tkn_count += 1;
        mod_elem = mod_elem.add(1);
    }

    0
}

unsafe fn audioreach_widget_dp_module_load(
    mod_: *mut audioreach_module,
    mod_array: *const snd_soc_tplg_vendor_array,
) -> c_int {
    let mut mod_elem: *const snd_soc_tplg_vendor_value_elem = (*mod_array).value.as_ptr();
    let mut tkn_count: c_int = 0;

    while tkn_count <= le32_to_cpu((*mod_array).num_elems) as c_int - 1 {
        match le32_to_cpu((*mod_elem).token) {
            AR_TKN_U32_MODULE_FMT_DATA => (*mod_).data_format = le32_to_cpu((*mod_elem).value),
            _ => {}
        }
        tkn_count += 1;
        mod_elem = mod_elem.add(1);
    }

    0
}

unsafe fn audioreach_widget_load_buffer(
    component: *mut snd_soc_component,
    index: c_int,
    w: *mut snd_soc_dapm_widget,
    tplg_w: *const snd_soc_tplg_dapm_widget,
) -> c_int {
    let mod_array: *const snd_soc_tplg_vendor_array;
    let mod_: *mut audioreach_module;
    let dobj: *mut snd_soc_dobj;
    let mut ret: c_int;

    ret = audioreach_widget_load_module_common(component, index, w, tplg_w);
    if ret != 0 {
        return ret;
    }

    dobj = &mut (*w).dobj;
    mod_ = (*dobj).private as *mut audioreach_module;

    mod_array = audioreach_get_module_array(&(*tplg_w).priv);

    match (*mod_).module_id {
        MODULE_ID_CODEC_DMA_SINK | MODULE_ID_CODEC_DMA_SOURCE => {
            audioreach_widget_dma_module_load(mod_, mod_array);
        }
        MODULE_ID_DATA_LOGGING => {
            audioreach_widget_log_module_load(mod_, mod_array);
        }
        MODULE_ID_I2S_SINK | MODULE_ID_I2S_SOURCE => {
            audioreach_widget_i2s_module_load(mod_, mod_array);
        }
        MODULE_ID_AUDIO_IF_SINK | MODULE_ID_AUDIO_IF_SOURCE => {
            ret = audioreach_widget_audio_if_module_load(mod_, mod_array);
            if ret != 0 {
                return ret;
            }
        }
        MODULE_ID_DISPLAY_PORT_SINK => {
            audioreach_widget_dp_module_load(mod_, mod_array);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe fn audioreach_widget_load_mixer(
    component: *mut snd_soc_component,
    _index: c_int,
    w: *mut snd_soc_dapm_widget,
    tplg_w: *const snd_soc_tplg_dapm_widget,
) -> c_int {
    let mut w_elem: *const snd_soc_tplg_vendor_value_elem;
    let w_array: *const snd_soc_tplg_vendor_array;
    let scontrol: *mut snd_ar_control;
    let data = dev_get_drvdata((*component).dev) as *mut q6apm;
    let dobj: *mut snd_soc_dobj;
    let mut tkn_count: c_int = 0;

    w_array = (*tplg_w).priv.array.as_ptr();

    scontrol = kzalloc_obj::<snd_ar_control>();
    if scontrol.is_null() {
        return -ENOMEM;
    }

    (*scontrol).scomp = component;
    dobj = &mut (*w).dobj;
    (*dobj).private = scontrol as *mut c_void;

    w_elem = (*w_array).value.as_ptr();
    while tkn_count <= le32_to_cpu((*w_array).num_elems) as c_int - 1 {
        match le32_to_cpu((*w_elem).token) {
            AR_TKN_U32_SUB_GRAPH_INSTANCE_ID => (*scontrol).sgid = le32_to_cpu((*w_elem).value),
            AR_TKN_DAI_INDEX => (*scontrol).graph_id = le32_to_cpu((*w_elem).value),
            _ => {}
        }
        tkn_count += 1;
        w_elem = w_elem.add(1);
    }

    (*scontrol).w = w;
    list_add_tail(&mut (*scontrol).node, &mut (*data).widget_list);

    0
}

unsafe extern "C" fn audioreach_pga_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let dapm = (*w).dapm;
    let c = snd_soc_dapm_to_component(dapm);
    let mod_ = (*w).dobj.private as *mut audioreach_module;
    let apm = dev_get_drvdata((*c).dev) as *mut q6apm;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* apply gain after power up of widget */
            audioreach_gain_set_vol_ctrl(apm, mod_, (*mod_).gain);
        }
        _ => {}
    }

    0
}

static audioreach_widget_ops: [snd_soc_tplg_widget_events; 1] = [
    snd_soc_tplg_widget_events {
        event_type: AR_PGA_DAPM_EVENT,
        event_handler: Some(audioreach_pga_event),
    },
];

unsafe fn audioreach_widget_load_pga(
    component: *mut snd_soc_component,
    index: c_int,
    w: *mut snd_soc_dapm_widget,
    tplg_w: *const snd_soc_tplg_dapm_widget,
) -> c_int {
    let mod_: *mut audioreach_module;
    let dobj: *mut snd_soc_dobj;
    let mut ret: c_int;

    ret = audioreach_widget_load_module_common(component, index, w, tplg_w);
    if ret != 0 {
        return ret;
    }

    dobj = &mut (*w).dobj;
    mod_ = (*dobj).private as *mut audioreach_module;
    (*mod_).gain = VOL_CTRL_DEFAULT_GAIN;

    ret = snd_soc_tplg_widget_bind_event(
        w,
        audioreach_widget_ops.as_ptr(),
        audioreach_widget_ops.len(),
        le16_to_cpu((*tplg_w).event_type),
    );
    if ret != 0 {
        dev_err(
            (*component).dev,
            b"matching event handlers NOT found for %d\n\0".as_ptr() as *const c_char,
            le16_to_cpu((*tplg_w).event_type) as c_int,
        );
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn audioreach_widget_ready(
    component: *mut snd_soc_component,
    index: c_int,
    w: *mut snd_soc_dapm_widget,
    tplg_w: *mut snd_soc_tplg_dapm_widget,
) -> c_int {
    match (*w).id {
        snd_soc_dapm_aif_in | snd_soc_dapm_aif_out => {
            audioreach_widget_load_buffer(component, index, w, tplg_w);
        }
        snd_soc_dapm_decoder | snd_soc_dapm_encoder | snd_soc_dapm_src => {
            audioreach_widget_load_enc_dec_cnv(component, index, w, tplg_w);
        }
        snd_soc_dapm_buffer => {
            audioreach_widget_load_buffer(component, index, w, tplg_w);
        }
        snd_soc_dapm_mixer => return audioreach_widget_load_mixer(component, index, w, tplg_w),
        snd_soc_dapm_pga => return audioreach_widget_load_pga(component, index, w, tplg_w),
        snd_soc_dapm_dai_link | snd_soc_dapm_scheduler | snd_soc_dapm_out_drv | _ => {
            dev_err((*component).dev, b"Widget type (0x%x) not yet supported\n\0".as_ptr() as *const c_char, (*w).id);
        }
    }

    0
}

unsafe extern "C" fn audioreach_widget_unload(
    scomp: *mut snd_soc_component,
    dobj: *mut snd_soc_dobj,
) -> c_int {
    let w = container_of_snd_soc_dapm_widget_from_dobj(dobj);
    let apm = dev_get_drvdata((*scomp).dev) as *mut q6apm;
    let cont: *mut audioreach_container;
    let mod_: *mut audioreach_module;

    if (*w).id == snd_soc_dapm_mixer {
        /* virtual widget */
        let scontrol = (*dobj).private as *mut snd_ar_control;

        list_del(&mut (*scontrol).node);
        kfree(scontrol as *mut c_void);
        return 0;
    }
    mod_ = (*dobj).private as *mut audioreach_module;
    if mod_.is_null() {
        return 0;
    }

    cont = (*mod_).container;

    mutex_lock(&mut (*apm).lock as *mut _ as *mut c_void);
    idr_remove(&mut (*apm).modules_idr as *mut _ as *mut c_void, (*mod_).instance_id);
    (*cont).num_modules -= 1;

    list_del(&mut (*mod_).node);
    kfree((*mod_).data as *mut c_void);
    kfree(mod_ as *mut c_void);
    /* Graph Info has N sub-graphs, sub-graph has N containers, Container has N Modules */
    if list_empty(&mut (*cont).modules_list) {
        /* if no modules in the container then remove it */
        let sg = (*cont).sub_graph;

        idr_remove(&mut (*apm).containers_idr as *mut _ as *mut c_void, (*cont).container_id);
        list_del(&mut (*cont).node);
        (*sg).num_containers -= 1;
        kfree(cont as *mut c_void);
        /* check if there are no more containers in the sub graph and remove it */
        if list_empty(&mut (*sg).container_list) {
            let info = (*sg).info;

            idr_remove(&mut (*apm).sub_graphs_idr as *mut _ as *mut c_void, (*sg).sub_graph_id);
            list_del(&mut (*sg).node);
            (*info).num_sub_graphs -= 1;
            kfree(sg as *mut c_void);
            /* Check if there are no more sub-graphs left then remove graph info */
            if list_empty(&mut (*info).sg_list) {
                idr_remove(&mut (*apm).graph_info_idr as *mut _ as *mut c_void, (*info).id);
                kfree(info as *mut c_void);
            }
        }
    }

    mutex_unlock(&mut (*apm).lock as *mut _ as *mut c_void);

    0
}

unsafe fn audioreach_find_widget(comp: *mut snd_soc_component, name: *const c_char) -> *mut snd_ar_control {
    let apm = dev_get_drvdata((*comp).dev) as *mut q6apm;
    let mut control: *mut snd_ar_control;

    control = list_first_entry_or_null_snd_ar_control(&mut (*apm).widget_list);
    while !control.is_null() {
        if !(*control).w.is_null() && strcmp(name, (*(*control).w).name) == 0 {
            return control;
        }
        control = list_next_entry_or_null_snd_ar_control(control, &mut (*apm).widget_list);
    }

    ptr::null_mut()
}

unsafe fn audioreach_find_module(comp: *mut snd_soc_component, name: *const c_char) -> *mut audioreach_module {
    let apm = dev_get_drvdata((*comp).dev) as *mut q6apm;
    let mut module: *mut audioreach_module = ptr::null_mut();
    let mut id: c_int = 0;

    while idr_for_each_entry_audioreach_module(&mut (*apm).modules_idr, &mut module, &mut id) {
        if strcmp(name, (*(*module).widget).name) == 0 {
            return module;
        }
    }

    ptr::null_mut()
}

unsafe extern "C" fn audioreach_route_load(
    scomp: *mut snd_soc_component,
    _index: c_int,
    route: *mut snd_soc_dapm_route,
) -> c_int {
    let src_module: *const audioreach_module;
    let sink_module: *const audioreach_module;
    let mut control: *mut snd_ar_control;
    let w: *mut snd_soc_dapm_widget;
    let mut i: c_int;

    /* check if these are actual modules */
    src_module = audioreach_find_module(scomp, (*route).source);
    sink_module = audioreach_find_module(scomp, (*route).sink);

    if !sink_module.is_null() && src_module.is_null() {
        control = audioreach_find_widget(scomp, (*route).source);
        if !control.is_null() {
            (*control).module_instance_id = (*sink_module).instance_id;
        }
    } else if sink_module.is_null() && !src_module.is_null() && !(*route).control.is_null() {
        /* check if this is a virtual mixer */
        control = audioreach_find_widget(scomp, (*route).sink);
        if control.is_null() || (*control).w.is_null() {
            return 0;
        }

        w = (*control).w;

        i = 0;
        while i < (*w).num_kcontrols {
            if strcmp((*route).control, (*(*w).kcontrol_news.add(i as usize)).name) == 0 {
                let sm: *mut soc_mixer_control;
                let dobj: *mut snd_soc_dobj;
                let scontrol: *mut snd_ar_control;

                sm = (*(*w).kcontrol_news.add(i as usize)).private_value as *mut soc_mixer_control;
                dobj = &mut (*sm).dobj;
                scontrol = (*dobj).private as *mut snd_ar_control;
                (*scontrol).module_instance_id = (*src_module).instance_id;
            }
            i += 1;
        }
    }

    0
}

unsafe extern "C" fn audioreach_route_unload(
    _scomp: *mut snd_soc_component,
    _dobj: *mut snd_soc_dobj,
) -> c_int {
    0
}

unsafe extern "C" fn audioreach_tplg_complete(_component: *mut snd_soc_component) -> c_int {
    /* TBD */
    0
}

/* DAI link - used for any driver specific init */
unsafe extern "C" fn audioreach_link_load(
    component: *mut snd_soc_component,
    _index: c_int,
    link: *mut snd_soc_dai_link,
    _cfg: *mut snd_soc_tplg_link_config,
) -> c_int {
    (*(*link).platforms).name = ptr::null();
    (*link).nonatomic = true;
    (*link).dynamic = true;
    (*(*link).platforms).of_node = of_get_compatible_child(
        (*(*component).dev).of_node,
        b"qcom,q6apm-dais\0".as_ptr() as *const c_char,
    );
    0
}

unsafe fn audioreach_connect_sub_graphs(
    apm: *mut q6apm,
    m1: *const snd_ar_control,
    m2: *const snd_ar_control,
    connect: bool,
) {
    let info: *mut audioreach_graph_info;

    mutex_lock(&mut (*apm).lock as *mut _ as *mut c_void);
    info = idr_find(&mut (*apm).graph_info_idr as *mut _ as *mut c_void, (*m2).graph_id) as *mut audioreach_graph_info;
    mutex_unlock(&mut (*apm).lock as *mut _ as *mut c_void);

    if connect {
        (*info).src_mod_inst_id = (*m1).module_instance_id;
        (*info).src_mod_op_port_id = 1;
        (*info).dst_mod_inst_id = (*m2).module_instance_id;
        (*info).dst_mod_ip_port_id = 2;
    } else {
        (*info).src_mod_inst_id = 0;
        (*info).src_mod_op_port_id = 0;
        (*info).dst_mod_inst_id = 0;
        (*info).dst_mod_ip_port_id = 0;
    }
}

unsafe fn audioreach_is_vmixer_connected(
    apm: *mut q6apm,
    m1: *const snd_ar_control,
    m2: *const snd_ar_control,
) -> bool {
    let info: *const audioreach_graph_info;

    mutex_lock(&mut (*apm).lock as *mut _ as *mut c_void);
    info = idr_find(&mut (*apm).graph_info_idr as *mut _ as *mut c_void, (*m2).graph_id) as *const audioreach_graph_info;
    mutex_unlock(&mut (*apm).lock as *mut _ as *mut c_void);

    if (*info).dst_mod_inst_id == (*m2).module_instance_id
        && (*info).src_mod_inst_id == (*m1).module_instance_id
    {
        return true;
    }

    false
}

unsafe extern "C" fn audioreach_get_audio_mixer(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let dw = snd_soc_dapm_kcontrol_to_widget(kcontrol) as *const snd_soc_dapm_widget;
    let c = snd_soc_dapm_to_component(dapm);
    let dapm_scontrol = (*dw).dobj.private as *const snd_ar_control;
    let scontrol = (*mc).dobj.private as *const snd_ar_control;
    let data = dev_get_drvdata((*c).dev) as *mut q6apm;
    let connected: bool;

    connected = audioreach_is_vmixer_connected(data, scontrol, dapm_scontrol);
    if connected {
        (*ucontrol).value.integer.value[0] = 1;
    } else {
        (*ucontrol).value.integer.value[0] = 0;
    }

    0
}

unsafe extern "C" fn audioreach_put_audio_mixer(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let dw = snd_soc_dapm_kcontrol_to_widget(kcontrol);
    let c = snd_soc_dapm_to_component(dapm);
    let dapm_scontrol = (*dw).dobj.private as *const snd_ar_control;
    let scontrol = (*mc).dobj.private as *const snd_ar_control;
    let data = dev_get_drvdata((*c).dev) as *mut q6apm;

    if (*ucontrol).value.integer.value[0] != 0 {
        audioreach_connect_sub_graphs(data, scontrol, dapm_scontrol, true);
        snd_soc_dapm_mixer_update_power(dapm, kcontrol, 1, ptr::null_mut());
    } else {
        audioreach_connect_sub_graphs(data, scontrol, dapm_scontrol, false);
        snd_soc_dapm_mixer_update_power(dapm, kcontrol, 0, ptr::null_mut());
    }
    0
}

unsafe extern "C" fn audioreach_get_vol_ctrl_audio_mixer(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dw = snd_soc_dapm_kcontrol_to_widget(kcontrol);
    let mod_ = (*dw).dobj.private as *mut audioreach_module;

    (*ucontrol).value.integer.value[0] = (*mod_).gain as i64;

    0
}

unsafe extern "C" fn audioreach_put_vol_ctrl_audio_mixer(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dw = snd_soc_dapm_kcontrol_to_widget(kcontrol);
    let mod_ = (*dw).dobj.private as *mut audioreach_module;

    (*mod_).gain = (*ucontrol).value.integer.value[0] as c_int;

    1
}

unsafe fn audioreach_control_load_mix(
    _scomp: *mut snd_soc_component,
    scontrol: *mut snd_ar_control,
    _kc: *mut snd_kcontrol_new,
    hdr: *const snd_soc_tplg_ctl_hdr,
) -> c_int {
    let mut c_elem: *const snd_soc_tplg_vendor_value_elem;
    let c_array: *const snd_soc_tplg_vendor_array;
    let mc: *const snd_soc_tplg_mixer_control;
    let mut tkn_count: c_int = 0;

    mc = container_of_const_snd_soc_tplg_mixer_control_from_hdr(hdr);
    c_array = (*mc).priv.data as *const snd_soc_tplg_vendor_array;

    c_elem = (*c_array).value.as_ptr();

    while tkn_count <= le32_to_cpu((*c_array).num_elems) as c_int - 1 {
        match le32_to_cpu((*c_elem).token) {
            AR_TKN_U32_SUB_GRAPH_INSTANCE_ID => (*scontrol).sgid = le32_to_cpu((*c_elem).value),
            AR_TKN_DAI_INDEX => (*scontrol).graph_id = le32_to_cpu((*c_elem).value),
            _ => {
                /* Ignore other tokens */
            }
        }
        c_elem = c_elem.add(1);
        tkn_count += 1;
    }

    0
}

unsafe extern "C" fn audioreach_control_load(
    scomp: *mut snd_soc_component,
    _index: c_int,
    kc: *mut snd_kcontrol_new,
    hdr: *mut snd_soc_tplg_ctl_hdr,
) -> c_int {
    let scontrol: *mut snd_ar_control;
    let sm: *mut soc_mixer_control;
    let dobj: *mut snd_soc_dobj;
    let mut ret: c_int = 0;

    scontrol = kzalloc_obj::<snd_ar_control>();
    if scontrol.is_null() {
        return -ENOMEM;
    }

    (*scontrol).scomp = scomp;

    match le32_to_cpu((*hdr).ops.get) {
        SND_SOC_AR_TPLG_FE_BE_GRAPH_CTL_MIX => {
            sm = (*kc).private_value as *mut soc_mixer_control;
            dobj = &mut (*sm).dobj;
            ret = audioreach_control_load_mix(scomp, scontrol, kc, hdr);
        }
        SND_SOC_AR_TPLG_VOL_CTL => {
            sm = (*kc).private_value as *mut soc_mixer_control;
            dobj = &mut (*sm).dobj;
        }
        _ => {
            dev_warn(
                (*scomp).dev,
                b"control type not supported %d:%d:%d\n\0".as_ptr() as *const c_char,
                (*hdr).ops.get,
                (*hdr).ops.put,
                (*hdr).ops.info,
            );
            kfree(scontrol as *mut c_void);
            return -EINVAL;
        }
    }

    (*dobj).private = scontrol as *mut c_void;
    ret
}

unsafe extern "C" fn audioreach_control_unload(
    _scomp: *mut snd_soc_component,
    dobj: *mut snd_soc_dobj,
) -> c_int {
    let scontrol = (*dobj).private as *mut snd_ar_control;

    kfree(scontrol as *mut c_void);

    0
}

static audioreach_io_ops: [snd_soc_tplg_kcontrol_ops; 2] = [
    snd_soc_tplg_kcontrol_ops {
        id: SND_SOC_AR_TPLG_FE_BE_GRAPH_CTL_MIX,
        get: Some(audioreach_get_audio_mixer),
        put: Some(audioreach_put_audio_mixer),
        info: Some(snd_soc_info_volsw),
    },
    snd_soc_tplg_kcontrol_ops {
        id: SND_SOC_AR_TPLG_VOL_CTL,
        get: Some(audioreach_get_vol_ctrl_audio_mixer),
        put: Some(audioreach_put_vol_ctrl_audio_mixer),
        info: Some(snd_soc_info_volsw),
    },
];

static audioreach_tplg_ops: snd_soc_tplg_ops = snd_soc_tplg_ops {
    io_ops: audioreach_io_ops.as_ptr(),
    io_ops_count: audioreach_io_ops.len() as u32,

    control_load: Some(audioreach_control_load),
    control_unload: Some(audioreach_control_unload),

    widget_ready: Some(audioreach_widget_ready),
    widget_unload: Some(audioreach_widget_unload),

    complete: Some(audioreach_tplg_complete),
    link_load: Some(audioreach_link_load),

    dapm_route_load: Some(audioreach_route_load),
    dapm_route_unload: Some(audioreach_route_unload),
};

#[no_mangle]
pub unsafe extern "C" fn audioreach_tplg_init(component: *mut snd_soc_component) -> c_int {
    let card = (*component).card;
    let dev = (*component).dev;
    let mut ret: c_int;

    /* Inline with Qualcomm UCM configs and linux-firmware path */
    let tplg_fw_name = kasprintf(
        GFP_KERNEL,
        b"qcom/%s/%s-tplg.bin\0".as_ptr() as *const c_char,
        (*card).driver_name,
        (*card).name,
    );
    if tplg_fw_name.is_null() {
        return -ENOMEM;
    }

    let mut fw: *const firmware = ptr::null();
    ret = request_firmware(&mut fw, tplg_fw_name, dev);
    if ret < 0 {
        dev_err(dev, b"tplg firmware loading %s failed %d\n\0".as_ptr() as *const c_char, tplg_fw_name, ret);
        kfree(tplg_fw_name as *mut c_void);
        return ret;
    }

    ret = snd_soc_tplg_component_load(component, &audioreach_tplg_ops, fw);
    if ret < 0 {
        if ret != -EPROBE_DEFER {
            dev_err(dev, b"tplg component load failed: %d\n\0".as_ptr() as *const c_char, ret);
        }
    }

    release_firmware(fw);
    kfree(tplg_fw_name as *mut c_void);
    ret
}

// EXPORT_SYMBOL_GPL(audioreach_tplg_init);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
