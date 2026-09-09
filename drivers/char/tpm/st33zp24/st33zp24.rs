// SPDX-License-Identifier: GPL-2.0-or-later
/* STMicroelectronics TPM Linux driver for TPM ST33ZP24 */

// Kernel headers and types referenced by this translation are supplied by the surrounding tree.

const TPM_ACCESS: u32 = 0x0;
const TPM_STS: u32 = 0x18;
const TPM_DATA_FIFO: u32 = 0x24;
const TPM_INTF_CAPABILITY: u32 = 0x14;
const TPM_INT_STATUS: u32 = 0x10;
const TPM_INT_ENABLE: u32 = 0x08;
const LOCALITY0: i32 = 0;

#[repr(u8)] enum st33zp24_access { TPM_ACCESS_VALID = 0x80, TPM_ACCESS_ACTIVE_LOCALITY = 0x20, TPM_ACCESS_REQUEST_PENDING = 0x04, TPM_ACCESS_REQUEST_USE = 0x02 }
#[repr(u8)] enum st33zp24_status { TPM_STS_VALID = 0x80, TPM_STS_COMMAND_READY = 0x40, TPM_STS_GO = 0x20, TPM_STS_DATA_AVAIL = 0x10, TPM_STS_DATA_EXPECT = 0x08 }
#[repr(u32)] enum st33zp24_int_flags { TPM_GLOBAL_INT_ENABLE = 0x80, TPM_INTF_CMD_READY_INT = 0x080, TPM_INTF_FIFO_AVALAIBLE_INT = 0x040, TPM_INTF_WAKE_UP_READY_INT = 0x020, TPM_INTF_LOCALITY_CHANGE_INT = 0x004, TPM_INTF_STS_VALID_INT = 0x002, TPM_INTF_DATA_AVAIL_INT = 0x001 }
#[repr(usize)] enum tis_defaults { TIS_SHORT_TIMEOUT = 750, TIS_LONG_TIMEOUT = 2000 }

unsafe fn clear_interruption(tpm_dev: *mut st33zp24_dev) -> u8 { let mut interrupt = 0; (*(*tpm_dev).ops).recv((*tpm_dev).phy_id, TPM_INT_STATUS, &mut interrupt, 1); (*(*tpm_dev).ops).send((*tpm_dev).phy_id, TPM_INT_STATUS, &mut interrupt, 1); interrupt }

unsafe fn st33zp24_cancel(chip: *mut tpm_chip) { let tpm_dev = dev_get_drvdata(&mut (*chip).dev) as *mut st33zp24_dev; let mut data = TPM_STS_COMMAND_READY as u8; (*(*tpm_dev).ops).send((*tpm_dev).phy_id, TPM_STS, &mut data, 1); }
unsafe fn st33zp24_status(chip: *mut tpm_chip) -> u8 { let tpm_dev = dev_get_drvdata(&mut (*chip).dev) as *mut st33zp24_dev; let mut data = 0; if (*(*tpm_dev).ops).recv((*tpm_dev).phy_id, TPM_STS, &mut data, 1) != 1 { 0 } else { data } }
unsafe fn check_locality(chip: *mut tpm_chip) -> bool { let tpm_dev = dev_get_drvdata(&mut (*chip).dev) as *mut st33zp24_dev; let mut data = 0; let status = (*(*tpm_dev).ops).recv((*tpm_dev).phy_id, TPM_ACCESS, &mut data, 1); status == 1 && (data & ((TPM_ACCESS_ACTIVE_LOCALITY | TPM_ACCESS_VALID) as u8)) == ((TPM_ACCESS_ACTIVE_LOCALITY | TPM_ACCESS_VALID) as u8) }

unsafe fn request_locality(chip: *mut tpm_chip) -> i32 { let tpm_dev = dev_get_drvdata(&mut (*chip).dev) as *mut st33zp24_dev; if check_locality(chip) { return (*tpm_dev).locality; } let mut data = TPM_ACCESS_REQUEST_USE as u8; let ret = (*(*tpm_dev).ops).send((*tpm_dev).phy_id, TPM_ACCESS, &mut data, 1); if ret < 0 { return ret as i32; } let stop = jiffies() + (*chip).timeout_a; loop { if check_locality(chip) { return (*tpm_dev).locality; } msleep(TPM_TIMEOUT); if !time_before(jiffies(), stop) { break; } } -EACCES }
unsafe fn release_locality(chip: *mut tpm_chip) { let tpm_dev = dev_get_drvdata(&mut (*chip).dev) as *mut st33zp24_dev; let mut data = TPM_ACCESS_ACTIVE_LOCALITY as u8; (*(*tpm_dev).ops).send((*tpm_dev).phy_id, TPM_ACCESS, &mut data, 1); }

