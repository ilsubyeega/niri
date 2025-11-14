use std::collections::HashMap;

use zbus::object_server::SignalEmitter;
use zbus::zvariant::{self, OwnedValue};
use zbus::{fdo, interface};

use crate::dbus::Start;
pub struct DBusMenu;

#[interface(name = "com.canonical.dbusmenu")]
impl DBusMenu {
    pub async fn about_to_show(&self, id: i32) -> fdo::Result<bool> {
        debug!("about_to_show({id})");
        unimplemented!()
    }

    pub async fn event(
        &self,
        id: i32,
        event_id: &str,
        data: OwnedValue,
        timestamp: u32,
    ) -> fdo::Result<()> {
        debug!("event({id}, {event_id}, {data:?}, {timestamp})");
        unimplemented!()
    }

    pub async fn get_group_properties(
        &self,
        ids: Vec<i32>,
        property_names: Vec<String>,
    ) -> fdo::Result<Vec<(i32, HashMap<String, OwnedValue>)>> {
        debug!("get_group_properties({ids:?}, {property_names:?})");
        unimplemented!()
    }

    #[zbus(out_args("revision", "layout"))]
    pub async fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        property_names: Vec<String>,
    ) -> fdo::Result<(
        u32,
        (
            i32,
            HashMap<String, zvariant::OwnedValue>,
            Vec<zvariant::OwnedValue>,
        ),
    )> {
        debug!("get_layout({parent_id}, {recursion_depth}, {property_names:?})");
        unimplemented!()
    }

    pub async fn get_property(&self, id: i32, property: String) -> fdo::Result<OwnedValue> {
        debug!("get_property({id}, {property})");
        unimplemented!()
    }

    #[zbus(signal)]
    pub async fn item_activation_requested(
        ctxt: &SignalEmitter<'_>,
        id: i32,
        timestamp: u32,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn items_properties_updated(
        ctxt: &SignalEmitter<'_>,
        updated_props: Vec<(i32, HashMap<String, OwnedValue>)>,
        removed_props: Vec<(i32, Vec<String>)>,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn layout_updated(
        ctxt: &SignalEmitter<'_>,
        revision: u32,
        parent_id: i32,
    ) -> zbus::Result<()>;

    #[zbus(property)]
    async fn status(&self) -> fdo::Result<String> {
        debug!("status()");
        unimplemented!()
    }

    #[zbus(property)]
    async fn version(&self) -> fdo::Result<u32> {
        debug!("version()");
        unimplemented!()
    }
}

impl Start for DBusMenu {
    fn start(self) -> anyhow::Result<zbus::blocking::Connection> {
        let conn = zbus::blocking::Connection::session()?;
        // TODO: Implement this interface.
        conn.request_name("com.canonical.dbusmenu")?;
        let _id = conn.object_server().at("/com/canonical/dbusmenu", self)?;
        Ok(conn)
    }
}
