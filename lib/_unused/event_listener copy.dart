// import 'package:dart_json_mapper/dart_json_mapper.dart';
// import 'package:flutter/gestures.dart';
// import 'package:flutter/material.dart';
// import 'package:hooks_riverpod/hooks_riverpod.dart';
// import 'package:plugin/generated/rid_api.dart' as rid;
// import 'package:rheetah/canvas.dart';
// import 'package:rheetah/serialization/input_mapping.dart';
// import 'package:rheetah/providers/store_provider.dart';

// class EventListener extends HookConsumerWidget {
//   EventListener({Key? key}) : super(key: key);
//   FocusNode focusNode = FocusNode();

//   @override
//   Widget build(BuildContext context, WidgetRef ref) {
//     focusNode.requestFocus;
//     final store = ref.watch(storeRepoProvider);
//     return Listener(
//       behavior: HitTestBehavior.translucent,
//       onPointerSignal: (ev) {
//         if (ev is PointerScrollEvent) {
//           final inputProperties = {
//             "runtimeType": ev.runtimeType.toString(),
//             "mouseScrollDeltaX": ev.scrollDelta.dx,
//             "mouseScrollDeltaY": ev.scrollDelta.dy,
//             "mouseScrollX": ev.position.dx,
//             "mouseScrollY": ev.position.dy,
//             "timestampMs": DateTime.now().millisecondsSinceEpoch,
//           };
//           String inputEvent =
//               JsonMapper.serialize(InputProperties(inputProperties));
//           store.store.msgSendEvent(inputEvent);
//         }
//       },
//       onPointerDown: (ev) {
//         final inputProperties = {
//           "buttons": ev.buttons,
//           "device": ev.device,
//           "kind": ev.kind.toString(),
//           "positionX": ev.position.dx,
//           "positionY": ev.position.dy,
//           "runtimeType": ev.runtimeType.toString(),
//           "timestampMs": DateTime.now().millisecondsSinceEpoch,
//         };
//         String inputEvent =
//             JsonMapper.serialize(InputProperties(inputProperties));
//         store.store.msgSendEvent(inputEvent);
//       },
//       onPointerMove: (ev) {
//         print("button ${ev.buttons}");

//         final inputProperties = {
//           "buttons": ev.buttons,
//           "device": ev.device,
//           "kind": ev.kind.toString(),
//           "positionX": ev.position.dx,
//           "positionY": ev.position.dy,
//           "runtimeType": ev.runtimeType.toString(),
//           "timestampMs": DateTime.now().millisecondsSinceEpoch,
//         };
//         String inputEvent =
//             JsonMapper.serialize(InputProperties(inputProperties));

//         store.store.msgSendEvent(inputEvent);
//       },
//       onPointerUp: (ev) {
//         final inputProperties = {
//           "buttons": ev.buttons,
//           "device": ev.device,
//           "kind": ev.kind.toString(),
//           "positionX": ev.position.dx,
//           "positionY": ev.position.dy,
//           "runtimeType": ev.runtimeType.toString(),
//           "timestampMs": DateTime.now().millisecondsSinceEpoch,
//         };
//         String inputEvent =
//             JsonMapper.serialize(InputProperties(inputProperties));

//         store.store.msgSendEvent(inputEvent);
//       },
//       onPointerCancel: (ev) {
//         // final inputProperties = {
//         //   "buttons": ev.buttons,
//         //   "device": ev.device,
//         //   "kind": ev.kind.toString(),
//         //   "positionX": ev.position.dx,
//         //   "positionY": ev.position.dy,
//         //   "runtimeType": ev.runtimeType.toString(),
//         //   "timestampMs": DateTime.now().millisecondsSinceEpoch,
//         // };
//         // String inputEvent =
//         //     JsonMapper.serialize(InputProperties(inputProperties));

//         // widget.store.msgSendEvent(inputEvent);
//       },
//       onPointerHover: (ev) {
//         final inputProperties = {
//           "buttons": ev.buttons,
//           "device": ev.device,
//           "kind": ev.kind.toString(),
//           "positionX": ev.position.dx,
//           "positionY": ev.position.dy,
//           "runtimeType": ev.runtimeType.toString(),
//           "timestampMs": DateTime.now().millisecondsSinceEpoch,
//         };
//         String inputEvent =
//             JsonMapper.serialize(InputProperties(inputProperties));

