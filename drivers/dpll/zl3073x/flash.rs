// SPDX-License-Identifier: GPL-2.0-or-later

// C dependencies are supplied by the surrounding driver translation.

const ZL_FLASH_ERR_PFX: &str = "FW update failed: ";

unsafe fn zl3073x_flash_download(
    zldev: *mut zl3073x_dev, component: *const c_char, mut addr: u32,
    data: *const c_void, size: usize, extack: *mut netlink_ext_ack,
) -> i32 {
    const ZL_CHECK_DELAY: u64 = 5000;
    let mut check_time: c_ulong;
    let mut rc: i32 = 0;
    let mut offset = 0usize;

    dev_dbg((*zldev).dev, "Downloading %zu bytes to device memory at 0x%0x\n", size, addr);
    check_time = jiffies + msecs_to_jiffies(ZL_CHECK_DELAY);

    while offset < size {
        rc = zl3073x_write_hwreg(zldev, addr, get_unaligned((data as *const u8).add(offset) as *const u32));
        if rc != 0 {
            NL_SET_ERR_MSG_FMT_MOD!(extack, "{}failed to write to memory at 0x{:0x}", ZL_FLASH_ERR_PFX, addr);
            return rc;
        }
        if time_is_before_jiffies(check_time) {
            if signal_pending(current) {
                NL_SET_ERR_MSG_FMT_MOD!(extack, "{}Flashing interrupted", ZL_FLASH_ERR_PFX);
                return -EINTR;
            }
            check_time = jiffies + msecs_to_jiffies(ZL_CHECK_DELAY);
        }
        if offset % 1024 == 0 {
            zl3073x_devlink_flash_notify(zldev, "Downloading image", component, offset, size);
        }
        offset += 4;
        addr = addr.wrapping_add(4);
    }
    zl3073x_devlink_flash_notify(zldev, "Downloading image", component, offset, size);
    dev_dbg((*zldev).dev, "%zu bytes downloaded to device memory\n", size);
    rc
}

unsafe fn zl3073x_flash_error_check(zldev: *mut zl3073x_dev, extack: *mut netlink_ext_ack) -> i32 {
    let (mut count, mut cause) = (0u32, 0u32);
    let mut rc = zl3073x_read_u32(zldev, ZL_REG_ERROR_COUNT, &mut count);
    if rc != 0 { return rc; }
    if count == 0 { return 0; }
    rc = zl3073x_read_u32(zldev, ZL_REG_ERROR_CAUSE, &mut cause);
    if rc != 0 { return rc; }
    NL_SET_ERR_MSG_FMT_MOD!(extack, "{}utility error occurred: count={} cause=0x{:x}", ZL_FLASH_ERR_PFX, count, cause);
    -EIO
}

unsafe fn zl3073x_flash_wait_ready(zldev: *mut zl3073x_dev, timeout_ms: c_uint) -> i32 {
    const ZL_FLASH_POLL_DELAY_MS: c_uint = 100;
    let timeout = jiffies + msecs_to_jiffies(timeout_ms);
    let mut i = 0;
    dev_dbg((*zldev).dev, "Waiting for flashing to be ready\n");
    while time_is_after_jiffies(timeout) {
        if i > 9 {
            if signal_pending(current) { return -EINTR; }
            i = 0;
        }
        let mut value = 0u8;
        let rc = zl3073x_read_u8(zldev, ZL_REG_WRITE_FLASH, &mut value);
        if rc != 0 { return rc; }
        value = FIELD_GET(ZL_WRITE_FLASH_OP, value);
        if value == ZL_WRITE_FLASH_OP_DONE { return 0; }
        msleep(ZL_FLASH_POLL_DELAY_MS);
        i += 1;
    }
    -ETIMEDOUT
}

unsafe fn zl3073x_flash_cmd_wait(zldev: *mut zl3073x_dev, operation: u32, extack: *mut netlink_ext_ack) -> i32 {
    let mut value = 0u8;
    dev_dbg((*zldev).dev, "Sending flash command: 0x%x\n", operation);
    let mut rc = zl3073x_flash_wait_ready(zldev, 60000);
    if rc != 0 { return rc; }
    rc = zl3073x_read_u8(zldev, ZL_REG_WRITE_FLASH, &mut value);
    if rc != 0 { return rc; }
    FIELD_MODIFY!(ZL_WRITE_FLASH_OP, &mut value, operation);
    rc = zl3073x_write_u8(zldev, ZL_REG_WRITE_FLASH, value);
    if rc != 0 { return rc; }
    rc = zl3073x_flash_wait_ready(zldev, 120000);
    if rc != 0 { return rc; }
    zl3073x_flash_error_check(zldev, extack)
}

