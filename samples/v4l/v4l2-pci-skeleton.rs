// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of v4l2-pci-skeleton.c. Kernel dependencies are external. */

#[repr(C)]
pub struct Skeleton {
    pub pdev: *mut pci_dev,
    pub v4l2_dev: v4l2_device,
    pub vdev: video_device,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub lock: mutex,
    pub std: v4l2_std_id,
    pub timings: v4l2_dv_timings,
    pub format: v4l2_pix_format,
    pub input: c_uint,
    pub queue: vb2_queue,
    pub qlock: spinlock_t,
    pub buf_list: list_head,
    pub field: c_uint,
    pub sequence: c_uint,
}

#[repr(C)]
pub struct SkelBuffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

#[inline]
unsafe fn to_skel_buffer(vbuf: *mut vb2_v4l2_buffer) -> *mut SkelBuffer {
    container_of(vbuf, SkelBuffer, vb)
}

static SKELETON_PCI_TBL: [pci_device_id; 2] = [
    pci_device_id { vendor: 0, device: 0 },
    pci_device_id { vendor: 0, device: 0 },
];

static SKEL_TIMINGS_CAP: v4l2_dv_timings_cap = v4l2_dv_timings_cap {
    type_: V4L2_DV_BT_656_1120,
    reserved: [0; 3],
    min_width: 720,
    max_width: 1920,
    min_height: 480,
    max_height: 1080,
    min_pixelclock: 27000000,
    max_pixelclock: 74250000,
    standards: V4L2_DV_BT_STD_CEA861,
    capabilities: V4L2_DV_BT_CAP_INTERLACED | V4L2_DV_BT_CAP_PROGRESSIVE,
};

const SKEL_TVNORMS: v4l2_std_id = V4L2_STD_ALL;

unsafe extern "C" fn skeleton_irq(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    // The C TODO block is intentionally empty until hardware capture is implemented.
    IRQ_HANDLED
}

unsafe extern "C" fn queue_setup(vq: *mut vb2_queue, nbuffers: *mut c_uint,
    nplanes: *mut c_uint, sizes: *mut usize, _alloc_devs: *mut *mut device) -> c_int {
    let skel = vb2_get_drv_priv(vq) as *mut Skeleton;
    let q_num_bufs = vb2_get_num_buffers(vq);
    (*skel).field = (*skel).format.field;
    if (*skel).field == V4L2_FIELD_ALTERNATE {
        if vb2_fileio_is_active(vq) { return -EINVAL; }
        (*skel).field = V4L2_FIELD_TOP;
    }
    if q_num_bufs + *nbuffers < 3 { *nbuffers = 3 - q_num_bufs; }
    if *nplanes != 0 { return if *sizes < (*skel).format.sizeimage as usize { -EINVAL } else { 0 }; }
    *nplanes = 1;
    *sizes = (*skel).format.sizeimage as usize;
    0
}

unsafe extern "C" fn buffer_prepare(vb: *mut vb2_buffer) -> c_int {
    let skel = vb2_get_drv_priv((*vb).vb2_queue) as *mut Skeleton;
    let size = (*skel).format.sizeimage as usize;
    if vb2_plane_size(vb, 0) < size {
        dev_err(&(*skel).pdev.as_ref().unwrap().dev, "buffer too small\n");
        return -EINVAL;
    }
    vb2_set_plane_payload(vb, 0, size); 0
}

unsafe extern "C" fn buffer_queue(vb: *mut vb2_buffer) {
    let vbuf = to_vb2_v4l2_buffer(vb);
    let skel = vb2_get_drv_priv((*vb).vb2_queue) as *mut Skeleton;
    let buf = to_skel_buffer(vbuf);
    let mut flags = 0;
    spin_lock_irqsave(&mut (*skel).qlock, &mut flags);
    list_add_tail(&mut (*buf).list, &mut (*skel).buf_list);
    // TODO: Update any DMA pointers if necessary.
    spin_unlock_irqrestore(&mut (*skel).qlock, flags);
}

unsafe fn return_all_buffers(skel: *mut Skeleton, state: vb2_buffer_state) {
    let mut flags = 0;
    spin_lock_irqsave(&mut (*skel).qlock, &mut flags);
    let mut pos = (*skel).buf_list.next;
    while pos != &mut (*skel).buf_list as *mut list_head {
        let next = (*pos).next;
        let buf = container_of(pos, SkelBuffer, list);
        vb2_buffer_done(&mut (*buf).vb.vb2_buf, state);
        list_del(&mut (*buf).list);
        pos = next;
    }
    spin_unlock_irqrestore(&mut (*skel).qlock, flags);
}

