use crate::{hittable::HitPayload, ray::Ray, texture::Texture, Color};

mod dielectric;
mod diffuse_light;
mod isotropic;
mod lambertian;
mod metal;

use dielectric::DielectricMaterial;
use diffuse_light::DiffuseLightMaterial;
use isotropic::IsotropicMaterial;
use lambertian::LambertianMaterial;
use metal::MetalMaterial;
use std::sync::Arc;

#[derive(Clone)]
pub enum Material {
    Lambertian(LambertianMaterial),
    Metal(MetalMaterial),
    Dielectric(DielectricMaterial),
    Isotropic(IsotropicMaterial),
    DiffuseLight(DiffuseLightMaterial),
}

impl Material {
    pub(crate) fn scatter(&self, r_in: &Ray, payload: &HitPayload) -> Option<(Color, Ray)> {
        match &self {
            &Self::Lambertian(material) => material.scatter(payload),
            &Self::Metal(material) => material.scatter(r_in, payload),
            &Self::Dielectric(material) => material.scatter(r_in, payload),
            &Self::Isotropic(material) => material.scatter(payload),
            &Self::DiffuseLight(_) => None,
        }
    }

    pub(crate) fn emitted(&self, payload: &HitPayload) -> Color {
        match &self {
            &Self::DiffuseLight(material) => material.emitted(payload),
            _ => Color::from([0.; 3]),
        }
    }

    #[allow(private_bounds)]
    pub fn lambertian<T: Texture + Send + Sync + 'static>(albedo: Arc<T>) -> Self {
        Self::Lambertian(LambertianMaterial::new(albedo))
    }

    pub fn metal(albedo: Color, fuzz: f64) -> Self {
        Self::Metal(MetalMaterial::new(albedo, fuzz))
    }

    pub fn dielectric(ir: f64) -> Self {
        Self::Dielectric(DielectricMaterial::new(ir))
    }

    #[allow(private_bounds)]
    pub fn isotropic<T: Texture + Send + Sync + 'static>(albedo: Arc<T>) -> Self {
        Self::Isotropic(IsotropicMaterial::new(albedo))
    }

    #[allow(private_bounds)]
    pub fn diffuse_light<T: Texture + Send + Sync + 'static>(emit: Arc<T>) -> Self {
        Self::DiffuseLight(DiffuseLightMaterial::new(emit))
    }
}
