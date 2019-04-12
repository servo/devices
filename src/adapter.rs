    
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
const NOT_SUPPORTED_ERROR: &'static str = "Error! Not supported platform!";

use bluetooth::BluetoothDevice;
use bluetooth::BluetoothDiscoverySession;

pub trait BluetoothAdapter {
    fn get_id(&self)-> String;
   // fn set_id(&self, id: String);
    fn get_devices(&self)-> Result<Vec<BluetoothDevice>, Box<Error>>;
    fn get_device(&self, address: String) -> Result<Option<BluetoothDevice>, Box<Error>>;
    fn get_address(&self) -> Result<String, Box<Error>>;
    fn set_address(&self, address: String) -> Result<(), Box<Error>>;
    fn get_name(&self)-> Result<String, Box<Error>>;
    fn set_name(&self, name: String) -> Result<(), Box<Error>>;
    fn get_alias(&self) -> Result<String, Box<Error>>;
    fn set_alias(&self, alias: String) -> Result<(), Box<Error>>;
    fn get_class(&self)-> Result<u32, Box<Error>>;
    fn set_class(&self, class: u32) -> Result<(), Box<Error>>;
    fn is_powered(&self)-> Result<bool, Box<Error>>;
    fn set_powered(&self, powered: bool) -> Result<(), Box<Error>>;
    fn is_present(&self) -> Result<bool, Box<Error>> ;
    fn set_present(&self, present: bool) -> Result<(), Box<Error>>;
    fn is_discoverable(&self) -> Result<bool, Box<Error>>;
    fn set_discoverable(&self, discoverable: bool) -> Result<(), Box<Error>> ;
    fn is_pairable(&self)-> Result<bool, Box<Error>>;
    fn set_pairable(&self, pairable: bool) -> Result<(), Box<Error>> ;
    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>>;
    fn set_pairable_timeout(&self, timeout: u32) -> Result<(), Box<Error>>;
    fn get_discoverable_timeout(&self)-> Result<u32, Box<Error>>;
    fn set_discoverable_timeout(&self, timeout: u32) -> Result<(), Box<Error>> ;
    fn is_discovering(&self)-> Result<bool, Box<Error>>;
    fn set_discovering(&self, discovering: bool) -> Result<(), Box<Error>> ;
    fn set_can_start_discovery(&self, can_start_discovery: bool) -> Result<(), Box<Error>>;
    fn set_can_stop_discovery(&self, can_stop_discovery: bool) -> Result<(), Box<Error>>;
    fn create_discovery_session(&self)-> Result<BluetoothDiscoverySession, Box<Error>> ;
    fn get_uuids(&self)-> Result<Vec<String>, Box<Error>>;
    fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>> ;
    fn get_vendor_id_source(&self)-> Result<String, Box<Error>>;
    fn get_vendor_id(&self)-> Result<u32, Box<Error>>;
    fn get_product_id(&self) -> Result<u32, Box<Error>> ;
    fn get_device_id(&self) -> Result<u32, Box<Error>>;
    fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>>;  
    fn set_modalias(&self, modalias: String) -> Result<(), Box<Error>>;
    fn get_ad_datas(&self) -> Result<Vec<String>, Box<Error>>;
    fn set_ad_datas(&self, ad_datas: Vec<String>) -> Result<(), Box<Error>>;
}