unsafe fn zl3073x_flash_get_sector_size(zldev: *mut zl3073x_dev, sector_size: *mut usize) -> i32 {
    let mut flash_info = 0u8;
    let rc = zl3073x_read_u8(zldev, ZL_REG_FLASH_INFO, &mut flash_info);
    if rc != 0 { return rc; }
    match FIELD_GET(ZL_FLASH_INFO_SECTOR_SIZE, flash_info) {
        ZL_FLASH_INFO_SECTOR_4K => *sector_size = SZ_4K,
        ZL_FLASH_INFO_SECTOR_64K => *sector_size = SZ_64K,
        _ => return -EINVAL,
    }
    0
}

unsafe fn zl3073x_flash_block(zldev: *mut zl3073x_dev, component: *const c_char, operation: u32, page: u32, addr: u32, data: *const c_void, size: usize, extack: *mut netlink_ext_ack) -> i32 {
    let mut rc = zl3073x_flash_download(zldev, component, addr, data, size, extack);
    if rc != 0 { return rc; }
    rc = zl3073x_write_u32(zldev, ZL_REG_IMAGE_START_ADDR, addr); if rc != 0 { return rc; }
    rc = zl3073x_write_u32(zldev, ZL_REG_IMAGE_SIZE, size as u32); if rc != 0 { return rc; }
    rc = zl3073x_write_u32(zldev, ZL_REG_FLASH_INDEX_WRITE, page); if rc != 0 { return rc; }
    rc = zl3073x_write_u32(zldev, ZL_REG_FILL_PATTERN, U32_MAX); if rc != 0 { return rc; }
    zl3073x_devlink_flash_notify(zldev, "Flashing image", component, 0, size);
    dev_dbg((*zldev).dev, "Flashing %zu bytes to page %u\n", size, page);
    rc = zl3073x_flash_cmd_wait(zldev, operation, extack);
    if rc != 0 { return rc; }
    zl3073x_devlink_flash_notify(zldev, "Flashing image", component, size, size);
    0
}

pub unsafe fn zl3073x_flash_sectors(zldev: *mut zl3073x_dev, component: *const c_char, mut page: u32, addr: u32, data: *const c_void, size: usize, extack: *mut netlink_ext_ack) -> i32 {
    const MAX_BLOCK: usize = 0x0001E000;
    const PAGE_SIZE: usize = 256;
    let mut sector_size = 0usize;
    let mut rc = zl3073x_flash_get_sector_size(zldev, &mut sector_size);
    if rc != 0 { NL_SET_ERR_MSG_FMT_MOD!(extack, "{}Failed to get flash sector size", ZL_FLASH_ERR_PFX); return rc; }
    let max_block_size = (MAX_BLOCK / sector_size) * sector_size;
    let mut offset = 0usize;
    while offset < size {
        let block_size = core::cmp::min(max_block_size, size - offset);
        let mut comp_str = [0i8; 32];
        let comp = if max_block_size < size { snprintf!(comp_str.as_mut_ptr(), comp_str.len(), "{}-part{}", component, offset / max_block_size + 1); comp_str.as_ptr() } else { component };
        rc = zl3073x_flash_block(zldev, comp, ZL_WRITE_FLASH_OP_SECTORS, page, addr, (data as *const u8).add(offset) as *const c_void, block_size, extack);
        if rc != 0 { break; }
        page += (block_size / PAGE_SIZE) as u32;
        offset += block_size;
    }
    zl3073x_devlink_flash_notify(zldev, if rc != 0 { "Flashing failed" } else { "Flashing done" }, component, 0, 0);
    rc
}

pub unsafe fn zl3073x_flash_page(zldev: *mut zl3073x_dev, component: *const c_char, page: u32, addr: u32, data: *const c_void, size: usize, extack: *mut netlink_ext_ack) -> i32 {
    let rc = zl3073x_flash_block(zldev, component, ZL_WRITE_FLASH_OP_PAGE, page, addr, data, size, extack);
    zl3073x_devlink_flash_notify(zldev, if rc != 0 { "Flashing failed" } else { "Flashing done" }, component, 0, 0);
    rc
}

pub unsafe fn zl3073x_flash_page_copy(zldev: *mut zl3073x_dev, component: *const c_char, src_page: u32, dst_page: u32, extack: *mut netlink_ext_ack) -> i32 {
    let mut rc = zl3073x_write_u32(zldev, ZL_REG_FLASH_INDEX_READ, src_page); if rc != 0 { return rc; }
    rc = zl3073x_write_u32(zldev, ZL_REG_FLASH_INDEX_WRITE, dst_page); if rc != 0 { return rc; }
    rc = zl3073x_flash_cmd_wait(zldev, ZL_WRITE_FLASH_OP_COPY_PAGE, extack);
    if rc != 0 { NL_SET_ERR_MSG_FMT_MOD!(extack, "{}Failed to copy page {} to page {}", ZL_FLASH_ERR_PFX, src_page, dst_page); }
    rc
}

