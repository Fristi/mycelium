#[derive(Debug, Default, PartialEq, Clone)]
pub struct Interval {
    pub r#start: i32,
    pub r#end: i32,
}
impl Interval {
    /// Return a reference to `start`
    #[inline]
    pub fn r#start(&self) -> &i32 {
        &self.r#start
    }
    /// Return a mutable reference to `start`
    #[inline]
    pub fn mut_start(&mut self) -> &mut i32 {
        &mut self.r#start
    }
    /// Set the value of `start`
    #[inline]
    pub fn set_start(&mut self, value: i32) -> &mut Self {
        self.r#start = value.into();
        self
    }
    /// Builder method that sets the value of `start`. Useful for initializing the message.
    #[inline]
    pub fn init_start(mut self, value: i32) -> Self {
        self.r#start = value.into();
        self
    }
    /// Return a reference to `end`
    #[inline]
    pub fn r#end(&self) -> &i32 {
        &self.r#end
    }
    /// Return a mutable reference to `end`
    #[inline]
    pub fn mut_end(&mut self) -> &mut i32 {
        &mut self.r#end
    }
    /// Set the value of `end`
    #[inline]
    pub fn set_end(&mut self, value: i32) -> &mut Self {
        self.r#end = value.into();
        self
    }
    /// Builder method that sets the value of `end`. Useful for initializing the message.
    #[inline]
    pub fn init_end(mut self, value: i32) -> Self {
        self.r#end = value.into();
        self
    }
}
impl ::micropb::MessageDecode for Interval {
    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
        &mut self,
        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
        len: usize,
    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
        let before = decoder.bytes_read();
        while decoder.bytes_read() - before < len {
            let tag = decoder.decode_tag()?;
            match tag.field_num() {
                0 => return Err(::micropb::DecodeError::ZeroField),
                1u32 => {
                    let mut_ref = &mut self.r#start;
                    {
                        let val = decoder.decode_int32()?;
                        let val_ref = &val;
                        if *val_ref != 0 {
                            *mut_ref = val as _;
                        }
                    };
                }
                2u32 => {
                    let mut_ref = &mut self.r#end;
                    {
                        let val = decoder.decode_int32()?;
                        let val_ref = &val;
                        if *val_ref != 0 {
                            *mut_ref = val as _;
                        }
                    };
                }
                _ => {
                    decoder.skip_wire_value(tag.wire_type())?;
                }
            }
        }
        Ok(())
    }
}
impl ::micropb::MessageEncode for Interval {
    const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
        let mut max_size = 0;
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option::Some(10usize), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option::Some(10usize), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        ::core::option::Option::Some(max_size)
    };
    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
        &self,
        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
        use ::micropb::{PbMap, FieldEncode};
        {
            let val_ref = &self.r#start;
            if *val_ref != 0 {
                encoder.encode_varint32(8u32)?;
                encoder.encode_int32(*val_ref as _)?;
            }
        }
        {
            let val_ref = &self.r#end;
            if *val_ref != 0 {
                encoder.encode_varint32(16u32)?;
                encoder.encode_int32(*val_ref as _)?;
            }
        }
        Ok(())
    }
    fn compute_size(&self) -> usize {
        use ::micropb::{PbMap, FieldEncode};
        let mut size = 0;
        {
            let val_ref = &self.r#start;
            if *val_ref != 0 {
                size += 1usize + ::micropb::size::sizeof_int32(*val_ref as _);
            }
        }
        {
            let val_ref = &self.r#end;
            if *val_ref != 0 {
                size += 1usize + ::micropb::size::sizeof_int32(*val_ref as _);
            }
        }
        size
    }
}
#[derive(Debug, Default, Clone)]
pub struct PlantProfile {
    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
    pub r#light_mmol: Interval,
    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
    pub r#light_lux: Interval,
    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
    pub r#temperature: Interval,
    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
    pub r#humidity: Interval,
    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
    pub r#soil_moisture: Interval,
    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
    pub r#soil_ec: Interval,
    /// Tracks presence of optional and message fields
    pub _has: PlantProfile_::_Hazzer,
}
impl ::core::cmp::PartialEq for PlantProfile {
    fn eq(&self, other: &Self) -> bool {
        let mut ret = true;
        ret &= (self.r#light_mmol() == other.r#light_mmol());
        ret &= (self.r#light_lux() == other.r#light_lux());
        ret &= (self.r#temperature() == other.r#temperature());
        ret &= (self.r#humidity() == other.r#humidity());
        ret &= (self.r#soil_moisture() == other.r#soil_moisture());
        ret &= (self.r#soil_ec() == other.r#soil_ec());
        ret
    }
}
impl PlantProfile {
    /// Return a reference to `light_mmol` as an `Option`
    #[inline]
    pub fn r#light_mmol(&self) -> ::core::option::Option<&Interval> {
        self._has.r#light_mmol().then_some(&self.r#light_mmol)
    }
    /// Set the value and presence of `light_mmol`
    #[inline]
    pub fn set_light_mmol(&mut self, value: Interval) -> &mut Self {
        self._has.set_light_mmol();
        self.r#light_mmol = value.into();
        self
    }
    /// Return a mutable reference to `light_mmol` as an `Option`
    #[inline]
    pub fn mut_light_mmol(&mut self) -> ::core::option::Option<&mut Interval> {
        self._has.r#light_mmol().then_some(&mut self.r#light_mmol)
    }
    /// Clear the presence of `light_mmol`
    #[inline]
    pub fn clear_light_mmol(&mut self) -> &mut Self {
        self._has.clear_light_mmol();
        self
    }
    /// Take the value of `light_mmol` and clear its presence
    #[inline]
    pub fn take_light_mmol(&mut self) -> ::core::option::Option<Interval> {
        let val = self
            ._has
            .r#light_mmol()
            .then(|| ::core::mem::take(&mut self.r#light_mmol));
        self._has.clear_light_mmol();
        val
    }
    /// Builder method that sets the value of `light_mmol`. Useful for initializing the message.
    #[inline]
    pub fn init_light_mmol(mut self, value: Interval) -> Self {
        self.set_light_mmol(value);
        self
    }
    /// Return a reference to `light_lux` as an `Option`
    #[inline]
    pub fn r#light_lux(&self) -> ::core::option::Option<&Interval> {
        self._has.r#light_lux().then_some(&self.r#light_lux)
    }
    /// Set the value and presence of `light_lux`
    #[inline]
    pub fn set_light_lux(&mut self, value: Interval) -> &mut Self {
        self._has.set_light_lux();
        self.r#light_lux = value.into();
        self
    }
    /// Return a mutable reference to `light_lux` as an `Option`
    #[inline]
    pub fn mut_light_lux(&mut self) -> ::core::option::Option<&mut Interval> {
        self._has.r#light_lux().then_some(&mut self.r#light_lux)
    }
    /// Clear the presence of `light_lux`
    #[inline]
    pub fn clear_light_lux(&mut self) -> &mut Self {
        self._has.clear_light_lux();
        self
    }
    /// Take the value of `light_lux` and clear its presence
    #[inline]
    pub fn take_light_lux(&mut self) -> ::core::option::Option<Interval> {
        let val = self
            ._has
            .r#light_lux()
            .then(|| ::core::mem::take(&mut self.r#light_lux));
        self._has.clear_light_lux();
        val
    }
    /// Builder method that sets the value of `light_lux`. Useful for initializing the message.
    #[inline]
    pub fn init_light_lux(mut self, value: Interval) -> Self {
        self.set_light_lux(value);
        self
    }
    /// Return a reference to `temperature` as an `Option`
    #[inline]
    pub fn r#temperature(&self) -> ::core::option::Option<&Interval> {
        self._has.r#temperature().then_some(&self.r#temperature)
    }
    /// Set the value and presence of `temperature`
    #[inline]
    pub fn set_temperature(&mut self, value: Interval) -> &mut Self {
        self._has.set_temperature();
        self.r#temperature = value.into();
        self
    }
    /// Return a mutable reference to `temperature` as an `Option`
    #[inline]
    pub fn mut_temperature(&mut self) -> ::core::option::Option<&mut Interval> {
        self._has.r#temperature().then_some(&mut self.r#temperature)
    }
    /// Clear the presence of `temperature`
    #[inline]
    pub fn clear_temperature(&mut self) -> &mut Self {
        self._has.clear_temperature();
        self
    }
    /// Take the value of `temperature` and clear its presence
    #[inline]
    pub fn take_temperature(&mut self) -> ::core::option::Option<Interval> {
        let val = self
            ._has
            .r#temperature()
            .then(|| ::core::mem::take(&mut self.r#temperature));
        self._has.clear_temperature();
        val
    }
    /// Builder method that sets the value of `temperature`. Useful for initializing the message.
    #[inline]
    pub fn init_temperature(mut self, value: Interval) -> Self {
        self.set_temperature(value);
        self
    }
    /// Return a reference to `humidity` as an `Option`
    #[inline]
    pub fn r#humidity(&self) -> ::core::option::Option<&Interval> {
        self._has.r#humidity().then_some(&self.r#humidity)
    }
    /// Set the value and presence of `humidity`
    #[inline]
    pub fn set_humidity(&mut self, value: Interval) -> &mut Self {
        self._has.set_humidity();
        self.r#humidity = value.into();
        self
    }
    /// Return a mutable reference to `humidity` as an `Option`
    #[inline]
    pub fn mut_humidity(&mut self) -> ::core::option::Option<&mut Interval> {
        self._has.r#humidity().then_some(&mut self.r#humidity)
    }
    /// Clear the presence of `humidity`
    #[inline]
    pub fn clear_humidity(&mut self) -> &mut Self {
        self._has.clear_humidity();
        self
    }
    /// Take the value of `humidity` and clear its presence
    #[inline]
    pub fn take_humidity(&mut self) -> ::core::option::Option<Interval> {
        let val = self
            ._has
            .r#humidity()
            .then(|| ::core::mem::take(&mut self.r#humidity));
        self._has.clear_humidity();
        val
    }
    /// Builder method that sets the value of `humidity`. Useful for initializing the message.
    #[inline]
    pub fn init_humidity(mut self, value: Interval) -> Self {
        self.set_humidity(value);
        self
    }
    /// Return a reference to `soil_moisture` as an `Option`
    #[inline]
    pub fn r#soil_moisture(&self) -> ::core::option::Option<&Interval> {
        self._has.r#soil_moisture().then_some(&self.r#soil_moisture)
    }
    /// Set the value and presence of `soil_moisture`
    #[inline]
    pub fn set_soil_moisture(&mut self, value: Interval) -> &mut Self {
        self._has.set_soil_moisture();
        self.r#soil_moisture = value.into();
        self
    }
    /// Return a mutable reference to `soil_moisture` as an `Option`
    #[inline]
    pub fn mut_soil_moisture(&mut self) -> ::core::option::Option<&mut Interval> {
        self._has.r#soil_moisture().then_some(&mut self.r#soil_moisture)
    }
    /// Clear the presence of `soil_moisture`
    #[inline]
    pub fn clear_soil_moisture(&mut self) -> &mut Self {
        self._has.clear_soil_moisture();
        self
    }
    /// Take the value of `soil_moisture` and clear its presence
    #[inline]
    pub fn take_soil_moisture(&mut self) -> ::core::option::Option<Interval> {
        let val = self
            ._has
            .r#soil_moisture()
            .then(|| ::core::mem::take(&mut self.r#soil_moisture));
        self._has.clear_soil_moisture();
        val
    }
    /// Builder method that sets the value of `soil_moisture`. Useful for initializing the message.
    #[inline]
    pub fn init_soil_moisture(mut self, value: Interval) -> Self {
        self.set_soil_moisture(value);
        self
    }
    /// Return a reference to `soil_ec` as an `Option`
    #[inline]
    pub fn r#soil_ec(&self) -> ::core::option::Option<&Interval> {
        self._has.r#soil_ec().then_some(&self.r#soil_ec)
    }
    /// Set the value and presence of `soil_ec`
    #[inline]
    pub fn set_soil_ec(&mut self, value: Interval) -> &mut Self {
        self._has.set_soil_ec();
        self.r#soil_ec = value.into();
        self
    }
    /// Return a mutable reference to `soil_ec` as an `Option`
    #[inline]
    pub fn mut_soil_ec(&mut self) -> ::core::option::Option<&mut Interval> {
        self._has.r#soil_ec().then_some(&mut self.r#soil_ec)
    }
    /// Clear the presence of `soil_ec`
    #[inline]
    pub fn clear_soil_ec(&mut self) -> &mut Self {
        self._has.clear_soil_ec();
        self
    }
    /// Take the value of `soil_ec` and clear its presence
    #[inline]
    pub fn take_soil_ec(&mut self) -> ::core::option::Option<Interval> {
        let val = self._has.r#soil_ec().then(|| ::core::mem::take(&mut self.r#soil_ec));
        self._has.clear_soil_ec();
        val
    }
    /// Builder method that sets the value of `soil_ec`. Useful for initializing the message.
    #[inline]
    pub fn init_soil_ec(mut self, value: Interval) -> Self {
        self.set_soil_ec(value);
        self
    }
}
impl ::micropb::MessageDecode for PlantProfile {
    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
        &mut self,
        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
        len: usize,
    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
        let before = decoder.bytes_read();
        while decoder.bytes_read() - before < len {
            let tag = decoder.decode_tag()?;
            match tag.field_num() {
                0 => return Err(::micropb::DecodeError::ZeroField),
                1u32 => {
                    let mut_ref = &mut self.r#light_mmol;
                    {
                        mut_ref.decode_len_delimited(decoder)?;
                    };
                    self._has.set_light_mmol();
                }
                2u32 => {
                    let mut_ref = &mut self.r#light_lux;
                    {
                        mut_ref.decode_len_delimited(decoder)?;
                    };
                    self._has.set_light_lux();
                }
                3u32 => {
                    let mut_ref = &mut self.r#temperature;
                    {
                        mut_ref.decode_len_delimited(decoder)?;
                    };
                    self._has.set_temperature();
                }
                4u32 => {
                    let mut_ref = &mut self.r#humidity;
                    {
                        mut_ref.decode_len_delimited(decoder)?;
                    };
                    self._has.set_humidity();
                }
                5u32 => {
                    let mut_ref = &mut self.r#soil_moisture;
                    {
                        mut_ref.decode_len_delimited(decoder)?;
                    };
                    self._has.set_soil_moisture();
                }
                6u32 => {
                    let mut_ref = &mut self.r#soil_ec;
                    {
                        mut_ref.decode_len_delimited(decoder)?;
                    };
                    self._has.set_soil_ec();
                }
                _ => {
                    decoder.skip_wire_value(tag.wire_type())?;
                }
            }
        }
        Ok(())
    }
}
impl ::micropb::MessageEncode for PlantProfile {
    const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
        let mut max_size = 0;
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::micropb::const_map!(< Interval as ::micropb::MessageEncode > ::MAX_SIZE, |
            size | ::micropb::size::sizeof_len_record(size)), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::micropb::const_map!(< Interval as ::micropb::MessageEncode > ::MAX_SIZE, |
            size | ::micropb::size::sizeof_len_record(size)), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::micropb::const_map!(< Interval as ::micropb::MessageEncode > ::MAX_SIZE, |
            size | ::micropb::size::sizeof_len_record(size)), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::micropb::const_map!(< Interval as ::micropb::MessageEncode > ::MAX_SIZE, |
            size | ::micropb::size::sizeof_len_record(size)), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::micropb::const_map!(< Interval as ::micropb::MessageEncode > ::MAX_SIZE, |
            size | ::micropb::size::sizeof_len_record(size)), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::micropb::const_map!(< Interval as ::micropb::MessageEncode > ::MAX_SIZE, |
            size | ::micropb::size::sizeof_len_record(size)), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        ::core::option::Option::Some(max_size)
    };
    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
        &self,
        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
        use ::micropb::{PbMap, FieldEncode};
        {
            if let ::core::option::Option::Some(val_ref) = self.r#light_mmol() {
                encoder.encode_varint32(10u32)?;
                val_ref.encode_len_delimited(encoder)?;
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#light_lux() {
                encoder.encode_varint32(18u32)?;
                val_ref.encode_len_delimited(encoder)?;
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#temperature() {
                encoder.encode_varint32(26u32)?;
                val_ref.encode_len_delimited(encoder)?;
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#humidity() {
                encoder.encode_varint32(34u32)?;
                val_ref.encode_len_delimited(encoder)?;
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#soil_moisture() {
                encoder.encode_varint32(42u32)?;
                val_ref.encode_len_delimited(encoder)?;
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#soil_ec() {
                encoder.encode_varint32(50u32)?;
                val_ref.encode_len_delimited(encoder)?;
            }
        }
        Ok(())
    }
    fn compute_size(&self) -> usize {
        use ::micropb::{PbMap, FieldEncode};
        let mut size = 0;
        {
            if let ::core::option::Option::Some(val_ref) = self.r#light_mmol() {
                size
                    += 1usize
                        + ::micropb::size::sizeof_len_record(val_ref.compute_size());
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#light_lux() {
                size
                    += 1usize
                        + ::micropb::size::sizeof_len_record(val_ref.compute_size());
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#temperature() {
                size
                    += 1usize
                        + ::micropb::size::sizeof_len_record(val_ref.compute_size());
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#humidity() {
                size
                    += 1usize
                        + ::micropb::size::sizeof_len_record(val_ref.compute_size());
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#soil_moisture() {
                size
                    += 1usize
                        + ::micropb::size::sizeof_len_record(val_ref.compute_size());
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#soil_ec() {
                size
                    += 1usize
                        + ::micropb::size::sizeof_len_record(val_ref.compute_size());
            }
        }
        size
    }
}
/// Inner types for `PlantProfile`
pub mod PlantProfile_ {
    /// Compact bitfield for tracking presence of optional and message fields
    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct _Hazzer([u8; 1]);
    impl _Hazzer {
        /// New hazzer with all fields set to off
        #[inline]
        pub const fn _new() -> Self {
            Self([0; 1])
        }
        /// Query presence of `light_mmol`
        #[inline]
        pub const fn r#light_mmol(&self) -> bool {
            (self.0[0] & 1) != 0
        }
        /// Set presence of `light_mmol`
        #[inline]
        pub const fn set_light_mmol(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem |= 1;
            self
        }
        /// Clear presence of `light_mmol`
        #[inline]
        pub const fn clear_light_mmol(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem &= !1;
            self
        }
        /// Builder method that sets the presence of `light_mmol`. Useful for initializing the Hazzer.
        #[inline]
        pub const fn init_light_mmol(mut self) -> Self {
            self.set_light_mmol();
            self
        }
        /// Query presence of `light_lux`
        #[inline]
        pub const fn r#light_lux(&self) -> bool {
            (self.0[0] & 2) != 0
        }
        /// Set presence of `light_lux`
        #[inline]
        pub const fn set_light_lux(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem |= 2;
            self
        }
        /// Clear presence of `light_lux`
        #[inline]
        pub const fn clear_light_lux(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem &= !2;
            self
        }
        /// Builder method that sets the presence of `light_lux`. Useful for initializing the Hazzer.
        #[inline]
        pub const fn init_light_lux(mut self) -> Self {
            self.set_light_lux();
            self
        }
        /// Query presence of `temperature`
        #[inline]
        pub const fn r#temperature(&self) -> bool {
            (self.0[0] & 4) != 0
        }
        /// Set presence of `temperature`
        #[inline]
        pub const fn set_temperature(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem |= 4;
            self
        }
        /// Clear presence of `temperature`
        #[inline]
        pub const fn clear_temperature(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem &= !4;
            self
        }
        /// Builder method that sets the presence of `temperature`. Useful for initializing the Hazzer.
        #[inline]
        pub const fn init_temperature(mut self) -> Self {
            self.set_temperature();
            self
        }
        /// Query presence of `humidity`
        #[inline]
        pub const fn r#humidity(&self) -> bool {
            (self.0[0] & 8) != 0
        }
        /// Set presence of `humidity`
        #[inline]
        pub const fn set_humidity(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem |= 8;
            self
        }
        /// Clear presence of `humidity`
        #[inline]
        pub const fn clear_humidity(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem &= !8;
            self
        }
        /// Builder method that sets the presence of `humidity`. Useful for initializing the Hazzer.
        #[inline]
        pub const fn init_humidity(mut self) -> Self {
            self.set_humidity();
            self
        }
        /// Query presence of `soil_moisture`
        #[inline]
        pub const fn r#soil_moisture(&self) -> bool {
            (self.0[0] & 16) != 0
        }
        /// Set presence of `soil_moisture`
        #[inline]
        pub const fn set_soil_moisture(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem |= 16;
            self
        }
        /// Clear presence of `soil_moisture`
        #[inline]
        pub const fn clear_soil_moisture(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem &= !16;
            self
        }
        /// Builder method that sets the presence of `soil_moisture`. Useful for initializing the Hazzer.
        #[inline]
        pub const fn init_soil_moisture(mut self) -> Self {
            self.set_soil_moisture();
            self
        }
        /// Query presence of `soil_ec`
        #[inline]
        pub const fn r#soil_ec(&self) -> bool {
            (self.0[0] & 32) != 0
        }
        /// Set presence of `soil_ec`
        #[inline]
        pub const fn set_soil_ec(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem |= 32;
            self
        }
        /// Clear presence of `soil_ec`
        #[inline]
        pub const fn clear_soil_ec(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem &= !32;
            self
        }
        /// Builder method that sets the presence of `soil_ec`. Useful for initializing the Hazzer.
        #[inline]
        pub const fn init_soil_ec(mut self) -> Self {
            self.set_soil_ec();
            self
        }
    }
}
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Timestamp {
    pub r#timestamp: u32,
}
impl Timestamp {
    /// Return a reference to `timestamp`
    #[inline]
    pub fn r#timestamp(&self) -> &u32 {
        &self.r#timestamp
    }
    /// Return a mutable reference to `timestamp`
    #[inline]
    pub fn mut_timestamp(&mut self) -> &mut u32 {
        &mut self.r#timestamp
    }
    /// Set the value of `timestamp`
    #[inline]
    pub fn set_timestamp(&mut self, value: u32) -> &mut Self {
        self.r#timestamp = value.into();
        self
    }
    /// Builder method that sets the value of `timestamp`. Useful for initializing the message.
    #[inline]
    pub fn init_timestamp(mut self, value: u32) -> Self {
        self.r#timestamp = value.into();
        self
    }
}
impl ::micropb::MessageDecode for Timestamp {
    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
        &mut self,
        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
        len: usize,
    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
        let before = decoder.bytes_read();
        while decoder.bytes_read() - before < len {
            let tag = decoder.decode_tag()?;
            match tag.field_num() {
                0 => return Err(::micropb::DecodeError::ZeroField),
                1u32 => {
                    let mut_ref = &mut self.r#timestamp;
                    {
                        let val = decoder.decode_varint32()?;
                        let val_ref = &val;
                        if *val_ref != 0 {
                            *mut_ref = val as _;
                        }
                    };
                }
                _ => {
                    decoder.skip_wire_value(tag.wire_type())?;
                }
            }
        }
        Ok(())
    }
}
impl ::micropb::MessageEncode for Timestamp {
    const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
        let mut max_size = 0;
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option::Some(5usize), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        ::core::option::Option::Some(max_size)
    };
    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
        &self,
        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
        use ::micropb::{PbMap, FieldEncode};
        {
            let val_ref = &self.r#timestamp;
            if *val_ref != 0 {
                encoder.encode_varint32(8u32)?;
                encoder.encode_varint32(*val_ref as _)?;
            }
        }
        Ok(())
    }
    fn compute_size(&self) -> usize {
        use ::micropb::{PbMap, FieldEncode};
        let mut size = 0;
        {
            let val_ref = &self.r#timestamp;
            if *val_ref != 0 {
                size += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
            }
        }
        size
    }
}
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Measurement {
    pub r#battery: u32,
    pub r#lux: f32,
    pub r#temperature: f32,
    pub r#humidity: f32,
    pub r#soil_pf: f32,
}
impl Measurement {
    /// Return a reference to `battery`
    #[inline]
    pub fn r#battery(&self) -> &u32 {
        &self.r#battery
    }
    /// Return a mutable reference to `battery`
    #[inline]
    pub fn mut_battery(&mut self) -> &mut u32 {
        &mut self.r#battery
    }
    /// Set the value of `battery`
    #[inline]
    pub fn set_battery(&mut self, value: u32) -> &mut Self {
        self.r#battery = value.into();
        self
    }
    /// Builder method that sets the value of `battery`. Useful for initializing the message.
    #[inline]
    pub fn init_battery(mut self, value: u32) -> Self {
        self.r#battery = value.into();
        self
    }
    /// Return a reference to `lux`
    #[inline]
    pub fn r#lux(&self) -> &f32 {
        &self.r#lux
    }
    /// Return a mutable reference to `lux`
    #[inline]
    pub fn mut_lux(&mut self) -> &mut f32 {
        &mut self.r#lux
    }
    /// Set the value of `lux`
    #[inline]
    pub fn set_lux(&mut self, value: f32) -> &mut Self {
        self.r#lux = value.into();
        self
    }
    /// Builder method that sets the value of `lux`. Useful for initializing the message.
    #[inline]
    pub fn init_lux(mut self, value: f32) -> Self {
        self.r#lux = value.into();
        self
    }
    /// Return a reference to `temperature`
    #[inline]
    pub fn r#temperature(&self) -> &f32 {
        &self.r#temperature
    }
    /// Return a mutable reference to `temperature`
    #[inline]
    pub fn mut_temperature(&mut self) -> &mut f32 {
        &mut self.r#temperature
    }
    /// Set the value of `temperature`
    #[inline]
    pub fn set_temperature(&mut self, value: f32) -> &mut Self {
        self.r#temperature = value.into();
        self
    }
    /// Builder method that sets the value of `temperature`. Useful for initializing the message.
    #[inline]
    pub fn init_temperature(mut self, value: f32) -> Self {
        self.r#temperature = value.into();
        self
    }
    /// Return a reference to `humidity`
    #[inline]
    pub fn r#humidity(&self) -> &f32 {
        &self.r#humidity
    }
    /// Return a mutable reference to `humidity`
    #[inline]
    pub fn mut_humidity(&mut self) -> &mut f32 {
        &mut self.r#humidity
    }
    /// Set the value of `humidity`
    #[inline]
    pub fn set_humidity(&mut self, value: f32) -> &mut Self {
        self.r#humidity = value.into();
        self
    }
    /// Builder method that sets the value of `humidity`. Useful for initializing the message.
    #[inline]
    pub fn init_humidity(mut self, value: f32) -> Self {
        self.r#humidity = value.into();
        self
    }
    /// Return a reference to `soil_pf`
    #[inline]
    pub fn r#soil_pf(&self) -> &f32 {
        &self.r#soil_pf
    }
    /// Return a mutable reference to `soil_pf`
    #[inline]
    pub fn mut_soil_pf(&mut self) -> &mut f32 {
        &mut self.r#soil_pf
    }
    /// Set the value of `soil_pf`
    #[inline]
    pub fn set_soil_pf(&mut self, value: f32) -> &mut Self {
        self.r#soil_pf = value.into();
        self
    }
    /// Builder method that sets the value of `soil_pf`. Useful for initializing the message.
    #[inline]
    pub fn init_soil_pf(mut self, value: f32) -> Self {
        self.r#soil_pf = value.into();
        self
    }
}
impl ::micropb::MessageDecode for Measurement {
    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
        &mut self,
        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
        len: usize,
    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
        let before = decoder.bytes_read();
        while decoder.bytes_read() - before < len {
            let tag = decoder.decode_tag()?;
            match tag.field_num() {
                0 => return Err(::micropb::DecodeError::ZeroField),
                1u32 => {
                    let mut_ref = &mut self.r#battery;
                    {
                        let val = decoder.decode_varint32()?;
                        let val_ref = &val;
                        if *val_ref != 0 {
                            *mut_ref = val as _;
                        }
                    };
                }
                2u32 => {
                    let mut_ref = &mut self.r#lux;
                    {
                        let val = decoder.decode_float()?;
                        let val_ref = &val;
                        if *val_ref != 0.0 {
                            *mut_ref = val as _;
                        }
                    };
                }
                3u32 => {
                    let mut_ref = &mut self.r#temperature;
                    {
                        let val = decoder.decode_float()?;
                        let val_ref = &val;
                        if *val_ref != 0.0 {
                            *mut_ref = val as _;
                        }
                    };
                }
                4u32 => {
                    let mut_ref = &mut self.r#humidity;
                    {
                        let val = decoder.decode_float()?;
                        let val_ref = &val;
                        if *val_ref != 0.0 {
                            *mut_ref = val as _;
                        }
                    };
                }
                5u32 => {
                    let mut_ref = &mut self.r#soil_pf;
                    {
                        let val = decoder.decode_float()?;
                        let val_ref = &val;
                        if *val_ref != 0.0 {
                            *mut_ref = val as _;
                        }
                    };
                }
                _ => {
                    decoder.skip_wire_value(tag.wire_type())?;
                }
            }
        }
        Ok(())
    }
}
impl ::micropb::MessageEncode for Measurement {
    const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
        let mut max_size = 0;
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option::Some(5usize), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option::Some(4usize), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option::Some(4usize), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option::Some(4usize), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option::Some(4usize), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        ::core::option::Option::Some(max_size)
    };
    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
        &self,
        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
        use ::micropb::{PbMap, FieldEncode};
        {
            let val_ref = &self.r#battery;
            if *val_ref != 0 {
                encoder.encode_varint32(8u32)?;
                encoder.encode_varint32(*val_ref as _)?;
            }
        }
        {
            let val_ref = &self.r#lux;
            if *val_ref != 0.0 {
                encoder.encode_varint32(21u32)?;
                encoder.encode_float(*val_ref)?;
            }
        }
        {
            let val_ref = &self.r#temperature;
            if *val_ref != 0.0 {
                encoder.encode_varint32(29u32)?;
                encoder.encode_float(*val_ref)?;
            }
        }
        {
            let val_ref = &self.r#humidity;
            if *val_ref != 0.0 {
                encoder.encode_varint32(37u32)?;
                encoder.encode_float(*val_ref)?;
            }
        }
        {
            let val_ref = &self.r#soil_pf;
            if *val_ref != 0.0 {
                encoder.encode_varint32(45u32)?;
                encoder.encode_float(*val_ref)?;
            }
        }
        Ok(())
    }
    fn compute_size(&self) -> usize {
        use ::micropb::{PbMap, FieldEncode};
        let mut size = 0;
        {
            let val_ref = &self.r#battery;
            if *val_ref != 0 {
                size += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
            }
        }
        {
            let val_ref = &self.r#lux;
            if *val_ref != 0.0 {
                size += 1usize + 4;
            }
        }
        {
            let val_ref = &self.r#temperature;
            if *val_ref != 0.0 {
                size += 1usize + 4;
            }
        }
        {
            let val_ref = &self.r#humidity;
            if *val_ref != 0.0 {
                size += 1usize + 4;
            }
        }
        {
            let val_ref = &self.r#soil_pf;
            if *val_ref != 0.0 {
                size += 1usize + 4;
            }
        }
        size
    }
}
#[derive(Debug, Default, Clone)]
pub struct MeasurementEntry {
    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
    pub r#start: Timestamp,
    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
    pub r#end: Timestamp,
    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
    pub r#measurement: Measurement,
    /// Tracks presence of optional and message fields
    pub _has: MeasurementEntry_::_Hazzer,
}
impl ::core::cmp::PartialEq for MeasurementEntry {
    fn eq(&self, other: &Self) -> bool {
        let mut ret = true;
        ret &= (self.r#start() == other.r#start());
        ret &= (self.r#end() == other.r#end());
        ret &= (self.r#measurement() == other.r#measurement());
        ret
    }
}
impl MeasurementEntry {
    /// Return a reference to `start` as an `Option`
    #[inline]
    pub fn r#start(&self) -> ::core::option::Option<&Timestamp> {
        self._has.r#start().then_some(&self.r#start)
    }
    /// Set the value and presence of `start`
    #[inline]
    pub fn set_start(&mut self, value: Timestamp) -> &mut Self {
        self._has.set_start();
        self.r#start = value.into();
        self
    }
    /// Return a mutable reference to `start` as an `Option`
    #[inline]
    pub fn mut_start(&mut self) -> ::core::option::Option<&mut Timestamp> {
        self._has.r#start().then_some(&mut self.r#start)
    }
    /// Clear the presence of `start`
    #[inline]
    pub fn clear_start(&mut self) -> &mut Self {
        self._has.clear_start();
        self
    }
    /// Take the value of `start` and clear its presence
    #[inline]
    pub fn take_start(&mut self) -> ::core::option::Option<Timestamp> {
        let val = self._has.r#start().then(|| ::core::mem::take(&mut self.r#start));
        self._has.clear_start();
        val
    }
    /// Builder method that sets the value of `start`. Useful for initializing the message.
    #[inline]
    pub fn init_start(mut self, value: Timestamp) -> Self {
        self.set_start(value);
        self
    }
    /// Return a reference to `end` as an `Option`
    #[inline]
    pub fn r#end(&self) -> ::core::option::Option<&Timestamp> {
        self._has.r#end().then_some(&self.r#end)
    }
    /// Set the value and presence of `end`
    #[inline]
    pub fn set_end(&mut self, value: Timestamp) -> &mut Self {
        self._has.set_end();
        self.r#end = value.into();
        self
    }
    /// Return a mutable reference to `end` as an `Option`
    #[inline]
    pub fn mut_end(&mut self) -> ::core::option::Option<&mut Timestamp> {
        self._has.r#end().then_some(&mut self.r#end)
    }
    /// Clear the presence of `end`
    #[inline]
    pub fn clear_end(&mut self) -> &mut Self {
        self._has.clear_end();
        self
    }
    /// Take the value of `end` and clear its presence
    #[inline]
    pub fn take_end(&mut self) -> ::core::option::Option<Timestamp> {
        let val = self._has.r#end().then(|| ::core::mem::take(&mut self.r#end));
        self._has.clear_end();
        val
    }
    /// Builder method that sets the value of `end`. Useful for initializing the message.
    #[inline]
    pub fn init_end(mut self, value: Timestamp) -> Self {
        self.set_end(value);
        self
    }
    /// Return a reference to `measurement` as an `Option`
    #[inline]
    pub fn r#measurement(&self) -> ::core::option::Option<&Measurement> {
        self._has.r#measurement().then_some(&self.r#measurement)
    }
    /// Set the value and presence of `measurement`
    #[inline]
    pub fn set_measurement(&mut self, value: Measurement) -> &mut Self {
        self._has.set_measurement();
        self.r#measurement = value.into();
        self
    }
    /// Return a mutable reference to `measurement` as an `Option`
    #[inline]
    pub fn mut_measurement(&mut self) -> ::core::option::Option<&mut Measurement> {
        self._has.r#measurement().then_some(&mut self.r#measurement)
    }
    /// Clear the presence of `measurement`
    #[inline]
    pub fn clear_measurement(&mut self) -> &mut Self {
        self._has.clear_measurement();
        self
    }
    /// Take the value of `measurement` and clear its presence
    #[inline]
    pub fn take_measurement(&mut self) -> ::core::option::Option<Measurement> {
        let val = self
            ._has
            .r#measurement()
            .then(|| ::core::mem::take(&mut self.r#measurement));
        self._has.clear_measurement();
        val
    }
    /// Builder method that sets the value of `measurement`. Useful for initializing the message.
    #[inline]
    pub fn init_measurement(mut self, value: Measurement) -> Self {
        self.set_measurement(value);
        self
    }
}
impl ::micropb::MessageDecode for MeasurementEntry {
    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
        &mut self,
        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
        len: usize,
    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
        let before = decoder.bytes_read();
        while decoder.bytes_read() - before < len {
            let tag = decoder.decode_tag()?;
            match tag.field_num() {
                0 => return Err(::micropb::DecodeError::ZeroField),
                1u32 => {
                    let mut_ref = &mut self.r#start;
                    {
                        mut_ref.decode_len_delimited(decoder)?;
                    };
                    self._has.set_start();
                }
                2u32 => {
                    let mut_ref = &mut self.r#end;
                    {
                        mut_ref.decode_len_delimited(decoder)?;
                    };
                    self._has.set_end();
                }
                3u32 => {
                    let mut_ref = &mut self.r#measurement;
                    {
                        mut_ref.decode_len_delimited(decoder)?;
                    };
                    self._has.set_measurement();
                }
                _ => {
                    decoder.skip_wire_value(tag.wire_type())?;
                }
            }
        }
        Ok(())
    }
}
impl ::micropb::MessageEncode for MeasurementEntry {
    const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
        let mut max_size = 0;
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::micropb::const_map!(< Timestamp as ::micropb::MessageEncode > ::MAX_SIZE, |
            size | ::micropb::size::sizeof_len_record(size)), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::micropb::const_map!(< Timestamp as ::micropb::MessageEncode > ::MAX_SIZE, |
            size | ::micropb::size::sizeof_len_record(size)), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::micropb::const_map!(< Measurement as ::micropb::MessageEncode > ::MAX_SIZE,
            | size | ::micropb::size::sizeof_len_record(size)), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        ::core::option::Option::Some(max_size)
    };
    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
        &self,
        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
        use ::micropb::{PbMap, FieldEncode};
        {
            if let ::core::option::Option::Some(val_ref) = self.r#start() {
                encoder.encode_varint32(10u32)?;
                val_ref.encode_len_delimited(encoder)?;
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#end() {
                encoder.encode_varint32(18u32)?;
                val_ref.encode_len_delimited(encoder)?;
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#measurement() {
                encoder.encode_varint32(26u32)?;
                val_ref.encode_len_delimited(encoder)?;
            }
        }
        Ok(())
    }
    fn compute_size(&self) -> usize {
        use ::micropb::{PbMap, FieldEncode};
        let mut size = 0;
        {
            if let ::core::option::Option::Some(val_ref) = self.r#start() {
                size
                    += 1usize
                        + ::micropb::size::sizeof_len_record(val_ref.compute_size());
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#end() {
                size
                    += 1usize
                        + ::micropb::size::sizeof_len_record(val_ref.compute_size());
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#measurement() {
                size
                    += 1usize
                        + ::micropb::size::sizeof_len_record(val_ref.compute_size());
            }
        }
        size
    }
}
/// Inner types for `MeasurementEntry`
pub mod MeasurementEntry_ {
    /// Compact bitfield for tracking presence of optional and message fields
    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct _Hazzer([u8; 1]);
    impl _Hazzer {
        /// New hazzer with all fields set to off
        #[inline]
        pub const fn _new() -> Self {
            Self([0; 1])
        }
        /// Query presence of `start`
        #[inline]
        pub const fn r#start(&self) -> bool {
            (self.0[0] & 1) != 0
        }
        /// Set presence of `start`
        #[inline]
        pub const fn set_start(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem |= 1;
            self
        }
        /// Clear presence of `start`
        #[inline]
        pub const fn clear_start(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem &= !1;
            self
        }
        /// Builder method that sets the presence of `start`. Useful for initializing the Hazzer.
        #[inline]
        pub const fn init_start(mut self) -> Self {
            self.set_start();
            self
        }
        /// Query presence of `end`
        #[inline]
        pub const fn r#end(&self) -> bool {
            (self.0[0] & 2) != 0
        }
        /// Set presence of `end`
        #[inline]
        pub const fn set_end(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem |= 2;
            self
        }
        /// Clear presence of `end`
        #[inline]
        pub const fn clear_end(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem &= !2;
            self
        }
        /// Builder method that sets the presence of `end`. Useful for initializing the Hazzer.
        #[inline]
        pub const fn init_end(mut self) -> Self {
            self.set_end();
            self
        }
        /// Query presence of `measurement`
        #[inline]
        pub const fn r#measurement(&self) -> bool {
            (self.0[0] & 4) != 0
        }
        /// Set presence of `measurement`
        #[inline]
        pub const fn set_measurement(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem |= 4;
            self
        }
        /// Clear presence of `measurement`
        #[inline]
        pub const fn clear_measurement(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem &= !4;
            self
        }
        /// Builder method that sets the presence of `measurement`. Useful for initializing the Hazzer.
        #[inline]
        pub const fn init_measurement(mut self) -> Self {
            self.set_measurement();
            self
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct WateringEntry {
    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
    pub r#occurred_at: Timestamp,
    pub r#duration_msec: u32,
    /// Tracks presence of optional and message fields
    pub _has: WateringEntry_::_Hazzer,
}
impl ::core::cmp::PartialEq for WateringEntry {
    fn eq(&self, other: &Self) -> bool {
        let mut ret = true;
        ret &= (self.r#occurred_at() == other.r#occurred_at());
        ret &= (self.r#duration_msec == other.r#duration_msec);
        ret
    }
}
impl WateringEntry {
    /// Return a reference to `occurred_at` as an `Option`
    #[inline]
    pub fn r#occurred_at(&self) -> ::core::option::Option<&Timestamp> {
        self._has.r#occurred_at().then_some(&self.r#occurred_at)
    }
    /// Set the value and presence of `occurred_at`
    #[inline]
    pub fn set_occurred_at(&mut self, value: Timestamp) -> &mut Self {
        self._has.set_occurred_at();
        self.r#occurred_at = value.into();
        self
    }
    /// Return a mutable reference to `occurred_at` as an `Option`
    #[inline]
    pub fn mut_occurred_at(&mut self) -> ::core::option::Option<&mut Timestamp> {
        self._has.r#occurred_at().then_some(&mut self.r#occurred_at)
    }
    /// Clear the presence of `occurred_at`
    #[inline]
    pub fn clear_occurred_at(&mut self) -> &mut Self {
        self._has.clear_occurred_at();
        self
    }
    /// Take the value of `occurred_at` and clear its presence
    #[inline]
    pub fn take_occurred_at(&mut self) -> ::core::option::Option<Timestamp> {
        let val = self
            ._has
            .r#occurred_at()
            .then(|| ::core::mem::take(&mut self.r#occurred_at));
        self._has.clear_occurred_at();
        val
    }
    /// Builder method that sets the value of `occurred_at`. Useful for initializing the message.
    #[inline]
    pub fn init_occurred_at(mut self, value: Timestamp) -> Self {
        self.set_occurred_at(value);
        self
    }
    /// Return a reference to `duration_msec`
    #[inline]
    pub fn r#duration_msec(&self) -> &u32 {
        &self.r#duration_msec
    }
    /// Return a mutable reference to `duration_msec`
    #[inline]
    pub fn mut_duration_msec(&mut self) -> &mut u32 {
        &mut self.r#duration_msec
    }
    /// Set the value of `duration_msec`
    #[inline]
    pub fn set_duration_msec(&mut self, value: u32) -> &mut Self {
        self.r#duration_msec = value.into();
        self
    }
    /// Builder method that sets the value of `duration_msec`. Useful for initializing the message.
    #[inline]
    pub fn init_duration_msec(mut self, value: u32) -> Self {
        self.r#duration_msec = value.into();
        self
    }
}
impl ::micropb::MessageDecode for WateringEntry {
    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
        &mut self,
        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
        len: usize,
    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
        let before = decoder.bytes_read();
        while decoder.bytes_read() - before < len {
            let tag = decoder.decode_tag()?;
            match tag.field_num() {
                0 => return Err(::micropb::DecodeError::ZeroField),
                1u32 => {
                    let mut_ref = &mut self.r#occurred_at;
                    {
                        mut_ref.decode_len_delimited(decoder)?;
                    };
                    self._has.set_occurred_at();
                }
                2u32 => {
                    let mut_ref = &mut self.r#duration_msec;
                    {
                        let val = decoder.decode_varint32()?;
                        let val_ref = &val;
                        if *val_ref != 0 {
                            *mut_ref = val as _;
                        }
                    };
                }
                _ => {
                    decoder.skip_wire_value(tag.wire_type())?;
                }
            }
        }
        Ok(())
    }
}
impl ::micropb::MessageEncode for WateringEntry {
    const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
        let mut max_size = 0;
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::micropb::const_map!(< Timestamp as ::micropb::MessageEncode > ::MAX_SIZE, |
            size | ::micropb::size::sizeof_len_record(size)), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option::Some(5usize), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        ::core::option::Option::Some(max_size)
    };
    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
        &self,
        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
        use ::micropb::{PbMap, FieldEncode};
        {
            if let ::core::option::Option::Some(val_ref) = self.r#occurred_at() {
                encoder.encode_varint32(10u32)?;
                val_ref.encode_len_delimited(encoder)?;
            }
        }
        {
            let val_ref = &self.r#duration_msec;
            if *val_ref != 0 {
                encoder.encode_varint32(16u32)?;
                encoder.encode_varint32(*val_ref as _)?;
            }
        }
        Ok(())
    }
    fn compute_size(&self) -> usize {
        use ::micropb::{PbMap, FieldEncode};
        let mut size = 0;
        {
            if let ::core::option::Option::Some(val_ref) = self.r#occurred_at() {
                size
                    += 1usize
                        + ::micropb::size::sizeof_len_record(val_ref.compute_size());
            }
        }
        {
            let val_ref = &self.r#duration_msec;
            if *val_ref != 0 {
                size += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
            }
        }
        size
    }
}
/// Inner types for `WateringEntry`
pub mod WateringEntry_ {
    /// Compact bitfield for tracking presence of optional and message fields
    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct _Hazzer([u8; 1]);
    impl _Hazzer {
        /// New hazzer with all fields set to off
        #[inline]
        pub const fn _new() -> Self {
            Self([0; 1])
        }
        /// Query presence of `occurred_at`
        #[inline]
        pub const fn r#occurred_at(&self) -> bool {
            (self.0[0] & 1) != 0
        }
        /// Set presence of `occurred_at`
        #[inline]
        pub const fn set_occurred_at(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem |= 1;
            self
        }
        /// Clear presence of `occurred_at`
        #[inline]
        pub const fn clear_occurred_at(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem &= !1;
            self
        }
        /// Builder method that sets the presence of `occurred_at`. Useful for initializing the Hazzer.
        #[inline]
        pub const fn init_occurred_at(mut self) -> Self {
            self.set_occurred_at();
            self
        }
    }
}
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Event {
    pub r#event: ::core::option::Option<Event_::Event>,
}
impl Event {}
impl ::micropb::MessageDecode for Event {
    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
        &mut self,
        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
        len: usize,
    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
        let before = decoder.bytes_read();
        while decoder.bytes_read() - before < len {
            let tag = decoder.decode_tag()?;
            match tag.field_num() {
                0 => return Err(::micropb::DecodeError::ZeroField),
                1u32 => {
                    let mut_ref = loop {
                        if let ::core::option::Option::Some(variant) = &mut self.r#event
                        {
                            if let Event_::Event::Measurement(variant) = &mut *variant {
                                break &mut *variant;
                            }
                        }
                        self.r#event = ::core::option::Option::Some(
                            Event_::Event::Measurement(
                                ::core::default::Default::default(),
                            ),
                        );
                    };
                    mut_ref.decode_len_delimited(decoder)?;
                }
                2u32 => {
                    let mut_ref = loop {
                        if let ::core::option::Option::Some(variant) = &mut self.r#event
                        {
                            if let Event_::Event::Watering(variant) = &mut *variant {
                                break &mut *variant;
                            }
                        }
                        self.r#event = ::core::option::Option::Some(
                            Event_::Event::Watering(::core::default::Default::default()),
                        );
                    };
                    mut_ref.decode_len_delimited(decoder)?;
                }
                _ => {
                    decoder.skip_wire_value(tag.wire_type())?;
                }
            }
        }
        Ok(())
    }
}
impl ::micropb::MessageEncode for Event {
    const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
        let mut max_size = 0;
        if let ::core::option::Option::Some(size) = 'oneof: {
            let mut max_size = 0;
            if let ::core::option::Option::Some(size) = ::micropb::const_map!(
                ::micropb::const_map!(< MeasurementEntry as ::micropb::MessageEncode >
                ::MAX_SIZE, | size | ::micropb::size::sizeof_len_record(size)), | size |
                size + 1usize
            ) {
                if size > max_size {
                    max_size = size;
                }
            } else {
                break 'oneof (::core::option::Option::<usize>::None);
            }
            if let ::core::option::Option::Some(size) = ::micropb::const_map!(
                ::micropb::const_map!(< WateringEntry as ::micropb::MessageEncode >
                ::MAX_SIZE, | size | ::micropb::size::sizeof_len_record(size)), | size |
                size + 1usize
            ) {
                if size > max_size {
                    max_size = size;
                }
            } else {
                break 'oneof (::core::option::Option::<usize>::None);
            }
            ::core::option::Option::Some(max_size)
        } {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        ::core::option::Option::Some(max_size)
    };
    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
        &self,
        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
        use ::micropb::{PbMap, FieldEncode};
        if let Some(oneof) = &self.r#event {
            match &*oneof {
                Event_::Event::Measurement(val_ref) => {
                    let val_ref = &*val_ref;
                    encoder.encode_varint32(10u32)?;
                    val_ref.encode_len_delimited(encoder)?;
                }
                Event_::Event::Watering(val_ref) => {
                    let val_ref = &*val_ref;
                    encoder.encode_varint32(18u32)?;
                    val_ref.encode_len_delimited(encoder)?;
                }
            }
        }
        Ok(())
    }
    fn compute_size(&self) -> usize {
        use ::micropb::{PbMap, FieldEncode};
        let mut size = 0;
        if let Some(oneof) = &self.r#event {
            match &*oneof {
                Event_::Event::Measurement(val_ref) => {
                    let val_ref = &*val_ref;
                    size
                        += 1usize
                            + ::micropb::size::sizeof_len_record(val_ref.compute_size());
                }
                Event_::Event::Watering(val_ref) => {
                    let val_ref = &*val_ref;
                    size
                        += 1usize
                            + ::micropb::size::sizeof_len_record(val_ref.compute_size());
                }
            }
        }
        size
    }
}
/// Inner types for `Event`
pub mod Event_ {
    #[derive(Debug, PartialEq, Clone)]
    pub enum Event {
        Measurement(super::MeasurementEntry),
        Watering(super::WateringEntry),
    }
}
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Events {
    pub r#events: ::micropb::heapless::Vec<Event, 16>,
}
impl Events {}
impl ::micropb::MessageDecode for Events {
    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
        &mut self,
        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
        len: usize,
    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
        let before = decoder.bytes_read();
        while decoder.bytes_read() - before < len {
            let tag = decoder.decode_tag()?;
            match tag.field_num() {
                0 => return Err(::micropb::DecodeError::ZeroField),
                1u32 => {
                    let mut val: Event = ::core::default::Default::default();
                    let mut_ref = &mut val;
                    {
                        mut_ref.decode_len_delimited(decoder)?;
                    };
                    if let (Err(_), false) = (
                        self.r#events.pb_push(val),
                        decoder.ignore_repeated_cap_err,
                    ) {
                        return Err(::micropb::DecodeError::Capacity);
                    }
                }
                _ => {
                    decoder.skip_wire_value(tag.wire_type())?;
                }
            }
        }
        Ok(())
    }
}
impl ::micropb::MessageEncode for Events {
    const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
        let mut max_size = 0;
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::micropb::const_map!(< Event as ::micropb::MessageEncode > ::MAX_SIZE, |
            size | ::micropb::size::sizeof_len_record(size)), | size | (size + 1usize) *
            16usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        ::core::option::Option::Some(max_size)
    };
    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
        &self,
        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
        use ::micropb::{PbMap, FieldEncode};
        {
            for val_ref in self.r#events.iter() {
                encoder.encode_varint32(10u32)?;
                val_ref.encode_len_delimited(encoder)?;
            }
        }
        Ok(())
    }
    fn compute_size(&self) -> usize {
        use ::micropb::{PbMap, FieldEncode};
        let mut size = 0;
        {
            for val_ref in self.r#events.iter() {
                size
                    += 1usize
                        + ::micropb::size::sizeof_len_record(val_ref.compute_size());
            }
        }
        size
    }
}
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MacAddress {
    pub r#mac_address: ::micropb::heapless::Vec<u8, 6>,
}
impl MacAddress {
    /// Return a reference to `mac_address`
    #[inline]
    pub fn r#mac_address(&self) -> &::micropb::heapless::Vec<u8, 6> {
        &self.r#mac_address
    }
    /// Return a mutable reference to `mac_address`
    #[inline]
    pub fn mut_mac_address(&mut self) -> &mut ::micropb::heapless::Vec<u8, 6> {
        &mut self.r#mac_address
    }
    /// Set the value of `mac_address`
    #[inline]
    pub fn set_mac_address(
        &mut self,
        value: ::micropb::heapless::Vec<u8, 6>,
    ) -> &mut Self {
        self.r#mac_address = value.into();
        self
    }
    /// Builder method that sets the value of `mac_address`. Useful for initializing the message.
    #[inline]
    pub fn init_mac_address(mut self, value: ::micropb::heapless::Vec<u8, 6>) -> Self {
        self.r#mac_address = value.into();
        self
    }
}
impl ::micropb::MessageDecode for MacAddress {
    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
        &mut self,
        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
        len: usize,
    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
        let before = decoder.bytes_read();
        while decoder.bytes_read() - before < len {
            let tag = decoder.decode_tag()?;
            match tag.field_num() {
                0 => return Err(::micropb::DecodeError::ZeroField),
                1u32 => {
                    let mut_ref = &mut self.r#mac_address;
                    {
                        decoder.decode_bytes(mut_ref, ::micropb::Presence::Implicit)?;
                    };
                }
                _ => {
                    decoder.skip_wire_value(tag.wire_type())?;
                }
            }
        }
        Ok(())
    }
}
impl ::micropb::MessageEncode for MacAddress {
    const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
        let mut max_size = 0;
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option::Some(7usize), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        ::core::option::Option::Some(max_size)
    };
    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
        &self,
        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
        use ::micropb::{PbMap, FieldEncode};
        {
            let val_ref = &self.r#mac_address;
            if !val_ref.is_empty() {
                encoder.encode_varint32(10u32)?;
                encoder.encode_bytes(val_ref)?;
            }
        }
        Ok(())
    }
    fn compute_size(&self) -> usize {
        use ::micropb::{PbMap, FieldEncode};
        let mut size = 0;
        {
            let val_ref = &self.r#mac_address;
            if !val_ref.is_empty() {
                size += 1usize + ::micropb::size::sizeof_len_record(val_ref.len());
            }
        }
        size
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SyncState(pub i8);
impl SyncState {
    /// Maximum encoded size of the enum
    pub const _MAX_SIZE: usize = 10usize;
    pub const Ready: Self = Self(0);
    pub const InProgress: Self = Self(1);
    pub const Done: Self = Self(2);
}
impl core::default::Default for SyncState {
    fn default() -> Self {
        Self(0)
    }
}
impl core::convert::From<i8> for SyncState {
    fn from(val: i8) -> Self {
        Self(val)
    }
}
