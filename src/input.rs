use core::fmt::Debug;
use core::hash::Hash;
use std::collections::{HashMap, HashSet};

use input_core::*;
use input_more::*;
use serde::Deserialize;

use crate::{
    event::{Coords, Event},
    model::{Model, NodeId, PortId},
    state::UiState,
};

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
pub struct KeyboardTrigger(&'static str);

//#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
//pub struct KeyboardCoords; // ADDED
pub type KeyboardCoords = (); // ADDED

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MouseTrigger(&'static str);

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
    CancelViewportMove,
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
    AddNodeToSelection,
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
    MaybeStartViewportMove,
    NotAViewportMove,
    StartViewportMove,
    EndViewportMove,
    CancelViewportMove,
    ContinueViewportMove,
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

    let mut is_maybe_start_viewport_move_used = false;
    let mut is_not_a_viewport_move_used = false;
    let mut is_start_viewport_move_used = false;
    let mut is_end_viewport_move_used = false;
    let mut is_cancel_viewport_move_used = false;
    let mut is_continue_viewport_move_used = false;

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

    for event in &events {
        match event {
            Event::Unselect => is_unselect_used = true,
            Event::SelectNode(_) => is_select_node_used = true,
            Event::AddNodeToSelection(_) => is_add_node_to_selection_used = true,
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
                && !is_end_or_cancel_move_or_selection
        }
        Event::SelectNode(_) => !command_input && !is_create_node_used,
        Event::AddNodeToSelection(_) => !command_input && !is_create_node_used,
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
        Event::MaybeStartViewportMove(_) => {
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
        Event::StartViewportMove(_, _) => {
            !command_input
                && !is_start_node_move_used
                && !is_start_edge_used
                && !is_create_node_used
                && !is_edit_node_used
        }
        Event::StartNodeMove(_, _) => {
            !command_input && !is_start_edge_used && !is_create_node_used && !is_edit_node_used
        }
        Event::StartEdge(_, _) => !command_input && !is_create_node_used && !is_edit_node_used,

        Event::NotASelection
        | Event::EndSelection(_, _)
        | Event::CancelSelection
        | Event::ContinueSelection(_, _)
        | Event::NotAViewportMove
        | Event::EndViewportMove(_, _)
        | Event::CancelViewportMove
        | Event::ContinueViewportMove(_, _)
        | Event::NotANodeMove
        | Event::CancelNodeMove
        | Event::ContinueNodeMove(_, _)
        | Event::EndNodeMove(_, _)
        | Event::NotAEdge
        | Event::CancelEdge(_)
        | Event::ContinueEdge(_, _)
        | Event::EndEdge(_, _) => true,
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
    mapping_cache: GlobalMappingCache,
    global_state: GlobalState,
    device_state: HashMap<Device, DeviceState>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Device {
    Mouse(u32),
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct DeviceState {
    pub buttons: u32,
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Copy)]
pub struct Context<'a> {
    pub model: &'a Model,
    pub ui_state: &'a UiState,
    pub selected_node_ids: &'a HashSet<NodeId>,
}

pub trait CapturedLifetime<'a> {}
impl<'a, T> CapturedLifetime<'a> for T {}

fn bindings_into_events<'a, Bi, Co>(
    context: Context<'a>,
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
    context: Context<'a>,
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
    context: Context<'a>,
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

        let global_state = GlobalState::new(
            Modifiers::default(),
            KeyboardCoordsState::with_coords(()),
            MouseCoordsState::with_coords(Coords { x: 0, y: 0 }),
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
            mapping_cache,
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
    position_x: i64,
    position_y: i64,
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
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum FlutterKeyboardEventKind {
    KeyDownEvent,
    KeyUpEvent,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_flutter_mouse_event<'a>(
        &'a mut self,
        msg: &str,
        context: Context<'a>,
    ) -> impl CapturedLifetime<'a> + Iterator<Item = Event> {
        let event: FlutterPointerEvent = serde_json::from_str(&msg).unwrap();

        let device = match event.kind {
            FlutterPointerKind::Mouse => Device::Mouse(event.device),
        };

        let x = event.position_x;
        let y = event.position_y;
        let buttons = event.buttons;
        // let timestamp_ms = event.timestamp_ms;

        let mut device_state =
            self.device_state
                .entry(device)
                .or_insert(DeviceState { buttons: 0, x, y });

        let is_dragged =
            |lhs: &Coords, rhs: &Coords| (lhs.x - rhs.x).pow(2) + (lhs.y - rhs.y).pow(2) >= 5 * 5;

        let bindings = self.global_state.with_timeout(
            event.timestamp_ms - 1000,
            event.timestamp_ms - 300,
            &self.mapping_cache,
        );
        let mut events: Vec<_> = timeout_bindings_into_events(bindings, context).collect();

        if device_state.x != x || device_state.y != y {
            let event = MouseCoordsEvent::new(event.timestamp_ms, Coords { x, y });
            let bindings =
                self.global_state
                    .with_mouse_coords_event(event, &self.mapping_cache, is_dragged);
            events.extend(event_bindings_into_events(bindings, context));
            //Self::handle_events(raw_event, store, req_id)
        }

        for (button, mask) in [
            (MouseSwitch("LeftMouseButton"), 1),
            (MouseSwitch("RightMouseButton"), 2),
            // (MouseButton::Auxiliary, 4),
        ] {
            if (device_state.buttons & mask) != (buttons & mask) {
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
        let event: FlutterKeyboardEvent = serde_json::from_str(&msg).unwrap();
        println!("{:?} {:?}", context.ui_state, event);
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
                    .with_keyboard_press_event(event, &self.mapping_cache);
                event_bindings_into_events(bindings, context).collect()
            }
            FlutterKeyboardEventKind::KeyUpEvent => {
                // are we in type ui_state
                // if we are, we do not continue usual processing, but emit Typing(with characts)
                let event = KeyboardSwitchEvent::new(event.timestamp_ms, switch.clone());
                let bindings = self
                    .global_state
                    .with_keyboard_release_event(event, &self.mapping_cache);

                let mut keyboard_events = event_bindings_into_events(bindings, context)/*.peekable()*/;
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
    fn build(&self, coords: &Co, context: Context<'_>) -> Option<Event>;
}

impl BuildAppEvent<KeyboardCoords> for BasicAppEventBuilder {
    fn build(&self, _: &KeyboardCoords, context: Context<'_>) -> Option<Event> {
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::CancelViewportMove => match context.ui_state {
                UiState::ViewportMove(_, _) => Some(Event::CancelViewportMove),
                UiState::Default
                | UiState::Selection(_, _)
                | UiState::MaybeSelection(_)
                | UiState::MaybeViewportMove(_)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::Selection(_, _)
                | UiState::MaybeNodeMove(_)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::CancelEdge => match context.ui_state {
                UiState::Edge(port_id, _) => Some(Event::CancelEdge(*port_id)),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::Selection(_, _)
                | UiState::MaybeNodeMove(_)
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
    }
}

impl BuildAppEvent<Coords> for PointerAppEventBuilder {
    fn build(&self, coords: &Coords, context: Context<'_>) -> Option<Event> {
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
            Self::AddNodeToSelection => context
                .model
                .next_movable_widget_node_at(coords)
                .map(|(node_id, _)| Event::AddNodeToSelection(*node_id)),

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
                UiState::Default | UiState::MaybeSelection(_) => {
                    Some(Event::MaybeStartSelection(*coords))
                }
                UiState::Selection(_, _)
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },

            // VIEWPORT MOVE
            Self::MaybeStartViewportMove => match context.ui_state {
                UiState::Default | UiState::MaybeViewportMove(_) => {
                    Some(Event::MaybeStartViewportMove(*coords))
                }
                UiState::Selection(_, _)
                | UiState::MaybeSelection(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::NotAViewportMove => match context.ui_state {
                UiState::MaybeViewportMove(_) => Some(Event::NotAViewportMove),
                UiState::Default
                | UiState::Selection(_, _)
                | UiState::MaybeSelection(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::StartViewportMove => match context.ui_state {
                UiState::MaybeViewportMove(start_coords) => {
                    Some(Event::StartViewportMove(*start_coords, *coords))
                }
                UiState::Default
                | UiState::Selection(_, _)
                | UiState::MaybeSelection(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::ContinueViewportMove => match context.ui_state {
                UiState::ViewportMove(start_coords, last_coords) => {
                    Some(Event::ContinueViewportMove(*last_coords, *coords))
                }
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
                | UiState::MaybeViewportMove(_)
                | UiState::MaybeNodeMove(_)
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::EndViewportMove => match context.ui_state {
                UiState::ViewportMove(start_coords, last_coords) => {
                    Some(Event::EndViewportMove(*last_coords, *coords))
                }
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
                | UiState::MaybeViewportMove(_)
                | UiState::MaybeNodeMove(_)
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::CancelViewportMove => match context.ui_state {
                UiState::ViewportMove(_, _) => Some(Event::CancelViewportMove),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
                | UiState::MaybeViewportMove(_)
                | UiState::MaybeNodeMove(_)
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },

            // MOVE
            Self::MaybeStartNodeMove => match context.ui_state {
                UiState::Default | UiState::MaybeNodeMove(_) => context
                    .model
                    .next_movable_widget_node_at(coords)
                    .map(|(node_id, _)| Event::MaybeStartNodeMove(*node_id, *coords)),
                UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::NotANodeMove => match context.ui_state {
                UiState::MaybeNodeMove(_) => Some(Event::NotANodeMove),
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::Edge(_, _)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
            Self::StartNodeMove => match context.ui_state {
                UiState::MaybeNodeMove(start_coords) => {
                    Some(Event::StartNodeMove(*start_coords, *coords))
                }
                UiState::Default
                | UiState::MaybeSelection(_)
                | UiState::Selection(_, _)
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
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
                | UiState::MaybeViewportMove(_)
                | UiState::ViewportMove(_, _)
                | UiState::MaybeNodeMove(_)
                | UiState::NodeMove(_, _)
                | UiState::MaybeEdge(_)
                | UiState::UiInput
                | UiState::CommandInput(_) => None,
            },
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
    }
}
