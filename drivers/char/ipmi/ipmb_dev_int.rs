// SPDX-License-Identifier: GPL-2.0

/*
 * IPMB driver to receive a request and send a response
 *
 * Copyright (C) 2019 Mellanox Techologies, Ltd.
 *
 * This was inspired by Brendan Higgins' ipmi-bmc-bt-i2c driver.
 */

// Linux dependencies supplied by the surrounding kernel/Rust bindings.

const MAX_MSG_LEN: usize = 240;
const IPMB_REQUEST_LEN_MIN: u8 = 7;
const NETFN_RSP_BIT_MASK: u8 = 0x4;
const REQUEST_QUEUE_MAX_LEN: i32 = 256;

const IPMB_MSG_LEN_IDX: usize = 0;
const RQ_SA_8BIT_IDX: usize = 1;
const NETFN_LUN_IDX: usize = 2;

#[inline]
fn get_7bit_addr(addr_8bit: u8) -> u8 { addr_8bit >> 1 }

#[inline]
fn get_8bit_addr(addr_7bit: u8) -> u8 { (addr_7bit << 1) & 0xff }

const IPMB_MSG_PAYLOAD_LEN_MAX: usize = MAX_MSG_LEN - IPMB_REQUEST_LEN_MIN as usize - 1;

const SMBUS_MSG_HEADER_LENGTH: usize = 2;
const SMBUS_MSG_IDX_OFFSET: usize = SMBUS_MSG_HEADER_LENGTH + 1;

#[repr(C, packed)]
struct IpmbMsg {
    len: u8,
    rs_sa: u8,
    netfn_rs_lun: u8,
    checksum1: u8,
    rq_sa: u8,
    rq_seq_rq_lun: u8,
    cmd: u8,
    payload: [u8; IPMB_MSG_PAYLOAD_LEN_MAX],
}

#[repr(C)]
struct IpmbRequestElem {
    list: ListHead,
    request: IpmbMsg,
}

#[repr(C)]
struct IpmbDev {
    client: *mut I2cClient,
    miscdev: Miscdevice,
    request: IpmbMsg,
    request_queue: ListHead,
    request_queue_len: AtomicT,
    msg_idx: usize,
    lock: SpinlockT,
    wait_queue: WaitQueueHeadT,
    file_mutex: Mutex,
    is_i2c_protocol: bool,
}

unsafe fn to_ipmb_dev(file: *mut File) -> *mut IpmbDev {
    container_of((*file).private_data, IpmbDev, miscdev)
}

unsafe fn ipmb_read(file: *mut File, buf: *mut u8, mut count: usize, _ppos: *mut LoFFT) -> Isize {
    let ipmb_dev = to_ipmb_dev(file);
    let mut ret: Isize = 0;
    let mut msg: IpmbMsg = core::mem::zeroed();

    spin_lock_irq(&mut (*ipmb_dev).lock);
    while list_empty(&(*ipmb_dev).request_queue) {
        spin_unlock_irq(&mut (*ipmb_dev).lock);
        if (*file).f_flags & O_NONBLOCK != 0 { return -EAGAIN as Isize; }
        ret = wait_event_interruptible(&mut (*ipmb_dev).wait_queue,
            !list_empty(&(*ipmb_dev).request_queue));
        if ret != 0 { return ret; }
        spin_lock_irq(&mut (*ipmb_dev).lock);
    }

    let queue_elem = list_first_entry(&mut (*ipmb_dev).request_queue,
        IpmbRequestElem, list);
    core::ptr::copy_nonoverlapping(&(*queue_elem).request, &mut msg, 1);
    list_del(&mut (*queue_elem).list);
    kfree(queue_elem as *mut core::ffi::c_void);
    atomic_dec(&mut (*ipmb_dev).request_queue_len);
    spin_unlock_irq(&mut (*ipmb_dev).lock);

    count = core::cmp::min(count, msg.len as usize + 1);
    if copy_to_user(buf, &msg as *const _ as *const u8, count) != 0 { ret = -EFAULT as Isize; }
    if ret < 0 { ret } else { count as Isize }
}