//         store.store.msgSendEvent(inputEvent);
//       },
//       child: KeyboardListener(
//         focusNode: focusNode,
//         autofocus: true,
//         onKeyEvent: (KeyEvent ev) {
//           final inputProperties = {
//             "keyLabel": ev.logicalKey.keyLabel.toString(),
//             "runtimeType": ev.runtimeType.toString(),
//             "timestampMs": DateTime.now().millisecondsSinceEpoch,
//           };
//           String inputEvent =
//               JsonMapper.serialize(InputProperties(inputProperties));

//           store.store.msgSendEvent(inputEvent);

//           // sl<InteractionManager>()
//           //     .onRawKeyEvent([buildContext, keyEvent]);
//         },
//         child: RawGestureDetector(
//             behavior: HitTestBehavior.translucent,
//             gestures: <Type, GestureRecognizerFactory>{
//               ImmediateMultiDragGestureRecognizer:
//                   GestureRecognizerFactoryWithHandlers<
//                       ImmediateMultiDragGestureRecognizer>(
//                 () => ImmediateMultiDragGestureRecognizer(),
//                 (ImmediateMultiDragGestureRecognizer instance) {
//                   print("button handlestart ${instance.hashCode}");
//                   // instance.onStart = _handleStart;
//                 },
//               ),
//               // TapGestureRecognizer:
//               //     GestureRecognizerFactoryWithHandlers<
//               //         TapGestureRecognizer>(
//               //   () => TapGestureRecognizer(),
//               //   (TapGestureRecognizer instance) {
//               //     instance
//               //       ..onTapDown = (TapDownDetails details) {
//               //         startTimeUTC = DateTime.now().toUtc();
//               //         String event = details.toString();
//               //         widget.msgSendEvent(event);
//               //         // tools.log.v("size - type ${details.kind}");

//               //         // sl<InteractionManager>().onTapDown(
//               //         //   [
//               //         //     buildContext,
//               //         //     details,
//               //         //     startTimeUTC,
//               //         //   ],
//               //         // );
//               //         // setState(() {
//               //         //   _last = "tap down ${details.kind}";
//               //         // });
//               //       }
//               //       ..onTapUp = (TapUpDetails details) {
//               //         String event = details.toString();
//               //         widget.msgSendEvent(event);
//               //         setState(() {
//               //           _last = 'up ${details.globalPosition}';
//               //         });
//               //       }
//               //       // ..onTapCancel = () {
//               //       //   setState(() {
//               //       //     _last = 'cancel';
//               //       //   });
//               //       // }
//               //       ..onSecondaryTapDown = (TapDownDetails details) {
//               //         if (details.kind == PointerDeviceKind.mouse) {
//               //           String event = details.toString();
//               //           widget.msgSendEvent(event);
//               //         }
//               //         // sl<InteractionManager>().onSecondaryTapDown(
//               //         //     [buildContext, details]);
//               //         setState(() {
//               //           _last = "secondary ${details.globalPosition}";
//               //         });
//               //       };
//               //   },
//               // ),

//               // DoubleTapGestureRecognizer:
//               //     GestureRecognizerFactoryWithHandlers<
//               //             DoubleTapGestureRecognizer>(
//               //         () => DoubleTapGestureRecognizer(),
//               //         (DoubleTapGestureRecognizer instance) {
//               //   instance
//               //     ..onDoubleTap = () {
//               //       setState(() {
//               //         _last = 'double tap';
//               //       });
//               //     };
//               // }),
//               // MultiTapGestureRecognizer:
//               //     GestureRecognizerFactoryWithHandlers<
//               //             MultiTapGestureRecognizer>(
//               //         () => MultiTapGestureRecognizer(),
//               //         (MultiTapGestureRecognizer instance) {
//               //   instance.onTapDown = (int, TapDownDetails details) {
//               //     print("multi $int ${details.kind}");
//               //     // setState(() {
//               //     //   _last = 'multi tap #$int $details';
//               //     // });
//               //   };
//               //   // ..onTap = ((number) {
//               //   //   print("multi ${number}");
//               //   // })
//               //   // ..onTapUp = (number, TapUpDetails details) {
//               //   //   print("multi ${details.kind}");
//               //   // };
//               // }),
//               // PanGestureRecognizer:
//               //     GestureRecognizerFactoryWithHandlers<
//               //             PanGestureRecognizer>(
//               //         () => PanGestureRecognizer(),
//               //         (PanGestureRecognizer instance) {
//               //   instance
//               //     ..onStart = (DragStartDetails details) {
//               //       startTimeUTC = DateTime.now().toUtc();
//               //       String event = details.toString();
//               //       widget.msgSendEvent(event);

