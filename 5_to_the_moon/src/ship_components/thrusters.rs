/// Can only be ignited once
/// Has its own fuel
struct Booster {}

/// Can be ignited multiple times
/// Needs fuel from a fuel tank
struct Thruster {}

/// Can be ignited multiple times, can steer
/// Needs fuel from a fuel tank. Can only be ignited on the launch pad
/// or in space
struct Engine {}
