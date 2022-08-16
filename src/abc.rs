/// Trait for all target types
pub trait Type {
    /// Name that can be read by humans, for example: IP address
    fn hname(&self) -> &'static str;

    /// Name that can be read by computer, for example: ip_address
    fn rname(&self) -> &'static str;

    /// Check if the target is compatible or not with the type
    fn validate(&self, target: String) -> bool;
}
