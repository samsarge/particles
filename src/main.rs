// Create and destroy objects on the heap

// provices maths operations and converstion for 2d vectors
use graphics::math::{Vec2d, add, mul_scalar};
// provides tools to create a gui program and draws shapes to it
use piston_window::*;
// random number generator
use rand::prelude::*;
// control memory allocation (alloc is the UNIX system call for requesting mem from the allocator)
use std::alloc::{GlobalAlloc, System, Layout};
// for access to the system clock
use std::time::Instant;

// marks following allocator as satisfying the GlobalAlloc trait
#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

struct ReportingAllocator;

// prints time taken for each allocation to STDOUT as the program runs
// This gives us an accurate indication of the time taken for dynamic memory allocation

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout); // default the actual allocation to the system's default memory allocator
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();

        eprintln!("Bytes requested: {}\t Time: {}", bytes_requested, time_taken.as_nanos());
        ptr // return raw pointer
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

// data useful for lifetime of program
struct World {
    current_turn: u64,
    particles: Vec<Box<Particle>>, // vector of heaped particles
    height: f64,
    width: f64,
    rng: ThreadRng,
}

// object in 2d space
struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Particle {
        let mut rng = thread_rng();
        // random x axis spawn point
        // y axis always spawn in the same place
        let x = rng.gen_range((0.0)..=world.width);
        let y = world.height;

        let x_velocity = 0.0;

        // rise vertically over time
        let y_velocity = rng.gen_range(-2.0..0.0);

        let x_acceleration = 0.0;

        // increase speed of rise over time
        let y_acceleration = rng.gen_range(0.0..0.15);

        Particle {
            height: 4.0,
            width: 4.0,
            position: [x, y].into(), // into() converts arrays of type [f64; 2] into Vec2d
            velocity: [x_velocity, y_velocity].into(),
            acceleration: [x_acceleration, y_acceleration].into(),
            color: [1.0, 1.0, 1.0, 0.99], // fully white with a tiny amount of transparency
        }
    }

    fn update(&mut self) {
        // move particle to next position
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);
        // slows down the particles rate of increase as it travels across the screen
        self.acceleration = mul_scalar(self.acceleration, 0.7);
        self.color[3] *= 0.995; // slowly make more transparent
    }
}

impl World {
    fn new(width: f64, height: f64) -> World {
        World {
            current_turn: 0,
            // use Box<Particle> instead of Particle to incur an extra memory allocation when every particle is created
            particles: Vec::<Box<Particle>>::new(),
            height: height,
            width: width,
            rng: thread_rng(),
        }
    }

    fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            // create a particle as a local var on the stack
            let particle = Particle::new(&self);
            // take ownership of particle, move its data to the heap and create a reference to that data on the stack
            let boxed_particle = Box::new(particle);
            // push the reference into self.particles
            self.particles.push(boxed_particle);
        }
    }

    fn remove_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut to_delete = None;
            // iter.enumerate() gives us the index too in this tuple

            // for n iterations, remove the first particle that's invisibile.
            // If there are no invisible particles, then remove the oldest
            for (i, particle) in self.particles.iter().enumerate() {
                if particle.color[3] < 0.02 { // 0.02 is basically invisible
                    to_delete = Some(i);
                }
                break;
            }

            if let Some(i) = to_delete {
                self.particles.remove(i);
            } else {
                self.particles.remove(0);
            };
        }
    }

    fn update(&mut self) {
        let n = self.rng.gen_range(-3..=3); // random int between -3 and 3, inclusive

        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }

        self.particles.shrink_to_fit();

        for shape in &mut self.particles {
            shape.update();
        }

        self.current_turn += 1;
    }
}

fn main() {
    let (width, height) = (1280.0, 960.0);

    let mut window: PistonWindow = WindowSettings::new(
        "particles", [width, height]
    )
    .exit_on_esc(true)
    .build()
    .expect("Could not create a window.");
    
    let mut world = World::new(width, height);
    world.add_shapes(1000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.width, s.height];
                rectangle(s.color, size, ctx.transform, renderer);
            }
        });
    }
}
