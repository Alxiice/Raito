# Engine architecture

This page describes how the Rust core of Raito (`src/`) is organised and how a
frame is produced, from the XML scene file to the final pixels. It reflects the
code as of version 0.1.0.

!!! note

    For the API reference generated from the doc comments, see
    [Code documentation](../../rust/doc/raito/index.html) (built with `cargo doc`).


## Overview

Raito is a CPU path tracer, close in spirit to *Ray Tracing in One Weekend*,
with an API shaped after Arnold (`RtShaderGlobals`, `RtTraceRay`, sampler
iterators, ...). The crate builds two targets:

- **`raito` library** (`src/lib.rs`): the render engine itself.
- **`raito` binary** (`src/main.rs`): a CLI that either renders a scene
  headless (`--scene` / `--output`) or opens the legacy egui render window.

The Python/QML scene editor (`ui/`) does not link against the library. It
exports its scene to XML, runs the binary in headless mode and reads back the
PPM image:

```text
 ui/ (Python, QML)                         raito binary (Rust)
 ─────────────────                         ───────────────────
 node graph ─► cooked Scene                main.rs::render_headless
      │                                         │
      └─ ui/render/raito.py::scene_to_xml       ├─ rt_scene::open_xml_scene   ─► RtScene
             │                                  ├─ rt_render::RtRenderScene    ─► RtRenderResult
             └─ scene.xml ── raito --scene ─────┘  └─ export_as_ppm            ─► render.ppm
                                                                                     │
 Viewer panel ◄──────────────────────────── QImage ◄──────────────────────────────┘
```

So any new engine feature that the editor should expose needs work in three
places: the Rust engine, the XML loader, and the Python exporter. See
[Extending the engine](extending.md).


## Module map

| Module | Role |
| --- | --- |
| `rt_types` | Math and color types: `RtVec3`, `RtPoint3`, `RtRGBA`, constants `RT_EPSILON`, `RT_PI`, `RT_ONEOVERPI` |
| `rt_sampler` | Random numbers: `random_float`, `random_float_range`, `RtVec3::random*`, and a seeded `RtSampler` / `RtSampleIterator` (not used by the renderer yet) |
| `rt_ray` | `RtRay`, `RtRayType`, and the two hit records `RtRayHit` (geometric) and `RtHit` (shaded) |
| `rt_camera` | `RtCamera` (pinhole), plus buckets: `RtRenderBucket`, `RtBucketMode`, `RtBucketRayIterator`, `RtPixel` |
| `rt_shader_globals` | `RtShaderGlobals`, the per-shading-point state handed to shaders |
| `rt_objects` | The `RtObject` trait, `ObjectParams`, `RtObjectList`; geometries (`RtSphere`) and lights (`RtSkydomeLight`, `RtPointLight`) |
| `rt_shaders` | The `RtShader` trait and the surface shaders: `StaticColorShader`, `StateVectorShader`, `LambertShader`, `Metal`, `Glass`, `LightShader` |
| `rt_scene` | `RtScene`, `RtRenderSettings`, and the XML loader `open_xml_scene` |
| `rt_render` | Ray helpers (`RtMakeRay`, `RtReflectRay`, `RtRefractRay`), tracing (`RtTraceRay`, `RtTraceToLights`) and the multithreaded bucket renderer `RtRenderScene` |
| `rt_render_output` | `RtRenderResult` (a pixel grid, also used for bucket tiles) and PPM export |

Binary-only modules: `render_window` and `render_window_params` (the legacy
egui UI, deprecated) and `rt_test` (scratch tests run with `raito --tests`).

`lib.rs` re-exports most modules with `pub use`, so `use raito::*;` brings the
whole API into scope. `rt_objects` and `rt_shaders` are not glob re-exported:
use their full paths (`raito::rt_objects::rt_geometries::RtSphere`).


## Core types

### Math (`rt_types`)

- `RtVec3` and `RtPoint3` are separate types to keep points and directions
  apart: `RtPoint3 - RtPoint3 = RtVec3`, `RtPoint3 + RtVec3 = RtPoint3`.
  `RtVec3 * RtVec3` is the **dot product** (returns `f32`); use
  `RtVec3::cross` for the cross product.
- `RtRGBA` holds linear `f32` channels. `+`, `*` (per channel and by scalar)
  and `/` are defined. Alpha is carried along but has no meaning in rendering
  yet: `+` blends it with an "over"-like formula and scalar `*` scales it.
- `RtRGBA::ERRCOLOR` (pink `0.9, 0.5, 0.6`) marks missing data: no shader, a
  missed ray, ... If it shows up in a render, something is wrong.

### Rays and hits (`rt_ray`)

- `RtRay { origin, dir, bounces, x, y }`: `bounces` is the depth (0 for camera
  rays) and `x`, `y` the raster pixel the ray tree belongs to.
