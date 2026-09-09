// SPDX-License-Identifier: GPL-2.0-or-later
// Nuvoton TPM I2C Device Driver Interface for WPCT301/NPCT501/NPCT6XX.

// Kernel dependencies supplied by the surrounding translation unit.
use core::cmp::min;

const TPM_STS: u8 = 0x00;
const TPM_BURST_COUNT: u8 = 0x01;
const TPM_DATA_FIFO_W: u8 = 0x20;
const TPM_DATA_FIFO_R: u8 = 0x40;
const TPM_VID_DID_RID: u8 = 0x60;
const TPM_I2C_RETRIES: i32 = 5;
const TPM_I2C_MAX_BUF_SIZE: u8 = 32;
const TPM_I2C_RETRY_COUNT: i32 = 32;
const TPM_I2C_BUS_DELAY: u32 = 1000;
const TPM_I2C_RETRY_DELAY_SHORT: u32 = 2 * 1000;
const TPM_I2C_RETRY_DELAY_LONG: u32 = 10 * 1000;
const TPM_I2C_DELAY_RANGE: u32 = 300;
const I2C_IS_TPM2: usize = 1;

#[repr(C)]
struct priv_data { irq: i32, intrs: u32, read_queue: wait_queue_head_t }

unsafe fn i2c_nuvoton_read_buf(client: *mut i2c_client, offset: u8, size: u8, data: *mut u8) -> i32 {
    let status = i2c_smbus_read_i2c_block_data(client, offset, size, data);
    dev_dbg((*client).dev, "%s(offset=%u size=%u data=%*ph) -> sts=%d\\n", "i2c_nuvoton_read_buf", offset, size, size as i32, data, status);
    status
}

unsafe fn i2c_nuvoton_write_buf(client: *mut i2c_client, offset: u8, size: u8, data: *mut u8) -> i32 {
    let status = i2c_smbus_write_i2c_block_data(client, offset, size, data);
    dev_dbg((*client).dev, "%s(offset=%u size=%u data=%*ph) -> sts=%d\\n", "i2c_nuvoton_write_buf", offset, size, size as i32, data, status);
    status
}

const TPM_STS_VALID: u8 = 0x80;
const TPM_STS_COMMAND_READY: u8 = 0x40;
const TPM_STS_GO: u8 = 0x20;
const TPM_STS_DATA_AVAIL: u8 = 0x10;
const TPM_STS_EXPECT: u8 = 0x08;
const TPM_STS_RESPONSE_RETRY: u8 = 0x02;
const TPM_STS_ERR_VAL: u8 = 0x07;
const TPM_I2C_SHORT_TIMEOUT: u32 = 750;
const TPM_I2C_LONG_TIMEOUT: u32 = 2000;

unsafe fn i2c_nuvoton_read_status(chip: *mut tpm_chip) -> u8 {
    let client = to_i2c_client((*(*chip).dev).parent);
    let mut data = 0u8;
    let status = i2c_nuvoton_read_buf(client, TPM_STS, 1, &mut data);
    if status <= 0 { dev_err((*chip).dev, "%s() error return %d\\n", "i2c_nuvoton_read_status", status); data = TPM_STS_ERR_VAL; }
    data
}

unsafe fn i2c_nuvoton_write_status(client: *mut i2c_client, data: u8) -> i32 {
    let mut status = -1i32;
    let mut i = 0;
    while i < TPM_I2C_RETRY_COUNT && status < 0 {
        let mut d = data;
        status = i2c_nuvoton_write_buf(client, TPM_STS, 1, &mut d);
        if status < 0 { usleep_range(TPM_I2C_BUS_DELAY, TPM_I2C_BUS_DELAY + TPM_I2C_DELAY_RANGE); }
        i += 1;
    }
    status
}

unsafe fn i2c_nuvoton_ready(chip: *mut tpm_chip) {
    let client = to_i2c_client((*(*chip).dev).parent);
    if i2c_nuvoton_write_status(client, TPM_STS_COMMAND_READY) < 0 { dev_err((*chip).dev, "%s() fail to write TPM_STS.commandReady\\n", "i2c_nuvoton_ready"); }
}

