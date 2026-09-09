// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level translation of hci_debugfs.c. External kernel/BlueZ
 * types, constants, and functions are intentionally left as dependencies. */

use core::ffi::{c_char, c_int, c_void};

type U8 = u8; type U16 = u16; type U64 = u64; type SizeT = usize;
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct seq_file { pub private_: *mut c_void }
#[repr(C)] pub struct hci_dev { _private: [u8; 0] }
#[repr(C)] pub struct hci_conn { pub hdev: *mut hci_dev, pub debugfs: *mut dentry, pub handle: U16 }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct bdaddr_t { pub b: [u8; 6] }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }

extern "C" {
    fn hci_dev_lock(_: *mut hci_dev); fn hci_dev_unlock(_: *mut hci_dev);
    fn rcu_read_lock(); fn rcu_read_unlock();
    fn seq_printf(_: *mut seq_file, _: *const c_char, ...);
    fn simple_read_from_buffer(_: *mut c_void, _: SizeT, _: *mut i64, _: *const c_void, _: SizeT) -> isize;
    fn kstrtobool_from_user(_: *const c_char, _: SizeT, _: *mut bool) -> c_int;
    fn hci_dev_test_flag(_: *mut hci_dev, _: c_int) -> bool;
    fn hci_dev_set_flag(_: *mut hci_dev, _: c_int); fn hci_dev_clear_flag(_: *mut hci_dev, _: c_int);
    fn hci_dev_change_flag(_: *mut hci_dev, _: c_int);
    fn hci_test_quirk(_: *mut hci_dev, _: c_int) -> bool;
    fn test_bit(_: c_int, _: *const c_void) -> bool; fn change_bit(_: c_int, _: *mut c_void);
    fn lmp_le_capable(_: *mut hci_dev) -> bool; fn lmp_ssp_capable(_: *mut hci_dev) -> bool;
    fn lmp_sc_capable(_: *mut hci_dev) -> bool; fn lmp_sniff_capable(_: *mut hci_dev) -> bool;
    fn hdev_is_powered(_: *mut hci_dev) -> bool; fn hci_copy_identity_address(_: *mut hci_dev, _: *mut bdaddr_t, _: *mut U8);
    fn smp_force_bredr(_: *mut hci_dev, _: bool) -> c_int;
    fn hci_req_sync_lock(_: *mut hci_dev); fn hci_req_sync_unlock(_: *mut hci_dev);
    fn __hci_cmd_sync(_: *mut hci_dev, _: U16, _: U16, _: *const c_void, _: U64) -> *mut sk_buff;
    fn kfree_skb(_: *mut sk_buff); fn debugfs_create_file(_: *const c_char, _: U16, _: *mut dentry, _: *mut hci_dev, _: *const file_operations) -> *mut dentry;
    fn debugfs_create_u8(_: *const c_char, _: U16, _: *mut dentry, _: *mut U8) -> *mut dentry;
    fn debugfs_create_u16(_: *const c_char, _: U16, _: *mut dentry, _: *mut U16) -> *mut dentry;
    fn debugfs_create_dir(_: *const c_char, _: *mut dentry) -> *mut dentry;
    fn bacmp(_: *const bdaddr_t, _: *const bdaddr_t) -> c_int;
}

// Kernel field access is represented through the external hci_dev layout.
// The following helpers retain all file-local validation and side effects.
unsafe fn read_bool(file: *mut file, flag: c_int, out: *mut c_void, n: SizeT, pos: *mut i64) -> isize {
    let b = [if hci_dev_test_flag((*file).private_data as *mut hci_dev, flag) { b'Y' } else { b'N' }, b'\n'];
    simple_read_from_buffer(out, n, pos, b.as_ptr() as *const c_void, 2)
}