unsafe fn ipmb_i2c_write(client: *mut I2cClient, msg: *mut u8, addr: u8) -> i32 {
    let mut i2c_msg: I2cMsg = core::mem::zeroed();
    i2c_msg.len = *msg.add(IPMB_MSG_LEN_IDX) - 1;
    i2c_msg.buf = msg.add(2);
    i2c_msg.addr = addr;
    i2c_msg.flags = (*client).flags & I2C_CLIENT_PEC;
    i2c_transfer((*client).adapter, &mut i2c_msg, 1)
}

unsafe fn ipmb_write(file: *mut File, buf: *const u8, count: usize, _ppos: *mut LoFFT) -> Isize {
    let ipmb_dev = to_ipmb_dev(file);
    let mut msg = [0u8; MAX_MSG_LEN];
    if count == 0 || count > msg.len() { return -EINVAL as Isize; }
    if copy_from_user(msg.as_mut_ptr(), buf, count) != 0 { return -EFAULT as Isize; }
    if msg[IPMB_MSG_LEN_IDX] < IPMB_REQUEST_LEN_MIN || count < msg[IPMB_MSG_LEN_IDX] as usize + 1 { return -EINVAL as Isize; }

    let rq_sa = get_7bit_addr(msg[RQ_SA_8BIT_IDX]);
    let netf_rq_lun = msg[NETFN_LUN_IDX];
    if (*ipmb_dev).is_i2c_protocol {
        let ret = ipmb_i2c_write((*ipmb_dev).client, msg.as_mut_ptr(), rq_sa);
        return if ret == 1 { count as Isize } else { ret as Isize };
    }
    let msg_len = msg[IPMB_MSG_LEN_IDX] - SMBUS_MSG_HEADER_LENGTH as u8;
    let temp_client = kmemdup((*ipmb_dev).client, core::mem::size_of::<I2cClient>(), GFP_KERNEL);
    if temp_client.is_null() { return -ENOMEM as Isize; }
    (*temp_client).addr = rq_sa;
    let ret = i2c_smbus_write_block_data(temp_client, netf_rq_lun, msg_len, msg.as_mut_ptr().add(SMBUS_MSG_IDX_OFFSET));
    kfree(temp_client as *mut core::ffi::c_void);
    if ret < 0 { ret as Isize } else { count as Isize }
}

unsafe fn ipmb_poll(file: *mut File, wait: *mut PollTable) -> PollT {
    let ipmb_dev = to_ipmb_dev(file);
    let mut mask: PollT = EPOLLOUT;
    mutex_lock(&mut (*ipmb_dev).file_mutex);
    poll_wait(file, &mut (*ipmb_dev).wait_queue, wait);
    if atomic_read(&(*ipmb_dev).request_queue_len) != 0 { mask |= EPOLLIN; }
    mutex_unlock(&mut (*ipmb_dev).file_mutex);
    mask
}

static IPMB_FOPS: FileOperations = FileOperations { owner: THIS_MODULE, read: Some(ipmb_read), write: Some(ipmb_write), poll: Some(ipmb_poll) };

unsafe fn ipmb_handle_request(ipmb_dev: *mut IpmbDev) {
    if atomic_read(&(*ipmb_dev).request_queue_len) >= REQUEST_QUEUE_MAX_LEN { return; }
    let queue_elem = kmalloc_obj::<IpmbRequestElem>(GFP_ATOMIC);
    if queue_elem.is_null() { return; }
    core::ptr::copy_nonoverlapping(&(*ipmb_dev).request, &mut (*queue_elem).request, 1);
    list_add(&mut (*queue_elem).list, &mut (*ipmb_dev).request_queue);
    atomic_inc(&mut (*ipmb_dev).request_queue_len);
    wake_up_all(&mut (*ipmb_dev).wait_queue);
}

unsafe fn ipmb_verify_checksum1(ipmb_dev: *mut IpmbDev, rs_sa: u8) -> u8 {
    rs_sa.wrapping_add((*ipmb_dev).request.netfn_rs_lun).wrapping_add((*ipmb_dev).request.checksum1)
}

