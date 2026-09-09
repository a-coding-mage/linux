/*
** linux/machw.h -- This header defines some macros and pointers for
**                    the various Macintosh custom hardware registers.
**
** Copyright 1997 by Michael Schmitz
**
** This file is subject to the terms and conditions of the GNU General Public
** License.  See the file COPYING in the main directory of this archive
** for more details.
**
*/

/*
 * head.S maps the videomem to VIDEOMEMBASE
 */

pub const VIDEOMEMBASE: u32 = 0xf000_0000;
pub const VIDEOMEMSIZE: i32 = 4096 * 1024;
pub const VIDEOMEMMASK: i32 = -4096 * 1024;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
