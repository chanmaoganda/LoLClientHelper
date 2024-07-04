use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    pub display_name: String,
    pub id: u16,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RuneTree {
    pub display_name: String,
    pub id: u16,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatRune {
    pub id: u16,
    pub raw_description: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FullRunes {
    pub keystone: Rune,
    pub primary_tree: RuneTree,
    pub secondary_tree: RuneTree,
    pub general_runes: Vec<Rune>,
    pub stat_runes: [StatRune; 3],
}