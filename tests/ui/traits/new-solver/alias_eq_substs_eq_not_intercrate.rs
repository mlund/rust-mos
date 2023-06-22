// compile-flags: -Ztrait-solver=next

// check-pass
// (should not pass, should be turned into a coherence-only test)

// check that a `alias-eq(<?0 as TraitB>::Assoc, <T as TraitB>::Assoc)` goal fails.

// FIXME(deferred_projection_equality): add a test that this is true during coherence

trait TraitB {
    type Assoc;
}

fn needs_a<T: TraitB>() -> T::Assoc {
    unimplemented!()
}

fn bar<T: TraitB>() {
    let _: <_ as TraitB>::Assoc = needs_a::<T>();
}

fn main() {}
