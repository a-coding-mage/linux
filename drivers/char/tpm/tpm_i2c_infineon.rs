// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012,2013 Infineon Technologies
 *
 * Authors:
 * Peter Huewe <peter.huewe@infineon.com>
 *
 * Device driver for TCG/TCPA TPM (trusted platform module).
 * Specifications at www.trustedcomputinggroup.org
 *
 * This device driver implements the TPM interface as defined in
 * the TCG TPM Interface Spec version 1.2, revision 1.0 and the
 * Infineon I2C Protocol Stack Specification v0.20.
 *
 * It is based on the original tpm_tis device driver from Leendert van
 * Dorn and Kyleen Hall.
 */

// Dependencies are supplied by the surrounding kernel translation.

const TPM_I2C_INFINEON_BUFSIZE: usize = 1260;
const MAX_COUNT: i32 = 3;
const SLEEP_DURATION_LOW: u32 = 55;
const SLEEP_DURATION_HI: u32 = 65;
const MAX_COUNT_LONG: i32 = 50;
const SLEEP_DURATION_LONG_LOW: u32 = 200;
const SLEEP_DURATION_LONG_HI: u32 = 220;
const SLEEP_DURATION_RESET_LOW: u32 = 2400;
const SLEEP_DURATION_RESET_HI: u32 = 2600;
const TPM_TIMEOUT_US_LOW: u64 = (TPM_TIMEOUT as u64) * 1000;
const TPM_TIMEOUT_US_HI: u64 = TPM_TIMEOUT_US_LOW + 2000;
const TPM_TIS_I2C_DID_VID_9635: u32 = 0xd1150b00;
const TPM_TIS_I2C_DID_VID_9645: u32 = 0x001a15d1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum i2c_chip_type { SLB9635, SLB9645, UNKNOWN }

#[repr(C)]
struct tpm_inf_dev {
    client: *mut i2c_client,
    locality: i32,
    buf: [u8; TPM_I2C_INFINEON_BUFSIZE + 1],
    chip: *mut tpm_chip,
    chip_type: i2c_chip_type,
    adapterlimit: u32,
}

static mut tpm_dev: tpm_inf_dev = tpm_inf_dev {
    client: core::ptr::null_mut(), locality: 0,
    buf: [0; TPM_I2C_INFINEON_BUFSIZE + 1], chip: core::ptr::null_mut(),
    chip_type: i2c_chip_type::UNKNOWN, adapterlimit: 0,
};

unsafe fn iic_tpm_read(addr: u8, buffer: *mut u8, mut len: usize) -> i32 {
    let mut msg1 = i2c_msg { addr: (*tpm_dev.client).addr, flags: 0, len: 1, buf: &addr as *const u8 as *mut u8 };
    let mut msg2 = i2c_msg { addr: (*tpm_dev.client).addr, flags: I2C_M_RD, len, buf: buffer };
    let mut rc = 0i32;
    let mut msglen = len as u32;
    if (*(*(*tpm_dev.client).adapter).algo).master_xfer.is_none() { return -EOPNOTSUPP; }
    i2c_lock_bus((*tpm_dev.client).adapter, I2C_LOCK_SEGMENT);
    if tpm_dev.chip_type == i2c_chip_type::SLB9645 {
        for _count in 0..MAX_COUNT { rc = __i2c_transfer((*tpm_dev.client).adapter, &mut [msg1, msg2], 2); if rc > 0 { break; } usleep_range(SLEEP_DURATION_LOW, SLEEP_DURATION_HI); }
    } else {
        while len > 0 {
            for _count in 0..MAX_COUNT { rc = __i2c_transfer((*tpm_dev.client).adapter, &mut [msg1], 1); if rc > 0 { break; } usleep_range(SLEEP_DURATION_LOW, SLEEP_DURATION_HI); }
            if rc <= 0 { break; }
            for _count in 0..MAX_COUNT {
                if tpm_dev.adapterlimit != 0 { msglen = core::cmp::min(tpm_dev.adapterlimit, len as u32); msg2.len = msglen as usize; }
                usleep_range(SLEEP_DURATION_LOW, SLEEP_DURATION_HI);
                rc = __i2c_transfer((*tpm_dev.client).adapter, &mut [msg2], 1);
                if rc > 0 { if msglen as usize > len { len = 0; } else { len -= msglen as usize; } msg2.buf = msg2.buf.add(msglen as usize); break; }
                if rc == -EOPNOTSUPP { tpm_dev.adapterlimit = I2C_SMBUS_BLOCK_MAX; }
            }
            if rc <= 0 { break; }
        }
    }
    i2c_unlock_bus((*tpm_dev.client).adapter, I2C_LOCK_SEGMENT);
    usleep_range(SLEEP_DURATION_LOW, SLEEP_DURATION_HI);
    if rc <= 0 { -EIO } else { 0 }
}

