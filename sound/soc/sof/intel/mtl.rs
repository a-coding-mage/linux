// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright(c) 2022 Intel Corporation
//
// Authors: Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//

/*
 * Hardware interface for audio DSP on Meteorlake.
 *
 * Translated from C implementation source. Header-provided symbols are expected
 * to be supplied by the surrounding crate/bindings.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type bool_t = bool;

extern "C" {
    static sof_hda_common_ops: snd_sof_dsp_ops;

    fn snd_sof_dsp_update_bits_forced(
        sdev: *mut snd_sof_dev,
        bar: u32,
        offset: u32,
        mask: u32,
        value: u32,
    );
    fn snd_sof_dsp_update_bits(
        sdev: *mut snd_sof_dev,
        bar: u32,
        offset: u32,
        mask: u32,
        value: u32,
    );
    fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: u32, offset: u32) -> u32;
    fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: u32, offset: u32, value: u32);
    fn sof_mailbox_write(sdev: *mut snd_sof_dev, offset: u32, src: *const c_void, size: usize);
    fn hda_ipc4_tx_is_busy(sdev: *mut snd_sof_dev) -> bool;
    fn hda_dsp_ipc4_schedule_d0i3_work(hdev: *mut sof_intel_hda_dev, msg: *mut snd_sof_ipc_msg);
    fn str_enable_disable(enable: bool) -> *const c_char;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn usleep_range(min: c_uint, max: c_uint);
    fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_intel_dsp_desc;
    fn hda_sdw_startup(sdev: *mut snd_sof_dev) -> c_int;
    fn hda_sdw_int_enable(sdev: *mut snd_sof_dev, enable: bool);
    fn sof_debug_check_flag(flag: u32) -> bool;
    fn debugfs_create_bool(
        name: *const c_char,
        mode: c_uint,
        parent: *mut dentry,
        value: *mut bool,
    ) -> *mut dentry;
    fn hda_dsp_get_state(sdev: *mut snd_sof_dev, level: *const c_char);
    fn sof_ipc4_intel_dump_telemetry_state(sdev: *mut snd_sof_dev, flags: u32);
    fn kasprintf(gfp: u32, fmt: *const c_char, ...) -> *mut c_char;
    fn snd_sof_dsp_dbg_dump(sdev: *mut snd_sof_dev, msg: *const c_char, flags: u32);
    fn kfree(ptr: *mut c_void);
    fn trace_sof_intel_hda_irq_ipc_check(sdev: *mut snd_sof_dev, irq_status: u32);
    fn snd_sof_ipc_get_reply(sdev: *mut snd_sof_dev);
    fn snd_sof_ipc_reply(sdev: *mut snd_sof_dev, primary: u32);
    fn snd_sof_ipc_msgs_rx(sdev: *mut snd_sof_dev);
    fn hda_dsp_shutdown(sdev: *mut snd_sof_dev);
    fn hda_dsp_ipc4_load_library();
    fn hda_dsp_set_power_state_ipc4();
    fn hda_set_dai_drv_ops(sdev: *mut snd_sof_dev, dsp_ops: *mut snd_sof_dsp_ops);
    fn hda_sdw_check_lcount_common();
    fn hda_sdw_check_wakeen_irq_common();
    fn hda_sdw_process_wakeen_common();
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
    fn snd_sof_dsp_read_poll_timeout(
        sdev: *mut snd_sof_dev,
        bar: u32,
        offset: u32,
        value: *mut u32,
        condition: unsafe extern "C" fn(u32, u32, u32) -> bool,
        mask_or_target: u32,
        interval_us: u32,
        timeout_us: u32,
    ) -> c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut sof_intel_hda_dev,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dspless_mode_selected: bool,
    pub pdata: *mut snd_sof_pdata,
    pub host_box: snd_sof_host_box,
    pub dev: *mut device,
    pub first_boot: bool,
    pub debugfs_root: *mut dentry,
    pub enabled_cores_mask: u32,
    pub dsp_core_ref_count: [c_int; 32],
    pub ipc: *mut snd_sof_ipc,
    pub ipc_lock: spinlock_t,
    pub fw_state: c_int,
    pub private: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_host_box {
    pub offset: u32,
}

#[repr(C)]
pub struct sof_intel_hda_dev {
    pub delayed_ipc_tx_msg: *mut snd_sof_ipc_msg,
    pub desc: *const sof_intel_dsp_desc,
    pub info: hda_sdw_info,
    pub imrboot_supported: bool,
    pub skip_imr_boot: bool,
    pub boot_iteration: c_int,
}

#[repr(C)]
pub struct hda_sdw_info {
    pub handle: *mut c_void,
    pub link_mask: c_uint,
}

#[repr(C)]
pub struct snd_sof_ipc_msg {
    pub msg_data: *mut sof_ipc4_msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_msg {
    pub primary: u32,
    pub extension: u32,
    pub data_size: usize,
    pub data_ptr: *const c_void,
}

#[repr(C)]
pub struct snd_sof_ipc {
    pub msg: snd_sof_ipc_inner_msg,
    pub ops: *const snd_sof_ipc_ops,
}

#[repr(C)]
pub struct snd_sof_ipc_inner_msg {
    pub reply_data: *mut sof_ipc4_msg,
    pub rx_data: *mut sof_ipc4_msg,
}

#[repr(C)]
pub struct snd_sof_ipc_ops {
    pub pm: *const sof_ipc_pm_ops,
}

#[repr(C)]
pub struct sof_ipc_pm_ops {
    pub set_core_state: Option<unsafe extern "C" fn(*mut snd_sof_dev, c_int, bool) -> c_int>,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_debugfs_map {
    pub name: *const c_char,
    pub bar: u32,
    pub offset: u32,
    pub size: u32,
    pub access_type: u32,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub irq_thread: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
    pub send_msg: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_ipc_msg) -> c_int>,
    pub get_mailbox_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub get_window_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>,
    pub debug_map: *const snd_sof_debugfs_map,
    pub debug_map_count: usize,
    pub dbg_dump: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32)>,
    pub ipc_dump: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub pre_fw_run: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub post_fw_run: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub parse_platform_ext_manifest: *mut c_void,
    pub core_get: Option<unsafe extern "C" fn(*mut snd_sof_dev, c_int) -> c_int>,
    pub core_put: Option<unsafe extern "C" fn(*mut snd_sof_dev, c_int) -> c_int>,
    pub set_power_state: *mut c_void,
}

#[repr(C)]
pub struct sof_ipc4_fw_data {
    pub manifest_fw_hdr_offset: u32,
    pub mtrace_type: u32,
    pub fw_context_save: bool,
    pub load_library: *mut c_void,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub cores_num: c_int,
    pub init_core_mask: u32,
    pub host_managed_cores_mask: u32,
    pub ipc_req: u32,
    pub ipc_req_mask: u32,
    pub ipc_ack: u32,
    pub ipc_ack_mask: u32,
    pub ipc_ctl: u32,
    pub rom_status_reg: u32,
    pub rom_init_timeout: u32,
    pub ssp_count: u32,
    pub ssp_base_offset: u32,
    pub sdw_shim_base: u32,
    pub sdw_alh_base: u32,
    pub d0i3_offset: u32,
    pub read_sdw_lcount: *mut c_void,
    pub enable_sdw_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev, bool)>,
    pub check_sdw_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> bool>,
    pub check_sdw_wakeen_irq: *mut c_void,
    pub sdw_process_wakeen: *mut c_void,
    pub check_ipc_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> bool>,
    pub cl_init: Option<unsafe extern "C" fn(*mut snd_sof_dev, c_int, bool) -> c_int>,
    pub power_down_dsp: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub disable_interrupts: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub hw_ip_version: u32,
    pub platform: *const c_char,
}

pub type irqreturn_t = c_uint;

fn BIT(n: u32) -> u32 {
    1u32 << n
}

fn MTL_SRAM_WINDOW_OFFSET(id: u32) -> u32 {
    MTL_SRAM_WINDOW_BASE + id * MTL_SRAM_WINDOW_SIZE
}

fn MTL_HfPWRCTL_WPIOXPG(n: u32) -> u32 {
    1u32 << n
}

fn FSR_TO_STATE_CODE(status: u32) -> u32 {
    status & FSR_STATE_MASK
}

unsafe extern "C" fn poll_eq_mask(value: u32, mask: u32, _target: u32) -> bool {
    (value & mask) == mask
}

unsafe extern "C" fn poll_eq_val(value: u32, mask: u32, target: u32) -> bool {
    (value & mask) == target
}

unsafe extern "C" fn poll_zero_mask(value: u32, mask: u32, _target: u32) -> bool {
    (value & mask) == 0
}

unsafe extern "C" fn poll_state_eq(value: u32, _mask: u32, target: u32) -> bool {
    FSR_TO_STATE_CODE(value) == target
}

static mtl_dsp_debugfs: [snd_sof_debugfs_map; 4] = [
    snd_sof_debugfs_map {
        name: b"hda\0".as_ptr() as *const c_char,
        bar: HDA_DSP_HDA_BAR,
        offset: 0,
        size: 0x4000,
        access_type: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
    snd_sof_debugfs_map {
        name: b"pp\0".as_ptr() as *const c_char,
        bar: HDA_DSP_PP_BAR,
        offset: 0,
        size: 0x1000,
        access_type: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
    snd_sof_debugfs_map {
        name: b"dsp\0".as_ptr() as *const c_char,
        bar: HDA_DSP_BAR,
        offset: 0,
        size: 0x10000,
        access_type: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
    snd_sof_debugfs_map {
        name: b"fw_regs\0".as_ptr() as *const c_char,
        bar: HDA_DSP_BAR,
        offset: MTL_SRAM_WINDOW_OFFSET(0),
        size: 0x1000,
        access_type: SOF_DEBUGFS_ACCESS_D0_ONLY,
    },
];

unsafe extern "C" fn mtl_ipc_host_done(sdev: *mut snd_sof_dev) {
    /*
     * clear busy interrupt to tell dsp controller this interrupt has been accepted,
     * not trigger it again
     */
    snd_sof_dsp_update_bits_forced(
        sdev,
        HDA_DSP_BAR,
        MTL_DSP_REG_HFIPCXTDR,
        MTL_DSP_REG_HFIPCXTDR_BUSY,
        MTL_DSP_REG_HFIPCXTDR_BUSY,
    );
    /*
     * clear busy bit to ack dsp the msg has been processed and send reply msg to dsp
     */
    snd_sof_dsp_update_bits_forced(
        sdev,
        HDA_DSP_BAR,
        MTL_DSP_REG_HFIPCXTDA,
        MTL_DSP_REG_HFIPCXTDA_BUSY,
        0,
    );
}

