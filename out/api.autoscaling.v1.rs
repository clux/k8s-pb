/// ContainerResourceMetricSource indicates how to scale on a resource metric known to
/// Kubernetes, as specified in the requests and limits, describing a single container in
/// each of the pods of the current scale target(e.g. CPU or memory). The values will be
/// averaged together before being compared to the target. Such metrics are built into
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source. Only one "target" type
/// should be set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerResourceMetricSource {
    /// name is the name of the resource in question.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// targetAverageUtilization is the target value of the average of the
    /// resource metric across all relevant pods, represented as a percentage of
    /// the requested value of the resource for the pods.
    /// +optional
    #[prost(int32, optional, tag="2")]
    pub target_average_utilization: ::core::option::Option<i32>,
    /// targetAverageValue is the target value of the average of the
    /// resource metric across all relevant pods, as a raw value (instead of as
    /// a percentage of the request), similar to the "pods" metric source type.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub target_average_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// container is the name of the container in the pods of the scaling target.
    #[prost(string, optional, tag="5")]
    pub container: ::core::option::Option<::prost::alloc::string::String>,
}
/// ContainerResourceMetricStatus indicates the current value of a resource metric known to
/// Kubernetes, as specified in requests and limits, describing a single container in each pod in the
/// current scale target (e.g. CPU or memory).  Such metrics are built in to
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerResourceMetricStatus {
    /// name is the name of the resource in question.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// currentAverageUtilization is the current value of the average of the
    /// resource metric across all relevant pods, represented as a percentage of
    /// the requested value of the resource for the pods.  It will only be
    /// present if `targetAverageValue` was set in the corresponding metric
    /// specification.
    /// +optional
    #[prost(int32, optional, tag="2")]
    pub current_average_utilization: ::core::option::Option<i32>,
    /// currentAverageValue is the current value of the average of the
    /// resource metric across all relevant pods, as a raw value (instead of as
    /// a percentage of the request), similar to the "pods" metric source type.
    /// It will always be set, regardless of the corresponding metric specification.
    #[prost(message, optional, tag="3")]
    pub current_average_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// container is the name of the container in the pods of the scaling taget
    #[prost(string, optional, tag="4")]
    pub container: ::core::option::Option<::prost::alloc::string::String>,
}
/// CrossVersionObjectReference contains enough information to let you identify the referred resource.
/// +structType=atomic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossVersionObjectReference {
    /// Kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"
    #[prost(string, optional, tag="1")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the referent; More info: http://kubernetes.io/docs/user-guide/identifiers#names
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// API version of the referent
    /// +optional
    #[prost(string, optional, tag="3")]
    pub api_version: ::core::option::Option<::prost::alloc::string::String>,
}
/// ExternalMetricSource indicates how to scale on a metric not associated with
/// any Kubernetes object (for example length of queue in cloud
/// messaging service, or QPS from loadbalancer running outside of cluster).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalMetricSource {
    /// metricName is the name of the metric in question.
    #[prost(string, optional, tag="1")]
    pub metric_name: ::core::option::Option<::prost::alloc::string::String>,
    /// metricSelector is used to identify a specific time series
    /// within a given metric.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub metric_selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// targetValue is the target value of the metric (as a quantity).
    /// Mutually exclusive with TargetAverageValue.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub target_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// targetAverageValue is the target per-pod value of global metric (as a quantity).
    /// Mutually exclusive with TargetValue.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub target_average_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// ExternalMetricStatus indicates the current value of a global metric