unsafe fn iic_tpm_write_generic(addr: u8, buffer: *mut u8, len: usize, sleep_low: u32, sleep_hi: u32, max_count: u8) -> i32 {
    if len > TPM_I2C_INFINEON_BUFSIZE { return -EINVAL; }
    let mut msg = i2c_msg { addr: (*tpm_dev.client).addr, flags: 0, len: len + 1, buf: tpm_dev.buf.as_mut_ptr() };
    if (*(*(*tpm_dev.client).adapter).algo).master_xfer.is_none() { return -EOPNOTSUPP; }
    i2c_lock_bus((*tpm_dev.client).adapter, I2C_LOCK_SEGMENT);
    tpm_dev.buf[0] = addr; core::ptr::copy_nonoverlapping(buffer, tpm_dev.buf.as_mut_ptr().add(1), len);
    let mut rc = -EIO;
    for _count in 0..max_count { rc = __i2c_transfer((*tpm_dev.client).adapter, &mut [msg], 1); if rc > 0 { break; } usleep_range(sleep_low, sleep_hi); }
    i2c_unlock_bus((*tpm_dev.client).adapter, I2C_LOCK_SEGMENT); usleep_range(SLEEP_DURATION_LOW, SLEEP_DURATION_HI);
    if rc <= 0 { -EIO } else { 0 }
}

unsafe fn iic_tpm_write(addr: u8, buffer: *mut u8, len: usize) -> i32 { iic_tpm_write_generic(addr, buffer, len, SLEEP_DURATION_LOW, SLEEP_DURATION_HI, MAX_COUNT as u8) }
unsafe fn iic_tpm_write_long(addr: u8, buffer: *mut u8, len: usize) -> i32 { iic_tpm_write_generic(addr, buffer, len, SLEEP_DURATION_LONG_LOW, SLEEP_DURATION_LONG_HI, MAX_COUNT_LONG as u8) }

#[repr(u8)] enum tis_access { TPM_ACCESS_VALID=0x80, TPM_ACCESS_ACTIVE_LOCALITY=0x20, TPM_ACCESS_REQUEST_PENDING=0x04, TPM_ACCESS_REQUEST_USE=0x02 }
#[repr(u8)] enum tis_status { TPM_STS_VALID=0x80, TPM_STS_COMMAND_READY=0x40, TPM_STS_GO=0x20, TPM_STS_DATA_AVAIL=0x10, TPM_STS_DATA_EXPECT=0x08 }
#[repr(u32)] enum tis_defaults { TIS_SHORT_TIMEOUT=750, TIS_LONG_TIMEOUT=2000 }
const fn TPM_ACCESS(l: i32) -> u8 { (0x0000 | (l << 4)) as u8 }
const fn TPM_STS(l: i32) -> u8 { (0x0001 | (l << 4)) as u8 }
const fn TPM_DATA_FIFO(l: i32) -> u8 { (0x0005 | (l << 4)) as u8 }
const fn TPM_DID_VID(l: i32) -> u8 { (0x0006 | (l << 4)) as u8 }

