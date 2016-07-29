/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::error::Error;

const NOT_SUPPORTED_ERROR: &'static str = "Error! Not supported platform!";

#[derive(Clone, Debug)]
pub struct BluetoothAdapter { }

impl BluetoothAdapter {
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        Ok(BluetoothAdapter::new())
    }

    fn new() -> BluetoothAdapter {
        BluetoothAdapter{ }
    }

    pub fn get_object_path(&self) -> String {
        String::new()
    }

    pub fn get_device_list(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn set_alias(&self, _value: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn is_powered(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn set_powered(&self, _value: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn set_discoverable(&self, _value: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn is_pairable(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn set_pairable(&self, _value: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn set_pairable_timeout(&self, _value: u32) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn set_discoverable_timeout(&self, _value: u32) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn is_discovering(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}

#[derive(Clone, Debug)]
pub struct BluetoothDiscoverySession { }

impl BluetoothDiscoverySession {
    pub fn create_session(_adapter: &BluetoothAdapter) -> Result<BluetoothDiscoverySession, Box<Error>> {
        Ok(BluetoothDiscoverySession{ })
    }

    pub fn start_discovery(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn stop_discovery(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}

#[derive(Clone, Debug)]
pub struct BluetoothDevice { }

impl BluetoothDevice {
    pub fn new(_device: String) -> BluetoothDevice {
        BluetoothDevice { }
    }

    pub fn get_object_path(&self) -> String {
        String::new()
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_icon(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_appearance(&self) -> Result<u16, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn is_paired(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn is_connected(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn is_trusted(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn is_blocked(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn set_alias(&self, _value: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_adapter(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn is_legacy_pairing(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_rssi(&self) -> Result<i16, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_tx_power(&self) -> Result<i16, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_gatt_services(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn connect(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn disconnect(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn connect_profile(&self, _uuid: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn disconnect_profile(&self, _uuid: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn pair(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn cancel_pairing(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}

#[derive(Clone, Debug)]
pub struct BluetoothGATTService { }

impl BluetoothGATTService {
    pub fn new(_service: String) -> BluetoothGATTService {
        BluetoothGATTService { }
    }

    pub fn get_object_path(&self) -> String {
        String::new()
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn is_primary(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_device(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_includes(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_gatt_characteristics(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}

#[derive(Clone, Debug)]
pub struct BluetoothGATTCharacteristic { }

impl BluetoothGATTCharacteristic {
    pub fn new(_characteristic: String) -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic { }
    }

    pub fn get_object_path(&self) -> String {
        String::new()
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_service(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn is_notifying(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_gatt_descriptors(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn write_value(&self, _values: Vec<u8>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn start_notify(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn stop_notify(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}

#[derive(Clone, Debug)]
pub struct BluetoothGATTDescriptor { }

impl BluetoothGATTDescriptor {
    pub fn new(_descriptor: String) -> BluetoothGATTDescriptor {
        BluetoothGATTDescriptor { }
    }

    pub fn get_object_path(&self) -> String {
        String::new()
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_characteristic(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn write_value(&self, _values: Vec<u8>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}
