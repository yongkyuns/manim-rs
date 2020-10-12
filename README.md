# manim-rs
manim-rs is an experimental project which aims to provide animation capability similar to manim, but implemented in Rust. This project will try to maintain similar APIs to manim, and use Nannou + lyon as the main dependencies for drawing and windowing. The goal of the project is to be able to do many things that manim can do, and do them in an interactive context with web environment in mind.

## Todo
- [x] Animation module breakdown
- [x] Position relative position w.r.t. window (e.g. to_edge())
- [ ] Multiple object animation
- [ ] Benchmark with multiple objects
- [ ] APIs to chain actions
- [ ] More animation types (fade, color, etc.)
- [ ] More shapes, design object -> shape architecture
- [ ] ShowCreation, timed object generation
- [ ] Lagged start