unsafe fn check_locality(_chip: *mut tpm_chip, loc: i32) -> bool { let mut b=0; if iic_tpm_read(TPM_ACCESS(loc), &mut b, 1)<0 { return false; } if b & 0xa0 == 0xa0 { tpm_dev.locality=loc; true } else { false } }
unsafe fn release_locality(chip: *mut tpm_chip, loc: i32, force: i32) { let mut b=0; if iic_tpm_read(TPM_ACCESS(loc), &mut b, 1)<0{return;} if force!=0 || b&0x84==0x84 { b=0x20; iic_tpm_write(TPM_ACCESS(loc), &mut b, 1); } }
unsafe fn request_locality(chip: *mut tpm_chip, loc: i32) -> i32 { let mut b=0x02; if check_locality(chip,loc){return loc;} iic_tpm_write(TPM_ACCESS(loc),&mut b,1); let stop=jiffies()+(*chip).timeout_a; loop { if check_locality(chip,loc){return loc;} usleep_range(TPM_TIMEOUT_US_LOW as u32,TPM_TIMEOUT_US_HI as u32); if !time_before(jiffies(),stop){break;} } -ETIME }
unsafe fn tpm_tis_i2c_status(_chip: *mut tpm_chip) -> u8 { let mut b=0xff; let mut i=0; loop { if iic_tpm_read(TPM_STS(tpm_dev.locality),&mut b,1)<0{return 0;} i+=1; if b!=0xff || i>=10 {break;} } b }
unsafe fn tpm_tis_i2c_ready(_chip: *mut tpm_chip) { let mut b=0x40; iic_tpm_write_long(TPM_STS(tpm_dev.locality),&mut b,1); }

// The remaining TPM class-operation and driver-registration declarations retain the C driver's
// interfaces and are supplied by the surrounding kernel translation.
unsafe fn tpm_tis_i2c_req_canceled(_chip: *mut tpm_chip, status: u8) -> bool { status == 0x40 }

unsafe fn get_burstcount(chip: *mut tpm_chip) -> isize {
    let stop=jiffies()+(*chip).timeout_d; let mut b=[0u8;3];
    loop { let n=if iic_tpm_read(TPM_STS(tpm_dev.locality)+1,b.as_mut_ptr(),3)<0 {0} else {(b[2] as isize)<<16 | (b[1] as isize)<<8 | b[0] as isize}; if n!=0{return n;} usleep_range(TPM_TIMEOUT_US_LOW as u32,TPM_TIMEOUT_US_HI as u32); if !time_before(jiffies(),stop){break;} } -EBUSY as isize
}
unsafe fn wait_for_stat(chip:*mut tpm_chip, mask:u8, timeout: u64, status:*mut i32)->i32 { *status=tpm_tis_i2c_status(chip) as i32; if *status!=0xff && (*status as u8&mask)==mask{return 0;} let stop=jiffies()+timeout; loop {usleep_range(TPM_TIMEOUT_US_LOW as u32,TPM_TIMEOUT_US_HI as u32); *status=tpm_tis_i2c_status(chip) as i32; if (*status as u8&mask)==mask{return 0;} if !time_before(jiffies(),stop){break;}} -ETIME }
unsafe fn recv_data(chip:*mut tpm_chip, buf:*mut u8, count:usize)->isize {let mut size=0;let mut retries=0;while size<count {let mut burst=get_burstcount(chip);if burst<0{return burst;}if burst as usize>count-size{burst=(count-size) as isize;}let rc=iic_tpm_read(TPM_DATA_FIFO(tpm_dev.locality),buf.add(size),burst as usize);if rc==0{size+=burst as usize;}else if rc<0{retries+=1;}if retries>MAX_COUNT_LONG{return -EIO as isize;}}size as isize}
unsafe fn tpm_tis_i2c_recv(chip:*mut tpm_chip,buf:*mut u8,count:usize)->i32 {if count<TPM_HEADER_SIZE{return -EIO;}let mut size=recv_data(chip,buf,TPM_HEADER_SIZE);if size<TPM_HEADER_SIZE as isize{dev_err(&(*chip).dev,"Unable to read header\n");}else{let expected=u32::from_be(*(buf.add(2) as *const u32)) as usize;if expected>count||expected<TPM_HEADER_SIZE{size=-EIO as isize;}else{size+=recv_data(chip,buf.add(TPM_HEADER_SIZE),expected-TPM_HEADER_SIZE);if size<expected as isize{dev_err(&(*chip).dev,"Unable to read remainder of result\n");size=-ETIME as isize;}else{let mut status=0;wait_for_stat(chip,0x80,(*chip).timeout_c,&mut status);if status&0x10!=0{dev_err(&(*chip).dev,"Error left over data\n");size=-EIO as isize;}}}}tpm_tis_i2c_ready(chip);usleep_range(SLEEP_DURATION_RESET_LOW,SLEEP_DURATION_RESET_HI);release_locality(chip,tpm_dev.locality,0);size as i32}
unsafe fn tpm_tis_i2c_send(chip:*mut tpm_chip,buf:*mut u8,_bufsiz:usize,len:usize)->i32 {if len>TPM_I2C_INFINEON_BUFSIZE{return -E2BIG;}if request_locality(chip,0)<0{return -EBUSY;}let mut status=tpm_tis_i2c_status(chip) as i32;if status&0x40==0{tpm_tis_i2c_ready(chip);if wait_for_stat(chip,0x40,(*chip).timeout_b,&mut status)<0{return -ETIME;}}let mut count=0;let mut retries=0;while count<len-1{let mut burst=get_burstcount(chip);if burst<0{return burst as i32;}if burst as usize>len-1-count{burst=(len-1-count) as isize;}let rc=iic_tpm_write(TPM_DATA_FIFO(tpm_dev.locality),buf.add(count),burst as usize);if rc==0{count+=burst as usize;}else{retries+=1;}if retries>MAX_COUNT_LONG{return -EIO;}wait_for_stat(chip,0x80,(*chip).timeout_c,&mut status);if status&0x08==0{return -EIO;}}iic_tpm_write(TPM_DATA_FIFO(tpm_dev.locality),buf.add(count),1);wait_for_stat(chip,0x80,(*chip).timeout_c,&mut status);if status&0x08!=0{return -EIO;}let mut sts=0x20;iic_tpm_write(TPM_STS(tpm_dev.locality),&mut sts,1);0}

