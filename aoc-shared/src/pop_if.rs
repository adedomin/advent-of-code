/// This feature adds the Vec::pop_if method, which takes a predicate, evaluates it with the last element in the Vec if present, and returns the item if the predicate returns true. This makes it possible to conditionally remove the last element without the use of unwrap.
///
/// See: https://github.com/rust-lang/rust/issues/122741
pub fn pop_if<T, F>(v: &mut Vec<T>, f: F) -> Option<T>
where
    F: FnOnce(&mut T) -> bool,
{
    let last = v.last_mut()?;
    if f(last) {
        v.pop()
    } else {
        None
    }
}
