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

//! Identifier conversions.

use std::borrow::Cow;

use super::error::Result;
use super::Id;

// ----------------------------------------------------------------------------
// Traits
// ----------------------------------------------------------------------------

/// Conversion to [`Id`].
///
/// This trait allows to convert an arbitrary value into an identifier, using a
/// [`Cow`] smart pointer to avoid unnecessary cloning, e.g. for references.
pub trait ToId {
    /// Converts to an identifier.
    #[allow(clippy::missing_errors_doc)]
    fn to_id(&self) -> Result<Cow<'_, Id>>;
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl ToId for &Id {
    /// Creates an identifier from a reference.
    #[inline]
    fn to_id(&self) -> Result<Cow<'_, Id>> {
        Ok(Cow::Borrowed(self))
    }
}

// ----------------------------------------------------------------------------
// Blanket implementations
// ----------------------------------------------------------------------------

impl<T> ToId for T
where
    T: AsRef<str>,
{
    /// Creates an identifier from a string.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Path`], if a component value contains a
    /// backslash or traversal, or [`Error::Format`], if the format is invalid.
    ///
    /// [`Error::Format`]: crate::id::Error::Format
    /// [`Error::Path`]: crate::id::Error::Path
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use zrx_id::{Id, ToId};
    ///
    /// // Create identifier from string
    /// let id = "zri:file:::docs:index.md:".to_id()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn to_id(&self) -> Result<Cow<'_, Id>> {
        self.as_ref().parse().map(Cow::Owned)
    }
}
