/// This is a straight copy-paste job from https://github.com/rust-lang/rust/pull/75026/commits/f240abc1dc9
/// I don't care about "runtime panic" for N == 0 case, unlike upstream.
#[derive(Debug, Clone, Copy)]
pub struct ArrayWindows<'a, T: 'a, const N: usize> {
    pub(crate) slice_head: *const T,
    pub(crate) num: usize,
    pub(crate) marker: std::marker::PhantomData<&'a [T; N]>,
}

impl<'a, T, const N: usize> Iterator for ArrayWindows<'a, T, N> {
    type Item = &'a [T; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.num == 0 {
            return None;
        }
        // SAFETY:
        // This is safe because it's indexing into a slice guaranteed to be length > N.
        let ret = unsafe { &*self.slice_head.cast::<[T; N]>() };
        // SAFETY: Guaranteed that there are at least 1 item remaining otherwise
        // earlier branch would've been hit
        self.slice_head = unsafe { self.slice_head.add(1) };

        self.num -= 1;
        Some(ret)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.num, Some(self.num))
    }

    #[inline]
    fn count(self) -> usize {
        self.num
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.num <= n {
            self.num = 0;
            return None;
        }
        // SAFETY:
        // This is safe because it's indexing into a slice guaranteed to be length > N.
        let ret = unsafe { &*self.slice_head.add(n).cast::<[T; N]>() };
        // SAFETY: Guaranteed that there are at least n items remaining
        self.slice_head = unsafe { self.slice_head.add(n + 1) };

        self.num -= n + 1;
        Some(ret)
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.nth(self.num.checked_sub(1)?)
    }
}

impl<'a, T, const N: usize> DoubleEndedIterator for ArrayWindows<'a, T, N> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T; N]> {
        if self.num == 0 {
            return None;
        }
        // SAFETY: Guaranteed that there are n items remaining, n-1 for 0-indexing.
        let ret = unsafe { &*self.slice_head.add(self.num - 1).cast::<[T; N]>() };
        self.num -= 1;
        Some(ret)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<&'a [T; N]> {
        if self.num <= n {
            self.num = 0;
            return None;
        }
        // SAFETY: Guaranteed that there are n items remaining, n-1 for 0-indexing.
        let ret = unsafe { &*self.slice_head.add(self.num - (n + 1)).cast::<[T; N]>() };
        self.num -= n + 1;
        Some(ret)
    }
}

// this is unstable as well.
// impl<T, const N: usize> ExactSizeIterator for ArrayWindows<'_, T, N> {
//     fn is_empty(&self) -> bool {
//         self.num == 0
//     }
// }

/// unlike upstream, this is a free function and not attached to slice impl.
pub fn array_windows<'a, T, const N: usize>(data: &'a [T]) -> ArrayWindows<'a, T, N> {
    assert_ne!(N, 0);

    let num_windows = data.len().saturating_sub(N - 1);
    ArrayWindows {
        slice_head: data.as_ptr(),
        num: num_windows,
        marker: std::marker::PhantomData,
    }
}
