// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Infineon Technologies AG
 * Copyright (C) 2016 STMicroelectronics SAS
 *
 * Device driver for TCG/TCPA TPM (trusted platform module), using raw/native
 * SPI access.  This is a direct low-level translation of the C implementation.
 */

// Kernel headers and the local TPM/SPI definitions are supplied by dependent
// translation units.

const MAX_SPI_FRAMESIZE: usize = 64;
const SPI_HDRSIZE: usize = 4;

unsafe fn tpm_tis_spi_flow_control(
    phy: *mut tpm_tis_spi_phy,
    spi_xfer: *mut spi_transfer,
) -> i32 {
    let mut m: spi_message = core::mem::zeroed();
    let mut ret: i32;
    let mut i: i32;

    if ((*(*phy).iobuf.add(3) & 0x01) == 0) {
        for i in 0..TPM_RETRY {
            (*spi_xfer).len = 1;
            spi_message_init(&mut m);
            spi_message_add_tail(spi_xfer, &mut m);
            ret = spi_sync_locked((*phy).spi_device, &mut m);
            if ret < 0 {
                return ret;
            }
            if (*(*phy).iobuf & 0x01) != 0 {
                break;
            }
        }
        if i == TPM_RETRY {
            return -ETIMEDOUT;
        }
    }
    0
}

unsafe fn tpm_tis_spi_transfer_half(
    data: *mut tpm_tis_data,
    mut addr: u32,
    mut len: u16,
    mut input: *mut u8,
    mut output: *const u8,
) -> i32 {
    let phy = to_tpm_tis_spi_phy(data);
    let mut spi_xfer: [spi_transfer; 3] = core::mem::zeroed();
    let mut m: spi_message = core::mem::zeroed();
    let mut transfer_len: u8;
    let mut ret: i32 = 0;

    while len != 0 {
        transfer_len = core::cmp::min(len, MAX_SPI_FRAMESIZE as u16) as u8;
        spi_message_init(&mut m);
        (*phy).iobuf[0] = if !input.is_null() { 0x80 } else { 0 } | (transfer_len - 1);
        (*phy).iobuf[1] = 0xd4;
        (*phy).iobuf[2] = (addr >> 8) as u8;
        (*phy).iobuf[3] = addr as u8;
        spi_xfer = core::mem::zeroed();
        spi_xfer[0].tx_buf = (*phy).iobuf.as_ptr();
        spi_xfer[0].len = 1;
        spi_message_add_tail(&mut spi_xfer[0], &mut m);
        spi_xfer[1].tx_buf = (*phy).iobuf.as_ptr().add(1);
        spi_xfer[1].len = 3;
        spi_message_add_tail(&mut spi_xfer[1], &mut m);
        if !output.is_null() {
            spi_xfer[2].tx_buf = (*phy).iobuf.as_ptr().add(4);
            spi_xfer[2].rx_buf = core::ptr::null_mut();
            core::ptr::copy_nonoverlapping(output, (*phy).iobuf.as_mut_ptr().add(4), transfer_len as usize);
            output = output.add(transfer_len as usize);
        }
        if !input.is_null() {
            spi_xfer[2].tx_buf = core::ptr::null();
            spi_xfer[2].rx_buf = (*phy).iobuf.as_mut_ptr().add(4);
        }
        spi_xfer[2].len = transfer_len as usize;
        spi_message_add_tail(&mut spi_xfer[2], &mut m);
        reinit_completion(&mut (*phy).ready);
        ret = spi_sync((*phy).spi_device, &mut m);
        if ret < 0 { return ret; }
        if !input.is_null() {
            core::ptr::copy_nonoverlapping((*phy).iobuf.as_ptr().add(4), input, transfer_len as usize);
            input = input.add(transfer_len as usize);
        }
        len -= transfer_len as u16;
        addr += transfer_len as u32;
    }
    ret
}

