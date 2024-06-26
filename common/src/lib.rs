#![no_std]
/// BasicFeaturesIpv4 is a struct collection all ipv4 traffic data and is 24 bytes in size.
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct BasicFeaturesIpv4 {
    pub ipv4_destination: u32,
    pub ipv4_source: u32,
    pub port_destination: u16,
    pub port_source: u16,
    pub data_length: u16,
    pub length: u16,
    pub window_size: u16,
    pub combined_flags: u8,
    pub protocol: u8,
    pub header_length: u8,
    pub _padding: [u8; 3], // Explicit padding to match the size of the struct in the eBPF program
}

impl BasicFeaturesIpv4 {
    pub fn new(
        ipv4_destination: u32,
        ipv4_source: u32,
        port_destination: u16,
        port_source: u16,
        data_length: u16,
        length: u16,
        window_size: u16,
        combined_flags: u8,
        protocol: u8,
        header_length: u8,
    ) -> Self {
        BasicFeaturesIpv4 {
            ipv4_destination,
            ipv4_source,
            port_destination,
            port_source,
            data_length,
            length,
            window_size,
            combined_flags,
            protocol,
            header_length,
            _padding: [0; 3],
        }
    }
}

#[cfg(feature = "user")]
unsafe impl aya::Pod for BasicFeaturesIpv4 {}

/// BasicFeaturesIpv6 is a struct collection all ipv6 traffic data and is 48 bytes in size.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BasicFeaturesIpv6 {
    pub ipv6_destination: u128,
    pub ipv6_source: u128,
    pub port_destination: u16,
    pub port_source: u16,
    pub data_length: u16,
    pub length: u16,
    pub window_size: u16,
    pub combined_flags: u8,
    pub protocol: u8,
    pub header_length: u8,
    _padding: [u8; 3], // Explicit padding to match the size of the struct in the eBPF program
}

impl BasicFeaturesIpv6 {
    pub fn new(
        ipv6_destination: u128,
        ipv6_source: u128,
        port_destination: u16,
        port_source: u16,
        data_length: u16,
        length: u16,
        window_size: u16,
        combined_flags: u8,
        protocol: u8,
        header_length: u8,
    ) -> Self {
        BasicFeaturesIpv6 {
            ipv6_destination,
            ipv6_source,
            port_destination,
            port_source,
            data_length,
            length,
            window_size,
            combined_flags,
            protocol,
            header_length,
            _padding: [0; 3],
        }
    }
}

#[cfg(feature = "user")]
unsafe impl aya::Pod for BasicFeaturesIpv6 {}
