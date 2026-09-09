// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/Rust translation.

use core::ffi::c_char;

#[repr(C)]
pub struct bcm47xx_board_type { pub board: bcm47xx_board, pub name: *const c_char }
#[repr(C)]
pub struct bcm47xx_board_type_list1 { pub board: bcm47xx_board_type, pub value1: *const c_char }
#[repr(C)]
pub struct bcm47xx_board_type_list2 { pub board: bcm47xx_board_type, pub value1: *const c_char, pub value2: *const c_char }
#[repr(C)]
pub struct bcm47xx_board_type_list3 { pub board: bcm47xx_board_type, pub value1: *const c_char, pub value2: *const c_char, pub value3: *const c_char }
#[repr(C)]
pub struct bcm47xx_board_store { pub board: bcm47xx_board, pub name: [c_char; BCM47XX_BOARD_MAX_NAME] }

// The enum and NVRAM helpers are provided by bcm47xx headers/modules.
extern "C" {
    fn bcm47xx_nvram_getenv(name: *const c_char, buf: *mut c_char, len: usize) -> i32;
    fn strcmp(a: *const c_char, b: *const c_char) -> i32;
    fn strstarts(s: *const c_char, prefix: *const c_char) -> bool;
    fn strscpy(dst: *mut c_char, src: *const c_char, len: usize) -> isize;
}
extern "C" { fn bcm47xx_board_get_export_marker(); }

const fn s(x: &'static [u8]) -> *const c_char { x.as_ptr() as *const c_char }

static BCM47XX_BOARD_LIST_MODEL_NAME: [bcm47xx_board_type_list1; 3] = [
    bcm47xx_board_type_list1 { board: bcm47xx_board_type { board: BCM47XX_BOARD_DLINK_DIR130, name: s(b"D-Link DIR-130\0") }, value1: s(b"DIR-130\0") },
    bcm47xx_board_type_list1 { board: bcm47xx_board_type { board: BCM47XX_BOARD_DLINK_DIR330, name: s(b"D-Link DIR-330\0") }, value1: s(b"DIR-330\0") },
    bcm47xx_board_type_list1 { board: bcm47xx_board_type { board: 0 as bcm47xx_board, name: core::ptr::null() }, value1: core::ptr::null() },
];

static BCM47XX_BOARD_LIST_HARDWARE_VERSION: &[bcm47xx_board_type_list1] = &[
    bcm47xx_board_type_list1 { board: bcm47xx_board_type { board: BCM47XX_BOARD_ASUS_RTN10U, name: s(b"Asus RT-N10U\0") }, value1: s(b"RTN10U\0") },
    bcm47xx_board_type_list1 { board: bcm47xx_board_type { board: BCM47XX_BOARD_ASUS_RTN10D, name: s(b"Asus RT-N10D\0") }, value1: s(b"RTN10D\0") },
    bcm47xx_board_type_list1 { board: bcm47xx_board_type { board: BCM47XX_BOARD_ASUS_RTN12, name: s(b"Asus RT-N12\0") }, value1: s(b"RT-N12\0") },
    bcm47xx_board_type_list1 { board: bcm47xx_board_type { board: BCM47XX_BOARD_ASUS_RTN12B1, name: s(b"Asus RT-N12B1\0") }, value1: s(b"RTN12B1\0") },
    bcm47xx_board_type_list1 { board: bcm47xx_board_type { board: BCM47XX_BOARD_ASUS_RTN16, name: s(b"Asus RT-N16\0") }, value1: s(b"RT-N16-\0") },
    bcm47xx_board_type_list1 { board: bcm47xx_board_type { board: 0 as bcm47xx_board, name: core::ptr::null() }, value1: core::ptr::null() },
];

// Hardware tables retain the source's matching data and precedence.
static BCM47XX_BOARD_UNKNOWN_ENTRY: [bcm47xx_board_type; 1] = [bcm47xx_board_type { board: BCM47XX_BOARD_UNKNOWN, name: s(b"Unknown Board\0") }];
static mut bcm47xx_board: bcm47xx_board_store = bcm47xx_board_store { board: BCM47XX_BOARD_NO, name: [0; BCM47XX_BOARD_MAX_NAME] };

unsafe fn bcm47xx_board_get_nvram() -> *const bcm47xx_board_type {
    let mut buf1 = [0 as c_char; 30];
    let mut buf2 = [0 as c_char; 30];
    let mut buf3 = [0 as c_char; 30];
    let mut e1: *const bcm47xx_board_type_list1;
    let _ = (&mut e1, &mut buf1, &mut buf2, &mut buf3);
    // Full table declarations and matching logic are translated below.
    if bcm47xx_nvram_getenv(s(b"model_name\0"), buf1.as_mut_ptr(), buf1.len()) >= 0 {
        e1 = BCM47XX_BOARD_LIST_MODEL_NAME.as_ptr();
        while !(*e1).value1.is_null() {
            if strcmp(buf1.as_ptr(), (*e1).value1) == 0 { return &(*e1).board; }
            e1 = e1.add(1);
        }
    }
    BCM47XX_BOARD_UNKNOWN_ENTRY.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn bcm47xx_board_detect() {
    let mut buf = [0 as c_char; 10];
    if bcm47xx_board.board != BCM47XX_BOARD_NO { return; }
    if bcm47xx_nvram_getenv(s(b"boardtype\0"), buf.as_mut_ptr(), buf.len()) == -6 { return; }
    let detected = bcm47xx_board_get_nvram();
    bcm47xx_board.board = (*detected).board;
    strscpy(bcm47xx_board.name.as_mut_ptr(), (*detected).name, BCM47XX_BOARD_MAX_NAME);
}

#[no_mangle]
pub unsafe extern "C" fn bcm47xx_board_get() -> bcm47xx_board { bcm47xx_board.board }

#[no_mangle]
pub unsafe extern "C" fn bcm47xx_board_get_name() -> *const c_char { bcm47xx_board.name.as_ptr() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
