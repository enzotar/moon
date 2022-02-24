import 'package:flutter/material.dart';

// import '../config/keys.dart';
// import '../config/service_locators.dart';
// import '../data/canvas_commands.dart';
// import '../foundation/graph.dart';
// import 'superstateful.dart';

import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

class WidgetInput extends HookConsumerWidget {
  const WidgetInput({
    Key? key,
    required this.node,
    required this.children,
    required this.selected,
  }) : super(key: key);

  final rid.NodeView node;
  final bool selected;
  final List<Widget> children;

  // InputsOutputs? _inputsOutputs = InputsOutputs.owner;
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Positioned(
        height: node.height.toDouble(),
        width: node.width.toDouble(),
        left: node.x.toDouble(),
        top: node.y.toDouble(),
        child: Container(
          color: Colors.greenAccent,
        ));
  }
}
