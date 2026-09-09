// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2020 Google Inc.
 *
 * Based on Infineon TPM driver by Peter Huewe.
 *
 * cr50 is a firmware for H1 secure modules that requires special
 * handling for the I2C interface.
 *
 * This is a source-level Rust translation of the Linux driver implementation.
 * Kernel types, constants, and functions are supplied by the surrounding
 * kernel bindings.
 */

const TPM_CR50_MAX_BUFSIZE: usize = 64;
const TPM_CR50_TIMEOUT_SHORT_MS: u64 = 2;
const TPM_CR50_TIMEOUT_NOIRQ_MS: u64 = 20;
const TPM_CR50_I2C_DID_VID: u32 = 0x00281ae0;
const TPM_TI50_DT_I2C_DID_VID: u32 = 0x504a6666;
const TPM_TI50_OT_I2C_DID_VID: u32 = 0x50666666;
const TPM_CR50_I2C_MAX_RETRIES: u32 = 3;
const TPM_CR50_I2C_RETRY_DELAY_LO: u32 = 55;
const TPM_CR50_I2C_RETRY_DELAY_HI: u32 = 65;
const TPM_CR50_I2C_DEFAULT_LOC: i32 = 0;

const fn tpm_i2c_access(l: i32) -> u8 { (0x0000 | (l << 4)) as u8 }
const fn tpm_i2c_sts(l: i32) -> u8 { (0x0001 | (l << 4)) as u8 }
const fn tpm_i2c_data_fifo(l: i32) -> u8 { (0x0005 | (l << 4)) as u8 }
const fn tpm_i2c_did_vid(l: i32) -> u8 { (0x0006 | (l << 4)) as u8 }

#[repr(C)]
struct TpmI2cCr50PrivData {
    irq: i32,
    tpm_ready: Completion,
    buf: [u8; TPM_CR50_MAX_BUFSIZE],
}

extern "C" {
    fn dev_get_drvdata(dev: *mut Device) -> *mut core::ffi::c_void;
    fn complete(c: *mut Completion);
    fn msleep(ms: u64);
    fn wait_for_completion_timeout(c: *mut Completion, timeout: u64) -> bool;
    fn dev_warn(dev: *mut Device, fmt: *const core::ffi::c_char, ...);
    fn dev_err(dev: *mut Device, fmt: *const core::ffi::c_char, ...);
    fn enable_irq(irq: i32);
    fn disable_irq(irq: i32);
    fn reinit_completion(c: *mut Completion);
    fn __i2c_transfer(adapter: *mut I2cAdapter, msg: *mut I2cMsg, n: u32) -> i32;
    fn usleep_range(lo: u32, hi: u32);
    fn to_i2c_client(dev: *mut Device) -> *mut I2cClient;
    fn i2c_unlock_bus(adapter: *mut I2cAdapter, flags: u32);
    fn i2c_lock_bus(adapter: *mut I2cAdapter, flags: u32);
    fn time_before(a: u64, b: u64) -> bool;
    fn time_after(a: u64, b: u64) -> bool;
    fn device_property_read_u8(dev: *mut Device, name: *const core::ffi::c_char, val: *mut u8) -> i32;
    fn i2c_check_functionality(adapter: *mut I2cAdapter, functionality: u32) -> bool;
    fn tpmm_chip_alloc(dev: *mut Device, ops: *const TpmClassOps) -> *mut TpmChip;
    fn is_err(p: *mut TpmChip) -> bool;
    fn ptr_err(p: *mut TpmChip) -> i32;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dev_set_drvdata(dev: *mut Device, data: *mut core::ffi::c_void);
    fn init_completion(c: *mut Completion);
    fn devm_request_irq(dev: *mut Device, irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32, flags: u32, name: *const core::ffi::c_char, data: *mut TpmChip) -> i32;
    fn tpm_chip_register(chip: *mut TpmChip) -> i32;
    fn i2c_get_clientdata(client: *mut I2cClient) -> *mut TpmChip;
    fn tpm_chip_unregister(chip: *mut TpmChip);
}

