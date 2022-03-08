import 'dart:collection';

import 'package:flutter/material.dart';

import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:rheetah/providers/store_provider.dart';

abstract class SuperBlock extends HookConsumerWidget {
  final List<Widget>? children;

  MapEntry<String, rid.NodeView>? node;

  final bool? selected;

  SuperBlock({Key? key, this.node, this.children, this.selected});
}

class Block extends SuperBlock {
  Block({
    Key? key,
    required this.treeNode,
  }) : super(key: key);

  final TreeNode treeNode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(nodeController);

    // print("rebuilding block $key");

    // print(treeNode);
    // get the tree node based on the id
    final newTreeNode = ref.read(treeNodeController).get(treeNode.node.key);

    // final nodeFrom = provider.entries.firstWhere((element) {
    //   return element.key == this.node.key;
    // });

    print(selected);
    return ProviderScope(
      overrides: [currentNode.overrideWithValue(newTreeNode)], //not working
      child: Positioned(
        height: newTreeNode.node.value.height.toDouble(),
        width: newTreeNode.node.value.width.toDouble(),
        left: newTreeNode.node.value.x.toDouble(),
        top: newTreeNode.node.value.y.toDouble(),
        child: Container(
          color: Colors.transparent,

          // margin: EdgeInsets.all(5),
          // color: Colors.transparent, // ? Colors.amber : Colors.white,
          child: Stack(
            children: [...newTreeNode.children!],
          ),
        ),
      ),
    );
  }
}
