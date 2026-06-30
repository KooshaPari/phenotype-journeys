"""Headless Blender 3D glassmorphic app-icon renderer (v2 — quality iteration).
Run: blender -b -P glass_icon.py -- <slug> <out.png>
Tokens: teal #7ebab5, midnight #090a0c. Rounded glass tile + emissive teal GRAPH
(nodes + connecting tubes) = traceability spine. Camera pulled back, tile fully framed,
true-midnight world, teal emission that actually reads, glass rim.
"""
import bpy, sys, math

argv = sys.argv
argv = argv[argv.index("--") + 1:] if "--" in argv else ["app", "//out.png"]
SLUG = argv[0] if len(argv) > 0 else "app"
OUT = argv[1] if len(argv) > 1 else "//out.png"

# saturated teal so COLOR reads under emission (not clipped to white)
TEAL = (0.18, 0.62, 0.56, 1.0)
TEAL_HI = (0.75, 0.95, 0.92, 1.0)
MID  = (0.020, 0.024, 0.030, 1.0)

def clear():
    bpy.ops.object.select_all(action="SELECT"); bpy.ops.object.delete()

def rounded_tile():
    bpy.ops.mesh.primitive_cube_add(size=2)
    t = bpy.context.active_object
    t.scale = (1.0, 1.0, 0.16)
    bev = t.modifiers.new("bev", "BEVEL"); bev.width = 0.4; bev.segments = 16
    sub = t.modifiers.new("sub", "SUBSURF"); sub.levels = 2; sub.render_levels = 3
    bpy.ops.object.shade_smooth()
    return t

def principled(name, base, rough=0.08, transmission=0.0, emis=None, estr=0.0, metallic=0.0):
    m = bpy.data.materials.new(name); m.use_nodes = True
    b = m.node_tree.nodes.get("Principled BSDF")
    b.inputs["Base Color"].default_value = base
    b.inputs["Roughness"].default_value = rough
    b.inputs["Metallic"].default_value = metallic
    if "Transmission Weight" in b.inputs: b.inputs["Transmission Weight"].default_value = transmission
    if "IOR" in b.inputs: b.inputs["IOR"].default_value = 1.45
    if emis and "Emission Color" in b.inputs:
        b.inputs["Emission Color"].default_value = emis
        b.inputs["Emission Strength"].default_value = estr
    return m

def sphere(pos, r, mat):
    bpy.ops.mesh.primitive_uv_sphere_add(radius=r, location=pos, segments=48, ring_count=24)
    s = bpy.context.active_object; bpy.ops.object.shade_smooth(); s.data.materials.append(mat); return s

def tube(p1, p2, r, mat):
    import mathutils
    v = mathutils.Vector(p2) - mathutils.Vector(p1)
    mid = (mathutils.Vector(p1) + mathutils.Vector(p2)) / 2
    bpy.ops.mesh.primitive_cylinder_add(radius=r, depth=v.length, location=mid, vertices=24)
    c = bpy.context.active_object
    c.rotation_mode = "QUATERNION"
    c.rotation_quaternion = v.to_track_quat("Z", "Y")
    bpy.ops.object.shade_smooth(); c.data.materials.append(mat); return c

clear()

# world: true midnight, very low ambient
w = bpy.data.worlds["World"]; w.use_nodes = True
bg = w.node_tree.nodes["Background"]
bg.inputs[0].default_value = MID; bg.inputs[1].default_value = 0.15

# dark glass tile
tile = rounded_tile()
tile.data.materials.append(principled("tile", (0.03, 0.05, 0.06, 1.0), rough=0.05, transmission=0.6))

# teal emissive graph spine: 3 nodes + 2 connecting tubes, lifted above the tile face
ZT = 0.30
# lower emission so the teal hue survives instead of clipping to white
node_mat = principled("node", TEAL, rough=0.15, emis=TEAL, estr=1.8)
link_mat = principled("link", TEAL, rough=0.2, emis=TEAL, estr=1.2)

