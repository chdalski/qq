use wayland_client::{
    globals::{registry_queue_init, GlobalListContents},
    protocol::{wl_data_device_manager, wl_registry},
    Connection, Dispatch, QueueHandle,
};
use wayland_protocols::wp;
struct ClientState {
    pub primary_selection_manager_state: Option,
    pub data_device_manager_state: Option<wl_data_device_manager::WlDataDeviceManager>,
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for ClientState {
    fn event(
        _state: &mut Self,
        _: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<ClientState>,
    ) {
        match event {
            wl_registry::Event::Global {
                name,
                interface,
                version,
            } => {
                println!("[{}] {} (v{})", name, interface, version);
            }
            _ => {}
        }
    }
}

impl Dispatch<wl_data_device_manager::WlDataDeviceManager, ()> for ClientState {
    fn event(
        state: &mut Self,
        proxy: &wl_data_device_manager::WlDataDeviceManager,
        event: <wl_data_device_manager::WlDataDeviceManager as wayland_client::Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        match event {
            _ => println!("EVENT"),
        }
    }
}

// The main function of our program
fn main() {
    // let conn = Connection::connect_to_env().unwrap();

    // let display = conn.display();

    // let mut event_queue = conn.new_event_queue();
    // let queue_handle = event_queue.handle();

    // let _registry = display.get_registry(&queue_handle, ());

    // let mut app_data = AppData {
    //     data_device_manager: None,
    // };
    // println!("Advertised globals:");
    // event_queue.blocking_dispatch(&mut app_data).unwrap();
    // while event_queue.blocking_dispatch(&mut app_data).unwrap() > 0 {}

    let conn = Connection::connect_to_env().unwrap();
    let mut client_state = ClientState {
        data_device_manager: None,
    };
    let (globals, mut queue) = registry_queue_init::<ClientState>(&conn).unwrap();

    client_state.data_device_manager = globals.bind(&queue.handle(), 2..=3, ()).ok();
    if let Some(bla) = client_state.data_device_manager {
        bla.get_data_device(seat, queue, ())
    }
    while queue.blocking_dispatch(&mut client_state).unwrap() > 0 {}
}
