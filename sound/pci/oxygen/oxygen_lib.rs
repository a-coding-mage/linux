// SPDX-License-Identifier: GPL-2.0-only
/*
 * C-Media CMI8788 driver - main driver module
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

// Rust translation of pci/oxygen/oxygen_lib.c.
// C include/module metadata dependencies are intentionally left to the
// surrounding kernel/driver bindings.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type u8 = ::std::os::raw::c_uchar;
type u16 = ::std::os::raw::c_ushort;
type u32 = ::std::os::raw::c_uint;
type c_int = ::std::os::raw::c_int;
type c_uint = ::std::os::raw::c_uint;
type c_char = ::std::os::raw::c_char;
type c_void = ::std::os::raw::c_void;

const DRIVER: &[u8] = b"oxygen\0";

const PEX811X: usize = 0;
const PI7C9X110: usize = 1;
const XIO2001: usize = 2;

#[repr(C)]
pub struct oxygen {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: c_uint,
    pub device: c_uint,
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub class: c_uint,
    pub class_mask: c_uint,
    pub driver_data: usize,
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub enum irqreturn_t {
    IRQ_NONE = 0,
    IRQ_HANDLED = 1,
}

unsafe extern "C" {
    static OXYGEN_MPU401: c_uint;
    static MPU401_RX_EMPTY: u8;
    static MPU401_ACK: u8;
    static OXYGEN_INTERRUPT_STATUS: c_uint;
    static OXYGEN_CHANNEL_A: c_uint;
    static OXYGEN_CHANNEL_B: c_uint;
    static OXYGEN_CHANNEL_C: c_uint;
    static OXYGEN_CHANNEL_SPDIF: c_uint;
    static OXYGEN_CHANNEL_MULTICH: c_uint;
    static OXYGEN_CHANNEL_AC97: c_uint;
    static OXYGEN_INT_SPDIF_IN_DETECT: c_uint;
    static OXYGEN_INT_GPIO: c_uint;
    static OXYGEN_INT_AC97: c_uint;
    static OXYGEN_INTERRUPT_MASK: c_uint;
    static PCM_COUNT: c_uint;
    static OXYGEN_SPDIF_CONTROL: c_uint;
    static OXYGEN_SPDIF_SENSE_INT: c_uint;
    static OXYGEN_SPDIF_LOCK_INT: c_uint;
    static OXYGEN_SPDIF_RATE_INT: c_uint;
    static OXYGEN_INT_MIDI: c_uint;
    static OXYGEN_SPDIF_SENSE_STATUS: c_uint;
    static OXYGEN_SPDIF_LOCK_STATUS: c_uint;
    static OXYGEN_SPDIF_IN_CLOCK_MASK: c_uint;
    static OXYGEN_SPDIF_IN_CLOCK_192: c_uint;
    static OXYGEN_SPDIF_IN_CLOCK_96: c_uint;
    static CONTROL_SPDIF_INPUT_BITS: usize;
    static SNDRV_CTL_EVENT_MASK_VALUE: c_uint;
    static OXYGEN_REVISION: c_uint;
    static OXYGEN_PACKAGE_ID_MASK: u8;
    static OXYGEN_PACKAGE_ID_8786: u8;
    static OXYGEN_PACKAGE_ID_8787: u8;
    static OXYGEN_PACKAGE_ID_8788: u8;
    static OXYGEN_IO_SIZE: c_uint;
    static OXYGEN_FUNCTION: c_uint;
    static OXYGEN_FUNCTION_ENABLE_SPI_4_5: u8;
    static BROKEN_EEPROM_DRIVER_DATA: usize;
    static OXYGEN_EEPROM_ID: u16;
    static OXYGEN_MISC: c_uint;
    static OXYGEN_MISC_WRITE_PCI_SUBID: u8;
    static PCI_SUBSYSTEM_VENDOR_ID: c_int;
    static PCI_SUBSYSTEM_ID: c_int;
    static OXYGEN_REVISION_2: u8;
    static OXYGEN_MISC_PCI_MEM_W_1_CLOCK: u8;
    static OXYGEN_AC97_CONTROL: c_uint;
    static OXYGEN_AC97_CODEC_0: c_uint;
    static OXYGEN_AC97_CODEC_1: c_uint;
    static OXYGEN_FUNCTION_RESET_CODEC: u8;
    static OXYGEN_FUNCTION_2WIRE_SPI_MASK: u8;
    static OXYGEN_DMA_STATUS: c_uint;
    static OXYGEN_DMA_PAUSE: c_uint;
    static OXYGEN_PLAY_CHANNELS: c_uint;
    static OXYGEN_PLAY_CHANNELS_2: u8;
    static OXYGEN_DMA_A_BURST_8: u8;
    static OXYGEN_DMA_MULTICH_BURST_8: u8;
    static OXYGEN_MISC_REC_C_FROM_SPDIF: u8;
    static OXYGEN_MISC_REC_B_FROM_AC97: u8;
    static OXYGEN_MISC_REC_A_FROM_MULTICH: u8;
    static OXYGEN_MISC_MIDI: u8;
    static OXYGEN_REC_FORMAT: c_uint;
    static OXYGEN_FORMAT_16: u8;
    static OXYGEN_REC_FORMAT_A_SHIFT: c_uint;
    static OXYGEN_REC_FORMAT_B_SHIFT: c_uint;
    static OXYGEN_REC_FORMAT_C_SHIFT: c_uint;
    static OXYGEN_PLAY_FORMAT: c_uint;
    static OXYGEN_SPDIF_FORMAT_SHIFT: c_uint;
    static OXYGEN_MULTICH_FORMAT_SHIFT: c_uint;
    static OXYGEN_REC_CHANNELS: c_uint;
    static OXYGEN_REC_CHANNELS_2_2_2: u8;
    static OXYGEN_I2S_MULTICH_FORMAT: c_uint;
    static OXYGEN_RATE_48000: c_uint;
    static OXYGEN_I2S_BITS_16: c_uint;
    static OXYGEN_I2S_MASTER: c_uint;
    static OXYGEN_I2S_BCLK_64: c_uint;
    static CAPTURE_0_FROM_I2S_1: c_uint;
    static OXYGEN_I2S_A_FORMAT: c_uint;
    static OXYGEN_I2S_MUTE_MCLK: c_uint;
    static CAPTURE_0_FROM_I2S_2: c_uint;
    static CAPTURE_2_FROM_I2S_2: c_uint;
    static OXYGEN_I2S_B_FORMAT: c_uint;
    static CAPTURE_3_FROM_I2S_3: c_uint;
    static OXYGEN_I2S_C_FORMAT: c_uint;
    static OXYGEN_SPDIF_OUT_ENABLE: c_uint;
    static OXYGEN_SPDIF_LOOPBACK: c_uint;
    static CAPTURE_1_FROM_SPDIF: c_uint;
    static OXYGEN_SPDIF_SENSE_MASK: c_uint;
    static OXYGEN_SPDIF_LOCK_MASK: c_uint;
    static OXYGEN_SPDIF_RATE_MASK: c_uint;
    static OXYGEN_SPDIF_LOCK_PAR: c_uint;
    static OXYGEN_SPDIF_SENSE_PAR: c_uint;
    static OXYGEN_SPDIF_OUTPUT_BITS: c_uint;
    static OXYGEN_2WIRE_BUS_STATUS: c_uint;
    static OXYGEN_2WIRE_LENGTH_8: c_uint;
    static OXYGEN_2WIRE_INTERRUPT_MASK: c_uint;
    static OXYGEN_2WIRE_SPEED_STANDARD: c_uint;
    static OXYGEN_MPU401_CONTROL: c_uint;
    static OXYGEN_MPU401_LOOPBACK: u8;
    static OXYGEN_GPI_INTERRUPT_MASK: c_uint;
    static OXYGEN_GPIO_INTERRUPT_MASK: c_uint;
    static OXYGEN_PLAY_ROUTING: c_uint;
    static OXYGEN_PLAY_MULTICH_I2S_DAC: c_uint;
    static OXYGEN_PLAY_SPDIF_SPDIF: c_uint;
    static OXYGEN_PLAY_DAC0_SOURCE_SHIFT: c_uint;
    static OXYGEN_PLAY_DAC1_SOURCE_SHIFT: c_uint;
    static OXYGEN_PLAY_DAC2_SOURCE_SHIFT: c_uint;
    static OXYGEN_PLAY_DAC3_SOURCE_SHIFT: c_uint;
    static OXYGEN_REC_ROUTING: c_uint;
    static OXYGEN_REC_A_ROUTE_I2S_ADC_1: u8;
    static OXYGEN_REC_B_ROUTE_I2S_ADC_2: u8;
    static OXYGEN_REC_C_ROUTE_SPDIF: u8;
    static OXYGEN_ADC_MONITOR: c_uint;
    static OXYGEN_A_MONITOR_ROUTING: c_uint;
    static OXYGEN_A_MONITOR_ROUTE_0_SHIFT: c_uint;
    static OXYGEN_A_MONITOR_ROUTE_1_SHIFT: c_uint;
    static OXYGEN_A_MONITOR_ROUTE_2_SHIFT: c_uint;
    static OXYGEN_A_MONITOR_ROUTE_3_SHIFT: c_uint;
    static OXYGEN_AC97_INTERRUPT_MASK: c_uint;
    static OXYGEN_AC97_INT_READ_DONE: u8;
    static OXYGEN_AC97_INT_WRITE_DONE: u8;
    static OXYGEN_AC97_OUT_CONFIG: c_uint;
    static OXYGEN_AC97_IN_CONFIG: c_uint;
    static OXYGEN_AC97_CLOCK_DISABLE: c_uint;
    static OXYGEN_AC97_NO_CODEC_0: c_uint;
    static AC97_RESET: c_uint;
    static CM9780_GPIO_SETUP: c_uint;
    static CM9780_GPIO0IO: c_uint;
    static CM9780_GPIO1IO: c_uint;
    static CM9780_MIXER: c_uint;
    static CM9780_BSTSEL: c_uint;
    static CM9780_STRO_MIC: c_uint;
    static CM9780_MIX2FR: c_uint;
    static CM9780_PCBSW: c_uint;
    static CM9780_JACK: c_uint;
    static CM9780_RSOE: c_uint;
    static CM9780_CBOE: c_uint;
    static CM9780_SSOE: c_uint;
    static CM9780_FROE: c_uint;
    static CM9780_MIC2MIC: c_uint;
    static CM9780_LI2LI: c_uint;
    static AC97_MASTER: c_uint;
    static AC97_PC_BEEP: c_uint;
    static AC97_MIC: c_uint;
    static AC97_LINE: c_uint;
    static AC97_CD: c_uint;
    static AC97_VIDEO: c_uint;
    static AC97_AUX: c_uint;
    static AC97_REC_GAIN: c_uint;
    static AC97_CENTER_LFE_MASTER: c_uint;
    static AC97_SURROUND_MASTER: c_uint;
    static CM9780_GPIO_STATUS: c_uint;
    static CM9780_GPO0: c_uint;
    static AC97_POWERDOWN: c_uint;
    static AC97_PD_PR0: c_uint;
    static AC97_PD_PR1: c_uint;
    static AC97_EXTENDED_STATUS: c_uint;
    static AC97_EA_PRI: c_uint;
    static AC97_EA_PRJ: c_uint;
    static AC97_EA_PRK: c_uint;
    static OXYGEN_AC97_CODEC1_SLOT3: c_uint;
    static OXYGEN_AC97_CODEC1_SLOT4: c_uint;
    static AC97_HEADPHONE: c_uint;
    static AC97_PCM: c_uint;
    static AC97_REC_SEL: c_uint;
    static IORESOURCE_IO: usize;
    static ENXIO: c_int;
    static ENODEV: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static IRQF_SHARED: c_ulong;
    static KBUILD_MODNAME: *const c_char;
    static MIDI_OUTPUT: c_uint;
    static MIDI_INPUT: c_uint;
    static MPU401_INFO_INTEGRATED: c_uint;
    static MPU401_INFO_IRQ_HOOK: c_uint;
    static MPU401_INFO_OUTPUT: c_uint;
    static MPU401_INFO_INPUT: c_uint;
    static MPU401_HW_CMIPCI: c_int;
    static SNDRV_CTL_POWER_D3hot: c_int;
    static SNDRV_CTL_POWER_D0: c_int;

    fn oxygen_read8(chip: *mut oxygen, reg: c_uint) -> u8;
    fn oxygen_read16(chip: *mut oxygen, reg: c_uint) -> c_uint;
    fn oxygen_read32(chip: *mut oxygen, reg: c_uint) -> u32;
    fn oxygen_write8(chip: *mut oxygen, reg: c_uint, value: u8);
    fn oxygen_write16(chip: *mut oxygen, reg: c_uint, value: c_uint);
    fn oxygen_write32(chip: *mut oxygen, reg: c_uint, value: u32);
    fn oxygen_write8_masked(chip: *mut oxygen, reg: c_uint, value: u8, mask: u8);
    fn oxygen_write32_masked(chip: *mut oxygen, reg: c_uint, value: c_uint, mask: c_uint);
    fn oxygen_clear_bits8(chip: *mut oxygen, reg: c_uint, bits: u8);
    fn oxygen_clear_bits32(chip: *mut oxygen, reg: c_uint, bits: c_uint);
    fn oxygen_set_bits8(chip: *mut oxygen, reg: c_uint, bits: u8);
    fn oxygen_set_bits16(chip: *mut oxygen, reg: c_uint, bits: c_uint);
    fn oxygen_set_bits32(chip: *mut oxygen, reg: c_uint, bits: c_uint);
    fn oxygen_read_eeprom(chip: *mut oxygen, index: c_uint) -> u16;
    fn oxygen_write_eeprom(chip: *mut oxygen, index: c_uint, value: c_uint);
    fn oxygen_read_ac97(chip: *mut oxygen, codec: c_uint, index: c_uint) -> c_uint;
    fn oxygen_write_ac97(chip: *mut oxygen, codec: c_uint, index: c_uint, value: c_uint);
    fn oxygen_ac97_set_bits(chip: *mut oxygen, codec: c_uint, index: c_uint, bits: c_uint);
    fn oxygen_ac97_clear_bits(chip: *mut oxygen, codec: c_uint, index: c_uint, bits: c_uint);
    fn snd_pcm_period_elapsed(stream: *mut c_void);
    fn snd_mpu401_uart_interrupt(dummy: c_int, private_data: *mut c_void);
    fn schedule_work(work: *mut work_struct);
    fn flush_work(work: *mut work_struct);
    fn wake_up(waitqueue: *mut c_void);
    fn msleep(ms: c_uint);
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *const c_void);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut c_void,
        read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer),
    );
    fn mutex_lock_interruptible(mutex: *mut c_void) -> c_int;
    fn mutex_unlock(mutex: *mut c_void);
    fn mutex_init(mutex: *mut c_void);
    fn mutex_destroy(mutex: *mut c_void);
    fn spin_lock_init(lock: *mut c_void);
    fn spin_lock_irq(lock: *mut c_void);
    fn spin_unlock_irq(lock: *mut c_void);
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn init_waitqueue_head(waitqueue: *mut c_void);
    fn pci_match_id(ids: *const pci_device_id, dev: *mut pci_dev) -> *const pci_device_id;
    fn pci_read_config_dword(pci: *mut pci_dev, pos: c_int, val: *mut u32);
    fn pci_write_config_dword(pci: *mut pci_dev, pos: c_int, val: u32);
    fn pci_write_config_word(pci: *mut pci_dev, pos: c_int, val: u16);
    fn snd_devm_card_new(
        dev: *mut device,
        index: c_int,
        id: *mut c_char,
        owner: *mut module,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const u8) -> c_int;
    fn pci_resource_flags(pci: *mut pci_dev, bar: c_int) -> usize;
    fn pci_resource_len(pci: *mut pci_dev, bar: c_int) -> usize;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> usize;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn pci_set_master(pci: *mut pci_dev);
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...);
    fn snd_component_add(card: *mut snd_card, component: *const c_char) -> c_int;
    fn oxygen_pcm_init(chip: *mut oxygen) -> c_int;
    fn oxygen_mixer_init(chip: *mut oxygen) -> c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_int,
        port: usize,
        info_flags: c_uint,
        irq: c_int,
        rmidi: *mut *mut c_void,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, ret: c_int) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn pci_get_drvdata(pci: *mut pci_dev) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

type c_ulong = ::std::os::raw::c_ulong;

macro_rules! field {
    ($ptr:expr, $($field:ident).+) => {
        compile_error!("external struct field access requires generated bindings")
    };
}

macro_rules! OXYGEN_I2S_MCLK {
    ($x:expr) => {
        OXYGEN_I2S_MCLK($x)
    };
}

unsafe extern "C" {
    fn OXYGEN_I2S_MCLK(x: c_uint) -> c_uint;
}

#[inline]
unsafe fn oxygen_uart_input_ready(chip: *mut oxygen) -> c_int {
    ((oxygen_read8(chip, OXYGEN_MPU401 + 1) & MPU401_RX_EMPTY) == 0) as c_int
}

unsafe extern "C" fn oxygen_read_uart(chip: *mut oxygen) {
    if oxygen_uart_input_ready(chip) == 0 {
        /* no data, but read it anyway to clear the interrupt */
        oxygen_read8(chip, OXYGEN_MPU401);
        return;
    }
    loop {
        let data: u8 = oxygen_read8(chip, OXYGEN_MPU401);
        if data != MPU401_ACK {
            if field!(chip, uart_input_count) >= field!(chip, uart_input).len() {
                field!(chip, uart_input_count) = 0;
            }
            field!(chip, uart_input)[field!(chip, uart_input_count)] = data;
            field!(chip, uart_input_count) += 1;
        }
        if oxygen_uart_input_ready(chip) == 0 {
            break;
        }
    }
    if field!(chip, model.uart_input).is_some() {
        field!(chip, model.uart_input).unwrap()(chip);
    }
}

