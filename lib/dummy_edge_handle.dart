import 'package:flutter/material.dart';

// import '../config/keys.dart';
// import '../config/service_locators.dart';
// import '../data/canvas_commands.dart';
// import '../foundation/graph.dart';
// import 'superstateful.dart';

import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/providers/store_provider.dart';

class DummyEdgeHandle extends HookConsumerWidget {
  const DummyEdgeHandle({Key? key, required this.treeNode}) : super(key: key);

  final TreeNode treeNode;
  // InputsOutputs? _inputsOutputs = InputsOutputs.owner;
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding dummy edge handle");
    final provider = ref.watch(widgetTreeController);
    // final nodeFrom = provider.entries.firstWhere((element) {
    //   return element.key == this.node.key;
    // });
    return Positioned(
        // height: nodeFrom.value.height.toDouble(),
        // width: nodeFrom.value.width.toDouble(),
        // left: nodeFrom.value.x.toDouble(),
        // top: nodeFrom.value.y.toDouble(),
        child: Container(
      color: Colors.yellowAccent,
    ));
  }
}
