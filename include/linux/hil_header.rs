/* Hewlett Packard Human Interface Loop (HP-HIL) Protocol -- header. */

// #include <asm/types.h>

pub const HIL_CLOCK: u32 = 8_000_000;
pub const HIL_EK1_CLOCK: u32 = 30;
pub const HIL_EK2_CLOCK: u32 = 60;
pub const HIL_TIMEOUT_DEV: u32 = 5;
pub const HIL_TIMEOUT_DEVS: u32 = 10;
pub const HIL_TIMEOUT_NORESP: u32 = 10;
pub const HIL_TIMEOUT_DEVS_DATA: u32 = 16;
pub const HIL_TIMEOUT_SELFTEST: u32 = 200;

pub const HIL_WIRE_PACKET_LEN: usize = 15;
pub const HIL_WIRE_START: u32 = 0;
pub const HIL_WIRE_ADDR2: u32 = 1;
pub const HIL_WIRE_ADDR1: u32 = 2;
pub const HIL_WIRE_ADDR0: u32 = 3;
pub const HIL_WIRE_COMMAND: u32 = 4;
pub const HIL_WIRE_DATA7: u32 = 5;
pub const HIL_WIRE_DATA6: u32 = 6;
pub const HIL_WIRE_DATA5: u32 = 7;
pub const HIL_WIRE_DATA4: u32 = 8;
pub const HIL_WIRE_DATA3: u32 = 9;
pub const HIL_WIRE_DATA2: u32 = 10;
pub const HIL_WIRE_DATA1: u32 = 11;
pub const HIL_WIRE_DATA0: u32 = 12;
pub const HIL_WIRE_PARITY: u32 = 13;
pub const HIL_WIRE_STOP: u32 = 14;

pub const HIL_PKT_CMD: u32 = 0x00000800;
pub const HIL_PKT_ADDR2: u32 = 0x00000400;
pub const HIL_PKT_ADDR1: u32 = 0x00000200;
pub const HIL_PKT_ADDR0: u32 = 0x00000100;
pub const HIL_PKT_ADDR_MASK: u32 = 0x00000700;
pub const HIL_PKT_ADDR_SHIFT: u32 = 8;
pub const HIL_PKT_DATA7: u32 = 0x80;
pub const HIL_PKT_DATA6: u32 = 0x40;
pub const HIL_PKT_DATA5: u32 = 0x20;
pub const HIL_PKT_DATA4: u32 = 0x10;
pub const HIL_PKT_DATA3: u32 = 0x08;
pub const HIL_PKT_DATA2: u32 = 0x04;
pub const HIL_PKT_DATA1: u32 = 0x02;
pub const HIL_PKT_DATA0: u32 = 0x01;
pub const HIL_PKT_DATA_MASK: u32 = 0xff;
pub const HIL_PKT_DATA_SHIFT: u32 = 0;

pub const HIL_ERR_OB: u32 = 0x00000800;
pub const HIL_ERR_INT: u32 = 0x00010000;
pub const HIL_ERR_NMI: u32 = 0x00020000;
pub const HIL_ERR_LERR: u32 = 0x00040000;
pub const HIL_ERR_PERR: u32 = 0x01000000;
pub const HIL_ERR_FERR: u32 = 0x02000000;
pub const HIL_ERR_FOF: u32 = 0x04000000;
pub const HIL_CTRL_TEST: u32 = 0x00010000;
pub const HIL_CTRL_IPF: u32 = 0x00040000;
pub const HIL_CTRL_APE: u32 = 0x02000000;
pub const HIL_DO_ALTER_CTRL: u32 = 0x40000000;
pub const HIL_CTRL_ONLY: u32 = 0xc0000000;
pub type HilPacket = u32;

