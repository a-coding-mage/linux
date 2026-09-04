// SPDX-License-Identifier: GPL-2.0
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.

// Kernel headers (dependencies provided by integration)
use core::ffi::c_void;
use core::ptr::{self, null, null_mut};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::mem::{self, size_of, zeroed};

// Stream disable request timeout during USB device disconnect
const DEV_RELEASE_WAIT_TIMEOUT: u32 = 10000; // in ms

// Data interval calculation parameters
const BUS_INTERVAL_FULL_SPEED: u32 = 1000; // in us
const BUS_INTERVAL_HIGHSPEED_AND_ABOVE: u32 = 125; // in us
const MAX_BINTERVAL_ISOC_EP: u32 = 16;

const QMI_STREAM_REQ_CARD_NUM_MASK: u32 = 0xffff0000;
const QMI_STREAM_REQ_DEV_NUM_MASK: u32 = 0xff00;
const QMI_STREAM_REQ_DIRECTION: u32 = 0xff;

// iommu resource parameters and management
fn prepend_sid_to_iova(iova: u64, sid: u32) -> u64 {
    iova | ((sid as u64) << 32)
}

fn iova_mask(iova: u64) -> u64 {
    iova & 0xFFFFFFFF
}

const IOVA_BASE: u64 = 0x1000;
const IOVA_XFER_RING_BASE: u64 = IOVA_BASE + 0x1000 * (32 + 1); // PAGE_SIZE * (SNDRV_CARDS + 1)
const IOVA_XFER_BUF_BASE: u64 = IOVA_XFER_RING_BASE + 0x1000 * 32 * 32; // PAGE_SIZE * SNDRV_CARDS * 32
const IOVA_XFER_RING_MAX: u64 = IOVA_XFER_BUF_BASE - 0x1000;
const IOVA_XFER_BUF_MAX: u64 = 0xfffff000 - 0x1000;

const MAX_XFER_BUFF_LEN: usize = 24 * 0x1000; // 24 * PAGE_SIZE

const SNDRV_CARDS: usize = 32;
const PAGE_SIZE: u64 = 0x1000;

#[repr(C)]
struct ListHead {
    next: *mut ListHead,
    prev: *mut ListHead,
}

#[repr(C)]
struct IovaInfo {
    list: ListHead,
    start_iova: usize,
    size: usize,
    in_use: bool,
}

#[repr(C)]
struct IntfInfo {
    // IOMMU ring/buffer mapping information
    data_xfer_ring_va: usize,
    data_xfer_ring_size: usize,
    sync_xfer_ring_va: usize,
    sync_xfer_ring_size: usize,
    xfer_buf_iova: u64,
    xfer_buf_size: usize,
    xfer_buf_dma: u64,
    xfer_buf_cpu: *mut u8,

    // USB endpoint information
    data_ep_pipe: u32,
    sync_ep_pipe: u32,
    data_ep_idx: u32,
    sync_ep_idx: u32,

    intf_num: u8,
    pcm_card_num: u8,
    pcm_dev_num: u8,
    direction: u8,
    in_use: bool,
}

#[repr(C)]
struct UaudioQmiDev {
    dev: *mut c_void,
    data: *mut c_void,
    auxdev: *mut c_void,

    xfer_ring_list: ListHead,
    xfer_ring_iova_size: usize,
    curr_xfer_ring_iova: usize,
    xfer_buf_list: ListHead,
    xfer_buf_iova_size: usize,
    curr_xfer_buf_iova: usize,

    card_slot: usize,
    er_mapped: bool,
}

#[repr(C)]
struct UaudioDev {
    udev: *mut c_void,
    ctrl_intf: *mut c_void,
    usb_core_id: u32,
    in_use: AtomicUsize,
    kref: Kref,
    disconnect_wq: WaitQueueHead,

    num_intf: i32,
    info: *mut IntfInfo,
    chip: *mut c_void,

    sb: *mut c_void,
    sdev: *mut c_void,
}

#[repr(C)]
struct Kref {
    refcount: AtomicUsize,
}

#[repr(C)]
struct WaitQueueHead {
    _data: u64,
}

