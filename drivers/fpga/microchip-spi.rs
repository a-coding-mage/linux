// SPDX-License-Identifier: GPL-2.0
/*
 * Microchip Polarfire FPGA programming over slave SPI interface.
 */

// Linux kernel dependencies supplied by the surrounding tree.

const MPF_SPI_ISC_ENABLE: u8 = 0x0B;
const MPF_SPI_ISC_DISABLE: u8 = 0x0C;
const MPF_SPI_READ_STATUS: u8 = 0x00;
const MPF_SPI_READ_DATA: u8 = 0x01;
const MPF_SPI_FRAME_INIT: u8 = 0xAE;
const MPF_SPI_FRAME: u8 = 0xEE;
const MPF_SPI_PRG_MODE: u8 = 0x01;
const MPF_SPI_RELEASE: u8 = 0x23;

const MPF_SPI_FRAME_SIZE: usize = 16;
const MPF_HEADER_SIZE_OFFSET: usize = 24;
const MPF_DATA_SIZE_OFFSET: usize = 55;
const MPF_LOOKUP_TABLE_RECORD_SIZE: usize = 9;
const MPF_LOOKUP_TABLE_BLOCK_ID_OFFSET: usize = 0;
const MPF_LOOKUP_TABLE_BLOCK_START_OFFSET: usize = 1;
const MPF_COMPONENTS_SIZE_ID: u8 = 5;
const MPF_BITSTREAM_ID: u8 = 8;
const MPF_BITS_PER_COMPONENT_SIZE: usize = 22;
const MPF_STATUS_POLL_TIMEOUT: u64 = 2 * USEC_PER_SEC;
const MPF_STATUS_BUSY: u8 = BIT(0);
const MPF_STATUS_READY: u8 = BIT(1);
const MPF_STATUS_SPI_VIOLATION: u8 = BIT(2);
const MPF_STATUS_SPI_ERROR: u8 = BIT(3);

#[repr(C)]
struct mpf_priv {
    spi: *mut spi_device,
    program_mode: bool,
    tx: u8,
    rx: u8,
}

unsafe fn mpf_read_status(priv_: *mut mpf_priv) -> i32 {
    /*
     * HW status is returned on MISO in the first byte after CS went
     * active. However, first reading can be inadequate, so we submit
     * two identical SPI transfers and use result of the later one.
     */
    let mut xfers: [spi_transfer; 2] = [
        spi_transfer {
            tx_buf: &(*priv_).tx as *const u8 as *const core::ffi::c_void,
            rx_buf: &mut (*priv_).rx as *mut u8 as *mut core::ffi::c_void,
            len: 1,
            cs_change: 1,
            ..core::mem::zeroed()
        },
        spi_transfer {
            tx_buf: &(*priv_).tx as *const u8 as *const core::ffi::c_void,
            rx_buf: &mut (*priv_).rx as *mut u8 as *mut core::ffi::c_void,
            len: 1,
            ..core::mem::zeroed()
        },
    ];
    (*priv_).tx = MPF_SPI_READ_STATUS;
    let ret = spi_sync_transfer((*priv_).spi, xfers.as_mut_ptr(), 2);
    if ret != 0 { return ret; }
    let status = (*priv_).rx;
    if (status & MPF_STATUS_SPI_VIOLATION) != 0 || (status & MPF_STATUS_SPI_ERROR) != 0 { return -EIO; }
    status as i32
}

unsafe fn mpf_ops_state(mgr: *mut fpga_manager) -> fpga_mgr_states {
    let priv_ = (*mgr).priv_ as *mut mpf_priv;
    let program_mode = (*priv_).program_mode;
    let status = mpf_read_status(priv_);
    if !program_mode && status == 0 { return FPGA_MGR_STATE_OPERATING; }
    FPGA_MGR_STATE_UNKNOWN
}