pub const HIL_CMD_IFC: u32 = 0x00; pub const HIL_CMD_EPT: u32 = 0x01;
pub const HIL_CMD_ELB: u32 = 0x02; pub const HIL_CMD_IDD: u32 = 0x03;
pub const HIL_CMD_DSR: u32 = 0x04; pub const HIL_CMD_PST: u32 = 0x05;
pub const HIL_CMD_RRG: u32 = 0x06; pub const HIL_CMD_WRG: u32 = 0x07;
pub const HIL_CMD_ACF: u32 = 0x08; pub const HIL_CMDID_ACF: u32 = 0x07;
pub const HIL_CMD_POL: u32 = 0x10; pub const HIL_CMDCT_POL: u32 = 0x0f;
pub const HIL_CMD_RPL: u32 = 0x20; pub const HIL_CMDCT_RPL: u32 = 0x0f;
pub const HIL_CMD_RNM: u32 = 0x30; pub const HIL_CMD_RST: u32 = 0x31;
pub const HIL_CMD_EXD: u32 = 0x32; pub const HIL_CMD_RSC: u32 = 0x33;
pub const HIL_CMD_DKA: u32 = 0x3d; pub const HIL_CMD_EK1: u32 = 0x3e;
pub const HIL_CMD_EK2: u32 = 0x3f; pub const HIL_CMD_PR1: u32 = 0x40;
pub const HIL_CMD_PR2: u32 = 0x41; pub const HIL_CMD_PR3: u32 = 0x42;
pub const HIL_CMD_PR4: u32 = 0x43; pub const HIL_CMD_PR5: u32 = 0x44;
pub const HIL_CMD_PR6: u32 = 0x45; pub const HIL_CMD_PR7: u32 = 0x46;
pub const HIL_CMD_PRM: u32 = 0x47; pub const HIL_CMD_AK1: u32 = 0x48;
pub const HIL_CMD_AK2: u32 = 0x49; pub const HIL_CMD_AK3: u32 = 0x4a;
pub const HIL_CMD_AK4: u32 = 0x4b; pub const HIL_CMD_AK5: u32 = 0x4c;
pub const HIL_CMD_AK6: u32 = 0x4d; pub const HIL_CMD_AK7: u32 = 0x4e;
pub const HIL_CMD_ACK: u32 = 0x4f; pub const HIL_CMD_RIO: u32 = 0xfa;
pub const HIL_CMD_SHR: u32 = 0xfb; pub const HIL_CMD_TER: u32 = 0xfc;
pub const HIL_CMD_CAE: u32 = 0xfd; pub const HIL_CMD_DHR: u32 = 0xfe;