unsafe extern "C" fn start_streaming(vq: *mut vb2_queue, _count: c_uint) -> c_int {
    let skel = vb2_get_drv_priv(vq) as *mut Skeleton;
    (*skel).sequence = 0;
    // TODO: start DMA.
    let ret = 0;
    if ret != 0 { return_all_buffers(skel, VB2_BUF_STATE_QUEUED); }
    ret
}

unsafe extern "C" fn stop_streaming(vq: *mut vb2_queue) {
    let skel = vb2_get_drv_priv(vq) as *mut Skeleton;
    // TODO: stop DMA.
    return_all_buffers(skel, VB2_BUF_STATE_ERROR);
}

static SKEL_QOPS: vb2_ops = vb2_ops { queue_setup: Some(queue_setup), buf_prepare: Some(buffer_prepare), buf_queue: Some(buffer_queue), start_streaming: Some(start_streaming), stop_streaming: Some(stop_streaming) };

unsafe fn skeleton_fill_pix_format(skel: *mut Skeleton, pix: *mut v4l2_pix_format) {
    (*pix).pixelformat = V4L2_PIX_FMT_YUYV;
    if (*skel).input == 0 {
        (*pix).width = 720; (*pix).height = if (*skel).std & V4L2_STD_525_60 != 0 { 480 } else { 576 };
        (*pix).field = V4L2_FIELD_INTERLACED; (*pix).colorspace = V4L2_COLORSPACE_SMPTE170M;
    } else {
        (*pix).width = (*skel).timings.bt.width; (*pix).height = (*skel).timings.bt.height;
        if (*skel).timings.bt.interlaced { (*pix).field = V4L2_FIELD_ALTERNATE; (*pix).height /= 2; }
        else { (*pix).field = V4L2_FIELD_NONE; }
        (*pix).colorspace = V4L2_COLORSPACE_REC709;
    }
    (*pix).bytesperline = (*pix).width * 2;
    (*pix).sizeimage = (*pix).bytesperline * (*pix).height;
    (*pix).priv = 0;
}

unsafe extern "C" fn skeleton_try_fmt_vid_cap(file: *mut file, _priv: *mut c_void, f: *mut v4l2_format) -> c_int {
    let skel = video_drvdata(file) as *mut Skeleton;
    if (*f).fmt.pix.pixelformat != V4L2_PIX_FMT_YUYV { return -EINVAL; }
    skeleton_fill_pix_format(skel, &mut (*f).fmt.pix); 0
}

unsafe extern "C" fn skeleton_s_fmt_vid_cap(file: *mut file, priv_: *mut c_void, f: *mut v4l2_format) -> c_int {
    let skel = video_drvdata(file) as *mut Skeleton;
    let ret = skeleton_try_fmt_vid_cap(file, priv_, f); if ret != 0 { return ret; }
    if vb2_is_busy(&mut (*skel).queue) { return -EBUSY; }
    // TODO: change format.
    (*skel).format = (*f).fmt.pix; 0
}

unsafe extern "C" fn skeleton_g_fmt_vid_cap(file: *mut file, _priv: *mut c_void, f: *mut v4l2_format) -> c_int {
    let skel = video_drvdata(file) as *mut Skeleton; (*f).fmt.pix = (*skel).format; 0
}

unsafe extern "C" fn skeleton_enum_fmt_vid_cap(_file: *mut file, _priv: *mut c_void, f: *mut v4l2_fmtdesc) -> c_int {
    if (*f).index != 0 { return -EINVAL; } (*f).pixelformat = V4L2_PIX_FMT_YUYV; 0
}

unsafe extern "C" fn skeleton_s_std(file: *mut file, _priv: *mut c_void, std: v4l2_std_id) -> c_int {
    let skel = video_drvdata(file) as *mut Skeleton;
    if (*skel).input != 0 { return -ENODATA; } if std == (*skel).std { return 0; }
    if vb2_is_busy(&mut (*skel).queue) { return -EBUSY; }
    // TODO: handle changing std.
    (*skel).std = std; skeleton_fill_pix_format(skel, &mut (*skel).format); 0
}

unsafe extern "C" fn skeleton_g_std(file: *mut file, _priv: *mut c_void, std: *mut v4l2_std_id) -> c_int {
    let skel = video_drvdata(file) as *mut Skeleton; if (*skel).input != 0 { return -ENODATA; } *std = (*skel).std; 0
}

