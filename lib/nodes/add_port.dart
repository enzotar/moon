import 'dart:collection';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

import 'package:flutter/material.dart';
<<<<<<< HEAD
import 'package:moon/nodes/input_disk.dart';
import 'package:moon/nodes/port_entry.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:tuple/tuple.dart';

enum PortType { output, input }
=======
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
>>>>>>> master

List<Widget> addPort(
  // PortType kind,
  // List<String> labels,
  // Color color,
  SplayTreeMap<int, Tuple2<String, rid.NodeView>> ports,
<<<<<<< HEAD
  Ref _ref,
  String commandName,
) {
  return ports.values.map((nodeEntry) {
    // find filled ports

    if (nodeEntry.item2.widgetType == rid.NodeViewType.WidgetInput) {
      return InputPort(
        key: ObjectKey(nodeEntry),
        nodeEntry: nodeEntry,
        commandName: commandName,
      );
    } else {
      return OutputPort(
        key: ObjectKey(nodeEntry),
        nodeEntry: nodeEntry,
        commandName: commandName,
=======
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
>>>>>>> master
      );
    }
  }).toList();
}

<<<<<<< HEAD
class InputPort extends HookConsumerWidget {
  const InputPort({
    Key? key,
    required this.nodeEntry,
    required this.commandName,
  }) : super(key: key);

  final Tuple2<String, rid.NodeView> nodeEntry;
  // final List edges;
  final String commandName;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding input port");
    return Tooltip(
        // triggerMode: TooltipTriggerMode.longPress,
        waitDuration: const Duration(milliseconds: 850),
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(5),
          border: Border.all(color: Colors.blueGrey.shade700, width: 1),
          color: Colors.lightBlue.shade50,
        ),
        height: 50,
        padding: const EdgeInsets.all(8.0),
        preferBelow: false,
        textStyle: const TextStyle(fontSize: 18, color: Colors.black87),
        message: nodeEntry.item2.tooltip,
        child: Container(
          width: 120,
          height: 50,
          decoration: const BoxDecoration(
            // backgroundBlendMode: BlendMode.multiply,
            color: Colors.transparent,
          ),
          child: Row(
            children: [
              InputDisk(nodeEntry: nodeEntry, key: ObjectKey(nodeEntry.item2)),
              BasicPort(PortType.input, nodeEntry, commandName)
            ],
          ),
        ));
  }
}

class OutputPort extends HookConsumerWidget {
  const OutputPort({
    Key? key,
    required this.nodeEntry,
    required this.commandName,
  }) : super(key: key);

  final Tuple2<String, rid.NodeView> nodeEntry;
  final String commandName;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print(nodeEntry.item1);

    // find highlighted ports
    String? highlighted = ref.watch(highlightedPort.select((list) {
      final match = list.where((element) => element == nodeEntry.item1);
      if (match.isNotEmpty) {
        // print("is highlighted");
        return match.first;
      }
    }));

    List<String> edges = [];

    bool currentlyDragged = false;
    ref.watch(nodeController.select((map) {
      final mapList = map.entries.where(
        (entry) => entry.key == nodeEntry.item1,
      );
      if (mapList.isNotEmpty) {
        // print("get outbound edges");
        edges = mapList.first.value.flowOutboundEdges;

        final dummy = ref
            .read(edgeController)
            .entries
            .where((element) => element.key == "dummy_edge");

        if (dummy.isNotEmpty && dummy.first.value.from == nodeEntry.item1)
          currentlyDragged = true;
        return mapList.first.value.flowOutboundEdges;
      }
      ;
    }));
    // print(edges);
=======
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
>>>>>>> master

    return Container(
      width: 120,
      height: 50,
      decoration: BoxDecoration(
<<<<<<< HEAD
        color: Colors.transparent,
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.end,
        children: [
          BasicPort(PortType.output, nodeEntry, commandName),
=======
        color: highlightedPort.contains(nodeEntry.item1)
            ? Colors.yellowAccent
            : Colors.transparent,
      ),
      child: Row(
        children: [
>>>>>>> master
          Container(
            width: 30,
            child: ElevatedButton(
              onHover: (value) {},
              // color:
              //     edges.contains(nodeEntry.key) ? Colors.green : Colors.white,
              onPressed: () {},
              style: ElevatedButton.styleFrom(
<<<<<<< HEAD
                primary: (edges.length > 0 || currentlyDragged == true) &&
                        highlighted == null
                    ? Colors.amber
                    : highlighted != null
                        ? Color.fromARGB(255, 168, 216, 114)
                        : Colors.white,
=======
                primary: edges.contains(nodeEntry.item1)
                    ? Colors.yellow
                    : Colors.white,
>>>>>>> master
                fixedSize: const Size(30, 30),
                shape: const CircleBorder(
                  side: BorderSide(
                    color: Colors.black26,
                    style: BorderStyle.solid,
                  ),
                ),
              ),
<<<<<<< HEAD
              child: edges.length > 1
                  ? Text(
                      edges.length.toString(),
                      textAlign: TextAlign.center,
                    )
                  : null,
            ),
          ),
=======
              child: null,
            ),
          ),
          PortEntry(inputType, nodeEntry)
>>>>>>> master
        ],
      ),
    );
  }
}
