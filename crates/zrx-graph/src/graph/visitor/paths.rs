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

//! Visitor for paths between two nodes.

use crate::graph::topology::Topology;
use crate::graph::Graph;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Visitor for paths between two nodes.
pub struct Paths<'a> {
    /// Graph topology.
    topology: &'a Topology,
    /// Target node.
    target: usize,
    /// Stack for depth-first search.
    stack: Vec<(usize, usize)>,
    /// Current path.
    path: Vec<usize>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<T> Graph<T> {
    /// Creates an iterator over all paths between the given nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    /// // Create iterator over paths
    /// for path in graph.paths(a, c) {
    ///     println!("{path:?}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn paths(&self, source: usize, target: usize) -> Paths<'_> {
        Paths {
            topology: &self.topology,
            target,
            stack: Vec::from([(source, 0)]),
            path: Vec::from([source]),
        }
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Iterator for Paths<'_> {
    type Item = Vec<usize>;

    /// Returns the next path.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    /// // Create iterator over paths
    /// let mut paths = graph.paths(a, c);
    /// while let Some(path) = paths.next() {
    ///     println!("{path:?}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let outgoing = self.topology.outgoing();

        // Perform a depth-first search to find all paths from the source to
        // the target, and emit them in the order of discovery
        while let Some((node, depth)) = self.stack.pop() {
            // Backtrack by truncating the current path to the depth of the
            // current node, and then add the current node to the path
            self.path.truncate(depth);
            self.path.push(node);

            // In case we've reached the target, yield the current path. Note
            // that we need to clone it, since we can't return a reference
            if node == self.target {
                return Some(self.path.clone());
            }

            // Add descendants to stack in reverse order for consistent depth-
            // first ordering. Additionally, perform a debug assertion to ensure
            // that we don't revisit nodes within the current path, which would
            // lead to infinite loops, but should never happen in a DAG.
            for &descendant in outgoing[node].iter().rev() {
                debug_assert!(!self.path.contains(&descendant));
                self.stack.push((descendant, depth + 1));
            }
        }

        // No more paths to visit
        None
    }
}
