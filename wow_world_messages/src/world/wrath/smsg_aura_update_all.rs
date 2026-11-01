use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_aura_update_all.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_aura_update_all.wowm#L1):
/// ```text
/// smsg SMSG_AURA_UPDATE_ALL = 0x0495 {
///     PackedGuid unit;
///     u8[-] auras;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_AURA_UPDATE_ALL {
    pub unit: Guid,
    /// One entry per slot, in a layout this language cannot state, so left to the reader.
    /// Each is the slot (`u8`) and the spell (`u32`). A spell of zero is an empty slot and ends the
    /// entry. Otherwise the flags (`u8`), the caster level (`u8`) and the stack count (`u8`) follow,
    /// then the caster as a packed guid only when the flags do not have `0x08` (self cast), then the
    /// full and the remaining duration (`i32` each, in milliseconds) when they have `0x20` (duration).
    /// TrinityCore: `AuraDataInfo` and `AuraInfo` in `SpellPackets.cpp`.
    pub auras: Vec<u8>,
}

impl crate::private::Sealed for SMSG_AURA_UPDATE_ALL {}
impl SMSG_AURA_UPDATE_ALL {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=65544).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unit: PackedGuid
        let unit = crate::util::read_packed_guid(&mut r)?;

        // auras: u8[-]
        let auras = {
            let mut current_size = {
                crate::util::packed_guid_size(&unit) // unit: PackedGuid
            };
            let mut auras = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                auras.push(crate::util::read_u8_le(&mut r)?);
                current_size += 1;
            }
            auras
        };

        Ok(Self {
            unit,
            auras,
        })
    }

}

impl crate::Message for SMSG_AURA_UPDATE_ALL {
    const OPCODE: u32 = 0x0495;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_AURA_UPDATE_ALL"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AURA_UPDATE_ALL {{").unwrap();
        // Members
        writeln!(s, "    unit = {};", self.unit.guid()).unwrap();
        writeln!(s, "    auras = [").unwrap();
        for v in self.auras.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1173_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.unit), "unit", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.auras.len(), "auras", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        crate::util::write_packed_guid(&self.unit, &mut w)?;

        // auras: u8[-]
        for i in self.auras.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1173, "SMSG_AURA_UPDATE_ALL", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AURA_UPDATE_ALL {}

impl SMSG_AURA_UPDATE_ALL {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.unit) // unit: PackedGuid
        + self.auras.len() * core::mem::size_of::<u8>() // auras: u8[-]
    }
}

