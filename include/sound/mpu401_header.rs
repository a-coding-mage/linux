/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Header file for MPU-401 and compatible cards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

/* Dependencies supplied by the surrounding kernel/Rust translation. */

pub const MPU401_HW_MPU401: i32 = 1; /* native MPU401 */
pub const MPU401_HW_SB: i32 = 2; /* SoundBlaster MPU-401 UART */
pub const MPU401_HW_ES1688: i32 = 3; /* AudioDrive ES1688 MPU-401 UART */
pub const MPU401_HW_OPL3SA2: i32 = 4; /* Yamaha OPL3-SA2 */
pub const MPU401_HW_SONICVIBES: i32 = 5; /* S3 SonicVibes */
pub const MPU401_HW_CS4232: i32 = 6; /* CS4232 */
pub const MPU401_HW_ES18XX: i32 = 7; /* AudioDrive ES18XX MPU-401 UART */
pub const MPU401_HW_FM801: i32 = 8; /* ForteMedia FM801 */
pub const MPU401_HW_TRID4DWAVE: i32 = 9; /* Trident 4DWave */
pub const MPU401_HW_AZT2320: i32 = 10; /* Aztech AZT2320 */
pub const MPU401_HW_ALS100: i32 = 11; /* Avance Logic ALS100 */
pub const MPU401_HW_ICE1712: i32 = 12; /* Envy24 */
pub const MPU401_HW_VIA686A: i32 = 13; /* VIA 82C686A */
pub const MPU401_HW_YMFPCI: i32 = 14; /* YMF DS-XG PCI */
pub const MPU401_HW_CMIPCI: i32 = 15; /* CMIPCI MPU-401 UART */
pub const MPU401_HW_ALS4000: i32 = 16; /* Avance Logic ALS4000 */
pub const MPU401_HW_INTEL8X0: i32 = 17; /* Intel8x0 driver */
pub const MPU401_HW_PC98II: i32 = 18; /* Roland PC98II */
pub const MPU401_HW_AUREAL: i32 = 19; /* Aureal Vortex */

pub const MPU401_INFO_INPUT: u32 = 1 << 0; /* input stream */
pub const MPU401_INFO_OUTPUT: u32 = 1 << 1; /* output stream */
pub const MPU401_INFO_INTEGRATED: u32 = 1 << 2; /* integrated h/w port */
pub const MPU401_INFO_MMIO: u32 = 1 << 3; /* MMIO access */
pub const MPU401_INFO_TX_IRQ: u32 = 1 << 4; /* independent TX irq */
pub const MPU401_INFO_IRQ_HOOK: u32 = 1 << 5; /* mpu401 irq handler is called from driver irq handler */
pub const MPU401_INFO_NO_ACK: u32 = 1 << 6; /* No ACK cmd needed */
pub const MPU401_INFO_USE_TIMER: u32 = 1 << 15; /* internal */

pub const MPU401_MODE_BIT_INPUT: u32 = 0;
pub const MPU401_MODE_BIT_OUTPUT: u32 = 1;
pub const MPU401_MODE_BIT_INPUT_TRIGGER: u32 = 2;
pub const MPU401_MODE_BIT_OUTPUT_TRIGGER: u32 = 3;

pub const MPU401_MODE_INPUT: u32 = 1 << MPU401_MODE_BIT_INPUT;
pub const MPU401_MODE_OUTPUT: u32 = 1 << MPU401_MODE_BIT_OUTPUT;
pub const MPU401_MODE_INPUT_TRIGGER: u32 = 1 << MPU401_MODE_BIT_INPUT_TRIGGER;
pub const MPU401_MODE_OUTPUT_TRIGGER: u32 = 1 << MPU401_MODE_BIT_OUTPUT_TRIGGER;

pub const MPU401_MODE_INPUT_TIMER: u32 = 1 << 0;
pub const MPU401_MODE_OUTPUT_TIMER: u32 = 1 << 1;

#[repr(C)]
pub struct snd_mpu401 {
    pub rmidi: *mut snd_rawmidi,
    pub hardware: u16,
    pub info_flags: u32,
    pub port: usize,
    pub cport: usize,
    pub res: *mut resource,
    pub irq: i32,
    pub mode: usize,
    pub timer_invoked: i32,
    pub open_input: Option<unsafe extern "C" fn(*mut snd_mpu401) -> i32>,
    pub close_input: Option<unsafe extern "C" fn(*mut snd_mpu401)>,
    pub open_output: Option<unsafe extern "C" fn(*mut snd_mpu401) -> i32>,
    pub close_output: Option<unsafe extern "C" fn(*mut snd_mpu401)>,
    pub private_data: *mut core::ffi::c_void,
    pub substream_input: *mut snd_rawmidi_substream,
    pub substream_output: *mut snd_rawmidi_substream,
    pub input_lock: spinlock_t,
    pub output_lock: spinlock_t,
    pub timer_lock: spinlock_t,
    pub timer: timer_list,
    pub write: Option<unsafe extern "C" fn(*mut snd_mpu401, u8, usize)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_mpu401, usize) -> u8>,
}

/* I/O ports */
pub unsafe fn MPU401C(mpu: *mut snd_mpu401) -> usize { (*mpu).cport }
pub unsafe fn MPU401D(mpu: *mut snd_mpu401) -> usize { (*mpu).port }

/* control register bits */
pub const MPU401_RX_EMPTY: u8 = 0x80;
pub const MPU401_TX_FULL: u8 = 0x40;
pub const MPU401_RESET: u8 = 0xff;
pub const MPU401_ENTER_UART: u8 = 0x3f;
pub const MPU401_ACK: u8 = 0xfe;

extern "C" {
    pub fn snd_mpu401_uart_interrupt(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn snd_mpu401_uart_interrupt_tx(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: i32,
        hardware: u16,
        port: usize,
        info_flags: u32,
        irq: i32,
        rrawmidi: *mut *mut snd_rawmidi,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
