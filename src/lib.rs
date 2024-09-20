use std::fmt::Display;
use std::error::Error;

use tracing_error::SpanTrace;

#[derive(Debug, Clone)]
pub struct SpanErr<T: Error + Display> {
    pub error: T,
    pub span: SpanTrace,
}

impl<T: Error + Display> Display for SpanErr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.error, f)
    }
}

impl<T: Error + Display> Error for SpanErr<T> {}

impl<T: Error + Display> SpanErr<T> {
    pub fn map<U: Error + Display>(self, f: impl FnOnce(T) -> U) -> SpanErr<U> {
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

#[cfg(test)]
mod tests {

}
