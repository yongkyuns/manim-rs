# manim-rs
manim-rs is an experimental project which aims to provide animation capability similar to manim, but implemented in Rust. This project will try to maintain similar APIs to manim, and use Nannou + lyon as the main dependencies for drawing and windowing. The goal of the project is to be able to do many things that manim can do, and do them in an interactive context with web environment in mind.

## Todo
- [ ] Add text object
- [ ] Add animation (Color, FadeIn/Out, Write)
- [ ] Draw context cleanup
- [ ] Coordinated animation (e.g. lagged start)
- [ ] Add more object types (Currently Circle and Rectangle only)
- [ ] APIs to chain actions