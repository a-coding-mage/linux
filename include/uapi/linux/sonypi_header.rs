/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Sony Programmable I/O Control Device driver for VAIO
 *
 * This is a Rust translation of the UAPI header.
 */

/* events the user application reading /dev/sonypi can use */
pub const SONYPI_EVENT_IGNORE: u32 = 0;
pub const SONYPI_EVENT_JOGDIAL_DOWN: u32 = 1;
pub const SONYPI_EVENT_JOGDIAL_UP: u32 = 2;
pub const SONYPI_EVENT_JOGDIAL_DOWN_PRESSED: u32 = 3;
pub const SONYPI_EVENT_JOGDIAL_UP_PRESSED: u32 = 4;
pub const SONYPI_EVENT_JOGDIAL_PRESSED: u32 = 5;
pub const SONYPI_EVENT_JOGDIAL_RELEASED: u32 = 6; /* obsolete */
pub const SONYPI_EVENT_CAPTURE_PRESSED: u32 = 7;
pub const SONYPI_EVENT_CAPTURE_RELEASED: u32 = 8; /* obsolete */
pub const SONYPI_EVENT_CAPTURE_PARTIALPRESSED: u32 = 9;
pub const SONYPI_EVENT_CAPTURE_PARTIALRELEASED: u32 = 10;
pub const SONYPI_EVENT_FNKEY_ESC: u32 = 11;
pub const SONYPI_EVENT_FNKEY_F1: u32 = 12;
pub const SONYPI_EVENT_FNKEY_F2: u32 = 13;
pub const SONYPI_EVENT_FNKEY_F3: u32 = 14;
pub const SONYPI_EVENT_FNKEY_F4: u32 = 15;
pub const SONYPI_EVENT_FNKEY_F5: u32 = 16;
pub const SONYPI_EVENT_FNKEY_F6: u32 = 17;
pub const SONYPI_EVENT_FNKEY_F7: u32 = 18;
pub const SONYPI_EVENT_FNKEY_F8: u32 = 19;
pub const SONYPI_EVENT_FNKEY_F9: u32 = 20;
pub const SONYPI_EVENT_FNKEY_F10: u32 = 21;
pub const SONYPI_EVENT_FNKEY_F11: u32 = 22;
pub const SONYPI_EVENT_FNKEY_F12: u32 = 23;
pub const SONYPI_EVENT_FNKEY_1: u32 = 24;
pub const SONYPI_EVENT_FNKEY_2: u32 = 25;
pub const SONYPI_EVENT_FNKEY_D: u32 = 26;
pub const SONYPI_EVENT_FNKEY_E: u32 = 27;
pub const SONYPI_EVENT_FNKEY_F: u32 = 28;
pub const SONYPI_EVENT_FNKEY_S: u32 = 29;
pub const SONYPI_EVENT_FNKEY_B: u32 = 30;
pub const SONYPI_EVENT_BLUETOOTH_PRESSED: u32 = 31;
pub const SONYPI_EVENT_PKEY_P1: u32 = 32;
pub const SONYPI_EVENT_PKEY_P2: u32 = 33;
pub const SONYPI_EVENT_PKEY_P3: u32 = 34;
pub const SONYPI_EVENT_BACK_PRESSED: u32 = 35;
pub const SONYPI_EVENT_LID_CLOSED: u32 = 36;
pub const SONYPI_EVENT_LID_OPENED: u32 = 37;
pub const SONYPI_EVENT_BLUETOOTH_ON: u32 = 38;
pub const SONYPI_EVENT_BLUETOOTH_OFF: u32 = 39;
pub const SONYPI_EVENT_HELP_PRESSED: u32 = 40;
pub const SONYPI_EVENT_FNKEY_ONLY: u32 = 41;
pub const SONYPI_EVENT_JOGDIAL_FAST_DOWN: u32 = 42;
pub const SONYPI_EVENT_JOGDIAL_FAST_UP: u32 = 43;
pub const SONYPI_EVENT_JOGDIAL_FAST_DOWN_PRESSED: u32 = 44;
pub const SONYPI_EVENT_JOGDIAL_FAST_UP_PRESSED: u32 = 45;
pub const SONYPI_EVENT_JOGDIAL_VFAST_DOWN: u32 = 46;
pub const SONYPI_EVENT_JOGDIAL_VFAST_UP: u32 = 47;
pub const SONYPI_EVENT_JOGDIAL_VFAST_DOWN_PRESSED: u32 = 48;
pub const SONYPI_EVENT_JOGDIAL_VFAST_UP_PRESSED: u32 = 49;
pub const SONYPI_EVENT_ZOOM_PRESSED: u32 = 50;
pub const SONYPI_EVENT_THUMBPHRASE_PRESSED: u32 = 51;
pub const SONYPI_EVENT_MEYE_FACE: u32 = 52;
pub const SONYPI_EVENT_MEYE_OPPOSITE: u32 = 53;
pub const SONYPI_EVENT_MEMORYSTICK_INSERT: u32 = 54;
pub const SONYPI_EVENT_MEMORYSTICK_EJECT: u32 = 55;
pub const SONYPI_EVENT_ANYBUTTON_RELEASED: u32 = 56;
pub const SONYPI_EVENT_BATTERY_INSERT: u32 = 57;
pub const SONYPI_EVENT_BATTERY_REMOVE: u32 = 58;
pub const SONYPI_EVENT_FNKEY_RELEASED: u32 = 59;
pub const SONYPI_EVENT_WIRELESS_ON: u32 = 60;
pub const SONYPI_EVENT_WIRELESS_OFF: u32 = 61;
pub const SONYPI_EVENT_ZOOM_IN_PRESSED: u32 = 62;
pub const SONYPI_EVENT_ZOOM_OUT_PRESSED: u32 = 63;
pub const SONYPI_EVENT_CD_EJECT_PRESSED: u32 = 64;
pub const SONYPI_EVENT_MODEKEY_PRESSED: u32 = 65;
pub const SONYPI_EVENT_PKEY_P4: u32 = 66;
pub const SONYPI_EVENT_PKEY_P5: u32 = 67;
pub const SONYPI_EVENT_SETTINGKEY_PRESSED: u32 = 68;
pub const SONYPI_EVENT_VOLUME_INC_PRESSED: u32 = 69;
pub const SONYPI_EVENT_VOLUME_DEC_PRESSED: u32 = 70;
pub const SONYPI_EVENT_BRIGHTNESS_PRESSED: u32 = 71;
pub const SONYPI_EVENT_MEDIA_PRESSED: u32 = 72;
pub const SONYPI_EVENT_VENDOR_PRESSED: u32 = 73;

