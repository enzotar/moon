// use std::collections::HashMap;

// use futures::executor::block_on;

// use serde::Deserialize;
// use serde_json::json;
// use sunshine_core::store::Datastore;

// use crate::{
//     input::{KeyboardSwitch, KeyboardSwitchEvent},
//     model::Model,
//     state::UiState,
//     Confirm, Coords, Input, MouseCoordsEvent, MouseSwitch, MouseSwitchEvent, RawEvent, Store,
// };

// #[derive(Clone, Debug, Default)]
// pub struct FlutterInput {
//     input: Input,
//     state: HashMap<FlutterInputDevice, FlutterInputState>,
// }

// impl FlutterInput {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

// #[derive(Clone, Copy, Debug, Default)]
// pub struct FlutterInputState {
//     pub buttons: u32,
//     pub x: i64,
//     pub y: i64,
//     pub lx: i64,
//     pub ly: i64,
// }

// #[derive(Clone, Debug, Eq, Hash, PartialEq)]
// pub enum FlutterInputDevice {
//     Mouse(u32),
// }

// #[derive(Debug, Deserialize)]
// #[serde(tag = "runtimeType", rename_all = "camelCase")]
// enum FlutterEvent {
//     #[serde(rename = "_TransformedPointerUpEvent")]
//     PointerUp(FlutterPointerEvent),
//     #[serde(rename = "_TransformedPointerDownEvent")]
//     PointerDown(FlutterPointerEvent),
//     #[serde(rename = "_TransformedPointerMoveEvent")]
//     PointerMove(FlutterPointerEvent),
//     #[serde(rename = "_TransformedPointerHoverEvent")]
//     PointerHover(FlutterPointerEvent),
//     #[serde(rename = "KeyDownEvent")]
//     KeyDown(FlutterKeyEvent),
//     #[serde(rename = "KeyUpEvent")]
//     KeyUp(FlutterKeyEvent),
// }

// #[derive(Debug, Deserialize)]
// pub struct FlutterPointerEvent {
//     #[serde(rename = "timestampMs")]
//     timestamp: i64,
//     device: u32,
//     kind: FlutterPointerKind,
//     buttons: u32,
//     #[serde(rename = "positionX")]
//     x: i64,
//     #[serde(rename = "positionY")]
//     y: i64,
//     #[serde(rename = "localPositionX")]
//     lx: i64,
//     #[serde(rename = "localPositionY")]
//     ly: i64,
// }

// #[derive(Debug, Deserialize)]
// pub enum FlutterPointerKind {
//     #[serde(rename = "PointerDeviceKind.mouse")]
//     Mouse,
// }

// #[derive(Clone, Debug, Deserialize)]
// struct FlutterKeyEvent {
//     #[serde(rename = "timestampMs")]
//     timestamp: f64,
//     #[serde(rename = "keyLabel")]
//     key_label: String,
// }

// impl FlutterInput {
//     //
//     fn with_pointer_event(&mut self, ev: FlutterPointerEvent, store: &mut Store, req_id: u64) {
//         let device = match ev.kind {
//             FlutterPointerKind::Mouse => FlutterInputDevice::Mouse(ev.device),
//         };

//         let x = ev.x;
//         let y = ev.y;
//         let lx = ev.lx;
//         let ly = ev.ly;
//         let buttons = ev.buttons;
//         let timestamp = ev.timestamp;

//         let mut state = self.state.entry(device).or_insert(FlutterInputState {
//             buttons: 0,
//             x,
//             y,
//             lx,
//             ly,
//         });
//         if state.x != x || state.y != y {
//             let raw_event = self.input.on_event(
//                 RawEvent::MouseCoords(MouseCoordsEvent::new(timestamp, Coords { x: lx, y: ly })),
//                 store,
//             );
//             Self::handle_events(raw_event, store, req_id)
//         }