// Driver operation tables, device-id tables, PM hooks, probe/remove routines, and module
// registration are declarations to be connected to the surrounding kernel bindings.
unsafe fn tpm_tis_i2c_init(dev:*mut device)->i32 {let chip=tpmm_chip_alloc(dev,&tpm_tis_i2c);if IS_ERR(chip){return PTR_ERR(chip);}(*chip).timeout_a=msecs_to_jiffies(750);(*chip).timeout_b=msecs_to_jiffies(2000);(*chip).timeout_c=msecs_to_jiffies(750);(*chip).timeout_d=msecs_to_jiffies(750);if request_locality(chip,0)!=0{dev_err(dev,"could not request locality\n");return -ENODEV;}let mut vendor=0u32;if iic_tpm_read(TPM_DID_VID(0),&mut vendor as *mut u32 as *mut u8,4)<0{return -EIO;}if vendor==TPM_TIS_I2C_DID_VID_9645{tpm_dev.chip_type=i2c_chip_type::SLB9645;}else if vendor==TPM_TIS_I2C_DID_VID_9635{tpm_dev.chip_type=i2c_chip_type::SLB9635;}else{return -ENODEV;}dev_info(dev,"1.2 TPM (device-id 0x%X)\n",vendor>>16);tpm_dev.chip=chip;tpm_chip_register(chip)}
unsafe fn tpm_tis_i2c_probe(client:*mut i2c_client)->i32 {if !tpm_dev.client.is_null(){return -EBUSY;}if !i2c_check_functionality((*client).adapter,I2C_FUNC_I2C){return -ENODEV;}tpm_dev.client=client;let rc=tpm_tis_i2c_init(&mut (*client).dev);if rc!=0{tpm_dev.client=core::ptr::null_mut();-ENODEV}else{rc}}
unsafe fn tpm_tis_i2c_remove(_client:*mut i2c_client){let chip=tpm_dev.chip;tpm_chip_unregister(chip);release_locality(chip,tpm_dev.locality,1);tpm_dev.client=core::ptr::null_mut();}
static tpm_tis_i2c: tpm_class_ops = tpm_class_ops {flags:TPM_OPS_AUTO_STARTUP,status:tpm_tis_i2c_status,recv:tpm_tis_i2c_recv,send:tpm_tis_i2c_send,cancel:tpm_tis_i2c_ready,req_complete_mask:0x90,req_complete_val:0x90,req_canceled:tpm_tis_i2c_req_canceled};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