unsafe fn mpf_ops_parse_header(mgr: *mut fpga_manager, info: *mut fpga_image_info, buf: *const i8, count: usize) -> i32 {
    if buf.is_null() { dev_err(&(*mgr).dev, "Image buffer is not provided\n"); return -EINVAL; }
    let header_size = *(buf.add(MPF_HEADER_SIZE_OFFSET) as *const u8) as usize;
    if header_size == 0 { return -EINVAL; }
    if header_size > count { (*info).header_size = header_size; return -EAGAIN; }
    let mut blocks_num = *(buf.add(header_size - 1) as *const u8);
    let mut block_id_offset = header_size + MPF_LOOKUP_TABLE_BLOCK_ID_OFFSET;
    let mut block_start_offset = header_size + MPF_LOOKUP_TABLE_BLOCK_START_OFFSET;
    let extended_header_size = header_size + blocks_num as usize * MPF_LOOKUP_TABLE_RECORD_SIZE;
    if extended_header_size > count { (*info).header_size = extended_header_size; return -EAGAIN; }
    let mut components_size_start: usize = 0;
    let mut bitstream_start: usize = 0;
    while blocks_num != 0 {
        let block_id = *(buf.add(block_id_offset) as *const u8);
        let block_start = get_unaligned_le32(buf.add(block_start_offset) as *const u8) as usize;
        match block_id {
            MPF_BITSTREAM_ID => { bitstream_start = block_start; (*info).header_size = block_start; if block_start > count { return -EAGAIN; } }
            MPF_COMPONENTS_SIZE_ID => components_size_start = block_start,
            _ => {}
        }
        if bitstream_start != 0 && components_size_start != 0 { break; }
        blocks_num -= 1; block_id_offset += MPF_LOOKUP_TABLE_RECORD_SIZE; block_start_offset += MPF_LOOKUP_TABLE_RECORD_SIZE;
    }
    if bitstream_start == 0 || components_size_start == 0 { dev_err(&(*mgr).dev, "Failed to parse header look-up table\n"); return -EFAULT; }
    let components_num = get_unaligned_le16(buf.add(MPF_DATA_SIZE_OFFSET) as *const u8);
    for i in 0..components_num {
        let bit_num = i as usize * MPF_BITS_PER_COMPONENT_SIZE;
        let byte_num = bit_num / BITS_PER_BYTE;
        let byte_off = bit_num % BITS_PER_BYTE;
        let mut component_size = get_unaligned_le32(buf.add(components_size_start + byte_num) as *const u8);
        component_size >>= byte_off;
        component_size &= GENMASK(MPF_BITS_PER_COMPONENT_SIZE - 1, 0);
        (*info).data_size += component_size as usize * MPF_SPI_FRAME_SIZE;
    }
    0
}

unsafe fn mpf_poll_status(priv_: *mut mpf_priv, mask: u8) -> i32 {
    let mut status: i32 = 0;
    let ret = read_poll_timeout(mpf_read_status, status, status < 0 || ((status & (MPF_STATUS_BUSY | mask) as i32) == mask as i32), 0, MPF_STATUS_POLL_TIMEOUT, false, priv_);
    if ret < 0 { return ret; }
    status
}

unsafe fn mpf_spi_write(priv_: *mut mpf_priv, buf: *const core::ffi::c_void, buf_size: usize) -> i32 {
    let status = mpf_poll_status(priv_, 0); if status < 0 { return status; }
    spi_write_then_read((*priv_).spi, buf, buf_size, core::ptr::null_mut(), 0)
}

unsafe fn mpf_spi_write_then_read(priv_: *mut mpf_priv, txbuf: *const core::ffi::c_void, txbuf_size: usize, rxbuf: *mut core::ffi::c_void, rxbuf_size: usize) -> i32 {
    let read_command = [MPF_SPI_READ_DATA];
    let ret = mpf_spi_write(priv_, txbuf, txbuf_size); if ret != 0 { return ret; }
    let ret = mpf_poll_status(priv_, MPF_STATUS_READY); if ret < 0 { return ret; }
    spi_write_then_read((*priv_).spi, read_command.as_ptr() as *const core::ffi::c_void, read_command.len(), rxbuf, rxbuf_size)
}

unsafe fn mpf_ops_write_init(mgr: *mut fpga_manager, info: *mut fpga_image_info, _buf: *const i8, _count: usize) -> i32 {
    let program_mode = [MPF_SPI_FRAME_INIT, MPF_SPI_PRG_MODE]; let isc_en_command = [MPF_SPI_ISC_ENABLE];
    let priv_ = (*mgr).priv_ as *mut mpf_priv; let dev = &(*mgr).dev; let mut isc_ret: u32 = 0;
    if ((*info).flags & FPGA_MGR_PARTIAL_RECONFIG) != 0 { dev_err(dev, "Partial reconfiguration is not supported\n"); return -EOPNOTSUPP; }
    let ret = mpf_spi_write_then_read(priv_, isc_en_command.as_ptr() as *const _, 1, &mut isc_ret as *mut _ as *mut _, 4);
    if ret != 0 || isc_ret != 0 { dev_err(dev, "Failed to enable ISC: spi_ret %d, isc_ret %u\n", ret, isc_ret); return -EFAULT; }
    let ret = mpf_spi_write(priv_, program_mode.as_ptr() as *const _, 2); if ret != 0 { dev_err(dev, "Failed to enter program mode: %d\n", ret); return ret; }
    (*priv_).program_mode = true; 0
}

