use crate::lock::{Gutex, GutexGroup, GutexWriteGuard};
use core::sync::atomic::{AtomicU16, AtomicU32};

/// Implementation of `thread` structure.
///
/// All thread **must** run to completion once execution has been started otherwise resource will be
/// leak if the thread is dropped while its execution currently in the kernel space.
///
/// We subtitute `TDP_NOSLEEPING` with `td_intr_nesting_level` since the only cases the thread
/// should not allow to sleep is when it being handle an interupt.
pub struct Thread {
    critical_sections: AtomicU32, // td_critnest
    active_interrupts: usize,     // td_intr_nesting_level
    active_mutexes: AtomicU16,    // td_locks
    sleeping: Gutex<usize>,       // td_wchan
}

impl Thread {
    /// # Safety
    /// This function does not do anything except initialize the struct memory. It is the caller
    /// responsibility to configure the thread after this so it have a proper states and trigger
    /// necessary events.
    pub unsafe fn new_bare() -> Self {
        // This function is not allowed to access the CPU context due to it can be called before the
        // context has been activated.
        //
        // td_critnest on the PS4 started with 1 but this does not work in our case because we use
        // RAII to increase and decrease it.
        let gg = GutexGroup::new();

        Self {
            critical_sections: AtomicU32::new(0),
            active_interrupts: 0,
            active_mutexes: AtomicU16::new(0),
            sleeping: gg.spawn(0),
        }
    }

    /// See [`crate::context::Context::pin()`] for a safe wrapper.
    ///
    /// # Safety
    /// This is a counter. Each increment must paired with a decrement. Failure to do so will cause
    /// the whole system to be in an undefined behavior.
    pub unsafe fn critical_sections(&self) -> &AtomicU32 {
        &self.critical_sections
    }

    pub fn active_interrupts(&self) -> usize {
        self.active_interrupts
    }

    pub fn active_mutexes(&self) -> &AtomicU16 {
        &self.active_mutexes
    }

    /// Sleeping address. Zero if this thread is not in a sleep queue.
    pub fn sleeping_mut(&self) -> GutexWriteGuard<usize> {
        self.sleeping.write()
    }
}