unsafe extern "C" fn oxygen_interrupt(_dummy: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip: *mut oxygen = dev_id as *mut oxygen;
    let mut i: c_uint;

    let status: c_uint = oxygen_read16(chip, OXYGEN_INTERRUPT_STATUS);
    if status == 0 {
        return irqreturn_t::IRQ_NONE;
    }

    // scoped_guard(spinlock, &chip->reg_lock)
    let elapsed_streams: c_uint;
    {
        let clear: c_uint = status
            & (OXYGEN_CHANNEL_A
                | OXYGEN_CHANNEL_B
                | OXYGEN_CHANNEL_C
                | OXYGEN_CHANNEL_SPDIF
                | OXYGEN_CHANNEL_MULTICH
                | OXYGEN_CHANNEL_AC97
                | OXYGEN_INT_SPDIF_IN_DETECT
                | OXYGEN_INT_GPIO
                | OXYGEN_INT_AC97);
        if clear != 0 {
            if (clear & OXYGEN_INT_SPDIF_IN_DETECT) != 0 {
                field!(chip, interrupt_mask) &= !OXYGEN_INT_SPDIF_IN_DETECT;
            }
            oxygen_write16(chip, OXYGEN_INTERRUPT_MASK, field!(chip, interrupt_mask) & !clear);
            oxygen_write16(chip, OXYGEN_INTERRUPT_MASK, field!(chip, interrupt_mask));
        }

        elapsed_streams = status & field!(chip, pcm_running);
    }

    i = 0;
    while i < PCM_COUNT {
        if (elapsed_streams & (1 << i)) != 0 && !field!(chip, streams)[i as usize].is_null() {
            snd_pcm_period_elapsed(field!(chip, streams)[i as usize]);
        }
        i += 1;
    }

    if (status & OXYGEN_INT_SPDIF_IN_DETECT) != 0 {
        // guard(spinlock)(&chip->reg_lock)
        i = oxygen_read32(chip, OXYGEN_SPDIF_CONTROL);
        if (i & (OXYGEN_SPDIF_SENSE_INT | OXYGEN_SPDIF_LOCK_INT | OXYGEN_SPDIF_RATE_INT)) != 0 {
            /* write the interrupt bit(s) to clear */
            oxygen_write32(chip, OXYGEN_SPDIF_CONTROL, i);
            schedule_work(&mut field!(chip, spdif_input_bits_work));
        }
    }

    if (status & OXYGEN_INT_GPIO) != 0 {
        schedule_work(&mut field!(chip, gpio_work));
    }

    if (status & OXYGEN_INT_MIDI) != 0 {
        if !field!(chip, midi).is_null() {
            snd_mpu401_uart_interrupt(0, field!(field!(chip, midi), private_data));
        } else {
            oxygen_read_uart(chip);
        }
    }

    if (status & OXYGEN_INT_AC97) != 0 {
        wake_up(&mut field!(chip, ac97_waitqueue));
    }

    irqreturn_t::IRQ_HANDLED
}

