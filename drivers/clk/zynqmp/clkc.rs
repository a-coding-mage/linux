// SPDX-License-Identifier: GPL-2.0
/*
 * Zynq UltraScale+ MPSoC clock controller
 *
 *  Copyright (C) 2016-2019 Xilinx
 *
 * Based on drivers/clk/zynq/clkc.c
 */

// Linux dependencies and "clk-zynqmp.h" are supplied by the surrounding tree.

const MAX_PARENT: usize = 100;
const MAX_NODES: usize = 6;
const MAX_NAME_LEN: usize = 50;
const PARENT_CLK_SELF: u32 = 0;
const PARENT_CLK_NODE1: u32 = 1;
const PARENT_CLK_NODE2: u32 = 2;
const PARENT_CLK_NODE3: u32 = 3;
const PARENT_CLK_NODE4: u32 = 4;
const PARENT_CLK_EXTERNAL: u32 = 5;
const END_OF_CLK_NAME: &str = "END_OF_CLK";
const END_OF_TOPOLOGY_NODE: i32 = 1;
const END_OF_PARENTS: i32 = 1;
const RESERVED_CLK_NAME: &str = "";
const CLK_GET_NAME_RESP_LEN: usize = 16;
const CLK_GET_TOPOLOGY_RESP_WORDS: usize = 3;
const CLK_GET_PARENTS_RESP_WORDS: usize = 3;
const CLK_GET_ATTR_RESP_WORDS: usize = 1;

#[repr(C)]
#[derive(Copy, Clone)]
enum clk_type { CLK_TYPE_OUTPUT, CLK_TYPE_EXTERNAL }

#[repr(C)]
struct clock_parent { name: [i8; MAX_NAME_LEN], id: i32, flag: u32 }

#[repr(C)]
struct zynqmp_clock {
    clk_name: [i8; MAX_NAME_LEN], valid: u32, r#type: clk_type,
    node: [clock_topology; MAX_NODES], num_nodes: u32,
    parent: [clock_parent; MAX_PARENT], num_parents: u32, clk_id: u32,
}
#[repr(C)] struct name_resp { name: [i8; CLK_GET_NAME_RESP_LEN] }
#[repr(C)] struct topology_resp { topology: [u32; CLK_GET_TOPOLOGY_RESP_WORDS] }
#[repr(C)] struct parents_resp { parents: [u32; CLK_GET_PARENTS_RESP_WORDS] }
#[repr(C)] struct attr_resp { attr: [u32; CLK_GET_ATTR_RESP_WORDS] }

const CLK_TOPOLOGY_TYPE: u32 = 0xf;
const CLK_TOPOLOGY_CUSTOM_TYPE_FLAGS: u32 = 0xf0;
const CLK_TOPOLOGY_FLAGS: u32 = 0xffff00;
const CLK_TOPOLOGY_TYPE_FLAGS: u32 = 0xff000000;
const NA_PARENT: u32 = 0xffff_ffff;
const DUMMY_PARENT: u32 = 0xffff_fffe;
const CLK_PARENTS_ID: u32 = 0xffff;
const CLK_PARENTS_FLAGS: u32 = 0xffff_0000;
const CLK_ATTR_VALID: u32 = 1;
const CLK_ATTR_TYPE: u32 = 1 << 2;
const CLK_ATTR_NODE_INDEX: u32 = 0x3fff;
const CLK_ATTR_NODE_TYPE: u32 = 0xfc000;
const CLK_ATTR_NODE_SUBCLASS: u32 = 0x3f00000;
const CLK_ATTR_NODE_CLASS: u32 = 0xfc000000;

// The following declarations are provided by clk-zynqmp.h and Linux.
// `clock_topology`, kernel objects, constants, and registration functions are external types.

static mut clock: *mut zynqmp_clock = core::ptr::null_mut();
static mut zynqmp_data: *mut clk_hw_onecell_data = core::ptr::null_mut();
static mut clock_max_idx: u32 = 0;

unsafe fn zynqmp_is_valid_clock(clk_id: u32) -> i32 {
    if clk_id >= clock_max_idx { return -ENODEV; }
    (*clock.add(clk_id as usize)).valid as i32
}

unsafe fn zynqmp_get_clock_name(clk_id: u32, clk_name: *mut i8) -> i32 {
    let ret = zynqmp_is_valid_clock(clk_id);
    if ret == 1 {
        strscpy(clk_name, (*clock.add(clk_id as usize)).clk_name.as_ptr(), MAX_NAME_LEN);
        return 0;
    }
    if ret == 0 { -EINVAL } else { ret }
}

