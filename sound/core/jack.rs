// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Jack abstraction layer
 *
 *  Copyright 2008 Wolfson Microelectronics
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type loff_t = i64;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const SNDRV_DEV_JACK: c_int = 0;
const EV_KEY: c_uint = 0x01;
const EV_SW: c_uint = 0x05;
const BTN_0: c_uint = 0x100;
const SW_HEADPHONE_INSERT: c_uint = 0x02;
const SW_MICROPHONE_INSERT: c_uint = 0x04;
const SW_LINEOUT_INSERT: c_uint = 0x06;
const SW_JACK_PHYSICAL_INSERT: c_uint = 0x07;
const SW_VIDEOOUT_INSERT: c_uint = 0x08;
const SW_LINEIN_INSERT: c_uint = 0x0d;
const SW_USB_INSERT: c_uint = 0x0e;
const SND_JACK_SWITCH_TYPES: usize = 7;
const SND_JACK_BTN_0: c_int = 0x4000;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct input_dev {
    pub name: *const c_char,
    pub phys: *const c_char,
    pub dev: device,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: [c_char; 44],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_ctl_elem_id,
    pub private_value: c_ulong,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_kcontrol)>,
}

#[repr(C)]
pub struct snd_card {
    pub shortname: [c_char; 32],
    pub debugfs_root: *mut dentry,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
    pub card: *mut snd_card,
    pub id: *mut c_char,
    pub name: [c_char; 100],
    pub input_dev: *mut input_dev,
    pub input_dev_lock: mutex,
    pub registered: c_int,
    pub type_: c_int,
    pub key: [c_int; 6],
    pub kctl_list: list_head,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_jack)>,
    pub hw_status_cache: c_int,
}

#[repr(C)]
pub struct snd_device {
    pub card: *mut snd_card,
    pub device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    /* CONFIG_SND_JACK_INPUT_DEV: dev_register is present when input devices are enabled. */
    pub dev_register: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_disconnect: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut c_void, *mut file) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
}

#[repr(C)]
pub struct snd_jack_kctl {
    pub kctl: *mut snd_kcontrol,
    pub list: list_head, /* list of controls belong to the same jack */
    pub mask_bits: c_uint, /* only masked status bits are reported via kctl */
    pub jack: *mut snd_jack, /* pointer to struct snd_jack */
    pub sw_inject_enable: bool_, /* allow to inject plug event via debugfs */
    /* CONFIG_SND_JACK_INJECTION_DEBUG */
    pub jack_debugfs_root: *mut dentry, /* jack_kctl debugfs root */
}

/* CONFIG_SND_JACK_INPUT_DEV */
static jack_switch_types: [c_int; SND_JACK_SWITCH_TYPES] = [
    SW_HEADPHONE_INSERT as c_int,
    SW_MICROPHONE_INSERT as c_int,
    SW_LINEOUT_INSERT as c_int,
    SW_JACK_PHYSICAL_INSERT as c_int,
    SW_VIDEOOUT_INSERT as c_int,
    SW_LINEIN_INSERT as c_int,
    SW_USB_INSERT as c_int,
];