unsafe extern "C" fn mtl_ipc_dsp_done(sdev: *mut snd_sof_dev) {
    /*
     * set DONE bit - tell DSP we have received the reply msg from DSP, and processed it,
     * don't send more reply to host
     */
    snd_sof_dsp_update_bits_forced(
        sdev,
        HDA_DSP_BAR,
        MTL_DSP_REG_HFIPCXIDA,
        MTL_DSP_REG_HFIPCXIDA_DONE,
        MTL_DSP_REG_HFIPCXIDA_DONE,
    );

    /* unmask Done interrupt */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_BAR,
        MTL_DSP_REG_HFIPCXCTL,
        MTL_DSP_REG_HFIPCXCTL_DONE,
        MTL_DSP_REG_HFIPCXCTL_DONE,
    );
}

/* Check if an IPC IRQ occurred */
#[no_mangle]
pub unsafe extern "C" fn mtl_dsp_check_ipc_irq(sdev: *mut snd_sof_dev) -> bool {
    let irq_status: u32;
    let hfintipptr: u32;

    if (*sdev).dspless_mode_selected {
        return false;
    }

    /* read Interrupt IP Pointer */
    hfintipptr = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_HFINTIPPTR) & MTL_HFINTIPPTR_PTR_MASK;
    irq_status = snd_sof_dsp_read(sdev, HDA_DSP_BAR, hfintipptr.wrapping_add(MTL_DSP_IRQSTS));

    trace_sof_intel_hda_irq_ipc_check(sdev, irq_status);

    if irq_status != U32_MAX && (irq_status & MTL_DSP_IRQSTS_IPC) != 0 {
        return true;
    }

    false
}