pub const HIL_IDD_DID_TYPE_MASK: u8 = 0xe0;
pub const HIL_IDD_DID_TYPE_KB_INTEGRAL: u8 = 0xa0;
pub const HIL_IDD_DID_TYPE_KB_ITF: u8 = 0xc0;
pub const HIL_IDD_DID_TYPE_KB_RSVD: u8 = 0xe0;
pub const HIL_IDD_DID_TYPE_KB_LANG_MASK: u8 = 0x1f;
pub const HIL_IDD_DID_KBLANG_USE_ESD: u8 = 0;
pub const HIL_IDD_DID_TYPE_ABS: u8 = 0x80;
pub const HIL_IDD_DID_ABS_RSVD1_MASK: u8 = 0xf8; pub const HIL_IDD_DID_ABS_RSVD1: u8 = 0x98;
pub const HIL_IDD_DID_ABS_TABLET_MASK: u8 = 0xf8; pub const HIL_IDD_DID_ABS_TABLET: u8 = 0x90;
pub const HIL_IDD_DID_ABS_TSCREEN_MASK: u8 = 0xfc; pub const HIL_IDD_DID_ABS_TSCREEN: u8 = 0x8c;
pub const HIL_IDD_DID_ABS_RSVD2_MASK: u8 = 0xfc; pub const HIL_IDD_DID_ABS_RSVD2: u8 = 0x88;
pub const HIL_IDD_DID_ABS_RSVD3_MASK: u8 = 0xfc; pub const HIL_IDD_DID_ABS_RSVD3: u8 = 0x80;
pub const HIL_IDD_DID_TYPE_REL: u8 = 0x60; pub const HIL_IDD_DID_REL_RSVD1_MASK: u8 = 0xf0; pub const HIL_IDD_DID_REL_RSVD1: u8 = 0x70;
pub const HIL_IDD_DID_REL_RSVD2_MASK: u8 = 0xfc; pub const HIL_IDD_DID_REL_RSVD2: u8 = 0x6c;
pub const HIL_IDD_DID_REL_MOUSE_MASK: u8 = 0xfc; pub const HIL_IDD_DID_REL_MOUSE: u8 = 0x68;
pub const HIL_IDD_DID_REL_QUAD_MASK: u8 = 0xf8; pub const HIL_IDD_DID_REL_QUAD: u8 = 0x60;
pub const HIL_IDD_DID_TYPE_CHAR: u8 = 0x40; pub const HIL_IDD_DID_CHAR_BARCODE_MASK: u8 = 0xfc; pub const HIL_IDD_DID_CHAR_BARCODE: u8 = 0x5c;
pub const HIL_IDD_DID_CHAR_RSVD1_MASK: u8 = 0xfc; pub const HIL_IDD_DID_CHAR_RSVD1: u8 = 0x58;
pub const HIL_IDD_DID_CHAR_RSVD2_MASK: u8 = 0xf8; pub const HIL_IDD_DID_CHAR_RSVD2: u8 = 0x50;
pub const HIL_IDD_DID_CHAR_RSVD3_MASK: u8 = 0xf0; pub const HIL_IDD_DID_CHAR_RSVD3: u8 = 0x40;
pub const HIL_IDD_DID_TYPE_OTHER: u8 = 0x20; pub const HIL_IDD_DID_OTHER_RSVD1_MASK: u8 = 0xf0; pub const HIL_IDD_DID_OTHER_RSVD1: u8 = 0x30;
pub const HIL_IDD_DID_OTHER_BARCODE_MASK: u8 = 0xfc; pub const HIL_IDD_DID_OTHER_BARCODE: u8 = 0x2c;
pub const HIL_IDD_DID_OTHER_RSVD2_MASK: u8 = 0xfc; pub const HIL_IDD_DID_OTHER_RSVD2: u8 = 0x28;
pub const HIL_IDD_DID_OTHER_RSVD3_MASK: u8 = 0xf8; pub const HIL_IDD_DID_OTHER_RSVD3: u8 = 0x20;
pub const HIL_IDD_DID_TYPE_KEYPAD: u8 = 0x00;

pub const HIL_IDD_HEADER_AXSET_MASK: u8 = 0x03; pub const HIL_IDD_HEADER_RSC: u8 = 0x04;
pub const HIL_IDD_HEADER_EXD: u8 = 0x08; pub const HIL_IDD_HEADER_IOD: u8 = 0x10;
pub const HIL_IDD_HEADER_16BIT: u8 = 0x20; pub const HIL_IDD_HEADER_ABS: u8 = 0x40;
pub const HIL_IDD_HEADER_2X_AXIS: u8 = 0x80; pub const HIL_IDD_IOD_NBUTTON_MASK: u8 = 0x07;
pub const HIL_IDD_IOD_PROXIMITY: u8 = 0x08; pub const HIL_IDD_IOD_PROMPT_MASK: u8 = 0x70;
pub const HIL_IDD_IOD_PROMPT_SHIFT: u8 = 4; pub const HIL_IDD_IOD_PROMPT: u8 = 0x80;

