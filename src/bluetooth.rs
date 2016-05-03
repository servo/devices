/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(target_os = "linux")]
use blurz::bluetooth_adapter::BluetoothAdapter as BluetoothAdapterBluez;
#[cfg(target_os = "linux")]
use blurz::bluetooth_device::BluetoothDevice as BluetoothDeviceBluez;
#[cfg(target_os = "linux")]
use blurz::bluetooth_gatt_characteristic::BluetoothGATTCharacteristic as BluetoothGATTCharacteristicBluez;
#[cfg(target_os = "linux")]
use blurz::bluetooth_gatt_descriptor::BluetoothGATTDescriptor as BluetoothGATTDescriptorBluez;
#[cfg(target_os = "linux")]
use blurz::bluetooth_gatt_service::BluetoothGATTService as BluetoothGATTServiceBluez;

use std::error::Error;
use std::result::Result::Ok;

#[cfg(not(target_os = "linux"))]
const NOT_SUPPORTED_ERROR: &'static str = "Error! Not supported platform!";

#[cfg(not(target_os = "linux"))]
struct BluetoothAdapterBluez {}

#[cfg(not(target_os = "linux"))]
struct BluetoothDeviceBluez {}

#[cfg(not(target_os = "linux"))]
struct BluetoothGATTCharacteristicBluez {}

#[cfg(not(target_os = "linux"))]
struct BluetoothGATTDescriptorBluez {}

#[cfg(not(target_os = "linux"))]
struct BluetoothGATTServiceBluez {}

#[derive(Clone, Debug)]
pub struct BluetoothAdapter {
    #[cfg(target_os = "linux")]
    adapter: BluetoothAdapterBluez,
}

#[derive(Clone, Debug)]
pub struct BluetoothDevice {
    #[cfg(target_os = "linux")]
    device: BluetoothDeviceBluez,
}

#[derive(Clone, Debug)]
pub struct BluetoothGATTService {
    #[cfg(target_os = "linux")]
    gatt_service: BluetoothGATTServiceBluez,
}

#[derive(Clone, Debug)]
pub struct BluetoothGATTCharacteristic {
    #[cfg(target_os = "linux")]
    gatt_characteristic: BluetoothGATTCharacteristicBluez,
}

#[derive(Clone, Debug)]
pub struct BluetoothGATTDescriptor {
    #[cfg(target_os = "linux")]
    gatt_descriptor: BluetoothGATTDescriptorBluez,
}

impl BluetoothAdapter {
    #[cfg(target_os = "linux")]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        let bluez_adapter = try!(BluetoothAdapterBluez::init());
        Ok(BluetoothAdapter::new(bluez_adapter))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(target_os = "linux")]
    pub fn create_adapter(object_path: String) -> Result<BluetoothAdapter, Box<Error>> {
        let bluez_adapter = try!(BluetoothAdapterBluez::create_adapter(object_path));
        Ok(BluetoothAdapter::new(bluez_adapter))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn create_adapter(_object_path: String) -> Result<BluetoothAdapter, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(target_os = "linux")]
    fn new(adapter: BluetoothAdapterBluez) -> BluetoothAdapter {
        BluetoothAdapter {
            adapter: adapter,
        }
    }

    pub fn get_object_path(&self) -> Result<String, Box<Error>> {
        let adapter = try!(self.get_adapter());
        Ok(adapter.get_object_path())
    }

    #[cfg(target_os = "linux")]
    fn get_adapter(&self) -> Result<BluetoothAdapterBluez, Box<Error>> {
        Ok(self.adapter.clone())
    }

    #[cfg(not(target_os = "linux"))]
    fn get_adapter(&self) -> Result<BluetoothAdapterBluez, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>> {
        let adapter = try!(self.get_adapter());
        let device_list = try!(adapter.get_device_list());
        Ok(device_list.into_iter().map(|device| BluetoothDevice::create_device(device)).collect())
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
        let adapter = try!(self.get_adapter());
         adapter.get_address()
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.get_name()
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.get_alias()
    }

    pub fn set_alias(&self, value: String) -> Result<(), Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.set_alias(value)
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.get_class()
    }

    pub fn is_powered(&self) -> Result<bool, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.is_powered()
    }

    pub fn set_powered(&self, value: bool) -> Result<(), Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.set_powered(value)
    }

    pub fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.is_discoverable()
    }

    pub fn set_discoverable(&self, value: bool) -> Result<(), Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.set_discoverable(value)
    }

    pub fn is_pairable(&self) -> Result<bool, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.is_pairable()
    }

    pub fn set_pairable(&self, value: bool) -> Result<(), Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.set_pairable(value)
    }

    pub fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.get_pairable_timeout()
    }

    pub fn set_pairable_timeout(&self, value: u32) -> Result<(), Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.set_pairable_timeout(value)
    }

    pub fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.get_discoverable_timeout()
    }

    pub fn set_discoverable_timeout(&self, value: u32) -> Result<(), Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.set_discoverable_timeout(value)
    }

    pub fn is_discovering(&self) -> Result<bool, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.is_discovering()
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.get_uuids()
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.get_vendor_id_source()
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.get_vendor_id()
    }

    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.get_product_id()
    }

    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.get_device_id()
    }

    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.get_modalias()
    }

    pub fn start_discovery(&self) -> Result<(), Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.start_discovery()
    }

    pub fn stop_discovery(&self) -> Result<(), Box<Error>> {
        let adapter = try!(self.get_adapter());
        adapter.stop_discovery()
    }
}

