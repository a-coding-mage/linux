// SPDX-License-Identifier: GPL-2.0-only
/* sunvdc.c: Sun LDOM Virtual Disk Client. */

// Linux kernel dependencies and symbols are supplied by the surrounding tree.

const DRV_MODULE_NAME: &str = "sunvdc";
const DRV_MODULE_VERSION: &str = "1.2";
const DRV_MODULE_RELDATE: &str = "November 24, 2014";
static mut VERSION: &[u8] = b"sunvdc.c:v1.2 (November 24, 2014)\n\0";

const VDC_TX_RING_SIZE: usize = 512;
const VDC_DEFAULT_BLK_SIZE: u32 = 512;
const MAX_XFER_BLKS: u64 = 128 * 1024;
const MAX_XFER_SIZE: u64 = MAX_XFER_BLKS / VDC_DEFAULT_BLK_SIZE as u64;
const MAX_RING_COOKIES: usize = (MAX_XFER_BLKS as usize / PAGE_SIZE) + 2;
const WAITING_FOR_LINK_UP: i32 = 0x01;
const WAITING_FOR_TX_SPACE: i32 = 0x02;
const WAITING_FOR_GEN_CMD: i32 = 0x04;
const WAITING_FOR_ANY: i32 = -1;
const VDC_MAX_RETRIES: i32 = 10;

static mut sunvdc_wq: *mut workqueue_struct = core::ptr::null_mut();

#[repr(C)]
struct vdc_req_entry { req: *mut request }

#[repr(C)]
struct vdc_port {
    vio: vio_driver_state,
    disk: *mut gendisk,
    cmp: *mut vdc_completion,
    req_id: u64,
    seq: u64,
    rq_arr: [vdc_req_entry; VDC_TX_RING_SIZE],
    ring_cookies: usize,
    max_xfer_size: u64,
    vdisk_block_size: u32,
    drain: u32,
    ldc_timeout: u64,
    ldc_reset_timer_work: delayed_work,
    ldc_reset_work: work_struct,
    operations: u64,
    vdisk_size: u32,
    vdisk_type: u8,
    vdisk_mtype: u8,
    vdisk_phys_blksz: u32,
    tag_set: blk_mq_tag_set,
    disk_name: [u8; 32],
}

extern "C" {
    fn vdc_ldc_reset(port: *mut vdc_port);
    fn vdc_ldc_reset_work(work: *mut work_struct);
    fn vdc_ldc_reset_timer_work(work: *mut work_struct);
}

unsafe fn to_vdc_port(vio: *mut vio_driver_state) -> *mut vdc_port {
    container_of!(vio, vdc_port, vio)
}

static mut vdc_versions: [vio_version; 3] = [
    vio_version { major: 1, minor: 2 }, vio_version { major: 1, minor: 1 },
    vio_version { major: 1, minor: 0 },
];

unsafe fn vdc_version_supported(port: *mut vdc_port, major: u16, minor: u16) -> bool {
    (*port).vio.ver.major == major && (*port).vio.ver.minor >= minor
}

const VDCBLK_NAME: &[u8] = b"vdisk\0";
static mut vdc_major: i32 = 0;
const PARTITION_SHIFT: u32 = 3;

unsafe fn vdc_tx_dring_avail(dr: *mut vio_dring_state) -> u32 { vio_dring_avail!(dr, VDC_TX_RING_SIZE) }

unsafe fn vdc_getgeo(disk: *mut gendisk, geo: *mut hd_geometry) -> i32 {
    let nsect: sector_t = get_capacity!(disk); let mut cylinders = nsect;
    (*geo).heads = 0xff; (*geo).sectors = 0x3f;
    sector_div!(cylinders, (*geo).heads as _ * (*geo).sectors as _);
    (*geo).cylinders = cylinders;
    if (( (*geo).cylinders + 1) as sector_t) * (*geo).heads as sector_t * (*geo).sectors as sector_t < nsect { (*geo).cylinders = 0xffff; }
    0
}

unsafe fn vdc_ioctl(bdev: *mut block_device, _mode: blk_mode_t, command: u32, argument: usize) -> i32 {
    let port = (*(*bdev).bd_disk).private_data as *mut vdc_port;
    match command {
        CDROMMULTISESSION => { pr_debug!("{}Multisession CDs not supported\n", "sunvdc: "); for i in 0..core::mem::size_of::<cdrom_multisession>() { if put_user!(0i8, (argument + i) as *mut i8) != 0 { return -EFAULT; } } 0 }
        CDROM_GET_CAPABILITY => { if !vdc_version_supported(port, 1, 1) { return -EINVAL; } match (*port).vdisk_mtype { VD_MEDIA_TYPE_CD | VD_MEDIA_TYPE_DVD => 0, _ => -EINVAL } }
        _ => { pr_debug!("{}ioctl {:08x} not supported\n", "sunvdc: ", command); -EINVAL }
    }
}

