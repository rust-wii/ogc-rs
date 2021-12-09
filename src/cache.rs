use crate::ffi;

const CACHELINE_SIZE: usize = 32;

/// Enable L1 data-cache.
pub fn dc_enable() {
    unsafe { ffi::DCEnable() }
}

/// Disable L1 data-cache.
pub fn dc_disable() {
    unsafe { ffi::DCDisable() }
}

/// Invalidate L1 data-cache.
///
/// Marks the state of each data cache block as invalid without writing back
/// modified cache blocks to memory. Cache access is blocked during this time.
/// Bus accesses to the cache are signaled as a miss during invalidate-all
/// operations.
pub fn dc_flash_invalidate() {
    unsafe { ffi::DCFlashInvalidate() }
}

/// Current contents of the L1 d-cache are locked down and will not be cast out.
///
/// Hits are still serviced, but misses go straight to L2 or 60x bus. Most cache
/// operations, such as `dc_flush_range()`, will still execute regardless of
/// whether the cache is frozen.
///
/// NOTE: In PowerPC architecture jargon, this feature is referred to as
/// "locking" the data cache. We use the word "freeze" to distinguish it from
/// the locked cache and DMA features.
pub fn dc_freeze() {
    unsafe { ffi::DCFreeze() }
}

/// Undoes actions of `dc_freeze()`.
///
/// Old cache blocks will now be cast out on subsequent L1 misses.
///
/// NOTE: In PowerPC architecture jargon, this feature is referred to as
/// "locking" the data cache. We use the word "freeze" to distinguish it from
/// the locked cache and DMA features.
pub fn dc_unfreeze() {
    unsafe { ffi::DCUnfreeze() }
}

/// Enable L1 i-cache.
pub fn ic_enable() {
    unsafe { ffi::ICEnable() }
}

/// Invalidate the L1 i-cache.
///
/// An invalidate operation is issued that marks the state of each instruction
/// cache block as invalid without writing back modified cache blocks to memory.
///
/// Cache access is blocked during this time. Bus accesses to the cache are
/// signaled as a miss during invalidate-all operations.
pub fn ic_flash_invalidate() {
    unsafe { ffi::ICFlashInvalidate() }
}

/// Current contents of the L1 i-cache are locked down and will not be cast out.
///
/// Hits are still serviced, but misses go straight to L2 or 60x bus.
///
/// NOTE: In PowerPC architecture jargon, this feature is referred to as
/// "locking" the data cache. We use the word "freeze" to distinguish it from
/// the locked cache and DMA features.
pub fn ic_freeze() {
    unsafe { ffi::ICFreeze() }
}

/// Undoes actions of `ic_freeze()`.
///
/// Old cache blocks will now be cast out on subsequent L1 misses.
///
/// NOTE: In PowerPC architecture jargon, this feature is referred to as
/// "locking" the data cache. We use the word "freeze" to distinguish it from
/// the locked cache and DMA features.
pub fn ic_unfreeze() {
    unsafe { ffi::ICUnfreeze() }
}

/// Performs an instruction cache synchronization.
///
/// This ensures that all instructions preceding this instruction have completed
/// before this instruction completes.
pub fn ic_sync() {
    unsafe { ffi::ICSync() }
}
