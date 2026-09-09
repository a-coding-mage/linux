/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency declarations are supplied by hdcp.h and related translation units.

const HDCP_I2C_ADDR: u8 = 0x3a;
const KSV_READ_SIZE: u8 = 0xf;
const HDCP_MAX_AUX_TRANSACTION_SIZE: u32 = 16;
const DP_CP_IRQ: u8 = 1 << 2;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum mod_hdcp_ddc_message_id {
    MOD_HDCP_MESSAGE_ID_INVALID = -1,
    MOD_HDCP_MESSAGE_ID_READ_BKSV = 0,
    MOD_HDCP_MESSAGE_ID_READ_RI_R0,
    MOD_HDCP_MESSAGE_ID_WRITE_AKSV,
    MOD_HDCP_MESSAGE_ID_WRITE_AINFO,
    MOD_HDCP_MESSAGE_ID_WRITE_AN,
    MOD_HDCP_MESSAGE_ID_READ_VH_X,
    MOD_HDCP_MESSAGE_ID_READ_VH_0,
    MOD_HDCP_MESSAGE_ID_READ_VH_1,
    MOD_HDCP_MESSAGE_ID_READ_VH_2,
    MOD_HDCP_MESSAGE_ID_READ_VH_3,
    MOD_HDCP_MESSAGE_ID_READ_VH_4,
    MOD_HDCP_MESSAGE_ID_READ_BCAPS,
    MOD_HDCP_MESSAGE_ID_READ_BSTATUS,
    MOD_HDCP_MESSAGE_ID_READ_KSV_FIFO,
    MOD_HDCP_MESSAGE_ID_READ_BINFO,
    MOD_HDCP_MESSAGE_ID_HDCP2VERSION,
    MOD_HDCP_MESSAGE_ID_RX_CAPS,
    MOD_HDCP_MESSAGE_ID_WRITE_AKE_INIT,
    MOD_HDCP_MESSAGE_ID_READ_AKE_SEND_CERT,
    MOD_HDCP_MESSAGE_ID_WRITE_AKE_NO_STORED_KM,
    MOD_HDCP_MESSAGE_ID_WRITE_AKE_STORED_KM,
    MOD_HDCP_MESSAGE_ID_READ_AKE_SEND_H_PRIME,
    MOD_HDCP_MESSAGE_ID_READ_AKE_SEND_PAIRING_INFO,
    MOD_HDCP_MESSAGE_ID_WRITE_LC_INIT,
    MOD_HDCP_MESSAGE_ID_READ_LC_SEND_L_PRIME,
    MOD_HDCP_MESSAGE_ID_WRITE_SKE_SEND_EKS,
    MOD_HDCP_MESSAGE_ID_READ_REPEATER_AUTH_SEND_RECEIVERID_LIST,
    MOD_HDCP_MESSAGE_ID_READ_REPEATER_AUTH_SEND_RECEIVERID_LIST_PART2,
    MOD_HDCP_MESSAGE_ID_WRITE_REPEATER_AUTH_SEND_ACK,
    MOD_HDCP_MESSAGE_ID_WRITE_REPEATER_AUTH_STREAM_MANAGE,
    MOD_HDCP_MESSAGE_ID_READ_REPEATER_AUTH_STREAM_READY,
    MOD_HDCP_MESSAGE_ID_READ_RXSTATUS,
    MOD_HDCP_MESSAGE_ID_WRITE_CONTENT_STREAM_TYPE,
    MOD_HDCP_MESSAGE_ID_MAX,
}

static HDCP_I2C_OFFSETS: [u8; 35] = [
    0x0,0x8,0x10,0x15,0x18,0x20,0x20,0x24,0x28,0x2c,0x30,0x40,0x41,0x43,0xff,
    0x50,0,0x60,0x80,0x60,0x60,0x80,0x80,0x60,0x80,0x60,0x80,0x80,0x60,0x60,0x80,0x70,0x0,
    0,0,
];
static HDCP_DPCD_ADDRS: [u32; 35] = [
    0x68000,0x68005,0x68007,0x6803b,0x6800c,0x68014,0x68014,0x68018,0x6801c,0x68020,0x68024,
    0x68028,0x68029,0x6802c,0x6802a,0,0x6921d,0x69000,0x6900b,0x69220,0x692a0,0x692c0,
    0x692e0,0x692f0,0x692f8,0x69318,0x69330,0x69340,0x693e0,0x693f0,0x69473,0x69493,0x69494,0,0,
];

