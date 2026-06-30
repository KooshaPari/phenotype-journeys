# Asset Engine

Blender-driven pipeline for generating rich application branding assets: icons, hero images, favicons, and OG meta tags.

## Structure

- `blender/` - Blender Python scripts for asset generation

## Design Tokens

The asset engine uses a standardized Phenotype design palette:

- **Teal (primary):** `#7ebab5`
- **Midnight (dark accent):** `#090a0c`

Additional tokens follow the [Phenotype Design System](../../docs/design-system.md).

## Scripts

### `glass_icon.py`

Generates application icons with glassmorphic styling.

**Usage:**

```bash
blender -b -P engines/asset/blender/glass_icon.py -- <slug> <output.png>
```

**Parameters:**

- `<slug>` - App identifier (e.g., `tracera`, `agileplus`, `civis`)
- `<output.png>` - Output file path for the PNG icon

**Output:**

Generates a single icon asset. Repeat for different sizes:
- 16x16 (favicon)
- 32x32 (favicon)
- 64x64 (taskbar)
- 128x128 (app launcher)
- 256x256 (app launcher / OG image thumbnail)
- 512x512 (high-res / splash screen)

**Design:**

Uses Phenotype teal/midnight palette with:
- Glassmorphic backdrop blur + frosted effect
- Depth layering for visual hierarchy
- Anti-aliasing for clean edges

### `hero.py`

Generates hero/OG images and 1200x630 social media cards.

**Usage:**

```bash
blender -b -P engines/asset/blender/hero.py -- <slug> <title> <output.png>
```

**Parameters:**

- `<slug>` - App identifier
- `<title>` - Hero title text (e.g., "Ship Features Fast")
- `<output.png>` - Output file path (PNG)

**Output:**

Generates a single 1200x630 image optimized for:
- Twitter/X (OG image)
- OpenGraph (website embeds)
- Slack/Discord cards
- Hero sections on landing pages

**Design:**

Combines:
- Teal/midnight gradient background
- Glassmorphic accent shapes
- Large, readable typography
- Whitespace for clarity

## Pipeline

Typical workflow to generate all assets for a new app (`<slug>`):

```bash
# Generate icons at various sizes
for SIZE in 16 32 64 128 256 512; do
  blender -b -P engines/asset/blender/glass_icon.py -- <slug> "assets/icons/<slug>-${SIZE}.png"
done

# Generate OG hero image
blender -b -P engines/asset/blender/hero.py -- <slug> "My App Title" "assets/heroes/<slug>-og-1200x630.png"

# Generate favicon variants
convert "assets/icons/<slug>-16.png" "assets/favicon-16.ico"
convert "assets/icons/<slug>-32.png" "assets/favicon-32.ico"
convert "assets/icons/<slug>-256.png" "assets/favicon.ico"
```

## Output Directory

Assets are typically stored per-application:

```
apps/<app-name>/
├── assets/
│   ├── icons/
│   │   ├── <app>-16.png
│   │   ├── <app>-32.png
│   │   ├── <app>-64.png
│   │   ├── <app>-128.png
│   │   ├── <app>-256.png
│   │   └── <app>-512.png
│   ├── heroes/
│   │   └── <app>-og-1200x630.png
│   └── favicon/
│       ├── favicon-16.ico
│       ├── favicon-32.ico
│       └── favicon.ico
```

## Integration

These assets integrate with:

- **App launchers** (Start Menu, Dock, etc.) - uses 128-256px icons
- **Web frontends** - favicon + OG image
- **Documentation** - hero images for section headers
- **Social sharing** - OG 1200x630 for link previews

## Dependencies

- **Blender** 3.6+ (headless mode via `-b`)
- **Python 3.8+** (Blender's bundled Python)
- **ImageMagick** (for .ico generation via `convert`)

## Future Enhancements

- Batch processing via `.blend` file templates
- Direct .ico output from Blender (no ImageMagick dependency)
- SVG export for vector scaling
- Animated favicon/loading states via Remotion integration