//               //       // sl<InteractionManager>().onStartPanning(
//               //       //   [
//               //       //     buildContext,
//               //       //     details,
//               //       //     startTimeUTC,
//               //       //   ],
//               //       // );

//               //       setState(() {
//               //         _last =
//               //             'panning start ${details.globalPosition}';
//               //       });
//               //     }
//               //     ..onUpdate = (DragUpdateDetails details) {
//               //       startTimeUTC = DateTime.now().toUtc();

//               //       // sl<InteractionManager>().onPointerMove(
//               //       //   [
//               //       //     buildContext,
//               //       //     details,
//               //       //     startTimeUTC,
//               //       //   ],
//               //       // );
//               //       String event = details.toString();
//               //       widget.msgSendEvent(event);
//               //       setState(() {
//               //         _last =
//               //             'panning update ${details.localPosition} ${details.delta}';
//               //       });
//               //     }
//               //     ..onEnd = (DragEndDetails details) {
//               //       startTimeUTC = DateTime.now().toUtc();

//               //       // sl<InteractionManager>().onPanEnd(
//               //       //   [
//               //       //     buildContext,
//               //       //     details,
//               //       //     startTimeUTC,
//               //       //   ],
//               //       // );

//               //       setState(() {
//               //         _last = 'panning end ${details.velocity} ';
//               //       });
//               //     };
//               // }),
//               // LongPressGestureRecognizer:
//               //     GestureRecognizerFactoryWithHandlers<
//               //             LongPressGestureRecognizer>(
//               //         () => LongPressGestureRecognizer(),
//               //         (LongPressGestureRecognizer instance) {
//               //   instance
//               //     ..onLongPressStart =
//               //         (LongPressStartDetails details) {
//               //       String event = details.toString();
//               //       widget.msgSendEvent(event);
//               //       startTimeUTC = DateTime.now().toUtc();

//               //       // sl<InteractionManager>().onLongPressStart([
//               //       //   buildContext,
//               //       //   details,
//               //       //   startTimeUTC,
//               //       // ]);
//               //       setState(() {
//               //         _last =
//               //             'long press start ${details.globalPosition}';
//               //       });
//               //     }
//               //     ..onLongPressMoveUpdate =
//               //         (LongPressMoveUpdateDetails details) {
//               //       startTimeUTC = DateTime.now().toUtc();
//               //       String event = details.toString();
//               //       widget.msgSendEvent(event);

//               //       // sl<InteractionManager>().onLongPressMove([
//               //       //   buildContext,
//               //       //   details,
//               //       //   startTimeUTC,
//               //       // ]);
//               //       setState(() {
//               //         _last =
//               //             'long press move ${details.globalPosition}';
//               //       });
//               //     }
//               //     ..onLongPressEnd = (LongPressEndDetails details) {
//               //       startTimeUTC = DateTime.now().toUtc();
//               //       String event = details.toString();
//               //       widget.msgSendEvent(event);

//               //       // sl<InteractionManager>().onLongPressEnd([
//               //       //   buildContext,
//               //       //   details,
//               //       //   startTimeUTC,
//               //       // ]);
//               //       setState(() {
//               //         _last =
//               //             'long press end ${details.globalPosition}';
//               //       });
//               //     };
//               // })
//             },
//             child: CanvasLayout()),
//       ),
//     );
//   }
// }