unsafe extern "C" fn skeleton_querystd(file: *mut file, _priv: *mut c_void, _std: *mut v4l2_std_id) -> c_int {
    let skel = video_drvdata(file) as *mut Skeleton; if (*skel).input != 0 { return -ENODATA; } 0
}

unsafe extern "C" fn skeleton_s_dv_timings(file: *mut file, _priv: *mut c_void, timings: *mut v4l2_dv_timings) -> c_int {
    let skel = video_drvdata(file) as *mut Skeleton;
    if (*skel).input == 0 { return -ENODATA; }
    if !v4l2_valid_dv_timings(timings, &SKEL_TIMINGS_CAP, core::ptr::null(), core::ptr::null()) { return -EINVAL; }
    if !v4l2_find_dv_timings_cap(timings, &SKEL_TIMINGS_CAP, 0, core::ptr::null(), core::ptr::null()) { return -EINVAL; }
    if v4l2_match_dv_timings(timings, &(*skel).timings, 0, false) { return 0; }
    if vb2_is_busy(&mut (*skel).queue) { return -EBUSY; }
    // TODO: Configure new timings.
    (*skel).timings = *timings; skeleton_fill_pix_format(skel, &mut (*skel).format); 0
}

unsafe extern "C" fn skeleton_g_dv_timings(file: *mut file, _priv: *mut c_void, timings: *mut v4l2_dv_timings) -> c_int {
    let skel = video_drvdata(file) as *mut Skeleton; if (*skel).input == 0 { return -ENODATA; } *timings = (*skel).timings; 0
}

unsafe extern "C" fn skeleton_enum_dv_timings(file: *mut file, _priv: *mut c_void, timings: *mut v4l2_enum_dv_timings) -> c_int {
    let skel = video_drvdata(file) as *mut Skeleton; if (*skel).input == 0 { return -ENODATA; }
    v4l2_enum_dv_timings_cap(timings, &SKEL_TIMINGS_CAP, core::ptr::null(), core::ptr::null())
}

unsafe extern "C" fn skeleton_query_dv_timings(file: *mut file, _priv: *mut c_void, _timings: *mut v4l2_dv_timings) -> c_int {
    let skel = video_drvdata(file) as *mut Skeleton; if (*skel).input == 0 { return -ENODATA; } 0
}

unsafe extern "C" fn skeleton_dv_timings_cap(file: *mut file, _priv: *mut c_void, cap: *mut v4l2_dv_timings_cap) -> c_int {
    let skel = video_drvdata(file) as *mut Skeleton; if (*skel).input == 0 { return -ENODATA; } *cap = SKEL_TIMINGS_CAP; 0
}

unsafe extern "C" fn skeleton_enum_input(_file: *mut file, _priv: *mut c_void, i: *mut v4l2_input) -> c_int {
    if (*i).index > 1 { return -EINVAL; } (*i).type_ = V4L2_INPUT_TYPE_CAMERA;
    if (*i).index == 0 { (*i).std = SKEL_TVNORMS; strscpy((*i).name.as_mut_ptr(), b"S-Video\0".as_ptr(), (*i).name.len()); (*i).capabilities = V4L2_IN_CAP_STD; }
    else { (*i).std = 0; strscpy((*i).name.as_mut_ptr(), b"HDMI\0".as_ptr(), (*i).name.len()); (*i).capabilities = V4L2_IN_CAP_DV_TIMINGS; } 0
}

unsafe extern "C" fn skeleton_s_input(file: *mut file, _priv: *mut c_void, i: c_uint) -> c_int {
    let skel = video_drvdata(file) as *mut Skeleton; if i > 1 { return -EINVAL; }
    if vb2_is_busy(&mut (*skel).queue) { return -EBUSY; } (*skel).input = i; (*skel).vdev.tvnorms = if i != 0 { 0 } else { SKEL_TVNORMS };
    skeleton_fill_pix_format(skel, &mut (*skel).format); 0
}

unsafe extern "C" fn skeleton_g_input(file: *mut file, _priv: *mut c_void, i: *mut c_uint) -> c_int { *i = (*(video_drvdata(file) as *mut Skeleton)).input; 0 }

unsafe extern "C" fn skeleton_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    match (*ctrl).id { V4L2_CID_BRIGHTNESS | V4L2_CID_CONTRAST | V4L2_CID_SATURATION | V4L2_CID_HUE => 0, _ => -EINVAL }
}

// The remaining ioctl/file-operation registration tables and PCI probe/remove entry points
// retain the C driver's external kernel-facing interfaces.
extern "C" {
    fn skeleton_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int;
    fn skeleton_remove(pdev: *mut pci_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
