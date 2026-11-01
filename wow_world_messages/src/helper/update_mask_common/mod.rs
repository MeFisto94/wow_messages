pub(crate) mod inners;

pub const OBJECT: u32 = 0x0001;
pub const ITEM: u32 = 0x0002;
pub const CONTAINER: u32 = 0x0004;
pub const UNIT: u32 = 0x0008;
pub const PLAYER: u32 = 0x0010;
pub const GAMEOBJECT: u32 = 0x0020;
pub const DYNAMICOBJECT: u32 = 0x0040;
pub const CORPSE: u32 = 0x0080;

macro_rules! update_item {
    ($name:ident, $builder_name:ident, $type_value:expr) => {
        #[derive(Debug, Hash, Clone, Default, PartialEq, Eq, PartialOrd)]
        pub struct $builder_name {
            header: Vec<u32>,
            values: std::collections::BTreeMap<u16, u32>,
        }

        impl $builder_name {
            pub fn finalize(self) -> $name {
                $name::from_inners(self.header, self.values)
            }

            #[allow(unused)]
            pub(crate) fn header_set(&mut self, bit: u16, value: u32) {
                $crate::helper::update_mask_common::inners::header_set(
                    &mut self.values,
                    &mut self.header,
                    None,
                    bit,
                    value,
                );
            }

            pub(crate) fn set_guid(&mut self, bit: u16, guid: $crate::Guid) {
                $crate::helper::update_mask_common::inners::set_guid(
                    &mut self.values,
                    &mut self.header,
                    None,
                    bit,
                    guid,
                );
            }

            #[allow(unused)]
            pub(crate) fn set_int(&mut self, bit: u16, v: i32) {
                $crate::helper::update_mask_common::inners::set_int(
                    &mut self.values,
                    &mut self.header,
                    None,
                    bit,
                    v,
                );
            }

            #[allow(unused)]
            pub(crate) fn set_float(&mut self, bit: u16, v: f32) {
                $crate::helper::update_mask_common::inners::set_float(
                    &mut self.values,
                    &mut self.header,
                    None,
                    bit,
                    v,
                );
            }

            #[allow(unused)]
            pub(crate) fn set_bytes(&mut self, bit: u16, a: u8, b: u8, c: u8, d: u8) {
                $crate::helper::update_mask_common::inners::set_bytes(
                    &mut self.values,
                    &mut self.header,
                    None,
                    bit,
                    a,
                    b,
                    c,
                    d,
                );
            }

            #[allow(unused)]
            pub(crate) fn set_shorts(&mut self, bit: u16, a: u16, b: u16) {
                $crate::helper::update_mask_common::inners::set_shorts(
                    &mut self.values,
                    &mut self.header,
                    None,
                    bit,
                    a,
                    b,
                );
            }

            pub fn new() -> Self {
                const OBJECT_FIELD_TYPE: u16 = 2;

                let mut header = vec![];
                let mut values = std::collections::BTreeMap::new();

                $crate::helper::update_mask_common::inners::array_set(
                    &mut header,
                    OBJECT_FIELD_TYPE,
                );
                values.insert(
                    OBJECT_FIELD_TYPE,
                    $crate::helper::update_mask_common::OBJECT | $type_value,
                );

                Self { header, values }
            }
        }

        impl From<$builder_name> for $name {
            fn from(e: $builder_name) -> $name {
                e.finalize()
            }
        }

        #[derive(Debug, Hash, Clone, Default, PartialEq, Eq, PartialOrd)]
        pub struct $name {
            header: Vec<u32>,
            dirty_mask: Vec<u32>,
            values: std::collections::BTreeMap<u16, u32>,
        }

        impl $name {
            pub fn new() -> Self {
                $builder_name::new().finalize()
            }

            pub fn builder() -> $builder_name {
                $builder_name::new()
            }

            fn from_inners(header: Vec<u32>, values: std::collections::BTreeMap<u16, u32>) -> Self {
                Self {
                    header: header.clone(),
                    dirty_mask: header,
                    values,
                }
            }

            pub(crate) fn set_guid(&mut self, bit: u16, guid: $crate::Guid) {
                $crate::helper::update_mask_common::inners::set_guid(
                    &mut self.values,
                    &mut self.header,
                    Some(&mut self.dirty_mask),
                    bit,
                    guid,
                );
            }

            pub(crate) fn get_guid(&self, bit: u16) -> Option<$crate::Guid> {
                $crate::helper::update_mask_common::inners::get_guid(&self.values, bit)
            }

            #[allow(unused)]
            pub(crate) fn set_int(&mut self, bit: u16, v: i32) {
                $crate::helper::update_mask_common::inners::set_int(
                    &mut self.values,
                    &mut self.header,
                    Some(&mut self.dirty_mask),
                    bit,
                    v,
                );
            }

            #[allow(unused)]
            pub(crate) fn get_int(&self, bit: u16) -> Option<i32> {
                $crate::helper::update_mask_common::inners::get_int(&self.values, bit)
            }

            #[allow(unused)]
            pub(crate) fn set_float(&mut self, bit: u16, v: f32) {
                $crate::helper::update_mask_common::inners::set_float(
                    &mut self.values,
                    &mut self.header,
                    Some(&mut self.dirty_mask),
                    bit,
                    v,
                );
            }

            #[allow(unused)]
            pub(crate) fn get_float(&self, bit: u16) -> Option<f32> {
                $crate::helper::update_mask_common::inners::get_float(&self.values, bit)
            }

            #[allow(unused)]
            pub(crate) fn set_bytes(&mut self, bit: u16, a: u8, b: u8, c: u8, d: u8) {
                $crate::helper::update_mask_common::inners::set_bytes(
                    &mut self.values,
                    &mut self.header,
                    Some(&mut self.dirty_mask),
                    bit,
                    a,
                    b,
                    c,
                    d,
                );
            }

            #[allow(unused)]
            pub(crate) fn get_bytes(&self, bit: u16) -> Option<(u8, u8, u8, u8)> {
                $crate::helper::update_mask_common::inners::get_bytes(&self.values, bit)
            }

            #[allow(unused)]
            pub(crate) fn set_shorts(&mut self, bit: u16, a: u16, b: u16) {
                $crate::helper::update_mask_common::inners::set_shorts(
                    &mut self.values,
                    &mut self.header,
                    Some(&mut self.dirty_mask),
                    bit,
                    a,
                    b,
                );
            }

            #[allow(unused)]
            pub(crate) fn get_shorts(&self, bit: u16) -> Option<(u16, u16)> {
                $crate::helper::update_mask_common::inners::get_shorts(&self.values, bit)
            }

            #[allow(unused)]
            pub(crate) fn header_set(&mut self, bit: u16, value: u32) {
                $crate::helper::update_mask_common::inners::header_set(
                    &mut self.values,
                    &mut self.header,
                    Some(&mut self.dirty_mask),
                    bit,
                    value,
                );
            }

            pub fn dirty_reset(&mut self) {
                $crate::helper::update_mask_common::inners::array_reset(&mut self.dirty_mask);
            }

            pub fn mark_fully_dirty(&mut self) {
                $crate::helper::update_mask_common::inners::array_fill_ones(&mut self.dirty_mask);
            }

            pub fn has_any_dirty_fields(&self) -> bool {
                $crate::helper::update_mask_common::inners::has_any_bit_set(&self.dirty_mask)
            }

            pub fn is_bit_dirty(&self, bit: u16) -> bool {
                $crate::helper::update_mask_common::inners::has_array_bit_set(&self.dirty_mask, bit)
            }

            pub(crate) fn write_into_vec(
                &self,
                v: impl std::io::Write,
            ) -> Result<(), std::io::Error> {
                $crate::helper::update_mask_common::inners::write_into_vec(
                    v,
                    &self.header,
                    &self.dirty_mask,
                    &self.values,
                )
            }
        }
    };
}
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) use update_item;

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
macro_rules! update_mask {
    () => {
        #[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd)]
        pub enum UpdateMask {
            Item(UpdateItem),
            Container(UpdateContainer),
            Unit(UpdateUnit),
            Player(UpdatePlayer),
            GameObject(UpdateGameObject),
            DynamicObject(UpdateDynamicObject),
            Corpse(UpdateCorpse),
            /// A values update, which carries only the fields that changed. `OBJECT_FIELD_TYPE` never
            /// changes, so it is not among them, and the kind of object is whatever the receiver
            /// already knows the guid to be.
            Untyped(UpdateUntyped),
        }

        /// The fields of an update that did not say what kind of object it updates.
        ///
        /// Read it as the kind the object was created as, with [`Self::into_unit`] and the rest.
        #[derive(Debug, Hash, Clone, Default, PartialEq, Eq, PartialOrd)]
        pub struct UpdateUntyped {
            header: Vec<u32>,
            values: std::collections::BTreeMap<u16, u32>,
        }

        impl UpdateUntyped {
            pub fn into_item(self) -> UpdateItem {
                UpdateItem::from_inners(self.header, self.values)
            }

            pub fn into_container(self) -> UpdateContainer {
                UpdateContainer::from_inners(self.header, self.values)
            }

            pub fn into_unit(self) -> UpdateUnit {
                UpdateUnit::from_inners(self.header, self.values)
            }

            pub fn into_player(self) -> UpdatePlayer {
                UpdatePlayer::from_inners(self.header, self.values)
            }

            pub fn into_game_object(self) -> UpdateGameObject {
                UpdateGameObject::from_inners(self.header, self.values)
            }

            pub fn into_dynamic_object(self) -> UpdateDynamicObject {
                UpdateDynamicObject::from_inners(self.header, self.values)
            }

            pub fn into_corpse(self) -> UpdateCorpse {
                UpdateCorpse::from_inners(self.header, self.values)
            }
        }

        impl UpdateMask {
            pub(crate) fn read(r: &mut impl std::io::Read) -> Result<Self, std::io::Error> {
                let (header, values) = $crate::helper::update_mask_common::inners::read_inner(r)?;

                let Some(&ty) = values.get(&2) else {
                    return Ok(Self::Untyped(UpdateUntyped { header, values }));
                };

                Ok(
                    if (ty & $crate::helper::update_mask_common::CONTAINER) != 0 {
                        Self::Container(UpdateContainer::from_inners(header, values))
                    } else if (ty & $crate::helper::update_mask_common::ITEM) != 0 {
                        Self::Item(UpdateItem::from_inners(header, values))
                    } else if (ty & $crate::helper::update_mask_common::PLAYER) != 0 {
                        Self::Player(UpdatePlayer::from_inners(header, values))
                    } else if (ty & $crate::helper::update_mask_common::UNIT) != 0 {
                        Self::Unit(UpdateUnit::from_inners(header, values))
                    } else if (ty & $crate::helper::update_mask_common::GAMEOBJECT) != 0 {
                        Self::GameObject(UpdateGameObject::from_inners(header, values))
                    } else if (ty & $crate::helper::update_mask_common::DYNAMICOBJECT) != 0 {
                        Self::DynamicObject(UpdateDynamicObject::from_inners(header, values))
                    } else if (ty & $crate::helper::update_mask_common::CORPSE) != 0 {
                        Self::Corpse(UpdateCorpse::from_inners(header, values))
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Object type not valid",
                        ));
                    },
                )
            }

            pub(crate) fn write_into_vec(
                &self,
                v: impl std::io::Write,
            ) -> Result<(), std::io::Error> {
                match self {
                    UpdateMask::Item(i) => i.write_into_vec(v),
                    UpdateMask::Container(i) => i.write_into_vec(v),
                    UpdateMask::Unit(i) => i.write_into_vec(v),
                    UpdateMask::Player(i) => i.write_into_vec(v),
                    UpdateMask::GameObject(i) => i.write_into_vec(v),
                    UpdateMask::DynamicObject(i) => i.write_into_vec(v),
                    UpdateMask::Corpse(i) => i.write_into_vec(v),
                    // Every field it holds, which is every field it was read with.
                    UpdateMask::Untyped(i) => $crate::helper::update_mask_common::inners::write_into_vec(
                        v, &i.header, &i.header, &i.values,
                    ),
                }
            }

            pub(crate) fn size(&self) -> usize {
                match self {
                    UpdateMask::Item(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                    UpdateMask::Container(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                    UpdateMask::Unit(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                    UpdateMask::Player(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                    UpdateMask::GameObject(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                    UpdateMask::DynamicObject(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                    UpdateMask::Corpse(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                    UpdateMask::Untyped(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.header,
                            &i.header,
                        )
                    }
                }
            }
        }

        impl From<UpdateItem> for UpdateMask {
            fn from(e: UpdateItem) -> UpdateMask {
                Self::Item(e)
            }
        }

        impl From<UpdateContainer> for UpdateMask {
            fn from(e: UpdateContainer) -> UpdateMask {
                Self::Container(e)
            }
        }

        impl From<UpdateUnit> for UpdateMask {
            fn from(e: UpdateUnit) -> UpdateMask {
                Self::Unit(e)
            }
        }

        impl From<UpdatePlayer> for UpdateMask {
            fn from(e: UpdatePlayer) -> UpdateMask {
                Self::Player(e)
            }
        }

        impl From<UpdateGameObject> for UpdateMask {
            fn from(e: UpdateGameObject) -> UpdateMask {
                Self::GameObject(e)
            }
        }

        impl From<UpdateDynamicObject> for UpdateMask {
            fn from(e: UpdateDynamicObject) -> UpdateMask {
                Self::DynamicObject(e)
            }
        }

        impl From<UpdateCorpse> for UpdateMask {
            fn from(e: UpdateCorpse) -> UpdateMask {
                Self::Corpse(e)
            }
        }

        impl Default for UpdateMask {
            fn default() -> Self {
                Self::Item(Default::default())
            }
        }
    };
}
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) use update_mask;
