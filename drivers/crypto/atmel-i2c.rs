// SPDX-License-Identifier: GPL-2.0
/* Microchip / Atmel ECC (I2C) driver. */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Types, constants, macros, and functions below are supplied by the Linux
// kernel and atmel-i2c.h dependencies of the original translation unit.

#[repr(C)]
struct ErrorEntry {
    value: u8,
    error_text: *const c_char,
}

static ERROR_LIST: &[ErrorEntry] = &[
    ErrorEntry { value: 0x01, error_text: b"CheckMac or Verify miscompare\0".as_ptr() as *const c_char },
    ErrorEntry { value: 0x03, error_text: b"Parse Error\0".as_ptr() as *const c_char },
    ErrorEntry { value: 0x05, error_text: b"ECC Fault\0".as_ptr() as *const c_char },
    ErrorEntry { value: 0x0F, error_text: b"Execution Error\0".as_ptr() as *const c_char },
    ErrorEntry { value: 0xEE, error_text: b"Watchdog about to expire\0".as_ptr() as *const c_char },
    ErrorEntry { value: 0xFF, error_text: b"CRC or other communication error\0".as_ptr() as *const c_char },
];

extern "C" {
    fn cpu_to_le16(value: u16) -> u16;
    fn bitrev16(value: u16) -> u16;
    fn crc16(seed: u16, data: *const u8, len: usize) -> u16;
    fn sg_copy_to_buffer(sgl: *mut scatterlist, nents: c_int, buf: *mut u8, len: usize) -> usize;
    fn sg_nents_for_len(sgl: *mut scatterlist, len: usize) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut atmel_i2c_client_priv;
    fn i2c_transfer_buffer_flags(client: *mut i2c_client, buf: *mut u8, len: usize, flags: u16) -> c_int;
    fn usleep_range(min: u32, max: u32);
    fn i2c_master_recv(client: *mut i2c_client, buf: *mut u8, len: usize) -> c_int;
    fn i2c_master_send(client: *mut i2c_client, buf: *const u8, len: usize) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn msleep(msecs: u32);
    fn init_work(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool;
    fn flush_workqueue(wq: *mut workqueue_struct);
    fn alloc_workqueue(name: *const c_char, flags: u32, max_active: c_int) -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn devm_kmalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn atomic_set(value: *mut atomic_t, number: c_int);
    fn i2c_check_functionality(adapter: *mut i2c_adapter, functionality: u32) -> bool;
    fn i2c_acpi_find_bus_speed(dev: *mut device) -> u32;
    fn device_property_read_u32(dev: *mut device, name: *const c_char, value: *mut u32) -> c_int;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut atmel_i2c_client_priv);
}

#[allow(non_camel_case_types)]
type __le16 = u16;

#[repr(C)] struct atmel_i2c_cmd { word_addr: u8, count: u8, opcode: u8, param1: u8, param2: __le16, data: [u8; 64], msecs: u32, rxsize: usize }
#[repr(C)] struct scatterlist { _opaque: [u8; 0] }
#[repr(C)] struct device { _opaque: [u8; 0] }
#[repr(C)] struct i2c_adapter { _opaque: [u8; 0] }
#[repr(C)] struct i2c_client { dev: device, adapter: *mut i2c_adapter }
#[repr(C)] struct mutex { _opaque: [u8; 0] }
#[repr(C)] struct atomic_t { counter: c_int }
#[repr(C)] struct work_struct { _opaque: [u8; 0] }
#[repr(C)] struct workqueue_struct { _opaque: [u8; 0] }
#[repr(C)] struct atmel_i2c_client_priv { client: *mut i2c_client, lock: mutex, wake_token: [u8; 8], wake_token_sz: usize, tfm_count: atomic_t }
#[repr(C)] struct atmel_i2c_work_data { work: work_struct, cmd: atmel_i2c_cmd, client: *mut i2c_client, cbk: Option<unsafe extern "C" fn(*mut atmel_i2c_work_data, *mut c_void, c_int)>, areq: *mut c_void }

unsafe fn atmel_i2c_checksum(cmd: *mut atmel_i2c_cmd) {
    let data = &mut (*cmd).count as *mut u8;
    let len = (*cmd).count as usize - CRC_SIZE;
    let crc16_ptr = data.add(len) as *mut __le16;
    *crc16_ptr = cpu_to_le16(bitrev16(crc16(0, data, len)));
}