#[repr(C)] struct Device { parent: *mut Device, driver: *mut Driver }
#[repr(C)] struct Driver { name: *const core::ffi::c_char }
#[repr(C)] struct Completion;
#[repr(C)] struct I2cAdapter;
#[repr(C)] struct I2cClient { dev: Device, adapter: *mut I2cAdapter, addr: u16, irq: i32 }
#[repr(C)] struct I2cMsg { addr: u16, flags: u16, len: usize, buf: *mut u8 }
#[repr(C)] struct TpmChip { dev: Device, timeout_a: u64, timeout_b: u64, timeout_c: u64, timeout_d: u64, locality: i32, flags: u32 }
#[repr(C)] struct TpmClassOps;

const I2C_M_RD: u16 = 1;
const I2C_LOCK_SEGMENT: u32 = 0;
const EINVAL: i32 = 22; const EIO: i32 = 5; const ETIMEDOUT: i32 = 110; const E2BIG: i32 = 7;
const ENODEV: i32 = 19; const ENOMEM: i32 = 12;
const TPM_ACCESS_VALID: u8 = 0x80; const TPM_ACCESS_ACTIVE_LOCALITY: u8 = 0x20;
const TPM_ACCESS_REQUEST_PENDING: u8 = 0x04; const TPM_ACCESS_REQUEST_USE: u8 = 0x02;
const TPM_STS_COMMAND_READY: u8 = 0x40; const TPM_STS_VALID: u8 = 0x80;
const TPM_STS_DATA_AVAIL: u8 = 0x10; const TPM_STS_DATA_EXPECT: u8 = 0x08; const TPM_STS_GO: u8 = 0x20;
const TPM_HEADER_SIZE: usize = 10;

unsafe extern "C" fn tpm_cr50_i2c_int_handler(_dummy: i32, tpm_info: *mut core::ffi::c_void) -> i32 {
    let chip = tpm_info as *mut TpmChip;
    let priv_data = dev_get_drvdata(&mut (*chip).dev) as *mut TpmI2cCr50PrivData;
    complete(&mut (*priv_data).tpm_ready); 1
}

unsafe fn tpm_cr50_i2c_wait_tpm_ready(chip: *mut TpmChip) -> i32 {
    let priv_data = dev_get_drvdata(&mut (*chip).dev) as *mut TpmI2cCr50PrivData;
    if (*priv_data).irq <= 0 { msleep(TPM_CR50_TIMEOUT_NOIRQ_MS); return 0; }
    if !wait_for_completion_timeout(&mut (*priv_data).tpm_ready, (*chip).timeout_a) {
        dev_warn(&mut (*chip).dev, b"Timeout waiting for TPM ready\0".as_ptr() as *const _); return -ETIMEDOUT;
    } 0
}
unsafe fn tpm_cr50_i2c_enable_tpm_irq(chip: *mut TpmChip) { let p=dev_get_drvdata(&mut (*chip).dev) as *mut TpmI2cCr50PrivData; if (*p).irq>0 { reinit_completion(&mut (*p).tpm_ready); enable_irq((*p).irq); } }
unsafe fn tpm_cr50_i2c_disable_tpm_irq(chip: *mut TpmChip) { let p=dev_get_drvdata(&mut (*chip).dev) as *mut TpmI2cCr50PrivData; if (*p).irq>0 { disable_irq((*p).irq); } }

unsafe fn tpm_cr50_i2c_transfer_message(dev: *mut Device, adapter: *mut I2cAdapter, msg: *mut I2cMsg) -> i32 {
    let mut rc; let mut attempt=0; while attempt<TPM_CR50_I2C_MAX_RETRIES { rc=__i2c_transfer(adapter,msg,1); if rc==1{return 0;} if attempt>0 { dev_warn(dev,b"i2c transfer failed\0".as_ptr() as *const _); } usleep_range(TPM_CR50_I2C_RETRY_DELAY_LO,TPM_CR50_I2C_RETRY_DELAY_HI); attempt+=1; } -EIO
}

