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

#[cfg(feature = "bluetooth-test")]
const NOT_SUPPORTED_ON_REAL_ERROR: &'static str = "Error! Test functions are not supported on real devices!";
#[cfg(feature = "bluetooth-test")]
const NOT_SUPPORTED_ON_MOCK_ERROR: &'static str = "Error! The first parameter must be a mock structure!";

use std::collections::HashMap;
use std::sync::Arc;
use std::error::Error;
use bluetooth::BluetoothDiscoverySession;
use bluetooth::BluetoothDevice;

pub trait BluetoothAdapter {
    fn new()-> Result<Box<BluetoothAdapter>, Box<Error>>;
    fn get_id(&self)-> String;
    fn get_devices(&self)-> Result<Vec<BluetoothDevice>, Box<Error>>;
    fn get_device(&self, address: String) -> Result<Option<BluetoothDevice>, Box<Error>>;
    fn get_address(&self) -> Result<String, Box<Error>>;
    fn get_name(&self)-> Result<String, Box<Error>>;
    fn get_alias(&self) -> Result<String, Box<Error>>;
    fn get_class(&self)-> Result<u32, Box<Error>>;
    fn is_powered(&self)-> Result<bool, Box<Error>>;
    fn is_discoverable(&self) -> Result<bool, Box<Error>>;
    fn is_pairable(&self)-> Result<bool, Box<Error>>;
    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>>;
    fn get_discoverable_timeout(&self)-> Result<u32, Box<Error>>;
    fn is_discovering(&self)-> Result<bool, Box<Error>>;
    fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> ;
    fn get_uuids(&self)-> Result<Vec<String>, Box<Error>>;
    fn get_vendor_id_source(&self)-> Result<String, Box<Error>>;
    fn get_vendor_id(&self)-> Result<u32, Box<Error>>;
    fn get_product_id(&self) -> Result<u32, Box<Error>> ;
    fn get_device_id(&self) -> Result<u32, Box<Error>>;
    fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>>;  
}
pub struct Bluez(Arc<BluetoothAdapterBluez>);

impl BluetoothAdapter for Bluez{

    fn new() -> Result<Box<BluetoothAdapter>, Box<Error>> {
        let bluez_adapter = try!(BluetoothAdapter::new());
        Ok(BluetoothAdapter::Bluez(Arc::new(bluez_adapter)))
    }

    fn get_id(&self) -> String{
        self.0.get_id()
    }

    fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>>{
        let device_list = try!(self.0.get_device_list());
        Ok(device_list.into_iter().map(|device| BluetoothDevice::create_device(self.clone(), device)).collect())
    }

    fn get_device(&self, address: String) -> Result<Option<BluetoothDevice>, Box<Error>> {
        let devices = try!(self.get_devices());
        for device in devices {
            if try!(device.get_address()) == address {
                return Ok(Some(device));
            }
        }
        Ok(None)
    }

    fn get_address(&self) -> Result<String, Box<Error>> {
        self.0.get_address()
    }

    fn get_name(&self) -> Result<String, Box<Error>> {
        self.0.get_name()
    }


    fn get_alias(&self) -> Result<String, Box<Error>> {
        self.0.get_alias()
    }


    fn get_class(&self) -> Result<u32, Box<Error>> {
        self.0.get_class()
    }

    fn is_powered(&self) -> Result<bool, Box<Error>> {
        self.0.is_powered()
    }


    fn is_discoverable(&self) -> Result<bool, Box<Error>> {
       self.0.is_discoverable()
    }


    fn is_pairable(&self) -> Result<bool, Box<Error>> {
       self.0.is_pairable()
    }


    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_pairable_timeout()
    }

    fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_discoverable_timeout()
    }

    fn is_discovering(&self) -> Result<bool, Box<Error>> {
       self.0.is_discovering()
    }


    fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        BluetoothDiscoverySession::create_session(self.clone())
    }

    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        self.0.get_uuids()
    }


    fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        self.0.get_vendor_id_source()
    }

    fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        self.0.get_vendor_id()
    }

    fn get_product_id(&self) -> Result<u32, Box<Error>> {
        self.0.get_product_id()
    }

    fn get_device_id(&self) -> Result<u32, Box<Error>> {
        self.0.get_device_id()
    }

    fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        self.0.get_modalias()
    }
}   

pub struct Android(Arc<BluetoothAdapterAndroid>);

impl BluetoothAdapter for Android{

    fn new() -> Result<Box<BluetoothAdapter>, Box<Error>> {
        let blurdroid_adapter = try!(BluetoothAdapter::get_adapter());
        Ok(BluetoothAdapter::Android(Arc::new(blurdroid_adapter)))
    }

    fn get_id(&self) -> String{
        self.0.get_id()
    }

    fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>>{
        let device_list = try!(self.0.get_device_list());
        Ok(device_list.into_iter().map(|device| BluetoothDevice::create_device(self.clone(), device)).collect())
    }

    fn get_device(&self, address: String) -> Result<Option<BluetoothDevice>, Box<Error>> {
        let devices = try!(self.get_devices());
        for device in devices {
            if try!(device.get_address()) == address {
                return Ok(Some(device));
            }
        }
        Ok(None)
    }

    fn get_address(&self) -> Result<String, Box<Error>> {
        self.0.get_address()
    }

    fn get_name(&self) -> Result<String, Box<Error>> {
        self.0.get_name()
    }


    fn get_alias(&self) -> Result<String, Box<Error>> {
        self.0.get_alias()
    }


    fn get_class(&self) -> Result<u32, Box<Error>> {
        self.0.get_class()
    }

