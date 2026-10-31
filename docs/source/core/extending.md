# Extending the engine

Step-by-step recipes for the most common additions. Each one goes through the
same chain as a render: Rust engine → XML loader → Python exporter → editor
node. Read [Engine architecture](architecture.md) first.

!!! tip

    Before adding features that change the look of a render, fix the shading
    issues listed in [Known issues](known-issues.md#light-transport). Otherwise
    new features are compared against a reference that is already biased.


## Add a surface shader

Example: an `emission` shader with `color` and `intensity`.

1. **Shader.** Create `src/rt_shaders/emission.rs`, following `metal.rs`:

    ```rust
    #[derive(Clone, Debug)]
    pub struct Emission {
        pub color: RtRGBA,
        pub intensity: f32
    }

    impl RtShader for Emission {
        fn clone_dyn(&self) -> Box<dyn RtShader> {
            Box::new(self.clone())
        }

        fn evaluate(&self, _scene: &RtScene, _sg: &RtShaderGlobals) -> RtRGBA {
            self.color * self.intensity
        }
    }
    ```

    A shader that needs incoming light builds a ray with `RtMakeRay(sg, ...)`
    (it increments `bounces` and keeps the pixel coordinates), sets its
    direction, **offsets the origin** off the surface, and calls
    `RtTraceRay(scene, &ray)`. A `None` result means the path was terminated
    by `max_bounces` and must contribute black.

2. **Register the module** in `src/rt_shaders.rs` (`pub mod emission;`).

3. **XML loader.** In `XMLScene::get_shaders` (`src/rt_scene.rs`), add a branch
   for `"emission"` that reads its parameters with `get_rgb_parameter` /
   `get_float_parameter` and inserts the boxed shader into `shaders_list`.

4. **Python exporter.** In `ui/render/raito.py`, add the shader and its
   exported attributes to `SHADER_PARAMS`:

    ```python
    SHADER_PARAMS = {
        ...
        "emission": ["color", "intensity"],
    }
    ```

    Attribute types are mapped by `_param`: `color` → `rgb`, `float`, `int`,
    `point`, `string`, `node`.

5. **Editor node.** In `ui/core/nodes.py`, register an `emission_material`
   `NodeType` with `color` and `intensity` parameters and
   `cook=_material_cook("emission", "color", "intensity")`, like
   `metal_material`. The node type id must be `<shader>_material`: that is how
   `_material_cook` finds the parameter specs.

6. **Test.** Add a scene in `tests/scenes/` and render it headless:

    ```bash
    cargo run --release -- --scene tests/scenes/scene_emission.xml --output out.ppm
    ```


## Add a geometry type

Example: a triangle or a plane.

1. **Object.** In `src/rt_objects/rt_geometries.rs`, add a `Clone` struct with
   an `object_params: ObjectParams` field and implement `RtObject`:
    - `get_intersection` returns the closest `t >= 0` (ideally `t > t_min`)
      as `RtRayHit::new(true, t, ray.origin + t * ray.dir)`.
    - `get_normal(point)` returns the outward unit normal at `point`. For a
      triangle the normal does not depend on `point`; store it.
    - If the shader needs more than `P` and `N` (UVs, barycentrics), override
      `get_sg` and add the fields to `RtShaderGlobals` first.

2. **XML loader.** In `XMLScene::as_rt_scene`, the shape loop rejects every
   type other than `"sphere"`. Replace that check with a `match` on the type
   and build the new object from its parameters. Shape parameters are read with
   `get_point_parameter`, `get_float_parameter`, `get_vec_parameter`.

3. **Python exporter.** In `scene_to_xml`, add a loop over
   `scene.of_type("<type>")` that writes the `<shape>` tag, the `shader` node
   parameter and the geometry parameters.

4. **Editor node.** Add the node type in `ui/core/nodes.py` that creates the
   location under `/root/world/geo` with the `geometry.*` attributes.

Since there is no acceleration structure, render time grows linearly with the
number of shapes. A mesh with thousands of triangles will need the BVH
(roadmap 0.1.0) before it is usable.


## Add a light

Lights are the least finished part of the engine. To get real light sources:

1. Make the light implement `RtObject` (`RtPointLight` already does; it has no
   intensity of its own and relies on its shader, `LightShader`).
2. Load lights in `as_rt_scene` and add them with `scene.add_light`.
3. Decide on the transport strategy:
    - **Hit lights by chance**: uncomment the lights loop in `RtTraceRay` so
      that secondary rays can hit them. Simple, but small lights are very
      noisy.
    - **Next event estimation**: in each shader, sample a point on each light,
      cast a shadow ray with `RtTraceToLights` (or a dedicated occlusion test),
      and add the direct contribution. Then indirect rays must ignore lights
      to avoid counting them twice.
4. Replace the hard-coded sky gradient in `RtTraceRay` with a skydome light
   (`RtSkydomeLight` is already declared with `color` and `intensity`) so that
   the background can be turned off or changed from the scene.


## Add a render setting

Example: image resolution.

1. Add the field to `RtRenderSettings` (`src/rt_scene.rs`), its constructor
   and `update`.
2. Read it in `XMLScene::get_settings` (and give it a default so that older
   scenes still load).
3. Use it: resolution goes to `RtCamera::new` in `XMLScene::get_camera`,
   which today hard-codes `1.0, 400`.
4. Export it from `scene_to_xml` (read it from the root `renderSettings.*`
   attributes with a default, like `spp`).
5. Expose it on the `render_settings` node in the editor.

!!! warning

    Non-square images and image sizes that are not a multiple of 50 will
    expose the bucket coordinate swap described in
    [Known issues](known-issues.md#rendering-and-threading). Fix it together
    with this change.


## Conventions

- Types and public functions keep the Arnold-like `Rt` prefix (`RtVec3`,
  `RtTraceRay`). The crate allows non-snake-case names for this.
- Each source file starts with the `Raito Render / Module description` banner
  and groups code under `// ====` section headers.
- Shaders are stateless and `Send + Sync`: everything they need comes from
  `RtShaderGlobals` and the `&RtScene`.
- The library must not depend on the UI. It currently imports `egui::Color32`
  in `rt_types.rs` and `rt_render_output.rs` (for `to_color32` and PPM
  export); avoid adding new UI dependencies, and prefer moving those out.