impl BluetoothAdapter{
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    pub fn new() -> Result<Box<BluetoothAdapterBluez>, Box<Error>> {
        let bluez_adapter = try!(BluetoothAdapterBluez::init());
        Ok(Box::new(bluez_adapter))
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    pub fn new() -> Result<Box<BluetoothAdapterAndroid>, Box<Error>> {
        let blurdroid_adapter = try!(BluetoothAdapterAndroid::get_adapter());
        Ok(Box::new(blurdroid_adapter))
    }

    #[cfg(all(target_os = "macos", feature = "bluetooth"))]
    pub fn new() -> Result<Box<BluetoothAdapterMac>, Box<Error>> {
        let mac_adapter = try!(BluetoothAdapterMac::init());
        Ok(Box::new(mac_adapter))
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                  all(target_os = "android", feature = "bluetooth"),
                  all(target_os = "macos", feature = "bluetooth"))))]
    pub fn new() -> Result<Box<BluetoothAdapterEmpty>, Box<Error>> {
        let adapter = try!(BluetoothAdapterEmpty::init());
        Ok(Box::new(adapter))
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn new_mock() -> Result<Box<FakeBluetoothAdapter>, Box<Error>> {
        Ok(Box::new_empty())
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
    fn set_address(&self, address: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    fn get_name(&self) -> Result<String, Box<Error>> {
        self.0.get_name()
    }
    fn set_name(&self, name: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    fn get_alias(&self) -> Result<String, Box<Error>> {
        self.0.get_alias()
    }

    fn set_alias(&self, alias: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }

    fn get_class(&self) -> Result<u32, Box<Error>> {
        self.0.get_class()
    }
    fn set_class(&self, class: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }


    fn is_powered(&self) -> Result<bool, Box<Error>> {
        self.0.is_powered()
    }

    fn set_powered(&self, powered: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn is_present(&self) -> Result<bool, Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_present(&self, present: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn is_discoverable(&self) -> Result<bool, Box<Error>> {
       self.0.is_discoverable()
    }
    fn set_discoverable(&self, discoverable: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }


    fn is_pairable(&self) -> Result<bool, Box<Error>> {
       self.0.is_pairable()
    }
    fn set_pairable(&self, pairable: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }


    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_pairable_timeout()
    }
    fn set_pairable_timeout(&self, timeout: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))   
    }

    fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_discoverable_timeout()
    }
    fn set_discoverable_timeout(&self, timeout: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    fn is_discovering(&self) -> Result<bool, Box<Error>> {
       self.0.is_discovering()
    }
    fn set_discovering(&self, discovering: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_can_start_discovery(&self, can_start_discovery: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_can_stop_discovery(&self, can_stop_discovery: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }


    fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        let bluez_adapter = try!(BluetoothAdapterBluez::init());
        let bluez_session = try!(BluetoothDiscoverySessionBluez::create_session(Arc::new(bluez_adapter)));
        Ok(BluetoothDiscoverySession::Bluez(Arc::new(bluez_session)))
    }

    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        self.0.get_uuids()
    }
    fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
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
    fn set_modalias(&self, modalias: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_ad_datas(&self) -> Result<Vec<String>, Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_ad_datas(&self, ad_datas: Vec<String>) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
      
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
        let android_adapter = try!(BluetoothAdapterAndroid::init());
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
    fn set_address(&self, address: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn get_name(&self) -> Result<String, Box<Error>> {
        self.0.get_name()
    }

    fn set_name(&self, name: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn get_alias(&self) -> Result<String, Box<Error>> {
        self.0.get_alias()
    }
    fn set_alias(&self, alias: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }

    fn get_class(&self) -> Result<u32, Box<Error>> {
        self.0.get_class()
    }
    fn set_class(&self, class: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }
    fn is_powered(&self) -> Result<bool, Box<Error>> {
        self.0.is_powered()
    }

    fn set_powered(&self, powered: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn is_present(&self) -> Result<bool, Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_present(&self, present: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn is_discoverable(&self) -> Result<bool, Box<Error>> {
       self.0.is_discoverable()
    }

    fn set_discoverable(&self, discoverable: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }


    fn is_pairable(&self) -> Result<bool, Box<Error>> {
       self.0.is_pairable()
    }
    fn set_pairable(&self, pairable: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }


    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_pairable_timeout()
    }
    fn set_pairable_timeout(&self, timeout: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))   
    }

    fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_discoverable_timeout()
    }
    fn set_discoverable_timeout(&self, timeout: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    fn is_discovering(&self) -> Result<bool, Box<Error>> {
       self.0.is_discovering()
    }
    fn set_discovering(&self, discovering: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_can_start_discovery(&self, can_start_discovery: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_can_stop_discovery(&self, can_stop_discovery: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }

    fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        let android_adapter = try!(BluetoothAdapterAndroid::init());
        let blurdroid_session = try!(BluetoothDiscoverySessionAndroid::create_session(Arc::new(android_adapter)));
        Ok(BluetoothDiscoverySession::Android(Arc::new(blurdroid_session)))
    }

    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        self.0.get_uuids()
    }
    fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
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
    fn set_modalias(&self, modalias: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn get_ad_datas(&self) -> Result<Vec<String>, Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_ad_datas(&self, ad_datas: Vec<String>) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
      
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
        let mac_adapter = try!(BluetoothAdapterMac::init());
        Ok(device_list.into_iter().map(|device| BluetoothDevice::Mac(Arc::new(BluetoothDeviceMac::new((Arc::new(mac_adapter)), device)))).collect())
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
    fn set_address(&self, address: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn get_name(&self) -> Result<String, Box<Error>> {
        self.0.get_name()
    }
    fn set_name(&self, name: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }

    fn get_alias(&self) -> Result<String, Box<Error>> {
        self.0.get_alias()
    }
    fn set_alias(&self, alias: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }

    fn get_class(&self) -> Result<u32, Box<Error>> {
        self.0.get_class()
    }
    fn set_class(&self, class: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }
    fn is_powered(&self) -> Result<bool, Box<Error>> {
        self.0.is_powered()
    }
    fn set_powered(&self, powered: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn is_present(&self) -> Result<bool, Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_present(&self, present: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }

    fn is_discoverable(&self) -> Result<bool, Box<Error>> {
       self.0.is_discoverable()
    }
    fn set_discoverable(&self, discoverable: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }


    fn is_pairable(&self) -> Result<bool, Box<Error>> {
       self.0.is_pairable()
    }
    fn set_pairable(&self, pairable: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }


    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_pairable_timeout()
    }
    fn set_pairable_timeout(&self, timeout: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))   
    }

    fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_discoverable_timeout()
    }
    fn set_discoverable_timeout(&self, timeout: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn is_discovering(&self) -> Result<bool, Box<Error>> {
       self.0.is_discovering()
    }

    fn set_discovering(&self, discovering: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_can_start_discovery(&self, can_start_discovery: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_can_stop_discovery(&self, can_stop_discovery: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        let mac_adapter = try!(BluetoothAdapterMac::init());
        let mac_session = try!(BluetoothDiscoverySessionMac::create_session(Arc::new(mac_adapter)));
        Ok(BluetoothDiscoverySession::Mac(Arc::new(mac_session)))
    }

    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        self.0.get_uuids()
    }

    fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
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
    fn set_modalias(&self, modalias: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn get_ad_datas(&self) -> Result<Vec<String>, Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_ad_datas(&self, ad_datas: Vec<String>) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
      
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
    fn set_address(&self, address: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn get_name(&self) -> Result<String, Box<Error>> {
        self.0.get_name()
    }

    fn set_name(&self, name: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn get_alias(&self) -> Result<String, Box<Error>> {
        self.0.get_alias()
    }
    fn set_alias(&self, alias: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }

    fn get_class(&self) -> Result<u32, Box<Error>> {
        self.0.get_class()
    }
    fn set_class(&self, class: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }
    fn is_powered(&self) -> Result<bool, Box<Error>> {
        self.0.is_powered()
    }

    fn set_powered(&self, powered: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn is_present(&self) -> Result<bool, Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_present(&self, present: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn is_discoverable(&self) -> Result<bool, Box<Error>> {
       self.0.is_discoverable()
    }
    fn set_discoverable(&self, discoverable: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }


    fn is_pairable(&self) -> Result<bool, Box<Error>> {
       self.0.is_pairable()
    }
    fn set_pairable(&self, pairable: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }


    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_pairable_timeout()
    }
    fn set_pairable_timeout(&self, timeout: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))   
    }

    fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
       self.0.get_discoverable_timeout()
    }
    fn set_discoverable_timeout(&self, timeout: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn is_discovering(&self) -> Result<bool, Box<Error>> {
       self.0.is_discovering()
    }

    fn set_discovering(&self, discovering: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_can_start_discovery(&self, can_start_discovery: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_can_stop_discovery(&self, can_stop_discovery: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        let adapter = try!(BluetoothAdapterEmpty::init());
        let empty_session = try!(BluetoothDiscoverySessionEmpty::create_session(Arc::new(adapter)));
        Ok(BluetoothDiscoverySession::Empty(Arc::new(empty_session)))
        
    }

    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        self.0.get_uuids()
    }

    fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
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
    fn set_modalias(&self, modalias: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn get_ad_datas(&self) -> Result<Vec<String>, Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))

    }
    fn set_ad_datas(&self, ad_datas: Vec<String>) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
      
    }
}

#[cfg(feature = "bluetooth-test")]
struct Mock(Arc<FakeBluetoothAdapter>);

#[cfg(feature = "bluetooth-test")]
impl BluetoothAdapter for Mock{

    fn get_id(&self) -> String {
        self.0.get_id()
    }

    fn set_id(&self, id: String) {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_id(id),
            _ => (),
        }
    }

    fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>> {
        let device_list = try!(set.0.get_device_list());
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

    fn set_address(&self, address: String) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_address(address),
            _ => Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR)),
        }
    }

    fn get_name(&self) -> Result<String, Box<Error>> {
       self.0.get_name()
    }

    fn set_name(&self, name: String) -> Result<(), Box<Error>> {
       self.0.set_name()
    }

    fn get_alias(&self) -> Result<String, Box<Error>> {
       self.0.get_alias()
    }

    fn set_alias(&self, alias: String) -> Result<(), Box<Error>> {
        self.0.set_alias()
    }

    fn get_class(&self) -> Result<u32, Box<Error>> {
        self.0.get_class()
    }

    fn set_class(&self, class: u32) -> Result<(), Box<Error>> {
        self.0.set_class()
    }

    fn is_powered(&self) -> Result<bool, Box<Error>> {
       self.0.is_powered()
    }

    fn set_powered(&self, powered: bool) -> Result<(), Box<Error>> {
        self.0.set_powered()
    }

    fn is_present(&self) -> Result<bool, Box<Error>> {
        self.0.is_present()
    }

    fn set_present(&self, present: bool) -> Result<(), Box<Error>> {
        self.0.set_present()
    }

    fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        self.0.is_discoverable()
    }

    
    fn set_discoverable(&self, discoverable: bool) -> Result<(), Box<Error>> {
       self.0.set_discoverable()
    }

    fn is_pairable(&self) -> Result<bool, Box<Error>> {
        self.0.is_pairable()
    }

    fn set_pairable(&self, pairable: bool) -> Result<(), Box<Error>> {
        self.0.set_pairable()
    }

    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        self.0.get_pairable_timeout()
    }

    fn set_pairable_timeout(&self, timeout: u32) -> Result<(), Box<Error>> {
        self.0.set_pairable_timeout()
    }

    fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        self.0.get_discoverable_timeout()
    }

    fn set_discoverable_timeout(&self, timeout: u32) -> Result<(), Box<Error>> {
        self.0.set_discoverable_timeout()
    }

    fn is_discovering(&self) -> Result<bool, Box<Error>> {
        self.0.is_discovering()
    }

    fn set_discovering(&self, discovering: bool) -> Result<(), Box<Error>> {
        self.0.set_discovering()
    }

    fn set_can_start_discovery(&self, can_start_discovery: bool) -> Result<(), Box<Error>> {
        self.0.set_can_start_discovery()
    }

    fn set_can_stop_discovery(&self, can_stop_discovery: bool) -> Result<(), Box<Error>> {
        self.0.set_can_stop_discovery()
    }

    fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        //BluetoothDiscoverySession::create_session(self.clone())
        let test_session = try!(FakeBluetoothDiscoverySession::create_session(fake_adapter));
        Ok(BluetoothDiscoverySession::Mock(Arc::new(test_session)))   
    }

    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
       self.0.get_uuids()
    }

    fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>> {
        self.0.set_uuids()
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

    fn set_modalias(&self, modalias: String) -> Result<(), Box<Error>> {
        self.0.set_modalias()
    }

    fn get_ad_datas(&self) -> Result<Vec<String>, Box<Error>> {
        self.0.get_ad_datas()
    }

    fn set_ad_datas(&self, ad_datas: Vec<String>) -> Result<(), Box<Error>> {
        self.0.set_ad_datas()
    }

}