/// not associated with any Kubernetes object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalMetricStatus {
    /// metricName is the name of a metric used for autoscaling in
    /// metric system.
    #[prost(string, optional, tag="1")]
    pub metric_name: ::core::option::Option<::prost::alloc::string::String>,
    /// metricSelector is used to identify a specific time series
    /// within a given metric.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub metric_selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// currentValue is the current value of the metric (as a quantity)
    #[prost(message, optional, tag="3")]
    pub current_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// currentAverageValue is the current value of metric averaged over autoscaled pods.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub current_average_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// configuration of a horizontal pod autoscaler.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HorizontalPodAutoscaler {
    /// Standard object metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// behaviour of autoscaler. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<HorizontalPodAutoscalerSpec>,
    /// current information about the autoscaler.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<HorizontalPodAutoscalerStatus>,
}
/// HorizontalPodAutoscalerCondition describes the state of
/// a HorizontalPodAutoscaler at a certain point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HorizontalPodAutoscalerCondition {
    /// type describes the current condition
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// status is the status of the condition (True, False, Unknown)
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// lastTransitionTime is the last time the condition transitioned from
    /// one status to another
    /// +optional
    #[prost(message, optional, tag="3")]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// reason is the reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// message is a human-readable explanation containing details about
    /// the transition
    /// +optional
    #[prost(string, optional, tag="5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// list of horizontal pod autoscaler objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HorizontalPodAutoscalerList {
    /// Standard list metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// list of horizontal pod autoscaler objects.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<HorizontalPodAutoscaler>,
}
/// specification of a horizontal pod autoscaler.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HorizontalPodAutoscalerSpec {
    /// reference to scaled resource; horizontal pod autoscaler will learn the current resource consumption
    /// and will set the desired number of pods by using its Scale subresource.
    #[prost(message, optional, tag="1")]
    pub scale_target_ref: ::core::option::Option<CrossVersionObjectReference>,
    /// minReplicas is the lower limit for the number of replicas to which the autoscaler
    /// can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the
    /// alpha feature gate HPAScaleToZero is enabled and at least one Object or External
    /// metric is configured.  Scaling is active as long as at least one metric value is
    /// available.
    /// +optional
    #[prost(int32, optional, tag="2")]
    pub min_replicas: ::core::option::Option<i32>,
    /// upper limit for the number of pods that can be set by the autoscaler; cannot be smaller than MinReplicas.
    #[prost(int32, optional, tag="3")]
    pub max_replicas: ::core::option::Option<i32>,
    /// target average CPU utilization (represented as a percentage of requested CPU) over all the pods;
    /// if not specified the default autoscaling policy will be used.
    /// +optional
    #[prost(int32, optional, tag="4")]
    pub target_cpu_utilization_percentage: ::core::option::Option<i32>,
}
/// current status of a horizontal pod autoscaler
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HorizontalPodAutoscalerStatus {
    /// most recent generation observed by this autoscaler.
    /// +optional
    #[prost(int64, optional, tag="1")]
    pub observed_generation: ::core::option::Option<i64>,
    /// last time the HorizontalPodAutoscaler scaled the number of pods;
    /// used by the autoscaler to control how often the number of pods is changed.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub last_scale_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// current number of replicas of pods managed by this autoscaler.
    #[prost(int32, optional, tag="3")]
    pub current_replicas: ::core::option::Option<i32>,
    /// desired number of replicas of pods managed by this autoscaler.
    #[prost(int32, optional, tag="4")]
    pub desired_replicas: ::core::option::Option<i32>,
    /// current average CPU utilization over all pods, represented as a percentage of requested CPU,
    /// e.g. 70 means that an average pod is using now 70% of its requested CPU.
    /// +optional
    #[prost(int32, optional, tag="5")]
    pub current_cpu_utilization_percentage: ::core::option::Option<i32>,
}
/// MetricSpec specifies how to scale based on a single metric
/// (only `type` and one other matching field should be set at once).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricSpec {
    /// type is the type of metric source.  It should be one of "ContainerResource",
    /// "External", "Object", "Pods" or "Resource", each mapping to a matching field in the object.
    /// Note: "ContainerResource" type is available on when the feature-gate
    /// HPAContainerMetrics is enabled
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// object refers to a metric describing a single kubernetes object
    /// (for example, hits-per-second on an Ingress object).
    /// +optional
    #[prost(message, optional, tag="2")]
    pub object: ::core::option::Option<ObjectMetricSource>,
    /// pods refers to a metric describing each pod in the current scale target
    /// (for example, transactions-processed-per-second).  The values will be
    /// averaged together before being compared to the target value.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub pods: ::core::option::Option<PodsMetricSource>,
    /// resource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing each pod in the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub resource: ::core::option::Option<ResourceMetricSource>,
    /// container resource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing a single container in each pod of the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    /// This is an alpha feature and can be enabled by the HPAContainerMetrics feature flag.
    /// +optional
    #[prost(message, optional, tag="7")]
    pub container_resource: ::core::option::Option<ContainerResourceMetricSource>,
    /// external refers to a global metric that is not associated
    /// with any Kubernetes object. It allows autoscaling based on information
    /// coming from components running outside of cluster
    /// (for example length of queue in cloud messaging service, or
    /// QPS from loadbalancer running outside of cluster).
    /// +optional
    #[prost(message, optional, tag="5")]
    pub external: ::core::option::Option<ExternalMetricSource>,
}
/// MetricStatus describes the last-read state of a single metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricStatus {
    /// type is the type of metric source.  It will be one of "ContainerResource",
    /// "External", "Object", "Pods" or "Resource", each corresponds to a matching field in the object.
    /// Note: "ContainerResource" type is available on when the feature-gate
    /// HPAContainerMetrics is enabled
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// object refers to a metric describing a single kubernetes object
    /// (for example, hits-per-second on an Ingress object).
    /// +optional
    #[prost(message, optional, tag="2")]
    pub object: ::core::option::Option<ObjectMetricStatus>,
    /// pods refers to a metric describing each pod in the current scale target
    /// (for example, transactions-processed-per-second).  The values will be
    /// averaged together before being compared to the target value.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub pods: ::core::option::Option<PodsMetricStatus>,
    /// resource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing each pod in the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub resource: ::core::option::Option<ResourceMetricStatus>,
    /// container resource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing a single container in each pod in the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    /// +optional
    #[prost(message, optional, tag="7")]
    pub container_resource: ::core::option::Option<ContainerResourceMetricStatus>,
    /// external refers to a global metric that is not associated
    /// with any Kubernetes object. It allows autoscaling based on information
    /// coming from components running outside of cluster
    /// (for example length of queue in cloud messaging service, or
    /// QPS from loadbalancer running outside of cluster).
    /// +optional
    #[prost(message, optional, tag="5")]
    pub external: ::core::option::Option<ExternalMetricStatus>,
}
/// ObjectMetricSource indicates how to scale on a metric describing a
/// kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectMetricSource {
    /// target is the described Kubernetes object.
    #[prost(message, optional, tag="1")]
    pub target: ::core::option::Option<CrossVersionObjectReference>,
    /// metricName is the name of the metric in question.
    #[prost(string, optional, tag="2")]
    pub metric_name: ::core::option::Option<::prost::alloc::string::String>,
    /// targetValue is the target value of the metric (as a quantity).
    #[prost(message, optional, tag="3")]
    pub target_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// selector is the string-encoded form of a standard kubernetes label selector for the given metric.
    /// When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping
    /// When unset, just the metricName will be used to gather metrics.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// averageValue is the target value of the average of the
    /// metric across all relevant pods (as a quantity)
    /// +optional
    #[prost(message, optional, tag="5")]
    pub average_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// ObjectMetricStatus indicates the current value of a metric describing a
/// kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectMetricStatus {
    /// target is the described Kubernetes object.
    #[prost(message, optional, tag="1")]
    pub target: ::core::option::Option<CrossVersionObjectReference>,
    /// metricName is the name of the metric in question.
    #[prost(string, optional, tag="2")]
    pub metric_name: ::core::option::Option<::prost::alloc::string::String>,
    /// currentValue is the current value of the metric (as a quantity).
    #[prost(message, optional, tag="3")]
    pub current_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// selector is the string-encoded form of a standard kubernetes label selector for the given metric
    /// When set in the ObjectMetricSource, it is passed as an additional parameter to the metrics server for more specific metrics scoping.
    /// When unset, just the metricName will be used to gather metrics.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// averageValue is the current value of the average of the
    /// metric across all relevant pods (as a quantity)
    /// +optional
    #[prost(message, optional, tag="5")]
    pub average_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// PodsMetricSource indicates how to scale on a metric describing each pod in
/// the current scale target (for example, transactions-processed-per-second).
/// The values will be averaged together before being compared to the target
/// value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodsMetricSource {
    /// metricName is the name of the metric in question
    #[prost(string, optional, tag="1")]
    pub metric_name: ::core::option::Option<::prost::alloc::string::String>,
    /// targetAverageValue is the target value of the average of the
    /// metric across all relevant pods (as a quantity)
    #[prost(message, optional, tag="2")]
    pub target_average_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// selector is the string-encoded form of a standard kubernetes label selector for the given metric
    /// When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping
    /// When unset, just the metricName will be used to gather metrics.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}
