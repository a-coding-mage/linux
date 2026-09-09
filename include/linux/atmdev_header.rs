/* SPDX-License-Identifier: GPL-2.0 */
/* atmdev.h - ATM device driver declarations and various related items */

/* Kernel include dependencies are supplied by other translated headers. */

#[cfg(feature = "CONFIG_PROC_FS")]
extern "C" {
    pub static mut atm_proc_root: *mut proc_dir_entry;
}

#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)]
pub struct compat_atm_iobuf {
    pub length: ::core::ffi::c_int,
    pub buffer: compat_uptr_t,
}

#[repr(C)]
pub struct k_atm_aal_stats {
    /* __AAL_STAT_ITEMS expands to atomic_t fields in the kernel build. */
}

#[repr(C)]
pub struct k_atm_dev_stats {
    pub aal0: k_atm_aal_stats,
    pub aal34: k_atm_aal_stats,
    pub aal5: k_atm_aal_stats,
}

#[repr(C)]
pub struct device;

pub const ATM_VF_ADDR: u32 = 0;
pub const ATM_VF_READY: u32 = 1;
pub const ATM_VF_PARTIAL: u32 = 2;
pub const ATM_VF_HASQOS: u32 = 3;
pub const ATM_VF_CLOSE: u32 = 4;
pub const ATM_DF_REMOVED: u32 = 0;

pub const ATM_PHY_SIG_LOST: i32 = 0;
pub const ATM_PHY_SIG_UNKNOWN: i32 = 1;
pub const ATM_PHY_SIG_FOUND: i32 = 2;
pub const ATM_ATMOPT_CLP: i32 = 1;

#[repr(C)]
pub struct atm_vcc {
    pub sk: sock,
    pub flags: ::core::ffi::c_ulong,
    pub vpi: i16,
    pub vci: ::core::ffi::c_int,
    pub aal_options: ::core::ffi::c_ulong,
    pub atm_options: ::core::ffi::c_ulong,
    pub dev: *mut atm_dev,
    pub qos: atm_qos,
    pub release_cb: Option<unsafe extern "C" fn(*mut atm_vcc)>,
    pub push: Option<unsafe extern "C" fn(*mut atm_vcc, *mut sk_buff)>,
    pub pop: Option<unsafe extern "C" fn(*mut atm_vcc, *mut sk_buff)>,
    pub send: Option<unsafe extern "C" fn(*mut atm_vcc, *mut sk_buff) -> ::core::ffi::c_int>,
    pub dev_data: *mut ::core::ffi::c_void,
    pub proto_data: *mut ::core::ffi::c_void,
    pub stats: *mut k_atm_aal_stats,
    pub owner: *mut module,
    pub user_back: *mut ::core::ffi::c_void,
}

#[inline]
pub unsafe fn atm_sk(sk: *mut sock) -> *mut atm_vcc { sk as *mut atm_vcc }

#[inline]
pub unsafe fn ATM_SD(sock: *mut socket) -> *mut atm_vcc { atm_sk((*sock).sk) }

#[inline]
pub unsafe fn sk_atm(vcc: *mut atm_vcc) -> *mut sock { vcc as *mut sock }

#[repr(C)]
pub struct atm_dev {
    pub ops: *const atmdev_ops,
    pub type_: *const ::core::ffi::c_char,
    pub number: ::core::ffi::c_int,
    pub dev_data: *mut ::core::ffi::c_void,
    pub phy_data: *mut ::core::ffi::c_void,
    pub flags: ::core::ffi::c_ulong,
    pub esi: [u8; ESI_LEN],
    pub ci_range: atm_cirange,
    pub stats: k_atm_dev_stats,
    pub signal: ::core::ffi::c_char,
    pub link_rate: ::core::ffi::c_int,
    pub refcnt: refcount_t,
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub proc_entry: *mut proc_dir_entry,
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub proc_name: *mut ::core::ffi::c_char,
    pub class_dev: device,
    pub dev_list: list_head,
}