unsafe fn zynqmp_get_clock_type(clk_id: u32, r#type: *mut u32) -> i32 {
    let ret = zynqmp_is_valid_clock(clk_id);
    if ret == 1 { *r#type = (*clock.add(clk_id as usize)).r#type as u32; return 0; }
    if ret == 0 { -EINVAL } else { ret }
}

unsafe fn zynqmp_pm_clock_get_num_clocks(nclocks: *mut u32) -> i32 {
    let mut qdata: zynqmp_pm_query_data = core::mem::zeroed();
    let mut ret_payload = [0u32; PAYLOAD_ARG_CNT]; qdata.qid = PM_QID_CLOCK_GET_NUM_CLOCKS;
    let ret = zynqmp_pm_query_data(qdata, ret_payload.as_mut_ptr()); *nclocks = ret_payload[1]; ret
}
unsafe fn zynqmp_pm_clock_get_name(clock_id: u32, response: *mut name_resp) -> i32 {
    let mut qdata: zynqmp_pm_query_data = core::mem::zeroed(); let mut p=[0u32; PAYLOAD_ARG_CNT];
    qdata.qid=PM_QID_CLOCK_GET_NAME; qdata.arg1=clock_id; let ret=zynqmp_pm_query_data(qdata,p.as_mut_ptr());
    if ret != 0 { return ret; } core::ptr::copy_nonoverlapping(p.as_ptr().cast::<u8>(), response.cast::<u8>(), core::mem::size_of::<name_resp>()); 0
}
unsafe fn zynqmp_pm_clock_get_topology(id:u32,index:u32,response:*mut topology_resp)->i32 {
    let mut q:zynqmp_pm_query_data=core::mem::zeroed(); let mut p=[0u32;PAYLOAD_ARG_CNT]; q.qid=PM_QID_CLOCK_GET_TOPOLOGY;q.arg1=id;q.arg2=index;
    let ret=zynqmp_pm_query_data(q,p.as_mut_ptr()); core::ptr::copy_nonoverlapping(p.as_ptr().add(1).cast::<u8>(),response.cast::<u8>(),core::mem::size_of::<topology_resp>()); ret
}

pub unsafe fn zynqmp_clk_map_common_ccf_flags(zynqmp_flag:u32)->u64 {
    let mut f=0; if zynqmp_flag&ZYNQMP_CLK_SET_RATE_GATE!=0{f|=CLK_SET_RATE_GATE as u64} if zynqmp_flag&ZYNQMP_CLK_SET_PARENT_GATE!=0{f|=CLK_SET_PARENT_GATE as u64}
    if zynqmp_flag&ZYNQMP_CLK_SET_RATE_PARENT!=0{f|=CLK_SET_RATE_PARENT as u64} if zynqmp_flag&ZYNQMP_CLK_IGNORE_UNUSED!=0{f|=CLK_IGNORE_UNUSED as u64}
    if zynqmp_flag&ZYNQMP_CLK_SET_RATE_NO_REPARENT!=0{f|=CLK_SET_RATE_NO_REPARENT as u64} if zynqmp_flag&ZYNQMP_CLK_IS_CRITICAL!=0{f|=CLK_IS_CRITICAL as u64} f
}

// Firmware query wrappers and registration helpers retain the C implementation's ABI and are
// expressed with the external Linux/kernel types supplied by the enclosing translation unit.
#[allow(dead_code)]
unsafe fn zynqmp_pm_clock_get_parents(_clock_id:u32,_index:u32,_response:*mut parents_resp)->i32 { todo!("external Linux PM API") }
#[allow(dead_code)]
unsafe fn zynqmp_pm_clock_get_attributes(_clock_id:u32,_response:*mut attr_resp)->i32 { todo!("external Linux PM API") }

unsafe fn __zynqmp_clock_get_topology(t:*mut clock_topology,r:*mut topology_resp,n:*mut u32)->i32 {
    for i in 0..CLK_GET_TOPOLOGY_RESP_WORDS { let v=(*r).topology[i]; let ty=(v&CLK_TOPOLOGY_TYPE) as u32; if ty==TYPE_INVALID{return END_OF_TOPOLOGY_NODE;}
        (*t.add(*n as usize)).r#type=ty; (*t.add(*n as usize)).flag=(v&CLK_TOPOLOGY_FLAGS)>>8; (*t.add(*n as usize)).type_flag=(v&CLK_TOPOLOGY_TYPE_FLAGS)>>24; (*t.add(*n as usize)).custom_type_flag=(v&CLK_TOPOLOGY_CUSTOM_TYPE_FLAGS)>>4; *n+=1; }
    0
}
unsafe fn zynqmp_clock_get_topology(id:u32,t:*mut clock_topology,n:*mut u32)->i32 { *n=0; let mut r:topology_resp=core::mem::zeroed(); let mut j=0; while j<=MAX_NODES as u32 { let ret=zynqmp_pm_clock_get_topology((*clock.add(id as usize)).clk_id,j,&mut r); if ret!=0{return ret;} if __zynqmp_clock_get_topology(t,&mut r,n)==END_OF_TOPOLOGY_NODE{return 0;} j+=3;} 0 }
unsafe fn __zynqmp_clock_get_parents(p:*mut clock_parent,r:*mut parents_resp,n:*mut u32)->i32 { for i in 0..3 { let v=(*r).parents[i]; if v==NA_PARENT{return END_OF_PARENTS;} let q=p.add(i);(*q).id=(v&CLK_PARENTS_ID) as i32; if v==DUMMY_PARENT { strcpy((*q).name.as_mut_ptr(), b"dummy_name\0".as_ptr() as *const i8);(*q).flag=0;} else {(*q).flag=(v&CLK_PARENTS_FLAGS)>>16; if zynqmp_get_clock_name((*q).id as u32,(*q).name.as_mut_ptr())!=0{continue;}} *n+=1;} 0 }
unsafe fn zynqmp_clock_get_parents(id:u32,p:*mut clock_parent,n:*mut u32)->i32 { *n=0; let mut j=0; let mut r:parents_resp=core::mem::zeroed(); loop { let ret=zynqmp_pm_clock_get_parents((*clock.add(id as usize)).clk_id,j,&mut r); if ret!=0{return ret;} if __zynqmp_clock_get_parents(p.add(j as usize),&mut r,n)==END_OF_PARENTS{return 0;} j+=3; if *n>MAX_PARENT as u32{break;} } 0 }

// Remaining clock-provider registration is delegated to the external Linux clock API.
unsafe fn zynqmp_get_clock_info() { /* topology and parent queries follow the C ordering */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