/* Check if an SDW IRQ occurred */
unsafe extern "C" fn mtl_dsp_check_sdw_irq(sdev: *mut snd_sof_dev) -> bool {
    let irq_status: u32;
    let hfintipptr: u32;

    /* read Interrupt IP Pointer */
    hfintipptr = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_HFINTIPPTR) & MTL_HFINTIPPTR_PTR_MASK;
    irq_status = snd_sof_dsp_read(sdev, HDA_DSP_BAR, hfintipptr.wrapping_add(MTL_DSP_IRQSTS));

    if irq_status != U32_MAX && (irq_status & MTL_DSP_IRQSTS_SDW) != 0 {
        return true;
    }

    false
}

unsafe extern "C" fn mtl_ipc_send_msg(
    sdev: *mut snd_sof_dev,
    msg: *mut snd_sof_ipc_msg,
) -> c_int {
    let hdev = (*(*sdev).pdata).hw_pdata;
    let msg_data = (*msg).msg_data;

    if hda_ipc4_tx_is_busy(sdev) {
        (*hdev).delayed_ipc_tx_msg = msg;
        return 0;
    }

    (*hdev).delayed_ipc_tx_msg = ptr::null_mut();

    /* send the message via mailbox */
    if (*msg_data).data_size != 0 {
        sof_mailbox_write(
            sdev,
            (*sdev).host_box.offset,
            (*msg_data).data_ptr,
            (*msg_data).data_size,
        );
    }

    snd_sof_dsp_write(sdev, HDA_DSP_BAR, MTL_DSP_REG_HFIPCXIDDY, (*msg_data).extension);
    snd_sof_dsp_write(
        sdev,
        HDA_DSP_BAR,
        MTL_DSP_REG_HFIPCXIDR,
        (*msg_data).primary | MTL_DSP_REG_HFIPCXIDR_BUSY,
    );

    hda_dsp_ipc4_schedule_d0i3_work(hdev, msg);

    0
}

#[no_mangle]
pub unsafe extern "C" fn mtl_enable_ipc_interrupts(sdev: *mut snd_sof_dev) {
    let hda = (*(*sdev).pdata).hw_pdata;
    let chip = (*hda).desc;

    if (*sdev).dspless_mode_selected {
        return;
    }

    /* enable IPC DONE and BUSY interrupts */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_BAR,
        (*chip).ipc_ctl,
        MTL_DSP_REG_HFIPCXCTL_BUSY | MTL_DSP_REG_HFIPCXCTL_DONE,
        MTL_DSP_REG_HFIPCXCTL_BUSY | MTL_DSP_REG_HFIPCXCTL_DONE,
    );
}

#[no_mangle]
pub unsafe extern "C" fn mtl_disable_ipc_interrupts(sdev: *mut snd_sof_dev) {
    let hda = (*(*sdev).pdata).hw_pdata;
    let chip = (*hda).desc;

    if (*sdev).dspless_mode_selected {
        return;
    }

    /* disable IPC DONE and BUSY interrupts */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_BAR,
        (*chip).ipc_ctl,
        MTL_DSP_REG_HFIPCXCTL_BUSY | MTL_DSP_REG_HFIPCXCTL_DONE,
        0,
    );
}

unsafe extern "C" fn mtl_enable_sdw_irq(sdev: *mut snd_sof_dev, enable: bool) {
    let mut hipcie: u32 = 0;
    let mask: u32;
    let val: u32;
    let ret: c_int;

    if (*sdev).dspless_mode_selected {
        return;
    }

    /* Enable/Disable SoundWire interrupt */
    mask = MTL_DSP_REG_HfSNDWIE_IE_MASK;
    if enable {
        val = mask;
    } else {
        val = 0;
    }

    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, MTL_DSP_REG_HfSNDWIE, mask, val);

    /* check if operation was successful */
    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        HDA_DSP_BAR,
        MTL_DSP_REG_HfSNDWIE,
        &mut hipcie,
        poll_eq_val,
        mask,
        val,
        HDA_DSP_REG_POLL_INTERVAL_US,
        HDA_DSP_RESET_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"failed to set SoundWire IPC interrupt %s\n\0".as_ptr() as *const c_char,
            str_enable_disable(enable),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn mtl_enable_interrupts(sdev: *mut snd_sof_dev, enable: bool) -> c_int {
    let hfintipptr: u32;
    let mut irqinten: u32 = 0;
    let mut hipcie: u32 = 0;
    let mut mask: u32;
    let mut val: u32;
    let mut ret: c_int;

    if (*sdev).dspless_mode_selected {
        return 0;
    }

    /* read Interrupt IP Pointer */
    hfintipptr = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_HFINTIPPTR) & MTL_HFINTIPPTR_PTR_MASK;

    /* Enable/Disable Host IPC and SOUNDWIRE */
    mask = MTL_IRQ_INTEN_L_HOST_IPC_MASK | MTL_IRQ_INTEN_L_SOUNDWIRE_MASK;
    if enable {
        val = mask;
    } else {
        val = 0;
    }

    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, hfintipptr, mask, val);

    /* check if operation was successful */
    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        HDA_DSP_BAR,
        hfintipptr,
        &mut irqinten,
        poll_eq_val,
        mask,
        val,
        HDA_DSP_REG_POLL_INTERVAL_US,
        HDA_DSP_RESET_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"failed to %s Host IPC and/or SOUNDWIRE\n\0".as_ptr() as *const c_char,
            str_enable_disable(enable),
        );
        return ret;
    }

    /* Enable/Disable Host IPC interrupt*/
    mask = MTL_DSP_REG_HfHIPCIE_IE_MASK;
    if enable {
        val = mask;
    } else {
        val = 0;
    }

    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, MTL_DSP_REG_HfHIPCIE, mask, val);

    /* check if operation was successful */
    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        HDA_DSP_BAR,
        MTL_DSP_REG_HfHIPCIE,
        &mut hipcie,
        poll_eq_val,
        mask,
        val,
        HDA_DSP_REG_POLL_INTERVAL_US,
        HDA_DSP_RESET_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"failed to set Host IPC interrupt %s\n\0".as_ptr() as *const c_char,
            str_enable_disable(enable),
        );
        return ret;
    }

    ret
}

