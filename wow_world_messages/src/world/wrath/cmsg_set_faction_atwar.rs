use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm:50`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm#L50):
/// ```text
/// cmsg CMSG_SET_FACTION_ATWAR = 0x0125 {
///     u32 reputation_list_id;
///     Bool at_war;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_SET_FACTION_ATWAR {
    /// The faction's place in the reputation list rather than its id. TrinityCore: `uint32 repListID`.
    pub reputation_list_id: u32,
    /// TrinityCore: any value but zero declares war.
    pub at_war: bool,
}

impl crate::private::Sealed for CMSG_SET_FACTION_ATWAR {}
impl CMSG_SET_FACTION_ATWAR {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 5 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(&mut r)?;

        // at_war: Bool
        let at_war = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            reputation_list_id,
            at_war,
        })
    }

}

impl crate::Message for CMSG_SET_FACTION_ATWAR {
    const OPCODE: u32 = 0x0125;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_SET_FACTION_ATWAR"
    }

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // at_war: Bool
        w.write_all(u8::from(self.at_war).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(293, "CMSG_SET_FACTION_ATWAR", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_FACTION_ATWAR {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_SET_FACTION_ATWAR;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 11] = [ 0x00, 0x09, 0x25, 0x01, 0x00, 0x00, 0x13, 0x00, 0x00,
         0x00, 0x01, ];

    pub(crate) fn expected0() -> CMSG_SET_FACTION_ATWAR {
        CMSG_SET_FACTION_ATWAR {
            reputation_list_id: 0x13,
            at_war: true, 
        }

    }

    // Generated from `wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm` line 59.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_set_faction_atwar0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SET_FACTION_ATWAR(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SET_FACTION_ATWAR, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(5 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm` line 59.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_set_faction_atwar0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SET_FACTION_ATWAR(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SET_FACTION_ATWAR, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(5 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm` line 59.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_set_faction_atwar0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SET_FACTION_ATWAR(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SET_FACTION_ATWAR, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(5 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