impl BluetoothDevice {
    #[cfg(target_os = "linux")]
    fn new(device: BluetoothDeviceBluez) -> BluetoothDevice {
        BluetoothDevice {
            device: device,
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn create_device(_device: String) -> BluetoothDevice {
        BluetoothDevice { }
    }

    #[cfg(target_os = "linux")]
    pub fn create_device(device: String) -> BluetoothDevice {
        BluetoothDevice::new(
            BluetoothDeviceBluez::new(device.clone()))
    }

    pub fn get_object_path(&self) -> Result<String, Box<Error>> {
        let device = try!(self.get_device());
        Ok(device.get_object_path())
    }

    #[cfg(target_os = "linux")]
    fn get_device(&self) -> Result<BluetoothDeviceBluez, Box<Error>> {
        Ok(self.device.clone())
    }

    #[cfg(not(target_os = "linux"))]
    fn get_device(&self) -> Result<BluetoothDeviceBluez, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        let device = try!(self.get_device());
        device.get_address()
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        let device = try!(self.get_device());
        device.get_name()
    }

    pub fn get_icon(&self) -> Result<String, Box<Error>> {
        let device = try!(self.get_device());
        device.get_icon()
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        let device = try!(self.get_device());
        device.get_class()
    }

    pub fn get_appearance(&self) -> Result<u16, Box<Error>> {
        let device = try!(self.get_device());
        device.get_appearance()
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        let device = try!(self.get_device());
        device.get_uuids()
    }

    pub fn is_paired(&self) -> Result<bool, Box<Error>> {
        let device = try!(self.get_device());
        device.is_paired()
    }

    pub fn is_connected(&self) -> Result<bool, Box<Error>> {
        let device = try!(self.get_device());
        device.is_connected()
    }

    pub fn is_trusted(&self) -> Result<bool, Box<Error>> {
        let device = try!(self.get_device());
        device.is_trusted()
    }

    pub fn is_blocked(&self) -> Result<bool, Box<Error>> {
        let device = try!(self.get_device());
        device.is_blocked()
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        let device = try!(self.get_device());
        device.get_alias()
    }

    pub fn set_alias(&self, value: String) -> Result<(), Box<Error>> {
        let device = try!(self.get_device());
        device.set_alias(value)
    }

    pub fn get_adapter(&self) -> Result<String, Box<Error>> {
        let device = try!(self.get_device());
        device.get_adapter()
    }

    pub fn is_legacy_pairing(&self) -> Result<bool, Box<Error>> {
        let device = try!(self.get_device());
        device.is_legacy_pairing()
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        let device = try!(self.get_device());
        device.get_vendor_id_source()
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        let device = try!(self.get_device());
        device.get_vendor_id()
    }

    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        let device = try!(self.get_device());
        device.get_product_id()
    }

    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        let device = try!(self.get_device());
        device.get_device_id()
    }

    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        let device = try!(self.get_device());
        device.get_modalias()
    }

    pub fn get_rssi(&self) -> Result<i16, Box<Error>> {
        let device = try!(self.get_device());
        device.get_rssi()
    }

    pub fn get_tx_power(&self) -> Result<i16, Box<Error>> {
        let device = try!(self.get_device());
        device.get_tx_power()
    }

    pub fn get_gatt_services(&self) -> Result<Vec<BluetoothGATTService>, Box<Error>> {
        let device = try!(self.get_device());
        let services = try!(device.get_gatt_services());
        Ok(services.into_iter().map(|service| BluetoothGATTService::create_service(service)).collect())
    }

    pub fn connect(&self) -> Result<(), Box<Error>> {
        let device = try!(self.get_device());
        device.connect()
    }

    pub fn disconnect(&self) -> Result<(), Box<Error>> {
        let device = try!(self.get_device());
        device.disconnect()
    }

    pub fn connect_profile(&self, uuid: String) -> Result<(), Box<Error>> {
        let device = try!(self.get_device());
        device.connect_profile(uuid)
    }

    pub fn disconnect_profile(&self, uuid: String) -> Result<(), Box<Error>> {
        let device = try!(self.get_device());
        device.disconnect_profile(uuid)
    }

    pub fn pair(&self) -> Result<(), Box<Error>> {
        let device = try!(self.get_device());
        device.pair()
    }

    pub fn cancel_pairing(&self) -> Result<(), Box<Error>> {
        let device = try!(self.get_device());
        device.cancel_pairing()
    }
}

