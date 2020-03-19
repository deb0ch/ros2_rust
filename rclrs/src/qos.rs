use crate::rcl_bindings::*;

pub enum QoSReliabilityPolicy {
    SystemDefault = 0,
    Reliable = 1,
    BestEffort = 2,
    Unknown = 3,
}

impl From<rmw_qos_reliability_policy_t> for QoSReliabilityPolicy {
    fn from(raw: rmw_qos_reliability_policy_t) -> Self {
        match raw {
            rmw_qos_reliability_policy_t::RMW_QOS_POLICY_RELIABILITY_SYSTEM_DEFAULT => QoSReliabilityPolicy::SystemDefault,
            rmw_qos_reliability_policy_t::RMW_QOS_POLICY_RELIABILITY_RELIABLE => QoSReliabilityPolicy::Reliable,
            rmw_qos_reliability_policy_t::RMW_QOS_POLICY_RELIABILITY_BEST_EFFORT => QoSReliabilityPolicy::BestEffort,
            rmw_qos_reliability_policy_t::RMW_QOS_POLICY_RELIABILITY_UNKNOWN => QoSReliabilityPolicy::Unknown,
        }
    }
}

pub enum QoSHistoryPolicy {
    SystemDefault = 0,
    KeepLast = 1,
    KeepAll = 2,
    Unknown = 3,
}

impl From<rmw_qos_history_policy_t> for QoSHistoryPolicy {
    fn from(raw: rmw_qos_history_policy_t) -> Self {
        match raw {
            rmw_qos_history_policy_t::RMW_QOS_POLICY_HISTORY_SYSTEM_DEFAULT => QoSHistoryPolicy::SystemDefault,
            rmw_qos_history_policy_t::RMW_QOS_POLICY_HISTORY_KEEP_LAST => QoSHistoryPolicy::KeepLast,
            rmw_qos_history_policy_t::RMW_QOS_POLICY_HISTORY_KEEP_ALL => QoSHistoryPolicy::KeepAll,
            rmw_qos_history_policy_t::RMW_QOS_POLICY_HISTORY_UNKNOWN => QoSHistoryPolicy::Unknown,
        }
    }
}

pub enum QoSDurabilityPolicy {
    SystemDefault = 0,
    TransientLocal = 1,
    Volatile = 2,
    Unknown = 3,
}

impl From<rmw_qos_durability_policy_t> for QoSDurabilityPolicy {
    fn from(raw: rmw_qos_durability_policy_t) -> Self {
        match raw {
            rmw_qos_durability_policy_t::RMW_QOS_POLICY_DURABILITY_SYSTEM_DEFAULT => QoSDurabilityPolicy::SystemDefault,
            rmw_qos_durability_policy_t::RMW_QOS_POLICY_DURABILITY_TRANSIENT_LOCAL => QoSDurabilityPolicy::TransientLocal,
            rmw_qos_durability_policy_t::RMW_QOS_POLICY_DURABILITY_VOLATILE => QoSDurabilityPolicy::Volatile,
            rmw_qos_durability_policy_t::RMW_QOS_POLICY_DURABILITY_UNKNOWN => QoSDurabilityPolicy::Unknown,
        }
    }
}

pub enum QoSLivelinessPolicy {
    SystemDefault = 0,
    Automatic = 1,
    ManualByNode = 2,
    ManualByTopic = 3,
    Unknown = 4,
}

impl From<rmw_qos_liveliness_policy_t> for QoSLivelinessPolicy {
    fn from(raw: rmw_qos_liveliness_policy_t) -> Self {
        match raw {
            rmw_qos_liveliness_policy_t::RMW_QOS_POLICY_LIVELINESS_SYSTEM_DEFAULT => QoSLivelinessPolicy::SystemDefault,
            rmw_qos_liveliness_policy_t::RMW_QOS_POLICY_LIVELINESS_AUTOMATIC => QoSLivelinessPolicy::Automatic,
            rmw_qos_liveliness_policy_t::RMW_QOS_POLICY_LIVELINESS_MANUAL_BY_NODE => QoSLivelinessPolicy::ManualByNode,
            rmw_qos_liveliness_policy_t::RMW_QOS_POLICY_LIVELINESS_MANUAL_BY_TOPIC => QoSLivelinessPolicy::ManualByTopic,
            rmw_qos_liveliness_policy_t::RMW_QOS_POLICY_LIVELINESS_UNKNOWN => QoSLivelinessPolicy::Unknown,
        }
    }
}

