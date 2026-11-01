use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/cmsg_set_watched_faction.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/cmsg_set_watched_faction.wowm#L13):
/// ```text
/// cmsg CMSG_SET_WATCHED_FACTION = 0x0318 {
///     u32 reputation_list_id;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_SET_WATCHED_FACTION {
    /// The faction's place in the reputation list, and `0xFFFFFFFF` for none. TrinityCore stores it as
    /// `PLAYER_FIELD_WATCHED_FACTION_INDEX`.
    pub reputation_list_id: u32,
}

impl crate::private::Sealed for CMSG_SET_WATCHED_FACTION {}
impl CMSG_SET_WATCHED_FACTION {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            reputation_list_id,
        })
    }

}

impl crate::Message for CMSG_SET_WATCHED_FACTION {
    const OPCODE: u32 = 0x0318;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_SET_WATCHED_FACTION"
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(792, "CMSG_SET_WATCHED_FACTION", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_WATCHED_FACTION {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_SET_WATCHED_FACTION;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 10] = [ 0x00, 0x08, 0x18, 0x03, 0x00, 0x00, 0x13, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> CMSG_SET_WATCHED_FACTION {
        CMSG_SET_WATCHED_FACTION {
            reputation_list_id: 0x13,
        }

    }

    // Generated from `wow_message_parser/wowm/world/faction/cmsg_set_watched_faction.wowm` line 21.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_set_watched_faction0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SET_WATCHED_FACTION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SET_WATCHED_FACTION, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/faction/cmsg_set_watched_faction.wowm` line 21.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_set_watched_faction0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SET_WATCHED_FACTION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SET_WATCHED_FACTION, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/faction/cmsg_set_watched_faction.wowm` line 21.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_set_watched_faction0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SET_WATCHED_FACTION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SET_WATCHED_FACTION, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