unsafe fn is_ipmb_msg(ipmb_dev: *mut IpmbDev, rs_sa: u8) -> bool {
    (*ipmb_dev).msg_idx >= IPMB_REQUEST_LEN_MIN as usize && ipmb_verify_checksum1(ipmb_dev, rs_sa) == 0
}

unsafe fn ipmb_slave_cb(client: *mut I2cClient, event: I2cSlaveEvent, val: *mut u8) -> i32 {
    let ipmb_dev = i2c_get_clientdata(client);
    let buf = &mut (*ipmb_dev).request as *mut IpmbMsg as *mut u8;
    let flags: &mut CULong = &mut core::mem::zeroed();
    spin_lock_irqsave(&mut (*ipmb_dev).lock, flags);
    match event {
        I2cSlaveEvent::WriteRequested => {
            core::ptr::write_bytes(&mut (*ipmb_dev).request, 0, 1);
            (*ipmb_dev).msg_idx = 0;
            (*buf.add({ (*ipmb_dev).msg_idx += 1; (*ipmb_dev).msg_idx })) = get_8bit_addr((*client).addr);
        }
        I2cSlaveEvent::WriteReceived => {
            if (*ipmb_dev).msg_idx < core::mem::size_of::<IpmbMsg>() - 1 {
                (*ipmb_dev).msg_idx += 1;
                *buf.add((*ipmb_dev).msg_idx) = *val;
            }
        }
        I2cSlaveEvent::Stop => {
            (*ipmb_dev).request.len = (*ipmb_dev).msg_idx as u8;
            if is_ipmb_msg(ipmb_dev, get_8bit_addr((*client).addr)) { ipmb_handle_request(ipmb_dev); }
        }
        _ => {}
    }
    spin_unlock_irqrestore(&mut (*ipmb_dev).lock, *flags);
    0
}

// The remaining probe/remove and driver registration declarations retain the
// original kernel integration points and are supplied by surrounding bindings.
unsafe fn ipmb_probe(client: *mut I2cClient) -> i32 {
    let ipmb_dev = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<IpmbDev>(), GFP_KERNEL);
    if ipmb_dev.is_null() { return -ENOMEM; }
    spin_lock_init(&mut (*ipmb_dev).lock);
    init_waitqueue_head(&mut (*ipmb_dev).wait_queue);
    atomic_set(&mut (*ipmb_dev).request_queue_len, 0);
    init_list_head(&mut (*ipmb_dev).request_queue);
    mutex_init(&mut (*ipmb_dev).file_mutex);
    (*ipmb_dev).miscdev.minor = MISC_DYNAMIC_MINOR;
    (*ipmb_dev).miscdev.name = devm_kasprintf(&mut (*client).dev, GFP_KERNEL, "ipmb-{}", (*(*client).adapter).nr);
    if (*ipmb_dev).miscdev.name.is_null() { return -ENOMEM; }
    (*ipmb_dev).miscdev.fops = &IPMB_FOPS;
    (*ipmb_dev).miscdev.parent = &mut (*client).dev;
    let mut ret = misc_register(&mut (*ipmb_dev).miscdev);
    if ret != 0 { return ret; }
    (*ipmb_dev).is_i2c_protocol = device_property_read_bool(&mut (*client).dev, "i2c-protocol");
    (*ipmb_dev).client = client;
    i2c_set_clientdata(client, ipmb_dev);
    ret = i2c_slave_register(client, ipmb_slave_cb);
    if ret != 0 { misc_deregister(&mut (*ipmb_dev).miscdev); return ret; }
    0
}

unsafe fn ipmb_remove(client: *mut I2cClient) {
    let ipmb_dev = i2c_get_clientdata(client);
    i2c_slave_unregister(client);
    misc_deregister(&mut (*ipmb_dev).miscdev);
}

// MODULE_DEVICE_TABLE(i2c, ipmb_id); MODULE_DEVICE_TABLE(acpi, acpi_ipmb_id);
// module_i2c_driver(ipmb_driver);
// MODULE_AUTHOR("Mellanox Technologies"); MODULE_DESCRIPTION("IPMB driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
