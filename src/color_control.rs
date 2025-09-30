
#[allow(async_fn_in_trait)]
#[allow(unknown_lints)]
#[allow(clippy::uninlined_format_args)]
#[allow(unexpected_cfgs)]

    #[cfg(not(feature = "defmt"))]
    rs_matter::reexport::bitflags::bitflags! {
        #[repr(transparent)]#[derive(Default,Debug,Copy,Clone,Eq,PartialEq,Hash)]pub struct ColorCapabilities:u16 {
            const HUE_SATURATION_SUPPORTED = 1;
            const ENHANCED_HUE_SUPPORTED = 2;
            const COLOR_LOOP_SUPPORTED = 4;
            const XY_ATTRIBUTES_SUPPORTED = 8;
            const COLOR_TEMPERATURE_SUPPORTED = 16;
            const _INTERNAL_ALL_BITS =  !0;
        }
    }
    #[cfg(feature = "defmt")]
    rs_matter::reexport::defmt::bitflags! {
        #[repr(transparent)]#[derive(Default)]pub struct ColorCapabilities:u16 {
            const HUE_SATURATION_SUPPORTED = 1;
            const ENHANCED_HUE_SUPPORTED = 2;
            const COLOR_LOOP_SUPPORTED = 4;
            const XY_ATTRIBUTES_SUPPORTED = 8;
            const COLOR_TEMPERATURE_SUPPORTED = 16;
            const _INTERNAL_ALL_BITS =  !0;
        }
    }
    rs_matter::bitflags_tlv!(ColorCapabilities, u16);
    #[cfg(not(feature = "defmt"))]
    rs_matter::reexport::bitflags::bitflags! {
        #[repr(transparent)]#[derive(Default,Debug,Copy,Clone,Eq,PartialEq,Hash)]pub struct ColorLoopUpdateFlags:u8 {
            const UPDATE_ACTION = 1;
            const UPDATE_DIRECTION = 2;
            const UPDATE_TIME = 4;
            const UPDATE_START_HUE = 8;
            const _INTERNAL_ALL_BITS =  !0;
        }
    }
    #[cfg(feature = "defmt")]
    rs_matter::reexport::defmt::bitflags! {
        #[repr(transparent)]#[derive(Default)]pub struct ColorLoopUpdateFlags:u8 {
            const UPDATE_ACTION = 1;
            const UPDATE_DIRECTION = 2;
            const UPDATE_TIME = 4;
            const UPDATE_START_HUE = 8;
            const _INTERNAL_ALL_BITS =  !0;
        }
    }
    rs_matter::bitflags_tlv!(ColorLoopUpdateFlags, u8);
    #[cfg(not(feature = "defmt"))]
    rs_matter::reexport::bitflags::bitflags! {
        #[repr(transparent)]#[derive(Default,Debug,Copy,Clone,Eq,PartialEq,Hash)]pub struct Feature:u32 {
            const HUE_AND_SATURATION = 1;
            const ENHANCED_HUE = 2;
            const COLOR_LOOP = 4;
            const XY = 8;
            const COLOR_TEMPERATURE = 16;
            const _INTERNAL_ALL_BITS =  !0;
        }
    }
    #[cfg(feature = "defmt")]
    rs_matter::reexport::defmt::bitflags! {
        #[repr(transparent)]#[derive(Default)]pub struct Feature:u32 {
            const HUE_AND_SATURATION = 1;
            const ENHANCED_HUE = 2;
            const COLOR_LOOP = 4;
            const XY = 8;
            const COLOR_TEMPERATURE = 16;
            const _INTERNAL_ALL_BITS =  !0;
        }
    }
    rs_matter::bitflags_tlv!(Feature, u32);
    #[derive(
        Debug, PartialEq, Eq, Copy, Clone, Hash, rs_matter::tlv::FromTLV, rs_matter::tlv::ToTLV,
    )]
    #[tlvargs(datatype = "u8")]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum ColorLoopAction {
        #[enumval(0)]
        Deactivate = 0,
        #[enumval(1)]
        ActivateFromColorLoopStartEnhancedHue = 1,
        #[enumval(2)]
        ActivateFromEnhancedCurrentHue = 2,
    }
    #[derive(
        Debug, PartialEq, Eq, Copy, Clone, Hash, rs_matter::tlv::FromTLV, rs_matter::tlv::ToTLV,
    )]
    #[tlvargs(datatype = "u8")]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum ColorLoopDirection {
        #[enumval(0)]
        DecrementHue = 0,
        #[enumval(1)]
        IncrementHue = 1,
    }
    #[derive(
        Debug, PartialEq, Eq, Copy, Clone, Hash, rs_matter::tlv::FromTLV, rs_matter::tlv::ToTLV,
    )]
    #[tlvargs(datatype = "u8")]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum ColorMode {
        #[enumval(0)]
        CurrentHueAndCurrentSaturation = 0,
        #[enumval(1)]
        CurrentXAndCurrentY = 1,
        #[enumval(2)]
        ColorTemperature = 2,
    }
    #[derive(
        Debug, PartialEq, Eq, Copy, Clone, Hash, rs_matter::tlv::FromTLV, rs_matter::tlv::ToTLV,
    )]
    #[tlvargs(datatype = "u8")]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum HueDirection {
        #[enumval(0)]
        ShortestDistance = 0,
        #[enumval(1)]
        LongestDistance = 1,
        #[enumval(2)]
        Up = 2,
        #[enumval(3)]
        Down = 3,
    }
    #[derive(
        Debug, PartialEq, Eq, Copy, Clone, Hash, rs_matter::tlv::FromTLV, rs_matter::tlv::ToTLV,
    )]
    #[tlvargs(datatype = "u8")]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum HueMoveMode {
        #[enumval(0)]
        Stop = 0,
        #[enumval(1)]
        Up = 1,
        #[enumval(3)]
        Down = 3,
    }
    #[derive(
        Debug, PartialEq, Eq, Copy, Clone, Hash, rs_matter::tlv::FromTLV, rs_matter::tlv::ToTLV,
    )]
    #[tlvargs(datatype = "u8")]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum HueStepMode {
        #[enumval(1)]
        Up = 1,
        #[enumval(3)]
        Down = 3,
    }
    #[derive(
        Debug, PartialEq, Eq, Copy, Clone, Hash, rs_matter::tlv::FromTLV, rs_matter::tlv::ToTLV,
    )]
    #[tlvargs(datatype = "u8")]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum SaturationMoveMode {
        #[enumval(0)]
        Stop = 0,
        #[enumval(1)]
        Up = 1,
        #[enumval(3)]
        Down = 3,
    }
    #[derive(
        Debug, PartialEq, Eq, Copy, Clone, Hash, rs_matter::tlv::FromTLV, rs_matter::tlv::ToTLV,
    )]
    #[tlvargs(datatype = "u8")]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum SaturationStepMode {
        #[enumval(1)]
        Up = 1,
        #[enumval(3)]
        Down = 3,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum MoveToHueRequestTag {
        Hue = 0,
        Direction = 1,
        TransitionTime = 2,
        OptionsMask = 3,
        OptionsOverride = 4,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum MoveHueRequestTag {
        MoveMode = 0,
        Rate = 1,
        OptionsMask = 2,
        OptionsOverride = 3,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum StepHueRequestTag {
        StepMode = 0,
        StepSize = 1,
        TransitionTime = 2,
        OptionsMask = 3,
        OptionsOverride = 4,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum MoveToSaturationRequestTag {
        Saturation = 0,
        TransitionTime = 1,
        OptionsMask = 2,
        OptionsOverride = 3,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum MoveSaturationRequestTag {
        MoveMode = 0,
        Rate = 1,
        OptionsMask = 2,
        OptionsOverride = 3,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum StepSaturationRequestTag {
        StepMode = 0,
        StepSize = 1,
        TransitionTime = 2,
        OptionsMask = 3,
        OptionsOverride = 4,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum MoveToHueAndSaturationRequestTag {
        Hue = 0,
        Saturation = 1,
        TransitionTime = 2,
        OptionsMask = 3,
        OptionsOverride = 4,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum MoveToColorRequestTag {
        ColorX = 0,
        ColorY = 1,
        TransitionTime = 2,
        OptionsMask = 3,
        OptionsOverride = 4,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum MoveColorRequestTag {
        RateX = 0,
        RateY = 1,
        OptionsMask = 2,
        OptionsOverride = 3,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum StepColorRequestTag {
        StepX = 0,
        StepY = 1,
        TransitionTime = 2,
        OptionsMask = 3,
        OptionsOverride = 4,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum MoveToColorTemperatureRequestTag {
        ColorTemperatureMireds = 0,
        TransitionTime = 1,
        OptionsMask = 2,
        OptionsOverride = 3,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum EnhancedMoveToHueRequestTag {
        EnhancedHue = 0,
        Direction = 1,
        TransitionTime = 2,
        OptionsMask = 3,
        OptionsOverride = 4,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum EnhancedMoveHueRequestTag {
        MoveMode = 0,
        Rate = 1,
        OptionsMask = 2,
        OptionsOverride = 3,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum EnhancedStepHueRequestTag {
        StepMode = 0,
        StepSize = 1,
        TransitionTime = 2,
        OptionsMask = 3,
        OptionsOverride = 4,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum EnhancedMoveToHueAndSaturationRequestTag {
        EnhancedHue = 0,
        Saturation = 1,
        TransitionTime = 2,
        OptionsMask = 3,
        OptionsOverride = 4,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum ColorLoopSetRequestTag {
        UpdateFlags = 0,
        Action = 1,
        Direction = 2,
        Time = 3,
        StartHue = 4,
        OptionsMask = 5,
        OptionsOverride = 6,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum StopMoveStepRequestTag {
        OptionsMask = 0,
        OptionsOverride = 1,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum MoveColorTemperatureRequestTag {
        MoveMode = 0,
        Rate = 1,
        ColorTemperatureMinimumMireds = 2,
        ColorTemperatureMaximumMireds = 3,
        OptionsMask = 4,
        OptionsOverride = 5,
    }
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u8)]
    pub enum StepColorTemperatureRequestTag {
        StepMode = 0,
        StepSize = 1,
        TransitionTime = 2,
        ColorTemperatureMinimumMireds = 3,
        ColorTemperatureMaximumMireds = 4,
        OptionsMask = 5,
        OptionsOverride = 6,
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct MoveToHueRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> MoveToHueRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn hue(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn direction(&self) -> Result<HueDirection, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn transition_time(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(4)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for MoveToHueRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for MoveToHueRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for MoveToHueRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "MoveToHueRequest")?;
            match self.hue() {
                Ok(value) => write!(f, "{}: {:?},", "hue", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "hue", e.code())?,
            }
            match self.direction() {
                Ok(value) => write!(f, "{}: {:?},", "direction", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "direction", e.code())?,
            }
            match self.transition_time() {
                Ok(value) => write!(f, "{}: {:?},", "transition_time", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "transition_time", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for MoveToHueRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "MoveToHueRequest");
            match self.hue() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "hue", value),
                Err(e) => rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "hue", e.code()),
            }
            match self.direction() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "direction", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "direction", e.code())
                }
            }
            match self.transition_time() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "transition_time", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "transition_time",
                    e.code()
                ),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct MoveHueRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> MoveHueRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn move_mode(&self) -> Result<HueMoveMode, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn rate(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for MoveHueRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for MoveHueRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for MoveHueRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "MoveHueRequest")?;
            match self.move_mode() {
                Ok(value) => write!(f, "{}: {:?},", "move_mode", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "move_mode", e.code())?,
            }
            match self.rate() {
                Ok(value) => write!(f, "{}: {:?},", "rate", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "rate", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for MoveHueRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "MoveHueRequest");
            match self.move_mode() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "move_mode", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "move_mode", e.code())
                }
            }
            match self.rate() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "rate", value),
                Err(e) => rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "rate", e.code()),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct StepHueRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> StepHueRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn step_mode(&self) -> Result<HueStepMode, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn step_size(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn transition_time(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(4)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for StepHueRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for StepHueRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for StepHueRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "StepHueRequest")?;
            match self.step_mode() {
                Ok(value) => write!(f, "{}: {:?},", "step_mode", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "step_mode", e.code())?,
            }
            match self.step_size() {
                Ok(value) => write!(f, "{}: {:?},", "step_size", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "step_size", e.code())?,
            }
            match self.transition_time() {
                Ok(value) => write!(f, "{}: {:?},", "transition_time", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "transition_time", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for StepHueRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "StepHueRequest");
            match self.step_mode() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "step_mode", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "step_mode", e.code())
                }
            }
            match self.step_size() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "step_size", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "step_size", e.code())
                }
            }
            match self.transition_time() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "transition_time", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "transition_time",
                    e.code()
                ),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct MoveToSaturationRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> MoveToSaturationRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn saturation(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn transition_time(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for MoveToSaturationRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for MoveToSaturationRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for MoveToSaturationRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "MoveToSaturationRequest")?;
            match self.saturation() {
                Ok(value) => write!(f, "{}: {:?},", "saturation", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "saturation", e.code())?,
            }
            match self.transition_time() {
                Ok(value) => write!(f, "{}: {:?},", "transition_time", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "transition_time", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for MoveToSaturationRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "MoveToSaturationRequest");
            match self.saturation() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "saturation", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "saturation", e.code())
                }
            }
            match self.transition_time() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "transition_time", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "transition_time",
                    e.code()
                ),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct MoveSaturationRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> MoveSaturationRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn move_mode(&self) -> Result<SaturationMoveMode, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn rate(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for MoveSaturationRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for MoveSaturationRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for MoveSaturationRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "MoveSaturationRequest")?;
            match self.move_mode() {
                Ok(value) => write!(f, "{}: {:?},", "move_mode", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "move_mode", e.code())?,
            }
            match self.rate() {
                Ok(value) => write!(f, "{}: {:?},", "rate", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "rate", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for MoveSaturationRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "MoveSaturationRequest");
            match self.move_mode() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "move_mode", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "move_mode", e.code())
                }
            }
            match self.rate() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "rate", value),
                Err(e) => rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "rate", e.code()),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct StepSaturationRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> StepSaturationRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn step_mode(&self) -> Result<SaturationStepMode, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn step_size(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn transition_time(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(4)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for StepSaturationRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for StepSaturationRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for StepSaturationRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "StepSaturationRequest")?;
            match self.step_mode() {
                Ok(value) => write!(f, "{}: {:?},", "step_mode", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "step_mode", e.code())?,
            }
            match self.step_size() {
                Ok(value) => write!(f, "{}: {:?},", "step_size", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "step_size", e.code())?,
            }
            match self.transition_time() {
                Ok(value) => write!(f, "{}: {:?},", "transition_time", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "transition_time", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for StepSaturationRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "StepSaturationRequest");
            match self.step_mode() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "step_mode", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "step_mode", e.code())
                }
            }
            match self.step_size() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "step_size", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "step_size", e.code())
                }
            }
            match self.transition_time() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "transition_time", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "transition_time",
                    e.code()
                ),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct MoveToHueAndSaturationRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> MoveToHueAndSaturationRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn hue(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn saturation(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn transition_time(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(4)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for MoveToHueAndSaturationRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for MoveToHueAndSaturationRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for MoveToHueAndSaturationRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "MoveToHueAndSaturationRequest")?;
            match self.hue() {
                Ok(value) => write!(f, "{}: {:?},", "hue", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "hue", e.code())?,
            }
            match self.saturation() {
                Ok(value) => write!(f, "{}: {:?},", "saturation", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "saturation", e.code())?,
            }
            match self.transition_time() {
                Ok(value) => write!(f, "{}: {:?},", "transition_time", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "transition_time", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for MoveToHueAndSaturationRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "MoveToHueAndSaturationRequest");
            match self.hue() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "hue", value),
                Err(e) => rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "hue", e.code()),
            }
            match self.saturation() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "saturation", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "saturation", e.code())
                }
            }
            match self.transition_time() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "transition_time", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "transition_time",
                    e.code()
                ),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct MoveToColorRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> MoveToColorRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn color_x(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn color_y(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn transition_time(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(4)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for MoveToColorRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for MoveToColorRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for MoveToColorRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "MoveToColorRequest")?;
            match self.color_x() {
                Ok(value) => write!(f, "{}: {:?},", "color_x", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "color_x", e.code())?,
            }
            match self.color_y() {
                Ok(value) => write!(f, "{}: {:?},", "color_y", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "color_y", e.code())?,
            }
            match self.transition_time() {
                Ok(value) => write!(f, "{}: {:?},", "transition_time", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "transition_time", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for MoveToColorRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "MoveToColorRequest");
            match self.color_x() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "color_x", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "color_x", e.code())
                }
            }
            match self.color_y() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "color_y", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "color_y", e.code())
                }
            }
            match self.transition_time() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "transition_time", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "transition_time",
                    e.code()
                ),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct MoveColorRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> MoveColorRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn rate_x(&self) -> Result<i16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn rate_y(&self) -> Result<i16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for MoveColorRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for MoveColorRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for MoveColorRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "MoveColorRequest")?;
            match self.rate_x() {
                Ok(value) => write!(f, "{}: {:?},", "rate_x", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "rate_x", e.code())?,
            }
            match self.rate_y() {
                Ok(value) => write!(f, "{}: {:?},", "rate_y", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "rate_y", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for MoveColorRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "MoveColorRequest");
            match self.rate_x() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "rate_x", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "rate_x", e.code())
                }
            }
            match self.rate_y() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "rate_y", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "rate_y", e.code())
                }
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct StepColorRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> StepColorRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn step_x(&self) -> Result<i16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn step_y(&self) -> Result<i16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn transition_time(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(4)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for StepColorRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for StepColorRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for StepColorRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "StepColorRequest")?;
            match self.step_x() {
                Ok(value) => write!(f, "{}: {:?},", "step_x", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "step_x", e.code())?,
            }
            match self.step_y() {
                Ok(value) => write!(f, "{}: {:?},", "step_y", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "step_y", e.code())?,
            }
            match self.transition_time() {
                Ok(value) => write!(f, "{}: {:?},", "transition_time", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "transition_time", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for StepColorRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "StepColorRequest");
            match self.step_x() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "step_x", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "step_x", e.code())
                }
            }
            match self.step_y() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "step_y", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "step_y", e.code())
                }
            }
            match self.transition_time() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "transition_time", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "transition_time",
                    e.code()
                ),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct MoveToColorTemperatureRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> MoveToColorTemperatureRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn color_temperature_mireds(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn transition_time(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for MoveToColorTemperatureRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for MoveToColorTemperatureRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for MoveToColorTemperatureRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "MoveToColorTemperatureRequest")?;
            match self.color_temperature_mireds() {
                Ok(value) => write!(f, "{}: {:?},", "color_temperature_mireds", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "color_temperature_mireds", e.code())?,
            }
            match self.transition_time() {
                Ok(value) => write!(f, "{}: {:?},", "transition_time", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "transition_time", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for MoveToColorTemperatureRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "MoveToColorTemperatureRequest");
            match self.color_temperature_mireds() {
                Ok(value) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: {:?},",
                    "color_temperature_mireds",
                    value
                ),
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "color_temperature_mireds",
                    e.code()
                ),
            }
            match self.transition_time() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "transition_time", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "transition_time",
                    e.code()
                ),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct EnhancedMoveToHueRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> EnhancedMoveToHueRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn enhanced_hue(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn direction(&self) -> Result<HueDirection, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn transition_time(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(4)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for EnhancedMoveToHueRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for EnhancedMoveToHueRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for EnhancedMoveToHueRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "EnhancedMoveToHueRequest")?;
            match self.enhanced_hue() {
                Ok(value) => write!(f, "{}: {:?},", "enhanced_hue", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "enhanced_hue", e.code())?,
            }
            match self.direction() {
                Ok(value) => write!(f, "{}: {:?},", "direction", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "direction", e.code())?,
            }
            match self.transition_time() {
                Ok(value) => write!(f, "{}: {:?},", "transition_time", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "transition_time", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for EnhancedMoveToHueRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "EnhancedMoveToHueRequest");
            match self.enhanced_hue() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "enhanced_hue", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "enhanced_hue", e.code())
                }
            }
            match self.direction() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "direction", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "direction", e.code())
                }
            }
            match self.transition_time() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "transition_time", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "transition_time",
                    e.code()
                ),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct EnhancedMoveHueRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> EnhancedMoveHueRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn move_mode(&self) -> Result<HueMoveMode, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn rate(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for EnhancedMoveHueRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for EnhancedMoveHueRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for EnhancedMoveHueRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "EnhancedMoveHueRequest")?;
            match self.move_mode() {
                Ok(value) => write!(f, "{}: {:?},", "move_mode", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "move_mode", e.code())?,
            }
            match self.rate() {
                Ok(value) => write!(f, "{}: {:?},", "rate", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "rate", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for EnhancedMoveHueRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "EnhancedMoveHueRequest");
            match self.move_mode() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "move_mode", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "move_mode", e.code())
                }
            }
            match self.rate() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "rate", value),
                Err(e) => rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "rate", e.code()),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct EnhancedStepHueRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> EnhancedStepHueRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn step_mode(&self) -> Result<HueStepMode, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn step_size(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn transition_time(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(4)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for EnhancedStepHueRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for EnhancedStepHueRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for EnhancedStepHueRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "EnhancedStepHueRequest")?;
            match self.step_mode() {
                Ok(value) => write!(f, "{}: {:?},", "step_mode", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "step_mode", e.code())?,
            }
            match self.step_size() {
                Ok(value) => write!(f, "{}: {:?},", "step_size", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "step_size", e.code())?,
            }
            match self.transition_time() {
                Ok(value) => write!(f, "{}: {:?},", "transition_time", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "transition_time", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for EnhancedStepHueRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "EnhancedStepHueRequest");
            match self.step_mode() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "step_mode", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "step_mode", e.code())
                }
            }
            match self.step_size() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "step_size", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "step_size", e.code())
                }
            }
            match self.transition_time() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "transition_time", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "transition_time",
                    e.code()
                ),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct EnhancedMoveToHueAndSaturationRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> EnhancedMoveToHueAndSaturationRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn enhanced_hue(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn saturation(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn transition_time(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(4)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for EnhancedMoveToHueAndSaturationRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for EnhancedMoveToHueAndSaturationRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for EnhancedMoveToHueAndSaturationRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "EnhancedMoveToHueAndSaturationRequest")?;
            match self.enhanced_hue() {
                Ok(value) => write!(f, "{}: {:?},", "enhanced_hue", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "enhanced_hue", e.code())?,
            }
            match self.saturation() {
                Ok(value) => write!(f, "{}: {:?},", "saturation", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "saturation", e.code())?,
            }
            match self.transition_time() {
                Ok(value) => write!(f, "{}: {:?},", "transition_time", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "transition_time", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for EnhancedMoveToHueAndSaturationRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "EnhancedMoveToHueAndSaturationRequest");
            match self.enhanced_hue() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "enhanced_hue", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "enhanced_hue", e.code())
                }
            }
            match self.saturation() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "saturation", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "saturation", e.code())
                }
            }
            match self.transition_time() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "transition_time", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "transition_time",
                    e.code()
                ),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct ColorLoopSetRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> ColorLoopSetRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn update_flags(&self) -> Result<ColorLoopUpdateFlags, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn action(&self) -> Result<ColorLoopAction, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn direction(&self) -> Result<ColorLoopDirection, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn time(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
        pub fn start_hue(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(4)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(5)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(6)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for ColorLoopSetRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for ColorLoopSetRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for ColorLoopSetRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "ColorLoopSetRequest")?;
            match self.update_flags() {
                Ok(value) => write!(f, "{}: {:?},", "update_flags", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "update_flags", e.code())?,
            }
            match self.action() {
                Ok(value) => write!(f, "{}: {:?},", "action", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "action", e.code())?,
            }
            match self.direction() {
                Ok(value) => write!(f, "{}: {:?},", "direction", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "direction", e.code())?,
            }
            match self.time() {
                Ok(value) => write!(f, "{}: {:?},", "time", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "time", e.code())?,
            }
            match self.start_hue() {
                Ok(value) => write!(f, "{}: {:?},", "start_hue", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "start_hue", e.code())?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for ColorLoopSetRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "ColorLoopSetRequest");
            match self.update_flags() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "update_flags", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "update_flags", e.code())
                }
            }
            match self.action() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "action", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "action", e.code())
                }
            }
            match self.direction() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "direction", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "direction", e.code())
                }
            }
            match self.time() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "time", value),
                Err(e) => rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "time", e.code()),
            }
            match self.start_hue() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "start_hue", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "start_hue", e.code())
                }
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct StopMoveStepRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> StopMoveStepRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for StopMoveStepRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for StopMoveStepRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for StopMoveStepRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "StopMoveStepRequest")?;
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for StopMoveStepRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "StopMoveStepRequest");
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct MoveColorTemperatureRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> MoveColorTemperatureRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn move_mode(&self) -> Result<HueMoveMode, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn rate(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn color_temperature_minimum_mireds(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn color_temperature_maximum_mireds(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(4)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(5)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for MoveColorTemperatureRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for MoveColorTemperatureRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for MoveColorTemperatureRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "MoveColorTemperatureRequest")?;
            match self.move_mode() {
                Ok(value) => write!(f, "{}: {:?},", "move_mode", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "move_mode", e.code())?,
            }
            match self.rate() {
                Ok(value) => write!(f, "{}: {:?},", "rate", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "rate", e.code())?,
            }
            match self.color_temperature_minimum_mireds() {
                Ok(value) => write!(f, "{}: {:?},", "color_temperature_minimum_mireds", value)?,
                Err(e) => write!(
                    f,
                    "{}: ??? {:?},",
                    "color_temperature_minimum_mireds",
                    e.code()
                )?,
            }
            match self.color_temperature_maximum_mireds() {
                Ok(value) => write!(f, "{}: {:?},", "color_temperature_maximum_mireds", value)?,
                Err(e) => write!(
                    f,
                    "{}: ??? {:?},",
                    "color_temperature_maximum_mireds",
                    e.code()
                )?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for MoveColorTemperatureRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "MoveColorTemperatureRequest");
            match self.move_mode() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "move_mode", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "move_mode", e.code())
                }
            }
            match self.rate() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "rate", value),
                Err(e) => rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "rate", e.code()),
            }
            match self.color_temperature_minimum_mireds() {
                Ok(value) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: {:?},",
                    "color_temperature_minimum_mireds",
                    value
                ),
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "color_temperature_minimum_mireds",
                    e.code()
                ),
            }
            match self.color_temperature_maximum_mireds() {
                Ok(value) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: {:?},",
                    "color_temperature_maximum_mireds",
                    value
                ),
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "color_temperature_maximum_mireds",
                    e.code()
                ),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub struct StepColorTemperatureRequest<'a>(rs_matter::tlv::TLVElement<'a>);

    impl<'a> StepColorTemperatureRequest<'a> {
        #[doc = "Create a new instance"]
        pub const fn new(element: rs_matter::tlv::TLVElement<'a>) -> Self {
            Self(element)
        }
        #[doc = "Return the underlying TLV element"]
        pub const fn tlv_element(&self) -> &rs_matter::tlv::TLVElement<'a> {
            &self.0
        }
        pub fn step_mode(&self) -> Result<HueStepMode, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(0)?)
        }
        pub fn step_size(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(1)?)
        }
        pub fn transition_time(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(2)?)
        }
        pub fn color_temperature_minimum_mireds(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(3)?)
        }
        pub fn color_temperature_maximum_mireds(&self) -> Result<u16, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(4)?)
        }
        pub fn options_mask(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(5)?)
        }
        pub fn options_override(&self) -> Result<u8, rs_matter::error::Error> {
            rs_matter::tlv::FromTLV::from_tlv(&self.0.structure()?.ctx(6)?)
        }
    }
    impl<'a> rs_matter::tlv::FromTLV<'a> for StepColorTemperatureRequest<'a> {
        fn from_tlv(
            element: &rs_matter::tlv::TLVElement<'a>,
        ) -> Result<Self, rs_matter::error::Error> {
            Ok(Self::new(element.clone()))
        }
    }
    impl rs_matter::tlv::ToTLV for StepColorTemperatureRequest<'_> {
        fn to_tlv<W: rs_matter::tlv::TLVWrite>(
            &self,
            tag: &rs_matter::tlv::TLVTag,
            tw: W,
        ) -> Result<(), rs_matter::error::Error> {
            self.0.to_tlv(tag, tw)
        }
        fn tlv_iter(
            &self,
            tag: rs_matter::tlv::TLVTag,
        ) -> impl Iterator<Item = Result<rs_matter::tlv::TLV, rs_matter::error::Error>> {
            self.0.tlv_iter(tag)
        }
    }
    impl core::fmt::Debug for StepColorTemperatureRequest<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{", "StepColorTemperatureRequest")?;
            match self.step_mode() {
                Ok(value) => write!(f, "{}: {:?},", "step_mode", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "step_mode", e.code())?,
            }
            match self.step_size() {
                Ok(value) => write!(f, "{}: {:?},", "step_size", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "step_size", e.code())?,
            }
            match self.transition_time() {
                Ok(value) => write!(f, "{}: {:?},", "transition_time", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "transition_time", e.code())?,
            }
            match self.color_temperature_minimum_mireds() {
                Ok(value) => write!(f, "{}: {:?},", "color_temperature_minimum_mireds", value)?,
                Err(e) => write!(
                    f,
                    "{}: ??? {:?},",
                    "color_temperature_minimum_mireds",
                    e.code()
                )?,
            }
            match self.color_temperature_maximum_mireds() {
                Ok(value) => write!(f, "{}: {:?},", "color_temperature_maximum_mireds", value)?,
                Err(e) => write!(
                    f,
                    "{}: ??? {:?},",
                    "color_temperature_maximum_mireds",
                    e.code()
                )?,
            }
            match self.options_mask() {
                Ok(value) => write!(f, "{}: {:?},", "options_mask", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_mask", e.code())?,
            }
            match self.options_override() {
                Ok(value) => write!(f, "{}: {:?},", "options_override", value)?,
                Err(e) => write!(f, "{}: ??? {:?},", "options_override", e.code())?,
            }
            write!(f, "}}")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for StepColorTemperatureRequest<'_> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{} {{", "StepColorTemperatureRequest");
            match self.step_mode() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "step_mode", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "step_mode", e.code())
                }
            }
            match self.step_size() {
                Ok(value) => rs_matter::reexport::defmt::write!(f, "{}: {:?},", "step_size", value),
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "step_size", e.code())
                }
            }
            match self.transition_time() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "transition_time", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "transition_time",
                    e.code()
                ),
            }
            match self.color_temperature_minimum_mireds() {
                Ok(value) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: {:?},",
                    "color_temperature_minimum_mireds",
                    value
                ),
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "color_temperature_minimum_mireds",
                    e.code()
                ),
            }
            match self.color_temperature_maximum_mireds() {
                Ok(value) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: {:?},",
                    "color_temperature_maximum_mireds",
                    value
                ),
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "color_temperature_maximum_mireds",
                    e.code()
                ),
            }
            match self.options_mask() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_mask", value)
                }
                Err(e) => {
                    rs_matter::reexport::defmt::write!(f, "{}: ??? {:?},", "options_mask", e.code())
                }
            }
            match self.options_override() {
                Ok(value) => {
                    rs_matter::reexport::defmt::write!(f, "{}: {:?},", "options_override", value)
                }
                Err(e) => rs_matter::reexport::defmt::write!(
                    f,
                    "{}: ??? {:?},",
                    "options_override",
                    e.code()
                ),
            }
            rs_matter::reexport::defmt::write!(f, "}}")
        }
    }
    pub struct MoveToHueRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> MoveToHueRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToHueRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn hue(
            mut self,
            value: u8,
        ) -> Result<MoveToHueRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "hue", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "hue", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToHueRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn hue(
            mut self,
            value: u8,
        ) -> Result<MoveToHueRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "hue", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToHueRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn direction(
            mut self,
            value: HueDirection,
        ) -> Result<MoveToHueRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "direction", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "direction", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToHueRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn direction(
            mut self,
            value: HueDirection,
        ) -> Result<MoveToHueRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "direction", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToHueRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<MoveToHueRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToHueRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<MoveToHueRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToHueRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveToHueRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToHueRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveToHueRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToHueRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveToHueRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(MoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToHueRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveToHueRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(MoveToHueRequestBuilder(self.0))
        }
    }
    impl<P> MoveToHueRequestBuilder<P, 5usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for MoveToHueRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveToHueRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for MoveToHueRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "MoveToHueRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for MoveToHueRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveToHueRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveToHueRequestArrayBuilder<P>(P);

    impl<P> MoveToHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<MoveToHueRequestBuilder<MoveToHueRequestArrayBuilder<P>>, rs_matter::error::Error>
        {
            rs_matter::tlv::TLVBuilder::new(
                MoveToHueRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for MoveToHueRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveToHueRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for MoveToHueRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "MoveToHueRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for MoveToHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveToHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveHueRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> MoveHueRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveHueRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn move_mode(
            mut self,
            value: HueMoveMode,
        ) -> Result<MoveHueRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "moveMode", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "moveMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveHueRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn move_mode(
            mut self,
            value: HueMoveMode,
        ) -> Result<MoveHueRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "moveMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveHueRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn rate(
            mut self,
            value: u8,
        ) -> Result<MoveHueRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "rate", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "rate", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveHueRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn rate(
            mut self,
            value: u8,
        ) -> Result<MoveHueRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "rate", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveHueRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveHueRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveHueRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveHueRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveHueRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveHueRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveHueRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveHueRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveHueRequestBuilder(self.0))
        }
    }
    impl<P> MoveHueRequestBuilder<P, 4usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for MoveHueRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveHueRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for MoveHueRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "MoveHueRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for MoveHueRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveHueRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveHueRequestArrayBuilder<P>(P);

    impl<P> MoveHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<MoveHueRequestBuilder<MoveHueRequestArrayBuilder<P>>, rs_matter::error::Error>
        {
            rs_matter::tlv::TLVBuilder::new(
                MoveHueRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for MoveHueRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveHueRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for MoveHueRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "MoveHueRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for MoveHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct StepHueRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> StepHueRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepHueRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn step_mode(
            mut self,
            value: HueStepMode,
        ) -> Result<StepHueRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "stepMode", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(StepHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepHueRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn step_mode(
            mut self,
            value: HueStepMode,
        ) -> Result<StepHueRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(StepHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepHueRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn step_size(
            mut self,
            value: u8,
        ) -> Result<StepHueRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "stepSize", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepSize", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(StepHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepHueRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn step_size(
            mut self,
            value: u8,
        ) -> Result<StepHueRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepSize", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(StepHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepHueRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn transition_time(
            mut self,
            value: u8,
        ) -> Result<StepHueRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(StepHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepHueRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn transition_time(
            mut self,
            value: u8,
        ) -> Result<StepHueRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(StepHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepHueRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<StepHueRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(StepHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepHueRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<StepHueRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(StepHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepHueRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<StepHueRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(StepHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepHueRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<StepHueRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(StepHueRequestBuilder(self.0))
        }
    }
    impl<P> StepHueRequestBuilder<P, 5usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for StepHueRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "StepHueRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for StepHueRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "StepHueRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for StepHueRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for StepHueRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct StepHueRequestArrayBuilder<P>(P);

    impl<P> StepHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<StepHueRequestBuilder<StepHueRequestArrayBuilder<P>>, rs_matter::error::Error>
        {
            rs_matter::tlv::TLVBuilder::new(
                StepHueRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for StepHueRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "StepHueRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for StepHueRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "StepHueRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for StepHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for StepHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveToSaturationRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> MoveToSaturationRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToSaturationRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn saturation(
            mut self,
            value: u8,
        ) -> Result<MoveToSaturationRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "saturation", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "saturation", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveToSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToSaturationRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn saturation(
            mut self,
            value: u8,
        ) -> Result<MoveToSaturationRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "saturation", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveToSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToSaturationRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<MoveToSaturationRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveToSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToSaturationRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<MoveToSaturationRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveToSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToSaturationRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveToSaturationRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveToSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToSaturationRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveToSaturationRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveToSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToSaturationRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveToSaturationRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveToSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToSaturationRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveToSaturationRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveToSaturationRequestBuilder(self.0))
        }
    }
    impl<P> MoveToSaturationRequestBuilder<P, 4usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for MoveToSaturationRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveToSaturationRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for MoveToSaturationRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "MoveToSaturationRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for MoveToSaturationRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveToSaturationRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveToSaturationRequestArrayBuilder<P>(P);

    impl<P> MoveToSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            MoveToSaturationRequestBuilder<MoveToSaturationRequestArrayBuilder<P>>,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                MoveToSaturationRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for MoveToSaturationRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveToSaturationRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for MoveToSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "MoveToSaturationRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for MoveToSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveToSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveSaturationRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> MoveSaturationRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveSaturationRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn move_mode(
            mut self,
            value: SaturationMoveMode,
        ) -> Result<MoveSaturationRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "moveMode", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "moveMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveSaturationRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn move_mode(
            mut self,
            value: SaturationMoveMode,
        ) -> Result<MoveSaturationRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "moveMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveSaturationRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn rate(
            mut self,
            value: u8,
        ) -> Result<MoveSaturationRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "rate", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "rate", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveSaturationRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn rate(
            mut self,
            value: u8,
        ) -> Result<MoveSaturationRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "rate", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveSaturationRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveSaturationRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveSaturationRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveSaturationRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveSaturationRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveSaturationRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveSaturationRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveSaturationRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveSaturationRequestBuilder(self.0))
        }
    }
    impl<P> MoveSaturationRequestBuilder<P, 4usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for MoveSaturationRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveSaturationRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for MoveSaturationRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "MoveSaturationRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for MoveSaturationRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveSaturationRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveSaturationRequestArrayBuilder<P>(P);

    impl<P> MoveSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            MoveSaturationRequestBuilder<MoveSaturationRequestArrayBuilder<P>>,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                MoveSaturationRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for MoveSaturationRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveSaturationRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for MoveSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "MoveSaturationRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for MoveSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct StepSaturationRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> StepSaturationRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepSaturationRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn step_mode(
            mut self,
            value: SaturationStepMode,
        ) -> Result<StepSaturationRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "stepMode", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(StepSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepSaturationRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn step_mode(
            mut self,
            value: SaturationStepMode,
        ) -> Result<StepSaturationRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(StepSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepSaturationRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn step_size(
            mut self,
            value: u8,
        ) -> Result<StepSaturationRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "stepSize", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepSize", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(StepSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepSaturationRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn step_size(
            mut self,
            value: u8,
        ) -> Result<StepSaturationRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepSize", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(StepSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepSaturationRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn transition_time(
            mut self,
            value: u8,
        ) -> Result<StepSaturationRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(StepSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepSaturationRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn transition_time(
            mut self,
            value: u8,
        ) -> Result<StepSaturationRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(StepSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepSaturationRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<StepSaturationRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(StepSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepSaturationRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<StepSaturationRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(StepSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepSaturationRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<StepSaturationRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(StepSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepSaturationRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<StepSaturationRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(StepSaturationRequestBuilder(self.0))
        }
    }
    impl<P> StepSaturationRequestBuilder<P, 5usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for StepSaturationRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "StepSaturationRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for StepSaturationRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "StepSaturationRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for StepSaturationRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for StepSaturationRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct StepSaturationRequestArrayBuilder<P>(P);

    impl<P> StepSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            StepSaturationRequestBuilder<StepSaturationRequestArrayBuilder<P>>,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                StepSaturationRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for StepSaturationRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "StepSaturationRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for StepSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "StepSaturationRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for StepSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for StepSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveToHueAndSaturationRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> MoveToHueAndSaturationRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToHueAndSaturationRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn hue(
            mut self,
            value: u8,
        ) -> Result<MoveToHueAndSaturationRequestBuilder<P, 1usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "hue", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "hue", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToHueAndSaturationRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn hue(
            mut self,
            value: u8,
        ) -> Result<MoveToHueAndSaturationRequestBuilder<P, 1usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "hue", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToHueAndSaturationRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn saturation(
            mut self,
            value: u8,
        ) -> Result<MoveToHueAndSaturationRequestBuilder<P, 2usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "saturation", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "saturation", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToHueAndSaturationRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn saturation(
            mut self,
            value: u8,
        ) -> Result<MoveToHueAndSaturationRequestBuilder<P, 2usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "saturation", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToHueAndSaturationRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<MoveToHueAndSaturationRequestBuilder<P, 3usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToHueAndSaturationRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<MoveToHueAndSaturationRequestBuilder<P, 3usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToHueAndSaturationRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveToHueAndSaturationRequestBuilder<P, 4usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToHueAndSaturationRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveToHueAndSaturationRequestBuilder<P, 4usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToHueAndSaturationRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveToHueAndSaturationRequestBuilder<P, 5usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(MoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToHueAndSaturationRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveToHueAndSaturationRequestBuilder<P, 5usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(MoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    impl<P> MoveToHueAndSaturationRequestBuilder<P, 5usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for MoveToHueAndSaturationRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveToHueAndSaturationRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format
        for MoveToHueAndSaturationRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(
                f,
                "{:?}::{}",
                self.0,
                "MoveToHueAndSaturationRequest"
            )
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent
        for MoveToHueAndSaturationRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveToHueAndSaturationRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveToHueAndSaturationRequestArrayBuilder<P>(P);

    impl<P> MoveToHueAndSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            MoveToHueAndSaturationRequestBuilder<MoveToHueAndSaturationRequestArrayBuilder<P>>,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                MoveToHueAndSaturationRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for MoveToHueAndSaturationRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveToHueAndSaturationRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for MoveToHueAndSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(
                f,
                "{:?}::{}",
                self.0,
                "MoveToHueAndSaturationRequest[]"
            )
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for MoveToHueAndSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveToHueAndSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveToColorRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> MoveToColorRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToColorRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn color_x(
            mut self,
            value: u16,
        ) -> Result<MoveToColorRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "colorX", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "colorX", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveToColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToColorRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn color_x(
            mut self,
            value: u16,
        ) -> Result<MoveToColorRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "colorX", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveToColorRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToColorRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn color_y(
            mut self,
            value: u16,
        ) -> Result<MoveToColorRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "colorY", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "colorY", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveToColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToColorRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn color_y(
            mut self,
            value: u16,
        ) -> Result<MoveToColorRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "colorY", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveToColorRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToColorRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<MoveToColorRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveToColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToColorRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<MoveToColorRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveToColorRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToColorRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveToColorRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveToColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToColorRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveToColorRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveToColorRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToColorRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveToColorRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(MoveToColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToColorRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveToColorRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(MoveToColorRequestBuilder(self.0))
        }
    }
    impl<P> MoveToColorRequestBuilder<P, 5usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for MoveToColorRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveToColorRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for MoveToColorRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "MoveToColorRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for MoveToColorRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveToColorRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveToColorRequestArrayBuilder<P>(P);

    impl<P> MoveToColorRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            MoveToColorRequestBuilder<MoveToColorRequestArrayBuilder<P>>,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                MoveToColorRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for MoveToColorRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveToColorRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for MoveToColorRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "MoveToColorRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for MoveToColorRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveToColorRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveColorRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> MoveColorRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveColorRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn rate_x(
            mut self,
            value: i16,
        ) -> Result<MoveColorRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "rateX", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "rateX", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveColorRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn rate_x(
            mut self,
            value: i16,
        ) -> Result<MoveColorRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "rateX", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveColorRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveColorRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn rate_y(
            mut self,
            value: i16,
        ) -> Result<MoveColorRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "rateY", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "rateY", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveColorRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn rate_y(
            mut self,
            value: i16,
        ) -> Result<MoveColorRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "rateY", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveColorRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveColorRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveColorRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveColorRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveColorRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveColorRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveColorRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveColorRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveColorRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveColorRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveColorRequestBuilder(self.0))
        }
    }
    impl<P> MoveColorRequestBuilder<P, 4usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for MoveColorRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveColorRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for MoveColorRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "MoveColorRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for MoveColorRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveColorRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveColorRequestArrayBuilder<P>(P);

    impl<P> MoveColorRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<MoveColorRequestBuilder<MoveColorRequestArrayBuilder<P>>, rs_matter::error::Error>
        {
            rs_matter::tlv::TLVBuilder::new(
                MoveColorRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for MoveColorRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveColorRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for MoveColorRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "MoveColorRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for MoveColorRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveColorRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct StepColorRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> StepColorRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepColorRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn step_x(
            mut self,
            value: i16,
        ) -> Result<StepColorRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "stepX", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepX", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(StepColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepColorRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn step_x(
            mut self,
            value: i16,
        ) -> Result<StepColorRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepX", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(StepColorRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepColorRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn step_y(
            mut self,
            value: i16,
        ) -> Result<StepColorRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "stepY", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepY", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(StepColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepColorRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn step_y(
            mut self,
            value: i16,
        ) -> Result<StepColorRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepY", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(StepColorRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepColorRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<StepColorRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(StepColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepColorRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<StepColorRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(StepColorRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepColorRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<StepColorRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(StepColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepColorRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<StepColorRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(StepColorRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepColorRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<StepColorRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(StepColorRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepColorRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<StepColorRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(StepColorRequestBuilder(self.0))
        }
    }
    impl<P> StepColorRequestBuilder<P, 5usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for StepColorRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "StepColorRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for StepColorRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "StepColorRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for StepColorRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for StepColorRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct StepColorRequestArrayBuilder<P>(P);

    impl<P> StepColorRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<StepColorRequestBuilder<StepColorRequestArrayBuilder<P>>, rs_matter::error::Error>
        {
            rs_matter::tlv::TLVBuilder::new(
                StepColorRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for StepColorRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "StepColorRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for StepColorRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "StepColorRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for StepColorRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for StepColorRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveToColorTemperatureRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> MoveToColorTemperatureRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToColorTemperatureRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn color_temperature_mireds(
            mut self,
            value: u16,
        ) -> Result<MoveToColorTemperatureRequestBuilder<P, 1usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMireds",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMireds",
                value
            );
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveToColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToColorTemperatureRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn color_temperature_mireds(
            mut self,
            value: u16,
        ) -> Result<MoveToColorTemperatureRequestBuilder<P, 1usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMireds",
                value
            );
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveToColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToColorTemperatureRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<MoveToColorTemperatureRequestBuilder<P, 2usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveToColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToColorTemperatureRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<MoveToColorTemperatureRequestBuilder<P, 2usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveToColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToColorTemperatureRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveToColorTemperatureRequestBuilder<P, 3usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveToColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToColorTemperatureRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveToColorTemperatureRequestBuilder<P, 3usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveToColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveToColorTemperatureRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveToColorTemperatureRequestBuilder<P, 4usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveToColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveToColorTemperatureRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveToColorTemperatureRequestBuilder<P, 4usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveToColorTemperatureRequestBuilder(self.0))
        }
    }
    impl<P> MoveToColorTemperatureRequestBuilder<P, 4usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for MoveToColorTemperatureRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveToColorTemperatureRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format
        for MoveToColorTemperatureRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(
                f,
                "{:?}::{}",
                self.0,
                "MoveToColorTemperatureRequest"
            )
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent
        for MoveToColorTemperatureRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveToColorTemperatureRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveToColorTemperatureRequestArrayBuilder<P>(P);

    impl<P> MoveToColorTemperatureRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            MoveToColorTemperatureRequestBuilder<MoveToColorTemperatureRequestArrayBuilder<P>>,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                MoveToColorTemperatureRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for MoveToColorTemperatureRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveToColorTemperatureRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for MoveToColorTemperatureRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(
                f,
                "{:?}::{}",
                self.0,
                "MoveToColorTemperatureRequest[]"
            )
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for MoveToColorTemperatureRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveToColorTemperatureRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct EnhancedMoveToHueRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> EnhancedMoveToHueRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveToHueRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn enhanced_hue(
            mut self,
            value: u16,
        ) -> Result<EnhancedMoveToHueRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "enhancedHue", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "enhancedHue", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveToHueRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn enhanced_hue(
            mut self,
            value: u16,
        ) -> Result<EnhancedMoveToHueRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "enhancedHue", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveToHueRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn direction(
            mut self,
            value: HueDirection,
        ) -> Result<EnhancedMoveToHueRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "direction", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "direction", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveToHueRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn direction(
            mut self,
            value: HueDirection,
        ) -> Result<EnhancedMoveToHueRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "direction", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveToHueRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<EnhancedMoveToHueRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveToHueRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<EnhancedMoveToHueRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveToHueRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveToHueRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveToHueRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveToHueRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveToHueRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveToHueRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveToHueRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveToHueRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueRequestBuilder(self.0))
        }
    }
    impl<P> EnhancedMoveToHueRequestBuilder<P, 5usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for EnhancedMoveToHueRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "EnhancedMoveToHueRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for EnhancedMoveToHueRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "EnhancedMoveToHueRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for EnhancedMoveToHueRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for EnhancedMoveToHueRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct EnhancedMoveToHueRequestArrayBuilder<P>(P);

    impl<P> EnhancedMoveToHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            EnhancedMoveToHueRequestBuilder<EnhancedMoveToHueRequestArrayBuilder<P>>,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                EnhancedMoveToHueRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for EnhancedMoveToHueRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "EnhancedMoveToHueRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for EnhancedMoveToHueRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "EnhancedMoveToHueRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for EnhancedMoveToHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for EnhancedMoveToHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct EnhancedMoveHueRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> EnhancedMoveHueRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveHueRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn move_mode(
            mut self,
            value: HueMoveMode,
        ) -> Result<EnhancedMoveHueRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "moveMode", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "moveMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveHueRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn move_mode(
            mut self,
            value: HueMoveMode,
        ) -> Result<EnhancedMoveHueRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "moveMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveHueRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn rate(
            mut self,
            value: u16,
        ) -> Result<EnhancedMoveHueRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "rate", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "rate", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveHueRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn rate(
            mut self,
            value: u16,
        ) -> Result<EnhancedMoveHueRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "rate", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveHueRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveHueRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveHueRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveHueRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveHueRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveHueRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveHueRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveHueRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveHueRequestBuilder(self.0))
        }
    }
    impl<P> EnhancedMoveHueRequestBuilder<P, 4usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for EnhancedMoveHueRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "EnhancedMoveHueRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for EnhancedMoveHueRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "EnhancedMoveHueRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for EnhancedMoveHueRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for EnhancedMoveHueRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct EnhancedMoveHueRequestArrayBuilder<P>(P);

    impl<P> EnhancedMoveHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            EnhancedMoveHueRequestBuilder<EnhancedMoveHueRequestArrayBuilder<P>>,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                EnhancedMoveHueRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for EnhancedMoveHueRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "EnhancedMoveHueRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for EnhancedMoveHueRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "EnhancedMoveHueRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for EnhancedMoveHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for EnhancedMoveHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct EnhancedStepHueRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> EnhancedStepHueRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedStepHueRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn step_mode(
            mut self,
            value: HueStepMode,
        ) -> Result<EnhancedStepHueRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "stepMode", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(EnhancedStepHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedStepHueRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn step_mode(
            mut self,
            value: HueStepMode,
        ) -> Result<EnhancedStepHueRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(EnhancedStepHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedStepHueRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn step_size(
            mut self,
            value: u16,
        ) -> Result<EnhancedStepHueRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "stepSize", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepSize", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(EnhancedStepHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedStepHueRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn step_size(
            mut self,
            value: u16,
        ) -> Result<EnhancedStepHueRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepSize", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(EnhancedStepHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedStepHueRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<EnhancedStepHueRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(EnhancedStepHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedStepHueRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<EnhancedStepHueRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(EnhancedStepHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedStepHueRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<EnhancedStepHueRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(EnhancedStepHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedStepHueRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<EnhancedStepHueRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(EnhancedStepHueRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedStepHueRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<EnhancedStepHueRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(EnhancedStepHueRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedStepHueRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<EnhancedStepHueRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(EnhancedStepHueRequestBuilder(self.0))
        }
    }
    impl<P> EnhancedStepHueRequestBuilder<P, 5usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for EnhancedStepHueRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "EnhancedStepHueRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for EnhancedStepHueRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "EnhancedStepHueRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for EnhancedStepHueRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for EnhancedStepHueRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct EnhancedStepHueRequestArrayBuilder<P>(P);

    impl<P> EnhancedStepHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            EnhancedStepHueRequestBuilder<EnhancedStepHueRequestArrayBuilder<P>>,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                EnhancedStepHueRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for EnhancedStepHueRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "EnhancedStepHueRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for EnhancedStepHueRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "EnhancedStepHueRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for EnhancedStepHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for EnhancedStepHueRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct EnhancedMoveToHueAndSaturationRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> EnhancedMoveToHueAndSaturationRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveToHueAndSaturationRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn enhanced_hue(
            mut self,
            value: u16,
        ) -> Result<EnhancedMoveToHueAndSaturationRequestBuilder<P, 1usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "enhancedHue", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "enhancedHue", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveToHueAndSaturationRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn enhanced_hue(
            mut self,
            value: u16,
        ) -> Result<EnhancedMoveToHueAndSaturationRequestBuilder<P, 1usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "enhancedHue", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveToHueAndSaturationRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn saturation(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveToHueAndSaturationRequestBuilder<P, 2usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "saturation", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "saturation", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveToHueAndSaturationRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn saturation(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveToHueAndSaturationRequestBuilder<P, 2usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "saturation", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveToHueAndSaturationRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<EnhancedMoveToHueAndSaturationRequestBuilder<P, 3usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveToHueAndSaturationRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<EnhancedMoveToHueAndSaturationRequestBuilder<P, 3usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveToHueAndSaturationRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveToHueAndSaturationRequestBuilder<P, 4usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveToHueAndSaturationRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveToHueAndSaturationRequestBuilder<P, 4usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> EnhancedMoveToHueAndSaturationRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveToHueAndSaturationRequestBuilder<P, 5usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> EnhancedMoveToHueAndSaturationRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<EnhancedMoveToHueAndSaturationRequestBuilder<P, 5usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(EnhancedMoveToHueAndSaturationRequestBuilder(self.0))
        }
    }
    impl<P> EnhancedMoveToHueAndSaturationRequestBuilder<P, 5usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for EnhancedMoveToHueAndSaturationRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(
                f,
                "{:?}::{}",
                self.0, "EnhancedMoveToHueAndSaturationRequest"
            )
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format
        for EnhancedMoveToHueAndSaturationRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(
                f,
                "{:?}::{}",
                self.0,
                "EnhancedMoveToHueAndSaturationRequest"
            )
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent
        for EnhancedMoveToHueAndSaturationRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for EnhancedMoveToHueAndSaturationRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct EnhancedMoveToHueAndSaturationRequestArrayBuilder<P>(P);

    impl<P> EnhancedMoveToHueAndSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            EnhancedMoveToHueAndSaturationRequestBuilder<
                EnhancedMoveToHueAndSaturationRequestArrayBuilder<P>,
            >,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                EnhancedMoveToHueAndSaturationRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for EnhancedMoveToHueAndSaturationRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(
                f,
                "{:?}::{}",
                self.0, "EnhancedMoveToHueAndSaturationRequest[]"
            )
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for EnhancedMoveToHueAndSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(
                f,
                "{:?}::{}",
                self.0,
                "EnhancedMoveToHueAndSaturationRequest[]"
            )
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for EnhancedMoveToHueAndSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for EnhancedMoveToHueAndSaturationRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct ColorLoopSetRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> ColorLoopSetRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> ColorLoopSetRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn update_flags(
            mut self,
            value: ColorLoopUpdateFlags,
        ) -> Result<ColorLoopSetRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "updateFlags", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "updateFlags", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> ColorLoopSetRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn update_flags(
            mut self,
            value: ColorLoopUpdateFlags,
        ) -> Result<ColorLoopSetRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "updateFlags", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> ColorLoopSetRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn action(
            mut self,
            value: ColorLoopAction,
        ) -> Result<ColorLoopSetRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "action", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "action", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> ColorLoopSetRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn action(
            mut self,
            value: ColorLoopAction,
        ) -> Result<ColorLoopSetRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "action", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> ColorLoopSetRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn direction(
            mut self,
            value: ColorLoopDirection,
        ) -> Result<ColorLoopSetRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "direction", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "direction", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> ColorLoopSetRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn direction(
            mut self,
            value: ColorLoopDirection,
        ) -> Result<ColorLoopSetRequestBuilder<P, 3usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "direction", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> ColorLoopSetRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn time(
            mut self,
            value: u16,
        ) -> Result<ColorLoopSetRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "time", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "time", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> ColorLoopSetRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn time(
            mut self,
            value: u16,
        ) -> Result<ColorLoopSetRequestBuilder<P, 4usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "time", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> ColorLoopSetRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn start_hue(
            mut self,
            value: u16,
        ) -> Result<ColorLoopSetRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "startHue", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "startHue", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> ColorLoopSetRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn start_hue(
            mut self,
            value: u16,
        ) -> Result<ColorLoopSetRequestBuilder<P, 5usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "startHue", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> ColorLoopSetRequestBuilder<P, 5>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<ColorLoopSetRequestBuilder<P, 6usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(5),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> ColorLoopSetRequestBuilder<P, 5>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<ColorLoopSetRequestBuilder<P, 6usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(5),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> ColorLoopSetRequestBuilder<P, 6>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<ColorLoopSetRequestBuilder<P, 7usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(6),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> ColorLoopSetRequestBuilder<P, 6>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<ColorLoopSetRequestBuilder<P, 7usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(6),
                self.0.writer(),
            )?;
            Ok(ColorLoopSetRequestBuilder(self.0))
        }
    }
    impl<P> ColorLoopSetRequestBuilder<P, 7usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for ColorLoopSetRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "ColorLoopSetRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for ColorLoopSetRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "ColorLoopSetRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for ColorLoopSetRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for ColorLoopSetRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct ColorLoopSetRequestArrayBuilder<P>(P);

    impl<P> ColorLoopSetRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            ColorLoopSetRequestBuilder<ColorLoopSetRequestArrayBuilder<P>>,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                ColorLoopSetRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for ColorLoopSetRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "ColorLoopSetRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for ColorLoopSetRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "ColorLoopSetRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for ColorLoopSetRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for ColorLoopSetRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct StopMoveStepRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> StopMoveStepRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StopMoveStepRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<StopMoveStepRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(StopMoveStepRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StopMoveStepRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<StopMoveStepRequestBuilder<P, 1usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(StopMoveStepRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StopMoveStepRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<StopMoveStepRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(StopMoveStepRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StopMoveStepRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<StopMoveStepRequestBuilder<P, 2usize>, rs_matter::error::Error> {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(StopMoveStepRequestBuilder(self.0))
        }
    }
    impl<P> StopMoveStepRequestBuilder<P, 2usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for StopMoveStepRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "StopMoveStepRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format for StopMoveStepRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "StopMoveStepRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent for StopMoveStepRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for StopMoveStepRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct StopMoveStepRequestArrayBuilder<P>(P);

    impl<P> StopMoveStepRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            StopMoveStepRequestBuilder<StopMoveStepRequestArrayBuilder<P>>,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                StopMoveStepRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for StopMoveStepRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "StopMoveStepRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for StopMoveStepRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "StopMoveStepRequest[]")
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for StopMoveStepRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for StopMoveStepRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveColorTemperatureRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> MoveColorTemperatureRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveColorTemperatureRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn move_mode(
            mut self,
            value: HueMoveMode,
        ) -> Result<MoveColorTemperatureRequestBuilder<P, 1usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "moveMode", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "moveMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveColorTemperatureRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn move_mode(
            mut self,
            value: HueMoveMode,
        ) -> Result<MoveColorTemperatureRequestBuilder<P, 1usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "moveMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(MoveColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveColorTemperatureRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn rate(
            mut self,
            value: u16,
        ) -> Result<MoveColorTemperatureRequestBuilder<P, 2usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "rate", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "rate", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveColorTemperatureRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn rate(
            mut self,
            value: u16,
        ) -> Result<MoveColorTemperatureRequestBuilder<P, 2usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "rate", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(MoveColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveColorTemperatureRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn color_temperature_minimum_mireds(
            mut self,
            value: u16,
        ) -> Result<MoveColorTemperatureRequestBuilder<P, 3usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMinimumMireds",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMinimumMireds",
                value
            );
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveColorTemperatureRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn color_temperature_minimum_mireds(
            mut self,
            value: u16,
        ) -> Result<MoveColorTemperatureRequestBuilder<P, 3usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMinimumMireds",
                value
            );
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(MoveColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveColorTemperatureRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn color_temperature_maximum_mireds(
            mut self,
            value: u16,
        ) -> Result<MoveColorTemperatureRequestBuilder<P, 4usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMaximumMireds",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMaximumMireds",
                value
            );
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveColorTemperatureRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn color_temperature_maximum_mireds(
            mut self,
            value: u16,
        ) -> Result<MoveColorTemperatureRequestBuilder<P, 4usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMaximumMireds",
                value
            );
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(MoveColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveColorTemperatureRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveColorTemperatureRequestBuilder<P, 5usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(MoveColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveColorTemperatureRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<MoveColorTemperatureRequestBuilder<P, 5usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(MoveColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> MoveColorTemperatureRequestBuilder<P, 5>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveColorTemperatureRequestBuilder<P, 6usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(5),
                self.0.writer(),
            )?;
            Ok(MoveColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> MoveColorTemperatureRequestBuilder<P, 5>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<MoveColorTemperatureRequestBuilder<P, 6usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(5),
                self.0.writer(),
            )?;
            Ok(MoveColorTemperatureRequestBuilder(self.0))
        }
    }
    impl<P> MoveColorTemperatureRequestBuilder<P, 6usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for MoveColorTemperatureRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveColorTemperatureRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format
        for MoveColorTemperatureRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "MoveColorTemperatureRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent
        for MoveColorTemperatureRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveColorTemperatureRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct MoveColorTemperatureRequestArrayBuilder<P>(P);

    impl<P> MoveColorTemperatureRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            MoveColorTemperatureRequestBuilder<MoveColorTemperatureRequestArrayBuilder<P>>,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                MoveColorTemperatureRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for MoveColorTemperatureRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "MoveColorTemperatureRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for MoveColorTemperatureRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(
                f,
                "{:?}::{}",
                self.0,
                "MoveColorTemperatureRequest[]"
            )
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for MoveColorTemperatureRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for MoveColorTemperatureRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct StepColorTemperatureRequestBuilder<P, const F: usize = 0usize>(P);

    impl<P> StepColorTemperatureRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_struct(tag)?;
            Ok(Self(parent))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepColorTemperatureRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn step_mode(
            mut self,
            value: HueStepMode,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 1usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "stepMode", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepColorTemperatureRequestBuilder<P, 0>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn step_mode(
            mut self,
            value: HueStepMode,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 1usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepMode", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(0),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepColorTemperatureRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn step_size(
            mut self,
            value: u16,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 2usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "stepSize", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepSize", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepColorTemperatureRequestBuilder<P, 1>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn step_size(
            mut self,
            value: u16,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 2usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "stepSize", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(1),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepColorTemperatureRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 3usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepColorTemperatureRequestBuilder<P, 2>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn transition_time(
            mut self,
            value: u16,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 3usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "transitionTime", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(2),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepColorTemperatureRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn color_temperature_minimum_mireds(
            mut self,
            value: u16,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 4usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMinimumMireds",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMinimumMireds",
                value
            );
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepColorTemperatureRequestBuilder<P, 3>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn color_temperature_minimum_mireds(
            mut self,
            value: u16,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 4usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMinimumMireds",
                value
            );
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(3),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepColorTemperatureRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn color_temperature_maximum_mireds(
            mut self,
            value: u16,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 5usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMaximumMireds",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMaximumMireds",
                value
            );
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepColorTemperatureRequestBuilder<P, 4>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn color_temperature_maximum_mireds(
            mut self,
            value: u16,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 5usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "colorTemperatureMaximumMireds",
                value
            );
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(4),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepColorTemperatureRequestBuilder<P, 5>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 6usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(5),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepColorTemperatureRequestBuilder<P, 5>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_mask(
            mut self,
            value: u8,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 6usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsMask", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(5),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> StepColorTemperatureRequestBuilder<P, 6>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug + rs_matter::reexport::defmt::Format,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 7usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "defmt")]
            rs_matter::reexport::defmt::debug!(
                "{:?}::{} -> {:?} +",
                self,
                "optionsOverride",
                value
            );
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(6),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    #[cfg(not(feature = "defmt"))]
    impl<P> StepColorTemperatureRequestBuilder<P, 6>
    where
        P: rs_matter::tlv::TLVBuilderParent + core::fmt::Debug,
    {
        pub fn options_override(
            mut self,
            value: u8,
        ) -> Result<StepColorTemperatureRequestBuilder<P, 7usize>, rs_matter::error::Error>
        {
            #[cfg(feature = "log")]
            rs_matter::reexport::log::debug!("{:?}::{} -> {:?} +", self, "optionsOverride", value);
            rs_matter::tlv::ToTLV::to_tlv(
                &value,
                &rs_matter::tlv::TLVTag::Context(6),
                self.0.writer(),
            )?;
            Ok(StepColorTemperatureRequestBuilder(self.0))
        }
    }
    impl<P> StepColorTemperatureRequestBuilder<P, 7usize>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Finish the struct and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P, const F: usize> core::fmt::Debug for StepColorTemperatureRequestBuilder<P, F>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "StepColorTemperatureRequest")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P, const F: usize> rs_matter::reexport::defmt::Format
        for StepColorTemperatureRequestBuilder<P, F>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "{:?}::{}", self.0, "StepColorTemperatureRequest")
        }
    }
    impl<P, const F: usize> rs_matter::tlv::TLVBuilderParent
        for StepColorTemperatureRequestBuilder<P, F>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for StepColorTemperatureRequestBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    pub struct StepColorTemperatureRequestArrayBuilder<P>(P);

    impl<P> StepColorTemperatureRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        #[doc = "Create a new instance"]
        pub fn new(
            mut parent: P,
            tag: &rs_matter::tlv::TLVTag,
        ) -> Result<Self, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            parent.writer().start_array(tag)?;
            Ok(Self(parent))
        }
        #[doc = "Push a new element into the array"]
        pub fn push(
            self,
        ) -> Result<
            StepColorTemperatureRequestBuilder<StepColorTemperatureRequestArrayBuilder<P>>,
            rs_matter::error::Error,
        > {
            rs_matter::tlv::TLVBuilder::new(
                StepColorTemperatureRequestArrayBuilder(self.0),
                &rs_matter::tlv::TLVTag::Anonymous,
            )
        }
        #[doc = "Finish the array and return the parent"]
        pub fn end(mut self) -> Result<P, rs_matter::error::Error> {
            use rs_matter::tlv::TLVWrite;
            self.0.writer().end_container()?;
            Ok(self.0)
        }
    }
    impl<P> core::fmt::Debug for StepColorTemperatureRequestArrayBuilder<P>
    where
        P: core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}::{}", self.0, "StepColorTemperatureRequest[]")
        }
    }
    #[cfg(feature = "defmt")]
    impl<P> rs_matter::reexport::defmt::Format for StepColorTemperatureRequestArrayBuilder<P>
    where
        P: rs_matter::reexport::defmt::Format,
    {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(
                f,
                "{:?}::{}",
                self.0,
                "StepColorTemperatureRequest[]"
            )
        }
    }
    impl<P> rs_matter::tlv::TLVBuilderParent for StepColorTemperatureRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        type Write = P::Write;
        fn writer(&mut self) -> &mut P::Write {
            self.0.writer()
        }
    }
    impl<P> rs_matter::tlv::TLVBuilder<P> for StepColorTemperatureRequestArrayBuilder<P>
    where
        P: rs_matter::tlv::TLVBuilderParent,
    {
        fn new(parent: P, tag: &rs_matter::tlv::TLVTag) -> Result<Self, rs_matter::error::Error> {
            Self::new(parent, tag)
        }
        fn unchecked_into_parent(self) -> P {
            self.0
        }
    }
    #[doc = "The attribute IDs for the cluster."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, rs_matter::reexport::strum::FromRepr)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u32)]
    pub enum AttributeId {
        CurrentHue = 0,
        CurrentSaturation = 1,
        RemainingTime = 2,
        CurrentX = 3,
        CurrentY = 4,
        DriftCompensation = 5,
        CompensationText = 6,
        ColorTemperatureMireds = 7,
        ColorMode = 8,
        Options = 15,
        NumberOfPrimaries = 16,
        Primary1X = 17,
        Primary1Y = 18,
        Primary1Intensity = 19,
        Primary2X = 21,
        Primary2Y = 22,
        Primary2Intensity = 23,
        Primary3X = 25,
        Primary3Y = 26,
        Primary3Intensity = 27,
        Primary4X = 32,
        Primary4Y = 33,
        Primary4Intensity = 34,
        Primary5X = 36,
        Primary5Y = 37,
        Primary5Intensity = 38,
        Primary6X = 40,
        Primary6Y = 41,
        Primary6Intensity = 42,
        WhitePointX = 48,
        WhitePointY = 49,
        ColorPointRX = 50,
        ColorPointRY = 51,
        ColorPointRIntensity = 52,
        ColorPointGX = 54,
        ColorPointGY = 55,
        ColorPointGIntensity = 56,
        ColorPointBX = 58,
        ColorPointBY = 59,
        ColorPointBIntensity = 60,
        EnhancedCurrentHue = 16384,
        EnhancedColorMode = 16385,
        ColorLoopActive = 16386,
        ColorLoopDirection = 16387,
        ColorLoopTime = 16388,
        ColorLoopStartEnhancedHue = 16389,
        ColorLoopStoredEnhancedHue = 16390,
        ColorCapabilities = 16394,
        ColorTempPhysicalMinMireds = 16395,
        ColorTempPhysicalMaxMireds = 16396,
        CoupleColorTempToLevelMinMireds = 16397,
        StartUpColorTemperatureMireds = 16400,
        GeneratedCommandList = 65528,
        AcceptedCommandList = 65529,
        EventList = 65530,
        AttributeList = 65531,
        FeatureMap = 65532,
        ClusterRevision = 65533,
    }
    impl core::convert::TryFrom<rs_matter::dm::AttrId> for AttributeId {
        type Error = rs_matter::error::Error;
        fn try_from(id: rs_matter::dm::CmdId) -> Result<Self, Self::Error> {
            AttributeId::from_repr(id)
                .ok_or_else(|| rs_matter::error::ErrorCode::AttributeNotFound.into())
        }
    }
    impl core::fmt::Debug for MetadataDebug<(AttributeId, bool)> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Attr::")?;
            match self.0 .0 {
                AttributeId::CurrentHue => write!(
                    f,
                    "{}(0x{:02x})",
                    "CurrentHue",
                    AttributeId::CurrentHue as u32
                )?,
                AttributeId::CurrentSaturation => write!(
                    f,
                    "{}(0x{:02x})",
                    "CurrentSaturation",
                    AttributeId::CurrentSaturation as u32
                )?,
                AttributeId::RemainingTime => write!(
                    f,
                    "{}(0x{:02x})",
                    "RemainingTime",
                    AttributeId::RemainingTime as u32
                )?,
                AttributeId::CurrentX => {
                    write!(f, "{}(0x{:02x})", "CurrentX", AttributeId::CurrentX as u32)?
                }
                AttributeId::CurrentY => {
                    write!(f, "{}(0x{:02x})", "CurrentY", AttributeId::CurrentY as u32)?
                }
                AttributeId::DriftCompensation => write!(
                    f,
                    "{}(0x{:02x})",
                    "DriftCompensation",
                    AttributeId::DriftCompensation as u32
                )?,
                AttributeId::CompensationText => write!(
                    f,
                    "{}(0x{:02x})",
                    "CompensationText",
                    AttributeId::CompensationText as u32
                )?,
                AttributeId::ColorTemperatureMireds => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorTemperatureMireds",
                    AttributeId::ColorTemperatureMireds as u32
                )?,
                AttributeId::ColorMode => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorMode",
                    AttributeId::ColorMode as u32
                )?,
                AttributeId::Options => {
                    write!(f, "{}(0x{:02x})", "Options", AttributeId::Options as u32)?
                }
                AttributeId::NumberOfPrimaries => write!(
                    f,
                    "{}(0x{:02x})",
                    "NumberOfPrimaries",
                    AttributeId::NumberOfPrimaries as u32
                )?,
                AttributeId::Primary1X => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary1X",
                    AttributeId::Primary1X as u32
                )?,
                AttributeId::Primary1Y => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary1Y",
                    AttributeId::Primary1Y as u32
                )?,
                AttributeId::Primary1Intensity => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary1Intensity",
                    AttributeId::Primary1Intensity as u32
                )?,
                AttributeId::Primary2X => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary2X",
                    AttributeId::Primary2X as u32
                )?,
                AttributeId::Primary2Y => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary2Y",
                    AttributeId::Primary2Y as u32
                )?,
                AttributeId::Primary2Intensity => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary2Intensity",
                    AttributeId::Primary2Intensity as u32
                )?,
                AttributeId::Primary3X => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary3X",
                    AttributeId::Primary3X as u32
                )?,
                AttributeId::Primary3Y => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary3Y",
                    AttributeId::Primary3Y as u32
                )?,
                AttributeId::Primary3Intensity => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary3Intensity",
                    AttributeId::Primary3Intensity as u32
                )?,
                AttributeId::Primary4X => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary4X",
                    AttributeId::Primary4X as u32
                )?,
                AttributeId::Primary4Y => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary4Y",
                    AttributeId::Primary4Y as u32
                )?,
                AttributeId::Primary4Intensity => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary4Intensity",
                    AttributeId::Primary4Intensity as u32
                )?,
                AttributeId::Primary5X => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary5X",
                    AttributeId::Primary5X as u32
                )?,
                AttributeId::Primary5Y => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary5Y",
                    AttributeId::Primary5Y as u32
                )?,
                AttributeId::Primary5Intensity => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary5Intensity",
                    AttributeId::Primary5Intensity as u32
                )?,
                AttributeId::Primary6X => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary6X",
                    AttributeId::Primary6X as u32
                )?,
                AttributeId::Primary6Y => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary6Y",
                    AttributeId::Primary6Y as u32
                )?,
                AttributeId::Primary6Intensity => write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary6Intensity",
                    AttributeId::Primary6Intensity as u32
                )?,
                AttributeId::WhitePointX => write!(
                    f,
                    "{}(0x{:02x})",
                    "WhitePointX",
                    AttributeId::WhitePointX as u32
                )?,
                AttributeId::WhitePointY => write!(
                    f,
                    "{}(0x{:02x})",
                    "WhitePointY",
                    AttributeId::WhitePointY as u32
                )?,
                AttributeId::ColorPointRX => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointRX",
                    AttributeId::ColorPointRX as u32
                )?,
                AttributeId::ColorPointRY => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointRY",
                    AttributeId::ColorPointRY as u32
                )?,
                AttributeId::ColorPointRIntensity => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointRIntensity",
                    AttributeId::ColorPointRIntensity as u32
                )?,
                AttributeId::ColorPointGX => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointGX",
                    AttributeId::ColorPointGX as u32
                )?,
                AttributeId::ColorPointGY => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointGY",
                    AttributeId::ColorPointGY as u32
                )?,
                AttributeId::ColorPointGIntensity => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointGIntensity",
                    AttributeId::ColorPointGIntensity as u32
                )?,
                AttributeId::ColorPointBX => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointBX",
                    AttributeId::ColorPointBX as u32
                )?,
                AttributeId::ColorPointBY => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointBY",
                    AttributeId::ColorPointBY as u32
                )?,
                AttributeId::ColorPointBIntensity => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointBIntensity",
                    AttributeId::ColorPointBIntensity as u32
                )?,
                AttributeId::EnhancedCurrentHue => write!(
                    f,
                    "{}(0x{:02x})",
                    "EnhancedCurrentHue",
                    AttributeId::EnhancedCurrentHue as u32
                )?,
                AttributeId::EnhancedColorMode => write!(
                    f,
                    "{}(0x{:02x})",
                    "EnhancedColorMode",
                    AttributeId::EnhancedColorMode as u32
                )?,
                AttributeId::ColorLoopActive => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorLoopActive",
                    AttributeId::ColorLoopActive as u32
                )?,
                AttributeId::ColorLoopDirection => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorLoopDirection",
                    AttributeId::ColorLoopDirection as u32
                )?,
                AttributeId::ColorLoopTime => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorLoopTime",
                    AttributeId::ColorLoopTime as u32
                )?,
                AttributeId::ColorLoopStartEnhancedHue => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorLoopStartEnhancedHue",
                    AttributeId::ColorLoopStartEnhancedHue as u32
                )?,
                AttributeId::ColorLoopStoredEnhancedHue => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorLoopStoredEnhancedHue",
                    AttributeId::ColorLoopStoredEnhancedHue as u32
                )?,
                AttributeId::ColorCapabilities => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorCapabilities",
                    AttributeId::ColorCapabilities as u32
                )?,
                AttributeId::ColorTempPhysicalMinMireds => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorTempPhysicalMinMireds",
                    AttributeId::ColorTempPhysicalMinMireds as u32
                )?,
                AttributeId::ColorTempPhysicalMaxMireds => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorTempPhysicalMaxMireds",
                    AttributeId::ColorTempPhysicalMaxMireds as u32
                )?,
                AttributeId::CoupleColorTempToLevelMinMireds => write!(
                    f,
                    "{}(0x{:02x})",
                    "CoupleColorTempToLevelMinMireds",
                    AttributeId::CoupleColorTempToLevelMinMireds as u32
                )?,
                AttributeId::StartUpColorTemperatureMireds => write!(
                    f,
                    "{}(0x{:02x})",
                    "StartUpColorTemperatureMireds",
                    AttributeId::StartUpColorTemperatureMireds as u32
                )?,
                AttributeId::GeneratedCommandList => write!(
                    f,
                    "{}(0x{:02x})",
                    "GeneratedCommandList",
                    AttributeId::GeneratedCommandList as u32
                )?,
                AttributeId::AcceptedCommandList => write!(
                    f,
                    "{}(0x{:02x})",
                    "AcceptedCommandList",
                    AttributeId::AcceptedCommandList as u32
                )?,
                AttributeId::EventList => write!(
                    f,
                    "{}(0x{:02x})",
                    "EventList",
                    AttributeId::EventList as u32
                )?,
                AttributeId::AttributeList => write!(
                    f,
                    "{}(0x{:02x})",
                    "AttributeList",
                    AttributeId::AttributeList as u32
                )?,
                AttributeId::FeatureMap => write!(
                    f,
                    "{}(0x{:02x})",
                    "FeatureMap",
                    AttributeId::FeatureMap as u32
                )?,
                AttributeId::ClusterRevision => write!(
                    f,
                    "{}(0x{:02x})",
                    "ClusterRevision",
                    AttributeId::ClusterRevision as u32
                )?,
            }
            if self.0 .1 {
                write!(f, "::Write")
            } else {
                write!(f, "::Read")
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for MetadataDebug<(AttributeId, bool)> {
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "Attr::");
            match self.0 .0 {
                AttributeId::CurrentHue => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "CurrentHue",
                    AttributeId::CurrentHue as u32
                ),
                AttributeId::CurrentSaturation => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "CurrentSaturation",
                    AttributeId::CurrentSaturation as u32
                ),
                AttributeId::RemainingTime => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "RemainingTime",
                    AttributeId::RemainingTime as u32
                ),
                AttributeId::CurrentX => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "CurrentX",
                    AttributeId::CurrentX as u32
                ),
                AttributeId::CurrentY => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "CurrentY",
                    AttributeId::CurrentY as u32
                ),
                AttributeId::DriftCompensation => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "DriftCompensation",
                    AttributeId::DriftCompensation as u32
                ),
                AttributeId::CompensationText => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "CompensationText",
                    AttributeId::CompensationText as u32
                ),
                AttributeId::ColorTemperatureMireds => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorTemperatureMireds",
                    AttributeId::ColorTemperatureMireds as u32
                ),
                AttributeId::ColorMode => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorMode",
                    AttributeId::ColorMode as u32
                ),
                AttributeId::Options => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Options",
                    AttributeId::Options as u32
                ),
                AttributeId::NumberOfPrimaries => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "NumberOfPrimaries",
                    AttributeId::NumberOfPrimaries as u32
                ),
                AttributeId::Primary1X => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary1X",
                    AttributeId::Primary1X as u32
                ),
                AttributeId::Primary1Y => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary1Y",
                    AttributeId::Primary1Y as u32
                ),
                AttributeId::Primary1Intensity => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary1Intensity",
                    AttributeId::Primary1Intensity as u32
                ),
                AttributeId::Primary2X => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary2X",
                    AttributeId::Primary2X as u32
                ),
                AttributeId::Primary2Y => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary2Y",
                    AttributeId::Primary2Y as u32
                ),
                AttributeId::Primary2Intensity => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary2Intensity",
                    AttributeId::Primary2Intensity as u32
                ),
                AttributeId::Primary3X => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary3X",
                    AttributeId::Primary3X as u32
                ),
                AttributeId::Primary3Y => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary3Y",
                    AttributeId::Primary3Y as u32
                ),
                AttributeId::Primary3Intensity => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary3Intensity",
                    AttributeId::Primary3Intensity as u32
                ),
                AttributeId::Primary4X => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary4X",
                    AttributeId::Primary4X as u32
                ),
                AttributeId::Primary4Y => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary4Y",
                    AttributeId::Primary4Y as u32
                ),
                AttributeId::Primary4Intensity => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary4Intensity",
                    AttributeId::Primary4Intensity as u32
                ),
                AttributeId::Primary5X => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary5X",
                    AttributeId::Primary5X as u32
                ),
                AttributeId::Primary5Y => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary5Y",
                    AttributeId::Primary5Y as u32
                ),
                AttributeId::Primary5Intensity => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary5Intensity",
                    AttributeId::Primary5Intensity as u32
                ),
                AttributeId::Primary6X => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary6X",
                    AttributeId::Primary6X as u32
                ),
                AttributeId::Primary6Y => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary6Y",
                    AttributeId::Primary6Y as u32
                ),
                AttributeId::Primary6Intensity => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "Primary6Intensity",
                    AttributeId::Primary6Intensity as u32
                ),
                AttributeId::WhitePointX => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "WhitePointX",
                    AttributeId::WhitePointX as u32
                ),
                AttributeId::WhitePointY => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "WhitePointY",
                    AttributeId::WhitePointY as u32
                ),
                AttributeId::ColorPointRX => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointRX",
                    AttributeId::ColorPointRX as u32
                ),
                AttributeId::ColorPointRY => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointRY",
                    AttributeId::ColorPointRY as u32
                ),
                AttributeId::ColorPointRIntensity => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointRIntensity",
                    AttributeId::ColorPointRIntensity as u32
                ),
                AttributeId::ColorPointGX => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointGX",
                    AttributeId::ColorPointGX as u32
                ),
                AttributeId::ColorPointGY => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointGY",
                    AttributeId::ColorPointGY as u32
                ),
                AttributeId::ColorPointGIntensity => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointGIntensity",
                    AttributeId::ColorPointGIntensity as u32
                ),
                AttributeId::ColorPointBX => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointBX",
                    AttributeId::ColorPointBX as u32
                ),
                AttributeId::ColorPointBY => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointBY",
                    AttributeId::ColorPointBY as u32
                ),
                AttributeId::ColorPointBIntensity => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorPointBIntensity",
                    AttributeId::ColorPointBIntensity as u32
                ),
                AttributeId::EnhancedCurrentHue => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "EnhancedCurrentHue",
                    AttributeId::EnhancedCurrentHue as u32
                ),
                AttributeId::EnhancedColorMode => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "EnhancedColorMode",
                    AttributeId::EnhancedColorMode as u32
                ),
                AttributeId::ColorLoopActive => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorLoopActive",
                    AttributeId::ColorLoopActive as u32
                ),
                AttributeId::ColorLoopDirection => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorLoopDirection",
                    AttributeId::ColorLoopDirection as u32
                ),
                AttributeId::ColorLoopTime => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorLoopTime",
                    AttributeId::ColorLoopTime as u32
                ),
                AttributeId::ColorLoopStartEnhancedHue => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorLoopStartEnhancedHue",
                    AttributeId::ColorLoopStartEnhancedHue as u32
                ),
                AttributeId::ColorLoopStoredEnhancedHue => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorLoopStoredEnhancedHue",
                    AttributeId::ColorLoopStoredEnhancedHue as u32
                ),
                AttributeId::ColorCapabilities => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorCapabilities",
                    AttributeId::ColorCapabilities as u32
                ),
                AttributeId::ColorTempPhysicalMinMireds => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorTempPhysicalMinMireds",
                    AttributeId::ColorTempPhysicalMinMireds as u32
                ),
                AttributeId::ColorTempPhysicalMaxMireds => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorTempPhysicalMaxMireds",
                    AttributeId::ColorTempPhysicalMaxMireds as u32
                ),
                AttributeId::CoupleColorTempToLevelMinMireds => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "CoupleColorTempToLevelMinMireds",
                    AttributeId::CoupleColorTempToLevelMinMireds as u32
                ),
                AttributeId::StartUpColorTemperatureMireds => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "StartUpColorTemperatureMireds",
                    AttributeId::StartUpColorTemperatureMireds as u32
                ),
                AttributeId::GeneratedCommandList => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "GeneratedCommandList",
                    AttributeId::GeneratedCommandList as u32
                ),
                AttributeId::AcceptedCommandList => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "AcceptedCommandList",
                    AttributeId::AcceptedCommandList as u32
                ),
                AttributeId::EventList => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "EventList",
                    AttributeId::EventList as u32
                ),
                AttributeId::AttributeList => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "AttributeList",
                    AttributeId::AttributeList as u32
                ),
                AttributeId::FeatureMap => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "FeatureMap",
                    AttributeId::FeatureMap as u32
                ),
                AttributeId::ClusterRevision => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ClusterRevision",
                    AttributeId::ClusterRevision as u32
                ),
            }
            if self.0 .1 {
                rs_matter::reexport::defmt::write!(f, "::Write")
            } else {
                rs_matter::reexport::defmt::write!(f, "::Read")
            }
        }
    }
    #[doc = "The command IDs for the cluster."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, rs_matter::reexport::strum::FromRepr)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    #[repr(u32)]
    pub enum CommandId {
        MoveToHue = 0,
        MoveHue = 1,
        StepHue = 2,
        MoveToSaturation = 3,
        MoveSaturation = 4,
        StepSaturation = 5,
        MoveToHueAndSaturation = 6,
        MoveToColor = 7,
        MoveColor = 8,
        StepColor = 9,
        MoveToColorTemperature = 10,
        EnhancedMoveToHue = 64,
        EnhancedMoveHue = 65,
        EnhancedStepHue = 66,
        EnhancedMoveToHueAndSaturation = 67,
        ColorLoopSet = 68,
        StopMoveStep = 71,
        MoveColorTemperature = 75,
        StepColorTemperature = 76,
    }
    impl core::convert::TryFrom<rs_matter::dm::CmdId> for CommandId {
        type Error = rs_matter::error::Error;
        fn try_from(id: rs_matter::dm::CmdId) -> Result<Self, Self::Error> {
            CommandId::from_repr(id)
                .ok_or_else(|| rs_matter::error::ErrorCode::CommandNotFound.into())
        }
    }
    impl core::fmt::Debug for MetadataDebug<CommandId> {
        #[allow(unreachable_code)]
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Cmd::")?;
            match self.0 {
                CommandId::MoveToHue => {
                    write!(f, "{}(0x{:02x})", "MoveToHue", CommandId::MoveToHue as u32)?
                }
                CommandId::MoveHue => {
                    write!(f, "{}(0x{:02x})", "MoveHue", CommandId::MoveHue as u32)?
                }
                CommandId::StepHue => {
                    write!(f, "{}(0x{:02x})", "StepHue", CommandId::StepHue as u32)?
                }
                CommandId::MoveToSaturation => write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveToSaturation",
                    CommandId::MoveToSaturation as u32
                )?,
                CommandId::MoveSaturation => write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveSaturation",
                    CommandId::MoveSaturation as u32
                )?,
                CommandId::StepSaturation => write!(
                    f,
                    "{}(0x{:02x})",
                    "StepSaturation",
                    CommandId::StepSaturation as u32
                )?,
                CommandId::MoveToHueAndSaturation => write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveToHueAndSaturation",
                    CommandId::MoveToHueAndSaturation as u32
                )?,
                CommandId::MoveToColor => write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveToColor",
                    CommandId::MoveToColor as u32
                )?,
                CommandId::MoveColor => {
                    write!(f, "{}(0x{:02x})", "MoveColor", CommandId::MoveColor as u32)?
                }
                CommandId::StepColor => {
                    write!(f, "{}(0x{:02x})", "StepColor", CommandId::StepColor as u32)?
                }
                CommandId::MoveToColorTemperature => write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveToColorTemperature",
                    CommandId::MoveToColorTemperature as u32
                )?,
                CommandId::EnhancedMoveToHue => write!(
                    f,
                    "{}(0x{:02x})",
                    "EnhancedMoveToHue",
                    CommandId::EnhancedMoveToHue as u32
                )?,
                CommandId::EnhancedMoveHue => write!(
                    f,
                    "{}(0x{:02x})",
                    "EnhancedMoveHue",
                    CommandId::EnhancedMoveHue as u32
                )?,
                CommandId::EnhancedStepHue => write!(
                    f,
                    "{}(0x{:02x})",
                    "EnhancedStepHue",
                    CommandId::EnhancedStepHue as u32
                )?,
                CommandId::EnhancedMoveToHueAndSaturation => write!(
                    f,
                    "{}(0x{:02x})",
                    "EnhancedMoveToHueAndSaturation",
                    CommandId::EnhancedMoveToHueAndSaturation as u32
                )?,
                CommandId::ColorLoopSet => write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorLoopSet",
                    CommandId::ColorLoopSet as u32
                )?,
                CommandId::StopMoveStep => write!(
                    f,
                    "{}(0x{:02x})",
                    "StopMoveStep",
                    CommandId::StopMoveStep as u32
                )?,
                CommandId::MoveColorTemperature => write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveColorTemperature",
                    CommandId::MoveColorTemperature as u32
                )?,
                CommandId::StepColorTemperature => write!(
                    f,
                    "{}(0x{:02x})",
                    "StepColorTemperature",
                    CommandId::StepColorTemperature as u32
                )?,
            }
            write!(f, "::Invoke")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for MetadataDebug<CommandId> {
        #[allow(unreachable_code)]
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "Cmd::");
            match self.0 {
                CommandId::MoveToHue => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveToHue",
                    CommandId::MoveToHue as u32
                ),
                CommandId::MoveHue => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveHue",
                    CommandId::MoveHue as u32
                ),
                CommandId::StepHue => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "StepHue",
                    CommandId::StepHue as u32
                ),
                CommandId::MoveToSaturation => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveToSaturation",
                    CommandId::MoveToSaturation as u32
                ),
                CommandId::MoveSaturation => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveSaturation",
                    CommandId::MoveSaturation as u32
                ),
                CommandId::StepSaturation => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "StepSaturation",
                    CommandId::StepSaturation as u32
                ),
                CommandId::MoveToHueAndSaturation => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveToHueAndSaturation",
                    CommandId::MoveToHueAndSaturation as u32
                ),
                CommandId::MoveToColor => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveToColor",
                    CommandId::MoveToColor as u32
                ),
                CommandId::MoveColor => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveColor",
                    CommandId::MoveColor as u32
                ),
                CommandId::StepColor => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "StepColor",
                    CommandId::StepColor as u32
                ),
                CommandId::MoveToColorTemperature => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveToColorTemperature",
                    CommandId::MoveToColorTemperature as u32
                ),
                CommandId::EnhancedMoveToHue => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "EnhancedMoveToHue",
                    CommandId::EnhancedMoveToHue as u32
                ),
                CommandId::EnhancedMoveHue => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "EnhancedMoveHue",
                    CommandId::EnhancedMoveHue as u32
                ),
                CommandId::EnhancedStepHue => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "EnhancedStepHue",
                    CommandId::EnhancedStepHue as u32
                ),
                CommandId::EnhancedMoveToHueAndSaturation => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "EnhancedMoveToHueAndSaturation",
                    CommandId::EnhancedMoveToHueAndSaturation as u32
                ),
                CommandId::ColorLoopSet => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "ColorLoopSet",
                    CommandId::ColorLoopSet as u32
                ),
                CommandId::StopMoveStep => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "StopMoveStep",
                    CommandId::StopMoveStep as u32
                ),
                CommandId::MoveColorTemperature => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "MoveColorTemperature",
                    CommandId::MoveColorTemperature as u32
                ),
                CommandId::StepColorTemperature => rs_matter::reexport::defmt::write!(
                    f,
                    "{}(0x{:02x})",
                    "StepColorTemperature",
                    CommandId::StepColorTemperature as u32
                ),
            }
            rs_matter::reexport::defmt::write!(f, "::Invoke")
        }
    }
    #[doc = "The command response IDs for the cluster."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, rs_matter::reexport::strum::FromRepr)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    pub enum CommandResponseId {}

    impl core::fmt::Debug for MetadataDebug<CommandResponseId> {
        #[allow(unreachable_code)]
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Cmd::")?;
            match self.0 {}
            write!(f, "::Response")
        }
    }
    #[cfg(feature = "defmt")]
    impl rs_matter::reexport::defmt::Format for MetadataDebug<CommandResponseId> {
        #[allow(unreachable_code)]
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(f, "Cmd::");
            match self.0 {}
            rs_matter::reexport::defmt::write!(f, "::Response")
        }
    }
    impl core::convert::TryFrom<rs_matter::dm::CmdId> for CommandResponseId {
        type Error = rs_matter::error::Error;
        fn try_from(id: rs_matter::dm::CmdId) -> Result<Self, Self::Error> {
            Err(rs_matter::error::ErrorCode::CommandNotFound.into())
        }
    }
    #[doc = "The cluster metadata. By default, all cluster attributes and commands are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure."]
    pub const FULL_CLUSTER: rs_matter::dm::Cluster<'static> = rs_matter::dm::Cluster::new(
        768,
        6,
        0,
        &[
            rs_matter::dm::Attribute::new(
                AttributeId::CurrentHue as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::CurrentSaturation as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::RemainingTime as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::CurrentX as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::CurrentY as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::DriftCompensation as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::CompensationText as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorTemperatureMireds as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorMode as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::NONE,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Options as _,
                rs_matter::dm::Access::READ
                    .union(rs_matter::dm::Access::WRITE)
                    .union(
                        rs_matter::dm::Access::NEED_OPERATE
                            .union(
                                rs_matter::dm::Access::NEED_MANAGE
                                    .union(rs_matter::dm::Access::NEED_ADMIN),
                            )
                            .union(rs_matter::dm::Access::NEED_VIEW),
                    ),
                rs_matter::dm::Quality::NONE,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::NumberOfPrimaries as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::NONE,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary1X as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary1Y as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary1Intensity as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary2X as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary2Y as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary2Intensity as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary3X as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary3Y as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary3Intensity as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary4X as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary4Y as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary4Intensity as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary5X as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary5Y as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary5Intensity as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary6X as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary6Y as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::Primary6Intensity as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::WhitePointX as _,
                rs_matter::dm::Access::READ
                    .union(rs_matter::dm::Access::WRITE)
                    .union(
                        rs_matter::dm::Access::NEED_MANAGE
                            .union(rs_matter::dm::Access::NEED_ADMIN)
                            .union(rs_matter::dm::Access::NEED_VIEW),
                    ),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::WhitePointY as _,
                rs_matter::dm::Access::READ
                    .union(rs_matter::dm::Access::WRITE)
                    .union(
                        rs_matter::dm::Access::NEED_MANAGE
                            .union(rs_matter::dm::Access::NEED_ADMIN)
                            .union(rs_matter::dm::Access::NEED_VIEW),
                    ),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorPointRX as _,
                rs_matter::dm::Access::READ
                    .union(rs_matter::dm::Access::WRITE)
                    .union(
                        rs_matter::dm::Access::NEED_MANAGE
                            .union(rs_matter::dm::Access::NEED_ADMIN)
                            .union(rs_matter::dm::Access::NEED_VIEW),
                    ),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorPointRY as _,
                rs_matter::dm::Access::READ
                    .union(rs_matter::dm::Access::WRITE)
                    .union(
                        rs_matter::dm::Access::NEED_MANAGE
                            .union(rs_matter::dm::Access::NEED_ADMIN)
                            .union(rs_matter::dm::Access::NEED_VIEW),
                    ),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorPointRIntensity as _,
                rs_matter::dm::Access::READ
                    .union(rs_matter::dm::Access::WRITE)
                    .union(
                        rs_matter::dm::Access::NEED_MANAGE
                            .union(rs_matter::dm::Access::NEED_ADMIN)
                            .union(rs_matter::dm::Access::NEED_VIEW),
                    ),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorPointGX as _,
                rs_matter::dm::Access::READ
                    .union(rs_matter::dm::Access::WRITE)
                    .union(
                        rs_matter::dm::Access::NEED_MANAGE
                            .union(rs_matter::dm::Access::NEED_ADMIN)
                            .union(rs_matter::dm::Access::NEED_VIEW),
                    ),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorPointGY as _,
                rs_matter::dm::Access::READ
                    .union(rs_matter::dm::Access::WRITE)
                    .union(
                        rs_matter::dm::Access::NEED_MANAGE
                            .union(rs_matter::dm::Access::NEED_ADMIN)
                            .union(rs_matter::dm::Access::NEED_VIEW),
                    ),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorPointGIntensity as _,
                rs_matter::dm::Access::READ
                    .union(rs_matter::dm::Access::WRITE)
                    .union(
                        rs_matter::dm::Access::NEED_MANAGE
                            .union(rs_matter::dm::Access::NEED_ADMIN)
                            .union(rs_matter::dm::Access::NEED_VIEW),
                    ),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorPointBX as _,
                rs_matter::dm::Access::READ
                    .union(rs_matter::dm::Access::WRITE)
                    .union(
                        rs_matter::dm::Access::NEED_MANAGE
                            .union(rs_matter::dm::Access::NEED_ADMIN)
                            .union(rs_matter::dm::Access::NEED_VIEW),
                    ),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorPointBY as _,
                rs_matter::dm::Access::READ
                    .union(rs_matter::dm::Access::WRITE)
                    .union(
                        rs_matter::dm::Access::NEED_MANAGE
                            .union(rs_matter::dm::Access::NEED_ADMIN)
                            .union(rs_matter::dm::Access::NEED_VIEW),
                    ),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorPointBIntensity as _,
                rs_matter::dm::Access::READ
                    .union(rs_matter::dm::Access::WRITE)
                    .union(
                        rs_matter::dm::Access::NEED_MANAGE
                            .union(rs_matter::dm::Access::NEED_ADMIN)
                            .union(rs_matter::dm::Access::NEED_VIEW),
                    ),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::EnhancedCurrentHue as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::EnhancedColorMode as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::NONE,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorLoopActive as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorLoopDirection as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorLoopTime as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorLoopStartEnhancedHue as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorLoopStoredEnhancedHue as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorCapabilities as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::NONE,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorTempPhysicalMinMireds as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ColorTempPhysicalMaxMireds as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::CoupleColorTempToLevelMinMireds as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::StartUpColorTemperatureMireds as _,
                rs_matter::dm::Access::READ
                    .union(rs_matter::dm::Access::WRITE)
                    .union(
                        rs_matter::dm::Access::NEED_MANAGE
                            .union(rs_matter::dm::Access::NEED_ADMIN)
                            .union(rs_matter::dm::Access::NEED_VIEW),
                    ),
                rs_matter::dm::Quality::O,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::GeneratedCommandList as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::A,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::AcceptedCommandList as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::A,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::EventList as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::A,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::AttributeList as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::A,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::FeatureMap as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::NONE,
            ),
            rs_matter::dm::Attribute::new(
                AttributeId::ClusterRevision as _,
                rs_matter::dm::Access::READ.union(rs_matter::dm::Access::NEED_VIEW),
                rs_matter::dm::Quality::NONE,
            ),
        ],
        &[
            rs_matter::dm::Command::new(CommandId::MoveToHue as _, None, rs_matter::dm::Access::WO),
            rs_matter::dm::Command::new(CommandId::MoveHue as _, None, rs_matter::dm::Access::WO),
            rs_matter::dm::Command::new(CommandId::StepHue as _, None, rs_matter::dm::Access::WO),
            rs_matter::dm::Command::new(
                CommandId::MoveToSaturation as _,
                None,
                rs_matter::dm::Access::WO,
            ),
            rs_matter::dm::Command::new(
                CommandId::MoveSaturation as _,
                None,
                rs_matter::dm::Access::WO,
            ),
            rs_matter::dm::Command::new(
                CommandId::StepSaturation as _,
                None,
                rs_matter::dm::Access::WO,
            ),
            rs_matter::dm::Command::new(
                CommandId::MoveToHueAndSaturation as _,
                None,
                rs_matter::dm::Access::WO,
            ),
            rs_matter::dm::Command::new(
                CommandId::MoveToColor as _,
                None,
                rs_matter::dm::Access::WO,
            ),
            rs_matter::dm::Command::new(CommandId::MoveColor as _, None, rs_matter::dm::Access::WO),
            rs_matter::dm::Command::new(CommandId::StepColor as _, None, rs_matter::dm::Access::WO),
            rs_matter::dm::Command::new(
                CommandId::MoveToColorTemperature as _,
                None,
                rs_matter::dm::Access::WO,
            ),
            rs_matter::dm::Command::new(
                CommandId::EnhancedMoveToHue as _,
                None,
                rs_matter::dm::Access::WO,
            ),
            rs_matter::dm::Command::new(
                CommandId::EnhancedMoveHue as _,
                None,
                rs_matter::dm::Access::WO,
            ),
            rs_matter::dm::Command::new(
                CommandId::EnhancedStepHue as _,
                None,
                rs_matter::dm::Access::WO,
            ),
            rs_matter::dm::Command::new(
                CommandId::EnhancedMoveToHueAndSaturation as _,
                None,
                rs_matter::dm::Access::WO,
            ),
            rs_matter::dm::Command::new(
                CommandId::ColorLoopSet as _,
                None,
                rs_matter::dm::Access::WO,
            ),
            rs_matter::dm::Command::new(
                CommandId::StopMoveStep as _,
                None,
                rs_matter::dm::Access::WO,
            ),
            rs_matter::dm::Command::new(
                CommandId::MoveColorTemperature as _,
                None,
                rs_matter::dm::Access::WO,
            ),
            rs_matter::dm::Command::new(
                CommandId::StepColorTemperature as _,
                None,
                rs_matter::dm::Access::WO,
            ),
        ],
        |_, _, _| true,
        |_, _, _| true,
    );
    #[doc = "A helper struct to generate the cluster debug info."]
    struct MetadataDebug<T>(pub T);

    #[doc = "The handler trait for the cluster."]
    pub trait ClusterHandler {
        #[doc = "The cluster-metadata corresponding to this handler trait."]
        const CLUSTER: rs_matter::dm::Cluster<'static>;
        fn dataver(&self) -> u32;

        fn dataver_changed(&self);

        fn current_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn current_saturation(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn remaining_time(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn current_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn current_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn drift_compensation(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn compensation_text<P: rs_matter::tlv::TLVBuilderParent>(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
            builder: rs_matter::tlv::Utf8StrBuilder<P>,
        ) -> Result<P, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_temperature_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_mode(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error>;

        fn options(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error>;

        fn number_of_primaries(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error>;

        fn primary_1_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_1_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_1_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_2_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_2_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_2_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_3_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_3_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_3_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_4_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_4_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_4_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_5_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_5_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_5_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_6_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_6_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn primary_6_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn white_point_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn white_point_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_point_rx(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_point_ry(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_point_r_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_point_gx(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_point_gy(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_point_g_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_point_bx(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_point_by(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_point_b_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn enhanced_current_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn enhanced_color_mode(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error>;

        fn color_loop_active(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_loop_direction(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_loop_time(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_loop_start_enhanced_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_loop_stored_enhanced_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_capabilities(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error>;

        fn color_temp_physical_min_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn color_temp_physical_max_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn couple_color_temp_to_level_min_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn start_up_color_temperature_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u16>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn set_options(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u8,
        ) -> Result<(), rs_matter::error::Error>;

        fn set_white_point_x(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn set_white_point_y(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn set_color_point_rx(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn set_color_point_ry(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn set_color_point_r_intensity(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u8>,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn set_color_point_gx(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn set_color_point_gy(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn set_color_point_g_intensity(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u8>,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn set_color_point_bx(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn set_color_point_by(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn set_color_point_b_intensity(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u8>,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn set_start_up_color_temperature_mireds(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u16>,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        fn handle_move_to_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_move_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_step_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_move_to_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_move_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_step_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_move_to_hue_and_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToHueAndSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_move_to_color(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToColorRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_move_color(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveColorRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_step_color(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepColorRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_move_to_color_temperature(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToColorTemperatureRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_enhanced_move_to_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedMoveToHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_enhanced_move_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedMoveHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_enhanced_step_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedStepHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_enhanced_move_to_hue_and_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedMoveToHueAndSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_color_loop_set(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: ColorLoopSetRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_stop_move_step(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StopMoveStepRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_move_color_temperature(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveColorTemperatureRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        fn handle_step_color_temperature(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepColorTemperatureRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;
    }
    impl<T> ClusterHandler for &T
    where
        T: ClusterHandler,
    {
        const CLUSTER: rs_matter::dm::Cluster<'static> = T::CLUSTER;
        fn dataver(&self) -> u32 {
            T::dataver(self)
        }
        fn dataver_changed(&self) {
            T::dataver_changed(self)
        }
        fn current_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::current_hue(self, ctx)
        }
        fn current_saturation(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::current_saturation(self, ctx)
        }
        fn remaining_time(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::remaining_time(self, ctx)
        }
        fn current_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::current_x(self, ctx)
        }
        fn current_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::current_y(self, ctx)
        }
        fn drift_compensation(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::drift_compensation(self, ctx)
        }
        fn compensation_text<P: rs_matter::tlv::TLVBuilderParent>(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
            builder: rs_matter::tlv::Utf8StrBuilder<P>,
        ) -> Result<P, rs_matter::error::Error> {
            T::compensation_text(self, ctx, builder)
        }
        fn color_temperature_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_temperature_mireds(self, ctx)
        }
        fn color_mode(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::color_mode(self, ctx)
        }
        fn options(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::options(self, ctx)
        }
        fn number_of_primaries(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::number_of_primaries(self, ctx)
        }
        fn primary_1_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_1_x(self, ctx)
        }
        fn primary_1_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_1_y(self, ctx)
        }
        fn primary_1_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::primary_1_intensity(self, ctx)
        }
        fn primary_2_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_2_x(self, ctx)
        }
        fn primary_2_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_2_y(self, ctx)
        }
        fn primary_2_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::primary_2_intensity(self, ctx)
        }
        fn primary_3_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_3_x(self, ctx)
        }
        fn primary_3_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_3_y(self, ctx)
        }
        fn primary_3_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::primary_3_intensity(self, ctx)
        }
        fn primary_4_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_4_x(self, ctx)
        }
        fn primary_4_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_4_y(self, ctx)
        }
        fn primary_4_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::primary_4_intensity(self, ctx)
        }
        fn primary_5_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_5_x(self, ctx)
        }
        fn primary_5_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_5_y(self, ctx)
        }
        fn primary_5_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::primary_5_intensity(self, ctx)
        }
        fn primary_6_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_6_x(self, ctx)
        }
        fn primary_6_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_6_y(self, ctx)
        }
        fn primary_6_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::primary_6_intensity(self, ctx)
        }
        fn white_point_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::white_point_x(self, ctx)
        }
        fn white_point_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::white_point_y(self, ctx)
        }
        fn color_point_rx(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_point_rx(self, ctx)
        }
        fn color_point_ry(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_point_ry(self, ctx)
        }
        fn color_point_r_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::color_point_r_intensity(self, ctx)
        }
        fn color_point_gx(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_point_gx(self, ctx)
        }
        fn color_point_gy(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_point_gy(self, ctx)
        }
        fn color_point_g_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::color_point_g_intensity(self, ctx)
        }
        fn color_point_bx(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_point_bx(self, ctx)
        }
        fn color_point_by(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_point_by(self, ctx)
        }
        fn color_point_b_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::color_point_b_intensity(self, ctx)
        }
        fn enhanced_current_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::enhanced_current_hue(self, ctx)
        }
        fn enhanced_color_mode(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::enhanced_color_mode(self, ctx)
        }
        fn color_loop_active(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::color_loop_active(self, ctx)
        }
        fn color_loop_direction(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::color_loop_direction(self, ctx)
        }
        fn color_loop_time(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_loop_time(self, ctx)
        }
        fn color_loop_start_enhanced_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_loop_start_enhanced_hue(self, ctx)
        }
        fn color_loop_stored_enhanced_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_loop_stored_enhanced_hue(self, ctx)
        }
        fn color_capabilities(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_capabilities(self, ctx)
        }
        fn color_temp_physical_min_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_temp_physical_min_mireds(self, ctx)
        }
        fn color_temp_physical_max_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_temp_physical_max_mireds(self, ctx)
        }
        fn couple_color_temp_to_level_min_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::couple_color_temp_to_level_min_mireds(self, ctx)
        }
        fn start_up_color_temperature_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u16>, rs_matter::error::Error> {
            T::start_up_color_temperature_mireds(self, ctx)
        }
        fn set_options(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u8,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_options(self, ctx, value)
        }
        fn set_white_point_x(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_white_point_x(self, ctx, value)
        }
        fn set_white_point_y(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_white_point_y(self, ctx, value)
        }
        fn set_color_point_rx(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_rx(self, ctx, value)
        }
        fn set_color_point_ry(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_ry(self, ctx, value)
        }
        fn set_color_point_r_intensity(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u8>,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_r_intensity(self, ctx, value)
        }
        fn set_color_point_gx(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_gx(self, ctx, value)
        }
        fn set_color_point_gy(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_gy(self, ctx, value)
        }
        fn set_color_point_g_intensity(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u8>,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_g_intensity(self, ctx, value)
        }
        fn set_color_point_bx(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_bx(self, ctx, value)
        }
        fn set_color_point_by(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_by(self, ctx, value)
        }
        fn set_color_point_b_intensity(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u8>,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_b_intensity(self, ctx, value)
        }
        fn set_start_up_color_temperature_mireds(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u16>,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_start_up_color_temperature_mireds(self, ctx, value)
        }
        fn handle_move_to_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_to_hue(self, ctx, request)
        }
        fn handle_move_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_hue(self, ctx, request)
        }
        fn handle_step_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_step_hue(self, ctx, request)
        }
        fn handle_move_to_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_to_saturation(self, ctx, request)
        }
        fn handle_move_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_saturation(self, ctx, request)
        }
        fn handle_step_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_step_saturation(self, ctx, request)
        }
        fn handle_move_to_hue_and_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToHueAndSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_to_hue_and_saturation(self, ctx, request)
        }
        fn handle_move_to_color(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToColorRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_to_color(self, ctx, request)
        }
        fn handle_move_color(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveColorRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_color(self, ctx, request)
        }
        fn handle_step_color(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepColorRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_step_color(self, ctx, request)
        }
        fn handle_move_to_color_temperature(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToColorTemperatureRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_to_color_temperature(self, ctx, request)
        }
        fn handle_enhanced_move_to_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedMoveToHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_enhanced_move_to_hue(self, ctx, request)
        }
        fn handle_enhanced_move_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedMoveHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_enhanced_move_hue(self, ctx, request)
        }
        fn handle_enhanced_step_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedStepHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_enhanced_step_hue(self, ctx, request)
        }
        fn handle_enhanced_move_to_hue_and_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedMoveToHueAndSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_enhanced_move_to_hue_and_saturation(self, ctx, request)
        }
        fn handle_color_loop_set(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: ColorLoopSetRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_color_loop_set(self, ctx, request)
        }
        fn handle_stop_move_step(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StopMoveStepRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_stop_move_step(self, ctx, request)
        }
        fn handle_move_color_temperature(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveColorTemperatureRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_color_temperature(self, ctx, request)
        }
        fn handle_step_color_temperature(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepColorTemperatureRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_step_color_temperature(self, ctx, request)
        }
    }
    #[doc = "The handler adaptor for the cluster-specific handler. This adaptor implements the generic `rs-matter` handler trait."]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    pub struct HandlerAdaptor<T>(pub T);

    impl<T> rs_matter::dm::Handler for HandlerAdaptor<T>
    where
        T: ClusterHandler,
    {
        #[allow(unreachable_code)]
        fn read(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
            reply: impl rs_matter::dm::ReadReply,
        ) -> Result<(), rs_matter::error::Error> {
            if let Some(mut writer) = reply.with_dataver(self.0.dataver())? {
                if ctx.attr().is_system() {
                    ctx.attr().cluster()?.read(ctx.attr(), writer)
                } else {
                    match AttributeId::try_from(ctx.attr().attr_id)? {
                        AttributeId::CurrentHue => {
                            let attr_read_result = self.0.current_hue(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentHue, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentHue, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::CurrentSaturation => {
                            let attr_read_result = self.0.current_saturation(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentSaturation, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentSaturation, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::RemainingTime => {
                            let attr_read_result = self.0.remaining_time(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::RemainingTime, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::RemainingTime, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::CurrentX => {
                            let attr_read_result = self.0.current_x(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentX, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentX, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::CurrentY => {
                            let attr_read_result = self.0.current_y(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentY, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentY, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::DriftCompensation => {
                            let attr_read_result = self.0.drift_compensation(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::DriftCompensation, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::DriftCompensation, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::CompensationText => {
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> (build) +",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CompensationText, false))
                                ))
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> (build) +",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CompensationText, false))
                                ))
                            );
                            let tag = rs_matter::dm::Reply::tag(&writer);
                            let tw = rs_matter::dm::Reply::writer(&mut writer);
                            let attr_read_result = self.0.compensation_text(
                                &ctx,
                                rs_matter::tlv::TLVBuilder::new(
                                    rs_matter::tlv::TLVWriteParent::new(
                                        MetadataDebug((
                                            ctx.attr().endpoint_id,
                                            self,
                                            MetadataDebug((AttributeId::CompensationText, false)),
                                        )),
                                        tw,
                                    ),
                                    tag,
                                )?,
                            );
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CompensationText, false))
                                )),
                                attr_read_result.as_ref().map(|_| ())
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} (end) -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CompensationText, false))
                                )),
                                attr_read_result.as_ref().map(|_| ())
                            );
                            attr_read_result?;
                            rs_matter::dm::Reply::complete(writer)
                        }
                        AttributeId::ColorTemperatureMireds => {
                            let attr_read_result = self.0.color_temperature_mireds(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorTemperatureMireds, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorTemperatureMireds, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorMode => {
                            let attr_read_result = self.0.color_mode(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorMode, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorMode, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Options => {
                            let attr_read_result = self.0.options(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Options, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Options, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::NumberOfPrimaries => {
                            let attr_read_result = self.0.number_of_primaries(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::NumberOfPrimaries, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::NumberOfPrimaries, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary1X => {
                            let attr_read_result = self.0.primary_1_x(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary1X, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary1X, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary1Y => {
                            let attr_read_result = self.0.primary_1_y(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary1Y, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary1Y, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary1Intensity => {
                            let attr_read_result = self.0.primary_1_intensity(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary1Intensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary1Intensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary2X => {
                            let attr_read_result = self.0.primary_2_x(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary2X, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary2X, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary2Y => {
                            let attr_read_result = self.0.primary_2_y(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary2Y, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary2Y, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary2Intensity => {
                            let attr_read_result = self.0.primary_2_intensity(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary2Intensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary2Intensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary3X => {
                            let attr_read_result = self.0.primary_3_x(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary3X, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary3X, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary3Y => {
                            let attr_read_result = self.0.primary_3_y(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary3Y, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary3Y, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary3Intensity => {
                            let attr_read_result = self.0.primary_3_intensity(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary3Intensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary3Intensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary4X => {
                            let attr_read_result = self.0.primary_4_x(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary4X, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary4X, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary4Y => {
                            let attr_read_result = self.0.primary_4_y(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary4Y, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary4Y, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary4Intensity => {
                            let attr_read_result = self.0.primary_4_intensity(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary4Intensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary4Intensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary5X => {
                            let attr_read_result = self.0.primary_5_x(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary5X, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary5X, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary5Y => {
                            let attr_read_result = self.0.primary_5_y(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary5Y, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary5Y, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary5Intensity => {
                            let attr_read_result = self.0.primary_5_intensity(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary5Intensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary5Intensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary6X => {
                            let attr_read_result = self.0.primary_6_x(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary6X, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary6X, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary6Y => {
                            let attr_read_result = self.0.primary_6_y(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary6Y, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary6Y, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary6Intensity => {
                            let attr_read_result = self.0.primary_6_intensity(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary6Intensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary6Intensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::WhitePointX => {
                            let attr_read_result = self.0.white_point_x(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::WhitePointX, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::WhitePointX, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::WhitePointY => {
                            let attr_read_result = self.0.white_point_y(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::WhitePointY, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::WhitePointY, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointRX => {
                            let attr_read_result = self.0.color_point_rx(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointRX, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointRX, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointRY => {
                            let attr_read_result = self.0.color_point_ry(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointRY, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointRY, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointRIntensity => {
                            let attr_read_result = self.0.color_point_r_intensity(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointRIntensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointRIntensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointGX => {
                            let attr_read_result = self.0.color_point_gx(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointGX, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointGX, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointGY => {
                            let attr_read_result = self.0.color_point_gy(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointGY, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointGY, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointGIntensity => {
                            let attr_read_result = self.0.color_point_g_intensity(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointGIntensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointGIntensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointBX => {
                            let attr_read_result = self.0.color_point_bx(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointBX, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointBX, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointBY => {
                            let attr_read_result = self.0.color_point_by(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointBY, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointBY, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointBIntensity => {
                            let attr_read_result = self.0.color_point_b_intensity(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointBIntensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointBIntensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::EnhancedCurrentHue => {
                            let attr_read_result = self.0.enhanced_current_hue(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::EnhancedCurrentHue, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::EnhancedCurrentHue, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::EnhancedColorMode => {
                            let attr_read_result = self.0.enhanced_color_mode(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::EnhancedColorMode, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::EnhancedColorMode, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorLoopActive => {
                            let attr_read_result = self.0.color_loop_active(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopActive, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopActive, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorLoopDirection => {
                            let attr_read_result = self.0.color_loop_direction(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopDirection, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopDirection, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorLoopTime => {
                            let attr_read_result = self.0.color_loop_time(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopTime, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopTime, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorLoopStartEnhancedHue => {
                            let attr_read_result = self.0.color_loop_start_enhanced_hue(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopStartEnhancedHue, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopStartEnhancedHue, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorLoopStoredEnhancedHue => {
                            let attr_read_result = self.0.color_loop_stored_enhanced_hue(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopStoredEnhancedHue, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopStoredEnhancedHue, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorCapabilities => {
                            let attr_read_result = self.0.color_capabilities(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorCapabilities, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorCapabilities, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorTempPhysicalMinMireds => {
                            let attr_read_result = self.0.color_temp_physical_min_mireds(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorTempPhysicalMinMireds, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorTempPhysicalMinMireds, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorTempPhysicalMaxMireds => {
                            let attr_read_result = self.0.color_temp_physical_max_mireds(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorTempPhysicalMaxMireds, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorTempPhysicalMaxMireds, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::CoupleColorTempToLevelMinMireds => {
                            let attr_read_result =
                                self.0.couple_color_temp_to_level_min_mireds(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((
                                        AttributeId::CoupleColorTempToLevelMinMireds,
                                        false
                                    ))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((
                                        AttributeId::CoupleColorTempToLevelMinMireds,
                                        false
                                    ))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::StartUpColorTemperatureMireds => {
                            let attr_read_result = self.0.start_up_color_temperature_mireds(&ctx);
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((
                                        AttributeId::StartUpColorTemperatureMireds,
                                        false
                                    ))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((
                                        AttributeId::StartUpColorTemperatureMireds,
                                        false
                                    ))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        #[allow(unreachable_code)]
                        other => {
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::error!(
                                "Attribute {:?} not supported",
                                other
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::error!("Attribute {:?} not supported", other);
                            Err(rs_matter::error::ErrorCode::AttributeNotFound.into())
                        }
                    }
                }
            } else {
                Ok(())
            }
        }
        #[allow(unreachable_code)]
        fn write(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
        ) -> Result<(), rs_matter::error::Error> {
            ctx.attr().check_dataver(self.0.dataver())?;
            if ctx.attr().is_system() {
                return Err(rs_matter::error::ErrorCode::InvalidAction.into());
            }
            match AttributeId::try_from(ctx.attr().attr_id)? {
                AttributeId::Options => {
                    let attr_data: u8 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self.0.set_options(&ctx, attr_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::Options, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::Options, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::WhitePointX => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self.0.set_white_point_x(&ctx, attr_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::WhitePointX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::WhitePointX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::WhitePointY => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self.0.set_white_point_y(&ctx, attr_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::WhitePointY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::WhitePointY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointRX => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self.0.set_color_point_rx(&ctx, attr_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointRX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointRX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointRY => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self.0.set_color_point_ry(&ctx, attr_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointRY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointRY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointRIntensity => {
                    let attr_data: rs_matter::tlv::Nullable<u8> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result =
                        self.0.set_color_point_r_intensity(&ctx, attr_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointRIntensity, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointRIntensity, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointGX => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self.0.set_color_point_gx(&ctx, attr_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointGX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointGX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointGY => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self.0.set_color_point_gy(&ctx, attr_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointGY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointGY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointGIntensity => {
                    let attr_data: rs_matter::tlv::Nullable<u8> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result =
                        self.0.set_color_point_g_intensity(&ctx, attr_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointGIntensity, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointGIntensity, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointBX => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self.0.set_color_point_bx(&ctx, attr_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointBX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointBX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointBY => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self.0.set_color_point_by(&ctx, attr_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointBY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointBY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointBIntensity => {
                    let attr_data: rs_matter::tlv::Nullable<u8> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result =
                        self.0.set_color_point_b_intensity(&ctx, attr_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointBIntensity, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointBIntensity, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::StartUpColorTemperatureMireds => {
                    let attr_data: rs_matter::tlv::Nullable<u16> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self
                        .0
                        .set_start_up_color_temperature_mireds(&ctx, attr_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::StartUpColorTemperatureMireds, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::StartUpColorTemperatureMireds, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                other => {
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::error!("Attribute {:?} not supported", other);
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::error!("Attribute {:?} not supported", other);
                    return Err(rs_matter::error::ErrorCode::AttributeNotFound.into());
                }
            }
            self.0.dataver_changed();
            Ok(())
        }
        #[allow(unreachable_code)]
        fn invoke(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            reply: impl rs_matter::dm::InvokeReply,
        ) -> Result<(), rs_matter::error::Error> {
            match CommandId::try_from(ctx.cmd().cmd_id)? {
                CommandId::MoveToHue => {
                    let cmd_data: MoveToHueRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_move_to_hue(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveHue => {
                    let cmd_data: MoveHueRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_move_hue(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::StepHue => {
                    let cmd_data: StepHueRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_step_hue(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveToSaturation => {
                    let cmd_data: MoveToSaturationRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result =
                        self.0.handle_move_to_saturation(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveSaturation => {
                    let cmd_data: MoveSaturationRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_move_saturation(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::StepSaturation => {
                    let cmd_data: StepSaturationRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_step_saturation(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveToHueAndSaturation => {
                    let cmd_data: MoveToHueAndSaturationRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self
                        .0
                        .handle_move_to_hue_and_saturation(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToHueAndSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToHueAndSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveToColor => {
                    let cmd_data: MoveToColorRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_move_to_color(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToColor)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToColor)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveColor => {
                    let cmd_data: MoveColorRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_move_color(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveColor)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveColor)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::StepColor => {
                    let cmd_data: StepColorRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_step_color(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepColor)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepColor)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveToColorTemperature => {
                    let cmd_data: MoveToColorTemperatureRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self
                        .0
                        .handle_move_to_color_temperature(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToColorTemperature)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToColorTemperature)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::EnhancedMoveToHue => {
                    let cmd_data: EnhancedMoveToHueRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result =
                        self.0.handle_enhanced_move_to_hue(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedMoveToHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedMoveToHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::EnhancedMoveHue => {
                    let cmd_data: EnhancedMoveHueRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_enhanced_move_hue(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedMoveHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedMoveHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::EnhancedStepHue => {
                    let cmd_data: EnhancedStepHueRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_enhanced_step_hue(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedStepHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedStepHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::EnhancedMoveToHueAndSaturation => {
                    let cmd_data: EnhancedMoveToHueAndSaturationRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self
                        .0
                        .handle_enhanced_move_to_hue_and_saturation(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedMoveToHueAndSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedMoveToHueAndSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::ColorLoopSet => {
                    let cmd_data: ColorLoopSetRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_color_loop_set(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::ColorLoopSet)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::ColorLoopSet)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::StopMoveStep => {
                    let cmd_data: StopMoveStepRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_stop_move_step(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StopMoveStep)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StopMoveStep)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveColorTemperature => {
                    let cmd_data: MoveColorTemperatureRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result =
                        self.0.handle_move_color_temperature(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveColorTemperature)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveColorTemperature)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::StepColorTemperature => {
                    let cmd_data: StepColorTemperatureRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result =
                        self.0.handle_step_color_temperature(&ctx, cmd_data.clone());
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepColorTemperature)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepColorTemperature)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                other => {
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::error!("Command {:?} not supported", other);
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::error!("Command {:?} not supported", other);
                    return Err(rs_matter::error::ErrorCode::CommandNotFound.into());
                }
            }
            self.0.dataver_changed();
            Ok(())
        }
    }
    impl<T, Q> core::fmt::Debug for MetadataDebug<(u16, &HandlerAdaptor<T>, Q)>
    where
        T: ClusterHandler,
        Q: core::fmt::Debug,
    {
        #[allow(unreachable_code)]
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(
                f,
                "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}",
                self.0 .0, "ColorControl", 768u32, self.0 .2
            )
        }
    }
    #[cfg(feature = "defmt")]
    impl<T, Q> rs_matter::reexport::defmt::Format for MetadataDebug<(u16, &HandlerAdaptor<T>, Q)>
    where
        T: ClusterHandler,
        Q: rs_matter::reexport::defmt::Format,
    {
        #[allow(unreachable_code)]
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(
                f,
                "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}",
                self.0 .0,
                "ColorControl",
                768u32,
                self.0 .2
            )
        }
    }
    impl<T> rs_matter::dm::NonBlockingHandler for HandlerAdaptor<T> where T: ClusterHandler {}

    #[doc = "The handler trait for the cluster."]
    pub trait ClusterAsyncHandler {
        #[doc = "The cluster-metadata corresponding to this handler trait."]
        const CLUSTER: rs_matter::dm::Cluster<'static>;
        fn dataver(&self) -> u32;

        fn dataver_changed(&self);

        async fn run(&self) -> Result<(), rs_matter::error::Error> {
            core::future::pending::<Result<(), rs_matter::error::Error>>().await
        }
        async fn current_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn current_saturation(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn remaining_time(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn current_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn current_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn drift_compensation(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn compensation_text<P: rs_matter::tlv::TLVBuilderParent>(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
            builder: rs_matter::tlv::Utf8StrBuilder<P>,
        ) -> Result<P, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_temperature_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_mode(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error>;

        async fn options(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error>;

        async fn number_of_primaries(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error>;

        async fn primary_1_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_1_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_1_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_2_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_2_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_2_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_3_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_3_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_3_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_4_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_4_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_4_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_5_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_5_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_5_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_6_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_6_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn primary_6_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn white_point_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn white_point_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_point_rx(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_point_ry(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_point_r_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_point_gx(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_point_gy(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_point_g_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_point_bx(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_point_by(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_point_b_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn enhanced_current_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn enhanced_color_mode(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error>;

        async fn color_loop_active(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_loop_direction(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_loop_time(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_loop_start_enhanced_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_loop_stored_enhanced_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_capabilities(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error>;

        async fn color_temp_physical_min_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn color_temp_physical_max_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn couple_color_temp_to_level_min_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn start_up_color_temperature_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u16>, rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn set_options(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u8,
        ) -> Result<(), rs_matter::error::Error>;

        async fn set_white_point_x(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn set_white_point_y(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn set_color_point_rx(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn set_color_point_ry(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn set_color_point_r_intensity(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u8>,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn set_color_point_gx(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn set_color_point_gy(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn set_color_point_g_intensity(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u8>,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn set_color_point_bx(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn set_color_point_by(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn set_color_point_b_intensity(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u8>,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn set_start_up_color_temperature_mireds(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u16>,
        ) -> Result<(), rs_matter::error::Error> {
            Err(rs_matter::error::ErrorCode::InvalidAction.into())
        }
        async fn handle_move_to_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_move_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_step_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_move_to_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_move_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_step_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_move_to_hue_and_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToHueAndSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_move_to_color(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToColorRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_move_color(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveColorRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_step_color(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepColorRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_move_to_color_temperature(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToColorTemperatureRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_enhanced_move_to_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedMoveToHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_enhanced_move_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedMoveHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_enhanced_step_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedStepHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_enhanced_move_to_hue_and_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedMoveToHueAndSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_color_loop_set(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: ColorLoopSetRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_stop_move_step(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StopMoveStepRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_move_color_temperature(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveColorTemperatureRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;

        async fn handle_step_color_temperature(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepColorTemperatureRequest<'_>,
        ) -> Result<(), rs_matter::error::Error>;
    }
    impl<T> ClusterAsyncHandler for &T
    where
        T: ClusterAsyncHandler,
    {
        const CLUSTER: rs_matter::dm::Cluster<'static> = T::CLUSTER;
        fn dataver(&self) -> u32 {
            T::dataver(self)
        }
        fn dataver_changed(&self) {
            T::dataver_changed(self)
        }
        async fn run(&self) -> Result<(), rs_matter::error::Error> {
            (**self).run().await
        }
        async fn current_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::current_hue(self, ctx).await
        }
        async fn current_saturation(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::current_saturation(self, ctx).await
        }
        async fn remaining_time(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::remaining_time(self, ctx).await
        }
        async fn current_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::current_x(self, ctx).await
        }
        async fn current_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::current_y(self, ctx).await
        }
        async fn drift_compensation(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::drift_compensation(self, ctx).await
        }
        async fn compensation_text<P: rs_matter::tlv::TLVBuilderParent>(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
            builder: rs_matter::tlv::Utf8StrBuilder<P>,
        ) -> Result<P, rs_matter::error::Error> {
            T::compensation_text(self, ctx, builder).await
        }
        async fn color_temperature_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_temperature_mireds(self, ctx).await
        }
        async fn color_mode(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::color_mode(self, ctx).await
        }
        async fn options(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::options(self, ctx).await
        }
        async fn number_of_primaries(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::number_of_primaries(self, ctx).await
        }
        async fn primary_1_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_1_x(self, ctx).await
        }
        async fn primary_1_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_1_y(self, ctx).await
        }
        async fn primary_1_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::primary_1_intensity(self, ctx).await
        }
        async fn primary_2_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_2_x(self, ctx).await
        }
        async fn primary_2_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_2_y(self, ctx).await
        }
        async fn primary_2_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::primary_2_intensity(self, ctx).await
        }
        async fn primary_3_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_3_x(self, ctx).await
        }
        async fn primary_3_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_3_y(self, ctx).await
        }
        async fn primary_3_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::primary_3_intensity(self, ctx).await
        }
        async fn primary_4_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_4_x(self, ctx).await
        }
        async fn primary_4_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_4_y(self, ctx).await
        }
        async fn primary_4_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::primary_4_intensity(self, ctx).await
        }
        async fn primary_5_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_5_x(self, ctx).await
        }
        async fn primary_5_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_5_y(self, ctx).await
        }
        async fn primary_5_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::primary_5_intensity(self, ctx).await
        }
        async fn primary_6_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_6_x(self, ctx).await
        }
        async fn primary_6_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::primary_6_y(self, ctx).await
        }
        async fn primary_6_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::primary_6_intensity(self, ctx).await
        }
        async fn white_point_x(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::white_point_x(self, ctx).await
        }
        async fn white_point_y(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::white_point_y(self, ctx).await
        }
        async fn color_point_rx(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_point_rx(self, ctx).await
        }
        async fn color_point_ry(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_point_ry(self, ctx).await
        }
        async fn color_point_r_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::color_point_r_intensity(self, ctx).await
        }
        async fn color_point_gx(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_point_gx(self, ctx).await
        }
        async fn color_point_gy(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_point_gy(self, ctx).await
        }
        async fn color_point_g_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::color_point_g_intensity(self, ctx).await
        }
        async fn color_point_bx(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_point_bx(self, ctx).await
        }
        async fn color_point_by(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_point_by(self, ctx).await
        }
        async fn color_point_b_intensity(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
            T::color_point_b_intensity(self, ctx).await
        }
        async fn enhanced_current_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::enhanced_current_hue(self, ctx).await
        }
        async fn enhanced_color_mode(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::enhanced_color_mode(self, ctx).await
        }
        async fn color_loop_active(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::color_loop_active(self, ctx).await
        }
        async fn color_loop_direction(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u8, rs_matter::error::Error> {
            T::color_loop_direction(self, ctx).await
        }
        async fn color_loop_time(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_loop_time(self, ctx).await
        }
        async fn color_loop_start_enhanced_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_loop_start_enhanced_hue(self, ctx).await
        }
        async fn color_loop_stored_enhanced_hue(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_loop_stored_enhanced_hue(self, ctx).await
        }
        async fn color_capabilities(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_capabilities(self, ctx).await
        }
        async fn color_temp_physical_min_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_temp_physical_min_mireds(self, ctx).await
        }
        async fn color_temp_physical_max_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::color_temp_physical_max_mireds(self, ctx).await
        }
        async fn couple_color_temp_to_level_min_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<u16, rs_matter::error::Error> {
            T::couple_color_temp_to_level_min_mireds(self, ctx).await
        }
        async fn start_up_color_temperature_mireds(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
        ) -> Result<rs_matter::tlv::Nullable<u16>, rs_matter::error::Error> {
            T::start_up_color_temperature_mireds(self, ctx).await
        }
        async fn set_options(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u8,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_options(self, ctx, value).await
        }
        async fn set_white_point_x(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_white_point_x(self, ctx, value).await
        }
        async fn set_white_point_y(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_white_point_y(self, ctx, value).await
        }
        async fn set_color_point_rx(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_rx(self, ctx, value).await
        }
        async fn set_color_point_ry(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_ry(self, ctx, value).await
        }
        async fn set_color_point_r_intensity(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u8>,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_r_intensity(self, ctx, value).await
        }
        async fn set_color_point_gx(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_gx(self, ctx, value).await
        }
        async fn set_color_point_gy(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_gy(self, ctx, value).await
        }
        async fn set_color_point_g_intensity(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u8>,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_g_intensity(self, ctx, value).await
        }
        async fn set_color_point_bx(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_bx(self, ctx, value).await
        }
        async fn set_color_point_by(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: u16,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_by(self, ctx, value).await
        }
        async fn set_color_point_b_intensity(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u8>,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_color_point_b_intensity(self, ctx, value).await
        }
        async fn set_start_up_color_temperature_mireds(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
            value: rs_matter::tlv::Nullable<u16>,
        ) -> Result<(), rs_matter::error::Error> {
            T::set_start_up_color_temperature_mireds(self, ctx, value).await
        }
        async fn handle_move_to_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_to_hue(self, ctx, request).await
        }
        async fn handle_move_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_hue(self, ctx, request).await
        }
        async fn handle_step_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_step_hue(self, ctx, request).await
        }
        async fn handle_move_to_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_to_saturation(self, ctx, request).await
        }
        async fn handle_move_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_saturation(self, ctx, request).await
        }
        async fn handle_step_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_step_saturation(self, ctx, request).await
        }
        async fn handle_move_to_hue_and_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToHueAndSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_to_hue_and_saturation(self, ctx, request).await
        }
        async fn handle_move_to_color(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToColorRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_to_color(self, ctx, request).await
        }
        async fn handle_move_color(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveColorRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_color(self, ctx, request).await
        }
        async fn handle_step_color(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepColorRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_step_color(self, ctx, request).await
        }
        async fn handle_move_to_color_temperature(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveToColorTemperatureRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_to_color_temperature(self, ctx, request).await
        }
        async fn handle_enhanced_move_to_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedMoveToHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_enhanced_move_to_hue(self, ctx, request).await
        }
        async fn handle_enhanced_move_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedMoveHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_enhanced_move_hue(self, ctx, request).await
        }
        async fn handle_enhanced_step_hue(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedStepHueRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_enhanced_step_hue(self, ctx, request).await
        }
        async fn handle_enhanced_move_to_hue_and_saturation(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: EnhancedMoveToHueAndSaturationRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_enhanced_move_to_hue_and_saturation(self, ctx, request).await
        }
        async fn handle_color_loop_set(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: ColorLoopSetRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_color_loop_set(self, ctx, request).await
        }
        async fn handle_stop_move_step(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StopMoveStepRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_stop_move_step(self, ctx, request).await
        }
        async fn handle_move_color_temperature(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: MoveColorTemperatureRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_move_color_temperature(self, ctx, request).await
        }
        async fn handle_step_color_temperature(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            request: StepColorTemperatureRequest<'_>,
        ) -> Result<(), rs_matter::error::Error> {
            T::handle_step_color_temperature(self, ctx, request).await
        }
    }
    #[doc = "The handler adaptor for the cluster-specific handler. This adaptor implements the generic `rs-matter` handler trait."]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "defmt", derive(rs_matter::reexport::defmt::Format))]
    pub struct HandlerAsyncAdaptor<T>(pub T);

    impl<T> rs_matter::dm::AsyncHandler for HandlerAsyncAdaptor<T>
    where
        T: ClusterAsyncHandler,
    {
        #[allow(unreachable_code)]
        async fn read(
            &self,
            ctx: impl rs_matter::dm::ReadContext,
            reply: impl rs_matter::dm::ReadReply,
        ) -> Result<(), rs_matter::error::Error> {
            if let Some(mut writer) = reply.with_dataver(self.0.dataver())? {
                if ctx.attr().is_system() {
                    ctx.attr().cluster()?.read(ctx.attr(), writer)
                } else {
                    match AttributeId::try_from(ctx.attr().attr_id)? {
                        AttributeId::CurrentHue => {
                            let attr_read_result = self.0.current_hue(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentHue, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentHue, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::CurrentSaturation => {
                            let attr_read_result = self.0.current_saturation(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentSaturation, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentSaturation, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::RemainingTime => {
                            let attr_read_result = self.0.remaining_time(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::RemainingTime, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::RemainingTime, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::CurrentX => {
                            let attr_read_result = self.0.current_x(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentX, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentX, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::CurrentY => {
                            let attr_read_result = self.0.current_y(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentY, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CurrentY, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::DriftCompensation => {
                            let attr_read_result = self.0.drift_compensation(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::DriftCompensation, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::DriftCompensation, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::CompensationText => {
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> (build) +",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CompensationText, false))
                                ))
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> (build) +",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CompensationText, false))
                                ))
                            );
                            let tag = rs_matter::dm::Reply::tag(&writer);
                            let tw = rs_matter::dm::Reply::writer(&mut writer);
                            let attr_read_result = self
                                .0
                                .compensation_text(
                                    &ctx,
                                    rs_matter::tlv::TLVBuilder::new(
                                        rs_matter::tlv::TLVWriteParent::new(
                                            MetadataDebug((
                                                ctx.attr().endpoint_id,
                                                self,
                                                MetadataDebug((
                                                    AttributeId::CompensationText,
                                                    false,
                                                )),
                                            )),
                                            tw,
                                        ),
                                        tag,
                                    )?,
                                )
                                .await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CompensationText, false))
                                )),
                                attr_read_result.as_ref().map(|_| ())
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} (end) -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::CompensationText, false))
                                )),
                                attr_read_result.as_ref().map(|_| ())
                            );
                            attr_read_result?;
                            rs_matter::dm::Reply::complete(writer)
                        }
                        AttributeId::ColorTemperatureMireds => {
                            let attr_read_result = self.0.color_temperature_mireds(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorTemperatureMireds, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorTemperatureMireds, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorMode => {
                            let attr_read_result = self.0.color_mode(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorMode, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorMode, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Options => {
                            let attr_read_result = self.0.options(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Options, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Options, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::NumberOfPrimaries => {
                            let attr_read_result = self.0.number_of_primaries(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::NumberOfPrimaries, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::NumberOfPrimaries, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary1X => {
                            let attr_read_result = self.0.primary_1_x(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary1X, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary1X, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary1Y => {
                            let attr_read_result = self.0.primary_1_y(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary1Y, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary1Y, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary1Intensity => {
                            let attr_read_result = self.0.primary_1_intensity(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary1Intensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary1Intensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary2X => {
                            let attr_read_result = self.0.primary_2_x(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary2X, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary2X, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary2Y => {
                            let attr_read_result = self.0.primary_2_y(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary2Y, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary2Y, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary2Intensity => {
                            let attr_read_result = self.0.primary_2_intensity(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary2Intensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary2Intensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary3X => {
                            let attr_read_result = self.0.primary_3_x(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary3X, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary3X, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary3Y => {
                            let attr_read_result = self.0.primary_3_y(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary3Y, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary3Y, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary3Intensity => {
                            let attr_read_result = self.0.primary_3_intensity(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary3Intensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary3Intensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary4X => {
                            let attr_read_result = self.0.primary_4_x(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary4X, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary4X, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary4Y => {
                            let attr_read_result = self.0.primary_4_y(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary4Y, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary4Y, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary4Intensity => {
                            let attr_read_result = self.0.primary_4_intensity(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary4Intensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary4Intensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary5X => {
                            let attr_read_result = self.0.primary_5_x(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary5X, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary5X, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary5Y => {
                            let attr_read_result = self.0.primary_5_y(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary5Y, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary5Y, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary5Intensity => {
                            let attr_read_result = self.0.primary_5_intensity(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary5Intensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary5Intensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary6X => {
                            let attr_read_result = self.0.primary_6_x(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary6X, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary6X, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary6Y => {
                            let attr_read_result = self.0.primary_6_y(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary6Y, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary6Y, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::Primary6Intensity => {
                            let attr_read_result = self.0.primary_6_intensity(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary6Intensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::Primary6Intensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::WhitePointX => {
                            let attr_read_result = self.0.white_point_x(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::WhitePointX, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::WhitePointX, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::WhitePointY => {
                            let attr_read_result = self.0.white_point_y(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::WhitePointY, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::WhitePointY, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointRX => {
                            let attr_read_result = self.0.color_point_rx(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointRX, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointRX, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointRY => {
                            let attr_read_result = self.0.color_point_ry(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointRY, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointRY, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointRIntensity => {
                            let attr_read_result = self.0.color_point_r_intensity(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointRIntensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointRIntensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointGX => {
                            let attr_read_result = self.0.color_point_gx(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointGX, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointGX, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointGY => {
                            let attr_read_result = self.0.color_point_gy(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointGY, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointGY, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointGIntensity => {
                            let attr_read_result = self.0.color_point_g_intensity(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointGIntensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointGIntensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointBX => {
                            let attr_read_result = self.0.color_point_bx(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointBX, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointBX, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointBY => {
                            let attr_read_result = self.0.color_point_by(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointBY, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointBY, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorPointBIntensity => {
                            let attr_read_result = self.0.color_point_b_intensity(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointBIntensity, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorPointBIntensity, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::EnhancedCurrentHue => {
                            let attr_read_result = self.0.enhanced_current_hue(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::EnhancedCurrentHue, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::EnhancedCurrentHue, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::EnhancedColorMode => {
                            let attr_read_result = self.0.enhanced_color_mode(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::EnhancedColorMode, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::EnhancedColorMode, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorLoopActive => {
                            let attr_read_result = self.0.color_loop_active(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopActive, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopActive, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorLoopDirection => {
                            let attr_read_result = self.0.color_loop_direction(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopDirection, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopDirection, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorLoopTime => {
                            let attr_read_result = self.0.color_loop_time(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopTime, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopTime, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorLoopStartEnhancedHue => {
                            let attr_read_result = self.0.color_loop_start_enhanced_hue(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopStartEnhancedHue, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopStartEnhancedHue, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorLoopStoredEnhancedHue => {
                            let attr_read_result =
                                self.0.color_loop_stored_enhanced_hue(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopStoredEnhancedHue, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorLoopStoredEnhancedHue, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorCapabilities => {
                            let attr_read_result = self.0.color_capabilities(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorCapabilities, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorCapabilities, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorTempPhysicalMinMireds => {
                            let attr_read_result =
                                self.0.color_temp_physical_min_mireds(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorTempPhysicalMinMireds, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorTempPhysicalMinMireds, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::ColorTempPhysicalMaxMireds => {
                            let attr_read_result =
                                self.0.color_temp_physical_max_mireds(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorTempPhysicalMaxMireds, false))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((AttributeId::ColorTempPhysicalMaxMireds, false))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::CoupleColorTempToLevelMinMireds => {
                            let attr_read_result =
                                self.0.couple_color_temp_to_level_min_mireds(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((
                                        AttributeId::CoupleColorTempToLevelMinMireds,
                                        false
                                    ))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((
                                        AttributeId::CoupleColorTempToLevelMinMireds,
                                        false
                                    ))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        AttributeId::StartUpColorTemperatureMireds => {
                            let attr_read_result =
                                self.0.start_up_color_temperature_mireds(&ctx).await;
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((
                                        AttributeId::StartUpColorTemperatureMireds,
                                        false
                                    ))
                                )),
                                attr_read_result
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::debug!(
                                "{:?} -> {:?}",
                                MetadataDebug((
                                    ctx.attr().endpoint_id,
                                    self,
                                    MetadataDebug((
                                        AttributeId::StartUpColorTemperatureMireds,
                                        false
                                    ))
                                )),
                                attr_read_result
                            );
                            rs_matter::dm::Reply::set(writer, attr_read_result?)
                        }
                        #[allow(unreachable_code)]
                        other => {
                            #[cfg(feature = "defmt")]
                            rs_matter::reexport::defmt::error!(
                                "Attribute {:?} not supported",
                                other
                            );
                            #[cfg(feature = "log")]
                            rs_matter::reexport::log::error!("Attribute {:?} not supported", other);
                            Err(rs_matter::error::ErrorCode::AttributeNotFound.into())
                        }
                    }
                }
            } else {
                Ok(())
            }
        }
        #[allow(unreachable_code)]
        async fn write(
            &self,
            ctx: impl rs_matter::dm::WriteContext,
        ) -> Result<(), rs_matter::error::Error> {
            ctx.attr().check_dataver(self.0.dataver())?;
            if ctx.attr().is_system() {
                return Err(rs_matter::error::ErrorCode::InvalidAction.into());
            }
            match AttributeId::try_from(ctx.attr().attr_id)? {
                AttributeId::Options => {
                    let attr_data: u8 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self.0.set_options(&ctx, attr_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::Options, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::Options, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::WhitePointX => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self.0.set_white_point_x(&ctx, attr_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::WhitePointX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::WhitePointX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::WhitePointY => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self.0.set_white_point_y(&ctx, attr_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::WhitePointY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::WhitePointY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointRX => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result =
                        self.0.set_color_point_rx(&ctx, attr_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointRX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointRX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointRY => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result =
                        self.0.set_color_point_ry(&ctx, attr_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointRY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointRY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointRIntensity => {
                    let attr_data: rs_matter::tlv::Nullable<u8> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self
                        .0
                        .set_color_point_r_intensity(&ctx, attr_data.clone())
                        .await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointRIntensity, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointRIntensity, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointGX => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result =
                        self.0.set_color_point_gx(&ctx, attr_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointGX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointGX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointGY => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result =
                        self.0.set_color_point_gy(&ctx, attr_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointGY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointGY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointGIntensity => {
                    let attr_data: rs_matter::tlv::Nullable<u8> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self
                        .0
                        .set_color_point_g_intensity(&ctx, attr_data.clone())
                        .await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointGIntensity, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointGIntensity, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointBX => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result =
                        self.0.set_color_point_bx(&ctx, attr_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointBX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointBX, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointBY => {
                    let attr_data: u16 = rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result =
                        self.0.set_color_point_by(&ctx, attr_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointBY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointBY, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::ColorPointBIntensity => {
                    let attr_data: rs_matter::tlv::Nullable<u8> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self
                        .0
                        .set_color_point_b_intensity(&ctx, attr_data.clone())
                        .await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointBIntensity, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::ColorPointBIntensity, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                AttributeId::StartUpColorTemperatureMireds => {
                    let attr_data: rs_matter::tlv::Nullable<u16> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let attr_write_result = self
                        .0
                        .set_start_up_color_temperature_mireds(&ctx, attr_data.clone())
                        .await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::StartUpColorTemperatureMireds, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.attr().endpoint_id,
                            self,
                            MetadataDebug((AttributeId::StartUpColorTemperatureMireds, false))
                        )),
                        attr_data,
                        attr_write_result
                    );
                    attr_write_result?;
                }
                other => {
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::error!("Attribute {:?} not supported", other);
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::error!("Attribute {:?} not supported", other);
                    return Err(rs_matter::error::ErrorCode::AttributeNotFound.into());
                }
            }
            self.0.dataver_changed();
            Ok(())
        }
        #[allow(unreachable_code)]
        async fn invoke(
            &self,
            ctx: impl rs_matter::dm::InvokeContext,
            reply: impl rs_matter::dm::InvokeReply,
        ) -> Result<(), rs_matter::error::Error> {
            match CommandId::try_from(ctx.cmd().cmd_id)? {
                CommandId::MoveToHue => {
                    let cmd_data: MoveToHueRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_move_to_hue(&ctx, cmd_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveHue => {
                    let cmd_data: MoveHueRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_move_hue(&ctx, cmd_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::StepHue => {
                    let cmd_data: StepHueRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_step_hue(&ctx, cmd_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveToSaturation => {
                    let cmd_data: MoveToSaturationRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self
                        .0
                        .handle_move_to_saturation(&ctx, cmd_data.clone())
                        .await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveSaturation => {
                    let cmd_data: MoveSaturationRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result =
                        self.0.handle_move_saturation(&ctx, cmd_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::StepSaturation => {
                    let cmd_data: StepSaturationRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result =
                        self.0.handle_step_saturation(&ctx, cmd_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveToHueAndSaturation => {
                    let cmd_data: MoveToHueAndSaturationRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self
                        .0
                        .handle_move_to_hue_and_saturation(&ctx, cmd_data.clone())
                        .await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToHueAndSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToHueAndSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveToColor => {
                    let cmd_data: MoveToColorRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result =
                        self.0.handle_move_to_color(&ctx, cmd_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToColor)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToColor)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveColor => {
                    let cmd_data: MoveColorRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_move_color(&ctx, cmd_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveColor)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveColor)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::StepColor => {
                    let cmd_data: StepColorRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self.0.handle_step_color(&ctx, cmd_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepColor)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepColor)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveToColorTemperature => {
                    let cmd_data: MoveToColorTemperatureRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self
                        .0
                        .handle_move_to_color_temperature(&ctx, cmd_data.clone())
                        .await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToColorTemperature)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveToColorTemperature)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::EnhancedMoveToHue => {
                    let cmd_data: EnhancedMoveToHueRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self
                        .0
                        .handle_enhanced_move_to_hue(&ctx, cmd_data.clone())
                        .await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedMoveToHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedMoveToHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::EnhancedMoveHue => {
                    let cmd_data: EnhancedMoveHueRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self
                        .0
                        .handle_enhanced_move_hue(&ctx, cmd_data.clone())
                        .await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedMoveHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedMoveHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::EnhancedStepHue => {
                    let cmd_data: EnhancedStepHueRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self
                        .0
                        .handle_enhanced_step_hue(&ctx, cmd_data.clone())
                        .await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedStepHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedStepHue)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::EnhancedMoveToHueAndSaturation => {
                    let cmd_data: EnhancedMoveToHueAndSaturationRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self
                        .0
                        .handle_enhanced_move_to_hue_and_saturation(&ctx, cmd_data.clone())
                        .await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedMoveToHueAndSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::EnhancedMoveToHueAndSaturation)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::ColorLoopSet => {
                    let cmd_data: ColorLoopSetRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result =
                        self.0.handle_color_loop_set(&ctx, cmd_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::ColorLoopSet)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::ColorLoopSet)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::StopMoveStep => {
                    let cmd_data: StopMoveStepRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result =
                        self.0.handle_stop_move_step(&ctx, cmd_data.clone()).await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StopMoveStep)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StopMoveStep)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::MoveColorTemperature => {
                    let cmd_data: MoveColorTemperatureRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self
                        .0
                        .handle_move_color_temperature(&ctx, cmd_data.clone())
                        .await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveColorTemperature)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::MoveColorTemperature)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                CommandId::StepColorTemperature => {
                    let cmd_data: StepColorTemperatureRequest<'_> =
                        rs_matter::tlv::FromTLV::from_tlv(ctx.data())?;
                    let cmd_invoke_result = self
                        .0
                        .handle_step_color_temperature(&ctx, cmd_data.clone())
                        .await;
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepColorTemperature)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::debug!(
                        "{:?}({:?}) -> {:?}",
                        MetadataDebug((
                            ctx.cmd().endpoint_id,
                            self,
                            MetadataDebug(CommandId::StepColorTemperature)
                        )),
                        cmd_data,
                        cmd_invoke_result
                    );
                    cmd_invoke_result?;
                }
                other => {
                    #[cfg(feature = "defmt")]
                    rs_matter::reexport::defmt::error!("Command {:?} not supported", other);
                    #[cfg(feature = "log")]
                    rs_matter::reexport::log::error!("Command {:?} not supported", other);
                    return Err(rs_matter::error::ErrorCode::CommandNotFound.into());
                }
            }
            self.0.dataver_changed();
            Ok(())
        }
        async fn run(&self) -> Result<(), rs_matter::error::Error> {
            self.0.run().await
        }
    }
    impl<T, Q> core::fmt::Debug for MetadataDebug<(u16, &HandlerAsyncAdaptor<T>, Q)>
    where
        T: ClusterAsyncHandler,
        Q: core::fmt::Debug,
    {
        #[allow(unreachable_code)]
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(
                f,
                "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}",
                self.0 .0, "ColorControl", 768u32, self.0 .2
            )
        }
    }
    #[cfg(feature = "defmt")]
    impl<T, Q> rs_matter::reexport::defmt::Format for MetadataDebug<(u16, &HandlerAsyncAdaptor<T>, Q)>
    where
        T: ClusterAsyncHandler,
        Q: rs_matter::reexport::defmt::Format,
    {
        #[allow(unreachable_code)]
        fn format(&self, f: rs_matter::reexport::defmt::Formatter<'_>) {
            rs_matter::reexport::defmt::write!(
                f,
                "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}",
                self.0 .0,
                "ColorControl",
                768u32,
                self.0 .2
            )
        }
    }