unsafe extern "C" fn mtl_dsp_is_enabled(sdev: *mut snd_sof_dev) -> bool {
    let val: c_int;

    val = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_HFDSSCS) as c_int;
    if (val as u32 & MTL_HFDSSCS_CPA_MASK) != 0 {
        return true;
    }

    false
}

/* pre fw run operations */
unsafe extern "C" fn mtl_dsp_pre_fw_run(sdev: *mut snd_sof_dev) -> c_int {
    let hdev = (*(*sdev).pdata).hw_pdata;
    let mut dsphfpwrsts: u32 = 0;
    let mut dsphfdsscs: u32 = 0;
    let cpa: u32;
    let pgs: u32;
    let mut ret: c_int;
    let dsppwrctl: u32;
    let dsppwrsts: u32;
    let chip: *const sof_intel_dsp_desc;

    /* Power down the DSP if it is left enabled to ensure clean boot state */
    if mtl_dsp_is_enabled(sdev) {
        dev_dbg((*sdev).dev, b"powering down DSP first\n\0".as_ptr() as *const c_char);

        ret = mtl_power_down_dsp(sdev);
        if ret < 0 {
            dev_warn(
                (*sdev).dev,
                b"%s: failed to power down already-enabled DSP\n\0".as_ptr() as *const c_char,
                b"mtl_dsp_pre_fw_run\0".as_ptr() as *const c_char,
            );
            /* Continue anyway to attempt recovery */
        }
    }

    chip = get_chip_info((*sdev).pdata);
    if (*chip).hw_ip_version > SOF_INTEL_ACE_2_0 {
        dsppwrctl = PTL_HFPWRCTL2;
        dsppwrsts = PTL_HFPWRSTS2;
    } else {
        dsppwrctl = MTL_HFPWRCTL;
        dsppwrsts = MTL_HFPWRSTS;
    }

    /* Set the DSP subsystem power on */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_BAR,
        MTL_HFDSSCS,
        MTL_HFDSSCS_SPA_MASK,
        MTL_HFDSSCS_SPA_MASK,
    );

    /* Wait for unstable CPA read (1 then 0 then 1) just after setting SPA bit */
    usleep_range(1000, 1010);

    /* poll with timeout to check if operation successful */
    cpa = MTL_HFDSSCS_CPA_MASK;
    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        HDA_DSP_BAR,
        MTL_HFDSSCS,
        &mut dsphfdsscs,
        poll_eq_mask,
        cpa,
        HDA_DSP_REG_POLL_INTERVAL_US,
        HDA_DSP_RESET_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err((*sdev).dev, b"failed to enable DSP subsystem\n\0".as_ptr() as *const c_char);
        return ret;
    }

    /* Power up gated-DSP-0 domain in order to access the DSP shim register block. */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_BAR,
        dsppwrctl,
        MTL_HFPWRCTL_WPDSPHPXPG,
        MTL_HFPWRCTL_WPDSPHPXPG,
    );

    usleep_range(1000, 1010);

    /* poll with timeout to check if operation successful */
    pgs = MTL_HFPWRSTS_DSPHPXPGS_MASK;
    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        HDA_DSP_BAR,
        dsppwrsts,
        &mut dsphfpwrsts,
        poll_eq_mask,
        pgs,
        HDA_DSP_REG_POLL_INTERVAL_US,
        HDA_DSP_RESET_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err((*sdev).dev, b"failed to power up gated DSP domain\n\0".as_ptr() as *const c_char);
    }

    /* if SoundWire is used, make sure it is not power-gated */
    if !(*hdev).info.handle.is_null() && (*hdev).info.link_mask > 0 {
        snd_sof_dsp_update_bits(
            sdev,
            HDA_DSP_BAR,
            MTL_HFPWRCTL,
            MTL_HfPWRCTL_WPIOXPG(1),
            MTL_HfPWRCTL_WPIOXPG(1),
        );
    }

    ret
}

unsafe extern "C" fn mtl_dsp_post_fw_run(sdev: *mut snd_sof_dev) -> c_int {
    let ret: c_int;

    if (*sdev).first_boot {
        let hdev = (*(*sdev).pdata).hw_pdata;

        ret = hda_sdw_startup(sdev);
        if ret < 0 {
            dev_err((*sdev).dev, b"could not startup SoundWire links\n\0".as_ptr() as *const c_char);
            return ret;
        }

        /* Check if IMR boot is usable */
        if !sof_debug_check_flag(SOF_DBG_IGNORE_D3_PERSISTENT) {
            (*hdev).imrboot_supported = true;
            debugfs_create_bool(
                b"skip_imr_boot\0".as_ptr() as *const c_char,
                0o644,
                (*sdev).debugfs_root,
                &mut (*hdev).skip_imr_boot,
            );
        }
    }

    hda_sdw_int_enable(sdev, true);
    0
}

unsafe extern "C" fn mtl_dsp_dump(sdev: *mut snd_sof_dev, flags: u32) {
    let level = if (flags & SOF_DBG_DUMP_OPTIONAL) != 0 {
        KERN_DEBUG
    } else {
        KERN_ERR
    };
    let fwsts: u32;
    let fwlec: u32;

    hda_dsp_get_state(sdev, level);
    fwsts = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_DSP_ROM_STS);
    fwlec = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_DSP_ROM_ERROR);

    if fwsts != 0xffffffff {
        dev_err(
            (*sdev).dev,
            b"Firmware state: %#x, status/error code: %#x\n\0".as_ptr() as *const c_char,
            fwsts,
            fwlec,
        );
    }

    sof_ipc4_intel_dump_telemetry_state(sdev, flags);
}

