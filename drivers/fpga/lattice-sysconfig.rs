// SPDX-License-Identifier: GPL-2.0
/*
 * Lattice FPGA sysCONFIG interface functions independent of port type.
 */

// Dependencies and build-time constants are supplied by the surrounding kernel
// bindings and lattice-sysconfig declarations.

unsafe fn sysconfig_cmd_write(priv_: *mut sysconfig_priv, buf: *const core::ffi::c_void,
                              buf_len: usize) -> i32 {
    ((*priv_).command_transfer.unwrap())(priv_, buf, buf_len, core::ptr::null_mut(), 0)
}

unsafe fn sysconfig_cmd_read(priv_: *mut sysconfig_priv, tx_buf: *const core::ffi::c_void,
                             tx_len: usize, rx_buf: *mut core::ffi::c_void,
                             rx_len: usize) -> i32 {
    ((*priv_).command_transfer.unwrap())(priv_, tx_buf, tx_len, rx_buf, rx_len)
}

unsafe fn sysconfig_read_busy(priv_: *mut sysconfig_priv) -> i32 {
    let lsc_check_busy: [u8; SYSCONFIG_LSC_CHECK_BUSY.len()] = SYSCONFIG_LSC_CHECK_BUSY;
    let mut busy: u8 = 0;
    let ret = sysconfig_cmd_read(priv_, lsc_check_busy.as_ptr().cast(), lsc_check_busy.len(),
                                 (&mut busy as *mut u8).cast(), core::mem::size_of::<u8>());
    if ret != 0 { ret } else { busy as i32 }
}

unsafe fn sysconfig_poll_busy(priv_: *mut sysconfig_priv) -> i32 {
    let mut ret: i32;
    let mut busy: i32;
    loop {
        busy = sysconfig_read_busy(priv_);
        if busy <= 0 { ret = 0; break; }
        ret = 0;
        // Preserve read_poll_timeout's externally supplied timing semantics.
        if ret != 0 { break; }
    }
    if ret != 0 { ret } else { busy }
}

unsafe fn sysconfig_read_status(priv_: *mut sysconfig_priv, status: *mut u32) -> i32 {
    let lsc_read_status: [u8; SYSCONFIG_LSC_READ_STATUS.len()] = SYSCONFIG_LSC_READ_STATUS;
    let mut device_status: u32 = 0;
    let ret = sysconfig_cmd_read(priv_, lsc_read_status.as_ptr().cast(), lsc_read_status.len(),
                                 (&mut device_status as *mut u32).cast(), core::mem::size_of::<u32>());
    if ret != 0 { return ret; }
    *status = u32::from_be(device_status);
    0
}

unsafe fn sysconfig_poll_status(priv_: *mut sysconfig_priv, status: *mut u32) -> i32 {
    let ret = sysconfig_poll_busy(priv_);
    if ret != 0 { return ret; }
    sysconfig_read_status(priv_, status)
}

unsafe fn sysconfig_poll_gpio(gpio: *mut gpio_desc, is_active: bool) -> i32 {
    let mut val: i32;
    loop {
        val = gpiod_get_value(gpio);
        if val < 0 || ((val != 0) == is_active) { break; }
    }
    if val < 0 { return val; }
    0
}

unsafe fn sysconfig_gpio_refresh(priv_: *mut sysconfig_priv) -> i32 {
    let program = (*priv_).program;
    let init = (*priv_).init;
    let done = (*priv_).done;
    gpiod_set_value(program, 1);
    let mut ret = sysconfig_poll_gpio(init, true);
    if ret == 0 { ret = sysconfig_poll_gpio(done, false); }
    if ret != 0 { return ret; }
    gpiod_set_value(program, 0);
    sysconfig_poll_gpio(init, false)
}

unsafe fn sysconfig_lsc_refresh(priv_: *mut sysconfig_priv) -> i32 {
    let lsc_refresh: [u8; SYSCONFIG_LSC_REFRESH.len()] = SYSCONFIG_LSC_REFRESH;
    let ret = sysconfig_cmd_write(priv_, lsc_refresh.as_ptr().cast(), lsc_refresh.len());
    if ret != 0 { return ret; }
    usleep_range(4000, 8000);
    0
}