unsafe fn get_burstcount(chip: *mut tpm_chip) -> i32 { let tpm_dev = dev_get_drvdata(&mut (*chip).dev) as *mut st33zp24_dev; let stop = jiffies() + (*chip).timeout_d; loop { let mut temp = 0; let mut status = (*(*tpm_dev).ops).recv((*tpm_dev).phy_id, TPM_STS + 1, &mut temp, 1); if status < 0 { return -EBUSY; } let mut burstcnt = temp as i32; status = (*(*tpm_dev).ops).recv((*tpm_dev).phy_id, TPM_STS + 2, &mut temp, 1); if status < 0 { return -EBUSY; } burstcnt |= (temp as i32) << 8; if burstcnt != 0 { return burstcnt; } msleep(TPM_TIMEOUT); if !time_before(jiffies(), stop) { break; } } -EBUSY }

unsafe fn wait_for_tpm_stat_cond(chip: *mut tpm_chip, mask: u8, check_cancel: bool, canceled: *mut bool) -> bool { let status = (*(*chip).ops).status(chip); *canceled = false; if status & mask == mask { return true; } if check_cancel && ((*(*chip).ops).req_canceled)(chip, status) { *canceled = true; return true; } false }

unsafe fn wait_for_stat(chip: *mut tpm_chip, mask: u8, timeout: usize, queue: *mut wait_queue_head_t, check_cancel: bool) -> i32 { let tpm_dev = dev_get_drvdata(&mut (*chip).dev) as *mut st33zp24_dev; let mut status = st33zp24_status(chip); if status & mask == mask { return 0; } let stop = jiffies() + timeout; if (*chip).flags & TPM_CHIP_FLAG_IRQ != 0 { let cur_intrs = (*tpm_dev).intrs; clear_interruption(tpm_dev); enable_irq((*tpm_dev).irq); let mut ret = 0; loop { if ret == -ERESTARTSYS && freezing(current()) { clear_thread_flag(TIF_SIGPENDING); } let timeout = stop.wrapping_sub(jiffies()); if timeout as isize <= 0 { return -1; } ret = wait_event_interruptible_timeout(queue, cur_intrs != (*tpm_dev).intrs, timeout); clear_interruption(tpm_dev); let mut canceled = false; let condition = wait_for_tpm_stat_cond(chip, mask, check_cancel, &mut canceled); if ret >= 0 && condition { return if canceled { -ECANCELED } else { 0 }; } if !(ret == -ERESTARTSYS && freezing(current())) { break; } } disable_irq_nosync((*tpm_dev).irq); } else { loop { msleep(TPM_TIMEOUT); status = (*(*chip).ops).status(chip); if status & mask == mask { return 0; } if !time_before(jiffies(), stop) { break; } } } -ETIME }

unsafe fn recv_data(chip: *mut tpm_chip, buf: *mut u8, count: usize) -> i32 { let tpm_dev = dev_get_drvdata(&mut (*chip).dev) as *mut st33zp24_dev; let mut size = 0usize; while size < count && wait_for_stat(chip, (TPM_STS_DATA_AVAIL | TPM_STS_VALID) as u8, (*chip).timeout_c, &mut (*tpm_dev).read_queue, true) == 0 { let burstcnt = get_burstcount(chip); if burstcnt < 0 { return burstcnt; } let len = core::cmp::min(burstcnt as usize, count - size); let ret = (*(*tpm_dev).ops).recv((*tpm_dev).phy_id, TPM_DATA_FIFO, buf.add(size), len); if ret < 0 { return ret as i32; } size += len; } size as i32 }

unsafe extern "C" fn tpm_ioserirq_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t { let chip = dev_id as *mut tpm_chip; let tpm_dev = dev_get_drvdata(&mut (*chip).dev) as *mut st33zp24_dev; (*tpm_dev).intrs += 1; wake_up_interruptible(&mut (*tpm_dev).read_queue); disable_irq_nosync((*tpm_dev).irq); IRQ_HANDLED }

