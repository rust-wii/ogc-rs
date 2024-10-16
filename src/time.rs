use core::{
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration,
};

use bit_field::BitField;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Instant(Duration);

impl Instant {
    #[must_use]
    pub fn now() -> Self {
        Self(now())
    }

    #[must_use]
    pub fn duration_since(&self, earlier: Self) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    #[must_use]
    pub fn checked_duration_since(&self, earlier: Self) -> Option<Duration> {
        self.0.checked_sub(earlier.0)
    }

    #[must_use]
    pub fn saturating_duration_since(&self, earlier: Self) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    #[must_use]
    pub fn elapsed(&self) -> Duration {
        Self::now() - *self
    }

    #[must_use]
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.0.checked_add(duration).map(Self)
    }

    #[must_use]
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        self.0.checked_sub(duration).map(Self)
    }
}

impl Add<Duration> for Instant {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self::Output {
        self.checked_add(rhs)
            .expect("overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;
    fn sub(self, rhs: Duration) -> Self::Output {
        self.checked_sub(rhs)
            .expect("overflow when adding duration to instant")
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Sub<Self> for Instant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.duration_since(rhs)
    }
}

fn move_from_time_base_upper() -> u32 {
    let upper: u32;

    unsafe { core::arch::asm!("mftbu {}",out(reg) upper) };

    upper
}

fn move_from_time_base() -> u32 {
    let lower: u32;

    unsafe { core::arch::asm!("mftb {}", out(reg) lower) };

    lower
}

fn time_base_to_duration(upper: u32, lower: u32) -> Duration {
    let mut timebase: u64 = 0;
    timebase.set_bits(..=31, lower.into());
    timebase.set_bits(32..=63, upper.into());
    const TIMEBASE_BUS_CLOCK: u64 = 243000000;
    const TIMEBASE_TIMER_CLOCK: u64 = TIMEBASE_BUS_CLOCK / 4000;
    let duration = timebase * 8 / (TIMEBASE_TIMER_CLOCK / 125);

    Duration::from_micros(duration)
}

fn now() -> Duration {
    let (upper, lower) = loop {
        let time_base_upper = move_from_time_base_upper();
        let time_base = move_from_time_base();

        if time_base_upper == move_from_time_base_upper() {
            break (time_base_upper, time_base);
        }
    };

    time_base_to_duration(upper, lower)
}
