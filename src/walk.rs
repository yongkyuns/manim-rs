//! Move at a defined speed along a path.
//!
//! # Path walking
//!
//! ## Overview
//!
//! In principle, walking a path is similar to iterating over it,
//! but instead of going from receiving path segments (of varying
//! sizes), the path walker makes it possible to advance by a certain
//! distance along the path.
//!
//! ## Example
//!
//! ```
//! use lyon_algorithms::walk::{RegularPattern, walk_along_path};
//! use lyon_algorithms::path::PathSlice;
//! use lyon_algorithms::path::iterator::*;
//! use lyon_algorithms::path::math::Point;
//!
//! fn dots_along_path(path: PathSlice, dots: &mut Vec<Point>) {
//!     let mut pattern = RegularPattern {
//!         callback: &mut |position, _tangent, _distance| {
//!             dots.push(position);
//!             true // Return true to continue walking the path.
//!         },
//!         // Invoke the callback above at a regular interval of 3 units.
//!         interval: 3.0,
//!     };
//!
//!     let tolerance = 0.01; // The path flattening tolerance.
//!     let start_offset = 0.0; // Start walking at the beginning of the path.
//!     walk_along_path(
//!         path.iter().flattened(tolerance),
//!         start_offset,
//!         &mut pattern
//!     );
//! }
//!
//! ```
//!

use nannou::lyon::geom::{Arc, CubicBezierSegment, QuadraticBezierSegment};
use nannou::lyon::math::*;
use nannou::lyon::path::builder::*;
use nannou::lyon::path::PathEvent;

use std::f32;

/// Calculate the total distance of a path by walking through it
#[allow(dead_code)]
pub fn print_distance<Iter>(path: Iter) -> f32
where
    Iter: Iterator<Item = PathEvent>,
{
    let mut pattern = RepeatedPattern {
        callback: &mut |pos, tan, dis| {
            println!("pos = {:<7.3}, tan = {:<7.3}, dis = {:<7.3}", pos, tan, dis);
            true // Return true to continue walking the path.
        },
        intervals: &[10.0],
        index: 0,
    };

    let mut walker = PathWalker::new(0.0, &mut pattern);
    for evt in path {
        walker.path_event(evt);
        if walker.done {
            break;
        }
    }
    0.0
}

/// Walks along the path staring at offset `start` and applies a `Pattern`.
#[allow(dead_code)]
pub fn walk_along_path<Iter>(path: Iter, start: f32, pattern: &mut dyn Pattern)
where
    Iter: Iterator<Item = PathEvent>,
{
    let mut walker = PathWalker::new(start, pattern);
    for evt in path {
        println!("{}", walker.leftover);
        walker.path_event(evt);
        if walker.done {
            return;
        }
    }
}

/// Types implementing the `Pattern` can be used to walk along a path
/// at constant speed.
///
/// At each step, the pattern receives the position, tangent and already
/// traversed distance along the path and returns the distance until the
/// next step.
///
/// See the `RegularPattern` and `RepeatedPattern` implementations.
/// This trait is also implemented for all functions/closures with signature
/// `FnMut(Point, Vector, f32) -> Option<f32>`.
pub trait Pattern {
    /// This method is invoked at each step along the path.
    ///
    /// If this method returns None, path walking stops. Otherwise the returned
    /// value is the distance along the path to the next element in the pattern.
    fn next(&mut self, position: Point, tangent: Vector, distance: f32) -> Option<f32>;

    /// Invoked at the start each sub-path.
    ///
    /// Takes the leftover requested distance from the previous sub-path path,
    /// if any.
    ///
    /// If this method returns None, path walking stops. Otherwise the returned
    /// value is the distance along the path to the next element in the pattern.
    fn begin(&mut self, distance: f32) -> Option<f32> {
        Some(distance)
    }
}

/// A helper struct to walk along a flattened path using a builder API.
pub struct PathWalker<'l> {
    prev: Point,
    advancement: f32,
    leftover: f32,
    next_distance: f32,
    first: Point,
    need_moveto: bool,
    done: bool,

    pattern: &'l mut dyn Pattern,
}