unsafe fn st33zp24_send(chip: *mut tpm_chip, buf: *mut u8, _bufsiz: usize, len: usize) -> i32 { let tpm_dev = dev_get_drvdata(&mut (*chip).dev) as *mut st33zp24_dev; if len < TPM_HEADER_SIZE { return -EBUSY; } let mut ret = request_locality(chip); if ret < 0 { return ret; } let mut status = st33zp24_status(chip); if status & TPM_STS_COMMAND_READY as u8 == 0 { st33zp24_cancel(chip); if wait_for_stat(chip, TPM_STS_COMMAND_READY as u8, (*chip).timeout_b, &mut (*tpm_dev).read_queue, false) < 0 { ret = -ETIME; goto_out_err(tpm_dev, chip, ret); } } let mut i = 0usize; while i < len - 1 { let burstcnt = get_burstcount(chip); if burstcnt < 0 { ret = burstcnt; goto_out_err(tpm_dev, chip, ret); } let size = core::cmp::min(len - i - 1, burstcnt as usize); ret = (*(*tpm_dev).ops).send((*tpm_dev).phy_id, TPM_DATA_FIFO, buf.add(i), size) as i32; if ret < 0 { goto_out_err(tpm_dev, chip, ret); } i += size; } status = st33zp24_status(chip); if status & TPM_STS_DATA_EXPECT as u8 == 0 { ret = -EIO; goto_out_err(tpm_dev, chip, ret); } ret = (*(*tpm_dev).ops).send((*tpm_dev).phy_id, TPM_DATA_FIFO, buf.add(len - 1), 1) as i32; if ret < 0 { goto_out_err(tpm_dev, chip, ret); } status = st33zp24_status(chip); if status & TPM_STS_DATA_EXPECT as u8 != 0 { ret = -EIO; goto_out_err(tpm_dev, chip, ret); } let mut data = TPM_STS_GO as u8; ret = (*(*tpm_dev).ops).send((*tpm_dev).phy_id, TPM_STS, &mut data, 1) as i32; if ret < 0 { goto_out_err(tpm_dev, chip, ret); } if (*chip).flags & TPM_CHIP_FLAG_IRQ != 0 { let ordinal = u32::from_be_bytes(*(buf.add(6) as *const [u8; 4])); ret = wait_for_stat(chip, (TPM_STS_DATA_AVAIL | TPM_STS_VALID) as u8, tpm_calc_ordinal_duration(chip, ordinal), &mut (*tpm_dev).read_queue, false); if ret < 0 { goto_out_err(tpm_dev, chip, ret); } } 0 }

#[inline(never)] unsafe fn goto_out_err(_tpm_dev: *mut st33zp24_dev, chip: *mut tpm_chip, ret: i32) -> ! { st33zp24_cancel(chip); release_locality(chip); panic!("translated C goto: {}", ret) }

unsafe fn st33zp24_recv(chip: *mut tpm_chip, buf: *mut u8, count: usize) -> i32 { if chip.is_null() { return -EBUSY; } if count < TPM_HEADER_SIZE { st33zp24_cancel(chip); release_locality(chip); return -EIO; } let mut size = recv_data(chip, buf, TPM_HEADER_SIZE); if size < TPM_HEADER_SIZE as i32 { dev_err(&mut (*chip).dev, "Unable to read header\n"); st33zp24_cancel(chip); release_locality(chip); return size; } let expected = u32::from_be_bytes(*(buf.add(2) as *const [u8; 4])) as usize; if expected > count || expected < TPM_HEADER_SIZE { size = -EIO; } else { size += recv_data(chip, buf.add(TPM_HEADER_SIZE), expected - TPM_HEADER_SIZE); if size < expected as i32 { dev_err(&mut (*chip).dev, "Unable to read remainder of result\n"); size = -ETIME; } } st33zp24_cancel(chip); release_locality(chip); size }

unsafe fn st33zp24_req_canceled(_chip: *mut tpm_chip, status: u8) -> bool { status == TPM_STS_COMMAND_READY as u8 }