unsafe extern "C" fn oxygen_spdif_input_bits_changed(work: *mut work_struct) {
    let chip: *mut oxygen = container_of_oxygen_spdif_input_bits_work(work);
    let mut reg: u32;

    /*
     * This function gets called when there is new activity on the SPDIF
     * input, or when we lose lock on the input signal, or when the rate
     * changes.
     */
    msleep(1);
    // scoped_guard(spinlock_irq, &chip->reg_lock)
    {
        reg = oxygen_read32(chip, OXYGEN_SPDIF_CONTROL);
        if (reg & (OXYGEN_SPDIF_SENSE_STATUS | OXYGEN_SPDIF_LOCK_STATUS))
            == OXYGEN_SPDIF_SENSE_STATUS
        {
            /*
             * If we detect activity on the SPDIF input but cannot lock to
             * a signal, the clock bit is likely to be wrong.
             */
            reg ^= OXYGEN_SPDIF_IN_CLOCK_MASK;
            oxygen_write32(chip, OXYGEN_SPDIF_CONTROL, reg);
            spin_unlock_irq(&mut field!(chip, reg_lock));
            msleep(1);
            spin_lock_irq(&mut field!(chip, reg_lock));
            reg = oxygen_read32(chip, OXYGEN_SPDIF_CONTROL);
            if (reg & (OXYGEN_SPDIF_SENSE_STATUS | OXYGEN_SPDIF_LOCK_STATUS))
                == OXYGEN_SPDIF_SENSE_STATUS
            {
                /* nothing detected with either clock; give up */
                if (reg & OXYGEN_SPDIF_IN_CLOCK_MASK) == OXYGEN_SPDIF_IN_CLOCK_192 {
                    /*
                     * Reset clock to <= 96 kHz because this is
                     * more likely to be received next time.
                     */
                    reg &= !OXYGEN_SPDIF_IN_CLOCK_MASK;
                    reg |= OXYGEN_SPDIF_IN_CLOCK_96;
                    oxygen_write32(chip, OXYGEN_SPDIF_CONTROL, reg);
                }
            }
        }
    }

    if !field!(chip, controls)[CONTROL_SPDIF_INPUT_BITS].is_null() {
        // scoped_guard(spinlock_irq, &chip->reg_lock)
        {
            field!(chip, interrupt_mask) |= OXYGEN_INT_SPDIF_IN_DETECT;
            oxygen_write16(chip, OXYGEN_INTERRUPT_MASK, field!(chip, interrupt_mask));
        }

        /*
         * We don't actually know that any channel status bits have
         * changed, but let's send a notification just to be sure.
         */
        snd_ctl_notify(
            field!(chip, card),
            SNDRV_CTL_EVENT_MASK_VALUE,
            &field!(field!(chip, controls)[CONTROL_SPDIF_INPUT_BITS], id) as *const _ as *const c_void,
        );
    }
}