impl<'l> PathWalker<'l> {
    pub fn new(start: f32, pattern: &'l mut dyn Pattern) -> PathWalker<'l> {
        let start = f32::max(start, 0.0);
        PathWalker {
            prev: point(0.0, 0.0),
            first: point(0.0, 0.0),
            advancement: 0.0,
            leftover: 0.0,
            next_distance: start,
            need_moveto: true,
            done: false,
            pattern,
        }
    }
}

impl<'l> FlatPathBuilder for PathWalker<'l> {
    fn move_to(&mut self, to: Point) {
        self.need_moveto = false;
        self.first = to;
        self.prev = to;

        if let Some(distance) = self.pattern.begin(self.next_distance) {
            self.next_distance = distance;
        } else {
            self.done = true;
        }
    }

    fn line_to(&mut self, to: Point) {
        if self.need_moveto {
            self.move_to(self.first);
            if self.done {
                return;
            }
        }

        let v = to - self.prev;
        let d = v.length();

        if d < 1e-5 {
            return;
        }

        let tangent = v / d;

        let mut distance = self.leftover + d;
        while distance >= self.next_distance {
            let position = self.prev + tangent * (self.next_distance - self.leftover);
            self.prev = position;
            self.leftover = 0.0;
            self.advancement += self.next_distance;
            distance -= self.next_distance;

            if let Some(distance) = self.pattern.next(position, tangent, self.advancement) {
                self.next_distance = distance;
            } else {
                self.done = true;
                return;
            }
        }

        self.prev = to;
        self.leftover = distance;
    }

    fn close(&mut self) {
        let first = self.first;
        self.line_to(first);
        self.need_moveto = true;
    }

    fn current_position(&self) -> Point {
        self.prev
    }
}

impl<'l> PathBuilder for PathWalker<'l> {
    fn quadratic_bezier_to(&mut self, ctrl: Point, to: Point) {
        let curve = QuadraticBezierSegment {
            from: self.prev,
            ctrl,
            to,
        };
        curve.for_each_flattened(0.01, &mut |p| {
            self.line_to(p);
        });
    }

    fn cubic_bezier_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point) {
        let curve = CubicBezierSegment {
            from: self.prev,
            ctrl1,
            ctrl2,
            to,
        };
        curve.for_each_flattened(0.01, &mut |p| {
            self.line_to(p);
        });
    }

    fn arc(&mut self, center: Point, radii: Vector, sweep_angle: Angle, x_rotation: Angle) {
        let start_angle = (self.current_position() - center).angle_from_x_axis() - x_rotation;
        Arc {
            center,
            radii,
            start_angle,
            sweep_angle,
            x_rotation,
        }
        .for_each_flattened(0.01, &mut |p| {
            self.line_to(p);
        });
    }
}

impl<'l> PolygonBuilder for PathWalker<'l> {
    /// Add a closed polygon.
    fn polygon(&mut self, points: &[Point]) {
        build_polygon(self, points);
    }
}

/// A simple pattern that invokes a callback at regular intervals.
///
/// If the callback returns false, path walking stops.
pub struct RegularPattern<Cb> {
    /// The function to call at each step.
    pub callback: Cb,
    /// A constant interval between each step.
    pub interval: f32,
}

impl<Cb> Pattern for RegularPattern<Cb>
where
    Cb: FnMut(Point, Vector, f32) -> bool,
{
    #[inline]
    fn next(&mut self, position: Point, tangent: Vector, distance: f32) -> Option<f32> {
        if !(self.callback)(position, tangent, distance) {
            return None;
        }
        Some(self.interval)
    }
}

/// A pattern that invokes a callback at a repeated sequence of
/// constant intervals.
///
/// If the callback returns false, path walking stops.
pub struct RepeatedPattern<'l, Cb> {
    /// The function to call at each step.
    pub callback: Cb,
    /// The repeated interval sequence.
    pub intervals: &'l [f32],
    /// The index of the next interval in the sequence.
    pub index: usize,
}