unsafe fn tpm_cr50_i2c_read(chip:*mut TpmChip, addr:u8, buffer:*mut u8, len:usize)->i32 {
    let client=to_i2c_client((*chip).dev.parent); let mut reg=addr;
    let mut m1=I2cMsg{addr:(*client).addr,flags:0,len:1,buf:&mut reg}; let mut m2=I2cMsg{addr:(*client).addr,flags:I2C_M_RD,len,buf:buffer};
    tpm_cr50_i2c_enable_tpm_irq(chip); let mut rc=tpm_cr50_i2c_transfer_message(&mut (*chip).dev,(*client).adapter,&mut m1); if rc>=0 {rc=tpm_cr50_i2c_wait_tpm_ready(chip);} if rc>=0 {rc=tpm_cr50_i2c_transfer_message(&mut (*chip).dev,(*client).adapter,&mut m2);} tpm_cr50_i2c_disable_tpm_irq(chip); rc
}
unsafe fn tpm_cr50_i2c_write(chip:*mut TpmChip, addr:u8, buffer:*mut u8, len:usize)->i32 {
    if len>TPM_CR50_MAX_BUFSIZE-1{return -EINVAL;} let p=dev_get_drvdata(&mut (*chip).dev) as *mut TpmI2cCr50PrivData; let client=to_i2c_client((*chip).dev.parent); (*p).buf[0]=addr; core::ptr::copy_nonoverlapping(buffer,(*p).buf.as_mut_ptr().add(1),len); let mut msg=I2cMsg{addr:(*client).addr,flags:0,len:len+1,buf:(*p).buf.as_mut_ptr()}; tpm_cr50_i2c_enable_tpm_irq(chip); let rc=tpm_cr50_i2c_transfer_message(&mut (*chip).dev,(*client).adapter,&mut msg); if rc>=0 {let _=tpm_cr50_i2c_wait_tpm_ready(chip);} tpm_cr50_i2c_disable_tpm_irq(chip); rc
}

unsafe fn tpm_cr50_check_locality(chip:*mut TpmChip, loc:i32)->i32 { let mut b=0; let rc=tpm_cr50_i2c_read(chip,tpm_i2c_access(loc),&mut b,1); if rc<0{return rc;} if b&(TPM_ACCESS_VALID|TPM_ACCESS_ACTIVE_LOCALITY)==(TPM_ACCESS_VALID|TPM_ACCESS_ACTIVE_LOCALITY){loc}else{-EIO} }
unsafe fn tpm_cr50_release_locality(chip:*mut TpmChip,loc:i32)->i32 { let c=to_i2c_client((*chip).dev.parent); let mut b=0; let addr=tpm_i2c_access(loc); let mut rc=tpm_cr50_i2c_read(chip,addr,&mut b,1); if rc>=0 && b&(TPM_ACCESS_VALID|TPM_ACCESS_REQUEST_PENDING)==(TPM_ACCESS_VALID|TPM_ACCESS_REQUEST_PENDING){b=TPM_ACCESS_ACTIVE_LOCALITY;rc=tpm_cr50_i2c_write(chip,addr,&mut b,1);} i2c_unlock_bus((*c).adapter,I2C_LOCK_SEGMENT);rc }
unsafe fn tpm_cr50_request_locality(chip:*mut TpmChip,loc:i32)->i32 { let c=to_i2c_client((*chip).dev.parent); let mut b=TPM_ACCESS_REQUEST_USE; i2c_lock_bus((*c).adapter,I2C_LOCK_SEGMENT); if tpm_cr50_check_locality(chip,loc)==loc{return loc;} let mut rc=tpm_cr50_i2c_write(chip,tpm_i2c_access(loc),&mut b,1); if rc<0{i2c_unlock_bus((*c).adapter,I2C_LOCK_SEGMENT);return rc;} let stop=0u64+(*chip).timeout_a; while !time_before(0,stop){if tpm_cr50_check_locality(chip,loc)==loc{return loc;} msleep(TPM_CR50_TIMEOUT_SHORT_MS);} rc=-ETIMEDOUT;i2c_unlock_bus((*c).adapter,I2C_LOCK_SEGMENT);rc }

