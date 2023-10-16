

// Will optimize later :)
// Change to Arc if needed
// pub type ImmutableString = Rc<str>;

#[inline]
#[must_use]
pub const fn id2<A, B>(a: A, b: B) -> (A, B) {
  (a, b)
}