impl BluetoothGATTService {
    #[cfg(target_os = "linux")]
    fn new(gatt_service: BluetoothGATTServiceBluez)
           -> BluetoothGATTService {
        BluetoothGATTService {
            gatt_service: gatt_service
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn create_service(_service: String) -> BluetoothGATTService {
        BluetoothGATTService { }
    }

    #[cfg(target_os = "linux")]
    pub fn create_service(service: String) -> BluetoothGATTService {
        BluetoothGATTService::new(
            BluetoothGATTServiceBluez::new(service.clone()))
    }

    pub fn get_object_path(&self) -> Result<String, Box<Error>> {
        let gatt_service = try!(self.get_gatt_service());
        Ok(gatt_service.get_object_path())
    }

    #[cfg(not(target_os = "linux"))]
    fn get_gatt_service(&self) -> Result<BluetoothGATTServiceBluez, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(target_os = "linux")]
    fn get_gatt_service(&self) -> Result<BluetoothGATTServiceBluez, Box<Error>> {
        Ok(self.gatt_service.clone())
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        let gatt_service = try!(self.get_gatt_service());
        gatt_service.get_uuid()
    }

    pub fn is_primary(&self) -> Result<bool, Box<Error>> {
        let gatt_service = try!(self.get_gatt_service());
        gatt_service.is_primary()
    }

    pub fn get_device(&self) -> Result<String, Box<Error>> {
        let gatt_service = try!(self.get_gatt_service());
        gatt_service.get_device()
    }

    pub fn get_gatt_characteristics(&self) -> Result<Vec<BluetoothGATTCharacteristic>, Box<Error>> {
        let gatt_service = try!(self.get_gatt_service());
        let characteristics = try!(gatt_service.get_gatt_characteristics());
        Ok(characteristics.into_iter().map(|characteristic| BluetoothGATTCharacteristic::create_characteristic(characteristic)).collect())
    }
}

impl BluetoothGATTCharacteristic {
    #[cfg(target_os = "linux")]
    fn new(gatt_characteristic: BluetoothGATTCharacteristicBluez)
           -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic {
            gatt_characteristic: gatt_characteristic
        }
    }