#[no_mangle] pub unsafe extern "C" fn atmel_i2c_init_read_config_cmd(cmd: *mut atmel_i2c_cmd) { (*cmd).word_addr=COMMAND; (*cmd).opcode=OPCODE_READ; (*cmd).param1=CONFIGURATION_ZONE; (*cmd).param2=cpu_to_le16(DEVICE_LOCK_ADDR); (*cmd).count=READ_COUNT; atmel_i2c_checksum(cmd); (*cmd).msecs=MAX_EXEC_TIME_READ; (*cmd).rxsize=READ_RSP_SIZE; }
#[no_mangle] pub unsafe extern "C" fn atmel_i2c_init_read_otp_cmd(cmd: *mut atmel_i2c_cmd, addr: u16) -> c_int { if addr >= OTP_ZONE_SIZE / 4 { return -EINVAL; } (*cmd).word_addr=COMMAND; (*cmd).opcode=OPCODE_READ; (*cmd).param1=OTP_ZONE; (*cmd).param2=cpu_to_le16(addr); (*cmd).count=READ_COUNT; atmel_i2c_checksum(cmd); (*cmd).msecs=MAX_EXEC_TIME_READ; (*cmd).rxsize=READ_RSP_SIZE; 0 }
#[no_mangle] pub unsafe extern "C" fn atmel_i2c_init_random_cmd(cmd: *mut atmel_i2c_cmd) { (*cmd).word_addr=COMMAND; (*cmd).opcode=OPCODE_RANDOM; (*cmd).param1=0; (*cmd).param2=0; (*cmd).count=RANDOM_COUNT; atmel_i2c_checksum(cmd); (*cmd).msecs=MAX_EXEC_TIME_RANDOM; (*cmd).rxsize=RANDOM_RSP_SIZE; }
#[no_mangle] pub unsafe extern "C" fn atmel_i2c_init_genkey_cmd(cmd: *mut atmel_i2c_cmd, keyid: u16) { (*cmd).word_addr=COMMAND; (*cmd).count=GENKEY_COUNT; (*cmd).opcode=OPCODE_GENKEY; (*cmd).param1=GENKEY_MODE_PRIVATE; (*cmd).param2=cpu_to_le16(keyid); atmel_i2c_checksum(cmd); (*cmd).msecs=MAX_EXEC_TIME_GENKEY; (*cmd).rxsize=GENKEY_RSP_SIZE; }
#[no_mangle] pub unsafe extern "C" fn atmel_i2c_init_ecdh_cmd(cmd: *mut atmel_i2c_cmd, pubkey: *mut scatterlist) -> c_int { (*cmd).word_addr=COMMAND; (*cmd).count=ECDH_COUNT; (*cmd).opcode=OPCODE_ECDH; (*cmd).param1=ECDH_PREFIX_MODE; (*cmd).param2=cpu_to_le16(DATA_SLOT_2); let copied=sg_copy_to_buffer(pubkey, sg_nents_for_len(pubkey, ATMEL_ECC_PUBKEY_SIZE), (*cmd).data.as_mut_ptr(), ATMEL_ECC_PUBKEY_SIZE); if copied != ATMEL_ECC_PUBKEY_SIZE { return -EINVAL; } atmel_i2c_checksum(cmd); (*cmd).msecs=MAX_EXEC_TIME_ECDH; (*cmd).rxsize=ECDH_RSP_SIZE; 0 }

unsafe fn atmel_i2c_status(dev: *mut device, status: *mut u8) -> c_int { if *status != STATUS_SIZE { return 0; } let err_id=*status.add(1); if err_id==STATUS_WAKE_SUCCESSFUL || err_id==STATUS_NOERR { return 0; } for e in ERROR_LIST { if e.value==err_id { dev_err(dev, b"%02x: %s:\n\0".as_ptr() as _, err_id, e.error_text); return err_id as c_int; } } 0 }

unsafe fn atmel_i2c_wakeup(client: *mut i2c_client) -> c_int { let p=i2c_get_clientdata(client); let mut status=[0u8; STATUS_RSP_SIZE]; i2c_transfer_buffer_flags(client, (*p).wake_token.as_mut_ptr(), (*p).wake_token_sz, I2C_M_IGNORE_NAK); usleep_range(TWHI_MIN,TWHI_MAX); let ret=i2c_master_recv(client,status.as_mut_ptr(),STATUS_SIZE); if ret<0 { return ret; } atmel_i2c_status(&mut (*client).dev,status.as_mut_ptr()) }
unsafe fn atmel_i2c_sleep(client: *mut i2c_client) -> c_int { let mut sleep=SLEEP_TOKEN; i2c_master_send(client,&mut sleep,1) }

