use std::collections::HashMap;

use smithay::reexports::wayland_server::{
    Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New,
};
use wayland_backend::server::{ClientId, GlobalId};

use super::raw::kde_appmenu::v2::server;
use super::raw::kde_appmenu::v2::server::org_kde_kwin_appmenu_manager::OrgKdeKwinAppmenuManager;
use crate::protocols::raw::kde_appmenu::v2::server::org_kde_kwin_appmenu::OrgKdeKwinAppmenu;

const VERSION: u32 = 2;

#[derive(Debug)]
struct ClientData {
    manager: OrgKdeKwinAppmenuManager,
}

pub struct KDEAppMenuManagerState {
    global: GlobalId,
    clients: HashMap<ClientId, ClientData>,
}

pub struct KDEAppMenuManagerGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

pub struct KDEAppMenuState {}
pub struct KDEAppMenuGlobalData {}

pub trait KDEAppMenuManagerHandler {
    fn state(&mut self) -> &mut KDEAppMenuManagerState;
}
pub trait KDEAppMenuHandler {}

impl KDEAppMenuManagerState {
    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> Self
    where
        D: GlobalDispatch<OrgKdeKwinAppmenuManager, KDEAppMenuManagerGlobalData>,
        D: Dispatch<OrgKdeKwinAppmenuManager, ()>,
        D: KDEAppMenuManagerHandler,
        D: 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let global_data = KDEAppMenuManagerGlobalData {
            filter: Box::new(filter),
        };
        let clients = HashMap::new();
        let global = display.create_global::<D, OrgKdeKwinAppmenuManager, _>(VERSION, global_data);

        Self { clients, global }
    }
}
impl<D> GlobalDispatch<OrgKdeKwinAppmenuManager, KDEAppMenuManagerGlobalData, D>
    for KDEAppMenuManagerState
where
    D: GlobalDispatch<OrgKdeKwinAppmenuManager, KDEAppMenuManagerGlobalData>,
    D: Dispatch<OrgKdeKwinAppmenuManager, ()>,
    D: KDEAppMenuManagerHandler,
    D: 'static,
{
    fn bind(
        _state: &mut D,
        _handle: &DisplayHandle,
        _client: &Client,
        manager: New<OrgKdeKwinAppmenuManager>,
        _manager_state: &KDEAppMenuManagerGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        debug!("KDE AppMenu Manager bound {manager:?}");
        let manager = data_init.init(manager, ());
    }

    fn can_view(client: Client, global_data: &KDEAppMenuManagerGlobalData) -> bool {
        (global_data.filter)(&client)
    }
}

impl<D> Dispatch<OrgKdeKwinAppmenuManager, (), D> for KDEAppMenuManagerState
where
    D: Dispatch<OrgKdeKwinAppmenuManager, ()>,
    D: KDEAppMenuManagerHandler,
    D: 'static,
{
    fn request(
        _state: &mut D,
        client: &Client,
        _manager: &OrgKdeKwinAppmenuManager,
        request: server::org_kde_kwin_appmenu_manager::Request,
        _data: &(),
        display: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        debug!("kde appmenu manager request {request:?}");
        match request {
            server::org_kde_kwin_appmenu_manager::Request::Create { id, surface } => {
                // Here you would typically create a new KDEAppMenuState and associate it with the
                // client For now, we just log the creation
                debug!("Creating new KDEAppMenu instance: {:?}", id);
                
            }
            server::org_kde_kwin_appmenu_manager::Request::Release {} => {
                // Handle release if necessary
            }
        }

        // No requests to handle
    }
}

impl KDEAppMenuState {
    pub fn new<D, F>(display: &DisplayHandle) -> Self
    where
        D: Dispatch<OrgKdeKwinAppmenu, ()>,
        D: KDEAppMenuHandler,
    {
        Self {}
    }
}

impl<D> GlobalDispatch<OrgKdeKwinAppmenu, (), D> for KDEAppMenuState
where
    D: Dispatch<OrgKdeKwinAppmenu, ()>,
    D: KDEAppMenuHandler,
    D: 'static,
{
    fn bind(
        _state: &mut D,
        _handle: &DisplayHandle,
        _client: &Client,
        resource: New<OrgKdeKwinAppmenu>,
        _manager_state: &(),
        data_init: &mut DataInit<'_, D>,
    ) {
        debug!("KDE AppMenu bound {resource:?}");
        data_init.init(resource, ());
    }
}

impl<D> Dispatch<OrgKdeKwinAppmenu, (), D> for KDEAppMenuState
where
    D: Dispatch<OrgKdeKwinAppmenu, ()>,
    D: KDEAppMenuHandler,
    D: 'static,
{
    fn request(
        _state: &mut D,
        _client: &Client,
        _resource: &OrgKdeKwinAppmenu,
        request: server::org_kde_kwin_appmenu::Request,
        _data: &(),
        _dhandle: &DisplayHandle,
        _data_init: &mut DataInit<'_, D>,
    ) {
        debug!("kde appmenu request {request:?}");
        match request {
            server::org_kde_kwin_appmenu::Request::SetAddress {
                service_name,
                object_path,
            } => {
                debug!(
                    "SetAddress called with service_name: {}, object_path: {}",
                    service_name, object_path
                );
            }
            server::org_kde_kwin_appmenu::Request::Release {} => {
                // Handle release if necessary
            }
        }
        // No requests to handle
    }
}

#[macro_export]
macro_rules! delegate_kde_appmenu {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        // FIXME: implement this
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::raw::kde_appmenu::v2::server::org_kde_kwin_appmenu_manager::OrgKdeKwinAppmenuManager: $crate::protocols::kde_appmenu::KDEAppMenuManagerGlobalData
        ] => $crate::protocols::kde_appmenu::KDEAppMenuManagerState);

        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::raw::kde_appmenu::v2::server::org_kde_kwin_appmenu_manager::OrgKdeKwinAppmenuManager: ()
        ] => $crate::protocols::kde_appmenu::KDEAppMenuManagerState);

        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::raw::kde_appmenu::v2::server::org_kde_kwin_appmenu::OrgKdeKwinAppmenu: ()
        ] => $crate::protocols::kde_appmenu::KDEAppMenuState);

        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::raw::kde_appmenu::v2::server::org_kde_kwin_appmenu::OrgKdeKwinAppmenu: ()
        ] => $crate::protocols::kde_appmenu::KDEAppMenuState);
    };
}
