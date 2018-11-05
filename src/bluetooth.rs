/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_adapter::BluetoothAdapter as BluetoothAdapterBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_adapter::Adapter as BluetoothAdapterAndroid;
#[cfg(all(target_os = "macos", feature = "bluetooth"))]
use blurmac::BluetoothAdapter as BluetoothAdapterMac;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
              all(target_os = "android", feature = "bluetooth"),
              all(target_os = "macos", feature = "bluetooth"))))]
use empty::BluetoothAdapter as BluetoothAdapterEmpty;
#[cfg(feature = "bluetooth-test")]
use blurmock::fake_adapter::FakeBluetoothAdapter;

#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_device::BluetoothDevice as BluetoothDeviceBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_device::Device as BluetoothDeviceAndroid;
#[cfg(all(target_os = "macos", feature = "bluetooth"))]
use blurmac::BluetoothDevice as BluetoothDeviceMac;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
              all(target_os = "android", feature = "bluetooth"),
              all(target_os = "macos", feature = "bluetooth"))))]
use empty::BluetoothDevice as BluetoothDeviceEmpty;
#[cfg(feature = "bluetooth-test")]
use blurmock::fake_device::FakeBluetoothDevice;

#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_gatt_characteristic::BluetoothGATTCharacteristic as BluetoothGATTCharacteristicBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_gatt_characteristic::Characteristic as BluetoothGATTCharacteristicAndroid;
#[cfg(all(target_os = "macos", feature = "bluetooth"))]
use blurmac::BluetoothGATTCharacteristic as BluetoothGATTCharacteristicMac;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
              all(target_os = "android", feature = "bluetooth"),
              all(target_os = "macos", feature = "bluetooth"))))]
use empty::BluetoothGATTCharacteristic as BluetoothGATTCharacteristicEmpty;
#[cfg(feature = "bluetooth-test")]
use blurmock::fake_characteristic::FakeBluetoothGATTCharacteristic;

#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_gatt_descriptor::BluetoothGATTDescriptor as BluetoothGATTDescriptorBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_gatt_descriptor::Descriptor as BluetoothGATTDescriptorAndroid;
#[cfg(all(target_os = "macos", feature = "bluetooth"))]
use blurmac::BluetoothGATTDescriptor as BluetoothGATTDescriptorMac;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
              all(target_os = "android", feature = "bluetooth"),
              all(target_os = "macos", feature = "bluetooth"))))]
use empty::BluetoothGATTDescriptor as BluetoothGATTDescriptorEmpty;
#[cfg(feature = "bluetooth-test")]
use blurmock::fake_descriptor::FakeBluetoothGATTDescriptor;

#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_gatt_service::BluetoothGATTService as BluetoothGATTServiceBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_gatt_service::Service as BluetoothGATTServiceAndroid;
#[cfg(all(target_os = "macos", feature = "bluetooth"))]
use blurmac::BluetoothGATTService as BluetoothGATTServiceMac;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
              all(target_os = "android", feature = "bluetooth"),
              all(target_os = "macos", feature = "bluetooth"))))]
use empty::BluetoothGATTService as BluetoothGATTServiceEmpty;
#[cfg(feature = "bluetooth-test")]
use blurmock::fake_service::FakeBluetoothGATTService;

#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as BluetoothDiscoverySessionBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_discovery_session::DiscoverySession as BluetoothDiscoverySessionAndroid;
#[cfg(all(target_os = "macos", feature = "bluetooth"))]
use blurmac::BluetoothDiscoverySession as BluetoothDiscoverySessionMac;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
              all(target_os = "android", feature = "bluetooth"),
              all(target_os = "macos", feature = "bluetooth"))))]
use empty::BluetoothDiscoverySession as BluetoothDiscoverySessionEmpty;
#[cfg(feature = "bluetooth-test")]
use blurmock::fake_discovery_session::FakeBluetoothDiscoverySession;

use std::collections::HashMap;
use std::sync::Arc;
use std::error::Error;

#[cfg(feature = "bluetooth-test")]
const NOT_SUPPORTED_ON_REAL_ERROR: &'static str = "Error! Test functions are not supported on real devices!";
#[cfg(feature = "bluetooth-test")]
const NOT_SUPPORTED_ON_MOCK_ERROR: &'static str = "Error! The first parameter must be a mock structure!";

