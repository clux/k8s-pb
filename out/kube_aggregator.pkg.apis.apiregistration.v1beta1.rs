/// APIService represents a server for a particular GroupVersion.
/// Name must be "version.group".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiService {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec contains information for locating and communicating with a server
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<ApiServiceSpec>,
    /// Status contains derived information about an API server
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<ApiServiceStatus>,
}
/// APIServiceCondition describes the state of an APIService at a particular point
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiServiceCondition {
    /// Type is the type of the condition.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status is the status of the condition.
    /// Can be True, False, Unknown.
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// Last time the condition transitioned from one status to another.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub last_transition_time: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// Human-readable message indicating details about last transition.
    /// +optional
    #[prost(string, optional, tag="5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// APIServiceList is a list of APIService objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiServiceList {
    /// Standard list metadata
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of APIService
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<ApiService>,
}
/// APIServiceSpec contains information for locating and communicating with a server.
/// Only https is supported, though you are able to disable certificate verification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiServiceSpec {
    /// Service is a reference to the service for this API server.  It must communicate
    /// on port 443.
    /// If the Service is nil, that means the handling for the API groupversion is handled locally on this server.
    /// The call will simply delegate to the normal handler chain to be fulfilled.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub service: ::core::option::Option<ServiceReference>,
    /// Group is the API group name this server hosts
    #[prost(string, optional, tag="2")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    /// Version is the API version this server hosts.  For example, "v1"
    #[prost(string, optional, tag="3")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    /// InsecureSkipTLSVerify disables TLS certificate verification when communicating with this server.
    /// This is strongly discouraged.  You should use the CABundle instead.
    #[prost(bool, optional, tag="4")]
    pub insecure_skip_tls_verify: ::core::option::Option<bool>,
    /// CABundle is a PEM encoded CA bundle which will be used to validate an API server's serving certificate.
    /// If unspecified, system trust roots on the apiserver are used.
    /// +listType=atomic
    /// +optional
    #[prost(bytes="vec", optional, tag="5")]
    pub ca_bundle: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// GroupPriorityMininum is the priority this group should have at least. Higher priority means that the group is preferred by clients over lower priority ones.
    /// Note that other versions of this group might specify even higher GroupPriorityMininum values such that the whole group gets a higher priority.
    /// The primary sort is based on GroupPriorityMinimum, ordered highest number to lowest (20 before 10).
    /// The secondary sort is based on the alphabetical comparison of the name of the object.  (v1.bar before v1.foo)
    /// We'd recommend something like: *.k8s.io (except extensions) at 18000 and
    /// PaaSes (OpenShift, Deis) are recommended to be in the 2000s
    #[prost(int32, optional, tag="7")]
    pub group_priority_minimum: ::core::option::Option<i32>,
    /// VersionPriority controls the ordering of this API version inside of its group.  Must be greater than zero.
    /// The primary sort is based on VersionPriority, ordered highest to lowest (20 before 10).
    /// Since it's inside of a group, the number can be small, probably in the 10s.
    /// In case of equal version priorities, the version string will be used to compute the order inside a group.
    /// If the version string is "kube-like", it will sort above non "kube-like" version strings, which are ordered
    /// lexicographically. "Kube-like" versions start with a "v", then are followed by a number (the major version),
    /// then optionally the string "alpha" or "beta" and another number (the minor version). These are sorted first
    /// by GA > beta > alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major
    /// version, then minor version. An example sorted list of versions:
    /// v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
    #[prost(int32, optional, tag="8")]
    pub version_priority: ::core::option::Option<i32>,
}
/// APIServiceStatus contains derived information about an API server
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiServiceStatus {
    /// Current service state of apiService.
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=type
    #[prost(message, repeated, tag="1")]
    pub conditions: ::prost::alloc::vec::Vec<ApiServiceCondition>,
}
/// ServiceReference holds a reference to Service.legacy.k8s.io
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceReference {
    /// Namespace is the namespace of the service
    #[prost(string, optional, tag="1")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Name is the name of the service
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// If specified, the port on the service that hosting webhook.
    /// Default to 443 for backward compatibility.
    /// `port` should be a valid port number (1-65535, inclusive).
    /// +optional
    #[prost(int32, optional, tag="3")]
    pub port: ::core::option::Option<i32>,
}
// TODO genericsfor kube_aggregator.pkg.apis.apiregistration.v1beta1