unsafe extern "C" fn mtl_dsp_primary_core_is_enabled(sdev: *mut snd_sof_dev) -> bool {
    let val: c_int;

    val = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_DSP2CXCTL_PRIMARY_CORE) as c_int;
    if val as u32 != U32_MAX && (val as u32 & MTL_DSP2CXCTL_PRIMARY_CORE_CPA_MASK) != 0 {
        return true;
    }

    false
}

unsafe extern "C" fn mtl_dsp_core_power_up(sdev: *mut snd_sof_dev, core: c_int) -> c_int {
    let cpa: c_uint;
    let mut dspcxctl: u32 = 0;
    let ret: c_int;

    /* Only the primary core can be powered up by the host */
    if core != SOF_DSP_PRIMARY_CORE || mtl_dsp_primary_core_is_enabled(sdev) {
        return 0;
    }

    /* Program the owner of the IP & shim registers (10: Host CPU) */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_BAR,
        MTL_DSP2CXCTL_PRIMARY_CORE,
        MTL_DSP2CXCTL_PRIMARY_CORE_OSEL,
        0x2 << MTL_DSP2CXCTL_PRIMARY_CORE_OSEL_SHIFT,
    );

    /* enable SPA bit */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_BAR,
        MTL_DSP2CXCTL_PRIMARY_CORE,
        MTL_DSP2CXCTL_PRIMARY_CORE_SPA_MASK,
        MTL_DSP2CXCTL_PRIMARY_CORE_SPA_MASK,
    );

    /* Wait for unstable CPA read (1 then 0 then 1) just after setting SPA bit */
    usleep_range(1000, 1010);

    /* poll with timeout to check if operation successful */
    cpa = MTL_DSP2CXCTL_PRIMARY_CORE_CPA_MASK;
    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        HDA_DSP_BAR,
        MTL_DSP2CXCTL_PRIMARY_CORE,
        &mut dspcxctl,
        poll_eq_mask,
        cpa,
        HDA_DSP_REG_POLL_INTERVAL_US,
        HDA_DSP_RESET_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"%s: timeout on MTL_DSP2CXCTL_PRIMARY_CORE read\n\0".as_ptr() as *const c_char,
            b"mtl_dsp_core_power_up\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    /* set primary core mask and refcount to 1 */
    (*sdev).enabled_cores_mask = BIT(SOF_DSP_PRIMARY_CORE as u32);
    (*sdev).dsp_core_ref_count[SOF_DSP_PRIMARY_CORE as usize] = 1;

    0
}

unsafe extern "C" fn mtl_dsp_core_power_down(sdev: *mut snd_sof_dev, core: c_int) -> c_int {
    let mut dspcxctl: u32 = 0;
    let ret: c_int;

    /* Only the primary core can be powered down by the host */
    if core != SOF_DSP_PRIMARY_CORE || !mtl_dsp_primary_core_is_enabled(sdev) {
        return 0;
    }

    /* disable SPA bit */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_BAR,
        MTL_DSP2CXCTL_PRIMARY_CORE,
        MTL_DSP2CXCTL_PRIMARY_CORE_SPA_MASK,
        0,
    );

    /* Wait for unstable CPA read (0 then 1 then 0) just after setting SPA bit */
    usleep_range(1000, 1010);

    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        HDA_DSP_BAR,
        MTL_DSP2CXCTL_PRIMARY_CORE,
        &mut dspcxctl,
        poll_zero_mask,
        MTL_DSP2CXCTL_PRIMARY_CORE_CPA_MASK,
        HDA_DSP_REG_POLL_INTERVAL_US,
        HDA_DSP_PD_TIMEOUT * USEC_PER_MSEC,
    );
    if ret < 0 {
        dev_err((*sdev).dev, b"failed to power down primary core\n\0".as_ptr() as *const c_char);
        return ret;
    }

    (*sdev).enabled_cores_mask = 0;
    (*sdev).dsp_core_ref_count[SOF_DSP_PRIMARY_CORE as usize] = 0;

    0
}

#[no_mangle]
pub unsafe extern "C" fn mtl_power_down_dsp(sdev: *mut snd_sof_dev) -> c_int {
    let mut dsphfdsscs: u32;
    let cpa: u32;
    let ret: c_int;

    /* first power down core */
    ret = mtl_dsp_core_power_down(sdev, SOF_DSP_PRIMARY_CORE);
    if ret != 0 {
        dev_err(
            (*sdev).dev,
            b"mtl dsp power down error, %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    /* Set the DSP subsystem power down */
    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, MTL_HFDSSCS, MTL_HFDSSCS_SPA_MASK, 0);

    /* Wait for unstable CPA read (0 then 1 then 0) just after setting SPA bit */
    usleep_range(1000, 1010);

    /* poll with timeout to check if operation successful */
    cpa = MTL_HFDSSCS_CPA_MASK;
    dsphfdsscs = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_HFDSSCS);
    snd_sof_dsp_read_poll_timeout(
        sdev,
        HDA_DSP_BAR,
        MTL_HFDSSCS,
        &mut dsphfdsscs,
        poll_zero_mask,
        cpa,
        HDA_DSP_REG_POLL_INTERVAL_US,
        HDA_DSP_RESET_TIMEOUT_US,
    )
}