//         for (button, mask) in [
//             (MouseSwitch("LeftMouseButton"), 1),
//             (MouseSwitch("RightMouseButton"), 2),
//             // (MouseButton::Auxiliary, 4),
//         ] {
//             if state.buttons & mask != buttons & mask {
//                 let raw_event = if buttons & mask != 0 {
//                     RawEvent::MousePress(MouseSwitchEvent::new(timestamp, button))
//                 } else {
//                     RawEvent::MouseRelease(MouseSwitchEvent::new(timestamp, button))
//                 };
//                 Self::handle_events(self.input.on_event(raw_event, store), store, req_id)
//             }
//         }

//         state.x = x;
//         state.y = y;
//         state.lx = lx;
//         state.ly = ly;
//         state.buttons = buttons;
//     }

//     //
//     fn with_keydown_event(&mut self, ev: FlutterKeyEvent, ui_store: &mut Model, req_id: u64) {
//         Self::handle_events(
//             self.input.on_event(
//                 RawEvent::KeyboardPress(KeyboardSwitchEvent::new(
//                     ev.timestamp as i64,
//                     KeyboardSwitch(ev.key_label),
//                 )),
//                 ui_store,
//             ),
//             ui_store,
//             req_id,
//         )
//     }

//     //
//     fn with_keyup_event(&mut self, ev: FlutterKeyEvent, ui_store: &mut Model, req_id: u64) {
//         Self::handle_events(
//             self.input.on_event(
//                 RawEvent::KeyboardRelease(KeyboardSwitchEvent::new(
//                     ev.timestamp as i64,
//                     KeyboardSwitch(ev.key_label),
//                 )),
//                 ui_store,
//             ),
//             ui_store,
//             req_id,
//         )
//     }

//     // converts JSON to FlutterInput
//     pub fn with_msg(&mut self, msg: &str, ui_store: &mut Model, req_id: u64) {
//         let ev: Result<FlutterEvent, _> = serde_json::from_str(&msg);

//         match ev {
//             Ok(
//                 FlutterEvent::PointerUp(ev)
//                 | FlutterEvent::PointerDown(ev)
//                 | FlutterEvent::PointerMove(ev)
//                 | FlutterEvent::PointerHover(ev),
//             ) => self.with_pointer_event(ev, ui_store, req_id),

//             Ok(FlutterEvent::KeyDown(ev)) => self.with_keydown_event(ev, ui_store, req_id),
//             Ok(FlutterEvent::KeyUp(ev)) => self.with_keyup_event(ev, ui_store, req_id),
//             Err(_) => {}
//         }

//         // let (input, events) = self.input.with_event(raw_event, &mut ui_store);
//     }

//     fn handle_events(events: impl Iterator<Item = AppEvent>, store: &mut Store, req_id: u64) {
//         for event in events {
//             // println!("{:?}", event);
//             match event {
//                 // BASIC SELECTION
//                 AppEvent::Unselect => {
//                     store.state.clear_selection();
//                     println!("unselect node");
//                     rid::post(Confirm::RefreshUI(req_id))
//                 }
//                 AppEvent::SelectNode(node_id) => {
//                     store.state.clear_selection();

//                     println!("selected node{:?}", node_id);

//                     store.state.add_to_selection(node_id);
//                     rid::post(Confirm::RefreshUI(req_id))
//                 }
//                 AppEvent::AddNodeToSelection(node_id) => {
//                     println!("selected node{:?}", node_id);

//                     store.state.add_to_selection(node_id);
//                     rid::post(Confirm::RefreshUI(req_id))
//                 }

//                 // CRUD
//                 AppEvent::CreateNode(coords) => {
//                     let coords = Coords {
//                         x: coords.x,
//                         y: coords.y,
//                     };

//                     store.create_starting_node_block(coords);
//                     // update view
//                     rid::post(Confirm::RefreshUI(req_id))
//                 }
//                 AppEvent::EditNode(_) => {}

