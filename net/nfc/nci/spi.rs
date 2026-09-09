// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2013  Intel Corporation. All rights reserved.
 */

// Linux kernel dependencies and build-time configuration are supplied by other
// translation units.

const NCI_SPI_ACK_SHIFT: u32 = 6;
const NCI_SPI_MSB_PAYLOAD_MASK: u8 = 0x3f;
const NCI_SPI_DIRECT_WRITE: u8 = 0x01;
const NCI_SPI_DIRECT_READ: u8 = 0x02;
const ACKNOWLEDGE_NONE: u8 = 0;
const ACKNOWLEDGE_ACK: u8 = 1;
const ACKNOWLEDGE_NACK: u8 = 2;
const CRC_INIT: u16 = 0xffff;

unsafe fn __nci_spi_send(
    nspi: *mut nci_spi,
    skb: *const sk_buff,
    cs_change: u8,
) -> i32 {
    let mut m: spi_message = core::mem::zeroed();
    let mut t: spi_transfer = core::mem::zeroed();

    if !skb.is_null() {
        t.tx_buf = (*skb).data as *const core::ffi::c_void;
        t.len = (*skb).len;
    } else {
        t.tx_buf = &t as *const spi_transfer as *const core::ffi::c_void;
        t.len = 0;
    }
    t.cs_change = cs_change;
    t.delay.value = (*nspi).xfer_udelay;
    t.delay.unit = SPI_DELAY_UNIT_USECS;
    t.speed_hz = (*nspi).xfer_speed_hz;

    spi_message_init(&mut m);
    spi_message_add_tail(&mut t, &mut m);
    spi_sync((*nspi).spi, &mut m)
}

pub unsafe fn nci_spi_send(
    nspi: *mut nci_spi,
    write_handshake_completion: *mut completion,
    skb: *mut sk_buff,
) -> i32 {
    let payload_len: usize = (*skb).len;
    let hdr = skb_push(skb, NCI_SPI_HDR_LEN) as *mut u8;
    *hdr.add(0) = NCI_SPI_DIRECT_WRITE;
    *hdr.add(1) = (*nspi).acknowledge_mode;
    *hdr.add(2) = (payload_len >> 8) as u8;
    *hdr.add(3) = payload_len as u8;

    if (*nspi).acknowledge_mode == NCI_SPI_CRC_ENABLED {
        let crc = crc_ccitt(CRC_INIT, (*skb).data, (*skb).len);
        skb_put_u8(skb, (crc >> 8) as u8);
        skb_put_u8(skb, crc as u8);
    }

    let mut ret: i32;
    if !write_handshake_completion.is_null() {
        ret = __nci_spi_send(nspi, core::ptr::null(), 1);
        if ret != 0 {
            kfree_skb(skb);
            return ret;
        }
        if wait_for_completion_timeout(write_handshake_completion, msecs_to_jiffies(1000)) == 0 {
            ret = -ETIME;
            kfree_skb(skb);
            return ret;
        }
    }

    ret = __nci_spi_send(nspi, skb, 0);
    if ret == 0 && (*nspi).acknowledge_mode != NCI_SPI_CRC_DISABLED {
        reinit_completion(&mut (*nspi).req_completion);
        let completion_rc = wait_for_completion_interruptible_timeout(
            &mut (*nspi).req_completion,
            NCI_SPI_SEND_TIMEOUT,
        );
        if completion_rc <= 0 || (*nspi).req_result == ACKNOWLEDGE_NACK {
            ret = -EIO;
        }
    }
    kfree_skb(skb);
    ret
}

pub unsafe fn nci_spi_allocate_spi(
    spi: *mut spi_device,
    acknowledge_mode: u8,
    delay: u32,
    ndev: *mut nci_dev,
) -> *mut nci_spi {
    let nspi = devm_kzalloc(&mut (*spi).dev, core::mem::size_of::<nci_spi>(), GFP_KERNEL)
        as *mut nci_spi;
    if nspi.is_null() {
        return core::ptr::null_mut();
    }
    (*nspi).acknowledge_mode = acknowledge_mode;
    (*nspi).xfer_udelay = delay;
    (*nspi).xfer_speed_hz = 0;
    (*nspi).spi = spi;
    (*nspi).ndev = ndev;
    init_completion(&mut (*nspi).req_completion);
    nspi
}