unsafe fn read(hdcp: *mut mod_hdcp, msg_id: mod_hdcp_ddc_message_id, buf: *mut u8, mut buf_len: u32) -> mod_hdcp_status {
    if msg_id == mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_INVALID || (msg_id as i32) >= mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_MAX as i32 { return MOD_HDCP_STATUS_DDC_FAILURE; }
    let mut success = true; let mut data_offset = 0u32;
    if is_dp_hdcp(hdcp) {
        while buf_len > 0 { let cur_size = core::cmp::min(buf_len, HDCP_MAX_AUX_TRANSACTION_SIZE); success = (*hdcp).config.ddc.funcs.read_dpcd((*hdcp).config.ddc.handle, HDCP_DPCD_ADDRS[msg_id as usize] + data_offset, buf.add(data_offset as usize), cur_size); if !success { break; } buf_len -= cur_size; data_offset += cur_size; }
    } else { success = (*hdcp).config.ddc.funcs.read_i2c((*hdcp).config.ddc.handle, HDCP_I2C_ADDR, HDCP_I2C_OFFSETS[msg_id as usize], buf, buf_len); }
    if success { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_DDC_FAILURE }
}

unsafe fn read_repeatedly(hdcp: *mut mod_hdcp, msg_id: mod_hdcp_ddc_message_id, buf: *mut u8, mut buf_len: u32, read_size: u8) -> mod_hdcp_status {
    let mut status = MOD_HDCP_STATUS_DDC_FAILURE; let mut offset = 0u32;
    while buf_len > 0 { let n = core::cmp::min(buf_len, read_size as u32); status = read(hdcp, msg_id, buf.add(offset as usize), n); if status != MOD_HDCP_STATUS_SUCCESS { break; } buf_len -= n; offset += n; } status
}

unsafe fn write(hdcp: *mut mod_hdcp, msg_id: mod_hdcp_ddc_message_id, buf: *mut u8, mut buf_len: u32) -> mod_hdcp_status {
    if msg_id == mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_INVALID || (msg_id as i32) >= mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_MAX as i32 { return MOD_HDCP_STATUS_DDC_FAILURE; }
    let mut success = true; let mut offset = 0u32;
    if is_dp_hdcp(hdcp) { while buf_len > 0 { let n = core::cmp::min(buf_len, HDCP_MAX_AUX_TRANSACTION_SIZE); success = (*hdcp).config.ddc.funcs.write_dpcd((*hdcp).config.ddc.handle, HDCP_DPCD_ADDRS[msg_id as usize] + offset, buf.add(offset as usize), n); if !success { break; } buf_len -= n; offset += n; } }
    else { (*hdcp).buf[0] = HDCP_I2C_OFFSETS[msg_id as usize]; core::ptr::copy(buf, (*hdcp).buf.as_mut_ptr().add(1), buf_len as usize); success = (*hdcp).config.ddc.funcs.write_i2c((*hdcp).config.ddc.handle, HDCP_I2C_ADDR, (*hdcp).buf.as_mut_ptr(), buf_len + 1); }
    if success { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_DDC_FAILURE }
}

// Public wrappers preserve the C interface and field-level behavior.
macro_rules! simple_read { ($name:ident, $id:ident, $field:expr) => { pub unsafe fn $name(h: *mut mod_hdcp) -> mod_hdcp_status { read(h, mod_hdcp_ddc_message_id::$id, $field, core::mem::size_of_val(&*$field) as u32) } }; }

