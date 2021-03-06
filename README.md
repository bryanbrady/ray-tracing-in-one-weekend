[_Ray Tracing_](https://raytracing.github.io) in Rust!
====================================================================================================

Why not learn about Ray Tracing and Rust at the same time?

This repostory contains a Rust implementation of 
[_Ray Tracing in One Weekend: The Book Series_](https://raytracing.github.io).
- [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
- [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
- [_Ray Tracing: The Rest of Your Life_](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)

## Usage
```bash
# Build
cargo build --release

# Run
cargo run --release > image.ppm

# Profile
RUSTFLAGS=-g cargo run --features profile > image.ppm
```

## Images

[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html), Final Render
------------------------------------------------------------------------------------------------------------
![In One Weekend, Final Render][final]

[_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html), Final Render
-----------------------------------------------------------------------------------------------------------
![The Next Week, Final Render][final_next]

[_Ray Tracing: The Rest of Your Life_](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html), Final Render
-----------------------------------------------------------------------------------------------------------
![The Rest of Your Life, Final Render][final_last]

[final]:  images/final.jpg
[final_next]:  images/final_next.jpg
[final_last]:  images/final_last.jpg

