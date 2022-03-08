import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:recase/recase.dart';
import 'package:rheetah/providers/store_provider.dart';

import '../widgets/block.dart';

class CommandWidget extends SuperBlock {
  CommandWidget({
    Key? key,
    required this.treeNode,
    required this.inputs,
    required this.outputs,
    required this.label,
    this.child,
    required this.parentId,
  }) : super(key: key);

  final TreeNode treeNode;
  final List<Widget> inputs;
  final List<Widget> outputs;
  final String label;
  final HookConsumerWidget? child;
  final String parentId;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.watch(nodeController);
    final selectedIds = ref.watch(selectedNodeIds);
    final selected = selectedIds.contains(parentId);

    ReCase rc = ReCase(label);

    return Container(
      // decoration: selected
      //     ? BoxDecoration(boxShadow: [
      //         BoxShadow(
      //             color: Colors.yellow,
      //             offset: Offset(5, 5),
      //             blurRadius: 15.0,
      //             blurStyle: BlurStyle.normal)
      //       ])
      //     : null,
      child: Column(
        children: [
          Container(
            height: 30,
            child: Padding(
              padding: EdgeInsets.fromLTRB(0, 0, 0, 5),
              child: Text(
                rc.titleCase,
                style: TextStyle(
                    color: Colors.white,
                    fontSize: 20.00,
                    fontWeight: FontWeight.bold),
              ),
            ),
          ),
          Stack(clipBehavior: Clip.none, children: [
            Row(
              children: [
                Container(width: 15, color: Colors.transparent),
                Expanded(
                  child: Card(
                    shape: ref.read(selectedNode(selected)),
                    color: treeNode.node.value.running
                        ? Color(Color.fromARGB(255, 238, 218, 39).value)
                        : treeNode.node.value.success == "success"
                            ? Color(Color.fromARGB(146, 143, 255, 147).value)
                            : (treeNode.node.value.success == "fail"
                                ? Color(
                                    Color.fromARGB(255, 255, 143, 135).value)
                                : Color.fromARGB(255, 255, 255, 255)),
                    margin: EdgeInsets.fromLTRB(0, 0, 0, 0),
                    child: Container(
                      decoration: BoxDecoration(
                          borderRadius: BorderRadius.circular(5),
                          border: Border.all(
                              color: treeNode.node.value.success == "success"
                                  ? Color(
                                      Color.fromARGB(255, 22, 158, 26).value)
                                  : (treeNode.node.value.success == "fail"
                                      ? Color(Colors.red.value)
                                      : Color.fromARGB(255, 255, 255, 255)),
                              width: 5)),
                      //adjustment for input size
                      width: treeNode.node.value.width.toDouble() - 30,
                      height: treeNode.node.value.height.toDouble() - 30,
                      child: child,
                    ),
                  ),
                ),
                Container(width: 15, color: Colors.transparent),
              ],
            ),
            inputs.length > 0
                ? Positioned(
                    left: 0,
                    child: Column(
                      mainAxisSize: MainAxisSize.max,
                      children: inputs,
                    ),
                  )
                : Container(),
            outputs.length > 0
                ? Positioned(
                    right: 0,
                    child: Column(
                      mainAxisSize: MainAxisSize.max,
                      children: outputs,
                    ),
                  )
                : Container(),
            if (treeNode.node.value.success == "fail")
              Positioned(
                  bottom: -65,
                  child: Padding(
                    padding: const EdgeInsets.all(15.0),
                    child: Container(
                        decoration: BoxDecoration(
                            color: Colors.red,
                            borderRadius: BorderRadius.circular(5),
                            border: Border.all(color: Colors.red, width: 2)),
                        height: 50,
                        width: treeNode.node.value.width.toDouble() - 30,
                        child: Text(
                          treeNode.node.value.error,
                          style: TextStyle(fontSize: 12),
                          textAlign: TextAlign.center,
                        )),
                  ))
          ]),
        ],
      ),
    );
  }
}
