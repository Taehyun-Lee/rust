use cell::UnsafeCell;
//use libc;
use mem;
pub struct Mutex { inner: UnsafeCell<::libc::pthread_mutex_t>}

#[inline]
pub unsafe fn raw(m: &Mutex) -> *mut ::libc::pthread_mutex_t {
	m.inner.get()
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

#[allow(dead_code)] /* Sys doesn't seem to be exported yet */

impl Mutex { 
	pub const fn new() -> Mutex {
		Mutex {inner: UnsafeCell::new(::libc::PTHREAD_MUTEX_INITIALIZER)} /* this should be POSIX, right? */
	}	
	#[inline]
	pub unsafe fn init(&mut self) {
		/*If  I was very confident about this I'd say more, but something something Issue #33770 ? */   
		let mut attr: ::libc::pthread_mutexattr_t = mem::uninitialized();
		let r = ::libc::pthread_mutexattr_init(&mut attr);
		debug_assert_eq!(r, 0);
		let r = ::libc::pthread_mutexattr_settype(&mut attr, ::libc::PTHREAD_MUTEX_NORMAL);
		debug_assert_eq!(r, 0);
		let r = ::libc::pthread_mutex_init(self.inner.get(), &attr);
		debug_assert_eq!(r, 0);
		let r = ::libc::pthread_mutexattr_destroy(&mut attr);
		debug_assert_eq!(r, 0);

	}
	#[inline]
	pub unsafe fn lock(&self) {
		let r = ::libc::pthread_mutex_lock(self.inner.get());
		debug_assert_eq!(r, 0);
	}
	#[inline]
	pub unsafe fn unlock(&self) {
		let r = ::libc::pthread_mutex_unlock(self.inner.get());
		debug_assert_eq!(r, 0);
	}
	#[inline]
	pub unsafe fn try_lock(&self) -> bool {
		::libc::pthread_mutex_trylock(self.inner.get()) == 0
	}
	#[inline]
	pub unsafe fn destroy(&self) {
		let r = ::libc::pthread_mutex_destroy(self.inner.get());
		debug_assert_eq!(r, 0);
	}
	}

pub struct ReentrantMutex { inner: UnsafeCell<::libc::pthread_mutex_t> }

unsafe impl Send for ReentrantMutex {}
unsafe impl Sync for ReentrantMutex {}

impl ReentrantMutex {
	pub unsafe fn uninitialized() -> ReentrantMutex {
		ReentrantMutex { inner: mem::uninitialized() }
	}

	pub unsafe fn init(&mut self) {
		let mut attr: ::libc::pthread_mutexattr_t = mem::uninitialized();
		let result = ::libc::pthread_mutexattr_init(&mut attr as *mut _);
		debug_assert_eq!(result, 0);
		let result = ::libc::pthread_mutexattr_settype(&mut attr as *mut _, ::libc::PTHREAD_MUTEX_RECURSIVE);
		debug_assert_eq!(result, 0);
		let result = ::libc::pthread_mutex_init(self.inner.get(), &attr as *const _);
		debug_assert_eq!(result, 0);
		let result = ::libc::pthread_mutexattr_destroy(&mut attr as *mut _);
		debug_assert_eq!(result, 0);
	}

	pub unsafe fn lock(&self) {
		let result = ::libc::pthread_mutex_lock(self.inner.get());
		debug_assert_eq!(result, 0);
	}

	#[inline]
	pub unsafe fn try_lock(&self) -> bool {
		::libc::pthread_mutex_trylock(self.inner.get()) == 0
	}

	pub unsafe fn unlock(&self) {
		let result = ::libc::pthread_mutex_unlock(self.inner.get());
		debug_assert_eq!(result, 0);
	}

	pub unsafe fn destroy(&self) {
		let result = ::libc::pthread_mutex_destroy(self.inner.get());
		debug_assert_eq!(result, 0);
	}
}


