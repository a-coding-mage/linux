/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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

/* C header dependency and CTX/REG/FD/IND_REG conventions are supplied externally. */

#[macro_export]
macro_rules! REG_READ { ($reg_name:expr) => { dm_read_reg(CTX, REG($reg_name)) }; }
#[macro_export]
macro_rules! REG_WRITE { ($reg_name:expr, $value:expr) => { dm_write_reg(CTX, REG($reg_name), $value) }; }
#[macro_export]
macro_rules! FN { ($reg_name:ident, $field:ident) => { FD(concat_idents!($reg_name, __, $field)) }; }

#[macro_export]
macro_rules! REG_SET_N { ($reg_name:expr, $n:expr, $initial_val:expr $(, $args:expr)*) => { generic_reg_set_ex(CTX, REG($reg_name), $initial_val, $n $(, $args)*) }; }
#[macro_export]
macro_rules! REG_SET { ($reg_name:expr, $initial_val:expr, $field:ident, $val:expr) => { REG_SET_N!($reg_name, 1, $initial_val, FN!($reg_name, $field), $val) }; }

#[macro_export]
macro_rules! REG_SET_N_FIELDS { ($reg:expr, $init:expr, $(($field:ident, $value:expr)),+) => { REG_SET_N!($reg, 0 $(, FN!($reg, $field), $value)*) }; }

#[macro_export]
macro_rules! REG_GET { ($reg_name:expr, $field:ident, $val:expr) => { generic_reg_get(CTX, REG($reg_name), FN!($reg_name, $field), $val as *mut u32) }; }
#[macro_export]
macro_rules! REG_GET_N { ($reg_name:expr, $n:expr, $(($field:ident, $val:expr)),+) => { generic_reg_get(CTX, REG($reg_name), 0, 0, $val as *mut u32) }; }
#[macro_export] macro_rules! REG_GET_2 { ($r:expr,$f1:ident,$v1:expr,$f2:ident,$v2:expr) => { generic_reg_get2(CTX,REG($r),FN!($r,$f1),$v1 as *mut u32,FN!($r,$f2),$v2 as *mut u32) }; }
#[macro_export] macro_rules! REG_GET_3 { ($r:expr,$f1:ident,$v1:expr,$f2:ident,$v2:expr,$f3:ident,$v3:expr) => { generic_reg_get3(CTX,REG($r),FN!($r,$f1),$v1 as *mut u32,FN!($r,$f2),$v2 as *mut u32,FN!($r,$f3),$v3 as *mut u32) }; }
#[macro_export] macro_rules! REG_GET_4 { ($r:expr,$f1:ident,$v1:expr,$f2:ident,$v2:expr,$f3:ident,$v3:expr,$f4:ident,$v4:expr) => { generic_reg_get4(CTX,REG($r),FN!($r,$f1),$v1 as *mut u32,FN!($r,$f2),$v2 as *mut u32,FN!($r,$f3),$v3 as *mut u32,FN!($r,$f4),$v4 as *mut u32) }; }
#[macro_export] macro_rules! REG_GET_5 { ($r:expr,$f1:ident,$v1:expr,$f2:ident,$v2:expr,$f3:ident,$v3:expr,$f4:ident,$v4:expr,$f5:ident,$v5:expr) => { generic_reg_get5(CTX,REG($r),FN!($r,$f1),$v1 as *mut u32,FN!($r,$f2),$v2 as *mut u32,FN!($r,$f3),$v3 as *mut u32,FN!($r,$f4),$v4 as *mut u32,FN!($r,$f5),$v5 as *mut u32) }; }
#[macro_export] macro_rules! REG_GET_6 { ($r:expr,$f1:ident,$v1:expr,$f2:ident,$v2:expr,$f3:ident,$v3:expr,$f4:ident,$v4:expr,$f5:ident,$v5:expr,$f6:ident,$v6:expr) => { generic_reg_get6(CTX,REG($r),FN!($r,$f1),$v1 as *mut u32,FN!($r,$f2),$v2 as *mut u32,FN!($r,$f3),$v3 as *mut u32,FN!($r,$f4),$v4 as *mut u32,FN!($r,$f5),$v5 as *mut u32,FN!($r,$f6),$v6 as *mut u32) }; }
#[macro_export] macro_rules! REG_GET_7 { ($r:expr,$f1:ident,$v1:expr,$f2:ident,$v2:expr,$f3:ident,$v3:expr,$f4:ident,$v4:expr,$f5:ident,$v5:expr,$f6:ident,$v6:expr,$f7:ident,$v7:expr) => { generic_reg_get7(CTX,REG($r),FN!($r,$f1),$v1 as *mut u32,FN!($r,$f2),$v2 as *mut u32,FN!($r,$f3),$v3 as *mut u32,FN!($r,$f4),$v4 as *mut u32,FN!($r,$f5),$v5 as *mut u32,FN!($r,$f6),$v6 as *mut u32,FN!($r,$f7),$v7 as *mut u32) }; }
#[macro_export] macro_rules! REG_GET_8 { ($r:expr,$f1:ident,$v1:expr,$f2:ident,$v2:expr,$f3:ident,$v3:expr,$f4:ident,$v4:expr,$f5:ident,$v5:expr,$f6:ident,$v6:expr,$f7:ident,$v7:expr,$f8:ident,$v8:expr) => { generic_reg_get8(CTX,REG($r),FN!($r,$f1),$v1 as *mut u32,FN!($r,$f2),$v2 as *mut u32,FN!($r,$f3),$v3 as *mut u32,FN!($r,$f4),$v4 as *mut u32,FN!($r,$f5),$v5 as *mut u32,FN!($r,$f6),$v6 as *mut u32,FN!($r,$f7),$v7 as *mut u32,FN!($r,$f8),$v8 as *mut u32) }; }
#[macro_export]
macro_rules! REG_WAIT { ($reg_name:expr, $field:ident, $val:expr, $delay:expr, $max_try:expr) => { generic_reg_wait(CTX, REG($reg_name), FN!($reg_name, $field), $val, $delay, $max_try, "", line!()) }; }
#[macro_export]
macro_rules! REG_UPDATE_N { ($reg_name:expr, $n:expr $(, $args:expr)*) => { generic_reg_update_ex(CTX, REG($reg_name), $n $(, $args)*) }; }
#[macro_export]
macro_rules! REG_UPDATE { ($reg_name:expr, $field:ident, $val:expr) => { REG_UPDATE_N!($reg_name, 1, FN!($reg_name, $field), $val) }; }

