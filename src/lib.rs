/* This Source Code Form is subject to the terms of the Mozilla Public
 *  * License, v. 2.0. If a copy of the MPL was not distributed with this
 *   * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(all(target_os = "linux", feature = "bluetooth"))]
extern crate blurz;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
extern crate blurdroid;
#[cfg(all(target_os = "macos", feature = "bluetooth"))]
extern crate blurmac;
#[cfg(feature = "bluetooth-test")]
extern crate blurmock;
pub mod adapter;
pub mod bluetooth;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
              all(target_os = "android", feature = "bluetooth"),
              all(target_os = "macos", feature = "bluetooth"))))]
mod empty;
mod macros;