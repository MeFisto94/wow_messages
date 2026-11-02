use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    MonsterMoveType, Vector3d,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm#L31):
/// ```text
/// smsg SMSG_MONSTER_MOVE = 0x00DD {
///     PackedGuid guid;
///     u8 unknown;
///     Vector3d spline_point;
///     u32 spline_id;
///     MonsterMoveType move_type;
///     u8[-] spline;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_MONSTER_MOVE {
    pub guid: Guid,
    /// cmangos-wotlk sets to 0
    pub unknown: u8,
    pub spline_point: Vector3d,
    pub spline_id: u32,
    pub move_type: MonsterMoveType,
    /// The rest, in a layout this language cannot state, so left to the reader. TrinityCore:
    /// `MovementMonsterSpline`, written by `MonsterMove::InitializeSplineData`. Nothing for `STOP`.
    /// Otherwise how the unit faces: a `Guid` for `FACING_TARGET`, an `f32` angle for `FACING_ANGLE`
    /// and a `Vector3d` for `FACING_SPOT`. Then the `u32` `SplineFlag`, a `u8` animation tier and a
    /// `u32` start time if they have `ANIMATION`, the `u32` duration in milliseconds, and an `f32`
    /// vertical acceleration and a `u32` start time if they have `PARABOLIC`. Then the path after
    /// `spline_point`: a `u32` count, and for a smooth path (`FLYING` or `CATMULLROM`) that many points
    /// as `Vector3d`. Otherwise the end as a `Vector3d` and each point between as its offset from the
    /// middle of start and end, packed into a `u32` as signed 11, 11 and 10 bits in quarter units.
    pub spline: Vec<u8>,
}

impl crate::private::Sealed for SMSG_MONSTER_MOVE {}
impl SMSG_MONSTER_MOVE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(19..=65562).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        // spline_point: Vector3d
        let spline_point = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

        // spline_id: u32
        let spline_id = crate::util::read_u32_le(&mut r)?;

        // move_type: MonsterMoveType
        let move_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // spline: u8[-]
        let spline = {
            let mut current_size = {
                crate::util::packed_guid_size(&guid) // guid: PackedGuid
                + 1 // unknown: u8
                + 12 // spline_point: Vector3d
                + 4 // spline_id: u32
                + 1 // move_type: MonsterMoveType
            };
            let mut spline = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                spline.push(crate::util::read_u8_le(&mut r)?);
                current_size += 1;
            }
            spline
        };

        Ok(Self {
            guid,
            unknown,
            spline_point,
            spline_id,
            move_type,
            spline,
        })
    }

}

impl crate::Message for SMSG_MONSTER_MOVE {
    const OPCODE: u32 = 0x00dd;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_MONSTER_MOVE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MONSTER_MOVE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();
        // spline_point: Vector3d
        writeln!(s, "    spline_point = {{").unwrap();
        // Members
        writeln!(s, "        x = {};", if self.spline_point.x.to_string().contains('.') { self.spline_point.x.to_string() } else { format!("{}.0", self.spline_point.x) }).unwrap();
        writeln!(s, "        y = {};", if self.spline_point.y.to_string().contains('.') { self.spline_point.y.to_string() } else { format!("{}.0", self.spline_point.y) }).unwrap();
        writeln!(s, "        z = {};", if self.spline_point.z.to_string().contains('.') { self.spline_point.z.to_string() } else { format!("{}.0", self.spline_point.z) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    spline_id = {};", self.spline_id).unwrap();
        writeln!(s, "    move_type = {};", self.move_type.as_test_case_value()).unwrap();
        writeln!(s, "    spline = [").unwrap();
        for v in self.spline.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 221_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.guid), "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown", "    ");
        writeln!(s, "    /* spline_point: Vector3d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "        ");
        writeln!(s, "    /* spline_point: Vector3d end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "move_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.spline.len(), "spline", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        // spline_point: Vector3d
        crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&self.spline_point, &mut w)?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&(self.move_type.as_int().to_le_bytes()))?;

        // spline: u8[-]
        for i in self.spline.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(221, "SMSG_MONSTER_MOVE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MONSTER_MOVE {}

impl SMSG_MONSTER_MOVE {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 1 // unknown: u8
        + 12 // spline_point: Vector3d
        + 4 // spline_id: u32
        + 1 // move_type: MonsterMoveType
        + self.spline.len() * core::mem::size_of::<u8>() // spline: u8[-]
    }
}

