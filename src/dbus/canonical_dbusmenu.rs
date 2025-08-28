use zbus::zvariant;

struct DBusMenu;


// ia{sv}
type IASV = (i32, std::collections::HashMap<String, zvariant::Value>);
// from kde dbus type
type DBusMenuItemList = Vec<IASV>;
type DBusMenuItemKeysList = Vec<(i32, Vec<String>)>;
type DBUSMenuLayoutItem = (IASV, Vec<zvariant::Value>);


#[interface(name = "com.canonical.dbusmenu")]
impl DBusMenu {
    #[zbus(property)]
    async fn version(&self) -> u32 {
        unimplemented!()
    }
    #[zbus(property)]
    async fn status(&self) -> String {
        unimplemented!()
    }

    #[zbus(signal)]
    async fn items_properties_updated(&self, item_list: DBusMenuItemList, item_keys_list: DBusMenuItemKeysList) -> () {
        unimplemented!()
    }

    #[zbus(signal)]
    async fn layout_updated(&self, revision: u32, parent_id: i32) -> () {
        unimplemented!()
    }

    #[zbus(signal)]
    async fn item_activation_requested(&self, id: i32, timestamp: u32) -> () {
        unimplemented!()
    }

    // noreply: true
    pub async fn event(&self, id: i32, event_id: String, data: zvariant::Value, timestamp: u32) -> () {
        unimplemented!()
    }

    pub async fn get_property(&self, id: i32, property: String) -> zvariant::Value {
        unimplemented!()
    }

    // out: (ia{sv}av) dbusmenulayoutitem
    pub async fn get_layout(&self, parent_id: i32, recursion_depth: i32, property_names: Vec<String>) -> (u32, DBUSMenuLayoutItem) {
        unimplemented!()
    }

    // out: a(ia{sv}) dbusmenuitemlist
    pub async fn get_group_properties(&self, ids: Vec<i32>, property_names: Vec<String>) -> DBusMenuItemList {
        unimplemented!()
    }

    pub async fn about_to_show(&self, id: i32) -> bool {
        unimplemented!()
    }
}