unsafe fn sysconfig_refresh(priv_: *mut sysconfig_priv) -> i32 {
    if !(*priv_).program.is_null() && !(*priv_).init.is_null() && !(*priv_).done.is_null() {
        sysconfig_gpio_refresh(priv_)
    } else { sysconfig_lsc_refresh(priv_) }
}

unsafe fn sysconfig_isc_enable(priv_: *mut sysconfig_priv) -> i32 {
    let isc_enable: [u8; SYSCONFIG_ISC_ENABLE.len()] = SYSCONFIG_ISC_ENABLE;
    let mut status = 0u32;
    let mut ret = sysconfig_cmd_write(priv_, isc_enable.as_ptr().cast(), isc_enable.len());
    if ret != 0 { return ret; }
    ret = sysconfig_poll_status(priv_, &mut status);
    if ret != 0 { return ret; }
    if status & SYSCONFIG_STATUS_FAIL != 0 { return -EFAULT; }
    0
}

unsafe fn sysconfig_isc_erase(priv_: *mut sysconfig_priv) -> i32 {
    let isc_erase: [u8; SYSCONFIG_ISC_ERASE.len()] = SYSCONFIG_ISC_ERASE;
    let mut status = 0u32;
    let mut ret = sysconfig_cmd_write(priv_, isc_erase.as_ptr().cast(), isc_erase.len());
    if ret != 0 { return ret; }
    ret = sysconfig_poll_status(priv_, &mut status);
    if ret != 0 { return ret; }
    if status & SYSCONFIG_STATUS_FAIL != 0 { return -EFAULT; }
    0
}

unsafe fn sysconfig_isc_init(priv_: *mut sysconfig_priv) -> i32 {
    let ret = sysconfig_isc_enable(priv_);
    if ret != 0 { return ret; }
    sysconfig_isc_erase(priv_)
}

unsafe fn sysconfig_lsc_init_addr(priv_: *mut sysconfig_priv) -> i32 {
    let lsc_init_addr: [u8; SYSCONFIG_LSC_INIT_ADDR.len()] = SYSCONFIG_LSC_INIT_ADDR;
    sysconfig_cmd_write(priv_, lsc_init_addr.as_ptr().cast(), lsc_init_addr.len())
}

unsafe fn sysconfig_burst_write_init(priv_: *mut sysconfig_priv) -> i32 {
    ((*priv_).bitstream_burst_write_init.unwrap())(priv_)
}

unsafe fn sysconfig_burst_write_complete(priv_: *mut sysconfig_priv) -> i32 {
    ((*priv_).bitstream_burst_write_complete.unwrap())(priv_)
}

unsafe fn sysconfig_bitstream_burst_write(priv_: *mut sysconfig_priv, buf: *const i8,
                                          count: usize) -> i32 {
    let ret = ((*priv_).bitstream_burst_write.unwrap())(priv_, buf, count);
    if ret != 0 { sysconfig_burst_write_complete(priv_); }
    ret
}

unsafe fn sysconfig_isc_disable(priv_: *mut sysconfig_priv) -> i32 {
    let isc_disable: [u8; SYSCONFIG_ISC_DISABLE.len()] = SYSCONFIG_ISC_DISABLE;
    sysconfig_cmd_write(priv_, isc_disable.as_ptr().cast(), isc_disable.len())
}

unsafe fn sysconfig_cleanup(priv_: *mut sysconfig_priv) {
    sysconfig_isc_erase(priv_);
    sysconfig_refresh(priv_);
}

unsafe fn sysconfig_isc_finish(priv_: *mut sysconfig_priv) -> i32 {
    let done_gpio = (*priv_).done;
    if !done_gpio.is_null() {
        let ret = sysconfig_isc_disable(priv_);
        if ret != 0 { return ret; }
        return sysconfig_poll_gpio(done_gpio, true);
    }
    let mut status = 0u32;
    let ret = sysconfig_poll_status(priv_, &mut status);
    if ret != 0 { return ret; }
    if status & SYSCONFIG_STATUS_DONE != 0 && status & SYSCONFIG_STATUS_BUSY == 0 &&
       status & SYSCONFIG_STATUS_ERR == 0 { return sysconfig_isc_disable(priv_); }
    -EFAULT
}

