<<<<<<< HEAD
// TODO:
// - all any timed_data (click, dbl_click, etc.)
// - allow specify event with modifiers that should not be overriden by events with more modifiers
//   for example cancel selection, cancel edge and other cancel events

=======
>>>>>>> master
use core::fmt::Debug;
use core::hash::Hash;
use std::collections::{HashMap, HashSet};

use input_core::*;
use input_more::*;
<<<<<<< HEAD
use serde::{Deserialize, Serialize};
=======
use serde::Deserialize;
>>>>>>> master

use crate::{
    event::{Coords, Event},
    model::{Model, NodeId, PortId},
<<<<<<< HEAD
    state::{Transform, UiState},
};

const MOUSE_SCROLL_DELTA_MULT: f64 = 257.0_f64 / 256.0; // zoom multiplier per one scroll delta

=======
    state::UiState,
};

>>>>>>> master
//type DurationMs = i64;
type TimestampMs = i64;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Switch {
    Keyboard(KeyboardSwitch),
    Mouse(MouseSwitch),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KeyboardSwitch(pub String);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MouseSwitch(pub &'static str);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
<<<<<<< HEAD
pub struct KeyboardTrigger(pub &'static str);
=======
pub struct KeyboardTrigger(&'static str);
>>>>>>> master

//#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
//pub struct KeyboardCoords; // ADDED
pub type KeyboardCoords = (); // ADDED

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
<<<<<<< HEAD
pub struct MouseTrigger(pub &'static str);
=======
pub struct MouseTrigger(&'static str);
>>>>>>> master

impl From<KeyboardSwitch> for Switch {
    fn from(switch: KeyboardSwitch) -> Self {
        Self::Keyboard(switch)
    }
}

impl From<MouseSwitch> for Switch {
    fn from(switch: MouseSwitch) -> Self {
        Self::Mouse(switch)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BasicAppEventBuilder {
    Unselect,
    RemoveNodes,
    CancelSelection,
<<<<<<< HEAD
    CancelTransformMove,
=======
    CancelViewportMove,
>>>>>>> master
    CancelNodeMove,
    CancelEdge,
    //StartCommandInput,
    //ApplyCommandInput,
    //ModifyCommandInput,
    //CancelCommandInput,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PointerAppEventBuilder {
    Unselect,
    SelectNode,
<<<<<<< HEAD
    AddOrRemoveNodeFromSelection,
=======
    AddNodeToSelection,
>>>>>>> master
    CreateNode,
    EditNode,
    RemoveNodes,
    //
    MaybeStartSelection,
    NotASelection,
    StartSelection,
    EndSelection,
    CancelSelection,
    ContinueSelection,
    //
<<<<<<< HEAD
    MaybeStartTransformMove,
    NotATransformMove,
    StartTransformMove,
    EndTransformMove,
    CancelTransfromMove,
    ContinueTransformMove,
=======
    MaybeStartViewportMove,
    NotAViewportMove,
    StartViewportMove,
    EndViewportMove,
    CancelViewportMove,
    ContinueViewportMove,
>>>>>>> master
    //
    MaybeStartNodeMove,
    NotANodeMove,
    StartNodeMove,
    EndNodeMove,
    CancelNodeMove,
    ContinueNodeMove,
    //
    MaybeStartEdge,
    NotAEdge,
    StartEdge,
    EndEdge,
    CancelEdge,
    ContinueEdge,
    //
<<<<<<< HEAD
    ScrollZoom,
    ScrollMoveScreen,
=======
>>>>>>> master
    //StartCommandInput,
    //ApplyCommandInput,
    //ModifyCommandInput,
    //CancelCommandInput,
}

fn filter_by_priority(events: Vec<Event>) -> impl Iterator<Item = Event> {
    let mut is_unselect_used = false;

    let mut is_select_node_used = false;
    let mut is_add_node_to_selection_used = false;
    let mut is_create_node_used = false;
    let mut is_edit_node_used = false;
    let mut is_remove_nodes_used = false;

    let mut is_maybe_start_selection_used = false;
    let mut is_not_a_selection_used = false;
    let mut is_start_selection_used = false;
    let mut is_end_selection_used = false;
    let mut is_cancel_selection_used = false;
    let mut is_continue_selection_used = false;

<<<<<<< HEAD
    let mut is_maybe_start_transform_move_used = false;
    let mut is_not_a_transform_move_used = false;
    let mut is_start_transform_move_used = false;
    let mut is_end_transform_move_used = false;
    let mut is_cancel_transform_move_used = false;
    let mut is_continue_transform_move_used = false;
=======
    let mut is_maybe_start_viewport_move_used = false;
    let mut is_not_a_viewport_move_used = false;
    let mut is_start_viewport_move_used = false;
    let mut is_end_viewport_move_used = false;
    let mut is_cancel_viewport_move_used = false;
    let mut is_continue_viewport_move_used = false;
>>>>>>> master

    let mut is_maybe_start_node_move_used = false;
    let mut is_not_a_node_move_used = false;
    let mut is_start_node_move_used = false;
    let mut is_end_node_move_used = false;
    let mut is_cancel_node_move_used = false;
    let mut is_continue_node_move_used = false;

    let mut is_maybe_start_edge_used = false;
    let mut is_not_a_edge_used = false;
    let mut is_start_edge_used = false;
    let mut is_end_edge_used = false;
    let mut is_cancel_edge_used = false;
    let mut is_continue_edge_used = false;

    let mut is_start_command_input_used = false;
    let mut is_apply_command_input_used = false;
    let mut is_modify_command_input_used = false;
    let mut is_cancel_command_input_used = false;

<<<<<<< HEAD
    let mut is_scroll_zoom_used = false;
    let mut is_scroll_move_screen_used = false;

=======
>>>>>>> master
    for event in &events {
        match event {
            Event::Unselect => is_unselect_used = true,
            Event::SelectNode(_) => is_select_node_used = true,
<<<<<<< HEAD
            Event::AddOrRemoveNodeToSelection(_) => is_add_node_to_selection_used = true,
=======
            Event::AddNodeToSelection(_) => is_add_node_to_selection_used = true,
>>>>>>> master
            Event::CreateNode(_) => is_create_node_used = true,
            Event::EditNode(_) => is_edit_node_used = true,
            Event::RemoveNodes(_) => is_remove_nodes_used = true,
            //
            Event::MaybeStartSelection(_) => is_maybe_start_selection_used = true,
            Event::NotASelection => is_not_a_selection_used = true,
            Event::StartSelection(_, _) => is_start_selection_used = true,
            Event::EndSelection(_, _) => is_end_selection_used = true,
            Event::CancelSelection => is_cancel_selection_used = true,
            Event::ContinueSelection(_, _) => is_continue_selection_used = true,
            //
<<<<<<< HEAD
            Event::MaybeStartTransformMove(_) => is_maybe_start_transform_move_used = true,
            Event::NotATransformMove => is_not_a_transform_move_used = true,
            Event::StartTransformMove(_, _) => is_start_transform_move_used = true,
            Event::EndTransformMove(_, _) => is_end_transform_move_used = true,
            Event::CancelTransformMove => is_cancel_transform_move_used = true,
            Event::ContinueTransformMove(_, _) => is_continue_transform_move_used = true,
            //
            Event::MaybeStartNodeMove(_, _) => is_maybe_start_node_move_used = true,
            Event::NotANodeMove => is_not_a_node_move_used = true,
            Event::StartNodeMove(_, _, _) => is_start_node_move_used = true,
=======
            Event::MaybeStartViewportMove(_) => is_maybe_start_viewport_move_used = true,
            Event::NotAViewportMove => is_not_a_viewport_move_used = true,
            Event::StartViewportMove(_, _) => is_start_viewport_move_used = true,
            Event::EndViewportMove(_, _) => is_end_viewport_move_used = true,
            Event::CancelViewportMove => is_cancel_viewport_move_used = true,
            Event::ContinueViewportMove(_, _) => is_continue_viewport_move_used = true,
            //
            Event::MaybeStartNodeMove(_, _) => is_maybe_start_node_move_used = true,
            Event::NotANodeMove => is_not_a_node_move_used = true,
            Event::StartNodeMove(_, _) => is_start_node_move_used = true,
>>>>>>> master
            Event::EndNodeMove(_, _) => is_end_node_move_used = true,
            Event::CancelNodeMove => is_cancel_node_move_used = true,
            Event::ContinueNodeMove(_, _) => is_continue_node_move_used = true,
            //
            Event::MaybeStartEdge(_) => is_maybe_start_edge_used = true,
            Event::NotAEdge => is_not_a_edge_used = true,
            Event::StartEdge(_, _) => is_start_edge_used = true,
            Event::EndEdge(_, _) => is_end_edge_used = true,
            Event::CancelEdge(_) => is_cancel_edge_used = true,
            Event::ContinueEdge(_, _) => is_continue_edge_used = true,
            //
<<<<<<< HEAD
            Event::ScrollZoom(_, _, _) => is_scroll_zoom_used = true,
            Event::ScrollMoveScreen(_, _) => is_scroll_move_screen_used = true,
            //
=======
>>>>>>> master
            //Event::StartCommandInput(_) => is_start_command_input_used = true,
            //Event::ApplyCommandInput(_) => is_apply_command_input_used = true,
            //Event::ModifyCommandInput(_) => is_modify_command_input_used = true,
            //Event::CancelCommandInput => is_cancel_command_input_used = true,
        }
    }

    let is_end_or_cancel_move_or_selection = is_end_selection_used
        || is_cancel_selection_used
        || is_end_node_move_used
        || is_cancel_node_move_used;
    let command_input = is_start_command_input_used
        | is_apply_command_input_used
        | is_modify_command_input_used
        | is_cancel_command_input_used;

    events.into_iter().filter(move |event| match event {
        Event::Unselect => {
            !command_input
                && !is_create_node_used
                && !is_select_node_used
<<<<<<< HEAD
                && !is_add_node_to_selection_used
                && !is_end_or_cancel_move_or_selection
        }
        Event::SelectNode(_) => !command_input && !is_create_node_used,
        Event::AddOrRemoveNodeToSelection(_) => !command_input && !is_create_node_used,
=======
                && !is_end_or_cancel_move_or_selection
        }
        Event::SelectNode(_) => !command_input && !is_create_node_used,
        Event::AddNodeToSelection(_) => !command_input && !is_create_node_used,
>>>>>>> master
        Event::CreateNode(_) => {
            !command_input && !is_edit_node_used && !is_end_or_cancel_move_or_selection
        }
        Event::EditNode(_) => !command_input && !is_end_or_cancel_move_or_selection,
        Event::RemoveNodes(_) => !command_input && !is_end_or_cancel_move_or_selection,
        //
        Event::MaybeStartSelection(_) => {
            !command_input
                && !is_maybe_start_node_move_used
                && !is_maybe_start_edge_used
                && !is_create_node_used
                && !is_edit_node_used
        }
<<<<<<< HEAD
        Event::MaybeStartTransformMove(_) => {
=======
        Event::MaybeStartViewportMove(_) => {
>>>>>>> master
            !command_input
                && !is_maybe_start_node_move_used
                && !is_maybe_start_edge_used
                && !is_create_node_used
                && !is_edit_node_used
        }
        Event::MaybeStartNodeMove(_, _) => {
            !command_input
                && !is_maybe_start_edge_used
                && !is_create_node_used
                && !is_edit_node_used
        }
        Event::MaybeStartEdge(_) => !command_input && !is_create_node_used && !is_edit_node_used,

        Event::StartSelection(_, _) => {
            !command_input
                && !is_start_node_move_used
                && !is_start_edge_used
                && !is_create_node_used
                && !is_edit_node_used
        }
<<<<<<< HEAD
        Event::StartTransformMove(_, _) => {
=======
        Event::StartViewportMove(_, _) => {
>>>>>>> master
            !command_input
                && !is_start_node_move_used
                && !is_start_edge_used
                && !is_create_node_used
                && !is_edit_node_used
        }
<<<<<<< HEAD
        Event::StartNodeMove(_, _, _) => {
=======
        Event::StartNodeMove(_, _) => {
>>>>>>> master
            !command_input && !is_start_edge_used && !is_create_node_used && !is_edit_node_used
        }
        Event::StartEdge(_, _) => !command_input && !is_create_node_used && !is_edit_node_used,

        Event::NotASelection
        | Event::EndSelection(_, _)
        | Event::CancelSelection
        | Event::ContinueSelection(_, _)
<<<<<<< HEAD
        | Event::NotATransformMove
        | Event::EndTransformMove(_, _)
        | Event::CancelTransformMove
        | Event::ContinueTransformMove(_, _)
=======
        | Event::NotAViewportMove
        | Event::EndViewportMove(_, _)
        | Event::CancelViewportMove
        | Event::ContinueViewportMove(_, _)
>>>>>>> master
        | Event::NotANodeMove
        | Event::CancelNodeMove
        | Event::ContinueNodeMove(_, _)
        | Event::EndNodeMove(_, _)
        | Event::NotAEdge
        | Event::CancelEdge(_)
        | Event::ContinueEdge(_, _)
<<<<<<< HEAD
        | Event::EndEdge(_, _)
        | Event::ScrollZoom(_, _, _)
        | Event::ScrollMoveScreen(_, _) => true,
=======
        | Event::EndEdge(_, _) => true,
>>>>>>> master
        //
        //Event::StartCommandInput(_) => true,
        //Event::ApplyCommandInput(_) => true,
        //Event::ModifyCommandInput(_) => true,
        //Event::CancelCommandInput => true,
    })
}

pub type KeyboardMapping = Mapping<KeyboardSwitch, KeyboardTrigger, Switch, BasicAppEventBuilder>;
pub type MouseMapping = Mapping<MouseSwitch, MouseTrigger, Switch, PointerAppEventBuilder>;

pub type KeyboardSwitchEvent = SwitchEvent<TimestampMs, KeyboardSwitch>;
pub type MouseSwitchEvent = SwitchEvent<TimestampMs, MouseSwitch>;
pub type KeyboardTriggerEvent = TriggerEvent<TimestampMs, KeyboardTrigger>;
pub type MouseTriggerEvent = TriggerEvent<TimestampMs, MouseTrigger>;
pub type KeyboardCoordsEvent = CoordsEvent<TimestampMs, KeyboardCoords>;
pub type MouseCoordsEvent = CoordsEvent<TimestampMs, Coords>;

// workaround for issue https://github.com/eqrion/cbindgen/issues/286
// pub use input_core::Modifiers as Mod;
pub type Modifiers = input_core::Modifiers<Switch>;
pub type KeyboardTimedState = TimedState<KeyboardSwitch>;
pub type MouseTimedState = TimedState<MouseSwitch>;

pub type KeyboardCoordsState = CoordsState<KeyboardCoords>;
pub type MouseCoordsState = CoordsState<Coords>;

pub type CustomScheduler<Sw, Re, Co> = DeviceSchedulerState<TimestampMs, Sw, Switch, Co, Re>;

pub type KeyboardLongPressScheduler =
    CustomScheduler<KeyboardSwitch, LongPressHandleRequest, KeyboardCoords>;
pub type KeyboardClickExactScheduler =
    CustomScheduler<KeyboardSwitch, ClickExactHandleRequest, KeyboardCoords>;
pub type MouseLongPressScheduler = CustomScheduler<MouseSwitch, LongPressHandleRequest, Coords>;
pub type MouseClickExactScheduler = CustomScheduler<MouseSwitch, ClickExactHandleRequest, Coords>;
pub type KeyboardPointerState = PointerState<KeyboardSwitch, KeyboardCoords>;
pub type MousePointerState = PointerState<MouseSwitch, Coords>;

// workaround for issue https://github.com/eqrion/cbindgen/issues/286
// pub use input_more::GlobalState as Global;
pub type GlobalState = input_more::GlobalState<
    Modifiers,
    KeyboardCoordsState,
    MouseCoordsState,
    KeyboardTimedState,
    MouseTimedState,
    KeyboardLongPressScheduler,
    KeyboardClickExactScheduler,
    MouseLongPressScheduler,
    MouseClickExactScheduler,
    KeyboardPointerState,
    MousePointerState,
>;

// workaround for issue https://github.com/eqrion/cbindgen/issues/286
// pub use input_more::GlobalMappingCache as GlobalMapCache;
pub type GlobalMappingCache = input_more::GlobalMappingCache<
    DeviceMappingCache<KeyboardSwitch, KeyboardTrigger, Switch, BasicAppEventBuilder>,
    DeviceMappingCache<MouseSwitch, MouseTrigger, Switch, PointerAppEventBuilder>,
    MappingModifiersCache<Switch>,
>;

#[derive(Clone, Debug)]
pub enum RawEvent {
    KeyboardPress(KeyboardSwitchEvent),
    KeyboardRelease(KeyboardSwitchEvent),
    KeyboardTrigger(KeyboardTriggerEvent),
    KeyboardCoords(KeyboardCoordsEvent),
    MousePress(MouseSwitchEvent),
    MouseRelease(MouseSwitchEvent),
    MouseTrigger(MouseTriggerEvent),
    MouseCoords(MouseCoordsEvent),
}

impl RawEvent {
    fn time(&self) -> TimestampMs {
        match self {
            RawEvent::KeyboardPress(event) => event.time,
            RawEvent::KeyboardRelease(event) => event.time,
            RawEvent::KeyboardTrigger(event) => event.time,
            RawEvent::KeyboardCoords(event) => event.time,
            RawEvent::MousePress(event) => event.time,
            RawEvent::MouseRelease(event) => event.time,
            RawEvent::MouseTrigger(event) => event.time,
            RawEvent::MouseCoords(event) => event.time,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Input {
<<<<<<< HEAD
    mouse_mapping_cache: GlobalMappingCache,
    touch_mapping_cache: GlobalMappingCache,
=======
    mapping_cache: GlobalMappingCache,
>>>>>>> master
    global_state: GlobalState,
    device_state: HashMap<Device, DeviceState>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Device {
    Mouse(u32),
}

<<<<<<< HEAD
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DeviceState {
    pub buttons: u32,
    pub x: f64,
    pub y: f64,
=======
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct DeviceState {
    pub buttons: u32,
    pub x: i64,
    pub y: i64,
>>>>>>> master
}

#[derive(Clone, Copy)]
pub struct Context<'a> {
    pub model: &'a Model,
    pub ui_state: &'a UiState,
<<<<<<< HEAD
    pub transform: Transform,
    pub selected_node_ids: &'a HashSet<NodeId>,
    pub mapping_kind: MappingKind,
}

#[derive(Clone, Copy)]
pub struct EventContext<'a> {
    pub model: &'a Model,
    pub ui_state: &'a UiState,
    pub selected_node_ids: &'a HashSet<NodeId>,
    pub last_scroll_dx: f64, // FIXME
    pub last_scroll_dy: f64, // FIXME
=======
    pub selected_node_ids: &'a HashSet<NodeId>,
>>>>>>> master
}

pub trait CapturedLifetime<'a> {}
impl<'a, T> CapturedLifetime<'a> for T {}

fn bindings_into_events<'a, Bi, Co>(
<<<<<<< HEAD
    context: EventContext<'a>,
=======
    context: Context<'a>,
>>>>>>> master
) -> impl CapturedLifetime<'a> + (FnMut((FilteredBindings<Switch, Bi>, Co)) -> Vec<Event>)
where
    Bi: BuildAppEvent<Co>,
    // FIXME
    Bi: core::fmt::Debug,
{
    move |(bindings, coords): (FilteredBindings<Switch, Bi>, Co)| {
        bindings.build(|builder| builder.build(&coords, context))
    }
}

fn timeout_bindings_into_events<'a>(
    result: GlobalStateWithTimeoutResult<
        'a,
        Switch,
        BasicAppEventBuilder,
        KeyboardCoords,
        PointerAppEventBuilder,
        Coords,
    >,
<<<<<<< HEAD
    context: EventContext<'a>,
=======
    context: Context<'a>,
>>>>>>> master
) -> impl Iterator<Item = Event> {
    let events = result
        .keyboard_long_press
        .into_iter()
        .map(bindings_into_events(context))
        .chain(
            result
                .keyboard_click_exact
                .into_iter()
                .map(bindings_into_events(context)),
        )
        .chain(
            result
                .mouse_long_press
                .into_iter()
                .map(bindings_into_events(context)),
        )
        .chain(
            result
                .mouse_click_exact
                .into_iter()
                .map(bindings_into_events(context)),
        )
        .flatten()
        .collect();
    filter_by_priority(events)
}

fn event_bindings_into_events<'a, Ti, Li, Bi, Co>(
    result: GlobalStateWithEventResult<Ti, Li>,
<<<<<<< HEAD
    context: EventContext<'a>,
=======
    context: Context<'a>,
>>>>>>> master
) -> impl CapturedLifetime<'a> + Iterator<Item = Event>
where
    Li: IntoIterator<Item = (FilteredBindings<'a, Switch, Bi>, Co)>,
    Bi: BuildAppEvent<Co>,
    // FIXME
    Bi: core::fmt::Debug,
    Bi: 'a,
{
    let events = result
        .bindings
        .into_iter()
        .flat_map(bindings_into_events(context))
        .collect();
    filter_by_priority(events)
}

<<<<<<< HEAD
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum MappingKind {
    Mouse,
    Touch,
}

fn generate_default_mapping(
    mapping_name: MappingKind,
) -> GlobalMapping<KeyboardMapping, MouseMapping> {
    let lmb = MouseSwitch("LeftMouseButton");
    let rmb = match mapping_name {
        MappingKind::Mouse => MouseSwitch("RightMouseButton"),
        MappingKind::Touch => MouseSwitch("NonExistentMouseButton"),
    };
    let click = Some(TimedEventData {
        kind: TimedReleaseEventKind::Click,
        num_possible_clicks: 1,
    });
    let dbl_click = Some(TimedEventData {
        kind: TimedReleaseEventKind::Click,
        num_possible_clicks: 2,
    });

    let shift_modifiers = {
        let mut modifiers = Modifiers::new();
        modifiers
            .on_press_event(Switch::Keyboard(KeyboardSwitch("Shift Left".to_owned())))
            .unwrap();
        modifiers
    };

    let keyboard_mapping = KeyboardMapping::new(
        [
            Binding::Release(SwitchBinding {
                switch: KeyboardSwitch("Escape".into()),
                modifiers: Modifiers::new(),
                timed_data: click,
                pointer_data: None,
                event: BasicAppEventBuilder::Unselect,
            }),
            /*Binding::Release(SwitchBinding {
                switch: KeyboardSwitch("/".into()),
                /*modifiers: {
                    let mut modifiers = Modifiers::new();
                    modifiers
                        .on_press_event(Switch::Keyboard(KeyboardSwitch(
                            "Control Left".to_owned(),
                        )))
                        .unwrap();
                    modifiers
                },*/
                modifiers: Modifiers::new(),
                timed_data: click,
                pointer_data: None,
                event: BasicAppEventBuilder::StartCommandInput,
            }),
            Binding::Release(SwitchBinding {
                switch: KeyboardSwitch("Enter".into()),
                modifiers: Modifiers::new(),
                timed_data: click,
                pointer_data: None,
                event: BasicAppEventBuilder::ApplyCommandInput,
            }),*/
            Binding::Release(SwitchBinding {
                switch: KeyboardSwitch("Escape".into()),
                modifiers: Modifiers::new(),
                timed_data: click,
                pointer_data: None,
                event: BasicAppEventBuilder::CancelSelection,
            }),
            Binding::Release(SwitchBinding {
                switch: KeyboardSwitch("Escape".into()),
                modifiers: Modifiers::new(),
                timed_data: click,
                pointer_data: None,
                event: BasicAppEventBuilder::CancelNodeMove,
            }),
            Binding::Release(SwitchBinding {
                switch: KeyboardSwitch("Escape".into()),
                modifiers: Modifiers::new(),
                timed_data: click,
                pointer_data: None,
                event: BasicAppEventBuilder::CancelEdge,
            }),
            /*Binding::Release(SwitchBinding {
                switch: KeyboardSwitch("Escape".into()),
                modifiers: Modifiers::new(),
                timed_data: click,
                pointer_data: None,
                event: BasicAppEventBuilder::CancelCommandInput,
            }),*/
            Binding::Release(SwitchBinding {
                switch: KeyboardSwitch("Delete".into()),
                modifiers: {
                    let mut modifiers = Modifiers::new();
                    modifiers
                        .on_press_event(Switch::Keyboard(KeyboardSwitch("Control Left".to_owned())))
                        .unwrap();
                    modifiers
                },
                timed_data: dbl_click, // FIXME
                pointer_data: None,
                event: BasicAppEventBuilder::RemoveNodes,
            }),
        ]
        .into_iter()
        .collect(),
    );
    let mouse_mapping = MouseMapping::new(
        [
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: click,
                pointer_data: None,
                event: PointerAppEventBuilder::Unselect,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: click,
                pointer_data: None,
                event: PointerAppEventBuilder::SelectNode,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(),
                timed_data: click,
                pointer_data: None,
                event: PointerAppEventBuilder::AddOrRemoveNodeFromSelection,
            }),
            // CREATE NODE
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: dbl_click,
                pointer_data: None,
                event: PointerAppEventBuilder::CreateNode,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: dbl_click,
                pointer_data: None,
                event: PointerAppEventBuilder::EditNode,
            }),
            // SELECTION
            Binding::Press(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: (),
                pointer_data: (),
                event: PointerAppEventBuilder::MaybeStartSelection,
            }),
            Binding::Coords(CoordsBinding {
                pointer_data: PointerMoveEventData {
                    switch: lmb,
                    kind: PointerMoveEventKind::DragStart,
                },
                modifiers: Modifiers::new(),
                event: PointerAppEventBuilder::StartSelection,
            }),
            Binding::Coords(CoordsBinding {
                pointer_data: PointerMoveEventData {
                    switch: lmb,
                    kind: PointerMoveEventKind::DragMove,
                },
                modifiers: Modifiers::new(),
                event: PointerAppEventBuilder::ContinueSelection,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::NotASelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: click,           // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotASelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,       // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotASelection,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::CancelSelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: click,           // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelSelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,       // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelSelection,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::NotASelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: click,                  // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotASelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,              // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotASelection,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::CancelSelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: click,                  // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelSelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,              // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelSelection,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: None,
                pointer_data: Some(PointerChangeEventData::DragEnd),
                event: PointerAppEventBuilder::EndSelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: click, // FIXME
                pointer_data: Some(PointerChangeEventData::DragEnd),
                event: PointerAppEventBuilder::EndSelection,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: dbl_click, // FIXME
                pointer_data: Some(PointerChangeEventData::DragEnd),
                event: PointerAppEventBuilder::EndSelection,
            }),
            // TRANSFORM MOVE
            Binding::Press(SwitchBinding {
                switch: rmb,
                modifiers: Modifiers::new(),
                timed_data: (),
                pointer_data: (),
                event: PointerAppEventBuilder::MaybeStartTransformMove,
            }),
            Binding::Coords(CoordsBinding {
                pointer_data: PointerMoveEventData {
                    switch: rmb,
                    kind: PointerMoveEventKind::DragStart,
                },
                modifiers: Modifiers::new(),
                event: PointerAppEventBuilder::StartTransformMove,
            }),
            Binding::Coords(CoordsBinding {
                pointer_data: PointerMoveEventData {
                    switch: rmb,
                    kind: PointerMoveEventKind::DragMove,
                },
                modifiers: Modifiers::new(),
                event: PointerAppEventBuilder::ContinueTransformMove,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::NotATransformMove,
            }),
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: click,           // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotATransformMove,
            }),
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,       // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotATransformMove,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::CancelTransfromMove,
            }),
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: click,           // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelTransfromMove,
            }),
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,       // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelTransfromMove,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::NotATransformMove,
            }),
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: click,                  // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotATransformMove,
            }),
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,              // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotATransformMove,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::CancelTransfromMove,
            }),
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: click,                  // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelTransfromMove,
            }),
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,              // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelTransfromMove,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: Modifiers::new(),
                timed_data: None,
                pointer_data: Some(PointerChangeEventData::DragEnd),
                event: PointerAppEventBuilder::EndTransformMove,
            }),
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: Modifiers::new(),
                timed_data: click, // FIXME
                pointer_data: Some(PointerChangeEventData::DragEnd),
                event: PointerAppEventBuilder::EndTransformMove,
            }),
            Binding::Release(SwitchBinding {
                switch: rmb,
                modifiers: Modifiers::new(),
                timed_data: dbl_click, // FIXME
                pointer_data: Some(PointerChangeEventData::DragEnd),
                event: PointerAppEventBuilder::EndTransformMove,
            }),
            // NODE MOVE
            Binding::Press(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: (),
                pointer_data: (),
                event: PointerAppEventBuilder::MaybeStartNodeMove,
            }),
            Binding::Coords(CoordsBinding {
                pointer_data: PointerMoveEventData {
                    switch: lmb,
                    kind: PointerMoveEventKind::DragStart,
                },
                modifiers: Modifiers::new(),
                event: PointerAppEventBuilder::StartNodeMove,
            }),
            Binding::Coords(CoordsBinding {
                pointer_data: PointerMoveEventData {
                    switch: lmb,
                    kind: PointerMoveEventKind::DragMove,
                },
                modifiers: Modifiers::new(),
                event: PointerAppEventBuilder::ContinueNodeMove,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::NotANodeMove,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: click,           // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotANodeMove,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,       // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotANodeMove,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::CancelNodeMove,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: click,           // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelNodeMove,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,       // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelNodeMove,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::NotANodeMove,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: click,                  // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotANodeMove,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,              // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotANodeMove,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::CancelNodeMove,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: click,                  // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelNodeMove,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,              // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelNodeMove,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: None,
                pointer_data: Some(PointerChangeEventData::DragEnd),
                event: PointerAppEventBuilder::EndNodeMove,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: click, // FIXME
                pointer_data: Some(PointerChangeEventData::DragEnd),
                event: PointerAppEventBuilder::EndNodeMove,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: dbl_click, // FIXME
                pointer_data: Some(PointerChangeEventData::DragEnd),
                event: PointerAppEventBuilder::EndNodeMove,
            }),
            // EDGE
            Binding::Press(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: (),
                pointer_data: (),
                event: PointerAppEventBuilder::MaybeStartEdge,
            }),
            Binding::Coords(CoordsBinding {
                pointer_data: PointerMoveEventData {
                    switch: lmb,
                    kind: PointerMoveEventKind::DragStart,
                },
                modifiers: Modifiers::new(),
                event: PointerAppEventBuilder::StartEdge,
            }),
            Binding::Coords(CoordsBinding {
                pointer_data: PointerMoveEventData {
                    switch: lmb,
                    kind: PointerMoveEventKind::DragMove,
                },
                modifiers: Modifiers::new(),
                event: PointerAppEventBuilder::ContinueEdge,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::NotAEdge,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: click, // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotAEdge,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: dbl_click, // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotAEdge,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::CancelEdge,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: click, // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelEdge,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: dbl_click, // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelEdge,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::NotAEdge,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: click,                  // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotAEdge,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,              // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::NotAEdge,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: None,
                pointer_data: None,
                event: PointerAppEventBuilder::CancelEdge,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: click,                  // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelEdge,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: shift_modifiers.clone(), // FIXME: do not override with event with more modifiers
                timed_data: dbl_click,              // FIXME
                pointer_data: None,
                event: PointerAppEventBuilder::CancelEdge,
            }),
            //
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: None,
                pointer_data: Some(PointerChangeEventData::DragEnd),
                event: PointerAppEventBuilder::EndEdge,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: click, // FIXME
                pointer_data: Some(PointerChangeEventData::DragEnd),
                event: PointerAppEventBuilder::EndEdge,
            }),
            Binding::Release(SwitchBinding {
                switch: lmb,
                modifiers: Modifiers::new(),
                timed_data: dbl_click, // FIXME
                pointer_data: Some(PointerChangeEventData::DragEnd),
                event: PointerAppEventBuilder::EndEdge,
            }),
        ]
        .into_iter()
        .chain(match mapping_name {
            MappingKind::Mouse => vec![
                //
                // FIXME: scoll should provide additions information,
                // but event builder can not contain it,
                // so event can not reach it.
                Binding::Trigger(TriggerBinding {
                    trigger: MouseTrigger("scroll"),
                    modifiers: Modifiers::new(),
                    event: PointerAppEventBuilder::ScrollZoom,
                }),
            ],
            MappingKind::Touch => vec![Binding::Trigger(TriggerBinding {
                trigger: MouseTrigger("scroll"),
                modifiers: Modifiers::new(),
                event: PointerAppEventBuilder::ScrollMoveScreen,
            })],
        })
        .collect(),
    );
    GlobalMapping {
        keyboard: keyboard_mapping,
        mouse: mouse_mapping,
    }
}

