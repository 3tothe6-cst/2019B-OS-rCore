pub use self::mutex::{Mutex as SleepLock, MutexGuard as SleepLockGuard};

pub mod condvar;
mod mutex;