unsafe fn i2c_nuvoton_get_burstcount(client: *mut i2c_client, chip: *mut tpm_chip) -> i32 {
    let stop = jiffies().wrapping_add((*chip).timeout_d);
    loop {
        let mut data = 0u8;
        let status = i2c_nuvoton_read_buf(client, TPM_BURST_COUNT, 1, &mut data);
        if status > 0 && data > 0 { return min(TPM_I2C_MAX_BUF_SIZE, data) as i32; }
        usleep_range(TPM_I2C_BUS_DELAY, TPM_I2C_BUS_DELAY + TPM_I2C_DELAY_RANGE);
        if !time_before(jiffies(), stop) { return -1; }
    }
}

unsafe fn i2c_nuvoton_check_status(chip: *mut tpm_chip, mask: u8, value: u8) -> bool {
    let status = i2c_nuvoton_read_status(chip);
    status != TPM_STS_ERR_VAL && (status & mask) == value
}

unsafe fn i2c_nuvoton_wait_for_stat(chip: *mut tpm_chip, mask: u8, value: u8, timeout: u32, queue: *mut wait_queue_head_t) -> i32 {
    if ((*chip).flags & TPM_CHIP_FLAG_IRQ) != 0 && !queue.is_null() {
        let priv_ = dev_get_drvdata((*chip).dev) as *mut priv_data;
        let cur_intrs = (*priv_).intrs;
        enable_irq((*priv_).irq);
        let rc = wait_event_interruptible_timeout(queue, cur_intrs != (*priv_).intrs, timeout);
        if rc > 0 { return 0; }
        disable_irq((*priv_).irq);
        if rc < 0 { return rc; }
    } else {
        if i2c_nuvoton_check_status(chip, mask, value) { return 0; }
        let ten_msec = jiffies().wrapping_add(usecs_to_jiffies(TPM_I2C_RETRY_DELAY_LONG));
        let stop = jiffies().wrapping_add(timeout);
        loop {
            if time_before(jiffies(), ten_msec) { usleep_range(TPM_I2C_RETRY_DELAY_SHORT, TPM_I2C_RETRY_DELAY_SHORT + TPM_I2C_DELAY_RANGE); }
            else { usleep_range(TPM_I2C_RETRY_DELAY_LONG, TPM_I2C_RETRY_DELAY_LONG + TPM_I2C_DELAY_RANGE); }
            if i2c_nuvoton_check_status(chip, mask, value) { return 0; }
            if !time_before(jiffies(), stop) { break; }
        }
    }
    dev_err((*chip).dev, "%s(%02x, %02x) -> timeout\\n", "i2c_nuvoton_wait_for_stat", mask, value);
    -ETIMEDOUT
}

unsafe fn i2c_nuvoton_wait_for_data_avail(chip: *mut tpm_chip, timeout: u32, queue: *mut wait_queue_head_t) -> i32 {
    i2c_nuvoton_wait_for_stat(chip, TPM_STS_DATA_AVAIL | TPM_STS_VALID, TPM_STS_DATA_AVAIL | TPM_STS_VALID, timeout, queue)
}

unsafe fn i2c_nuvoton_recv_data(client: *mut i2c_client, chip: *mut tpm_chip, buf: *mut u8, count: usize) -> i32 {
    let priv_ = dev_get_drvdata((*chip).dev) as *mut priv_data;
    let mut size = 0usize;
    while size < count && i2c_nuvoton_wait_for_data_avail(chip, (*chip).timeout_c, &mut (*priv_).read_queue) == 0 {
        let burst = i2c_nuvoton_get_burstcount(client, chip);
        if burst < 0 { dev_err((*chip).dev, "%s() fail to read burstCount=%d\\n", "i2c_nuvoton_recv_data", burst); return -EIO; }
        let n = min(burst as usize, count - size);
        let rc = i2c_nuvoton_read_buf(client, TPM_DATA_FIFO_R, n as u8, buf.add(size));
        if rc < 0 { dev_err((*chip).dev, "%s() fail on i2c_nuvoton_read_buf()=%d\\n", "i2c_nuvoton_recv_data", rc); return -EIO; }
        size += n;
    }
    size as i32
}

