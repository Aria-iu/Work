/*
RAII代表"Resource Acquisition is Initialisation"，”资源获取即初始化“。 
该模式的本质是，资源初始化在对象的构造器中完成，最终化（资源释放）在析构器中完成。 这种模式在Rust中得到了扩展，
即使用RAII对象作为某些资源的守护对象，并依靠类型系统来确保访问总是由守护对象来调解
*/

use std::ops::Deref;

struct Foo {}

struct Mutex<T> {
    // We keep a reference to our data: T here.
    //..
}

struct MutexGuard<'a, T: 'a> {
    data: &'a T,
    //..
}

// Locking the mutex is explicit.
impl<T> Mutex<T> {
    // 在Mutex上进行lock，返回MutexGuard
    // 返回的MutexGuard与self具有相同的生命周期('a)。 
    // 因此，借用检查器确保对MutexGuard的生命周期短于（不超过）self的生命周期
    fn lock(&self) -> MutexGuard<T> {
        // Lock the underlying OS mutex.
        //..

        // MutexGuard keeps a reference to self
        // Mutex<T> into &T ?
        MutexGuard {
            data: self,
            //..
        }
    }
}

// Destructor for unlocking the mutex.
impl<'a, T> Drop for MutexGuard<'a, T> {
    // MutexGuard生命周期结束后，只是释放OS下的🔓，但是不会释放资源（data）。
    fn drop(&mut self) {
        // Unlock the underlying OS mutex.
        //..
    }
}

// Implementing Deref means we can treat MutexGuard like a pointer to T.
impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    // 返回的资源引用与self具有相同的生命周期('a)。 
    // 因此，借用检查器确保对T的引用的生命周期短于（不超过）self的生命周期。
    fn deref(&self) -> &T {
        self.data
    }
}

fn baz(x: Mutex<Foo>) {
    let xx = x.lock();
    xx.foo(); // foo is a method on Foo.
    // The borrow checker ensures we can't store a reference to the underlying
    // Foo which will outlive the guard xx.

    // x is unlocked when we exit this function and xx's destructor is executed.
}