- `RtRayHit { hit, dist, P }` is what an object returns from
  `get_intersection`: distance along the ray and hit point, no shading.
- `RtHit { hit, colorOutput, P }` is what `RtTraceRay` returns: the radiance
  carried back along the ray after the hit object's shader ran.

### Shader globals (`rt_shader_globals`)

`RtShaderGlobals` is built for each shading point by `RtObject::get_sg`:

| Field | Meaning |
| --- | --- |
| `x`, `y` | Raster pixel of the ray tree |
| `P` | Hit point, world space |
| `Po` | Name of the object hit |
| `ray_origin`, `ray_dir` | Incoming ray (`ray_dir` is normalized) |
| `bounces` | Depth of the incoming ray |
| `N` | Shading normal, **outward-facing** (not flipped toward the ray) |

UVs, geometric normal (`Ng`), front-facing normal (`Nf`) and ray type are
listed as TODOs in the struct.


## Scene (`rt_scene`)

```rust
pub struct RtScene {
    pub settings: RtRenderSettings,   // render_spp, inv_nb_spp, max_bounces, progressive
    camera: RtCamera,
    shapes: RtObjectList,             // Vec<Box<dyn RtObject>>
    lights: RtObjectList,
}
```

`RtScene` is `Clone` (objects and shaders are boxed trait objects with
`clone_box` / `clone_dyn`) and is shared read-only between render threads
through an `Arc`. The scene is immutable during a render; there is no
acceleration structure yet, so every ray tests every shape.

### Objects

Every renderable implements `RtObject` (`rt_objects/rt_object_base.rs`):

```rust
pub trait RtObject: Send + Sync {
    fn getObjectParams(&self) -> &ObjectParams;             // name, type, shader
    fn clone_box(&self) -> Box<dyn RtObject>;
    fn get_intersection(&self, ray: &RtRay) -> Option<RtRayHit>;
    fn get_normal(&self, point: &RtPoint3) -> RtVec3;
    // Provided: get_name, get_type, get_shader, get_sg, intersect
}
```

Only two implementations exist: `RtSphere` (geometry) and `RtPointLight`
(a sphere-shaped light). `RtSkydomeLight` is declared but does not implement
`RtObject`. The enums `RtObjectTypes`, `RtGeometryTypes`, `RtLightTypes`,
`RtSurfaceShadersTypes` and `RtLightShadersTypes` are placeholders for a
future registry and are not used anywhere.

### Shaders

```rust
pub trait RtShader: Send + Sync {
    fn clone_dyn(&self) -> Box<dyn RtShader>;
    fn evaluate(&self, scene: &RtScene, sg: &RtShaderGlobals) -> RtRGBA;
}
```

A shader returns the radiance leaving the shading point toward the incoming
ray. Shaders that need indirect light trace their own secondary rays with
`RtTraceRay`, which recursively runs the next hit's shader. Light transport is
therefore **recursive and fully inside the shaders**: there is no separate
integrator.

| Shader | XML `type` | Parameters | Behaviour |
| --- | --- | --- | --- |
| `LambertShader` | `lambert` | `color` | One diffuse bounce per evaluation (`NB_SAMPLES = 1`) |
| `Metal` | `metal` | `color`, `fuzz` | Mirror reflection, direction jittered by `fuzz` |
| `Glass` | `glass` | `ior` | Reflection or refraction picked stochastically with Schlick's approximation |
| `StaticColorShader` | — | `color` | Constant color. `DEFAULT_SHADER` is one, set to `ERRCOLOR` |
| `StateVectorShader` | — | `output` (ignored) | Displays the normal `N` remapped to `[0, 1]` |
| `LightShader` | — | `color`, `intensity` | Constant emission, `color * intensity` |

### Camera (`rt_camera`)

`RtCamera::new(aspect_ratio, image_width, vfov, lookfrom, lookat, vup)` builds a
pinhole camera (right-handed, `vup` is `+Y` when loaded from XML). It
precomputes the position of pixel (0, 0) and per-pixel deltas;
`get_camera_ray(x, y, px, py)` returns a normalized ray through the point
`(x + px, y + py)` of the image plane, `px, py ∈ [0, 1)` being the sub-pixel
jitter.

The XML loader always creates the camera with `aspect_ratio = 1.0` and
`image_width = 400`, so every render is 400 × 400 for now.


## Rendering a frame

### `RtRenderScene(scene, &mut result)`

`rt_render.rs`. Constants at the top of the section control it:
`NB_THREADS = 8`, `BUCKET_MODE = BUCKET_MODE_TOP`, `BUCKET_SIZE = [50, 50]`.

1. **Buckets.** `RtRenderQueue::new` splits the image into 50 × 50 tiles
   (`RtRenderBucket::get_bucket_list`, edge tiles are smaller) and puts them in
   a `Mutex<BinaryHeap>` ordered by `samples_nb`.
   `BUCKET_MODE_SPIRAL` panics (not implemented).