#[derive(Clone, Debug)]
pub enum BluetoothAdapter {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    Bluez(Arc<BluetoothAdapterBluez>),
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    Android(Arc<BluetoothAdapterAndroid>),
    #[cfg(all(target_os = "macos", feature = "bluetooth"))]
    Mac(Arc<BluetoothAdapterMac>),
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                  all(target_os = "android", feature = "bluetooth"),
                  all(target_os = "macos", feature = "bluetooth"))))]
    Empty(Arc<BluetoothAdapterEmpty>),
    #[cfg(feature = "bluetooth-test")]
    Mock(Arc<FakeBluetoothAdapter>),
}

#[derive(Debug)]
pub enum BluetoothDiscoverySession {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    Bluez(Arc<BluetoothDiscoverySessionBluez>),
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    Android(Arc<BluetoothDiscoverySessionAndroid>),
    #[cfg(all(target_os = "macos", feature = "bluetooth"))]
    Mac(Arc<BluetoothDiscoverySessionMac>),
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                  all(target_os = "android", feature = "bluetooth"),
                  all(target_os = "macos", feature = "bluetooth"))))]
    Empty(Arc<BluetoothDiscoverySessionEmpty>),
    #[cfg(feature = "bluetooth-test")]
    Mock(Arc<FakeBluetoothDiscoverySession>),
}

#[derive(Clone, Debug)]
pub enum BluetoothDevice {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    Bluez(Arc<BluetoothDeviceBluez>),
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    Android(Arc<BluetoothDeviceAndroid>),
    #[cfg(all(target_os = "macos", feature = "bluetooth"))]
    Mac(Arc<BluetoothDeviceMac>),
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                  all(target_os = "android", feature = "bluetooth"),
                  all(target_os = "macos", feature = "bluetooth"))))]
    Empty(Arc<BluetoothDeviceEmpty>),
    #[cfg(feature = "bluetooth-test")]
    Mock(Arc<FakeBluetoothDevice>),
}

#[derive(Clone, Debug)]
pub enum BluetoothGATTService {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    Bluez(Arc<BluetoothGATTServiceBluez>),
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    Android(Arc<BluetoothGATTServiceAndroid>),
    #[cfg(all(target_os = "macos", feature = "bluetooth"))]
    Mac(Arc<BluetoothGATTServiceMac>),
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                  all(target_os = "android", feature = "bluetooth"),
                  all(target_os = "macos", feature = "bluetooth"))))]
    Empty(Arc<BluetoothGATTServiceEmpty>),
    #[cfg(feature = "bluetooth-test")]
    Mock(Arc<FakeBluetoothGATTService>),
}

#[derive(Clone, Debug)]
pub enum BluetoothGATTCharacteristic {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    Bluez(Arc<BluetoothGATTCharacteristicBluez>),
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    Android(Arc<BluetoothGATTCharacteristicAndroid>),
    #[cfg(all(target_os = "macos", feature = "bluetooth"))]
    Mac(Arc<BluetoothGATTCharacteristicMac>),
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                  all(target_os = "android", feature = "bluetooth"),
                  all(target_os = "macos", feature = "bluetooth"))))]
    Empty(Arc<BluetoothGATTCharacteristicEmpty>),
    #[cfg(feature = "bluetooth-test")]
    Mock(Arc<FakeBluetoothGATTCharacteristic>),
}

#[derive(Clone, Debug)]
pub enum BluetoothGATTDescriptor {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    Bluez(Arc<BluetoothGATTDescriptorBluez>),
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    Android(Arc<BluetoothGATTDescriptorAndroid>),
    #[cfg(all(target_os = "macos", feature = "bluetooth"))]
    Mac(Arc<BluetoothGATTDescriptorMac>),
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                  all(target_os = "android", feature = "bluetooth"),
                  all(target_os = "macos", feature = "bluetooth"))))]
    Empty(Arc<BluetoothGATTDescriptorEmpty>),
    #[cfg(feature = "bluetooth-test")]
    Mock(Arc<FakeBluetoothGATTDescriptor>),
}