unsafe fn zl3073x_flash_mode_verify(zldev: *mut zl3073x_dev) -> i32 {
    let (mut hash, mut family, mut release) = (0u32, 0u8, 0u8);
    let mut rc = zl3073x_read_u32(zldev, ZL_REG_FLASH_HASH, &mut hash); if rc != 0 { return rc; }
    rc = zl3073x_read_u8(zldev, ZL_REG_FLASH_FAMILY, &mut family); if rc != 0 { return rc; }
    rc = zl3073x_read_u8(zldev, ZL_REG_FLASH_RELEASE, &mut release); if rc != 0 { return rc; }
    dev_dbg((*zldev).dev, "Flash utility check: hash 0x%08x, fam 0x%02x, rel 0x%02x\n", hash, family, release);
    if family == 0x21 { 0 } else { -ENODEV }
}

unsafe fn zl3073x_flash_host_ctrl_enable(zldev: *mut zl3073x_dev) -> i32 {
    let mut host_ctrl = 0u8;
    let rc = zl3073x_read_u8(zldev, ZL_REG_HOST_CONTROL, &mut host_ctrl);
    if rc != 0 { return rc; }
    host_ctrl |= ZL_HOST_CONTROL_ENABLE;
    zl3073x_write_u8(zldev, ZL_REG_HOST_CONTROL, host_ctrl)
}

pub unsafe fn zl3073x_flash_mode_enter(zldev: *mut zl3073x_dev, util_ptr: *const c_void, util_size: usize, extack: *mut netlink_ext_ack) -> i32 {
    zl3073x_devlink_flash_notify(zldev, "Prepare flash mode", c"utility".as_ptr(), 0, 0);
    let mut rc = zl3073x_write_hwreg_seq(zldev, PRE_SEQ.as_ptr(), PRE_SEQ.len());
    if rc != 0 { NL_SET_ERR_MSG_FMT_MOD!(extack, "{}cannot execute pre-load sequence", ZL_FLASH_ERR_PFX); zl3073x_flash_mode_leave(zldev, extack); return rc; }
    rc = zl3073x_flash_download(zldev, c"utility".as_ptr(), 0x20000000, util_ptr, util_size, extack);
    if rc != 0 { zl3073x_flash_mode_leave(zldev, extack); return rc; }
    rc = zl3073x_write_hwreg_seq(zldev, POST_SEQ.as_ptr(), POST_SEQ.len());
    if rc != 0 { zl3073x_flash_mode_leave(zldev, extack); return rc; }
    let rc = zl3073x_flash_mode_verify(zldev); if rc != 0 { zl3073x_flash_mode_leave(zldev, extack); return rc; }
    let rc = zl3073x_flash_host_ctrl_enable(zldev); if rc != 0 { zl3073x_flash_mode_leave(zldev, extack); return rc; }
    zl3073x_devlink_flash_notify(zldev, "Flash mode enabled", c"utility".as_ptr(), 0, 0); 0
}

pub unsafe fn zl3073x_flash_mode_leave(zldev: *mut zl3073x_dev, _extack: *mut netlink_ext_ack) -> i32 {
    let mut reset_status = 0u8;
    let mut rc = zl3073x_read_u8(zldev, ZL_REG_RESET_STATUS, &mut reset_status); if rc != 0 { return rc; }
    reset_status |= ZL_REG_RESET_STATUS_RESET;
    rc = zl3073x_write_u8(zldev, ZL_REG_RESET_STATUS, reset_status); if rc != 0 { return rc; }
    zl3073x_write_hwreg_seq(zldev, FW_RESET_SEQ.as_ptr(), FW_RESET_SEQ.len());
    msleep(500);
    rc = zl3073x_read_u8(zldev, ZL_REG_RESET_STATUS, &mut reset_status); if rc != 0 { return rc; }
    if reset_status & ZL_REG_RESET_STATUS_RESET != 0 { dev_err((*zldev).dev, "Reset not confirmed after switch to normal mode\n"); return -EINVAL; }
    0
}

// The register-sequence entries are supplied by the surrounding translation.
extern {
    static PRE_SEQ: [zl3073x_hwreg_seq_item; 6];
    static POST_SEQ: [zl3073x_hwreg_seq_item; 7];
    static FW_RESET_SEQ: [zl3073x_hwreg_seq_item; 2];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
