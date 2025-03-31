use voladdress::{Safe, VolAddress};

pub unsafe fn move_to_machine_state_register(value: MachineStateRegister) {
    core::arch::asm!("mtmsr {VALUE}", VALUE = in(reg) value.bits());
}

pub unsafe fn move_from_machine_state_register() -> MachineStateRegister {
    let value: u32;

    core::arch::asm!("mfmsr {VALUE}", VALUE = out(reg) value);

    MachineStateRegister::from_bits_truncate(value)
}

pub unsafe fn disable_interrupts() -> MachineStateRegister {
    let mut register = unsafe { move_from_machine_state_register() };
    let ret = register;

    register.remove(MachineStateRegister::EE);
    unsafe { move_to_machine_state_register(register) };

    ret
}

pub unsafe fn enable_interrupts() {
    let mut register = unsafe { move_from_machine_state_register() };
    register.insert(MachineStateRegister::EE);
    unsafe { move_to_machine_state_register(register) };
}

pub fn with_interrupts_disabled<R>(func: impl FnOnce() -> R) -> R {
    let msr = unsafe { disable_interrupts() };

    let r = func();

    if msr.contains(MachineStateRegister::EE) {
        unsafe { enable_interrupts() };
    }

    r
}

bitflags::bitflags! {
    pub struct MachineStateRegister: u32 {
        const LE = 1 << 0;
        const RI = 1 << 1;
        const PM = 1 << 2;
        const DR = 1 << 4;
        const IR = 1 << 5;
        const IP = 1 << 6;
        const FE1 = 1 << 8;
        const BE = 1 << 9;
        const SE = 1 << 10;
        const FE0 = 1 << 11;
        const ME = 1 << 12;
        const FP = 1 << 13;
        const PR = 1 << 14;
        /// External Interrupt Enable
        const EE = 1 << 15;
        const ILE = 1 << 16;
        const POW = 1 << 18;
    }
}

mod allocator {
    use core::{
        alloc::Layout,
        ptr::NonNull,
        sync::atomic::{AtomicPtr, Ordering},
    };

    pub struct Allocator;
    pub struct AllocatorError;

    static MEM1_START: AtomicPtr<u8> = AtomicPtr::new(core::ptr::null_mut());
    static MEM2_START: AtomicPtr<u8> = AtomicPtr::new(core::ptr::null_mut());
    static MEM1_END: AtomicPtr<u8> = AtomicPtr::new(core::ptr::null_mut());
    static MEM2_END: AtomicPtr<u8> = AtomicPtr::new(core::ptr::null_mut());

    impl Allocator {
        pub fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocatorError> {
            if let Some(ptr) = self.try_mem1_allocate(layout) {
                MEM1_START.store(
                    ptr.as_ptr().cast::<u8>().wrapping_add(layout.size()),
                    Ordering::Relaxed,
                );
                return Ok(ptr);
            }

            if let Some(ptr) = self.try_mem2_allocate(layout) {
                MEM2_START.store(
                    ptr.as_ptr().cast::<u8>().wrapping_add(layout.size()),
                    Ordering::Relaxed,
                );
                return Ok(ptr);
            }

            Err(AllocatorError)
        }

        pub unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {}

        fn try_allocate(
            &self,
            layout: Layout,
            region_start: *mut u8,
            region_end: *const u8,
        ) -> Option<NonNull<[u8]>> {
            let size = layout.size();
            let align = layout.align();

            let mut ptr = region_start;
            let offset = ptr.align_offset(align);
            ptr = ptr.wrapping_add(offset);

            let ptr_end = ptr.wrapping_add(size);

            if ptr_end.addr() >= region_end.addr() {
                None
            } else {
                Some(NonNull::slice_from_raw_parts(NonNull::new(ptr)?, size))
            }
        }

        fn try_mem1_allocate(&self, layout: Layout) -> Option<NonNull<[u8]>> {
            let _ = MEM1_START.compare_exchange_weak(
                core::ptr::null_mut(),
                super::MEM1_START.read(),
                Ordering::SeqCst,
                Ordering::Relaxed,
            );
            let _ = MEM1_END.compare_exchange_weak(
                core::ptr::null_mut(),
                super::MEM1_END.read().cast_mut(),
                Ordering::SeqCst,
                Ordering::Relaxed,
            );

            self.try_allocate(
                layout,
                MEM1_START.load(Ordering::Relaxed),
                MEM1_END.load(Ordering::Relaxed),
            )
        }

        fn try_mem2_allocate(&self, layout: Layout) -> Option<NonNull<[u8]>> {
            let _ = MEM2_START.compare_exchange_weak(
                core::ptr::null_mut(),
                super::MEM2_START.read(),
                Ordering::SeqCst,
                Ordering::Relaxed,
            );
            let _ = MEM2_END.compare_exchange_weak(
                core::ptr::null_mut(),
                super::MEM2_END.read().cast_mut(),
                Ordering::SeqCst,
                Ordering::Relaxed,
            );

            self.try_allocate(
                layout,
                MEM2_START.load(Ordering::Relaxed),
                MEM2_END.load(Ordering::Relaxed),
            )
        }
    }
}

const MEM1_START: VolAddress<*mut u8, Safe, Safe> = unsafe { VolAddress::new(0x8000_310C) };
const MEM1_END: VolAddress<*const u8, Safe, Safe> = unsafe { VolAddress::new(0x8000_3110) };

const MEM2_START: VolAddress<*mut u8, Safe, Safe> = unsafe { VolAddress::new(0x8000_3124) };
const MEM2_END: VolAddress<*const u8, Safe, Safe> = unsafe { VolAddress::new(0x8000_3128) };
