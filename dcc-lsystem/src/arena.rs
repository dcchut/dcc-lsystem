use std::slice::{Iter, IterMut};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ArenaId(pub usize);

/// A simple arena wrapping around a Vec<T>.
///
/// # Examples
///
/// ```rust
/// use dcc_lsystem::Arena;
///
/// let mut arena = Arena::new();
///
/// let u = arena.push(1);
/// let v = arena.push(2);
///
/// assert_eq!(arena.len(), 2);
/// assert_eq!(arena.get(u), Some(&1));
/// ```
#[derive(Debug, Clone)]
pub struct Arena<T> {
    arena: Vec<T>,
}

impl<T> Arena<T> {
    /// Creates a new empty arena.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::Arena;
    ///
    /// let mut arena = Arena::new();
    ///
    /// arena.push(1);
    /// arena.push(3);
    /// ```
    pub fn new() -> Self {
        Self { arena: Vec::new() }
    }

    /// Returns the length of this arena.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::Arena;
    ///
    /// let mut arena = Arena::new();
    ///
    /// for i in 0..420 {
    ///     arena.push(i);
    /// }
    ///
    /// assert_eq!(arena.len(), 420);
    /// ```
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    /// Returns `true` if the arena contains no elements.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::Arena;
    ///
    /// let mut arena = Arena::new();
    ///
    /// assert!(arena.is_empty());
    ///
    /// arena.push(3);
    ///
    /// assert!(!arena.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }

    /// Returns a reference to an entry of the arena,
    /// if the provided ArenaId is valid.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::Arena;
    ///
    /// let mut arena = Arena::new();
    ///
    /// let x = arena.push("x");
    /// let y = arena.push("y");
    /// let z = arena.push("z");
    ///
    /// assert_eq!(arena.get(y), Some(&"y"));
    /// ```
    pub fn get(&self, id: ArenaId) -> Option<&T> {
        self.arena.get(id.0)
    }

    /// Returns a mutable reference to the entry corresponding
    /// to the given index.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::Arena;
    ///
    /// let mut arena = Arena::new();
    ///
    /// let x = arena.push("x");
    ///
    /// if let Some(entry) = arena.get_mut(x) {
    ///     *entry = "y";
    /// }
    ///
    /// assert_eq!(arena.get(x), Some(&"y"));
    /// ```
    pub fn get_mut(&mut self, id: ArenaId) -> Option<&mut T> {
        self.arena.get_mut(id.0)
    }

    /// Returns an iterator over this arena.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::Arena;
    ///
    /// let mut arena = Arena::new();
    /// let x = arena.push(3);
    /// let y = arena.push(5);
    ///
    /// let mut iterator = arena.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&3));
    /// assert_eq!(iterator.next(), Some(&5));
    /// assert_eq!(iterator.next(), None)
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        self.arena.iter()
    }

    /// Returns an iterator that allows modifying each value.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::Arena;
    ///
    /// let mut arena = Arena::new();
    /// let x = arena.push(3);
    /// let y = arena.push(-4);
    ///
    /// for entry in arena.iter_mut() {
    ///     *entry = *entry * *entry;
    /// }
    ///
    /// let mut iterator = arena.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&9));
    /// assert_eq!(iterator.next(), Some(&16));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.arena.iter_mut()
    }

    /// Returns true if the provided id corresponds to an element of this arena.
    ///
    /// ```rust
    /// use dcc_lsystem::{Arena, ArenaId};
    ///
    /// let mut arena = Arena::new();
    /// let x = arena.push(17);
    /// let y = arena.push(21);
    ///
    /// assert!(arena.is_valid(x));
    /// assert!(arena.is_valid(y));
    ///
    /// assert!(!arena.is_valid(ArenaId(2)));
    /// ```
    pub fn is_valid(&self, id: ArenaId) -> bool {
        id.0 < self.arena.len()
    }

    /// Returns `true` if the every id in the provided slice is valid.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::{Arena, ArenaId};
    ///
    /// let mut arena = Arena::new();
    /// let x = arena.push(1);
    /// let y = arena.push(3);
    /// let z = arena.push(7);
    ///
    /// assert!(arena.is_valid_slice(&[x,y]));
    /// assert!(arena.is_valid_slice(&[x,y,z]));
    /// assert!(!arena.is_valid_slice(&[x,y,ArenaId(3)]));
    /// ```
    pub fn is_valid_slice(&self, slice: &[ArenaId]) -> bool {
        slice.iter().all(|id| self.is_valid(*id))
    }

    /// Add a new value to our arena.
    ///
    /// Returns an ArenaId which uniquely identifies this element of the arena.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::{Arena, ArenaId};
    ///
    /// let mut arena = Arena::new();
    /// let x = arena.push(11);
    /// let y = arena.push(-3);
    ///
    /// assert_eq!(x, ArenaId(0));
    /// assert_eq!(y, ArenaId(1));
    /// ```
    pub fn push(&mut self, value: T) -> ArenaId {
        self.arena.push(value);
        ArenaId(self.arena.len() - 1)
    }

    ///  Returns an EnumerableArena.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::Arena;
    ///
    /// let mut arena = Arena::new();
    /// let x = arena.push(3);
    /// let y = arena.push(5);
    /// let z = arena.push(-7);
    ///
    /// for (id, entry) in arena.enumerate() {
    ///     /* do some work here */
    /// }
    /// ```
    pub fn enumerate(&self) -> EnumerableArena<'_, T> {
        EnumerableArena {
            inner: &self,
            pos: 0,
        }
    }
}

