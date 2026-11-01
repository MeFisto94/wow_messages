use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm#L9):
/// ```text
/// struct FactionStanding {
///     u32 reputation_list_id;
///     i32 standing;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct FactionStanding {
    /// The faction's place in the reputation list, which is `Faction.dbc`'s reputation index rather than its id. TrinityCore: `ReputationListID`.
    pub reputation_list_id: u32,
    /// What was earned on top of where the character started. TrinityCore: `FactionState::Standing`.
    pub standing: i32,
}

impl FactionStanding {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // standing: i32
        w.write_all(&self.standing.to_le_bytes())?;

        Ok(())
    }
}

impl FactionStanding {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(&mut r)?;

        // standing: i32
        let standing = crate::util::read_i32_le(&mut r)?;

        Ok(Self {
            reputation_list_id,
            standing,
        })
    }

}

