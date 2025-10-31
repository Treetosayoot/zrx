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

//! Visitor for ancestors of a node.

use ahash::HashSet;

use crate::graph::topology::Topology;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Visitor for ancestors of a node.
pub struct Ancestors<'a> {
    /// Graph topology.
    topology: &'a Topology,
    /// Stack for depth-first search.
    stack: Vec<usize>,
    /// Set of visited nodes.
    visited: HashSet<usize>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<'a> Ancestors<'a> {
    /// Creates a visitor that yields all ancestors of the given node.
    #[must_use]
    pub fn new(topology: &'a Topology, node: usize) -> Self {
        Self {
            topology,
            stack: Vec::from([node]),
            visited: HashSet::default(),
        }
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Iterator for Ancestors<'_> {
    type Item = usize;

    /// Returns the next ancestor.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use zrx_graph::visitor::Ancestors;
    /// use zrx_graph::Graph;
    ///
    /// // Create graph builder and add nodes
    /// let mut builder = Graph::builder();
    /// let a = builder.add_node("a");
    /// let b = builder.add_node("b");
    /// let c = builder.add_node("c");
    ///
    /// // Create edges between nodes
    /// builder.add_edge(a, b, 0)?;
    /// builder.add_edge(b, c, 0)?;
    ///
    /// // Create graph from builder
    /// let graph = builder.build();
    ///
    /// // Create iterator over ancestors
    /// let mut ancestors = Ancestors::new(graph.topology(), c);
    /// while let Some(ancestor) = ancestors.next() {
    ///     println!("{ancestor:?}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let incoming = self.topology.incoming();

        // Perform a depth-first search to find all ancestors, using a stack
        // over recursion, as it's faster and more efficient memory-wise
        while let Some(node) = self.stack.pop() {
            for &ancestor in &incoming[node] {
                // If we haven't visited this ancestor yet, we put it on the
                // stack after marking it as visited and return it immediately
                if self.visited.insert(ancestor) {
                    self.stack.push(ancestor);
                    return Some(ancestor);
                }
            }
        }

        // No more ancestors to visit
        None
    }
}
