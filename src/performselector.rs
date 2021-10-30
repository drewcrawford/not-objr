///Trait that provides `PerformSelector` implementations.  Autoimplelmented for `T: PerformablePointer`
///
/// # Stability
/// Do not implement this trait yourself.  Instead use [objc_instance!] or [objc_class!]
pub trait PerformsSelector  {
    ///Performs selector, returning a primitive type.
    /// # Safety
    /// See the safety section of [objc_instance!].
    unsafe fn perform_primitive() -> ();

    ///Performs, returning the specified [ObjcInstance].  You must coerce this into some type according to your knowledge of ObjC convention.
    unsafe fn perform();
    ///Performs, returning the result of the specified [ObjcInstance].  You must coerce this into some type according to your knowledge of ObjC convention.
    ///
    /// By convention, the error value is an autoreleased [NSError].
    ///
    ///# Safety
    ///See the safety section of [objc_instance!].
    unsafe fn perform_result() -> ();

    ///Performs, returning the specified [ObjcInstance].
    ///
    /// This variant assumes 1) the calling convention is +0, 2) the type returned to you is +1.  The implementation
    /// knows a trick to perform this conversion faster than you can do it manually.
    ///# Safety
    ///See the safety section of [objc_instance!].
    unsafe fn perform_autorelease_to_retain();

    ///Performs, returning the specified [ObjcInstance].
    ///
    /// This variant assumes 1) the calling convention is +0, 2) the type returned to you is +1.  The implementation
    /// knows a trick to perform this conversion faster than you can do it manually.
    ///By convention, the error value is an autoreleased [NSError].
    ///# Safety
    ///See the safety section of [objc_instance!].
    unsafe fn perform_result_autorelease_to_retain() -> ();
}