unsafe fn tpm_tis_spi_transfer_full(
    data: *mut tpm_tis_data, mut addr: u32, mut len: u16,
    mut input: *mut u8, mut output: *const u8,
) -> i32 {
    let phy = to_tpm_tis_spi_phy(data);
    let mut ret = 0;
    let mut m: spi_message = core::mem::zeroed();
    let mut spi_xfer: spi_transfer = core::mem::zeroed();
    spi_bus_lock((*(*phy).spi_device).controller);
    while len != 0 {
        let transfer_len = core::cmp::min(len, MAX_SPI_FRAMESIZE as u16) as u8;
        (*phy).iobuf[0] = if !input.is_null() { 0x80 } else { 0 } | (transfer_len - 1);
        (*phy).iobuf[1] = 0xd4; (*phy).iobuf[2] = (addr >> 8) as u8; (*phy).iobuf[3] = addr as u8;
        spi_xfer = core::mem::zeroed(); spi_xfer.tx_buf = (*phy).iobuf.as_ptr(); spi_xfer.rx_buf = (*phy).iobuf.as_mut_ptr(); spi_xfer.len = 4; spi_xfer.cs_change = 1;
        spi_message_init(&mut m); spi_message_add_tail(&mut spi_xfer, &mut m);
        ret = spi_sync_locked((*phy).spi_device, &mut m); if ret < 0 { break; }
        spi_xfer.tx_buf = core::ptr::null(); ret = ((*phy).flow_control)(phy, &mut spi_xfer); if ret < 0 { break; }
        spi_xfer.cs_change = 0; spi_xfer.len = transfer_len as usize; spi_xfer.delay.value = 5; spi_xfer.delay.unit = SPI_DELAY_UNIT_USECS;
        if !output.is_null() { spi_xfer.tx_buf = (*phy).iobuf.as_ptr(); spi_xfer.rx_buf = core::ptr::null_mut(); core::ptr::copy_nonoverlapping(output, (*phy).iobuf.as_mut_ptr(), transfer_len as usize); output = output.add(transfer_len as usize); }
        spi_message_init(&mut m); spi_message_add_tail(&mut spi_xfer, &mut m); reinit_completion(&mut (*phy).ready);
        ret = spi_sync_locked((*phy).spi_device, &mut m); if ret < 0 { break; }
        if !input.is_null() { core::ptr::copy_nonoverlapping((*phy).iobuf.as_ptr(), input, transfer_len as usize); input = input.add(transfer_len as usize); }
        len -= transfer_len as u16; addr += transfer_len as u32;
    }
    if ret < 0 { spi_xfer = core::mem::zeroed(); spi_message_init(&mut m); spi_message_add_tail(&mut spi_xfer, &mut m); spi_sync_locked((*phy).spi_device, &mut m); }
    spi_bus_unlock((*(*phy).spi_device).controller); ret
}

pub unsafe fn tpm_tis_spi_transfer(data: *mut tpm_tis_data, addr: u32, len: u16, input: *mut u8, output: *const u8) -> i32 {
    let phy = to_tpm_tis_spi_phy(data);
    if ((*(*phy).spi_device).controller.flags & SPI_CONTROLLER_HALF_DUPLEX) != 0 { tpm_tis_spi_transfer_half(data, addr, len, input, output) } else { tpm_tis_spi_transfer_full(data, addr, len, input, output) }
}

unsafe fn tpm_tis_spi_read_bytes(data: *mut tpm_tis_data, addr: u32, len: u16, result: *mut u8, _io_mode: tpm_tis_io_mode) -> i32 { tpm_tis_spi_transfer(data, addr, len, result, core::ptr::null()) }
unsafe fn tpm_tis_spi_write_bytes(data: *mut tpm_tis_data, addr: u32, len: u16, value: *const u8, _io_mode: tpm_tis_io_mode) -> i32 { tpm_tis_spi_transfer(data, addr, len, core::ptr::null_mut(), value) }

// The remaining driver registration and probe declarations are provided by the
// kernel-facing translation layer; preserve their externally visible symbols.
pub unsafe fn tpm_tis_spi_init(spi: *mut spi_device, phy: *mut tpm_tis_spi_phy, irq: i32, ops: *const tpm_tis_phy_ops) -> i32 {
    (*phy).iobuf = devm_kmalloc((*spi).dev, SPI_HDRSIZE + MAX_SPI_FRAMESIZE, GFP_KERNEL);
    if (*phy).iobuf.is_null() { return -ENOMEM; }
    (*phy).spi_device = spi;
    tpm_tis_core_init((*spi).dev, &mut (*phy).priv_data, irq, ops, core::ptr::null_mut())
}

static TpmSpiPhyOps: tpm_tis_phy_ops = tpm_tis_phy_ops {
    read_bytes: Some(tpm_tis_spi_read_bytes),
    write_bytes: Some(tpm_tis_spi_write_bytes),
};

pub unsafe fn tpm_tis_spi_probe(dev: *mut spi_device) -> i32 {
    let phy = devm_kzalloc((*dev).dev, core::mem::size_of::<tpm_tis_spi_phy>(), GFP_KERNEL) as *mut tpm_tis_spi_phy;
    if phy.is_null() { return -ENOMEM; }
    (*phy).flow_control = Some(tpm_tis_spi_flow_control);
    if ((*(*dev).controller).flags & SPI_CONTROLLER_HALF_DUPLEX) != 0 { (*dev).mode |= SPI_TPM_HW_FLOW; }
    let irq = if (*dev).irq > 0 { (*dev).irq } else { -1 };
    init_completion(&mut (*phy).ready);
    tpm_tis_spi_init(dev, phy, irq, &TpmSpiPhyOps)
}

pub unsafe fn tpm_tis_spi_driver_probe(spi: *mut spi_device) -> i32 {
    let spi_dev_id = spi_get_device_id(spi);
    let mut probe_func = of_device_get_match_data((*spi).dev);
    if probe_func.is_none() {
        probe_func = if !spi_dev_id.is_null() { (*spi_dev_id).driver_data } else { Some(tpm_tis_spi_probe) };
        if probe_func.is_none() { return -ENODEV; }
    }
    probe_func.unwrap()(spi)
}

pub unsafe fn tpm_tis_spi_remove(dev: *mut spi_device) {
    let chip = spi_get_drvdata(dev);
    tpm_chip_unregister(chip);
    tpm_tis_remove(chip);
}

// Device-ID, OF/ACPI match tables, PM operations, SPI-driver registration,
// MODULE_DEVICE_TABLE, MODULE_DESCRIPTION, and MODULE_LICENSE retain their C
// kernel-registration semantics and are emitted by the platform bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
