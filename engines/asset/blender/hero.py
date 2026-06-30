"""Headless Blender wide hero / OG-social image (1200x630) per app.
Run: blender -b -P hero.py -- <slug> <out.png>
Reuses the glass motif but in a cinematic wide frame: large floating glass emblem
left-of-center, soft teal volumetric glow, midnight gradient — for landing hero + OG cards.
Text (app name) is added later via magick (Blender text needs a font path; keep render clean).
"""
import bpy, sys, math, mathutils

argv = sys.argv
argv = argv[argv.index("--") + 1:] if "--" in argv else ["app", "//hero.png"]
SLUG = argv[0] if len(argv) > 0 else "app"
OUT = argv[1] if len(argv) > 1 else "//hero.png"

TEAL = (0.18, 0.62, 0.56, 1.0)
MID  = (0.020, 0.024, 0.030, 1.0)

def clear():
    bpy.ops.object.select_all(action="SELECT"); bpy.ops.object.delete()

def principled(name, base, rough=0.08, transmission=0.0, emis=None, estr=0.0):
    m = bpy.data.materials.new(name); m.use_nodes = True
    b = m.node_tree.nodes.get("Principled BSDF")
    b.inputs["Base Color"].default_value = base
    b.inputs["Roughness"].default_value = rough
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
    v = mathutils.Vector(p2) - mathutils.Vector(p1); mid = (mathutils.Vector(p1)+mathutils.Vector(p2))/2
    bpy.ops.mesh.primitive_cylinder_add(radius=r, depth=v.length, location=mid, vertices=24)
    c = bpy.context.active_object; c.rotation_mode="QUATERNION"; c.rotation_quaternion=v.to_track_quat("Z","Y")
    bpy.ops.object.shade_smooth(); c.data.materials.append(mat); return c

clear()
w = bpy.data.worlds["World"]; w.use_nodes = True
bg = w.node_tree.nodes["Background"]; bg.inputs[0].default_value = MID; bg.inputs[1].default_value = 0.12

node_mat = principled("node", TEAL, rough=0.15, emis=TEAL, estr=1.8)
link_mat = principled("link", TEAL, rough=0.2, emis=TEAL, estr=1.2)
glass = principled("emblem_glass", (0.03, 0.05, 0.06, 1.0), rough=0.05, transmission=0.7)

# big rounded glass emblem slab, left third
bpy.ops.mesh.primitive_cube_add(size=2, location=(-1.6, 0, 0))
slab = bpy.context.active_object; slab.scale=(0.9,0.9,0.14)
bev=slab.modifiers.new("b","BEVEL"); bev.width=0.35; bev.segments=14
sub=slab.modifiers.new("s","SUBSURF"); sub.levels=2; sub.render_levels=3
bpy.ops.object.shade_smooth(); slab.data.materials.append(glass)

ZT=0.30; OX=-1.6
def at(p): return (p[0]+OX, p[1], p[2])
if SLUG == "tokn":
    bpy.ops.mesh.primitive_torus_add(location=at((0,0,ZT)), major_radius=0.42, minor_radius=0.08)
    t=bpy.context.active_object; bpy.ops.object.shade_smooth(); t.data.materials.append(node_mat)
    sphere(at((0,0,ZT)), 0.16, node_mat)
elif SLUG == "byteport":
    for i,dz in enumerate((-0.04,0.04,0.12)):
        bpy.ops.mesh.primitive_cube_add(size=0.7, location=at((i*0.12-0.12,-i*0.10+0.10,ZT+dz)))
        s=bpy.context.active_object; s.scale=(1.0,0.62,0.06); s.modifiers.new("b","BEVEL").width=0.06
        bpy.ops.object.shade_smooth(); s.data.materials.append(link_mat if i<2 else node_mat)
elif SLUG == "agileplus":
    for cx in (-0.42,0.0,0.42):
        for cy in (0.30,-0.05):
            bpy.ops.mesh.primitive_cube_add(size=0.30, location=at((cx,cy,ZT)))
            s=bpy.context.active_object; s.scale=(1.0,0.7,0.18); s.modifiers.new("b","BEVEL").width=0.05
            bpy.ops.object.shade_smooth(); s.data.materials.append(node_mat)
else:  # tracera / app
    P=[at((-0.45,0.38,ZT)), at((0.30,0.02,ZT)), at((-0.05,-0.45,ZT))]
    tube(P[0],P[1],0.045,link_mat); tube(P[1],P[2],0.045,link_mat)
    for p,r in zip(P,(0.17,0.22,0.15)): sphere(p,r,node_mat)

# camera: wide, pulled back + raised so the WHOLE emblem sits in the left third
bpy.ops.object.camera_add(location=(0.6,-5.6,3.4), rotation=(math.radians(58),0,math.radians(10)))
cam=bpy.context.active_object; bpy.context.scene.camera=cam; cam.data.lens=58

bpy.ops.object.light_add(type="AREA", location=(3,-2,4)); k=bpy.context.active_object; k.data.energy=200; k.data.size=8; k.data.color=(0.8,0.9,1.0)
bpy.ops.object.light_add(type="AREA", location=(-3,2,2)); r=bpy.context.active_object; r.data.energy=90; r.data.size=6; r.data.color=TEAL[:3]

sc=bpy.context.scene
sc.render.engine="CYCLES"
try: sc.cycles.device="GPU"
except Exception: pass
sc.cycles.samples=160
sc.render.film_transparent=False
sc.render.resolution_x=1200; sc.render.resolution_y=630
sc.render.image_settings.file_format="PNG"
sc.use_nodes=True; nt=sc.node_tree
rl=nt.nodes.get("Render Layers"); comp=nt.nodes.get("Composite")
glare=nt.nodes.new("CompositorNodeGlare"); glare.glare_type="FOG_GLOW"; glare.quality="HIGH"; glare.threshold=0.5
nt.links.new(rl.outputs["Image"], glare.inputs["Image"]); nt.links.new(glare.outputs["Image"], comp.inputs["Image"])
sc.render.filepath=OUT
bpy.ops.render.render(write_still=True)
print(f"[asset-engine] hero {SLUG} -> {OUT}")