unsafe extern "C" {
    static simple_open: Option<unsafe extern "C" fn(*mut c_void, *mut file) -> c_int>;
    static default_llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>;

    fn input_unregister_device(dev: *mut input_dev);
    fn input_free_device(dev: *mut input_dev);
    fn input_allocate_device() -> *mut input_dev;
    fn input_register_device(dev: *mut input_dev) -> c_int;
    fn input_set_capability(dev: *mut input_dev, ty: c_uint, code: c_int);
    fn input_report_key(dev: *mut input_dev, code: c_int, value: c_int);
    fn input_report_switch(dev: *mut input_dev, code: c_int, value: c_int);
    fn input_sync(dev: *mut input_dev);
    fn input_get_device(dev: *mut input_dev) -> *mut input_dev;
    fn input_put_device(dev: *mut input_dev);

    fn snd_card_get_device_link(card: *mut snd_card) -> *mut device;
    fn snd_ctl_remove(card: *mut snd_card, kctl: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kctl: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_free_one(kctl: *mut snd_kcontrol);
    fn snd_kctl_jack_new(name: *const c_char, card: *mut snd_card) -> *mut snd_kcontrol;
    fn snd_kctl_jack_report(card: *mut snd_card, kctl: *mut snd_kcontrol, status: c_int);
    fn snd_device_new(card: *mut snd_card, ty: c_int, data: *mut c_void, ops: *const snd_device_ops) -> c_int;

    fn kfree(ptr: *mut c_void);
    fn kstrdup(s: *const c_char, flags: c_uint) -> *mut c_char;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn simple_read_from_buffer(to: *mut c_char, count: size_t, ppos: *mut loff_t, from: *const c_char, available: size_t) -> ssize_t;
    fn simple_write_to_buffer(to: *mut c_char, available: size_t, ppos: *mut loff_t, from: *const c_char, count: size_t) -> ssize_t;
    fn kstrtoul(s: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn isalnum(c: c_int) -> c_int;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn debugfs_remove(dentry: *mut dentry);
    fn fls(x: c_int) -> c_int;
    fn WARN_ON(condition: c_int) -> c_int;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_del_init(entry: *mut list_head);
}

unsafe fn snd_jack_remove_debugfs(jack: *mut snd_jack) {
    let mut pos = (*jack).kctl_list.next;
    while pos != &mut (*jack).kctl_list {
        let jack_kctl = container_of_snd_jack_kctl(pos);
        debugfs_remove((*jack_kctl).jack_debugfs_root);
        (*jack_kctl).jack_debugfs_root = ptr::null_mut();
        pos = (*pos).next;
    }
}

unsafe extern "C" fn snd_jack_dev_disconnect(device: *mut snd_device) -> c_int {
    let jack = (*device).device_data as *mut snd_jack;

    snd_jack_remove_debugfs(jack);

    /* CONFIG_SND_JACK_INPUT_DEV: guard(mutex)(&jack->input_dev_lock); */
    if (*jack).input_dev.is_null() {
        return 0;
    }

    /*
     * If the input device is registered with the input subsystem
     * then we need to use a different deallocator.
     */
    if (*jack).registered != 0 {
        input_unregister_device((*jack).input_dev);
    } else {
        input_free_device((*jack).input_dev);
    }
    (*jack).input_dev = ptr::null_mut();

    0
}

unsafe extern "C" fn snd_jack_dev_free(device: *mut snd_device) -> c_int {
    let jack = (*device).device_data as *mut snd_jack;
    let card = (*device).card;
    let mut pos = (*jack).kctl_list.next;

    while pos != &mut (*jack).kctl_list {
        let next = (*pos).next;
        let jack_kctl = container_of_snd_jack_kctl(pos);
        list_del_init(&mut (*jack_kctl).list);
        snd_ctl_remove(card, (*jack_kctl).kctl);
        pos = next;
    }

    if let Some(private_free) = (*jack).private_free {
        private_free(jack);
    }

    snd_jack_dev_disconnect(device);

    kfree((*jack).id as *mut c_void);
    kfree(jack as *mut c_void);

    0
}

/* CONFIG_SND_JACK_INPUT_DEV */
unsafe extern "C" fn snd_jack_dev_register(device: *mut snd_device) -> c_int {
    let jack = (*device).device_data as *mut snd_jack;
    let card = (*device).card;
    let mut err: c_int;

    snprintf((*jack).name.as_mut_ptr(), (*jack).name.len(), c"%s %s".as_ptr(), (*card).shortname.as_ptr(), (*jack).id);

    /* guard(mutex)(&jack->input_dev_lock); */
    if (*jack).input_dev.is_null() {
        return 0;
    }

    (*(*jack).input_dev).name = (*jack).name.as_ptr();

    /* Default to the sound card device. */
    if (*(*jack).input_dev).dev.parent.is_null() {
        (*(*jack).input_dev).dev.parent = snd_card_get_device_link(card);
    }

    /* Add capabilities for any keys that are enabled */
    for i in 0..(*jack).key.len() {
        let testbit = SND_JACK_BTN_0 >> i;

        if ((*jack).type_ & testbit) == 0 {
            continue;
        }

        if (*jack).key[i] == 0 {
            (*jack).key[i] = BTN_0 as c_int + i as c_int;
        }

        input_set_capability((*jack).input_dev, EV_KEY, (*jack).key[i]);
    }

    err = input_register_device((*jack).input_dev);
    if err == 0 {
        (*jack).registered = 1;
    }

    err
}

unsafe fn snd_jack_inject_report(jack_kctl: *mut snd_jack_kctl, status: c_int) {
    if jack_kctl.is_null() {
        return;
    }

    let jack = (*jack_kctl).jack;

    if (*jack_kctl).sw_inject_enable {
        snd_kctl_jack_report((*jack).card, (*jack_kctl).kctl, status & (*jack_kctl).mask_bits as c_int);
    }

    /* CONFIG_SND_JACK_INPUT_DEV */
    if (*jack).input_dev.is_null() {
        return;
    }

    for i in 0..(*jack).key.len() {
        let testbit = (SND_JACK_BTN_0 >> i) & (*jack_kctl).mask_bits as c_int;

        if ((*jack).type_ & testbit) != 0 {
            input_report_key((*jack).input_dev, (*jack).key[i], status & testbit);
        }
    }

    for i in 0..jack_switch_types.len() {
        let testbit = (1 << i) & (*jack_kctl).mask_bits as c_int;

        if ((*jack).type_ & testbit) != 0 {
            input_report_switch((*jack).input_dev, jack_switch_types[i], status & testbit);
        }
    }

    input_sync((*jack).input_dev);
}

unsafe extern "C" fn sw_inject_enable_read(file: *mut file, to: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let jack_kctl = (*file).private_data as *mut snd_jack_kctl;
    let mut buf = [0 as c_char; 128];

    let len = scnprintf(
        buf.as_mut_ptr(),
        buf.len(),
        c"%s: %s\t\t%s: %i\n".as_ptr(),
        c"Jack".as_ptr(),
        (*(*jack_kctl).kctl).id.name.as_ptr(),
        c"Inject Enabled".as_ptr(),
        (*jack_kctl).sw_inject_enable as c_int,
    );
    simple_read_from_buffer(to, count, ppos, buf.as_ptr(), len as size_t)
}

unsafe extern "C" fn sw_inject_enable_write(file: *mut file, from: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let jack_kctl = (*file).private_data as *mut snd_jack_kctl;
    let mut enable: c_ulong = 0;
    let mut buf = [0 as c_char; 8];

    let ret = simple_write_to_buffer(buf.as_mut_ptr(), buf.len() - 1, ppos, from, count);
    let err = kstrtoul(buf.as_ptr(), 0, &mut enable);
    if err != 0 {
        return err as ssize_t;
    }

    if (*jack_kctl).sw_inject_enable == (enable != 0) {
        return ret;
    }

    (*jack_kctl).sw_inject_enable = enable != 0;

    if !(*jack_kctl).sw_inject_enable {
        snd_jack_report((*jack_kctl).jack, (*(*jack_kctl).jack).hw_status_cache);
    }

    ret
}

unsafe extern "C" fn jackin_inject_write(file: *mut file, from: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let jack_kctl = (*file).private_data as *mut snd_jack_kctl;
    let mut enable: c_ulong = 0;
    let mut buf = [0 as c_char; 8];

    if !(*jack_kctl).sw_inject_enable {
        return -EINVAL as ssize_t;
    }

    let ret = simple_write_to_buffer(buf.as_mut_ptr(), buf.len() - 1, ppos, from, count);
    let err = kstrtoul(buf.as_ptr(), 0, &mut enable);
    if err != 0 {
        return err as ssize_t;
    }

    snd_jack_inject_report(jack_kctl, if enable != 0 { (*jack_kctl).mask_bits as c_int } else { 0 });

    ret
}

unsafe extern "C" fn jack_kctl_id_read(file: *mut file, to: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let jack_kctl = (*file).private_data as *mut snd_jack_kctl;
    let mut buf = [0 as c_char; 64];

    let len = scnprintf(buf.as_mut_ptr(), buf.len(), c"%s\n".as_ptr(), (*(*jack_kctl).kctl).id.name.as_ptr());
    simple_read_from_buffer(to, count, ppos, buf.as_ptr(), len as size_t)
}

/* the bit definition is aligned with snd_jack_types in jack.h */
static jack_events_name: [*const c_char; 16] = [
    c"HEADPHONE(0x0001)".as_ptr(), c"MICROPHONE(0x0002)".as_ptr(), c"LINEOUT(0x0004)".as_ptr(),
    c"MECHANICAL(0x0008)".as_ptr(), c"VIDEOOUT(0x0010)".as_ptr(), c"LINEIN(0x0020)".as_ptr(),
    c"USB(0x0040)".as_ptr(), c"".as_ptr(), c"".as_ptr(), c"BTN_5(0x0200)".as_ptr(), c"BTN_4(0x0400)".as_ptr(),
    c"BTN_3(0x0800)".as_ptr(), c"BTN_2(0x1000)".as_ptr(), c"BTN_1(0x2000)".as_ptr(), c"BTN_0(0x4000)".as_ptr(),
    c"".as_ptr(),
];

/* the recommended buffer size is 256 */
unsafe fn parse_mask_bits(mask_bits: c_uint, buf: *mut c_char, buf_size: size_t) -> c_int {
    let mut len = scnprintf(buf, buf_size, c"0x%04x".as_ptr(), mask_bits);

    for i in 0..jack_events_name.len() {
        if (mask_bits & (1_u32 << i)) != 0 {
            len += scnprintf(buf.add(len as usize), buf_size - len as usize, c" %s".as_ptr(), jack_events_name[i]);
        }
    }
    len += scnprintf(buf.add(len as usize), buf_size - len as usize, c"\n".as_ptr());

    len
}

unsafe extern "C" fn jack_kctl_mask_bits_read(file: *mut file, to: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let jack_kctl = (*file).private_data as *mut snd_jack_kctl;
    let mut buf = [0 as c_char; 256];

    let len = parse_mask_bits((*jack_kctl).mask_bits, buf.as_mut_ptr(), buf.len());
    simple_read_from_buffer(to, count, ppos, buf.as_ptr(), len as size_t)
}

unsafe extern "C" fn jack_kctl_status_read(file: *mut file, to: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let jack_kctl = (*file).private_data as *mut snd_jack_kctl;
    let mut buf = [0 as c_char; 16];

    let len = scnprintf(
        buf.as_mut_ptr(),
        buf.len(),
        c"%s\n".as_ptr(),
        if (*(*jack_kctl).kctl).private_value != 0 { c"Plugged".as_ptr() } else { c"Unplugged".as_ptr() },
    );
    simple_read_from_buffer(to, count, ppos, buf.as_ptr(), len as size_t)
}

/* CONFIG_SND_JACK_INPUT_DEV */
unsafe extern "C" fn jack_type_read(file: *mut file, to: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let jack_kctl = (*file).private_data as *mut snd_jack_kctl;
    let mut buf = [0 as c_char; 256];

    let len = parse_mask_bits((*(*jack_kctl).jack).type_ as c_uint, buf.as_mut_ptr(), buf.len());
    simple_read_from_buffer(to, count, ppos, buf.as_ptr(), len as size_t)
}

static jack_type_fops: file_operations = file_operations {
    open: unsafe { simple_open },
    read: Some(jack_type_read),
    write: None,
    llseek: unsafe { default_llseek },
};

static sw_inject_enable_fops: file_operations = file_operations {
    open: unsafe { simple_open },
    read: Some(sw_inject_enable_read),
    write: Some(sw_inject_enable_write),
    llseek: unsafe { default_llseek },
};

static jackin_inject_fops: file_operations = file_operations {
    open: unsafe { simple_open },
    read: None,
    write: Some(jackin_inject_write),
    llseek: unsafe { default_llseek },
};

static jack_kctl_id_fops: file_operations = file_operations {
    open: unsafe { simple_open },
    read: Some(jack_kctl_id_read),
    write: None,
    llseek: unsafe { default_llseek },
};

static jack_kctl_mask_bits_fops: file_operations = file_operations {
    open: unsafe { simple_open },
    read: Some(jack_kctl_mask_bits_read),
    write: None,
    llseek: unsafe { default_llseek },
};

static jack_kctl_status_fops: file_operations = file_operations {
    open: unsafe { simple_open },
    read: Some(jack_kctl_status_read),
    write: None,
    llseek: unsafe { default_llseek },
};

unsafe fn snd_jack_debugfs_add_inject_node(jack: *mut snd_jack, jack_kctl: *mut snd_jack_kctl) -> c_int {
    let mut i: c_int;

    /* Don't create injection interface for Phantom jacks */
    if !strstr((*(*jack_kctl).kctl).id.name.as_ptr(), c"Phantom".as_ptr()).is_null() {
        return 0;
    }

    let tname = kstrdup((*(*jack_kctl).kctl).id.name.as_ptr(), GFP_KERNEL);
    if tname.is_null() {
        return -ENOMEM;
    }

    /* replace the chars which are not suitable for folder's name with _ */
    i = 0;
    while *tname.add(i as usize) != 0 {
        if isalnum(*tname.add(i as usize) as c_int) == 0 {
            *tname.add(i as usize) = b'_' as c_char;
        }
        i += 1;
    }

    (*jack_kctl).jack_debugfs_root = debugfs_create_dir(tname, (*(*jack).card).debugfs_root);
    kfree(tname as *mut c_void);

    debugfs_create_file(c"sw_inject_enable".as_ptr(), 0o644, (*jack_kctl).jack_debugfs_root, jack_kctl as *mut c_void, &sw_inject_enable_fops);
    debugfs_create_file(c"jackin_inject".as_ptr(), 0o200, (*jack_kctl).jack_debugfs_root, jack_kctl as *mut c_void, &jackin_inject_fops);
    debugfs_create_file(c"kctl_id".as_ptr(), 0o444, (*jack_kctl).jack_debugfs_root, jack_kctl as *mut c_void, &jack_kctl_id_fops);
    debugfs_create_file(c"mask_bits".as_ptr(), 0o444, (*jack_kctl).jack_debugfs_root, jack_kctl as *mut c_void, &jack_kctl_mask_bits_fops);
    debugfs_create_file(c"status".as_ptr(), 0o444, (*jack_kctl).jack_debugfs_root, jack_kctl as *mut c_void, &jack_kctl_status_fops);
    debugfs_create_file(c"type".as_ptr(), 0o444, (*jack_kctl).jack_debugfs_root, jack_kctl as *mut c_void, &jack_type_fops);

    0
}

unsafe extern "C" fn snd_jack_kctl_private_free(kctl: *mut snd_kcontrol) {
    let jack_kctl = (*kctl).private_data as *mut snd_jack_kctl;

    if !jack_kctl.is_null() {
        list_del(&mut (*jack_kctl).list);
        kfree(jack_kctl as *mut c_void);
    }
}

unsafe fn snd_jack_kctl_add(jack: *mut snd_jack, jack_kctl: *mut snd_jack_kctl) {
    (*jack_kctl).jack = jack;
    list_add_tail(&mut (*jack_kctl).list, &mut (*jack).kctl_list);
    snd_jack_debugfs_add_inject_node(jack, jack_kctl);
}

unsafe fn snd_jack_kctl_new(card: *mut snd_card, name: *const c_char, mask: c_uint) -> *mut snd_jack_kctl {
    let kctl = snd_kctl_jack_new(name, card);
    if kctl.is_null() {
        return ptr::null_mut();
    }

    let err = snd_ctl_add(card, kctl);
    if err < 0 {
        return ptr::null_mut();
    }

    let jack_kctl = kzalloc(size_of::<snd_jack_kctl>(), GFP_KERNEL) as *mut snd_jack_kctl;
    if jack_kctl.is_null() {
        snd_ctl_free_one(kctl);
        return ptr::null_mut();
    }

    (*jack_kctl).kctl = kctl;
    (*jack_kctl).mask_bits = mask;

    (*kctl).private_data = jack_kctl as *mut c_void;
    (*kctl).private_free = Some(snd_jack_kctl_private_free);

    jack_kctl
}

/**
 * snd_jack_add_new_kctl - Create a new snd_jack_kctl and add it to jack
 * @jack:  the jack instance which the kctl will attaching to
 * @name:  the name for the snd_kcontrol object
 * @mask:  a bitmask of enum snd_jack_type values that can be detected
 *         by this snd_jack_kctl object.
 *
 * Creates a new snd_kcontrol object and adds it to the jack kctl_list.
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_jack_add_new_kctl(jack: *mut snd_jack, name: *const c_char, mask: c_int) -> c_int {
    let jack_kctl = snd_jack_kctl_new((*jack).card, name, mask as c_uint);
    if jack_kctl.is_null() {
        return -ENOMEM;
    }

    snd_jack_kctl_add(jack, jack_kctl);
    0
}

/**
 * snd_jack_new - Create a new jack
 * @card:  the card instance
 * @id:    an identifying string for this jack
 * @type:  a bitmask of enum snd_jack_type values that can be detected by
 *         this jack
 * @jjack: Used to provide the allocated jack object to the caller.
 * @initial_kctl: if true, create a kcontrol and add it to the jack list.
 * @phantom_jack: Don't create a input device for phantom jacks.
 *
 * Creates a new jack object.
 *
 * Return: Zero if successful, or a negative error code on failure.
 * On success @jjack will be initialised.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_jack_new(
    card: *mut snd_card,
    id: *const c_char,
    type_: c_int,
    jjack: *mut *mut snd_jack,
    initial_kctl: bool_,
    phantom_jack: bool_,
) -> c_int {
    let mut jack_kctl: *mut snd_jack_kctl = ptr::null_mut();
    let mut err: c_int;
    let ops = snd_device_ops {
        dev_free: Some(snd_jack_dev_free),
        dev_register: Some(snd_jack_dev_register),
        dev_disconnect: Some(snd_jack_dev_disconnect),
    };

    if initial_kctl {
        jack_kctl = snd_jack_kctl_new(card, id, type_ as c_uint);
        if jack_kctl.is_null() {
            return -ENOMEM;
        }
    }

    let jack = kzalloc(size_of::<snd_jack>(), GFP_KERNEL) as *mut snd_jack;
    if jack.is_null() {
        return -ENOMEM;
    }

    (*jack).id = kstrdup(id, GFP_KERNEL);
    if (*jack).id.is_null() {
        kfree(jack as *mut c_void);
        return -ENOMEM;
    }

    /* CONFIG_SND_JACK_INPUT_DEV */
    mutex_init(&mut (*jack).input_dev_lock);

    /* don't create input device for phantom jack */
    if !phantom_jack {
        (*jack).input_dev = input_allocate_device();
        if (*jack).input_dev.is_null() {
            err = -ENOMEM;
            input_free_device((*jack).input_dev);
            kfree((*jack).id as *mut c_void);
            kfree(jack as *mut c_void);
            return err;
        }

        (*(*jack).input_dev).phys = c"ALSA".as_ptr();
        (*jack).type_ = type_;

        for i in 0..SND_JACK_SWITCH_TYPES {
            if (type_ & (1 << i)) != 0 {
                input_set_capability((*jack).input_dev, EV_SW, jack_switch_types[i]);
            }
        }
    }

    err = snd_device_new(card, SNDRV_DEV_JACK, jack as *mut c_void, &ops);
    if err < 0 {
        input_free_device((*jack).input_dev);
        kfree((*jack).id as *mut c_void);
        kfree(jack as *mut c_void);
        return err;
    }

    (*jack).card = card;
    INIT_LIST_HEAD(&mut (*jack).kctl_list);

    if initial_kctl {
        snd_jack_kctl_add(jack, jack_kctl);
    }

    *jjack = jack;

    0
}

/**
 * snd_jack_set_key - Set a key mapping on a jack
 *
 * @jack:    The jack to configure
 * @type:    Jack report type for this key
 * @keytype: Input layer key type to be reported
 *
 * Map a SND_JACK_BTN_* button type to an input layer key, allowing
 * reporting of keys on accessories via the jack abstraction.  If no
 * mapping is provided but keys are enabled in the jack type then
 * BTN_n numeric buttons will be reported.
 *
 * If jacks are not reporting via the input API this call will have no
 * effect.
 *
 * Note that this is intended to be use by simple devices with small
 * numbers of keys that can be reported.  It is also possible to
 * access the input device directly - devices with complex input
 * capabilities on accessories should consider doing this rather than
 * using this abstraction.
 *
 * This function may only be called prior to registration of the jack.
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int) -> c_int {
    let key = fls(SND_JACK_BTN_0) - fls(type_);

    WARN_ON((*jack).registered);

    if keytype == 0 || key as usize >= (*jack).key.len() {
        return -EINVAL;
    }

    (*jack).type_ |= type_;
    (*jack).key[key as usize] = keytype;
    0
}

/**
 * snd_jack_report - Report the current status of a jack
 * Note: This function uses mutexes and should be called from a
 * context which can sleep (such as a workqueue).
 *
 * @jack:   The jack to report status for
 * @status: The current status of the jack
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_jack_report(jack: *mut snd_jack, status: c_int) {
    let mut mask_bits: c_uint = 0;

    if jack.is_null() {
        return;
    }

    (*jack).hw_status_cache = status;

    let mut pos = (*jack).kctl_list.next;
    while pos != &mut (*jack).kctl_list {
        let jack_kctl = container_of_snd_jack_kctl(pos);
        if (*jack_kctl).sw_inject_enable {
            mask_bits |= (*jack_kctl).mask_bits;
        } else {
            snd_kctl_jack_report((*jack).card, (*jack_kctl).kctl, status & (*jack_kctl).mask_bits as c_int);
        }
        pos = (*pos).next;
    }

    /* CONFIG_SND_JACK_INPUT_DEV */
    let idev = input_get_device((*jack).input_dev);
    if idev.is_null() {
        return;
    }

    for i in 0..(*jack).key.len() {
        let testbit = (SND_JACK_BTN_0 >> i) & !(mask_bits as c_int);

        if ((*jack).type_ & testbit) != 0 {
            input_report_key(idev, (*jack).key[i], status & testbit);
        }
    }

    for i in 0..jack_switch_types.len() {
        let testbit = (1 << i) & !(mask_bits as c_int);

        if ((*jack).type_ & testbit) != 0 {
            input_report_switch(idev, jack_switch_types[i], status & testbit);
        }
    }

    input_sync(idev);
    input_put_device(idev);
}

unsafe fn container_of_snd_jack_kctl(list: *mut list_head) -> *mut snd_jack_kctl {
    (list as *mut u8).sub(offset_of_snd_jack_kctl_list()) as *mut snd_jack_kctl
}

const fn offset_of_snd_jack_kctl_list() -> usize {
    let uninit = core::mem::MaybeUninit::<snd_jack_kctl>::uninit();
    let base = uninit.as_ptr();
    unsafe { (&(*base).list as *const list_head as usize) - (base as usize) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