macro_rules! get_inner_and_call(
    ($enum_value: expr, $enum_type: ident, $function_name: ident) => {
        match $enum_value {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &$enum_type::Bluez(ref bluez) => bluez.$function_name(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &$enum_type::Android(ref android) => android.$function_name(),
            #[cfg(all(target_os = "macos", feature = "bluetooth"))]
            &$enum_type::Mac(ref mac) => mac.$function_name(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                          all(target_os = "android", feature = "bluetooth"),
                          all(target_os = "macos", feature = "bluetooth"))))]
            &$enum_type::Empty(ref empty) => empty.$function_name(),
            #[cfg(feature = "bluetooth-test")]
            &$enum_type::Mock(ref fake) => fake.$function_name(),
        }
    };

    (@with_bluez_offset, $enum_value: expr, $enum_type: ident, $function_name: ident) => {
        match $enum_value {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &$enum_type::Bluez(ref bluez) => bluez.$function_name(None),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &$enum_type::Android(ref android) => android.$function_name(),
            #[cfg(all(target_os = "macos", feature = "bluetooth"))]
            &$enum_type::Mac(ref mac) => mac.$function_name(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                          all(target_os = "android", feature = "bluetooth"),
                          all(target_os = "macos", feature = "bluetooth"))))]
            &$enum_type::Empty(ref empty) => empty.$function_name(),
            #[cfg(feature = "bluetooth-test")]
            &$enum_type::Mock(ref fake) => fake.$function_name(),
        }
    };

    ($enum_value: expr, $enum_type: ident, $function_name: ident, $value: expr) => {
        match $enum_value {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &$enum_type::Bluez(ref bluez) => bluez.$function_name($value),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &$enum_type::Android(ref android) => android.$function_name($value),
            #[cfg(all(target_os = "macos", feature = "bluetooth"))]
            &$enum_type::Mac(ref mac) => mac.$function_name($value),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                          all(target_os = "android", feature = "bluetooth"),
                          all(target_os = "macos", feature = "bluetooth"))))]
            &$enum_type::Empty(ref empty) => empty.$function_name($value),
            #[cfg(feature = "bluetooth-test")]
            &$enum_type::Mock(ref fake) => fake.$function_name($value),
        }
    };

    (@with_bluez_offset, $enum_value: expr, $enum_type: ident, $function_name: ident, $value: expr) => {
        match $enum_value {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &$enum_type::Bluez(ref bluez) => bluez.$function_name($value, None),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &$enum_type::Android(ref android) => android.$function_name($value),
            #[cfg(all(target_os = "macos", feature = "bluetooth"))]
            &$enum_type::Mac(ref mac) => mac.$function_name($value),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                          all(target_os = "android", feature = "bluetooth"),
                          all(target_os = "macos", feature = "bluetooth"))))]
            &$enum_type::Empty(ref empty) => empty.$function_name($value),
            #[cfg(feature = "bluetooth-test")]
            &$enum_type::Mock(ref fake) => fake.$function_name($value),
        }
    };
);

#[cfg(feature = "bluetooth-test")]
macro_rules! get_inner_and_call_test_func {
    ($enum_value: expr, $enum_type: ident, $function_name: ident, $value: expr) => {
        match $enum_value {
            &$enum_type::Mock(ref fake) => fake.$function_name($value),
            _ => Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR)),
        }
    };

    ($enum_value: expr, $enum_type: ident, $function_name: ident) => {
        match $enum_value {
            &$enum_type::Mock(ref fake) => fake.$function_name(),
            _ => Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR)),
        }
    };
}

