# The Lead (Rust) Renderer

This is an experimental render taking certain inspirations from the Nori renderer. I plan to follow the [PBR Book (4ed)](https://pbr-book.org/) in my pursuit to create a Pathtracer.

I initially started this project in C++, but I quickly realized that rather than learning what was going on behind the scenes, I started copying code verbatim, or with minimal changes. What did I do next? Why I switched over to Rust! With its borrow-checker and type-system I hope to better learn how to manage different coding patterns accross different languages.

Current To-Do list -

- [x] ~~Move away from Nori towards a more PBR-like implementation~~ Since it is in Rust, the implementation is fundamentally different

- [x] Add Geometry stuff (coordinate systems, vectors, points, rays, b boxes, transformations and interactions)

- [ ] Add animated transformations

- [x] Add basic Shape interface

- [ ] Add ~~Sphere~~, Cylinders, Disks, Quadrics, **Triangle Meshes**, Curves, and Subdivision Surfaces

- [x] Add Primitives interface

- [x] Add aggregates Bounding Volume Heirarchy

- [x] Add Spectra and Color stuff

- [ ] Add Camera models

  - [x] Orthographic camera

  - [x] Perspective camera

  - [x] Environment (panoramic) camera

- [ ] Add samplers and film

- [ ] Add BxDFs

- [ ] Add textures (procedural and images)

- [ ] **Add Volumes!!!!!!**

- [ ] Add Light sources

- [ ] Create Direct integrator

- [ ] Create recursive Path integrator

- [ ] Create Volume Renderer

- [ ] Create Bidirectional Renderer

- [ ] Create Photon Mapping integrator (maybe)

- [ ] Add parallelism

- [ ] And much much more....

Note - **This list of to-dos is NOT exhaustive it is just what I could think up of right now**
