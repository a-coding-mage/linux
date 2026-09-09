/* SPDX-License-Identifier: GPL-2.0+ */
/*
 *  comedi/drivers/ni_routing/ni_device_routes.c
 *  List of valid routes for specific NI boards.
 *
 *  COMEDI - Linux Control and Measurement Device Interface
 *  Copyright (C) 2016 Spencer E. Olson <olsonse@umich.edu>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 */

/*
 * This file is meant to be included by comedi/drivers/ni_routes.c
 */

/* Dependency corresponding to: #include "../ni_routes.h" */

#[allow(non_camel_case_types)]
pub struct ni_device_routes;

extern "C" {
    pub static ni_device_routes_list: [*mut ni_device_routes; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