    #[cfg(target_os = "linux")]
    pub fn create_characteristic(characteristic: String) -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic::new(
            BluetoothGATTCharacteristicBluez::new(characteristic.clone()))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn create_characteristic(_characteristic: String) -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic { }
    }

    pub fn get_object_path(&self) -> Result<String, Box<Error>> {
        let gatt_characteristic = try!(self.get_gatt_characteristic());
        Ok(gatt_characteristic.get_object_path())
    }

    #[cfg(target_os = "linux")]
    fn get_gatt_characteristic(&self) -> Result<BluetoothGATTCharacteristicBluez, Box<Error>> {
        Ok(self.gatt_characteristic.clone())
    }

    #[cfg(not(target_os = "linux"))]
    fn get_gatt_characteristic(&self) -> Result<BluetoothGATTCharacteristicBluez, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        let gatt_characteristic = try!(self.get_gatt_characteristic());
        gatt_characteristic.get_uuid()
    }

    pub fn get_service(&self) -> Result<String, Box<Error>> {
        let gatt_characteristic = try!(self.get_gatt_characteristic());
        gatt_characteristic.get_service()
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        let gatt_characteristic = try!(self.get_gatt_characteristic());
        gatt_characteristic.get_value()
    }

    pub fn is_notifying(&self) -> Result<bool, Box<Error>> {
        let gatt_characteristic = try!(self.get_gatt_characteristic());
        gatt_characteristic.is_notifying()
    }

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        let gatt_characteristic = try!(self.get_gatt_characteristic());
        gatt_characteristic.get_flags()
    }

    pub fn get_gatt_descriptors(&self) -> Result<Vec<BluetoothGATTDescriptor>, Box<Error>> {
        let gatt_characteristic = try!(self.get_gatt_characteristic());
        let descriptors =  try!(gatt_characteristic.get_gatt_descriptors());
        Ok(descriptors.into_iter().map(|descriptor| BluetoothGATTDescriptor::create_descriptor(descriptor)).collect())
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        let gatt_characteristic = try!(self.get_gatt_characteristic());
        gatt_characteristic.read_value()
    }

    pub fn write_value(&self, values: Vec<u8>) -> Result<(), Box<Error>> {
        let gatt_characteristic = try!(self.get_gatt_characteristic());
        gatt_characteristic.write_value(values)
    }

    pub fn start_notify(&self) -> Result<(), Box<Error>> {
        let gatt_characteristic = try!(self.get_gatt_characteristic());
        gatt_characteristic.start_notify()
    }

    pub fn stop_notify(&self) -> Result<(), Box<Error>> {
        let gatt_characteristic = try!(self.get_gatt_characteristic());
        gatt_characteristic.stop_notify()
    }
}

impl BluetoothGATTDescriptor {
    #[cfg(target_os = "linux")]
    fn new(gatt_descriptor: BluetoothGATTDescriptorBluez)
           -> BluetoothGATTDescriptor {
        BluetoothGATTDescriptor {
            gatt_descriptor: gatt_descriptor
        }
    }

    #[cfg(target_os = "linux")]
    pub fn create_descriptor(descriptor: String) -> BluetoothGATTDescriptor {
        BluetoothGATTDescriptor::new(
            BluetoothGATTDescriptorBluez::new(descriptor.clone()))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn create_descriptor(_descriptor: String) -> BluetoothGATTDescriptor {
        BluetoothGATTDescriptor { }
    }

    pub fn get_object_path(&self) -> Result<String, Box<Error>> {
        let gatt_descriptor = try!(self.get_gatt_descriptor());
        Ok(gatt_descriptor.get_object_path())
    }

    #[cfg(not(target_os = "linux"))]
    fn get_gatt_descriptor(&self) -> Result<BluetoothGATTDescriptorBluez, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(target_os = "linux")]
    fn get_gatt_descriptor(&self) -> Result<BluetoothGATTDescriptorBluez, Box<Error>> {
        Ok(self.gatt_descriptor.clone())
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        let gatt_descriptor = try!(self.get_gatt_descriptor());
        gatt_descriptor.get_uuid()
    }

    pub fn get_characteristic(&self) -> Result<String, Box<Error>> {
        let gatt_descriptor = try!(self.get_gatt_descriptor());
        gatt_descriptor.get_characteristic()
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        let gatt_descriptor = try!(self.get_gatt_descriptor());
        gatt_descriptor.get_value()
    }

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        let gatt_descriptor = try!(self.get_gatt_descriptor());
        gatt_descriptor.get_flags()
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        let gatt_descriptor = try!(self.get_gatt_descriptor());
        gatt_descriptor.read_value()
    }

    pub fn write_value(&self, values: Vec<u8>) -> Result<(), Box<Error>> {
        let gatt_descriptor = try!(self.get_gatt_descriptor());
        gatt_descriptor.write_value(values)
    }
}