pub struct QoSTime {
    pub sec: u64,
    pub nsec: u64,
}

impl From<rmw_time_t> for QoSTime {
    fn from(raw: rmw_time_t) -> Self {
        QoSTime {
            sec: raw.sec,
            nsec: raw.nsec,
        }
    }
}

impl From<QoSTime> for rmw_time_t {
    fn from(raw: QoSTime) -> Self {
        rmw_time_t {
            sec: raw.sec,
            nsec: raw.nsec,
        }
    }
}

pub struct QoSProfile {
    pub history: QoSHistoryPolicy,
    pub depth: usize,
    pub reliability: QoSReliabilityPolicy,
    pub durability: QoSDurabilityPolicy,
    pub deadline: QoSTime,
    pub lifespan: QoSTime,
    pub liveliness: QoSLivelinessPolicy,
    pub liveliness_lease_duration: QoSTime,
    pub avoid_ros_namespace_conventions: bool,
}

pub const SYSTEM_DEFAULT: usize = 0;
pub const RMW_QOS_DEADLINE_DEFAULT: QoSTime = QoSTime { sec: 0, nsec: 0 };
pub const RMW_QOS_LIFESPAN_DEFAULT: QoSTime = QoSTime { sec: 0, nsec: 0 };
pub const RMW_QOS_LIVELINESS_LEASE_DURATION_DEFAULT: QoSTime = QoSTime { sec: 0, nsec: 0 };

pub const QOS_PROFILE_SENSOR_DATA: QoSProfile = QoSProfile {
    history: QoSHistoryPolicy::KeepLast,
    depth: 5,
    reliability: QoSReliabilityPolicy::BestEffort,
    durability: QoSDurabilityPolicy::Volatile,
    deadline: RMW_QOS_DEADLINE_DEFAULT,
    lifespan: RMW_QOS_LIFESPAN_DEFAULT,
    liveliness: QoSLivelinessPolicy::SystemDefault,
    liveliness_lease_duration: RMW_QOS_LIVELINESS_LEASE_DURATION_DEFAULT,
    avoid_ros_namespace_conventions: false,
};

pub const QOS_PROFILE_PARAMETERS: QoSProfile = QoSProfile {
    history: QoSHistoryPolicy::KeepLast,
    depth: 1000,
    reliability: QoSReliabilityPolicy::Reliable,
    durability: QoSDurabilityPolicy::Volatile,
    deadline: RMW_QOS_DEADLINE_DEFAULT,
    lifespan: RMW_QOS_LIFESPAN_DEFAULT,
    liveliness: QoSLivelinessPolicy::SystemDefault,
    liveliness_lease_duration: RMW_QOS_LIVELINESS_LEASE_DURATION_DEFAULT,
    avoid_ros_namespace_conventions: false,
};

pub const QOS_PROFILE_DEFAULT: QoSProfile = QoSProfile {
    history: QoSHistoryPolicy::KeepLast,
    depth: 10,
    reliability: QoSReliabilityPolicy::Reliable,
    durability: QoSDurabilityPolicy::Volatile,
    deadline: RMW_QOS_DEADLINE_DEFAULT,
    lifespan: RMW_QOS_LIFESPAN_DEFAULT,
    liveliness: QoSLivelinessPolicy::SystemDefault,
    liveliness_lease_duration: RMW_QOS_LIVELINESS_LEASE_DURATION_DEFAULT,
    avoid_ros_namespace_conventions: false,
};

pub const QOS_PROFILE_SERVICES_DEFAULT: QoSProfile = QoSProfile {
    history: QoSHistoryPolicy::KeepLast,
    depth: 10,
    reliability: QoSReliabilityPolicy::Reliable,
    durability: QoSDurabilityPolicy::Volatile,
    deadline: RMW_QOS_DEADLINE_DEFAULT,
    lifespan: RMW_QOS_LIFESPAN_DEFAULT,
    liveliness: QoSLivelinessPolicy::SystemDefault,
    liveliness_lease_duration: RMW_QOS_LIVELINESS_LEASE_DURATION_DEFAULT,
    avoid_ros_namespace_conventions: false,
};

