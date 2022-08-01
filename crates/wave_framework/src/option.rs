//! `Option` extensions

// LICENCE
//
// This is free and unencumbered software released into the public domain.
//
// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.
//
// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// For more information, please refer to <https://unlicense.org>

/// Takes two `Option`s and returns a single
/// `Option` wrapping both inner values.
///
/// If either `Option` is `None`, the result
/// will also be `None`.
///
/// # Example
///
/// ```ignore
/// let a = Some("a");
/// let b = Some(1);
/// let c = lift2(a, b);
///
/// assert_eq!(Some(("a", 1)), c);
/// ```
#[inline]
pub fn lift2<A, B>(a: Option<A>, b: Option<B>) -> Option<(A, B)> {
    a.and_then(|ai| b.map(|bi| (ai, bi)))
}

/// Takes three `Option`s and returns a single
/// `Option` wrapping both inner values.
///
/// If any `Option` is `None`, the result
/// will also be `None`.
///
/// # Example
///
/// ```ignore
/// let a = Some("a");
/// let b = Some(1);
/// let c = Some(false);
/// let d = lift3(a, b, c);
///
/// assert_eq!(Some(("a", 1, false)), d);
/// ```
#[inline]
pub fn lift3<A, B, C>(a: Option<A>, b: Option<B>, c: Option<C>) -> Option<(A, B, C)> {
    a.and_then(|ai| b.and_then(|bi| c.map(|ci| (ai, bi, ci))))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lift2() {
        assert_eq!(Some(("a", 1)), lift2(Some("a"), Some(1)));
        assert_eq!(None, lift2::<&str, i32>(None, Some(1)));
        assert_eq!(None, lift2::<&str, i32>(Some("a"), None));
        assert_eq!(None, lift2::<&str, i32>(None, None));
    }

    #[test]
    fn test_lift3() {
        assert_eq!(Some(("a", 1, true)), lift3(Some("a"), Some(1), Some(true)));
        assert_eq!(None, lift3::<&str, i32, bool>(None, Some(1), Some(true)));
        assert_eq!(None, lift3::<&str, i32, bool>(Some("a"), None, Some(true)));
        assert_eq!(None, lift3::<&str, i32, bool>(None, None, Some(true)));
        assert_eq!(None, lift3::<&str, i32, bool>(None, Some(1), None));
        assert_eq!(None, lift3::<&str, i32, bool>(Some("a"), Some(1), None));
    }
}