unsafe fn mpf_spi_frame_write(priv_: *mut mpf_priv, buf: *const i8) -> i32 {
    let mut xfers: [spi_transfer; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    xfers[0].tx_buf = &(*priv_).tx as *const _ as *const _; xfers[0].len = 1;
    xfers[1].tx_buf = buf as *const _; xfers[1].len = MPF_SPI_FRAME_SIZE;
    let ret = mpf_poll_status(priv_, 0); if ret < 0 { return ret; }
    (*priv_).tx = MPF_SPI_FRAME; spi_sync_transfer((*priv_).spi, xfers.as_mut_ptr(), 2)
}

unsafe fn mpf_ops_write(mgr: *mut fpga_manager, buf: *const i8, count: usize) -> i32 {
    let priv_ = (*mgr).priv_ as *mut mpf_priv; let dev = &(*mgr).dev;
    if count % MPF_SPI_FRAME_SIZE != 0 { dev_err(dev, "Bitstream size is not a multiple of %d\n", MPF_SPI_FRAME_SIZE); return -EINVAL; }
    for i in 0..count / MPF_SPI_FRAME_SIZE { let ret = mpf_spi_frame_write(priv_, buf.add(i * MPF_SPI_FRAME_SIZE)); if ret != 0 { dev_err(dev, "Failed to write bitstream frame %d/%zu\n", i, count / MPF_SPI_FRAME_SIZE); return ret; } }
    0
}

unsafe fn mpf_ops_write_complete(mgr: *mut fpga_manager, _info: *mut fpga_image_info) -> i32 {
    let isc_dis_command = [MPF_SPI_ISC_DISABLE]; let release_command = [MPF_SPI_RELEASE]; let priv_ = (*mgr).priv_ as *mut mpf_priv; let dev = &(*mgr).dev;
    let ret = mpf_spi_write(priv_, isc_dis_command.as_ptr() as *const _, 1); if ret != 0 { dev_err(dev, "Failed to disable ISC: %d\n", ret); return ret; }
    usleep_range(1000, 2000);
    let ret = mpf_spi_write(priv_, release_command.as_ptr() as *const _, 1); if ret != 0 { dev_err(dev, "Failed to exit program mode: %d\n", ret); return ret; }
    (*priv_).program_mode = false; 0
}

static mpf_ops: fpga_manager_ops = fpga_manager_ops { state: Some(mpf_ops_state), initial_header_size: 71, skip_header: true, parse_header: Some(mpf_ops_parse_header), write_init: Some(mpf_ops_write_init), write: Some(mpf_ops_write), write_complete: Some(mpf_ops_write_complete) };

unsafe fn mpf_probe(spi: *mut spi_device) -> i32 {
    let dev = &(*spi).dev;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<mpf_priv>(), GFP_KERNEL) as *mut mpf_priv;
    if priv_.is_null() { return -ENOMEM; }
    (*priv_).spi = spi;
    let mgr = devm_fpga_mgr_register(dev, "Microchip Polarfire SPI FPGA Manager", &mpf_ops, priv_);
    PTR_ERR_OR_ZERO(mgr)
}

static mpf_spi_ids: [spi_device_id; 2] = [spi_device_id { name: "mpf-spi-fpga-mgr" }, spi_device_id { name: "" }];
MODULE_DEVICE_TABLE!(spi, mpf_spi_ids);

#[cfg(CONFIG_OF)]
static mpf_of_ids: [of_device_id; 2] = [of_device_id { compatible: "microchip,mpf-spi-fpga-mgr" }, of_device_id { compatible: "" }];

static mut mpf_driver: spi_driver = spi_driver { probe: Some(mpf_probe), id_table: mpf_spi_ids.as_ptr(), driver: device_driver { name: "microchip_mpf_spi_fpga_mgr", of_match_table: core::ptr::null() } };

module_spi_driver!(mpf_driver);
MODULE_DESCRIPTION!("Microchip Polarfire SPI FPGA Manager");
MODULE_AUTHOR!("Ivan Bornyakov <i.bornyakov@metrotek.ru>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