macro_rules! reg_set_many { ($name:ident, $n:expr, $( $f:ident, $v:ident ),+) => { #[macro_export] macro_rules! $name { ($reg:expr, $init:expr, $( $f:ident, $v:expr ),+) => { REG_SET_N!($reg, $n, $init $(, FN!($reg, $f), $v)*) }; } }; }
reg_set_many!(REG_SET_2, 2, f1, v1, f2, v2);
reg_set_many!(REG_SET_3, 3, f1, v1, f2, v2, f3, v3);
reg_set_many!(REG_SET_4, 4, f1, v1, f2, v2, f3, v3, f4, v4);
reg_set_many!(REG_SET_5, 5, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5);
reg_set_many!(REG_SET_6, 6, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5, f6, v6);
reg_set_many!(REG_SET_7, 7, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5, f6, v6, f7, v7);
reg_set_many!(REG_SET_8, 8, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5, f6, v6, f7, v7, f8, v8);
reg_set_many!(REG_SET_9, 9, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5, f6, v6, f7, v7, f8, v8, f9, v9);
reg_set_many!(REG_SET_10, 10, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5, f6, v6, f7, v7, f8, v8, f9, v9, f10, v10);

macro_rules! reg_update_many { ($name:ident, $n:expr, $( $f:ident, $v:ident ),+) => { #[macro_export] macro_rules! $name { ($reg:expr, $( $f:ident, $v:expr ),+) => { REG_UPDATE_N!($reg, $n $(, FN!($reg, $f), $v)*) }; } }; }
reg_update_many!(REG_UPDATE_2, 2, f1, v1, f2, v2); reg_update_many!(REG_UPDATE_3, 3, f1, v1, f2, v2, f3, v3); reg_update_many!(REG_UPDATE_4, 4, f1, v1, f2, v2, f3, v3, f4, v4);
reg_update_many!(REG_UPDATE_5, 5, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5); reg_update_many!(REG_UPDATE_6, 6, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5, f6, v6);
reg_update_many!(REG_UPDATE_7, 7, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5, f6, v6, f7, v7); reg_update_many!(REG_UPDATE_8, 8, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5, f6, v6, f7, v7, f8, v8);
reg_update_many!(REG_UPDATE_9, 9, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5, f6, v6, f7, v7, f8, v8, f9, v9); reg_update_many!(REG_UPDATE_10, 10, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5, f6, v6, f7, v7, f8, v8, f9, v9, f10, v10);
reg_update_many!(REG_UPDATE_14, 14, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5, f6, v6, f7, v7, f8, v8, f9, v9, f10, v10, f11, v11, f12, v12, f13, v13, f14, v14);
reg_update_many!(REG_UPDATE_19, 19, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5, f6, v6, f7, v7, f8, v8, f9, v9, f10, v10, f11, v11, f12, v12, f13, v13, f14, v14, f15, v15, f16, v16, f17, v17, f18, v18, f19, v19);
reg_update_many!(REG_UPDATE_20, 20, f1, v1, f2, v2, f3, v3, f4, v4, f5, v5, f6, v6, f7, v7, f8, v8, f9, v9, f10, v10, f11, v11, f12, v12, f13, v13, f14, v14, f15, v15, f16, v16, f17, v17, f18, v18, f19, v19, f20, v20);

#[macro_export] macro_rules! REG_UPDATE_SEQ_2 { ($r:expr,$f1:ident,$v1:expr,$f2:ident,$v2:expr) => {{ let mut val:u32=REG_UPDATE!($r,$f1,$v1); val=REG_SET!($r,val,$f2,$v2); val }}; }
#[macro_export] macro_rules! REG_UPDATE_SEQ_3 { ($r:expr,$f1:ident,$v1:expr,$f2:ident,$v2:expr,$f3:ident,$v3:expr) => {{ let mut val:u32=REG_UPDATE!($r,$f1,$v1); val=REG_SET!($r,val,$f2,$v2); REG_SET!($r,val,$f3,$v3) }}; }

#[macro_export] macro_rules! IX_REG_READ { ($i:expr,$d:expr,$index:expr) => { generic_read_indirect_reg(CTX,REG($i),REG($d),IND_REG($index)) }; }
#[macro_export] macro_rules! IX_REG_SET_N { ($i:expr,$d:expr,$index:expr,$n:expr,$initial:expr $(,$a:expr)*) => { generic_indirect_reg_update_ex(CTX,REG($i),REG($d),IND_REG($index),$initial,$n $(,$a)*) }; }
#[macro_export] macro_rules! IX_REG_GET_N { ($i:expr,$d:expr,$index:expr,$n:expr $(,$a:expr)*) => { generic_indirect_reg_get(CTX,REG($i),REG($d),IND_REG($index),$n $(,$a)*) }; }
#[macro_export] macro_rules! IX_REG_GET { ($i:expr,$d:expr,$index:expr,$field:ident,$val:expr) => { IX_REG_GET_N!($i,$d,$index,1,FN!($d,$field),$val) }; }
#[macro_export] macro_rules! IX_REG_UPDATE_N { ($i:expr,$d:expr,$index:expr,$n:expr $(,$a:expr)*) => { generic_indirect_reg_update_ex(CTX,REG($i),REG($d),IND_REG($index),IX_REG_READ!($i,$d,$index),$n $(,$a)*) }; }
#[macro_export] macro_rules! IX_REG_SET_N_SYNC { ($index:expr,$n:expr,$initial:expr $(,$a:expr)*) => { generic_indirect_reg_update_ex_sync(CTX,IND_REG($index),$initial,$n $(,$a)*) }; }
#[macro_export] macro_rules! IX_REG_GET_N_SYNC { ($index:expr,$n:expr $(,$a:expr)*) => { generic_indirect_reg_get_sync(CTX,IND_REG($index),$n $(,$a)*) }; }

extern "C" {
    pub fn generic_reg_get(ctx: *const dc_context, addr: u32, shift: u8, mask: u32, field_value: *mut u32) -> u32;
    pub fn generic_reg_get2(ctx: *const dc_context, addr: u32, shift1: u8, mask1: u32, field_value1: *mut u32, shift2: u8, mask2: u32, field_value2: *mut u32) -> u32;
    pub fn generic_reg_get3(ctx: *const dc_context, shift: u32, s1: u8, m1: u32, v1: *mut u32, s2: u8, m2: u32, v2: *mut u32, s3: u8, m3: u32, v3: *mut u32) -> u32;
    pub fn generic_reg_get4(ctx: *const dc_context, addr: u32, s1:u8,m1:u32,v1:*mut u32,s2:u8,m2:u32,v2:*mut u32,s3:u8,m3:u32,v3:*mut u32,s4:u8,m4:u32,v4:*mut u32)->u32;
    pub fn generic_reg_get5(ctx:*const dc_context,addr:u32,s1:u8,m1:u32,v1:*mut u32,s2:u8,m2:u32,v2:*mut u32,s3:u8,m3:u32,v3:*mut u32,s4:u8,m4:u32,v4:*mut u32,s5:u8,m5:u32,v5:*mut u32)->u32;
    pub fn generic_reg_get6(ctx:*const dc_context,addr:u32,s1:u8,m1:u32,v1:*mut u32,s2:u8,m2:u32,v2:*mut u32,s3:u8,m3:u32,v3:*mut u32,s4:u8,m4:u32,v4:*mut u32,s5:u8,m5:u32,v5:*mut u32,s6:u8,m6:u32,v6:*mut u32)->u32;
    pub fn generic_reg_get7(ctx:*const dc_context,addr:u32,s1:u8,m1:u32,v1:*mut u32,s2:u8,m2:u32,v2:*mut u32,s3:u8,m3:u32,v3:*mut u32,s4:u8,m4:u32,v4:*mut u32,s5:u8,m5:u32,v5:*mut u32,s6:u8,m6:u32,v6:*mut u32,s7:u8,m7:u32,v7:*mut u32)->u32;
    pub fn generic_reg_get8(ctx:*const dc_context,addr:u32,s1:u8,m1:u32,v1:*mut u32,s2:u8,m2:u32,v2:*mut u32,s3:u8,m3:u32,v3:*mut u32,s4:u8,m4:u32,v4:*mut u32,s5:u8,m5:u32,v5:*mut u32,s6:u8,m6:u32,v6:*mut u32,s7:u8,m7:u32,v7:*mut u32,s8:u8,m8:u32,v8:*mut u32)->u32;
    pub fn generic_write_indirect_reg(ctx:*const dc_context,addr_index:u32,addr_data:u32,index:u32,data:u32);
    pub fn generic_read_indirect_reg(ctx:*const dc_context,addr_index:u32,addr_data:u32,index:u32)->u32;
    pub fn generic_indirect_reg_get(ctx:*const dc_context,addr_index:u32,addr_data:u32,index:u32,n:i32,shift1:u8,mask1:u32,field_value1:*mut u32,...)->u32;
    pub fn generic_indirect_reg_update_ex(ctx:*const dc_context,addr_index:u32,addr_data:u32,index:u32,reg_val:u32,n:i32,shift1:u8,mask1:u32,field_value1:u32,...)->u32;
    pub fn generic_indirect_reg_get_sync(ctx:*const dc_context,index:u32,n:i32,shift1:u8,mask1:u32,field_value1:*mut u32,...)->u32;
    pub fn generic_indirect_reg_update_ex_sync(ctx:*const dc_context,index:u32,reg_val:u32,n:i32,shift1:u8,mask1:u32,field_value1:u32,...)->u32;
}

#[allow(non_camel_case_types)] pub enum dc_context {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