#[inline] pub unsafe fn HIL_IDD_NUM_AXES_PER_SET(h: *const u32) -> u32 { *h & HIL_IDD_HEADER_AXSET_MASK as u32 }
#[inline] pub unsafe fn HIL_IDD_NUM_AXSETS(h: *const u32) -> u32 { 2 - ((!(*h & HIL_IDD_HEADER_2X_AXIS as u32) != 0) as u32) }
#[inline] pub unsafe fn HIL_IDD_LEN(h: u32) -> u32 { 4 - ((!((h & HIL_IDD_HEADER_IOD as u32) != 0)) as u32) - 2 * ((!((HIL_IDD_NUM_AXES_PER_SET(&h) != 0))) as u32) + 2 * HIL_IDD_NUM_AXES_PER_SET(&h) * (((h & HIL_IDD_HEADER_ABS as u32) != 0) as u32) }
#[inline] pub unsafe fn HIL_IDD_AXIS_COUNTS_PER_M(h: *const u32) -> i32 { if HIL_IDD_NUM_AXSETS(h) == 0 { -1 } else { (((*h.add(1) & HIL_PKT_DATA_MASK) + ((*h.add(2) & HIL_PKT_DATA_MASK) << 8)) * if (*h & HIL_IDD_HEADER_16BIT as u32) != 0 { 100 } else { 1 }) as i32 } }
#[inline] pub unsafe fn HIL_IDD_AXIS_MAX(h: *const u32, n: u32) -> u32 { if (*h & HIL_IDD_HEADER_ABS as u32) == 0 || HIL_IDD_NUM_AXES_PER_SET(h) <= n { 0 } else { (HIL_PKT_DATA_MASK & *h.add((3 + 2 * n) as usize)) + ((HIL_PKT_DATA_MASK & *h.add((4 + 2 * n) as usize)) << 8) } }
#[inline] pub unsafe fn HIL_IDD_IOD(h: *const u32) -> u32 { *h.add(HIL_IDD_LEN(*h) as usize - 1) }
#[inline] pub unsafe fn HIL_IDD_HAS_GEN_PROMPT(h: *const u32) -> bool { (*h & HIL_IDD_HEADER_IOD as u32) != 0 && HIL_IDD_IOD(h) & HIL_IDD_IOD_PROMPT as u32 != 0 }
#[inline] pub unsafe fn HIL_IDD_HAS_GEN_PROXIMITY(h: *const u32) -> bool { (*h & HIL_IDD_HEADER_IOD as u32) != 0 && HIL_IDD_IOD(h) & HIL_IDD_IOD_PROXIMITY as u32 != 0 }
#[inline] pub unsafe fn HIL_IDD_NUM_BUTTONS(h: *const u32) -> u32 { if (*h & HIL_IDD_HEADER_IOD as u32) != 0 { HIL_IDD_IOD(h) & HIL_IDD_IOD_NBUTTON_MASK as u32 } else { 0 } }
#[inline] pub unsafe fn HIL_IDD_NUM_PROMPTS(h: *const u32) -> u32 { if (*h & HIL_IDD_HEADER_IOD as u32) != 0 { (HIL_IDD_IOD(h) & HIL_IDD_IOD_NPROMPT_MASK as u32) >> HIL_IDD_IOD_PROMPT_SHIFT } else { 0 } }

pub const HIL_EXD_HEADER_WRG: u32 = 0x03; pub const HIL_EXD_HEADER_WRG_TYPE1: u32 = 0x01; pub const HIL_EXD_HEADER_WRG_TYPE2: u32 = 0x02;
pub const HIL_EXD_HEADER_RRG: u32 = 0x04; pub const HIL_EXD_HEADER_RNM: u32 = 0x10; pub const HIL_EXD_HEADER_RST: u32 = 0x20; pub const HIL_EXD_HEADER_LOCALE: u32 = 0x40;
#[inline] pub unsafe fn HIL_EXD_NUM_RRG(h: *const u32) -> u32 { if *h & HIL_EXD_HEADER_RRG != 0 { *h.add(1) & HIL_PKT_DATA_MASK } else { 0 } }
#[inline] pub unsafe fn HIL_EXD_NUM_WWG(h: *const u32) -> u32 { if *h & HIL_EXD_HEADER_WRG != 0 { *h.add((2 - ((*h & HIL_EXD_HEADER_RRG == 0) as usize))) & HIL_PKT_DATA_MASK } else { 0 } }
#[inline] pub unsafe fn HIL_EXD_LEN(h: *const u32) -> u32 { ((*h & HIL_EXD_HEADER_RRG != 0) as u32) + ((*h & HIL_EXD_HEADER_WRG != 0) as u32) + ((*h & HIL_EXD_HEADER_LOCALE != 0) as u32) + 2 * ((*h & HIL_EXD_HEADER_WRG_TYPE2 != 0) as u32) + 1 }
#[inline] pub unsafe fn HIL_EXD_LOCALE(h: *const u32) -> i32 { if *h & HIL_EXD_HEADER_LOCALE == 0 { -1 } else { (*h.add(HIL_EXD_LEN(h) as usize - 1) & HIL_PKT_DATA_MASK) as i32 } }
#[inline] pub unsafe fn HIL_EXD_WRG_TYPE2_LEN(h: *const u32) -> i32 { if *h & HIL_EXD_HEADER_WRG_TYPE2 == 0 { -1 } else { ((*h.add((HIL_EXD_LEN(h) - 2 - ((*h & HIL_EXD_HEADER_LOCALE != 0) as u32)) as usize) & HIL_PKT_DATA_MASK) + ((*h.add((HIL_EXD_LEN(h) - 1 - ((*h & HIL_EXD_HEADER_LOCALE != 0) as u32)) as usize) & HIL_PKT_DATA_MASK) << 8)) as i32 } }