unsafe extern "C" {
    fn container_of_oxygen_spdif_input_bits_work(work: *mut work_struct) -> *mut oxygen;
    fn container_of_oxygen_gpio_work(work: *mut work_struct) -> *mut oxygen;
}

unsafe extern "C" fn oxygen_gpio_changed(work: *mut work_struct) {
    let chip: *mut oxygen = container_of_oxygen_gpio_work(work);

    if field!(chip, model.gpio_changed).is_some() {
        field!(chip, model.gpio_changed).unwrap()(chip);
    }
}

unsafe extern "C" fn oxygen_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip: *mut oxygen = field!(entry, private_data);
    let mut i: c_int;
    let mut j: c_int;

    match oxygen_read8(chip, OXYGEN_REVISION) & OXYGEN_PACKAGE_ID_MASK {
        x if x == OXYGEN_PACKAGE_ID_8786 => i = '6' as c_int,
        x if x == OXYGEN_PACKAGE_ID_8787 => i = '7' as c_int,
        x if x == OXYGEN_PACKAGE_ID_8788 => i = '8' as c_int,
        _ => i = '?' as c_int,
    }
    snd_iprintf(buffer, b"CMI878%c:\n\0".as_ptr() as *const c_char, i);
    i = 0;
    while i < OXYGEN_IO_SIZE as c_int {
        snd_iprintf(buffer, b"%02x:\0".as_ptr() as *const c_char, i);
        j = 0;
        while j < 0x10 {
            snd_iprintf(
                buffer,
                b" %02x\0".as_ptr() as *const c_char,
                oxygen_read8(chip, (i + j) as c_uint) as c_int,
            );
            j += 1;
        }
        snd_iprintf(buffer, b"\n\0".as_ptr() as *const c_char);
        i += 0x10;
    }
    if mutex_lock_interruptible(&mut field!(chip, mutex)) < 0 {
        return;
    }
    if field!(chip, has_ac97_0) {
        snd_iprintf(buffer, b"\nAC97:\n\0".as_ptr() as *const c_char);
        i = 0;
        while i < 0x80 {
            snd_iprintf(buffer, b"%02x:\0".as_ptr() as *const c_char, i);
            j = 0;
            while j < 0x10 {
                snd_iprintf(
                    buffer,
                    b" %04x\0".as_ptr() as *const c_char,
                    oxygen_read_ac97(chip, 0, (i + j) as c_uint),
                );
                j += 2;
            }
            snd_iprintf(buffer, b"\n\0".as_ptr() as *const c_char);
            i += 0x10;
        }
    }
    if field!(chip, has_ac97_1) {
        snd_iprintf(buffer, b"\nAC97 2:\n\0".as_ptr() as *const c_char);
        i = 0;
        while i < 0x80 {
            snd_iprintf(buffer, b"%02x:\0".as_ptr() as *const c_char, i);
            j = 0;
            while j < 0x10 {
                snd_iprintf(
                    buffer,
                    b" %04x\0".as_ptr() as *const c_char,
                    oxygen_read_ac97(chip, 1, (i + j) as c_uint),
                );
                j += 2;
            }
            snd_iprintf(buffer, b"\n\0".as_ptr() as *const c_char);
            i += 0x10;
        }
    }
    mutex_unlock(&mut field!(chip, mutex));
    if field!(chip, model.dump_registers).is_some() {
        field!(chip, model.dump_registers).unwrap()(chip, buffer);
    }
}

unsafe extern "C" fn oxygen_proc_init(chip: *mut oxygen) {
    snd_card_ro_proc_new(
        field!(chip, card),
        b"oxygen\0".as_ptr() as *const c_char,
        chip as *mut c_void,
        oxygen_proc_read,
    );
}

unsafe fn oxygen_search_pci_id(
    chip: *mut oxygen,
    mut ids: *const pci_device_id,
) -> *const pci_device_id {
    let mut subdevice: u16;

    /*
     * Make sure the EEPROM pins are available, i.e., not used for SPI.
     * (This function is called before we initialize or use SPI.)
     */
    oxygen_clear_bits8(chip, OXYGEN_FUNCTION, OXYGEN_FUNCTION_ENABLE_SPI_4_5);
    /*
     * Read the subsystem device ID directly from the EEPROM, because the
     * chip didn't if the first EEPROM word was overwritten.
     */
    subdevice = oxygen_read_eeprom(chip, 2);
    /* use default ID if EEPROM is missing */
    if subdevice == 0xffff && oxygen_read_eeprom(chip, 1) == 0xffff {
        subdevice = 0x8788;
    }
    /*
     * We use only the subsystem device ID for searching because it is
     * unique even without the subsystem vendor ID, which may have been
     * overwritten in the EEPROM.
     */
    while (*ids).vendor != 0 {
        if (*ids).subdevice == subdevice as c_uint
            && (*ids).driver_data != BROKEN_EEPROM_DRIVER_DATA
        {
            return ids;
        }
        ids = ids.add(1);
    }
    ::std::ptr::null()
}

