use core::fmt;

pub fn is_valid(register_00: u32) -> bool {
    (register_00 as u16) != 0xFFFF
}

#[derive(Debug, Clone, Copy)]
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
    pub fn from_u8(c: u8) -> DeviceClass {
        if c <= DeviceClass::DataAndSignalProcessing as u8 {
            unsafe { ::core::intrinsics::transmute(c) }
        } else {
            DeviceClass::Unknown
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[allow(dead_code)]
pub enum HeaderType {
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
            debugln!("unknown type: {}", c);
            HeaderType::Unknown
        }
    }
}

#[allow(dead_code)]
#[repr(packed)]
pub struct Header {
    // Common Header Fields
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
    // Standard Header Fields
    pub base_addresses: [u32; 6],
    pub cardbus_pointer: u32,
    pub subsystem_vendor_id: u16,
    pub subsystem_id: u16,
    pub expansion_rom_address: u32,
    pub capabilities_pointer: u8,
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub min_grant: u8,
    pub max_latency: u8,
    // Enum to store device class
    pub device_class: DeviceClass,
    pub header_enum: HeaderType,
    pub multifunction: bool,
}

impl Header {
    #[allow(unused_variables, unused_assignments)]
    pub fn new(registers: [u32; 18]) -> Header {
        let vendor_id = registers[0] as u16;
        let device_id = (registers[0] >> 16) as u16;
        let command = registers[1] as u16;
        let status = (registers[1] >> 16) as u16;
        let revision_id = registers[2] as u8;
        let prog_if = (registers[2] >> 8) as u8;
        let subclass = (registers[2] >> 16) as u8;
        let class_code = (registers[2] >> 24) as u8;
        let cache_line_size = registers[3] as u8;
        let latency_timer = (registers[3] >> 8) as u8;
        let header_type = (registers[3] >> 16) as u8;
        let bist = (registers[3] >> 24) as u8;

        // Standard Header Fields
        let mut base_addresses = [0u32; 6];
        let mut cardbus_pointer = 0u32;
        let mut subsystem_vendor_id = 0u16;
        let mut subsystem_id = 0u16;
        let mut expansion_rom_address = 0u32;
        let mut capabilities_pointer = 0u8;
        let mut interrupt_line = 0u8;
        let mut interrupt_pin = 0u8;
        let mut min_grant = 0u8;
        let mut max_latency = 0u8;

        let device_class = DeviceClass::from_u8(class_code);
        let header_enum = HeaderType::from_u8(header_type & 0b0111_1111);
        let multifunction = (header_type >> 7) != 0;
        match header_enum {
            HeaderType::Standard => {
                base_addresses = [
                    registers[4],
                    registers[5],
                    registers[6],
                    registers[7],
                    registers[8],
                    registers[9],
                ];
                cardbus_pointer = registers[10];
                subsystem_vendor_id = registers[11] as u16;
                subsystem_id = (registers[11] >> 16) as u16;
                expansion_rom_address = registers[12];
                capabilities_pointer = registers[13] as u8;
                interrupt_line = registers[15] as u8;
                interrupt_pin = (registers[15] >> 8) as u8;
                min_grant = (registers[15] >> 16) as u8;
                max_latency = (registers[15] >> 24) as u8;
            }
            _ => unimplemented!("Header {:?} not implemented", header_enum),
        };

        Header {
            // Common Header Fields
            vendor_id: vendor_id,
            device_id: device_id,
            command: command,
            status: status,
            revision_id: revision_id,
            prog_if: prog_if,
            subclass: subclass,
            class_code: class_code,
            cache_line_size: cache_line_size,
            latency_timer: latency_timer,
            header_type: header_type,
            bist: bist,
            // Standard Header Fields
            base_addresses: base_addresses,
            cardbus_pointer: cardbus_pointer,
            subsystem_vendor_id: subsystem_vendor_id,
            subsystem_id: subsystem_id,
            expansion_rom_address: expansion_rom_address,
            capabilities_pointer: capabilities_pointer,
            interrupt_line: interrupt_line,
            interrupt_pin: interrupt_pin,
            min_grant: min_grant,
            max_latency: max_latency,
            // Other precalculated values
            device_class: device_class,
            header_enum: header_enum,
            multifunction: multifunction,
        }
    }

