# Daen.rs

## Disclaimer
This is a Pre-Alpha. There won´t be any improvements to be made. The current goal is to port this project to another implementation with the RayLib API - either in Zig or C#

## Purpose
Ever wanted to do fancy animations in a good programming language? Do you always need to resort to Javascript? Or even worse - a game engine like Unity?
Fear no more, fellow low-level-friend and connoisseur of expressive APIs! The future is now. Although this renderer is not written in C (or Assembly for that manner),
it is still _blazingly_ fast. With the use of the pixels-crate, a hardware-accelerated highly optimizied buffer will convey all your artistic intentions to your
display. Images will follow.

## Final goal

Do the bad Apple
Edit: Scrapped

## Current features
-Creation of Scenes with Json-Files
-Support for custom frontends via CLI
-Basic Shapes: ellipses, circles (*), lines, triangles, squares
-Collision between circles<->Lines and circles<->Ellipses
-dynamic changing of colour and velocity

*which are just a special case of an ellipse

## Bugs and missing features
-Advanced and reliable collision 
-good optimisation of filling algorithms
-reliable drawing algorithms

## Recommendations

Don't look at the source code. It will cause severe depression. Please