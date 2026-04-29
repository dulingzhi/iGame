use crate::{error::MapPackageError, map_package::MapPackage};

/// Validate a [`MapPackage`] and return the first error found, if any.
pub fn validate(package: &MapPackage) -> Result<(), MapPackageError> {
    let m = &package.manifest;

    if m.name.trim().is_empty() {
        return Err(MapPackageError::Validation(
            "Manifest `name` must not be empty".into(),
        ));
    }

    if m.version.trim().is_empty() {
        return Err(MapPackageError::Validation(
            "Manifest `version` must not be empty".into(),
        ));
    }

    if m.entry_scene.trim().is_empty() {
        return Err(MapPackageError::Validation(
            "Manifest `entry_scene` must not be empty".into(),
        ));
    }

    // Entity-level checks
    for (i, entity) in package.scene.entities.iter().enumerate() {
        let t = &entity.transform;
        let scale_zero = t.scale.contains(&0.0);
        if scale_zero {
            return Err(MapPackageError::Validation(format!(
                "Entity[{i}] has a zero scale component, which is not allowed"
            )));
        }
    }

    Ok(())
}