unsafe fn oxygen_restore_eeprom(chip: *mut oxygen, id: *const pci_device_id) {
    let eeprom_id: u16 = oxygen_read_eeprom(chip, 0);
    if eeprom_id != OXYGEN_EEPROM_ID && (eeprom_id != 0xffff || (*id).subdevice != 0x8788) {
        /*
         * This function gets called only when a known card model has
         * been detected, i.e., we know there is a valid subsystem
         * product ID at index 2 in the EEPROM.  Therefore, we have
         * been able to deduce the correct subsystem vendor ID, and
         * this is enough information to restore the original EEPROM
         * contents.
         */
        oxygen_write_eeprom(chip, 1, (*id).subvendor);
        oxygen_write_eeprom(chip, 0, OXYGEN_EEPROM_ID as c_uint);

        oxygen_set_bits8(chip, OXYGEN_MISC, OXYGEN_MISC_WRITE_PCI_SUBID);
        pci_write_config_word(field!(chip, pci), PCI_SUBSYSTEM_VENDOR_ID, (*id).subvendor as u16);
        pci_write_config_word(field!(chip, pci), PCI_SUBSYSTEM_ID, (*id).subdevice as u16);
        oxygen_clear_bits8(chip, OXYGEN_MISC, OXYGEN_MISC_WRITE_PCI_SUBID);

        dev_info(field!(field!(chip, card), dev), b"EEPROM ID restored\n\0".as_ptr() as *const c_char);
    }
}

unsafe fn configure_pcie_bridge(pci: *mut pci_dev) {
    static bridge_ids: [pci_device_id; 5] = [
        pci_device_id { vendor: 0x10b5, device: 0x8111, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: PEX811X },
        pci_device_id { vendor: 0x10b5, device: 0x8112, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: PEX811X },
        pci_device_id { vendor: 0x12d8, device: 0xe110, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: PI7C9X110 },
        pci_device_id { vendor: 0x104c, device: 0x8240, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: XIO2001 },
        pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
    ];
    let bridge: *mut pci_dev;
    let id: *const pci_device_id;
    let mut tmp: u32 = 0;

    if field!(pci, bus).is_null() || field!(field!(pci, bus), self).is_null() {
        return;
    }
    bridge = field!(field!(pci, bus), self);

    id = pci_match_id(bridge_ids.as_ptr(), bridge);
    if id.is_null() {
        return;
    }

    match (*id).driver_data {
        PEX811X => {
            /* PLX PEX8111/PEX8112 PCIe/PCI bridge */
            pci_read_config_dword(bridge, 0x48, &mut tmp);
            tmp |= 1; /* enable blind prefetching */
            tmp |= 1 << 11; /* enable beacon generation */
            pci_write_config_dword(bridge, 0x48, tmp);

            pci_write_config_dword(bridge, 0x84, 0x0c);
            pci_read_config_dword(bridge, 0x88, &mut tmp);
            tmp &= !(7 << 27);
            tmp |= 2 << 27; /* set prefetch size to 128 bytes */
            pci_write_config_dword(bridge, 0x88, tmp);
        }
        PI7C9X110 => {
            /* Pericom PI7C9X110 PCIe/PCI bridge */
            pci_read_config_dword(bridge, 0x40, &mut tmp);
            tmp |= 1; /* park the PCI arbiter to the sound chip */
            pci_write_config_dword(bridge, 0x40, tmp);
        }
        XIO2001 => {
            /* Texas Instruments XIO2001 PCIe/PCI bridge */
            pci_read_config_dword(bridge, 0xe8, &mut tmp);
            tmp &= !0xf; /* request length limit: 64 bytes */
            tmp &= !(0xf << 8);
            tmp |= 1 << 8; /* request count limit: one buffer */
            pci_write_config_dword(bridge, 0xe8, tmp);
        }
        _ => {}
    }
}