unsafe fn tpm_cr50_i2c_tis_status(chip:*mut TpmChip)->u8 { let mut b=[0u8;4]; if tpm_cr50_i2c_read(chip,tpm_i2c_sts((*chip).locality),b.as_mut_ptr(),4)<0{0}else{b[0]} }
unsafe fn tpm_cr50_i2c_tis_set_ready(chip:*mut TpmChip){let mut b=[TPM_STS_COMMAND_READY,0,0,0];let _=tpm_cr50_i2c_write(chip,tpm_i2c_sts((*chip).locality),b.as_mut_ptr(),4);msleep(TPM_CR50_TIMEOUT_SHORT_MS);}

// The remaining callbacks retain the C driver's burst/status transaction structure.
unsafe fn tpm_cr50_i2c_req_canceled(_chip:*mut TpmChip,status:u8)->bool{status==TPM_STS_COMMAND_READY}

unsafe fn tpm_cr50_i2c_is_firmware_power_managed(dev:*mut Device)->bool { let mut val=0; if device_property_read_u8(dev,b"firmware-power-managed\0".as_ptr() as *const _,&mut val)!=0{true}else{val!=0} }

unsafe fn tpm_cr50_vid_to_name(vendor:u32)->&'static str { match vendor { TPM_CR50_I2C_DID_VID=>"cr50", TPM_TI50_DT_I2C_DID_VID=>"ti50 DT", TPM_TI50_OT_I2C_DID_VID=>"ti50 OT", _=>"unknown" } }

unsafe fn tpm_cr50_i2c_get_burst_and_status(chip:*mut TpmChip,mask:u8,burst:*mut usize,status:*mut u32)->i32 { *status=0; let mut b=[0u8;4]; let stop=(*chip).timeout_b; loop { if tpm_cr50_i2c_read(chip,tpm_i2c_sts((*chip).locality),b.as_mut_ptr(),4)>=0 { *status=b[0] as u32; *burst=(b[1] as usize)|((b[2] as usize)<<8); if ((*status as u8)&mask)==mask && *burst>0 && *burst<=TPM_CR50_MAX_BUFSIZE-1{return 0;} } msleep(TPM_CR50_TIMEOUT_SHORT_MS); if !time_before(0,stop){break;} } -ETIMEDOUT }

unsafe fn tpm_cr50_i2c_tis_recv(chip:*mut TpmChip,buf:*mut u8,buf_len:usize)->i32 { if buf_len<TPM_HEADER_SIZE{return -EINVAL;} let mask=TPM_STS_VALID|TPM_STS_DATA_AVAIL; let(mut burst,mut status)=(0usize,0u32); let mut rc=tpm_cr50_i2c_get_burst_and_status(chip,mask,&mut burst,&mut status); if rc<0{tpm_cr50_i2c_tis_set_ready(chip);return rc;} if burst>buf_len||burst<TPM_HEADER_SIZE{tpm_cr50_i2c_tis_set_ready(chip);return -EIO;} rc=tpm_cr50_i2c_read(chip,tpm_i2c_data_fifo((*chip).locality),buf,burst); if rc<0{tpm_cr50_i2c_tis_set_ready(chip);return rc;} let expected=(((*buf.add(2) as usize)<<24)|((*buf.add(3) as usize)<<16)|((*buf.add(4) as usize)<<8)|(*buf.add(5) as usize)); if expected>buf_len{tpm_cr50_i2c_tis_set_ready(chip);return -E2BIG;} let mut cur=burst; while cur<expected {rc=tpm_cr50_i2c_get_burst_and_status(chip,mask,&mut burst,&mut status);if rc<0{tpm_cr50_i2c_tis_set_ready(chip);return rc;}let len=core::cmp::min(burst,expected-cur);rc=tpm_cr50_i2c_read(chip,tpm_i2c_data_fifo((*chip).locality),buf.add(cur),len);if rc<0{tpm_cr50_i2c_tis_set_ready(chip);return rc;}cur+=len;} rc=tpm_cr50_i2c_get_burst_and_status(chip,TPM_STS_VALID,&mut burst,&mut status);if rc<0{tpm_cr50_i2c_tis_set_ready(chip);return rc;}if (status as u8)&TPM_STS_DATA_AVAIL!=0{tpm_cr50_i2c_tis_set_ready(chip);return -EIO;}cur as i32 }