def motif_tracera():
    P = [(-0.45, 0.38, ZT), (0.30, 0.02, ZT), (-0.05, -0.45, ZT)]
    tube(P[0], P[1], 0.045, link_mat); tube(P[1], P[2], 0.045, link_mat)
    for p, r in zip(P, (0.17, 0.22, 0.15)): sphere(p, r, node_mat)

def motif_tokn():
    bpy.ops.mesh.primitive_torus_add(location=(0, 0, ZT), major_radius=0.42, minor_radius=0.08)
    t = bpy.context.active_object; bpy.ops.object.shade_smooth(); t.data.materials.append(node_mat)
    sphere((0, 0, ZT), 0.16, node_mat)

def motif_byteport():
    for i, dz in enumerate((-0.04, 0.04, 0.12)):
        bpy.ops.mesh.primitive_cube_add(size=0.7, location=(i * 0.12 - 0.12, -i * 0.10 + 0.10, ZT + dz))
        s = bpy.context.active_object; s.scale = (1.0, 0.62, 0.06)
        b = s.modifiers.new("b", "BEVEL"); b.width = 0.06; b.segments = 6
        bpy.ops.object.shade_smooth(); s.data.materials.append(link_mat if i < 2 else node_mat)

def motif_agileplus():
    for cx in (-0.42, 0.0, 0.42):
        for cy in (0.30, -0.05):
            bpy.ops.mesh.primitive_cube_add(size=0.30, location=(cx, cy, ZT))
            s = bpy.context.active_object; s.scale = (1.0, 0.7, 0.18)
            b = s.modifiers.new("b", "BEVEL"); b.width = 0.05; b.segments = 5
            bpy.ops.object.shade_smooth(); s.data.materials.append(node_mat)

MOTIFS = {"tracera": motif_tracera, "tokn": motif_tokn,
          "byteport": motif_byteport, "agileplus": motif_agileplus, "app": motif_tracera}
MOTIFS.get(SLUG, motif_tracera)()

# camera: pulled BACK, ortho-ish, tile fully in frame
bpy.ops.object.camera_add(location=(0.0, -1.1, 4.2), rotation=(math.radians(15), 0, 0))
cam = bpy.context.active_object; bpy.context.scene.camera = cam
cam.data.lens = 80   # longer lens = less distortion, tighter but framed

# lights: dim, cool — let the EMISSION carry the teal (don't wash it white)
bpy.ops.object.light_add(type="AREA", location=(2.2, -1.6, 3.2)); k = bpy.context.active_object
k.data.energy = 120; k.data.size = 6; k.data.color = (0.8, 0.9, 1.0)
bpy.ops.object.light_add(type="AREA", location=(-2.0, 1.2, 1.6)); r = bpy.context.active_object
r.data.energy = 60; r.data.size = 5; r.data.color = TEAL[:3]

sc = bpy.context.scene
sc.render.engine = "CYCLES"
try: sc.cycles.device = "GPU"
except Exception: pass
sc.cycles.samples = 160
sc.render.film_transparent = True
sc.render.resolution_x = 512; sc.render.resolution_y = 512
sc.render.image_settings.file_format = "PNG"; sc.render.image_settings.color_mode = "RGBA"
# subtle bloom via compositor glare so emission glows
sc.use_nodes = True
nt = sc.node_tree
rl = nt.nodes.get("Render Layers") or nt.nodes.new("CompositorNodeRLayers")
comp = nt.nodes.get("Composite") or nt.nodes.new("CompositorNodeComposite")
glare = nt.nodes.new("CompositorNodeGlare"); glare.glare_type = "FOG_GLOW"; glare.quality = "HIGH"; glare.threshold = 0.6
nt.links.new(rl.outputs["Image"], glare.inputs["Image"])
nt.links.new(glare.outputs["Image"], comp.inputs["Image"])
sc.render.filepath = OUT
bpy.ops.render.render(write_still=True)
print(f"[asset-engine] rendered {SLUG} -> {OUT}")
