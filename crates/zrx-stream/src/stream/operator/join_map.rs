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

//! Join map operator.

use std::marker::PhantomData;
use zrx_scheduler::action::descriptor::Property;
use zrx_scheduler::action::output::IntoOutputs;
use zrx_scheduler::action::Descriptor;
use zrx_scheduler::effect::{Item, Task};
use zrx_scheduler::{Id, Value};

use crate::stream::combinator::tuple::cons::IntoStreamTupleCons;
use crate::stream::combinator::tuple::join::IntoJoinMap;
use crate::stream::combinator::tuple::StreamTupleJoin;
use crate::stream::function::{MapFn, Splat};
use crate::stream::value::tuple::{All, Any, First, Presence};
use crate::stream::value::Tuple;
use crate::stream::Stream;

use super::Operator;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Join map operator.
struct JoinMap<F, U, P> {
    /// Operator function.
    function: F,
    /// Type marker.
    marker: PhantomData<(U, P)>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<I, T> Stream<I, T>
where
    I: Id,
    T: Value,
{
    pub fn join_map<S, O, F, U>(&self, streams: S, f: F) -> Stream<I, U>
    where
        S: IntoStreamTupleCons<I, T, Output = O>,
        O: IntoJoinMap<I, All>,
        F: MapFn<I, Splat<O::Item>, U> + Clone,
        U: Value,
    {
        streams
            .into_stream_tuple_cons(self.clone())
            .into_join_map(f)
    }

    pub fn left_join_map<S, O, F, U>(&self, streams: S, f: F) -> Stream<I, U>
    where
        S: IntoStreamTupleCons<I, T, Output = O>,
        O: IntoJoinMap<I, First>,
        F: MapFn<I, Splat<O::Item>, U> + Clone,
        U: Value,
    {
        streams
            .into_stream_tuple_cons(self.clone())
            .into_join_map(f)
    }

    pub fn full_join_map<S, O, F, U>(&self, streams: S, f: F) -> Stream<I, U>
    where
        S: IntoStreamTupleCons<I, T, Output = O>,
        O: IntoJoinMap<I, Any>,
        F: MapFn<I, Splat<O::Item>, U> + Clone,
        U: Value,
    {
        streams
            .into_stream_tuple_cons(self.clone())
            .into_join_map(f)
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<I, T, F, U, P> Operator<I, T> for JoinMap<F, U, P>
where
    I: Id,
    T: Tuple<P>,
    F: MapFn<I, Splat<T>, U> + Clone,
    U: Value,
{
    type Item<'a> = Item<&'a I, T::Arguments<'a>>;

    /// Handles the given item.
    ///
    /// This operator returns a task that produces an output item by applying
    /// the operator function to the input item. The input item is moved into
    /// the task, and the output item is sent back to the main thread when
    /// the worker thread finishes.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug" skip_all, fields(id = %item.id))
    )]
    fn handle(&mut self, item: Self::Item<'_>) -> impl IntoOutputs<I> {
        let item = item.into_owned().map(Splat::from);
        Task::new({
            let function = self.function.clone();
            move || {
                function.execute(&item.id, item.data).map(|report| {
                    report.map(|data| Item::new(item.id, Some(data)))
                })
            }
        })
    }

    /// Returns the descriptor.
    #[inline]
    fn descriptor(&self) -> Descriptor {
        Descriptor::builder()
            .property(Property::Pure)
            .property(Property::Stable)
            .property(Property::Flush)
            .build()
    }
}

// ----------------------------------------------------------------------------
// Blanket implementations
// ----------------------------------------------------------------------------

impl<S, I, P> IntoJoinMap<I, P> for S
where
    S: StreamTupleJoin<I, P>,
    I: Id,
    P: Presence,
{
    fn into_join_map<F, U>(self, f: F) -> Stream<I, U>
    where
        F: MapFn<I, Splat<Self::Item>, U> + Clone,
        U: Value,
    {
        self.workflow().add_operator(
            self.ids(),
            JoinMap {
                function: f,
                marker: PhantomData,
            },
        )
    }
}