impl BluetoothAdapter {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        let bluez_adapter = try!(BluetoothAdapterBluez::init());
        Ok(BluetoothAdapter::Bluez(Arc::new(bluez_adapter)))
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        let blurdroid_adapter = try!(BluetoothAdapterAndroid::get_adapter());
        Ok(BluetoothAdapter::Android(Arc::new(blurdroid_adapter)))
    }

    #[cfg(all(target_os = "macos", feature = "bluetooth"))]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        let mac_adapter = try!(BluetoothAdapterMac::init());
        Ok(BluetoothAdapter::Mac(Arc::new(mac_adapter)))
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                  all(target_os = "android", feature = "bluetooth"),
                  all(target_os = "macos", feature = "bluetooth"))))]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        let adapter = try!(BluetoothAdapterEmpty::init());
        Ok(BluetoothAdapter::Empty(Arc::new(adapter)))
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn init_mock() -> Result<BluetoothAdapter, Box<Error>> {
        Ok(BluetoothAdapter::Mock(FakeBluetoothAdapter::new_empty()))
    }

    pub fn get_id(&self) -> String {
        get_inner_and_call!(self, BluetoothAdapter, get_id)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_id(&self, id: String) {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_id(id),
            _ => (),
        }
    }

    pub fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>> {
        let device_list = try!(get_inner_and_call!(self, BluetoothAdapter, get_device_list));
        Ok(device_list.into_iter().map(|device| BluetoothDevice::create_device(self.clone(), device)).collect())
    }

    pub fn get_device(&self, address: String) -> Result<Option<BluetoothDevice>, Box<Error>> {
        let devices = try!(self.get_devices());
        for device in devices {
            if try!(device.get_address()) == address {
                return Ok(Some(device));
            }
        }
        Ok(None)
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, get_address)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_address(&self, address: String) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_address(address),
            _ => Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR)),
        }
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, get_name)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_name(&self, name: String) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_name, name)
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, get_alias)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_alias(&self, alias: String) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_alias, alias)
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, get_class)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_class(&self, class: u32) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_class, class)
    }

    pub fn is_powered(&self) -> Result<bool, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, is_powered)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_powered, powered)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn is_present(&self) -> Result<bool, Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, is_present)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_present(&self, present: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_present, present)
    }

    pub fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, is_discoverable)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_discoverable(&self, discoverable: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_discoverable, discoverable)
    }

    pub fn is_pairable(&self) -> Result<bool, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, is_pairable)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_pairable(&self, pairable: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_pairable, pairable)
    }

    pub fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, get_pairable_timeout)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_pairable_timeout(&self, timeout: u32) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_pairable_timeout, timeout)
    }

    pub fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, get_discoverable_timeout)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_discoverable_timeout(&self, timeout: u32) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_discoverable_timeout, timeout)
    }

    pub fn is_discovering(&self) -> Result<bool, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, is_discovering)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_discovering(&self, discovering: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_discovering, discovering)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_can_start_discovery(&self, can_start_discovery: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_can_start_discovery, can_start_discovery)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_can_stop_discovery(&self, can_stop_discovery: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_can_stop_discovery, can_stop_discovery)
    }

    pub fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        BluetoothDiscoverySession::create_session(self.clone())
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, get_uuids)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_uuids, uuids)
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, get_vendor_id_source)
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, get_vendor_id)
    }

    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, get_product_id)
    }

    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, get_device_id)
    }

    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        get_inner_and_call!(self, BluetoothAdapter, get_modalias)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_modalias(&self, modalias: String) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_modalias, modalias)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn get_ad_datas(&self) -> Result<Vec<String>, Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, get_ad_datas)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_ad_datas(&self, ad_datas: Vec<String>) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothAdapter, set_ad_datas, ad_datas)
    }
}

impl BluetoothDiscoverySession {
    fn create_session(adapter: BluetoothAdapter) -> Result<BluetoothDiscoverySession, Box<Error>> {
        match adapter {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            BluetoothAdapter::Bluez(bluez_adapter) => {
                let bluez_session = try!(BluetoothDiscoverySessionBluez::create_session(bluez_adapter.get_id()));
                Ok(BluetoothDiscoverySession::Bluez(Arc::new(bluez_session)))
            },
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            BluetoothAdapter::Android(android_adapter) => {
                let blurdroid_session = try!(BluetoothDiscoverySessionAndroid::create_session(android_adapter));
                Ok(BluetoothDiscoverySession::Android(Arc::new(blurdroid_session)))
            },
            #[cfg(all(target_os = "macos", feature = "bluetooth"))]
            BluetoothAdapter::Mac(mac_adapter) => {
                let mac_session = try!(BluetoothDiscoverySessionMac::create_session(mac_adapter));
                Ok(BluetoothDiscoverySession::Mac(Arc::new(mac_session)))
            },
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                          all(target_os = "android", feature = "bluetooth"),
                          all(target_os = "macos", feature = "bluetooth"))))]
            BluetoothAdapter::Empty(adapter) => {
                let empty_session = try!(BluetoothDiscoverySessionEmpty::create_session(adapter));
                Ok(BluetoothDiscoverySession::Empty(Arc::new(empty_session)))
            },
            #[cfg(feature = "bluetooth-test")]
            BluetoothAdapter::Mock(fake_adapter) => {
                let test_session = try!(FakeBluetoothDiscoverySession::create_session(fake_adapter));
                Ok(BluetoothDiscoverySession::Mock(Arc::new(test_session)))
            },
        }
    }

    pub fn start_discovery(&self) -> Result<(), Box<Error>> {
        get_inner_and_call!(self, BluetoothDiscoverySession, start_discovery)
    }

    pub fn stop_discovery(&self) -> Result<(), Box<Error>> {
        get_inner_and_call!(self, BluetoothDiscoverySession, stop_discovery)
    }
}

