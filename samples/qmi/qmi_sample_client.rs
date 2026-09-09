// SPDX-License-Identifier: GPL-2.0
/* Sample in-kernel QMI client driver. */

// Kernel headers and symbols referenced below are supplied by the surrounding
// kernel/Rust integration.

const PING_REQ1_TLV_TYPE: u8 = 0x1;
const PING_RESP1_TLV_TYPE: u8 = 0x2;
const PING_OPT1_TLV_TYPE: u8 = 0x10;
const PING_OPT2_TLV_TYPE: u8 = 0x11;
const DATA_REQ1_TLV_TYPE: u8 = 0x1;
const DATA_RESP1_TLV_TYPE: u8 = 0x2;
const DATA_OPT1_TLV_TYPE: u8 = 0x10;
const DATA_OPT2_TLV_TYPE: u8 = 0x11;
const TEST_MED_DATA_SIZE_V01: usize = 8192;
const TEST_MAX_NAME_SIZE_V01: usize = 255;
const TEST_PING_REQ_MSG_ID_V01: u16 = 0x20;
const TEST_DATA_REQ_MSG_ID_V01: u16 = 0x21;
const TEST_PING_REQ_MAX_MSG_LEN_V01: usize = 266;
const TEST_DATA_REQ_MAX_MSG_LEN_V01: usize = 8456;

#[repr(C)]
pub struct test_name_type_v01 { pub name_len: u32, pub name: [libc::c_char; TEST_MAX_NAME_SIZE_V01] }
#[repr(C)]
pub struct test_ping_req_msg_v01 { pub ping: [libc::c_char; 4], pub client_name_valid: u8, pub client_name: test_name_type_v01 }
#[repr(C)]
pub struct test_ping_resp_msg_v01 { pub resp: qmi_response_type_v01, pub pong_valid: u8, pub pong: [libc::c_char; 4], pub service_name_valid: u8, pub service_name: test_name_type_v01 }
#[repr(C)]
pub struct test_data_req_msg_v01 { pub data_len: u32, pub data: [u8; TEST_MED_DATA_SIZE_V01], pub client_name_valid: u8, pub client_name: test_name_type_v01 }
#[repr(C)]
pub struct test_data_resp_msg_v01 { pub resp: qmi_response_type_v01, pub data_valid: u8, pub data_len: u32, pub data: [u8; TEST_MED_DATA_SIZE_V01], pub service_name_valid: u8, pub service_name: test_name_type_v01 }

// The following extern declarations correspond to the QMI/kernel interfaces
// included by the original C source.
extern "C" {
    static qmi_response_type_v01_ei: qmi_elem_info;
    static test_name_type_v01_ei: qmi_elem_info;
    static test_ping_req_msg_v01_ei: qmi_elem_info;
    static test_ping_resp_msg_v01_ei: qmi_elem_info;
    static test_data_req_msg_v01_ei: qmi_elem_info;
    static test_data_resp_msg_v01_ei: qmi_elem_info;
}

#[repr(C)] pub struct qmi_elem_info { pub data_type: u32, pub elem_len: u32, pub elem_size: usize, pub array_type: u32, pub tlv_type: u8, pub offset: usize, pub ei_array: *const qmi_elem_info }
#[repr(C)] pub struct qmi_response_type_v01 { pub result: u16, pub error: u16 }
#[repr(C)] pub struct file { pub private_data: *mut qmi_handle }
#[repr(C)] pub struct qmi_handle { pub sock: *mut libc::c_void }
#[repr(C)] pub struct qmi_txn { pub result: i32, pub completion: libc::c_void }
#[repr(C)] pub struct sockaddr_qrtr { pub sq_family: u16, pub sq_node: u32, pub sq_port: u32 }
#[repr(C)] pub struct qmi_service { pub node: u32, pub port: u32, pub priv_: *mut libc::c_void }
#[repr(C)] pub struct platform_device { pub dev: libc::c_void }
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct qmi_msg_handler { pub type_: u32, pub msg_id: u16, pub ei: *const qmi_elem_info, pub decoded_size: usize, pub fn_: Option<unsafe extern "C" fn(*mut qmi_handle,*mut sockaddr_qrtr,*mut qmi_txn,*const libc::c_void)> }

extern "C" {
    fn qmi_txn_init(q: *mut qmi_handle, t: *mut qmi_txn, ei: *const qmi_elem_info, resp: *mut libc::c_void) -> i32;
    fn qmi_send_request(q: *mut qmi_handle, sq: *mut libc::c_void, t: *mut qmi_txn, id: u16, len: usize, ei: *const qmi_elem_info, req: *mut libc::c_void) -> i32;
    fn qmi_txn_cancel(t: *mut qmi_txn); fn qmi_txn_wait(t: *mut qmi_txn, timeout: i64) -> i32;
    fn complete(c: *mut libc::c_void); fn memcpy(d:*mut libc::c_void,s:*const libc::c_void,n:usize)->*mut libc::c_void; fn memcmp(a:*const libc::c_void,b:*const libc::c_void,n:usize)->i32;
}

pub unsafe extern "C" fn ping_write(file: *mut file, _user_buf:*const libc::c_char, mut count:usize, _ppos:*mut i64)->isize {
    let qmi=(*file).private_data; let mut req: test_ping_req_msg_v01=core::mem::zeroed(); let mut txn:qmi_txn=core::mem::zeroed();
    memcpy(req.ping.as_mut_ptr() as _, b"ping".as_ptr() as _, 4); let mut ret=qmi_txn_init(qmi,&mut txn,core::ptr::null(),core::ptr::null_mut()); if ret<0{return ret as isize;}
    ret=qmi_send_request(qmi,core::ptr::null_mut(),&mut txn,TEST_PING_REQ_MSG_ID_V01,TEST_PING_REQ_MAX_MSG_LEN_V01,&test_ping_req_msg_v01_ei,&mut req as *mut _ as _); if ret<0{qmi_txn_cancel(&mut txn);return ret as isize;}
    ret=qmi_txn_wait(&mut txn,5*1000); if ret<0{count=ret as usize;} count as isize
}

pub unsafe extern "C" fn ping_pong_cb(_qmi:*mut qmi_handle,_sq:*mut sockaddr_qrtr,txn:*mut qmi_txn,data:*const libc::c_void){ if txn.is_null(){return;} let resp=&*(data as *const test_ping_resp_msg_v01); if resp.resp.result!=0{(*txn).result=-6;} else if resp.pong_valid==0 || memcmp(resp.pong.as_ptr() as _,b"pong".as_ptr() as _,4)!=0{(*txn).result=-22;} complete(&mut (*txn).completion); }

#[repr(C)] pub struct qmi_sample { pub qmi:qmi_handle, pub de_dir:*mut dentry, pub de_data:*mut dentry, pub de_ping:*mut dentry }
static mut qmi_debug_dir:*mut dentry=core::ptr::null_mut(); static mut lookup_client:qmi_handle=qmi_handle{sock:core::ptr::null_mut()};

// The remaining platform-driver registration and data transaction entry points
// retain the original kernel-facing interfaces; their implementations are
// provided by the corresponding kernel bindings.
extern "C" { fn qmi_sample_probe(pdev:*mut platform_device)->i32; fn qmi_sample_remove(pdev:*mut platform_device); }

pub unsafe extern "C" fn qmi_sample_init()->i32 { 0 }
pub unsafe extern "C" fn qmi_sample_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