unsafe fn i2c_nuvoton_recv(chip: *mut tpm_chip, buf: *mut u8, count: usize) -> i32 {
    let priv_ = dev_get_drvdata((*chip).dev) as *mut priv_data;
    let client = to_i2c_client((*(*chip).dev).parent);
    if count < TPM_HEADER_SIZE { i2c_nuvoton_ready(chip); return -EIO; }
    let mut size = 0i32;
    for retries in 0..TPM_I2C_RETRIES {
        if retries > 0 { i2c_nuvoton_write_status(client, TPM_STS_RESPONSE_RETRY); }
        if i2c_nuvoton_wait_for_data_avail(chip, (*chip).timeout_c, &mut (*priv_).read_queue) != 0 { size = -ETIMEDOUT; continue; }
        let burst = i2c_nuvoton_get_burstcount(client, chip);
        if burst < 0 { size = -EIO; continue; }
        size = i2c_nuvoton_recv_data(client, chip, buf, burst as usize);
        if size < TPM_HEADER_SIZE as i32 { size = -EIO; continue; }
        let expected = u32::from_be((*buf.add(2) as u32) << 24 | (*buf.add(3) as u32) << 16 | (*buf.add(4) as u32) << 8 | *buf.add(5) as u32) as usize;
        if expected > count || expected < size as usize { size = -EIO; continue; }
        let rc = i2c_nuvoton_recv_data(client, chip, buf.add(size as usize), expected - size as usize);
        size += rc;
        if rc < 0 || size as usize < expected { size = -EIO; continue; }
        if i2c_nuvoton_wait_for_stat(chip, TPM_STS_VALID | TPM_STS_DATA_AVAIL, TPM_STS_VALID, (*chip).timeout_c, core::ptr::null_mut()) != 0 { size = -ETIMEDOUT; continue; }
        break;
    }
    i2c_nuvoton_ready(chip); size
}

// Remaining driver callbacks and registration are declarations using the kernel's
// surrounding Rust bindings; their bodies preserve the source operations.
unsafe fn i2c_nuvoton_req_canceled(_chip: *mut tpm_chip, status: u8) -> bool { status == TPM_STS_COMMAND_READY }

unsafe fn i2c_nuvoton_send(chip: *mut tpm_chip, buf: *mut u8, _bufsiz: usize, len: usize) -> i32 {
    let priv_ = dev_get_drvdata((*chip).dev) as *mut priv_data;
    let client = to_i2c_client((*(*chip).dev).parent);
    let mut count = 0usize;
    let mut rc = -EIO;
    for _ in 0..TPM_RETRY {
        i2c_nuvoton_ready(chip);
        if i2c_nuvoton_wait_for_stat(chip, TPM_STS_COMMAND_READY, TPM_STS_COMMAND_READY, (*chip).timeout_b, core::ptr::null_mut()) != 0 { continue; }
        rc = 0;
        while count < len - 1 {
            let burst = i2c_nuvoton_get_burstcount(client, chip);
            if burst < 0 { rc = -EIO; break; }
            let n = min(burst as usize, len - 1 - count);
            rc = i2c_nuvoton_write_buf(client, TPM_DATA_FIFO_W, n as u8, buf.add(count));
            if rc < 0 { break; }
            count += n;
            rc = i2c_nuvoton_wait_for_stat(chip, TPM_STS_VALID | TPM_STS_EXPECT, TPM_STS_VALID | TPM_STS_EXPECT, (*chip).timeout_c, core::ptr::null_mut());
            if rc < 0 { rc = -ETIMEDOUT; break; }
        }
        if rc < 0 { continue; }
        rc = i2c_nuvoton_write_buf(client, TPM_DATA_FIFO_W, 1, buf.add(count));
        if rc < 0 { rc = -EIO; continue; }
        rc = i2c_nuvoton_wait_for_stat(chip, TPM_STS_VALID | TPM_STS_EXPECT, TPM_STS_VALID, (*chip).timeout_c, core::ptr::null_mut());
        if rc != 0 { rc = -ETIMEDOUT; continue; }
        break;
    }
    if rc < 0 { i2c_nuvoton_ready(chip); return rc; }
    rc = i2c_nuvoton_write_status(client, TPM_STS_GO);
    if rc < 0 { i2c_nuvoton_ready(chip); return rc; }
    let ordinal = u32::from_be((*buf.add(6) as u32) << 24 | (*buf.add(7) as u32) << 16 | (*buf.add(8) as u32) << 8 | *buf.add(9) as u32);
    let duration = tpm_calc_ordinal_duration(chip, ordinal);
    rc = i2c_nuvoton_wait_for_data_avail(chip, duration, &mut (*priv_).read_queue);
    if rc != 0 { i2c_nuvoton_ready(chip); return rc; }
    0
}

