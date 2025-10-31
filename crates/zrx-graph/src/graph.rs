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

//! Graph.

use std::ops::{Index, IndexMut};
use std::slice::Iter;

pub mod algorithm;
mod builder;
mod error;
pub mod topology;
pub mod traversal;
pub mod visitor;

pub use builder::Builder;
pub use error::{Error, Result};
use topology::Topology;
use traversal::Traversal;
use visitor::{Ancestors, Descendants, Paths};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Graph.
///
/// This data type represents a directed graph with nodes of type `T`, which is
/// optimized for very efficient traversal, since it offers lookups of nodes and
/// edges in O(1), i.e., constant time. It's built with the [`Graph::builder`]
/// method, which allows to add nodes and edges, before building the graph.
///
/// Note that this graph implementation is unweighted, which means edges do not
/// carry associated weights, something that we don't need for our case.
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
/// // Create topological traversal
/// let mut traversal = graph.traverse([a]);
/// while let Some(node) = traversal.take() {
///     println!("{node:?}");
///     traversal.complete(node)?;
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Graph<T> {
    /// Graph data.
    data: Vec<T>,
    /// Graph topology.
    topology: Topology,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<T> Graph<T> {
    /// Creates a graph builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use zrx_graph::Graph;
    ///
    /// // Create graph builder
    /// let mut builder = Graph::builder();
    /// let a = builder.add_node("a");
    /// let b = builder.add_node("b");
    ///
    /// // Create edges between nodes
    /// builder.add_edge(a, b, 0)?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn builder<W>() -> Builder<T, W>
    where
        W: Clone,
    {
        Builder::new()
    }

    /// Creates an empty graph.
    ///
    /// While an empty graph is not very useful, it's sometimes practical as a
    /// placeholder in documentation or examples, where a graph is expected.
    ///
    /// # Examples
    ///
    /// ```
    /// use zrx_graph::Graph;
    ///
    /// // Create empty graph
    /// let graph = Graph::empty();
    /// # let _: Graph<()> = graph;
    /// assert!(graph.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Builder::<T>::new().build()
    }

    /// Maps the nodes to a different type.
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
    /// // Create graph from builder and map data
    /// let graph = builder.build();
    /// graph.map(str::to_uppercase);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn map<F, U>(self, f: F) -> Graph<U>
    where
        F: FnMut(T) -> U,
    {
        Graph {
            data: self.data.into_iter().map(f).collect(),
            topology: self.topology,
        }
    }

    /// Creates a topogical traversal starting from the given initial nodes.
    ///
    /// This method creates a topological traversal of the graph, which allows
    /// to visit nodes in a topological order, i.e., visiting a node only after
    /// all its dependencies have been visited. The traversal is initialized
    /// with the given initial nodes, which are the starting points.
    ///
    /// Note that an arbitrary number of parallel traversals can be created
    /// from the same graph, as the underlying topology is shared between them.
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
    /// // Create topological traversal
    /// let mut traversal = graph.traverse([a]);
    /// while let Some(node) = traversal.take() {
    ///     println!("{node:?}");
    ///     traversal.complete(node)?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn traverse<I>(&self, initial: I) -> Traversal
    where
        I: IntoIterator<Item = usize>,
    {
        Traversal::new(&self.topology, initial)
    }

    /// Creates an iterator over the sources of the graph.
    ///
    /// This method returns an iterator over the source node indices of the
    /// graph, which are the nodes with no incoming edges.
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
    /// // Create iterator over sources
    /// for node in graph.sources() {
    ///     println!("{node:?}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn sources(&self) -> impl Iterator<Item = usize> {
        let incoming = self.topology.incoming();
        incoming.iter().filter(|&node| incoming[node].is_empty())
    }

    /// Creates an iterator over the sinks of the graph.
    ///
    /// This method returns an iterator over the sink node indices of the
    /// graph, which are the nodes with no incoming edges.
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
    /// // Create iterator over sinks
    /// for node in graph.sinks() {
    ///     println!("{node:?}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn sinks(&self) -> impl Iterator<Item = usize> {
        let outgoing = self.topology.outgoing();
        outgoing.iter().filter(|&node| outgoing[node].is_empty())
    }

    /// Creates an iterator over the ancestors of the given node.
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
    /// // Create iterator over ancestors
    /// for node in graph.ancestors(c) {
    ///     println!("{node:?}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn ancestors(&self, node: usize) -> Ancestors<'_> {
        Ancestors::new(&self.topology, node)
    }

    /// Creates an iterator over the descendants of the given node.
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
    /// // Create iterator over descendants
    /// for node in graph.descendants(a) {
    ///     println!("{node:?}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn descendants(&self, node: usize) -> Descendants<'_> {
        Descendants::new(&self.topology, node)
    }

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
        Paths::new(&self.topology, source, target)
    }

    /// Creates an iterator over the graph.
    ///
    /// This iterator yields the data `T` associated with each node. If you need
    /// to iterate over the node indices of a graph, use [`Graph::topology`] to
    /// obtain the [`Topology::incoming`] or [`Topology::outgoing`] adjacency
    /// list, and iterate over those.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use zrx_graph::topology::Adjacency;
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
    /// // Create iterator over data
    /// for data in graph.iter() {
    ///     println!("{data:?}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }
}

#[allow(clippy::must_use_candidate)]
impl<T> Graph<T> {
    /// Returns the graph topology.
    #[inline]
    pub fn topology(&self) -> &Topology {
        &self.topology
    }

    /// Returns the number of nodes.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether there are any nodes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<T> Index<usize> for Graph<T> {
    type Output = T;

    /// Returns a reference to the node at the index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use zrx_graph::topology::Adjacency;
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
    /// // Obtain references to nodes
    /// assert_eq!(&graph[a], &"a");
    /// assert_eq!(&graph[b], &"b");
    /// assert_eq!(&graph[c], &"c");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Graph<T> {
    /// Returns a mutable reference to the node at the index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use zrx_graph::topology::Adjacency;
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
    /// let mut graph = builder.build();
    ///
    /// // Obtain mutable references to nodes
    /// assert_eq!(&mut graph[a], &mut "a");
    /// assert_eq!(&mut graph[b], &mut "b");
    /// assert_eq!(&mut graph[c], &mut "c");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// ----------------------------------------------------------------------------

impl<'a, T> IntoIterator for &'a Graph<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    /// Creates an iterator over the graph.
    ///
    /// This iterator yields the data `T` associated with each node. If you need
    /// to iterate over the node indices of a graph, use [`Graph::topology`] to
    /// obtain the [`Topology::incoming`] or [`Topology::outgoing`] adjacency
    /// list, and iterate over those.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use zrx_graph::topology::Adjacency;
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
    /// // Create iterator over data
    /// for data in &graph {
    ///     println!("{data:?}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
