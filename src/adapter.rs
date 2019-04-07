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


#[cfg(feature = "bluetooth-test")]
const NOT_SUPPORTED_ON_REAL_ERROR: &'static str = "Error! Test functions are not supported on real devices!";
#[cfg(feature = "bluetooth-test")]
const NOT_SUPPORTED_ON_MOCK_ERROR: &'static str = "Error! The first parameter must be a mock structure!";

use std::collections::HashMap;
use std::sync::Arc;
use std::error::Error;
use bluetooth::BluetoothDevice;
use bluetooth::BluetoothDiscoverySession;

pub trait BluetoothAdapter {
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
    fn create_discovery_session(&self)-> Result<BluetoothDiscoverySession, Box<Error>> ;
    fn get_uuids(&self)-> Result<Vec<String>, Box<Error>>;
    fn get_vendor_id_source(&self)-> Result<String, Box<Error>>;
    fn get_vendor_id(&self)-> Result<u32, Box<Error>>;
    fn get_product_id(&self) -> Result<u32, Box<Error>> ;
    fn get_device_id(&self) -> Result<u32, Box<Error>>;
    fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>>;  
}

impl BluetoothAdapter{
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    pub fn new() -> Result<Box<BluetoothAdapterBluez>, Box<Error>> {
        let bluez_adapter = try!(BluetoothAdapterBluez::init());
        Ok(BluetoothAdapter::Bluez(Arc::new(bluez_adapter)))
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    pub fn new() -> Result<Box<BluetoothAdapterAndroid>, Box<Error>> {
        let blurdroid_adapter = try!(BluetoothAdapterAndroid::get_adapter());
        Ok(BluetoothAdapter::Android(Arc::new(blurdroid_adapter)))
    }

    #[cfg(all(target_os = "macos", feature = "bluetooth"))]
    pub fn new() -> Result<Box<BluetoothAdapterMac>, Box<Error>> {
        let mac_adapter = try!(BluetoothAdapterMac::init());
        Ok(BluetoothAdapter::Mac(Arc::new(mac_adapter)))
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                  all(target_os = "android", feature = "bluetooth"),
                  all(target_os = "macos", feature = "bluetooth"))))]
    pub fn new() -> Result<Box<BluetoothAdapterEmpty>, Box<Error>> {
        let adapter = try!(BluetoothAdapterEmpty::init());
        Ok(Box::new(adapter))
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn new() -> Result<Box<BluetoothAdapter>, Box<Error>> {
        Ok(BluetoothAdapter::Mock(FakeBluetoothAdapter::new_empty()))
    }
}   

#[derive(Clone, Debug)]
#[cfg(all(target_os = "linux", feature = "bluetooth"))]
struct Bluez(Arc<BluetoothAdapterBluez>);

#[cfg(all(target_os = "linux", feature = "bluetooth"))]
impl BluetoothAdapter for Bluez{

    fn get_id(&self) -> String{
        self.0.get_id()
    }

    fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>>{
        let device_list = try!(self.0.get_device_list());
        Ok(device_list.into_iter().map(|device|  BluetoothDevice::Bluez(Arc::new(BluetoothDeviceBluez::new(device)))).collect())
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
        let bluez_adapter = try!(BluetoothAdapterBluez::init());
        let bluez_session = try!(BluetoothDiscoverySessionBluez::create_session(Arc::new(bluez_adapter)));
        Ok(BluetoothDiscoverySession::Bluez(Arc::new(bluez_session)))
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
#[cfg(all(target_os = "android", feature = "bluetooth"))]
struct Android(Arc<BluetoothAdapterAndroid>);

#[cfg(all(target_os = "android", feature = "bluetooth"))]
impl BluetoothAdapter for Android{

    fn get_id(&self) -> String{
        self.0.get_id()
    }

    fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>>{
        let device_list = try!(self.0.get_device_list());
        Ok(device_list.into_iter().map(|device|  BluetoothDevice::Android(Arc::new(BluetoothDeviceAndroid::new(android_adapter, device)))).collect())
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
         let android_adapter = try!(BluetoothAdapterAndroid::init());
        let blurdroid_session = try!(BluetoothDiscoverySessionAndroid::create_session(Arc::new(android_adapter)));
        Ok(BluetoothDiscoverySession::Android(Arc::new(blurdroid_session)))
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

#[cfg(all(target_os = "macos", feature = "bluetooth"))]
struct Mac(Arc<BluetoothAdapterMac>);

#[cfg(all(target_os = "macos", feature = "bluetooth"))]
impl BluetoothAdapter for Mac{

    fn get_id(&self) -> String{
        self.0.get_id()
    }

    fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>>{
        let device_list = try!(self.0.get_device_list());
        Ok(device_list.into_iter().map(|device| BluetoothDevice::Mac(Arc::new(BluetoothDeviceMac::new(mac_adapter, device)))).collect())
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


    fn create_discovery_session(&self) -> Result<BluetoothDiscoveySession, Box<Error>> {
        let mac_adapter = try!(BluetoothAdapterMac::init());
        let mac_session = try!(BluetoothDiscoverySessionMac::create_session(Arc::new(mac_adapter)));
        Ok(BluetoothDiscoverySession::Mac(Arc::new(mac_session)))
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
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
              all(target_os = "android", feature = "bluetooth"),
              all(target_os = "macos", feature = "bluetooth"))))]
struct Empty(Arc<BluetoothAdapterEmpty>);

#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
              all(target_os = "android", feature = "bluetooth"),
              all(target_os = "macos", feature = "bluetooth"))))]
impl BluetoothAdapter for Empty{
    
    fn get_id(&self) -> String{
        self.0.get_id()
    }

    fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>>{
        let device_list = try!(self.0.get_device_list());
        Ok(device_list.into_iter().map(|device| BluetoothDevice::Empty(Arc::new(BluetoothDeviceEmpty::new(device)))).collect())

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
        let adapter = try!(BluetoothAdapterEmpty::init());
        let empty_session = try!(BluetoothDiscoverySessionEmpty::create_session(Arc::new(adapter)));
        Ok(BluetoothDiscoverySession::Empty(Arc::new(empty_session)))
        
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
