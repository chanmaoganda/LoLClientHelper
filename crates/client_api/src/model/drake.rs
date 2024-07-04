use serde::Deserialize;


#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum Drake {
    ChemtechDrake,
    CloudDrake,
    HextechDrake,
    InfernalDrake,
    MountainDrake,
    OceanDrake,
    ElderDrake,
}