import 'package:dart_json_mapper/dart_json_mapper.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/canvas.dart';
import 'package:moon/serialization/input_mapping.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:screenshot/screenshot.dart';

class EventListener extends HookConsumerWidget {
  EventListener({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final ScreenshotController screenshotControl =
        ref.read(screenshotController);
    // @override
    // void dispose() {
    //   super.dispose();

    //   focusScopeNode.dispose();
    // }

    FocusNode focusNode = useFocusNode();
    // print(focusNode);

    // FocusNodeManager.instance.addNode('main', FocusNode());
    // final mainNode = FocusNodeManager.instance.getNode('main');
    final store = ref.watch(storeRepoProvider);
    // final changes = ref.watch(viewportController);
    final rejectList = ref.watch(focusRejectController);
    print("rebuilding event listener");

    return Listener(
      behavior: HitTestBehavior.translucent,
      onPointerSignal: (ev) {
        final rejectList = ref.read(focusRejectProvider).rects;
        if (!rejectList.single.contains(ev.position)) {
          if (ev is PointerScrollEvent) {
            final inputProperties = {
              "buttons": ev.buttons,
              "device": ev.device,
              "kind": ev.kind.toString(),
              "scrollDeltaX": ev.scrollDelta.dx,
              "scrollDeltaY": ev.scrollDelta.dy,
              "positionX": (ev.localPosition.dx).toDouble(),
              "positionY": (ev.localPosition.dy).toDouble(),
              "runtimeType": ev.runtimeType.toString(),
              "timestampMs": DateTime.now().millisecondsSinceEpoch,
            };
            String inputEvent =
                JsonMapper.serialize(InputProperties(inputProperties));
            store.store.msgMouseEvent(inputEvent);
          }
          // print(inputEvent);
        }
      },
      onPointerDown: (ev) {
        // print("here");
        print(ev.localPosition);
        final rejectList = ref.read(focusRejectProvider).rects;
        if (!rejectList.single.contains(ev.position)) {
          // print(ev.position);
          final inputProperties = {
            "buttons": ev.buttons,
            "device": ev.device,
            "kind": ev.kind.toString(),
            // "positionX": (ev.position.dx.toInt()).toInt(),
            // "positionY": (ev.position.dy).toInt(),
            // "localPositionX": (ev.localPosition.dx).toInt(),
            // "localPositionY": (ev.localPosition.dy).toInt(),
            "positionX": (ev.localPosition.dx).toDouble(),
            "positionY": (ev.localPosition.dy).toDouble(),
            "runtimeType": ev.runtimeType.toString(),
            "timestampMs": DateTime.now().millisecondsSinceEpoch,
          };
          String inputEvent =
              JsonMapper.serialize(InputProperties(inputProperties));
          store.store.msgMouseEvent(inputEvent);
        }
      },
      onPointerMove: (ev) {
        // print("button ${ev.buttons}");

        final rejectList = ref.read(focusRejectProvider).rects;
        if (!rejectList.single.contains(ev.position)) {
          final inputProperties = {
            "buttons": ev.buttons,
            "device": ev.device,
            "kind": ev.kind.toString(),
            // "positionX": (ev.position.dx.toInt()).toInt(),
            // "positionY": (ev.position.dy).toInt(),
            // "localPositionX": (ev.localPosition.dx).toInt(),
            // "localPositionY": (ev.localPosition.dy).toInt(),
            "positionX": (ev.localPosition.dx).toDouble(),
            "positionY": (ev.localPosition.dy).toDouble(),
            "runtimeType": ev.runtimeType.toString(),
            "timestampMs": DateTime.now().millisecondsSinceEpoch,
          };
          String inputEvent =
              JsonMapper.serialize(InputProperties(inputProperties));

          store.store.msgMouseEvent(inputEvent);
        }
      },
      onPointerUp: (ev) {
        final rejectList = ref.read(focusRejectProvider).rects;
        if (!rejectList.single.contains(ev.position)) {
          final inputProperties = {
            "buttons": ev.buttons,
            "device": ev.device,
            "kind": ev.kind.toString(),
            // "positionX": (ev.position.dx.toInt()).toInt(),
            // "positionY": (ev.position.dy).toInt(),
            // "localPositionX": (ev.localPosition.dx).toInt(),
            // "localPositionY": (ev.localPosition.dy).toInt(),
            "positionX": (ev.localPosition.dx).toDouble(),
            "positionY": (ev.localPosition.dy).toDouble(),
            "runtimeType": ev.runtimeType.toString(),
            "timestampMs": DateTime.now().millisecondsSinceEpoch,
          };
          String inputEvent =
              JsonMapper.serialize(InputProperties(inputProperties));

          store.store.msgMouseEvent(inputEvent);
        }
      },
      onPointerCancel: (ev) {
        // final inputProperties = {
        //   "buttons": ev.buttons,
        //   "device": ev.device,
        //   "kind": ev.kind.toString(),
        //   "positionX": ev.position.dx,
        //   "positionY": ev.position.dy,
        //     "localPositionX": ev.localPosition.dx,
        // "localPositionY": ev.localPosition.dy,
        //   "runtimeType": ev.runtimeType.toString(),
        //   "timestampMs": DateTime.now().millisecondsSinceEpoch,
        // };
        // String inputEvent =
        //     JsonMapper.serialize(InputProperties(inputProperties));

        // widget.store.msgSendEvent(inputEvent);
      },
      onPointerHover: (ev) {
        final inputProperties = {
          "buttons": ev.buttons,
          "device": ev.device,
          "kind": ev.kind.toString(),
          // "positionX": (ev.position.dx.toInt()).toInt(),
          // "positionY": (ev.position.dy).toInt(),
          // "localPositionX": (ev.localPosition.dx).toInt(),
          // "localPositionY": (ev.localPosition.dy).toInt(),
          "positionX": (ev.localPosition.dx).toDouble(),
          "positionY": (ev.localPosition.dy).toDouble(),
          "runtimeType": ev.runtimeType.toString(),
          "timestampMs": DateTime.now().millisecondsSinceEpoch,
        };
        String inputEvent =
            JsonMapper.serialize(InputProperties(inputProperties));

        store.store.msgMouseEvent(inputEvent);
      },
      child: KeyboardListener(
        focusNode: focusNode,
        autofocus: false,
        onKeyEvent: (KeyEvent ev) {
          // print(focusNode);
          // print("Keyboard $ev");
          final inputProperties = {
            "chars": ev.character,
            "keyLabel": ev.logicalKey.keyLabel.toString(),
            "runtimeType": ev.runtimeType.toString(),
            "timestampMs": DateTime.now().millisecondsSinceEpoch,
          };
          String inputEvent =
              JsonMapper.serialize(InputProperties(inputProperties));
          // print(inputProperties);
          store.store.msgKeyboardEvent(inputEvent);
        },
        child: RawGestureDetector(
            behavior: HitTestBehavior.translucent,
            gestures: <Type, GestureRecognizerFactory>{
              ImmediateMultiDragGestureRecognizer:
                  GestureRecognizerFactoryWithHandlers<
                      ImmediateMultiDragGestureRecognizer>(
                () => ImmediateMultiDragGestureRecognizer(),
                (ImmediateMultiDragGestureRecognizer instance) {
                  // print("button handlestart ${instance.hashCode}");
                  // instance.onStart = _handleStart;
                },
              ),
              // TapGestureRecognizer:
              //     GestureRecognizerFactoryWithHandlers<
              //         TapGestureRecognizer>(
              //   () => TapGestureRecognizer(),
              //   (TapGestureRecognizer instance) {
              //     instance
              //       ..onTapDown = (TapDownDetails details) {
              //         startTimeUTC = DateTime.now().toUtc();
              //         String event = details.toString();
              //         widget.msgSendEvent(event);
              //         // tools.log.v("size - type ${details.kind}");

              //         // sl<InteractionManager>().onTapDown(
              //         //   [
              //         //     buildContext,
              //         //     details,
              //         //     startTimeUTC,
              //         //   ],
              //         // );
              //         // setState(() {
              //         //   _last = "tap down ${details.kind}";
              //         // });
              //       }
              //       ..onTapUp = (TapUpDetails details) {
              //         String event = details.toString();
              //         widget.msgSendEvent(event);
              //         setState(() {
              //           _last = 'up ${details.globalPosition}';
              //         });
              //       }
              //       // ..onTapCancel = () {
              //       //   setState(() {
              //       //     _last = 'cancel';
              //       //   });
              //       // }
              //       ..onSecondaryTapDown = (TapDownDetails details) {
              //         if (details.kind == PointerDeviceKind.mouse) {
              //           String event = details.toString();
              //           widget.msgSendEvent(event);
              //         }
              //         // sl<InteractionManager>().onSecondaryTapDown(
              //         //     [buildContext, details]);
              //         setState(() {
              //           _last = "secondary ${details.globalPosition}";
              //         });
              //       };
              //   },
              // ),

              // DoubleTapGestureRecognizer:
              //     GestureRecognizerFactoryWithHandlers<
              //             DoubleTapGestureRecognizer>(
              //         () => DoubleTapGestureRecognizer(),
              //         (DoubleTapGestureRecognizer instance) {
              //   instance
              //     ..onDoubleTap = () {
              //       setState(() {
              //         _last = 'double tap';
              //       });
              //     };
              // }),
              // MultiTapGestureRecognizer:
              //     GestureRecognizerFactoryWithHandlers<
              //             MultiTapGestureRecognizer>(
              //         () => MultiTapGestureRecognizer(),
              //         (MultiTapGestureRecognizer instance) {
              //   instance.onTapDown = (int, TapDownDetails details) {
              //     print("multi $int ${details.kind}");
              //     // setState(() {
              //     //   _last = 'multi tap #$int $details';
              //     // });
              //   };
              //   // ..onTap = ((number) {
              //   //   print("multi ${number}");
              //   // })
              //   // ..onTapUp = (number, TapUpDetails details) {
              //   //   print("multi ${details.kind}");
              //   // };
              // }),
              // PanGestureRecognizer:
              //     GestureRecognizerFactoryWithHandlers<
              //             PanGestureRecognizer>(
              //         () => PanGestureRecognizer(),
              //         (PanGestureRecognizer instance) {
              //   instance
              //     ..onStart = (DragStartDetails details) {
              //       startTimeUTC = DateTime.now().toUtc();
              //       String event = details.toString();
              //       widget.msgSendEvent(event);

              //       // sl<InteractionManager>().onStartPanning(
              //       //   [
              //       //     buildContext,
              //       //     details,
              //       //     startTimeUTC,
              //       //   ],
              //       // );

              //       setState(() {
              //         _last =
              //             'panning start ${details.globalPosition}';
              //       });
              //     }
              //     ..onUpdate = (DragUpdateDetails details) {
              //       startTimeUTC = DateTime.now().toUtc();

              //       // sl<InteractionManager>().onPointerMove(
              //       //   [
              //       //     buildContext,
              //       //     details,
              //       //     startTimeUTC,
              //       //   ],
              //       // );
              //       String event = details.toString();
              //       widget.msgSendEvent(event);
              //       setState(() {
              //         _last =
              //             'panning update ${details.localPosition} ${details.delta}';
              //       });
              //     }
              //     ..onEnd = (DragEndDetails details) {
              //       startTimeUTC = DateTime.now().toUtc();

              //       // sl<InteractionManager>().onPanEnd(
              //       //   [
              //       //     buildContext,
              //       //     details,
              //       //     startTimeUTC,
              //       //   ],
              //       // );

              //       setState(() {
              //         _last = 'panning end ${details.velocity} ';
              //       });
              //     };
              // }),
              // LongPressGestureRecognizer:
              //     GestureRecognizerFactoryWithHandlers<
              //             LongPressGestureRecognizer>(
              //         () => LongPressGestureRecognizer(),
              //         (LongPressGestureRecognizer instance) {
              //   instance
              //     ..onLongPressStart =
              //         (LongPressStartDetails details) {
              //       String event = details.toString();
              //       widget.msgSendEvent(event);
              //       startTimeUTC = DateTime.now().toUtc();

              //       // sl<InteractionManager>().onLongPressStart([
              //       //   buildContext,
              //       //   details,
              //       //   startTimeUTC,
              //       // ]);
              //       setState(() {
              //         _last =
              //             'long press start ${details.globalPosition}';
              //       });
              //     }
              //     ..onLongPressMoveUpdate =
              //         (LongPressMoveUpdateDetails details) {
              //       startTimeUTC = DateTime.now().toUtc();
              //       String event = details.toString();
              //       widget.msgSendEvent(event);

              //       // sl<InteractionManager>().onLongPressMove([
              //       //   buildContext,
              //       //   details,
              //       //   startTimeUTC,
              //       // ]);
              //       setState(() {
              //         _last =
              //             'long press move ${details.globalPosition}';
              //       });
              //     }
              //     ..onLongPressEnd = (LongPressEndDetails details) {
              //       startTimeUTC = DateTime.now().toUtc();
              //       String event = details.toString();
              //       widget.msgSendEvent(event);

              //       // sl<InteractionManager>().onLongPressEnd([
              //       //   buildContext,
              //       //   details,
              //       //   startTimeUTC,
              //       // ]);
              //       setState(() {
              //         _last =
              //             'long press end ${details.globalPosition}';
              //       });
              //     };
              // })
            },
            child: Screenshot(
              controller: screenshotControl,
              child: CanvasLayout(
                storedContext: context,
              ),
            )),
      ),
    );
  }
}