/* Linux _IOC encoding used by the original _IOR/_IOW declarations. */
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;
const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT) | (size << IOC_SIZESHIFT) | (ty << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT)
}
const fn ior<T>(ty: u32, nr: u32) -> u32 { ioc(IOC_READ, ty, nr, core::mem::size_of::<T>() as u32) }
const fn iow<T>(ty: u32, nr: u32) -> u32 { ioc(IOC_WRITE, ty, nr, core::mem::size_of::<T>() as u32) }

/* get/set brightness */
pub const SONYPI_IOCGBRT: u32 = ior::<u8>(b'v' as u32, 0);
pub const SONYPI_IOCSBRT: u32 = iow::<u8>(b'v' as u32, 0);
/* get battery full capacity/remaining capacity */
pub const SONYPI_IOCGBAT1CAP: u32 = ior::<u16>(b'v' as u32, 2);
pub const SONYPI_IOCGBAT1REM: u32 = ior::<u16>(b'v' as u32, 3);
pub const SONYPI_IOCGBAT2CAP: u32 = ior::<u16>(b'v' as u32, 4);
pub const SONYPI_IOCGBAT2REM: u32 = ior::<u16>(b'v' as u32, 5);
/* get battery flags: battery1/battery2/ac adapter present */
pub const SONYPI_BFLAGS_B1: u8 = 0x01;
pub const SONYPI_BFLAGS_B2: u8 = 0x02;
pub const SONYPI_BFLAGS_AC: u8 = 0x04;
pub const SONYPI_IOCGBATFLAGS: u32 = ior::<u8>(b'v' as u32, 7);
/* get/set bluetooth subsystem state on/off */
pub const SONYPI_IOCGBLUE: u32 = ior::<u8>(b'v' as u32, 8);
pub const SONYPI_IOCSBLUE: u32 = iow::<u8>(b'v' as u32, 9);
/* get/set fan state on/off */
pub const SONYPI_IOCGFAN: u32 = ior::<u8>(b'v' as u32, 10);
pub const SONYPI_IOCSFAN: u32 = iow::<u8>(b'v' as u32, 11);
/* get temperature (C) */
pub const SONYPI_IOCGTEMP: u32 = ior::<u8>(b'v' as u32, 12);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