pub const QOS_PROFILE_PARAMETER_EVENTS: QoSProfile = QoSProfile {
    history: QoSHistoryPolicy::KeepLast,
    depth: 1000,
    reliability: QoSReliabilityPolicy::Reliable,
    durability: QoSDurabilityPolicy::Volatile,
    deadline: RMW_QOS_DEADLINE_DEFAULT,
    lifespan: RMW_QOS_LIFESPAN_DEFAULT,
    liveliness: QoSLivelinessPolicy::SystemDefault,
    liveliness_lease_duration: RMW_QOS_LIVELINESS_LEASE_DURATION_DEFAULT,
    avoid_ros_namespace_conventions: false,
};

pub const QOS_PROFILE_SYSTEM_DEFAULT: QoSProfile = QoSProfile {
    history: QoSHistoryPolicy::SystemDefault,
    depth: SYSTEM_DEFAULT,
    reliability: QoSReliabilityPolicy::SystemDefault,
    durability: QoSDurabilityPolicy::SystemDefault,
    deadline: RMW_QOS_DEADLINE_DEFAULT,
    lifespan: RMW_QOS_LIFESPAN_DEFAULT,
    liveliness: QoSLivelinessPolicy::SystemDefault,
    liveliness_lease_duration: RMW_QOS_LIVELINESS_LEASE_DURATION_DEFAULT,
    avoid_ros_namespace_conventions: false,
};

pub const QOS_PROFILE_UNKNOWN: QoSProfile = QoSProfile {
    history: QoSHistoryPolicy::SystemDefault,
    depth: SYSTEM_DEFAULT,
    reliability: QoSReliabilityPolicy::Unknown,
    durability: QoSDurabilityPolicy::Unknown,
    deadline: RMW_QOS_DEADLINE_DEFAULT,
    lifespan: RMW_QOS_LIFESPAN_DEFAULT,
    liveliness: QoSLivelinessPolicy::Unknown,
    liveliness_lease_duration: RMW_QOS_LIVELINESS_LEASE_DURATION_DEFAULT,
    avoid_ros_namespace_conventions: false,
};

// pub const QOS_PROFILE_SENSOR_DATA: QoSProfile = unsafe { rmw_qos_profile_sensor_data.into() };
// pub const QOS_PROFILE_PARAMETERS: QoSProfile = unsafe { rmw_qos_profile_parameters.into() };
// pub const QOS_PROFILE_DEFAULT: QoSProfile = unsafe { rmw_qos_profile_default.into() };
// pub const QOS_PROFILE_SERVICES_DEFAULT: QoSProfile = unsafe { rmw_qos_profile_services_default.into() };
// pub const QOS_PROFILE_PARAMETER_EVENTS: QoSProfile = unsafe { rmw_qos_profile_parameter_events.into() };
// pub const QOS_PROFILE_SYSTEM_DEFAULT: QoSProfile = unsafe { rmw_qos_profile_system_default.into() };

// impl From<rmw_qos_profile_t> for QoSProfile {
//     fn from(qos_profile: rmw_qos_profile_t) -> Self {
//         QoSProfile {
//             history: qos_profile.history.into(),
//             depth: qos_profile.depth,
//             reliability: qos_profile.reliability.into(),
//             durability: qos_profile.durability.into(),
//             deadline: qos_profile.deadline.into(),
//             lifespan: qos_profile.lifespan.into(),
//             liveliness: qos_profile.liveliness.into(),
//             liveliness_lease_duration: qos_profile.liveliness_lease_duration.into(),
//             avoid_ros_namespace_conventions: qos_profile.avoid_ros_namespace_conventions,
//         }
//     }
// }

