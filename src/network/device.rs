use super::address::{IpAddr, Ipv4Addr, Ipv6Addr, MacAddress};
use super::interface::Interface;

use bevy::prelude::*;

pub trait NetworkDevice {
    fn ping(&self, ip: IpAddr) -> bool;
}

pub enum RouterModel {
    Generic,
    Cisco1841,
    Cisco1921,
    Cisco2911,
    Cisco4331,
    Cisco4431,
    Cisco4451,
}

pub enum SwitchModel {
    Generic,
    Cisco2960,
    Cisco3560,
    Cisco3750,
    Cisco3850,
}

#[derive(Component)]
pub struct Router {
    pub model: RouterModel,
    pub interfaces: Vec<Interface>,
}

impl Router {
    pub fn new(model: RouterModel) -> Self {
        Self {
            model,
            interfaces: Vec::new(),
        }
    }

    pub fn add_interface(&mut self, interface: Interface) {
        self.interfaces.push(interface);
    }
}

impl NetworkDevice for Router {
    // TODO: implement ping
    fn ping(&self, ip: IpAddr) -> bool {
        true
    }
}

#[derive(Component)]
pub struct Switch {
    pub model: SwitchModel,
}

impl Switch {
    pub fn new(model: SwitchModel) -> Self {
        Self { model }
    }
}
#[derive(Component)]
pub struct Endpoint {
    pub os_type: OsType,
}

impl Endpoint {
    pub fn new(os_type: OsType) -> Self {
        Self { os_type }
    }
}

pub enum OsType {
    Windows,
    MacOS,
    Linux,
}
