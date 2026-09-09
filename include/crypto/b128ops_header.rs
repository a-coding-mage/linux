/* b128ops.h - common 128-bit block operations
 *
 * Copyright (c) 2003, Dr Brian Gladman, Worcester, UK.
 * Copyright (c) 2006, Rik Snel <rsnel@cube.dyndns.org>
 *
 * Based on Dr Brian Gladman's (GPL'd) work published at
 * http://fp.gladman.plus.com/cryptography_technology/index.htm
 * See the original copyright notice below.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation; either version 2 of the License, or (at your option)
 * any later version.
 */
/*
 ---------------------------------------------------------------------------
 Copyright (c) 2003, Dr Brian Gladman, Worcester, UK.   All rights reserved.

 LICENSE TERMS

 The free distribution and use of this software in both source and binary
 form is allowed (with or without changes) provided that:

   1. distributions of this source code include the above copyright
      notice, this list of conditions and the following disclaimer;

   2. distributions in binary form include the above copyright
      notice, this list of conditions and the following disclaimer
      in the documentation and/or other associated materials;

   3. the copyright holder's name is not used to endorse products
      built using this software without specific written permission.

 ALTERNATIVELY, provided that this notice is retained in full, this product
 may be distributed under the terms of the GNU General Public License (GPL),
 in which case the provisions of the GPL apply INSTEAD OF those given above.

 DISCLAIMER

 This software is provided 'as is' with no explicit or implied warranties
 in respect of its properties, including, but not limited to, correctness
 and/or fitness for purpose.
 ---------------------------------------------------------------------------
 Issue Date: 13/06/2006
*/

// The C header includes <linux/types.h>; __be64 and __le64 are supplied by
// the surrounding translation unit.

#[repr(C)]
pub struct be128 {
    pub a: __be64,
    pub b: __be64,
}

#[repr(C)]
pub struct le128 {
    pub b: __le64,
    pub a: __le64,
}

pub unsafe fn be128_xor(r: *mut be128, p: *const be128, q: *const be128) {
    (*r).a = (*p).a ^ (*q).a;
    (*r).b = (*p).b ^ (*q).b;
}

pub unsafe fn le128_xor(r: *mut le128, p: *const le128, q: *const le128) {
    (*r).a = (*p).a ^ (*q).a;
    (*r).b = (*p).b ^ (*q).b;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