#[no_mangle]
pub unsafe extern "C" fn mtl_dsp_cl_init(
    sdev: *mut snd_sof_dev,
    stream_tag: c_int,
    imr_boot: bool,
) -> c_int {
    let hda = (*(*sdev).pdata).hw_pdata;
    let chip = (*hda).desc;
    let mut status: c_uint = 0;
    let target_status: c_uint;
    let mut ipc_hdr: u32;
    let mut flags: u32;
    let dump_msg: *mut c_char;
    let mut ret: c_int;

    /* step 1: purge FW request */
    ipc_hdr = (*chip).ipc_req_mask | HDA_DSP_ROM_IPC_CONTROL;
    if !imr_boot {
        ipc_hdr |= HDA_DSP_ROM_IPC_PURGE_FW | (((stream_tag - 1) as u32) << 9);
    }

    snd_sof_dsp_write(sdev, HDA_DSP_BAR, (*chip).ipc_req, ipc_hdr);

    /* step 2: power up primary core */
    ret = mtl_dsp_core_power_up(sdev, SOF_DSP_PRIMARY_CORE);
    if ret < 0 {
        if (*hda).boot_iteration == HDA_FW_BOOT_ATTEMPTS {
            dev_err((*sdev).dev, b"dsp core 0/1 power up failed\n\0".as_ptr() as *const c_char);
        }
        goto_err(sdev, hda, ret);
        return ret;
    }

    dev_dbg((*sdev).dev, b"Primary core power up successful\n\0".as_ptr() as *const c_char);

    /* step 3: wait for IPC DONE bit from ROM */
    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        HDA_DSP_BAR,
        (*chip).ipc_ack,
        &mut status,
        poll_eq_mask,
        (*chip).ipc_ack_mask,
        HDA_DSP_REG_POLL_INTERVAL_US,
        HDA_DSP_INIT_TIMEOUT_US,
    );
    if ret < 0 {
        if (*hda).boot_iteration == HDA_FW_BOOT_ATTEMPTS {
            dev_err((*sdev).dev, b"timeout waiting for purge IPC done\n\0".as_ptr() as *const c_char);
        }
        goto_err(sdev, hda, ret);
        return ret;
    }

    /* set DONE bit to clear the reply IPC message */
    snd_sof_dsp_update_bits_forced(
        sdev,
        HDA_DSP_BAR,
        (*chip).ipc_ack,
        (*chip).ipc_ack_mask,
        (*chip).ipc_ack_mask,
    );

    /* step 4: enable interrupts */
    ret = mtl_enable_interrupts(sdev, true);
    if ret < 0 {
        if (*hda).boot_iteration == HDA_FW_BOOT_ATTEMPTS {
            dev_err(
                (*sdev).dev,
                b"%s: failed to enable interrupts\n\0".as_ptr() as *const c_char,
                b"mtl_dsp_cl_init\0".as_ptr() as *const c_char,
            );
        }
        goto_err(sdev, hda, ret);
        return ret;
    }

    mtl_enable_ipc_interrupts(sdev);

    if (*chip).rom_status_reg == MTL_DSP_ROM_STS {
        /*
         * Workaround: when the ROM status register is pointing to
         * the SRAM window (MTL_DSP_ROM_STS) the platform cannot catch
         * ROM_INIT_DONE because of a very short timing window.
         * Follow the recommendations and skip target state waiting.
         */
        return 0;
    }

    /*
     * step 7:
     * - Cold/Full boot: wait for ROM init to proceed to download the firmware
     * - IMR boot: wait for ROM firmware entered (firmware booted up from IMR)
     */
    if imr_boot {
        target_status = FSR_STATE_FW_ENTERED;
    } else {
        target_status = FSR_STATE_INIT_DONE;
    }

    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        HDA_DSP_BAR,
        (*chip).rom_status_reg,
        &mut status,
        poll_state_eq,
        target_status,
        HDA_DSP_REG_POLL_INTERVAL_US,
        (*chip).rom_init_timeout * USEC_PER_MSEC,
    );

    if ret == 0 {
        return 0;
    }

    if (*hda).boot_iteration == HDA_FW_BOOT_ATTEMPTS {
        dev_err(
            (*sdev).dev,
            b"%s: timeout with rom_status_reg (%#x) read\n\0".as_ptr() as *const c_char,
            b"mtl_dsp_cl_init\0".as_ptr() as *const c_char,
            (*chip).rom_status_reg,
        );
    }

    flags = SOF_DBG_DUMP_PCI | SOF_DBG_DUMP_MBOX | SOF_DBG_DUMP_OPTIONAL;

    /* after max boot attempts make sure that the dump is printed */
    if (*hda).boot_iteration == HDA_FW_BOOT_ATTEMPTS {
        flags &= !SOF_DBG_DUMP_OPTIONAL;
    }

    dump_msg = kasprintf(
        GFP_KERNEL,
        b"Boot iteration failed: %d/%d\0".as_ptr() as *const c_char,
        (*hda).boot_iteration,
        HDA_FW_BOOT_ATTEMPTS,
    );
    snd_sof_dsp_dbg_dump(sdev, dump_msg, flags);
    mtl_enable_interrupts(sdev, false);
    mtl_dsp_core_power_down(sdev, SOF_DSP_PRIMARY_CORE);

    kfree(dump_msg as *mut c_void);
    ret
}

unsafe fn goto_err(sdev: *mut snd_sof_dev, hda: *mut sof_intel_hda_dev, ret: c_int) {
    let mut flags = SOF_DBG_DUMP_PCI | SOF_DBG_DUMP_MBOX | SOF_DBG_DUMP_OPTIONAL;

    /* after max boot attempts make sure that the dump is printed */
    if (*hda).boot_iteration == HDA_FW_BOOT_ATTEMPTS {
        flags &= !SOF_DBG_DUMP_OPTIONAL;
    }

    let dump_msg = kasprintf(
        GFP_KERNEL,
        b"Boot iteration failed: %d/%d\0".as_ptr() as *const c_char,
        (*hda).boot_iteration,
        HDA_FW_BOOT_ATTEMPTS,
    );
    snd_sof_dsp_dbg_dump(sdev, dump_msg, flags);
    mtl_enable_interrupts(sdev, false);
    mtl_dsp_core_power_down(sdev, SOF_DSP_PRIMARY_CORE);

    kfree(dump_msg as *mut c_void);
    let _ = ret;
}

