import 'dart:collection';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

import 'package:flutter/material.dart';
import 'package:moon/nodes/port_entry.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:moon/widget_input.dart';
import 'package:tuple/tuple.dart';

// enum PortType {
//   Input,
//   Output,
// }

//  edges.contains(nodeEntry.key)
//                         ? Colors.black45
//                         : Colors.white24,

List<Widget> addPort(
  // PortType kind,
  // List<String> labels,
  // Color color,
  SplayTreeMap<int, Tuple2<String, rid.NodeView>> ports,
  List<String> highlightedPort,
  rid.View store,
  String commandName,
) {
// find the command
  final textCommand = store.textCommands
      .where((element) => element.widgetName == commandName)
      .toList();
  // print(textCommand);

  return ports.values.map((nodeEntry) {
    // find filled ports
    List<String> inboundEdges = nodeEntry.item2.flowInboundEdges;
    List<String> outboundEdges = nodeEntry.item2.flowOutboundEdges;
    List edges = [...inboundEdges, ...outboundEdges];

//     // get first acceptable kind
//     final outputType = outputName.name;

    if (nodeEntry.item2.widgetType == rid.NodeViewType.WidgetInput) {
      final inputName = textCommand[0].inputs.where((input) {
        return input.name == nodeEntry.item2.text;
      }).toList();
      final inputType =
          inputName.isNotEmpty ? inputName[0].acceptableKinds[0] : "wait";
      // print(inputType);

      return InputWidget(
        nodeEntry: nodeEntry,
        edges: edges,
        inputType: inputType,
        highlightedPort: highlightedPort,
      );
    } else {
      final outputName = textCommand[0]
          .outputs
          .where((output) => output.name == nodeEntry.item2.text)
          .toList();
      final outputType = outputName.isNotEmpty ? outputName[0].kind : "wait";
      // print(outputType);

      return Container(
        width: 120,
        height: 50,
        decoration: BoxDecoration(
          color: highlightedPort.contains(nodeEntry.item1)
              ? Colors.yellowAccent
              : Colors.transparent,
        ),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.end,
          children: [
            Padding(
              padding: const EdgeInsets.fromLTRB(0, 0, 5, 0),
              child: Text(
                nodeEntry.item2.text,
                style: TextStyle(),
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
              ),
            ),
            Container(
              width: 30,
              child: ElevatedButton(
                onHover: (value) {},
                // color:
                //     edges.contains(nodeEntry.key) ? Colors.green : Colors.white,
                onPressed: () {},
                style: ElevatedButton.styleFrom(
                  primary: edges.contains(nodeEntry.item1)
                      ? Colors.yellow
                      : Colors.white,
                  // fixedSize: const Size(30, 30),
                  shape: const CircleBorder(
                    side: BorderSide(
                      color: Colors.black12,
                      style: BorderStyle.solid,
                    ),
                  ),
                ),
                child: null,
              ),
            ),
          ],
        ),
      );
    }
  }).toList();
}

class InputWidget extends HookConsumerWidget {
  const InputWidget({
    Key? key,
    required this.nodeEntry,
    required this.edges,
    required this.inputType,
    required this.highlightedPort,
  }) : super(key: key);

  final Tuple2<String, rid.NodeView> nodeEntry;
  final List edges;
  final String inputType;
  final List<String> highlightedPort;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(edgeProvider);

    return Container(
      width: 120,
      height: 50,
      decoration: BoxDecoration(
        color: highlightedPort.contains(nodeEntry.item1)
            ? Colors.yellowAccent
            : Colors.transparent,
      ),
      child: Row(
        children: [
          Container(
            width: 30,
            child: ElevatedButton(
              onHover: (value) {},
              // color:
              //     edges.contains(nodeEntry.key) ? Colors.green : Colors.white,
              onPressed: () {},
              style: ElevatedButton.styleFrom(
                primary: edges.contains(nodeEntry.item1)
                    ? Colors.yellow
                    : Colors.white,
                fixedSize: const Size(30, 30),
                shape: const CircleBorder(
                  side: BorderSide(
                    color: Colors.black26,
                    style: BorderStyle.solid,
                  ),
                ),
              ),
              child: null,
            ),
          ),
          PortEntry(inputType, nodeEntry)
        ],
      ),
    );
  }
}
