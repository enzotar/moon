import 'dart:collection';

import 'package:flutter/material.dart';

import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/providers/store_provider.dart';

class Block extends HookConsumerWidget {
  const Block(
      {Key? key,
      required this.node,
      required this.children,
      required this.selected})
      : super(key: key);

  final MapEntry<String, rid.NodeView> node;
  final List<Widget> children;
  final bool selected;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(nodeProvider);
    final nodeFrom = provider.entries.firstWhere((element) {
      return element.key == this.node.key;
    });
    return Positioned(
      height: nodeFrom.value.height.toDouble(),
      width: nodeFrom.value.width.toDouble(),
      left: nodeFrom.value.x.toDouble(),
      top: nodeFrom.value.y.toDouble(),
      child: Container(
        // margin: EdgeInsets.all(5),
        color: Colors.transparent, // ? Colors.amber : Colors.white,
        child: Stack(
          children: [...children],
        ),
      ),
    );
  }
}
