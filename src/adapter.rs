#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_adapter::BluetoothAdapter as BluetoothAdapterBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_adapter::Adapter as BluetoothAdapterAndroid;
#[cfg(all(target_os = "macos", feature = "bluetooth"))]
use blurmac::BluetoothAdapter as BluetoothAdapterMac;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
              all(target_os = "android", feature = "bluetooth"),
              all(target_os = "macos", feature = "bluetooth"))))]
use empty::EmptyAdapter as BluetoothAdapterEmpty;
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
   fn set_id(&self, id: String)-> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
     }
    fn get_devices(&self)-> Result<Vec<BluetoothDevice>, Box<Error>>;
    fn get_device(&self, address: String) -> Result<Option<BluetoothDevice>, Box<Error>>;
    fn create_mock_device(&self, device: String) -> Result<BluetoothDevice, Box<Error>> {
         Err(Box::from(NOT_SUPPORTED_ERROR))   
    }
    fn get_address(&self) -> Result<String, Box<Error>>;
    fn set_address(&self, address: String) -> Result<(), Box<Error>>{
         Err(Box::from(NOT_SUPPORTED_ERROR))
     }
    fn get_name(&self)-> Result<String, Box<Error>>;
    fn set_name(&self, name: String) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_alias(&self) -> Result<String, Box<Error>>;
    fn set_alias(&self, alias: String) -> Result<(), Box<Error>>{
         Err(Box::from(NOT_SUPPORTED_ERROR))  
    }
    fn get_class(&self)-> Result<u32, Box<Error>>;
    fn set_class(&self, class: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }
    fn is_powered(&self)-> Result<bool, Box<Error>>;
    fn set_powered(&self, powered: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }
    fn is_present(&self) -> Result<bool, Box<Error>>{
         Err(Box::from(NOT_SUPPORTED_ERROR))  
    }
    fn set_present(&self, present: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }
    fn is_discoverable(&self) -> Result<bool, Box<Error>>;
    fn set_discoverable(&self, discoverable: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }
    fn is_pairable(&self)-> Result<bool, Box<Error>>;
    fn set_pairable(&self, pairable: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }
    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>>;
    fn set_pairable_timeout(&self, timeout: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }
    fn get_discoverable_timeout(&self)-> Result<u32, Box<Error>>;
    fn set_discoverable_timeout(&self, timeout: u32) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    } 
    fn is_discovering(&self)-> Result<bool, Box<Error>>;
    fn set_discovering(&self, discovering: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    } 
    fn set_can_start_discovery(&self, can_start_discovery: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }
    fn set_can_stop_discovery(&self, can_stop_discovery: bool) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    }
    fn create_discovery_session(&self)-> Result<BluetoothDiscoverySession, Box<Error>>;
    fn get_uuids(&self)-> Result<Vec<String>, Box<Error>>;
    fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>>{
        Err(Box::from(NOT_SUPPORTED_ERROR))  
    } 
    fn get_vendor_id_source(&self)-> Result<String, Box<Error>>;
    fn get_vendor_id(&self)-> Result<u32, Box<Error>>;
    fn get_product_id(&self) -> Result<u32, Box<Error>> ;
    fn get_device_id(&self) -> Result<u32, Box<Error>>;
    fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>>;  
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

