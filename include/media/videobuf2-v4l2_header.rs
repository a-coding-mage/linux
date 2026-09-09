/*
 * videobuf2-v4l2.h - V4L2 driver helper framework
 *
 * Rust translation of the C header. Linux header dependencies are supplied
 * externally.
 */

/* C includes and header guard omitted; dependency intent preserved above. */
/* C build-time checks: VB2_MAX_FRAME must equal VIDEO_MAX_FRAME and
 * VB2_MAX_PLANES must equal VIDEO_MAX_PLANES. */

pub struct video_device;

#[repr(C)]
pub struct vb2_v4l2_buffer {
    pub vb2_buf: vb2_buffer,
    pub flags: __u32,
    pub field: __u32,
    pub timecode: v4l2_timecode,
    pub sequence: __u32,
    pub request_fd: __s32,
    pub is_held: bool,
    pub planes: [vb2_plane; VB2_MAX_PLANES as usize],
}

pub const VB2_V4L2_FL_SUPPORTS_M2M_HOLD_CAPTURE_BUF: u32 = 1 << 0;

/* to_vb2_v4l2_buffer(): cast struct vb2_buffer * to struct vb2_v4l2_buffer *.
 * vb2_buf is the first member, so the C container_of conversion is a cast. */
#[inline]
pub unsafe fn to_vb2_v4l2_buffer(vb: *mut vb2_buffer) -> *mut vb2_v4l2_buffer {
    vb as *mut vb2_v4l2_buffer
}

extern "C" {
pub fn vb2_find_buffer(q: *mut vb2_queue, timestamp: u64) -> *mut vb2_buffer;
pub fn vb2_querybuf(q: *mut vb2_queue, b: *mut v4l2_buffer) -> i32;
pub fn vb2_reqbufs(q: *mut vb2_queue, req: *mut v4l2_requestbuffers) -> i32;
pub fn vb2_create_bufs(q: *mut vb2_queue, create: *mut v4l2_create_buffers) -> i32;
pub fn vb2_prepare_buf(q: *mut vb2_queue, mdev: *mut media_device, b: *mut v4l2_buffer) -> i32;
pub fn vb2_qbuf(q: *mut vb2_queue, mdev: *mut media_device, b: *mut v4l2_buffer) -> i32;
pub fn vb2_expbuf(q: *mut vb2_queue, eb: *mut v4l2_exportbuffer) -> i32;
pub fn vb2_dqbuf(q: *mut vb2_queue, b: *mut v4l2_buffer, nonblocking: bool) -> i32;
pub fn vb2_streamon(q: *mut vb2_queue, r#type: v4l2_buf_type) -> i32;
pub fn vb2_streamoff(q: *mut vb2_queue, r#type: v4l2_buf_type) -> i32;
pub fn vb2_queue_init(q: *mut vb2_queue) -> i32;
pub fn vb2_queue_init_name(q: *mut vb2_queue, name: *const c_char) -> i32;
pub fn vb2_queue_release(q: *mut vb2_queue);
pub fn vb2_queue_change_type(q: *mut vb2_queue, r#type: c_uint) -> i32;
pub fn vb2_poll(q: *mut vb2_queue, file: *mut file, wait: *mut poll_table) -> __poll_t;

#[inline]
pub unsafe fn vb2_queue_is_busy(q: *mut vb2_queue, file: *mut file) -> bool {
    !(*q).owner.is_null() && (*q).owner != (*file).private_data
}

pub fn vb2_ioctl_reqbufs(file: *mut file, priv_: *mut c_void, p: *mut v4l2_requestbuffers) -> i32;
pub fn vb2_ioctl_create_bufs(file: *mut file, priv_: *mut c_void, p: *mut v4l2_create_buffers) -> i32;
pub fn vb2_ioctl_prepare_buf(file: *mut file, priv_: *mut c_void, p: *mut v4l2_buffer) -> i32;
pub fn vb2_ioctl_querybuf(file: *mut file, priv_: *mut c_void, p: *mut v4l2_buffer) -> i32;
pub fn vb2_ioctl_qbuf(file: *mut file, priv_: *mut c_void, p: *mut v4l2_buffer) -> i32;
pub fn vb2_ioctl_dqbuf(file: *mut file, priv_: *mut c_void, p: *mut v4l2_buffer) -> i32;
pub fn vb2_ioctl_streamon(file: *mut file, priv_: *mut c_void, i: v4l2_buf_type) -> i32;
pub fn vb2_ioctl_streamoff(file: *mut file, priv_: *mut c_void, i: v4l2_buf_type) -> i32;
pub fn vb2_ioctl_expbuf(file: *mut file, priv_: *mut c_void, p: *mut v4l2_exportbuffer) -> i32;
pub fn vb2_ioctl_remove_bufs(file: *mut file, priv_: *mut c_void, p: *mut v4l2_remove_buffers) -> i32;

pub fn vb2_fop_mmap(file: *mut file, vma: *mut vm_area_struct) -> i32;
pub fn vb2_fop_release(file: *mut file) -> i32;
pub fn _vb2_fop_release(file: *mut file, lock: *mut mutex) -> i32;
pub fn vb2_fop_write(file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
pub fn vb2_fop_read(file: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
pub fn vb2_fop_poll(file: *mut file, wait: *mut poll_table) -> __poll_t;

/* Preserved conditionally: available only when CONFIG_MMU is not defined. */
#[cfg(not(CONFIG_MMU))]
pub fn vb2_fop_get_unmapped_area(file: *mut file, addr: c_ulong, len: c_ulong,
                                 pgoff: c_ulong, flags: c_ulong) -> c_ulong;

pub fn vb2_video_unregister_device(vdev: *mut video_device);

pub struct media_request;
pub fn vb2_request_validate(req: *mut media_request) -> i32;
pub fn vb2_request_queue(req: *mut media_request);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