impl Default for Input {
    fn default() -> Self {
        let mouse_mapping = generate_default_mapping(MappingKind::Mouse);
        let touch_mapping = generate_default_mapping(MappingKind::Touch);

        let mouse_mapping_cache = GlobalMappingCache::from_mapping(mouse_mapping);
        let touch_mapping_cache = GlobalMappingCache::from_mapping(touch_mapping);
=======
impl Default for Input {
    fn default() -> Self {
        let lmb = MouseSwitch("LeftMouseButton");
        let rmb = MouseSwitch("RightMouseButton");
        let click = Some(TimedEventData {
            kind: TimedReleaseEventKind::Click,
            num_possible_clicks: 1,
        });
        let dbl_click = Some(TimedEventData {
            kind: TimedReleaseEventKind::Click,
            num_possible_clicks: 2,
        });

        let keyboard_mapping = KeyboardMapping::new(
            [
                Binding::Release(SwitchBinding {
                    switch: KeyboardSwitch("Escape".into()),
                    modifiers: Modifiers::new(),
                    timed_data: click,
                    pointer_data: None,
                    event: BasicAppEventBuilder::Unselect,
                }),
                /*Binding::Release(SwitchBinding {
                    switch: KeyboardSwitch("/".into()),
                    /*modifiers: {
                        let mut modifiers = Modifiers::new();
                        modifiers
                            .on_press_event(Switch::Keyboard(KeyboardSwitch(
                                "Control Left".to_owned(),
                            )))
                            .unwrap();
                        modifiers
                    },*/
                    modifiers: Modifiers::new(),
                    timed_data: click,
                    pointer_data: None,
                    event: BasicAppEventBuilder::StartCommandInput,
                }),
                Binding::Release(SwitchBinding {
                    switch: KeyboardSwitch("Enter".into()),
                    modifiers: Modifiers::new(),
                    timed_data: click,
                    pointer_data: None,
                    event: BasicAppEventBuilder::ApplyCommandInput,
                }),*/
                Binding::Release(SwitchBinding {
                    switch: KeyboardSwitch("Escape".into()),
                    modifiers: Modifiers::new(),
                    timed_data: click,
                    pointer_data: None,
                    event: BasicAppEventBuilder::CancelSelection,
                }),
                Binding::Release(SwitchBinding {
                    switch: KeyboardSwitch("Escape".into()),
                    modifiers: Modifiers::new(),
                    timed_data: click,
                    pointer_data: None,
                    event: BasicAppEventBuilder::CancelNodeMove,
                }),
                Binding::Release(SwitchBinding {
                    switch: KeyboardSwitch("Escape".into()),
                    modifiers: Modifiers::new(),
                    timed_data: click,
                    pointer_data: None,
                    event: BasicAppEventBuilder::CancelEdge,
                }),
                /*Binding::Release(SwitchBinding {
                    switch: KeyboardSwitch("Escape".into()),
                    modifiers: Modifiers::new(),
                    timed_data: click,
                    pointer_data: None,
                    event: BasicAppEventBuilder::CancelCommandInput,
                }),*/
                Binding::Release(SwitchBinding {
                    switch: KeyboardSwitch("Delete".into()),
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click, // FIXME
                    pointer_data: None,
                    event: BasicAppEventBuilder::RemoveNodes,
                }),
            ]
            .into_iter()
            .collect(),
        );
        let mouse_mapping = MouseMapping::new(
            [
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: click,
                    pointer_data: None,
                    event: PointerAppEventBuilder::Unselect,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: click,
                    pointer_data: None,
                    event: PointerAppEventBuilder::SelectNode,
                }),
                //
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: {
                        let mut modifiers = Modifiers::new();
                        modifiers
                            .on_press_event(Switch::Keyboard(KeyboardSwitch(
                                "Shift Left".to_owned(),
                            )))
                            .unwrap();
                        modifiers
                    },
                    timed_data: click,
                    pointer_data: None,
                    event: PointerAppEventBuilder::AddNodeToSelection,
                }),
                // CREATE NODE
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click,
                    pointer_data: None,
                    event: PointerAppEventBuilder::CreateNode,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click,
                    pointer_data: None,
                    event: PointerAppEventBuilder::EditNode,
                }),
                // SELECTION
                Binding::Press(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: (),
                    pointer_data: (),
                    event: PointerAppEventBuilder::MaybeStartSelection,
                }),
                Binding::Coords(CoordsBinding {
                    pointer_data: PointerMoveEventData {
                        switch: lmb,
                        kind: PointerMoveEventKind::DragStart,
                    },
                    modifiers: Modifiers::new(),
                    event: PointerAppEventBuilder::StartSelection,
                }),
                Binding::Coords(CoordsBinding {
                    pointer_data: PointerMoveEventData {
                        switch: lmb,
                        kind: PointerMoveEventKind::DragMove,
                    },
                    modifiers: Modifiers::new(),
                    event: PointerAppEventBuilder::ContinueSelection,
                }),
                //
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: None,
                    pointer_data: None,
                    event: PointerAppEventBuilder::NotASelection,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::NotASelection,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::NotASelection,
                }),
                //
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: None,
                    pointer_data: None,
                    event: PointerAppEventBuilder::CancelSelection,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::CancelSelection,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::CancelSelection,
                }),
                //
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: None,
                    pointer_data: Some(PointerChangeEventData::DragEnd),
                    event: PointerAppEventBuilder::EndSelection,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: click, // FIXME
                    pointer_data: Some(PointerChangeEventData::DragEnd),
                    event: PointerAppEventBuilder::EndSelection,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click, // FIXME
                    pointer_data: Some(PointerChangeEventData::DragEnd),
                    event: PointerAppEventBuilder::EndSelection,
                }),
                // VIEWPORT MOVE
                Binding::Press(SwitchBinding {
                    switch: rmb,
                    modifiers: Modifiers::new(),
                    timed_data: (),
                    pointer_data: (),
                    event: PointerAppEventBuilder::MaybeStartViewportMove,
                }),
                Binding::Coords(CoordsBinding {
                    pointer_data: PointerMoveEventData {
                        switch: rmb,
                        kind: PointerMoveEventKind::DragStart,
                    },
                    modifiers: Modifiers::new(),
                    event: PointerAppEventBuilder::StartViewportMove,
                }),
                Binding::Coords(CoordsBinding {
                    pointer_data: PointerMoveEventData {
                        switch: rmb,
                        kind: PointerMoveEventKind::DragMove,
                    },
                    modifiers: Modifiers::new(),
                    event: PointerAppEventBuilder::ContinueViewportMove,
                }),
                //
                Binding::Release(SwitchBinding {
                    switch: rmb,
                    modifiers: Modifiers::new(),
                    timed_data: None,
                    pointer_data: None,
                    event: PointerAppEventBuilder::NotAViewportMove,
                }),
                Binding::Release(SwitchBinding {
                    switch: rmb,
                    modifiers: Modifiers::new(),
                    timed_data: click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::NotAViewportMove,
                }),
                Binding::Release(SwitchBinding {
                    switch: rmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::NotAViewportMove,
                }),
                //
                Binding::Release(SwitchBinding {
                    switch: rmb,
                    modifiers: Modifiers::new(),
                    timed_data: None,
                    pointer_data: None,
                    event: PointerAppEventBuilder::CancelViewportMove,
                }),
                Binding::Release(SwitchBinding {
                    switch: rmb,
                    modifiers: Modifiers::new(),
                    timed_data: click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::CancelViewportMove,
                }),
                Binding::Release(SwitchBinding {
                    switch: rmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::CancelViewportMove,
                }),
                //
                Binding::Release(SwitchBinding {
                    switch: rmb,
                    modifiers: Modifiers::new(),
                    timed_data: None,
                    pointer_data: Some(PointerChangeEventData::DragEnd),
                    event: PointerAppEventBuilder::EndViewportMove,
                }),
                Binding::Release(SwitchBinding {
                    switch: rmb,
                    modifiers: Modifiers::new(),
                    timed_data: click, // FIXME
                    pointer_data: Some(PointerChangeEventData::DragEnd),
                    event: PointerAppEventBuilder::EndViewportMove,
                }),
                Binding::Release(SwitchBinding {
                    switch: rmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click, // FIXME
                    pointer_data: Some(PointerChangeEventData::DragEnd),
                    event: PointerAppEventBuilder::EndViewportMove,
                }),
                // NODE MOVE
                Binding::Press(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: (),
                    pointer_data: (),
                    event: PointerAppEventBuilder::MaybeStartNodeMove,
                }),
                Binding::Coords(CoordsBinding {
                    pointer_data: PointerMoveEventData {
                        switch: lmb,
                        kind: PointerMoveEventKind::DragStart,
                    },
                    modifiers: Modifiers::new(),
                    event: PointerAppEventBuilder::StartNodeMove,
                }),
                Binding::Coords(CoordsBinding {
                    pointer_data: PointerMoveEventData {
                        switch: lmb,
                        kind: PointerMoveEventKind::DragMove,
                    },
                    modifiers: Modifiers::new(),
                    event: PointerAppEventBuilder::ContinueNodeMove,
                }),
                //
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: None,
                    pointer_data: None,
                    event: PointerAppEventBuilder::NotANodeMove,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::NotANodeMove,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::NotANodeMove,
                }),
                //
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: None,
                    pointer_data: None,
                    event: PointerAppEventBuilder::CancelNodeMove,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::CancelNodeMove,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::CancelNodeMove,
                }),
                //
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: None,
                    pointer_data: Some(PointerChangeEventData::DragEnd),
                    event: PointerAppEventBuilder::EndNodeMove,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: click, // FIXME
                    pointer_data: Some(PointerChangeEventData::DragEnd),
                    event: PointerAppEventBuilder::EndNodeMove,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click, // FIXME
                    pointer_data: Some(PointerChangeEventData::DragEnd),
                    event: PointerAppEventBuilder::EndNodeMove,
                }),
                // EDGE
                Binding::Press(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: (),
                    pointer_data: (),
                    event: PointerAppEventBuilder::MaybeStartEdge,
                }),
                Binding::Coords(CoordsBinding {
                    pointer_data: PointerMoveEventData {
                        switch: lmb,
                        kind: PointerMoveEventKind::DragStart,
                    },
                    modifiers: Modifiers::new(),
                    event: PointerAppEventBuilder::StartEdge,
                }),
                Binding::Coords(CoordsBinding {
                    pointer_data: PointerMoveEventData {
                        switch: lmb,
                        kind: PointerMoveEventKind::DragMove,
                    },
                    modifiers: Modifiers::new(),
                    event: PointerAppEventBuilder::ContinueEdge,
                }),
                //
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: None,
                    pointer_data: None,
                    event: PointerAppEventBuilder::NotAEdge,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::NotAEdge,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::NotAEdge,
                }),
                //
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: None,
                    pointer_data: None,
                    event: PointerAppEventBuilder::CancelEdge,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::CancelEdge,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click, // FIXME
                    pointer_data: None,
                    event: PointerAppEventBuilder::CancelEdge,
                }),
                //
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: None,
                    pointer_data: Some(PointerChangeEventData::DragEnd),
                    event: PointerAppEventBuilder::EndEdge,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: click, // FIXME
                    pointer_data: Some(PointerChangeEventData::DragEnd),
                    event: PointerAppEventBuilder::EndEdge,
                }),
                Binding::Release(SwitchBinding {
                    switch: lmb,
                    modifiers: Modifiers::new(),
                    timed_data: dbl_click, // FIXME
                    pointer_data: Some(PointerChangeEventData::DragEnd),
                    event: PointerAppEventBuilder::EndEdge,
                }),
            ]
            .into_iter()
            .collect(),
        );
        let mapping = GlobalMapping {
            keyboard: keyboard_mapping,
            mouse: mouse_mapping,
        };

        let mapping_cache = GlobalMappingCache::from_mapping(mapping);