impl From<QoSProfile> for rmw_qos_profile_t {
    fn from(qos: QoSProfile) -> Self {
        Self {
            history: qos.history.into(),
            depth: qos.depth as usize,
            reliability: qos.reliability.into(),
            durability: qos.durability.into(),
            deadline: qos.deadline.into(),
            lifespan: qos.lifespan.into(),
            liveliness: qos.liveliness.into(),
            liveliness_lease_duration: qos.liveliness_lease_duration.into(),
            avoid_ros_namespace_conventions: qos.avoid_ros_namespace_conventions,
        }
    }
}

impl From<QoSHistoryPolicy> for rmw_qos_history_policy_t {
    fn from(policy: QoSHistoryPolicy) -> Self {
        match policy {
            QoSHistoryPolicy::SystemDefault => {
                rmw_qos_history_policy_t::RMW_QOS_POLICY_HISTORY_SYSTEM_DEFAULT
            }
            QoSHistoryPolicy::KeepLast => {
                rmw_qos_history_policy_t::RMW_QOS_POLICY_HISTORY_KEEP_LAST
            }
            QoSHistoryPolicy::KeepAll => {
                rmw_qos_history_policy_t::RMW_QOS_POLICY_HISTORY_KEEP_ALL
            },
            QoSHistoryPolicy::Unknown => {
                rmw_qos_history_policy_t::RMW_QOS_POLICY_HISTORY_UNKNOWN
            },
        }
    }
}

impl From<QoSReliabilityPolicy> for rmw_qos_reliability_policy_t {
    fn from(policy: QoSReliabilityPolicy) -> Self {
        match policy {
            QoSReliabilityPolicy::SystemDefault => {
                rmw_qos_reliability_policy_t::RMW_QOS_POLICY_RELIABILITY_SYSTEM_DEFAULT
            }
            QoSReliabilityPolicy::Reliable => {
                rmw_qos_reliability_policy_t::RMW_QOS_POLICY_RELIABILITY_RELIABLE
            }
            QoSReliabilityPolicy::BestEffort => {
                rmw_qos_reliability_policy_t::RMW_QOS_POLICY_RELIABILITY_BEST_EFFORT
            }
            QoSReliabilityPolicy::Unknown => {
                rmw_qos_reliability_policy_t::RMW_QOS_POLICY_RELIABILITY_UNKNOWN
            }
        }
    }
}

impl From<QoSDurabilityPolicy> for rmw_qos_durability_policy_t {
    fn from(policy: QoSDurabilityPolicy) -> Self {
        match policy {
            QoSDurabilityPolicy::SystemDefault => {
                rmw_qos_durability_policy_t::RMW_QOS_POLICY_DURABILITY_SYSTEM_DEFAULT
            }
            QoSDurabilityPolicy::TransientLocal => {
                rmw_qos_durability_policy_t::RMW_QOS_POLICY_DURABILITY_TRANSIENT_LOCAL
            }
            QoSDurabilityPolicy::Volatile => {
                rmw_qos_durability_policy_t::RMW_QOS_POLICY_DURABILITY_VOLATILE
            }
            QoSDurabilityPolicy::Unknown => {
                rmw_qos_durability_policy_t::RMW_QOS_POLICY_DURABILITY_UNKNOWN
            }
        }
    }
}

impl From<QoSLivelinessPolicy> for rmw_qos_liveliness_policy_t {
    fn from(policy: QoSLivelinessPolicy) -> Self {
        match policy {
            QoSLivelinessPolicy::SystemDefault => {
                rmw_qos_liveliness_policy_t::RMW_QOS_POLICY_LIVELINESS_SYSTEM_DEFAULT
            }
            QoSLivelinessPolicy::Automatic => {
                rmw_qos_liveliness_policy_t::RMW_QOS_POLICY_LIVELINESS_AUTOMATIC
            }
            QoSLivelinessPolicy::ManualByNode => {
                rmw_qos_liveliness_policy_t::RMW_QOS_POLICY_LIVELINESS_MANUAL_BY_NODE
            }
            QoSLivelinessPolicy::ManualByTopic => {
                rmw_qos_liveliness_policy_t::RMW_QOS_POLICY_LIVELINESS_MANUAL_BY_TOPIC
            }
            QoSLivelinessPolicy::Unknown => {
                rmw_qos_liveliness_policy_t::RMW_QOS_POLICY_LIVELINESS_UNKNOWN
            }
        }
    }
}