unsafe fn oxygen_init(chip: *mut oxygen) {
    let mut i: c_uint;

    field!(chip, dac_routing) = 1;
    i = 0;
    while i < 8 {
        field!(chip, dac_volume)[i as usize] = field!(chip, model.dac_volume_min);
        i += 1;
    }
    field!(chip, dac_mute) = 1;
    field!(chip, spdif_playback_enable) = 0;
    field!(chip, spdif_bits) =
        OXYGEN_SPDIF_C | OXYGEN_SPDIF_ORIGINAL | (IEC958_AES1_CON_PCM_CODER << OXYGEN_SPDIF_CATEGORY_SHIFT);
    field!(chip, spdif_pcm_bits) = field!(chip, spdif_bits);

    if (oxygen_read8(chip, OXYGEN_REVISION) & OXYGEN_REVISION_2) == 0 {
        oxygen_set_bits8(chip, OXYGEN_MISC, OXYGEN_MISC_PCI_MEM_W_1_CLOCK);
    }

    i = oxygen_read16(chip, OXYGEN_AC97_CONTROL);
    field!(chip, has_ac97_0) = (i & OXYGEN_AC97_CODEC_0) != 0;
    field!(chip, has_ac97_1) = (i & OXYGEN_AC97_CODEC_1) != 0;

    oxygen_write8_masked(
        chip,
        OXYGEN_FUNCTION,
        OXYGEN_FUNCTION_RESET_CODEC | field!(chip, model.function_flags),
        OXYGEN_FUNCTION_RESET_CODEC | OXYGEN_FUNCTION_2WIRE_SPI_MASK | OXYGEN_FUNCTION_ENABLE_SPI_4_5,
    );
    oxygen_write8(chip, OXYGEN_DMA_STATUS, 0);
    oxygen_write8(chip, OXYGEN_DMA_PAUSE, 0);
    oxygen_write8(chip, OXYGEN_PLAY_CHANNELS, OXYGEN_PLAY_CHANNELS_2 | OXYGEN_DMA_A_BURST_8 | OXYGEN_DMA_MULTICH_BURST_8);
    oxygen_write16(chip, OXYGEN_INTERRUPT_MASK, 0);
    oxygen_write8_masked(
        chip,
        OXYGEN_MISC,
        field!(chip, model.misc_flags),
        OXYGEN_MISC_WRITE_PCI_SUBID
            | OXYGEN_MISC_REC_C_FROM_SPDIF
            | OXYGEN_MISC_REC_B_FROM_AC97
            | OXYGEN_MISC_REC_A_FROM_MULTICH
            | OXYGEN_MISC_MIDI,
    );
    oxygen_write8(
        chip,
        OXYGEN_REC_FORMAT,
        (OXYGEN_FORMAT_16 << OXYGEN_REC_FORMAT_A_SHIFT)
            | (OXYGEN_FORMAT_16 << OXYGEN_REC_FORMAT_B_SHIFT)
            | (OXYGEN_FORMAT_16 << OXYGEN_REC_FORMAT_C_SHIFT),
    );
    oxygen_write8(
        chip,
        OXYGEN_PLAY_FORMAT,
        (OXYGEN_FORMAT_16 << OXYGEN_SPDIF_FORMAT_SHIFT) | (OXYGEN_FORMAT_16 << OXYGEN_MULTICH_FORMAT_SHIFT),
    );
    oxygen_write8(chip, OXYGEN_REC_CHANNELS, OXYGEN_REC_CHANNELS_2_2_2);
    oxygen_write16(
        chip,
        OXYGEN_I2S_MULTICH_FORMAT,
        OXYGEN_RATE_48000
            | field!(chip, model.dac_i2s_format)
            | OXYGEN_I2S_MCLK!(field!(chip, model.dac_mclks))
            | OXYGEN_I2S_BITS_16
            | OXYGEN_I2S_MASTER
            | OXYGEN_I2S_BCLK_64,
    );
    if (field!(chip, model.device_config) & CAPTURE_0_FROM_I2S_1) != 0 {
        oxygen_write16(
            chip,
            OXYGEN_I2S_A_FORMAT,
            OXYGEN_RATE_48000
                | field!(chip, model.adc_i2s_format)
                | OXYGEN_I2S_MCLK!(field!(chip, model.adc_mclks))
                | OXYGEN_I2S_BITS_16
                | OXYGEN_I2S_MASTER
                | OXYGEN_I2S_BCLK_64,
        );
    } else {
        oxygen_write16(chip, OXYGEN_I2S_A_FORMAT, OXYGEN_I2S_MASTER | OXYGEN_I2S_MUTE_MCLK);
    }
    if (field!(chip, model.device_config) & (CAPTURE_0_FROM_I2S_2 | CAPTURE_2_FROM_I2S_2)) != 0 {
        oxygen_write16(
            chip,
            OXYGEN_I2S_B_FORMAT,
            OXYGEN_RATE_48000
                | field!(chip, model.adc_i2s_format)
                | OXYGEN_I2S_MCLK!(field!(chip, model.adc_mclks))
                | OXYGEN_I2S_BITS_16
                | OXYGEN_I2S_MASTER
                | OXYGEN_I2S_BCLK_64,
        );
    } else {
        oxygen_write16(chip, OXYGEN_I2S_B_FORMAT, OXYGEN_I2S_MASTER | OXYGEN_I2S_MUTE_MCLK);
    }
    if (field!(chip, model.device_config) & CAPTURE_3_FROM_I2S_3) != 0 {
        oxygen_write16(
            chip,
            OXYGEN_I2S_C_FORMAT,
            OXYGEN_RATE_48000
                | field!(chip, model.adc_i2s_format)
                | OXYGEN_I2S_MCLK!(field!(chip, model.adc_mclks))
                | OXYGEN_I2S_BITS_16
                | OXYGEN_I2S_MASTER
                | OXYGEN_I2S_BCLK_64,
        );
    } else {
        oxygen_write16(chip, OXYGEN_I2S_C_FORMAT, OXYGEN_I2S_MASTER | OXYGEN_I2S_MUTE_MCLK);
    }
    oxygen_clear_bits32(chip, OXYGEN_SPDIF_CONTROL, OXYGEN_SPDIF_OUT_ENABLE | OXYGEN_SPDIF_LOOPBACK);
    if (field!(chip, model.device_config) & CAPTURE_1_FROM_SPDIF) != 0 {
        oxygen_write32_masked(
            chip,
            OXYGEN_SPDIF_CONTROL,
            OXYGEN_SPDIF_SENSE_MASK
                | OXYGEN_SPDIF_LOCK_MASK
                | OXYGEN_SPDIF_RATE_MASK
                | OXYGEN_SPDIF_LOCK_PAR
                | OXYGEN_SPDIF_IN_CLOCK_96,
            OXYGEN_SPDIF_SENSE_MASK
                | OXYGEN_SPDIF_LOCK_MASK
                | OXYGEN_SPDIF_RATE_MASK
                | OXYGEN_SPDIF_SENSE_PAR
                | OXYGEN_SPDIF_LOCK_PAR
                | OXYGEN_SPDIF_IN_CLOCK_MASK,
        );
    } else {
        oxygen_clear_bits32(
            chip,
            OXYGEN_SPDIF_CONTROL,
            OXYGEN_SPDIF_SENSE_MASK | OXYGEN_SPDIF_LOCK_MASK | OXYGEN_SPDIF_RATE_MASK,
        );
    }
    oxygen_write32(chip, OXYGEN_SPDIF_OUTPUT_BITS, field!(chip, spdif_bits));
    oxygen_write16(chip, OXYGEN_2WIRE_BUS_STATUS, OXYGEN_2WIRE_LENGTH_8 | OXYGEN_2WIRE_INTERRUPT_MASK | OXYGEN_2WIRE_SPEED_STANDARD);
    oxygen_clear_bits8(chip, OXYGEN_MPU401_CONTROL, OXYGEN_MPU401_LOOPBACK);
    oxygen_write8(chip, OXYGEN_GPI_INTERRUPT_MASK, 0);
    oxygen_write16(chip, OXYGEN_GPIO_INTERRUPT_MASK, 0);
    oxygen_write16(
        chip,
        OXYGEN_PLAY_ROUTING,
        OXYGEN_PLAY_MULTICH_I2S_DAC
            | OXYGEN_PLAY_SPDIF_SPDIF
            | (0 << OXYGEN_PLAY_DAC0_SOURCE_SHIFT)
            | (1 << OXYGEN_PLAY_DAC1_SOURCE_SHIFT)
            | (2 << OXYGEN_PLAY_DAC2_SOURCE_SHIFT)
            | (3 << OXYGEN_PLAY_DAC3_SOURCE_SHIFT),
    );
    oxygen_write8(chip, OXYGEN_REC_ROUTING, OXYGEN_REC_A_ROUTE_I2S_ADC_1 | OXYGEN_REC_B_ROUTE_I2S_ADC_2 | OXYGEN_REC_C_ROUTE_SPDIF);
    oxygen_write8(chip, OXYGEN_ADC_MONITOR, 0);
    oxygen_write8(
        chip,
        OXYGEN_A_MONITOR_ROUTING,
        (0 << OXYGEN_A_MONITOR_ROUTE_0_SHIFT)
            | (1 << OXYGEN_A_MONITOR_ROUTE_1_SHIFT)
            | (2 << OXYGEN_A_MONITOR_ROUTE_2_SHIFT)
            | (3 << OXYGEN_A_MONITOR_ROUTE_3_SHIFT),
    );

    if field!(chip, has_ac97_0) | field!(chip, has_ac97_1) {
        oxygen_write8(chip, OXYGEN_AC97_INTERRUPT_MASK, OXYGEN_AC97_INT_READ_DONE | OXYGEN_AC97_INT_WRITE_DONE);
    } else {
        oxygen_write8(chip, OXYGEN_AC97_INTERRUPT_MASK, 0);
    }
    oxygen_write32(chip, OXYGEN_AC97_OUT_CONFIG, 0);
    oxygen_write32(chip, OXYGEN_AC97_IN_CONFIG, 0);
    if !(field!(chip, has_ac97_0) | field!(chip, has_ac97_1)) {
        oxygen_set_bits16(chip, OXYGEN_AC97_CONTROL, OXYGEN_AC97_CLOCK_DISABLE);
    }
    if !field!(chip, has_ac97_0) {
        oxygen_set_bits16(chip, OXYGEN_AC97_CONTROL, OXYGEN_AC97_NO_CODEC_0);
    } else {
        oxygen_write_ac97(chip, 0, AC97_RESET, 0);
        msleep(1);
        oxygen_ac97_set_bits(chip, 0, CM9780_GPIO_SETUP, CM9780_GPIO0IO | CM9780_GPIO1IO);
        oxygen_ac97_set_bits(chip, 0, CM9780_MIXER, CM9780_BSTSEL | CM9780_STRO_MIC | CM9780_MIX2FR | CM9780_PCBSW);
        oxygen_ac97_set_bits(chip, 0, CM9780_JACK, CM9780_RSOE | CM9780_CBOE | CM9780_SSOE | CM9780_FROE | CM9780_MIC2MIC | CM9780_LI2LI);
        oxygen_write_ac97(chip, 0, AC97_MASTER, 0x0000);
        oxygen_write_ac97(chip, 0, AC97_PC_BEEP, 0x8000);
        oxygen_write_ac97(chip, 0, AC97_MIC, 0x8808);
        oxygen_write_ac97(chip, 0, AC97_LINE, 0x0808);
        oxygen_write_ac97(chip, 0, AC97_CD, 0x8808);
        oxygen_write_ac97(chip, 0, AC97_VIDEO, 0x8808);
        oxygen_write_ac97(chip, 0, AC97_AUX, 0x8808);
        oxygen_write_ac97(chip, 0, AC97_REC_GAIN, 0x8000);
        oxygen_write_ac97(chip, 0, AC97_CENTER_LFE_MASTER, 0x8080);
        oxygen_write_ac97(chip, 0, AC97_SURROUND_MASTER, 0x8080);
        oxygen_ac97_clear_bits(chip, 0, CM9780_GPIO_STATUS, CM9780_GPO0);
        /* power down unused ADCs and DACs */
        oxygen_ac97_set_bits(chip, 0, AC97_POWERDOWN, AC97_PD_PR0 | AC97_PD_PR1);
        oxygen_ac97_set_bits(chip, 0, AC97_EXTENDED_STATUS, AC97_EA_PRI | AC97_EA_PRJ | AC97_EA_PRK);
    }
    if field!(chip, has_ac97_1) {
        oxygen_set_bits32(chip, OXYGEN_AC97_OUT_CONFIG, OXYGEN_AC97_CODEC1_SLOT3 | OXYGEN_AC97_CODEC1_SLOT4);
        oxygen_write_ac97(chip, 1, AC97_RESET, 0);
        msleep(1);
        oxygen_write_ac97(chip, 1, AC97_MASTER, 0x0000);
        oxygen_write_ac97(chip, 1, AC97_HEADPHONE, 0x8000);
        oxygen_write_ac97(chip, 1, AC97_PC_BEEP, 0x8000);
        oxygen_write_ac97(chip, 1, AC97_MIC, 0x8808);
        oxygen_write_ac97(chip, 1, AC97_LINE, 0x8808);
        oxygen_write_ac97(chip, 1, AC97_CD, 0x8808);
        oxygen_write_ac97(chip, 1, AC97_VIDEO, 0x8808);
        oxygen_write_ac97(chip, 1, AC97_AUX, 0x8808);
        oxygen_write_ac97(chip, 1, AC97_PCM, 0x0808);
        oxygen_write_ac97(chip, 1, AC97_REC_SEL, 0x0000);
        oxygen_write_ac97(chip, 1, AC97_REC_GAIN, 0x0000);
        oxygen_ac97_set_bits(chip, 1, 0x6a, 0x0040);
    }
}