/// An iterator that yields the current ArenaId and the element during iterator.
///
/// # Examples
///
/// ```rust
/// use dcc_lsystem::Arena;
///
/// let mut arena = Arena::new();
///
/// let x = arena.push(1);
/// let y = arena.push(2);
/// let z = arena.push(-3);
///
/// let mut enumerable = arena.enumerate();
///
/// assert_eq!(enumerable.next(), Some((x, &1)));
/// assert_eq!(enumerable.next(), Some((y, &2)));
/// assert_eq!(enumerable.next(), Some((z, &-3)));
/// assert_eq!(enumerable.next(), None);
/// ```
///
/// This method is prefer over calling enumerate() on arena.iter():
///
/// ```rust
/// use dcc_lsystem::{Arena, ArenaId};
///
/// let mut arena = Arena::new();
/// arena.push(1);
/// arena.push(-2);
/// arena.push(17);
///
/// // Good:
/// for (id, entry) in arena.enumerate() {
///     /* Do some work here */
/// }
///
/// // Less good:
/// for (index, entry) in arena.iter().enumerate() {
///     // Convert the raw index to an ArenaId
///     let id = ArenaId(index);
///
///     /* Do some work here */
/// }
/// ```
pub struct EnumerableArena<'a, T: 'a> {
    inner: &'a Arena<T>,

    // Current position of our iterator
    pos: usize,
}

impl<'a, T> Iterator for EnumerableArena<'a, T> {
    type Item = (ArenaId, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.inner.arena.len() {
            None
        } else {
            self.pos += 1;
            Some((
                ArenaId(self.pos - 1),
                self.inner.arena.get(self.pos - 1).unwrap(),
            ))
        }
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arena_basic() {
        let mut arena = Arena::new();

        let a = arena.push("Hello!");
        let b = arena.push("World");

        assert_eq!(a.0, 0);
        assert_eq!(b.0, 1);
        assert_eq!(arena.len(), 2);

        let a_ref = arena.get(a).expect("Failed to get a");

        assert_eq!(*a_ref, "Hello!");

        {
            let b_ref_mut = arena.get_mut(b).expect("Failed to get b");
            *b_ref_mut = "Jenkins";
        }

        assert_eq!(arena.get(b).unwrap(), &"Jenkins");
    }

    #[test]
    fn arena_iterator() {
        let mut arena = Arena::new();

        arena.push("my first entry");
        arena.push("my second entry");
        arena.push("my third entry");

        let mut iter = arena.iter();

        assert_eq!(iter.next(), Some(&"my first entry"));
        assert_eq!(iter.next(), Some(&"my second entry"));
        assert_eq!(iter.next(), Some(&"my third entry"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn arena_iterator_mut() {
        let mut arena = Arena::new();

        arena.push(1);
        arena.push(3);
        arena.push(-5);
        arena.push(7);

        // Square each entry in our arena
        for entry in arena.iter_mut() {
            *entry = *entry * *entry;
        }

        let mut iter = arena.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), Some(&25));
        assert_eq!(iter.next(), Some(&49));
    }

    #[test]
    fn arena_enumerate() {
        let mut arena = Arena::new();

        let a = arena.push(1);
        let b = arena.push(3);
        let c = arena.push(4);
        let d = arena.push(8);

        let mut enumerator = arena.enumerate();

        assert_eq!(enumerator.next(), Some((a, &1)));
        assert_eq!(enumerator.next(), Some((b, &3)));
        assert_eq!(enumerator.next(), Some((c, &4)));
        assert_eq!(enumerator.next(), Some((d, &8)));
    }
}