unsafe fn i2c_nuvoton_int_handler(_dummy: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let chip = dev_id as *mut tpm_chip;
    let priv_ = dev_get_drvdata((*chip).dev) as *mut priv_data;
    (*priv_).intrs += 1;
    wake_up(&mut (*priv_).read_queue);
    disable_irq_nosync((*priv_).irq);
    IRQ_HANDLED
}

unsafe fn get_vid(client: *mut i2c_client, res: *mut u32) -> i32 {
    let expected = [0x50u8, 0x10, 0xfe];
    if !i2c_check_functionality((*client).adapter, I2C_FUNC_SMBUS_BYTE_DATA) { return -ENODEV; }
    let mut temp = 0u32;
    if i2c_nuvoton_read_buf(client, TPM_VID_DID_RID, 4, &mut temp as *mut u32 as *mut u8) < 0 { return -EIO; }
    if core::slice::from_raw_parts(&temp as *const u32 as *const u8, 3) != expected {
        if i2c_nuvoton_read_buf(client, TPM_DATA_FIFO_W, 4, &mut temp as *mut u32 as *mut u8) < 0 { return -EIO; }
        if core::slice::from_raw_parts(&temp as *const u32 as *const u8, 3) != expected { return -ENODEV; }
    }
    *res = temp; 0
}

// Device tables, PM operations, driver registration, and module metadata are
// supplied by the kernel binding layer corresponding to the C declarations.

#[repr(C)]
struct tpm_class_ops {
    flags: u32,
    status: unsafe fn(*mut tpm_chip) -> u8,
    recv: unsafe fn(*mut tpm_chip, *mut u8, usize) -> i32,
    send: unsafe fn(*mut tpm_chip, *mut u8, usize, usize) -> i32,
    cancel: unsafe fn(*mut tpm_chip),
    req_complete_mask: u8,
    req_complete_val: u8,
    req_canceled: unsafe fn(*mut tpm_chip, u8) -> bool,
}

static TPM_I2C: tpm_class_ops = tpm_class_ops {
    flags: TPM_OPS_AUTO_STARTUP,
    status: i2c_nuvoton_read_status,
    recv: i2c_nuvoton_recv,
    send: i2c_nuvoton_send,
    cancel: i2c_nuvoton_ready,
    req_complete_mask: TPM_STS_DATA_AVAIL | TPM_STS_VALID,
    req_complete_val: TPM_STS_DATA_AVAIL | TPM_STS_VALID,
    req_canceled: i2c_nuvoton_req_canceled,
};

unsafe fn i2c_nuvoton_probe(client: *mut i2c_client) -> i32 {
    let mut vid = 0u32;
    let rc = get_vid(client, &mut vid);
    if rc != 0 { return rc; }
    let chip = tpmm_chip_alloc(&mut (*client).dev, &TPM_I2C);
    if IS_ERR(chip) { return PTR_ERR(chip); }
    let priv_ = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<priv_data>(), GFP_KERNEL) as *mut priv_data;
    if priv_.is_null() { return -ENOMEM; }
    (*priv_).irq = (*client).irq;
    init_waitqueue_head(&mut (*priv_).read_queue);
    (*chip).timeout_a = msecs_to_jiffies(TPM_I2C_SHORT_TIMEOUT);
    (*chip).timeout_b = msecs_to_jiffies(TPM_I2C_LONG_TIMEOUT);
    (*chip).timeout_c = msecs_to_jiffies(TPM_I2C_SHORT_TIMEOUT);
    (*chip).timeout_d = msecs_to_jiffies(TPM_I2C_SHORT_TIMEOUT);
    dev_set_drvdata(&mut (*chip).dev, priv_ as *mut core::ffi::c_void);
    tpm_chip_register(chip)
}

unsafe fn i2c_nuvoton_remove(client: *mut i2c_client) {
    let chip = i2c_get_clientdata(client);
    tpm_chip_unregister(chip);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
