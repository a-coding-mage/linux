// SPDX-License-Identifier: GPL-2.0
/* Turris Mox rWTM firmware driver */

// External Linux kernel and firmware declarations are supplied by dependencies.

pub const DRIVER_NAME: &str = "turris-mox-rwtm";
pub const RWTM_DMA_BUFFER_SIZE: usize = 4096;

pub const MOX_ECC_NUM_BITS: usize = 521;
pub const MOX_ECC_NUM_LEN: usize = (MOX_ECC_NUM_BITS + 7) / 8;
pub const MOX_ECC_NUM_WORDS: usize = (MOX_ECC_NUM_BITS + 31) / 32;
pub const MOX_ECC_SIG_LEN: usize = 2 * MOX_ECC_NUM_LEN;
pub const MOX_ECC_PUBKEY_LEN: usize = 1 + MOX_ECC_NUM_LEN;

pub const MBOX_STS_SUCCESS: u32 = 0 << 30;
pub const MBOX_STS_FAIL: u32 = 1 << 30;
pub const MBOX_STS_BADCMD: u32 = 2 << 30;

#[inline]
pub const fn mbox_sts_error(s: u32) -> u32 { s & (3 << 30) }
#[inline]
pub const fn mbox_sts_value(s: u32) -> u32 { (s >> 10) & 0xfffff }
#[inline]
pub const fn mbox_sts_cmd(s: u32) -> u32 { s & 0x3ff }

#[repr(u32)]
pub enum MboxCmd {
    GetRandom = 1,
    BoardInfo = 2,
    EcdsaPubKey = 3,
    Hash = 4,
    Sign = 5,
    Verify = 6,
    OtpRead = 7,
    OtpWrite = 8,
}

#[repr(C)]
pub struct MoxRwtm {
    pub mbox_client: mbox_client,
    pub mbox: *mut mbox_chan,
    pub hwrng: hwrng,
    pub reply: armada_37xx_rwtm_rx_msg,
    pub buf: *mut u8,
    pub buf_phys: dma_addr_t,
    pub busy: mutex,
    pub cmd_done: completion,
    pub has_board_info: bool,
    pub serial_number: u64,
    pub board_version: i32,
    pub ram_size: i32,
    pub mac_address1: [u8; 6],
    pub mac_address2: [u8; 6],
    // Present when CONFIG_TURRIS_MOX_RWTM_KEYCTL is enabled.
    #[cfg(feature = "CONFIG_TURRIS_MOX_RWTM_KEYCTL")]
    pub pubkey: [u8; MOX_ECC_PUBKEY_LEN],
}

#[inline]
unsafe fn rwtm_dev(rwtm: *mut MoxRwtm) -> *mut device {
    (*rwtm).mbox_client.dev
}

unsafe fn mox_get_status(cmd: MboxCmd, retval: u32) -> i32 {
    if mbox_sts_cmd(retval) != cmd as u32 { -EIO }
    else if mbox_sts_error(retval) == MBOX_STS_FAIL { -(mbox_sts_value(retval) as i32) }
    else if mbox_sts_error(retval) == MBOX_STS_BADCMD { -EOPNOTSUPP }
    else if mbox_sts_error(retval) != MBOX_STS_SUCCESS { -EIO }
    else { mbox_sts_value(retval) as i32 }
}

unsafe extern "C" fn mox_rwtm_rx_callback(cl: *mut mbox_client, data: *mut core::ffi::c_void) {
    let rwtm = dev_get_drvdata((*cl).dev) as *mut MoxRwtm;
    if completion_done(&mut (*rwtm).cmd_done) { return; }
    (*rwtm).reply = *(data as *const armada_37xx_rwtm_rx_msg);
    complete(&mut (*rwtm).cmd_done);
}

unsafe fn mox_rwtm_exec(rwtm: *mut MoxRwtm, cmd: MboxCmd,
                        msg: *mut armada_37xx_rwtm_tx_msg,
                        interruptible: bool) -> i32 {
    let mut empty: armada_37xx_rwtm_tx_msg = core::mem::zeroed();
    let msg = if msg.is_null() { &mut empty } else { &mut *msg };
    (*msg).command = cmd as u32;
    let mut ret = mbox_send_message((*rwtm).mbox, msg as *mut _);
    if ret < 0 { return ret; }
    if interruptible {
        ret = wait_for_completion_interruptible(&mut (*rwtm).cmd_done);
        if ret < 0 { return ret; }
    } else if wait_for_completion_timeout(&mut (*rwtm).cmd_done, HZ / 2) == 0 { return -ETIMEDOUT; }
    mox_get_status(cmd, (*rwtm).reply.retval)
}

unsafe fn reply_to_mac_addr(mac: *mut u8, t1: u32, t2: u32) {
    *mac.add(0) = (t1 >> 8) as u8; *mac.add(1) = t1 as u8;
    *mac.add(2) = (t2 >> 24) as u8; *mac.add(3) = (t2 >> 16) as u8;
    *mac.add(4) = (t2 >> 8) as u8; *mac.add(5) = t2 as u8;
}

unsafe fn mox_get_board_info(rwtm: *mut MoxRwtm) -> i32 {
    let ret = mox_rwtm_exec(rwtm, MboxCmd::BoardInfo, core::ptr::null_mut(), false);
    if ret >= 0 {
        let s = (*rwtm).reply.status;
        (*rwtm).serial_number = ((s[1] as u64) << 32) | s[0] as u64;
        (*rwtm).board_version = s[2] as i32; (*rwtm).ram_size = s[3] as i32;
        reply_to_mac_addr((*rwtm).mac_address1.as_mut_ptr(), s[4], s[5]);
        reply_to_mac_addr((*rwtm).mac_address2.as_mut_ptr(), s[6], s[7]);
        (*rwtm).has_board_info = true;
    }
    0
}

