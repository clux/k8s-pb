/// PartialObjectMetadataList contains a list of objects containing only their metadata.
/// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartialObjectMetadataList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    /// +optional
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::v1::ListMeta>,
    /// items contains each of the included items.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<super::v1::PartialObjectMetadata>,
}
