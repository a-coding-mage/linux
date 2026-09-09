// SPDX-License-Identifier: GPL-2.0
/*
 * Alchemy DB/PB1xxx board support.
 */

use core::ffi::c_char;

// Dependencies supplied by the surrounding kernel sources.
extern "C" {
    fn bcsr_read(reg: i32) -> i32;
    fn alchemy_get_cputype() -> i32;
    fn db1000_board_setup() -> i32;
    fn db1550_board_setup() -> i32;
    fn db1200_board_setup() -> i32;
    fn db1300_board_setup() -> i32;
    fn db1550_pci_setup(rev: i32) -> i32;
    fn db1500_pci_setup() -> i32;
    fn mips_set_machine_name(name: *const c_char);
    fn db1000_dev_setup() -> i32;
    fn db1200_dev_setup() -> i32;
    fn db1300_dev_setup() -> i32;
    fn db1550_dev_setup() -> i32;
    fn panic(message: *const c_char) -> !;
}

// Constants and board/CPU decoding macros are provided by the included kernel headers.
extern "C" {
    static ENODEV: i32;
}

unsafe fn board_type_str() -> *const c_char {
    match BCSR_WHOAMI_BOARD(bcsr_read(BCSR_WHOAMI)) {
        BCSR_WHOAMI_DB1000 => b"DB1000\0".as_ptr() as *const c_char,
        BCSR_WHOAMI_DB1500 => b"DB1500\0".as_ptr() as *const c_char,
        BCSR_WHOAMI_DB1100 => b"DB1100\0".as_ptr() as *const c_char,
        BCSR_WHOAMI_PB1500 | BCSR_WHOAMI_PB1500R2 => b"PB1500\0".as_ptr() as *const c_char,
        BCSR_WHOAMI_PB1100 => b"PB1100\0".as_ptr() as *const c_char,
        BCSR_WHOAMI_PB1200_DDR1 | BCSR_WHOAMI_PB1200_DDR2 => b"PB1200\0".as_ptr() as *const c_char,
        BCSR_WHOAMI_DB1200 => b"DB1200\0".as_ptr() as *const c_char,
        BCSR_WHOAMI_DB1300 => b"DB1300\0".as_ptr() as *const c_char,
        BCSR_WHOAMI_DB1550 => b"DB1550\0".as_ptr() as *const c_char,
        BCSR_WHOAMI_PB1550_SDR | BCSR_WHOAMI_PB1550_DDR => b"PB1550\0".as_ptr() as *const c_char,
        _ => b"(unknown)\0".as_ptr() as *const c_char,
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_system_type() -> *const c_char {
    board_type_str()
}

#[no_mangle]
pub unsafe extern "C" fn board_setup() {
    let ret: i32;

    ret = match alchemy_get_cputype() {
        ALCHEMY_CPU_AU1000 | ALCHEMY_CPU_AU1500 | ALCHEMY_CPU_AU1100 => db1000_board_setup(),
        ALCHEMY_CPU_AU1550 => db1550_board_setup(),
        ALCHEMY_CPU_AU1200 => db1200_board_setup(),
        ALCHEMY_CPU_AU1300 => db1300_board_setup(),
        _ => {
            pr_err(b"unsupported CPU on board\n\0".as_ptr() as *const c_char);
            -ENODEV
        }
    };
    if ret != 0 {
        panic(b"cannot initialize board support\0".as_ptr() as *const c_char);
    }
}

unsafe fn db1xxx_arch_init() -> i32 {
    let id = BCSR_WHOAMI_BOARD(bcsr_read(BCSR_WHOAMI));
    if id == BCSR_WHOAMI_DB1550 {
        return db1550_pci_setup(0);
    } else if id == BCSR_WHOAMI_PB1550_SDR || id == BCSR_WHOAMI_PB1550_DDR {
        return db1550_pci_setup(1);
    } else if id == BCSR_WHOAMI_DB1500 || id == BCSR_WHOAMI_PB1500 || id == BCSR_WHOAMI_PB1500R2 {
        return db1500_pci_setup();
    }
    0
}

// arch_initcall(db1xxx_arch_init);
unsafe fn db1xxx_dev_init() -> i32 {
    mips_set_machine_name(board_type_str());
    match BCSR_WHOAMI_BOARD(bcsr_read(BCSR_WHOAMI)) {
        BCSR_WHOAMI_DB1000 | BCSR_WHOAMI_DB1500 | BCSR_WHOAMI_DB1100 |
        BCSR_WHOAMI_PB1500 | BCSR_WHOAMI_PB1500R2 | BCSR_WHOAMI_PB1100 => db1000_dev_setup(),
        BCSR_WHOAMI_PB1200_DDR1 | BCSR_WHOAMI_PB1200_DDR2 | BCSR_WHOAMI_DB1200 => db1200_dev_setup(),
        BCSR_WHOAMI_DB1300 => db1300_dev_setup(),
        BCSR_WHOAMI_DB1550 | BCSR_WHOAMI_PB1550_SDR | BCSR_WHOAMI_PB1550_DDR => db1550_dev_setup(),
        _ => 0,
    }
}

// device_initcall(db1xxx_dev_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
