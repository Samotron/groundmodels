#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Association of Geotechnical & Geoenvironmental Specialists transfer format for ground model and interpreted data (AGSi)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://ags-data-format-wg.gitlab.io/agsi/agsi_standard/1.0.1/\","]
#[doc = "  \"title\": \"AGSi v1.0.1\","]
#[doc = "  \"description\": \"Association of Geotechnical & Geoenvironmental Specialists transfer format for ground model and interpreted data (AGSi)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agsFile\","]
#[doc = "    \"agsSchema\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsFile\": {"]
#[doc = "      \"$ref\": \"#/$defs/agsFile\""]
#[doc = "    },"]
#[doc = "    \"agsProject\": {"]
#[doc = "      \"$ref\": \"#/$defs/agsProject\""]
#[doc = "    },"]
#[doc = "    \"agsSchema\": {"]
#[doc = "      \"$ref\": \"#/$defs/agsSchema\""]
#[doc = "    },"]
#[doc = "    \"agsiModel\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiModel\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgSiV101 {
    #[serde(rename = "agsFile")]
    pub ags_file: AgsFile,
    #[serde(
        rename = "agsProject",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ags_project: ::std::option::Option<AgsProject>,
    #[serde(rename = "agsSchema")]
    pub ags_schema: AgsSchema,
    #[serde(
        rename = "agsiModel",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_model: ::std::vec::Vec<AgsiModel>,
}
impl ::std::convert::From<&AgSiV101> for AgSiV101 {
    fn from(value: &AgSiV101) -> Self {
        value.clone()
    }
}
impl AgSiV101 {
    pub fn builder() -> builder::AgSiV101 {
        Default::default()
    }
}
#[doc = "Metadata for the AGSi package (which comprises the AGSi file and included supporting files).  The AGSi package should be treated as a document in accordance with standards established for the project. The attributes provided align with good practice BIM, in accordance with ISO19650. It is recommended that, where possible, this object is output at the top of the file, after the schema object, for human readability."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Metadata for the AGSi package (which comprises the AGSi file and included supporting files).  The AGSi package should be treated as a document in accordance with standards established for the project. The attributes provided align with good practice BIM, in accordance with ISO19650. It is recommended that, where possible, this object is output at the top of the file, after the schema object, for human readability.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"producedBy\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"approvedBy\": {"]
#[doc = "      \"description\": \"Person(s) identified as approver.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"T Black\""]
#[doc = "    },"]
#[doc = "    \"checkedBy\": {"]
#[doc = "      \"description\": \"Person(s) identified as checker.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"P Brown\""]
#[doc = "    },"]
#[doc = "    \"date\": {"]
#[doc = "      \"description\": \"Date of production.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date\","]
#[doc = "      \"example\": \"2018-10-05\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Additional description, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Geological model and geotechnical design models produced for Stage 4 design\""]
#[doc = "    },"]
#[doc = "    \"fileURI\": {"]
#[doc = "      \"description\": \"URI (link address) for the location of this AGSi package within the project document system. Spaces are not permitted in URI strings. Refer to 1.6.6. URI for how to handle spaces in file paths or names.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"example\": \"https://gothammetro.sharepoint.com/C999/docs/C999-ABC-AX-XX-M3-CG-1234\""]
#[doc = "    },"]
#[doc = "    \"fileUUID\": {"]
#[doc = "      \"description\": \"Universal unique identifier (UUID) for the AGSi package. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"98e17952-c99d-4d87-8f01-8ba75d29b6ad\""]
#[doc = "    },"]
#[doc = "    \"madeBy\": {"]
#[doc = "      \"description\": \"Person(s) identified as originator.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"A Green\""]
#[doc = "    },"]
#[doc = "    \"producedBy\": {"]
#[doc = "      \"description\": \"Organisation that produced this AGSi package.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"ABC Consultants\""]
#[doc = "    },"]
#[doc = "    \"projectTitle\": {"]
#[doc = "      \"description\": \"Name of project (as used for document management system).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Gotham City Metro Purple Line, C999 Geotechnical Package X\""]
#[doc = "    },"]
#[doc = "    \"reference\": {"]
#[doc = "      \"description\": \"Document reference (typically in accordance with ISO19650, BS1192 or project standards).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"C999-ABC-AX-XX-M3-CG-1234\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    },"]
#[doc = "    \"revision\": {"]
#[doc = "      \"description\": \"Revision reference (typically in accordance with ISO19650 or BS1192 or project standards).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"P1\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Status, typically following recommendations of BS8574.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Final\""]
#[doc = "    },"]
#[doc = "    \"statusCode\": {"]
#[doc = "      \"description\": \"Status code in accordance with ISO19650 (or BS1192 suitability code).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"S2\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"Title of the AGSi package (as used for document management system).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"Stage 3 Sitewide Ground models\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsFile {
    #[doc = "Person(s) identified as approver."]
    #[serde(
        rename = "approvedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub approved_by: ::std::option::Option<::std::string::String>,
    #[doc = "Person(s) identified as checker."]
    #[serde(
        rename = "checkedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub checked_by: ::std::option::Option<::std::string::String>,
    #[doc = "Date of production."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::chrono::naive::NaiveDate>,
    #[doc = "Additional description, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "URI (link address) for the location of this AGSi package within the project document system. Spaces are not permitted in URI strings. Refer to 1.6.6. URI for how to handle spaces in file paths or names."]
    #[serde(
        rename = "fileURI",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_uri: ::std::option::Option<::std::string::String>,
    #[doc = "Universal unique identifier (UUID) for the AGSi package. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity. "]
    #[serde(
        rename = "fileUUID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_uuid: ::std::option::Option<::std::string::String>,
    #[doc = "Person(s) identified as originator."]
    #[serde(
        rename = "madeBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub made_by: ::std::option::Option<::std::string::String>,
    #[doc = "Organisation that produced this AGSi package."]
    #[serde(rename = "producedBy")]
    pub produced_by: AgsFileProducedBy,
    #[doc = "Name of project (as used for document management system)."]
    #[serde(
        rename = "projectTitle",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub project_title: ::std::option::Option<::std::string::String>,
    #[doc = "Document reference (typically in accordance with ISO19650, BS1192 or project standards)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reference: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[doc = "Revision reference (typically in accordance with ISO19650 or BS1192 or project standards)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub revision: ::std::option::Option<::std::string::String>,
    #[doc = "Status, typically following recommendations of BS8574."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    #[doc = "Status code in accordance with ISO19650 (or BS1192 suitability code)."]
    #[serde(
        rename = "statusCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_code: ::std::option::Option<::std::string::String>,
    #[doc = "Title of the AGSi package (as used for document management system)."]
    pub title: AgsFileTitle,
}
impl ::std::convert::From<&AgsFile> for AgsFile {
    fn from(value: &AgsFile) -> Self {
        value.clone()
    }
}
impl AgsFile {
    pub fn builder() -> builder::AgsFile {
        Default::default()
    }
}
#[doc = "Organisation that produced this AGSi package."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Organisation that produced this AGSi package.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"ABC Consultants\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsFileProducedBy(::std::string::String);
impl ::std::ops::Deref for AgsFileProducedBy {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsFileProducedBy> for ::std::string::String {
    fn from(value: AgsFileProducedBy) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsFileProducedBy> for AgsFileProducedBy {
    fn from(value: &AgsFileProducedBy) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsFileProducedBy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsFileProducedBy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsFileProducedBy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsFileProducedBy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsFileProducedBy {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Title of the AGSi package (as used for document management system)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Title of the AGSi package (as used for document management system).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"Stage 3 Sitewide Ground models\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsFileTitle(::std::string::String);
impl ::std::ops::Deref for AgsFileTitle {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsFileTitle> for ::std::string::String {
    fn from(value: AgsFileTitle) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsFileTitle> for AgsFileTitle {
    fn from(value: &AgsFileTitle) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsFileTitle {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsFileTitle {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsFileTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsFileTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsFileTitle {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Metadata for the specific project/commission (the Project) under which this AGSi package has been delivered. There can be only one project per AGSi file. The parent project, including the ultimate parent project, may be identified using the relevant attributes. "]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Metadata for the specific project/commission (the Project) under which this AGSi package has been delivered. There can be only one project per AGSi file. The parent project, including the ultimate parent project, may be identified using the relevant attributes. \","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"projectName\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsProjectCodeSet\": {"]
#[doc = "      \"description\": \"Array of embedded agsProjectCodeSet object(s)\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsProjectCodeSet\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"agsProjectCoordinateSystem\": {"]
#[doc = "      \"description\": \"Array of embedded agsProjectCoordinateSystem object(s)\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsProjectCoordinateSystem\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"agsProjectDocumentSet\": {"]
#[doc = "      \"description\": \"Array of embedded agsProjectDocumentSet object(s)\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsProjectDocumentSet\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"agsProjectInvestigation\": {"]
#[doc = "      \"description\": \"Array of embedded agsProjectInvestigation object(s)\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsProjectInvestigation\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"briefDocumentSetID\": {"]
#[doc = "      \"description\": \"Reference to the brief and/or specification for the project, details of which should be provided by way of an agsProjectDocumentSet object.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"ExampleDocSetID\""]
#[doc = "    },"]
#[doc = "    \"client\": {"]
#[doc = "      \"description\": \"Client for the Project.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"XYZ D&B Contractor\""]
#[doc = "    },"]
#[doc = "    \"clientProjectID\": {"]
#[doc = "      \"description\": \"Identifier for this Project used by the client. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"C999/ABC\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Brief project description.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Stage 3 sitewide ground modelling, including incorporation of new 2018 GI data.\""]
#[doc = "    },"]
#[doc = "    \"parentProjectName\": {"]
#[doc = "      \"description\": \"If applicable, the parent project/commission under which the Project has been procured, or which the Project reports to.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"C999 Area A Phase 1 Design and Build\""]
#[doc = "    },"]
#[doc = "    \"producer\": {"]
#[doc = "      \"description\": \"Organisation employed by the client responsible for the Project.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"ABC Consultants\""]
#[doc = "    },"]
#[doc = "    \"producerProjectID\": {"]
#[doc = "      \"description\": \"Identifier for this Project used by the producer of this file. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"P12345\""]
#[doc = "    },"]
#[doc = "    \"producerSuppliers\": {"]
#[doc = "      \"description\": \"If applicable, subconsultant(s) or subcontractor(s) employed  on the Project. Typically only include suppliers with direct involvement in producing the data included in this file. Input required as a text string not an array.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Acme Environmental, AN Other Organisation\""]
#[doc = "    },"]
#[doc = "    \"projectCountry\": {"]
#[doc = "      \"description\": \"Normally the country in which the ultimate project is taking place.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"United Kingdom\""]
#[doc = "    },"]
#[doc = "    \"projectName\": {"]
#[doc = "      \"description\": \"Name of the specific project/commission for the Project.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"C999 Geotechnical Package X\""]
#[doc = "    },"]
#[doc = "    \"projectUUID\": {"]
#[doc = "      \"description\": \"Universal/global unique identifier (UUID) for the project. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity. Other attributes should be used for IDs specific to the producer and/or client (see below).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"f7884d64-77ae-4eaf-b223-13c21bc2504b\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    },"]
#[doc = "    \"reportDocumentSetID\": {"]
#[doc = "      \"description\": \"Reference to report(s) and other documentation produced as part of this project and identified as supporting files. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"ExampleDocSetID\""]
#[doc = "    },"]
#[doc = "    \"ultimateProjectClient\": {"]
#[doc = "      \"description\": \"Client for the top level parent project.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"City Transport Authority\""]
#[doc = "    },"]
#[doc = "    \"ultimateProjectName\": {"]
#[doc = "      \"description\": \"If applicable, the top level parent project that the Project is ultimately for. Typically the works that are to be constructed, or a framework.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Gotham City Metro Purple Line\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsProject {
    #[doc = "Array of embedded agsProjectCodeSet object(s)"]
    #[serde(
        rename = "agsProjectCodeSet",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub ags_project_code_set: ::std::vec::Vec<AgsProjectCodeSet>,
    #[doc = "Array of embedded agsProjectCoordinateSystem object(s)"]
    #[serde(
        rename = "agsProjectCoordinateSystem",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub ags_project_coordinate_system: ::std::vec::Vec<AgsProjectCoordinateSystem>,
    #[doc = "Array of embedded agsProjectDocumentSet object(s)"]
    #[serde(
        rename = "agsProjectDocumentSet",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub ags_project_document_set: ::std::vec::Vec<AgsProjectDocumentSet>,
    #[doc = "Array of embedded agsProjectInvestigation object(s)"]
    #[serde(
        rename = "agsProjectInvestigation",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub ags_project_investigation: ::std::vec::Vec<AgsProjectInvestigation>,
    #[doc = "Reference to the brief and/or specification for the project, details of which should be provided by way of an agsProjectDocumentSet object."]
    #[serde(
        rename = "briefDocumentSetID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub brief_document_set_id: ::std::option::Option<::std::string::String>,
    #[doc = "Client for the Project."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub client: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for this Project used by the client. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity. "]
    #[serde(
        rename = "clientProjectID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_project_id: ::std::option::Option<::std::string::String>,
    #[doc = "Brief project description."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "If applicable, the parent project/commission under which the Project has been procured, or which the Project reports to."]
    #[serde(
        rename = "parentProjectName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub parent_project_name: ::std::option::Option<::std::string::String>,
    #[doc = "Organisation employed by the client responsible for the Project."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub producer: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for this Project used by the producer of this file. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity. "]
    #[serde(
        rename = "producerProjectID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub producer_project_id: ::std::option::Option<::std::string::String>,
    #[doc = "If applicable, subconsultant(s) or subcontractor(s) employed  on the Project. Typically only include suppliers with direct involvement in producing the data included in this file. Input required as a text string not an array."]
    #[serde(
        rename = "producerSuppliers",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub producer_suppliers: ::std::option::Option<::std::string::String>,
    #[doc = "Normally the country in which the ultimate project is taking place."]
    #[serde(
        rename = "projectCountry",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub project_country: ::std::option::Option<::std::string::String>,
    #[doc = "Name of the specific project/commission for the Project."]
    #[serde(rename = "projectName")]
    pub project_name: AgsProjectProjectName,
    #[doc = "Universal/global unique identifier (UUID) for the project. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity. Other attributes should be used for IDs specific to the producer and/or client (see below)."]
    #[serde(
        rename = "projectUUID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub project_uuid: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[doc = "Reference to report(s) and other documentation produced as part of this project and identified as supporting files. "]
    #[serde(
        rename = "reportDocumentSetID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub report_document_set_id: ::std::option::Option<::std::string::String>,
    #[doc = "Client for the top level parent project."]
    #[serde(
        rename = "ultimateProjectClient",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ultimate_project_client: ::std::option::Option<::std::string::String>,
    #[doc = "If applicable, the top level parent project that the Project is ultimately for. Typically the works that are to be constructed, or a framework."]
    #[serde(
        rename = "ultimateProjectName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ultimate_project_name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsProject> for AgsProject {
    fn from(value: &AgsProject) -> Self {
        value.clone()
    }
}
impl AgsProject {
    pub fn builder() -> builder::AgsProject {
        Default::default()
    }
}
#[doc = "Codes referenced by other parts of the schema such as the Data group objects (property and parameter codes) and Observation group objects (hole types, legend codes and geology codes). The codes may be project specific or from a standard list, e.g. AGSi standard code list or ABBR codes inherited from AGS factual data. Inclusion of standard AGSi code list or AGS ABBR codes used is optional (unless required by specification) provided that the code list used is identifed using agsProjectCodeSet. Refer to 3.2.4. Codes for Data objects and 3.2.5. Codes where use of AGS ABBR recommended for further details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Codes referenced by other parts of the schema such as the Data group objects (property and parameter codes) and Observation group objects (hole types, legend codes and geology codes). The codes may be project specific or from a standard list, e.g. AGSi standard code list or ABBR codes inherited from AGS factual data. Inclusion of standard AGSi code list or AGS ABBR codes used is optional (unless required by specification) provided that the code list used is identifed using agsProjectCodeSet. Refer to 3.2.4. Codes for Data objects and 3.2.5. Codes where use of AGS ABBR recommended for further details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"codeID\","]
#[doc = "    \"description\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"codeID\": {"]
#[doc = "      \"description\": \"Codes, including project specific codes, used in this AGSi file. All codes within each code set shall be unique. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"UndrainedShearStrength\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Short description of what the code represents.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"Undrained shear strength\""]
#[doc = "    },"]
#[doc = "    \"isStandard\": {"]
#[doc = "      \"description\": \"true if code is from standard dictionary such as the AGSi code list. If omitted, should be assumed to be false, i.e. project specific or other non-standard code.\","]
#[doc = "      \"type\": \"boolean\","]
#[doc = "      \"example\": true"]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some additional remarks\""]
#[doc = "    },"]
#[doc = "    \"units\": {"]
#[doc = "      \"description\": \"Units of measurement if code represents a property or parameter. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"kPa\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsProjectCode {
    #[doc = "Codes, including project specific codes, used in this AGSi file. All codes within each code set shall be unique. "]
    #[serde(rename = "codeID")]
    pub code_id: AgsProjectCodeCodeId,
    #[doc = "Short description of what the code represents."]
    pub description: AgsProjectCodeDescription,
    #[doc = "true if code is from standard dictionary such as the AGSi code list. If omitted, should be assumed to be false, i.e. project specific or other non-standard code."]
    #[serde(
        rename = "isStandard",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_standard: ::std::option::Option<bool>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[doc = "Units of measurement if code represents a property or parameter. "]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsProjectCode> for AgsProjectCode {
    fn from(value: &AgsProjectCode) -> Self {
        value.clone()
    }
}
impl AgsProjectCode {
    pub fn builder() -> builder::AgsProjectCode {
        Default::default()
    }
}
#[doc = "Codes, including project specific codes, used in this AGSi file. All codes within each code set shall be unique. "]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Codes, including project specific codes, used in this AGSi file. All codes within each code set shall be unique. \","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"UndrainedShearStrength\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsProjectCodeCodeId(::std::string::String);
impl ::std::ops::Deref for AgsProjectCodeCodeId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsProjectCodeCodeId> for ::std::string::String {
    fn from(value: AgsProjectCodeCodeId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsProjectCodeCodeId> for AgsProjectCodeCodeId {
    fn from(value: &AgsProjectCodeCodeId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsProjectCodeCodeId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsProjectCodeCodeId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsProjectCodeCodeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsProjectCodeCodeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsProjectCodeCodeId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Short description of what the code represents."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Short description of what the code represents.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"Undrained shear strength\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsProjectCodeDescription(::std::string::String);
impl ::std::ops::Deref for AgsProjectCodeDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsProjectCodeDescription> for ::std::string::String {
    fn from(value: AgsProjectCodeDescription) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsProjectCodeDescription> for AgsProjectCodeDescription {
    fn from(value: &AgsProjectCodeDescription) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsProjectCodeDescription {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsProjectCodeDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsProjectCodeDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsProjectCodeDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsProjectCodeDescription {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Sets of codes referenced by other parts of the schema such as the Data group objects (property and parameter codes) and Observation group objects (hole types, legend codes and geology codes). The codes may be project specific or from a standard list, e.g. AGSi standard code list or ABBR codes inherited from AGS factual data. An agsProjectCodeSet object is required for each object attribute using a set of codes. The codes for each code set are defined using embedded agsProjectCode objects or found at the external source specified in this object. Refer to 3.2.4. Codes for Data objects and 3.2.5. Codes where use of AGS ABBR recommended for further details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sets of codes referenced by other parts of the schema such as the Data group objects (property and parameter codes) and Observation group objects (hole types, legend codes and geology codes). The codes may be project specific or from a standard list, e.g. AGSi standard code list or ABBR codes inherited from AGS factual data. An agsProjectCodeSet object is required for each object attribute using a set of codes. The codes for each code set are defined using embedded agsProjectCode objects or found at the external source specified in this object. Refer to 3.2.4. Codes for Data objects and 3.2.5. Codes where use of AGS ABBR recommended for further details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"usedByAttribute\","]
#[doc = "    \"usedByObject\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsProjectCode\": {"]
#[doc = "      \"description\": \"Array of embedded agsProjectCode object(s).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsProjectCode\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"codeSetID\": {"]
#[doc = "      \"description\": \"Identifier for this code set. May be local to this file or a UUID as required/specified.  This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity.  If used, identifiers for codeSetID should be unique within the AGSi file. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"CodeSetParameter\""]
#[doc = "    },"]
#[doc = "    \"concatenationAllow\": {"]
#[doc = "      \"description\": \"true if concatenation of any combination of codes in the list is permitted, e.g. composite exploratory hole types when using AGS ABBR codes. Assume false (not permitted) if attribute omitted.\","]
#[doc = "      \"type\": \"boolean\","]
#[doc = "      \"example\": false"]
#[doc = "    },"]
#[doc = "    \"concatenationCharacter\": {"]
#[doc = "      \"description\": \"Linking character(s) used for concatenation, if permitted. Input blank string if none. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"+\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Name or short description of the code set.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Parameter codes\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some additional remarks\""]
#[doc = "    },"]
#[doc = "    \"sourceDescription\": {"]
#[doc = "      \"description\": \"Description of the source of the list of codes to be used for this set, if applicable. This could be a published source, a project reference or a file provided within the AGSi package. For properties or parameters, use of the  AGSi code list is recommended, but it can be changed to an alternate list, e.g. lists published by other agencies (UK or overseas) or major projects/clientsOptional if the codes are provided as agsProjectCode objects.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"AGSi standard code list\""]
#[doc = "    },"]
#[doc = "    \"sourceURI\": {"]
#[doc = "      \"description\": \"URI-reference link to source of list of codes to be used for this set, if applicable. This could be a published source, link to a project reference, or a file provided within the AGSi package. For properties or parameters, use of the AGSi code list is recommended, but it can be changed to an alternate list, e.g. lists published by other agencies (UK or overseas) or major projects/clients. Optional if the codes are provided as agsProjectCode objects. Spaces are not permitted in URI-reference strings. Refer to 1.6.6. URI for how to handle spaces in file paths or names.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-reference\","]
#[doc = "      \"example\": \"https://ags-data-format-wg.gitlab.io/agsi/agsi_standard/1.0.1/Codes_Codelist/\""]
#[doc = "    },"]
#[doc = "    \"usedByAttribute\": {"]
#[doc = "      \"description\": \"Name of the attribute of the AGSi object that references this code set.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"codeID\""]
#[doc = "    },"]
#[doc = "    \"usedByObject\": {"]
#[doc = "      \"description\": \"Name of the AGSi object that references this code set.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"agsiDataParameterValue\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsProjectCodeSet {
    #[doc = "Array of embedded agsProjectCode object(s)."]
    #[serde(
        rename = "agsProjectCode",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub ags_project_code: ::std::vec::Vec<AgsProjectCode>,
    #[doc = "Identifier for this code set. May be local to this file or a UUID as required/specified.  This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity.  If used, identifiers for codeSetID should be unique within the AGSi file. "]
    #[serde(
        rename = "codeSetID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub code_set_id: ::std::option::Option<::std::string::String>,
    #[doc = "true if concatenation of any combination of codes in the list is permitted, e.g. composite exploratory hole types when using AGS ABBR codes. Assume false (not permitted) if attribute omitted."]
    #[serde(
        rename = "concatenationAllow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub concatenation_allow: ::std::option::Option<bool>,
    #[doc = "Linking character(s) used for concatenation, if permitted. Input blank string if none. "]
    #[serde(
        rename = "concatenationCharacter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub concatenation_character: ::std::option::Option<::std::string::String>,
    #[doc = "Name or short description of the code set."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[doc = "Description of the source of the list of codes to be used for this set, if applicable. This could be a published source, a project reference or a file provided within the AGSi package. For properties or parameters, use of the  AGSi code list is recommended, but it can be changed to an alternate list, e.g. lists published by other agencies (UK or overseas) or major projects/clientsOptional if the codes are provided as agsProjectCode objects."]
    #[serde(
        rename = "sourceDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub source_description: ::std::option::Option<::std::string::String>,
    #[doc = "URI-reference link to source of list of codes to be used for this set, if applicable. This could be a published source, link to a project reference, or a file provided within the AGSi package. For properties or parameters, use of the AGSi code list is recommended, but it can be changed to an alternate list, e.g. lists published by other agencies (UK or overseas) or major projects/clients. Optional if the codes are provided as agsProjectCode objects. Spaces are not permitted in URI-reference strings. Refer to 1.6.6. URI for how to handle spaces in file paths or names."]
    #[serde(
        rename = "sourceURI",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub source_uri: ::std::option::Option<::std::string::String>,
    #[doc = "Name of the attribute of the AGSi object that references this code set."]
    #[serde(rename = "usedByAttribute")]
    pub used_by_attribute: AgsProjectCodeSetUsedByAttribute,
    #[doc = "Name of the AGSi object that references this code set."]
    #[serde(rename = "usedByObject")]
    pub used_by_object: AgsProjectCodeSetUsedByObject,
}
impl ::std::convert::From<&AgsProjectCodeSet> for AgsProjectCodeSet {
    fn from(value: &AgsProjectCodeSet) -> Self {
        value.clone()
    }
}
impl AgsProjectCodeSet {
    pub fn builder() -> builder::AgsProjectCodeSet {
        Default::default()
    }
}
#[doc = "Name of the attribute of the AGSi object that references this code set."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Name of the attribute of the AGSi object that references this code set.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"codeID\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsProjectCodeSetUsedByAttribute(::std::string::String);
impl ::std::ops::Deref for AgsProjectCodeSetUsedByAttribute {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsProjectCodeSetUsedByAttribute> for ::std::string::String {
    fn from(value: AgsProjectCodeSetUsedByAttribute) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsProjectCodeSetUsedByAttribute> for AgsProjectCodeSetUsedByAttribute {
    fn from(value: &AgsProjectCodeSetUsedByAttribute) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsProjectCodeSetUsedByAttribute {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsProjectCodeSetUsedByAttribute {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsProjectCodeSetUsedByAttribute {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsProjectCodeSetUsedByAttribute {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsProjectCodeSetUsedByAttribute {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Name of the AGSi object that references this code set."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Name of the AGSi object that references this code set.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"agsiDataParameterValue\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsProjectCodeSetUsedByObject(::std::string::String);
impl ::std::ops::Deref for AgsProjectCodeSetUsedByObject {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsProjectCodeSetUsedByObject> for ::std::string::String {
    fn from(value: AgsProjectCodeSetUsedByObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsProjectCodeSetUsedByObject> for AgsProjectCodeSetUsedByObject {
    fn from(value: &AgsProjectCodeSetUsedByObject) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsProjectCodeSetUsedByObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsProjectCodeSetUsedByObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsProjectCodeSetUsedByObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsProjectCodeSetUsedByObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsProjectCodeSetUsedByObject {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Defines the spatial coordinate system(s) used by the models. The coordinate system(s) used by the model is considered to be the model coordinate system, although this could be an established regional or national system. A secondary global coordinate system, which will normally be an established regional or national system, may also be defined (for each model system) but this will only exist via transformation from the model coordinate system. Refer to 3.2.2. Coordinate systems for further details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines the spatial coordinate system(s) used by the models. The coordinate system(s) used by the model is considered to be the model coordinate system, although this could be an established regional or national system. A secondary global coordinate system, which will normally be an established regional or national system, may also be defined (for each model system) but this will only exist via transformation from the model coordinate system. Refer to 3.2.2. Coordinate systems for further details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"axisUnitsXY\","]
#[doc = "    \"axisUnitsZ\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"axisNameX\": {"]
#[doc = "      \"description\": \"Axis name for X axis of model coordinate system.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Easting\""]
#[doc = "    },"]
#[doc = "    \"axisNameY\": {"]
#[doc = "      \"description\": \"Axis name for Y axis of model coordinate system.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Northing\""]
#[doc = "    },"]
#[doc = "    \"axisNameZ\": {"]
#[doc = "      \"description\": \"Axis name for Z axis of model coordinate system.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Elevation\""]
#[doc = "    },"]
#[doc = "    \"axisUnitsXY\": {"]
#[doc = "      \"description\": \"Units for X and Y axis (or X axis only if no Y axis). Units are considered to be case sensitive.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"m\""]
#[doc = "    },"]
#[doc = "    \"axisUnitsZ\": {"]
#[doc = "      \"description\": \"Units for Z axis (elevation). May include optional prefix and/or suffix as commonly used to identify the datum used. Considered to be case sensitive.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"mOD\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Name or short description for this coordinate system.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"3D model coordinate system: Gotham Metro Grid + OS elevations\""]
#[doc = "    },"]
#[doc = "    \"globalXYSystem\": {"]
#[doc = "      \"description\": \"Recognised national or regional horizontal coordinate system that the model coordinate system can be mapped to. This is intended to facilitate coordination with data sets in alternative systems and, in particular, encourage legacy use from archive. Transformation information provided in relevant attributes.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"British National Grid\""]
#[doc = "    },"]
#[doc = "    \"globalZSystem\": {"]
#[doc = "      \"description\": \"Recognised national or regional vertical coordinate system that the model coordinate system can be mapped to. This is intended to facilitate coordination with data sets in alternative systems and, in particular, encourage legacy use from archive. Transformation information provided in relevant attributes.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Ordnance Datum Newlyn\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    },"]
#[doc = "    \"systemID\": {"]
#[doc = "      \"description\": \"Identifier for this coordinate system.  May be local to this file or a UUID as required/specified. May be referenced by agsiModel. Identifiers for systemID shall be unique within an AGSi file. Optional if only one system is being used. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"MetroXYZ\""]
#[doc = "    },"]
#[doc = "    \"systemNameXY\": {"]
#[doc = "      \"description\": \"Name/description of horizontal coordinate (XY) reference system used for  model coordinate system.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Gotham Metro Grid\""]
#[doc = "    },"]
#[doc = "    \"systemNameZ\": {"]
#[doc = "      \"description\": \"Name/description of vertical coordinate (Z) reference system used for  model coordinate system.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Ordnance Datum Newlyn\""]
#[doc = "    },"]
#[doc = "    \"systemType\": {"]
#[doc = "      \"description\": \"Type of system. Only cartesian systems fully supported at present: XYZ (3D), XZ (2D vertical section), XY (2D map), Z (elevation only, i.e. simple layer profiles). For other types of system input other and describe in description.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"XYZ\","]
#[doc = "        \"XZ\","]
#[doc = "        \"XY\","]
#[doc = "        \"Z\","]
#[doc = "        \"other\""]
#[doc = "      ],"]
#[doc = "      \"example\": \"XYZ\""]
#[doc = "    },"]
#[doc = "    \"transformShiftX\": {"]
#[doc = "      \"description\": \"Shift in X (or Easting) direction of origin of model coordinate system relative to global coordinate system, i.e. X value of the model origin in the global system.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 450000"]
#[doc = "    },"]
#[doc = "    \"transformShiftY\": {"]
#[doc = "      \"description\": \"Shift in Y (or Northing) direction of origin of model coordinate system relative to global coordinate system, i.e. Y value of the model origin in the global system.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 125000"]
#[doc = "    },"]
#[doc = "    \"transformShiftZ\": {"]
#[doc = "      \"description\": \"Shift in Z (or Elevation) direction of origin of model coordinate system relative to global coordinate system, i.e. Z value of the model origin in the global system.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": -100"]
#[doc = "    },"]
#[doc = "    \"transformXYRotation\": {"]
#[doc = "      \"description\": \"Rotation in anticlockwise direction of model coordinate system XY axes relative to global coordinate system XY axes. Units of rotation are decimal degrees. \","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 1.44450116"]
#[doc = "    },"]
#[doc = "    \"transformXYScaleFactor\": {"]
#[doc = "      \"description\": \"Scale factor as ratio of distance in global coordinate system to model coordinate system, i.e. global distance divided by model distance.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 0.9999745653"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsProjectCoordinateSystem {
    #[doc = "Axis name for X axis of model coordinate system."]
    #[serde(
        rename = "axisNameX",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub axis_name_x: ::std::option::Option<::std::string::String>,
    #[doc = "Axis name for Y axis of model coordinate system."]
    #[serde(
        rename = "axisNameY",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub axis_name_y: ::std::option::Option<::std::string::String>,
    #[doc = "Axis name for Z axis of model coordinate system."]
    #[serde(
        rename = "axisNameZ",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub axis_name_z: ::std::option::Option<::std::string::String>,
    #[doc = "Units for X and Y axis (or X axis only if no Y axis). Units are considered to be case sensitive."]
    #[serde(rename = "axisUnitsXY")]
    pub axis_units_xy: AgsProjectCoordinateSystemAxisUnitsXy,
    #[doc = "Units for Z axis (elevation). May include optional prefix and/or suffix as commonly used to identify the datum used. Considered to be case sensitive."]
    #[serde(rename = "axisUnitsZ")]
    pub axis_units_z: AgsProjectCoordinateSystemAxisUnitsZ,
    #[doc = "Name or short description for this coordinate system."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Recognised national or regional horizontal coordinate system that the model coordinate system can be mapped to. This is intended to facilitate coordination with data sets in alternative systems and, in particular, encourage legacy use from archive. Transformation information provided in relevant attributes."]
    #[serde(
        rename = "globalXYSystem",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub global_xy_system: ::std::option::Option<::std::string::String>,
    #[doc = "Recognised national or regional vertical coordinate system that the model coordinate system can be mapped to. This is intended to facilitate coordination with data sets in alternative systems and, in particular, encourage legacy use from archive. Transformation information provided in relevant attributes."]
    #[serde(
        rename = "globalZSystem",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub global_z_system: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for this coordinate system.  May be local to this file or a UUID as required/specified. May be referenced by agsiModel. Identifiers for systemID shall be unique within an AGSi file. Optional if only one system is being used. "]
    #[serde(
        rename = "systemID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system_id: ::std::option::Option<::std::string::String>,
    #[doc = "Name/description of horizontal coordinate (XY) reference system used for  model coordinate system."]
    #[serde(
        rename = "systemNameXY",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system_name_xy: ::std::option::Option<::std::string::String>,
    #[doc = "Name/description of vertical coordinate (Z) reference system used for  model coordinate system."]
    #[serde(
        rename = "systemNameZ",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system_name_z: ::std::option::Option<::std::string::String>,
    #[doc = "Type of system. Only cartesian systems fully supported at present: XYZ (3D), XZ (2D vertical section), XY (2D map), Z (elevation only, i.e. simple layer profiles). For other types of system input other and describe in description."]
    #[serde(
        rename = "systemType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system_type: ::std::option::Option<AgsProjectCoordinateSystemSystemType>,
    #[serde(
        rename = "transformShiftX",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transform_shift_x: ::std::option::Option<f64>,
    #[serde(
        rename = "transformShiftY",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transform_shift_y: ::std::option::Option<f64>,
    #[serde(
        rename = "transformShiftZ",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transform_shift_z: ::std::option::Option<f64>,
    #[serde(
        rename = "transformXYRotation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transform_xy_rotation: ::std::option::Option<f64>,
    #[serde(
        rename = "transformXYScaleFactor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transform_xy_scale_factor: ::std::option::Option<f64>,
}
impl ::std::convert::From<&AgsProjectCoordinateSystem> for AgsProjectCoordinateSystem {
    fn from(value: &AgsProjectCoordinateSystem) -> Self {
        value.clone()
    }
}
impl AgsProjectCoordinateSystem {
    pub fn builder() -> builder::AgsProjectCoordinateSystem {
        Default::default()
    }
}
#[doc = "Units for X and Y axis (or X axis only if no Y axis). Units are considered to be case sensitive."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Units for X and Y axis (or X axis only if no Y axis). Units are considered to be case sensitive.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"m\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsProjectCoordinateSystemAxisUnitsXy(::std::string::String);
impl ::std::ops::Deref for AgsProjectCoordinateSystemAxisUnitsXy {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsProjectCoordinateSystemAxisUnitsXy> for ::std::string::String {
    fn from(value: AgsProjectCoordinateSystemAxisUnitsXy) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsProjectCoordinateSystemAxisUnitsXy>
    for AgsProjectCoordinateSystemAxisUnitsXy
{
    fn from(value: &AgsProjectCoordinateSystemAxisUnitsXy) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsProjectCoordinateSystemAxisUnitsXy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsProjectCoordinateSystemAxisUnitsXy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsProjectCoordinateSystemAxisUnitsXy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsProjectCoordinateSystemAxisUnitsXy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsProjectCoordinateSystemAxisUnitsXy {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Units for Z axis (elevation). May include optional prefix and/or suffix as commonly used to identify the datum used. Considered to be case sensitive."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Units for Z axis (elevation). May include optional prefix and/or suffix as commonly used to identify the datum used. Considered to be case sensitive.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"mOD\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsProjectCoordinateSystemAxisUnitsZ(::std::string::String);
impl ::std::ops::Deref for AgsProjectCoordinateSystemAxisUnitsZ {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsProjectCoordinateSystemAxisUnitsZ> for ::std::string::String {
    fn from(value: AgsProjectCoordinateSystemAxisUnitsZ) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsProjectCoordinateSystemAxisUnitsZ>
    for AgsProjectCoordinateSystemAxisUnitsZ
{
    fn from(value: &AgsProjectCoordinateSystemAxisUnitsZ) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsProjectCoordinateSystemAxisUnitsZ {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsProjectCoordinateSystemAxisUnitsZ {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsProjectCoordinateSystemAxisUnitsZ {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsProjectCoordinateSystemAxisUnitsZ {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsProjectCoordinateSystemAxisUnitsZ {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Type of system. Only cartesian systems fully supported at present: XYZ (3D), XZ (2D vertical section), XY (2D map), Z (elevation only, i.e. simple layer profiles). For other types of system input other and describe in description."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Type of system. Only cartesian systems fully supported at present: XYZ (3D), XZ (2D vertical section), XY (2D map), Z (elevation only, i.e. simple layer profiles). For other types of system input other and describe in description.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"XYZ\","]
#[doc = "    \"XZ\","]
#[doc = "    \"XY\","]
#[doc = "    \"Z\","]
#[doc = "    \"other\""]
#[doc = "  ],"]
#[doc = "  \"example\": \"XYZ\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AgsProjectCoordinateSystemSystemType {
    #[serde(rename = "XYZ")]
    Xyz,
    #[serde(rename = "XZ")]
    Xz,
    #[serde(rename = "XY")]
    Xy,
    Z,
    #[serde(rename = "other")]
    Other,
}
impl ::std::convert::From<&Self> for AgsProjectCoordinateSystemSystemType {
    fn from(value: &AgsProjectCoordinateSystemSystemType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AgsProjectCoordinateSystemSystemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Xyz => write!(f, "XYZ"),
            Self::Xz => write!(f, "XZ"),
            Self::Xy => write!(f, "XY"),
            Self::Z => write!(f, "Z"),
            Self::Other => write!(f, "other"),
        }
    }
}
impl ::std::str::FromStr for AgsProjectCoordinateSystemSystemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "XYZ" => Ok(Self::Xyz),
            "XZ" => Ok(Self::Xz),
            "XY" => Ok(Self::Xy),
            "Z" => Ok(Self::Z),
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AgsProjectCoordinateSystemSystemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsProjectCoordinateSystemSystemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsProjectCoordinateSystemSystemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Metadata for the individual documents or references contained within a document set. Refer to 3.2.3. Documents for further details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Metadata for the individual documents or references contained within a document set. Refer to 3.2.3. Documents for further details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"author\": {"]
#[doc = "      \"description\": \"The original author of the document.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Boring Drilling Ltd\""]
#[doc = "    },"]
#[doc = "    \"client\": {"]
#[doc = "      \"description\": \"The original commissioning client for the document.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"XYZ D&B Contractor\""]
#[doc = "    },"]
#[doc = "    \"date\": {"]
#[doc = "      \"description\": \"Date of the document (current revision).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date\","]
#[doc = "      \"example\": \"2018-09-06\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Further description if required. Typically what the document is commonly known as, given that the formal title may be verbose. Alternatively use for name/title given in project documentation system if this differs from original title.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Package A factual report, Volume 1\""]
#[doc = "    },"]
#[doc = "    \"documentID\": {"]
#[doc = "      \"description\": \"Identifier for the document. May be local to this file or a UUID as required/specified. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity. If used, identifiers for documentID should be unique at least within each document set (agsProjectDocumentSet object), and preferably unique within the AGSi file. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"eac20ae4-25a1-4e68-96f8-cf43b9761b11\""]
#[doc = "    },"]
#[doc = "    \"documentSystemURI\": {"]
#[doc = "      \"description\": \"URI (link address) for the location of the document within the project document management system. To be used if documentURI attribute used for relative link. Spaces are not permitted in URI strings. Refer to 1.6.6. URI for how to handle spaces in file paths or names.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"example\": \"https://gothammetro.sharepoint.com/C999/docs/C999-BDL-AX-XX-RP-WG-0002\""]
#[doc = "    },"]
#[doc = "    \"documentURI\": {"]
#[doc = "      \"description\": \"URI-reference (link address) for the document. This will be a relative link if document included within the AGSi package. For a public domain published reference, the link should be provided here. Spaces are not permitted in URI-reference strings. Refer to 1.6.6. URI for how to handle spaces in file paths or names.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-reference\","]
#[doc = "      \"example\": \"docs/GI/C999-BDL-AX-XX-RP-WG-0002.pdf\""]
#[doc = "    },"]
#[doc = "    \"originalReference\": {"]
#[doc = "      \"description\": \"Original reference shown on the document, if different from reference used by project document system (see systemReference).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"12345/GI/2\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some additional remarks\""]
#[doc = "    },"]
#[doc = "    \"revision\": {"]
#[doc = "      \"description\": \"Revision reference (typically in accordance with ISO19650 or BS1192).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"P2\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Status as indicated on or within the document, typically following recommendations of BS8574 if applicable.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Final\""]
#[doc = "    },"]
#[doc = "    \"statusCode\": {"]
#[doc = "      \"description\": \"Status code, typically in accordance with ISO19650, or BS1192 suitability code.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"S2\""]
#[doc = "    },"]
#[doc = "    \"systemReference\": {"]
#[doc = "      \"description\": \"Document reference used in the project document management system. May differ from original reference shown on document.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"C999-BDL-AX-XX-RP-WG-0002\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"Original title on the document.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Factual ground investigation report, Gotham City Metro Purple Line, C999 Package A, Volume 1 of 3\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsProjectDocument {
    #[doc = "The original author of the document."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub author: ::std::option::Option<::std::string::String>,
    #[doc = "The original commissioning client for the document."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub client: ::std::option::Option<::std::string::String>,
    #[doc = "Date of the document (current revision)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::chrono::naive::NaiveDate>,
    #[doc = "Further description if required. Typically what the document is commonly known as, given that the formal title may be verbose. Alternatively use for name/title given in project documentation system if this differs from original title."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for the document. May be local to this file or a UUID as required/specified. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity. If used, identifiers for documentID should be unique at least within each document set (agsProjectDocumentSet object), and preferably unique within the AGSi file. "]
    #[serde(
        rename = "documentID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub document_id: ::std::option::Option<::std::string::String>,
    #[doc = "URI (link address) for the location of the document within the project document management system. To be used if documentURI attribute used for relative link. Spaces are not permitted in URI strings. Refer to 1.6.6. URI for how to handle spaces in file paths or names."]
    #[serde(
        rename = "documentSystemURI",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub document_system_uri: ::std::option::Option<::std::string::String>,
    #[doc = "URI-reference (link address) for the document. This will be a relative link if document included within the AGSi package. For a public domain published reference, the link should be provided here. Spaces are not permitted in URI-reference strings. Refer to 1.6.6. URI for how to handle spaces in file paths or names."]
    #[serde(
        rename = "documentURI",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub document_uri: ::std::option::Option<::std::string::String>,
    #[doc = "Original reference shown on the document, if different from reference used by project document system (see systemReference)."]
    #[serde(
        rename = "originalReference",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub original_reference: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[doc = "Revision reference (typically in accordance with ISO19650 or BS1192)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub revision: ::std::option::Option<::std::string::String>,
    #[doc = "Status as indicated on or within the document, typically following recommendations of BS8574 if applicable."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    #[doc = "Status code, typically in accordance with ISO19650, or BS1192 suitability code."]
    #[serde(
        rename = "statusCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_code: ::std::option::Option<::std::string::String>,
    #[doc = "Document reference used in the project document management system. May differ from original reference shown on document."]
    #[serde(
        rename = "systemReference",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system_reference: ::std::option::Option<::std::string::String>,
    #[doc = "Original title on the document."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsProjectDocument> for AgsProjectDocument {
    fn from(value: &AgsProjectDocument) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AgsProjectDocument {
    fn default() -> Self {
        Self {
            author: Default::default(),
            client: Default::default(),
            date: Default::default(),
            description: Default::default(),
            document_id: Default::default(),
            document_system_uri: Default::default(),
            document_uri: Default::default(),
            original_reference: Default::default(),
            remarks: Default::default(),
            revision: Default::default(),
            status: Default::default(),
            status_code: Default::default(),
            system_reference: Default::default(),
            title: Default::default(),
        }
    }
}
impl AgsProjectDocument {
    pub fn builder() -> builder::AgsProjectDocument {
        Default::default()
    }
}
#[doc = "Container and metadata for a set of supporting documents or reference information, which may be referenced from other parts of the schema. This container must be used and referenced, even if there is only one document within it. Refer to 3.2.3. Documents for further details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Container and metadata for a set of supporting documents or reference information, which may be referenced from other parts of the schema. This container must be used and referenced, even if there is only one document within it. Refer to 3.2.3. Documents for further details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"documentSetID\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsProjectDocument\": {"]
#[doc = "      \"description\": \"Array of embedded agsProjectDocument object(s).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsProjectDocument\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Brief description, i.e. what this set of documents is commonly known as.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Package A factual report\""]
#[doc = "    },"]
#[doc = "    \"documentSetID\": {"]
#[doc = "      \"description\": \"Identifier for this document set. May be local to this file or a UUID as required/specified.  Identifiers for documentSetID shall be unique within an AGSi file. Referenced by other parts of the schema.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"ExampleDocSetID\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some additional remarks\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsProjectDocumentSet {
    #[doc = "Array of embedded agsProjectDocument object(s)."]
    #[serde(
        rename = "agsProjectDocument",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub ags_project_document: ::std::vec::Vec<AgsProjectDocument>,
    #[doc = "Brief description, i.e. what this set of documents is commonly known as."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for this document set. May be local to this file or a UUID as required/specified.  Identifiers for documentSetID shall be unique within an AGSi file. Referenced by other parts of the schema."]
    #[serde(rename = "documentSetID")]
    pub document_set_id: AgsProjectDocumentSetDocumentSetId,
    #[doc = "Additional remarks, if required"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsProjectDocumentSet> for AgsProjectDocumentSet {
    fn from(value: &AgsProjectDocumentSet) -> Self {
        value.clone()
    }
}
impl AgsProjectDocumentSet {
    pub fn builder() -> builder::AgsProjectDocumentSet {
        Default::default()
    }
}
#[doc = "Identifier for this document set. May be local to this file or a UUID as required/specified.  Identifiers for documentSetID shall be unique within an AGSi file. Referenced by other parts of the schema."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Identifier for this document set. May be local to this file or a UUID as required/specified.  Identifiers for documentSetID shall be unique within an AGSi file. Referenced by other parts of the schema.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"ExampleDocSetID\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsProjectDocumentSetDocumentSetId(::std::string::String);
impl ::std::ops::Deref for AgsProjectDocumentSetDocumentSetId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsProjectDocumentSetDocumentSetId> for ::std::string::String {
    fn from(value: AgsProjectDocumentSetDocumentSetId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsProjectDocumentSetDocumentSetId>
    for AgsProjectDocumentSetDocumentSetId
{
    fn from(value: &AgsProjectDocumentSetDocumentSetId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsProjectDocumentSetDocumentSetId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsProjectDocumentSetDocumentSetId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsProjectDocumentSetDocumentSetId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsProjectDocumentSetDocumentSetId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsProjectDocumentSetDocumentSetId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Basic metadata for investigations, generally ground investigations. Referenced from various parts of the schema. More detailed metadata may be provided using embedded agsiDataPropertyValue or agsiDataPropertyFromFile objects. Refer to 3.2.1. Projects and investigations for further details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Basic metadata for investigations, generally ground investigations. Referenced from various parts of the schema. More detailed metadata may be provided using embedded agsiDataPropertyValue or agsiDataPropertyFromFile objects. Refer to 3.2.1. Projects and investigations for further details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"investigationID\","]
#[doc = "    \"investigationName\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsiDataPropertyFromFile\": {"]
#[doc = "      \"description\": \"An embedded agsiDataPropertyFromFile object, which may be used to reference to an external supporting data file.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiDataPropertyFromFile\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"agsiDataPropertyValue\": {"]
#[doc = "      \"description\": \"Array of embedded agsiDataPropertyValue objects. Used to provide further metadata relating to the investigation, if required.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiDataPropertyValue\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"client\": {"]
#[doc = "      \"description\": \"Commissioning (contracting) client for the investigation.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"XYZ D&B Contractor\""]
#[doc = "    },"]
#[doc = "    \"contractor\": {"]
#[doc = "      \"description\": \"Contractor undertaking the investigation.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Boring Drilling Ltd\""]
#[doc = "    },"]
#[doc = "    \"dataDocumentSetID\": {"]
#[doc = "      \"description\": \"Reference to the data for the GI, typically the AGS (factual) data, details of which should be provided in an agsProjectDocumentSet object.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"ExampleDocSetID\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Further description of investigation, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Preliminary sitewide investigation, March-July 2018\""]
#[doc = "    },"]
#[doc = "    \"engineer\": {"]
#[doc = "      \"description\": \"Organisation acting as Engineer, Investigation Supervisor, Contract Administrator or equivalent. If technical and contractual roles are split, then include both.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"ABC Consultants\""]
#[doc = "    },"]
#[doc = "    \"fieldworkDateStart\": {"]
#[doc = "      \"description\": \"Date of start of fieldwork. Date in ISO 8601 format so it could be to nearest month (2019-05) or just the year if exact date not available.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date\","]
#[doc = "      \"example\": \"2018-08-21\""]
#[doc = "    },"]
#[doc = "    \"investigationID\": {"]
#[doc = "      \"description\": \"Identifier for this investigation. May be local to this file or a UUID as required/specified. Identifiers for investigationID shall be unique within an AGSi file. Referenced by other parts of the schema such as agsiObservationSet.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"GIPackageA\""]
#[doc = "    },"]
#[doc = "    \"investigationName\": {"]
#[doc = "      \"description\": \"Name of investigation.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"Gotham City Metro Purple Line, C999 Package A\""]
#[doc = "    },"]
#[doc = "    \"locationCoordinateGlobal\": {"]
#[doc = "      \"description\": \"Coordinates, in global coordinate system (national or regional system, as defined in agsProjectCoordinateSystem) of a point that represents the general location of the investigation, typically the middle of the site.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"$ref\": \"#/$defs/coordinateTuple\","]
#[doc = "      \"example\": ["]
#[doc = "        475270.0,"]
#[doc = "        137965.0"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"locationCoordinateProject\": {"]
#[doc = "      \"description\": \"Coordinates, in a relevant model coordinate system (if applicable), of a point that represents the general location of the investigation, typically the middle of the site. Relevant system will be usually be a 3D system that is used for all 3D models, or a 2D map system. Do not use this attribute if multiple different 3D model systems are in use.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"$ref\": \"#/$defs/coordinateTuple\","]
#[doc = "      \"example\": ["]
#[doc = "        25500.0,"]
#[doc = "        13200.0"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"locationDescription\": {"]
#[doc = "      \"description\": \"Brief description that locates the site. Could be a postal address.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Gotham City, west, central and southeast\""]
#[doc = "    },"]
#[doc = "    \"parentProjectName\": {"]
#[doc = "      \"description\": \"Name of the parent project that this investigation is for. If parent is ultimate parent project, then may be left blank.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"C999 Area A Phase 1 Design and Build\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some additional remarks\""]
#[doc = "    },"]
#[doc = "    \"reportDocumentSetID\": {"]
#[doc = "      \"description\": \"Reference to the reports relating to the investigation, details of which should be provided in an agsProjectDocumentSet object.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"ExampleDocSetID\""]
#[doc = "    },"]
#[doc = "    \"scopeDescription\": {"]
#[doc = "      \"description\": \"Brief description of scope.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Preliminary investigation comprising 27 boreholes of which 15 extended by rotary coring up to 55m depth, 45 CPT (max 15m), 35 trial pits, 26 dynamic sampling holes, geotechnical and contamination sampling and testing, piezometric monitoring, limited gas monitoring.\""]
#[doc = "    },"]
#[doc = "    \"specificationDocumentSetID\": {"]
#[doc = "      \"description\": \"Reference to the specification for the GI, details of which should be provided in an agsProjectDocumentSet object.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"ExampleDocSetID\""]
#[doc = "    },"]
#[doc = "    \"subcontractors\": {"]
#[doc = "      \"description\": \"List of significant subcontractors or suppliers working on the investigation. List as a text string, not an array.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Acme Specialist Lab, XYZ Environmental Lab\""]
#[doc = "    },"]
#[doc = "    \"ultimateProjectClient\": {"]
#[doc = "      \"description\": \"Client for the ultimate parent project.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"City Transport Authority\""]
#[doc = "    },"]
#[doc = "    \"ultimateProjectName\": {"]
#[doc = "      \"description\": \"Name of the ultimate parent project that this investigation is for.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Gotham City Metro Purple Line\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsProjectInvestigation {
    #[doc = "An embedded agsiDataPropertyFromFile object, which may be used to reference to an external supporting data file."]
    #[serde(
        rename = "agsiDataPropertyFromFile",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_data_property_from_file: ::std::vec::Vec<AgsiDataPropertyFromFile>,
    #[doc = "Array of embedded agsiDataPropertyValue objects. Used to provide further metadata relating to the investigation, if required."]
    #[serde(
        rename = "agsiDataPropertyValue",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_data_property_value: ::std::vec::Vec<AgsiDataPropertyValue>,
    #[doc = "Commissioning (contracting) client for the investigation."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub client: ::std::option::Option<::std::string::String>,
    #[doc = "Contractor undertaking the investigation."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub contractor: ::std::option::Option<::std::string::String>,
    #[doc = "Reference to the data for the GI, typically the AGS (factual) data, details of which should be provided in an agsProjectDocumentSet object."]
    #[serde(
        rename = "dataDocumentSetID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub data_document_set_id: ::std::option::Option<::std::string::String>,
    #[doc = "Further description of investigation, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Organisation acting as Engineer, Investigation Supervisor, Contract Administrator or equivalent. If technical and contractual roles are split, then include both."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub engineer: ::std::option::Option<::std::string::String>,
    #[doc = "Date of start of fieldwork. Date in ISO 8601 format so it could be to nearest month (2019-05) or just the year if exact date not available."]
    #[serde(
        rename = "fieldworkDateStart",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fieldwork_date_start: ::std::option::Option<::chrono::naive::NaiveDate>,
    #[doc = "Identifier for this investigation. May be local to this file or a UUID as required/specified. Identifiers for investigationID shall be unique within an AGSi file. Referenced by other parts of the schema such as agsiObservationSet."]
    #[serde(rename = "investigationID")]
    pub investigation_id: AgsProjectInvestigationInvestigationId,
    #[doc = "Name of investigation."]
    #[serde(rename = "investigationName")]
    pub investigation_name: AgsProjectInvestigationInvestigationName,
    #[doc = "Coordinates, in global coordinate system (national or regional system, as defined in agsProjectCoordinateSystem) of a point that represents the general location of the investigation, typically the middle of the site."]
    #[serde(
        rename = "locationCoordinateGlobal",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub location_coordinate_global: ::std::option::Option<CoordinateTuple>,
    #[doc = "Coordinates, in a relevant model coordinate system (if applicable), of a point that represents the general location of the investigation, typically the middle of the site. Relevant system will be usually be a 3D system that is used for all 3D models, or a 2D map system. Do not use this attribute if multiple different 3D model systems are in use."]
    #[serde(
        rename = "locationCoordinateProject",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub location_coordinate_project: ::std::option::Option<CoordinateTuple>,
    #[doc = "Brief description that locates the site. Could be a postal address."]
    #[serde(
        rename = "locationDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub location_description: ::std::option::Option<::std::string::String>,
    #[doc = "Name of the parent project that this investigation is for. If parent is ultimate parent project, then may be left blank."]
    #[serde(
        rename = "parentProjectName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub parent_project_name: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[doc = "Reference to the reports relating to the investigation, details of which should be provided in an agsProjectDocumentSet object."]
    #[serde(
        rename = "reportDocumentSetID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub report_document_set_id: ::std::option::Option<::std::string::String>,
    #[doc = "Brief description of scope."]
    #[serde(
        rename = "scopeDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scope_description: ::std::option::Option<::std::string::String>,
    #[doc = "Reference to the specification for the GI, details of which should be provided in an agsProjectDocumentSet object."]
    #[serde(
        rename = "specificationDocumentSetID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub specification_document_set_id: ::std::option::Option<::std::string::String>,
    #[doc = "List of significant subcontractors or suppliers working on the investigation. List as a text string, not an array."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subcontractors: ::std::option::Option<::std::string::String>,
    #[doc = "Client for the ultimate parent project."]
    #[serde(
        rename = "ultimateProjectClient",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ultimate_project_client: ::std::option::Option<::std::string::String>,
    #[doc = "Name of the ultimate parent project that this investigation is for."]
    #[serde(
        rename = "ultimateProjectName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ultimate_project_name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsProjectInvestigation> for AgsProjectInvestigation {
    fn from(value: &AgsProjectInvestigation) -> Self {
        value.clone()
    }
}
impl AgsProjectInvestigation {
    pub fn builder() -> builder::AgsProjectInvestigation {
        Default::default()
    }
}
#[doc = "Identifier for this investigation. May be local to this file or a UUID as required/specified. Identifiers for investigationID shall be unique within an AGSi file. Referenced by other parts of the schema such as agsiObservationSet."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Identifier for this investigation. May be local to this file or a UUID as required/specified. Identifiers for investigationID shall be unique within an AGSi file. Referenced by other parts of the schema such as agsiObservationSet.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"GIPackageA\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsProjectInvestigationInvestigationId(::std::string::String);
impl ::std::ops::Deref for AgsProjectInvestigationInvestigationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsProjectInvestigationInvestigationId> for ::std::string::String {
    fn from(value: AgsProjectInvestigationInvestigationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsProjectInvestigationInvestigationId>
    for AgsProjectInvestigationInvestigationId
{
    fn from(value: &AgsProjectInvestigationInvestigationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsProjectInvestigationInvestigationId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsProjectInvestigationInvestigationId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsProjectInvestigationInvestigationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsProjectInvestigationInvestigationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsProjectInvestigationInvestigationId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Name of investigation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Name of investigation.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"Gotham City Metro Purple Line, C999 Package A\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsProjectInvestigationInvestigationName(::std::string::String);
impl ::std::ops::Deref for AgsProjectInvestigationInvestigationName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsProjectInvestigationInvestigationName> for ::std::string::String {
    fn from(value: AgsProjectInvestigationInvestigationName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsProjectInvestigationInvestigationName>
    for AgsProjectInvestigationInvestigationName
{
    fn from(value: &AgsProjectInvestigationInvestigationName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsProjectInvestigationInvestigationName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsProjectInvestigationInvestigationName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsProjectInvestigationInvestigationName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsProjectInvestigationInvestigationName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsProjectInvestigationInvestigationName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Name of the specific project/commission for the Project."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Name of the specific project/commission for the Project.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"C999 Geotechnical Package X\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsProjectProjectName(::std::string::String);
impl ::std::ops::Deref for AgsProjectProjectName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsProjectProjectName> for ::std::string::String {
    fn from(value: AgsProjectProjectName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsProjectProjectName> for AgsProjectProjectName {
    fn from(value: &AgsProjectProjectName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsProjectProjectName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsProjectProjectName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsProjectProjectName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsProjectProjectName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsProjectProjectName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Defines the schema used for the AGSi file. It is recommended that, where possible, this object is output at the top of the file, for human readability."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines the schema used for the AGSi file. It is recommended that, where possible, this object is output at the top of the file, for human readability.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"link\": {"]
#[doc = "      \"description\": \"Web link (uri) to the AGS schema used herein.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"example\": \"https://ags-data-format-wg.gitlab.io/agsi/agsi_standard/1.0.1/\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Name of the AGS schema used herein.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"AGSi\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"description\": \"Version of the named AGS schema used herein.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"1.0.1\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsSchema {
    #[doc = "Web link (uri) to the AGS schema used herein."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub link: ::std::option::Option<::std::string::String>,
    #[doc = "Name of the AGS schema used herein."]
    pub name: AgsSchemaName,
    #[doc = "Version of the named AGS schema used herein."]
    pub version: AgsSchemaVersion,
}
impl ::std::convert::From<&AgsSchema> for AgsSchema {
    fn from(value: &AgsSchema) -> Self {
        value.clone()
    }
}
impl AgsSchema {
    pub fn builder() -> builder::AgsSchema {
        Default::default()
    }
}
#[doc = "Name of the AGS schema used herein."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Name of the AGS schema used herein.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"AGSi\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsSchemaName(::std::string::String);
impl ::std::ops::Deref for AgsSchemaName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsSchemaName> for ::std::string::String {
    fn from(value: AgsSchemaName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsSchemaName> for AgsSchemaName {
    fn from(value: &AgsSchemaName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsSchemaName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsSchemaName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsSchemaName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsSchemaName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsSchemaName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Version of the named AGS schema used herein."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Version of the named AGS schema used herein.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"1.0.1\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsSchemaVersion(::std::string::String);
impl ::std::ops::Deref for AgsSchemaVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsSchemaVersion> for ::std::string::String {
    fn from(value: AgsSchemaVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsSchemaVersion> for AgsSchemaVersion {
    fn from(value: &AgsSchemaVersion) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsSchemaVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsSchemaVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsSchemaVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsSchemaVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsSchemaVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Each agsiDataParameterValue object provides the data for a single defined parameter. The parameter value conveyed may be numeric, a profile of numeric values (e.g. a design line) or text. Refer to 7.2. Data rules and conventions for further details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Each agsiDataParameterValue object provides the data for a single defined parameter. The parameter value conveyed may be numeric, a profile of numeric values (e.g. a design line) or text. Refer to 7.2. Data rules and conventions for further details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"codeID\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"caseID\": {"]
#[doc = "      \"description\": \"Code (or text) that identifies the use case for a parameter. See  7.2.4. Use of (data) case for example use cases. If the input is a code, this shall be defined in an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. May be left blank or omitted, but the combination of codeID and caseID shall be unique for the instances of this object contained within a single parent object instance.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"EC7Pile\""]
#[doc = "    },"]
#[doc = "    \"codeID\": {"]
#[doc = "      \"description\": \"Code that identifies the parameter. Codes should be defined in either an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. The codes used by the instances of this object contained within a single parent object instance shall be unique, except that if caseID is used then only the combination of codeID and caseID needs to be unique.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"UndrainedShearStrength\""]
#[doc = "    },"]
#[doc = "    \"dataID\": {"]
#[doc = "      \"description\": \"Identifier for this data object. May be local to this file but all identifiers used within the Data group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"42f18976-7352-4f67-9a6e-df89788343a7\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    },"]
#[doc = "    \"valueNumeric\": {"]
#[doc = "      \"description\": \"Numeric value of parameter, if applicable.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 75"]
#[doc = "    },"]
#[doc = "    \"valueProfile\": {"]
#[doc = "      \"description\": \"The profile of values as an ordered list of tuples of [independent variable value, parameter value]. Typically used to represent design lines. Refer to 1.6.9. Profiles or arrays of coordinate tuples for further information.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"$ref\": \"#/$defs/valueProfile\","]
#[doc = "      \"example\": ["]
#[doc = "        ["]
#[doc = "          6.0,"]
#[doc = "          100.0"]
#[doc = "        ],"]
#[doc = "        ["]
#[doc = "          -24.0,"]
#[doc = "          280.0"]
#[doc = "        ]"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"valueProfileIndVarCodeID\": {"]
#[doc = "      \"description\": \"Code that identifies the independent variable for a profile, i.e. what the property value varies against. The code shall be defined in an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Elevation\""]
#[doc = "    },"]
#[doc = "    \"valueText\": {"]
#[doc = "      \"description\": \"Text based value of parameter, if applicable. For a profile (see below), this could be used for a concise description or representation of the profile. Unless specified otherwise, this attribute should only be used when the value is not numeric, i.e. valueNumeric not used.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"100 + 6z (z=0 @ +6.0mOD)\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsiDataParameterValue {
    #[doc = "Code (or text) that identifies the use case for a parameter. See  7.2.4. Use of (data) case for example use cases. If the input is a code, this shall be defined in an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. May be left blank or omitted, but the combination of codeID and caseID shall be unique for the instances of this object contained within a single parent object instance."]
    #[serde(
        rename = "caseID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub case_id: ::std::option::Option<::std::string::String>,
    #[doc = "Code that identifies the parameter. Codes should be defined in either an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. The codes used by the instances of this object contained within a single parent object instance shall be unique, except that if caseID is used then only the combination of codeID and caseID needs to be unique."]
    #[serde(rename = "codeID")]
    pub code_id: AgsiDataParameterValueCodeId,
    #[doc = "Identifier for this data object. May be local to this file but all identifiers used within the Data group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it."]
    #[serde(
        rename = "dataID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub data_id: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "valueNumeric",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_numeric: ::std::option::Option<f64>,
    #[doc = "The profile of values as an ordered list of tuples of [independent variable value, parameter value]. Typically used to represent design lines. Refer to 1.6.9. Profiles or arrays of coordinate tuples for further information."]
    #[serde(
        rename = "valueProfile",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_profile: ::std::option::Option<ValueProfile>,
    #[doc = "Code that identifies the independent variable for a profile, i.e. what the property value varies against. The code shall be defined in an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object."]
    #[serde(
        rename = "valueProfileIndVarCodeID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_profile_ind_var_code_id: ::std::option::Option<::std::string::String>,
    #[doc = "Text based value of parameter, if applicable. For a profile (see below), this could be used for a concise description or representation of the profile. Unless specified otherwise, this attribute should only be used when the value is not numeric, i.e. valueNumeric not used."]
    #[serde(
        rename = "valueText",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_text: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsiDataParameterValue> for AgsiDataParameterValue {
    fn from(value: &AgsiDataParameterValue) -> Self {
        value.clone()
    }
}
impl AgsiDataParameterValue {
    pub fn builder() -> builder::AgsiDataParameterValue {
        Default::default()
    }
}
#[doc = "Code that identifies the parameter. Codes should be defined in either an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. The codes used by the instances of this object contained within a single parent object instance shall be unique, except that if caseID is used then only the combination of codeID and caseID needs to be unique."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Code that identifies the parameter. Codes should be defined in either an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. The codes used by the instances of this object contained within a single parent object instance shall be unique, except that if caseID is used then only the combination of codeID and caseID needs to be unique.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"UndrainedShearStrength\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsiDataParameterValueCodeId(::std::string::String);
impl ::std::ops::Deref for AgsiDataParameterValueCodeId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsiDataParameterValueCodeId> for ::std::string::String {
    fn from(value: AgsiDataParameterValueCodeId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsiDataParameterValueCodeId> for AgsiDataParameterValueCodeId {
    fn from(value: &AgsiDataParameterValueCodeId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsiDataParameterValueCodeId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsiDataParameterValueCodeId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsiDataParameterValueCodeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsiDataParameterValueCodeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsiDataParameterValueCodeId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "An agsiDataPropertyFromFile object is a pointer to data contained within an external file, such as an AGS, CSV or spreadsheet file. This object also includes metadata describing the file being referenced. Refer to 7.2.5. Limitations of agsiDataPropertyFromFile for further requirements and recommendations relating to this object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An agsiDataPropertyFromFile object is a pointer to data contained within an external file, such as an AGS, CSV or spreadsheet file. This object also includes metadata describing the file being referenced. Refer to 7.2.5. Limitations of agsiDataPropertyFromFile for further requirements and recommendations relating to this object.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"fileURI\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"dataID\": {"]
#[doc = "      \"description\": \"Identifier for this data object. May be local to this file but all identifiers used within the Data group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"42f18976-7352-4f67-9a6e-df89788343a7\""]
#[doc = "    },"]
#[doc = "    \"date\": {"]
#[doc = "      \"description\": \"Date of issue of this revision.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"2018-10-01\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Short description of data file defined here.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Additional data points for top of Gotham Clay from legacy boreholes, based on points marked on plan reference xxxx in report yyyy etc\""]
#[doc = "    },"]
#[doc = "    \"fileFormat\": {"]
#[doc = "      \"description\": \"Format/encoding of the data, i.e. file format. Refer to 9.2. Vocabulary for list of common formats that may be used, or provide concise identification if other format used.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"XLSX\""]
#[doc = "    },"]
#[doc = "    \"fileFormatVersion\": {"]
#[doc = "      \"description\": \"Additional version information for file format used, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"2019\""]
#[doc = "    },"]
#[doc = "    \"filePart\": {"]
#[doc = "      \"description\": \"Pointer to a specific part of a file, where required for disambiguation. For a spreadsheet file, this could be the name of the sheet used.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"GothamClay\""]
#[doc = "    },"]
#[doc = "    \"fileURI\": {"]
#[doc = "      \"description\": \"URI-reference for the data file. This will be a relative link if file is included as part of the AGSi package. Alternatively, a link to a location within a project document system. Spaces are not permitted in URI-reference strings. Refer to 1.6.6. URI for how to handle spaces in file paths or names.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-reference\","]
#[doc = "      \"example\": \"data/geology/legacydata.xlsx\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    },"]
#[doc = "    \"revision\": {"]
#[doc = "      \"description\": \"Revision of the referenced file.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"P3\""]
#[doc = "    },"]
#[doc = "    \"revisionInfo\": {"]
#[doc = "      \"description\": \"Revision notes for this revision of the referenced file.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Minor corrections, updated for GIR rev P2.\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsiDataPropertyFromFile {
    #[doc = "Identifier for this data object. May be local to this file but all identifiers used within the Data group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it."]
    #[serde(
        rename = "dataID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub data_id: ::std::option::Option<::std::string::String>,
    #[doc = "Date of issue of this revision."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::std::string::String>,
    #[doc = "Short description of data file defined here."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Format/encoding of the data, i.e. file format. Refer to 9.2. Vocabulary for list of common formats that may be used, or provide concise identification if other format used."]
    #[serde(
        rename = "fileFormat",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_format: ::std::option::Option<::std::string::String>,
    #[doc = "Additional version information for file format used, if required."]
    #[serde(
        rename = "fileFormatVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_format_version: ::std::option::Option<::std::string::String>,
    #[doc = "Pointer to a specific part of a file, where required for disambiguation. For a spreadsheet file, this could be the name of the sheet used."]
    #[serde(
        rename = "filePart",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_part: ::std::option::Option<::std::string::String>,
    #[doc = "URI-reference for the data file. This will be a relative link if file is included as part of the AGSi package. Alternatively, a link to a location within a project document system. Spaces are not permitted in URI-reference strings. Refer to 1.6.6. URI for how to handle spaces in file paths or names."]
    #[serde(rename = "fileURI")]
    pub file_uri: ::std::string::String,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[doc = "Revision of the referenced file."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub revision: ::std::option::Option<::std::string::String>,
    #[doc = "Revision notes for this revision of the referenced file."]
    #[serde(
        rename = "revisionInfo",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub revision_info: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsiDataPropertyFromFile> for AgsiDataPropertyFromFile {
    fn from(value: &AgsiDataPropertyFromFile) -> Self {
        value.clone()
    }
}
impl AgsiDataPropertyFromFile {
    pub fn builder() -> builder::AgsiDataPropertyFromFile {
        Default::default()
    }
}
#[doc = "Each agsiDataPropertySummary object provides a summary of data for a single defined property. Refer to 7.2. Data rules and conventions for further details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Each agsiDataPropertySummary object provides a summary of data for a single defined property. Refer to 7.2. Data rules and conventions for further details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"codeID\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"caseID\": {"]
#[doc = "      \"description\": \"Code (or text) that identifies the use case for a property. See 7.2.4. Use of (data) case for example use cases. If the input is a code, this shall be defined in an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. May be left blank or omitted, but the combination of codeID and caseID shall be unique for the instances of this object contained within a single parent object instance.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Clay\""]
#[doc = "    },"]
#[doc = "    \"codeID\": {"]
#[doc = "      \"description\": \"Code that identifies the property. Codes should be defined in either an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. The codes used by the instances of this object contained within a single parent object instance shall be unique, except that if caseID is used then only the combination of codeID and caseID needs to be unique.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"TRIG_CU\""]
#[doc = "    },"]
#[doc = "    \"dataID\": {"]
#[doc = "      \"description\": \"Identifier for this data object. May be local to this file but all identifiers used within the Data group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"42f18976-7352-4f67-9a6e-df89788343a7\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    },"]
#[doc = "    \"valueCount\": {"]
#[doc = "      \"description\": \"Number of results.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 58"]
#[doc = "    },"]
#[doc = "    \"valueMax\": {"]
#[doc = "      \"description\": \"Maximum value.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 345"]
#[doc = "    },"]
#[doc = "    \"valueMean\": {"]
#[doc = "      \"description\": \"Mean value.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 178.2"]
#[doc = "    },"]
#[doc = "    \"valueMin\": {"]
#[doc = "      \"description\": \"Minimum value.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 78"]
#[doc = "    },"]
#[doc = "    \"valueStdDev\": {"]
#[doc = "      \"description\": \"Standard deviation.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 36.4"]
#[doc = "    },"]
#[doc = "    \"valueSummaryText\": {"]
#[doc = "      \"description\": \"Alternative text based summary, if required or preferred. May be needed when some or all values are not numeric, e.g. <0.001.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"<0.01 to 12.57, mean 3.21, (16 results)\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsiDataPropertySummary {
    #[doc = "Code (or text) that identifies the use case for a property. See 7.2.4. Use of (data) case for example use cases. If the input is a code, this shall be defined in an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. May be left blank or omitted, but the combination of codeID and caseID shall be unique for the instances of this object contained within a single parent object instance."]
    #[serde(
        rename = "caseID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub case_id: ::std::option::Option<::std::string::String>,
    #[doc = "Code that identifies the property. Codes should be defined in either an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. The codes used by the instances of this object contained within a single parent object instance shall be unique, except that if caseID is used then only the combination of codeID and caseID needs to be unique."]
    #[serde(rename = "codeID")]
    pub code_id: AgsiDataPropertySummaryCodeId,
    #[doc = "Identifier for this data object. May be local to this file but all identifiers used within the Data group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it."]
    #[serde(
        rename = "dataID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub data_id: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "valueCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_count: ::std::option::Option<f64>,
    #[serde(
        rename = "valueMax",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_max: ::std::option::Option<f64>,
    #[serde(
        rename = "valueMean",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_mean: ::std::option::Option<f64>,
    #[serde(
        rename = "valueMin",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_min: ::std::option::Option<f64>,
    #[serde(
        rename = "valueStdDev",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_std_dev: ::std::option::Option<f64>,
    #[doc = "Alternative text based summary, if required or preferred. May be needed when some or all values are not numeric, e.g. <0.001."]
    #[serde(
        rename = "valueSummaryText",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_summary_text: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsiDataPropertySummary> for AgsiDataPropertySummary {
    fn from(value: &AgsiDataPropertySummary) -> Self {
        value.clone()
    }
}
impl AgsiDataPropertySummary {
    pub fn builder() -> builder::AgsiDataPropertySummary {
        Default::default()
    }
}
#[doc = "Code that identifies the property. Codes should be defined in either an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. The codes used by the instances of this object contained within a single parent object instance shall be unique, except that if caseID is used then only the combination of codeID and caseID needs to be unique."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Code that identifies the property. Codes should be defined in either an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. The codes used by the instances of this object contained within a single parent object instance shall be unique, except that if caseID is used then only the combination of codeID and caseID needs to be unique.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"TRIG_CU\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsiDataPropertySummaryCodeId(::std::string::String);
impl ::std::ops::Deref for AgsiDataPropertySummaryCodeId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsiDataPropertySummaryCodeId> for ::std::string::String {
    fn from(value: AgsiDataPropertySummaryCodeId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsiDataPropertySummaryCodeId> for AgsiDataPropertySummaryCodeId {
    fn from(value: &AgsiDataPropertySummaryCodeId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsiDataPropertySummaryCodeId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsiDataPropertySummaryCodeId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsiDataPropertySummaryCodeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsiDataPropertySummaryCodeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsiDataPropertySummaryCodeId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Each agsiDataPropertyValue object provides the data for a single defined property. The property value conveyed may be numeric, a profile of numeric values (e.g. a design line) or text. Refer to 7.2. Data rules and conventions for further details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Each agsiDataPropertyValue object provides the data for a single defined property. The property value conveyed may be numeric, a profile of numeric values (e.g. a design line) or text. Refer to 7.2. Data rules and conventions for further details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"codeID\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"caseID\": {"]
#[doc = "      \"description\": \"Code (or text) that identifies the use case for a property. See  7.2.4. Use of (data) case for example use cases. If the input is a code, this shall be defined in an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. May be left blank or omitted, but the combination of codeID and caseID shall be unique for the instances of this object contained within a single parent object instance.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Clay\""]
#[doc = "    },"]
#[doc = "    \"codeID\": {"]
#[doc = "      \"description\": \"Code that identifies the property. Codes should be defined in either an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. The codes used by the instances of this object contained within a single parent object instance shall be unique, except that if caseID is used then only the combination of codeID and caseID needs to be unique.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"example\": \"TRIG_CU\""]
#[doc = "    },"]
#[doc = "    \"dataID\": {"]
#[doc = "      \"description\": \"Identifier for this data object. May be local to this file but all identifiers used within the Data group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"42f18976-7352-4f67-9a6e-df89788343a7\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    },"]
#[doc = "    \"valueNumeric\": {"]
#[doc = "      \"description\": \"Numeric value of a single property.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 120"]
#[doc = "    },"]
#[doc = "    \"valueProfile\": {"]
#[doc = "      \"description\": \"A profile of values as an ordered list of tuples of [independent variable value, property value]. Typically used to provide properties at different elevations. Refer to 1.6.9. Profiles or arrays of coordinate tuples for further information.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"$ref\": \"#/$defs/valueProfile\","]
#[doc = "      \"example\": ["]
#[doc = "        ["]
#[doc = "          15.5,"]
#[doc = "          60.0"]
#[doc = "        ],"]
#[doc = "        ["]
#[doc = "          14.0,"]
#[doc = "          75.0"]
#[doc = "        ],"]
#[doc = "        ["]
#[doc = "          12.5,"]
#[doc = "          105.0"]
#[doc = "        ]"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"valueProfileIndVarCodeID\": {"]
#[doc = "      \"description\": \"Code that identifies the independent variable for a profile, i.e. what the property value varies against. The code shall be defined in an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Elevation\""]
#[doc = "    },"]
#[doc = "    \"valueText\": {"]
#[doc = "      \"description\": \"Text value for property, if applicable.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Dry\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsiDataPropertyValue {
    #[doc = "Code (or text) that identifies the use case for a property. See  7.2.4. Use of (data) case for example use cases. If the input is a code, this shall be defined in an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. May be left blank or omitted, but the combination of codeID and caseID shall be unique for the instances of this object contained within a single parent object instance."]
    #[serde(
        rename = "caseID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub case_id: ::std::option::Option<::std::string::String>,
    #[doc = "Code that identifies the property. Codes should be defined in either an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. The codes used by the instances of this object contained within a single parent object instance shall be unique, except that if caseID is used then only the combination of codeID and caseID needs to be unique."]
    #[serde(rename = "codeID")]
    pub code_id: AgsiDataPropertyValueCodeId,
    #[doc = "Identifier for this data object. May be local to this file but all identifiers used within the Data group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it."]
    #[serde(
        rename = "dataID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub data_id: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "valueNumeric",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_numeric: ::std::option::Option<f64>,
    #[doc = "A profile of values as an ordered list of tuples of [independent variable value, property value]. Typically used to provide properties at different elevations. Refer to 1.6.9. Profiles or arrays of coordinate tuples for further information."]
    #[serde(
        rename = "valueProfile",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_profile: ::std::option::Option<ValueProfile>,
    #[doc = "Code that identifies the independent variable for a profile, i.e. what the property value varies against. The code shall be defined in an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object."]
    #[serde(
        rename = "valueProfileIndVarCodeID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_profile_ind_var_code_id: ::std::option::Option<::std::string::String>,
    #[doc = "Text value for property, if applicable."]
    #[serde(
        rename = "valueText",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_text: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsiDataPropertyValue> for AgsiDataPropertyValue {
    fn from(value: &AgsiDataPropertyValue) -> Self {
        value.clone()
    }
}
impl AgsiDataPropertyValue {
    pub fn builder() -> builder::AgsiDataPropertyValue {
        Default::default()
    }
}
#[doc = "Code that identifies the property. Codes should be defined in either an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. The codes used by the instances of this object contained within a single parent object instance shall be unique, except that if caseID is used then only the combination of codeID and caseID needs to be unique."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Code that identifies the property. Codes should be defined in either an agsProjectCode object, or in the code dictionary defined in the relevant agsProjectCodeSet object. The codes used by the instances of this object contained within a single parent object instance shall be unique, except that if caseID is used then only the combination of codeID and caseID needs to be unique.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"example\": \"TRIG_CU\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgsiDataPropertyValueCodeId(::std::string::String);
impl ::std::ops::Deref for AgsiDataPropertyValueCodeId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgsiDataPropertyValueCodeId> for ::std::string::String {
    fn from(value: AgsiDataPropertyValueCodeId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AgsiDataPropertyValueCodeId> for AgsiDataPropertyValueCodeId {
    fn from(value: &AgsiDataPropertyValueCodeId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AgsiDataPropertyValueCodeId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgsiDataPropertyValueCodeId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgsiDataPropertyValueCodeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgsiDataPropertyValueCodeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgsiDataPropertyValueCodeId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "An agsiGeometryAreaFromLines object defines an element as the area between top and/or bottom lines. This will typically be used on cross sections or fence diagrams. This is a linking object between model element and the source geometry for the lines. Refer to 6.2.4. Areas from lines for full details of how the area should be interpreted."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An agsiGeometryAreaFromLines object defines an element as the area between top and/or bottom lines. This will typically be used on cross sections or fence diagrams. This is a linking object between model element and the source geometry for the lines. Refer to 6.2.4. Areas from lines for full details of how the area should be interpreted.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"agsiGeometryTop\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"agsiGeometryBottom\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsiGeometryBottom\": {"]
#[doc = "      \"description\": \"Geometry for bottom line, as embedded agsiGeometryFromFile object. Definition of both top and bottom lines is strongly recommended to minimise the risk of error. Refer to 6.2.4. Areas from lines for further details.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "    },"]
#[doc = "    \"agsiGeometryTop\": {"]
#[doc = "      \"description\": \"Geometry for top line, as embedded agsiGeometryFromFile object. Definition of both top and bottom lines is strongly recommended to minimise the risk of error. Refer to 6.2.4. Areas from lines for further details.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Short description of geometry defined here.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Section AA, Gotham Clay\""]
#[doc = "    },"]
#[doc = "    \"geometryID\": {"]
#[doc = "      \"description\": \"Identifier for this geometry object. May be local to this file but all identifiers used within the Geometry group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"SectionAAGCC\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum AgsiGeometryAreaFromLines {
    Variant0 {
        #[doc = "Geometry for top line, as embedded agsiGeometryFromFile object. Definition of both top and bottom lines is strongly recommended to minimise the risk of error. Refer to 6.2.4. Areas from lines for further details."]
        #[serde(rename = "agsiGeometryTop")]
        agsi_geometry_top: AgsiGeometryFromFile,
        #[doc = "Short description of geometry defined here."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        #[doc = "Identifier for this geometry object. May be local to this file but all identifiers used within the Geometry group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it."]
        #[serde(
            rename = "geometryID",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geometry_id: ::std::option::Option<::std::string::String>,
        #[doc = "Additional remarks, if required."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        remarks: ::std::option::Option<::std::string::String>,
    },
    Variant1 {
        #[doc = "Geometry for bottom line, as embedded agsiGeometryFromFile object. Definition of both top and bottom lines is strongly recommended to minimise the risk of error. Refer to 6.2.4. Areas from lines for further details."]
        #[serde(rename = "agsiGeometryBottom")]
        agsi_geometry_bottom: AgsiGeometryFromFile,
        #[doc = "Short description of geometry defined here."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        #[doc = "Identifier for this geometry object. May be local to this file but all identifiers used within the Geometry group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it."]
        #[serde(
            rename = "geometryID",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geometry_id: ::std::option::Option<::std::string::String>,
        #[doc = "Additional remarks, if required."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        remarks: ::std::option::Option<::std::string::String>,
    },
}
impl ::std::convert::From<&Self> for AgsiGeometryAreaFromLines {
    fn from(value: &AgsiGeometryAreaFromLines) -> Self {
        value.clone()
    }
}
#[doc = "An agsiGeometryFromFile object is a pointer to geometry data contained within an external file, such as a CAD or model file. This object also includes metadata describing the file being referenced. Refer to 6.2. Geometry rules and conventions for further requirements and recommendations relating to this object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An agsiGeometryFromFile object is a pointer to geometry data contained within an external file, such as a CAD or model file. This object also includes metadata describing the file being referenced. Refer to 6.2. Geometry rules and conventions for further requirements and recommendations relating to this object.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"fileURI\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"date\": {"]
#[doc = "      \"description\": \"Date of issue of this revision.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date\","]
#[doc = "      \"example\": \"2018-10-07\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Short description of geometry defined here.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Top of GC\""]
#[doc = "    },"]
#[doc = "    \"fileFormat\": {"]
#[doc = "      \"description\": \"Format/encoding of the data, i.e. file format. Refer to vocabulary for list of common formats that may be used, or provide concise identification if other format used. Refer to 6.2.1. File formats for geometry for requirements and recommendations relating to file formats.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"LandXML\""]
#[doc = "    },"]
#[doc = "    \"fileFormatVersion\": {"]
#[doc = "      \"description\": \"Additional version information for file format used, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"filePart\": {"]
#[doc = "      \"description\": \"Pointer to a specific part of a file, where required for disambiguation. For CAD or model files this could be used for the layer/level on which the required data is located. For a geoJSON file with a feature collection this could be used to specify the id of the feature of interest.  Use with caution as the ability to interrogate only a specified layer/level/feature etc. may not be supported in all software.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"GCTop\""]
#[doc = "    },"]
#[doc = "    \"fileURI\": {"]
#[doc = "      \"description\": \"URI-reference for the geometry file. This will be a relative link if file is included as part of the AGSi package. Alternatively, a link to a project document system location. Refer to 6.2.1. File formats for geometry for requirements and recommendations relating to linked files. Spaces are not permitted in URI-reference strings. Refer to 1.6.6. URI for how to handle spaces in file paths or names.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-reference\","]
#[doc = "      \"example\": \"geometry/geology/GCtop.xml\""]
#[doc = "    },"]
#[doc = "    \"geometryID\": {"]
#[doc = "      \"description\": \"Identifier for this geometry object. May be local to this file but all identifiers used within the Geometry group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"GeologyGCCTop\""]
#[doc = "    },"]
#[doc = "    \"geometryType\": {"]
#[doc = "      \"description\": \"Nature of geometry represented.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Surface\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    },"]
#[doc = "    \"revision\": {"]
#[doc = "      \"description\": \"Revision of the referenced file.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"P2\""]
#[doc = "    },"]
#[doc = "    \"revisionInfo\": {"]
#[doc = "      \"description\": \"Revision notes for this revision of the referenced file.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Updated for GIR rev P2. Additional BH from 2018 GI included.\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsiGeometryFromFile {
    #[doc = "Date of issue of this revision."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::chrono::naive::NaiveDate>,
    #[doc = "Short description of geometry defined here."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Format/encoding of the data, i.e. file format. Refer to vocabulary for list of common formats that may be used, or provide concise identification if other format used. Refer to 6.2.1. File formats for geometry for requirements and recommendations relating to file formats."]
    #[serde(
        rename = "fileFormat",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_format: ::std::option::Option<::std::string::String>,
    #[doc = "Additional version information for file format used, if required."]
    #[serde(
        rename = "fileFormatVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_format_version: ::std::option::Option<::std::string::String>,
    #[doc = "Pointer to a specific part of a file, where required for disambiguation. For CAD or model files this could be used for the layer/level on which the required data is located. For a geoJSON file with a feature collection this could be used to specify the id of the feature of interest.  Use with caution as the ability to interrogate only a specified layer/level/feature etc. may not be supported in all software."]
    #[serde(
        rename = "filePart",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_part: ::std::option::Option<::std::string::String>,
    #[doc = "URI-reference for the geometry file. This will be a relative link if file is included as part of the AGSi package. Alternatively, a link to a project document system location. Refer to 6.2.1. File formats for geometry for requirements and recommendations relating to linked files. Spaces are not permitted in URI-reference strings. Refer to 1.6.6. URI for how to handle spaces in file paths or names."]
    #[serde(rename = "fileURI")]
    pub file_uri: ::std::string::String,
    #[doc = "Identifier for this geometry object. May be local to this file but all identifiers used within the Geometry group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it."]
    #[serde(
        rename = "geometryID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub geometry_id: ::std::option::Option<::std::string::String>,
    #[doc = "Nature of geometry represented."]
    #[serde(
        rename = "geometryType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub geometry_type: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[doc = "Revision of the referenced file."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub revision: ::std::option::Option<::std::string::String>,
    #[doc = "Revision notes for this revision of the referenced file."]
    #[serde(
        rename = "revisionInfo",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub revision_info: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsiGeometryFromFile> for AgsiGeometryFromFile {
    fn from(value: &AgsiGeometryFromFile) -> Self {
        value.clone()
    }
}
impl AgsiGeometryFromFile {
    pub fn builder() -> builder::AgsiGeometryFromFile {
        Default::default()
    }
}
#[doc = "An agsiGeometryLayer object is a volumetric element bounded by two infinite horizontal planes at specified elevations. May be used for defining each element in a stratigraphical column (one dimensional) model. See 6.2.5. Simple 1D layers for interpretation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An agsiGeometryLayer object is a volumetric element bounded by two infinite horizontal planes at specified elevations. May be used for defining each element in a stratigraphical column (one dimensional) model. See 6.2.5. Simple 1D layers for interpretation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"topElevation\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"bottomElevation\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bottomElevation\": {"]
#[doc = "      \"description\": \"Elevation (z) of the bottom surface. Definition of both top and bottom surfaces is recommended to minimise the risk of error.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": -30"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Short description of geometry defined here.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Design profile, pile design: Gotham Clay\""]
#[doc = "    },"]
#[doc = "    \"geometryID\": {"]
#[doc = "      \"description\": \"Identifier for this geometry object. May be local to this file but all identifiers used within the Geometry group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"DesignPileGCC\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    },"]
#[doc = "    \"topElevation\": {"]
#[doc = "      \"description\": \"Elevation (z) of the top surface. Definition of both top and bottom surfaces is recommended to minimise the risk of error.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 6"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum AgsiGeometryLayer {
    Variant0 {
        #[doc = "Short description of geometry defined here."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        #[doc = "Identifier for this geometry object. May be local to this file but all identifiers used within the Geometry group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it."]
        #[serde(
            rename = "geometryID",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geometry_id: ::std::option::Option<::std::string::String>,
        #[doc = "Additional remarks, if required."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        remarks: ::std::option::Option<::std::string::String>,
        #[serde(rename = "topElevation")]
        top_elevation: f64,
    },
    Variant1 {
        #[serde(rename = "bottomElevation")]
        bottom_elevation: f64,
        #[doc = "Short description of geometry defined here."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        #[doc = "Identifier for this geometry object. May be local to this file but all identifiers used within the Geometry group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it."]
        #[serde(
            rename = "geometryID",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geometry_id: ::std::option::Option<::std::string::String>,
        #[doc = "Additional remarks, if required."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        remarks: ::std::option::Option<::std::string::String>,
    },
}
impl ::std::convert::From<&Self> for AgsiGeometryLayer {
    fn from(value: &AgsiGeometryLayer) -> Self {
        value.clone()
    }
}
#[doc = "An agsiGeometryPlane object is an infinite horizontal plane at a specified elevation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An agsiGeometryPlane object is an infinite horizontal plane at a specified elevation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"elevation\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Short description of geometry defined here.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Base of design model\""]
#[doc = "    },"]
#[doc = "    \"elevation\": {"]
#[doc = "      \"description\": \"Elevation (z) of the plane.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": -30"]
#[doc = "    },"]
#[doc = "    \"geometryID\": {"]
#[doc = "      \"description\": \"Identifier for this geometry object. May be local to this file but all identifiers used within the Geometry group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"DesignBase\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsiGeometryPlane {
    #[doc = "Short description of geometry defined here."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub elevation: f64,
    #[doc = "Identifier for this geometry object. May be local to this file but all identifiers used within the Geometry group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it."]
    #[serde(
        rename = "geometryID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub geometry_id: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsiGeometryPlane> for AgsiGeometryPlane {
    fn from(value: &AgsiGeometryPlane) -> Self {
        value.clone()
    }
}
impl AgsiGeometryPlane {
    pub fn builder() -> builder::AgsiGeometryPlane {
        Default::default()
    }
}
#[doc = "An agsiGeometryVolFromSurfaces object defines an element as the volumetric element (solid) between top and/or bottom surfaces. This is a linking object between model element and the source geometry for the surfaces, which will normally be agsiGeometryFromFile objects. Refer to 6.2.3. Volumes from surfaces for full details of how the volume should be interpreted."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An agsiGeometryVolFromSurfaces object defines an element as the volumetric element (solid) between top and/or bottom surfaces. This is a linking object between model element and the source geometry for the surfaces, which will normally be agsiGeometryFromFile objects. Refer to 6.2.3. Volumes from surfaces for full details of how the volume should be interpreted.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"agsiGeometryTop\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"agsiGeometryBottom\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsiGeometryBottom\": {"]
#[doc = "      \"description\": \"Geometry for bottom surface, as embedded agsiGeometryFromFile or agsiGeometryPlane object. Definition of both top and bottom surfaces is recommended to minimise the risk of error. Refer to 6.2.3. Volumes from surfaces for further details.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/agsiGeometryPlane\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"agsiGeometryTop\": {"]
#[doc = "      \"description\": \"Geometry for top surface, as embedded agsiGeometryFromFile or agsiGeometryPlane object. Definition of both top and bottom surfaces is recommended to minimise the risk of error. Refer to 6.2.3. Volumes from surfaces for further details.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/agsiGeometryPlane\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Short description of geometry defined here.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Gotham Clay\""]
#[doc = "    },"]
#[doc = "    \"geometryID\": {"]
#[doc = "      \"description\": \"Identifier for this geometry object. May be local to this file but all identifiers used within the Geometry group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"GeologyGCC\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum AgsiGeometryVolFromSurfaces {
    Variant0 {
        #[doc = "Geometry for top surface, as embedded agsiGeometryFromFile or agsiGeometryPlane object. Definition of both top and bottom surfaces is recommended to minimise the risk of error. Refer to 6.2.3. Volumes from surfaces for further details."]
        #[serde(rename = "agsiGeometryTop")]
        agsi_geometry_top: AgsiGeometryVolFromSurfacesVariant0AgsiGeometryTop,
        #[doc = "Short description of geometry defined here."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        #[doc = "Identifier for this geometry object. May be local to this file but all identifiers used within the Geometry group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it."]
        #[serde(
            rename = "geometryID",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geometry_id: ::std::option::Option<::std::string::String>,
        #[doc = "Additional remarks, if required."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        remarks: ::std::option::Option<::std::string::String>,
    },
    Variant1 {
        #[doc = "Geometry for bottom surface, as embedded agsiGeometryFromFile or agsiGeometryPlane object. Definition of both top and bottom surfaces is recommended to minimise the risk of error. Refer to 6.2.3. Volumes from surfaces for further details."]
        #[serde(rename = "agsiGeometryBottom")]
        agsi_geometry_bottom: AgsiGeometryVolFromSurfacesVariant1AgsiGeometryBottom,
        #[doc = "Short description of geometry defined here."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        #[doc = "Identifier for this geometry object. May be local to this file but all identifiers used within the Geometry group of objects shall be unique. Alternatively a UUID may be used (recommended for large datasets). Use of this attribute is optional and it is not referenced anywhere else in the schema, but it may be beneficial to include it to help with data control and integrity, and some applications may require or benefit from it."]
        #[serde(
            rename = "geometryID",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geometry_id: ::std::option::Option<::std::string::String>,
        #[doc = "Additional remarks, if required."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        remarks: ::std::option::Option<::std::string::String>,
    },
}
impl ::std::convert::From<&Self> for AgsiGeometryVolFromSurfaces {
    fn from(value: &AgsiGeometryVolFromSurfaces) -> Self {
        value.clone()
    }
}
#[doc = "Geometry for top surface, as embedded agsiGeometryFromFile or agsiGeometryPlane object. Definition of both top and bottom surfaces is recommended to minimise the risk of error. Refer to 6.2.3. Volumes from surfaces for further details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Geometry for top surface, as embedded agsiGeometryFromFile or agsiGeometryPlane object. Definition of both top and bottom surfaces is recommended to minimise the risk of error. Refer to 6.2.3. Volumes from surfaces for further details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryPlane\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum AgsiGeometryVolFromSurfacesVariant0AgsiGeometryTop {
    FromFile(AgsiGeometryFromFile),
    Plane(AgsiGeometryPlane),
}
impl ::std::convert::From<&Self> for AgsiGeometryVolFromSurfacesVariant0AgsiGeometryTop {
    fn from(value: &AgsiGeometryVolFromSurfacesVariant0AgsiGeometryTop) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<AgsiGeometryFromFile>
    for AgsiGeometryVolFromSurfacesVariant0AgsiGeometryTop
{
    fn from(value: AgsiGeometryFromFile) -> Self {
        Self::FromFile(value)
    }
}
impl ::std::convert::From<AgsiGeometryPlane>
    for AgsiGeometryVolFromSurfacesVariant0AgsiGeometryTop
{
    fn from(value: AgsiGeometryPlane) -> Self {
        Self::Plane(value)
    }
}
#[doc = "Geometry for bottom surface, as embedded agsiGeometryFromFile or agsiGeometryPlane object. Definition of both top and bottom surfaces is recommended to minimise the risk of error. Refer to 6.2.3. Volumes from surfaces for further details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Geometry for bottom surface, as embedded agsiGeometryFromFile or agsiGeometryPlane object. Definition of both top and bottom surfaces is recommended to minimise the risk of error. Refer to 6.2.3. Volumes from surfaces for further details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryPlane\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum AgsiGeometryVolFromSurfacesVariant1AgsiGeometryBottom {
    FromFile(AgsiGeometryFromFile),
    Plane(AgsiGeometryPlane),
}
impl ::std::convert::From<&Self> for AgsiGeometryVolFromSurfacesVariant1AgsiGeometryBottom {
    fn from(value: &AgsiGeometryVolFromSurfacesVariant1AgsiGeometryBottom) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<AgsiGeometryFromFile>
    for AgsiGeometryVolFromSurfacesVariant1AgsiGeometryBottom
{
    fn from(value: AgsiGeometryFromFile) -> Self {
        Self::FromFile(value)
    }
}
impl ::std::convert::From<AgsiGeometryPlane>
    for AgsiGeometryVolFromSurfacesVariant1AgsiGeometryBottom
{
    fn from(value: AgsiGeometryPlane) -> Self {
        Self::Plane(value)
    }
}
#[doc = "An agsiModel object is the parent object for a single model. It contains general metadata for a model as well as the embedded element and boundary objects that make up the model. It may also contain embedded sets of observation objects, which can be used to represent exploratory holes, their geology and other data, as well as general observations. agsiModel may also include the alignments of related sections (the sections themselves willl normally be separate models).  There can be several models, each defined by an agsiModel object, in an AGSi file."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An agsiModel object is the parent object for a single model. It contains general metadata for a model as well as the embedded element and boundary objects that make up the model. It may also contain embedded sets of observation objects, which can be used to represent exploratory holes, their geology and other data, as well as general observations. agsiModel may also include the alignments of related sections (the sections themselves willl normally be separate models).  There can be several models, each defined by an agsiModel object, in an AGSi file.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsiModelAlignment\": {"]
#[doc = "      \"description\": \"Array of embedded agsiModelAlignment objects. Used to define the (section) alignments in this (primary) model object.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiModelAlignment\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"agsiModelBoundary\": {"]
#[doc = "      \"description\": \"Single embedded agsiModelBoundary object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiModelBoundary\""]
#[doc = "    },"]
#[doc = "    \"agsiModelElement\": {"]
#[doc = "      \"description\": \"Array of embedded agsiModelElement objects.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiModelElement\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"agsiObservationSet\": {"]
#[doc = "      \"description\": \"Array of embedded agsiObservationSet objects.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiObservationSet\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"alignmentID\": {"]
#[doc = "      \"description\": \"Reference to ID of an agsiModelAlignment object found in another model. Required by 2D section models to identify the alignment of the section.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"sectionAA\""]
#[doc = "    },"]
#[doc = "    \"category\": {"]
#[doc = "      \"description\": \"Category of model.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Observational\""]
#[doc = "    },"]
#[doc = "    \"coordSystemID\": {"]
#[doc = "      \"description\": \"Reference to coordinate system applicable to this model (relevant agsProjectCoordinateSystem object).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"MetroXYZ\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"More verbose description of model, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"C999 Package X Sitewide geological model exported from SomeGeoModelSoftware. Incorporates 2018 GI data \""]
#[doc = "    },"]
#[doc = "    \"documentSetID\": {"]
#[doc = "      \"description\": \"Reference to documentation relating to model (reference to agsProjectDocumentSet object).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"ExampleDocSetID\""]
#[doc = "    },"]
#[doc = "    \"domain\": {"]
#[doc = "      \"description\": \"Domain of model.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Engineering geology\""]
#[doc = "    },"]
#[doc = "    \"input\": {"]
#[doc = "      \"description\": \"Short description of input data used by model an/or cross reference to document describing this.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Input data described in GIR section 3.3.2\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"description\": \"Short description of method used to create model, including software used, and/or reference to the document where this is discussed.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"3D model created in SomeGeoModelSoftware. See GIR section 3.3.3 for details.\""]
#[doc = "    },"]
#[doc = "    \"modelID\": {"]
#[doc = "      \"description\": \"Identifier for the model. May be local to this file or a UUID as required/specified. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it. If used, identifiers for modelID should be unique within the AGSi file. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"1fb599ab-c040-408d-aba0-85b18bb506c2\""]
#[doc = "    },"]
#[doc = "    \"modelName\": {"]
#[doc = "      \"description\": \"Short name of model.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Sitewide geological model\""]
#[doc = "    },"]
#[doc = "    \"modelType\": {"]
#[doc = "      \"description\": \"Type of model. Incorporates domain and category of model.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Geological model\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some additional remarks\""]
#[doc = "    },"]
#[doc = "    \"uncertainty\": {"]
#[doc = "      \"description\": \"Short statement discussing uncertainty with respect to the information presented in this model.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"The boundaries of the geological units presented in this model are a best estimate based on interpolation between exploratory holes, which in some cases are >100m apart. In addition, in some places the boundaries are based on interpretation of CPT results. Therefore the unit boundaries shown are subject to uncertainty, which increases with distance from the exploratory holes. Refer to GIR section 3.3.4 for more information. \""]
#[doc = "    },"]
#[doc = "    \"usage\": {"]
#[doc = "      \"description\": \"Short description of intended and/or permitted usage including limitations or restrictions. Strongly recommended.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Observational and interpolated geological profile. For reference and visualisation only. Not suitable for direct use in design. See GIR section 3.3.4 for details.\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsiModel {
    #[doc = "Array of embedded agsiModelAlignment objects. Used to define the (section) alignments in this (primary) model object."]
    #[serde(
        rename = "agsiModelAlignment",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_model_alignment: ::std::vec::Vec<AgsiModelAlignment>,
    #[doc = "Single embedded agsiModelBoundary object."]
    #[serde(
        rename = "agsiModelBoundary",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agsi_model_boundary: ::std::option::Option<AgsiModelBoundary>,
    #[doc = "Array of embedded agsiModelElement objects."]
    #[serde(
        rename = "agsiModelElement",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_model_element: ::std::vec::Vec<AgsiModelElement>,
    #[doc = "Array of embedded agsiObservationSet objects."]
    #[serde(
        rename = "agsiObservationSet",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_observation_set: ::std::vec::Vec<AgsiObservationSet>,
    #[doc = "Reference to ID of an agsiModelAlignment object found in another model. Required by 2D section models to identify the alignment of the section."]
    #[serde(
        rename = "alignmentID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub alignment_id: ::std::option::Option<::std::string::String>,
    #[doc = "Category of model."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub category: ::std::option::Option<::std::string::String>,
    #[doc = "Reference to coordinate system applicable to this model (relevant agsProjectCoordinateSystem object)."]
    #[serde(
        rename = "coordSystemID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub coord_system_id: ::std::option::Option<::std::string::String>,
    #[doc = "More verbose description of model, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Reference to documentation relating to model (reference to agsProjectDocumentSet object)."]
    #[serde(
        rename = "documentSetID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub document_set_id: ::std::option::Option<::std::string::String>,
    #[doc = "Domain of model."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub domain: ::std::option::Option<::std::string::String>,
    #[doc = "Short description of input data used by model an/or cross reference to document describing this."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<::std::string::String>,
    #[doc = "Short description of method used to create model, including software used, and/or reference to the document where this is discussed."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub method: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for the model. May be local to this file or a UUID as required/specified. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it. If used, identifiers for modelID should be unique within the AGSi file. "]
    #[serde(
        rename = "modelID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_id: ::std::option::Option<::std::string::String>,
    #[doc = "Short name of model."]
    #[serde(
        rename = "modelName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_name: ::std::option::Option<::std::string::String>,
    #[doc = "Type of model. Incorporates domain and category of model."]
    #[serde(
        rename = "modelType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_type: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[doc = "Short statement discussing uncertainty with respect to the information presented in this model."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub uncertainty: ::std::option::Option<::std::string::String>,
    #[doc = "Short description of intended and/or permitted usage including limitations or restrictions. Strongly recommended."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsiModel> for AgsiModel {
    fn from(value: &AgsiModel) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AgsiModel {
    fn default() -> Self {
        Self {
            agsi_model_alignment: Default::default(),
            agsi_model_boundary: Default::default(),
            agsi_model_element: Default::default(),
            agsi_observation_set: Default::default(),
            alignment_id: Default::default(),
            category: Default::default(),
            coord_system_id: Default::default(),
            description: Default::default(),
            document_set_id: Default::default(),
            domain: Default::default(),
            input: Default::default(),
            method: Default::default(),
            model_id: Default::default(),
            model_name: Default::default(),
            model_type: Default::default(),
            remarks: Default::default(),
            uncertainty: Default::default(),
            usage: Default::default(),
        }
    }
}
impl AgsiModel {
    pub fn builder() -> builder::AgsiModel {
        Default::default()
    }
}
#[doc = "An alignment comprises the geometry and metadata defining a line of interest, most commonly used for the line of a section (cross section, fence or profile  - see Guidance for discussion of terminology). Alignments are typically included in a primary 3D model (alternatively a 2D map) but the sections themselves are normally defined as separate models because each 2D section will have its own coordinate system. The alignmentID attribute can be referenced by section models to provide a link to the alignment defined here. Sections must be drawn in the vertical (Z) plane, with alignments defined in the XY plane. The geometry is specified using an embedded agsiGeometryFromFile object referencing an external file."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An alignment comprises the geometry and metadata defining a line of interest, most commonly used for the line of a section (cross section, fence or profile  - see Guidance for discussion of terminology). Alignments are typically included in a primary 3D model (alternatively a 2D map) but the sections themselves are normally defined as separate models because each 2D section will have its own coordinate system. The alignmentID attribute can be referenced by section models to provide a link to the alignment defined here. Sections must be drawn in the vertical (Z) plane, with alignments defined in the XY plane. The geometry is specified using an embedded agsiGeometryFromFile object referencing an external file.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsiGeometry\": {"]
#[doc = "      \"description\": \"An embedded Geometry group object defining the geometry of the  alignment as a 2D line in XY space. This will be an embedded agsiGeometryFromFile object pointing to a line defined in an external file.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "    },"]
#[doc = "    \"alignmentID\": {"]
#[doc = "      \"description\": \"Identifier for the alignment. May be local to this file or a UUID as required/specified. May be referenced by alignmentID attribute of (a different) agsiModel object. Identifiers for alignmentID shall be unique within the AGSi file.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"sectionAA\""]
#[doc = "    },"]
#[doc = "    \"alignmentName\": {"]
#[doc = "      \"description\": \"Name or short description of what this alignment represents.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Section AA\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"More verbose description, as required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"East-west section through site\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some additional remarks\""]
#[doc = "    },"]
#[doc = "    \"startChainage\": {"]
#[doc = "      \"description\": \"The section chainage/baseline distance for the first point defined on the alignment. Assumed to be zero if not used.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 1000"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsiModelAlignment {
    #[doc = "An embedded Geometry group object defining the geometry of the  alignment as a 2D line in XY space. This will be an embedded agsiGeometryFromFile object pointing to a line defined in an external file."]
    #[serde(
        rename = "agsiGeometry",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agsi_geometry: ::std::option::Option<AgsiGeometryFromFile>,
    #[doc = "Identifier for the alignment. May be local to this file or a UUID as required/specified. May be referenced by alignmentID attribute of (a different) agsiModel object. Identifiers for alignmentID shall be unique within the AGSi file."]
    #[serde(
        rename = "alignmentID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub alignment_id: ::std::option::Option<::std::string::String>,
    #[doc = "Name or short description of what this alignment represents."]
    #[serde(
        rename = "alignmentName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub alignment_name: ::std::option::Option<::std::string::String>,
    #[doc = "More verbose description, as required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "startChainage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub start_chainage: ::std::option::Option<f64>,
}
impl ::std::convert::From<&AgsiModelAlignment> for AgsiModelAlignment {
    fn from(value: &AgsiModelAlignment) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AgsiModelAlignment {
    fn default() -> Self {
        Self {
            agsi_geometry: Default::default(),
            alignment_id: Default::default(),
            alignment_name: Default::default(),
            description: Default::default(),
            remarks: Default::default(),
            start_chainage: Default::default(),
        }
    }
}
impl AgsiModelAlignment {
    pub fn builder() -> builder::AgsiModelAlignment {
        Default::default()
    }
}
#[doc = "An agsiModelBoundary object defines the model boundary, i.e. the maximum extent of the model. Any elements or parts of elements lying outside the boundary are deemed to not be part of the model. Only one boundary per agsiModel is permitted. Only plan boundaries with vertical sides are permitted, defined by either limiting coordinates, or a bounding closed polygon. The top and base may be either a flat plane at a defined elevation, or a surface. Top boundary may not be required, depending on nature of model and/or software/application used (to be confirmed in specification). "]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An agsiModelBoundary object defines the model boundary, i.e. the maximum extent of the model. Any elements or parts of elements lying outside the boundary are deemed to not be part of the model. Only one boundary per agsiModel is permitted. Only plan boundaries with vertical sides are permitted, defined by either limiting coordinates, or a bounding closed polygon. The top and base may be either a flat plane at a defined elevation, or a surface. Top boundary may not be required, depending on nature of model and/or software/application used (to be confirmed in specification). \","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsiGeometryBoundaryXY\": {"]
#[doc = "      \"description\": \"Embedded agsiGeometryFromFile object that provides the geometry of the closed polygon defining the plan extent of model, as an alternative to the box boundary. Use with caution as this may not be supported by all software/applications. Confirm use in specification.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "    },"]
#[doc = "    \"agsiGeometrySurfaceBottom\": {"]
#[doc = "      \"description\": \"Embedded agsiGeometryFromFile object that defines the base of the model, as an alternative to the box boundary. Use with caution as this may not be supported by all software/applications. Confirm use in specification.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "    },"]
#[doc = "    \"agsiGeometrySurfaceTop\": {"]
#[doc = "      \"description\": \"Embedded agsiGeometryFromFile object that defines the top of the model, as an alternative to the box boundary. Use with caution as this may not be supported by all software/applications. May not be required for some software/applications. Confirm use in specification.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "    },"]
#[doc = "    \"bottomElevation\": {"]
#[doc = "      \"description\": \"Elevation (Z) of bottom plane of model for box boundary.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": -40"]
#[doc = "    },"]
#[doc = "    \"boundaryID\": {"]
#[doc = "      \"description\": \"Identifier for the model boundary. May be local to this file or a UUID as required/specified. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it.  If used, identifiers for boundaryID should be unique within the AGSi file. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"BoundarySitewide\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Short description.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Boundary for Geological Model: sitewide\""]
#[doc = "    },"]
#[doc = "    \"maxX\": {"]
#[doc = "      \"description\": \"Maximum X for box boundary.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 35000"]
#[doc = "    },"]
#[doc = "    \"maxY\": {"]
#[doc = "      \"description\": \"Maximum Y for box boundary.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 15000"]
#[doc = "    },"]
#[doc = "    \"minX\": {"]
#[doc = "      \"description\": \"Minimum X for box boundary.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 20000"]
#[doc = "    },"]
#[doc = "    \"minY\": {"]
#[doc = "      \"description\": \"Minimum Y for box boundary.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 10000"]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some additional remarks\""]
#[doc = "    },"]
#[doc = "    \"topElevation\": {"]
#[doc = "      \"description\": \"Elevation (Z) of top plane of model for box boundary.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 40"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsiModelBoundary {
    #[doc = "Embedded agsiGeometryFromFile object that provides the geometry of the closed polygon defining the plan extent of model, as an alternative to the box boundary. Use with caution as this may not be supported by all software/applications. Confirm use in specification."]
    #[serde(
        rename = "agsiGeometryBoundaryXY",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agsi_geometry_boundary_xy: ::std::option::Option<AgsiGeometryFromFile>,
    #[doc = "Embedded agsiGeometryFromFile object that defines the base of the model, as an alternative to the box boundary. Use with caution as this may not be supported by all software/applications. Confirm use in specification."]
    #[serde(
        rename = "agsiGeometrySurfaceBottom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agsi_geometry_surface_bottom: ::std::option::Option<AgsiGeometryFromFile>,
    #[doc = "Embedded agsiGeometryFromFile object that defines the top of the model, as an alternative to the box boundary. Use with caution as this may not be supported by all software/applications. May not be required for some software/applications. Confirm use in specification."]
    #[serde(
        rename = "agsiGeometrySurfaceTop",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agsi_geometry_surface_top: ::std::option::Option<AgsiGeometryFromFile>,
    #[serde(
        rename = "bottomElevation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bottom_elevation: ::std::option::Option<f64>,
    #[doc = "Identifier for the model boundary. May be local to this file or a UUID as required/specified. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it.  If used, identifiers for boundaryID should be unique within the AGSi file. "]
    #[serde(
        rename = "boundaryID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub boundary_id: ::std::option::Option<::std::string::String>,
    #[doc = "Short description."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "maxX",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_x: ::std::option::Option<f64>,
    #[serde(
        rename = "maxY",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_y: ::std::option::Option<f64>,
    #[serde(
        rename = "minX",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_x: ::std::option::Option<f64>,
    #[serde(
        rename = "minY",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_y: ::std::option::Option<f64>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "topElevation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub top_elevation: ::std::option::Option<f64>,
}
impl ::std::convert::From<&AgsiModelBoundary> for AgsiModelBoundary {
    fn from(value: &AgsiModelBoundary) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AgsiModelBoundary {
    fn default() -> Self {
        Self {
            agsi_geometry_boundary_xy: Default::default(),
            agsi_geometry_surface_bottom: Default::default(),
            agsi_geometry_surface_top: Default::default(),
            bottom_elevation: Default::default(),
            boundary_id: Default::default(),
            description: Default::default(),
            max_x: Default::default(),
            max_y: Default::default(),
            min_x: Default::default(),
            min_y: Default::default(),
            remarks: Default::default(),
            top_elevation: Default::default(),
        }
    }
}
impl AgsiModelBoundary {
    pub fn builder() -> builder::AgsiModelBoundary {
        Default::default()
    }
}
#[doc = "A model is made up of elements. These elements are defined by agsiModelElement objects. Each element will have embedded geometry. Which class of object is referenced will depend on the form of geometry required. Elements may also have embedded data (properties and parameters)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A model is made up of elements. These elements are defined by agsiModelElement objects. Each element will have embedded geometry. Which class of object is referenced will depend on the form of geometry required. Elements may also have embedded data (properties and parameters).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsiDataParameterValue\": {"]
#[doc = "      \"description\": \"Array of embedded agsiDataParameterValue objects providing parameter data specific to this model element.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiDataParameterValue\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"agsiDataPropertyFromFile\": {"]
#[doc = "      \"description\": \"An embedded agsiDataPropertyFromFile object, which may be used to reference to an external supporting data file.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiDataPropertyFromFile\""]
#[doc = "    },"]
#[doc = "    \"agsiDataPropertySummary\": {"]
#[doc = "      \"description\": \"Array of embedded agsiDataPropertySummary objects providing summaries of property data specific to this model element.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiDataPropertySummary\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"agsiDataPropertyValue\": {"]
#[doc = "      \"description\": \"Array of embedded agsiDataPropertyValue objects providing discrete property data specific to this model element.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiDataPropertyValue\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"agsiGeometry\": {"]
#[doc = "      \"description\": \"An embedded Geometry group object defining the geometry for this element. The object type referenced will depend on the type of geometry, which should be defined in geometryObject. Only one object per element allowed.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/agsiGeometryVolFromSurfaces\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/agsiGeometryAreaFromLines\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/agsiGeometryPlane\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/agsiGeometryLayer\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"agsiGeometryAreaLimit\": {"]
#[doc = "      \"description\": \"If required, an embedded agsiGeometryFromFile object defining the limiting plan area for the element may be defined by reference to a closed polygon object. The polygon acts as a 'cookie cutter' so the element boundary will be curtailed to stay within the polygon. Geometry beyond the boundary is ignored. This allows a large element to be easily divided up into parts, e.g. to allow different properties or parameters to be reported for each part. Use with caution as it may not be supported by all software/applications. Confirm usage in specification. Only one object per element allowed.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "    },"]
#[doc = "    \"colourRGB\": {"]
#[doc = "      \"description\": \"Preferred display colour (RGB hexadecimal)\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"#c0c0c0\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"More verbose description, as required. Usage may be determined by type of element, e.g. for a geological unit this could be used to describe typical lithology.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Stiff to very stiff slightly sandy blue/grey CLAY, with occasional claystone layers (typically <0.1m). Becoming very sandy towards base of unit.\""]
#[doc = "    },"]
#[doc = "    \"elementID\": {"]
#[doc = "      \"description\": \"Identifier for the model element. May be local to this file or a UUID as required/specified. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it. If used, identifiers for elementID should be unique within the AGSi file. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"GC/W\""]
#[doc = "    },"]
#[doc = "    \"elementName\": {"]
#[doc = "      \"description\": \"Name or short description of what this element represents.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Gotham Clay, west zone\""]
#[doc = "    },"]
#[doc = "    \"elementType\": {"]
#[doc = "      \"description\": \"Type of element, i.e. what the element represents in general terms.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Geological unit\""]
#[doc = "    },"]
#[doc = "    \"geometryObject\": {"]
#[doc = "      \"description\": \"Object type (from Geometry group) embedded in agsiGeometry attribute.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"agsiGeometryVolFromSurfaces\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some additional remarks\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsiModelElement {
    #[doc = "Array of embedded agsiDataParameterValue objects providing parameter data specific to this model element."]
    #[serde(
        rename = "agsiDataParameterValue",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_data_parameter_value: ::std::vec::Vec<AgsiDataParameterValue>,
    #[doc = "An embedded agsiDataPropertyFromFile object, which may be used to reference to an external supporting data file."]
    #[serde(
        rename = "agsiDataPropertyFromFile",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agsi_data_property_from_file: ::std::option::Option<AgsiDataPropertyFromFile>,
    #[doc = "Array of embedded agsiDataPropertySummary objects providing summaries of property data specific to this model element."]
    #[serde(
        rename = "agsiDataPropertySummary",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_data_property_summary: ::std::vec::Vec<AgsiDataPropertySummary>,
    #[doc = "Array of embedded agsiDataPropertyValue objects providing discrete property data specific to this model element."]
    #[serde(
        rename = "agsiDataPropertyValue",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_data_property_value: ::std::vec::Vec<AgsiDataPropertyValue>,
    #[doc = "An embedded Geometry group object defining the geometry for this element. The object type referenced will depend on the type of geometry, which should be defined in geometryObject. Only one object per element allowed."]
    #[serde(
        rename = "agsiGeometry",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agsi_geometry: ::std::option::Option<AgsiModelElementAgsiGeometry>,
    #[doc = "If required, an embedded agsiGeometryFromFile object defining the limiting plan area for the element may be defined by reference to a closed polygon object. The polygon acts as a 'cookie cutter' so the element boundary will be curtailed to stay within the polygon. Geometry beyond the boundary is ignored. This allows a large element to be easily divided up into parts, e.g. to allow different properties or parameters to be reported for each part. Use with caution as it may not be supported by all software/applications. Confirm usage in specification. Only one object per element allowed."]
    #[serde(
        rename = "agsiGeometryAreaLimit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agsi_geometry_area_limit: ::std::option::Option<AgsiGeometryFromFile>,
    #[doc = "Preferred display colour (RGB hexadecimal)"]
    #[serde(
        rename = "colourRGB",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub colour_rgb: ::std::option::Option<::std::string::String>,
    #[doc = "More verbose description, as required. Usage may be determined by type of element, e.g. for a geological unit this could be used to describe typical lithology."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for the model element. May be local to this file or a UUID as required/specified. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it. If used, identifiers for elementID should be unique within the AGSi file. "]
    #[serde(
        rename = "elementID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub element_id: ::std::option::Option<::std::string::String>,
    #[doc = "Name or short description of what this element represents."]
    #[serde(
        rename = "elementName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub element_name: ::std::option::Option<::std::string::String>,
    #[doc = "Type of element, i.e. what the element represents in general terms."]
    #[serde(
        rename = "elementType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub element_type: ::std::option::Option<::std::string::String>,
    #[doc = "Object type (from Geometry group) embedded in agsiGeometry attribute."]
    #[serde(
        rename = "geometryObject",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub geometry_object: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsiModelElement> for AgsiModelElement {
    fn from(value: &AgsiModelElement) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AgsiModelElement {
    fn default() -> Self {
        Self {
            agsi_data_parameter_value: Default::default(),
            agsi_data_property_from_file: Default::default(),
            agsi_data_property_summary: Default::default(),
            agsi_data_property_value: Default::default(),
            agsi_geometry: Default::default(),
            agsi_geometry_area_limit: Default::default(),
            colour_rgb: Default::default(),
            description: Default::default(),
            element_id: Default::default(),
            element_name: Default::default(),
            element_type: Default::default(),
            geometry_object: Default::default(),
            remarks: Default::default(),
        }
    }
}
impl AgsiModelElement {
    pub fn builder() -> builder::AgsiModelElement {
        Default::default()
    }
}
#[doc = "An embedded Geometry group object defining the geometry for this element. The object type referenced will depend on the type of geometry, which should be defined in geometryObject. Only one object per element allowed."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An embedded Geometry group object defining the geometry for this element. The object type referenced will depend on the type of geometry, which should be defined in geometryObject. Only one object per element allowed.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryVolFromSurfaces\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryAreaFromLines\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryPlane\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryLayer\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AgsiModelElementAgsiGeometry {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_0: ::std::option::Option<AgsiGeometryVolFromSurfaces>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_1: ::std::option::Option<AgsiGeometryFromFile>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_2: ::std::option::Option<AgsiGeometryAreaFromLines>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_3: ::std::option::Option<AgsiGeometryPlane>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_4: ::std::option::Option<AgsiGeometryLayer>,
}
impl ::std::convert::From<&AgsiModelElementAgsiGeometry> for AgsiModelElementAgsiGeometry {
    fn from(value: &AgsiModelElementAgsiGeometry) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AgsiModelElementAgsiGeometry {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
            subtype_3: Default::default(),
            subtype_4: Default::default(),
        }
    }
}
impl AgsiModelElementAgsiGeometry {
    pub fn builder() -> builder::AgsiModelElementAgsiGeometry {
        Default::default()
    }
}
#[doc = "An agsiObservationColumn object provides the data for a single column segment within an exporatory hole, i.e. value/text over a defined range of depth/elevation. Typically used for geological logging descriptions and attributes are included to facilitate this. May alternatively be used with embedded agsiDataPropertyValue objects for other column segment data. Segments may be defined using depth (relative to top of parent hole) or elevation, or both."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An agsiObservationColumn object provides the data for a single column segment within an exporatory hole, i.e. value/text over a defined range of depth/elevation. Typically used for geological logging descriptions and attributes are included to facilitate this. May alternatively be used with embedded agsiDataPropertyValue objects for other column segment data. Segments may be defined using depth (relative to top of parent hole) or elevation, or both.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"topDepth\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"topElevation\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsiDataPropertyValue\": {"]
#[doc = "      \"description\": \"Array of embedded agsiDataPropertyValue objects. May be used to provide other data for this range of depth/elevation.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiDataPropertyValue\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"bottomDepth\": {"]
#[doc = "      \"description\": \"Depth from top of parent exploratory hole to the bottom of the column segment. For inclined holes defined using a profile, this shall be the length measured along the line of the hole, not adjusted vertical depth.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 34.7"]
#[doc = "    },"]
#[doc = "    \"bottomElevation\": {"]
#[doc = "      \"description\": \"Elevation of the bottom of the column segment. For inclined holes, this shall be the true calculated elevation.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": -28.4"]
#[doc = "    },"]
#[doc = "    \"columnID\": {"]
#[doc = "      \"description\": \"Identifier for this particular column observation. May be local to this file or a UUID as required/specified. This is optional and not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"8526ef28-7a26-4c6f-b305-5e9607a7ab6d\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Geological description.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Stiff to very stiff blue grey slightly sandy silty CLAY with rare claystone layers (GOTHAM CLAY)\""]
#[doc = "    },"]
#[doc = "    \"geologyBGS\": {"]
#[doc = "      \"description\": \"BGS Lexicon code (applicable in UK only).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"GC\""]
#[doc = "    },"]
#[doc = "    \"geologyCode\": {"]
#[doc = "      \"description\": \"Geology code. Typically a project specific code defined defined using agsProjectCode.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"GC\""]
#[doc = "    },"]
#[doc = "    \"geologyCode2\": {"]
#[doc = "      \"description\": \"2nd geology code, if applicable. Typically a project specific code defined using agsProjectCode.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"A2\""]
#[doc = "    },"]
#[doc = "    \"geologyFormation\": {"]
#[doc = "      \"description\": \"Geological formation or stratum name.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Gotham Clay\""]
#[doc = "    },"]
#[doc = "    \"legendCode\": {"]
#[doc = "      \"description\": \"Legend code. Recommend using code from AGS format ABBR code list.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"201\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    },"]
#[doc = "    \"topDepth\": {"]
#[doc = "      \"description\": \"Depth from top of parent exploratory hole to the top of the column segment. For inclined holes defined using a profile, this shall be the length measured along the line of the hole, not adjusted vertical depth.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 8.9"]
#[doc = "    },"]
#[doc = "    \"topElevation\": {"]
#[doc = "      \"description\": \"Elevation of the top of the column segment. For inclined holes, this shall be the true calculated elevation.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 6.3"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum AgsiObservationColumn {
    Variant0 {
        #[doc = "Array of embedded agsiDataPropertyValue objects. May be used to provide other data for this range of depth/elevation."]
        #[serde(
            rename = "agsiDataPropertyValue",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        agsi_data_property_value: ::std::vec::Vec<AgsiDataPropertyValue>,
        #[serde(
            rename = "bottomDepth",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        bottom_depth: ::std::option::Option<f64>,
        #[serde(
            rename = "bottomElevation",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        bottom_elevation: ::std::option::Option<f64>,
        #[doc = "Identifier for this particular column observation. May be local to this file or a UUID as required/specified. This is optional and not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it."]
        #[serde(
            rename = "columnID",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        column_id: ::std::option::Option<::std::string::String>,
        #[doc = "Geological description."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        #[doc = "BGS Lexicon code (applicable in UK only)."]
        #[serde(
            rename = "geologyBGS",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geology_bgs: ::std::option::Option<::std::string::String>,
        #[doc = "Geology code. Typically a project specific code defined defined using agsProjectCode."]
        #[serde(
            rename = "geologyCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geology_code: ::std::option::Option<::std::string::String>,
        #[doc = "2nd geology code, if applicable. Typically a project specific code defined using agsProjectCode."]
        #[serde(
            rename = "geologyCode2",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geology_code2: ::std::option::Option<::std::string::String>,
        #[doc = "Geological formation or stratum name."]
        #[serde(
            rename = "geologyFormation",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geology_formation: ::std::option::Option<::std::string::String>,
        #[doc = "Legend code. Recommend using code from AGS format ABBR code list."]
        #[serde(
            rename = "legendCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        legend_code: ::std::option::Option<::std::string::String>,
        #[doc = "Additional remarks, if required."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        remarks: ::std::option::Option<::std::string::String>,
        #[serde(rename = "topDepth")]
        top_depth: f64,
    },
    Variant1 {
        #[doc = "Array of embedded agsiDataPropertyValue objects. May be used to provide other data for this range of depth/elevation."]
        #[serde(
            rename = "agsiDataPropertyValue",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        agsi_data_property_value: ::std::vec::Vec<AgsiDataPropertyValue>,
        #[serde(
            rename = "bottomDepth",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        bottom_depth: ::std::option::Option<f64>,
        #[serde(
            rename = "bottomElevation",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        bottom_elevation: ::std::option::Option<f64>,
        #[doc = "Identifier for this particular column observation. May be local to this file or a UUID as required/specified. This is optional and not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it."]
        #[serde(
            rename = "columnID",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        column_id: ::std::option::Option<::std::string::String>,
        #[doc = "Geological description."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        #[doc = "BGS Lexicon code (applicable in UK only)."]
        #[serde(
            rename = "geologyBGS",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geology_bgs: ::std::option::Option<::std::string::String>,
        #[doc = "Geology code. Typically a project specific code defined defined using agsProjectCode."]
        #[serde(
            rename = "geologyCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geology_code: ::std::option::Option<::std::string::String>,
        #[doc = "2nd geology code, if applicable. Typically a project specific code defined using agsProjectCode."]
        #[serde(
            rename = "geologyCode2",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geology_code2: ::std::option::Option<::std::string::String>,
        #[doc = "Geological formation or stratum name."]
        #[serde(
            rename = "geologyFormation",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        geology_formation: ::std::option::Option<::std::string::String>,
        #[doc = "Legend code. Recommend using code from AGS format ABBR code list."]
        #[serde(
            rename = "legendCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        legend_code: ::std::option::Option<::std::string::String>,
        #[doc = "Additional remarks, if required."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        remarks: ::std::option::Option<::std::string::String>,
        #[serde(rename = "topElevation")]
        top_elevation: f64,
    },
}
impl ::std::convert::From<&Self> for AgsiObservationColumn {
    fn from(value: &AgsiObservationColumn) -> Self {
        value.clone()
    }
}
#[doc = "An agsiObservationExpHole object provides geometry and common metadata for a single exploratory hole (borehole, trial pit, CPT etc.). Further data can be provided using embedded agsiDataPropertyValue objects if required. Embedded agsiObservationColumn objects may be used to provide a representation of the geology encountered in hole. A link to a supporting data file can be provided using an embedded agsiDataPropertyFromFile object. "]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An agsiObservationExpHole object provides geometry and common metadata for a single exploratory hole (borehole, trial pit, CPT etc.). Further data can be provided using embedded agsiDataPropertyValue objects if required. Embedded agsiObservationColumn objects may be used to provide a representation of the geology encountered in hole. A link to a supporting data file can be provided using an embedded agsiDataPropertyFromFile object. \","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"holeID\","]
#[doc = "        \"topCoordinate\","]
#[doc = "        \"verticalHoleDepth\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"holeID\","]
#[doc = "        \"profileCoordinates\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsiDataPropertyFromFile\": {"]
#[doc = "      \"description\": \"An embedded agsiDataPropertyFromFile object, which may be used to reference an external supporting data file.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiDataPropertyFromFile\""]
#[doc = "    },"]
#[doc = "    \"agsiDataPropertyValue\": {"]
#[doc = "      \"description\": \"Array of embedded agsiDataPropertyValue objects. May be used for additional hole metadata or for profiles of test results for this hole, e.g. SPT vs depth/elevation.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiDataPropertyValue\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"agsiObservationColumn\": {"]
#[doc = "      \"description\": \"Array of embedded agsiObservationColumn objects which are typically used to represent geology within the hole, but can also be used for other data. \","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiObservationColumn\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"date\": {"]
#[doc = "      \"description\": \"Date of exploration. Recommend using start date for holes that take more than one day.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date\","]
#[doc = "      \"example\": \"2018-05-23\""]
#[doc = "    },"]
#[doc = "    \"holeID\": {"]
#[doc = "      \"description\": \"Identifier that is unique across the project for exploratory holes. Not necessarily the same as the original hole ID (see holeName). If used, identifiers for holeID should be unique within the AGSi file. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"A/BH01\""]
#[doc = "    },"]
#[doc = "    \"holeName\": {"]
#[doc = "      \"description\": \"Current name or ID of the exploratory hole for general use.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"BH01\""]
#[doc = "    },"]
#[doc = "    \"holeType\": {"]
#[doc = "      \"description\": \"Type of exploratory hole. Recommend using code from AGS format ABBR code list, e.g. CP+RC, with project specific codes defined using agsProjectCode. Alternatively, short description may be provided, e.g. cable percussion borehole with rotary follow on.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"CP+RC\""]
#[doc = "    },"]
#[doc = "    \"holeUUID\": {"]
#[doc = "      \"description\": \"Universal/global unique identifier (UUID) for the hole. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity. Other attributes should be used for IDs specific to the producer and/or client (see below).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"523ad9ed-4f75-4a55-b251-c566a8b998bd\""]
#[doc = "    },"]
#[doc = "    \"profileCoordinates\": {"]
#[doc = "      \"description\": \"Coordinates of the line of the exploratory hole (3D, including elevation), i.e. top, bottom and intermediate changes in direction if required. Input as ordered list of coordinate tuples starting at the top. Used for holes that are not vertical, or not straight. May be used for straight vertical holes as alternative to topCoordinate and verticalHoleDepth.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/coordinateTuple\""]
#[doc = "      },"]
#[doc = "      \"example\": ["]
#[doc = "        ["]
#[doc = "          1275.5,"]
#[doc = "          2195.0,"]
#[doc = "          15.25"]
#[doc = "        ],"]
#[doc = "        ["]
#[doc = "          1275.5,"]
#[doc = "          2195.0,"]
#[doc = "          -9.75"]
#[doc = "        ]"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Original name on logs: BH1\""]
#[doc = "    },"]
#[doc = "    \"topCoordinate\": {"]
#[doc = "      \"description\": \"Coordinates of the top of the exploratory hole (3D, including elevation) as a coordinate tuple.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"$ref\": \"#/$defs/coordinateTuple\","]
#[doc = "      \"example\": ["]
#[doc = "        1275.5,"]
#[doc = "        2195.0,"]
#[doc = "        15.25"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"verticalHoleDepth\": {"]
#[doc = "      \"description\": \"Final depth of exploratory hole for vertical holes only. For non-vertical or non-straight holes use profileCoordinates instead.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 25"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum AgsiObservationExpHole {
    Variant0(AgsiObservationExpHoleVariant0),
    Variant1(AgsiObservationExpHoleVariant1),
}
impl ::std::convert::From<&Self> for AgsiObservationExpHole {
    fn from(value: &AgsiObservationExpHole) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<AgsiObservationExpHoleVariant0> for AgsiObservationExpHole {
    fn from(value: AgsiObservationExpHoleVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<AgsiObservationExpHoleVariant1> for AgsiObservationExpHole {
    fn from(value: AgsiObservationExpHoleVariant1) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`AgsiObservationExpHoleVariant0`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"agsiDataPropertyFromFile\": {"]
#[doc = "          \"description\": \"An embedded agsiDataPropertyFromFile object, which may be used to reference an external supporting data file.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"$ref\": \"#/$defs/agsiDataPropertyFromFile\""]
#[doc = "        },"]
#[doc = "        \"agsiDataPropertyValue\": {"]
#[doc = "          \"description\": \"Array of embedded agsiDataPropertyValue objects. May be used for additional hole metadata or for profiles of test results for this hole, e.g. SPT vs depth/elevation.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/agsiDataPropertyValue\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"agsiObservationColumn\": {"]
#[doc = "          \"description\": \"Array of embedded agsiObservationColumn objects which are typically used to represent geology within the hole, but can also be used for other data. \","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/agsiObservationColumn\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"date\": {"]
#[doc = "          \"description\": \"Date of exploration. Recommend using start date for holes that take more than one day.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"date\","]
#[doc = "          \"example\": \"2018-05-23\""]
#[doc = "        },"]
#[doc = "        \"holeID\": {"]
#[doc = "          \"description\": \"Identifier that is unique across the project for exploratory holes. Not necessarily the same as the original hole ID (see holeName). If used, identifiers for holeID should be unique within the AGSi file. \","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"example\": \"A/BH01\""]
#[doc = "        },"]
#[doc = "        \"holeName\": {"]
#[doc = "          \"description\": \"Current name or ID of the exploratory hole for general use.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"example\": \"BH01\""]
#[doc = "        },"]
#[doc = "        \"holeType\": {"]
#[doc = "          \"description\": \"Type of exploratory hole. Recommend using code from AGS format ABBR code list, e.g. CP+RC, with project specific codes defined using agsProjectCode. Alternatively, short description may be provided, e.g. cable percussion borehole with rotary follow on.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"example\": \"CP+RC\""]
#[doc = "        },"]
#[doc = "        \"holeUUID\": {"]
#[doc = "          \"description\": \"Universal/global unique identifier (UUID) for the hole. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity. Other attributes should be used for IDs specific to the producer and/or client (see below).\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"example\": \"523ad9ed-4f75-4a55-b251-c566a8b998bd\""]
#[doc = "        },"]
#[doc = "        \"profileCoordinates\": {"]
#[doc = "          \"description\": \"Coordinates of the line of the exploratory hole (3D, including elevation), i.e. top, bottom and intermediate changes in direction if required. Input as ordered list of coordinate tuples starting at the top. Used for holes that are not vertical, or not straight. May be used for straight vertical holes as alternative to topCoordinate and verticalHoleDepth.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/coordinateTuple\""]
#[doc = "          },"]
#[doc = "          \"example\": ["]
#[doc = "            ["]
#[doc = "              1275.5,"]
#[doc = "              2195.0,"]
#[doc = "              15.25"]
#[doc = "            ],"]
#[doc = "            ["]
#[doc = "              1275.5,"]
#[doc = "              2195.0,"]
#[doc = "              -9.75"]
#[doc = "            ]"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"remarks\": {"]
#[doc = "          \"description\": \"Additional remarks, if required.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"example\": \"Original name on logs: BH1\""]
#[doc = "        },"]
#[doc = "        \"topCoordinate\": {"]
#[doc = "          \"description\": \"Coordinates of the top of the exploratory hole (3D, including elevation) as a coordinate tuple.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"$ref\": \"#/$defs/coordinateTuple\","]
#[doc = "          \"example\": ["]
#[doc = "            1275.5,"]
#[doc = "            2195.0,"]
#[doc = "            15.25"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"verticalHoleDepth\": {"]
#[doc = "          \"description\": \"Final depth of exploratory hole for vertical holes only. For non-vertical or non-straight holes use profileCoordinates instead.\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"example\": 25"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"holeID\","]
#[doc = "        \"topCoordinate\","]
#[doc = "        \"verticalHoleDepth\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"required\": ["]
#[doc = "          \"holeID\","]
#[doc = "          \"profileCoordinates\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(deny_unknown_fields)]
pub enum AgsiObservationExpHoleVariant0 {}
impl ::std::convert::From<&Self> for AgsiObservationExpHoleVariant0 {
    fn from(value: &AgsiObservationExpHoleVariant0) -> Self {
        value.clone()
    }
}
#[doc = "`AgsiObservationExpHoleVariant1`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"agsiDataPropertyFromFile\": {"]
#[doc = "          \"description\": \"An embedded agsiDataPropertyFromFile object, which may be used to reference an external supporting data file.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"$ref\": \"#/$defs/agsiDataPropertyFromFile\""]
#[doc = "        },"]
#[doc = "        \"agsiDataPropertyValue\": {"]
#[doc = "          \"description\": \"Array of embedded agsiDataPropertyValue objects. May be used for additional hole metadata or for profiles of test results for this hole, e.g. SPT vs depth/elevation.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/agsiDataPropertyValue\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"agsiObservationColumn\": {"]
#[doc = "          \"description\": \"Array of embedded agsiObservationColumn objects which are typically used to represent geology within the hole, but can also be used for other data. \","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/agsiObservationColumn\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"date\": {"]
#[doc = "          \"description\": \"Date of exploration. Recommend using start date for holes that take more than one day.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"date\","]
#[doc = "          \"example\": \"2018-05-23\""]
#[doc = "        },"]
#[doc = "        \"holeID\": {"]
#[doc = "          \"description\": \"Identifier that is unique across the project for exploratory holes. Not necessarily the same as the original hole ID (see holeName). If used, identifiers for holeID should be unique within the AGSi file. \","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"example\": \"A/BH01\""]
#[doc = "        },"]
#[doc = "        \"holeName\": {"]
#[doc = "          \"description\": \"Current name or ID of the exploratory hole for general use.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"example\": \"BH01\""]
#[doc = "        },"]
#[doc = "        \"holeType\": {"]
#[doc = "          \"description\": \"Type of exploratory hole. Recommend using code from AGS format ABBR code list, e.g. CP+RC, with project specific codes defined using agsProjectCode. Alternatively, short description may be provided, e.g. cable percussion borehole with rotary follow on.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"example\": \"CP+RC\""]
#[doc = "        },"]
#[doc = "        \"holeUUID\": {"]
#[doc = "          \"description\": \"Universal/global unique identifier (UUID) for the hole. This is optional and is not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity. Other attributes should be used for IDs specific to the producer and/or client (see below).\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"example\": \"523ad9ed-4f75-4a55-b251-c566a8b998bd\""]
#[doc = "        },"]
#[doc = "        \"profileCoordinates\": {"]
#[doc = "          \"description\": \"Coordinates of the line of the exploratory hole (3D, including elevation), i.e. top, bottom and intermediate changes in direction if required. Input as ordered list of coordinate tuples starting at the top. Used for holes that are not vertical, or not straight. May be used for straight vertical holes as alternative to topCoordinate and verticalHoleDepth.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/coordinateTuple\""]
#[doc = "          },"]
#[doc = "          \"example\": ["]
#[doc = "            ["]
#[doc = "              1275.5,"]
#[doc = "              2195.0,"]
#[doc = "              15.25"]
#[doc = "            ],"]
#[doc = "            ["]
#[doc = "              1275.5,"]
#[doc = "              2195.0,"]
#[doc = "              -9.75"]
#[doc = "            ]"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"remarks\": {"]
#[doc = "          \"description\": \"Additional remarks, if required.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"example\": \"Original name on logs: BH1\""]
#[doc = "        },"]
#[doc = "        \"topCoordinate\": {"]
#[doc = "          \"description\": \"Coordinates of the top of the exploratory hole (3D, including elevation) as a coordinate tuple.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"$ref\": \"#/$defs/coordinateTuple\","]
#[doc = "          \"example\": ["]
#[doc = "            1275.5,"]
#[doc = "            2195.0,"]
#[doc = "            15.25"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"verticalHoleDepth\": {"]
#[doc = "          \"description\": \"Final depth of exploratory hole for vertical holes only. For non-vertical or non-straight holes use profileCoordinates instead.\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"example\": 25"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"holeID\","]
#[doc = "        \"profileCoordinates\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"required\": ["]
#[doc = "          \"holeID\","]
#[doc = "          \"topCoordinate\","]
#[doc = "          \"verticalHoleDepth\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(deny_unknown_fields)]
pub enum AgsiObservationExpHoleVariant1 {}
impl ::std::convert::From<&Self> for AgsiObservationExpHoleVariant1 {
    fn from(value: &AgsiObservationExpHoleVariant1) -> Self {
        value.clone()
    }
}
#[doc = "An agsiObservationPoint is an observation related to a single geographic point, which may be defined in 2D or 3D space as required. Observations can be text or numeric value(s). Alternatively, a single agsiObservationPoint object can be used to provide attributed data using embedded agsiDataPropertyValue objects. For observations that relate to a line, area, volume or series of points, use agsiObservationShape. For GI exploratory holes and their data, use agsiObservationExpHole."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An agsiObservationPoint is an observation related to a single geographic point, which may be defined in 2D or 3D space as required. Observations can be text or numeric value(s). Alternatively, a single agsiObservationPoint object can be used to provide attributed data using embedded agsiDataPropertyValue objects. For observations that relate to a line, area, volume or series of points, use agsiObservationShape. For GI exploratory holes and their data, use agsiObservationExpHole.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"coordinate\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsiDataPropertyFromFile\": {"]
#[doc = "      \"description\": \"An embedded agsiDataPropertyFromFile object, which may be used to reference to an external supporting data file.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiDataPropertyFromFile\""]
#[doc = "    },"]
#[doc = "    \"agsiDataPropertyValue\": {"]
#[doc = "      \"description\": \"Array of embedded agsiDataPropertyValue objects. May be used to provide attributed data for this observation (numeric and/or text).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiDataPropertyValue\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"coordinate\": {"]
#[doc = "      \"description\": \"Coordinates of the location of the observation point, as a coordinate tuple. Coordinates provided may be 2D (x,y) or 3D (x,y,z).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"$ref\": \"#/$defs/coordinateTuple\","]
#[doc = "      \"example\": ["]
#[doc = "        1280.0,"]
#[doc = "        2195.0"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"date\": {"]
#[doc = "      \"description\": \"Date of observation, if applicable.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date\","]
#[doc = "      \"example\": \"2018-05-18\""]
#[doc = "    },"]
#[doc = "    \"madeBy\": {"]
#[doc = "      \"description\": \"Name and/or organisation of person(s) making the observation, if applicable.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"J Smith (ABC Consultants)\""]
#[doc = "    },"]
#[doc = "    \"observationID\": {"]
#[doc = "      \"description\": \"Identifier for this observation. May be local to this file or a UUID as required/specified. This is optional and not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it. If used, identifiers for observationID should be unique at least within each observation set (agsiObservationSet object), and preferably unique within the AGSi file. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"GIHole/A/Obs/001\""]
#[doc = "    },"]
#[doc = "    \"observationNature\": {"]
#[doc = "      \"description\": \"Description of the nature of the observation (use observationText for the observation itself).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"GI field notes\""]
#[doc = "    },"]
#[doc = "    \"observationText\": {"]
#[doc = "      \"description\": \"Description of the observation as text. Not required if agsiDataPropertyValue used to provide attributed data.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Original proposed location of BH01.  Hole moved due to concrete obstruction encountered at this location. \""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsiObservationPoint {
    #[doc = "An embedded agsiDataPropertyFromFile object, which may be used to reference to an external supporting data file."]
    #[serde(
        rename = "agsiDataPropertyFromFile",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agsi_data_property_from_file: ::std::option::Option<AgsiDataPropertyFromFile>,
    #[doc = "Array of embedded agsiDataPropertyValue objects. May be used to provide attributed data for this observation (numeric and/or text)."]
    #[serde(
        rename = "agsiDataPropertyValue",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_data_property_value: ::std::vec::Vec<AgsiDataPropertyValue>,
    #[doc = "Coordinates of the location of the observation point, as a coordinate tuple. Coordinates provided may be 2D (x,y) or 3D (x,y,z)."]
    pub coordinate: CoordinateTuple,
    #[doc = "Date of observation, if applicable."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::chrono::naive::NaiveDate>,
    #[doc = "Name and/or organisation of person(s) making the observation, if applicable."]
    #[serde(
        rename = "madeBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub made_by: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for this observation. May be local to this file or a UUID as required/specified. This is optional and not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it. If used, identifiers for observationID should be unique at least within each observation set (agsiObservationSet object), and preferably unique within the AGSi file. "]
    #[serde(
        rename = "observationID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub observation_id: ::std::option::Option<::std::string::String>,
    #[doc = "Description of the nature of the observation (use observationText for the observation itself)."]
    #[serde(
        rename = "observationNature",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub observation_nature: ::std::option::Option<::std::string::String>,
    #[doc = "Description of the observation as text. Not required if agsiDataPropertyValue used to provide attributed data."]
    #[serde(
        rename = "observationText",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub observation_text: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsiObservationPoint> for AgsiObservationPoint {
    fn from(value: &AgsiObservationPoint) -> Self {
        value.clone()
    }
}
impl AgsiObservationPoint {
    pub fn builder() -> builder::AgsiObservationPoint {
        Default::default()
    }
}
#[doc = "An agsiObservationSet object is a user defined set of observations. The individual observations are included as embedded objects agsiObservationPoint, agsiObservationShape and agsiObservationExpHole. For exploratory holes, a set may typically correspond to the holes from a particular ground investigation. For other types of observations, users/specifiers may decide how best to group the observations."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An agsiObservationSet object is a user defined set of observations. The individual observations are included as embedded objects agsiObservationPoint, agsiObservationShape and agsiObservationExpHole. For exploratory holes, a set may typically correspond to the holes from a particular ground investigation. For other types of observations, users/specifiers may decide how best to group the observations.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsiDataPropertyFromFile\": {"]
#[doc = "      \"description\": \"An embedded agsiDataPropertyFromFile object, which may be used to reference to an external supporting data file.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiDataPropertyFromFile\""]
#[doc = "    },"]
#[doc = "    \"agsiGeometryFromFile\": {"]
#[doc = "      \"description\": \"An embedded Geometry object defining an appropriate geometry for this set of observations, which will be a reference to an external file. Optional, and should only be used for geometry appropriate to the entire set. Geometry for individual observations should be defined in the embedded child observation objects.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "    },"]
#[doc = "    \"agsiObservationExpHole\": {"]
#[doc = "      \"description\": \"Array of embedded agsiObservationExpHole objects.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiObservationExpHole\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"agsiObservationPoint\": {"]
#[doc = "      \"description\": \"Array of embedded agsiObservationPoint objects.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiObservationPoint\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"agsiObservationShape\": {"]
#[doc = "      \"description\": \"Array of embedded agsiObservationShape objects.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiObservationShape\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Short description of the set of observations defined here.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"2018 GI Package A\""]
#[doc = "    },"]
#[doc = "    \"documentSetID\": {"]
#[doc = "      \"description\": \"Reference to report(s) relating to this set of observations, details of which should be provided in agsProjectDocumentSet\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"ExampleDocSetID\""]
#[doc = "    },"]
#[doc = "    \"investigationID\": {"]
#[doc = "      \"description\": \"For a set that represents a GI, reference to the identifier for the corresponding agsProjectInvestigation object, if used.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"GIPackageA\""]
#[doc = "    },"]
#[doc = "    \"observationSetID\": {"]
#[doc = "      \"description\": \"Identifier for this set of observations. May be local to this file or a UUID as required/specified. This is optional and not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it.  If used, identifiers for observationSetID should be unique within the AGSi file. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Obs/GIHolesA\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsiObservationSet {
    #[doc = "An embedded agsiDataPropertyFromFile object, which may be used to reference to an external supporting data file."]
    #[serde(
        rename = "agsiDataPropertyFromFile",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agsi_data_property_from_file: ::std::option::Option<AgsiDataPropertyFromFile>,
    #[doc = "An embedded Geometry object defining an appropriate geometry for this set of observations, which will be a reference to an external file. Optional, and should only be used for geometry appropriate to the entire set. Geometry for individual observations should be defined in the embedded child observation objects."]
    #[serde(
        rename = "agsiGeometryFromFile",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agsi_geometry_from_file: ::std::option::Option<AgsiGeometryFromFile>,
    #[doc = "Array of embedded agsiObservationExpHole objects."]
    #[serde(
        rename = "agsiObservationExpHole",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_observation_exp_hole: ::std::vec::Vec<AgsiObservationExpHole>,
    #[doc = "Array of embedded agsiObservationPoint objects."]
    #[serde(
        rename = "agsiObservationPoint",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_observation_point: ::std::vec::Vec<AgsiObservationPoint>,
    #[doc = "Array of embedded agsiObservationShape objects."]
    #[serde(
        rename = "agsiObservationShape",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_observation_shape: ::std::vec::Vec<AgsiObservationShape>,
    #[doc = "Short description of the set of observations defined here."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Reference to report(s) relating to this set of observations, details of which should be provided in agsProjectDocumentSet"]
    #[serde(
        rename = "documentSetID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub document_set_id: ::std::option::Option<::std::string::String>,
    #[doc = "For a set that represents a GI, reference to the identifier for the corresponding agsProjectInvestigation object, if used."]
    #[serde(
        rename = "investigationID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub investigation_id: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for this set of observations. May be local to this file or a UUID as required/specified. This is optional and not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it.  If used, identifiers for observationSetID should be unique within the AGSi file. "]
    #[serde(
        rename = "observationSetID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub observation_set_id: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsiObservationSet> for AgsiObservationSet {
    fn from(value: &AgsiObservationSet) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AgsiObservationSet {
    fn default() -> Self {
        Self {
            agsi_data_property_from_file: Default::default(),
            agsi_geometry_from_file: Default::default(),
            agsi_observation_exp_hole: Default::default(),
            agsi_observation_point: Default::default(),
            agsi_observation_shape: Default::default(),
            description: Default::default(),
            document_set_id: Default::default(),
            investigation_id: Default::default(),
            observation_set_id: Default::default(),
            remarks: Default::default(),
        }
    }
}
impl AgsiObservationSet {
    pub fn builder() -> builder::AgsiObservationSet {
        Default::default()
    }
}
#[doc = "An agsiObservationShape is an observation related to a geographic shape, e.g. line, area, volume or series of points, defined in 2D or 3D space as required. Observations can be text or numeric value(s). Alternatively, a single agsiObservationPoint object can be used to provide attributed data using embedded agsiDataPropertyValue objects. For observations that relate to a line, area, volume or series of points, use agsiObservationShape. For GI exploratory holes and their data, use agsiObservationExpHole."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An agsiObservationShape is an observation related to a geographic shape, e.g. line, area, volume or series of points, defined in 2D or 3D space as required. Observations can be text or numeric value(s). Alternatively, a single agsiObservationPoint object can be used to provide attributed data using embedded agsiDataPropertyValue objects. For observations that relate to a line, area, volume or series of points, use agsiObservationShape. For GI exploratory holes and their data, use agsiObservationExpHole.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agsiGeometryFromFile\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agsiDataPropertyFromFile\": {"]
#[doc = "      \"description\": \"An embedded agsiDataPropertyFromFile object, which may be used to reference to an external supporting data file.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiDataPropertyFromFile\""]
#[doc = "    },"]
#[doc = "    \"agsiDataPropertyValue\": {"]
#[doc = "      \"description\": \"Array of embedded agsiDataPropertyValue objects. May be used to provide attributed data for this observation (numeric values and/or text).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/agsiDataPropertyValue\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"agsiGeometryFromFile\": {"]
#[doc = "      \"description\": \"An embedded Geometry object defining the geometry for this observation, or the location of the observation, which will be a reference to an external file. The geometry may comprise: line, area, volume (or sets therefore), or a set of points.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/agsiGeometryFromFile\""]
#[doc = "    },"]
#[doc = "    \"date\": {"]
#[doc = "      \"description\": \"Date of observation, if applicable.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date\","]
#[doc = "      \"example\": \"2018-03-13\""]
#[doc = "    },"]
#[doc = "    \"madeBy\": {"]
#[doc = "      \"description\": \"Name and/or organisation of person(s) making the observation, if applicable.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"S Jones (ABC Consultants)\""]
#[doc = "    },"]
#[doc = "    \"observationID\": {"]
#[doc = "      \"description\": \"Identifier for this observation. May be local to this file or a UUID as required/specified. This is optional and not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it. If used, identifiers for observationID should be unique at least within each observation set (agsiObservationSet object), and preferably unique within the AGSi file. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"FieldGeology/Obs/001\""]
#[doc = "    },"]
#[doc = "    \"observationNature\": {"]
#[doc = "      \"description\": \"Description of the nature of the observation (use observationText for the observation itself).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Geological field survey of visible outcrop\""]
#[doc = "    },"]
#[doc = "    \"observationText\": {"]
#[doc = "      \"description\": \"Description of the observation as text. Not required if agsiDataPropertyValue used to provide attributed data.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Surveyed boundary of top of Gotham Clay, in side of exposed cutting.\""]
#[doc = "    },"]
#[doc = "    \"remarks\": {"]
#[doc = "      \"description\": \"Additional remarks, if required.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Some remarks if required\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgsiObservationShape {
    #[doc = "An embedded agsiDataPropertyFromFile object, which may be used to reference to an external supporting data file."]
    #[serde(
        rename = "agsiDataPropertyFromFile",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agsi_data_property_from_file: ::std::option::Option<AgsiDataPropertyFromFile>,
    #[doc = "Array of embedded agsiDataPropertyValue objects. May be used to provide attributed data for this observation (numeric values and/or text)."]
    #[serde(
        rename = "agsiDataPropertyValue",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agsi_data_property_value: ::std::vec::Vec<AgsiDataPropertyValue>,
    #[doc = "An embedded Geometry object defining the geometry for this observation, or the location of the observation, which will be a reference to an external file. The geometry may comprise: line, area, volume (or sets therefore), or a set of points."]
    #[serde(rename = "agsiGeometryFromFile")]
    pub agsi_geometry_from_file: AgsiGeometryFromFile,
    #[doc = "Date of observation, if applicable."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::chrono::naive::NaiveDate>,
    #[doc = "Name and/or organisation of person(s) making the observation, if applicable."]
    #[serde(
        rename = "madeBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub made_by: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for this observation. May be local to this file or a UUID as required/specified. This is optional and not referenced anywhere else in the schema, but it may be beneficial to include this to help with data control and integrity, and some software/applications may require it. If used, identifiers for observationID should be unique at least within each observation set (agsiObservationSet object), and preferably unique within the AGSi file. "]
    #[serde(
        rename = "observationID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub observation_id: ::std::option::Option<::std::string::String>,
    #[doc = "Description of the nature of the observation (use observationText for the observation itself)."]
    #[serde(
        rename = "observationNature",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub observation_nature: ::std::option::Option<::std::string::String>,
    #[doc = "Description of the observation as text. Not required if agsiDataPropertyValue used to provide attributed data."]
    #[serde(
        rename = "observationText",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub observation_text: ::std::option::Option<::std::string::String>,
    #[doc = "Additional remarks, if required."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remarks: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AgsiObservationShape> for AgsiObservationShape {
    fn from(value: &AgsiObservationShape) -> Self {
        value.clone()
    }
}
impl AgsiObservationShape {
    pub fn builder() -> builder::AgsiObservationShape {
        Default::default()
    }
}
#[doc = "Sub-schema for a coordinate tuple, referenced from other objects"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sub-schema for a coordinate tuple, referenced from other objects\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"number\""]
#[doc = "  },"]
#[doc = "  \"maxItems\": 3,"]
#[doc = "  \"minItems\": 2"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CoordinateTuple(pub ::std::vec::Vec<f64>);
impl ::std::ops::Deref for CoordinateTuple {
    type Target = ::std::vec::Vec<f64>;
    fn deref(&self) -> &::std::vec::Vec<f64> {
        &self.0
    }
}
impl ::std::convert::From<CoordinateTuple> for ::std::vec::Vec<f64> {
    fn from(value: CoordinateTuple) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoordinateTuple> for CoordinateTuple {
    fn from(value: &CoordinateTuple) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<f64>> for CoordinateTuple {
    fn from(value: ::std::vec::Vec<f64>) -> Self {
        Self(value)
    }
}
#[doc = "Sub-schema for a list of coordinate tuples, referenced from other objects"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sub-schema for a list of coordinate tuples, referenced from other objects\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"array\","]
#[doc = "    \"items\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"maxItems\": 2,"]
#[doc = "    \"minItems\": 2"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ValueProfile(pub ::std::vec::Vec<[f64; 2usize]>);
impl ::std::ops::Deref for ValueProfile {
    type Target = ::std::vec::Vec<[f64; 2usize]>;
    fn deref(&self) -> &::std::vec::Vec<[f64; 2usize]> {
        &self.0
    }
}
impl ::std::convert::From<ValueProfile> for ::std::vec::Vec<[f64; 2usize]> {
    fn from(value: ValueProfile) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ValueProfile> for ValueProfile {
    fn from(value: &ValueProfile) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<[f64; 2usize]>> for ValueProfile {
    fn from(value: ::std::vec::Vec<[f64; 2usize]>) -> Self {
        Self(value)
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AgSiV101 {
        ags_file: ::std::result::Result<super::AgsFile, ::std::string::String>,
        ags_project:
            ::std::result::Result<::std::option::Option<super::AgsProject>, ::std::string::String>,
        ags_schema: ::std::result::Result<super::AgsSchema, ::std::string::String>,
        agsi_model: ::std::result::Result<::std::vec::Vec<super::AgsiModel>, ::std::string::String>,
    }
    impl ::std::default::Default for AgSiV101 {
        fn default() -> Self {
            Self {
                ags_file: Err("no value supplied for ags_file".to_string()),
                ags_project: Ok(Default::default()),
                ags_schema: Err("no value supplied for ags_schema".to_string()),
                agsi_model: Ok(Default::default()),
            }
        }
    }
    impl AgSiV101 {
        pub fn ags_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsFile>,
            T::Error: ::std::fmt::Display,
        {
            self.ags_file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ags_file: {}", e));
            self
        }
        pub fn ags_project<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsProject>>,
            T::Error: ::std::fmt::Display,
        {
            self.ags_project = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ags_project: {}", e));
            self
        }
        pub fn ags_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsSchema>,
            T::Error: ::std::fmt::Display,
        {
            self.ags_schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ags_schema: {}", e));
            self
        }
        pub fn agsi_model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiModel>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agsi_model: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgSiV101> for super::AgSiV101 {
        type Error = super::error::ConversionError;
        fn try_from(value: AgSiV101) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ags_file: value.ags_file?,
                ags_project: value.ags_project?,
                ags_schema: value.ags_schema?,
                agsi_model: value.agsi_model?,
            })
        }
    }
    impl ::std::convert::From<super::AgSiV101> for AgSiV101 {
        fn from(value: super::AgSiV101) -> Self {
            Self {
                ags_file: Ok(value.ags_file),
                ags_project: Ok(value.ags_project),
                ags_schema: Ok(value.ags_schema),
                agsi_model: Ok(value.agsi_model),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsFile {
        approved_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        checked_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        date: ::std::result::Result<
            ::std::option::Option<::chrono::naive::NaiveDate>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        file_uri: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        file_uuid: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        made_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        produced_by: ::std::result::Result<super::AgsFileProducedBy, ::std::string::String>,
        project_title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        reference: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        revision: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<super::AgsFileTitle, ::std::string::String>,
    }
    impl ::std::default::Default for AgsFile {
        fn default() -> Self {
            Self {
                approved_by: Ok(Default::default()),
                checked_by: Ok(Default::default()),
                date: Ok(Default::default()),
                description: Ok(Default::default()),
                file_uri: Ok(Default::default()),
                file_uuid: Ok(Default::default()),
                made_by: Ok(Default::default()),
                produced_by: Err("no value supplied for produced_by".to_string()),
                project_title: Ok(Default::default()),
                reference: Ok(Default::default()),
                remarks: Ok(Default::default()),
                revision: Ok(Default::default()),
                status: Ok(Default::default()),
                status_code: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl AgsFile {
        pub fn approved_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.approved_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for approved_by: {}", e));
            self
        }
        pub fn checked_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.checked_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for checked_by: {}", e));
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::chrono::naive::NaiveDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn file_uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_uri: {}", e));
            self
        }
        pub fn file_uuid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_uuid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_uuid: {}", e));
            self
        }
        pub fn made_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.made_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for made_by: {}", e));
            self
        }
        pub fn produced_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsFileProducedBy>,
            T::Error: ::std::fmt::Display,
        {
            self.produced_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for produced_by: {}", e));
            self
        }
        pub fn project_title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_title: {}", e));
            self
        }
        pub fn reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reference = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reference: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn revision<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.revision = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for revision: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn status_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status_code: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsFileTitle>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsFile> for super::AgsFile {
        type Error = super::error::ConversionError;
        fn try_from(value: AgsFile) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                approved_by: value.approved_by?,
                checked_by: value.checked_by?,
                date: value.date?,
                description: value.description?,
                file_uri: value.file_uri?,
                file_uuid: value.file_uuid?,
                made_by: value.made_by?,
                produced_by: value.produced_by?,
                project_title: value.project_title?,
                reference: value.reference?,
                remarks: value.remarks?,
                revision: value.revision?,
                status: value.status?,
                status_code: value.status_code?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::AgsFile> for AgsFile {
        fn from(value: super::AgsFile) -> Self {
            Self {
                approved_by: Ok(value.approved_by),
                checked_by: Ok(value.checked_by),
                date: Ok(value.date),
                description: Ok(value.description),
                file_uri: Ok(value.file_uri),
                file_uuid: Ok(value.file_uuid),
                made_by: Ok(value.made_by),
                produced_by: Ok(value.produced_by),
                project_title: Ok(value.project_title),
                reference: Ok(value.reference),
                remarks: Ok(value.remarks),
                revision: Ok(value.revision),
                status: Ok(value.status),
                status_code: Ok(value.status_code),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsProject {
        ags_project_code_set:
            ::std::result::Result<::std::vec::Vec<super::AgsProjectCodeSet>, ::std::string::String>,
        ags_project_coordinate_system: ::std::result::Result<
            ::std::vec::Vec<super::AgsProjectCoordinateSystem>,
            ::std::string::String,
        >,
        ags_project_document_set: ::std::result::Result<
            ::std::vec::Vec<super::AgsProjectDocumentSet>,
            ::std::string::String,
        >,
        ags_project_investigation: ::std::result::Result<
            ::std::vec::Vec<super::AgsProjectInvestigation>,
            ::std::string::String,
        >,
        brief_document_set_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        client: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        client_project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        parent_project_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        producer: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        producer_project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        producer_suppliers: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_country: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_name: ::std::result::Result<super::AgsProjectProjectName, ::std::string::String>,
        project_uuid: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        report_document_set_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ultimate_project_client: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ultimate_project_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsProject {
        fn default() -> Self {
            Self {
                ags_project_code_set: Ok(Default::default()),
                ags_project_coordinate_system: Ok(Default::default()),
                ags_project_document_set: Ok(Default::default()),
                ags_project_investigation: Ok(Default::default()),
                brief_document_set_id: Ok(Default::default()),
                client: Ok(Default::default()),
                client_project_id: Ok(Default::default()),
                description: Ok(Default::default()),
                parent_project_name: Ok(Default::default()),
                producer: Ok(Default::default()),
                producer_project_id: Ok(Default::default()),
                producer_suppliers: Ok(Default::default()),
                project_country: Ok(Default::default()),
                project_name: Err("no value supplied for project_name".to_string()),
                project_uuid: Ok(Default::default()),
                remarks: Ok(Default::default()),
                report_document_set_id: Ok(Default::default()),
                ultimate_project_client: Ok(Default::default()),
                ultimate_project_name: Ok(Default::default()),
            }
        }
    }
    impl AgsProject {
        pub fn ags_project_code_set<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsProjectCodeSet>>,
            T::Error: ::std::fmt::Display,
        {
            self.ags_project_code_set = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ags_project_code_set: {}",
                    e
                )
            });
            self
        }
        pub fn ags_project_coordinate_system<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsProjectCoordinateSystem>>,
            T::Error: ::std::fmt::Display,
        {
            self.ags_project_coordinate_system = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ags_project_coordinate_system: {}",
                    e
                )
            });
            self
        }
        pub fn ags_project_document_set<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsProjectDocumentSet>>,
            T::Error: ::std::fmt::Display,
        {
            self.ags_project_document_set = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ags_project_document_set: {}",
                    e
                )
            });
            self
        }
        pub fn ags_project_investigation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsProjectInvestigation>>,
            T::Error: ::std::fmt::Display,
        {
            self.ags_project_investigation = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ags_project_investigation: {}",
                    e
                )
            });
            self
        }
        pub fn brief_document_set_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.brief_document_set_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for brief_document_set_id: {}",
                    e
                )
            });
            self
        }
        pub fn client<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client: {}", e));
            self
        }
        pub fn client_project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_project_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for client_project_id: {}",
                    e
                )
            });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn parent_project_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent_project_name = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for parent_project_name: {}",
                    e
                )
            });
            self
        }
        pub fn producer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.producer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for producer: {}", e));
            self
        }
        pub fn producer_project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.producer_project_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for producer_project_id: {}",
                    e
                )
            });
            self
        }
        pub fn producer_suppliers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.producer_suppliers = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for producer_suppliers: {}",
                    e
                )
            });
            self
        }
        pub fn project_country<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_country = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_country: {}", e));
            self
        }
        pub fn project_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsProjectProjectName>,
            T::Error: ::std::fmt::Display,
        {
            self.project_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_name: {}", e));
            self
        }
        pub fn project_uuid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_uuid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_uuid: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn report_document_set_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.report_document_set_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for report_document_set_id: {}",
                    e
                )
            });
            self
        }
        pub fn ultimate_project_client<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ultimate_project_client = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ultimate_project_client: {}",
                    e
                )
            });
            self
        }
        pub fn ultimate_project_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ultimate_project_name = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ultimate_project_name: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AgsProject> for super::AgsProject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsProject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ags_project_code_set: value.ags_project_code_set?,
                ags_project_coordinate_system: value.ags_project_coordinate_system?,
                ags_project_document_set: value.ags_project_document_set?,
                ags_project_investigation: value.ags_project_investigation?,
                brief_document_set_id: value.brief_document_set_id?,
                client: value.client?,
                client_project_id: value.client_project_id?,
                description: value.description?,
                parent_project_name: value.parent_project_name?,
                producer: value.producer?,
                producer_project_id: value.producer_project_id?,
                producer_suppliers: value.producer_suppliers?,
                project_country: value.project_country?,
                project_name: value.project_name?,
                project_uuid: value.project_uuid?,
                remarks: value.remarks?,
                report_document_set_id: value.report_document_set_id?,
                ultimate_project_client: value.ultimate_project_client?,
                ultimate_project_name: value.ultimate_project_name?,
            })
        }
    }
    impl ::std::convert::From<super::AgsProject> for AgsProject {
        fn from(value: super::AgsProject) -> Self {
            Self {
                ags_project_code_set: Ok(value.ags_project_code_set),
                ags_project_coordinate_system: Ok(value.ags_project_coordinate_system),
                ags_project_document_set: Ok(value.ags_project_document_set),
                ags_project_investigation: Ok(value.ags_project_investigation),
                brief_document_set_id: Ok(value.brief_document_set_id),
                client: Ok(value.client),
                client_project_id: Ok(value.client_project_id),
                description: Ok(value.description),
                parent_project_name: Ok(value.parent_project_name),
                producer: Ok(value.producer),
                producer_project_id: Ok(value.producer_project_id),
                producer_suppliers: Ok(value.producer_suppliers),
                project_country: Ok(value.project_country),
                project_name: Ok(value.project_name),
                project_uuid: Ok(value.project_uuid),
                remarks: Ok(value.remarks),
                report_document_set_id: Ok(value.report_document_set_id),
                ultimate_project_client: Ok(value.ultimate_project_client),
                ultimate_project_name: Ok(value.ultimate_project_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsProjectCode {
        code_id: ::std::result::Result<super::AgsProjectCodeCodeId, ::std::string::String>,
        description: ::std::result::Result<super::AgsProjectCodeDescription, ::std::string::String>,
        is_standard: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        units: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsProjectCode {
        fn default() -> Self {
            Self {
                code_id: Err("no value supplied for code_id".to_string()),
                description: Err("no value supplied for description".to_string()),
                is_standard: Ok(Default::default()),
                remarks: Ok(Default::default()),
                units: Ok(Default::default()),
            }
        }
    }
    impl AgsProjectCode {
        pub fn code_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsProjectCodeCodeId>,
            T::Error: ::std::fmt::Display,
        {
            self.code_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_id: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsProjectCodeDescription>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn is_standard<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_standard = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_standard: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for units: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsProjectCode> for super::AgsProjectCode {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsProjectCode,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_id: value.code_id?,
                description: value.description?,
                is_standard: value.is_standard?,
                remarks: value.remarks?,
                units: value.units?,
            })
        }
    }
    impl ::std::convert::From<super::AgsProjectCode> for AgsProjectCode {
        fn from(value: super::AgsProjectCode) -> Self {
            Self {
                code_id: Ok(value.code_id),
                description: Ok(value.description),
                is_standard: Ok(value.is_standard),
                remarks: Ok(value.remarks),
                units: Ok(value.units),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsProjectCodeSet {
        ags_project_code:
            ::std::result::Result<::std::vec::Vec<super::AgsProjectCode>, ::std::string::String>,
        code_set_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        concatenation_allow:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        concatenation_character: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        source_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        source_uri: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        used_by_attribute:
            ::std::result::Result<super::AgsProjectCodeSetUsedByAttribute, ::std::string::String>,
        used_by_object:
            ::std::result::Result<super::AgsProjectCodeSetUsedByObject, ::std::string::String>,
    }
    impl ::std::default::Default for AgsProjectCodeSet {
        fn default() -> Self {
            Self {
                ags_project_code: Ok(Default::default()),
                code_set_id: Ok(Default::default()),
                concatenation_allow: Ok(Default::default()),
                concatenation_character: Ok(Default::default()),
                description: Ok(Default::default()),
                remarks: Ok(Default::default()),
                source_description: Ok(Default::default()),
                source_uri: Ok(Default::default()),
                used_by_attribute: Err("no value supplied for used_by_attribute".to_string()),
                used_by_object: Err("no value supplied for used_by_object".to_string()),
            }
        }
    }
    impl AgsProjectCodeSet {
        pub fn ags_project_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsProjectCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.ags_project_code = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ags_project_code: {}",
                    e
                )
            });
            self
        }
        pub fn code_set_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.code_set_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_set_id: {}", e));
            self
        }
        pub fn concatenation_allow<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.concatenation_allow = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for concatenation_allow: {}",
                    e
                )
            });
            self
        }
        pub fn concatenation_character<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.concatenation_character = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for concatenation_character: {}",
                    e
                )
            });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn source_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.source_description = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for source_description: {}",
                    e
                )
            });
            self
        }
        pub fn source_uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.source_uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_uri: {}", e));
            self
        }
        pub fn used_by_attribute<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsProjectCodeSetUsedByAttribute>,
            T::Error: ::std::fmt::Display,
        {
            self.used_by_attribute = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for used_by_attribute: {}",
                    e
                )
            });
            self
        }
        pub fn used_by_object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsProjectCodeSetUsedByObject>,
            T::Error: ::std::fmt::Display,
        {
            self.used_by_object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for used_by_object: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsProjectCodeSet> for super::AgsProjectCodeSet {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsProjectCodeSet,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ags_project_code: value.ags_project_code?,
                code_set_id: value.code_set_id?,
                concatenation_allow: value.concatenation_allow?,
                concatenation_character: value.concatenation_character?,
                description: value.description?,
                remarks: value.remarks?,
                source_description: value.source_description?,
                source_uri: value.source_uri?,
                used_by_attribute: value.used_by_attribute?,
                used_by_object: value.used_by_object?,
            })
        }
    }
    impl ::std::convert::From<super::AgsProjectCodeSet> for AgsProjectCodeSet {
        fn from(value: super::AgsProjectCodeSet) -> Self {
            Self {
                ags_project_code: Ok(value.ags_project_code),
                code_set_id: Ok(value.code_set_id),
                concatenation_allow: Ok(value.concatenation_allow),
                concatenation_character: Ok(value.concatenation_character),
                description: Ok(value.description),
                remarks: Ok(value.remarks),
                source_description: Ok(value.source_description),
                source_uri: Ok(value.source_uri),
                used_by_attribute: Ok(value.used_by_attribute),
                used_by_object: Ok(value.used_by_object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsProjectCoordinateSystem {
        axis_name_x: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        axis_name_y: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        axis_name_z: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        axis_units_xy: ::std::result::Result<
            super::AgsProjectCoordinateSystemAxisUnitsXy,
            ::std::string::String,
        >,
        axis_units_z: ::std::result::Result<
            super::AgsProjectCoordinateSystemAxisUnitsZ,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        global_xy_system: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        global_z_system: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        system_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        system_name_xy: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        system_name_z: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        system_type: ::std::result::Result<
            ::std::option::Option<super::AgsProjectCoordinateSystemSystemType>,
            ::std::string::String,
        >,
        transform_shift_x: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        transform_shift_y: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        transform_shift_z: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        transform_xy_rotation:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        transform_xy_scale_factor:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for AgsProjectCoordinateSystem {
        fn default() -> Self {
            Self {
                axis_name_x: Ok(Default::default()),
                axis_name_y: Ok(Default::default()),
                axis_name_z: Ok(Default::default()),
                axis_units_xy: Err("no value supplied for axis_units_xy".to_string()),
                axis_units_z: Err("no value supplied for axis_units_z".to_string()),
                description: Ok(Default::default()),
                global_xy_system: Ok(Default::default()),
                global_z_system: Ok(Default::default()),
                remarks: Ok(Default::default()),
                system_id: Ok(Default::default()),
                system_name_xy: Ok(Default::default()),
                system_name_z: Ok(Default::default()),
                system_type: Ok(Default::default()),
                transform_shift_x: Ok(Default::default()),
                transform_shift_y: Ok(Default::default()),
                transform_shift_z: Ok(Default::default()),
                transform_xy_rotation: Ok(Default::default()),
                transform_xy_scale_factor: Ok(Default::default()),
            }
        }
    }
    impl AgsProjectCoordinateSystem {
        pub fn axis_name_x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.axis_name_x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for axis_name_x: {}", e));
            self
        }
        pub fn axis_name_y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.axis_name_y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for axis_name_y: {}", e));
            self
        }
        pub fn axis_name_z<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.axis_name_z = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for axis_name_z: {}", e));
            self
        }
        pub fn axis_units_xy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsProjectCoordinateSystemAxisUnitsXy>,
            T::Error: ::std::fmt::Display,
        {
            self.axis_units_xy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for axis_units_xy: {}", e));
            self
        }
        pub fn axis_units_z<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsProjectCoordinateSystemAxisUnitsZ>,
            T::Error: ::std::fmt::Display,
        {
            self.axis_units_z = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for axis_units_z: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn global_xy_system<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.global_xy_system = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for global_xy_system: {}",
                    e
                )
            });
            self
        }
        pub fn global_z_system<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.global_z_system = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for global_z_system: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn system_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for system_id: {}", e));
            self
        }
        pub fn system_name_xy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_name_xy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for system_name_xy: {}", e));
            self
        }
        pub fn system_name_z<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_name_z = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for system_name_z: {}", e));
            self
        }
        pub fn system_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AgsProjectCoordinateSystemSystemType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.system_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for system_type: {}", e));
            self
        }
        pub fn transform_shift_x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.transform_shift_x = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for transform_shift_x: {}",
                    e
                )
            });
            self
        }
        pub fn transform_shift_y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.transform_shift_y = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for transform_shift_y: {}",
                    e
                )
            });
            self
        }
        pub fn transform_shift_z<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.transform_shift_z = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for transform_shift_z: {}",
                    e
                )
            });
            self
        }
        pub fn transform_xy_rotation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.transform_xy_rotation = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for transform_xy_rotation: {}",
                    e
                )
            });
            self
        }
        pub fn transform_xy_scale_factor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.transform_xy_scale_factor = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for transform_xy_scale_factor: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AgsProjectCoordinateSystem> for super::AgsProjectCoordinateSystem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsProjectCoordinateSystem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                axis_name_x: value.axis_name_x?,
                axis_name_y: value.axis_name_y?,
                axis_name_z: value.axis_name_z?,
                axis_units_xy: value.axis_units_xy?,
                axis_units_z: value.axis_units_z?,
                description: value.description?,
                global_xy_system: value.global_xy_system?,
                global_z_system: value.global_z_system?,
                remarks: value.remarks?,
                system_id: value.system_id?,
                system_name_xy: value.system_name_xy?,
                system_name_z: value.system_name_z?,
                system_type: value.system_type?,
                transform_shift_x: value.transform_shift_x?,
                transform_shift_y: value.transform_shift_y?,
                transform_shift_z: value.transform_shift_z?,
                transform_xy_rotation: value.transform_xy_rotation?,
                transform_xy_scale_factor: value.transform_xy_scale_factor?,
            })
        }
    }
    impl ::std::convert::From<super::AgsProjectCoordinateSystem> for AgsProjectCoordinateSystem {
        fn from(value: super::AgsProjectCoordinateSystem) -> Self {
            Self {
                axis_name_x: Ok(value.axis_name_x),
                axis_name_y: Ok(value.axis_name_y),
                axis_name_z: Ok(value.axis_name_z),
                axis_units_xy: Ok(value.axis_units_xy),
                axis_units_z: Ok(value.axis_units_z),
                description: Ok(value.description),
                global_xy_system: Ok(value.global_xy_system),
                global_z_system: Ok(value.global_z_system),
                remarks: Ok(value.remarks),
                system_id: Ok(value.system_id),
                system_name_xy: Ok(value.system_name_xy),
                system_name_z: Ok(value.system_name_z),
                system_type: Ok(value.system_type),
                transform_shift_x: Ok(value.transform_shift_x),
                transform_shift_y: Ok(value.transform_shift_y),
                transform_shift_z: Ok(value.transform_shift_z),
                transform_xy_rotation: Ok(value.transform_xy_rotation),
                transform_xy_scale_factor: Ok(value.transform_xy_scale_factor),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsProjectDocument {
        author: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        client: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        date: ::std::result::Result<
            ::std::option::Option<::chrono::naive::NaiveDate>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        document_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        document_system_uri: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        document_uri: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        original_reference: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        revision: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        system_reference: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsProjectDocument {
        fn default() -> Self {
            Self {
                author: Ok(Default::default()),
                client: Ok(Default::default()),
                date: Ok(Default::default()),
                description: Ok(Default::default()),
                document_id: Ok(Default::default()),
                document_system_uri: Ok(Default::default()),
                document_uri: Ok(Default::default()),
                original_reference: Ok(Default::default()),
                remarks: Ok(Default::default()),
                revision: Ok(Default::default()),
                status: Ok(Default::default()),
                status_code: Ok(Default::default()),
                system_reference: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl AgsProjectDocument {
        pub fn author<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.author = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for author: {}", e));
            self
        }
        pub fn client<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client: {}", e));
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::chrono::naive::NaiveDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn document_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.document_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for document_id: {}", e));
            self
        }
        pub fn document_system_uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.document_system_uri = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for document_system_uri: {}",
                    e
                )
            });
            self
        }
        pub fn document_uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.document_uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for document_uri: {}", e));
            self
        }
        pub fn original_reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.original_reference = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for original_reference: {}",
                    e
                )
            });
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn revision<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.revision = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for revision: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn status_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status_code: {}", e));
            self
        }
        pub fn system_reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_reference = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for system_reference: {}",
                    e
                )
            });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsProjectDocument> for super::AgsProjectDocument {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsProjectDocument,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                author: value.author?,
                client: value.client?,
                date: value.date?,
                description: value.description?,
                document_id: value.document_id?,
                document_system_uri: value.document_system_uri?,
                document_uri: value.document_uri?,
                original_reference: value.original_reference?,
                remarks: value.remarks?,
                revision: value.revision?,
                status: value.status?,
                status_code: value.status_code?,
                system_reference: value.system_reference?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::AgsProjectDocument> for AgsProjectDocument {
        fn from(value: super::AgsProjectDocument) -> Self {
            Self {
                author: Ok(value.author),
                client: Ok(value.client),
                date: Ok(value.date),
                description: Ok(value.description),
                document_id: Ok(value.document_id),
                document_system_uri: Ok(value.document_system_uri),
                document_uri: Ok(value.document_uri),
                original_reference: Ok(value.original_reference),
                remarks: Ok(value.remarks),
                revision: Ok(value.revision),
                status: Ok(value.status),
                status_code: Ok(value.status_code),
                system_reference: Ok(value.system_reference),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsProjectDocumentSet {
        ags_project_document: ::std::result::Result<
            ::std::vec::Vec<super::AgsProjectDocument>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        document_set_id:
            ::std::result::Result<super::AgsProjectDocumentSetDocumentSetId, ::std::string::String>,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsProjectDocumentSet {
        fn default() -> Self {
            Self {
                ags_project_document: Ok(Default::default()),
                description: Ok(Default::default()),
                document_set_id: Err("no value supplied for document_set_id".to_string()),
                remarks: Ok(Default::default()),
            }
        }
    }
    impl AgsProjectDocumentSet {
        pub fn ags_project_document<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsProjectDocument>>,
            T::Error: ::std::fmt::Display,
        {
            self.ags_project_document = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ags_project_document: {}",
                    e
                )
            });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn document_set_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsProjectDocumentSetDocumentSetId>,
            T::Error: ::std::fmt::Display,
        {
            self.document_set_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for document_set_id: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsProjectDocumentSet> for super::AgsProjectDocumentSet {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsProjectDocumentSet,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ags_project_document: value.ags_project_document?,
                description: value.description?,
                document_set_id: value.document_set_id?,
                remarks: value.remarks?,
            })
        }
    }
    impl ::std::convert::From<super::AgsProjectDocumentSet> for AgsProjectDocumentSet {
        fn from(value: super::AgsProjectDocumentSet) -> Self {
            Self {
                ags_project_document: Ok(value.ags_project_document),
                description: Ok(value.description),
                document_set_id: Ok(value.document_set_id),
                remarks: Ok(value.remarks),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsProjectInvestigation {
        agsi_data_property_from_file: ::std::result::Result<
            ::std::vec::Vec<super::AgsiDataPropertyFromFile>,
            ::std::string::String,
        >,
        agsi_data_property_value: ::std::result::Result<
            ::std::vec::Vec<super::AgsiDataPropertyValue>,
            ::std::string::String,
        >,
        client: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        contractor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        data_document_set_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        engineer: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        fieldwork_date_start: ::std::result::Result<
            ::std::option::Option<::chrono::naive::NaiveDate>,
            ::std::string::String,
        >,
        investigation_id: ::std::result::Result<
            super::AgsProjectInvestigationInvestigationId,
            ::std::string::String,
        >,
        investigation_name: ::std::result::Result<
            super::AgsProjectInvestigationInvestigationName,
            ::std::string::String,
        >,
        location_coordinate_global: ::std::result::Result<
            ::std::option::Option<super::CoordinateTuple>,
            ::std::string::String,
        >,
        location_coordinate_project: ::std::result::Result<
            ::std::option::Option<super::CoordinateTuple>,
            ::std::string::String,
        >,
        location_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        parent_project_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        report_document_set_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scope_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        specification_document_set_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        subcontractors: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ultimate_project_client: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ultimate_project_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsProjectInvestigation {
        fn default() -> Self {
            Self {
                agsi_data_property_from_file: Ok(Default::default()),
                agsi_data_property_value: Ok(Default::default()),
                client: Ok(Default::default()),
                contractor: Ok(Default::default()),
                data_document_set_id: Ok(Default::default()),
                description: Ok(Default::default()),
                engineer: Ok(Default::default()),
                fieldwork_date_start: Ok(Default::default()),
                investigation_id: Err("no value supplied for investigation_id".to_string()),
                investigation_name: Err("no value supplied for investigation_name".to_string()),
                location_coordinate_global: Ok(Default::default()),
                location_coordinate_project: Ok(Default::default()),
                location_description: Ok(Default::default()),
                parent_project_name: Ok(Default::default()),
                remarks: Ok(Default::default()),
                report_document_set_id: Ok(Default::default()),
                scope_description: Ok(Default::default()),
                specification_document_set_id: Ok(Default::default()),
                subcontractors: Ok(Default::default()),
                ultimate_project_client: Ok(Default::default()),
                ultimate_project_name: Ok(Default::default()),
            }
        }
    }
    impl AgsProjectInvestigation {
        pub fn agsi_data_property_from_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiDataPropertyFromFile>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_data_property_from_file = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_data_property_from_file: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_data_property_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiDataPropertyValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_data_property_value = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_data_property_value: {}",
                    e
                )
            });
            self
        }
        pub fn client<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client: {}", e));
            self
        }
        pub fn contractor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.contractor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contractor: {}", e));
            self
        }
        pub fn data_document_set_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.data_document_set_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for data_document_set_id: {}",
                    e
                )
            });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn engineer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.engineer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for engineer: {}", e));
            self
        }
        pub fn fieldwork_date_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::chrono::naive::NaiveDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.fieldwork_date_start = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for fieldwork_date_start: {}",
                    e
                )
            });
            self
        }
        pub fn investigation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsProjectInvestigationInvestigationId>,
            T::Error: ::std::fmt::Display,
        {
            self.investigation_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for investigation_id: {}",
                    e
                )
            });
            self
        }
        pub fn investigation_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsProjectInvestigationInvestigationName>,
            T::Error: ::std::fmt::Display,
        {
            self.investigation_name = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for investigation_name: {}",
                    e
                )
            });
            self
        }
        pub fn location_coordinate_global<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CoordinateTuple>>,
            T::Error: ::std::fmt::Display,
        {
            self.location_coordinate_global = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for location_coordinate_global: {}",
                    e
                )
            });
            self
        }
        pub fn location_coordinate_project<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CoordinateTuple>>,
            T::Error: ::std::fmt::Display,
        {
            self.location_coordinate_project = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for location_coordinate_project: {}",
                    e
                )
            });
            self
        }
        pub fn location_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.location_description = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for location_description: {}",
                    e
                )
            });
            self
        }
        pub fn parent_project_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent_project_name = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for parent_project_name: {}",
                    e
                )
            });
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn report_document_set_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.report_document_set_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for report_document_set_id: {}",
                    e
                )
            });
            self
        }
        pub fn scope_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.scope_description = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for scope_description: {}",
                    e
                )
            });
            self
        }
        pub fn specification_document_set_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.specification_document_set_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for specification_document_set_id: {}",
                    e
                )
            });
            self
        }
        pub fn subcontractors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.subcontractors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subcontractors: {}", e));
            self
        }
        pub fn ultimate_project_client<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ultimate_project_client = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ultimate_project_client: {}",
                    e
                )
            });
            self
        }
        pub fn ultimate_project_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ultimate_project_name = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ultimate_project_name: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AgsProjectInvestigation> for super::AgsProjectInvestigation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsProjectInvestigation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agsi_data_property_from_file: value.agsi_data_property_from_file?,
                agsi_data_property_value: value.agsi_data_property_value?,
                client: value.client?,
                contractor: value.contractor?,
                data_document_set_id: value.data_document_set_id?,
                description: value.description?,
                engineer: value.engineer?,
                fieldwork_date_start: value.fieldwork_date_start?,
                investigation_id: value.investigation_id?,
                investigation_name: value.investigation_name?,
                location_coordinate_global: value.location_coordinate_global?,
                location_coordinate_project: value.location_coordinate_project?,
                location_description: value.location_description?,
                parent_project_name: value.parent_project_name?,
                remarks: value.remarks?,
                report_document_set_id: value.report_document_set_id?,
                scope_description: value.scope_description?,
                specification_document_set_id: value.specification_document_set_id?,
                subcontractors: value.subcontractors?,
                ultimate_project_client: value.ultimate_project_client?,
                ultimate_project_name: value.ultimate_project_name?,
            })
        }
    }
    impl ::std::convert::From<super::AgsProjectInvestigation> for AgsProjectInvestigation {
        fn from(value: super::AgsProjectInvestigation) -> Self {
            Self {
                agsi_data_property_from_file: Ok(value.agsi_data_property_from_file),
                agsi_data_property_value: Ok(value.agsi_data_property_value),
                client: Ok(value.client),
                contractor: Ok(value.contractor),
                data_document_set_id: Ok(value.data_document_set_id),
                description: Ok(value.description),
                engineer: Ok(value.engineer),
                fieldwork_date_start: Ok(value.fieldwork_date_start),
                investigation_id: Ok(value.investigation_id),
                investigation_name: Ok(value.investigation_name),
                location_coordinate_global: Ok(value.location_coordinate_global),
                location_coordinate_project: Ok(value.location_coordinate_project),
                location_description: Ok(value.location_description),
                parent_project_name: Ok(value.parent_project_name),
                remarks: Ok(value.remarks),
                report_document_set_id: Ok(value.report_document_set_id),
                scope_description: Ok(value.scope_description),
                specification_document_set_id: Ok(value.specification_document_set_id),
                subcontractors: Ok(value.subcontractors),
                ultimate_project_client: Ok(value.ultimate_project_client),
                ultimate_project_name: Ok(value.ultimate_project_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsSchema {
        link: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::AgsSchemaName, ::std::string::String>,
        version: ::std::result::Result<super::AgsSchemaVersion, ::std::string::String>,
    }
    impl ::std::default::Default for AgsSchema {
        fn default() -> Self {
            Self {
                link: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl AgsSchema {
        pub fn link<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.link = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsSchemaName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsSchemaVersion>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsSchema> for super::AgsSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                link: value.link?,
                name: value.name?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::AgsSchema> for AgsSchema {
        fn from(value: super::AgsSchema) -> Self {
            Self {
                link: Ok(value.link),
                name: Ok(value.name),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiDataParameterValue {
        case_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        code_id: ::std::result::Result<super::AgsiDataParameterValueCodeId, ::std::string::String>,
        data_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value_numeric: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value_profile: ::std::result::Result<
            ::std::option::Option<super::ValueProfile>,
            ::std::string::String,
        >,
        value_profile_ind_var_code_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value_text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsiDataParameterValue {
        fn default() -> Self {
            Self {
                case_id: Ok(Default::default()),
                code_id: Err("no value supplied for code_id".to_string()),
                data_id: Ok(Default::default()),
                remarks: Ok(Default::default()),
                value_numeric: Ok(Default::default()),
                value_profile: Ok(Default::default()),
                value_profile_ind_var_code_id: Ok(Default::default()),
                value_text: Ok(Default::default()),
            }
        }
    }
    impl AgsiDataParameterValue {
        pub fn case_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.case_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for case_id: {}", e));
            self
        }
        pub fn code_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsiDataParameterValueCodeId>,
            T::Error: ::std::fmt::Display,
        {
            self.code_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_id: {}", e));
            self
        }
        pub fn data_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.data_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data_id: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn value_numeric<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_numeric = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_numeric: {}", e));
            self
        }
        pub fn value_profile<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ValueProfile>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_profile = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_profile: {}", e));
            self
        }
        pub fn value_profile_ind_var_code_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_profile_ind_var_code_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for value_profile_ind_var_code_id: {}",
                    e
                )
            });
            self
        }
        pub fn value_text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_text: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiDataParameterValue> for super::AgsiDataParameterValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiDataParameterValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                case_id: value.case_id?,
                code_id: value.code_id?,
                data_id: value.data_id?,
                remarks: value.remarks?,
                value_numeric: value.value_numeric?,
                value_profile: value.value_profile?,
                value_profile_ind_var_code_id: value.value_profile_ind_var_code_id?,
                value_text: value.value_text?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiDataParameterValue> for AgsiDataParameterValue {
        fn from(value: super::AgsiDataParameterValue) -> Self {
            Self {
                case_id: Ok(value.case_id),
                code_id: Ok(value.code_id),
                data_id: Ok(value.data_id),
                remarks: Ok(value.remarks),
                value_numeric: Ok(value.value_numeric),
                value_profile: Ok(value.value_profile),
                value_profile_ind_var_code_id: Ok(value.value_profile_ind_var_code_id),
                value_text: Ok(value.value_text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiDataPropertyFromFile {
        data_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        date: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        file_format: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        file_format_version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        file_part: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        file_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        revision: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        revision_info: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsiDataPropertyFromFile {
        fn default() -> Self {
            Self {
                data_id: Ok(Default::default()),
                date: Ok(Default::default()),
                description: Ok(Default::default()),
                file_format: Ok(Default::default()),
                file_format_version: Ok(Default::default()),
                file_part: Ok(Default::default()),
                file_uri: Err("no value supplied for file_uri".to_string()),
                remarks: Ok(Default::default()),
                revision: Ok(Default::default()),
                revision_info: Ok(Default::default()),
            }
        }
    }
    impl AgsiDataPropertyFromFile {
        pub fn data_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.data_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data_id: {}", e));
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn file_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_format: {}", e));
            self
        }
        pub fn file_format_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_format_version = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for file_format_version: {}",
                    e
                )
            });
            self
        }
        pub fn file_part<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_part = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_part: {}", e));
            self
        }
        pub fn file_uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file_uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_uri: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn revision<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.revision = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for revision: {}", e));
            self
        }
        pub fn revision_info<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.revision_info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for revision_info: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiDataPropertyFromFile> for super::AgsiDataPropertyFromFile {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiDataPropertyFromFile,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data_id: value.data_id?,
                date: value.date?,
                description: value.description?,
                file_format: value.file_format?,
                file_format_version: value.file_format_version?,
                file_part: value.file_part?,
                file_uri: value.file_uri?,
                remarks: value.remarks?,
                revision: value.revision?,
                revision_info: value.revision_info?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiDataPropertyFromFile> for AgsiDataPropertyFromFile {
        fn from(value: super::AgsiDataPropertyFromFile) -> Self {
            Self {
                data_id: Ok(value.data_id),
                date: Ok(value.date),
                description: Ok(value.description),
                file_format: Ok(value.file_format),
                file_format_version: Ok(value.file_format_version),
                file_part: Ok(value.file_part),
                file_uri: Ok(value.file_uri),
                remarks: Ok(value.remarks),
                revision: Ok(value.revision),
                revision_info: Ok(value.revision_info),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiDataPropertySummary {
        case_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        code_id: ::std::result::Result<super::AgsiDataPropertySummaryCodeId, ::std::string::String>,
        data_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value_count: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value_max: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value_mean: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value_min: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value_std_dev: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value_summary_text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsiDataPropertySummary {
        fn default() -> Self {
            Self {
                case_id: Ok(Default::default()),
                code_id: Err("no value supplied for code_id".to_string()),
                data_id: Ok(Default::default()),
                remarks: Ok(Default::default()),
                value_count: Ok(Default::default()),
                value_max: Ok(Default::default()),
                value_mean: Ok(Default::default()),
                value_min: Ok(Default::default()),
                value_std_dev: Ok(Default::default()),
                value_summary_text: Ok(Default::default()),
            }
        }
    }
    impl AgsiDataPropertySummary {
        pub fn case_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.case_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for case_id: {}", e));
            self
        }
        pub fn code_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsiDataPropertySummaryCodeId>,
            T::Error: ::std::fmt::Display,
        {
            self.code_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_id: {}", e));
            self
        }
        pub fn data_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.data_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data_id: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn value_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_count: {}", e));
            self
        }
        pub fn value_max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_max: {}", e));
            self
        }
        pub fn value_mean<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_mean = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_mean: {}", e));
            self
        }
        pub fn value_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_min: {}", e));
            self
        }
        pub fn value_std_dev<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_std_dev = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_std_dev: {}", e));
            self
        }
        pub fn value_summary_text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_summary_text = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for value_summary_text: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiDataPropertySummary> for super::AgsiDataPropertySummary {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiDataPropertySummary,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                case_id: value.case_id?,
                code_id: value.code_id?,
                data_id: value.data_id?,
                remarks: value.remarks?,
                value_count: value.value_count?,
                value_max: value.value_max?,
                value_mean: value.value_mean?,
                value_min: value.value_min?,
                value_std_dev: value.value_std_dev?,
                value_summary_text: value.value_summary_text?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiDataPropertySummary> for AgsiDataPropertySummary {
        fn from(value: super::AgsiDataPropertySummary) -> Self {
            Self {
                case_id: Ok(value.case_id),
                code_id: Ok(value.code_id),
                data_id: Ok(value.data_id),
                remarks: Ok(value.remarks),
                value_count: Ok(value.value_count),
                value_max: Ok(value.value_max),
                value_mean: Ok(value.value_mean),
                value_min: Ok(value.value_min),
                value_std_dev: Ok(value.value_std_dev),
                value_summary_text: Ok(value.value_summary_text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiDataPropertyValue {
        case_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        code_id: ::std::result::Result<super::AgsiDataPropertyValueCodeId, ::std::string::String>,
        data_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value_numeric: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value_profile: ::std::result::Result<
            ::std::option::Option<super::ValueProfile>,
            ::std::string::String,
        >,
        value_profile_ind_var_code_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value_text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsiDataPropertyValue {
        fn default() -> Self {
            Self {
                case_id: Ok(Default::default()),
                code_id: Err("no value supplied for code_id".to_string()),
                data_id: Ok(Default::default()),
                remarks: Ok(Default::default()),
                value_numeric: Ok(Default::default()),
                value_profile: Ok(Default::default()),
                value_profile_ind_var_code_id: Ok(Default::default()),
                value_text: Ok(Default::default()),
            }
        }
    }
    impl AgsiDataPropertyValue {
        pub fn case_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.case_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for case_id: {}", e));
            self
        }
        pub fn code_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsiDataPropertyValueCodeId>,
            T::Error: ::std::fmt::Display,
        {
            self.code_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_id: {}", e));
            self
        }
        pub fn data_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.data_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data_id: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn value_numeric<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_numeric = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_numeric: {}", e));
            self
        }
        pub fn value_profile<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ValueProfile>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_profile = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_profile: {}", e));
            self
        }
        pub fn value_profile_ind_var_code_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_profile_ind_var_code_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for value_profile_ind_var_code_id: {}",
                    e
                )
            });
            self
        }
        pub fn value_text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_text: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiDataPropertyValue> for super::AgsiDataPropertyValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiDataPropertyValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                case_id: value.case_id?,
                code_id: value.code_id?,
                data_id: value.data_id?,
                remarks: value.remarks?,
                value_numeric: value.value_numeric?,
                value_profile: value.value_profile?,
                value_profile_ind_var_code_id: value.value_profile_ind_var_code_id?,
                value_text: value.value_text?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiDataPropertyValue> for AgsiDataPropertyValue {
        fn from(value: super::AgsiDataPropertyValue) -> Self {
            Self {
                case_id: Ok(value.case_id),
                code_id: Ok(value.code_id),
                data_id: Ok(value.data_id),
                remarks: Ok(value.remarks),
                value_numeric: Ok(value.value_numeric),
                value_profile: Ok(value.value_profile),
                value_profile_ind_var_code_id: Ok(value.value_profile_ind_var_code_id),
                value_text: Ok(value.value_text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiGeometryFromFile {
        date: ::std::result::Result<
            ::std::option::Option<::chrono::naive::NaiveDate>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        file_format: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        file_format_version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        file_part: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        file_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        geometry_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        geometry_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        revision: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        revision_info: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsiGeometryFromFile {
        fn default() -> Self {
            Self {
                date: Ok(Default::default()),
                description: Ok(Default::default()),
                file_format: Ok(Default::default()),
                file_format_version: Ok(Default::default()),
                file_part: Ok(Default::default()),
                file_uri: Err("no value supplied for file_uri".to_string()),
                geometry_id: Ok(Default::default()),
                geometry_type: Ok(Default::default()),
                remarks: Ok(Default::default()),
                revision: Ok(Default::default()),
                revision_info: Ok(Default::default()),
            }
        }
    }
    impl AgsiGeometryFromFile {
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::chrono::naive::NaiveDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn file_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_format: {}", e));
            self
        }
        pub fn file_format_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_format_version = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for file_format_version: {}",
                    e
                )
            });
            self
        }
        pub fn file_part<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_part = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_part: {}", e));
            self
        }
        pub fn file_uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file_uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_uri: {}", e));
            self
        }
        pub fn geometry_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.geometry_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geometry_id: {}", e));
            self
        }
        pub fn geometry_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.geometry_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geometry_type: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn revision<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.revision = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for revision: {}", e));
            self
        }
        pub fn revision_info<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.revision_info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for revision_info: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiGeometryFromFile> for super::AgsiGeometryFromFile {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiGeometryFromFile,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                date: value.date?,
                description: value.description?,
                file_format: value.file_format?,
                file_format_version: value.file_format_version?,
                file_part: value.file_part?,
                file_uri: value.file_uri?,
                geometry_id: value.geometry_id?,
                geometry_type: value.geometry_type?,
                remarks: value.remarks?,
                revision: value.revision?,
                revision_info: value.revision_info?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiGeometryFromFile> for AgsiGeometryFromFile {
        fn from(value: super::AgsiGeometryFromFile) -> Self {
            Self {
                date: Ok(value.date),
                description: Ok(value.description),
                file_format: Ok(value.file_format),
                file_format_version: Ok(value.file_format_version),
                file_part: Ok(value.file_part),
                file_uri: Ok(value.file_uri),
                geometry_id: Ok(value.geometry_id),
                geometry_type: Ok(value.geometry_type),
                remarks: Ok(value.remarks),
                revision: Ok(value.revision),
                revision_info: Ok(value.revision_info),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiGeometryPlane {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        elevation: ::std::result::Result<f64, ::std::string::String>,
        geometry_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsiGeometryPlane {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                elevation: Err("no value supplied for elevation".to_string()),
                geometry_id: Ok(Default::default()),
                remarks: Ok(Default::default()),
            }
        }
    }
    impl AgsiGeometryPlane {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn elevation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.elevation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for elevation: {}", e));
            self
        }
        pub fn geometry_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.geometry_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geometry_id: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiGeometryPlane> for super::AgsiGeometryPlane {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiGeometryPlane,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                elevation: value.elevation?,
                geometry_id: value.geometry_id?,
                remarks: value.remarks?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiGeometryPlane> for AgsiGeometryPlane {
        fn from(value: super::AgsiGeometryPlane) -> Self {
            Self {
                description: Ok(value.description),
                elevation: Ok(value.elevation),
                geometry_id: Ok(value.geometry_id),
                remarks: Ok(value.remarks),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiModel {
        agsi_model_alignment: ::std::result::Result<
            ::std::vec::Vec<super::AgsiModelAlignment>,
            ::std::string::String,
        >,
        agsi_model_boundary: ::std::result::Result<
            ::std::option::Option<super::AgsiModelBoundary>,
            ::std::string::String,
        >,
        agsi_model_element:
            ::std::result::Result<::std::vec::Vec<super::AgsiModelElement>, ::std::string::String>,
        agsi_observation_set: ::std::result::Result<
            ::std::vec::Vec<super::AgsiObservationSet>,
            ::std::string::String,
        >,
        alignment_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        category: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        coord_system_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        document_set_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        domain: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        input: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        method: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        model_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        model_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        model_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uncertainty: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        usage: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsiModel {
        fn default() -> Self {
            Self {
                agsi_model_alignment: Ok(Default::default()),
                agsi_model_boundary: Ok(Default::default()),
                agsi_model_element: Ok(Default::default()),
                agsi_observation_set: Ok(Default::default()),
                alignment_id: Ok(Default::default()),
                category: Ok(Default::default()),
                coord_system_id: Ok(Default::default()),
                description: Ok(Default::default()),
                document_set_id: Ok(Default::default()),
                domain: Ok(Default::default()),
                input: Ok(Default::default()),
                method: Ok(Default::default()),
                model_id: Ok(Default::default()),
                model_name: Ok(Default::default()),
                model_type: Ok(Default::default()),
                remarks: Ok(Default::default()),
                uncertainty: Ok(Default::default()),
                usage: Ok(Default::default()),
            }
        }
    }
    impl AgsiModel {
        pub fn agsi_model_alignment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiModelAlignment>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_model_alignment = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_model_alignment: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_model_boundary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiModelBoundary>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_model_boundary = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_model_boundary: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_model_element<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiModelElement>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_model_element = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_model_element: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_observation_set<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiObservationSet>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_observation_set = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_observation_set: {}",
                    e
                )
            });
            self
        }
        pub fn alignment_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.alignment_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alignment_id: {}", e));
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
        pub fn coord_system_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.coord_system_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coord_system_id: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn document_set_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.document_set_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for document_set_id: {}", e));
            self
        }
        pub fn domain<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.domain = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for domain: {}", e));
            self
        }
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {}", e));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {}", e));
            self
        }
        pub fn model_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model_id: {}", e));
            self
        }
        pub fn model_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model_name: {}", e));
            self
        }
        pub fn model_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model_type: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn uncertainty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.uncertainty = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uncertainty: {}", e));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiModel> for super::AgsiModel {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiModel,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agsi_model_alignment: value.agsi_model_alignment?,
                agsi_model_boundary: value.agsi_model_boundary?,
                agsi_model_element: value.agsi_model_element?,
                agsi_observation_set: value.agsi_observation_set?,
                alignment_id: value.alignment_id?,
                category: value.category?,
                coord_system_id: value.coord_system_id?,
                description: value.description?,
                document_set_id: value.document_set_id?,
                domain: value.domain?,
                input: value.input?,
                method: value.method?,
                model_id: value.model_id?,
                model_name: value.model_name?,
                model_type: value.model_type?,
                remarks: value.remarks?,
                uncertainty: value.uncertainty?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiModel> for AgsiModel {
        fn from(value: super::AgsiModel) -> Self {
            Self {
                agsi_model_alignment: Ok(value.agsi_model_alignment),
                agsi_model_boundary: Ok(value.agsi_model_boundary),
                agsi_model_element: Ok(value.agsi_model_element),
                agsi_observation_set: Ok(value.agsi_observation_set),
                alignment_id: Ok(value.alignment_id),
                category: Ok(value.category),
                coord_system_id: Ok(value.coord_system_id),
                description: Ok(value.description),
                document_set_id: Ok(value.document_set_id),
                domain: Ok(value.domain),
                input: Ok(value.input),
                method: Ok(value.method),
                model_id: Ok(value.model_id),
                model_name: Ok(value.model_name),
                model_type: Ok(value.model_type),
                remarks: Ok(value.remarks),
                uncertainty: Ok(value.uncertainty),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiModelAlignment {
        agsi_geometry: ::std::result::Result<
            ::std::option::Option<super::AgsiGeometryFromFile>,
            ::std::string::String,
        >,
        alignment_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        alignment_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        start_chainage: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for AgsiModelAlignment {
        fn default() -> Self {
            Self {
                agsi_geometry: Ok(Default::default()),
                alignment_id: Ok(Default::default()),
                alignment_name: Ok(Default::default()),
                description: Ok(Default::default()),
                remarks: Ok(Default::default()),
                start_chainage: Ok(Default::default()),
            }
        }
    }
    impl AgsiModelAlignment {
        pub fn agsi_geometry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiGeometryFromFile>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_geometry = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agsi_geometry: {}", e));
            self
        }
        pub fn alignment_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.alignment_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alignment_id: {}", e));
            self
        }
        pub fn alignment_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.alignment_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alignment_name: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn start_chainage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.start_chainage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_chainage: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiModelAlignment> for super::AgsiModelAlignment {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiModelAlignment,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agsi_geometry: value.agsi_geometry?,
                alignment_id: value.alignment_id?,
                alignment_name: value.alignment_name?,
                description: value.description?,
                remarks: value.remarks?,
                start_chainage: value.start_chainage?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiModelAlignment> for AgsiModelAlignment {
        fn from(value: super::AgsiModelAlignment) -> Self {
            Self {
                agsi_geometry: Ok(value.agsi_geometry),
                alignment_id: Ok(value.alignment_id),
                alignment_name: Ok(value.alignment_name),
                description: Ok(value.description),
                remarks: Ok(value.remarks),
                start_chainage: Ok(value.start_chainage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiModelBoundary {
        agsi_geometry_boundary_xy: ::std::result::Result<
            ::std::option::Option<super::AgsiGeometryFromFile>,
            ::std::string::String,
        >,
        agsi_geometry_surface_bottom: ::std::result::Result<
            ::std::option::Option<super::AgsiGeometryFromFile>,
            ::std::string::String,
        >,
        agsi_geometry_surface_top: ::std::result::Result<
            ::std::option::Option<super::AgsiGeometryFromFile>,
            ::std::string::String,
        >,
        bottom_elevation: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        boundary_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        max_x: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        max_y: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        min_x: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        min_y: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        top_elevation: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for AgsiModelBoundary {
        fn default() -> Self {
            Self {
                agsi_geometry_boundary_xy: Ok(Default::default()),
                agsi_geometry_surface_bottom: Ok(Default::default()),
                agsi_geometry_surface_top: Ok(Default::default()),
                bottom_elevation: Ok(Default::default()),
                boundary_id: Ok(Default::default()),
                description: Ok(Default::default()),
                max_x: Ok(Default::default()),
                max_y: Ok(Default::default()),
                min_x: Ok(Default::default()),
                min_y: Ok(Default::default()),
                remarks: Ok(Default::default()),
                top_elevation: Ok(Default::default()),
            }
        }
    }
    impl AgsiModelBoundary {
        pub fn agsi_geometry_boundary_xy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiGeometryFromFile>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_geometry_boundary_xy = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_geometry_boundary_xy: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_geometry_surface_bottom<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiGeometryFromFile>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_geometry_surface_bottom = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_geometry_surface_bottom: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_geometry_surface_top<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiGeometryFromFile>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_geometry_surface_top = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_geometry_surface_top: {}",
                    e
                )
            });
            self
        }
        pub fn bottom_elevation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.bottom_elevation = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for bottom_elevation: {}",
                    e
                )
            });
            self
        }
        pub fn boundary_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.boundary_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boundary_id: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn max_x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_x: {}", e));
            self
        }
        pub fn max_y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_y: {}", e));
            self
        }
        pub fn min_x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_x: {}", e));
            self
        }
        pub fn min_y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_y: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
        pub fn top_elevation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.top_elevation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_elevation: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiModelBoundary> for super::AgsiModelBoundary {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiModelBoundary,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agsi_geometry_boundary_xy: value.agsi_geometry_boundary_xy?,
                agsi_geometry_surface_bottom: value.agsi_geometry_surface_bottom?,
                agsi_geometry_surface_top: value.agsi_geometry_surface_top?,
                bottom_elevation: value.bottom_elevation?,
                boundary_id: value.boundary_id?,
                description: value.description?,
                max_x: value.max_x?,
                max_y: value.max_y?,
                min_x: value.min_x?,
                min_y: value.min_y?,
                remarks: value.remarks?,
                top_elevation: value.top_elevation?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiModelBoundary> for AgsiModelBoundary {
        fn from(value: super::AgsiModelBoundary) -> Self {
            Self {
                agsi_geometry_boundary_xy: Ok(value.agsi_geometry_boundary_xy),
                agsi_geometry_surface_bottom: Ok(value.agsi_geometry_surface_bottom),
                agsi_geometry_surface_top: Ok(value.agsi_geometry_surface_top),
                bottom_elevation: Ok(value.bottom_elevation),
                boundary_id: Ok(value.boundary_id),
                description: Ok(value.description),
                max_x: Ok(value.max_x),
                max_y: Ok(value.max_y),
                min_x: Ok(value.min_x),
                min_y: Ok(value.min_y),
                remarks: Ok(value.remarks),
                top_elevation: Ok(value.top_elevation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiModelElement {
        agsi_data_parameter_value: ::std::result::Result<
            ::std::vec::Vec<super::AgsiDataParameterValue>,
            ::std::string::String,
        >,
        agsi_data_property_from_file: ::std::result::Result<
            ::std::option::Option<super::AgsiDataPropertyFromFile>,
            ::std::string::String,
        >,
        agsi_data_property_summary: ::std::result::Result<
            ::std::vec::Vec<super::AgsiDataPropertySummary>,
            ::std::string::String,
        >,
        agsi_data_property_value: ::std::result::Result<
            ::std::vec::Vec<super::AgsiDataPropertyValue>,
            ::std::string::String,
        >,
        agsi_geometry: ::std::result::Result<
            ::std::option::Option<super::AgsiModelElementAgsiGeometry>,
            ::std::string::String,
        >,
        agsi_geometry_area_limit: ::std::result::Result<
            ::std::option::Option<super::AgsiGeometryFromFile>,
            ::std::string::String,
        >,
        colour_rgb: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        element_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        element_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        element_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        geometry_object: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsiModelElement {
        fn default() -> Self {
            Self {
                agsi_data_parameter_value: Ok(Default::default()),
                agsi_data_property_from_file: Ok(Default::default()),
                agsi_data_property_summary: Ok(Default::default()),
                agsi_data_property_value: Ok(Default::default()),
                agsi_geometry: Ok(Default::default()),
                agsi_geometry_area_limit: Ok(Default::default()),
                colour_rgb: Ok(Default::default()),
                description: Ok(Default::default()),
                element_id: Ok(Default::default()),
                element_name: Ok(Default::default()),
                element_type: Ok(Default::default()),
                geometry_object: Ok(Default::default()),
                remarks: Ok(Default::default()),
            }
        }
    }
    impl AgsiModelElement {
        pub fn agsi_data_parameter_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiDataParameterValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_data_parameter_value = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_data_parameter_value: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_data_property_from_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiDataPropertyFromFile>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_data_property_from_file = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_data_property_from_file: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_data_property_summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiDataPropertySummary>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_data_property_summary = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_data_property_summary: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_data_property_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiDataPropertyValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_data_property_value = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_data_property_value: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_geometry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiModelElementAgsiGeometry>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_geometry = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agsi_geometry: {}", e));
            self
        }
        pub fn agsi_geometry_area_limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiGeometryFromFile>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_geometry_area_limit = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_geometry_area_limit: {}",
                    e
                )
            });
            self
        }
        pub fn colour_rgb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.colour_rgb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for colour_rgb: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn element_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.element_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for element_id: {}", e));
            self
        }
        pub fn element_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.element_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for element_name: {}", e));
            self
        }
        pub fn element_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.element_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for element_type: {}", e));
            self
        }
        pub fn geometry_object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.geometry_object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geometry_object: {}", e));
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiModelElement> for super::AgsiModelElement {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiModelElement,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agsi_data_parameter_value: value.agsi_data_parameter_value?,
                agsi_data_property_from_file: value.agsi_data_property_from_file?,
                agsi_data_property_summary: value.agsi_data_property_summary?,
                agsi_data_property_value: value.agsi_data_property_value?,
                agsi_geometry: value.agsi_geometry?,
                agsi_geometry_area_limit: value.agsi_geometry_area_limit?,
                colour_rgb: value.colour_rgb?,
                description: value.description?,
                element_id: value.element_id?,
                element_name: value.element_name?,
                element_type: value.element_type?,
                geometry_object: value.geometry_object?,
                remarks: value.remarks?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiModelElement> for AgsiModelElement {
        fn from(value: super::AgsiModelElement) -> Self {
            Self {
                agsi_data_parameter_value: Ok(value.agsi_data_parameter_value),
                agsi_data_property_from_file: Ok(value.agsi_data_property_from_file),
                agsi_data_property_summary: Ok(value.agsi_data_property_summary),
                agsi_data_property_value: Ok(value.agsi_data_property_value),
                agsi_geometry: Ok(value.agsi_geometry),
                agsi_geometry_area_limit: Ok(value.agsi_geometry_area_limit),
                colour_rgb: Ok(value.colour_rgb),
                description: Ok(value.description),
                element_id: Ok(value.element_id),
                element_name: Ok(value.element_name),
                element_type: Ok(value.element_type),
                geometry_object: Ok(value.geometry_object),
                remarks: Ok(value.remarks),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiModelElementAgsiGeometry {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::AgsiGeometryVolFromSurfaces>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::AgsiGeometryFromFile>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<
            ::std::option::Option<super::AgsiGeometryAreaFromLines>,
            ::std::string::String,
        >,
        subtype_3: ::std::result::Result<
            ::std::option::Option<super::AgsiGeometryPlane>,
            ::std::string::String,
        >,
        subtype_4: ::std::result::Result<
            ::std::option::Option<super::AgsiGeometryLayer>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsiModelElementAgsiGeometry {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
                subtype_4: Ok(Default::default()),
            }
        }
    }
    impl AgsiModelElementAgsiGeometry {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiGeometryVolFromSurfaces>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiGeometryFromFile>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiGeometryAreaFromLines>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {}", e));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiGeometryPlane>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {}", e));
            self
        }
        pub fn subtype_4<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiGeometryLayer>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_4 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_4: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiModelElementAgsiGeometry> for super::AgsiModelElementAgsiGeometry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiModelElementAgsiGeometry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
                subtype_4: value.subtype_4?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiModelElementAgsiGeometry> for AgsiModelElementAgsiGeometry {
        fn from(value: super::AgsiModelElementAgsiGeometry) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
                subtype_4: Ok(value.subtype_4),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiObservationPoint {
        agsi_data_property_from_file: ::std::result::Result<
            ::std::option::Option<super::AgsiDataPropertyFromFile>,
            ::std::string::String,
        >,
        agsi_data_property_value: ::std::result::Result<
            ::std::vec::Vec<super::AgsiDataPropertyValue>,
            ::std::string::String,
        >,
        coordinate: ::std::result::Result<super::CoordinateTuple, ::std::string::String>,
        date: ::std::result::Result<
            ::std::option::Option<::chrono::naive::NaiveDate>,
            ::std::string::String,
        >,
        made_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        observation_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        observation_nature: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        observation_text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsiObservationPoint {
        fn default() -> Self {
            Self {
                agsi_data_property_from_file: Ok(Default::default()),
                agsi_data_property_value: Ok(Default::default()),
                coordinate: Err("no value supplied for coordinate".to_string()),
                date: Ok(Default::default()),
                made_by: Ok(Default::default()),
                observation_id: Ok(Default::default()),
                observation_nature: Ok(Default::default()),
                observation_text: Ok(Default::default()),
                remarks: Ok(Default::default()),
            }
        }
    }
    impl AgsiObservationPoint {
        pub fn agsi_data_property_from_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiDataPropertyFromFile>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_data_property_from_file = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_data_property_from_file: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_data_property_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiDataPropertyValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_data_property_value = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_data_property_value: {}",
                    e
                )
            });
            self
        }
        pub fn coordinate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CoordinateTuple>,
            T::Error: ::std::fmt::Display,
        {
            self.coordinate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coordinate: {}", e));
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::chrono::naive::NaiveDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn made_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.made_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for made_by: {}", e));
            self
        }
        pub fn observation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.observation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for observation_id: {}", e));
            self
        }
        pub fn observation_nature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.observation_nature = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for observation_nature: {}",
                    e
                )
            });
            self
        }
        pub fn observation_text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.observation_text = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for observation_text: {}",
                    e
                )
            });
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiObservationPoint> for super::AgsiObservationPoint {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiObservationPoint,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agsi_data_property_from_file: value.agsi_data_property_from_file?,
                agsi_data_property_value: value.agsi_data_property_value?,
                coordinate: value.coordinate?,
                date: value.date?,
                made_by: value.made_by?,
                observation_id: value.observation_id?,
                observation_nature: value.observation_nature?,
                observation_text: value.observation_text?,
                remarks: value.remarks?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiObservationPoint> for AgsiObservationPoint {
        fn from(value: super::AgsiObservationPoint) -> Self {
            Self {
                agsi_data_property_from_file: Ok(value.agsi_data_property_from_file),
                agsi_data_property_value: Ok(value.agsi_data_property_value),
                coordinate: Ok(value.coordinate),
                date: Ok(value.date),
                made_by: Ok(value.made_by),
                observation_id: Ok(value.observation_id),
                observation_nature: Ok(value.observation_nature),
                observation_text: Ok(value.observation_text),
                remarks: Ok(value.remarks),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiObservationSet {
        agsi_data_property_from_file: ::std::result::Result<
            ::std::option::Option<super::AgsiDataPropertyFromFile>,
            ::std::string::String,
        >,
        agsi_geometry_from_file: ::std::result::Result<
            ::std::option::Option<super::AgsiGeometryFromFile>,
            ::std::string::String,
        >,
        agsi_observation_exp_hole: ::std::result::Result<
            ::std::vec::Vec<super::AgsiObservationExpHole>,
            ::std::string::String,
        >,
        agsi_observation_point: ::std::result::Result<
            ::std::vec::Vec<super::AgsiObservationPoint>,
            ::std::string::String,
        >,
        agsi_observation_shape: ::std::result::Result<
            ::std::vec::Vec<super::AgsiObservationShape>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        document_set_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        investigation_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        observation_set_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsiObservationSet {
        fn default() -> Self {
            Self {
                agsi_data_property_from_file: Ok(Default::default()),
                agsi_geometry_from_file: Ok(Default::default()),
                agsi_observation_exp_hole: Ok(Default::default()),
                agsi_observation_point: Ok(Default::default()),
                agsi_observation_shape: Ok(Default::default()),
                description: Ok(Default::default()),
                document_set_id: Ok(Default::default()),
                investigation_id: Ok(Default::default()),
                observation_set_id: Ok(Default::default()),
                remarks: Ok(Default::default()),
            }
        }
    }
    impl AgsiObservationSet {
        pub fn agsi_data_property_from_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiDataPropertyFromFile>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_data_property_from_file = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_data_property_from_file: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_geometry_from_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiGeometryFromFile>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_geometry_from_file = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_geometry_from_file: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_observation_exp_hole<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiObservationExpHole>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_observation_exp_hole = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_observation_exp_hole: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_observation_point<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiObservationPoint>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_observation_point = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_observation_point: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_observation_shape<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiObservationShape>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_observation_shape = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_observation_shape: {}",
                    e
                )
            });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn document_set_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.document_set_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for document_set_id: {}", e));
            self
        }
        pub fn investigation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.investigation_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for investigation_id: {}",
                    e
                )
            });
            self
        }
        pub fn observation_set_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.observation_set_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for observation_set_id: {}",
                    e
                )
            });
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiObservationSet> for super::AgsiObservationSet {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiObservationSet,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agsi_data_property_from_file: value.agsi_data_property_from_file?,
                agsi_geometry_from_file: value.agsi_geometry_from_file?,
                agsi_observation_exp_hole: value.agsi_observation_exp_hole?,
                agsi_observation_point: value.agsi_observation_point?,
                agsi_observation_shape: value.agsi_observation_shape?,
                description: value.description?,
                document_set_id: value.document_set_id?,
                investigation_id: value.investigation_id?,
                observation_set_id: value.observation_set_id?,
                remarks: value.remarks?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiObservationSet> for AgsiObservationSet {
        fn from(value: super::AgsiObservationSet) -> Self {
            Self {
                agsi_data_property_from_file: Ok(value.agsi_data_property_from_file),
                agsi_geometry_from_file: Ok(value.agsi_geometry_from_file),
                agsi_observation_exp_hole: Ok(value.agsi_observation_exp_hole),
                agsi_observation_point: Ok(value.agsi_observation_point),
                agsi_observation_shape: Ok(value.agsi_observation_shape),
                description: Ok(value.description),
                document_set_id: Ok(value.document_set_id),
                investigation_id: Ok(value.investigation_id),
                observation_set_id: Ok(value.observation_set_id),
                remarks: Ok(value.remarks),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgsiObservationShape {
        agsi_data_property_from_file: ::std::result::Result<
            ::std::option::Option<super::AgsiDataPropertyFromFile>,
            ::std::string::String,
        >,
        agsi_data_property_value: ::std::result::Result<
            ::std::vec::Vec<super::AgsiDataPropertyValue>,
            ::std::string::String,
        >,
        agsi_geometry_from_file:
            ::std::result::Result<super::AgsiGeometryFromFile, ::std::string::String>,
        date: ::std::result::Result<
            ::std::option::Option<::chrono::naive::NaiveDate>,
            ::std::string::String,
        >,
        made_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        observation_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        observation_nature: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        observation_text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remarks: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgsiObservationShape {
        fn default() -> Self {
            Self {
                agsi_data_property_from_file: Ok(Default::default()),
                agsi_data_property_value: Ok(Default::default()),
                agsi_geometry_from_file: Err(
                    "no value supplied for agsi_geometry_from_file".to_string()
                ),
                date: Ok(Default::default()),
                made_by: Ok(Default::default()),
                observation_id: Ok(Default::default()),
                observation_nature: Ok(Default::default()),
                observation_text: Ok(Default::default()),
                remarks: Ok(Default::default()),
            }
        }
    }
    impl AgsiObservationShape {
        pub fn agsi_data_property_from_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgsiDataPropertyFromFile>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_data_property_from_file = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_data_property_from_file: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_data_property_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgsiDataPropertyValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_data_property_value = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_data_property_value: {}",
                    e
                )
            });
            self
        }
        pub fn agsi_geometry_from_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgsiGeometryFromFile>,
            T::Error: ::std::fmt::Display,
        {
            self.agsi_geometry_from_file = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for agsi_geometry_from_file: {}",
                    e
                )
            });
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::chrono::naive::NaiveDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn made_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.made_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for made_by: {}", e));
            self
        }
        pub fn observation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.observation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for observation_id: {}", e));
            self
        }
        pub fn observation_nature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.observation_nature = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for observation_nature: {}",
                    e
                )
            });
            self
        }
        pub fn observation_text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.observation_text = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for observation_text: {}",
                    e
                )
            });
            self
        }
        pub fn remarks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remarks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remarks: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgsiObservationShape> for super::AgsiObservationShape {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgsiObservationShape,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agsi_data_property_from_file: value.agsi_data_property_from_file?,
                agsi_data_property_value: value.agsi_data_property_value?,
                agsi_geometry_from_file: value.agsi_geometry_from_file?,
                date: value.date?,
                made_by: value.made_by?,
                observation_id: value.observation_id?,
                observation_nature: value.observation_nature?,
                observation_text: value.observation_text?,
                remarks: value.remarks?,
            })
        }
    }
    impl ::std::convert::From<super::AgsiObservationShape> for AgsiObservationShape {
        fn from(value: super::AgsiObservationShape) -> Self {
            Self {
                agsi_data_property_from_file: Ok(value.agsi_data_property_from_file),
                agsi_data_property_value: Ok(value.agsi_data_property_value),
                agsi_geometry_from_file: Ok(value.agsi_geometry_from_file),
                date: Ok(value.date),
                made_by: Ok(value.made_by),
                observation_id: Ok(value.observation_id),
                observation_nature: Ok(value.observation_nature),
                observation_text: Ok(value.observation_text),
                remarks: Ok(value.remarks),
            }
        }
    }
}
