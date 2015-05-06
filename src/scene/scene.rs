use std::ops::Deref;
use std::ptr;
use std::slice::from_raw_parts;

use ffi::*;

// Import all types (and internal traits for instantiating them)
use super::animation::{Animation, AnimationInternal};
use super::camera::{Camera, CameraInternal};
use super::face::{Face, FaceInternal};
use super::light::{Light, LightInternal};
use super::material::{Material, MaterialInternal};
use super::mesh::{Mesh, MeshInternal};
use super::node::{Node, NodeInternal};
use super::texture::{Texture, TextureInternal};

/// The `Scene` type represents immutable scene data.
pub struct Scene(*const AiScene);
/// The `SceneMut` type represents mutable scene data.
pub struct SceneMut(*mut AiScene);


////////////////////////////////////////////////////////////////////////////////////////////////////
// Immutable scene methods
////////////////////////////////////////////////////////////////////////////////////////////////////

impl Scene {
    /// Returns true if the imported scene is not complete.
    pub fn is_incomplete(&self) -> bool {
        self.flags.contains(AI_SCENE_FLAGS_INCOMPLETE)
    }

    /// Returns true if the imported scene was successfully validated by the
    /// `validate_data_structure` post-process step.
    pub fn is_validated(&self) -> bool {
        self.flags.contains(AI_SCENE_FLAGS_VALIDATED)
    }

    /// Returns true if any warnings were generated by the `validate_data_structure`
    /// post-process step. The details of the warnings are written to the output log.
    pub fn has_validation_warning(&self) -> bool {
        self.flags.contains(AI_SCENE_FLAGS_VALIDATION_WARNING)
    }

    /// Returns true if the `join_identical_vertices` post-process step was run.
    pub fn is_non_verbose_format(&self) -> bool {
        self.flags.contains(AI_SCENE_FLAGS_NON_VERBOSE_FORMAT)
    }

    /// Returns true if the imported mesh contained height-map terrain data.
    pub fn is_terrain(&self) -> bool {
        self.flags.contains(AI_SCENE_FLAGS_TERRAIN)
    }

    /// Returns the root node of the scene hierarchy
    pub fn root_node(&self) -> Node {
        Node::new(self.root_node)
    }

    /// Returns the number of meshes in the scene.
    pub fn num_meshes(&self) -> u32 {
        self.num_meshes
    }

    /// Returns a vector containing all of the meshes in the scene
    pub fn meshes(&self) -> Vec<Mesh> {
        let len = self.num_meshes as usize;
        unsafe { from_raw_parts(self.meshes, len).iter().map(|x| Mesh::new(*x)).collect() }
    }

    /// Return an individual mesh from the scene.
    ///
    /// Panics if `id` is invalid.
    pub fn mesh(&self, id: usize) -> Mesh {
        assert!(id < self.num_meshes as usize);
        unsafe { Mesh::new(*(self.meshes.offset(id as isize))) }
    }

    /// Returns the number of materials in the scene.
    pub fn num_materials(&self) -> u32 {
        self.num_materials
    }

    /// Returns a vector containing all of the materials in the scene.
    pub fn materials(&self) -> Vec<Material> {
        let len = self.num_materials as usize;
        unsafe { from_raw_parts(self.materials, len).iter().map(|x| Material::new(*x)).collect() }
    }

    /// Returns the number of animations in the scene.
    pub fn num_animations(&self) -> u32 {
        self.num_animations
    }

    /// Returns a vector containing all of the animations in the scene.
    pub fn animations(&self) -> Vec<Animation> {
        let len = self.num_animations as usize;
        unsafe { from_raw_parts(self.animations, len).iter().map(|x| Animation::new(*x)).collect() }
    }

    /// Returns the number of animations in the scene.
    pub fn num_textures(&self) -> u32 {
        self.num_textures
    }

    /// Returns a vector containing all of the textures in the scene.
    pub fn textures(&self) -> Vec<Texture> {
        unsafe {
            let len = self.num_textures as usize;
            from_raw_parts(self.textures, len).iter().map(|x| Texture::new(*x)).collect()
        }
    }

    /// Returns the number of lights in the scene.
    pub fn num_lights(&self) -> u32 {
        self.num_lights
    }

    /// Returns a vector containing all of the lights in the scene.
    pub fn lights(&self) -> Vec<Light> {
        let len = self.num_lights as usize;
        unsafe { from_raw_parts(self.lights, len).iter().map(|x| Light::new(*x)).collect() }
    }

    /// Returns the number of cameras in the scene.
    pub fn num_cameras(&self) -> u32 {
        self.num_cameras
    }

    /// Returns a vector containing all of the cameras in the scene
    pub fn cameras(&self) -> Vec<Camera> {
        let len = self.num_cameras as usize;
        unsafe { from_raw_parts(self.cameras, len).iter().map(|x| Camera::new(*x)).collect() }
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
// Mutable scene methods
////////////////////////////////////////////////////////////////////////////////////////////////////

impl SceneMut {
    // TODO
}

impl Deref for SceneMut {
    type Target = Scene;
    fn deref<'a>(&'a self) -> &'a Scene {
        unsafe { ::std::mem::transmute(self) }
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
// Implement standard traits
////////////////////////////////////////////////////////////////////////////////////////////////////

// Conversion into a mutable version of the scene
impl From<Scene> for SceneMut {
    fn from(scene: Scene) -> SceneMut {
        let mut new_scene = ptr::null_mut();
        unsafe { aiCopyScene(scene.0, &mut new_scene) };
        SceneMut(new_scene)
    }
}

// Drop implementation for a scene owned by Assimp.
// Scenes returned by aiImportFile* methods must be freed with aiReleaseImport.
impl Drop for Scene {
    fn drop(&mut self) {
        unsafe { aiReleaseImport(self.0); }
    }
}

// Drop implementation for a scene not owned by Assimp.
// Scenes returned by aiCopyScene must be freed with aiFreeScene.
impl Drop for SceneMut {
    fn drop(&mut self) {
        unsafe { aiFreeScene(self.0); }
    }
}

#[doc(hidden)]
mod private {
    use std::ops::Deref;
    use ffi::AiScene;

    impl Deref for super::Scene {
        type Target = AiScene;
        fn deref<'a>(&'a self) -> &'a AiScene { unsafe { &*self.0 } }
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
// Internal implementation details
////////////////////////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
pub trait SceneInternal {
    fn new(raw_scene: *const AiScene) -> Scene { Scene(raw_scene) }
    fn get_raw_ptr(&self) -> *const AiScene;
}

#[doc(hidden)]
pub trait SceneMutInternal: SceneInternal {
    fn new(raw_scene: *mut AiScene) -> SceneMut { SceneMut(raw_scene) }
    fn get_raw_ptr_mut(&mut self) -> *mut AiScene;
}

impl SceneInternal for Scene {
    fn get_raw_ptr(&self) -> *const AiScene { self.0 }
}

impl SceneInternal for SceneMut {
    fn get_raw_ptr(&self) -> *const AiScene { self.0 }
}

impl SceneMutInternal for SceneMut {
    fn get_raw_ptr_mut(&mut self) -> *mut AiScene { self.0 }
}
