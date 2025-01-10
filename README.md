# Fractality

Simple and fast fractal visualizer.

## Overview

The fractal is calculated by iterating the equation

$z_{n} = z_{n-1}^p + c$

where $z_{n}$, $c$, and $c_{p}$ are complex number parameters.

If $z_{n}$ stays bounded, the point is contained in the fractal,
otherwise, if it blows up to infinity,
it is colored based on how many iterations it took it to escape.

The real and imaginary components of $z_{0}$, $c$, and $p$ can be set to
constant values or parametrized over the $X$ or $Y$ coordinates.

### Parameters

| Prameter        | Description                                                                                                                                                                                |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Iteration Count | How many iterations to calculate. The higher the number, the more detailed the image, however, high iteration count might hurt performance or cause the application to crash.              |
| Escape Radius   | If the iterated point crosses this radius, it is considered to be escaped.                                                                                                                 |
| Initial Z       | $z_0$ at the start of the calculation.                                                                                                                                                     |
| C               | $c$ at the start of the calculation.                                                                                                                                                       |
| Exponent        | $p$ at the start of the calculation. <br> Note that exponents other than 2, non-integer exponents, or exponents having a non-zero imaginary component will be more expensive to calculate. |

## Interesting fractals

### Mandelbrot set

| Parameter | real         | imaginary    |
| --------- | ------------ | ------------ |
| Initial Z | 0.0          | 0.0          |
| C         | X coordinate | Y coordinate |
| Exponent  | 2.0          | 0.0          |

![Mandlebrot set image](materials/mandelbrot_set.png)

### Julia set

| Parameter | real            | imaginary       |
| --------- | --------------- | --------------- |
| Initial Z | X coordinate    | Y coordinate    |
| C         | \<any constant> | \<any constant> |
| Exponent  | 2.0             | 0.0             |

![julia set image](materials/julia_set.png)

## Planned future features

- [ ] Double precision
- [ ] Coloring options
- [ ] More equations
- [ ] Display the path of a single point
- [ ] Using cursor position as a parameter
- [ ] WASM support
- [ ] Rendering and performance improvements
- [ ] Displaying multiple fractals at the same time

## Installation

Compiled executables will be available in the
[Releases](https://github.com/SophieSilver/fractality/releases/) section on GitHub.

Just download the one for your platform and run it.

## Platform Support

Fractality aims to fully support x86_64 Windows and Linux.

Other platforms like Mac OS, ARM Windows and Linux might receive experimental support in the future.

## Build Instructions

Follow these instructions if you want to compile the project yourself.

- Install the Rust toolchain if you haven't already. Instructions on how to do that are available
  [in the Rust book](https://doc.rust-lang.org/book/ch01-01-installation.html).
- Make sure your `rustc` version is `1.84.0` or greater:
  ```sh
  $ rustc --version
  rustc 1.84.0 (9fc6b4312 2025-01-07)
  ```
- Install [OS dependencies for Bevy](https://bevyengine.org/learn/quick-start/getting-started/setup/#installing-os-dependencies)

  On Linux you must also install `libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libgtk-3-dev libatk1.0-dev`

- Clone the repository and run:
  ```sh
  cargo build --release
  ```
  Alternatively, if you want to marginally improve performance
  and reduce the binary size at the cost of a longer compile time, you can run:
  ```
  cargo build --profile=hyperoptimize
  ```
  The executable file will be placed in target/{PROFILE}/
