# The Lead (Rust) Renderer

This is an experimental render taking certain inspirations from the Nori renderer. I plan to follow the [PBR Book (4ed)](https://pbr-book.org/) in my pursuit to create a Pathtracer.

I initially started this project in C++, but I quickly realized that rather than learning what was going on behind the scenes, I started copying code verbatim, or with minimal changes. What did I do next? Why I switched over to Rust! With its borrow-checker and type-system I hope to better learn how to manage different coding patterns accross different languages.

Current To-Do list -

- [ ] Reach the point I left the original Lead Renderer at

~~- [ ] Move away from Nori towards a more PBR-like implementation~~ (Since it is in Rust, the implementation is fundamentally different)

- [ ] Add a spectrum class

- [ ] Convert renderer to spectrum based renderer rather than RGB-based

- [ ] Add parallelism

- [ ] And much much more....

Note - **This list of to-dos is NOT exhaustive it is just what I could think up of right now**