>>>>>>> master

        let global_state = GlobalState::new(
            Modifiers::default(),
            KeyboardCoordsState::with_coords(()),
<<<<<<< HEAD
            MouseCoordsState::with_coords(Coords { x: 0.0, y: 0.0 }),
=======
            MouseCoordsState::with_coords(Coords { x: 0, y: 0 }),
>>>>>>> master
            KeyboardTimedState::default(),
            MouseTimedState::default(),
            KeyboardLongPressScheduler::default(),
            KeyboardClickExactScheduler::default(),
            MouseLongPressScheduler::default(),
            MouseClickExactScheduler::default(),
            KeyboardPointerState::default(),
            MousePointerState::default(),
        );

        let device_state = HashMap::new();

        Self {
<<<<<<< HEAD
            mouse_mapping_cache,
            touch_mapping_cache,
=======
            mapping_cache,
>>>>>>> master
            global_state,
            device_state,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlutterPointerEvent {
    timestamp_ms: i64,
    device: u32,
    kind: FlutterPointerKind,
    buttons: u32,
<<<<<<< HEAD
    position_x: f64,
    position_y: f64,
    #[serde(default)]
    scroll_delta_x: Option<f64>,
    #[serde(default)]
    scroll_delta_y: Option<f64>,
=======
    position_x: i64,
    position_y: i64,
>>>>>>> master
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FlutterKeyboardEvent {
    timestamp_ms: i64,
    runtime_type: FlutterKeyboardEventKind,
    key_label: String,
    chars: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum FlutterPointerKind {
    #[serde(rename = "PointerDeviceKind.mouse")]
    Mouse,
<<<<<<< HEAD
    #[serde(rename = "PointerDeviceKind.touch")]
    Touch,
=======
>>>>>>> master
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum FlutterKeyboardEventKind {
    KeyDownEvent,
    KeyUpEvent,
}

impl Input {
<<<<<<< HEAD
    fn new() -> Self {
=======
    pub fn new() -> Self {
>>>>>>> master
        Self::default()
    }

    pub fn on_flutter_mouse_event<'a>(
        &'a mut self,
        msg: &str,
        context: Context<'a>,
    ) -> impl CapturedLifetime<'a> + Iterator<Item = Event> {
<<<<<<< HEAD
        //static MOVE_EVENTS_COUNT: std::sync::atomic::AtomicUsize =
        //    std::sync::atomic::AtomicUsize::new(0);

=======
>>>>>>> master
        let event: FlutterPointerEvent = serde_json::from_str(&msg).unwrap();

        let device = match event.kind {
            FlutterPointerKind::Mouse => Device::Mouse(event.device),
<<<<<<< HEAD
            FlutterPointerKind::Touch => Device::Mouse(event.device), // FIXME windows trackpad
        };

        let mapping_cache = match context.mapping_kind {
            MappingKind::Mouse => &self.mouse_mapping_cache,
            MappingKind::Touch => &self.touch_mapping_cache,
        };

        let x = event.position_x / context.transform.scale - context.transform.x;
        let y = event.position_y / context.transform.scale - context.transform.y;
        /*println!(
            "Mouse {:10} {:10} : {:10} {:10}",
            event.position_x, event.position_y, x, y
        );*/
        let buttons = event.buttons;
        // dbg!(x, y);
        // let timestamp_ms = event.timestamp_ms;

        let context = EventContext {
            model: context.model,
            ui_state: context.ui_state,
            selected_node_ids: context.selected_node_ids,
            last_scroll_dx: event.scroll_delta_x.unwrap_or(0.0),
            last_scroll_dy: event.scroll_delta_y.unwrap_or(0.0),
        };

=======
        };

        let x = event.position_x;
        let y = event.position_y;
        let buttons = event.buttons;
        // let timestamp_ms = event.timestamp_ms;

>>>>>>> master
        let mut device_state =
            self.device_state
                .entry(device)
                .or_insert(DeviceState { buttons: 0, x, y });

<<<<<<< HEAD
        let is_dragged = |lhs: &Coords, rhs: &Coords| {
            (lhs.x - rhs.x).powi(2) + (lhs.y - rhs.y).powi(2) >= 5.0 * 5.0
        };
=======
        let is_dragged =
            |lhs: &Coords, rhs: &Coords| (lhs.x - rhs.x).pow(2) + (lhs.y - rhs.y).pow(2) >= 5 * 5;
>>>>>>> master

        let bindings = self.global_state.with_timeout(
            event.timestamp_ms - 1000,
            event.timestamp_ms - 300,
<<<<<<< HEAD
            mapping_cache,
=======
            &self.mapping_cache,
>>>>>>> master
        );
        let mut events: Vec<_> = timeout_bindings_into_events(bindings, context).collect();

        if device_state.x != x || device_state.y != y {
<<<<<<< HEAD
            //MOVE_EVENTS_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let event = MouseCoordsEvent::new(event.timestamp_ms, Coords { x, y });
            let bindings =
                self.global_state
                    .with_mouse_coords_event(event, mapping_cache, is_dragged);
=======
            let event = MouseCoordsEvent::new(event.timestamp_ms, Coords { x, y });
            let bindings =
                self.global_state
                    .with_mouse_coords_event(event, &self.mapping_cache, is_dragged);
>>>>>>> master
            events.extend(event_bindings_into_events(bindings, context));
            //Self::handle_events(raw_event, store, req_id)
        }

<<<<<<< HEAD
        if context.last_scroll_dx != 0.0 || context.last_scroll_dy != 0.0 {
            // context.last_scroll_dy.abs() < 0.1
            let event = MouseTriggerEvent::new(event.timestamp_ms, MouseTrigger("scroll"));
            let bindings = self
                .global_state
                .with_mouse_trigger_event(event, mapping_cache);
            events.extend(event_bindings_into_events(bindings, context));
        }

=======
>>>>>>> master
        for (button, mask) in [
            (MouseSwitch("LeftMouseButton"), 1),
            (MouseSwitch("RightMouseButton"), 2),
            // (MouseButton::Auxiliary, 4),
        ] {
            if (device_state.buttons & mask) != (buttons & mask) {
<<<<<<< HEAD
                //let move_events_count =
                //    MOVE_EVENTS_COUNT.swap(0, std::sync::atomic::Ordering::SeqCst);
                //if move_events_count != 0 {
                //println!("mouse move {move_events_count}");
                //}
                if buttons & mask != 0 {
                    //println!("mouse {button:?} press");
                    let event = MouseSwitchEvent::new(event.timestamp_ms, button);
                    let bindings = self
                        .global_state
                        .with_mouse_press_event(event, mapping_cache);
                    events.extend(event_bindings_into_events(bindings, context));
                } else {
                    //println!("mouse {button:?} release");
                    let event = MouseSwitchEvent::new(event.timestamp_ms, button);
                    let bindings = self
                        .global_state
                        .with_mouse_release_event(event, mapping_cache);
                    let mouse_events = event_bindings_into_events(bindings, context)/*.peekable()*/;
=======
                if buttons & mask != 0 {
                    let event = MouseSwitchEvent::new(event.timestamp_ms, button);
                    let bindings = self
                        .global_state
                        .with_mouse_press_event(event, &self.mapping_cache);
                    events.extend(event_bindings_into_events(bindings, context));
                } else {
                    let event = MouseSwitchEvent::new(event.timestamp_ms, button);
                    let bindings = self
                        .global_state
                        .with_mouse_release_event(event, &self.mapping_cache);
                    let mut mouse_events = event_bindings_into_events(bindings, context)/*.peekable()*/;
>>>>>>> master
                    /*if mouse_events.peek().is_some() {
                        self.global_state
                            .mouse_timed_state
                            .on_reset_click_count(&button)
                            .unwrap();
                    }*/
                    events.extend(mouse_events);
                };
                //Self::handle_events(self.input.on_event(raw_event, store), store, req_id)
            }
        }

        device_state.x = x;
        device_state.y = y;
        device_state.buttons = buttons;
        events.into_iter()
    }

    pub fn on_flutter_keyboard_event<'a>(
        &'a mut self,
        msg: &str,
        context: Context<'a>,
    ) -> impl CapturedLifetime<'a> + Iterator<Item = Event> {
<<<<<<< HEAD
        let mapping_cache = match context.mapping_kind {
            MappingKind::Mouse => &self.mouse_mapping_cache,
            MappingKind::Touch => &self.touch_mapping_cache,
        };

        let context = EventContext {
            model: context.model,
            ui_state: context.ui_state,
            selected_node_ids: context.selected_node_ids,
            last_scroll_dx: 0.0,
            last_scroll_dy: 0.0,
        };

        let event: FlutterKeyboardEvent = serde_json::from_str(&msg).unwrap();
        // println!("{:?} {:?}", context.ui_state, event);
=======
        let event: FlutterKeyboardEvent = serde_json::from_str(&msg).unwrap();
        println!("{:?} {:?}", context.ui_state, event);
>>>>>>> master
        let switch = KeyboardSwitch(event.key_label);
        let events: Vec<_> = match event.runtime_type {
            FlutterKeyboardEventKind::KeyDownEvent => {
                // are we in type ui_state
                // if we are, we do not continue usual processing, but emit Typing(with characts)
                // if keychar event has not character (e.g. for ctrl+a)
                // then we process as usual
                let event = KeyboardSwitchEvent::new(event.timestamp_ms, switch);
                let bindings = self
                    .global_state
<<<<<<< HEAD
                    .with_keyboard_press_event(event, mapping_cache);
=======
                    .with_keyboard_press_event(event, &self.mapping_cache);
>>>>>>> master
                event_bindings_into_events(bindings, context).collect()
            }
            FlutterKeyboardEventKind::KeyUpEvent => {
                // are we in type ui_state
                // if we are, we do not continue usual processing, but emit Typing(with characts)
                let event = KeyboardSwitchEvent::new(event.timestamp_ms, switch.clone());
                let bindings = self
                    .global_state
<<<<<<< HEAD
                    .with_keyboard_release_event(event, mapping_cache);

                let  keyboard_events = event_bindings_into_events(bindings, context)/*.peekable()*/;
=======
                    .with_keyboard_release_event(event, &self.mapping_cache);

                let mut keyboard_events = event_bindings_into_events(bindings, context)/*.peekable()*/;
>>>>>>> master
                /*if keyboard_events.peek().is_some() {
                    self.global_state
                        .keyboard_timed_state
                        .on_reset_click_count(&switch)
                        .unwrap();
                }*/
                keyboard_events.collect()
            }
        }; // FIXME: Remove recollection

        let events = match (&context.ui_state, event.chars.as_deref()) {
            (UiState::UiInput, Some(chars)) => None,
            _ => Some(events),
            /*let event = if chars == "\n" || chars == "\r\n" || chars == "\r" {
                // TODO: emit ApplyCommandInput with higher priority without `\n` check
                Event::ApplyCommandInput(command.to_owned())
            } else if chars == "\x08" {
                if command.is_empty() {
                    Event::CancelCommandInput.to_owned()
                } else {
                    Event::ModifyCommandInput(command[..command.len() - 1].to_owned())
                }
            } else {
                Event::ModifyCommandInput(command.to_owned() + &chars)
            };
            let events: Vec<_> = std::iter::once(event).collect(); // FIXME: Remove recollection
            events.into_iter()*/
        };
        events.into_iter().flatten()
    }
}

pub trait BuildAppEvent<Co> {
<<<<<<< HEAD
    fn build(&self, coords: &Co, context: EventContext<'_>) -> Option<Event>;
}

impl BuildAppEvent<KeyboardCoords> for BasicAppEventBuilder {
    fn build(&self, _: &KeyboardCoords, context: EventContext<'_>) -> Option<Event> {
=======
    fn build(&self, coords: &Co, context: Context<'_>) -> Option<Event>;
}

impl BuildAppEvent<KeyboardCoords> for BasicAppEventBuilder {
    fn build(&self, _: &KeyboardCoords, context: Context<'_>) -> Option<Event> {
>>>>>>> master
        match self {
            Self::Unselect => {
                if context.selected_node_ids.is_empty() {
                    None
                } else {
                    Some(Event::Unselect)
                }
            }
            Self::RemoveNodes => {
                if context.selected_node_ids.is_empty() {
                    None
                } else {
                    Some(Event::RemoveNodes(context.selected_node_ids.clone()))
                }
            }
            Self::CancelSelection => match context.ui_state {
                UiState::Selection(_, _) => Some(Event::CancelSelection),
                UiState::Default
                | UiState::MaybeSelection(_)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
<<<<<<< HEAD
            Self::CancelTransformMove => match context.ui_state {
                UiState::TransformMove(_) => Some(Event::CancelTransformMove),
                UiState::Default
                | UiState::Selection(_, _)
                | UiState::MaybeSelection(_)
                | UiState::MaybeTransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
            Self::CancelViewportMove => match context.ui_state {
                UiState::ViewportMove(_, _) => Some(Event::CancelViewportMove),
                UiState::Default
                | UiState::Selection(_, _)
                | UiState::MaybeSelection(_)
                | UiState::MaybeViewportMove(_)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::CancelNodeMove => match context.ui_state {
                UiState::NodeMove(_, _) => Some(Event::CancelNodeMove),
                UiState::Default
                | UiState::MaybeSelection(_)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::Selection(_, _)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::Selection(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::CancelEdge => match context.ui_state {
                UiState::Edge(port_id, _) => Some(Event::CancelEdge(*port_id)),
                UiState::Default
                | UiState::MaybeSelection(_)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::Selection(_, _)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::Selection(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            /*Self::StartCommandInput => Some(Event::StartCommandInput("".to_owned())),
            Self::ApplyCommandInput => match context.ui_state {
                UiState::CommandInput(command) => Some(Event::ApplyCommandInput(command.clone())),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::UiInput
                | UiState::Edge(_, _) => None,
            },
            Self::CancelCommandInput => match context.ui_state {
                UiState::CommandInput(_) => Some(Event::CancelCommandInput),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::UiInput
                | UiState::Edge(_, _) => None,
            },*/
        }
    }
}

impl BuildAppEvent<Coords> for PointerAppEventBuilder {
<<<<<<< HEAD
    fn build(&self, coords: &Coords, context: EventContext<'_>) -> Option<Event> {
=======
    fn build(&self, coords: &Coords, context: Context<'_>) -> Option<Event> {
>>>>>>> master
        match self {
            Self::Unselect => {
                // TODO: But we probably should cancel click after we handle press for selection
                if context.selected_node_ids.is_empty() {
                    // || ctx.next_node_at(coords).is_some()
                    None
                } else {
                    Some(Event::Unselect)
                }
            }
            Self::SelectNode => context
                .model
                .next_movable_widget_node_at(coords)
                .map(|(node_id, _)| Event::SelectNode(*node_id)),
<<<<<<< HEAD
            Self::AddOrRemoveNodeFromSelection => context
                .model
                .next_movable_widget_node_at(coords)
                .map(|(node_id, _)| Event::AddOrRemoveNodeToSelection(*node_id)),
=======
            Self::AddNodeToSelection => context
                .model
                .next_movable_widget_node_at(coords)
                .map(|(node_id, _)| Event::AddNodeToSelection(*node_id)),
>>>>>>> master

            // CREATE
            Self::CreateNode => Some(Event::CreateNode(*coords)),
            Self::EditNode => context
                .model
                .next_movable_widget_node_at(coords)
                .map(|(node_id, _)| Event::EditNode(*node_id)),
            Self::RemoveNodes => {
                if context.selected_node_ids.is_empty() {
                    None
                } else {
                    Some(Event::RemoveNodes(context.selected_node_ids.clone()))
                }
            }

            // SELECTION
            Self::MaybeStartSelection => match context.ui_state {
<<<<<<< HEAD
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::MaybeTransformMove(_)
                | UiState::MaybeNodeMove(_, _) => Some(Event::MaybeStartSelection(*coords)),
                UiState::Selection(_, _)
                | UiState::TransformMove(_)
=======
                UiState::Default | UiState::MaybeSelection(_) => {
                    Some(Event::MaybeStartSelection(*coords))
                }
                UiState::Selection(_, _)
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::NotASelection => match context.ui_state {
                UiState::MaybeSelection(_) => Some(Event::NotASelection),
                UiState::Default
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::StartSelection => match context.ui_state {
                UiState::MaybeSelection(start_coords) => {
                    Some(Event::StartSelection(*start_coords, *coords))
                }
                UiState::Default
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::ContinueSelection => match context.ui_state {
                UiState::Selection(start_coords, _) => {
                    Some(Event::ContinueSelection(*start_coords, *coords))
                }
                UiState::Default
                | UiState::MaybeSelection(_)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::EndSelection => match context.ui_state {
                UiState::Selection(start_coords, _) => {
                    Some(Event::EndSelection(*start_coords, *coords))
                }
                UiState::Default
                | UiState::MaybeSelection(_)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::CancelSelection => match context.ui_state {
                UiState::Selection(_, _) => Some(Event::CancelSelection),
                UiState::Default
                | UiState::MaybeSelection(_)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },

<<<<<<< HEAD
            // TRANSFORM MOVE
            Self::MaybeStartTransformMove => match context.ui_state {
                UiState::Default
                | UiState::MaybeTransformMove(_)
                | UiState::MaybeSelection(_)
                | UiState::MaybeNodeMove(_, _) => Some(Event::MaybeStartTransformMove(*coords)),
                UiState::Selection(_, _)
                | UiState::TransformMove(_)
=======
            // VIEWPORT MOVE
            Self::MaybeStartViewportMove => match context.ui_state {
                UiState::Default | UiState::MaybeViewportMove(_) => {
                    Some(Event::MaybeStartViewportMove(*coords))
                }
                UiState::Selection(_, _)
                | UiState::MaybeSelection(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
<<<<<<< HEAD
            Self::NotATransformMove => match context.ui_state {
                UiState::MaybeTransformMove(_) => Some(Event::NotATransformMove),
                UiState::Default
                | UiState::Selection(_, _)
                | UiState::MaybeSelection(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
            Self::NotAViewportMove => match context.ui_state {
                UiState::MaybeViewportMove(_) => Some(Event::NotAViewportMove),
                UiState::Default
                | UiState::Selection(_, _)
                | UiState::MaybeSelection(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
<<<<<<< HEAD
            Self::StartTransformMove => match context.ui_state {
                UiState::MaybeTransformMove(start_coords) => {
                    Some(Event::StartTransformMove(*start_coords, *coords))
=======
            Self::StartViewportMove => match context.ui_state {
                UiState::MaybeViewportMove(start_coords) => {
                    Some(Event::StartViewportMove(*start_coords, *coords))
>>>>>>> master
                }
                UiState::Default
                | UiState::Selection(_, _)
                | UiState::MaybeSelection(_)
<<<<<<< HEAD
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
<<<<<<< HEAD
            Self::ContinueTransformMove => match context.ui_state {
                UiState::TransformMove(start_coords) => {
                    Some(Event::ContinueTransformMove(*start_coords, *coords))
=======
            Self::ContinueViewportMove => match context.ui_state {
                UiState::ViewportMove(start_coords, last_coords) => {
                    Some(Event::ContinueViewportMove(*last_coords, *coords))
>>>>>>> master
                }
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
<<<<<<< HEAD
            Self::EndTransformMove => match context.ui_state {
                UiState::TransformMove(start_coords) => {
                    Some(Event::EndTransformMove(*start_coords, *coords))
=======
            Self::EndViewportMove => match context.ui_state {
                UiState::ViewportMove(start_coords, last_coords) => {
                    Some(Event::EndViewportMove(*last_coords, *coords))
>>>>>>> master
                }
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
<<<<<<< HEAD
            Self::CancelTransfromMove => match context.ui_state {
                UiState::TransformMove(_) => Some(Event::CancelTransformMove),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
                | UiState::MaybeTransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
            Self::CancelViewportMove => match context.ui_state {
                UiState::ViewportMove(_, _) => Some(Event::CancelViewportMove),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
                | UiState::MaybeViewportMove(_)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },

            // MOVE
            Self::MaybeStartNodeMove => match context.ui_state {
<<<<<<< HEAD
                UiState::Default
                | UiState::MaybeNodeMove(_, _)
                | UiState::MaybeSelection(_)
                | UiState::MaybeTransformMove(_) => context
                    .model
                    .next_movable_widget_node_at(coords)
                    .map(|(node_id, _)| Event::MaybeStartNodeMove(*node_id, *coords)),
                UiState::Selection(_, _)
                | UiState::TransformMove(_)
=======
                UiState::Default | UiState::MaybeNodeMove(_) => context
                    .model
                    .next_movable_widget_node_at(coords)
                    .map(|(node_id, _)| Event::MaybeStartNodeMove(*node_id, *coords)),
                UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::NotANodeMove => match context.ui_state {
<<<<<<< HEAD
                UiState::MaybeNodeMove(_, _) => Some(Event::NotANodeMove),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
=======
                UiState::MaybeNodeMove(_) => Some(Event::NotANodeMove),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::StartNodeMove => match context.ui_state {
<<<<<<< HEAD
                UiState::MaybeNodeMove(node_id, start_coords) => {
                    Some(Event::StartNodeMove(*node_id, *start_coords, *coords))
=======
                UiState::MaybeNodeMove(start_coords) => {
                    Some(Event::StartNodeMove(*start_coords, *coords))
>>>>>>> master
                }
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::ContinueNodeMove => match context.ui_state {
                UiState::NodeMove(start_coords, _) => {
                    Some(Event::ContinueNodeMove(*start_coords, *coords))
                }
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::EndNodeMove => match context.ui_state {
                UiState::NodeMove(start_coords, _) => {
                    Some(Event::EndNodeMove(*start_coords, *coords))
                }
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::CancelNodeMove => match context.ui_state {
                UiState::NodeMove(_, _) => Some(Event::CancelNodeMove),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },

            //EDGE
            Self::MaybeStartEdge => match context.ui_state {
                UiState::Default | UiState::MaybeEdge(_) => context
                    .model
                    .next_input_or_output_at(coords)
                    .map(|port_id| Event::MaybeStartEdge(port_id)),
                UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::NotAEdge => match context.ui_state {
                UiState::MaybeEdge(_) => Some(Event::NotAEdge),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::StartEdge => match context.ui_state {
                UiState::MaybeEdge(port_id) => Some(Event::StartEdge(*port_id, *coords)),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::ContinueEdge => match context.ui_state {
                UiState::Edge(port_id, _) => Some(Event::ContinueEdge(*port_id, *coords)),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::EndEdge => match context.ui_state {
                UiState::Edge(port_id, _) => match port_id {
                    PortId::Input(input_id) => context
                        .model
                        .next_output_at(coords)
                        .map(|(output_id, _)| Event::EndEdge(*input_id, *output_id))
                        .or_else(|| Some(Event::CancelEdge(PortId::Input(*input_id)))),
                    PortId::Output(output_id) => context
                        .model
                        .next_input_at(coords)
                        .map(|(input_id, _)| Event::EndEdge(*input_id, *output_id))
                        .or_else(|| Some(Event::CancelEdge(PortId::Output(*output_id)))),
                },
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::CancelEdge => match context.ui_state {
                UiState::Edge(port_id, _) => Some(Event::CancelEdge(*port_id)),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
<<<<<<< HEAD
                | UiState::MaybeTransformMove(_)
                | UiState::TransformMove(_)
                | UiState::MaybeNodeMove(_, _)
=======
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
>>>>>>> master
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
<<<<<<< HEAD
            Self::ScrollZoom => Some(Event::ScrollZoom(
                coords.x as f64,
                coords.y as f64,
                MOUSE_SCROLL_DELTA_MULT.powf(-context.last_scroll_dy),
            )),
            Self::ScrollMoveScreen => Some(Event::ScrollMoveScreen(
                context.last_scroll_dx,
                context.last_scroll_dy,
            )),
        }
        /*
            Self::StartCommandInput => Some(Event::StartCommandInput("".to_owned())),
            Self::ApplyCommandInput => match context.ui_state {
            UiState::CommandInput(command) => Some(Event::ApplyCommandInput(command.clone())),
            UiState::Default
            | UiState::MaybeSelection(_)
            | UiState::Selection(_, _)
            | UiState::MaybeNodeMove(_, _)
            | UiState::NodeMove(_, _)
            | UiState::MaybeEdge(_)
            | UiState::UiInput
            | UiState::Edge(_, _) => None,
        },
        Self::CancelCommandInput => match context.ui_state {
            UiState::CommandInput(_) => Some(Event::CancelCommandInput),
            UiState::Default
            | UiState::MaybeSelection(_)
            | UiState::Selection(_, _)
            | UiState::MaybeNodeMove(_, _)
            | UiState::NodeMove(_, _)
            | UiState::MaybeEdge(_)
            | UiState::UiInput
            | UiState::Edge(_, _) => None,
        },
        */
=======
            // COMMAND
            /*Self::StartCommandInput => Some(Event::StartCommandInput("".to_owned())),
            Self::ApplyCommandInput => match context.ui_state {
                UiState::CommandInput(command) => Some(Event::ApplyCommandInput(command.clone())),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
                | UiState::MaybeNodeMove(_)
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::UiInput
                | UiState::Edge(_, _) => None,
            },
            Self::CancelCommandInput => match context.ui_state {
                UiState::CommandInput(_) => Some(Event::CancelCommandInput),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
                | UiState::MaybeNodeMove(_)
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::UiInput
                | UiState::Edge(_, _) => None,
            },*/
        }
>>>>>>> master
    }
}
