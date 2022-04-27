import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

class CreateAccount extends HookConsumerWidget {
  CreateAccount({
    Key? key,
    required this.node,
    required this.selected,
    required this.inputs,
    required this.outputs,
  }) : super(key: key);

  final MapEntry<String, rid.NodeView> node;
  final bool selected;
  final List<Widget> inputs;
  final List<Widget> outputs;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Positioned(
        // height: node.value.height.toDouble(),
        // width: node.value.width.toDouble(),
        // left: node.value.x.toDouble(),
        // top: node.value.y.toDouble(),
        child: Card(
      color: Colors.blue.shade100,
      child: Container(
        decoration: BoxDecoration(
          color: Colors.blue.shade100,
        ),
        child: Column(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Expanded(
              child: Row(
                mainAxisSize: MainAxisSize.max,
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Column(
                    mainAxisSize: MainAxisSize.max,
                    children: inputs,
                  ),
                  Expanded(
                    child: Padding(
                      padding: EdgeInsetsDirectional.fromSTEB(10, 5, 5, 10),
                      child: Column(
                        mainAxisSize: MainAxisSize.max,
                        mainAxisAlignment: MainAxisAlignment.start,
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          Container(
                            width: 100,
                            height: 100,
                            decoration: BoxDecoration(
                              borderRadius: BorderRadius.circular(0),
                            ),
                            child: Padding(
                              padding: EdgeInsetsDirectional.fromSTEB(
                                  10, 10, 10, 10),
                              child: Text(
                                'Create Account',
                                style: TextStyle(
                                    backgroundColor: Colors.blue.shade100,
                                    fontSize: 20.00,
                                    fontWeight: FontWeight.bold),
                                // style: FlutterFlowTheme.title1
                                //     .override(
                                //   fontFamily: 'Poppins',
                                //   fontSize: 18,
                                // ),
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                  Column(
                    mainAxisSize: MainAxisSize.max,
                    children: outputs,
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    ));
  }
}



// ///
// /// Definition for Blocks
// ///
// ///

// class Block extends HookConsumerWidget {
//   final ObjectKey key;
//   // final NodeElement nodeElement;
//   final List<Widget> children;
//   // BuildContext? storedContext;

//   Block({
//     required this.nodeElement,
//     required this.key,
//     required this.children,
//     this.storedContext,
//   });

//   build(buildContext) {
//     final node = nodeElement;

//     String dataNodeType = "";
//     String nodeType = "";
//     if (node.properties["type"] != "block") {
//       dataNodeType = node.edges
//           .where((element) => element.facets["relationship"] == "data")
//           .first
//           .nodeList[1]
//           .properties["type"];

//       nodeType = node.properties["type"];
//     }

//     return RxLoader<List<Widget>>(
//       spinnerKey: AppKeys.loadingSpinner,
//       radius: 25.0,
//       commandResults: sl<CanvasCommands>().rebuildNodes.results,
//       dataBuilder: (context, data) {
//         storedContext = context;
//         return Positioned(
//           height: node.properties["type"] == "data"
//               ? 100
//               : node.properties["height"],
//           width: node.properties["type"] == "data"
//               ? 100
//               : node.properties["width"],
//           key: key,
//           left: node.properties["lx"],
//           top: node.properties["ly"],
//           child: MetaData(
//             metaData: ["block", key],
//             behavior: HitTestBehavior.translucent,
//             child: Card(
//               color: node.properties["type"] == "block"
//                   ? Colors.amber
//                   : Colors.blue,
//               child: Container(
//                 child: Stack(
//                   children: [
//                     Text(node.nodeKey),
//                     if (node.properties["type"] != "block")
//                       Text("\n $nodeType \n $dataNodeType"),

//                     ...children,

//                     ///
//                     /// Header
//                     ///
//                     ///

//                     Container(
//                       height: 60,
//                       child: Row(
//                         mainAxisAlignment: MainAxisAlignment.end,
//                         children: <Widget>[
//                           IconButton(
//                             icon: Icon(Icons.remove),
//                             onPressed: () {
//                               // removeNode(buildContext);
//                             },
//                           ),
//                           // IconButton(
//                           //   icon: Icon(Icons.add),
//                           //   onPressed: () {
//                           //     final newOffset = Offset(100, 100);
//                           //     sl<CanvasCommands>()
//                           //         .addNodeCommand([newOffset, "child"]);
//                           //   },
//                           // )
//                         ],
//                       ),
//                     ),

//                     ///
//                     /// Resize area
//                     ///
//                     Positioned(
//                       right: 0,
//                       bottom: 0,
//                       child: MetaData(
//                         metaData: ["blockSelection", key],
//                         child: Container(
//                           color: Colors.black12,
//                           height: 25,
//                           width: 25,
//                         ),
//                       ),
//                     ),
//                   ],
//                 ),
//               ),
//             ),
//           ),
//         );
//       },
//       placeHolderBuilder: (context) => Center(key: AppKeys.loadingSpinner),
//       errorBuilder: (context, ex) => Center(
//           key: AppKeys.loaderError, child: Text("Error: ${ex.toString()}")),
//     );
//   }
// }
