use core::array::FixedSizeArray;
use alloc::boxed::Box;

#[derive(Debug)]
#[repr(u8)]
#[allow(dead_code)]
pub enum DeviceClass {
    Legacy = 0x00,
    MassStorage = 0x01,
    Network = 0x02,
    Display = 0x03,
    Multimedia = 0x04,
    Memory = 0x05,
    BridgeDevice = 0x06,
    SimpleCommunication = 0x07,
    BaseSystemPeripheral = 0x08,
    InputDevice = 0x09,
    DockingStation = 0x0A,
    Processor = 0x0B,
    SerialBus = 0x0C,
    Wireless = 0x0D,
    IntelligentIO = 0x0E,
    SatelliteCommunication = 0x0F,
    EncryptionDecryption = 0x10,
    DataAndSignalProcessing = 0x11,
    Unknown,
}


impl DeviceClass {
    fn from_u8(c: u8) -> DeviceClass {
        if c <= DeviceClass::DataAndSignalProcessing as u8 {
            unsafe { ::core::intrinsics::transmute(c) }
        } else {
            DeviceClass::Unknown
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
#[allow(dead_code)]
enum HeaderType {
    Standard = 0x00,
    Pci2PciBridge = 0x01,
    CardBusBridge = 0x02,
    Unknown,
}


impl HeaderType {
    fn from_u8(c: u8) -> HeaderType {
        if c <= HeaderType::CardBusBridge as u8 {
            unsafe { ::core::intrinsics::transmute(c) }
        } else {
            println!("unknown type: {}", c);
            HeaderType::Unknown
        }
    }
}

#[allow(dead_code)]
#[repr(packed)]
pub struct CommonHeader {
    pub vendor_id: u16,
    pub device_id: u16,
    pub command: u16,
    pub status: u16,
    pub revision_id: u8,
    pub prog_if: u8,
    pub subclass: u8,
    pub class_code: u8,
    pub cache_line_size: u8,
    pub latency_timer: u8,
    pub header_type: u8,
    pub bist: u8,
}

impl CommonHeader {
    fn new(registers: &[u32]) -> CommonHeader {
        assert_eq!(registers.len(), 4);
        CommonHeader {
            vendor_id: registers[0] as u16,
            device_id: (registers[0] >> 16) as u16,
            command: registers[1] as u16,
            status: (registers[1] >> 16) as u16,
            revision_id: registers[2] as u8,
            prog_if: (registers[2] >> 8) as u8,
            subclass: (registers[2] >> 16) as u8,
            class_code: (registers[2] >> 24) as u8,
            cache_line_size: registers[3] as u8,
            latency_timer: (registers[3] >> 8) as u8,
            header_type: (registers[3] >> 16) as u8,
            bist: (registers[3] >> 24) as u8,
        }
    }

    fn header_type(&self) -> HeaderType {
        HeaderType::from_u8(self.header_type & 0b0111_1111)
    }
}

pub fn is_valid(register_00: u32) -> bool {
    (register_00 as u16) != 0xFFFF
}

pub fn build_header(registers: [u32; 18]) -> Option<Box<PciHeader>> {
    let (common_registers, other_registers) = array_refs!(&registers, 4, 14);
    match common_header.header_type() {
        HeaderType::Standard => Some(Box::new(StandardHeader::new(
            CommonHeader::new(common_registers),
            other_registers,
        ))),
        HeaderType::Pci2PciBridge => unimplemented!(),
        HeaderType::CardBusBridge => unimplemented!(),
        _ => None,
    }
}

pub trait PciHeader {
    fn new(common_header: CommonHeader, registers: &[u32; 14]) -> Self
    where
        Self: Sized;

    fn common(&self) -> &CommonHeader;
}


// impl fmt::Display for FunctionInfo {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{}.{}.{}: {:04x} {:04x} {:?} {:02x}",
//             self.bus,
//             self.device,
//             self.function,
//             self.vendor_id,
//             self.device_id,
//             self.class_code,
//             self.subclass
//         )
//     }
// }

#[allow(dead_code)]
#[repr(packed)]
struct StandardHeader {
    common: CommonHeader,
    base_addresses: [u32; 6],
    cardbus_pointer: u32,
    subsystem_vendor_id: u16,
    subsystem_id: u16,
    expansion_rom_address: u32,
    capabilities_pointer: u8,
    reserved: [u8; 7],
    interrupt_line: u8,
    interrupt_pin: u8,
    min_grant: u8,
    max_latency: u8,
}

impl PciHeader for StandardHeader {
    fn new(common_header: CommonHeader, registers: &[u32; 14]) -> StandardHeader {
        StandardHeader {
            common: common_header,
            base_addresses: [
                registers[0],
                registers[1],
                registers[2],
                registers[3],
                registers[4],
                registers[5],
            ],
            cardbus_pointer: registers[6],
            subsystem_vendor_id: registers[7] as u16,
            subsystem_id: (registers[7] >> 16) as u16,
            expansion_rom_address: registers[8],
            capabilities_pointer: registers[11] as u8,
            reserved: [0; 7],
            interrupt_line: registers[11] as u8,
            interrupt_pin: (registers[11] >> 8) as u8,
            min_grant: (registers[11] >> 16) as u8,
            max_latency: (registers[11] >> 24) as u8,
        }
    }

    fn common(&self) -> &CommonHeader {
        &self.common
    }
}