impl BluetoothDevice {

    fn create_device(adapter: BluetoothAdapter, device: String) -> BluetoothDevice {
        match adapter {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            BluetoothAdapter::Bluez(_bluez_adapter) => {
                BluetoothDevice::Bluez(Arc::new(BluetoothDeviceBluez::new(device)))
            },
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            BluetoothAdapter::Android(android_adapter) => {
                BluetoothDevice::Android(Arc::new(BluetoothDeviceAndroid::new(android_adapter, device)))
            },
            #[cfg(all(target_os = "macos", feature = "bluetooth"))]
            BluetoothAdapter::Mac(mac_adapter) => {
                BluetoothDevice::Mac(Arc::new(BluetoothDeviceMac::new(mac_adapter, device)))
            },
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                          all(target_os = "android", feature = "bluetooth"),
                          all(target_os = "macos", feature = "bluetooth"))))]
            BluetoothAdapter::Empty(_adapter) => {
                BluetoothDevice::Empty(Arc::new(BluetoothDeviceEmpty::new(device)))
            },
            #[cfg(feature = "bluetooth-test")]
            BluetoothAdapter::Mock(fake_adapter) => {
                BluetoothDevice::Mock(FakeBluetoothDevice::new_empty(fake_adapter, device))
            },
        }
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn create_mock_device(adapter: BluetoothAdapter, device: String) -> Result<BluetoothDevice, Box<Error>> {
        match adapter {
            BluetoothAdapter::Mock(fake_adapter) => {
                Ok(BluetoothDevice::Mock(FakeBluetoothDevice::new_empty(fake_adapter, device)))
            },
            _ => {
                Err(Box::from(NOT_SUPPORTED_ON_MOCK_ERROR))
            },
        }
    }

    pub fn get_id(&self) -> String {
        get_inner_and_call!(self, BluetoothDevice, get_id)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_id(&self, id: String) {
        match self {
            &BluetoothDevice::Mock(ref fake_adapter) => fake_adapter.set_id(id),
            _ => (),
        }
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_address)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_address(&self, address: String) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_address, address)
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_name)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_name(&self, name: Option<String>) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_name, name)
    }

    pub fn get_icon(&self) -> Result<String, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_icon)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_icon(&self, icon: String) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_icon, icon)
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_class)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_class(&self, class: u32) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_class, class)
    }

    pub fn get_appearance(&self) -> Result<u16, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_appearance)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_appearance(&self, appearance: u16) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_appearance, Some(appearance))
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_uuids)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_uuids, uuids)
    }

    pub fn is_paired(&self) -> Result<bool, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, is_paired)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_paired(&self, paired: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_paired, paired)
    }

    pub fn is_connected(&self) -> Result<bool, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, is_connected)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_connected(&self, connected: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_connected, connected)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn is_connectable(&self) -> Result<bool, Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, is_connectable)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_connectable(&self, connectable: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_connectable, connectable)
    }

    pub fn is_trusted(&self) -> Result<bool, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, is_trusted)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_trusted(&self, trusted: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_trusted, trusted)
    }

    pub fn is_blocked(&self) -> Result<bool, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, is_blocked)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_blocked(&self, blocked: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_blocked, blocked)
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_alias)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_alias(&self, alias: String) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_alias, alias)
    }

    pub fn is_legacy_pairing(&self) -> Result<bool, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, is_legacy_pairing)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_legacy_pairing(&self, legacy_pairing: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_legacy_pairing, legacy_pairing)
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_vendor_id_source)
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_vendor_id)
    }

    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_product_id)
    }

    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_device_id)
    }

    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_modalias)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_modalias(&self, modalias: String) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_modalias, modalias)
    }

    pub fn get_rssi(&self) -> Result<i16, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_rssi)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_rssi(&self, rssi: i16) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_rssi, Some(rssi))
    }

    pub fn get_tx_power(&self) -> Result<i16, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_tx_power)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_tx_power(&self, tx_power: i16) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_tx_power, Some(tx_power))
    }

    pub fn get_manufacturer_data(&self) -> Result<HashMap<u16, Vec<u8>>, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_manufacturer_data)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_manufacturer_data(&self, manufacturer_data: HashMap<u16, Vec<u8>>) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_manufacturer_data, Some(manufacturer_data))
    }

    pub fn get_service_data(&self) -> Result<HashMap<String, Vec<u8>>, Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, get_service_data)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_service_data(&self, service_data: HashMap<String, Vec<u8>>) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothDevice, set_service_data, Some(service_data))
    }

    pub fn get_gatt_services(&self) -> Result<Vec<BluetoothGATTService>, Box<Error>> {
        let services = try!(get_inner_and_call!(self, BluetoothDevice, get_gatt_services));
        Ok(services.into_iter().map(|service| BluetoothGATTService::create_service(self.clone(), service)).collect())
    }

    pub fn connect(&self) -> Result<(), Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, connect)
    }

    pub fn disconnect(&self) -> Result<(), Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, disconnect)
    }

    pub fn connect_profile(&self, uuid: String) -> Result<(), Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, connect_profile, uuid)
    }

    pub fn disconnect_profile(&self, uuid: String) -> Result<(), Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, disconnect_profile, uuid)
    }

    pub fn pair(&self) -> Result<(), Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, pair)
    }

    pub fn cancel_pairing(&self) -> Result<(), Box<Error>> {
        get_inner_and_call!(self, BluetoothDevice, cancel_pairing)
    }
}