impl<'l, Cb> Pattern for RepeatedPattern<'l, Cb>
where
    Cb: FnMut(Point, Vector, f32) -> bool,
{
    #[inline]
    fn next(&mut self, position: Point, tangent: Vector, distance: f32) -> Option<f32> {
        if !(self.callback)(position, tangent, distance) {
            return None;
        }
        let idx = self.index % self.intervals.len();
        self.index += 1;
        Some(self.intervals[idx])
    }
}

impl<Cb> Pattern for Cb
where
    Cb: FnMut(Point, Vector, f32) -> Option<f32>,
{
    #[inline]
    fn next(&mut self, position: Point, tangent: Vector, distance: f32) -> Option<f32> {
        (self)(position, tangent, distance)
    }
}

#[test]
fn walk_square() {
    let expected = [
        (point(0.0, 0.0), vector(1.0, 0.0), 0.0),
        (point(2.0, 0.0), vector(1.0, 0.0), 2.0),
        (point(4.0, 0.0), vector(1.0, 0.0), 4.0),
        (point(6.0, 0.0), vector(1.0, 0.0), 6.0),
        (point(6.0, 2.0), vector(0.0, 1.0), 8.0),
        (point(6.0, 4.0), vector(0.0, 1.0), 10.0),
        (point(6.0, 6.0), vector(0.0, 1.0), 12.0),
        (point(4.0, 6.0), vector(-1.0, 0.0), 14.0),
        (point(2.0, 6.0), vector(-1.0, 0.0), 16.0),
        (point(0.0, 6.0), vector(-1.0, 0.0), 18.0),
        (point(0.0, 4.0), vector(0.0, -1.0), 20.0),
        (point(0.0, 2.0), vector(0.0, -1.0), 22.0),
        (point(0.0, 0.0), vector(0.0, -1.0), 24.0),
    ];

    let mut i = 0;
    let mut pattern = RegularPattern {
        interval: 2.0,
        callback: |pos, n, d| {
            println!("p:{:?} n:{:?} d:{:?}", pos, n, d);
            assert_eq!(pos, expected[i].0);
            assert_eq!(n, expected[i].1);
            assert_eq!(d, expected[i].2);
            i += 1;
            true
        },
    };

    let mut walker = PathWalker::new(0.0, &mut pattern);

    walker.move_to(point(0.0, 0.0));
    walker.line_to(point(6.0, 0.0));
    walker.line_to(point(6.0, 6.0));
    walker.line_to(point(0.0, 6.0));
    walker.close();
}

#[test]
fn walk_with_leftover() {
    let expected = [
        (point(1.0, 0.0), vector(1.0, 0.0), 1.0),
        (point(4.0, 0.0), vector(1.0, 0.0), 4.0),
        (point(5.0, 2.0), vector(0.0, 1.0), 7.0),
        (point(5.0, 5.0), vector(0.0, 1.0), 10.0),
        (point(2.0, 5.0), vector(-1.0, 0.0), 13.0),
        (point(0.0, 4.0), vector(0.0, -1.0), 16.0),
        (point(0.0, 1.0), vector(0.0, -1.0), 19.0),
    ];

    let mut i = 0;
    let mut pattern = RegularPattern {
        interval: 3.0,
        callback: |pos, n, d| {
            println!("p:{:?} n:{:?} d:{:?}", pos, n, d);
            assert_eq!(pos, expected[i].0);
            assert_eq!(n, expected[i].1);
            assert_eq!(d, expected[i].2);
            i += 1;
            true
        },
    };

    let mut walker = PathWalker::new(1.0, &mut pattern);

    walker.move_to(point(0.0, 0.0));
    walker.line_to(point(5.0, 0.0));
    walker.line_to(point(5.0, 5.0));
    walker.line_to(point(0.0, 5.0));
    walker.close();
}

#[test]
fn walk_starting_after() {
    // With a starting distance that is greater than the path, the
    // callback should never be called.
    let cb = &mut |_, _, _| -> Option<f32> { panic!() };
    let mut walker = PathWalker::new(10.0, cb);

    walker.move_to(point(0.0, 0.0));
    walker.line_to(point(5.0, 0.0));
}
