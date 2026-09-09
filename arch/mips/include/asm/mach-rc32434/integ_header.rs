/*
 *  Definitions for the Watchdog registers
 *
 *  Copyright 2002 Ryan Holm <ryan.holmQVist@idt.com>
 *  Copyright 2008 Florian Fainelli <florian@openwrt.org>
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under  the terms of  the GNU General  Public License as published by
 *  the Free Software Foundation;  either version 2 of the  License, or
 *  (at your option) any later version.
 *
 *  THIS  SOFTWARE  IS  PROVIDED   ``AS  IS''  AND   ANY  EXPRESS OR IMPLIED
 *  WARRANTIES,   INCLUDING, BUT NOT  LIMITED  TO, THE IMPLIED WARRANTIES OF
 *  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN
 *  NO  EVENT  SHALL   THE AUTHOR  BE    LIABLE FOR ANY   DIRECT, INDIRECT,
 *  INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *  NOT LIMITED   TO, PROCUREMENT OF  SUBSTITUTE GOODS  OR SERVICES; LOSS OF
 *  USE, DATA,  OR PROFITS; OR  BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
 *  ANY THEORY OF LIABILITY, WHETHER IN  CONTRACT, STRICT LIABILITY, OR TORT
 *  (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 *  THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 *  You should have received a copy of the  GNU General Public License along
 *  with this program; if not, write  to the Free Software Foundation, Inc.,
 *  675 Mass Ave, Cambridge, MA 02139, USA.
 *
 */

// Dependency supplied by the surrounding translation.

pub const INTEG0_BASE_ADDR: u32 = 0x18030030;

#[repr(C)]
pub struct integ {
    pub errcs: u32,     /* sticky use ERRCS_ */
    pub wtcount: u32,  /* Watchdog timer count reg. */
    pub wtcompare: u32, /* Watchdog timer timeout value. */
    pub wtc: u32,      /* Watchdog timer control. use WTC_ */
}

/* Error counters */
pub const RC32434_ERR_WTO: u32 = 0;
pub const RC32434_ERR_WNE: u32 = 1;
pub const RC32434_ERR_UCW: u32 = 2;
pub const RC32434_ERR_UCR: u32 = 3;
pub const RC32434_ERR_UPW: u32 = 4;
pub const RC32434_ERR_UPR: u32 = 5;
pub const RC32434_ERR_UDW: u32 = 6;
pub const RC32434_ERR_UDR: u32 = 7;
pub const RC32434_ERR_SAE: u32 = 8;
pub const RC32434_ERR_WRE: u32 = 9;

/* Watchdog control bits */
pub const RC32434_WTC_EN: u32 = 0;
pub const RC32434_WTC_TO: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
