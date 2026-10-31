# Known issues

Problems found while documenting the core (version 0.1.0). They are not
fixed yet. Items in **Light transport** change the look of every render, so
they are worth fixing before building new shading features on top.


## Light transport

### Lambert: missing PDF, renders too dark

`LambertShader::evaluate` (`src/rt_shaders/lambert.rs`) samples directions
around the normal (`normalize(N + random point in the unit ball)`, roughly
cosine-distributed) but weights each sample with

```rust
self.color * Li * RT_ONEOVERPI * dot(dir, N)
```

without dividing by the sampling PDF. With cosine-weighted sampling,
`pdf = cos θ / π`, so the estimator should reduce to `color * Li`. The
current one is `cos θ / π` times too small (about 1/5 on average): diffuse
surfaces lose most of their energy at each bounce.

**Fix:** sample a true cosine distribution (`normalize(N + random_unit_vector())`,
with a point *on* the unit sphere, not in the ball) and return `color * Li`.

### Lambert: terminated paths add pink light

When `RtTraceRay` returns `None` (path terminated by `max_bounces`), the
lambert shader adds `self.color * RtRGBA::ERRCOLOR * ...` instead of nothing.
Every path that reaches the bounce limit on a diffuse surface adds a pink
tint. `Metal` and `Glass` correctly add black in that case.

**Fix:** treat `None` as black.

### Glass: wrong Schlick formula

`reflectance` in `src/rt_shaders/glass.rs` computes

```rust
let r0 = (1.0 - ior) / (1.0 + ior);
r0 + (1.0 - r0*r0) * (1.0 - cosine).powf(5.0)
```

Schlick's approximation is `r0 = ((1 - η) / (1 + η))²` and
`R = r0 + (1 - r0) (1 - cos θ)⁵`. Here `r0` is not squared, so it is negative
when the ray exits the glass (`η = ior > 1`), and the second term uses
`1 - r0²` instead of `1 - r0`. Reflection probability is wrong at all angles.

**Fix:** `let r0 = ((1.0 - ior) / (1.0 + ior)).powi(2); r0 + (1.0 - r0) * (1.0 - cosine).powi(5)`.

### Metal: fuzzed rays can go below the surface

`Metal::evaluate` adds `fuzz * random_unit_vector()` to the reflected
direction without normalizing it or checking that it is still above the
surface. With a high `fuzz`, some rays point into the object and immediately
hit it from inside. (The Ray Tracing in One Weekend reference absorbs those
rays.) The fuzz vector is also sampled *in* the unit ball, not on its
surface.

### Approximate constants

`RT_PI = 3.1416` and `RT_ONEOVERPI = 0.3183` are rounded. Use
`std::f32::consts::PI` and `FRAC_1_PI`.


## Rendering and threading

### Bucket coordinates are swapped

`RtRenderBucket::get_top_buckets` (`src/rt_camera.rs`) calls
`RtRenderBucket::new(top, left, width, height, ...)` with the **x** offset as
`top` and the **y** offset as `left`. The rest of the pipeline uses the two
consistently in the swapped way (`left_coordinate` is added to x when making
camera rays, and is the tile's `x_offset`), so square images whose size is a
multiple of the bucket size render correctly, which is always the case today
(400 × 400, 50 × 50 buckets).

With a non-square image, or a size that is not a multiple of 50, edge tiles
get the wrong width and height and pixels will be missing or out of bounds.

**Fix:** pass `bucket_size[1] * bucket_y` as `top` and
`bucket_size[0] * bucket_x` as `left`. `RtBucketRayIterator::top_left` is
built in the wrong order too (`[top, left]`) but is unused.

### Possible hang in the thread stack

`RtThreadsStack::finish` calls `cv.notify_one()` from a worker **before** the
worker thread has returned, and without holding the mutex. The main thread,
woken in `wait_for_free`, calls `close_finished`, which only removes a thread
whose `handle.is_finished()` is already true, and removes at most one per
wake-up. A notification sent while the main thread is not waiting is lost. If
all `NB_THREADS` workers notify during such windows, the main thread waits
forever with a full stack of finished threads.

This is rare but becomes more likely with small buckets or more threads.

**Fix:** use a persistent pool of `NB_THREADS` workers that loop on
`bucket_queue.pop()` until it returns `None`, then join them
(`std::thread::scope` makes this simple and removes the `Arc`s). This also
removes the cost of spawning one thread per bucket.

### Progressive rendering is not implemented

`RtRenderSettings::progressive`, `RtRenderBucket::samples_nb` and
`RtRenderQueue::release` exist for it but are unused: each bucket is rendered
with all its samples in one go, and the `BinaryHeap` ordering on
`samples_nb` has no effect (all zero). `BUCKET_MODE_SPIRAL` panics.

### Renders are not reproducible

All random numbers come from `rand::thread_rng()`. The seeded `RtSampler` is
not used by the renderer. Seed the sampler per pixel (and per sample) to get
deterministic renders, which also makes regression tests on images possible.


## Scene loading

### Missing `max_bounces` panics

In `XMLScene::get_settings` the second check tests `p_spp.is_err()` again
instead of `p_bounces.is_err()`, so a scene without `max_bounces` panics on
`unwrap` instead of logging an error.

### Error messages are not formatted

The `get_*_parameter` helpers return `String::from("No parameter named {parameter}")`.
`String::from` does not interpolate, so the message literally contains
`{parameter}`. Use `format!`.

### Panics instead of errors

The loader panics on unknown or nested tags, unparsable values
(`extract_param_value`), a shape without a `shader` parameter, and a shape
missing `center` or `radius`. Since the editor runs the binary as a
subprocess, a panic shows up as a crash in the render log. Returning
`Result` through `open_xml_scene` would give clearer errors in the editor.

### Hard-coded camera and lights

- The camera is always created with aspect ratio `1.0` and a width of `400`.
- No light is ever loaded; the lights list stays empty.
- `RtCamera::_vfov`, `_look_from` and `_look_at` are stored but only used by
  the egui UI.


## Miscellaneous

- `--log-level warning` sets `RUST_LOG=warning`, which `env_logger` does not
  recognise as a level (it expects `warn`). It is read as a module filter, so
  this option silences all logs.
- `RtRenderResult`'s doc comment says `render_grid[col][row]`; the grid is
  indexed `[row][col]` (`render_grid[y][x]`).
- `RtMakeRay` ignores its `_raytype` and `_maxdist` arguments. The
  `MAX_RAY_LENGTH` and `NB_SAMPLES` constants in `metal.rs` and `glass.rs` are
  unused.
- `rt_test.rs` uses absolute Linux paths to the test scenes; use paths
  relative to the crate (`env!("CARGO_MANIFEST_DIR")`).
- The library depends on `egui` only for `Color32` conversions.