unsafe fn oxygen_shutdown(chip: *mut oxygen) {
    // guard(spinlock_irq)(&chip->reg_lock)
    field!(chip, interrupt_mask) = 0;
    field!(chip, pcm_running) = 0;
    oxygen_write16(chip, OXYGEN_DMA_STATUS, 0);
    oxygen_write16(chip, OXYGEN_INTERRUPT_MASK, 0);
}

unsafe extern "C" fn oxygen_card_free(card: *mut snd_card) {
    let chip: *mut oxygen = field!(card, private_data);

    oxygen_shutdown(chip);
    flush_work(&mut field!(chip, spdif_input_bits_work));
    flush_work(&mut field!(chip, gpio_work));
    field!(chip, model.cleanup).unwrap()(chip);
    mutex_destroy(&mut field!(chip, mutex));
}

unsafe fn __oxygen_pci_probe(
    pci: *mut pci_dev,
    index: c_int,
    id: *mut c_char,
    owner: *mut module,
    ids: *const pci_device_id,
    get_model: unsafe extern "C" fn(*mut oxygen, *const pci_device_id) -> c_int,
) -> c_int {
    let mut card: *mut snd_card = ::std::ptr::null_mut();
    let chip: *mut oxygen;
    let pci_id: *const pci_device_id;
    let mut err: c_int;

    err = snd_devm_card_new(&mut field!(pci, dev), index, id, owner, ::std::mem::size_of::<oxygen>(), &mut card);
    if err < 0 {
        return err;
    }

    chip = field!(card, private_data);
    field!(chip, card) = card;
    field!(chip, pci) = pci;
    field!(chip, irq) = -1;
    spin_lock_init(&mut field!(chip, reg_lock));
    mutex_init(&mut field!(chip, mutex));
    INIT_WORK(&mut field!(chip, spdif_input_bits_work), oxygen_spdif_input_bits_changed);
    INIT_WORK(&mut field!(chip, gpio_work), oxygen_gpio_changed);
    init_waitqueue_head(&mut field!(chip, ac97_waitqueue));

    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }

    err = pcim_request_all_regions(pci, DRIVER.as_ptr());
    if err < 0 {
        dev_err(field!(card, dev), b"cannot reserve PCI resources\n\0".as_ptr() as *const c_char);
        return err;
    }

    if (pci_resource_flags(pci, 0) & IORESOURCE_IO) == 0 || pci_resource_len(pci, 0) < OXYGEN_IO_SIZE as usize {
        dev_err(field!(card, dev), b"invalid PCI I/O range\n\0".as_ptr() as *const c_char);
        return -ENXIO;
    }
    field!(chip, addr) = pci_resource_start(pci, 0);

    pci_id = oxygen_search_pci_id(chip, ids);
    if pci_id.is_null() {
        return -ENODEV;
    }

    oxygen_restore_eeprom(chip, pci_id);
    err = get_model(chip, pci_id);
    if err < 0 {
        return err;
    }

    if field!(chip, model.model_data_size) != 0 {
        field!(chip, model_data) = devm_kzalloc(&mut field!(pci, dev), field!(chip, model.model_data_size), GFP_KERNEL);
        if field!(chip, model_data).is_null() {
            return -ENOMEM;
        }
    }

    pci_set_master(pci);
    field!(card, private_free) = Some(oxygen_card_free);

    configure_pcie_bridge(pci);
    oxygen_init(chip);
    field!(chip, model.init).unwrap()(chip);

    err = devm_request_irq(
        &mut field!(pci, dev),
        field!(pci, irq),
        oxygen_interrupt,
        IRQF_SHARED,
        KBUILD_MODNAME,
        chip as *mut c_void,
    );
    if err < 0 {
        dev_err(field!(card, dev), b"cannot grab interrupt %d\n\0".as_ptr() as *const c_char, field!(pci, irq));
        return err;
    }
    field!(chip, irq) = field!(pci, irq);
    field!(card, sync_irq) = field!(chip, irq);

    strscpy(field!(card, driver), field!(chip, model.chip));
    strscpy(field!(card, shortname), field!(chip, model.shortname));
    sprintf(
        field!(card, longname),
        b"%s at %#lx, irq %i\0".as_ptr() as *const c_char,
        field!(chip, model.longname),
        field!(chip, addr),
        field!(chip, irq),
    );
    strscpy(field!(card, mixername), field!(chip, model.chip));
    snd_component_add(card, field!(chip, model.chip));

    err = oxygen_pcm_init(chip);
    if err < 0 {
        return err;
    }

    err = oxygen_mixer_init(chip);
    if err < 0 {
        return err;
    }

    if (field!(chip, model.device_config) & (MIDI_OUTPUT | MIDI_INPUT)) != 0 {
        let mut info_flags: c_uint = MPU401_INFO_INTEGRATED | MPU401_INFO_IRQ_HOOK;
        if (field!(chip, model.device_config) & MIDI_OUTPUT) != 0 {
            info_flags |= MPU401_INFO_OUTPUT;
        }
        if (field!(chip, model.device_config) & MIDI_INPUT) != 0 {
            info_flags |= MPU401_INFO_INPUT;
        }
        err = snd_mpu401_uart_new(
            card,
            0,
            MPU401_HW_CMIPCI,
            field!(chip, addr) + OXYGEN_MPU401 as usize,
            info_flags,
            -1,
            &mut field!(chip, midi),
        );
        if err < 0 {
            return err;
        }
    }

    oxygen_proc_init(chip);

    // scoped_guard(spinlock_irq, &chip->reg_lock)
    {
        if (field!(chip, model.device_config) & CAPTURE_1_FROM_SPDIF) != 0 {
            field!(chip, interrupt_mask) |= OXYGEN_INT_SPDIF_IN_DETECT;
        }
        if field!(chip, has_ac97_0) | field!(chip, has_ac97_1) {
            field!(chip, interrupt_mask) |= OXYGEN_INT_AC97;
        }
        oxygen_write16(chip, OXYGEN_INTERRUPT_MASK, field!(chip, interrupt_mask));
    }

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    pci_set_drvdata(pci, card as *mut c_void);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_pci_probe(
    pci: *mut pci_dev,
    index: c_int,
    id: *mut c_char,
    owner: *mut module,
    ids: *const pci_device_id,
    get_model: unsafe extern "C" fn(*mut oxygen, *const pci_device_id) -> c_int,
) -> c_int {
    snd_card_free_on_error(
        &mut field!(pci, dev),
        __oxygen_pci_probe(pci, index, id, owner, ids, get_model),
    )
}