impl BluetoothGATTService {
    fn create_service(device: BluetoothDevice, service: String) -> BluetoothGATTService {
        match device {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            BluetoothDevice::Bluez(_bluez_device) => {
                BluetoothGATTService::Bluez(Arc::new(BluetoothGATTServiceBluez::new(service)))
            },
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            BluetoothDevice::Android(android_device) => {
                BluetoothGATTService::Android(Arc::new(BluetoothGATTServiceAndroid::new(android_device, service)))
            },
            #[cfg(all(target_os = "macos", feature = "bluetooth"))]
            BluetoothDevice::Mac(mac_device) => {
                BluetoothGATTService::Mac(Arc::new(BluetoothGATTServiceMac::new(mac_device, service)))
            },
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                          all(target_os = "android", feature = "bluetooth"),
                          all(target_os = "macos", feature = "bluetooth"))))]
            BluetoothDevice::Empty(_device) => {
                BluetoothGATTService::Empty(Arc::new(BluetoothGATTServiceEmpty::new(service)))
            },
            #[cfg(feature = "bluetooth-test")]
            BluetoothDevice::Mock(fake_device) => {
                BluetoothGATTService::Mock(FakeBluetoothGATTService::new_empty(fake_device, service))
            },
        }
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn create_mock_service(device: BluetoothDevice, service: String) -> Result<BluetoothGATTService, Box<Error>> {
        match device {
            BluetoothDevice::Mock(fake_device) => {
                Ok(BluetoothGATTService::Mock(FakeBluetoothGATTService::new_empty(fake_device, service)))
            },
            _ => {
                Err(Box::from(NOT_SUPPORTED_ON_MOCK_ERROR))
            },
        }
    }

    pub fn get_id(&self) -> String {
        get_inner_and_call!(self, BluetoothGATTService, get_id)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_id(&self, id: String) {
        match self {
            &BluetoothGATTService::Mock(ref fake_service) => fake_service.set_id(id),
            _ => (),
        }
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        get_inner_and_call!(self, BluetoothGATTService, get_uuid)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_uuid(&self, uuid: String) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothGATTService, set_uuid, uuid)
    }

    pub fn is_primary(&self) -> Result<bool, Box<Error>> {
        get_inner_and_call!(self, BluetoothGATTService, is_primary)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_primary(&self, primary: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothGATTService, set_is_primary, primary)
    }

    pub fn get_includes(&self, device: BluetoothDevice) -> Result<Vec<BluetoothGATTService>, Box<Error>> {
        let services = try!(get_inner_and_call!(self, BluetoothGATTService, get_includes));
        Ok(services.into_iter().map(|service| BluetoothGATTService::create_service(device.clone(), service)).collect())
    }

    pub fn get_gatt_characteristics(&self) -> Result<Vec<BluetoothGATTCharacteristic>, Box<Error>> {
        let characteristics = try!(get_inner_and_call!(self, BluetoothGATTService, get_gatt_characteristics));
        Ok(characteristics.into_iter()
                          .map(|characteristic|
                              BluetoothGATTCharacteristic::create_characteristic(self.clone(), characteristic))
                          .collect())
    }
}

