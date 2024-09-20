use std::error::Error;

use crate::SpanErr;

pub trait ResultTracingExt<T, E: Error> {
    fn in_current_span(self) -> Result<T, SpanErr<E>>;
}

impl<T, E: Error> ResultTracingExt<T, E> for Result<T, E> {
    #[inline(always)]
    fn in_current_span(self) -> Result<T, SpanErr<E>> {
        self.map_err(|e| e.into())
    }
}

pub trait SpannedResultExt<T, E: Error> {
    fn spanned_map_err<E2: Error>(self, f: impl FnOnce(E) -> E2) -> Result<T, SpanErr<E2>>;
}

impl<T, E: Error> SpannedResultExt<T, E> for Result<T, SpanErr<E>> {
    #[inline(always)]
    fn spanned_map_err<E2: Error>(self, f: impl FnOnce(E) -> E2) -> Result<T, SpanErr<E2>> {
        self.map_err(|e| e.map(f))
    }
}