unsafe extern "C" fn oxygen_pci_suspend(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let chip: *mut oxygen = field!(card, private_data);
    let saved_interrupt_mask: c_uint;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);

    if field!(chip, model.suspend).is_some() {
        field!(chip, model.suspend).unwrap()(chip);
    }

    // scoped_guard(spinlock_irq, &chip->reg_lock)
    {
        saved_interrupt_mask = field!(chip, interrupt_mask);
        field!(chip, interrupt_mask) = 0;
        oxygen_write16(chip, OXYGEN_DMA_STATUS, 0);
        oxygen_write16(chip, OXYGEN_INTERRUPT_MASK, 0);
    }

    flush_work(&mut field!(chip, spdif_input_bits_work));
    flush_work(&mut field!(chip, gpio_work));
    field!(chip, interrupt_mask) = saved_interrupt_mask;
    0
}

static registers_to_restore: [u32; 8] = [
    0xffffffff, 0x00ff077f, 0x00011d08, 0x007f00ff, 0x00300000, 0x00000fe4, 0x0ff7001f, 0x00000000,
];
static ac97_registers_to_restore: [[u32; 2]; 2] = [[0x18284fa2, 0x03060000], [0x00007fa6, 0x00200000]];

#[inline]
unsafe fn is_bit_set(bitmap: *const u32, bit: c_uint) -> c_int {
    ((*bitmap.add((bit / 32) as usize) & (1 << (bit & 31))) != 0) as c_int
}

unsafe fn oxygen_restore_ac97(chip: *mut oxygen, codec: c_uint) {
    let mut i: c_uint;

    oxygen_write_ac97(chip, codec, AC97_RESET, 0);
    msleep(1);
    i = 1;
    while i < 0x40 {
        if is_bit_set(ac97_registers_to_restore[codec as usize].as_ptr(), i) != 0 {
            oxygen_write_ac97(chip, codec, i * 2, field!(chip, saved_ac97_registers)[codec as usize][i as usize]);
        }
        i += 1;
    }
}

unsafe extern "C" fn oxygen_pci_resume(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let chip: *mut oxygen = field!(card, private_data);
    let mut i: c_uint;

    oxygen_write16(chip, OXYGEN_DMA_STATUS, 0);
    oxygen_write16(chip, OXYGEN_INTERRUPT_MASK, 0);
    i = 0;
    while i < OXYGEN_IO_SIZE {
        if is_bit_set(registers_to_restore.as_ptr(), i) != 0 {
            oxygen_write8(chip, i, field!(chip, saved_registers)._8[i as usize]);
        }
        i += 1;
    }
    if field!(chip, has_ac97_0) {
        oxygen_restore_ac97(chip, 0);
    }
    if field!(chip, has_ac97_1) {
        oxygen_restore_ac97(chip, 1);
    }

    if field!(chip, model.resume).is_some() {
        field!(chip, model.resume).unwrap()(chip);
    }

    oxygen_write16(chip, OXYGEN_INTERRUPT_MASK, field!(chip, interrupt_mask));

    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

// EXPORT_SIMPLE_DEV_PM_OPS(oxygen_pci_pm, oxygen_pci_suspend, oxygen_pci_resume);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_pci_shutdown(pci: *mut pci_dev) {
    let card: *mut snd_card = pci_get_drvdata(pci) as *mut snd_card;
    let chip: *mut oxygen = field!(card, private_data);

    oxygen_shutdown(chip);
    field!(chip, model.cleanup).unwrap()(chip);
}

unsafe extern "C" {
    static OXYGEN_SPDIF_C: c_uint;
    static OXYGEN_SPDIF_ORIGINAL: c_uint;
    static IEC958_AES1_CON_PCM_CODER: c_uint;
    static OXYGEN_SPDIF_CATEGORY_SHIFT: c_uint;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