/// PodsMetricStatus indicates the current value of a metric describing each pod in
/// the current scale target (for example, transactions-processed-per-second).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodsMetricStatus {
    /// metricName is the name of the metric in question
    #[prost(string, optional, tag="1")]
    pub metric_name: ::core::option::Option<::prost::alloc::string::String>,
    /// currentAverageValue is the current value of the average of the
    /// metric across all relevant pods (as a quantity)
    #[prost(message, optional, tag="2")]
    pub current_average_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// selector is the string-encoded form of a standard kubernetes label selector for the given metric
    /// When set in the PodsMetricSource, it is passed as an additional parameter to the metrics server for more specific metrics scoping.
    /// When unset, just the metricName will be used to gather metrics.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}
/// ResourceMetricSource indicates how to scale on a resource metric known to
/// Kubernetes, as specified in requests and limits, describing each pod in the
/// current scale target (e.g. CPU or memory).  The values will be averaged
/// together before being compared to the target.  Such metrics are built in to
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source.  Only one "target" type
/// should be set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceMetricSource {
    /// name is the name of the resource in question.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// targetAverageUtilization is the target value of the average of the
    /// resource metric across all relevant pods, represented as a percentage of
    /// the requested value of the resource for the pods.
    /// +optional
    #[prost(int32, optional, tag="2")]
    pub target_average_utilization: ::core::option::Option<i32>,
    /// targetAverageValue is the target value of the average of the
    /// resource metric across all relevant pods, as a raw value (instead of as
    /// a percentage of the request), similar to the "pods" metric source type.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub target_average_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// ResourceMetricStatus indicates the current value of a resource metric known to
/// Kubernetes, as specified in requests and limits, describing each pod in the
/// current scale target (e.g. CPU or memory).  Such metrics are built in to
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceMetricStatus {
    /// name is the name of the resource in question.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// currentAverageUtilization is the current value of the average of the
    /// resource metric across all relevant pods, represented as a percentage of
    /// the requested value of the resource for the pods.  It will only be
    /// present if `targetAverageValue` was set in the corresponding metric
    /// specification.
    /// +optional
    #[prost(int32, optional, tag="2")]
    pub current_average_utilization: ::core::option::Option<i32>,
    /// currentAverageValue is the current value of the average of the
    /// resource metric across all relevant pods, as a raw value (instead of as
    /// a percentage of the request), similar to the "pods" metric source type.
    /// It will always be set, regardless of the corresponding metric specification.
    #[prost(message, optional, tag="3")]
    pub current_average_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// Scale represents a scaling request for a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scale {
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<ScaleSpec>,
    /// current status of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status. Read-only.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<ScaleStatus>,
}
/// ScaleSpec describes the attributes of a scale subresource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaleSpec {
    /// desired number of instances for the scaled object.
    /// +optional
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
}
/// ScaleStatus represents the current status of a scale subresource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaleStatus {
    /// actual number of observed instances of the scaled object.
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
    /// label query over pods that should match the replicas count. This is same
    /// as the label selector but in the string format to avoid introspection
    /// by clients. The string will be in the same format as the query-param syntax.
    /// More info about label selectors: http://kubernetes.io/docs/user-guide/labels#label-selectors
    /// +optional
    #[prost(string, optional, tag="2")]
    pub selector: ::core::option::Option<::prost::alloc::string::String>,
}
