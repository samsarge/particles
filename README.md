# particles
measure time taken for dynamic memory allocation

`cargo run`


You'll see that heap allocation time is pretty undepredictable, slow and has nothing to do with the amount of memory requested.

Should try to minimize heap allocation as much as possible:
- Use arrays of uninitiialized objects
- Use an allocator that is tuned for your application's access memory profile
- Investigate arena::Arena and arena::TypesArena. These allow objected to be created on the fly,
  but alloc() and free() are only called when the arena is created and destroyed.