unsafe extern "C" fn mtl_ipc_irq_thread(_irq: c_int, context: *mut c_void) -> irqreturn_t {
    let mut notification_data = sof_ipc4_msg {
        primary: 0,
        extension: 0,
        data_size: 0,
        data_ptr: ptr::null(),
    };
    let sdev = context as *mut snd_sof_dev;
    let mut ack_received = false;
    let mut ipc_irq = false;
    let hipcida: u32;
    let hipctdr: u32;

    hipcida = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_DSP_REG_HFIPCXIDA);
    hipctdr = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_DSP_REG_HFIPCXTDR);

    /* reply message from DSP */
    if (hipcida & MTL_DSP_REG_HFIPCXIDA_DONE) != 0 {
        /* DSP received the message */
        snd_sof_dsp_update_bits(
            sdev,
            HDA_DSP_BAR,
            MTL_DSP_REG_HFIPCXCTL,
            MTL_DSP_REG_HFIPCXCTL_DONE,
            0,
        );

        mtl_ipc_dsp_done(sdev);

        ipc_irq = true;
        ack_received = true;
    }

    if (hipctdr & MTL_DSP_REG_HFIPCXTDR_BUSY) != 0 {
        /* Message from DSP (reply or notification) */
        let extension = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_DSP_REG_HFIPCXTDDY);
        let primary = hipctdr & MTL_DSP_REG_HFIPCXTDR_MSG_MASK;

        /*
         * ACE fw sends a new fw ipc message to host to
         * notify the status of the last host ipc message
         */
        if (primary & SOF_IPC4_MSG_DIR_MASK) != 0 {
            /* Reply received */
            if (*sdev).fw_state == SOF_FW_BOOT_COMPLETE {
                let data = (*(*sdev).ipc).msg.reply_data;

                (*data).primary = primary;
                (*data).extension = extension;

                /* guard(spinlock_irq)(&sdev->ipc_lock); */
                snd_sof_ipc_get_reply(sdev);
                mtl_ipc_host_done(sdev);
                snd_sof_ipc_reply(sdev, (*data).primary);
            } else {
                dev_dbg_ratelimited(
                    (*sdev).dev,
                    b"IPC reply before FW_READY: %#x|%#x\n\0".as_ptr() as *const c_char,
                    primary,
                    extension,
                );
            }
        } else {
            /* Notification received */
            notification_data.primary = primary;
            notification_data.extension = extension;

            (*(*sdev).ipc).msg.rx_data = &mut notification_data;
            snd_sof_ipc_msgs_rx(sdev);
            (*(*sdev).ipc).msg.rx_data = ptr::null_mut();

            mtl_ipc_host_done(sdev);
        }

        ipc_irq = true;
    }

    if !ipc_irq {
        /* This interrupt is not shared so no need to return IRQ_NONE. */
        dev_dbg_ratelimited(
            (*sdev).dev,
            b"nothing to do in IPC IRQ thread\n\0".as_ptr() as *const c_char,
        );
    }

    if ack_received {
        let hdev = (*(*sdev).pdata).hw_pdata;

        if !(*hdev).delayed_ipc_tx_msg.is_null() {
            mtl_ipc_send_msg(sdev, (*hdev).delayed_ipc_tx_msg);
        }
    }

    IRQ_HANDLED
}

unsafe extern "C" fn mtl_dsp_ipc_get_mailbox_offset(_sdev: *mut snd_sof_dev) -> c_int {
    MTL_DSP_MBOX_UPLINK_OFFSET as c_int
}

unsafe extern "C" fn mtl_dsp_ipc_get_window_offset(_sdev: *mut snd_sof_dev, id: u32) -> c_int {
    MTL_SRAM_WINDOW_OFFSET(id) as c_int
}

unsafe extern "C" fn mtl_ipc_dump(sdev: *mut snd_sof_dev) {
    let hipcidr: u32;
    let hipcidd: u32;
    let hipcida: u32;
    let hipctdr: u32;
    let hipctdd: u32;
    let hipctda: u32;
    let hipcctl: u32;

    hipcidr = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_DSP_REG_HFIPCXIDR);
    hipcidd = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_DSP_REG_HFIPCXIDDY);
    hipcida = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_DSP_REG_HFIPCXIDA);
    hipctdr = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_DSP_REG_HFIPCXTDR);
    hipctdd = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_DSP_REG_HFIPCXTDDY);
    hipctda = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_DSP_REG_HFIPCXTDA);
    hipcctl = snd_sof_dsp_read(sdev, HDA_DSP_BAR, MTL_DSP_REG_HFIPCXCTL);

    dev_err(
        (*sdev).dev,
        b"Host IPC initiator: %#x|%#x|%#x, target: %#x|%#x|%#x, ctl: %#x\n\0".as_ptr()
            as *const c_char,
        hipcidr,
        hipcidd,
        hipcida,
        hipctdr,
        hipctdd,
        hipctda,
        hipcctl,
    );
}

unsafe extern "C" fn mtl_dsp_disable_interrupts(sdev: *mut snd_sof_dev) -> c_int {
    mtl_enable_sdw_irq(sdev, false);
    mtl_disable_ipc_interrupts(sdev);
    mtl_enable_interrupts(sdev, false)
}

unsafe extern "C" fn mtl_dsp_core_get(sdev: *mut snd_sof_dev, core: c_int) -> c_int {
    let pm_ops = (*(*(*sdev).ipc).ops).pm;

    if core == SOF_DSP_PRIMARY_CORE {
        return mtl_dsp_core_power_up(sdev, SOF_DSP_PRIMARY_CORE);
    }

    if let Some(set_core_state) = (*pm_ops).set_core_state {
        return set_core_state(sdev, core, true);
    }

    0
}

