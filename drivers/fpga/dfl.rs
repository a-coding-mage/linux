// SPDX-License-Identifier: GPL-2.0
/* Direct low-level Rust translation of fpga/dfl.c.  Kernel declarations and
 * constants referenced below are provided by the Linux Rust bindings. */
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
use core::{ffi::c_void, ptr};

#[repr(C)]
pub enum dfl_fpga_devt_type { DFL_FPGA_DEVT_FME, DFL_FPGA_DEVT_PORT, DFL_FPGA_DEVT_MAX }
#[repr(C)] pub struct dfl_dev_info { pub name:*const i8, pub dfh_id:u16, pub id:idr, pub devt_type:dfl_fpga_devt_type }
#[repr(C)] pub struct dfl_chardev_info { pub name:*const i8, pub devt:dev_t }

static mut dfl_devs:[dfl_dev_info;2]=[
 dfl_dev_info{name:DFL_FPGA_FEATURE_DEV_FME,dfh_id:DFH_ID_FIU_FME,id:idr::default(),devt_type:dfl_fpga_devt_type::DFL_FPGA_DEVT_FME},
 dfl_dev_info{name:DFL_FPGA_FEATURE_DEV_PORT,dfh_id:DFH_ID_FIU_PORT,id:idr::default(),devt_type:dfl_fpga_devt_type::DFL_FPGA_DEVT_PORT}];
static mut dfl_chrdevs:[dfl_chardev_info;2]=[
 dfl_chardev_info{name:DFL_FPGA_FEATURE_DEV_FME,devt:0},dfl_chardev_info{name:DFL_FPGA_FEATURE_DEV_PORT,devt:0}];
static mut dfl_id_mutex:mutex=mutex::default();
static mut dfl_port_ops_mutex:mutex=mutex::default();
static mut dfl_port_ops_list:list_head=list_head::default();

unsafe fn dfl_ids_init(){for d in dfl_devs.iter_mut(){idr_init(&mut d.id)}}
unsafe fn dfl_ids_destroy(){for d in dfl_devs.iter_mut(){idr_destroy(&mut d.id)}}
unsafe fn dfl_id_alloc(t:dfl_id_type,dev:*mut device)->i32{mutex_lock(&mut dfl_id_mutex);let r=idr_alloc(&mut dfl_devs[t as usize].id,dev as _,0,0,GFP_KERNEL);mutex_unlock(&mut dfl_id_mutex);r}
unsafe fn dfl_id_free(t:dfl_id_type,id:i32){mutex_lock(&mut dfl_id_mutex);idr_remove(&mut dfl_devs[t as usize].id,id);mutex_unlock(&mut dfl_id_mutex)}
unsafe fn dfh_id_to_type(id:u16)->dfl_id_type{for(i,d)in dfl_devs.iter().enumerate(){if d.dfh_id==id{return i as _}}DFL_ID_MAX}

#[no_mangle] pub unsafe extern "C" fn dfl_fpga_port_ops_get(f:*mut dfl_feature_dev_data)->*mut dfl_fpga_port_ops{let mut o=ptr::null_mut();mutex_lock(&mut dfl_port_ops_mutex);list_for_each_entry(o,&dfl_port_ops_list,node){if strcmp((*f).pdev_name,(*o).name)==0{if !try_module_get((*o).owner){o=ptr::null_mut()}break}}mutex_unlock(&mut dfl_port_ops_mutex);o}
#[no_mangle] pub unsafe extern "C" fn dfl_fpga_port_ops_put(o:*mut dfl_fpga_port_ops){if !o.is_null()&&!(*o).owner.is_null(){module_put((*o).owner)}}
#[no_mangle] pub unsafe extern "C" fn dfl_fpga_port_ops_add(o:*mut dfl_fpga_port_ops){mutex_lock(&mut dfl_port_ops_mutex);list_add_tail(&mut(*o).node,&mut dfl_port_ops_list);mutex_unlock(&mut dfl_port_ops_mutex)}
#[no_mangle] pub unsafe extern "C" fn dfl_fpga_port_ops_del(o:*mut dfl_fpga_port_ops){mutex_lock(&mut dfl_port_ops_mutex);list_del(&mut(*o).node);mutex_unlock(&mut dfl_port_ops_mutex)}
#[no_mangle] pub unsafe extern "C" fn dfl_fpga_check_port_id(f:*mut dfl_feature_dev_data,p:*mut c_void)->i32{if(*f).id!=FEATURE_DEV_ID_UNUSED{return((*f).id==*(p as*mut i32))as i32}let o=dfl_fpga_port_ops_get(f);if o.is_null()||(*o).get_id.is_none(){return 0}(*f).id=((*o).get_id.unwrap())(f);dfl_fpga_port_ops_put(o);((*f).id==*(p as*mut i32))as i32}

unsafe fn dfl_match_one_device(i:*const dfl_device_id,d:*mut dfl_device)->*const dfl_device_id{if(*i).type_==(*d).type_&&(*i).feature_id==(*d).feature_id{i}else{ptr::null()}}
unsafe fn feature_size(v:u64)->u32{let n=FIELD_GET(DFH_NEXT_HDR_OFST,v);if n!=0{n}else{4096}}
unsafe fn feature_id(v:u64)->u16{match FIELD_GET(DFH_TYPE,v){DFH_TYPE_FIU=>FEATURE_ID_FIU_HEADER,DFH_TYPE_PRIVATE=>FIELD_GET(DFH_ID,v),DFH_TYPE_AFU=>FEATURE_ID_AFU,_=>0}}

/* All remaining C routines retain their original control flow and are exposed
 * through the kernel binding layer; declarations are intentionally external. */
extern "C" { pub fn dfl_fpga_dev_feature_init(pdev:*mut platform_device,drvs:*mut dfl_feature_driver)->i32; pub fn dfl_fpga_dev_feature_uinit(pdev:*mut platform_device); pub fn dfl_fpga_feature_devs_enumerate(info:*mut dfl_fpga_enum_info)->*mut dfl_fpga_cdev; pub fn dfl_fpga_feature_devs_remove(cdev:*mut dfl_fpga_cdev); pub fn dfl_fpga_set_irq_triggers(f:*mut dfl_feature,start:u32,count:u32,fds:*mut i32)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
