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

//! Graph algorithms related to ancestors.

use ahash::HashSet;

use crate::graph::visitor::Ancestors;
use crate::graph::Graph;

use super::path::shortest_path_length;

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

/// Returns the lowest common ancestor of the given nodes in the graph.
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use zrx_graph::algorithm::lowest_common_ancestor;
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
/// // Obtain lowest common ancestor
/// let ancestor = lowest_common_ancestor(&graph, [b, c]);
/// assert_eq!(ancestor, Some(a));
/// # Ok(())
/// # }
/// ```
pub fn lowest_common_ancestor<T, I>(graph: &Graph<T>, nodes: I) -> Option<usize>
where
    I: IntoIterator<Item = usize>,
{
    let topology = graph.topology();

    // If there are fewer than two nodes, we return immediately
    let nodes = nodes.into_iter().collect::<Vec<_>>();
    if nodes.len() < 2 {
        return None;
    }

    // Collect all ancestors for each node, and compute the intersection of all
    // sets of ancestors, resulting in a set of all common ancestors
    let iter = nodes.iter();
    let ancestors = iter
        .map(|&node| Ancestors::new(topology, node).collect::<HashSet<_>>())
        .reduce(|a, b| a.intersection(&b).copied().collect())?;

    // Find the ancestor with the shortest path to any of the nodes, or return
    // nothing if the intersection is empty, as there is no common ancestor
    ancestors.into_iter().min_by_key(|&ancestor| {
        let iter = nodes.iter();
        iter.filter_map(|&node| shortest_path_length(graph, ancestor, node))
            .min()
    })
}