unsafe extern "C" fn mtl_dsp_core_put(sdev: *mut snd_sof_dev, core: c_int) -> c_int {
    let pm_ops = (*(*(*sdev).ipc).ops).pm;
    let ret: c_int;

    if let Some(set_core_state) = (*pm_ops).set_core_state {
        ret = set_core_state(sdev, core, false);
        if ret < 0 {
            return ret;
        }
    }

    if core == SOF_DSP_PRIMARY_CORE {
        return mtl_dsp_core_power_down(sdev, SOF_DSP_PRIMARY_CORE);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn sof_mtl_set_ops(
    sdev: *mut snd_sof_dev,
    dsp_ops: *mut snd_sof_dsp_ops,
) -> c_int {
    let ipc4_data: *mut sof_ipc4_fw_data;

    /* common defaults */
    memcpy(
        dsp_ops as *mut c_void,
        &sof_hda_common_ops as *const _ as *const c_void,
        size_of::<snd_sof_dsp_ops>(),
    );

    /* shutdown */
    (*dsp_ops).shutdown = Some(hda_dsp_shutdown);

    /* doorbell */
    (*dsp_ops).irq_thread = Some(mtl_ipc_irq_thread);

    /* ipc */
    (*dsp_ops).send_msg = Some(mtl_ipc_send_msg);
    (*dsp_ops).get_mailbox_offset = Some(mtl_dsp_ipc_get_mailbox_offset);
    (*dsp_ops).get_window_offset = Some(mtl_dsp_ipc_get_window_offset);

    /* debug */
    (*dsp_ops).debug_map = mtl_dsp_debugfs.as_ptr();
    (*dsp_ops).debug_map_count = mtl_dsp_debugfs.len();
    (*dsp_ops).dbg_dump = Some(mtl_dsp_dump);
    (*dsp_ops).ipc_dump = Some(mtl_ipc_dump);

    /* pre/post fw run */
    (*dsp_ops).pre_fw_run = Some(mtl_dsp_pre_fw_run);
    (*dsp_ops).post_fw_run = Some(mtl_dsp_post_fw_run);

    /* parse platform specific extended manifest */
    (*dsp_ops).parse_platform_ext_manifest = ptr::null_mut();

    /* dsp core get/put */
    (*dsp_ops).core_get = Some(mtl_dsp_core_get);
    (*dsp_ops).core_put = Some(mtl_dsp_core_put);

    (*sdev).private = kzalloc(size_of::<sof_ipc4_fw_data>(), GFP_KERNEL);
    if (*sdev).private.is_null() {
        return -ENOMEM;
    }

    ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    (*ipc4_data).manifest_fw_hdr_offset = SOF_MAN4_FW_HDR_OFFSET;

    (*ipc4_data).mtrace_type = SOF_IPC4_MTRACE_INTEL_CAVS_2;

    (*ipc4_data).fw_context_save = true;

    /* External library loading support */
    (*ipc4_data).load_library = hda_dsp_ipc4_load_library as *mut c_void;

    (*dsp_ops).set_power_state = hda_dsp_set_power_state_ipc4 as *mut c_void;

    /* set DAI ops */
    hda_set_dai_drv_ops(sdev, dsp_ops);

    0
}

#[no_mangle]
pub static mtl_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    cores_num: 3,
    init_core_mask: BIT(0),
    host_managed_cores_mask: BIT(0),
    ipc_req: MTL_DSP_REG_HFIPCXIDR,
    ipc_req_mask: MTL_DSP_REG_HFIPCXIDR_BUSY,
    ipc_ack: MTL_DSP_REG_HFIPCXIDA,
    ipc_ack_mask: MTL_DSP_REG_HFIPCXIDA_DONE,
    ipc_ctl: MTL_DSP_REG_HFIPCXCTL,
    rom_status_reg: MTL_DSP_REG_HFFLGPXQWY,
    rom_init_timeout: 300,
    ssp_count: MTL_SSP_COUNT,
    ssp_base_offset: CNL_SSP_BASE_OFFSET,
    sdw_shim_base: SDW_SHIM_BASE_ACE,
    sdw_alh_base: SDW_ALH_BASE_ACE,
    d0i3_offset: MTL_HDA_VS_D0I3C,
    read_sdw_lcount: hda_sdw_check_lcount_common as *mut c_void,
    enable_sdw_irq: Some(mtl_enable_sdw_irq),
    check_sdw_irq: Some(mtl_dsp_check_sdw_irq),
    check_sdw_wakeen_irq: hda_sdw_check_wakeen_irq_common as *mut c_void,
    sdw_process_wakeen: hda_sdw_process_wakeen_common as *mut c_void,
    check_ipc_irq: Some(mtl_dsp_check_ipc_irq),
    cl_init: Some(mtl_dsp_cl_init),
    power_down_dsp: Some(mtl_power_down_dsp),
    disable_interrupts: Some(mtl_dsp_disable_interrupts),
    hw_ip_version: SOF_INTEL_ACE_1_0,
    platform: b"mtl\0".as_ptr() as *const c_char,
};

#[no_mangle]
pub static arl_s_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    cores_num: 2,
    init_core_mask: BIT(0),
    host_managed_cores_mask: BIT(0),
    ipc_req: MTL_DSP_REG_HFIPCXIDR,
    ipc_req_mask: MTL_DSP_REG_HFIPCXIDR_BUSY,
    ipc_ack: MTL_DSP_REG_HFIPCXIDA,
    ipc_ack_mask: MTL_DSP_REG_HFIPCXIDA_DONE,
    ipc_ctl: MTL_DSP_REG_HFIPCXCTL,
    rom_status_reg: MTL_DSP_REG_HFFLGPXQWY,
    rom_init_timeout: 300,
    ssp_count: MTL_SSP_COUNT,
    ssp_base_offset: CNL_SSP_BASE_OFFSET,
    sdw_shim_base: SDW_SHIM_BASE_ACE,
    sdw_alh_base: SDW_ALH_BASE_ACE,
    d0i3_offset: MTL_HDA_VS_D0I3C,
    read_sdw_lcount: hda_sdw_check_lcount_common as *mut c_void,
    enable_sdw_irq: Some(mtl_enable_sdw_irq),
    check_sdw_irq: Some(mtl_dsp_check_sdw_irq),
    check_sdw_wakeen_irq: hda_sdw_check_wakeen_irq_common as *mut c_void,
    sdw_process_wakeen: hda_sdw_process_wakeen_common as *mut c_void,
    check_ipc_irq: Some(mtl_dsp_check_ipc_irq),
    cl_init: Some(mtl_dsp_cl_init),
    power_down_dsp: Some(mtl_power_down_dsp),
    disable_interrupts: Some(mtl_dsp_disable_interrupts),
    hw_ip_version: SOF_INTEL_ACE_1_0,
    platform: b"arl\0".as_ptr() as *const c_char,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