pub const HIL_LOCALE_MAX: usize = 0x1f;
pub const HIL_LOCALE_MAP: [&str; 32] = ["", "", "", "swiss.french", "portuguese", "arabic", "hebrew", "english.canadian", "turkish", "greek", "thai", "italian", "korean", "dutch", "swedish", "german", "chinese", "chinese", "swiss.french", "spanish", "swiss.german", "flemish", "finnish", "english.uk", "french.canadian", "swiss.german", "norwegian", "french", "danish", "japanese", "spanish", "english.us"];

pub const HIL_KEYCODES_SET1_TBLSIZE: usize = 128;
pub const HIL_KEYCODES_SET3_TBLSIZE: usize = 128;
// KEY_* names are supplied by the input subsystem dependency.
pub const HIL_KEYCODES_SET1: [u32; 128] = [
 KEY_5,KEY_RESERVED,KEY_RIGHTALT,KEY_LEFTALT,KEY_RIGHTSHIFT,KEY_LEFTSHIFT,KEY_LEFTCTRL,KEY_SYSRQ,
 KEY_KP4,KEY_KP8,KEY_KP5,KEY_KP9,KEY_KP6,KEY_KP7,KEY_KPCOMMA,KEY_KPENTER,KEY_KP1,KEY_KPSLASH,KEY_KP2,KEY_KPPLUS,KEY_KP3,KEY_KPASTERISK,KEY_KP0,KEY_KPMINUS,
 KEY_B,KEY_V,KEY_C,KEY_X,KEY_Z,KEY_RESERVED,KEY_RESERVED,KEY_ESC,KEY_6,KEY_F10,KEY_3,KEY_F11,KEY_KPDOT,KEY_F9,KEY_TAB,KEY_F12,
 KEY_H,KEY_G,KEY_F,KEY_D,KEY_S,KEY_A,KEY_RESERVED,KEY_CAPSLOCK,KEY_U,KEY_Y,KEY_T,KEY_R,KEY_E,KEY_W,KEY_Q,KEY_TAB,
 KEY_7,KEY_6,KEY_5,KEY_4,KEY_3,KEY_2,KEY_1,KEY_GRAVE,KEY_F13,KEY_F14,KEY_F15,KEY_F16,KEY_F17,KEY_F18,KEY_F19,KEY_F20,
 KEY_MENU,KEY_F4,KEY_F3,KEY_F2,KEY_F1,KEY_VOLUMEUP,KEY_STOP,KEY_SENDFILE,KEY_SYSRQ,KEY_F5,KEY_F6,KEY_F7,KEY_F8,KEY_VOLUMEDOWN,KEY_DEL_EOL,KEY_DEL_EOS,
 KEY_8,KEY_9,KEY_0,KEY_MINUS,KEY_EQUAL,KEY_BACKSPACE,KEY_INS_LINE,KEY_DEL_LINE,KEY_I,KEY_O,KEY_P,KEY_LEFTBRACE,KEY_RIGHTBRACE,KEY_BACKSLASH,KEY_INSERT,KEY_DELETE,
 KEY_J,KEY_K,KEY_L,KEY_SEMICOLON,KEY_APOSTROPHE,KEY_ENTER,KEY_HOME,KEY_PAGEUP,KEY_M,KEY_COMMA,KEY_DOT,KEY_SLASH,KEY_BACKSLASH,KEY_SELECT,KEY_102ND,KEY_PAGEDOWN,
 KEY_N,KEY_SPACE,KEY_NEXT,KEY_RESERVED,KEY_LEFT,KEY_DOWN,KEY_UP,KEY_RIGHT];
