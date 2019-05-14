use std::fmt::{Debug, Formatter, Result};

#[allow(clippy::large_enum_variant)]
pub enum RdfError {
    CreateWorld,
    CreateModel,
    AddToModel,
    AddStmtToModel,
    AddStringLiteralToStatement,
    CreateQuery,
    ExecuteQuery,
    CreateStringFromQueryResult,
    CreateStatement,
    CreateURI,
    CreateNode,
    CreateNodeFromLiteral,
    CreateNodeFromLocalName,
    CreateNodeFromURI,
    CreateParser,
    ParseStringIntoModel,
    ParseFileIntoModel,
    CreateSerializer,
    SerializeModel,
    SetNamespaceToSerializer,
    ConvertToStringInSerializer,
    ConvertToCString,
}

impl Debug for RdfError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match *self {
            RdfError::CreateWorld => write!(formatter, "Failed to create Redland World"),
            RdfError::CreateModel => write!(formatter, "Failed to create Redland Model"),
            RdfError::AddToModel => write!(
                formatter,
                "Failed to add a Statement using Nodes(Subject, Predicate, Object) to the Model"
            ),
            RdfError::AddStmtToModel => write!(
                formatter,
                "Failed to add a Statement using Statement to the Model"
            ),
            RdfError::AddStringLiteralToStatement => {
                write!(formatter, "Failed to add string Literal to Statement")
            }
            RdfError::CreateQuery => write!(formatter, "Failed to create Query instance"),
            RdfError::ExecuteQuery => write!(formatter, "Failed to execute the Query"),
            RdfError::CreateStringFromQueryResult => {
                write!(formatter, "Failed to convert Query Result to String")
            }
            RdfError::CreateStatement => write!(formatter, "Failed to create Statement"),
            RdfError::CreateURI => write!(formatter, "Failed to create URI"),
            RdfError::CreateNode => write!(formatter, "Failed to create Node"),
            RdfError::CreateNodeFromLiteral => {
                write!(formatter, "Failed to create Node from Literal")
            }
            RdfError::CreateNodeFromLocalName => {
                write!(formatter, "Failed to create Node from Local Name")
            }
            RdfError::CreateNodeFromURI => write!(formatter, "Failed to create Node from URI"),
            RdfError::CreateParser => write!(formatter, "Failed to create Parser instance"),
            RdfError::ParseStringIntoModel => {
                write!(formatter, "Failed to parse String into Model")
            }
            RdfError::ParseFileIntoModel => write!(formatter, "Failed to parse File into Model"),
            RdfError::CreateSerializer => write!(formatter, "Failed to create Serializer instance"),
            RdfError::SerializeModel => write!(formatter, "Failed to Serialize Model"),
            RdfError::SetNamespaceToSerializer => {
                write!(formatter, "Failed to set Namespace to the serializer")
            }
            RdfError::ConvertToStringInSerializer => {
                write!(formatter, "Failed to convert Model to String in Serializer")
            }
            RdfError::ConvertToCString => {
                write!(formatter, "Failed to convert Rust String to C String")
            }
        }
    }
}