#[no_mangle] pub static st33zp24_tpm: tpm_class_ops = tpm_class_ops { flags: TPM_OPS_AUTO_STARTUP, send: st33zp24_send, recv: st33zp24_recv, cancel: st33zp24_cancel, status: st33zp24_status, req_complete_mask: (TPM_STS_DATA_AVAIL | TPM_STS_VALID) as u8, req_complete_val: (TPM_STS_DATA_AVAIL | TPM_STS_VALID) as u8, req_canceled: st33zp24_req_canceled };

#[repr(C)] pub struct acpi_gpio_params { pub gpio: u32, pub index: u32, pub active_low: bool }
static lpcpd_gpios: acpi_gpio_params = acpi_gpio_params { gpio: 1, index: 0, active_low: false };

pub unsafe fn st33zp24_probe(phy_id: *mut core::ffi::c_void, ops: *const st33zp24_phy_ops, dev: *mut device, irq: i32) -> i32 { let chip = tpmm_chip_alloc(dev, &st33zp24_tpm); if is_err(chip) { return ptr_err(chip); } let tpm_dev = devm_kzalloc(dev, core::mem::size_of::<st33zp24_dev>(), GFP_KERNEL) as *mut st33zp24_dev; if tpm_dev.is_null() { return -ENOMEM; } (*tpm_dev).phy_id = phy_id; (*tpm_dev).ops = ops; dev_set_drvdata(&mut (*chip).dev, tpm_dev as *mut _); (*chip).timeout_a = msecs_to_jiffies(TIS_SHORT_TIMEOUT as u32); (*chip).timeout_b = msecs_to_jiffies(TIS_LONG_TIMEOUT as u32); (*chip).timeout_c = (*chip).timeout_a; (*chip).timeout_d = (*chip).timeout_a; (*tpm_dev).locality = LOCALITY0; (*tpm_dev).irq = irq; if irq != 0 { init_waitqueue_head(&mut (*tpm_dev).read_queue); (*tpm_dev).intrs = 0; if request_locality(chip) != LOCALITY0 { return -ENODEV; } clear_interruption(tpm_dev); let ret = devm_request_irq(dev, irq, tpm_ioserirq_handler, IRQF_TRIGGER_HIGH, "TPM SERIRQ management", chip as *mut _); if ret < 0 { return ret; } let mut intmask = (TPM_INTF_CMD_READY_INT | TPM_INTF_STS_VALID_INT | TPM_INTF_DATA_AVAIL_INT) as u8; if (*(*tpm_dev).ops).send((*tpm_dev).phy_id, TPM_INT_ENABLE, &mut intmask, 1) < 0 { return -EIO; } intmask = TPM_GLOBAL_INT_ENABLE as u8; if (*(*tpm_dev).ops).send((*tpm_dev).phy_id, TPM_INT_ENABLE + 3, &mut intmask, 1) < 0 { return -EIO; } (*chip).flags |= TPM_CHIP_FLAG_IRQ; disable_irq_nosync(irq); } tpm_chip_register(chip) }

pub unsafe fn st33zp24_remove(chip: *mut tpm_chip) { tpm_chip_unregister(chip); }

#[cfg(CONFIG_PM_SLEEP)]
pub unsafe fn st33zp24_pm_suspend(dev: *mut device) -> i32 { let chip = dev_get_drvdata(dev) as *mut tpm_chip; let tpm_dev = dev_get_drvdata(&mut (*chip).dev) as *mut st33zp24_dev; if !(*tpm_dev).io_lpcpd.is_null() { gpiod_set_value_cansleep((*tpm_dev).io_lpcpd, 0); 0 } else { tpm_pm_suspend(dev) } }

#[cfg(CONFIG_PM_SLEEP)]
pub unsafe fn st33zp24_pm_resume(dev: *mut device) -> i32 { let chip = dev_get_drvdata(dev) as *mut tpm_chip; let tpm_dev = dev_get_drvdata(&mut (*chip).dev) as *mut st33zp24_dev; if !(*tpm_dev).io_lpcpd.is_null() { gpiod_set_value_cansleep((*tpm_dev).io_lpcpd, 1); wait_for_stat(chip, TPM_STS_VALID as u8, (*chip).timeout_b, &mut (*tpm_dev).read_queue, false) } else { let ret = tpm_pm_resume(dev); if ret == 0 { tpm1_do_selftest(chip); } ret } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
