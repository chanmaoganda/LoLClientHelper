use serde::Deserialize;


#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Drake {
    ChemtechDrake,
    CloudDrake,
    HextechDrake,
    InfernalDrake,
    MountainDrake,
    OceanDrake,
    ElderDrake,
}