    fn is_powered(&self) -> Result<bool, Box<Error>> {
        self.0.is_powered()
    }


    fn is_discoverable(&self) -> Result<bool, Box<Error>> {
       self.0.is_discoverable()
    }


    fn is_pairable(&self) -> Result<bool, Box<Error>> {
       self.0.is_pairable()
    }


    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_pairable_timeout()
    }

    fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_discoverable_timeout()
    }

    fn is_discovering(&self) -> Result<bool, Box<Error>> {
       self.0.is_discovering()
    }


    fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        BluetoothDiscoverySession::create_session(self.clone())
    }

    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        self.0.get_uuids()
    }


    fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        self.0.get_vendor_id_source()
    }

    fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        self.0.get_vendor_id()
    }

    fn get_product_id(&self) -> Result<u32, Box<Error>> {
        self.0.get_product_id()
    }

    fn get_device_id(&self) -> Result<u32, Box<Error>> {
        self.0.get_device_id()
    }

    fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        self.0.get_modalias()
    }
} 

pub struct Mac(Arc<BluetoothAdapterMac>);

impl BluetoothAdapter for Mac{

    fn new() -> Result<Box<BluetoothAdapter>, Box<Error>> {
        let mac_adapter = try!(BluetoothAdapter::new());
        Ok(BluetoothAdapter::Mac(Arc::new(mac_adapter)))
    }

    fn get_id(&self) -> String{
        self.0.get_id()
    }

    fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>>{
        let device_list = try!(self.0.get_device_list());
        Ok(device_list.into_iter().map(|device| BluetoothDevice::create_device(self.clone(), device)).collect())
    }
    fn get_device(&self, address: String) -> Result<Option<BluetoothDevice>, Box<Error>> {
        let devices = try!(self.get_devices());
        for device in devices {
            if try!(device.get_address()) == address {
                return Ok(Some(device));
            }
        }
        Ok(None)
    }
    fn get_address(&self) -> Result<String, Box<Error>> {
        self.0.get_address()
    }

    fn get_name(&self) -> Result<String, Box<Error>> {
        self.0.get_name()
    }


    fn get_alias(&self) -> Result<String, Box<Error>> {
        self.0.get_alias()
    }


    fn get_class(&self) -> Result<u32, Box<Error>> {
        self.0.get_class()
    }

    fn is_powered(&self) -> Result<bool, Box<Error>> {
        self.0.is_powered()
    }


    fn is_discoverable(&self) -> Result<bool, Box<Error>> {
       self.0.is_discoverable()
    }


    fn is_pairable(&self) -> Result<bool, Box<Error>> {
       self.0.is_pairable()
    }


    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_pairable_timeout()
    }

    fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_discoverable_timeout()
    }

    fn is_discovering(&self) -> Result<bool, Box<Error>> {
       self.0.is_discovering()
    }


    fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        BluetoothDiscoverySession::create_session(self.clone())
    }

    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        self.0.get_uuids()
    }


    fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        self.0.get_vendor_id_source()
    }

    fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        self.0.get_vendor_id()
    }

    fn get_product_id(&self) -> Result<u32, Box<Error>> {
        self.0.get_product_id()
    }

    fn get_device_id(&self) -> Result<u32, Box<Error>> {
        self.0.get_device_id()
    }

    fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        self.0.get_modalias()
    }
} 

pub struct Empty(Arc<BluetoothAdapterEmpty>);

impl BluetoothAdapter for Empty{
    fn new(&self) -> Result<Box<BluetoothAdapter>, Box<Error>> {
        let adapter = try!(BluetoothAdapter::init());
        Ok(BluetoothAdapter::Empty(Arc::new(adapter)))
    }

    fn get_id(&self) -> String{
        self.0.get_id()
    }

    fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>>{
        let device_list = try!(self.0.get_device_list());
        Ok(device_list.into_iter().map(|device| BluetoothDevice::create_device(self.clone(), device)).collect())
    }

    fn get_device(&self, address: String) -> Result<Option<BluetoothDevice>, Box<Error>> {
        let devices = try!(self.get_devices());
        for device in devices {
            if try!(device.get_address()) == address {
                return Ok(Some(device));
            }
        }
        Ok(None)
    }

    fn get_address(&self) -> Result<String, Box<Error>> {
        self.0.get_address()
    }

    fn get_name(&self) -> Result<String, Box<Error>> {
        self.0.get_name()
    }


    fn get_alias(&self) -> Result<String, Box<Error>> {
        self.0.get_alias()
    }


    fn get_class(&self) -> Result<u32, Box<Error>> {
        self.0.get_class()
    }

    fn is_powered(&self) -> Result<bool, Box<Error>> {
        self.0.is_powered()
    }


    fn is_discoverable(&self) -> Result<bool, Box<Error>> {
       self.0.is_discoverable()
    }


    fn is_pairable(&self) -> Result<bool, Box<Error>> {
       self.0.is_pairable()
    }


    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_pairable_timeout()
    }

    fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_discoverable_timeout()
    }

    fn is_discovering(&self) -> Result<bool, Box<Error>> {
       self.0.is_discovering()
    }


    fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        BluetoothDiscoverySession::create_session(self.clone())
    }

    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        self.0.get_uuids()
    }


    fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        self.0.get_vendor_id_source()
    }

    fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        self.0.get_vendor_id()
    }

    fn get_product_id(&self) -> Result<u32, Box<Error>> {
        self.0.get_product_id()
    }

    fn get_device_id(&self) -> Result<u32, Box<Error>> {
        self.0.get_device_id()
    }

    fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        self.0.get_modalias()
    }    
}


