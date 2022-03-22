import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:recase/recase.dart';
import 'package:moon/logger.dart';
import 'package:moon/providers/popup_menu.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:flutter/services.dart';
import 'package:moon/widgets/block.dart';

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
    log.d(
        "rebuilding Command ${treeNode.node.value.widgetType} ${treeNode.node.key}");

    Future<void> _copyToClipboard(text) async {
      await Clipboard.setData(ClipboardData(text: text));
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(
        content: Text('Copied to clipboard', textAlign: TextAlign.center),
      ));
      // Scaffold.of(context).showSnackBar(snackbar)
    }

    final store = ref.read(storeRepoProvider).store;
    ref.watch(
        nodeController.select((value) => value.keys == treeNode.node.key));
    final selectedIds = ref.watch(selectedNodeIds);
    final selected = selectedIds.contains(parentId);
    final timeElapsed = Duration(milliseconds: treeNode.node.value.elapsedTime);

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
            child: Stack(
              children: [
                Container(
                  width: treeNode.node.value.width.toDouble(),
                ),
                Center(
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
                Positioned(
                  height: 30,
                  top: -5,
                  right: 10,
                  child: ref.read(
                    popUpMenuProvider(parentId),
                  ),
                )
              ],
            ),
          ),
          Stack(
            clipBehavior: Clip.none,
            children: [
              Row(
                children: [
                  Container(width: 15, color: Colors.transparent),
                  Expanded(
                    child: Card(
                      shape: ref.read(selectedNode(selected)),
                      color: treeNode.node.value.runState ==
                              rid.RunStateView.Running
                          ? Color(Color.fromARGB(255, 238, 218, 39).value)
                          : treeNode.node.value.runState ==
                                  rid.RunStateView.Success
                              ? Color(Color.fromARGB(146, 143, 255, 147).value)
                              : (treeNode.node.value.runState ==
                                      rid.RunStateView.Failed
                                  ? Color(
                                      Color.fromARGB(255, 255, 143, 135).value)
                                  : treeNode.node.value.runState ==
                                          rid.RunStateView.Canceled
                                      ? Color(Color.fromARGB(255, 185, 185, 185)
                                          .value)
                                      : Colors.white),
                      margin: EdgeInsets.fromLTRB(0, 0, 0, 0),
                      child: Container(
                        decoration: BoxDecoration(
                          borderRadius: BorderRadius.circular(5),
                          border: Border.all(
                              color: treeNode.node.value.runState ==
                                      rid.RunStateView.Running
                                  ? Color(
                                      Color.fromARGB(255, 238, 218, 39).value)
                                  : treeNode.node.value.runState ==
                                          rid.RunStateView.Success
                                      ? Color(Color.fromARGB(146, 143, 255, 147)
                                          .value)
                                      : (treeNode.node.value.runState ==
                                              rid.RunStateView.Failed
                                          ? Color(
                                              Color.fromARGB(255, 255, 143, 135)
                                                  .value)
                                          : Color.fromARGB(255, 255, 255, 255)),
                              width: 5),
                        ),
                        //adjustment for input size
                        width: treeNode.node.value.width.toDouble() - 30,
                        height: treeNode.node.value.runState ==
                                    rid.RunStateView.Failed &&
                                treeNode.node.value.widgetType !=
                                    rid.NodeViewType.Print
                            ? treeNode.node.value.height.toDouble() + 40
                            : treeNode.node.value.runState ==
                                        rid.RunStateView.Failed &&
                                    treeNode.node.value.widgetType ==
                                        rid.NodeViewType.Print
                                ? treeNode.node.value.height.toDouble() + 20
                                : treeNode.node.value.height.toDouble() - 30,
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
              if (treeNode.node.value.runState == rid.RunStateView.Failed &&
                  treeNode.node.value.widgetType != rid.NodeViewType.Print)
                Positioned(
                  bottom: 0,
                  child: Padding(
                    padding: const EdgeInsets.fromLTRB(17, 0, 0, 2),
                    child: Container(
                      decoration: BoxDecoration(
                        color: Colors.red,
                        borderRadius: BorderRadius.only(
                            bottomRight: Radius.circular(5),
                            bottomLeft: Radius.circular(5)),
                        border: Border.all(color: Colors.red, width: 2),
                      ),
                      height: 60,
                      width: treeNode.node.value.width.toDouble() - 30 - 4,
                      child: Padding(
                        padding: const EdgeInsets.fromLTRB(8, 0, 8, 0),
                        child: Row(
                          children: [
                            Expanded(
                              child: Text(
                                "Time elapsed ${timeElapsed.inSeconds}s \n${treeNode.node.value.error}",
                                style: TextStyle(fontSize: 12),
                                textAlign: TextAlign.left,
                              ),
                            ),
                            IconButton(
                                icon: Icon(Icons.copy),
                                onPressed: () {
                                  _copyToClipboard(
                                      "Time elapsed ${timeElapsed.inSeconds}s \n${treeNode.node.value.error}");
                                }),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),
              if (treeNode.node.value.runState == rid.RunStateView.Failed &&
                  treeNode.node.value.widgetType == rid.NodeViewType.Print)
                Positioned(
                  bottom: 0,
                  child: Padding(
                    padding: const EdgeInsets.fromLTRB(17, 0, 0, 2),
                    child: Container(
                      decoration: BoxDecoration(
                        color: Colors.red,
                        borderRadius: BorderRadius.only(
                            bottomRight: Radius.circular(5),
                            bottomLeft: Radius.circular(5)),
                        border: Border.all(color: Colors.red, width: 2),
                      ),
                      height: 40,
                      width: treeNode.node.value.width.toDouble() - 30 - 4,
                      child: Padding(
                        padding: const EdgeInsets.fromLTRB(8, 0, 8, 0),
                        child: Row(
                          children: [
                            Expanded(
                              child: Text(
                                "${treeNode.node.value.error}",
                                style: TextStyle(fontSize: 12),
                                textAlign: TextAlign.left,
                              ),
                            ),
                            IconButton(
                                icon: Icon(Icons.copy),
                                onPressed: () {
                                  _copyToClipboard(treeNode.node.value.error);
                                }),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),
              if (treeNode.node.value.runState == rid.RunStateView.Success &&
                  timeElapsed.inSeconds > 0)
                Positioned(
                    bottom: -28,
                    right: 10,
                    child: Container(
                      // decoration: BoxDecoration(
                      //   shape: BoxShape.circle,
                      //   color: Colors.green,
                      // ),
                      child: Padding(
                        padding: const EdgeInsets.all(8.0),
                        child: Text("Run time: ${timeElapsed.inSeconds}s"),
                      ),
                    ))
            ],
          ),
        ],
      ),
    );
  }
}