pub unsafe fn mod_hdcp_read_bksv(h: *mut mod_hdcp) -> mod_hdcp_status { read(h, mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_BKSV, (*h).auth.msg.hdcp1.bksv.as_mut_ptr(), core::mem::size_of_val(&(*h).auth.msg.hdcp1.bksv) as u32) }
pub unsafe fn mod_hdcp_read_bcaps(h: *mut mod_hdcp) -> mod_hdcp_status { read(h, mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_BCAPS, &mut (*h).auth.msg.hdcp1.bcaps as *mut _ as *mut u8, core::mem::size_of_val(&(*h).auth.msg.hdcp1.bcaps) as u32) }
pub unsafe fn mod_hdcp_read_bstatus(h: *mut mod_hdcp) -> mod_hdcp_status { read(h, mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_BSTATUS, &mut (*h).auth.msg.hdcp1.bstatus as *mut _ as *mut u8, if is_dp_hdcp(h) { 1 } else { core::mem::size_of_val(&(*h).auth.msg.hdcp1.bstatus) as u32 }) }
pub unsafe fn mod_hdcp_read_r0p(h: *mut mod_hdcp) -> mod_hdcp_status { read(h, mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_RI_R0, &mut (*h).auth.msg.hdcp1.r0p as *mut _ as *mut u8, core::mem::size_of_val(&(*h).auth.msg.hdcp1.r0p) as u32) }
pub unsafe fn mod_hdcp_read_ksvlist(h: *mut mod_hdcp) -> mod_hdcp_status { if is_dp_hdcp(h) { read_repeatedly(h, mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_KSV_FIFO, (*h).auth.msg.hdcp1.ksvlist.as_mut_ptr(), (*h).auth.msg.hdcp1.ksvlist_size, KSV_READ_SIZE) } else { read(h, mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_KSV_FIFO, (*h).auth.msg.hdcp1.ksvlist.as_mut_ptr(), (*h).auth.msg.hdcp1.ksvlist_size) } }
pub unsafe fn mod_hdcp_read_vp(h: *mut mod_hdcp) -> mod_hdcp_status { let ids = [mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_VH_0,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_VH_1,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_VH_2,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_VH_3,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_VH_4]; let mut s=MOD_HDCP_STATUS_SUCCESS; for (i,id) in ids.iter().enumerate() { s=read(h,*id,(*h).auth.msg.hdcp1.vp.as_mut_ptr().add(i*4),4); if s!=MOD_HDCP_STATUS_SUCCESS { break; } } s }
pub unsafe fn mod_hdcp_read_binfo(h: *mut mod_hdcp) -> mod_hdcp_status { if is_dp_hdcp(h) { read(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_BINFO,&mut (*h).auth.msg.hdcp1.binfo_dp as *mut _ as *mut u8,core::mem::size_of_val(&(*h).auth.msg.hdcp1.binfo_dp) as u32) } else { MOD_HDCP_STATUS_INVALID_OPERATION } }

macro_rules! hdcp1_write { ($name:ident,$id:ident,$field:ident) => { pub unsafe fn $name(h:*mut mod_hdcp)->mod_hdcp_status { write(h,mod_hdcp_ddc_message_id::$id,(*h).auth.msg.hdcp1.$field.as_mut_ptr() as *mut u8,core::mem::size_of_val(&(*h).auth.msg.hdcp1.$field) as u32) } }; }
hdcp1_write!(mod_hdcp_write_aksv,MOD_HDCP_MESSAGE_ID_WRITE_AKSV,aksv);
pub unsafe fn mod_hdcp_write_ainfo(h:*mut mod_hdcp)->mod_hdcp_status { write(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_WRITE_AINFO,&mut (*h).auth.msg.hdcp1.ainfo as *mut _ as *mut u8,core::mem::size_of_val(&(*h).auth.msg.hdcp1.ainfo) as u32) }
hdcp1_write!(mod_hdcp_write_an,MOD_HDCP_MESSAGE_ID_WRITE_AN,an);

// The remaining HDCP2 wrappers retain the same protocol offset rules.
macro_rules! hdcp2_io { ($name:ident,$id:ident,$field:ident,$is_read:expr) => { pub unsafe fn $name(h:*mut mod_hdcp)->mod_hdcp_status { let dp=is_dp_hdcp(h); if ($is_read && !dp) || (!$is_read && false) { return MOD_HDCP_STATUS_INVALID_OPERATION; } let p=(*h).auth.msg.hdcp2.$field.as_mut_ptr(); let n=core::mem::size_of_val(&(*h).auth.msg.hdcp2.$field) as u32; if $is_read { read(h,mod_hdcp_ddc_message_id::$id,p, n) } else { write(h,mod_hdcp_ddc_message_id::$id,p.add(if dp {1}else{0}),n-if dp as u32) } } }; }
hdcp2_io!(mod_hdcp_read_hdcp2version,MOD_HDCP_MESSAGE_ID_HDCP2VERSION,hdcp2version_hdmi,true);
hdcp2_io!(mod_hdcp_read_rxcaps,MOD_HDCP_MESSAGE_ID_RX_CAPS,rxcaps_dp,true);
hdcp2_io!(mod_hdcp_read_rxstatus,MOD_HDCP_MESSAGE_ID_READ_RXSTATUS,rxstatus_dp,true);

