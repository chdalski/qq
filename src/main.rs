use wayland_client::{
    globals::{registry_queue_init, GlobalListContents},
    protocol::{
        wl_data_device, wl_data_device_manager, wl_registry,
        wl_seat::{self, WlSeat},
    },
    Connection, Dispatch, Proxy, QueueHandle,
};
use wayland_protocols_wlr::data_control::v1::client::zwlr_data_control_device_v1::ZwlrDataControlDeviceV1;

struct ClientState {
    // pub primary_selection_manager_state: Option<,
    pub data_device_manager: wl_data_device_manager::WlDataDeviceManager,
    seats: Vec<wl_seat::WlSeat>,
    data_device: wl_data_device::WlDataDevice,
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
            _ => println!("Global Event: {:?}", event),
        }
    }
}

impl Dispatch<wl_data_device_manager::WlDataDeviceManager, ()> for ClientState {
    fn event(
        _state: &mut Self,
        _proxy: &wl_data_device_manager::WlDataDeviceManager,
        event: <wl_data_device_manager::WlDataDeviceManager as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match event {
            _ => println!("EVENT: WlDataDeviceManager: {:?}", event),
        }
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for ClientState {
    fn event(
        _state: &mut Self,
        _proxy: &wl_seat::WlSeat,
        event: <wl_seat::WlSeat as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match event {
            _ => println!("EVENT: WlSeat: {:?}", event),
        }
    }
}

impl Dispatch<wl_data_device::WlDataDevice, ()> for ClientState {
    fn event(
        _state: &mut Self,
        _proxy: &wl_data_device::WlDataDevice,
        event: <wl_data_device::WlDataDevice as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match event {
            _ => println!("EVENT: WlDataDevice: {:?}", event),
        }
    }
}

impl Dispatch<ZwlrDataControlDeviceV1, WlSeat> for ClientState {
    fn event(
        _state: &mut Self,
        _proxy: &ZwlrDataControlDeviceV1,
        event: <ZwlrDataControlDeviceV1 as Proxy>::Event,
        _data: &WlSeat,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match event {
            _ => println!("EVENT: ZwlrDataControlDeviceV1: {:?}", event),
        }
    }
}

fn main() {
    // connect to wayland
    let conn = Connection::connect_to_env().unwrap();

    // get globals and event queue
    let (globals, mut queue) = registry_queue_init::<ClientState>(&conn).unwrap();
    let data_device_manager: wl_data_device_manager::WlDataDeviceManager =
        match globals.bind(&queue.handle(), 2..=2, ()) {
            Ok(x) => x,
            Err(e) => panic!("bla: {}", e),
        };

    let registry = globals.registry();
    let seats = globals.contents().with_list(|gs| {
        gs.iter()
            .filter(|global| {
                global.interface == wl_seat::WlSeat::interface().name && global.version >= 2
            })
            .map(|g| {
                let seat: wl_seat::WlSeat = registry.bind(g.name, g.version, &queue.handle(), ());
                seat
            })
            .collect::<Vec<wl_seat::WlSeat>>()
    });
    let data_device = data_device_manager.get_data_device(&seats[0], &queue.handle(), ());

    let mut client_state = ClientState {
        data_device_manager,
        seats,
        data_device,
    };
    while queue.blocking_dispatch(&mut client_state).unwrap() > 0 {}
}
