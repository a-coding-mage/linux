/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Definitions for talking to the PMU.  The PMU is a microcontroller
 * which controls battery charging and system power on PowerBook 3400
 * and 2400 models as well as the RTC and various other things.
 *
 * Copyright (C) 1998 Paul Mackerras.
 */

pub const PMU_DRIVER_VERSION: i32 = 2;

/* PMU commands */
pub const PMU_POWER_CTRL0: i32 = 0x10;
pub const PMU_POWER_CTRL: i32 = 0x11;
pub const PMU_ADB_CMD: i32 = 0x20;
pub const PMU_ADB_POLL_OFF: i32 = 0x21;
pub const PMU_WRITE_XPRAM: i32 = 0x32;
pub const PMU_WRITE_NVRAM: i32 = 0x33;
pub const PMU_READ_XPRAM: i32 = 0x3a;
pub const PMU_READ_NVRAM: i32 = 0x3b;
pub const PMU_SET_RTC: i32 = 0x30;
pub const PMU_READ_RTC: i32 = 0x38;
pub const PMU_SET_VOLBUTTON: i32 = 0x40;
pub const PMU_BACKLIGHT_BRIGHT: i32 = 0x41;
pub const PMU_GET_VOLBUTTON: i32 = 0x48;
pub const PMU_PCEJECT: i32 = 0x4c;
pub const PMU_BATTERY_STATE: i32 = 0x6b;
pub const PMU_SMART_BATTERY_STATE: i32 = 0x6f;
pub const PMU_SET_INTR_MASK: i32 = 0x70;
pub const PMU_INT_ACK: i32 = 0x78;
pub const PMU_SHUTDOWN: i32 = 0x7e;
pub const PMU_CPU_SPEED: i32 = 0x7d;
pub const PMU_SLEEP: i32 = 0x7f;
pub const PMU_POWER_EVENTS: i32 = 0x8f;
pub const PMU_I2C_CMD: i32 = 0x9a;
pub const PMU_RESET: i32 = 0xd0;
pub const PMU_GET_BRIGHTBUTTON: i32 = 0xd9;
pub const PMU_GET_COVER: i32 = 0xdc;
pub const PMU_SYSTEM_READY: i32 = 0xdf;
pub const PMU_GET_VERSION: i32 = 0xea;

pub const PMU_POW0_ON: i32 = 0x80;
pub const PMU_POW0_OFF: i32 = 0x00;
pub const PMU_POW0_HARD_DRIVE: i32 = 0x04;
pub const PMU_POW_ON: i32 = 0x80;
pub const PMU_POW_OFF: i32 = 0x00;
pub const PMU_POW_BACKLIGHT: i32 = 0x01;
pub const PMU_POW_CHARGER: i32 = 0x02;
pub const PMU_POW_IRLED: i32 = 0x04;
pub const PMU_POW_MEDIABAY: i32 = 0x08;

pub const PMU_INT_PCEJECT: i32 = 0x04;
pub const PMU_INT_SNDBRT: i32 = 0x08;
pub const PMU_INT_ADB: i32 = 0x10;
pub const PMU_INT_BATTERY: i32 = 0x20;
pub const PMU_INT_ENVIRONMENT: i32 = 0x40;
pub const PMU_INT_TICK: i32 = 0x80;
pub const PMU_INT_ADB_AUTO: i32 = 0x04;
pub const PMU_INT_WAITING_CHARGER: i32 = 0x01;
pub const PMU_INT_AUTO_SRQ_POLL: i32 = 0x02;
pub const PMU_ENV_LID_CLOSED: i32 = 0x01;

pub const PMU_I2C_MODE_SIMPLE: i32 = 0;
pub const PMU_I2C_MODE_STDSUB: i32 = 1;
pub const PMU_I2C_MODE_COMBINED: i32 = 2;
pub const PMU_I2C_BUS_STATUS: i32 = 0;
pub const PMU_I2C_BUS_SYSCLK: i32 = 1;
pub const PMU_I2C_BUS_POWER: i32 = 2;
pub const PMU_I2C_STATUS_OK: i32 = 0;
pub const PMU_I2C_STATUS_DATAREAD: i32 = 1;
pub const PMU_I2C_STATUS_BUSY: i32 = 0xfe;

/* Kind of PMU (model) */
pub const PMU_UNKNOWN: i32 = 0;
pub const PMU_OHARE_BASED: i32 = 1;
pub const PMU_HEATHROW_BASED: i32 = 2;
pub const PMU_PADDINGTON_BASED: i32 = 3;
pub const PMU_KEYLARGO_BASED: i32 = 4;
pub const PMU_68K_V1: i32 = 5;
pub const PMU_68K_V2: i32 = 6;

/* PMU PMU_POWER_EVENTS commands */
pub const PMU_PWR_GET_POWERUP_EVENTS: i32 = 0x00;
pub const PMU_PWR_SET_POWERUP_EVENTS: i32 = 0x01;
pub const PMU_PWR_CLR_POWERUP_EVENTS: i32 = 0x02;
pub const PMU_PWR_GET_WAKEUP_EVENTS: i32 = 0x03;
pub const PMU_PWR_SET_WAKEUP_EVENTS: i32 = 0x04;
pub const PMU_PWR_CLR_WAKEUP_EVENTS: i32 = 0x05;

/* Power events wakeup bits */
pub const PMU_PWR_WAKEUP_KEY: i32 = 0x01;
pub const PMU_PWR_WAKEUP_AC_INSERT: i32 = 0x02;
pub const PMU_PWR_WAKEUP_AC_CHANGE: i32 = 0x04;
pub const PMU_PWR_WAKEUP_LID_OPEN: i32 = 0x08;
pub const PMU_PWR_WAKEUP_RING: i32 = 0x10;

/* Ioctl commands require the external Linux ioctl macro definitions. */
pub const PMU_IOC_SLEEP: usize = _IO('B', 0);
pub const PMU_IOC_GET_BACKLIGHT: usize = _IOR('B', 1, usize);
pub const PMU_IOC_SET_BACKLIGHT: usize = _IOW('B', 2, usize);
pub const PMU_IOC_GET_MODEL: usize = _IOR('B', 3, usize);
pub const PMU_IOC_HAS_ADB: usize = _IOR('B', 4, usize);
pub const PMU_IOC_CAN_SLEEP: usize = _IOR('B', 5, usize);
pub const PMU_IOC_GRAB_BACKLIGHT: usize = _IOR('B', 6, usize);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
