use crate::traits;
use crate::traits::project::Normalized;
use rustc_middle::ty::fold::{FallibleTypeFolder, TypeFoldable};
use rustc_middle::ty::visit::{TypeVisitable, TypeVisitor};
use rustc_middle::ty::{self, TyCtxt};

use std::fmt;
use std::ops::ControlFlow;

// Structural impls for the structs in `traits`.

impl<'tcx, T: fmt::Debug> fmt::Debug for Normalized<'tcx, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Normalized({:?}, {:?})", self.value, self.obligations)
    }
}

impl<'tcx, O: fmt::Debug> fmt::Debug for traits::Obligation<'tcx, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if ty::tls::with(|tcx| tcx.sess.verbose_internals()) {
            write!(
                f,
                "Obligation(predicate={:?}, cause={:?}, param_env={:?}, depth={})",
                self.predicate, self.cause, self.param_env, self.recursion_depth
            )
        } else {
            write!(f, "Obligation(predicate={:?}, depth={})", self.predicate, self.recursion_depth)
        }
    }
}

impl<'tcx> fmt::Debug for traits::FulfillmentError<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FulfillmentError({:?},{:?})", self.obligation, self.code)
    }
}

impl<'tcx> fmt::Debug for traits::FulfillmentErrorCode<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use traits::FulfillmentErrorCode::*;
        match *self {
            SelectionError(ref e) => write!(f, "{e:?}"),
            ProjectionError(ref e) => write!(f, "{e:?}"),
            SubtypeError(ref a, ref b) => {
                write!(f, "CodeSubtypeError({a:?}, {b:?})")
            }
            ConstEquateError(ref a, ref b) => {
                write!(f, "CodeConstEquateError({a:?}, {b:?})")
            }
            Ambiguity { overflow: None } => write!(f, "Ambiguity"),
            Ambiguity { overflow: Some(suggest_increasing_limit) } => {
                write!(f, "Overflow({suggest_increasing_limit})")
            }
            Cycle(ref cycle) => write!(f, "Cycle({cycle:?})"),
        }
    }
}

impl<'tcx> fmt::Debug for traits::MismatchedProjectionTypes<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MismatchedProjectionTypes({:?})", self.err)
    }
}

///////////////////////////////////////////////////////////////////////////
// TypeFoldable implementations.

impl<'tcx, O: TypeFoldable<TyCtxt<'tcx>>> TypeFoldable<TyCtxt<'tcx>>
    for traits::Obligation<'tcx, O>
{
    fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        Ok(traits::Obligation {
            cause: self.cause,
            recursion_depth: self.recursion_depth,
            predicate: self.predicate.try_fold_with(folder)?,
            param_env: self.param_env.try_fold_with(folder)?,
        })
    }
}

impl<'tcx, O: TypeVisitable<TyCtxt<'tcx>>> TypeVisitable<TyCtxt<'tcx>>
    for traits::Obligation<'tcx, O>
{
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> ControlFlow<V::BreakTy> {
        self.predicate.visit_with(visitor)?;
        self.param_env.visit_with(visitor)
    }
}