static mut UADEV: [UaudioDev; SNDRV_CARDS] = [UaudioDev {
    udev: null_mut(),
    ctrl_intf: null_mut(),
    usb_core_id: 0,
    in_use: AtomicUsize::new(0),
    kref: Kref { refcount: AtomicUsize::new(0) },
    disconnect_wq: WaitQueueHead { _data: 0 },
    num_intf: 0,
    info: null_mut(),
    chip: null_mut(),
    sb: null_mut(),
    sdev: null_mut(),
}; SNDRV_CARDS];

static mut UAUDIO_QDEV: *mut UaudioQmiDev = null_mut();
static mut UAUDIO_SVC: *mut UaudioQmiSvc = null_mut();
static QDEV_MUTEX: core::sync::atomic::AtomicBool = AtomicBool::new(false);

#[repr(C)]
struct UaudioQmiSvc {
    uaudio_svc_hdl: *mut c_void,
    client_sq: QrtrSockaddr,
    client_connected: bool,
}

#[repr(C)]
struct QrtrSockaddr {
    sq_node: u32,
    sq_port: u32,
    sq_family: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum MemType {
    MemEventRing,
    MemXferRing,
    MemXferBuf,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum UsbQmiAudioFormat {
    UsbQmiPcmFormatS8 = 0,
    UsbQmiPcmFormatU8,
    UsbQmiPcmFormatS16Le,
    UsbQmiPcmFormatS16Be,
    UsbQmiPcmFormatU16Le,
    UsbQmiPcmFormatU16Be,
    UsbQmiPcmFormatS24Le,
    UsbQmiPcmFormatS24Be,
    UsbQmiPcmFormatU24Le,
    UsbQmiPcmFormatU24Be,
    UsbQmiPcmFormatS243Le,
    UsbQmiPcmFormatS243Be,
    UsbQmiPcmFormatU243Le,
    UsbQmiPcmFormatU243Be,
    UsbQmiPcmFormatS32Le,
    UsbQmiPcmFormatS32Be,
    UsbQmiPcmFormatU32Le,
    UsbQmiPcmFormatU32Be,
}

fn usb_qmi_get_pcm_num(_chip: *mut c_void, _direction: i32) -> i32 {
    // Kernel dependency: iterate through pcm_list
    0
}

fn get_speed_info(_udev_speed: u32) -> u32 {
    // Maps USB speed to QMI speed enum (external dependency)
    0
}

fn find_substream(_card_num: u32, _pcm_idx: u32, _direction: u32) -> *mut c_void {
    unsafe {
        if _card_num >= SNDRV_CARDS as u32 {
            return null_mut();
        }
        let chip = UADEV[_card_num as usize].chip;
        if chip.is_null() {
            return null_mut();
        }
        // Kernel dependency: search through pcm_list
        null_mut()
    }
}

fn info_idx_from_ifnum(card_num: i32, intf_num: i32, enable: bool) -> i32 {
    unsafe {
        if card_num < 0 || card_num >= SNDRV_CARDS as i32 {
            return -22; // -EINVAL
        }
        let udev = &UADEV[card_num as usize];

        if enable && udev.info.is_null() {
            return 0;
        }

        for i in 0..udev.num_intf {
            if enable && !(*udev.info.add(i as usize)).in_use {
                return i;
            } else if !enable && (*udev.info.add(i as usize)).intf_num as i32 == intf_num {
                return i;
            }
        }
    }
    -22 // -EINVAL
}

fn get_data_interval_from_si(_subs: *mut c_void, service_interval: u32) -> i32 {
    let bus_intval = if unsafe {
        // Kernel dependency: check device speed
        false
    } {
        BUS_INTERVAL_HIGHSPEED_AND_ABOVE
    } else {
        BUS_INTERVAL_FULL_SPEED
    };

    if service_interval % bus_intval != 0 {
        return -22; // -EINVAL
    }

    let bus_intval_mult = service_interval / bus_intval;
    let binterval = (32 - bus_intval_mult.leading_zeros()) as u32;
    if binterval == 0 || binterval > MAX_BINTERVAL_ISOC_EP {
        return -22; // -EINVAL
    }

    let check_mult = bus_intval_mult >> binterval;
    if check_mult != 0 {
        return -22; // -EINVAL
    }

    (binterval - 1) as i32
}

fn map_pcm_format(fmt_received: UsbQmiAudioFormat) -> u32 {
    match fmt_received {
        UsbQmiAudioFormat::UsbQmiPcmFormatS8 => 0,  // SNDRV_PCM_FORMAT_S8
        UsbQmiAudioFormat::UsbQmiPcmFormatU8 => 1,  // SNDRV_PCM_FORMAT_U8
        UsbQmiAudioFormat::UsbQmiPcmFormatS16Le => 2,
        UsbQmiAudioFormat::UsbQmiPcmFormatS16Be => 3,
        UsbQmiAudioFormat::UsbQmiPcmFormatU16Le => 4,
        UsbQmiAudioFormat::UsbQmiPcmFormatU16Be => 5,
        UsbQmiAudioFormat::UsbQmiPcmFormatS24Le => 6,
        UsbQmiAudioFormat::UsbQmiPcmFormatS24Be => 7,
        UsbQmiAudioFormat::UsbQmiPcmFormatU24Le => 8,
        UsbQmiAudioFormat::UsbQmiPcmFormatU24Be => 9,
        UsbQmiAudioFormat::UsbQmiPcmFormatS243Le => 10,
        UsbQmiAudioFormat::UsbQmiPcmFormatS243Be => 11,
        UsbQmiAudioFormat::UsbQmiPcmFormatU243Le => 12,
        UsbQmiAudioFormat::UsbQmiPcmFormatU243Be => 13,
        UsbQmiAudioFormat::UsbQmiPcmFormatS32Le => 14,
        UsbQmiAudioFormat::UsbQmiPcmFormatS32Be => 15,
        UsbQmiAudioFormat::UsbQmiPcmFormatU32Le => 16,
        UsbQmiAudioFormat::UsbQmiPcmFormatU32Be => 17,
    }
}

fn uaudio_send_disconnect_ind(_chip: *mut c_void) -> i32 {
    unsafe {
        let qdev = UAUDIO_QDEV;
        if qdev.is_null() {
            return 0;
        }
        // Kernel dependency: QMI messaging
        0
    }
}

fn uaudio_get_iova(
    curr_iova: &mut usize,
    curr_iova_size: &mut usize,
    _head: *mut ListHead,
    size: usize,
) -> usize {
    if size % PAGE_SIZE as usize != 0 {
        return 0;
    }

    if size > *curr_iova_size {
        return 0;
    }

    if *curr_iova_size == 0 {
        return 0;
    }

    let iova = *curr_iova;
    *curr_iova += size;
    *curr_iova_size -= size;
    iova
}

fn uaudio_put_iova(
    _iova: usize,
    size: usize,
    _head: *mut ListHead,
    curr_iova_size: &mut usize,
) {
    *curr_iova_size += size;
}

fn uaudio_iommu_unmap(
    mtype: MemType,
    _iova: usize,
    _iova_size: usize,
    _mapped_iova_size: usize,
) {
    unsafe {
        let qdev = UAUDIO_QDEV;
        if qdev.is_null() || _iova == 0 || _iova_size == 0 {
            return;
        }

        match mtype {
            MemType::MemEventRing => {
                if (*qdev).er_mapped {
                    (*qdev).er_mapped = false;
                }
            }
            MemType::MemXferRing => {
                uaudio_put_iova(_iova, _iova_size, &mut (*qdev).xfer_ring_list, &mut (*qdev).xfer_ring_iova_size);
            }
            MemType::MemXferBuf => {
                uaudio_put_iova(_iova, _iova_size, &mut (*qdev).xfer_buf_list, &mut (*qdev).xfer_buf_iova_size);
            }
        }
    }
}

fn uaudio_iommu_map_prot(dma_coherent: bool) -> i32 {
    let mut prot = 3; // IOMMU_READ | IOMMU_WRITE
    if dma_coherent {
        prot |= 4; // IOMMU_CACHE
    }
    prot
}

fn uaudio_iommu_map_pa(
    mtype: MemType,
    dma_coherent: bool,
    _pa: u64,
    _size: usize,
) -> usize {
    unsafe {
        let qdev = UAUDIO_QDEV;
        if qdev.is_null() {
            return 0;
        }

        match mtype {
            MemType::MemEventRing => {
                if (*qdev).er_mapped {
                    return IOVA_BASE as usize;
                }
                IOVA_BASE as usize
            }
            MemType::MemXferRing => {
                uaudio_get_iova(
                    &mut (*qdev).curr_xfer_ring_iova,
                    &mut (*qdev).xfer_ring_iova_size,
                    &mut (*qdev).xfer_ring_list,
                    _size,
                )
            }
            _ => 0,
        }
    }
}

fn uaudio_iommu_map_xfer_buf(
    _dma_coherent: bool,
    size: usize,
    _sgt: *mut c_void,
) -> usize {
    unsafe {
        let qdev = UAUDIO_QDEV;
        if qdev.is_null() {
            return 0;
        }

        uaudio_get_iova(
            &mut (*qdev).curr_xfer_buf_iova,
            &mut (*qdev).xfer_buf_iova_size,
            &mut (*qdev).xfer_buf_list,
            size,
        )
    }
}

fn usb_get_controller_id(_udev: *mut c_void) -> i32 {
    // Kernel dependency: device tree lookup
    -19 // -ENODEV
}

fn uaudio_dev_intf_cleanup(_udev: *mut c_void, info: *mut IntfInfo) {
    unsafe {
        if info.is_null() {
            return;
        }
        uaudio_iommu_unmap(
            MemType::MemXferRing,
            (*info).data_xfer_ring_va,
            (*info).data_xfer_ring_size,
            (*info).data_xfer_ring_size,
        );
        (*info).data_xfer_ring_va = 0;
        (*info).data_xfer_ring_size = 0;

        uaudio_iommu_unmap(
            MemType::MemXferRing,
            (*info).sync_xfer_ring_va,
            (*info).sync_xfer_ring_size,
            (*info).sync_xfer_ring_size,
        );
        (*info).sync_xfer_ring_va = 0;
        (*info).sync_xfer_ring_size = 0;

        uaudio_iommu_unmap(
            MemType::MemXferBuf,
            (*info).xfer_buf_iova as usize,
            (*info).xfer_buf_size,
            (*info).xfer_buf_size,
        );
        (*info).xfer_buf_iova = 0;

        // Kernel dependency: usb_free_coherent
        (*info).xfer_buf_size = 0;
        (*info).xfer_buf_cpu = null_mut();
        (*info).xfer_buf_dma = 0;
        (*info).in_use = false;
    }
}

fn uaudio_event_ring_cleanup_free(dev: *mut UaudioDev) {
    unsafe {
        if dev.is_null() || (*dev).chip.is_null() {
            return;
        }
        let qdev = UAUDIO_QDEV;
        if qdev.is_null() {
            return;
        }

        let card_num = 0; // Kernel dependency: get from chip
        if card_num < SNDRV_CARDS {
            (*qdev).card_slot &= !(1 << card_num);
            if (*qdev).card_slot == 0 {
                uaudio_iommu_unmap(MemType::MemEventRing, IOVA_BASE as usize, PAGE_SIZE as usize, PAGE_SIZE as usize);
                // Kernel dependency: xhci_sideband_remove_interrupter
                // Kernel dependency: usb_offload_put
            }
        }
    }
}

fn uaudio_dev_cleanup(dev: *mut UaudioDev) {
    unsafe {
        if dev.is_null() || (*dev).udev.is_null() {
            return;
        }

        for if_idx in 0..(*dev).num_intf {
            if !(*dev).info.is_null() {
                let info = &mut *(*dev).info.add(if_idx as usize);
                if info.in_use {
                    uaudio_dev_intf_cleanup((*dev).udev, info);
                }
            }
        }

        (*dev).num_intf = 0;

        if !(*dev).info.is_null() {
            // Kernel dependency: kfree
            (*dev).info = null_mut();
        }
        uaudio_event_ring_cleanup_free(dev);
        (*dev).udev = null_mut();
    }
}

fn disable_audio_stream(_subs: *mut c_void) {
    // Kernel dependency: snd_usb_hw_free, snd_usb_autosuspend
}

fn qmi_stop_session() {
    unsafe {
        // Kernel dependency: iterate through active interfaces
        for idx in 0..SNDRV_CARDS {
            if UADEV[idx].in_use.load(Ordering::Acquire) == 0 {
                continue;
            }

            let chip = UADEV[idx].chip;
            if chip.is_null() {
                continue;
            }

            for if_idx in 0..UADEV[idx].num_intf {
                if UADEV[idx].info.is_null() || !(*UADEV[idx].info.add(if_idx as usize)).in_use {
                    continue;
                }

                let info = &mut *UADEV[idx].info.add(if_idx as usize);
                disable_audio_stream(null_mut());
            }
            UADEV[idx].in_use.store(0, Ordering::Release);
            uaudio_dev_cleanup(&mut UADEV[idx]);
        }
    }
}

fn uaudio_sideband_notifier(
    _intf: *mut c_void,
    _evt: *mut c_void,
) -> i32 {
    // Kernel dependency: xHCI sideband event handler
    0
}

fn qmi_bye_cb(_handle: *mut c_void, _node: u32) {
    unsafe {
        let svc = UAUDIO_SVC;
        if svc.is_null() {
            return;
        }

        if (*svc).client_connected && (*svc).client_sq.sq_node == _node {
            qmi_stop_session();
            (*svc).client_sq.sq_node = 0;
            (*svc).client_sq.sq_port = 0;
            (*svc).client_sq.sq_family = 0;
            (*svc).client_connected = false;
        }
    }
}

fn qmi_svc_disconnect_cb(_handle: *mut c_void, _node: u32, _port: u32) {
    unsafe {
        if UAUDIO_SVC.is_null() {
            return;
        }

        let svc = UAUDIO_SVC;
        if (*svc).client_connected && (*svc).client_sq.sq_node == _node && (*svc).client_sq.sq_port == _port {
            qmi_stop_session();
            (*svc).client_sq.sq_node = 0;
            (*svc).client_sq.sq_port = 0;
            (*svc).client_sq.sq_family = 0;
            (*svc).client_connected = false;
        }
    }
}

fn uaudio_dev_release(_kref: *mut c_void) {
    unsafe {
        // Kernel dependency: container_of to get uaudio_dev
        let dev: *mut UaudioDev = null_mut();
        uaudio_event_ring_cleanup_free(dev);
        if !dev.is_null() {
            (*dev).in_use.store(0, Ordering::Release);
            // Kernel dependency: wake_up
        }
    }
}

fn enable_audio_stream(
    _subs: *mut c_void,
    _pcm_format: u32,
    _channels: u32,
    _cur_rate: u32,
    _datainterval: i32,
) -> i32 {
    // Kernel dependency: snd_usb_hw_params, endpoint preparation
    0
}

fn uaudio_transfer_buffer_setup(
    _subs: *mut c_void,
    _xfer_buf_cpu: *mut *mut c_void,
    _xfer_buf_len: u32,
    _mem_info: *mut c_void,
) -> i32 {
    // Kernel dependency: DMA buffer allocation and mapping
    0
}

fn uaudio_endpoint_setup(
    _subs: *mut c_void,
    _endpoint: *mut c_void,
    _card_num: i32,
    _mem_info: *mut c_void,
    _ep_desc: *mut c_void,
) -> u64 {
    // Kernel dependency: xhci sideband endpoint setup
    0
}

fn uaudio_event_ring_setup(
    _subs: *mut c_void,
    _card_num: i32,
    _mem_info: *mut c_void,
) -> i32 {
    // Kernel dependency: xhci sideband interrupter setup
    0
}

fn uaudio_populate_uac_desc(
    _subs: *mut c_void,
    _resp: *mut c_void,
) -> i32 {
    // Kernel dependency: UAC descriptor parsing
    0
}

fn prepare_qmi_response(
    _subs: *mut c_void,
    _req_msg: *mut c_void,
    _resp: *mut c_void,
    _info_idx: i32,
) -> i32 {
    // Kernel dependency: comprehensive stream setup
    0
}

fn handle_uaudio_stream_req(
    _handle: *mut c_void,
    _sq: *mut c_void,
    _txn: *mut c_void,
    _decoded_msg: *const c_void,
) {
    // Kernel dependency: QMI message handling
}

#[repr(C)]
struct QmiMsgHandler {
    msg_type: u32,
    msg_id: u32,
    ei: *mut c_void,
    decoded_size: usize,
    fn_ptr: *mut c_void,
}

static UAUDIO_STREAM_REQ_HANDLERS: QmiMsgHandler = QmiMsgHandler {
    msg_type: 0,
    msg_id: 0,
    ei: null_mut(),
    decoded_size: 0,
    fn_ptr: null_mut(),
};

fn qc_usb_audio_offload_init_qmi_dev() -> i32 {
    unsafe {
        if UAUDIO_QDEV.is_null() {
            UAUDIO_QDEV = alloc_zeroed(size_of::<UaudioQmiDev>()) as *mut UaudioQmiDev;
        }
        if UAUDIO_QDEV.is_null() {
            return -12; // -ENOMEM
        }

        let qdev = UAUDIO_QDEV;
        // Initialize list heads
        (*qdev).curr_xfer_ring_iova = IOVA_XFER_RING_BASE as usize;
        (*qdev).xfer_ring_iova_size = (IOVA_XFER_RING_MAX - IOVA_XFER_RING_BASE) as usize;

        (*qdev).curr_xfer_buf_iova = IOVA_XFER_BUF_BASE as usize;
        (*qdev).xfer_buf_iova_size = (IOVA_XFER_BUF_MAX - IOVA_XFER_BUF_BASE) as usize;

        0
    }
}

fn qc_usb_audio_offload_fill_avail_pcms(
    _chip: *mut c_void,
    _sdev: *mut c_void,
) -> i32 {
    // Kernel dependency: iterate PCM list
    0
}

fn qc_usb_audio_offload_probe(_chip: *mut c_void) {
    // Kernel dependency: device probe handler
}

fn qc_usb_audio_cleanup_qmi_dev() {
    unsafe {
        if !UAUDIO_QDEV.is_null() {
            // Kernel dependency: kfree
            UAUDIO_QDEV = null_mut();
        }
    }
}

fn qc_usb_audio_offload_disconnect(_chip: *mut c_void) {
    // Kernel dependency: device disconnect handler
}

fn qc_usb_audio_offload_suspend(_intf: *mut c_void, _message: u32) {
    // Kernel dependency: suspend handler
}

#[repr(C)]
struct SndUsbPlatformOps {
    connect_cb: extern "C" fn(*mut c_void),
    disconnect_cb: extern "C" fn(*mut c_void),
    suspend_cb: extern "C" fn(*mut c_void, u32),
}

static OFFLOAD_OPS: SndUsbPlatformOps = SndUsbPlatformOps {
    connect_cb: qc_usb_audio_offload_probe,
    disconnect_cb: qc_usb_audio_offload_disconnect,
    suspend_cb: qc_usb_audio_offload_suspend,
};

fn qc_usb_audio_probe(_auxdev: *mut c_void, _id: *mut c_void) -> i32 {
    unsafe {
        let svc = alloc_zeroed(size_of::<UaudioQmiSvc>()) as *mut UaudioQmiSvc;
        if svc.is_null() {
            return -12; // -ENOMEM
        }

        let hdl = alloc_zeroed(0);
        if hdl.is_null() {
            // Kernel dependency: kfree(svc)
            return -12; // -ENOMEM
        }

        (*svc).uaudio_svc_hdl = hdl;
        UAUDIO_SVC = svc;

        qc_usb_audio_offload_init_qmi_dev();

        0
    }
}

fn qc_usb_audio_remove(_auxdev: *mut c_void) {
    unsafe {
        for idx in 0..SNDRV_CARDS {
            qc_usb_audio_offload_disconnect(UADEV[idx].chip);
        }

        qc_usb_audio_cleanup_qmi_dev();

        if !UAUDIO_SVC.is_null() {
            // Kernel dependency: kfree
            UAUDIO_SVC = null_mut();
        }
    }
}

unsafe fn alloc_zeroed(size: usize) -> *mut c_void {
    if size == 0 {
        return null_mut();
    }
    // Kernel dependency: allocate and zero memory
    null_mut()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
