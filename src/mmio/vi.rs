use voladdress::{Safe, Unsafe, VolAddress};

#[repr(transparent)]
pub struct VerticalTimingControl(u16);
pub const VERTICAL_TIMING_REGISTER: VolAddress<VerticalTimingControl, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_2000) };

#[repr(transparent)]
pub struct DisplayConfig(u16);
pub const DISPLAY_CONFIG_REGISTER: VolAddress<DisplayConfig, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_2002) };

#[repr(transparent)]
pub struct HorizontalTimingZero(u32);
pub const HORIZONTAL_TIMING_ZERO: VolAddress<HorizontalTimingZero, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_2004) };

#[repr(transparent)]
pub struct HorizontalTimingOne(u32);
pub const HORIZONTAL_TIMING_ONE: VolAddress<HorizontalTimingOne, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_2008) };

#[repr(transparent)]
pub struct BlankingLines(u32);
pub const ODD_VERTICAL_TIMING_REGISTER: VolAddress<BlankingLines, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_200C) };
pub const EVEN_VERTICAL_TIMING_REGISTER: VolAddress<BlankingLines, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_2010) };

#[repr(transparent)]
pub struct BurstInterval(u32);
pub const ODD_BURST_BLANKING_INTERVAL_REGISTER: VolAddress<BurstInterval, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_2014) };
pub const EVEN_BURST_BLANKING_INTERVAL_REGISTER: VolAddress<BurstInterval, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_2018) };

#[repr(transparent)]
pub struct TopFieldBase(u32);
pub const TOP_FIELD_BASE_REGISTER_L: VolAddress<TopFieldBase, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_201C) };

pub const TOP_FIELD_BASE_REGISTER_R: VolAddress<u32, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_2020) };

#[repr(transparent)]
pub struct BottomFieldBase(u32);
pub const BOTTOM_FIELD_BASE_REGISTER_L: VolAddress<BottomFieldBase, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_2024) };

pub const BOTTOM_FIELD_BASE_REGISTER_R: VolAddress<u32, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_2028) };

#[repr(transparent)]
pub struct ScreenPosition(u16);
pub const CURRENT_VERTICAL_POSITION: VolAddress<ScreenPosition, Safe, ()> =
    unsafe { VolAddress::new(0xCC00_202C) };
pub const CURRENT_HORIZONTAL_POSITION: VolAddress<ScreenPosition, Safe, ()> =
    unsafe { VolAddress::new(0xCC00_202E) };

#[repr(transparent)]
pub struct DisplayInterrupt(u32);
pub const DISPLAY_INTERRUPT_ZERO: VolAddress<DisplayInterrupt, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_2030) };
pub const DISPLAY_INTERRUPT_ONE: VolAddress<DisplayInterrupt, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_2034) };
pub const DISPLAY_INTERRUPT_TWO: VolAddress<DisplayInterrupt, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_2038) };
pub const DISPLAY_INTERRUPT_THREE: VolAddress<DisplayInterrupt, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_203C) };

#[repr(transparent)]
pub struct DisplayLatchControl(u32);
pub const DISPLAY_LATCH_REGISTER_ZERO: VolAddress<DisplayLatchControl, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_2040) };
pub const DISPLAY_LATCH_REGISTER_ONE: VolAddress<DisplayLatchControl, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_2044) };

#[repr(transparent)]
pub struct ScalerControl(u16);
pub const SCALING_WIDTH_REGISTER: VolAddress<ScalingControl, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_2048) };

#[repr(transparent)]
pub struct ScalingControl(u16);
pub const HORIZONTAL_SCALING_REGISTER: VolAddress<ScalingControl, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_204a) };

#[repr(transparent)]
pub struct FilterCoefficents(u32);
pub const FILTER_COEFFICENT_TABLE_ZERO: VolAddress<FilterCoefficents, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_204C) };
pub const FILTER_COEFFICENT_TABLE_ONE: VolAddress<FilterCoefficents, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_2050) };
pub const FILTER_COEFFICENT_TABLE_TWO: VolAddress<FilterCoefficents, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_2054) };
pub const FILTER_COEFFICENT_TABLE_THREE: VolAddress<FilterCoefficents, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_2058) };
pub const FILTER_COEFFICENT_TABLE_FOUR: VolAddress<FilterCoefficents, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_205C) };
pub const FILTER_COEFFICENT_TABLE_FIVE: VolAddress<FilterCoefficents, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_2060) };
pub const FILTER_COEFFICENT_TABLE_SIX: VolAddress<FilterCoefficents, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_2064) };

pub const UNKNOWN_ANTIALIASING: VolAddress<u32, Unsafe, Unsafe> =
    unsafe { VolAddress::new(0xCC00_2068) };

#[repr(transparent)]
pub struct VideoClockControl(u16);
pub const VIDEO_CLOCK_SELECT_REGISTER: VolAddress<VideoClockControl, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_206C) };

#[repr(transparent)]
pub struct VideoSelect(u16);
pub const VIDEO_DTV_SELECT_REGISTER: VolAddress<VideoSelect, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_206E) };

#[repr(transparent)]
pub struct BorderHBEControl(u16);
pub const BORDER_HBE: VolAddress<BorderHBEControl, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_2072) };

#[repr(transparent)]
pub struct BorderHBSControl(u16);
pub const BORDER_HBS: VolAddress<BorderHBSControl, Safe, Safe> =
    unsafe { VolAddress::new(0xCC00_2074) };

pub const UNUSED_ZERO: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0xCC00_2076) };
pub const UNUSED_ONE: VolAddress<u32, Safe, Safe> = unsafe { VolAddress::new(0xCC00_2078) };
pub const UNUSED_TWO: VolAddress<u32, Safe, Safe> = unsafe { VolAddress::new(0xCC00_207C) };
