// https://spec.openapis.org/oas/latest.html#openapi-object
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApi {
    pub openapi: String,
    pub info: Info,
    pub servers: Option<Vec<Server>>,
    pub paths: HashMap<String, PathItem>,
    pub components: Option<Components>,
    pub tags: Option<Vec<Tag>>,
    pub external_docs: Option<ExternalDocumentation>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaType<T> {
    Reference {
        #[serde(rename = "$ref")]
        ref_obj: String,
    },
    Definition(T)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    pub external_docs: Option<ExternalDocumentation>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalDocumentation {
    pub description: Option<String>,
    pub url: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub title: String,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct License {
    pub name: String,
    pub url: Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
    pub description: Option<String>,
    pub variables: Vec<ServerVariable>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerVariable {
    pub r#enum: Vec<String>,
    pub default: String,
    pub description: Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PathItem {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub options: Option<Operation>,
    pub head: Option<Operation>,
    pub patch: Option<Operation>,
    pub trace: Option<Operation>,
    pub servers: Option<Vec<Server>>,
    pub parameters: Option<SchemaType<Parameter>>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub external_docs: Option<ExternalDocumentation>,
    pub operation_id: Option<String>,
    pub parameters: Vec<SchemaType<Parameter>>,
    pub request_body: Option<SchemaType<RequestBody>>,
    pub responses: Option<HashMap<String, Response>>,
    // callbacks
    pub deprecated: Option<bool>,
    // security
    pub server: Option<Server>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub placement: Option<String>,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    pub allow_empty_value: Option<bool>,
    pub schema: Option<JsonSchema>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    pub schemas: Option<HashMap<String, JsonSchema>>,
    pub responses: Option<HashMap<String, Response>>,
    pub parameters: Option<HashMap<String, Parameter>>,
    pub examples: Option<HashMap<String, Example>>,
    pub request_bodies: Option<HashMap<String, RequestBody>>,
    pub headers: Option<HashMap<String, Header>>,
    // securitySchemes
    pub path_items: Option<HashMap<String, PathItem>>,
    // pub callbacks: HashMap<String, JsonSchema>,
    // pub links: HashMap<String, JsonSchema>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum JsonSchema {
    Obj(DataSchema),
    AllOf {
      description: Option<String>,
      all_of: Vec<SchemaType<DataSchema>>
    },
    OneOf {
        description: Option<String>,
        one_of: Vec<SchemaType<DataSchema>>
    },
    AnyOf {
        description: Option<String>,
        any_of: Vec<SchemaType<DataSchema>>
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSchema {
    #[serde(rename = "type")]
    pub data_type: Option<String>,
    pub items: Option<Box<JsonSchema>>,

    pub format: Option<String>,

    pub maximum: Option<f32>,
    pub minimum: Option<f32>,

    pub min_length: Option<u32>,
    pub max_length: Option<u32>,

    pub min_items: Option<u32>,
    pub max_items: Option<u32>,

    pub required: Option<Vec<String>>,

    pub properties: Option<HashMap<String, Box<JsonSchema>>>,

    pub additional_properties: Option<HashMap<String, Box<JsonSchema>>>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub description: Option<String>,
    pub headers: Option<HashMap<String, Header>>,
    pub content: Option<HashMap<String, MediaType>>,
    // pub links: HashMap<String, Link>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    pub allow_empty_value: Option<bool>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaType {
    pub schema: SchemaType<JsonSchema>,
    // pub example: Option<Example>,
    pub examples: Option<HashMap<String, Example>,>
    // encoding
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub value: Option<String>,
    pub external_value: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestBody {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub content: HashMap<String, MediaType>,
}