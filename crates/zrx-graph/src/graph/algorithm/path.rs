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

//! Graph algorithms related to paths.

use ahash::HashSet;
use std::collections::VecDeque;

use crate::graph::Graph;

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

/// Returns the length of the shortest path between two nodes in the graph.
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use zrx_graph::algorithm::shortest_path_length;
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
/// builder.add_edge(a, c, 0)?;
///
/// // Create graph from builder
/// let graph = builder.build();
///
/// // Obtain shortest path length
/// let len = shortest_path_length(&graph, a, c);
/// assert_eq!(len, Some(1));
/// # Ok(())
/// # }
/// ```
#[must_use]
pub fn shortest_path_length<T>(
    graph: &Graph<T>, source: usize, target: usize,
) -> Option<usize> {
    let outgoing = graph.topology().outgoing();

    // Initialize set to track visited nodes
    let mut visited = HashSet::default();

    // Perform a breadth-first search to find the shortest path between the
    // two given nodes within the graph, starting at the source node. Breadth-
    // first search guarantees, that we reach the target node on the shortest
    // path, which makes this implementation ridiculously simple.
    let mut queue = VecDeque::from([(source, 0)]);
    while let Some((node, len)) = queue.pop_front() {
        if node == target {
            return Some(len);
        }

        // Add unvisited descendants to the queue, since we're performing a
        // breadth-first search, and mark them as visited
        for &descendant in &outgoing[node] {
            if visited.insert(descendant) {
                queue.push_back((descendant, len + 1));
            }
        }
    }

    // No path between nodes found
    None
}
