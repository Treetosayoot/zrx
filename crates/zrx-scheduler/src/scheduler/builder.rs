// // Copyright (c) 2025 Zensical LLC

// // Permission is hereby granted, free of charge, to any person obtaining a copy
// // of this software and associated documentation files (the "Software"), to
// // deal in the Software without restriction, including without limitation the
// // rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// // sell copies of the Software, and to permit persons to whom the Software is
// // furnished to do so, subject to the following conditions:

// // The above copyright notice and this permission notice shall be included in
// // all copies or substantial portions of the Software.

// // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// // IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// // FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// // AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// // LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// // FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// // IN THE SOFTWARE.

// // ----------------------------------------------------------------------------

// //! Scheduler builder.

// use zrx_executor::{Executor, Strategy};

// // use crate::scheduler::executor::Executor;
// use crate::scheduler::Id;

// use super::graph::GraphOld;
// use super::session::Connector;

// // ----------------------------------------------------------------------------
// // Structs
// // ----------------------------------------------------------------------------

// /// Scheduler builder.
// pub struct Builder<I, S>
// where
//     I: Id,
//     S: Strategy,
// {
//     /// Execution graph.
//     graph: GraphOld<I>,
//     /// Task executor.
//     executor: Option<Executor<S>>,
//     /// Session connector.
//     connector: Option<Connector<I>>,
// }

// // ----------------------------------------------------------------------------
// // Implementations
// // ----------------------------------------------------------------------------

// impl<I, S> Builder<I, S>
// where
//     I: Id,
//     S: Strategy,
// {
//     /// Creates a scheduler builder.
//     pub fn new(graph: GraphOld<I>) -> Self {
//         Self {
//             graph,
//             executor: None,
//             connector: None,
//         }
//     }

//     // /// Builds the scheduler.
//     // pub fn build(self) -> Scheduler<I, S> {
//     //     Scheduler {
//     //         executor: Executor::new(self.graph),
//     //         tasks: Tasks::new(self.executor.unwrap_or_default()),
//     //         timers: Timers::new(),
//     //         connector: Connector::new(),
//     //     }
//     // }
// }
