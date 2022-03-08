// use std::collections::{BTreeMap, HashMap};
// use std::sync::{Arc, Weak};

// use input_processor::{
//     ButtonKind, CombinedInput, Duration, KeyboardKey, ModifiedInput, ModifiersAxes,
//     ModifiersFilter, MouseButton, ProcessorContext, ProcessorState, ProcessorStateData, RawInput,
//     TimedInput, TriggerKind,
// };

// #[derive(Clone, Debug)]
// struct Timeout(ButtonKind);

// #[derive(Clone, Debug, Default)]
// pub struct Input {
//     data: ProcessorStateData<Timeout>,
//     scheduled: BTreeMap<InputtimestampMs, Vec<Weak<Timeout>>>,
// }

// pub type InputtimestampMs = u64;
// pub type AppEvent = &'static str;

// impl Input {
//     pub fn with_event<F>(
//         self,
//         ev: RawInput<()>,
//         timestamp: InputtimestampMs,
//         mut handler: F,
//     ) -> Self
//     where
//         F: FnMut(AppEvent, ModifiersAxes),
//     {
//         let mut data = self.data;
//         let mut scheduled = self.scheduled;
//         while let Some(entry) = scheduled.first_entry() {
//             if *entry.key() > timestamp {
//                 break;
//             }
//             let (timestamp, timeouts) = entry.remove_entry();
//             for timeout in timeouts {
//                 if let Some(timeout) = timeout.upgrade() {
//                     let context = Context {
//                         timestamp,
//                         scheduled,
//                         handler,
//                     };
//                     let state = ProcessorState::from_parts(data, context);
//                     let (state, err) = state.with_timeout_event(timeout.0.clone());
//                     let state = state.split();
//                     data = state.0;
//                     scheduled = state.1.scheduled;
//                     handler = state.1.handler;
//                     err.unwrap();
//                 }
//             }
//         }

//         let context = Context {
//             timestamp,
//             scheduled,
//             handler,
//         };
//         let state = ProcessorState::from_parts(data, context);
//         let (state, err) = state.with_event(ev);
//         err.unwrap();
//         let (data, context) = state.split();
//         Self {
//             data,
//             scheduled: context.scheduled,
//         }
//     }
// }

// #[derive(Clone, Debug)]
// struct Context<F: FnMut(AppEvent, ModifiersAxes)> {
//     timestamp: InputtimestampMs,
//     scheduled: BTreeMap<InputtimestampMs, Vec<Weak<Timeout>>>,
//     handler: F,
// }

// impl<F: FnMut(AppEvent, ModifiersAxes)> ProcessorContext for Context<F> {
//     type Timeout = Timeout;
//     type CustomEvent = ();
//     type MappedEvent = AppEvent;

//     fn schedule(mut self, button: ButtonKind, delay: Duration) -> (Self, Arc<Self::Timeout>) {
//         let delay: InputtimestampMs = match delay {
//             Duration::LongClick(_) => 1000,
//             Duration::MultiClick(_) => 300,
//         };
//         let timeout = Arc::new(Timeout(button));
//         self.scheduled
//             .entry(self.timestamp + delay)
//             .and_modify(|timeouts| timeouts.push(Arc::downgrade(&timeout)))
//             .or_insert_with(|| vec![Arc::downgrade(&timeout)]);
//         (self, timeout)
//     }

//     fn events(&self, input: &CombinedInput<Self::CustomEvent>) -> Vec<(AppEvent, ModifiersFilter)> {
//         //dbg!(input);
//         match input {
//             CombinedInput::Timed(TimedInput::Click {
//                 button: ButtonKind::MouseButton(button),
//                 num_clicks,
//             }) => match (button, num_clicks) {
//                 (MouseButton::Primary, 1) => vec![
//                     ("LmbClick_01", ModifiersFilter::default()),
//                     (
//                         "RmbLmbClick_02",
//                         ModifiersFilter {
//                             buttons: [ButtonKind::MouseButton(MouseButton::Secondary)]
//                                 .into_iter()
//                                 .collect(),
//                             axes_ranges: HashMap::new(),
//                         },
//                     ),
//                 ],
//                 (MouseButton::Secondary, 1) => vec![
//                     ("RmbClick_02", ModifiersFilter::default()),
//                     (
//                         "LmbRmbClick_02",
//                         ModifiersFilter {
//                             buttons: [ButtonKind::MouseButton(MouseButton::Primary)]
//                                 .into_iter()
//                                 .collect(),
//                             axes_ranges: HashMap::new(),
//                         },
//                     ),
//                 ],
//                 (MouseButton::Primary, 2) => vec![("LmbDblClick_02", ModifiersFilter::default())],
//                 (MouseButton::Secondary, 2) => vec![("RmbDblClick_02", ModifiersFilter::default())],
//                 _ => vec![],
//             },

//             CombinedInput::Timed(TimedInput::Click {
//                 button: ButtonKind::KeyboardKey(KeyboardKey(key)),
//                 num_clicks: 2,
//             }) => match key.as_str() {
//                 " " => vec![("Dbl\" \"", ModifiersFilter::default())],
//                 "Space" => vec![("DblSpace", ModifiersFilter::default())],
//                 "\n" => vec![("Dbl\"\\n\"", ModifiersFilter::default())],
//                 "Return" => vec![("DblReturn", ModifiersFilter::default())],
//                 "Enter" => vec![("DblEnter", ModifiersFilter::default())],
//                 _ => vec![],
//             },

//             CombinedInput::Modified(ModifiedInput::Press(ButtonKind::MouseButton(
//                 MouseButton::Primary,
//             ))) => vec![("LmbDown", ModifiersFilter::default())],

//             CombinedInput::Modified(ModifiedInput::Release(ButtonKind::MouseButton(
//                 MouseButton::Primary,
//             ))) => vec![("LmbUp", ModifiersFilter::default())],

//             CombinedInput::Modified(ModifiedInput::Trigger(TriggerKind::MouseMove)) => {
//                 vec![(
//                     "LmbMove",
//                     ModifiersFilter {
//                         buttons: [ButtonKind::MouseButton(MouseButton::Primary)]
//                             .into_iter()
//                             .collect(),
//                         axes_ranges: HashMap::new(),
//                     },
//                 )]
//             }

//             _ => vec![],
//         }
//     }

//     fn emit(mut self, ev: AppEvent, axes: ModifiersAxes) -> Self {
//         (self.handler)(ev, axes);
//         self
//     }
// }

// /*
// let state = ProcessorState {
//     processor: Processor,
//     modifiers: Arc::new(Modifiers::default()),
//     buttons: TimedStateButtons::default(),
// };*/
