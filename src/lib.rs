pub mod result_ext;

pub use result_ext::{ResultTracingExt, SpannedResultExt};
use std::error::Error;
use std::fmt::Display;
use tracing_error::SpanTrace;

/// [`tracing_error::SpanTrace`] wrapped with an error.
///
///
/// By [`ResultTracingExt::in_current_span`], you can easily wrap an error with a [`SpanTrace`].
/// ```
/// use tracing_spanned::{SpanErr, ResultTracingExt};
///
/// let my_string = String::from("abc");
/// let number: Result<u32, SpanErr<_>> = my_string.parse::<u32>().in_current_span();
/// ```
///
/// SpanErr does not implement [`std::error::Error`] trait because SpanErr does not have any
/// information about the error directly.
///
/// It also prevents double wrapping of the same error.
/// (Following code will not compile, because [`SpanErr`] does not implement [`std::error::Error`])
/// ```compile_fail
/// use tracing_spanned::{SpanErr, ResultTracingExt};
///
/// let my_string = String::from("abc");
/// let number: Result<u32, SpanErr<SpanErr<_>>>
///     = my_string.parse::<u32>()
///         .in_current_span()
///         .in_current_span();
/// ```
#[derive(Debug, Clone)]
pub struct SpanErr<T: Error> {
    pub error: T,
    pub span: SpanTrace,
}

impl<T: Error> Display for SpanErr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.error, f)
    }
}

impl<T: Error> SpanErr<T> {
    #[inline(always)]
    pub fn map<U: Error>(self, f: impl FnOnce(T) -> U) -> SpanErr<U> {
        SpanErr {
            error: f(self.error),
            span: self.span,
        }
    }
}

impl<T: Error + Display> From<T> for SpanErr<T> {
    fn from(error: T) -> Self {
        Self {
            error,
            span: SpanTrace::capture(),
        }
    }
}
