// import 'dart:collection';

// import 'package:flutter/material.dart';
// import 'package:flutter_hooks/flutter_hooks.dart';
// import 'package:hooks_riverpod/hooks_riverpod.dart';
// import 'package:plugin/generated/rid_api.dart' as rid;
// import 'package:moon/canvas.dart';
// import 'package:moon/widgets/block.dart';
// import 'package:moon/edge.dart';
// import 'package:moon/providers/store_provider.dart';
// import 'package:moon/widget_builder.dart';

// import 'graph_selection.dart';

// class CanvasScreenshot extends HookConsumerWidget {
//   CanvasScreenshot({
//     Key? key,
//     required rid.Camera this.transform,
//   }) : super(key: key);

//   final rid.Camera transform;
//   @override
//   Widget build(BuildContext context, WidgetRef ref) {
//     // final provider = ref.watch(viewportController);

//     // final transform = ref.read(storeRepoProvider).transform;

//     // if(){
//     //   FocusScope.of(storedContext).requestFocus();
//     // }

//     // final transform = provider.first;

//     final tf = useTransformationController();

//     tf.value = Matrix4.identity()
//       ..scale(
//         transform.scale.numer.toDouble() / transform.scale.denom.toDouble(),
//         transform.scale.numer.toDouble() / transform.scale.denom.toDouble(),
//       )
//       ..translate(
//         transform.x.numer.toDouble() / transform.x.denom.toDouble(),
//         transform.y.numer.toDouble() / transform.y.denom.toDouble(),
//       );

//     // print("rebuilding canvas");
// // InteractiveViewer
//     // return InteractiveViewer(
//     //   // minScale: 0.1, maxScale: 1,
//     //   transformationController: tf,
//     //   // scaleEnabled: true,
//     //   clipBehavior: Clip.none,
//     //   // panEnabled: true,
//     //   child: Container(
//     //       // width: 5000,
//     //       // height: 5000,
//     //       color: Colors.blueGrey,
//     //       child: Stack(children: [Edges(), Nodes()])),
//     // );
//     // //     transform: Matrix4.identity()
//     // //       ..scale(
//     // //         transform.scale.numer.toDouble() / transform.scale.denom.toDouble(),
//     // //         transform.scale.numer.toDouble() / transform.scale.denom.toDouble(),
//     // //       )
//     // //       ..translate(
//     // //         transform.x.numer.toDouble() / transform.x.denom.toDouble(),
//     // //         transform.y.numer.toDouble() / transform.y.denom.toDouble(),
//     // //       ),
//     // //     transformHitTests: true,
//     // //     child:
//     // //   ),
//     // // );

//     // print("rebuilding canvas");
// // InteractiveViewer // !UnconstrainedBox, SizedOverflowBox
//     return OverflowBox(
//       alignment: Alignment.topLeft,
//       minWidth: 0.0,
//       minHeight: 0.0,
//       maxWidth: 5000,
//       maxHeight: 5000,
//       child: Transform(
//         transform: tf.value,
//         transformHitTests: true,
//         child: Container(
//           // width: 5000,
//           // height: 5000,
//           color: Colors.blueGrey,
//           child: Stack(
//             children: [
//               const Edges(
//                   // key: UniqueKey(),
//                   ),
//               // Nodes()
//               ref.watch(constNodeProvider)
//             ],
//           ),
//         ),
//       ),
//     );
//   }
// }
