//! # D-Bus interface proxy for: `net.connman.iwd.p2p.ServiceManager`
//!
//! This code was generated by `zbus-xmlgen` `5.1.0` from D-Bus introspection data.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//! This type implements the [D-Bus standard interfaces], (`org.freedesktop.DBus.*`) for which the
//! following zbus API can be used:
//!
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! Consequently `zbus-xmlgen` did not generate code for the above interfaces.
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::proxy;
#[proxy(
    interface = "net.connman.iwd.p2p.ServiceManager",
    assume_defaults = true
)]
pub trait ServiceManager {
    /// RegisterDisplayService method
    fn register_display_service(
        &self,
        properties: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// UnregisterDisplayService method
    fn unregister_display_service(&self) -> zbus::Result<()>;
}
