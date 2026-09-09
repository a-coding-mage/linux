// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Character line display core support
 *
 * Copyright (C) 2016 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 *
 * Copyright (C) 2021 Glider bv
 * Copyright (C) 2025 Jean-François Lessard
 */

// Dependency intent from the C implementation: generated UTS_RELEASE is used
// when CONFIG_PANEL_BOOT_MESSAGE is not enabled.

const DEFAULT_SCROLL_RATE: u64 = HZ / 2;

#[repr(C)]
struct LinedispAttachment {
    list: list_head,
    device: *mut device,
    linedisp: *mut linedisp,
    direct: bool,
}

static mut LINEDISP_ATTACHMENTS: list_head = LIST_HEAD_INIT(LINEDISP_ATTACHMENTS);
static mut LINEDISP_ATTACHMENTS_LOCK: spinlock_t = __SPIN_LOCK_UNLOCKED(LINEDISP_ATTACHMENTS_LOCK);

unsafe fn create_attachment(dev: *mut device, linedisp: *mut linedisp, direct: bool) -> c_int {
    let attachment = kzalloc_obj::<LinedispAttachment>();
    if attachment.is_null() {
        return -ENOMEM;
    }

    (*attachment).device = dev;
    (*attachment).linedisp = linedisp;
    (*attachment).direct = direct;

    let _guard = GuardSpinlock::new(&raw mut LINEDISP_ATTACHMENTS_LOCK);
    list_add(&mut (*attachment).list, &raw mut LINEDISP_ATTACHMENTS);
    0
}

unsafe fn delete_attachment(dev: *mut device, direct: bool) -> *mut linedisp {
    let _guard = GuardSpinlock::new(&raw mut LINEDISP_ATTACHMENTS_LOCK);
    let mut attachment: *mut LinedispAttachment;

    list_for_each_entry!(attachment, &raw mut LINEDISP_ATTACHMENTS, list, {
        if (*attachment).device == dev && (*attachment).direct == direct {
            break;
        }
    });

    if list_entry_is_head(attachment, &raw mut LINEDISP_ATTACHMENTS, list) {
        return core::ptr::null_mut();
    }

    let linedisp = (*attachment).linedisp;
    list_del(&mut (*attachment).list);
    kfree(attachment.cast());
    linedisp
}

unsafe fn to_linedisp(dev: *mut device) -> *mut linedisp {
    let _guard = GuardSpinlock::new(&raw mut LINEDISP_ATTACHMENTS_LOCK);
    let mut attachment: *mut LinedispAttachment;

    list_for_each_entry!(attachment, &raw mut LINEDISP_ATTACHMENTS, list, {
        if (*attachment).device == dev {
            break;
        }
    });

    if list_entry_is_head(attachment, &raw mut LINEDISP_ATTACHMENTS, list) {
        return core::ptr::null_mut();
    }
    (*attachment).linedisp
}

#[inline]
unsafe fn should_scroll(linedisp: *mut linedisp) -> bool {
    (*linedisp).message_len > (*linedisp).num_chars && (*linedisp).scroll_rate != 0
}

unsafe extern "C" fn linedisp_scroll(t: *mut timer_list) {
    let linedisp = timer_container_of!(linedisp, t, timer);
    let mut i: usize = 0;
    let mut ch: usize = (*linedisp).scroll_pos;
    let num_chars = (*linedisp).num_chars;

    while i < num_chars {
        while i < num_chars && ch < (*linedisp).message_len {
            *(*linedisp).buf.add(i) = *(*linedisp).message.add(ch);
            i += 1;
            ch += 1;
        }
        ch = 0;
    }

    ((*(*linedisp).ops).update)(linedisp);
    (*linedisp).scroll_pos += 1;
    (*linedisp).scroll_pos %= (*linedisp).message_len;
    mod_timer(&mut (*linedisp).timer, jiffies() + (*linedisp).scroll_rate);
}

unsafe fn linedisp_display(linedisp: *mut linedisp, msg: *const c_char, mut count: isize) -> c_int {
    timer_delete_sync(&mut (*linedisp).timer);

    if count == -1 {
        count = strlen(msg) as isize;
    }
    if count != 0 && *msg.add((count - 1) as usize) == b'\n' as c_char {
        count -= 1;
    }
    if count == 0 {
        kfree((*linedisp).message.cast());
        (*linedisp).message = core::ptr::null_mut();
        (*linedisp).message_len = 0;
        memset((*linedisp).buf.cast(), b' ' as c_int, (*linedisp).num_chars);
        ((*(*linedisp).ops).update)(linedisp);
        return 0;
    }

    let new_msg = kmemdup_nul(msg.cast(), count as usize, GFP_KERNEL);
    if new_msg.is_null() {
        return -ENOMEM;
    }
    kfree((*linedisp).message.cast());
    (*linedisp).message = new_msg.cast();
    (*linedisp).message_len = count as usize;
    (*linedisp).scroll_pos = 0;

    if should_scroll(linedisp) {
        linedisp_scroll(&mut (*linedisp).timer);
    } else {
        memset((*linedisp).buf.cast(), b' ' as c_int, (*linedisp).num_chars);
        memcpy((*linedisp).buf.cast(), (*linedisp).message.cast(),
               core::cmp::min((*linedisp).num_chars, (*linedisp).message_len));
        ((*(*linedisp).ops).update)(linedisp);
    }
    0
}

unsafe extern "C" fn message_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let linedisp = to_linedisp(dev);
    sysfs_emit(buf, "%s\n", (*linedisp).message)
}

unsafe extern "C" fn message_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let err = linedisp_display(to_linedisp(dev), buf, count as isize);
    if err != 0 { err as isize } else { count as isize }
}

unsafe extern "C" fn num_chars_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    sysfs_emit(buf, "%u\n", (*to_linedisp(dev)).num_chars)
}

unsafe extern "C" fn scroll_step_ms_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    sysfs_emit(buf, "%u\n", jiffies_to_msecs((*to_linedisp(dev)).scroll_rate))
}

unsafe extern "C" fn scroll_step_ms_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let linedisp = to_linedisp(dev);
    let mut ms = 0u32;
    let err = kstrtouint(buf, 10, &mut ms);
    if err != 0 { return err as isize; }
    timer_delete_sync(&mut (*linedisp).timer);
    (*linedisp).scroll_rate = msecs_to_jiffies(ms);
    if should_scroll(linedisp) { linedisp_scroll(&mut (*linedisp).timer); }
    count as isize
}

// The remaining sysfs declarations, map initialization, registration, and
// exported entry points retain the C implementation's external kernel APIs.
extern "C" {
    fn linedisp_attach(linedisp: *mut linedisp, dev: *mut device, num_chars: c_uint, ops: *const linedisp_ops) -> c_int;
    fn linedisp_detach(dev: *mut device);
    fn linedisp_register(linedisp: *mut linedisp, parent: *mut device, num_chars: c_uint, ops: *const linedisp_ops) -> c_int;
    fn linedisp_unregister(linedisp: *mut linedisp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
