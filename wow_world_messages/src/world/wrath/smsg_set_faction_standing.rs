use std::io::{Read, Write};

use crate::wrath::FactionStanding;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm:34`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm#L34):
/// ```text
/// smsg SMSG_SET_FACTION_STANDING = 0x0124 {
///     f32 refer_a_friend_bonus;
///     Bool any_rank_increased;
///     u32 amount_of_faction_standings;
///     FactionStanding[amount_of_faction_standings] faction_standings;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_SET_FACTION_STANDING {
    /// All emus set to 0.
    pub refer_a_friend_bonus: f32,
    /// mangostwo: display visual effect
    pub any_rank_increased: bool,
    pub faction_standings: Vec<FactionStanding>,
}

impl crate::private::Sealed for SMSG_SET_FACTION_STANDING {}
impl SMSG_SET_FACTION_STANDING {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // refer_a_friend_bonus: f32
        let refer_a_friend_bonus = crate::util::read_f32_le(&mut r)?;

        // any_rank_increased: Bool
        let any_rank_increased = crate::util::read_bool_u8(&mut r)?;

        // amount_of_faction_standings: u32
        let amount_of_faction_standings = crate::util::read_u32_le(&mut r)?;

        // faction_standings: FactionStanding[amount_of_faction_standings]
        let faction_standings = {
            let mut faction_standings = Vec::with_capacity(amount_of_faction_standings as usize);

            let allocation_size = u64::from(amount_of_faction_standings) * 8;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_faction_standings {
                faction_standings.push(FactionStanding::read(&mut r)?);
            }
            faction_standings
        };

        Ok(Self {
            refer_a_friend_bonus,
            any_rank_increased,
            faction_standings,
        })
    }

}

impl crate::Message for SMSG_SET_FACTION_STANDING {
    const OPCODE: u32 = 0x0124;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SET_FACTION_STANDING"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // refer_a_friend_bonus: f32
        w.write_all(&self.refer_a_friend_bonus.to_le_bytes())?;

        // any_rank_increased: Bool
        w.write_all(u8::from(self.any_rank_increased).to_le_bytes().as_slice())?;

        // amount_of_faction_standings: u32
        w.write_all(&(self.faction_standings.len() as u32).to_le_bytes())?;

        // faction_standings: FactionStanding[amount_of_faction_standings]
        for i in self.faction_standings.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(292, "SMSG_SET_FACTION_STANDING", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SET_FACTION_STANDING {}

impl SMSG_SET_FACTION_STANDING {
    pub(crate) fn size(&self) -> usize {
        4 // refer_a_friend_bonus: f32
        + 1 // any_rank_increased: Bool
        + 4 // amount_of_faction_standings: u32
        + self.faction_standings.len() * 8 // faction_standings: FactionStanding[amount_of_faction_standings]
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_SET_FACTION_STANDING;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 21] = [ 0x00, 0x13, 0x24, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x01, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0xFA, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_SET_FACTION_STANDING {
        SMSG_SET_FACTION_STANDING {
            refer_a_friend_bonus: 0_f32,
            any_rank_increased: false, 
            faction_standings: vec![
                FactionStanding {
                    reputation_list_id: 0xC,
                    standing: 0xFA,
                },
            ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm` line 45.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_set_faction_standing0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SET_FACTION_STANDING(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SET_FACTION_STANDING, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm` line 45.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_set_faction_standing0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SET_FACTION_STANDING(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SET_FACTION_STANDING, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm` line 45.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_set_faction_standing0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SET_FACTION_STANDING(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SET_FACTION_STANDING, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

