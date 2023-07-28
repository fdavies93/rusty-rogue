# Rusty Rogue

This is a simple ASCII game made in Rust and modelled on roguelikes such as NetHack and Brogue. It's a project for the Rust study group on the [MTT Discord server]().

## Controls

| Keys | Action | 
|------|--------|
| Arrow Keys / WASD | Movement |
| ESC | Quit |

## Definitions / Architecture

A **component** is a struct used to store data. While components are keyed to **objects** the latter does not exist; components are the primary concept for data. **No object can have more than one component of the same type attached to it.**

A **listener** waits for a given type of event and delivers relevant data about it to a callback function.

An **event** is triggered.

## Possible Changes

We can change the above architecture to use **systems** by having each system have a list of components that care about it. Systems would be similar to callback functions except they operate over many components at once (and therefore don't need specific callback function bindings.)