#[no_mangle] pub unsafe extern "C" fn atmel_i2c_send_receive(client:*mut i2c_client,cmd:*mut atmel_i2c_cmd)->c_int { let p=i2c_get_clientdata(client); mutex_lock(&mut (*p).lock); let mut ret=atmel_i2c_wakeup(client); if ret!=0 { mutex_unlock(&mut (*p).lock); return ret; } ret=i2c_master_send(client,cmd as *mut u8,(*cmd).count as usize+WORD_ADDR_SIZE); if ret<0 { mutex_unlock(&mut (*p).lock); return ret; } msleep((*cmd).msecs); ret=i2c_master_recv(client,(*cmd).data.as_mut_ptr(),(*cmd).rxsize); if ret<0 { mutex_unlock(&mut (*p).lock); return ret; } ret=atmel_i2c_sleep(client); if ret<0 { mutex_unlock(&mut (*p).lock); return ret; } mutex_unlock(&mut (*p).lock); atmel_i2c_status(&mut (*client).dev,(*cmd).data.as_mut_ptr()) }

static mut ATMEL_WQ:*mut workqueue_struct=core::ptr::null_mut();
#[no_mangle] pub unsafe extern "C" fn atmel_i2c_enqueue(work_data:*mut atmel_i2c_work_data,cbk:Option<unsafe extern "C" fn(*mut atmel_i2c_work_data,*mut c_void,c_int)>,areq:*mut c_void){(*work_data).cbk=cbk;(*work_data).areq=areq;init_work(&mut (*work_data).work,atmel_i2c_work_handler);queue_work(ATMEL_WQ,&mut (*work_data).work);}
unsafe extern "C" fn atmel_i2c_work_handler(work:*mut work_struct) { let work_data=work as *mut atmel_i2c_work_data; let status=atmel_i2c_send_receive((*work_data).client,&mut (*work_data).cmd); if let Some(cbk)=(*work_data).cbk { cbk(work_data,(*work_data).areq,status); } }
#[no_mangle] pub unsafe extern "C" fn atmel_i2c_flush_queue(){flush_workqueue(ATMEL_WQ);}

unsafe fn atmel_i2c_wake_token_sz(bus_clk_rate:u32)->usize { let no_of_bits=((TWLO_USEC as u64*bus_clk_rate as u64 + USEC_PER_SEC as u64-1)/USEC_PER_SEC as u64) as usize; (no_of_bits+7)/8 }
unsafe fn device_sanity_check(client:*mut i2c_client)->c_int { let cmd=kmalloc(core::mem::size_of::<atmel_i2c_cmd>(),GFP_KERNEL) as *mut atmel_i2c_cmd; if cmd.is_null(){return -ENOMEM;} atmel_i2c_init_read_config_cmd(cmd); let mut ret=atmel_i2c_send_receive(client,cmd); if ret==0 && ((*cmd).data[LOCK_CONFIG_IDX]!=0 || (*cmd).data[LOCK_VALUE_IDX]!=0) { dev_err(&mut (*client).dev,b"Configuration or Data and OTP zones are unlocked!\n\0".as_ptr() as _); ret=-ENOTSUPP; } kfree(cmd as *mut c_void); ret }
#[no_mangle] pub unsafe extern "C" fn atmel_i2c_probe(client:*mut i2c_client)->c_int { let dev=&mut (*client).dev; if !i2c_check_functionality((*client).adapter,I2C_FUNC_I2C){dev_err(dev,b"I2C_FUNC_I2C not supported\n\0".as_ptr() as _);return -ENODEV;} let mut rate=i2c_acpi_find_bus_speed(&mut (*(*client).adapter).device); if rate==0 { let ret=device_property_read_u32(&mut (*(*client).adapter).device,b"clock-frequency\0".as_ptr() as _,&mut rate); if ret!=0{return ret;} } if rate>I2C_MAX_FAST_MODE_PLUS_FREQ{return -EINVAL;} let p=devm_kmalloc(dev,core::mem::size_of::<atmel_i2c_client_priv>(),GFP_KERNEL) as *mut atmel_i2c_client_priv; if p.is_null(){return -ENOMEM;} (*p).client=client;mutex_init(&mut (*p).lock);(*p).wake_token_sz=atmel_i2c_wake_token_sz(rate);(*p).wake_token=[0;8];atomic_set(&mut (*p).tfm_count,0);i2c_set_clientdata(client,p);device_sanity_check(client) }
unsafe extern "C" fn atmel_i2c_init()->c_int { ATMEL_WQ=alloc_workqueue(b"atmel_wq\0".as_ptr() as _,WQ_PERCPU,0);if ATMEL_WQ.is_null(){-ENOMEM}else{0} }
unsafe extern "C" fn atmel_i2c_exit(){destroy_workqueue(ATMEL_WQ);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