unsafe fn send_acknowledge(nspi: *mut nci_spi, acknowledge: u8) -> i32 {
    let skb = nci_skb_alloc((*nspi).ndev, 0, GFP_KERNEL);
    if skb.is_null() {
        return -ENOMEM;
    }
    let hdr = skb_push(skb, NCI_SPI_HDR_LEN) as *mut u8;
    *hdr.add(0) = NCI_SPI_DIRECT_WRITE;
    *hdr.add(1) = NCI_SPI_CRC_ENABLED;
    *hdr.add(2) = acknowledge << NCI_SPI_ACK_SHIFT;
    *hdr.add(3) = 0;
    let crc = crc_ccitt(CRC_INIT, (*skb).data, (*skb).len);
    skb_put_u8(skb, (crc >> 8) as u8);
    skb_put_u8(skb, crc as u8);
    let ret = __nci_spi_send(nspi, skb, 0);
    kfree_skb(skb);
    ret
}

unsafe fn __nci_spi_read(nspi: *mut nci_spi) -> *mut sk_buff {
    let mut m: spi_message = core::mem::zeroed();
    let mut req = [0u8; 2];
    let mut resp_hdr = [0u8; 2];
    let mut tx: spi_transfer = core::mem::zeroed();
    let mut rx: spi_transfer = core::mem::zeroed();
    spi_message_init(&mut m);
    req[0] = NCI_SPI_DIRECT_READ;
    req[1] = (*nspi).acknowledge_mode;
    tx.tx_buf = req.as_ptr() as *const core::ffi::c_void;
    tx.len = 2;
    tx.speed_hz = (*nspi).xfer_speed_hz;
    spi_message_add_tail(&mut tx, &mut m);
    rx.rx_buf = resp_hdr.as_mut_ptr() as *mut core::ffi::c_void;
    rx.len = 2;
    rx.cs_change = 1;
    rx.speed_hz = (*nspi).xfer_speed_hz;
    spi_message_add_tail(&mut rx, &mut m);
    if spi_sync((*nspi).spi, &mut m) != 0 {
        return core::ptr::null_mut();
    }
    let rx_len = if (*nspi).acknowledge_mode == NCI_SPI_CRC_ENABLED {
        (((resp_hdr[0] & NCI_SPI_MSB_PAYLOAD_MASK) as u16) << 8)
            + resp_hdr[1] as u16 + NCI_SPI_CRC_LEN
    } else {
        ((resp_hdr[0] as u16) << 8) | resp_hdr[1] as u16
    };
    let skb = nci_skb_alloc((*nspi).ndev, rx_len as usize, GFP_KERNEL);
    if skb.is_null() {
        return core::ptr::null_mut();
    }
    spi_message_init(&mut m);
    rx = core::mem::zeroed();
    rx.rx_buf = skb_put(skb, rx_len as usize) as *mut core::ffi::c_void;
    rx.len = rx_len as usize;
    rx.delay.value = (*nspi).xfer_udelay;
    rx.delay.unit = SPI_DELAY_UNIT_USECS;
    rx.speed_hz = (*nspi).xfer_speed_hz;
    spi_message_add_tail(&mut rx, &mut m);
    if spi_sync((*nspi).spi, &mut m) != 0 {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }
    if (*nspi).acknowledge_mode == NCI_SPI_CRC_ENABLED {
        *(skb_push(skb, 1) as *mut u8) = resp_hdr[1];
        *(skb_push(skb, 1) as *mut u8) = resp_hdr[0];
    }
    skb
}

unsafe fn nci_spi_check_crc(skb: *mut sk_buff) -> i32 {
    let crc_data = ((*(*skb).data.add((*skb).len - 2) as u16) << 8)
        | *(*skb).data.add((*skb).len - 1) as u16;
    let ret = (crc_ccitt(CRC_INIT, (*skb).data, (*skb).len - NCI_SPI_CRC_LEN) == crc_data) as i32;
    skb_trim(skb, (*skb).len - NCI_SPI_CRC_LEN);
    ret
}

unsafe fn nci_spi_get_ack(skb: *mut sk_buff) -> u8 {
    let ret = *(*skb).data >> NCI_SPI_ACK_SHIFT;
    skb_pull(skb, 2);
    ret
}

pub unsafe fn nci_spi_read(nspi: *mut nci_spi) -> *mut sk_buff {
    let skb = __nci_spi_read(nspi);
    if skb.is_null() {
        return core::ptr::null_mut();
    }
    if (*nspi).acknowledge_mode == NCI_SPI_CRC_ENABLED {
        if nci_spi_check_crc(skb) == 0 {
            send_acknowledge(nspi, ACKNOWLEDGE_NACK);
            kfree_skb(skb);
            return core::ptr::null_mut();
        }
        (*nspi).req_result = nci_spi_get_ack(skb);
        if (*nspi).req_result != 0 {
            complete(&mut (*nspi).req_completion);
        }
    }
    if (*skb).len == 0 {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }
    if (*nspi).acknowledge_mode == NCI_SPI_CRC_ENABLED {
        send_acknowledge(nspi, ACKNOWLEDGE_ACK);
    }
    skb
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
