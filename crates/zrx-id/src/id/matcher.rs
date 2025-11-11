// Copyright (c) 2025 Zensical and contributors

// SPDX-License-Identifier: MIT
// Third-party contributions licensed under DCO

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

//! Matcher.

use globset::GlobSet;
use std::str::FromStr;

use super::ToId;

mod builder;
mod error;
mod selector;

pub use builder::Builder;
pub use error::{Error, Result};
pub use selector::{Selector, ToSelector};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Matcher.
///
/// The [`Matcher`] provides efficient [`Selector`] matching of identifiers by
/// leveraging the [`globset`] crate. Matchers can be built from an arbitrary
/// number of selectors, which are then combined into a single [`GlobSet`] for
/// each of the five components.
///
/// [`GlobSet`] implements matching using deterministic finite automata (DFA),
/// which allow for efficient matching of multiple selectors against a single
/// identifier in linear time in relation to the length of the input string,
/// and which return the set of matched selectors.
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use zrx_id::{Id, Matcher};
///
/// // Create matcher builder and add selector
/// let mut builder = Matcher::builder();
/// builder.add("zrs:::::**/*.md:")?;
///
/// // Create matcher from builder
/// let matcher = builder.build()?;
///
/// // Create identifier and match selector
/// let id: Id = "zri:file:::docs:index.md:".parse()?;
/// assert!(matcher.is_match(&id)?);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Matcher {
    /// Glob set for provider.
    provider: GlobSet,
    /// Glob set for resource.
    resource: GlobSet,
    /// Glob set for variant.
    variant: GlobSet,
    /// Glob set for context.
    context: GlobSet,
    /// Glob set for location.
    location: GlobSet,
    /// Glob set for selector.
    fragment: GlobSet,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Matcher {
    /// Creates a matcher builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use zrx_id::Matcher;
    ///
    /// // Create matcher builder
    /// let mut builder = Matcher::builder();
    /// ```
    #[inline]
    #[must_use]
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Returns whether the given identifier matches any selector.
    ///
    /// Components are compared in descending variability and their likelihood
    /// for mismatch, starting with the `location`. This approach effectively
    /// tries to short-circuits the comparison. Note that empty components are
    /// considered wildcards, so they will always match.
    ///
    /// # Errors
    ///
    /// This method returns an error if the given identifier is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use zrx_id::{Id, Matcher};
    ///
    /// // Create matcher builder and add selector
    /// let mut builder = Matcher::builder();
    /// builder.add("zrs:::::**/*.md:")?;
    ///
    /// // Create matcher from builder
    /// let matcher = builder.build()?;
    ///
    /// // Create identifier and match selector
    /// let id: Id = "zri:file:::docs:index.md:".parse()?;
    /// assert!(matcher.is_match(&id)?);
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_match<I>(&self, id: I) -> Result<bool>
    where
        I: ToId,
    {
        let id = id.to_id()?;

        // Compare components in descending variability
        Ok(compare(&self.location, Some(id.location().as_ref()))
            && compare(&self.context, Some(id.context().as_ref()))
            && compare(&self.provider, Some(id.provider().as_ref()))
            && compare(&self.resource, id.resource().as_deref())
            && compare(&self.fragment, id.fragment().as_deref())
            && compare(&self.variant, id.variant().as_deref()))
    }

    /// Returns the match set of the selectors that match the identifier.
    ///
    /// This method compares each component of the identifier against the
    /// corresponding component of a selector using the compiled globs, and
    /// returns the indices of the matching selectors in the order they were
    /// added to the [`Matcher`].
    ///
    /// Components are compared in descending variability and their likelihood
    /// for mismatch, starting with the `location`. This approach effectively
    /// tries to short-circuits the comparison. Note that empty components are
    /// considered wildcards, so they will always match.
    ///
    /// # Errors
    ///
    /// This method returns an error if the given identifier is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use zrx_id::{Id, Matcher};
    ///
    /// // Create matcher builder and add selector
    /// let mut builder = Matcher::builder();
    /// builder.add("zrs:::::**/*.md:")?;
    ///
    /// // Create matcher from builder
    /// let matcher = builder.build()?;
    ///
    /// // Create identifier and obtain matched selectors
    /// let id: Id = "zri:file:::docs:index.md:".parse()?;
    /// assert_eq!(matcher.matches(&id)?, [0]);
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::if_not_else)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn matches<I>(&self, id: I) -> Result<Vec<usize>>
    where
        I: ToId,
    {
        let id = id.to_id()?;

        // Create a vector and count the matches of each component in the slots
        // of the vector to find all selectors that match the given identifier
        let mut slots = vec![0u8; self.provider.len()];
        for (component, value) in [
            (&self.location, Some(id.location())),
            (&self.context, Some(id.context())),
            (&self.provider, Some(id.provider())),
            (&self.resource, id.resource()),
            (&self.fragment, id.fragment()),
            (&self.variant, id.variant()),
        ] {
            if let Some(value) = value {
                let matches = component.matches(value.as_ref());
                if !matches.is_empty() {
                    for index in matches {
                        slots[index] += 1;
                    }

                // Short-circuit, as the current component doesn't match, so we
                // know the result must be empty and can return immediately
                } else {
                    return Ok(Vec::new());
                }

            // Wildcard match, which means all slots must be updated
            } else {
                for count in &mut slots {
                    *count += 1;
                }
            }
        }

        // Obtain match set by collecting the indices of all matching selectors,
        // which are the slots that match exactly five components
        let iter = slots
            .iter()
            .enumerate()
            .filter_map(|(index, &count)| (count == 6).then_some(index));

        // Return match set
        Ok(iter.collect())
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl FromStr for Matcher {
    type Err = Error;

    /// Creates a matcher from a string.
    ///
    /// The string must adhere to the following format and include exactly six
    /// `:` separators, even if some components are empty. All components are
    /// optional, which means they can be left empty, which is equivalent to
    /// setting them to a `**` wildcard.
    ///
    /// ``` text
    /// zrs:<provider>:<resource>:<variant>:<context>:<location>:<fragment>
    /// ```
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Path`], if a component value contains a
    /// backslash, or [`Error::Format`], if the format is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use zrx_id::Matcher;
    ///
    /// // Create matcher from string
    /// let matcher: Matcher = "zrs:::::**/*.md:".parse()?;
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(value: &str) -> Result<Self> {
        Matcher::builder().with(value)?.build()
    }
}

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

/// Compares a component against a value.
///
/// If the value is absent, we must consider this as a wildcard match if and
/// only if the globset was initially constructed with wildcards (i.e. `**`).
/// Unfortunately, this information is not retained in the globset, and we do
/// not want to use more space than necessary to track empty components.
///
/// However, falling back to `U+FFFE`, which is a non-character that should
/// never appear in a proper UTF-8 string should be sufficient for the check.
fn compare(component: &GlobSet, value: Option<&str>) -> bool {
    component.is_match(value.unwrap_or("\u{FFFE}"))
}