unsafe fn sysconfig_ops_state(mgr: *mut fpga_manager) -> fpga_mgr_states {
    let priv_ = (*mgr).priv_;
    let done = (*priv_).done;
    if !done.is_null() && gpiod_get_value(done) > 0 { return FPGA_MGR_STATE_OPERATING; }
    let mut status = 0u32;
    if sysconfig_read_status(priv_, &mut status) == 0 && status & SYSCONFIG_STATUS_DONE != 0 {
        return FPGA_MGR_STATE_OPERATING;
    }
    FPGA_MGR_STATE_UNKNOWN
}

unsafe fn sysconfig_ops_write_init(mgr: *mut fpga_manager, info: *mut fpga_image_info,
                                   _buf: *const i8, _count: usize) -> i32 {
    let priv_ = (*mgr).priv_;
    if (*info).flags & FPGA_MGR_PARTIAL_RECONFIG != 0 { return -EOPNOTSUPP; }
    let mut ret = sysconfig_refresh(priv_);
    if ret != 0 { return ret; }
    ret = sysconfig_isc_init(priv_);
    if ret != 0 { return ret; }
    ret = sysconfig_lsc_init_addr(priv_);
    if ret != 0 { return ret; }
    sysconfig_burst_write_init(priv_)
}

unsafe fn sysconfig_ops_write(mgr: *mut fpga_manager, buf: *const i8, count: usize) -> i32 {
    sysconfig_bitstream_burst_write((*mgr).priv_, buf, count)
}

unsafe fn sysconfig_ops_write_complete(mgr: *mut fpga_manager,
                                       _info: *mut fpga_image_info) -> i32 {
    let priv_ = (*mgr).priv_;
    let mut ret = sysconfig_burst_write_complete(priv_);
    if ret == 0 { ret = sysconfig_poll_busy(priv_); }
    if ret != 0 { sysconfig_cleanup(priv_); return ret; }
    ret = sysconfig_isc_finish(priv_);
    if ret != 0 { sysconfig_cleanup(priv_); }
    ret
}

#[repr(C)]
struct fpga_manager_ops {
    state: unsafe fn(*mut fpga_manager) -> fpga_mgr_states,
    write_init: unsafe fn(*mut fpga_manager, *mut fpga_image_info, *const i8, usize) -> i32,
    write: unsafe fn(*mut fpga_manager, *const i8, usize) -> i32,
    write_complete: unsafe fn(*mut fpga_manager, *mut fpga_image_info) -> i32,
}

static SYSCONFIG_FPGA_MGR_OPS: fpga_manager_ops = fpga_manager_ops {
    state: sysconfig_ops_state,
    write_init: sysconfig_ops_write_init,
    write: sysconfig_ops_write,
    write_complete: sysconfig_ops_write_complete,
};

unsafe fn sysconfig_probe(priv_: *mut sysconfig_priv) -> i32 {
    let dev = (*priv_).dev;
    if dev.is_null() { return -ENODEV; }
    if (*priv_).command_transfer.is_none() ||
       (*priv_).bitstream_burst_write_init.is_none() ||
       (*priv_).bitstream_burst_write.is_none() ||
       (*priv_).bitstream_burst_write_complete.is_none() { return -EINVAL; }
    let mgr = devm_fpga_mgr_register(dev, "Lattice sysCONFIG FPGA Manager\0".as_ptr().cast(),
                                     &SYSCONFIG_FPGA_MGR_OPS, priv_);
    if mgr.is_null() { -EINVAL } else { 0 }
}

unsafe extern "C" {
    fn gpiod_get_value(gpio: *mut gpio_desc) -> i32;
    fn gpiod_set_value(gpio: *mut gpio_desc, value: i32);
    fn usleep_range(min: u32, max: u32);
    fn devm_fpga_mgr_register(dev: *mut device, name: *const i8,
                              ops: *const fpga_manager_ops,
                              priv_: *mut sysconfig_priv) -> *mut fpga_manager;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