    fn device_class(&self) -> DeviceClass {
        self.device_class
    }

    fn header_type(&self) -> HeaderType {
        self.header_enum
    }

    pub fn description(&self) -> &'static str {
        get_device_description(self.device_class, self.subclass, self.prog_if)
    }

    pub fn is_multifunction(&self) -> bool {
        self.multifunction
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(
                f,
                "Vendor: {:04x} Device: {:04x} {:?} {}",
                self.vendor_id,
                self.device_id,
                self.device_class,
                self.description(),
            )
        }
    }
}



fn get_device_description(device_class: DeviceClass, subclass: u8, prog_if: u8) -> &'static str {
    // Descriptions from http://wiki.osdev.org/PCI
    match device_class {
        DeviceClass::Legacy => match (subclass, prog_if) {
            (0x00, 0x00) => "Any device except for VGA-Compatible devices",
            (0x01, 0x00) => "VGA-Compatible Device",
            (_, _) => "",
        },
        DeviceClass::MassStorage => match (subclass, prog_if) {
            (0x00, 0x00) => "SCSI Bus Controller",
            (0x01, _) => "IDE Controller",
            (0x02, 0x00) => "Floppy Disk Controller",
            (0x03, 0x00) => "IPI Bus Controller",
            (0x04, 0x00) => "RAID Controller",
            (0x05, 0x20) => "ATA Controller (Single DMA)",
            (0x05, 0x30) => "ATA Controller (Chained DMA)",
            (0x06, 0x00) => "Serial ATA (Vendor Specific Interface)",
            (0x06, 0x01) => "Serial ATA (AHCI 1.0)",
            (0x07, 0x00) => "Serial Attached SCSI (SAS)",
            (0x08, 0x00) => "Other Mass Storage Controller",
            (_, _) => "",
        },
        DeviceClass::Network => match (subclass, prog_if) {
            (0x00, 0x00) => "Ethernet Controller",
            (0x01, 0x00) => "Token Ring Controller",
            (0x02, 0x00) => "FDDI Controller",
            (0x03, 0x00) => "ATM Controller",
            (0x04, 0x00) => "ISDN Controller",
            (0x05, 0x00) => "WorldFip Controller",
            (0x06, _) => "PICMG 2.14 Multi Computing",
            (0x80, 0x00) => "Other Network Controller",
            (_, _) => "",
        },
        DeviceClass::Display => match (subclass, prog_if) {
            (0x00, 0x00) => "VGA-Compatible Controller",
            (0x00, 0x01) => "8512-Compatible Controller",
            (0x01, 0x00) => "XGA Controller",
            (0x02, 0x00) => "3D Controller (Not VGA-Compatible)",
            (0x80, 0x00) => "Other Display Controller",
            (_, _) => "",
        },
        DeviceClass::Multimedia => match (subclass, prog_if) {
            (0x00, 0x00) => "Video Device",
            (0x01, 0x00) => "Audio Device",
            (0x02, 0x00) => "Computer Telephony Device",
            (0x80, 0x00) => "Other Multimedia Device",
            (_, _) => "",
        },
        DeviceClass::Memory => match (subclass, prog_if) {
            (0x00, 0x00) => "RAM Controller",
            (0x01, 0x00) => "Flash Controller",
            (0x80, 0x00) => "Other Memory Controller",
            (_, _) => "",
        },
        DeviceClass::BridgeDevice => match (subclass, prog_if) {
            (0x00, 0x00) => "Host Bridge",
            (0x01, 0x00) => "ISA Bridge",
            (0x02, 0x00) => "EISA Bridge",
            (0x03, 0x00) => "MCA Bridge",
            (0x04, 0x00) => "PCI-to-PCI Bridge",
            (0x04, 0x01) => "PCI-to-PCI Bridge (Subtractive Decode)",
            (0x05, 0x00) => "PCMCIA Bridge",
            (0x06, 0x00) => "NuBus Bridge",
            (0x07, 0x00) => "CardBus Bridge",
            (0x08, _) => "RACEway Bridge",
            (0x09, 0x40) => "PCI-to-PCI Bridge (Semi-Transparent, Primary)",
            (0x09, 0x80) => "PCI-to-PCI Bridge (Semi-Transparent, Secondary)",
            (0x0A, 0x00) => "InfiniBrand-to-PCI Host Bridge",
            (0x80, 0x00) => "Other Bridge Device",
            (_, _) => "",
        },
        DeviceClass::SimpleCommunication => match (subclass, prog_if) {
            (0x00, 0x00) => "Generic XT-Compatible Serial Controller",
            (0x00, 0x01) => "16450-Compatible Serial Controller",
            (0x00, 0x02) => "16550-Compatible Serial Controller",
            (0x00, 0x03) => "16650-Compatible Serial Controller",
            (0x00, 0x04) => "16750-Compatible Serial Controller",
            (0x00, 0x05) => "16850-Compatible Serial Controller",
            (0x00, 0x06) => "16950-Compatible Serial Controller",
            (0x01, 0x00) => "Parallel Port",
            (0x01, 0x01) => "Bi-Directional Parallel Port",
            (0x01, 0x02) => "ECP 1.X Compliant Parallel Port",
            (0x01, 0x03) => "IEEE 1284 Controller",
            (0x01, 0xFE) => "IEEE 1284 Target Device",
            (0x02, 0x00) => "Multiport Serial Controller",
            (0x03, 0x00) => "Generic Modem",
            (0x03, 0x01) => "Hayes Compatible Modem (16450-Compatible Interface)",
            (0x03, 0x02) => "Hayes Compatible Modem (16550-Compatible Interface)",
            (0x03, 0x03) => "Hayes Compatible Modem (16650-Compatible Interface)",
            (0x03, 0x04) => "Hayes Compatible Modem (16750-Compatible Interface)",
            (0x04, 0x00) => "IEEE 488.1/2 (GPIB) Controller",
            (0x05, 0x00) => "Smart Card",
            (0x80, 0x00) => "Other Communications Device",
            (_, _) => "",
        },
        DeviceClass::BaseSystemPeripheral => match (subclass, prog_if) {
            (0x00, 0x00) => "Generic 8259 PIC",
            (0x00, 0x01) => "ISA PIC",
            (0x00, 0x02) => "EISA PIC",
            (0x00, 0x10) => "I/O APIC Interrupt Controller",
            (0x00, 0x20) => "I/O(x) APIC Interrupt Controller",
            (0x01, 0x00) => "Generic 8237 DMA Controller",
            (0x01, 0x01) => "ISA DMA Controller",
            (0x01, 0x02) => "EISA DMA Controller",
            (0x02, 0x00) => "Generic 8254 System Timer",
            (0x02, 0x01) => "ISA System Timer",
            (0x02, 0x02) => "EISA System Timer",
            (0x03, 0x00) => "Generic RTC Controller",
            (0x03, 0x01) => "ISA RTC Controller",
            (0x04, 0x00) => "Generic PCI Hot-Plug Controller",
            (0x80, 0x00) => "Other System Peripheral",
            (_, _) => "",
        },
        DeviceClass::InputDevice => match (subclass, prog_if) {
            (0x00, 0x00) => "Keyboard Controller",
            (0x01, 0x00) => "Digitizer",
            (0x02, 0x00) => "Mouse Controller",
            (0x03, 0x00) => "Scanner Controller",
            (0x04, 0x00) => "Gameport Controller (Generic)",
            (0x04, 0x10) => "Gameport Contrlller (Legacy)",
            (0x80, 0x00) => "Other Input Controller",
            (_, _) => "",
        },
        DeviceClass::DockingStation => match (subclass, prog_if) {
            (0x00, 0x00) => "Generic Docking Station",
            (0x80, 0x00) => "Other Docking Station",
            (_, _) => "",
        },
        DeviceClass::Processor => match (subclass, prog_if) {
            (0x00, 0x00) => "386 Processor",
            (0x01, 0x00) => "486 Processor",
            (0x02, 0x00) => "Pentium Processor",
            (0x10, 0x00) => "Alpha Processor",
            (0x20, 0x00) => "PowerPC Processor",
            (0x30, 0x00) => "MIPS Processor",
            (0x40, 0x00) => "Co-Processor",
            (_, _) => "",
        },
        DeviceClass::SerialBus => match (subclass, prog_if) {
            (0x00, 0x00) => "IEEE 1394 Controller (FireWire)",
            (0x00, 0x01) => "IEEE 1394 Controller (1394 OpenHCI Spec)",
            (0x01, 0x00) => "ACCESS.bus",
            (0x02, 0x00) => "SSA",
            (0x03, 0x00) => "USB (Universal Host Controller Spec)",
            (0x03, 0x10) => "USB (Open Host Controller Spec)",
            (0x03, 0x20) => "USB2 Host Controller (Intel Enhanced Host Controller Interface)",
            (0x03, 0x30) => "USB3 XHCI Controller",
            (0x03, 0x80) => "Unspecified USB Controller",
            (0x03, 0xFE) => "USB (Not Host Controller)",
            (0x04, 0x00) => "Fibre Channel",
            (0x05, 0x00) => "SMBus",
            (0x06, 0x00) => "InfiniBand",
            (0x07, 0x00) => "IPMI SMIC Interface",
            (0x07, 0x01) => "IPMI Kybd Controller Style Interface",
            (0x07, 0x02) => "IPMI Block Transfer Interface",
            (0x08, 0x00) => "SERCOS Interface Standard (IEC 61491)",
            (0x09, 0x00) => "CANbus",
            (_, _) => "",
        },
        DeviceClass::Wireless => match (subclass, prog_if) {
            (0x00, 0x00) => "iRDA Compatible Controller",
            (0x01, 0x00) => "Consumer IR Controller",
            (0x10, 0x00) => "RF Controller",
            (0x11, 0x00) => "Bluetooth Controller",
            (0x12, 0x00) => "Broadband Controller",
            (0x20, 0x00) => "Ethernet Controller (802.11a)",
            (0x21, 0x00) => "Ethernet Controller (802.11b)",
            (0x80, 0x00) => "Other Wireless Controller",
            (_, _) => "",
        },
        DeviceClass::IntelligentIO => match (subclass, prog_if) {
            (0x00, 0x00) => "I20 Architecture",
            (0x00, _) => "Message FIFO",
            (_, _) => "",
        },
        DeviceClass::SatelliteCommunication => match (subclass, prog_if) {
            (0x01, 0x00) => "TV Controller",
            (0x02, 0x00) => "Audio Controller",
            (0x03, 0x00) => "Voice Controller",
            (0x04, 0x00) => "Data Controller",
            (_, _) => "",
        },
        DeviceClass::EncryptionDecryption => match (subclass, prog_if) {
            (0x00, 0x00) => "Network and Computing Encrpytion/Decryption",
            (0x10, 0x00) => "Entertainment Encryption/Decryption",
            (0x80, 0x00) => "Other Encryption/Decryption",
            (_, _) => "",
        },
        DeviceClass::DataAndSignalProcessing => match (subclass, prog_if) {
            (0x00, 0x00) => "DPIO Modules",
            (0x01, 0x00) => "Performance Counters",
            (0x10, 0x00) => {
                "Communications Syncrhonization Plus Time and Frequency Test/Measurment"
            }
            (0x20, 0x00) => "Management Card",
            (0x80, 0x00) => "Other Data Acquisition/Signal Processing Controller",
            (_, _) => "",
        },
        _ => "Unknown device description",
    }
}