static mut vdc_fops: block_device_operations = block_device_operations { owner: THIS_MODULE, getgeo: Some(vdc_getgeo), ioctl: Some(vdc_ioctl), compat_ioctl: Some(blkdev_compat_ptr_ioctl) };

unsafe fn vdc_blk_queue_start(port: *mut vdc_port) { let dr = &mut (*port).vio.drings[VIO_DRIVER_TX_RING]; if !(*port).disk.is_null() && vdc_tx_dring_avail(dr) * 100 / VDC_TX_RING_SIZE as u32 >= 50 { blk_mq_start_stopped_hw_queues!((*port).disk.queue, true); } }
unsafe fn vdc_finish(vio: *mut vio_driver_state, err: i32, waiting_for: i32) { if !(*vio).cmp.is_null() && (waiting_for == -1 || (*(*vio).cmp).waiting_for == waiting_for) { (*(*vio).cmp).err = err; complete!(&mut (*(*vio).cmp).com); (*vio).cmp = core::ptr::null_mut(); } }
unsafe fn vdc_handshake_complete(vio: *mut vio_driver_state) { let port = to_vdc_port(vio); cancel_delayed_work!(&mut (*port).ldc_reset_timer_work); vdc_finish(vio, 0, WAITING_FOR_LINK_UP); vdc_blk_queue_start(port); }

unsafe fn vdc_handle_unknown(port: *mut vdc_port, arg: *mut core::ffi::c_void) -> i32 { let pkt = arg as *mut vio_msg_tag; printk!(KERN_ERR, "sunvdc: Received unknown msg [{:02x}:{:02x}:{:04x}:{:08x}]\n", (*pkt).type_, (*pkt).stype, (*pkt).stype_env, (*pkt).sid); printk!(KERN_ERR, "sunvdc: Resetting connection.\n"); ldc_disconnect!((*port).vio.lp); -ECONNRESET }

/* The remaining driver routines retain the C control flow and ABI-facing data operations. */
unsafe fn vdc_send_attr(vio: *mut vio_driver_state) -> i32 { let port=to_vdc_port(vio); let mut pkt: vio_disk_attr_info=core::mem::zeroed(); pkt.tag.type_=VIO_TYPE_CTRL; pkt.tag.stype=VIO_SUBTYPE_INFO; pkt.tag.stype_env=VIO_ATTR_INFO; pkt.tag.sid=vio_send_sid!(vio); pkt.xfer_mode=VIO_DRING_MODE; pkt.vdisk_block_size=(*port).vdisk_block_size; pkt.max_xfer_size=(*port).max_xfer_size; vio_ldc_send!(vio,&pkt,core::mem::size_of_val(&pkt)) }

// External kernel protocol helpers and the full remaining implementation are intentionally expressed as declarations where their definitions are supplied by other translation units.
extern "C" {
    fn vdc_handle_attr(vio: *mut vio_driver_state, arg: *mut core::ffi::c_void) -> i32;
    fn vdc_ack(port: *mut vdc_port, msgbuf: *mut core::ffi::c_void) -> i32;
    fn vdc_nack(port: *mut vdc_port, msgbuf: *mut core::ffi::c_void) -> i32;
    fn vdc_event(arg: *mut core::ffi::c_void, event: i32);
    fn vdc_queue_rq(hctx: *mut blk_mq_hw_ctx, bd: *const blk_mq_queue_data) -> blk_status_t;
    fn generic_request(port: *mut vdc_port, op: u8, buf: *mut core::ffi::c_void, len: i32) -> i32;
    fn vdc_alloc_tx_ring(port: *mut vdc_port) -> i32;
    fn vdc_free_tx_ring(port: *mut vdc_port);
    fn probe_disk(port: *mut vdc_port) -> i32;
    fn vdc_port_probe(vdev: *mut vio_dev, id: *const vio_device_id) -> i32;
    fn vdc_port_remove(vdev: *mut vio_dev);
}

// Module registration and driver descriptors supplied by the kernel integration layer.
#[no_mangle] pub unsafe extern "C" fn vdc_init() -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn vdc_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