//                 // SELECTION RECTANGLE
//                 AppEvent::MaybeStartSelection(start) => {
//                     store.state.ui_state = UiState::MaybeSelection(start);
//                 }
//                 AppEvent::NotASelection => {
//                     store.state.ui_state = UiState::Default;
//                 }
//                 AppEvent::StartSelection(start, _) => {
//                     store.state.ui_state = UiState::Selection(start);
//                 }
//                 AppEvent::ContinueSelection(_, _) => {}
//                 AppEvent::EndSelection(_, _) => {
//                     store.state.ui_state = UiState::Default; // question
//                 }
//                 AppEvent::CancelSelection => {
//                     println!("selection cancelled");
//                     store.state.ui_state = UiState::Default;
//                 }
//                 // MOVE
//                 AppEvent::MaybeStartMove(node, start) => {
//                     store.state.ui_state = UiState::MaybeMove(node, start);
//                 }
//                 AppEvent::NotAMove => {
//                     store.state.ui_state = UiState::Default;
//                 }
//                 AppEvent::StartMove(node_id, start_coords, coords) => {
//                     // select node
//                     store.state.clear_selection(); // clears selection on multi select // FIXME: workaround

//                     println!("selected node{:?}", node_id);
//                     store.state.add_to_selection(node_id.clone());

//                     store.state.ui_state = UiState::Move(start_coords);

//                     let node_ids: Vec<String> = store
//                         .state
//                         .selected_node_ids()
//                         .map(ToOwned::to_owned)
//                         .collect();

//                     for node_id in node_ids {
//                         let view = store.view.nodes.get_mut(&node_id).unwrap();
//                         view.x = view.origin_x + coords.x - start_coords.x;
//                         view.y = view.origin_y + coords.y - start_coords.y;

//                         //ui_store.view.set_node_start(&node_id);
//                         // update node in view cache
//                         //ui_store.view.update_view_coords(&node_id, start, coords);
//                     }
//                     rid::post(Confirm::RefreshUI(req_id))
//                 }
//                 AppEvent::ContinueMove(start_coords, coords) => {
//                     let node_ids: Vec<String> = store
//                         .state
//                         .selected_node_ids()
//                         .map(ToOwned::to_owned)
//                         .collect();

//                     for node_id in node_ids {
//                         let node = store
//                             .get_node(node_id)
//                             .unwrap()
//                             .move_to(start_coords, coords);

//                         // let view = model.view.nodes.get_mut(&node_id).unwrap();
//                         // view.x = view.origin_x + coords.x - start_coords.x;
//                         // view.y = view.origin_y + coords.y - start_coords.y;
//                         //ui_store
//                         //    .view
//                         //    .update_view_coords(&node_id, start_coords, coords);
//                     }

//                     rid::post(Confirm::RefreshUI(req_id))
//                 }
//                 AppEvent::EndMove(start_coords, coords) => {
//                     let node_ids: Vec<String> = store
//                         .state
//                         .selected_node_ids()
//                         .map(ToOwned::to_owned)
//                         .collect();

//                     for node_id in node_ids {
//                         store
//                             .get_node(node_id)
//                             .unwrap()
//                             .move_to(start_coords, coords);

//                         // add update view

//                         view.x = x;
//                         view.y = y;
//                         view.origin_x = view.x;
//                         view.origin_y = view.y;
//                         model.update_node(&node_id, Coords { x, y });
//                         // wrong coordinates
//                         // FIXME
//                         //update node in db
//                         //let coords =
//                         //    ui_store
//                         //        .view
//                         //        .update_view_coords(&node_id, start_coords, coords);
//                     }

//                     // ui_store.update_view_from_db();

//                     rid::post(Confirm::RefreshUI(req_id));
//                     store.state.ui_state = UiState::Default;
//                 }
//                 AppEvent::CancelMove => {
//                     store.state.ui_state = UiState::Default;
//                 }
//                 AppEvent::CommandInput => {
//                     println!("command input");
//                 }
//             }
//         }
//     }
// }
