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

//! Session message.

use crate::scheduler::action::output::OutputItem;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Session message.
///
/// This data type is used to associate an item or drop notification with the
/// session identifier, so that the owner of the [`Connector`][] can identify
/// which session a message was sent from, which is essential for attributing
/// the item to the correct [`Descriptor`][].
///
/// [`Connector`]: crate::scheduler::session::Connector
/// [`Descriptor`]: crate::scheduler::graph::Descriptor
#[derive(Debug)]
pub enum Message<I> {
    /// Item insert or remove.
    Item(usize, OutputItem<I>),
    /// Drop notification.
    Drop(usize),
}