impl BluetoothAdapter{
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    pub fn new() -> Result<Box<BluetoothAdapter>, Box<Error>> {
        let bluez_adapter = try!(BluetoothAdapterBluez::init());
        Ok(Box::new(bluez_adapter))
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    pub fn new() -> Result<Box<BluetoothAdapter>, Box<Error>> {
        let blurdroid_adapter = try!(BluetoothAdapterAndroid::get_adapter());
        Ok(Box::new(blurdroid_adapter))
    }

    #[cfg(all(target_os = "macos", feature = "bluetooth"))]
    pub fn new() -> Result<Box<BluetoothAdapter>, Box<Error>> {
        let mac_adapter = try!(BluetoothAdapterMac::init());
        Ok(Box::new(mac_adapter))
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                  all(target_os = "android", feature = "bluetooth"),
                  all(target_os = "macos", feature = "bluetooth"))))]
    pub fn new() -> Result<Box<BluetoothAdapter>, Box<Error>> {
        let adapter = try!(BluetoothAdapterEmpty::init());
        Ok(Box::new(Empty(Arc::new(adapter))))
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn new_mock() -> Result<Box<BluetoothAdapter>, Box<Error>> {
        let fake_adapter = Mock(FakeBluetoothAdapter::new_empty());
        Ok(Box::new(fake_adapter))
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
        let bluez_session = try!(BluetoothDiscoverySessionBluez::create_session(self.get_id()));
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
    fn set_modalias(&self, modalias: String) -> Result<(), Box<Error>>{
        self.0.set_modalias(modalias)
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
        Ok(device_list.into_iter().map(|device|  BluetoothDevice::Android(Arc::new(BluetoothDeviceAndroid::new(self.0.clone(), device)))).collect())
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
        let blurdroid_session = try!(BluetoothDiscoverySessionAndroid::create_session(self.0.clone()));
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
        Ok(device_list.into_iter().map(|device| BluetoothDevice::Mac(Arc::new(BluetoothDeviceMac::new(self.0.clone(),device)))).collect())
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
        let mac_session = BluetoothDiscoverySessionMac{};
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

    fn set_discoverable(&self, discoverable: bool) -> Result<(), Box<Error>>{
        self.0.set_discoverable(discoverable)
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
    fn set_discoverable_timeout(&self, timeout: u32) -> Result<(), Box<Error>>{
       self.0.set_discoverable_timeout(timeout)
    }

    fn is_discovering(&self) -> Result<bool, Box<Error>> {
       self.0.is_discovering()
    }
    
    fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        let empty_session = BluetoothDiscoverySessionEmpty{};
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


#[cfg(feature = "bluetooth-test")]
struct Mock(Arc<FakeBluetoothAdapter>);

#[cfg(feature = "bluetooth-test")]
impl BluetoothAdapter for Mock{

    fn get_id(&self) -> String {
        self.0.get_id()
    }

    fn set_id(&self, id: String) -> Result<(), Box<Error>> {
       Ok(self.0.set_id(id))
    }

    fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>> {
        let device_list = try!(self.0.get_device_list());
        Ok(device_list.into_iter().map(|device| BluetoothDevice::Mock(FakeBluetoothDevice::new_empty(self.0.clone(), device))).collect())
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

    fn create_mock_device(&self, device: String) -> Result<BluetoothDevice, Box<Error>> {
        Ok(BluetoothDevice::Mock(FakeBluetoothDevice::new_empty(self.0.clone(), device)))
        
    }

    fn get_address(&self) -> Result<String, Box<Error>> {
       self.0.get_address()
    }

    fn set_address(&self, address: String) -> Result<(), Box<Error>> {
        self.0.set_address(address)
    }

    fn get_name(&self) -> Result<String, Box<Error>> {
       self.0.get_name()
    }

    fn set_name(&self, name: String) -> Result<(), Box<Error>> {
       self.0.set_name(name)
    }

    fn get_alias(&self) -> Result<String, Box<Error>> {
       self.0.get_alias()
    }

    fn set_alias(&self, alias: String) -> Result<(), Box<Error>> {
        self.0.set_alias(alias)
    }

    fn get_class(&self) -> Result<u32, Box<Error>> {
        self.0.get_class()
    }

    fn set_class(&self, class: u32) -> Result<(), Box<Error>> {
        self.0.set_class(class)
    }

    fn is_powered(&self) -> Result<bool, Box<Error>> {
       self.0.is_powered()
    }

    fn set_powered(&self, powered: bool) -> Result<(), Box<Error>> {
        self.0.set_powered(powered)
    }

    fn is_present(&self) -> Result<bool, Box<Error>> {
        self.0.is_present()
    }

    fn set_present(&self, present: bool) -> Result<(), Box<Error>> {
        self.0.set_present(present)
    }

    fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        self.0.is_discoverable()
    }

    
    fn set_discoverable(&self, discoverable: bool) -> Result<(), Box<Error>> {
       self.0.set_discoverable(discoverable)
    }

    fn is_pairable(&self) -> Result<bool, Box<Error>> {
        self.0.is_pairable()
    }

    fn set_pairable(&self, pairable: bool) -> Result<(), Box<Error>> {
        self.0.set_pairable(pairable)
    }

    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        self.0.get_pairable_timeout()
    }

    fn set_pairable_timeout(&self, timeout: u32) -> Result<(), Box<Error>> {
        self.0.set_pairable_timeout(timeout)
    }

    fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        self.0.get_discoverable_timeout()
    }

    fn set_discoverable_timeout(&self, timeout: u32) -> Result<(), Box<Error>> {
        self.0.set_discoverable_timeout(timeout)
    }

    fn is_discovering(&self) -> Result<bool, Box<Error>> {
        self.0.is_discovering()
    }

    fn set_discovering(&self, discovering: bool) -> Result<(), Box<Error>> {
        self.0.set_discovering(discovering)
    }

    fn set_can_start_discovery(&self, can_start_discovery: bool) -> Result<(), Box<Error>> {
        self.0.set_can_start_discovery(can_start_discovery)
    }

    fn set_can_stop_discovery(&self, can_stop_discovery: bool) -> Result<(), Box<Error>> {
        self.0.set_can_stop_discovery(can_stop_discovery)
    }

    fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        let test_session = try!(FakeBluetoothDiscoverySession::create_session(self.0.clone()));
        Ok(BluetoothDiscoverySession::Mock(Arc::new(test_session)))   

    }

    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
       self.0.get_uuids()
    }

    fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>> {
        self.0.set_uuids(uuids)
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
        self.0.set_modalias(modalias)
    }

    fn get_ad_datas(&self) -> Result<Vec<String>, Box<Error>> {
        self.0.get_ad_datas()
    }

    fn set_ad_datas(&self, ad_datas: Vec<String>) -> Result<(), Box<Error>> {
        self.0.set_ad_datas(ad_datas)
    }

}