2. **Threads.** While buckets remain, the main thread waits for a free slot
   (`RtThreadsStack::wait_for_free`, a `Mutex` + `Condvar` capped at
   `NB_THREADS`) and **spawns a new OS thread** that pops one bucket. There is
   no persistent thread pool: one thread is spawned per bucket.
3. **Bucket render.** `RtRenderBucket` (the function) iterates over the
   bucket's pixels with `RtBucketRayIterator`. For each pixel it fires
   `render_spp` camera rays with random jitter, traces them with `RtTraceRay`
   and averages the result (`inv_nb_spp`). Pixels are written into the
   bucket's own `RtRenderResult` tile.
4. **Compositing.** When a bucket is done, its thread locks the shared final
   image (`Arc<Mutex<RtRenderResult>>`) and adds the tile at its offset.
5. **Output.** After all threads are joined, each pixel is converted from
   linear to display with `linear_to_gamma` (gamma 2, i.e. `sqrt`) and written
   to `result`.

### `RtTraceRay(scene, ray)`

1. If `ray.bounces >= max_bounces`, return `None` (the path is terminated).
2. Test the ray against every shape and keep the closest hit.
3. On a hit, build the shader globals with `get_sg` and run the object's
   shader, returning `Some(RtHit { hit: true, colorOutput, P })`.
4. On a miss, return the **sky gradient** (white to `0.5, 0.7, 1.0` along
   `dir.y`) as `Some(RtHit { hit: false, ... })`.

The sky gradient is the only light source in practice: the lights list is
never filled by the XML loader, and the loop over lights in `RtTraceRay` is
commented out. `RtTraceToLights` (closest-light test, returns `None` when a
shape is in the way) exists but nothing calls it.

### Self-intersection

Sphere intersection accepts any `t >= 0`, so shaders move the secondary ray
origin off the surface themselves: `+ RT_EPSILON * N` (lambert, metal) or
`+ RT_EPSILON * dir` (glass), with `RT_EPSILON = 0.0001`. New shaders must
do the same, or better, the intersection routines should take a `t_min`.

### Randomness

All shaders and the camera jitter use `rand::thread_rng()` through
`random_float()`. Renders are therefore not reproducible from one run to the
next. `RtSampler` / `RtSampleIterator` (seeded ChaCha8, modelled on Arnold's
`AiSamplerIterator`) is implemented and tested in `rt_test.rs` but not wired
into the renderer.


## XML scene format

`open_xml_scene(path) -> Option<RtScene>` parses files like
`tests/scenes/scene_000.xml`:

```xml
<scene version="0.0.0">
    <render_settings>
        <parameter type="int" name="spp" value="16"/>
        <parameter type="int" name="max_bounces" value="8"/>
    </render_settings>
    <camera>
        <parameter type="float" name="v_fov" value="20"/>
        <parameter type="point" name="look_from" x="13.0" y="2.0" z="3.0"/>
        <parameter type="point" name="look_at" x="0.0" y="0.0" z="0.0"/>
    </camera>
    <shader type="lambert" name="mat1">
        <parameter type="rgb" name="color" r="0.5" g="0.5" b="0.5"/>
    </shader>
    <shape type="sphere" name="ground">
        <parameter type="node" name="shader" id="mat1"/>
        <parameter type="float" name="radius" value="1000.0"/>
        <parameter type="point" name="center" x="0.0" y="-1000.0" z="0.0"/>
    </shape>
</scene>
```

- Top-level tags must be one of `render_settings`, `camera`, `shader`,
  `shape` (`XML_ELEMENTS_LIST`). Tags cannot be nested.
- `name` on a tag is its identifier, `type` its sub-type.
- `<parameter>` carries `type` and `name`, plus the value attributes for its
  type: `value` (`int`, `float`, `string`), `r g b` (`rgb`), `x y z`
  (`point`, `vec3`), `id` (`node`, a reference to another tag's `name`).
- Parsing is done in two steps: `XMLScene::parse` builds a generic list of
  `XMLSceneElement`s, then `as_rt_scene` turns them into engine objects with
  hard-coded branches per shader and shape type.
- A shape whose shader id is unknown gets `DEFAULT_SHADER` (pink) and a
  warning. Most other errors panic.


## Command line

```text
raito [--log-level debug|info|warning|error] [--tests]
      [--scene <scene.xml> --output <image.ppm>]
```

With `--scene` and `--output`, `render_headless` loads the scene, renders it
at the camera resolution and writes a binary PPM (P6). Without them, the
legacy egui window opens. `--tests` runs `rt_test::rt_test()`.

The egui app (`render_window.rs`) builds its own default scenes directly in
Rust (`get_default_scene_0`, `get_default_scene_1`). It is deprecated, but it
still uses the library API, so library changes must keep it compiling until
it is removed.