impl BluetoothGATTCharacteristic {
    fn create_characteristic(service: BluetoothGATTService, characteristic: String) -> BluetoothGATTCharacteristic {
        match service {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            BluetoothGATTService::Bluez(_bluez_service) => {
                BluetoothGATTCharacteristic::Bluez(Arc::new(BluetoothGATTCharacteristicBluez::new(characteristic)))
            },
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            BluetoothGATTService::Android(android_service) => {
                BluetoothGATTCharacteristic::Android(
                    Arc::new(BluetoothGATTCharacteristicAndroid::new(android_service, characteristic)))
            },
            #[cfg(all(target_os = "macos", feature = "bluetooth"))]
            BluetoothGATTService::Mac(mac_service) => {
                BluetoothGATTCharacteristic::Mac(Arc::new(BluetoothGATTCharacteristicMac::new(mac_service, characteristic)))
            },
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                          all(target_os = "android", feature = "bluetooth"),
                          all(target_os = "macos", feature = "bluetooth"))))]
            BluetoothGATTService::Empty(_service) => {
                BluetoothGATTCharacteristic::Empty(Arc::new(BluetoothGATTCharacteristicEmpty::new(characteristic)))
            },
            #[cfg(feature = "bluetooth-test")]
            BluetoothGATTService::Mock(fake_service) => {
                BluetoothGATTCharacteristic::Mock(
                    FakeBluetoothGATTCharacteristic::new_empty(fake_service, characteristic))
            },
        }
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn create_mock_characteristic(service: BluetoothGATTService,
                                      characteristic: String)
                                      -> Result<BluetoothGATTCharacteristic, Box<Error>> {
        match service {
            BluetoothGATTService::Mock(fake_service) => {
                Ok(BluetoothGATTCharacteristic::Mock(
                    FakeBluetoothGATTCharacteristic::new_empty(fake_service, characteristic)))
            },
            _ => {
                Err(Box::from(NOT_SUPPORTED_ON_MOCK_ERROR))
            },
        }
    }

    pub fn get_id(&self) -> String {
        get_inner_and_call!(self, BluetoothGATTCharacteristic, get_id)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_id(&self, id: String) {
        match self {
            &BluetoothGATTCharacteristic::Mock(ref fake_characteristic) => fake_characteristic.set_id(id),
            _ => (),
        }
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        get_inner_and_call!(self, BluetoothGATTCharacteristic, get_uuid)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_uuid(&self, uuid: String) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothGATTCharacteristic, set_uuid, uuid)
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        get_inner_and_call!(self, BluetoothGATTCharacteristic, get_value)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_value(&self, value: Vec<u8>) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothGATTCharacteristic, set_value, Some(value))
    }

    pub fn is_notifying(&self) -> Result<bool, Box<Error>> {
        get_inner_and_call!(self, BluetoothGATTCharacteristic, is_notifying)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_notifying(&self, notifying: bool) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothGATTCharacteristic, set_notifying, notifying)
    }

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        get_inner_and_call!(self, BluetoothGATTCharacteristic, get_flags)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_flags(&self, flags: Vec<String>) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothGATTCharacteristic, set_flags, flags)
    }

    pub fn get_gatt_descriptors(&self) -> Result<Vec<BluetoothGATTDescriptor>, Box<Error>> {
        let descriptors = try!(get_inner_and_call!(self, BluetoothGATTCharacteristic, get_gatt_descriptors));
        Ok(descriptors.into_iter()
                      .map(|descriptor| BluetoothGATTDescriptor::create_descriptor(self.clone(), descriptor))
                      .collect())
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        get_inner_and_call!(@with_bluez_offset, self, BluetoothGATTCharacteristic, read_value)
    }

    pub fn write_value(&self, values: Vec<u8>) -> Result<(), Box<Error>> {
        get_inner_and_call!(@with_bluez_offset, self, BluetoothGATTCharacteristic, write_value, values)
    }

    pub fn start_notify(&self) -> Result<(), Box<Error>> {
        get_inner_and_call!(self, BluetoothGATTCharacteristic, start_notify)
    }

    pub fn stop_notify(&self) -> Result<(), Box<Error>> {
        get_inner_and_call!(self, BluetoothGATTCharacteristic, stop_notify)
    }
}

