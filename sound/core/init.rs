// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Initialization routines
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

// Translated from core/init.c.  Linux kernel include dependencies are kept as
// external declarations and opaque C-compatible types.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of_mut, null, null_mut};

type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type __poll_t = u32;
type bool_t = bool;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const EEXIST: c_int = 17;
const ENOSPC: c_int = 28;
const ENOENT: c_int = 2;
const GFP_KERNEL: c_uint = 0;
const FASYNC: c_int = 0x2000;
const EPOLLERR: __poll_t = 0x008;
const EPOLLNVAL: __poll_t = 0x020;
const SNDRV_CARDS: usize = 32;
const SND_MIXER_OSS_NOTIFY_DISCONNECT: c_int = 0;
const SND_MIXER_OSS_NOTIFY_FREE: c_int = 1;
const SND_MIXER_OSS_NOTIFY_REGISTER: c_int = 2;
const SNDRV_CTL_POWER_D0: c_int = 0;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub class: *mut class,
    pub release: Option<unsafe extern "C" fn(*mut device)>,
    pub groups: *mut *const attribute_group,
    pub kobj: kobject,
}

#[repr(C)]
pub struct file {
    pub f_op: *const file_operations,
    pub f_flags: c_int,
}

#[repr(C)]
pub struct file_operations {
    pub owner: *mut module,
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table) -> __poll_t>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    // CONFIG_COMPAT: .compat_ioctl is also snd_disconnect_ioctl in C.
    pub mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> c_int>,
    pub fasync: Option<unsafe extern "C" fn(c_int, *mut file, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_monitor_file {
    pub file: *mut file,
    pub disconnected_f_op: *const file_operations,
    pub shutdown_list: list_head, /* still need to shutdown */
    pub list: list_head,          /* link of monitor files */
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub private_data_area: [u8; 0],
    pub id: [c_char; 16],
    pub managed: bool_t,
    pub dev: *mut device,
    pub number: c_int,
    pub module: *mut module,
    pub devices: list_head,
    pub controls_rwsem: rw_semaphore,
    pub controls_rwlock: rwlock_t,
    pub controls: list_head,
    pub ctl_files: list_head,
    pub files_lock: spinlock_t,
    pub files_list: list_head,
    pub memory_mutex: mutex,
    pub remove_sleep: wait_queue_head_t,
    pub sync_irq: c_int,
    pub card_dev: device,
    pub dev_groups: [*const attribute_group; 8],
    pub irq_descr: [c_char; 64],
    pub shutdown: c_int,
    pub registered: bool_t,
    pub releasing: bool_t,
    pub components: *mut c_char,
    pub components_alloc_size: c_uint,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub release_completion: *mut completion,
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
    pub driver: [c_char; 16],
    pub proc_root: *mut snd_info_entry,
    // CONFIG_PM fields:
    pub power_sleep: wait_queue_head_t,
    pub power_ref: snd_refcount,
    // CONFIG_SND_DEBUG / CONFIG_SND_CTL_DEBUG fields:
    pub debugfs_root: *mut dentry,
    pub value_buf: *mut c_void,
}

#[repr(C)]
pub struct module {
    pub name: [c_char; 56],
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute_group {
    pub attrs: *mut *mut attribute,
}

#[repr(C)]
pub struct device_attribute {
    pub attr: attribute,
}

#[repr(C)]
pub union snd_info_entry_union {
    pub text: snd_info_entry_text,
}

#[repr(C)]
pub struct snd_info_entry_text {
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub struct snd_info_entry {
    pub c: snd_info_entry_union,
    pub name: *const c_char,
}

#[repr(C)]
pub struct class {
    _private: [u8; 0],
}
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct poll_table {
    _private: [u8; 0],
}
#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rwlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_refcount {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

static mut shutdown_lock: spinlock_t = spinlock_t { _private: [] };
static mut shutdown_files: list_head = list_head {
    next: unsafe { addr_of_mut!(shutdown_files) },
    prev: unsafe { addr_of_mut!(shutdown_files) },
};

static mut snd_cards_lock: [c_ulong; (SNDRV_CARDS + c_ulong::BITS as usize - 1) / c_ulong::BITS as usize] = [0; 1];
static mut snd_cards: [*mut snd_card; SNDRV_CARDS] = [null_mut(); SNDRV_CARDS];
static mut snd_card_mutex: mutex = mutex { _private: [] };
static mut slots: [*mut c_char; SNDRV_CARDS] = [null_mut(); SNDRV_CARDS];

unsafe extern "C" {
    static mut sound_class: class;
    static mut snd_ecards_limit: c_int;
    static mut snd_ioctl_rwsem: rw_semaphore;
    static mut THIS_MODULE: module;
    static mut sound_debugfs_root: *mut dentry;

    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn krealloc(ptr: *mut c_void, size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn devres_alloc(release: unsafe extern "C" fn(*mut device, *mut c_void), size: size_t, flags: c_uint) -> *mut c_void;
    fn devres_free(res: *mut c_void);
    fn devres_add(dev: *mut device, res: *mut c_void);
    fn devres_find(dev: *mut device, release: unsafe extern "C" fn(*mut device, *mut c_void), match_fn: *mut c_void, match_data: *mut c_void) -> *mut c_void;
    fn devm_add_action(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> c_int;
    fn devm_remove_action(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void);
    fn device_initialize(dev: *mut device);
    fn put_device(dev: *mut device);
    fn get_device(dev: *mut device);
    fn device_add(dev: *mut device) -> c_int;
    fn device_del(dev: *mut device);
    fn kobject_set_name(kobj: *mut kobject, fmt: *const c_char, ...) -> c_int;
    fn dev_driver_string(dev: *mut device) -> *const c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn strscpy(dest: *mut c_char, src: *const c_char, size: size_t) -> ssize_t;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn isascii(c: c_int) -> c_int;
    fn isalnum(c: c_int) -> c_int;
    fn isspace(c: c_int) -> c_int;
    fn isdigit(c: c_int) -> c_int;
    fn isalpha(c: c_int) -> c_int;
    fn panic(fmt: *const c_char, ...) -> !;

    fn snd_ctl_create(card: *mut snd_card) -> c_int;
    fn snd_device_free_all(card: *mut snd_card);
    fn snd_info_card_create(card: *mut snd_card) -> c_int;
    fn snd_info_card_disconnect(card: *mut snd_card);
    fn snd_info_card_free(card: *mut snd_card) -> c_int;
    fn snd_info_card_register(card: *mut snd_card) -> c_int;
    fn snd_info_card_id_change(card: *mut snd_card);
    fn snd_device_disconnect_all(card: *mut snd_card);
    fn snd_device_register_all(card: *mut snd_card) -> c_int;
    fn snd_info_check_reserved_words(id: *const c_char) -> bool_t;
    fn snd_info_create_module_entry(module: *mut module, name: *const c_char, parent: *mut c_void) -> *mut snd_info_entry;
    fn snd_info_register(entry: *mut snd_info_entry) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_remove(root: *mut dentry);
    fn synchronize_irq(irq: c_int);
    fn complete(x: *mut completion);
    fn wait_for_completion(x: *mut completion);
    fn wake_up_all(q: *mut wait_queue_head_t);
    fn snd_refcount_init(refcount: *mut snd_refcount);
    fn snd_power_ref(card: *mut snd_card);
    fn snd_power_unref(card: *mut snd_card);
    fn snd_power_get_state(card: *mut snd_card) -> c_int;
    fn snd_power_sync_ref(card: *mut snd_card);
    fn fops_get(fops: *const file_operations);
    fn fops_put(fops: *const file_operations);
}

#[cfg(any())]
extern "C" {
    // IS_ENABLED(CONFIG_SND_MIXER_OSS)
    pub static mut snd_mixer_oss_notify_callback: Option<unsafe extern "C" fn(*mut snd_card, c_int) -> c_int>;
}

unsafe fn test_bit(bit: c_int, addr: *const c_ulong) -> bool {
    let word = bit as usize / c_ulong::BITS as usize;
    let off = bit as usize % c_ulong::BITS as usize;
    ((*addr.add(word)) & (1 as c_ulong).wrapping_shl(off as u32)) != 0
}

unsafe fn set_bit(bit: c_int, addr: *mut c_ulong) {
    let word = bit as usize / c_ulong::BITS as usize;
    let off = bit as usize % c_ulong::BITS as usize;
    *addr.add(word) |= (1 as c_ulong).wrapping_shl(off as u32);
}

unsafe fn clear_bit(bit: c_int, addr: *mut c_ulong) {
    let word = bit as usize / c_ulong::BITS as usize;
    let off = bit as usize % c_ulong::BITS as usize;
    *addr.add(word) &= !(1 as c_ulong).wrapping_shl(off as u32);
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn list_empty(head: *const list_head) -> bool {
    (*head).next == head as *mut list_head
}

unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = prev;
    (*prev).next = new;
}

unsafe fn list_add(new: *mut list_head, head: *mut list_head) {
    __list_add(new, head, (*head).next);
}

unsafe fn __list_del(prev: *mut list_head, next: *mut list_head) {
    (*next).prev = prev;
    (*prev).next = next;
}

unsafe fn list_del(entry: *mut list_head) {
    __list_del((*entry).prev, (*entry).next);
}

unsafe fn list_del_init(entry: *mut list_head) {
    list_del(entry);
    INIT_LIST_HEAD(entry);
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! container_of {
    ($ptr:expr, $type:ty, $field:ident) => {{
        ($ptr as *mut u8).sub(core::mem::offset_of!($type, $field)) as *mut $type
    }};
}

unsafe fn roundup(x: c_uint, y: c_uint) -> c_uint {
    ((x + y - 1) / y) * y
}

unsafe fn snd_BUG_ON(cond: bool) -> bool {
    cond
}

unsafe fn snd_BUG() {}

unsafe fn init_rwsem(_sem: *mut rw_semaphore) {}
unsafe fn rwlock_init(_lock: *mut rwlock_t) {}
unsafe fn spin_lock_init(_lock: *mut spinlock_t) {}
unsafe fn mutex_init(_mutex: *mut mutex) {}
unsafe fn init_waitqueue_head(_q: *mut wait_queue_head_t) {}

/* return non-zero if the given index is reserved for the given
 * module via slots option
 */
unsafe fn module_slot_match(module: *mut module, idx: c_int) -> c_int {
    let mut match_: c_int = 1;
    // CONFIG_MODULES
    let mut s1: *const c_char;
    let mut s2: *const c_char;

    if module.is_null() || (*module).name[0] == 0 || slots[idx as usize].is_null() {
        return 0;
    }

    s1 = (*module).name.as_ptr();
    s2 = slots[idx as usize];
    if *s2 == b'!' as c_char {
        match_ = 0; /* negative match */
        s2 = s2.add(1);
    }
    /* compare module name strings
     * hyphens are handled as equivalent with underscore
     */
    loop {
        let mut c1 = *s1;
        let mut c2 = *s2;
        s1 = s1.add(1);
        s2 = s2.add(1);
        if c1 == b'-' as c_char {
            c1 = b'_' as c_char;
        }
        if c2 == b'-' as c_char {
            c2 = b'_' as c_char;
        }
        if c1 != c2 {
            return if match_ == 0 { 1 } else { 0 };
        }
        if c1 == 0 {
            break;
        }
    }
    match_
}

unsafe fn check_empty_slot(_module: *mut module, slot: c_int) -> c_int {
    (slots[slot as usize].is_null() || *slots[slot as usize] == 0) as c_int
}

/* return an empty slot number (>= 0) found in the given bitmask @mask.
 * @mask == -1 == 0xffffffff means: take any free slot up to 32
 * when no slot is available, return the original @mask as is.
 */
unsafe fn get_slot_from_bitmask(
    mask: c_int,
    check: unsafe fn(*mut module, c_int) -> c_int,
    module: *mut module,
) -> c_int {
    let mut slot: c_int = 0;

    while slot < SNDRV_CARDS as c_int {
        if slot < 32 && (mask & ((1u32 << slot) as c_int)) == 0 {
            slot += 1;
            continue;
        }
        if !test_bit(slot, snd_cards_lock.as_ptr()) {
            if check(module, slot) != 0 {
                return slot; /* found */
            }
        }
        slot += 1;
    }
    mask /* unchanged */
}

/* the default release callback set in snd_device_alloc() */
unsafe extern "C" fn default_release_alloc(dev: *mut device) {
    kfree(dev as *mut c_void);
}

/**
 * snd_device_alloc - Allocate and initialize struct device for sound devices
 * @dev_p: pointer to store the allocated device
 * @card: card to assign, optional
 *
 * For releasing the allocated device, call put_device().
 */
#[no_mangle]
pub unsafe extern "C" fn snd_device_alloc(dev_p: *mut *mut device, card: *mut snd_card) -> c_int {
    let dev: *mut device;

    *dev_p = null_mut();
    dev = kzalloc(size_of::<device>(), GFP_KERNEL) as *mut device;
    if dev.is_null() {
        return -ENOMEM;
    }
    device_initialize(dev);
    if !card.is_null() {
        (*dev).parent = addr_of_mut!((*card).card_dev);
    }
    (*dev).class = addr_of_mut!(sound_class);
    (*dev).release = Some(default_release_alloc);
    *dev_p = dev;
    0
}

unsafe fn snd_card_init(
    card: *mut snd_card,
    parent: *mut device,
    mut idx: c_int,
    xid: *const c_char,
    module: *mut module,
    extra_size: size_t,
) -> c_int {
    let mut err: c_int;

    if extra_size > 0 {
        (*card).private_data = (*card).private_data_area.as_mut_ptr() as *mut c_void;
    }
    if !xid.is_null() {
        strscpy((*card).id.as_mut_ptr(), xid, size_of_val(&(*card).id));
    }
    err = 0;
    {
        if idx < 0 {
            /* first check the matching module-name slot */
            idx = get_slot_from_bitmask(idx, module_slot_match, module);
        }
        if idx < 0 {
            /* if not matched, assign an empty slot */
            idx = get_slot_from_bitmask(idx, check_empty_slot, module);
        }
        if idx < 0 {
            err = -ENODEV;
        } else if idx < snd_ecards_limit {
            if test_bit(idx, snd_cards_lock.as_ptr()) {
                err = -EBUSY; /* invalid */
            }
        } else if idx >= SNDRV_CARDS as c_int {
            err = -ENODEV;
        }
        if err == 0 {
            set_bit(idx, snd_cards_lock.as_mut_ptr()); /* lock it */
            if idx >= snd_ecards_limit {
                snd_ecards_limit = idx + 1; /* increase the limit */
            }
        }
    }
    if err < 0 {
        dev_err(
            parent,
            cstr!("cannot find the slot for index %d (range 0-%i), error: %d\n"),
            idx,
            snd_ecards_limit - 1,
            err,
        );
        if !(*card).managed {
            kfree(card as *mut c_void); /* manually free here, as no destructor called */
        }
        return err;
    }
    (*card).dev = parent;
    (*card).number = idx;
    // WARN_ON(IS_MODULE(CONFIG_SND) && !module);
    (*card).module = module;
    INIT_LIST_HEAD(addr_of_mut!((*card).devices));
    init_rwsem(addr_of_mut!((*card).controls_rwsem));
    rwlock_init(addr_of_mut!((*card).controls_rwlock));
    INIT_LIST_HEAD(addr_of_mut!((*card).controls));
    INIT_LIST_HEAD(addr_of_mut!((*card).ctl_files));
    // CONFIG_SND_CTL_FAST_LOOKUP: xa_init(&card->ctl_numids); xa_init(&card->ctl_hash);
    spin_lock_init(addr_of_mut!((*card).files_lock));
    INIT_LIST_HEAD(addr_of_mut!((*card).files_list));
    mutex_init(addr_of_mut!((*card).memory_mutex));
    // CONFIG_PM
    init_waitqueue_head(addr_of_mut!((*card).power_sleep));
    snd_refcount_init(addr_of_mut!((*card).power_ref));
    init_waitqueue_head(addr_of_mut!((*card).remove_sleep));
    (*card).sync_irq = -1;

    device_initialize(addr_of_mut!((*card).card_dev));
    (*card).card_dev.parent = parent;
    (*card).card_dev.class = addr_of_mut!(sound_class);
    (*card).card_dev.release = Some(release_card_device);
    (*card).card_dev.groups = (*card).dev_groups.as_mut_ptr();
    (*card).dev_groups[0] = addr_of!(card_dev_attr_group);
    err = kobject_set_name(addr_of_mut!((*card).card_dev.kobj), cstr!("card%d"), idx);
    if err < 0 {
        return snd_card_init_error(card, err);
    }

    snprintf(
        (*card).irq_descr.as_mut_ptr(),
        size_of_val(&(*card).irq_descr),
        cstr!("%s:%s"),
        dev_driver_string((*card).dev),
        dev_name(addr_of_mut!((*card).card_dev)),
    );

    /* the control interface cannot be accessed from the user space until */
    /* snd_cards_bitmask and snd_cards are set with snd_card_register */
    err = snd_ctl_create(card);
    if err < 0 {
        dev_err(parent, cstr!("unable to register control minors\n"));
        return snd_card_init_error(card, err);
    }
    err = snd_info_card_create(card);
    if err < 0 {
        dev_err(parent, cstr!("unable to create card info\n"));
        snd_device_free_all(card);
        return snd_card_init_error(card, err);
    }

    // CONFIG_SND_DEBUG
    (*card).debugfs_root = debugfs_create_dir(dev_name(addr_of_mut!((*card).card_dev)), sound_debugfs_root);
    // CONFIG_SND_CTL_DEBUG
    (*card).value_buf = kmalloc(size_of::<*mut c_void>(), GFP_KERNEL);
    if (*card).value_buf.is_null() {
        return -ENOMEM;
    }
    0
}

unsafe fn snd_card_init_error(card: *mut snd_card, err: c_int) -> c_int {
    put_device(addr_of_mut!((*card).card_dev));
    err
}

unsafe extern "C" fn release_card_device(dev: *mut device) {
    snd_card_do_free(container_of!(dev, snd_card, card_dev));
}

/**
 *  snd_card_new - create and initialize a soundcard structure
 */
#[no_mangle]
pub unsafe extern "C" fn snd_card_new(
    parent: *mut device,
    idx: c_int,
    xid: *const c_char,
    module: *mut module,
    mut extra_size: c_int,
    card_ret: *mut *mut snd_card,
) -> c_int {
    let card: *mut snd_card;
    let err: c_int;

    if snd_BUG_ON(card_ret.is_null()) {
        return -EINVAL;
    }
    *card_ret = null_mut();

    if extra_size < 0 {
        extra_size = 0;
    }
    card = kzalloc(size_of::<snd_card>() + extra_size as size_t, GFP_KERNEL) as *mut snd_card;
    if card.is_null() {
        return -ENOMEM;
    }

    err = snd_card_init(card, parent, idx, xid, module, extra_size as size_t);
    if err < 0 {
        return err; /* card is freed by error handler */
    }

    *card_ret = card;
    0
}

unsafe extern "C" fn __snd_card_release(_dev: *mut device, data: *mut c_void) {
    snd_card_free(data as *mut snd_card);
}

#[no_mangle]
pub unsafe extern "C" fn snd_devm_card_new(
    parent: *mut device,
    idx: c_int,
    xid: *const c_char,
    module: *mut module,
    extra_size: size_t,
    card_ret: *mut *mut snd_card,
) -> c_int {
    let card: *mut snd_card;
    let err: c_int;

    *card_ret = null_mut();
    card = devres_alloc(__snd_card_release, size_of::<snd_card>() + extra_size, GFP_KERNEL) as *mut snd_card;
    if card.is_null() {
        return -ENOMEM;
    }
    (*card).managed = true;

    err = snd_card_init(card, parent, idx, xid, module, extra_size);
    if err < 0 {
        devres_free(card as *mut c_void); /* in managed mode, we need to free manually */
        return err;
    }

    devres_add(parent, card as *mut c_void);
    *card_ret = card;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_card_free_on_error(dev: *mut device, ret: c_int) -> c_int {
    let card: *mut snd_card;

    if ret == 0 {
        return 0;
    }
    card = devres_find(dev, __snd_card_release, null_mut(), null_mut()) as *mut snd_card;
    if !card.is_null() {
        snd_card_free(card);
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_card_ref(idx: c_int) -> *mut snd_card {
    let card = snd_cards[idx as usize];
    if !card.is_null() {
        get_device(addr_of_mut!((*card).card_dev));
    }
    card
}

#[no_mangle]
pub unsafe extern "C" fn snd_card_locked(card: c_int) -> c_int {
    test_bit(card, snd_cards_lock.as_ptr()) as c_int
}

unsafe extern "C" fn snd_disconnect_llseek(_file: *mut file, _offset: loff_t, _orig: c_int) -> loff_t {
    -(ENODEV as loff_t)
}

unsafe extern "C" fn snd_disconnect_read(
    _file: *mut file,
    _buf: *mut c_char,
    _count: size_t,
    _offset: *mut loff_t,
) -> ssize_t {
    -(ENODEV as ssize_t)
}

unsafe extern "C" fn snd_disconnect_write(
    _file: *mut file,
    _buf: *const c_char,
    _count: size_t,
    _offset: *mut loff_t,
) -> ssize_t {
    -(ENODEV as ssize_t)
}

unsafe extern "C" fn snd_disconnect_release(inode: *mut inode, file: *mut file) -> c_int {
    let mut df: *mut snd_monitor_file = null_mut();
    let mut pos = shutdown_files.next;

    while pos != addr_of_mut!(shutdown_files) {
        let _df = container_of!(pos, snd_monitor_file, shutdown_list);
        if (*_df).file == file {
            df = _df;
            list_del_init(addr_of_mut!((*df).shutdown_list));
            break;
        }
        pos = (*pos).next;
    }

    if !df.is_null() {
        if ((*file).f_flags & FASYNC) != 0 {
            if let Some(fasync) = (*(*df).disconnected_f_op).fasync {
                fasync(-1, file, 0);
            }
        }
        return ((*(*df).disconnected_f_op).release.unwrap())(inode, file);
    }

    panic(cstr!("%s(%p, %p) failed!"), cstr!("snd_disconnect_release"), inode, file);
}

unsafe extern "C" fn snd_disconnect_poll(_file: *mut file, _wait: *mut poll_table) -> __poll_t {
    EPOLLERR | EPOLLNVAL
}

unsafe extern "C" fn snd_disconnect_ioctl(_file: *mut file, _cmd: c_uint, _arg: c_ulong) -> c_long {
    -(ENODEV as c_long)
}

unsafe extern "C" fn snd_disconnect_mmap(_file: *mut file, _vma: *mut vm_area_struct) -> c_int {
    -ENODEV
}

unsafe extern "C" fn snd_disconnect_fasync(_fd: c_int, _file: *mut file, _on: c_int) -> c_int {
    -ENODEV
}

static snd_shutdown_f_ops: file_operations = file_operations {
    owner: unsafe { addr_of_mut!(THIS_MODULE) },
    llseek: Some(snd_disconnect_llseek),
    read: Some(snd_disconnect_read),
    write: Some(snd_disconnect_write),
    release: Some(snd_disconnect_release),
    poll: Some(snd_disconnect_poll),
    unlocked_ioctl: Some(snd_disconnect_ioctl),
    mmap: Some(snd_disconnect_mmap),
    fasync: Some(snd_disconnect_fasync),
};

#[no_mangle]
pub unsafe extern "C" fn snd_card_disconnect(card: *mut snd_card) {
    if card.is_null() {
        return;
    }

    if (*card).shutdown != 0 {
        return;
    }
    (*card).shutdown = 1;

    /* replace file->f_op with special dummy operations */
    let mut pos = (*card).files_list.next;
    while pos != addr_of_mut!((*card).files_list) {
        let mfile = container_of!(pos, snd_monitor_file, list);
        pos = (*pos).next;
        /* it's critical part, use endless loop */
        /* we have no room to fail */
        (*mfile).disconnected_f_op = (*(*mfile).file).f_op;
        list_add(addr_of_mut!((*mfile).shutdown_list), addr_of_mut!(shutdown_files));
        (*(*mfile).file).f_op = addr_of!(snd_shutdown_f_ops);
        fops_get((*(*mfile).file).f_op);
    }

    // CONFIG_PM
    /* wake up sleepers here before other callbacks for avoiding potential
     * deadlocks with other locks (e.g. in kctls);
     * then this notifies the shutdown and sleepers would abort immediately
     */
    wake_up_all(addr_of_mut!((*card).power_sleep));

    /* notify all connected devices about disconnection */
    /* at this point, they cannot respond to any calls except release() */

    // IS_ENABLED(CONFIG_SND_MIXER_OSS): call snd_mixer_oss_notify_callback(...DISCONNECT)

    /* notify all devices that we are disconnected */
    snd_device_disconnect_all(card);

    if (*card).sync_irq > 0 {
        synchronize_irq((*card).sync_irq);
    }

    snd_info_card_disconnect(card);
    // CONFIG_SND_DEBUG
    debugfs_remove((*card).debugfs_root);
    (*card).debugfs_root = null_mut();

    if (*card).registered {
        device_del(addr_of_mut!((*card).card_dev));
        (*card).registered = false;
    }

    /* disable fops (user space) operations for ALSA API */
    snd_cards[(*card).number as usize] = null_mut();
    clear_bit((*card).number, snd_cards_lock.as_mut_ptr());

    snd_power_sync_ref(card);
}

#[no_mangle]
pub unsafe extern "C" fn snd_card_disconnect_sync(card: *mut snd_card) {
    snd_card_disconnect(card);

    // wait_event_lock_irq(card->remove_sleep, list_empty(&card->files_list), card->files_lock);
    while !list_empty(addr_of!((*card).files_list)) {}
}

unsafe fn snd_card_do_free(card: *mut snd_card) -> c_int {
    let managed = (*card).managed;

    (*card).releasing = true;
    // IS_ENABLED(CONFIG_SND_MIXER_OSS): call snd_mixer_oss_notify_callback(...FREE)
    snd_device_free_all(card);
    kfree((*card).components as *mut c_void);
    (*card).components = null_mut();
    (*card).components_alloc_size = 0;
    if let Some(private_free) = (*card).private_free {
        private_free(card);
    }
    // CONFIG_SND_CTL_DEBUG
    kfree((*card).value_buf);
    if snd_info_card_free(card) < 0 {
        dev_warn((*card).dev, cstr!("unable to free card info\n"));
        /* Not fatal error */
    }
    if !(*card).release_completion.is_null() {
        complete((*card).release_completion);
    }
    if !managed {
        kfree(card as *mut c_void);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_card_free_when_closed(card: *mut snd_card) {
    if card.is_null() {
        return;
    }

    snd_card_disconnect(card);
    put_device(addr_of_mut!((*card).card_dev));
}

#[no_mangle]
pub unsafe extern "C" fn snd_card_free(card: *mut snd_card) {
    let mut released: completion = zeroed();

    if (*card).releasing {
        return;
    }

    (*card).release_completion = addr_of_mut!(released);
    snd_card_free_when_closed(card);

    /* wait, until all devices are ready for the free operation */
    wait_for_completion(addr_of_mut!(released));
}

/* check, if the character is in the valid ASCII range */
unsafe fn safe_ascii_char(c: c_char) -> bool {
    isascii(c as c_int) != 0 && isalnum(c as c_int) != 0
}

/* retrieve the last word of shortname or longname */
unsafe fn retrieve_id_from_card_name(mut name: *const c_char) -> *const c_char {
    let mut spos = name;

    while *name != 0 {
        if isspace(*name as c_int) != 0 && safe_ascii_char(*name.add(1)) {
            spos = name.add(1);
        }
        name = name.add(1);
    }
    spos
}

/* return true if the given id string doesn't conflict any other card ids */
unsafe fn card_id_ok(card: *mut snd_card, id: *const c_char) -> bool {
    let mut i: c_int;
    if !snd_info_check_reserved_words(id) {
        return false;
    }
    i = 0;
    while i < snd_ecards_limit {
        if !snd_cards[i as usize].is_null()
            && snd_cards[i as usize] != card
            && strcmp((*snd_cards[i as usize]).id.as_ptr(), id) == 0
        {
            return false;
        }
        i += 1;
    }
    true
}

/* copy to card->id only with valid letters from nid */
unsafe fn copy_valid_id_string(card: *mut snd_card, src: *const c_char, mut nid: *const c_char) {
    let mut id = (*card).id.as_mut_ptr();

    while *nid != 0 && !safe_ascii_char(*nid) {
        nid = nid.add(1);
    }
    if isdigit(*nid as c_int) != 0 {
        *id = if isalpha(*src as c_int) != 0 { *src } else { b'D' as c_char };
        id = id.add(1);
    }
    while *nid != 0 && (id as usize - (*card).id.as_ptr() as usize) < size_of_val(&(*card).id) - 1 {
        if safe_ascii_char(*nid) {
            *id = *nid;
            id = id.add(1);
        }
        nid = nid.add(1);
    }
    *id = 0;
}

/* Set card->id from the given string
 * If the string conflicts with other ids, add a suffix to make it unique.
 */
unsafe fn snd_card_set_id_no_lock(card: *mut snd_card, src: *const c_char, nid: *const c_char) {
    let mut len: c_int;
    let mut loops: c_int;
    let mut is_default = false;
    let id: *mut c_char;

    copy_valid_id_string(card, src, nid);
    id = (*card).id.as_mut_ptr();

    loop {
        /* use "Default" for obviously invalid strings
         * ("card" conflicts with proc directories)
         */
        if *id == 0 || strncmp(id, cstr!("card"), 4) == 0 {
            strscpy((*card).id.as_mut_ptr(), cstr!("Default"), size_of_val(&(*card).id));
            is_default = true;
        }

        len = strlen(id) as c_int;
        loops = 0;
        while loops < SNDRV_CARDS as c_int {
            let mut sfxstr = [0 as c_char; 5]; /* "_012" */
            let sfxlen: c_int;
            let slen: c_int;

            if card_id_ok(card, id) {
                return; /* OK */
            }

            /* Add _XYZ suffix */
            sfxlen = scnprintf(sfxstr.as_mut_ptr(), size_of_val(&sfxstr), cstr!("_%X"), loops + 1) as c_int;
            if len + sfxlen >= size_of_val(&(*card).id) as c_int {
                slen = size_of_val(&(*card).id) as c_int - sfxlen - 1;
            } else {
                slen = len;
            }
            strscpy(id.add(slen as usize), sfxstr.as_ptr(), size_of_val(&(*card).id) - slen as usize);
            loops += 1;
        }
        /* fallback to the default id */
        if !is_default {
            *id = 0;
            continue;
        }
        break;
    }
    /* last resort... */
    dev_err((*card).dev, cstr!("unable to set card id (%s)\n"), id);
    if !(*(*card).proc_root).name.is_null() {
        strscpy((*card).id.as_mut_ptr(), (*(*card).proc_root).name, size_of_val(&(*card).id));
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_card_set_id(card: *mut snd_card, nid: *const c_char) {
    /* check if user specified own card->id */
    if (*card).id[0] != 0 {
        return;
    }
    snd_card_set_id_no_lock(card, nid, nid);
}

unsafe extern "C" fn id_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let card = container_of!(dev, snd_card, card_dev);
    sysfs_emit(buf, cstr!("%s\n"), (*card).id.as_ptr())
}

unsafe extern "C" fn id_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let card = container_of!(dev, snd_card, card_dev);
    let mut buf1 = [0 as c_char; 16];
    let copy = if count > size_of_val(&(*card).id) - 1 {
        size_of_val(&(*card).id) - 1
    } else {
        count
    };
    let mut idx: size_t;
    let mut c: c_int;

    idx = 0;
    while idx < copy {
        c = *buf.add(idx) as c_int;
        if !safe_ascii_char(c as c_char) && c != b'_' as c_int && c != b'-' as c_int {
            return -(EINVAL as ssize_t);
        }
        idx += 1;
    }
    memcpy(buf1.as_mut_ptr() as *mut c_void, buf as *const c_void, copy);
    buf1[copy] = 0;
    if !card_id_ok(null_mut(), buf1.as_ptr()) {
        return -(EEXIST as ssize_t);
    }
    strscpy((*card).id.as_mut_ptr(), buf1.as_ptr(), size_of_val(&(*card).id));
    snd_info_card_id_change(card);

    count as ssize_t
}

static mut dev_attr_id: device_attribute = device_attribute { attr: attribute { _private: [] } };

unsafe extern "C" fn number_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let card = container_of!(dev, snd_card, card_dev);
    sysfs_emit(buf, cstr!("%i\n"), (*card).number)
}

static mut dev_attr_number: device_attribute = device_attribute { attr: attribute { _private: [] } };

static mut card_dev_attrs: [*mut attribute; 3] = unsafe {
    [
        addr_of_mut!(dev_attr_id.attr),
        addr_of_mut!(dev_attr_number.attr),
        null_mut(),
    ]
};

static card_dev_attr_group: attribute_group = attribute_group {
    attrs: unsafe { card_dev_attrs.as_mut_ptr() },
};

#[no_mangle]
pub unsafe extern "C" fn snd_card_add_dev_attr(card: *mut snd_card, group: *const attribute_group) -> c_int {
    let mut i: usize;

    /* loop for (arraysize-1) here to keep NULL at the last entry */
    i = 0;
    while i < (*card).dev_groups.len() - 1 {
        if (*card).dev_groups[i].is_null() {
            (*card).dev_groups[i] = group;
            return 0;
        }
        i += 1;
    }

    dev_err((*card).dev, cstr!("Too many groups assigned\n"));
    -ENOSPC
}

unsafe extern "C" fn trigger_card_free(data: *mut c_void) {
    snd_card_free(data as *mut snd_card);
}

#[no_mangle]
pub unsafe extern "C" fn snd_card_register(card: *mut snd_card) -> c_int {
    let mut err: c_int;

    if snd_BUG_ON(card.is_null()) {
        return -EINVAL;
    }

    if !(*card).registered {
        err = device_add(addr_of_mut!((*card).card_dev));
        if err < 0 {
            return err;
        }
        (*card).registered = true;
    } else if (*card).managed {
        devm_remove_action((*card).dev, trigger_card_free, card as *mut c_void);
    }

    if (*card).managed {
        err = devm_add_action((*card).dev, trigger_card_free, card as *mut c_void);
        if err < 0 {
            return err;
        }
    }

    err = snd_device_register_all(card);
    if err < 0 {
        return err;
    }
    {
        if !snd_cards[(*card).number as usize].is_null() {
            /* already registered */
            return snd_info_card_register(card); /* register pending info */
        }
        if (*card).id[0] != 0 {
            /* make a unique id name from the given string */
            let mut tmpid = [0 as c_char; 16];
            memcpy(tmpid.as_mut_ptr() as *mut c_void, (*card).id.as_ptr() as *const c_void, size_of_val(&(*card).id));
            snd_card_set_id_no_lock(card, tmpid.as_ptr(), tmpid.as_ptr());
        } else {
            /* create an id from either shortname or longname */
            let src: *const c_char = if (*card).shortname[0] != 0 {
                (*card).shortname.as_ptr()
            } else {
                (*card).longname.as_ptr()
            };
            snd_card_set_id_no_lock(card, src, retrieve_id_from_card_name(src));
        }
        snd_cards[(*card).number as usize] = card;
    }
    err = snd_info_card_register(card);
    if err < 0 {
        return err;
    }

    // IS_ENABLED(CONFIG_SND_MIXER_OSS): call snd_mixer_oss_notify_callback(...REGISTER)
    0
}

// CONFIG_SND_PROC_FS
unsafe extern "C" fn snd_card_info_read(_entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let mut idx: c_int;
    let mut count: c_int;
    let mut card: *mut snd_card;

    idx = 0;
    count = 0;
    while idx < SNDRV_CARDS as c_int {
        card = snd_cards[idx as usize];
        if !card.is_null() {
            count += 1;
            snd_iprintf(
                buffer,
                cstr!("%2i [%-15s]: %s - %s\n"),
                idx,
                (*card).id.as_ptr(),
                (*card).driver.as_ptr(),
                (*card).shortname.as_ptr(),
            );
            snd_iprintf(buffer, cstr!("                      %s\n"), (*card).longname.as_ptr());
        }
        idx += 1;
    }
    if count == 0 {
        snd_iprintf(buffer, cstr!("--- no soundcards ---\n"));
    }
}

// CONFIG_SND_OSSEMUL
#[no_mangle]
pub unsafe extern "C" fn snd_card_info_read_oss(buffer: *mut snd_info_buffer) {
    let mut idx: c_int;
    let mut count: c_int;
    let mut card: *mut snd_card;

    idx = 0;
    count = 0;
    while idx < SNDRV_CARDS as c_int {
        card = snd_cards[idx as usize];
        if !card.is_null() {
            count += 1;
            snd_iprintf(buffer, cstr!("%s\n"), (*card).longname.as_ptr());
        }
        idx += 1;
    }
    if count == 0 {
        snd_iprintf(buffer, cstr!("--- no soundcards ---\n"));
    }
}

// CONFIG_MODULES
unsafe extern "C" fn snd_card_module_info_read(_entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let mut idx: c_int = 0;
    let mut card: *mut snd_card;

    while idx < SNDRV_CARDS as c_int {
        card = snd_cards[idx as usize];
        if !card.is_null() {
            snd_iprintf(buffer, cstr!("%2i %s\n"), idx, (*(*card).module).name.as_ptr());
        }
        idx += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_card_info_init() -> c_int {
    let mut entry: *mut snd_info_entry;

    entry = snd_info_create_module_entry(addr_of_mut!(THIS_MODULE), cstr!("cards"), null_mut());
    if entry.is_null() {
        return -ENOMEM;
    }
    (*entry).c.text.read = Some(snd_card_info_read);
    if snd_info_register(entry) < 0 {
        return -ENOMEM; /* freed in error path */
    }

    // CONFIG_MODULES
    entry = snd_info_create_module_entry(addr_of_mut!(THIS_MODULE), cstr!("modules"), null_mut());
    if entry.is_null() {
        return -ENOMEM;
    }
    (*entry).c.text.read = Some(snd_card_module_info_read);
    if snd_info_register(entry) < 0 {
        return -ENOMEM; /* freed in error path */
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_component_add(card: *mut snd_card, component: *const c_char) -> c_int {
    let mut ptr: *mut c_char;
    let len: c_int = strlen(component) as c_int;
    let cur_len: c_uint;
    let need_len: c_uint;

    if !(*card).components.is_null() {
        ptr = strstr((*card).components, component);
        if !ptr.is_null() {
            if *ptr.add(len as usize) == 0 || *ptr.add(len as usize) == b' ' as c_char {
                /* already there */
                return 1;
            }
        }
        cur_len = strlen((*card).components) as c_uint + 1;
    } else {
        cur_len = 0;
    }

    need_len = cur_len + len as c_uint + 1;
    if need_len > 512 {
        snd_BUG();
        return -ENOMEM;
    }

    if need_len > (*card).components_alloc_size {
        let new_alloc = roundup(need_len, 32);

        ptr = krealloc((*card).components as *mut c_void, new_alloc as size_t, GFP_KERNEL) as *mut c_char;
        if ptr.is_null() {
            return -ENOMEM;
        }
        if (*card).components.is_null() {
            *ptr = 0;
        }
        (*card).components = ptr;
        (*card).components_alloc_size = new_alloc;
    }

    if *(*card).components != 0 {
        strcat((*card).components, cstr!(" "));
    }
    strcat((*card).components, component);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_card_file_add(card: *mut snd_card, file: *mut file) -> c_int {
    let mfile: *mut snd_monitor_file;

    mfile = kmalloc(size_of::<snd_monitor_file>(), GFP_KERNEL) as *mut snd_monitor_file;
    if mfile.is_null() {
        return -ENOMEM;
    }
    (*mfile).file = file;
    (*mfile).disconnected_f_op = null();
    INIT_LIST_HEAD(addr_of_mut!((*mfile).shutdown_list));
    if (*card).shutdown != 0 {
        kfree(mfile as *mut c_void);
        return -ENODEV;
    }
    list_add(addr_of_mut!((*mfile).list), addr_of_mut!((*card).files_list));
    get_device(addr_of_mut!((*card).card_dev));
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_card_file_remove(card: *mut snd_card, file: *mut file) -> c_int {
    let mut found: *mut snd_monitor_file = null_mut();

    let mut pos = (*card).files_list.next;
    while pos != addr_of_mut!((*card).files_list) {
        let mfile = container_of!(pos, snd_monitor_file, list);
        pos = (*pos).next;
        if (*mfile).file == file {
            list_del(addr_of_mut!((*mfile).list));
            list_del(addr_of_mut!((*mfile).shutdown_list));
            if !(*mfile).disconnected_f_op.is_null() {
                fops_put((*mfile).disconnected_f_op);
            }
            found = mfile;
            break;
        }
    }
    if list_empty(addr_of!((*card).files_list)) {
        wake_up_all(addr_of_mut!((*card).remove_sleep));
    }
    if found.is_null() {
        dev_err((*card).dev, cstr!("card file remove problem (%p)\n"), file);
        return -ENOENT;
    }
    kfree(found as *mut c_void);
    put_device(addr_of_mut!((*card).card_dev));
    0
}

// CONFIG_PM
#[no_mangle]
pub unsafe extern "C" fn snd_power_ref_and_wait(card: *mut snd_card) -> c_int {
    snd_power_ref(card);
    if snd_power_get_state(card) == SNDRV_CTL_POWER_D0 {
        return 0;
    }
    // wait_event_cmd(card->power_sleep, card->shutdown || state == D0,
    //                snd_power_unref(card), snd_power_ref(card));
    snd_power_unref(card);
    while (*card).shutdown == 0 && snd_power_get_state(card) != SNDRV_CTL_POWER_D0 {}
    snd_power_ref(card);
    if (*card).shutdown != 0 {
        snd_power_unref(card);
        return -ENODEV;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_power_wait(card: *mut snd_card) -> c_int {
    let ret: c_int;

    ret = snd_power_ref_and_wait(card);
    if ret == 0 {
        snd_power_unref(card);
    }
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
