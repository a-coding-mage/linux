/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

#[repr(C)]
pub struct irq_service_init_data {
    pub ctx: *mut dc_context,
}

pub enum dc_context {}
pub enum irq_service {}
pub enum dc_irq_source {}

extern "C" {
    pub fn dal_irq_service_destroy(irq_service: *mut *mut irq_service);

    pub fn dal_irq_service_set(
        irq_service: *mut irq_service,
        source: dc_irq_source,
        enable: bool,
    ) -> bool;

    pub fn dal_irq_service_ack(
        irq_service: *mut irq_service,
        source: dc_irq_source,
    ) -> bool;

    pub fn dal_irq_service_to_irq_source(
        irq_service: *mut irq_service,
        src_id: u32,
        ext_id: u32,
    ) -> dc_irq_source;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
