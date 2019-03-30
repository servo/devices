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

#[derive(Clone, Debug)]
pub trait BluetoothAdapter {
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

pub struct DeviceType
{
	target_os: String,
	feature: String
}

impl BluetoothAdapter for DeviceType
{
	#[cfg(all(target_os = "linux", feature = "bluetooth"))]
    pub fn new() -> Result<BluetoothAdapter, Box<Error>> {
        let bluez_adapter = try!(BluetoothAdapterBluez::init());
        Ok(BluetoothAdapter::Bluez(Arc::new(bluez_adapter)))
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    pub fn new() -> Result<BluetoothAdapter, Box<Error>> {
        let blurdroid_adapter = try!(BluetoothAdapterAndroid::get_adapter());
        Ok(BluetoothAdapter::Android(Arc::new(blurdroid_adapter)))
    }

    #[cfg(all(target_os = "macos", feature = "bluetooth"))]
    pub fn new() -> Result<BluetoothAdapter, Box<Error>> {
        let mac_adapter = try!(BluetoothAdapterMac::init());
        Ok(BluetoothAdapter::Mac(Arc::new(mac_adapter)))
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"),
                  all(target_os = "android", feature = "bluetooth"),
                  all(target_os = "macos", feature = "bluetooth"))))]
    pub fn new() -> Result<BluetoothAdapter, Box<Error>> {
        let adapter = try!(BluetoothAdapterEmpty::init());
        Ok(BluetoothAdapter::Empty(Arc::new(adapter)))
    }

    #[cfg(feature = "bluetooth-test")]
    pub fn new() -> Result<BluetoothAdapter, Box<Error>> {
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
