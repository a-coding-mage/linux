// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2016 Google, Inc
 *
 * This device driver implements a TCG PTP FIFO interface over SPI for chips
 * with Cr50 firmware.
 * It is based on tpm_tis_spi driver by Peter Huewe and Christophe Ricard.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const CR50_SLEEP_DELAY_MSEC: u64 = 1000;
const CR50_WAKE_START_DELAY_USEC: u64 = 1000;
const CR50_NOIRQ_ACCESS_DELAY: u64 = msecs_to_jiffies(2);
const CR50_READY_IRQ_TIMEOUT: u64 = msecs_to_jiffies(TPM2_TIMEOUT_A);
const CR50_FLOW_CONTROL: u64 = msecs_to_jiffies(TPM2_TIMEOUT_A);
const MAX_IRQ_CONFIRMATION_ATTEMPTS: u32 = 3;

#[inline]
const fn tpm_cr50_fw_ver(l: u32) -> u32 { 0x0f90 | (l << 12) }
const TPM_CR50_MAX_FW_VER_LEN: usize = 64;
const TPM_CR50_DEFAULT_RNG_QUALITY: u32 = 700;

#[repr(C)]
struct Cr50SpiPhy {
    spi_phy: TpmTisSpiPhy,
    time_track_mutex: Mutex,
    last_access: u64,
    access_delay: u64,
    irq_confirmation_attempt: u32,
    irq_needs_confirmation: bool,
    irq_confirmed: bool,
}

#[inline]
unsafe fn to_cr50_spi_phy(phy: *mut TpmTisSpiPhy) -> *mut Cr50SpiPhy {
    (phy as *mut u8).sub(offset_of!(Cr50SpiPhy, spi_phy)) as *mut Cr50SpiPhy
}

unsafe extern "C" fn cr50_spi_irq_handler(_dummy: i32, dev_id: *mut core::ffi::c_void) -> i32 {
    let cr50_phy = dev_id as *mut Cr50SpiPhy;
    (*cr50_phy).irq_confirmed = true;
    complete(&mut (*cr50_phy).spi_phy.ready);
    IRQ_HANDLED
}

unsafe fn cr50_ensure_access_delay(phy: *mut Cr50SpiPhy) {
    let allowed_access = (*phy).last_access.wrapping_add((*phy).access_delay);
    let time_now = jiffies();
    let dev = &(*(*phy).spi_phy.spi_device).dev;

    if time_in_range_open(time_now, (*phy).last_access, allowed_access) {
        let timeout = allowed_access.wrapping_sub(time_now);
        let remaining = wait_for_completion_timeout(&mut (*phy).spi_phy.ready, timeout);
        if remaining == 0 && (*phy).irq_confirmed {
            dev_warn(dev, "Timeout waiting for TPM ready IRQ\n");
        }
    }

    if (*phy).irq_needs_confirmation {
        let attempt = (*phy).irq_confirmation_attempt.wrapping_add(1);
        (*phy).irq_confirmation_attempt = attempt;
        if (*phy).irq_confirmed {
            (*phy).irq_needs_confirmation = false;
            (*phy).access_delay = CR50_READY_IRQ_TIMEOUT;
            dev_info(dev, "TPM ready IRQ confirmed on attempt %u\n", attempt);
        } else if attempt > MAX_IRQ_CONFIRMATION_ATTEMPTS {
            (*phy).irq_needs_confirmation = false;
            dev_warn(dev, "IRQ not confirmed - will use delays\n");
        }
    }
}

unsafe fn cr50_needs_waking(phy: *mut Cr50SpiPhy) -> bool {
    !time_in_range_open(jiffies(), (*phy).last_access, (*phy).spi_phy.wake_after)
}

unsafe fn cr50_wake_if_needed(cr50_phy: *mut Cr50SpiPhy) {
    let phy = &mut (*cr50_phy).spi_phy;
    if cr50_needs_waking(cr50_phy) {
        let mut spi_cs_wake = SpiTransfer {
            delay: SpiDelay { value: 1000, unit: SPI_DELAY_UNIT_USECS },
            ..core::mem::zeroed()
        };
        spi_sync_transfer(phy.spi_device, &mut spi_cs_wake, 1);
        usleep_range(CR50_WAKE_START_DELAY_USEC, CR50_WAKE_START_DELAY_USEC * 2);
    }
    phy.wake_after = jiffies().wrapping_add(msecs_to_jiffies(CR50_SLEEP_DELAY_MSEC));
}

unsafe extern "C" fn cr50_spi_flow_control(phy: *mut TpmTisSpiPhy, spi_xfer: *mut SpiTransfer) -> i32 {
    let dev = &(*(*phy).spi_device).dev;
    let timeout = jiffies().wrapping_add(CR50_FLOW_CONTROL);
    let mut m: SpiMessage = core::mem::zeroed();
    (*spi_xfer).len = 1;
    loop {
        spi_message_init(&mut m);
        spi_message_add_tail(spi_xfer, &mut m);
        let ret = spi_sync_locked((*phy).spi_device, &mut m);
        if ret < 0 { return ret; }
        if time_after(jiffies(), timeout) {
            dev_warn(dev, "Timeout during flow control\n");
            return -EBUSY;
        }
        if (*phy).iobuf[0] & 1 != 0 { return 0; }
    }
}