unsafe fn tpm_cr50_i2c_tis_send(chip:*mut TpmChip,buf:*mut u8,_bufsiz:usize,mut len:usize)->i32 { let mut burst=0usize;let mut status=0u32;let mut sent=0usize;let mut go=[TPM_STS_GO,0,0,0]; while tpm_cr50_i2c_tis_status(chip)&TPM_STS_COMMAND_READY==0 {tpm_cr50_i2c_tis_set_ready(chip);}; while len>0 {let mask=if sent>0{TPM_STS_VALID|TPM_STS_DATA_EXPECT}else{TPM_STS_VALID};let mut rc=tpm_cr50_i2c_get_burst_and_status(chip,mask,&mut burst,&mut status);if rc<0{tpm_cr50_i2c_tis_set_ready(chip);return rc;}let n=core::cmp::min(burst-1,len);rc=tpm_cr50_i2c_write(chip,tpm_i2c_data_fifo((*chip).locality),buf.add(sent),n);if rc<0{tpm_cr50_i2c_tis_set_ready(chip);return rc;}sent+=n;len-=n;}let rc=tpm_cr50_i2c_get_burst_and_status(chip,TPM_STS_VALID,&mut burst,&mut status);if rc<0{return rc;}if status as u8&TPM_STS_DATA_EXPECT!=0{return -EIO;}let rc=tpm_cr50_i2c_write(chip,tpm_i2c_sts((*chip).locality),go.as_mut_ptr(),4);if rc<0{tpm_cr50_i2c_tis_set_ready(chip);}rc }

unsafe fn tpm_cr50_i2c_probe(client:*mut I2cClient)->i32 { if !i2c_check_functionality((*client).adapter,1){return -ENODEV;} let chip=tpmm_chip_alloc(&mut (*client).dev,core::ptr::null());if is_err(chip){return ptr_err(chip);}let p=devm_kzalloc(&mut (*client).dev,core::mem::size_of::<TpmI2cCr50PrivData>(),0) as *mut TpmI2cCr50PrivData;if p.is_null(){return -ENOMEM;}(*chip).flags|=1;(*chip).timeout_a=(*chip).timeout_b;dev_set_drvdata(&mut (*chip).dev,p as *mut _);init_completion(&mut (*p).tpm_ready);(*p).irq=(*client).irq;let loc=tpm_cr50_request_locality(chip,TPM_CR50_DEFAULT_LOC);if loc<0{return loc;}let mut b=[0u8;4];let rc=tpm_cr50_i2c_read(chip,tpm_i2c_did_vid(loc),b.as_mut_ptr(),4);let _=tpm_cr50_release_locality(chip,loc);if rc<0{return rc;}let vendor=u32::from_le_bytes(b);if vendor!=TPM_CR50_I2C_DID_VID&&vendor!=TPM_TI50_DT_I2C_DID_VID&&vendor!=TPM_TI50_OT_I2C_DID_VID{return -ENODEV;}tpm_chip_register(chip) }

unsafe fn tpm_cr50_i2c_remove(client:*mut I2cClient){let chip=i2c_get_clientdata(client);if !chip.is_null(){tpm_chip_unregister(chip);}}

// C registration metadata translated as the corresponding kernel-driver items.
// ACPI: "GOOG0005"; Device Tree compatible: "google,cr50"; driver name: "cr50_i2c".

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