#[repr(C)]
pub struct atmdev_ops {
    pub dev_close: Option<unsafe extern "C" fn(*mut atm_dev)>,
    pub open: Option<unsafe extern "C" fn(*mut atm_vcc) -> ::core::ffi::c_int>,
    pub close: Option<unsafe extern "C" fn(*mut atm_vcc)>,
    pub ioctl: Option<unsafe extern "C" fn(*mut atm_dev, u32, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    #[cfg(feature = "CONFIG_COMPAT")]
    pub compat_ioctl: Option<unsafe extern "C" fn(*mut atm_dev, u32, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub send: Option<unsafe extern "C" fn(*mut atm_vcc, *mut sk_buff) -> ::core::ffi::c_int>,
    pub proc_read: Option<unsafe extern "C" fn(*mut atm_dev, *mut loff_t, *mut ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub owner: *mut module,
}

#[repr(C, packed)]
pub struct atm_skb_data {
    pub vcc: *mut atm_vcc,
    pub atm_options: ::core::ffi::c_ulong,
    pub acct_truesize: u32,
}

pub const VCC_HTABLE_SIZE: usize = 32;

extern "C" {
    pub static mut vcc_hash: [hlist_head; VCC_HTABLE_SIZE];
    pub static mut vcc_sklist_lock: rwlock_t;
    pub fn atm_dev_register(type_: *const ::core::ffi::c_char, parent: *mut device, ops: *const atmdev_ops, number: ::core::ffi::c_int, flags: *mut ::core::ffi::c_ulong) -> *mut atm_dev;
    pub fn atm_dev_lookup(number: ::core::ffi::c_int) -> *mut atm_dev;
    pub fn atm_dev_deregister(dev: *mut atm_dev);
    pub fn atm_dev_signal_change(dev: *mut atm_dev, signal: ::core::ffi::c_char);
    pub fn vcc_insert_socket(sk: *mut sock);
    pub fn atm_dev_release_vccs(dev: *mut atm_dev);
    pub fn atm_charge(vcc: *mut atm_vcc, truesize: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn vcc_release_async(vcc: *mut atm_vcc, reply: ::core::ffi::c_int);
}

#[inline]
pub unsafe fn atm_account_tx(vcc: *mut atm_vcc, skb: *mut sk_buff) {
    refcount_add((*skb).truesize, &mut (*sk_atm(vcc)).sk_wmem_alloc);
    (*skb).cb_as_atm_skb_data().acct_truesize = (*skb).truesize;
    (*skb).cb_as_atm_skb_data().atm_options = (*vcc).atm_options;
}

#[inline]
pub unsafe fn atm_return_tx(vcc: *mut atm_vcc, skb: *mut sk_buff) {
    WARN_ON_ONCE(refcount_sub_and_test((*skb).cb_as_atm_skb_data().acct_truesize, &mut (*sk_atm(vcc)).sk_wmem_alloc));
}

#[inline]
pub unsafe fn atm_force_charge(vcc: *mut atm_vcc, truesize: ::core::ffi::c_int) { atomic_add(truesize, &mut (*sk_atm(vcc)).sk_rmem_alloc); }
#[inline]
pub unsafe fn atm_return(vcc: *mut atm_vcc, truesize: ::core::ffi::c_int) { atomic_sub(truesize, &mut (*sk_atm(vcc)).sk_rmem_alloc); }
#[inline]
pub unsafe fn atm_may_send(vcc: *mut atm_vcc, size: u32) -> ::core::ffi::c_int { ((size as usize + refcount_read(&(*sk_atm(vcc)).sk_wmem_alloc)) < (*sk_atm(vcc)).sk_sndbuf as usize) as ::core::ffi::c_int }
#[inline]
pub unsafe fn atm_dev_hold(dev: *mut atm_dev) { refcount_inc(&mut (*dev).refcnt); }
#[inline]
pub unsafe fn atm_dev_put(dev: *mut atm_dev) {
    if refcount_dec_and_test(&mut (*dev).refcnt) {
        BUG_ON(!test_bit(ATM_DF_REMOVED as usize, &(*dev).flags));
        if let Some(f) = (*dev).ops.as_ref().and_then(|ops| ops.dev_close) { f(dev); }
        put_device(&mut (*dev).class_dev);
    }
}

#[repr(C)]
pub struct atm_ioctl {
    pub owner: *mut module,
    pub ioctl: Option<unsafe extern "C" fn(*mut socket, u32, ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    pub list: list_head,
}

extern "C" {
    pub fn register_atm_ioctl(ioctl: *mut atm_ioctl);
    pub fn deregister_atm_ioctl(ioctl: *mut atm_ioctl);
    pub fn register_atmdevice_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn unregister_atmdevice_notifier(nb: *mut notifier_block);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