impl BluetoothGATTDescriptor {
    fn create_descriptor(characteristic: BluetoothGATTCharacteristic, descriptor: String) -> BluetoothGATTDescriptor {
        match characteristic {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            BluetoothGATTCharacteristic::Bluez(_bluez_characteristic) => {
                BluetoothGATTDescriptor::Bluez(Arc::new(BluetoothGATTDescriptorBluez::new(descriptor)))
            },
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            BluetoothGATTCharacteristic::Android(android_characteristic) => {
                BluetoothGATTDescriptor::Android(
                    Arc::new(BluetoothGATTDescriptorAndroid::new(android_characteristic, descriptor)))
            },
            #[cfg(all(target_os = "macos", feature = "bluetooth"))]
            BluetoothGATTCharacteristic::Mac(_mac_characteristic) => {
                BluetoothGATTDescriptor::Mac(Arc::new(BluetoothGATTDescriptorMac::new(descriptor)))
            },
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                          all(target_os = "android", feature = "bluetooth"),
                          all(target_os = "macos", feature = "bluetooth"))))]
            BluetoothGATTCharacteristic::Empty(_characteristic) => {
                BluetoothGATTDescriptor::Empty(Arc::new(BluetoothGATTDescriptorEmpty::new(descriptor)))
            },
            #[cfg(feature = "bluetooth-test")]
            BluetoothGATTCharacteristic::Mock(fake_characteristic) => {
                BluetoothGATTDescriptor::Mock(FakeBluetoothGATTDescriptor::new_empty(fake_characteristic, descriptor))
            },
        }
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn create_mock_descriptor(characteristic: BluetoothGATTCharacteristic,
                                  descriptor: String)
                                  -> Result<BluetoothGATTDescriptor, Box<Error>> {
        match characteristic {
            BluetoothGATTCharacteristic::Mock(fake_characteristic) => {
                Ok(BluetoothGATTDescriptor::Mock(
                    FakeBluetoothGATTDescriptor::new_empty(fake_characteristic, descriptor)))
            },
            _ => {
                Err(Box::from(NOT_SUPPORTED_ON_MOCK_ERROR))
            },
        }
    }

    pub fn get_id(&self) -> String {
        get_inner_and_call!(self, BluetoothGATTDescriptor, get_id)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_id(&self, id: String) {
        match self {
            &BluetoothGATTDescriptor::Mock(ref fake_descriptor) => fake_descriptor.set_id(id),
            _ => (),
        }
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        get_inner_and_call!(self, BluetoothGATTDescriptor, get_uuid)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_uuid(&self, uuid: String) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothGATTDescriptor, set_uuid, uuid)
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        get_inner_and_call!(self, BluetoothGATTDescriptor, get_value)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_value(&self, value: Vec<u8>) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothGATTDescriptor, set_value, Some(value))
    }

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        get_inner_and_call!(self, BluetoothGATTDescriptor, get_flags)
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn set_flags(&self, flags: Vec<String>) -> Result<(), Box<Error>> {
        get_inner_and_call_test_func!(self, BluetoothGATTDescriptor, set_flags, flags)
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        get_inner_and_call!(@with_bluez_offset, self, BluetoothGATTDescriptor, read_value)
    }

    pub fn write_value(&self, values: Vec<u8>) -> Result<(), Box<Error>> {
        get_inner_and_call!(@with_bluez_offset, self, BluetoothGATTDescriptor, write_value, values)
    }
}
