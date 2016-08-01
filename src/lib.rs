/* This Source Code Form is subject to the terms of the Mozilla Public
 *  * License, v. 2.0. If a copy of the MPL was not distributed with this
 *   * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(all(target_os = "linux", feature = "bluetooth"))]
extern crate blurz;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
extern crate blurdroid;

pub mod bluetooth;
#[cfg(not(any(target_os = "linux", target_os = "android")))]
mod empty;