unsafe fn check_get_random_support(rwtm: *mut MoxRwtm) -> i32 {
    let mut msg: armada_37xx_rwtm_tx_msg = core::mem::zeroed();
    msg.args = [1, (*rwtm).buf_phys, 4];
    mox_rwtm_exec(rwtm, MboxCmd::GetRandom, &mut msg, false)
}

unsafe extern "C" fn mox_hwrng_read(rng: *mut hwrng, data: *mut core::ffi::c_void,
                                     max: usize, wait: bool) -> i32 {
    let rwtm = container_of!(rng, MoxRwtm, hwrng);
    let max = core::cmp::min(max, RWTM_DMA_BUFFER_SIZE);
    let mut msg: armada_37xx_rwtm_tx_msg = core::mem::zeroed();
    msg.args = [1, (*rwtm).buf_phys, ((max + 3) & !3) as _];
    let ret = if wait { mutex_lock(&mut (*rwtm).busy); 0 } else if mutex_trylock(&mut (*rwtm).busy) == 0 { -EBUSY } else { 0 };
    if ret < 0 { return ret; }
    let ret = mox_rwtm_exec(rwtm, MboxCmd::GetRandom, &mut msg, true);
    if ret < 0 { mutex_unlock(&mut (*rwtm).busy); return ret; }
    core::ptr::copy_nonoverlapping((*rwtm).buf, data as *mut u8, max);
    mutex_unlock(&mut (*rwtm).busy); max as i32
}

#[cfg(feature = "CONFIG_TURRIS_MOX_RWTM_KEYCTL")]
unsafe fn mox_ecc_number_to_bin(dst: *mut u8, src: *const u32) {
    let mut tmp = [0u32; MOX_ECC_NUM_WORDS];
    cpu_to_be32_array(tmp.as_mut_ptr(), src, MOX_ECC_NUM_WORDS);
    core::ptr::copy_nonoverlapping((tmp.as_ptr() as *const u8).add(2), dst, MOX_ECC_NUM_LEN);
}

#[cfg(feature = "CONFIG_TURRIS_MOX_RWTM_KEYCTL")]
unsafe fn mox_ecc_public_key_to_bin(dst: *mut u8, src_first: u32, src_rest: *const u32) {
    *dst = (src_first >> 16) as u8; *dst.add(1) = (src_first >> 8) as u8; *dst.add(2) = src_first as u8;
    let mut tmp = [0u32; MOX_ECC_NUM_WORDS - 1];
    cpu_to_be32_array(tmp.as_mut_ptr(), src_rest, MOX_ECC_NUM_WORDS - 1);
    core::ptr::copy_nonoverlapping(tmp.as_ptr() as *const u8, dst.add(3), core::mem::size_of_val(&tmp));
}

#[cfg(not(feature = "CONFIG_TURRIS_MOX_RWTM_KEYCTL"))]
unsafe fn mox_register_signing_key(_: *mut MoxRwtm) -> i32 { 0 }

unsafe fn rwtm_devm_mbox_release(mbox: *mut core::ffi::c_void) { mbox_free_channel(mbox as *mut mbox_chan); }
unsafe fn rwtm_firmware_symlink_drop(parent: *mut core::ffi::c_void) { sysfs_remove_link(parent, DRIVER_NAME); }

unsafe extern "C" fn turris_mox_rwtm_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let rwtm = devm_kzalloc(dev, core::mem::size_of::<MoxRwtm>(), GFP_KERNEL) as *mut MoxRwtm;
    if rwtm.is_null() { return -ENOMEM; }
    platform_set_drvdata(pdev, rwtm as *mut _);
    let mut ret = devm_mutex_init(dev, &mut (*rwtm).busy);
    if ret != 0 { return ret; }
    init_completion(&mut (*rwtm).cmd_done);
    (*rwtm).mbox_client.dev = dev;
    (*rwtm).mbox_client.rx_callback = Some(mox_rwtm_rx_callback);
    (*rwtm).mbox = mbox_request_channel(&mut (*rwtm).mbox_client, 0);
    if is_err((*rwtm).mbox) { return ptr_err((*rwtm).mbox); }
    ret = mox_get_board_info(rwtm);
    if ret < 0 { dev_warn(dev, "Cannot read board information: %i\n", ret); }
    ret = mox_register_signing_key(rwtm);
    if ret < 0 { return ret; }
    ret = check_get_random_support(rwtm);
    if ret < 0 { return ret; }
    ret = devm_hwrng_register(dev, &mut (*rwtm).hwrng);
    if ret != 0 { return ret; }
    0
}

#[repr(C)]
pub struct of_device_id { pub compatible: *const u8 }
#[repr(C)] pub struct platform_driver { pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32> }
pub static TURRIS_MOX_RWTM_MATCH: &[of_device_id] = &[
    of_device_id { compatible: b"cznic,turris-mox-rwtm\0".as_ptr() },
    of_device_id { compatible: b"marvell,armada-3700-rwtm-firmware\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];
pub static TURRIS_MOX_RWTM_DRIVER: platform_driver = platform_driver { probe: Some(turris_mox_rwtm_probe) };

// MODULE_DEVICE_TABLE(of, turris_mox_rwtm_match);
// module_platform_driver(turris_mox_rwtm_driver);
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("Turris Mox rWTM firmware driver");
// MODULE_AUTHOR("Marek Behun <kabel@kernel.org>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