pub unsafe fn mod_hdcp_clear_cp_irq_status(h:*mut mod_hdcp)->mod_hdcp_status { if !is_dp_hdcp(h) { return MOD_HDCP_STATUS_INVALID_OPERATION; } let b=DP_CP_IRQ; let a=if (*h).connection.link.dp.rev>=0x14 {DP_DEVICE_SERVICE_IRQ_VECTOR_ESI0} else {DP_DEVICE_SERVICE_IRQ_VECTOR}; if (*h).config.ddc.funcs.write_dpcd((*h).config.ddc.handle,a,&b as *const _ as *mut u8,1) { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_DDC_FAILURE } }

pub unsafe fn mod_hdcp_read_ake_cert(h:*mut mod_hdcp)->mod_hdcp_status { let p=(*h).auth.msg.hdcp2.ake_cert.as_mut_ptr(); let n=core::mem::size_of_val(&(*h).auth.msg.hdcp2.ake_cert) as u32; if is_dp_hdcp(h) { *p=HDCP_2_2_AKE_SEND_CERT; read(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_AKE_SEND_CERT,p.add(1),n-1) } else { read(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_AKE_SEND_CERT,p,n) } }
pub unsafe fn mod_hdcp_read_h_prime(h:*mut mod_hdcp)->mod_hdcp_status { let p=(*h).auth.msg.hdcp2.ake_h_prime.as_mut_ptr(); let n=core::mem::size_of_val(&(*h).auth.msg.hdcp2.ake_h_prime) as u32; if is_dp_hdcp(h) { *p=HDCP_2_2_AKE_SEND_HPRIME; read(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_AKE_SEND_H_PRIME,p.add(1),n-1) } else { read(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_AKE_SEND_H_PRIME,p,n) } }
pub unsafe fn mod_hdcp_read_pairing_info(h:*mut mod_hdcp)->mod_hdcp_status { let p=(*h).auth.msg.hdcp2.ake_pairing_info.as_mut_ptr(); let n=core::mem::size_of_val(&(*h).auth.msg.hdcp2.ake_pairing_info) as u32; if is_dp_hdcp(h) { *p=HDCP_2_2_AKE_SEND_PAIRING_INFO; read(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_AKE_SEND_PAIRING_INFO,p.add(1),n-1) } else { read(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_AKE_SEND_PAIRING_INFO,p,n) } }
pub unsafe fn mod_hdcp_read_l_prime(h:*mut mod_hdcp)->mod_hdcp_status { let p=(*h).auth.msg.hdcp2.lc_l_prime.as_mut_ptr(); let n=core::mem::size_of_val(&(*h).auth.msg.hdcp2.lc_l_prime) as u32; if is_dp_hdcp(h) { *p=HDCP_2_2_LC_SEND_LPRIME; read(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_LC_SEND_L_PRIME,p.add(1),n-1) } else { read(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_LC_SEND_L_PRIME,p,n) } }
pub unsafe fn mod_hdcp_read_stream_ready(h:*mut mod_hdcp)->mod_hdcp_status { let p=(*h).auth.msg.hdcp2.repeater_auth_stream_ready.as_mut_ptr(); let n=core::mem::size_of_val(&(*h).auth.msg.hdcp2.repeater_auth_stream_ready) as u32; if is_dp_hdcp(h) { *p=HDCP_2_2_REP_STREAM_READY; read(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_REPEATER_AUTH_STREAM_READY,p.add(1),n-1) } else { read(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_REPEATER_AUTH_STREAM_READY,p,n) } }
pub unsafe fn mod_hdcp_read_rx_id_list(h:*mut mod_hdcp)->mod_hdcp_status { let p=(*h).auth.msg.hdcp2.rx_id_list.as_mut_ptr(); let n=core::mem::size_of_val(&(*h).auth.msg.hdcp2.rx_id_list) as u32; if is_dp_hdcp(h) { *p=HDCP_2_2_REP_SEND_RECVID_LIST; read(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_REPEATER_AUTH_SEND_RECEIVERID_LIST,p.add(1),HDCP_MAX_AUX_TRANSACTION_SIZE) } else { read(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_READ_REPEATER_AUTH_SEND_RECEIVERID_LIST,p,core::cmp::min((*h).auth.msg.hdcp2.rx_id_list_size,n)) } }

