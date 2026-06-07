#[derive(Debug, Default, PartialEq, Clone)]
pub struct WifiConfig {
    pub r#ssid: ::std::string::String,
    pub r#password: ::std::string::String,
}
impl WifiConfig {
    /// Return a reference to `ssid`
    #[inline]
    pub fn r#ssid(&self) -> &::std::string::String {
        &self.r#ssid
    }
    /// Return a mutable reference to `ssid`
    #[inline]
    pub fn mut_ssid(&mut self) -> &mut ::std::string::String {
        &mut self.r#ssid
    }
    /// Set the value of `ssid`
    #[inline]
    pub fn set_ssid(&mut self, value: ::std::string::String) -> &mut Self {
        self.r#ssid = value.into();
        self
    }
    /// Builder method that sets the value of `ssid`. Useful for initializing the message.
    #[inline]
    pub fn init_ssid(mut self, value: ::std::string::String) -> Self {
        self.r#ssid = value.into();
        self
    }
    /// Return a reference to `password`
    #[inline]
    pub fn r#password(&self) -> &::std::string::String {
        &self.r#password
    }
    /// Return a mutable reference to `password`
    #[inline]
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.r#password
    }
    /// Set the value of `password`
    #[inline]
    pub fn set_password(&mut self, value: ::std::string::String) -> &mut Self {
        self.r#password = value.into();
        self
    }
    /// Builder method that sets the value of `password`. Useful for initializing the message.
    #[inline]
    pub fn init_password(mut self, value: ::std::string::String) -> Self {
        self.r#password = value.into();
        self
    }
}
impl ::micropb::MessageDecode for WifiConfig {
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
                    let mut_ref = &mut self.r#ssid;
                    {
                        decoder.decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                    };
                }
                2u32 => {
                    let mut_ref = &mut self.r#password;
                    {
                        decoder.decode_string(mut_ref, ::micropb::Presence::Implicit)?;
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
impl ::micropb::MessageEncode for WifiConfig {
    const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
        let mut max_size = 0;
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option:: < usize > ::None, | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option:: < usize > ::None, | size | size + 1usize
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
            let val_ref = &self.r#ssid;
            if !val_ref.is_empty() {
                encoder.encode_varint32(10u32)?;
                encoder.encode_string(val_ref)?;
            }
        }
        {
            let val_ref = &self.r#password;
            if !val_ref.is_empty() {
                encoder.encode_varint32(18u32)?;
                encoder.encode_string(val_ref)?;
            }
        }
        Ok(())
    }
    fn compute_size(&self) -> usize {
        use ::micropb::{PbMap, FieldEncode};
        let mut size = 0;
        {
            let val_ref = &self.r#ssid;
            if !val_ref.is_empty() {
                size += 1usize + ::micropb::size::sizeof_len_record(val_ref.len());
            }
        }
        {
            let val_ref = &self.r#password;
            if !val_ref.is_empty() {
                size += 1usize + ::micropb::size::sizeof_len_record(val_ref.len());
            }
        }
        size
    }
}
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OnboardingStatus {
    pub r#phase: OnboardingStatus_::Phase,
    pub r#user_code: ::std::string::String,
    pub r#verification_uri_complete: ::std::string::String,
    pub r#error: ::std::string::String,
}
impl OnboardingStatus {
    /// Return a reference to `phase`
    #[inline]
    pub fn r#phase(&self) -> &OnboardingStatus_::Phase {
        &self.r#phase
    }
    /// Return a mutable reference to `phase`
    #[inline]
    pub fn mut_phase(&mut self) -> &mut OnboardingStatus_::Phase {
        &mut self.r#phase
    }
    /// Set the value of `phase`
    #[inline]
    pub fn set_phase(&mut self, value: OnboardingStatus_::Phase) -> &mut Self {
        self.r#phase = value.into();
        self
    }
    /// Builder method that sets the value of `phase`. Useful for initializing the message.
    #[inline]
    pub fn init_phase(mut self, value: OnboardingStatus_::Phase) -> Self {
        self.r#phase = value.into();
        self
    }
    /// Return a reference to `user_code`
    #[inline]
    pub fn r#user_code(&self) -> &::std::string::String {
        &self.r#user_code
    }
    /// Return a mutable reference to `user_code`
    #[inline]
    pub fn mut_user_code(&mut self) -> &mut ::std::string::String {
        &mut self.r#user_code
    }
    /// Set the value of `user_code`
    #[inline]
    pub fn set_user_code(&mut self, value: ::std::string::String) -> &mut Self {
        self.r#user_code = value.into();
        self
    }
    /// Builder method that sets the value of `user_code`. Useful for initializing the message.
    #[inline]
    pub fn init_user_code(mut self, value: ::std::string::String) -> Self {
        self.r#user_code = value.into();
        self
    }
    /// Return a reference to `verification_uri_complete`
    #[inline]
    pub fn r#verification_uri_complete(&self) -> &::std::string::String {
        &self.r#verification_uri_complete
    }
    /// Return a mutable reference to `verification_uri_complete`
    #[inline]
    pub fn mut_verification_uri_complete(&mut self) -> &mut ::std::string::String {
        &mut self.r#verification_uri_complete
    }
    /// Set the value of `verification_uri_complete`
    #[inline]
    pub fn set_verification_uri_complete(
        &mut self,
        value: ::std::string::String,
    ) -> &mut Self {
        self.r#verification_uri_complete = value.into();
        self
    }
    /// Builder method that sets the value of `verification_uri_complete`. Useful for initializing the message.
    #[inline]
    pub fn init_verification_uri_complete(
        mut self,
        value: ::std::string::String,
    ) -> Self {
        self.r#verification_uri_complete = value.into();
        self
    }
    /// Return a reference to `error`
    #[inline]
    pub fn r#error(&self) -> &::std::string::String {
        &self.r#error
    }
    /// Return a mutable reference to `error`
    #[inline]
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.r#error
    }
    /// Set the value of `error`
    #[inline]
    pub fn set_error(&mut self, value: ::std::string::String) -> &mut Self {
        self.r#error = value.into();
        self
    }
    /// Builder method that sets the value of `error`. Useful for initializing the message.
    #[inline]
    pub fn init_error(mut self, value: ::std::string::String) -> Self {
        self.r#error = value.into();
        self
    }
}
impl ::micropb::MessageDecode for OnboardingStatus {
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
                    let mut_ref = &mut self.r#phase;
                    {
                        let val = decoder
                            .decode_int32()
                            .map(|n| OnboardingStatus_::Phase(n as _))?;
                        let val_ref = &val;
                        if val_ref.0 != 0 {
                            *mut_ref = val as _;
                        }
                    };
                }
                2u32 => {
                    let mut_ref = &mut self.r#user_code;
                    {
                        decoder.decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                    };
                }
                3u32 => {
                    let mut_ref = &mut self.r#verification_uri_complete;
                    {
                        decoder.decode_string(mut_ref, ::micropb::Presence::Implicit)?;
                    };
                }
                4u32 => {
                    let mut_ref = &mut self.r#error;
                    {
                        decoder.decode_string(mut_ref, ::micropb::Presence::Implicit)?;
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
impl ::micropb::MessageEncode for OnboardingStatus {
    const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
        let mut max_size = 0;
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option::Some(OnboardingStatus_::Phase::_MAX_SIZE), | size |
            size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option:: < usize > ::None, | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option:: < usize > ::None, | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option:: < usize > ::None, | size | size + 1usize
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
            let val_ref = &self.r#phase;
            if val_ref.0 != 0 {
                encoder.encode_varint32(8u32)?;
                encoder.encode_int32(val_ref.0 as _)?;
            }
        }
        {
            let val_ref = &self.r#user_code;
            if !val_ref.is_empty() {
                encoder.encode_varint32(18u32)?;
                encoder.encode_string(val_ref)?;
            }
        }
        {
            let val_ref = &self.r#verification_uri_complete;
            if !val_ref.is_empty() {
                encoder.encode_varint32(26u32)?;
                encoder.encode_string(val_ref)?;
            }
        }
        {
            let val_ref = &self.r#error;
            if !val_ref.is_empty() {
                encoder.encode_varint32(34u32)?;
                encoder.encode_string(val_ref)?;
            }
        }
        Ok(())
    }
    fn compute_size(&self) -> usize {
        use ::micropb::{PbMap, FieldEncode};
        let mut size = 0;
        {
            let val_ref = &self.r#phase;
            if val_ref.0 != 0 {
                size += 1usize + ::micropb::size::sizeof_int32(val_ref.0 as _);
            }
        }
        {
            let val_ref = &self.r#user_code;
            if !val_ref.is_empty() {
                size += 1usize + ::micropb::size::sizeof_len_record(val_ref.len());
            }
        }
        {
            let val_ref = &self.r#verification_uri_complete;
            if !val_ref.is_empty() {
                size += 1usize + ::micropb::size::sizeof_len_record(val_ref.len());
            }
        }
        {
            let val_ref = &self.r#error;
            if !val_ref.is_empty() {
                size += 1usize + ::micropb::size::sizeof_len_record(val_ref.len());
            }
        }
        size
    }
}
/// Inner types for `OnboardingStatus`
pub mod OnboardingStatus_ {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct Phase(pub i32);
    impl Phase {
        /// Maximum encoded size of the enum
        pub const _MAX_SIZE: usize = 10usize;
        pub const AwaitingWifi: Self = Self(0);
        pub const ProvisioningWifi: Self = Self(1);
        pub const AwaitingAuth: Self = Self(2);
        pub const Complete: Self = Self(3);
        pub const Failed: Self = Self(4);
    }
    impl core::default::Default for Phase {
        fn default() -> Self {
            Self(0)
        }
    }
    impl core::convert::From<i32> for Phase {
        fn from(val: i32) -> Self {
            Self(val)
        }
    }
}