unsafe fn tpm_cr50_spi_is_firmware_power_managed(dev: *mut Device) -> bool {
    let mut val: u8 = 0;
    if device_property_read_u8(dev, b"firmware-power-managed\0".as_ptr(), &mut val) != 0 { return true; }
    val != 0
}

unsafe extern "C" fn tpm_tis_spi_cr50_transfer(data: *mut TpmTisData, addr: u32, len: u16, input: *mut u8, output: *const u8) -> i32 {
    let phy = to_tpm_tis_spi_phy(data);
    let cr50_phy = to_cr50_spi_phy(phy);
    mutex_lock(&mut (*cr50_phy).time_track_mutex);
    cr50_ensure_access_delay(cr50_phy);
    cr50_wake_if_needed(cr50_phy);
    let ret = tpm_tis_spi_transfer(data, addr, len, input, output);
    (*cr50_phy).last_access = jiffies();
    mutex_unlock(&mut (*cr50_phy).time_track_mutex);
    ret
}

unsafe extern "C" fn tpm_tis_spi_cr50_read_bytes(data: *mut TpmTisData, addr: u32, len: u16, result: *mut u8, _io_mode: TpmTisIoMode) -> i32 {
    tpm_tis_spi_cr50_transfer(data, addr, len, result, core::ptr::null())
}

unsafe extern "C" fn tpm_tis_spi_cr50_write_bytes(data: *mut TpmTisData, addr: u32, len: u16, value: *const u8, _io_mode: TpmTisIoMode) -> i32 {
    tpm_tis_spi_cr50_transfer(data, addr, len, core::ptr::null_mut(), value)
}

#[repr(C)]
static TpmSpiCr50PhyOps: TpmTisPhyOps = TpmTisPhyOps {
    read_bytes: Some(tpm_tis_spi_cr50_read_bytes), write_bytes: Some(tpm_tis_spi_cr50_write_bytes),
};

unsafe fn cr50_print_fw_version(data: *mut TpmTisData) {
    let phy = to_tpm_tis_spi_phy(data);
    let mut len = 0usize;
    let mut fw_ver = [0u8; TPM_CR50_MAX_FW_VER_LEN + 1];
    let mut block = [0u8; 4];
    tpm_tis_write8(data, tpm_cr50_fw_ver((*data).locality), 0);
    loop {
        tpm_tis_read_bytes(data, tpm_cr50_fw_ver((*data).locality), 4, block.as_mut_ptr());
        let mut i = 0;
        while i < 4 && block[i] != 0 { fw_ver[len] = block[i]; len += 1; i += 1; }
        if i != 4 || len >= TPM_CR50_MAX_FW_VER_LEN { break; }
    }
    fw_ver[len] = 0;
    dev_info(&(*(*phy).spi_device).dev, "Cr50 firmware version: %s\n", fw_ver.as_ptr());
}

// The probe and resume entry points retain their kernel-facing signatures; dependent kernel types and helpers are supplied externally.
pub unsafe extern "C" fn cr50_spi_probe(spi: *mut SpiDevice) -> i32 {
    let cr50_phy = devm_kzalloc(&mut (*spi).dev, core::mem::size_of::<Cr50SpiPhy>(), GFP_KERNEL) as *mut Cr50SpiPhy;
    if cr50_phy.is_null() { return -ENOMEM; }
    let phy = &mut (*cr50_phy).spi_phy;
    phy.flow_control = Some(cr50_spi_flow_control);
    phy.wake_after = jiffies();
    phy.priv_.rng_quality = TPM_CR50_DEFAULT_RNG_QUALITY;
    init_completion(&mut phy.ready);
    (*cr50_phy).access_delay = CR50_NOIRQ_ACCESS_DELAY;
    (*cr50_phy).last_access = jiffies();
    mutex_init(&mut (*cr50_phy).time_track_mutex);
    if (*spi).irq > 0 {
        let ret = devm_request_irq(&mut (*spi).dev, (*spi).irq, Some(cr50_spi_irq_handler), IRQF_TRIGGER_RISING, b"cr50_spi\0".as_ptr(), cr50_phy as *mut _);
        if ret < 0 && ret == -EPROBE_DEFER { return ret; }
        if ret >= 0 { (*cr50_phy).irq_needs_confirmation = true; }
    }
    let ret = tpm_tis_spi_init(spi, phy, -1, &TpmSpiCr50PhyOps);
    if ret != 0 { return ret; }
    cr50_print_fw_version(&mut phy.priv_.data);
    let chip = dev_get_drvdata(&mut (*spi).dev) as *mut TpmChip;
    if tpm_cr50_spi_is_firmware_power_managed(&mut (*spi).dev) { (*chip).flags |= TPM_CHIP_FLAG_FIRMWARE_POWER_MANAGED; }
    0
}

#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn tpm_tis_spi_resume(dev: *mut Device) -> i32 {
    let chip = dev_get_drvdata(dev) as *mut TpmChip;
    let data = dev_get_drvdata(&mut (*chip).dev) as *mut TpmTisData;
    let phy = to_tpm_tis_spi_phy(data);
    // Jiffies do not increase during suspend, so reset the wake deadline after resume.
    (*phy).wake_after = jiffies();
    tpm_tis_resume(dev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