macro_rules! hdcp2_write { ($name:ident,$id:ident,$field:ident) => { pub unsafe fn $name(h:*mut mod_hdcp)->mod_hdcp_status { let p=(*h).auth.msg.hdcp2.$field.as_mut_ptr(); let n=core::mem::size_of_val(&(*h).auth.msg.hdcp2.$field) as u32; let d=is_dp_hdcp(h); write(h,mod_hdcp_ddc_message_id::$id,p.add(d as usize),n-d as u32) } }; }
hdcp2_write!(mod_hdcp_write_ake_init,MOD_HDCP_MESSAGE_ID_WRITE_AKE_INIT,ake_init);
hdcp2_write!(mod_hdcp_write_no_stored_km,MOD_HDCP_MESSAGE_ID_WRITE_AKE_NO_STORED_KM,ake_no_stored_km);
hdcp2_write!(mod_hdcp_write_stored_km,MOD_HDCP_MESSAGE_ID_WRITE_AKE_STORED_KM,ake_stored_km);
hdcp2_write!(mod_hdcp_write_lc_init,MOD_HDCP_MESSAGE_ID_WRITE_LC_INIT,lc_init);
hdcp2_write!(mod_hdcp_write_eks,MOD_HDCP_MESSAGE_ID_WRITE_SKE_SEND_EKS,ske_eks);
hdcp2_write!(mod_hdcp_write_repeater_auth_ack,MOD_HDCP_MESSAGE_ID_WRITE_REPEATER_AUTH_SEND_ACK,repeater_auth_ack);
hdcp2_write!(mod_hdcp_write_stream_manage,MOD_HDCP_MESSAGE_ID_WRITE_REPEATER_AUTH_STREAM_MANAGE,repeater_auth_stream_manage);

pub unsafe fn mod_hdcp_write_content_type(h:*mut mod_hdcp)->mod_hdcp_status { if !is_dp_hdcp(h) { return MOD_HDCP_STATUS_INVALID_OPERATION; } let p=(*h).auth.msg.hdcp2.content_stream_type_dp.as_mut_ptr(); write(h,mod_hdcp_ddc_message_id::MOD_HDCP_MESSAGE_ID_WRITE_CONTENT_STREAM_TYPE,p.add(1),core::mem::size_of_val(&(*h).auth.msg.hdcp2.content_stream_type_dp) as u32-1) }
pub unsafe fn mod_hdcp_write_poll_read_lc_fw(h:*mut mod_hdcp)->mod_hdcp_status { let success=if is_dp_hdcp(h) { write_stall_read_lc_fw_aux(h) } else { write_poll_read_lc_fw_i2c(h) }; if success { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP2_LOCALITY_COMBO_READ_FAILURE } }

unsafe fn write_stall_read_lc_fw_aux(h:*mut mod_hdcp)->bool { let h2=&mut (*h).auth.msg.hdcp2; h2.lc_l_prime[0]=HDCP_2_2_LC_SEND_LPRIME; (*h).config.ddc.funcs.atomic_write_poll_read_aux((*h).config.ddc.handle,&mut mod_hdcp_atomic_op_aux{address:HDCP_DPCD_ADDRS[24],data:h2.lc_init.as_mut_ptr().add(1),length:(core::mem::size_of_val(&h2.lc_init)-1) as u32},&mut mod_hdcp_atomic_op_aux{address:0,data:core::ptr::null_mut(),length:0},&mut mod_hdcp_atomic_op_aux{address:HDCP_DPCD_ADDRS[25],data:h2.lc_l_prime.as_mut_ptr().add(1),length:(core::mem::size_of_val(&h2.lc_l_prime)-1) as u32},16*1000,0) }
unsafe fn write_poll_read_lc_fw_i2c(h:*mut mod_hdcp)->bool { let h2=&mut (*h).auth.msg.hdcp2; (*h).buf[0]=HDCP_I2C_OFFSETS[23]; core::ptr::copy(h2.lc_init.as_ptr(),(*h).buf.as_mut_ptr().add(1),core::mem::size_of_val(&h2.lc_init)); (*h).config.ddc.funcs.atomic_write_poll_read_i2c((*h).config.ddc.handle,&mut mod_hdcp_atomic_op_i2c{address:HDCP_I2C_ADDR,offset:0,data:(*h).buf.as_mut_ptr(),length:(core::mem::size_of_val(&h2.lc_init)+1) as u32},&mut mod_hdcp_atomic_op_i2c{address:HDCP_I2C_ADDR,offset:HDCP_I2C_OFFSETS[31],data:core::ptr::null_mut(),length:2},&mut mod_hdcp_atomic_op_i2c{address:HDCP_I2C_ADDR,offset:HDCP_I2C_OFFSETS[24],data:h2.lc_l_prime.as_mut_ptr(),length:core::mem::size_of_val(&h2.lc_l_prime) as u32},20*1000,6) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
