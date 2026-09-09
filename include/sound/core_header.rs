/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of sound/core.h; kernel dependencies are supplied externally. */

/* CONFIG_SND_DYNAMIC_MINORS: SNDRV_CARDS = CONFIG_SND_MAX_CARDS; otherwise 8. */
pub const SNDRV_CARDS: i32 = 8;
pub const CONFIG_SND_MAJOR: i32 = 116;

/* Forward declarations supplied by the surrounding kernel translation. */
pub enum pci_dev {}
pub enum module {}
pub enum completion {}
pub enum snd_card {}
pub enum snd_info_entry {}
pub enum proc_dir_entry {}
pub enum snd_shutdown_f_ops {}
pub enum attribute_group {}
pub enum file_operations {}
pub enum resource {}
pub enum snd_ctl_elem_value {}
pub enum snd_mixer_oss {}
pub enum dentry {}
pub enum snd_fasync {}
pub enum device {}
pub enum list_head {}
pub enum atomic_t {}
pub enum wait_queue_head_t {}
pub enum rw_semaphore {}
pub enum rwlock_t {}
pub enum xarray {}
pub enum mutex {}
pub enum spinlock_t {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum snd_device_type {
    SNDRV_DEV_LOWLEVEL,
    SNDRV_DEV_INFO,
    SNDRV_DEV_BUS,
    SNDRV_DEV_CODEC,
    SNDRV_DEV_PCM,
    SNDRV_DEV_COMPRESS,
    SNDRV_DEV_RAWMIDI,
    SNDRV_DEV_TIMER,
    SNDRV_DEV_SEQUENCER,
    SNDRV_DEV_HWDEP,
    SNDRV_DEV_JACK,
    SNDRV_DEV_CONTROL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum snd_device_state {
    SNDRV_DEV_BUILD,
    SNDRV_DEV_REGISTERED,
    SNDRV_DEV_DISCONNECTED,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> i32>,
    pub dev_register: Option<unsafe extern "C" fn(*mut snd_device) -> i32>,
    pub dev_disconnect: Option<unsafe extern "C" fn(*mut snd_device) -> i32>,
}

#[repr(C)]
pub struct snd_device {
    pub list: list_head,
    pub card: *mut snd_card,
    pub state: snd_device_state,
    pub type_: snd_device_type,
    pub device_data: *mut core::ffi::c_void,
    pub ops: *const snd_device_ops,
}

/* snd_device(n) is the C list_entry container macro. */

#[repr(C)]
pub struct snd_refcount {
    pub count: atomic_t,
    pub waiter: wait_queue_head_t,
}

extern "C" {
    pub fn snd_refcount_init(reference: *mut snd_refcount);
    pub fn snd_refcount_put(reference: *mut snd_refcount);
    pub fn snd_refcount_sync(reference: *mut snd_refcount);
}

#[inline]
pub unsafe fn snd_refcount_get(reference: *mut snd_refcount) {
    /* atomic_inc(&ref->count); */
    unsafe { atomic_inc(&mut (*reference).count) }
}

#[repr(C)]
pub struct snd_card {
    pub number: i32,
    pub id: [core::ffi::c_char; 16],
    pub driver: [core::ffi::c_char; 16],
    pub shortname: [core::ffi::c_char; 32],
    pub longname: [core::ffi::c_char; 80],
    pub irq_descr: [core::ffi::c_char; 32],
    pub mixername: [core::ffi::c_char; 80],
    pub components: *mut core::ffi::c_char,
    pub components_alloc_size: u32,
    pub module: *mut module,
    pub private_data: *mut core::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub devices: list_head,
    pub ctl_dev: *mut device,
    pub last_numid: u32,
    pub controls_rwsem: rw_semaphore,
    pub controls_rwlock: rwlock_t,
    pub controls_count: i32,
    pub user_ctl_alloc_size: usize,
    pub controls: list_head,
    pub ctl_files: list_head,
    pub proc_root: *mut snd_info_entry,
    pub proc_root_link: *mut proc_dir_entry,
    pub files_list: list_head,
    pub s_f_ops: *mut snd_shutdown_f_ops,
    pub files_lock: spinlock_t,
    pub shutdown: i32,
    pub release_completion: *mut completion,
    pub dev: *mut device,
    pub card_dev: device,
    pub dev_groups: [*const attribute_group; 4],
    pub registered: bool,
    pub managed: bool,
    pub releasing: bool,
    pub sync_irq: i32,
    pub remove_sleep: wait_queue_head_t,
    pub total_pcm_alloc_bytes: usize,
    pub memory_mutex: mutex,
    pub private_data_area: [u8; 0],
}

#[repr(C)]
pub struct snd_minor {
    pub type_: i32,
    pub card: i32,
    pub device: i32,
    pub f_ops: *const file_operations,
    pub private_data: *mut core::ffi::c_void,
    pub dev: *mut device,
    pub card_ptr: *mut snd_card,
}

extern "C" {
    pub static mut snd_major: i32;
    pub static mut snd_ecards_limit: i32;
    pub fn snd_request_card(card: i32);
    pub fn snd_device_alloc(dev_p: *mut *mut device, card: *mut snd_card) -> i32;
    pub fn snd_unregister_device(dev: *mut device) -> i32;
    pub fn snd_lookup_minor_data(minor: u32, type_: i32) -> *mut core::ffi::c_void;
    pub fn snd_minor_info_init() -> i32;
    pub fn copy_to_user_fromio(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: usize) -> i32;
    pub fn copy_from_user_toio(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: usize) -> i32;
    pub fn snd_card_locked(card: i32) -> i32;
    pub fn snd_card_new(parent: *mut device, idx: i32, xid: *const core::ffi::c_char,
        module: *mut module, extra_size: i32, card_ret: *mut *mut snd_card) -> i32;
    pub fn snd_card_register(card: *mut snd_card) -> i32;
    pub fn snd_card_disconnect(card: *mut snd_card);
    pub fn snd_card_disconnect_sync(card: *mut snd_card);
    pub fn snd_card_free(card: *mut snd_card);
    pub fn snd_card_set_id(card: *mut snd_card, id: *const core::ffi::c_char);
    pub fn snd_card_ref(card: i32) -> *mut snd_card;
    pub fn snd_device_new(card: *mut snd_card, type_: snd_device_type, device_data: *mut core::ffi::c_void, ops: *const snd_device_ops) -> i32;
    pub fn snd_device_register(card: *mut snd_card, device_data: *mut core::ffi::c_void) -> i32;
    pub fn snd_device_register_all(card: *mut snd_card) -> i32;
    pub fn snd_device_disconnect(card: *mut snd_card, device_data: *mut core::ffi::c_void);
    pub fn snd_device_disconnect_all(card: *mut snd_card);
    pub fn snd_device_free(card: *mut snd_card, device_data: *mut core::ffi::c_void);
    pub fn snd_device_free_all(card: *mut snd_card);
    pub fn release_and_free_resource(res: *mut resource);
}

#[inline]
pub unsafe fn snd_card_get_device_link(card: *mut snd_card) -> *mut device {
    if !card.is_null() { &mut (*card).card_dev } else { core::ptr::null_mut() }
}

#[repr(C)]
pub struct snd_pci_quirk {
    pub subvendor: u16,
    pub subdevice: u16,
    pub subdevice_mask: u16,
    pub value: i32,
}

pub const DMA_MODE_NO_ENABLE: u32 = 0x0100;
pub const SNDRV_OSS_VERSION: u32 = (3 << 16) | (8 << 8) | (1 << 4);

extern "C" {
    pub fn snd_fasync_helper(fd: i32, file: *mut file, on: i32, fasyncp: *mut *mut snd_fasync) -> i32;
    pub fn snd_kill_fasync(fasync: *mut snd_fasync, signal: i32, poll: i32);
    pub fn snd_fasync_free(fasync: *mut snd_fasync);
}

pub enum file {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