pub const HIL_KEYCODES_SET3: [u32; 128] = [
 KEY_RESERVED,KEY_ESC,KEY_1,KEY_2,KEY_3,KEY_4,KEY_5,KEY_6,KEY_7,KEY_8,KEY_9,KEY_0,KEY_MINUS,KEY_EQUAL,KEY_BACKSPACE,KEY_TAB,
 KEY_Q,KEY_W,KEY_E,KEY_R,KEY_T,KEY_Y,KEY_U,KEY_I,KEY_O,KEY_P,KEY_LEFTBRACE,KEY_RIGHTBRACE,KEY_ENTER,KEY_LEFTCTRL,KEY_A,KEY_S,
 KEY_D,KEY_F,KEY_G,KEY_H,KEY_J,KEY_K,KEY_L,KEY_SEMICOLON,KEY_APOSTROPHE,KEY_GRAVE,KEY_LEFTSHIFT,KEY_BACKSLASH,KEY_Z,KEY_X,KEY_C,KEY_V,
 KEY_B,KEY_N,KEY_M,KEY_COMMA,KEY_DOT,KEY_SLASH,KEY_RIGHTSHIFT,KEY_KPASTERISK,KEY_LEFTALT,KEY_SPACE,KEY_CAPSLOCK,KEY_F1,KEY_F2,KEY_F3,
 KEY_F4,KEY_F5,KEY_F6,KEY_F7,KEY_F8,KEY_F9,KEY_F10,KEY_NUMLOCK,KEY_SCROLLLOCK,KEY_KP7,KEY_KP8,KEY_KP9,KEY_KPMINUS,KEY_KP4,KEY_KP5,KEY_KP6,
 KEY_KPPLUS,KEY_KP1,KEY_KP2,KEY_KP3,KEY_KP0,KEY_KPDOT,KEY_SYSRQ,KEY_RESERVED,KEY_RESERVED,KEY_RESERVED,KEY_RESERVED,KEY_RESERVED,KEY_RESERVED,KEY_RESERVED,KEY_RESERVED,KEY_RESERVED,
 KEY_RESERVED,KEY_RESERVED,KEY_RESERVED,KEY_RESERVED,KEY_UP,KEY_LEFT,KEY_DOWN,KEY_RIGHT,KEY_HOME,KEY_PAGEUP,KEY_END,KEY_PAGEDOWN,KEY_INSERT,KEY_DELETE,KEY_102ND,KEY_RESERVED,
 KEY_RESERVED,KEY_RESERVED,KEY_RESERVED,KEY_RESERVED,KEY_F1,KEY_F2,KEY_F3,KEY_F4,KEY_F5,KEY_F6,KEY_F7,KEY_F8,KEY_RESERVED,KEY_RESERVED,KEY_RESERVED,KEY_RESERVED];

pub const HIL_POL_NUM_AXES_MASK: u8 = 0x03; pub const HIL_POL_CTS: u8 = 0x04;
pub const HIL_POL_STATUS_PENDING: u8 = 0x08; pub const HIL_POL_CHARTYPE_MASK: u8 = 0x70;
pub const HIL_POL_CHARTYPE_NONE: u8 = 0x00; pub const HIL_POL_CHARTYPE_RSVD1: u8 = 0x10;
pub const HIL_POL_CHARTYPE_ASCII: u8 = 0x20; pub const HIL_POL_CHARTYPE_BINARY: u8 = 0x30;
pub const HIL_POL_CHARTYPE_SET1: u8 = 0x40; pub const HIL_POL_CHARTYPE_RSVD2: u8 = 0x50;
pub const HIL_POL_CHARTYPE_SET2: u8 = 0x60; pub const HIL_POL_CHARTYPE_SET3: u8 = 0x70;
pub const HIL_POL_AXIS_ALT: u8 = 0x80;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
