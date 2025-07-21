use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "$schema")]
    pub schema: String,
    #[serde(rename = "$id")]
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties,
    pub additional_properties: bool,
    pub required: Vec<String>,
    #[serde(rename = "$defs")]
    pub defs: Defs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub ags_schema: AgsSchema,
    pub ags_file: AgsFile,
    pub ags_project: AgsProject,
    pub agsi_model: AgsiModel,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsSchema {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsFile {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProject {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiModel {
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defs {
    pub coordinate_tuple: CoordinateTuple,
    pub value_profile: ValueProfile,
    pub ags_schema: AgsSchema2,
    pub ags_file: AgsFile2,
    pub ags_project: AgsProject2,
    pub ags_project_coordinate_system: AgsProjectCoordinateSystem2,
    pub ags_project_investigation: AgsProjectInvestigation2,
    pub ags_project_document_set: AgsProjectDocumentSet2,
    pub ags_project_document: AgsProjectDocument2,
    pub ags_project_code_set: AgsProjectCodeSet2,
    pub ags_project_code: AgsProjectCode2,
    pub agsi_model: AgsiModel2,
    pub agsi_model_element: AgsiModelElement2,
    pub agsi_model_boundary: AgsiModelBoundary2,
    pub agsi_model_alignment: AgsiModelAlignment2,
    pub agsi_observation_set: AgsiObservationSet2,
    pub agsi_observation_point: AgsiObservationPoint2,
    pub agsi_observation_shape: AgsiObservationShape2,
    pub agsi_observation_exp_hole: AgsiObservationExpHole2,
    pub agsi_observation_column: AgsiObservationColumn2,
    pub agsi_geometry_from_file: AgsiGeometryFromFile3,
    pub agsi_geometry_layer: AgsiGeometryLayer,
    pub agsi_geometry_plane: AgsiGeometryPlane,
    pub agsi_geometry_vol_from_surfaces: AgsiGeometryVolFromSurfaces,
    pub agsi_geometry_area_from_lines: AgsiGeometryAreaFromLines,
    pub agsi_data_property_value: AgsiDataPropertyValue7,
    pub agsi_data_property_summary: AgsiDataPropertySummary2,
    pub agsi_data_property_from_file: AgsiDataPropertyFromFile7,
    pub agsi_data_parameter_value: AgsiDataParameterValue2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoordinateTuple {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items2,
    pub min_items: i64,
    pub max_items: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items2 {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueProfile {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items3 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items4,
    pub min_items: i64,
    pub max_items: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items4 {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsSchema2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties2,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties2 {
    pub name: Name,
    pub version: Version,
    pub link: Link,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsFile2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties3,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties3 {
    #[serde(rename = "fileUUID")]
    pub file_uuid: FileUuid,
    pub title: Title,
    pub project_title: ProjectTitle,
    pub description: Description,
    pub produced_by: ProducedBy,
    #[serde(rename = "fileURI")]
    pub file_uri: FileUri,
    pub reference: Reference,
    pub revision: Revision,
    pub date: Date,
    pub status: Status,
    pub status_code: StatusCode,
    pub made_by: MadeBy,
    pub checked_by: CheckedBy,
    pub approved_by: ApprovedBy,
    pub remarks: Remarks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUuid {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectTitle {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProducedBy {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUri {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revision {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCode {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MadeBy {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckedBy {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApprovedBy {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProject2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties4,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties4 {
    #[serde(rename = "projectUUID")]
    pub project_uuid: ProjectUuid,
    pub project_name: ProjectName,
    pub producer: Producer,
    pub producer_suppliers: ProducerSuppliers,
    pub client: Client,
    pub description: Description2,
    pub project_country: ProjectCountry,
    #[serde(rename = "producerProjectID")]
    pub producer_project_id: ProducerProjectId,
    #[serde(rename = "clientProjectID")]
    pub client_project_id: ClientProjectId,
    pub parent_project_name: ParentProjectName,
    pub ultimate_project_name: UltimateProjectName,
    pub ultimate_project_client: UltimateProjectClient,
    #[serde(rename = "briefDocumentSetID")]
    pub brief_document_set_id: BriefDocumentSetId,
    #[serde(rename = "reportDocumentSetID")]
    pub report_document_set_id: ReportDocumentSetId,
    pub ags_project_coordinate_system: AgsProjectCoordinateSystem,
    pub ags_project_investigation: AgsProjectInvestigation,
    pub ags_project_document_set: AgsProjectDocumentSet,
    pub ags_project_code_set: AgsProjectCodeSet,
    pub remarks: Remarks2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUuid {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectName {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Producer {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProducerSuppliers {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectCountry {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProducerProjectId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientProjectId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentProjectName {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltimateProjectName {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltimateProjectClient {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BriefDocumentSetId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDocumentSetId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProjectCoordinateSystem {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items5 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProjectInvestigation {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items6 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProjectDocumentSet {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items7 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProjectCodeSet {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items8 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProjectCoordinateSystem2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties5,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties5 {
    #[serde(rename = "systemID")]
    pub system_id: SystemId,
    pub description: Description3,
    pub system_type: SystemType,
    #[serde(rename = "systemNameXY")]
    pub system_name_xy: SystemNameXy,
    pub system_name_z: SystemNameZ,
    pub axis_name_x: AxisNameX,
    pub axis_name_y: AxisNameY,
    pub axis_name_z: AxisNameZ,
    #[serde(rename = "axisUnitsXY")]
    pub axis_units_xy: AxisUnitsXy,
    pub axis_units_z: AxisUnitsZ,
    #[serde(rename = "globalXYSystem")]
    pub global_xysystem: GlobalXysystem,
    #[serde(rename = "globalZSystem")]
    pub global_zsystem: GlobalZsystem,
    pub transform_shift_x: TransformShiftX,
    pub transform_shift_y: TransformShiftY,
    #[serde(rename = "transformXYRotation")]
    pub transform_xyrotation: TransformXyrotation,
    #[serde(rename = "transformXYScaleFactor")]
    pub transform_xyscale_factor: TransformXyscaleFactor,
    pub transform_shift_z: TransformShiftZ,
    pub remarks: Remarks3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemType {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "enum")]
    pub enum_field: Vec<String>,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemNameXy {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemNameZ {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisNameX {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisNameY {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisNameZ {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisUnitsXy {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisUnitsZ {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalXysystem {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalZsystem {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformShiftX {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformShiftY {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformXyrotation {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformXyscaleFactor {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformShiftZ {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProjectInvestigation2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties6,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties6 {
    #[serde(rename = "investigationID")]
    pub investigation_id: InvestigationId,
    pub investigation_name: InvestigationName,
    pub description: Description4,
    pub contractor: Contractor,
    pub client: Client2,
    pub engineer: Engineer,
    pub parent_project_name: ParentProjectName2,
    pub ultimate_project_name: UltimateProjectName2,
    pub ultimate_project_client: UltimateProjectClient2,
    pub subcontractors: Subcontractors,
    pub fieldwork_date_start: FieldworkDateStart,
    pub scope_description: ScopeDescription,
    pub location_coordinate_project: LocationCoordinateProject,
    pub location_coordinate_global: LocationCoordinateGlobal,
    pub location_description: LocationDescription,
    #[serde(rename = "specificationDocumentSetID")]
    pub specification_document_set_id: SpecificationDocumentSetId,
    #[serde(rename = "reportDocumentSetID")]
    pub report_document_set_id: ReportDocumentSetId2,
    #[serde(rename = "dataDocumentSetID")]
    pub data_document_set_id: DataDocumentSetId,
    pub agsi_data_property_value: AgsiDataPropertyValue,
    pub agsi_data_property_from_file: AgsiDataPropertyFromFile,
    pub remarks: Remarks4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvestigationId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvestigationName {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description4 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contractor {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Engineer {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentProjectName2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltimateProjectName2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltimateProjectClient2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subcontractors {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldworkDateStart {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeDescription {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationCoordinateProject {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
    pub example: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationCoordinateGlobal {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
    pub example: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationDescription {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecificationDocumentSetId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDocumentSetId2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataDocumentSetId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyValue {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items9,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items9 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyFromFile {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items10 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks4 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProjectDocumentSet2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties7,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties7 {
    #[serde(rename = "documentSetID")]
    pub document_set_id: DocumentSetId,
    pub description: Description5,
    pub ags_project_document: AgsProjectDocument,
    pub remarks: Remarks5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSetId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description5 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProjectDocument {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items11,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items11 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks5 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProjectDocument2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties8,
    pub additional_properties: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties8 {
    #[serde(rename = "documentID")]
    pub document_id: DocumentId,
    pub title: Title2,
    pub description: Description6,
    pub author: Author,
    pub client: Client3,
    pub original_reference: OriginalReference,
    pub system_reference: SystemReference,
    pub revision: Revision2,
    pub date: Date2,
    pub status: Status2,
    pub status_code: StatusCode2,
    #[serde(rename = "documentURI")]
    pub document_uri: DocumentUri,
    #[serde(rename = "documentSystemURI")]
    pub document_system_uri: DocumentSystemUri,
    pub remarks: Remarks6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description6 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginalReference {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemReference {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revision2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCode2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentUri {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSystemUri {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks6 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProjectCodeSet2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties9,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties9 {
    #[serde(rename = "codeSetID")]
    pub code_set_id: CodeSetId,
    pub description: Description7,
    pub used_by_object: UsedByObject,
    pub used_by_attribute: UsedByAttribute,
    pub source_description: SourceDescription,
    #[serde(rename = "sourceURI")]
    pub source_uri: SourceUri,
    pub concatenation_allow: ConcatenationAllow,
    pub concatenation_character: ConcatenationCharacter,
    pub ags_project_code: AgsProjectCode,
    pub remarks: Remarks7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSetId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description7 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsedByObject {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsedByAttribute {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceDescription {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceUri {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConcatenationAllow {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConcatenationCharacter {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProjectCode {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items12,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items12 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks7 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProjectCode2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties10,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties10 {
    #[serde(rename = "codeID")]
    pub code_id: CodeId,
    pub description: Description8,
    pub units: Units,
    pub is_standard: IsStandard,
    pub remarks: Remarks8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description8 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Units {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsStandard {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks8 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiModel2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties11,
    pub additional_properties: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties11 {
    #[serde(rename = "modelID")]
    pub model_id: ModelId,
    pub model_name: ModelName,
    pub description: Description9,
    #[serde(rename = "coordSystemID")]
    pub coord_system_id: CoordSystemId,
    pub model_type: ModelType,
    pub category: Category,
    pub domain: Domain,
    pub input: Input,
    pub method: Method,
    pub usage: Usage,
    pub uncertainty: Uncertainty,
    #[serde(rename = "documentSetID")]
    pub document_set_id: DocumentSetId2,
    #[serde(rename = "alignmentID")]
    pub alignment_id: AlignmentId,
    pub agsi_model_element: AgsiModelElement,
    pub agsi_model_boundary: AgsiModelBoundary,
    pub agsi_model_alignment: AgsiModelAlignment,
    pub agsi_observation_set: AgsiObservationSet,
    pub remarks: Remarks9,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelName {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description9 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoordSystemId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelType {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uncertainty {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSetId2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlignmentId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiModelElement {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items13,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items13 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiModelBoundary {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiModelAlignment {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items14,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items14 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiObservationSet {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items15,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items15 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks9 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiModelElement2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties12,
    pub additional_properties: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties12 {
    #[serde(rename = "elementID")]
    pub element_id: ElementId,
    pub element_name: ElementName,
    pub description: Description10,
    pub element_type: ElementType,
    pub geometry_object: GeometryObject,
    pub agsi_geometry: AgsiGeometry,
    pub agsi_geometry_area_limit: AgsiGeometryAreaLimit,
    pub agsi_data_parameter_value: AgsiDataParameterValue,
    pub agsi_data_property_value: AgsiDataPropertyValue2,
    pub agsi_data_property_summary: AgsiDataPropertySummary,
    pub agsi_data_property_from_file: AgsiDataPropertyFromFile2,
    #[serde(rename = "colourRGB")]
    pub colour_rgb: ColourRgb,
    pub remarks: Remarks10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementName {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description10 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementType {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryObject {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometry {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub any_of: Vec<AnyOf>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyOf {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometryAreaLimit {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataParameterValue {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items16 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyValue2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items17,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items17 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertySummary {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items18,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items18 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyFromFile2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColourRgb {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks10 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiModelBoundary2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties13,
    pub additional_properties: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties13 {
    #[serde(rename = "boundaryID")]
    pub boundary_id: BoundaryId,
    pub description: Description11,
    pub min_x: MinX,
    pub max_x: MaxX,
    pub min_y: MinY,
    pub max_y: MaxY,
    pub top_elevation: TopElevation,
    pub bottom_elevation: BottomElevation,
    #[serde(rename = "agsiGeometryBoundaryXY")]
    pub agsi_geometry_boundary_xy: AgsiGeometryBoundaryXy,
    pub agsi_geometry_surface_top: AgsiGeometrySurfaceTop,
    pub agsi_geometry_surface_bottom: AgsiGeometrySurfaceBottom,
    pub remarks: Remarks11,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundaryId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description11 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinX {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxX {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinY {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxY {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopElevation {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BottomElevation {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometryBoundaryXy {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometrySurfaceTop {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometrySurfaceBottom {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks11 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiModelAlignment2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties14,
    pub additional_properties: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties14 {
    #[serde(rename = "alignmentID")]
    pub alignment_id: AlignmentId2,
    pub alignment_name: AlignmentName,
    pub description: Description12,
    pub agsi_geometry: AgsiGeometry2,
    pub start_chainage: StartChainage,
    pub remarks: Remarks12,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlignmentId2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlignmentName {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description12 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometry2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartChainage {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks12 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiObservationSet2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties15,
    pub additional_properties: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties15 {
    #[serde(rename = "observationSetID")]
    pub observation_set_id: ObservationSetId,
    pub description: Description13,
    #[serde(rename = "investigationID")]
    pub investigation_id: InvestigationId2,
    #[serde(rename = "documentSetID")]
    pub document_set_id: DocumentSetId3,
    pub agsi_observation_exp_hole: AgsiObservationExpHole,
    pub agsi_observation_point: AgsiObservationPoint,
    pub agsi_observation_shape: AgsiObservationShape,
    pub agsi_geometry_from_file: AgsiGeometryFromFile,
    pub agsi_data_property_from_file: AgsiDataPropertyFromFile3,
    pub remarks: Remarks13,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationSetId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description13 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvestigationId2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSetId3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiObservationExpHole {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items19,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items19 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiObservationPoint {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items20,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items20 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiObservationShape {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items21,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items21 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometryFromFile {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyFromFile3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks13 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiObservationPoint2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties16,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties16 {
    #[serde(rename = "observationID")]
    pub observation_id: ObservationId,
    pub observation_nature: ObservationNature,
    pub coordinate: Coordinate,
    pub made_by: MadeBy2,
    pub date: Date3,
    pub observation_text: ObservationText,
    pub agsi_data_property_value: AgsiDataPropertyValue3,
    pub agsi_data_property_from_file: AgsiDataPropertyFromFile4,
    pub remarks: Remarks14,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationNature {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinate {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
    pub example: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MadeBy2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationText {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyValue3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items22,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items22 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyFromFile4 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks14 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiObservationShape2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties17,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties17 {
    #[serde(rename = "observationID")]
    pub observation_id: ObservationId2,
    pub observation_nature: ObservationNature2,
    pub made_by: MadeBy3,
    pub date: Date4,
    pub observation_text: ObservationText2,
    pub agsi_geometry_from_file: AgsiGeometryFromFile2,
    pub agsi_data_property_value: AgsiDataPropertyValue4,
    pub agsi_data_property_from_file: AgsiDataPropertyFromFile5,
    pub remarks: Remarks15,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationId2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationNature2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MadeBy3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date4 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationText2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometryFromFile2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyValue4 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items23,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items23 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyFromFile5 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks15 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiObservationExpHole2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties18,
    pub additional_properties: bool,
    pub any_of: Vec<AnyOf2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties18 {
    #[serde(rename = "holeID")]
    pub hole_id: HoleId,
    #[serde(rename = "holeUUID")]
    pub hole_uuid: HoleUuid,
    pub hole_name: HoleName,
    pub top_coordinate: TopCoordinate,
    pub vertical_hole_depth: VerticalHoleDepth,
    pub profile_coordinates: ProfileCoordinates,
    pub hole_type: HoleType,
    pub date: Date5,
    pub agsi_observation_column: AgsiObservationColumn,
    pub agsi_data_property_value: AgsiDataPropertyValue5,
    pub agsi_data_property_from_file: AgsiDataPropertyFromFile6,
    pub remarks: Remarks16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoleId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoleUuid {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoleName {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopCoordinate {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
    pub example: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerticalHoleDepth {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileCoordinates {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items24,
    pub example: Vec<Vec<f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items24 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoleType {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date5 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiObservationColumn {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items25,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items25 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyValue5 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items26,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items26 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyFromFile6 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks16 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyOf2 {
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiObservationColumn2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties19,
    pub additional_properties: bool,
    pub any_of: Vec<AnyOf3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties19 {
    #[serde(rename = "columnID")]
    pub column_id: ColumnId,
    pub top_depth: TopDepth,
    pub bottom_depth: BottomDepth,
    pub top_elevation: TopElevation2,
    pub bottom_elevation: BottomElevation2,
    pub description: Description14,
    pub legend_code: LegendCode,
    pub geology_code: GeologyCode,
    pub geology_code2: GeologyCode2,
    pub geology_formation: GeologyFormation,
    #[serde(rename = "geologyBGS")]
    pub geology_bgs: GeologyBgs,
    pub agsi_data_property_value: AgsiDataPropertyValue6,
    pub remarks: Remarks17,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopDepth {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BottomDepth {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopElevation2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BottomElevation2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description14 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegendCode {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeologyCode {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeologyCode2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeologyFormation {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeologyBgs {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyValue6 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Items27,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items27 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks17 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyOf3 {
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometryFromFile3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties20,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties20 {
    #[serde(rename = "geometryID")]
    pub geometry_id: GeometryId,
    pub description: Description15,
    pub geometry_type: GeometryType,
    pub file_format: FileFormat,
    pub file_format_version: FileFormatVersion,
    #[serde(rename = "fileURI")]
    pub file_uri: FileUri2,
    pub file_part: FilePart,
    pub revision: Revision3,
    pub date: Date6,
    pub revision_info: RevisionInfo,
    pub remarks: Remarks18,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description15 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryType {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileFormat {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileFormatVersion {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUri2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePart {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revision3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date6 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionInfo {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks18 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometryLayer {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties21,
    pub additional_properties: bool,
    pub any_of: Vec<AnyOf4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties21 {
    #[serde(rename = "geometryID")]
    pub geometry_id: GeometryId2,
    pub description: Description16,
    pub top_elevation: TopElevation3,
    pub bottom_elevation: BottomElevation3,
    pub remarks: Remarks19,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryId2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description16 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopElevation3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BottomElevation3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks19 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyOf4 {
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometryPlane {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties22,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties22 {
    #[serde(rename = "geometryID")]
    pub geometry_id: GeometryId3,
    pub description: Description17,
    pub elevation: Elevation,
    pub remarks: Remarks20,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryId3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description17 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Elevation {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks20 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometryVolFromSurfaces {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties23,
    pub additional_properties: bool,
    pub any_of: Vec<AnyOf7>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties23 {
    #[serde(rename = "geometryID")]
    pub geometry_id: GeometryId4,
    pub description: Description18,
    pub agsi_geometry_top: AgsiGeometryTop,
    pub agsi_geometry_bottom: AgsiGeometryBottom,
    pub remarks: Remarks21,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryId4 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description18 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometryTop {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub any_of: Vec<AnyOf5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyOf5 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometryBottom {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub any_of: Vec<AnyOf6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyOf6 {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks21 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyOf7 {
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometryAreaFromLines {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties24,
    pub additional_properties: bool,
    pub any_of: Vec<AnyOf8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties24 {
    #[serde(rename = "geometryID")]
    pub geometry_id: GeometryId5,
    pub description: Description19,
    pub agsi_geometry_top: AgsiGeometryTop2,
    pub agsi_geometry_bottom: AgsiGeometryBottom2,
    pub remarks: Remarks22,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryId5 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description19 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometryTop2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiGeometryBottom2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks22 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyOf8 {
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyValue7 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties25,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties25 {
    #[serde(rename = "dataID")]
    pub data_id: DataId,
    #[serde(rename = "codeID")]
    pub code_id: CodeId2,
    #[serde(rename = "caseID")]
    pub case_id: CaseId,
    pub value_numeric: ValueNumeric,
    pub value_text: ValueText,
    #[serde(rename = "valueProfileIndVarCodeID")]
    pub value_profile_ind_var_code_id: ValueProfileIndVarCodeId,
    pub value_profile: ValueProfile2,
    pub remarks: Remarks23,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeId2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueNumeric {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueText {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueProfileIndVarCodeId {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueProfile2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
    pub example: Vec<Vec<f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks23 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertySummary2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties26,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties26 {
    #[serde(rename = "dataID")]
    pub data_id: DataId2,
    #[serde(rename = "codeID")]
    pub code_id: CodeId3,
    #[serde(rename = "caseID")]
    pub case_id: CaseId2,
    pub value_min: ValueMin,
    pub value_max: ValueMax,
    pub value_mean: ValueMean,
    pub value_std_dev: ValueStdDev,
    pub value_count: ValueCount,
    pub value_summary_text: ValueSummaryText,
    pub remarks: Remarks24,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataId2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeId3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseId2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueMin {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueMax {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueMean {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueStdDev {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueCount {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSummaryText {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks24 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyFromFile7 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties27,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties27 {
    #[serde(rename = "dataID")]
    pub data_id: DataId3,
    pub description: Description20,
    pub file_format: FileFormat2,
    pub file_format_version: FileFormatVersion2,
    #[serde(rename = "fileURI")]
    pub file_uri: FileUri3,
    pub file_part: FilePart2,
    pub revision: Revision4,
    pub date: Date7,
    pub revision_info: RevisionInfo2,
    pub remarks: Remarks25,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataId3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description20 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileFormat2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileFormatVersion2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUri3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePart2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revision4 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date7 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionInfo2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks25 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataParameterValue2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties28,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties28 {
    #[serde(rename = "dataID")]
    pub data_id: DataId4,
    #[serde(rename = "codeID")]
    pub code_id: CodeId4,
    #[serde(rename = "caseID")]
    pub case_id: CaseId3,
    pub value_numeric: ValueNumeric2,
    pub value_text: ValueText2,
    #[serde(rename = "valueProfileIndVarCodeID")]
    pub value_profile_ind_var_code_id: ValueProfileIndVarCodeId2,
    pub value_profile: ValueProfile3,
    pub remarks: Remarks26,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataId4 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeId4 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub min_length: i64,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseId3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueNumeric2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueText2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueProfileIndVarCodeId2 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueProfile3 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$ref")]
    pub ref_field: String,
    pub example: Vec<Vec<f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks26 {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: String,
}