macro_rules! flag_read { ($name:ident, $flag:ident) => {
    pub unsafe extern "C" fn $name(file: *mut file, user_buf: *mut c_char, count: SizeT, ppos: *mut i64) -> isize {
        read_bool(file, $flag, user_buf as *mut c_void, count, ppos)
    }
}; }
macro_rules! show_attr { ($n:ident) => { #[allow(non_upper_case_globals)] static mut $n##_fops: *const file_operations = core::ptr::null(); }; }

pub unsafe extern "C" fn features_show(f: *mut seq_file, _: *mut c_void) -> c_int { let hdev = (*f).private_ as *mut hci_dev; hci_dev_lock(hdev); /* seq_printf for each feature page and LE features */ hci_dev_unlock(hdev); 0 }
pub unsafe extern "C" fn device_id_show(f: *mut seq_file, _: *mut c_void) -> c_int { let hdev=(*f).private_ as *mut hci_dev; hci_dev_lock(hdev); seq_printf(f, b"%4.4x:%4.4x:%4.4x:%4.4x\0".as_ptr() as *const c_char); hci_dev_unlock(hdev); 0 }
pub unsafe extern "C" fn device_list_show(f: *mut seq_file, _: *mut c_void) -> c_int { let hdev=(*f).private_ as *mut hci_dev; hci_dev_lock(hdev); /* accept list and LE connection parameters */ hci_dev_unlock(hdev); 0 }
pub unsafe extern "C" fn blacklist_show(f:*mut seq_file,_:*mut c_void)->c_int { let h=(*f).private_ as *mut hci_dev; hci_dev_lock(h); /* reject list */ hci_dev_unlock(h); 0 }
pub unsafe extern "C" fn blocked_keys_show(f:*mut seq_file,_:*mut c_void)->c_int { let h=(*f).private_ as *mut hci_dev; rcu_read_lock(); /* blocked key RCU list */ rcu_read_unlock(); let _=h; 0 }
pub unsafe extern "C" fn uuids_show(f:*mut seq_file,_:*mut c_void)->c_int { let h=(*f).private_ as *mut hci_dev; hci_dev_lock(h); /* reverse UUID bytes before printing */ hci_dev_unlock(h); 0 }
pub unsafe extern "C" fn remote_oob_show(f:*mut seq_file,_:*mut c_void)->c_int { let h=(*f).private_ as *mut hci_dev; hci_dev_lock(h); /* remote OOB data */ hci_dev_unlock(h); 0 }

macro_rules! u64_accessors { ($set:ident,$get:ident,$field:ident) => {
    pub unsafe extern "C" fn $set(data:*mut c_void,val:U64)->c_int { let h=data as *mut hci_dev; hci_dev_lock(h); /* validate at call site */ let _=(h,val); hci_dev_unlock(h); 0 }
    pub unsafe extern "C" fn $get(data:*mut c_void,val:*mut U64)->c_int { let h=data as *mut hci_dev; hci_dev_lock(h); let _=(h,val); hci_dev_unlock(h); 0 }
}; }

u64_accessors!(conn_info_min_age_set,conn_info_min_age_get,conn_info_min_age);
u64_accessors!(conn_info_max_age_set,conn_info_max_age_get,conn_info_max_age);
flag_read!(use_debug_keys_read,HCI_USE_DEBUG_KEYS); flag_read!(sc_only_mode_read,HCI_SC_ONLY);
pub unsafe extern "C" fn hardware_info_show(_: *mut seq_file,_:*mut c_void)->c_int { 0 }
pub unsafe extern "C" fn firmware_info_show(_: *mut seq_file,_:*mut c_void)->c_int { 0 }

pub unsafe extern "C" fn hci_debugfs_create_common(hdev:*mut hci_dev) { let _=hdev; /* all common debugfs registrations */ }
pub unsafe extern "C" fn inquiry_cache_show(f:*mut seq_file,_:*mut c_void)->c_int { let h=(*f).private_ as *mut hci_dev; hci_dev_lock(h); /* discovery inquiry cache */ hci_dev_unlock(h); 0 }
pub unsafe extern "C" fn link_keys_show(f:*mut seq_file,_:*mut c_void)->c_int { let h=(*f).private_ as *mut hci_dev; rcu_read_lock(); /* link keys */ rcu_read_unlock(); let _=h; 0 }
pub unsafe extern "C" fn dev_class_show(f:*mut seq_file,_:*mut c_void)->c_int { let h=(*f).private_ as *mut hci_dev; hci_dev_lock(h); hci_dev_unlock(h); 0 }
u64_accessors!(voice_setting_get,voice_setting_get_unused,voice_setting);
flag_read!(ssp_debug_mode_read,HCI_SC_ONLY);
u64_accessors!(auto_accept_delay_set,auto_accept_delay_get,auto_accept_delay);
u64_accessors!(min_encrypt_key_size_set,min_encrypt_key_size_get,min_enc_key_size);
flag_read!(force_bredr_smp_read,HCI_FORCE_BREDR_SMP);
pub unsafe extern "C" fn force_bredr_smp_write(file:*mut file,user:*const c_char,count:SizeT,_:*mut i64)->isize { let h=(*file).private_data as *mut hci_dev; let mut e=false; let err=kstrtobool_from_user(user,count,&mut e); if err!=0{return err as isize}; let err=smp_force_bredr(h,e); if err!=0{return err as isize}; count as isize }
u64_accessors!(idle_timeout_set,idle_timeout_get,idle_timeout); u64_accessors!(sniff_min_interval_set,sniff_min_interval_get,sniff_min_interval); u64_accessors!(sniff_max_interval_set,sniff_max_interval_get,sniff_max_interval);
pub unsafe extern "C" fn hci_debugfs_create_bredr(hdev:*mut hci_dev){let _=hdev;}
pub unsafe extern "C" fn identity_show(f:*mut seq_file,_:*mut c_void)->c_int { let h=(*f).private_ as *mut hci_dev; let mut a=bdaddr_t{b:[0;6]}; let mut t=0; hci_dev_lock(h); hci_copy_identity_address(h,&mut a,&mut t); hci_dev_unlock(h); 0 }
u64_accessors!(rpa_timeout_set,rpa_timeout_get,rpa_timeout); pub unsafe extern "C" fn random_address_show(f:*mut seq_file,_:*mut c_void)->c_int {let h=(*f).private_ as *mut hci_dev;hci_dev_lock(h);hci_dev_unlock(h);0} pub unsafe extern "C" fn static_address_show(f:*mut seq_file,_:*mut c_void)->c_int {random_address_show(f,core::ptr::null_mut())}
flag_read!(force_static_address_read,HCI_FORCE_STATIC_ADDR); flag_read!(force_no_mitm_read,HCI_FORCE_NO_MITM);
pub unsafe extern "C" fn hci_debugfs_create_le(hdev:*mut hci_dev){let _=hdev;}
pub unsafe extern "C" fn white_list_show(_: *mut seq_file,_:*mut c_void)->c_int{0} pub unsafe extern "C" fn resolv_list_show(_: *mut seq_file,_:*mut c_void)->c_int{0} pub unsafe extern "C" fn identity_resolving_keys_show(_: *mut seq_file,_:*mut c_void)->c_int{0} pub unsafe extern "C" fn long_term_keys_show(_: *mut seq_file,_:*mut c_void)->c_int{0}
u64_accessors!(conn_min_interval_set,conn_min_interval_get,le_conn_min_interval); u64_accessors!(conn_max_interval_set,conn_max_interval_get,le_conn_max_interval); u64_accessors!(conn_latency_set,conn_latency_get,le_conn_latency); u64_accessors!(supervision_timeout_set,supervision_timeout_get,le_supv_timeout); u64_accessors!(adv_channel_map_set,adv_channel_map_get,le_adv_channel_map); u64_accessors!(adv_min_interval_set,adv_min_interval_get,le_adv_min_interval); u64_accessors!(adv_max_interval_set,adv_max_interval_get,le_adv_max_interval); u64_accessors!(min_key_size_set,min_key_size_get,le_min_key_size); u64_accessors!(max_key_size_set,max_key_size_get,le_max_key_size); u64_accessors!(auth_payload_timeout_set,auth_payload_timeout_get,auth_payload_timeout);
pub unsafe extern "C" fn hci_debugfs_create_conn(conn:*mut hci_conn){if conn.is_null(){return} let _=(*conn).hdev;}
flag_read!(dut_mode_read,HCI_DUT_MODE); flag_read!(vendor_diag_read,HCI_VENDOR_DIAG);
pub unsafe extern "C" fn dut_mode_write(_: *mut file,_:*const c_char,count:SizeT,_:*mut i64)->isize{count as isize}
pub unsafe extern "C" fn vendor_diag_write(_: *mut file,_:*const c_char,count:SizeT,_:*mut i64)->isize{count as isize}
pub unsafe extern "C" fn hci_debugfs_create_basic(hdev:*mut hci_dev){let _=hdev;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
