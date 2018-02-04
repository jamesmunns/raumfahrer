// Top Level Registers
//   User Manual Sec 7.1
// TODO: Some better mechanism for keeping Coarse:Fine addresses?

#![allow(dead_code)]
#![allow(non_snake_case)]

// User Manual Sec 2.2.1.2.1
pub const READ_MASK: u8 = 0b0000_0000;
pub const WRITE_MASK: u8 = 0b1000_0000;
pub const NO_SUB_INDEX_MASK: u8 = 0b0000_0000;
pub const SUB_INDEX_MASK: u8 = 0b0100_0000;

pub mod DEV_ID {
    pub const BASE: u8 = 0x00;
    pub const DEV_ID: u8 = 0x00;
} // 4 octets

pub mod EUI {
    pub const BASE: u8 = 0x01;
} // 8 octets

pub mod PANADR {
    pub const BASE: u8 = 0x03;
} // 4 octets

pub mod SYS_CFG {
    pub const BASE: u8 = 0x04;
} // 4 octets

pub mod SYS_TIME {
    pub const BASE: u8 = 0x06;
} // 5 octets

pub mod TX_FCTRL {
    pub const BASE: u8 = 0x08;
} // 5 octets

pub mod TX_BUFFER {
    pub const BASE: u8 = 0x09;
} // 1024 octets

pub mod DX_TIME {
    pub const BASE: u8 = 0x0A;
} // 5 octets

pub mod RX_FWTO {
    pub const BASE: u8 = 0x0C;
} // 2 octets

pub mod SYS_CTRL {
    pub const BASE: u8 = 0x0D;
} // 4 octets

pub mod SYS_MASK {
    pub const BASE: u8 = 0x0E;
} // 4 octets

pub mod SYS_STATUS {
    pub const BASE: u8 = 0x0F;
}

pub mod RX_FINFO {
    pub const BASE: u8 = 0x10;
}

pub mod RX_BUFFER {
    pub const BASE: u8 = 0x11;
}

pub mod RX_FQUAL {
    pub const BASE: u8 = 0x12;
}

pub mod RX_TTCKI {
    pub const BASE: u8 = 0x13;
}

pub mod RX_TTCKO {
    pub const BASE: u8 = 0x14;
}

pub mod RX_TIME {
    pub const BASE: u8 = 0x15;
}

pub mod TX_TIME {
    pub const BASE: u8 = 0x17;
}

pub mod TX_ANTD {
    pub const BASE: u8 = 0x18;
}

pub mod SYS_STATE {
    pub const BASE: u8 = 0x19;
}

pub mod ACK_RESP_T {
    pub const BASE: u8 = 0x1A;
}

pub mod RX_SNIFF {
    pub const BASE: u8 = 0x1D;
}

pub mod TX_POWER {
    pub const BASE: u8 = 0x1E;
}

pub mod CHAN_CTRL {
    pub const BASE: u8 = 0x1F;
}

pub mod USR_SFD {
    pub const BASE: u8 = 0x21;
}

pub mod AGC_CTRL {
    pub const BASE: u8 = 0x23;
}

pub mod EXT_SYNC {
    pub const BASE: u8 = 0x24;
}

pub mod ACC_MEM {
    pub const BASE: u8 = 0x25;
}

pub mod GPIO_CTRL {
    pub const BASE: u8 = 0x26;
}

pub mod DRX_CONF {
    pub const BASE: u8 = 0x27;
}

pub mod RF_CONF {
    pub const BASE: u8 = 0x28;
}

pub mod TX_CAL {
    pub const BASE: u8 = 0x2A;
}

pub mod FS_CTRL {
    pub const BASE: u8 = 0x2B;
}

pub mod AON {
    pub const BASE: u8 = 0x2C;
}

pub mod OTP_IF {
    pub const BASE: u8 = 0x2D;
}

pub mod LDE_CTRL {
    pub const BASE: u8 = 0x2E;
}

pub mod DIG_DIAG {
    pub const BASE: u8 = 0x2F;
}

pub mod PMSC {
    pub const BASE: u8 = 0x36;
}
