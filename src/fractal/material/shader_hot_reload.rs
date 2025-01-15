use crate::fractal::material::{FRACTAL_SHADER_F64_HANDLE, FRACTAL_SHADER_HANDLE};
use bevy::{prelude::*, render::render_resource::ShaderDefVal};

#[derive(Debug, Clone, Copy, Default)]
pub struct ShaderHotReloadPlugin;

impl Plugin for ShaderHotReloadPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TempShaderAsset(Handle::default()));

        app.add_systems(Startup, load_temp_shader);
        app.add_systems(
            PreUpdate,
            finalize_shader.run_if(on_event::<AssetEvent<Shader>>),
        );
    }
}

#[derive(Debug, Clone, Deref, DerefMut, Resource)]
pub struct TempShaderAsset(Handle<Shader>);

// load the temporary shader from the asset server for hot reloading
pub fn load_temp_shader(asset_server: Res<AssetServer>, mut temp_asset: ResMut<TempShaderAsset>) {
    let shader = asset_server.load::<Shader>("shaders/fractal.wgsl");
    temp_asset.0 = shader;
}

// add deps to the loaded shader and store it in another handle
// this allows us to add defs to the shader since you can't do that
// in Material2d
// it also allows us to update both f32 and f64 versions of the shader at the same time
pub fn finalize_shader(
    mut asset_events: EventReader<AssetEvent<Shader>>,
    mut shaders: ResMut<Assets<Shader>>,
    temp_asset: Res<TempShaderAsset>,
) {
    for event in asset_events.read().copied() {
        use AssetEvent as E;
        let (E::Modified { id } | E::LoadedWithDependencies { id }) = event else {
            continue;
        };
        if id != temp_asset.id() {
            continue;
        }
        let Some(mut temp_shader) = shaders.get(&temp_asset.0).cloned() else {
            warn!("Shader change detected but no shader in assets");
            continue;
        };

        shaders.insert(&FRACTAL_SHADER_HANDLE, temp_shader.clone());
        temp_shader
            .shader_defs
            .push(ShaderDefVal::Bool("DOUBLE_PRECISION".into(), true));

        shaders.insert(&FRACTAL_SHADER_F64_HANDLE, temp_shader);
    }
}
