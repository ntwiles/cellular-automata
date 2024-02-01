# Cellular Automata

## Description

A demonstration of several popular cellular automata implementations.

## Installation

Rust must be installed to build this project.

## Usage

Pass the name of the algorithm you want to visualize as the first command line argument:

- Conway's Game of Life: `cargo run conway`
- Langton's Ant: `cargo run langton`
- Brian's Brain: `cargo run brian`

Each simulation will begin paused. To begin the simulation (or to pause later) press the space key.

## Configuration

Configuration can overridden per-algorithm by creating a `settings.toml` file. See
`default.toml` for details on the available fields.
