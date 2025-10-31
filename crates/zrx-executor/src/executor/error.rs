// Copyright (c) Zensical LLC <https://zensical.org>

// SPDX-License-Identifier: MIT
// Third-party contributions licensed under CLA

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

// ----------------------------------------------------------------------------

//! Executor error.

use crossbeam::channel::TrySendError;
use std::result;
use thiserror::Error;

use super::task::Task;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Executor error.
#[derive(Debug, Error)]
pub enum Error {
    /// Task submission failed.
    #[error("task submission failed")]
    Submit(Box<dyn Task>),

    /// Signal poisoned.
    #[error("signal poisoned")]
    Signal,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl From<TrySendError<Box<dyn Task>>> for Error {
    /// Creates an error from a crossbeam channel error.
    ///
    /// This implementation extracts the [`Task`] that could not be submitted,
    /// and wraps it in an [`Error::Submit`] variant for a later retry. To our
    /// current knowledge, it can't possibly happen that the channel becomes
    /// disconnected without explicitly terminating the executor.
    #[inline]
    fn from(err: TrySendError<Box<dyn Task>>) -> Self {
        Error::Submit(err.into_inner())
    }
}

// ----------------------------------------------------------------------------
// Type aliases
// ----------------------------------------------------------------------------

/// Executor result.
pub type Result<T = ()> = result::Result<T, Error>;
