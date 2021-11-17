use log::*;
///IpAddress enum which have variants for classes of ipaddress
///
/// #variant
///
/// ClassA:-ClassA is variant of enum IpAddress and it is String type
///
/// ClassB:-ClassB is variant of enum IpAddress and it is String type
///
/// ClassC:-ClassC is variant of enum IpAddress and it is String type
///
/// ClassD:-ClassD is variant of enum IpAddress and it is String type
///
/// ClassE:- ClassE is variant of enum IpAddress and it is String type
#[derive(PartialEq, Eq, Debug)]
pub enum IpAddress {
    ClassA(String),
    ClassB(String),
    ClassC(String),
    ClassD(String),
    ClassE(String),
}

/// check_ip_address which is used check the given ip_Address
///
/// #Arguments
///
///ipconfig: A ipconfig is tuple object of unsigned integer type
///
/// #Return
///
/// No return
pub fn check_ip_address(ipconfig: (u128, u128, u128, u128)) -> Result<IpAddress, String> {
    match ipconfig {
        (1..=127, 0..=255, 0..=255, 1..=254) => Ok(IpAddress::ClassA(format!(
            "{}.{}.{}.{}",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ))),
        (128..=191, 0..=255, 0..=255, 1..=254) => Ok(IpAddress::ClassB(format!(
            "{}.{}.{}.{}",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ))),
        (192..=223, 0..=255, 1..=255, 1..=255) => Ok(IpAddress::ClassC(format!(
            "{}.{}.{}.{}",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ))),
        (224..=239, 0..=255, 0..=255, 0..=255) => Ok(IpAddress::ClassD(format!(
            "{}.{}.{}.{}",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ))),
        (240..=254, 0..=255, 0..=255, 0..=254) => Ok(IpAddress::ClassE(format!(
            "{}.{}.{}.{}",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ))),
        _ => {
            error!("Wrong Ip Provided");
            Err("Unwanted Input".to_string())
    }
    }
}
