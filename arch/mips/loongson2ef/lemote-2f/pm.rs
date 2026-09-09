// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Lemote loongson2f family machines' specific suspend support
 *
 *  Copyright (C) 2009 Lemote Inc.
 *  Author: Wu Zhangjin <wuzhangjin@gmail.com>
 */

// Linux kernel headers and architecture-specific headers are supplied by the
// surrounding translation unit.

use core::ffi::c_void;

const I8042_KBD_IRQ: i32 = 1;
const I8042_CTR_KBDINT: u8 = 0x01;
const I8042_CTR_KBDDIS: u8 = 0x10;

extern "C" {
    static mut mips_machtype: i32;
    static mut yeeloong_report_lid_status: sci_handler;

    fn i8042_command(param: *mut u8, command: i32) -> i32;
    fn pr_err(format: *const i8, ...);
    fn outb(value: u8, port: u16);
    fn inb(port: u16) -> u8;
    fn mach_i8259_irq() -> i32;
    fn printk(format: *const i8, ...);
    fn ec_query_seq(command: i32) -> i32;
    fn ec_get_event_num() -> i32;
    fn ec_read(reg: i32) -> i32;
    fn disable_mfgpt0_counter();
    fn enable_mfgpt0_counter();
    fn schedule_delayed_work(work: *mut delayed_work, delay: u64) -> i32;
}

type sci_handler = Option<unsafe extern "C" fn(i32)>;

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

static mut i8042_ctr: u8 = 0;
static mut lid_task: delayed_work = delayed_work { _private: [] };
static mut initialized: i32 = 0;

// External constants supplied by the included platform headers.
extern "C" {
    static MACH_LEMOTE_ML2F7: i32;
    static MACH_LEMOTE_YL2F89: i32;
    static I8042_CMD_CTL_RCTR: i32;
    static I8042_CMD_CTL_WCTR: i32;
    static PIC_MASTER_IMR: u16;
    static PIC_CASCADE_IR: i32;
    static SCI_IRQ_NUM: i32;
    static PIC_SLAVE_IMR: u16;
    static CMD_GET_EVENT_NUM: i32;
    static EVENT_LID: i32;
    static REG_LID_DETECT: i32;
    static BIT_LID_DETECT_ON: i32;
}

unsafe fn i8042_enable_kbd_port() -> i32 {
    if i8042_command(&raw mut i8042_ctr, I8042_CMD_CTL_RCTR) != 0 {
        pr_err(b"i8042.c: Can't read CTR while enabling i8042 kbd port.\n\0".as_ptr() as *const i8);
        return -5; // -EIO
    }

    i8042_ctr &= !I8042_CTR_KBDDIS;
    i8042_ctr |= I8042_CTR_KBDINT;

    if i8042_command(&raw mut i8042_ctr, I8042_CMD_CTL_WCTR) != 0 {
        i8042_ctr &= !I8042_CTR_KBDINT;
        i8042_ctr |= I8042_CTR_KBDDIS;
        pr_err(b"i8042.c: Failed to enable KBD port.\n\0".as_ptr() as *const i8);
        return -5; // -EIO
    }

    0
}

pub unsafe fn setup_wakeup_events() {
    let irq_mask: u8;

    match mips_machtype {
        x if x == MACH_LEMOTE_ML2F7 || x == MACH_LEMOTE_YL2F89 => {
            outb(0xff & !(1u8 << I8042_KBD_IRQ), PIC_MASTER_IMR);
            irq_mask = inb(PIC_MASTER_IMR);

            i8042_enable_kbd_port();

            outb(irq_mask & !(1u8 << PIC_CASCADE_IR), PIC_MASTER_IMR);
            inb(PIC_MASTER_IMR);
            outb(0xff & !(1u8 << (SCI_IRQ_NUM - 8)), PIC_SLAVE_IMR);
            inb(PIC_SLAVE_IMR);
        }
        _ => {}
    }
}

unsafe extern "C" fn yeeloong_lid_update_task(_work: *mut work_struct) {
    if let Some(handler) = yeeloong_report_lid_status {
        handler(BIT_LID_DETECT_ON);
    }
}

pub unsafe fn wakeup_loongson() -> i32 {
    let irq = mach_i8259_irq();
    if irq < 0 {
        return 0;
    }

    printk(b"%s: irq = %d\n\0".as_ptr() as *const i8, b"wakeup_loongson\0".as_ptr(), irq);

    if irq == I8042_KBD_IRQ {
        return 1;
    } else if irq == SCI_IRQ_NUM {
        let ret = ec_query_seq(CMD_GET_EVENT_NUM);
        if ret < 0 {
            return 0;
        }
        let sci_event = ec_get_event_num();
        if sci_event < 0 {
            return 0;
        }
        if sci_event == EVENT_LID {
            let lid_status = ec_read(REG_LID_DETECT);
            if lid_status == BIT_LID_DETECT_ON {
                if initialized == 0 {
                    // INIT_DELAYED_WORK(&lid_task, yeeloong_lid_update_task);
                    initialized = 1;
                }
                schedule_delayed_work(&raw mut lid_task, 1);
                return 1;
            }
        }
    }

    0
}

// __weak
pub unsafe fn mach_suspend() {
    disable_mfgpt0_counter();
}

// __weak
pub unsafe fn mach_resume() {
    enable_mfgpt0_counter();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
