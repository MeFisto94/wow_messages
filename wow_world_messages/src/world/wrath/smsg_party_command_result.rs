use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_command_result.wowm:59`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_command_result.wowm#L59):
/// ```text
/// smsg SMSG_PARTY_COMMAND_RESULT = 0x007F {
///     u32 operation;
///     CString member;
///     u32 result;
///     u32 lfd_cooldown;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_COMMAND_RESULT {
    /// TrinityCore's `PartyOperation`, which has an uninvite (1) and a swap (4) as well.
    pub operation: u32,
    pub member: String,
    /// TrinityCore's `PartyResult`, which goes on past 13.
    pub result: u32,
    /// TrinityCore: "LFD cooldown related".
    pub lfd_cooldown: u32,
}

impl crate::private::Sealed for SMSG_PARTY_COMMAND_RESULT {}
impl SMSG_PARTY_COMMAND_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(13..=268).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // operation: u32
        let operation = crate::util::read_u32_le(&mut r)?;

        // member: CString
        let member = {
            let member = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(member)?
        };

        // result: u32
        let result = crate::util::read_u32_le(&mut r)?;

        // lfd_cooldown: u32
        let lfd_cooldown = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            operation,
            member,
            result,
            lfd_cooldown,
        })
    }

}

impl crate::Message for SMSG_PARTY_COMMAND_RESULT {
    const OPCODE: u32 = 0x007f;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_PARTY_COMMAND_RESULT"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // operation: u32
        w.write_all(&self.operation.to_le_bytes())?;

        // member: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.member.as_bytes().iter().next_back(), Some(&0_u8), "String `member` must not be null-terminated.");
        w.write_all(self.member.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: u32
        w.write_all(&self.result.to_le_bytes())?;

        // lfd_cooldown: u32
        w.write_all(&self.lfd_cooldown.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(127, "SMSG_PARTY_COMMAND_RESULT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PARTY_COMMAND_RESULT {}

impl SMSG_PARTY_COMMAND_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // operation: u32
        + self.member.len() + 1 // member: CString
        + 4 // result: u32
        + 4 // lfd_cooldown: u32
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_PARTY_COMMAND_RESULT;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 23] = [ 0x00, 0x15, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54,
         0x68, 0x72, 0x61, 0x6C, 0x6C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_PARTY_COMMAND_RESULT {
        SMSG_PARTY_COMMAND_RESULT {
            operation: 0x0,
            member: String::from("Thrall"),
            result: 0x0,
            lfd_cooldown: 0x0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_party_command_result.wowm` line 73.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_party_command_result0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PARTY_COMMAND_RESULT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PARTY_COMMAND_RESULT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_party_command_result.wowm` line 73.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_party_command_result0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PARTY_COMMAND_RESULT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PARTY_COMMAND_RESULT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_party_command_result.wowm` line 73.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_party_command_result0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PARTY_COMMAND_RESULT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PARTY_COMMAND_RESULT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

