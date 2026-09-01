// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

use core::ffi::c_void;

unsafe extern "C" {
    static AVS_COPIER_MOD_UUID: guid_t;

    fn snd_hdac_adsp_readl(adev: *mut avs_dev, reg: u32) -> u32;
    fn avs_skl_ipc_interrupt(adev: *mut avs_dev);
    fn fls_long(x: c_ulong) -> c_int;
    fn kzalloc(size: u32, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn avs_ipc_set_enable_logs(adev: *mut avs_dev, data: *mut u8, size: u32) -> c_int;
    fn avs_log_buffer_addr(adev: *mut avs_dev, core: u32) -> *mut c_void;
    fn memcpy_fromio(to: *mut c_void, from: *const c_void, count: usize);
    fn avs_logging_fw(adev: *mut avs_dev) -> bool;
    fn avs_apl_log_payload_addr(addr: *mut c_void) -> *mut c_void;
    fn avs_apl_log_payload_size(adev: *mut avs_dev) -> u32;
    fn avs_dump_fw_log(adev: *mut avs_dev, buf: *mut c_void, size: u32);
    fn avs_dump_fw_log_wakeup(adev: *mut avs_dev, buf: *mut c_void, size: u32);
    fn writel(value: u32, addr: *mut c_void);
    fn msecs_to_jiffies(m: u32) -> c_ulong;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn time_after(a: c_ulong, b: c_ulong) -> bool;
    fn readl(addr: *const c_void) -> u32;
    fn vzalloc(size: usize) -> *mut u8;
    fn avs_sram_addr(adev: *mut avs_dev, offset: u32) -> *mut c_void;
    fn avs_log_buffer_status_locked(adev: *mut avs_dev, msg: *mut avs_notify_msg);
    fn dev_coredumpv(dev: *mut device, data: *mut u8, datalen: usize, gfp: gfp_t);
    fn guid_equal(a: *const guid_t, b: *const guid_t) -> bool;
    fn list_empty(head: *const list_head) -> bool;
    fn avs_ipc_set_d0ix(adev: *mut avs_dev, enable: bool, streaming: bool) -> c_int;

    fn avs_dsp_core_power();
    fn avs_dsp_core_reset();
    fn avs_dsp_core_stall();
    fn avs_dsp_interrupt_control();
    fn avs_hda_load_basefw();
    fn avs_hda_load_library();
    fn avs_hda_transfer_modules();
    fn avs_skl_log_buffer_offset();
}

type c_int = i32;
type c_ulong = u64;
type gfp_t = u32;
type irqreturn_t = c_int;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const UINT_MAX: u32 = u32::MAX;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const ETIMEDOUT: c_int = 110;
const GFP_KERNEL: gfp_t = 0;
const AVS_ADSP_REG_ADSPIS: u32 = 0;
const AVS_ADSP_ADSPIS_IPC: u32 = 0;
const AVS_FW_REGS_SIZE: usize = 0;
const AVS_FW_REGS_WINDOW: u32 = 0;
const INVALID_OBJECT_ID: u32 = 0;

#[repr(C)]
struct avs_dev {
    hw_cfg: avs_hw_cfg,
    dev: *mut device,
    path_list_lock: spinlock_t,
    path_list: list_head,
}

#[repr(C)]
struct avs_hw_cfg {
    dsp_cores: u32,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
struct guid_t {
    _private: [u8; 0],
}

#[repr(C)]
struct avs_ipc_msg {
    _private: [u8; 0],
}

#[repr(C)]
struct avs_apl_log_buffer_layout {
    read_ptr: u32,
    write_ptr: u32,
}

#[repr(C)]
struct avs_apl_log_state_info {
    aging_timer_period: u32,
    fifo_full_timer_period: u32,
    core_mask: c_ulong,
    logs_core: [avs_apl_log_core_state; 0],
}

#[repr(C)]
struct avs_apl_log_core_state {
    enable: avs_log_enable,
    min_priority: u32,
}

type avs_log_enable = c_int;

#[repr(C)]
struct avs_path {
    ppl_list: list_head,
    node: list_head,
}

#[repr(C)]
struct avs_path_pipeline {
    mod_list: list_head,
    node: list_head,
}

#[repr(C)]
struct avs_path_module {
    template: *mut avs_tplg_module_template,
    gtw_attrs: avs_gtw_attrs,
    node: list_head,
}

#[repr(C)]
struct avs_tplg_module_template {
    cfg_ext: *mut avs_tplg_modcfg_ext,
}

#[repr(C)]
struct avs_tplg_modcfg_ext {
    type_: guid_t,
    copier: avs_tplg_copier_cfg,
}

#[repr(C)]
struct avs_tplg_copier_cfg {
    dma_type: u32,
}

#[repr(C)]
struct avs_gtw_attrs {
    lp_buffer_alloc: bool,
}

#[repr(C)]
union avs_notify_msg {
    log: avs_notify_msg_log,
    ext: avs_notify_msg_ext,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct avs_notify_msg_log {
    core: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct avs_notify_msg_ext {
    coredump: avs_notify_msg_coredump,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct avs_notify_msg_coredump {
    stack_dump_size: usize,
    core_id: u32,
}

#[repr(C)]
struct avs_dsp_ops {
    power: Option<unsafe extern "C" fn()>,
    reset: Option<unsafe extern "C" fn()>,
    stall: Option<unsafe extern "C" fn()>,
    dsp_interrupt: Option<unsafe extern "C" fn(*mut avs_dev) -> irqreturn_t>,
    int_control: Option<unsafe extern "C" fn()>,
    load_basefw: Option<unsafe extern "C" fn()>,
    load_lib: Option<unsafe extern "C" fn()>,
    transfer_mods: Option<unsafe extern "C" fn()>,
    log_buffer_offset: Option<unsafe extern "C" fn()>,
    log_buffer_status: Option<unsafe extern "C" fn(*mut avs_dev, *mut avs_notify_msg) -> c_int>,
    coredump: Option<unsafe extern "C" fn(*mut avs_dev, *mut avs_notify_msg) -> c_int>,
    d0ix_toggle: Option<unsafe extern "C" fn(*mut avs_dev, *mut avs_ipc_msg, bool) -> bool>,
    set_d0ix: Option<unsafe extern "C" fn(*mut avs_dev, bool) -> c_int>,
    // AVS_SET_ENABLE_LOGS_OP(apl)
}

extern "C" {
    static mut jiffies: c_ulong;
}

macro_rules! AVS_IPC_RET {
    ($ret:expr) => {
        $ret
    };
}

macro_rules! AVS_NOTIFICATION {
    (LOG_BUFFER_STATUS) => {
        avs_notify_msg {
            log: avs_notify_msg_log { core: 0 },
        }
    };
}

unsafe extern "C" fn avs_apl_dsp_interrupt(adev: *mut avs_dev) -> irqreturn_t {
    let adspis: u32 = unsafe { snd_hdac_adsp_readl(adev, AVS_ADSP_REG_ADSPIS) };
    let mut ret: irqreturn_t = IRQ_NONE;

    if adspis == UINT_MAX {
        return ret;
    }

    if (adspis & AVS_ADSP_ADSPIS_IPC) != 0 {
        unsafe { avs_skl_ipc_interrupt(adev) };
        ret = IRQ_HANDLED;
    }

    ret
}

// CONFIG_DEBUG_FS
unsafe extern "C" fn avs_apl_enable_logs(
    adev: *mut avs_dev,
    enable: avs_log_enable,
    aging_period: u32,
    fifo_full_period: u32,
    resource_mask: c_ulong,
    mut priorities: *mut u32,
) -> c_int {
    let mut info: *mut avs_apl_log_state_info;
    let size: u32;
    let num_cores: u32 = unsafe { (*adev).hw_cfg.dsp_cores };
    let mut ret: c_int;
    let mut i: c_ulong;

    if unsafe { fls_long(resource_mask) } as u32 > num_cores {
        return -EINVAL;
    }
    size = (core::mem::size_of::<avs_apl_log_state_info>()
        + core::mem::size_of::<avs_apl_log_core_state>() * num_cores as usize) as u32;
    info = unsafe { kzalloc(size, GFP_KERNEL) as *mut avs_apl_log_state_info };
    if info.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*info).aging_timer_period = aging_period;
        (*info).fifo_full_timer_period = fifo_full_period;
        (*info).core_mask = resource_mask;
    }
    if enable != 0 {
        i = 0;
        while i < num_cores as c_ulong {
            if (resource_mask & (1u64 << i)) != 0 {
                unsafe {
                    let logs_core = (*info).logs_core.as_mut_ptr();
                    (*logs_core.add(i as usize)).enable = enable;
                    (*logs_core.add(i as usize)).min_priority = *priorities;
                    priorities = priorities.add(1);
                }
            }
            i += 1;
        }
    } else {
        i = 0;
        while i < num_cores as c_ulong {
            if (resource_mask & (1u64 << i)) != 0 {
                unsafe {
                    let logs_core = (*info).logs_core.as_mut_ptr();
                    (*logs_core.add(i as usize)).enable = enable;
                }
            }
            i += 1;
        }
    }

    ret = unsafe { avs_ipc_set_enable_logs(adev, info as *mut u8, size) };
    unsafe { kfree(info as *mut c_void) };
    if ret != 0 {
        return AVS_IPC_RET!(ret);
    }

    0
}

unsafe extern "C" fn avs_apl_log_buffer_status(
    adev: *mut avs_dev,
    msg: *mut avs_notify_msg,
) -> c_int {
    let mut layout: avs_apl_log_buffer_layout = unsafe { core::mem::zeroed() };
    let addr: *mut c_void;
    let buf: *mut c_void;

    addr = unsafe { avs_log_buffer_addr(adev, (*msg).log.core) };
    if addr.is_null() {
        return -ENXIO;
    }

    unsafe {
        memcpy_fromio(
            &mut layout as *mut _ as *mut c_void,
            addr,
            core::mem::size_of_val(&layout),
        )
    };

    if !unsafe { avs_logging_fw(adev) } {
        /* consume the logs regardless of consumer presence */
    } else {
        buf = unsafe { avs_apl_log_payload_addr(addr) };

        if layout.read_ptr > layout.write_ptr {
            unsafe {
                avs_dump_fw_log(
                    adev,
                    buf.add(layout.read_ptr as usize),
                    avs_apl_log_payload_size(adev) - layout.read_ptr,
                )
            };
            layout.read_ptr = 0;
        }
        unsafe {
            avs_dump_fw_log_wakeup(
                adev,
                buf.add(layout.read_ptr as usize),
                layout.write_ptr - layout.read_ptr,
            )
        };
    }

    unsafe { writel(layout.write_ptr, addr) };
    0
}

unsafe extern "C" fn avs_apl_wait_log_entry(
    adev: *mut avs_dev,
    core: u32,
    layout: *mut avs_apl_log_buffer_layout,
) -> c_int {
    let timeout: c_ulong;
    let addr: *mut c_void;

    addr = unsafe { avs_log_buffer_addr(adev, core) };
    if addr.is_null() {
        return -ENXIO;
    }

    timeout = unsafe { jiffies + msecs_to_jiffies(10) };

    loop {
        unsafe {
            memcpy_fromio(
                layout as *mut c_void,
                addr,
                core::mem::size_of::<avs_apl_log_buffer_layout>(),
            )
        };
        if unsafe { (*layout).read_ptr != (*layout).write_ptr } {
            return 0;
        }
        unsafe { usleep_range(500, 1000) };
        if unsafe { time_after(jiffies, timeout) } {
            break;
        }
    }

    -ETIMEDOUT
}

/* reads log header and tests its type */
unsafe fn avs_apl_is_entry_stackdump(addr: *const c_void) -> u32 {
    unsafe { (readl(addr) >> 30) & 0x1 }
}

unsafe extern "C" fn avs_apl_coredump(adev: *mut avs_dev, msg: *mut avs_notify_msg) -> c_int {
    let mut layout: avs_apl_log_buffer_layout = unsafe { core::mem::zeroed() };
    let mut addr: *mut c_void;
    let buf: *mut c_void;
    let dump_size: usize;
    let mut offset: u32 = 0;
    let dump: *mut u8;
    let pos: *mut u8;

    dump_size = AVS_FW_REGS_SIZE + unsafe { (*msg).ext.coredump.stack_dump_size };
    dump = unsafe { vzalloc(dump_size) };
    if dump.is_null() {
        return -ENOMEM;
    }

    unsafe {
        memcpy_fromio(
            dump as *mut c_void,
            avs_sram_addr(adev, AVS_FW_REGS_WINDOW),
            AVS_FW_REGS_SIZE,
        )
    };

    if unsafe { (*msg).ext.coredump.stack_dump_size } != 0 {
        /* Dump the registers even if an external error prevents gathering the stack. */
        addr = unsafe { avs_log_buffer_addr(adev, (*msg).ext.coredump.core_id) };
        if !addr.is_null() {
            buf = unsafe { avs_apl_log_payload_addr(addr) };
            unsafe {
                memcpy_fromio(
                    &mut layout as *mut _ as *mut c_void,
                    addr,
                    core::mem::size_of_val(&layout),
                )
            };
            if unsafe { avs_apl_is_entry_stackdump(buf.add(layout.read_ptr as usize)) } == 0 {
                let mut lbs_msg: avs_notify_msg = AVS_NOTIFICATION!(LOG_BUFFER_STATUS);

                /*
                 * DSP awaits the remaining logs to be
                 * gathered before dumping stack
                 */
                unsafe {
                    lbs_msg.log.core = (*msg).ext.coredump.core_id;
                    avs_log_buffer_status_locked(adev, &mut lbs_msg);
                }
            }

            pos = unsafe { dump.add(AVS_FW_REGS_SIZE) };
            /* gather the stack */
            loop {
                let mut count: u32;

                if unsafe {
                    avs_apl_wait_log_entry(adev, (*msg).ext.coredump.core_id, &mut layout)
                } != 0
                {
                    break;
                }

                if layout.read_ptr > layout.write_ptr {
                    count = unsafe { avs_apl_log_payload_size(adev) } - layout.read_ptr;
                    unsafe {
                        memcpy_fromio(
                            pos.add(offset as usize) as *mut c_void,
                            buf.add(layout.read_ptr as usize),
                            count as usize,
                        )
                    };
                    layout.read_ptr = 0;
                    offset += count;
                }
                count = layout.write_ptr - layout.read_ptr;
                unsafe {
                    memcpy_fromio(
                        pos.add(offset as usize) as *mut c_void,
                        buf.add(layout.read_ptr as usize),
                        count as usize,
                    )
                };
                offset += count;

                /* update read pointer */
                unsafe { writel(layout.write_ptr, addr) };
                if offset >= unsafe { (*msg).ext.coredump.stack_dump_size as u32 } {
                    break;
                }
            }
        }
    }

    unsafe { dev_coredumpv((*adev).dev, dump, dump_size, GFP_KERNEL) };

    0
}

unsafe fn avs_apl_lp_streaming(adev: *mut avs_dev) -> bool {
    let mut path: *mut avs_path;

    guard_spinlock!(&mut (*adev).path_list_lock);
    /* Any gateway without buffer allocated in LP area disqualifies D0IX. */
    list_for_each_entry!(path, &mut (*adev).path_list, node, {
        let mut ppl: *mut avs_path_pipeline;

        list_for_each_entry!(ppl, &mut (*path).ppl_list, node, {
            let mut mod_: *mut avs_path_module;

            list_for_each_entry!(mod_, &mut (*ppl).mod_list, node, {
                let cfg: *mut avs_tplg_modcfg_ext;

                cfg = (*(*mod_).template).cfg_ext;

                /* only copiers have gateway attributes */
                if !guid_equal(&(*cfg).type_, &AVS_COPIER_MOD_UUID) {
                    continue;
                }
                /* non-gateway copiers do not prevent PG */
                if (*cfg).copier.dma_type == INVALID_OBJECT_ID {
                    continue;
                }

                if !(*mod_).gtw_attrs.lp_buffer_alloc {
                    return false;
                }
            });
        });
    });

    true
}

unsafe extern "C" fn avs_apl_d0ix_toggle(
    adev: *mut avs_dev,
    _tx: *mut avs_ipc_msg,
    wake: bool,
) -> bool {
    /* wake in all cases */
    if wake {
        return true;
    }

    /*
     * If no pipelines are running, allow for d0ix schedule.
     * If all gateways have lp=1, allow for d0ix schedule.
     * If any gateway with lp=0 is allocated, abort scheduling d0ix.
     *
     * Note: for cAVS 1.5+ and 1.8, D0IX is LP-firmware transition,
     * not the power-gating mechanism known from cAVS 2.0.
     */
    unsafe { avs_apl_lp_streaming(adev) }
}

unsafe extern "C" fn avs_apl_set_d0ix(adev: *mut avs_dev, enable: bool) -> c_int {
    let mut streaming: bool = false;
    let ret: c_int;

    if enable {
        /* Either idle or all gateways with lp=1. */
        streaming = unsafe { !list_empty(&(*adev).path_list) };
    }

    ret = unsafe { avs_ipc_set_d0ix(adev, enable, streaming) };
    AVS_IPC_RET!(ret)
}

#[no_mangle]
static avs_apl_dsp_ops: avs_dsp_ops = avs_dsp_ops {
    power: Some(avs_dsp_core_power),
    reset: Some(avs_dsp_core_reset),
    stall: Some(avs_dsp_core_stall),
    dsp_interrupt: Some(avs_apl_dsp_interrupt),
    int_control: Some(avs_dsp_interrupt_control),
    load_basefw: Some(avs_hda_load_basefw),
    load_lib: Some(avs_hda_load_library),
    transfer_mods: Some(avs_hda_transfer_modules),
    log_buffer_offset: Some(avs_skl_log_buffer_offset),
    log_buffer_status: Some(avs_apl_log_buffer_status),
    coredump: Some(avs_apl_coredump),
    d0ix_toggle: Some(avs_apl_d0ix_toggle),
    set_d0ix: Some(avs_apl_set_d0ix),
    // AVS_SET_ENABLE_LOGS_OP(apl)
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
