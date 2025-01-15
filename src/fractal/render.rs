use super::{material::FractalMaterial, Fractal};
use crate::ui::UiSystemSet;
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        mesh::{MeshVertexAttribute, VertexAttributeValues},
        renderer::RenderDevice,
    },
};
use wgpu::{Features, PrimitiveTopology, VertexFormat};

pub const FRACTAL_MESH_HANDLE: Handle<Mesh> =
    Handle::weak_from_u128(0xf63c7bcd_c2e8_46c1_b057_d18549ef3415);

#[derive(Debug, Clone, Copy, Default)]
pub struct FractalRenderPlugin;

impl Plugin for FractalRenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DoublePrecisionSupported(false));
        app.add_systems(PreStartup, set_double_precision_supported);
        app.add_systems(PreUpdate, init_fractal_renderer);
        app.add_systems(Update, swap_fractal_materials.after(UiSystemSet));

        let mut meshes = app.world_mut().resource_mut::<Assets<Mesh>>();
        meshes.insert(&FRACTAL_MESH_HANDLE, create_fractal_mesh());
    }
}

#[derive(Debug, Clone, Copy, Deref, DerefMut, Default, Resource)]
pub struct DoublePrecisionSupported(pub bool);

#[derive(Debug, Clone, Component, Default)]
pub struct FractalRenderer {
    material_f32_handle: Handle<FractalMaterial<f32>>,
    material_f64_handle: Handle<FractalMaterial<f64>>,
}

pub fn set_double_precision_supported(
    mut f64_supported: ResMut<DoublePrecisionSupported>,
    device: Res<RenderDevice>,
) {
    let supported = device
        .features()
        .contains(Features::SHADER_F64 | Features::SHADER_INT64);

    f64_supported.0 = supported;
    info!(?supported, "Set DoublePrecisionSupported");
}

pub fn init_fractal_renderer(
    mut commands: Commands,
    mut fractals: Query<(Entity, &Fractal, &mut FractalRenderer), Added<FractalRenderer>>,
    mut material_f32_assets: ResMut<Assets<FractalMaterial<f32>>>,
    mut material_f64_assets: ResMut<Assets<FractalMaterial<f64>>>,
) {
    for (id, &fractal, mut renderer) in fractals.iter_mut() {
        let material_f32_handle = material_f32_assets.add(FractalMaterial::from(fractal));
        let material_f64_handle = material_f64_assets.add(FractalMaterial::from(fractal));

        *renderer = FractalRenderer {
            material_f32_handle,
            material_f64_handle,
        };

        commands.entity(id).insert(Mesh2d(FRACTAL_MESH_HANDLE));
    }
}

#[allow(clippy::type_complexity)]
pub fn swap_fractal_materials(
    mut commands: Commands,
    fractals: Query<
        (
            Entity,
            &Fractal,
            &FractalRenderer,
            Option<&MeshMaterial2d<FractalMaterial<f32>>>,
            Option<&MeshMaterial2d<FractalMaterial<f64>>>,
        ),
        Changed<Fractal>,
    >,
    f64_supported: Res<DoublePrecisionSupported>,
) {
    for (id, fractal, renderer, material_f32, material_f64) in fractals.iter() {
        let mut fractal_ref = commands.entity(id);
        if fractal.use_f64 && f64_supported.0 {
            if material_f32.is_some() {
                fractal_ref.remove::<MeshMaterial2d<FractalMaterial<f32>>>();
                debug!("Removed f32 material from fractal");
            }
            if material_f64.is_none() {
                fractal_ref.insert(MeshMaterial2d(renderer.material_f64_handle.clone()));
                debug!("Added f64 material to fractal");
            }
        } else {
            if material_f64.is_some() {
                fractal_ref.remove::<MeshMaterial2d<FractalMaterial<f64>>>();
                debug!("Removed f64 material from fractal");
            }
            if material_f32.is_none() {
                fractal_ref.insert(MeshMaterial2d(renderer.material_f32_handle.clone()));
                debug!("Added f32 material to fractal");
            }
        }
    }
}

pub fn create_fractal_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        MeshVertexAttribute::new("FRACTAL_POSITION", 0, VertexFormat::Float32x2),
        // a tri that will cover the entire viewport
        VertexAttributeValues::Float32x2(vec![[-1.0, -1.0], [-1.0, 3.0], [3.0, -1.0]]),
    )
}
