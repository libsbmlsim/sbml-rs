use super::tag::TagIndex;

#[derive(Clone, Debug, Default)]
pub struct ListOfParameters {
    pub parameters: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Clone, Debug, Default)]
pub struct Parameter {
    pub id: Option<String>,
    pub metaid: Option<String>,
    pub name: Option<String>,
    pub value: Option<f64>,
    pub units: Option<String>,
    pub sbo_term: Option<String>,
    pub constant: Option<bool>,
    pub parent: Option<TagIndex>,
}
