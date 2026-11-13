# Logo

The Raito logo is a lowercase wordmark set in [Space Grotesk](https://fonts.google.com/specimen/Space+Grotesk) Medium,
where the dot of the *i* is a light and the *o* is a toon shaded sphere.
The app icon reuses the same light and sphere on a dark rounded square.

All files live in [`docs/source/images/logo`](https://github.com/Alxiice/Raito/tree/main/docs/source/images/logo).
The SVGs are the source of truth (text is converted to paths, no font needed) and the PNG / ICO variants are exported from them.


## Wordmark

Use the dark text version on light backgrounds, and the light text version on dark backgrounds.

<div class="grid" markdown>

<figure markdown="span" class="raito-logo-light">
  ![Raito logo, dark text](images/logo/raito-logo-light.svg){ width="320" }
  <figcaption>Dark text · <code>raito-logo-light.svg</code></figcaption>
</figure>

<figure markdown="span" class="raito-logo-dark">
  ![Raito logo, light text](images/logo/raito-logo-dark.svg){ width="320" }
  <figcaption>Light text · <code>raito-logo-dark.svg</code></figcaption>
</figure>

</div>


## App icon

<figure>
  <p class="raito-icon-sizes">
    <img src="images/logo/raito-icon.svg" width="128" alt="Raito app icon">
    <img src="images/logo/png/raito-icon-48.png" alt="Raito app icon, 48 px">
    <img src="images/logo/png/raito-icon-32.png" alt="Raito app icon, 32 px">
    <img src="images/logo/png/raito-icon-16.png" alt="Raito app icon, 16 px">
  </p>
  <figcaption>App icon · <code>raito-icon.svg</code>, <code>raito.ico</code></figcaption>
</figure>

The scene editor (`ui/`) uses it as its window icon (`ui/icons/raito.ico` and `ui/icons/raito.svg`).


## Files

| File | Use |
| --- | --- |
| `raito-logo-light.svg` | Wordmark, dark text for light backgrounds |
| `raito-logo-dark.svg` | Wordmark, light text for dark backgrounds |
| `raito-icon.svg` | App icon |
| `raito.ico` | Windows icon (16, 24, 32, 48, 64, 128, 256 px) |
| `png/raito-logo-{light,dark}-{64,128,256,512}.png` | Wordmark PNGs, by height in px |
| `png/raito-icon-{16,24,32,48,64,128,256,512}.png` | App icon PNGs |

The PNG and ICO files are generated from the SVGs, regenerate them after editing an SVG :

```bash
python scripts/export_logo.py
```


## Colors

| Color | Hex | Role |
| --- | --- | --- |
| Ink | `#0F1B24` | Text on light, sphere shadow, icon background |
| Primary | `#265F80` | Sphere on light, docs theme color |
| Primary on dark | `#4E9AC4` | Sphere on dark |
| Shadow on dark | `#1C3546` | Sphere shadow on dark |
| Sky | `#8EC3E0` | Sphere highlight |
| Amber | `#F2A33A` | The light (dot of the *i*) |
| Paper | `#F4F6F8` | Light background |
| Ink on dark | `#E8EEF2` | Text on dark |
