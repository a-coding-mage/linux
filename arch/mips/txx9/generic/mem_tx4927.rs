/*
 * common tx4927 memory interface
 *
 * Author: MontaVista Software, Inc.
 *	   source@mvista.com
 *
 * Copyright 2001-2002 MontaVista Software Inc.
 *
 *  This program is free software; you can redistribute it and/or modify it
 *  under the terms of the GNU General Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 *
 *  THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 *  WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 *  IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 *  INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 *  BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS
 *  OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 *  ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
 *  TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
 *  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 *  You should have received a copy of the GNU General Public License along
 *  with this program; if not, write to the Free Software Foundation, Inc.,
 *  675 Mass Ave, Cambridge, MA 02139, USA.
 */

// Kernel headers: linux/init.h, linux/types.h, linux/io.h,
// and asm/txx9/tx4927.h provide the corresponding external definitions.

#[repr(C)]
pub struct Tx4927Sdramc {
    pub cr: [u64; 4],
}

extern "C" {
    pub static mut tx4927_sdramcptr: *mut Tx4927Sdramc;
    pub fn __raw_readq(addr: *const u64) -> u64;
}

unsafe fn tx4927_process_sdccr(addr: *const u64) -> u32 {
    let val: u64;
    let sdccr_ce: u32;
    let sdccr_bs: u32;
    let sdccr_rs: u32;
    let sdccr_cs: u32;
    let sdccr_mw: u32;
    let mut bs: u32 = 0;
    let mut rs: u32 = 0;
    let mut cs: u32 = 0;
    let mut mw: u32 = 0;

    val = __raw_readq(addr);

    /* MVMCP -- need #defs for these bits masks */
    sdccr_ce = ((val & (1u64 << 10)) >> 10) as u32;
    sdccr_bs = ((val & (1u64 << 8)) >> 8) as u32;
    sdccr_rs = ((val & (3u64 << 5)) >> 5) as u32;
    sdccr_cs = ((val & (7u64 << 2)) >> 2) as u32;
    sdccr_mw = ((val & (1u64 << 0)) >> 0) as u32;

    if sdccr_ce != 0 {
        bs = 2u32 << sdccr_bs;
        rs = 2048u32 << sdccr_rs;
        cs = 256u32 << sdccr_cs;
        mw = 8u32 >> sdccr_mw;
    }

    rs * cs * mw * bs
}

pub unsafe fn tx4927_get_mem_size() -> u32 {
    let mut total: u32 = 0;
    let mut i: usize = 0;

    while i < (*tx4927_sdramcptr).cr.len() {
        total += tx4927_process_sdccr(&(*tx4927_sdramcptr).cr[i] as *const u64);
        i += 1;
    }
    total
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
