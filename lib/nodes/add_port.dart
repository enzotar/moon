import 'dart:collection';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

import 'package:flutter/material.dart';
import 'package:rheetah/nodes/port_entry.dart';
import 'package:rheetah/providers/store_provider.dart';
import 'package:rheetah/widget_input.dart';
import 'package:tuple/tuple.dart';

enum PortType { output, input }

List<Widget> addPort(
  // PortType kind,
  // List<String> labels,
  // Color color,
  SplayTreeMap<int, Tuple2<String, rid.NodeView>> ports,
  Ref _ref,
  String commandName,
) {
// find the command
  final textCommand = _ref
      .read(storeRepoProvider)
      .text_commands
      .where((element) => element.widgetName == commandName)
      .toList();

  return ports.values.map((nodeEntry) {
    // find filled ports

//     // get first acceptable kind
//     final outputType = outputName.name;

    if (nodeEntry.item2.widgetType == rid.NodeViewType.WidgetInput) {
      final inputName = textCommand[0].inputs.where((input) {
        print(input.name);
        print(nodeEntry.item2.text);
        return input.name == nodeEntry.item2.text;
      }).toList();
      final inputType = inputName[0].acceptableKinds[0];

      return InputWidget(
        nodeEntry: nodeEntry,
        inputType: inputType,
        // key: UniqueKey(),
      );
    } else {
      final outputName = textCommand[0]
          .outputs
          .where((output) => output.name == nodeEntry.item2.text)
          .toList();

      final outputType = outputName[0].kind;

      return OutputPort(
        nodeEntry: nodeEntry,
        outputType: outputType,
        // key: UniqueKey(),
      );
    }
  }).toList();
}

class InputWidget extends HookConsumerWidget {
  const InputWidget({
    Key? key,
    required this.nodeEntry,
    required this.inputType,
  }) : super(key: key);

  final Tuple2<String, rid.NodeView> nodeEntry;
  // final List edges;
  final String inputType;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(edgeController);
    ref.watch(nodeController);
    // find highlighted ports
    List<String> highlightedPort = ref.watch(storeRepoProvider).highlighted;

    // get latest node entry
    final newNodeEntry = ref
        .read(nodeController)
        .entries
        .where((element) => element.key == nodeEntry.item1)
        .single;

    List<String> inboundEdges = newNodeEntry.value.flowInboundEdges;
    List<String> outboundEdges = newNodeEntry.value.flowOutboundEdges;
    List edges = [...inboundEdges, ...outboundEdges];

    final dummy = provider.values.where(
      (element) {
        return element.to == nodeEntry.item1;
      },
    ).toList();

    // final edgeEntries =
    //     ref.read(storeRepoProvider).flow_edges.entries.where(((element) {
    //   return edges.contains(element.key);
    // })).toList();

    return Container(
      width: 120,
      height: 50,
      decoration: BoxDecoration(
        // backgroundBlendMode: BlendMode.multiply,
        color: Colors.transparent,
      ),
      child: Row(
        children: [
          Container(
            width: 30,
            child: edges.length <= 1
                ? ElevatedButton(
                    onHover: (value) {},
                    // color:
                    //     edges.contains(nodeEntry.key) ? Colors.green : Colors.white,
                    onPressed: () {},
                    style: ElevatedButton.styleFrom(
                      primary: edges.length != 1 || dummy.isEmpty
                          ? Colors.white
                          : highlightedPort.contains(nodeEntry.item1)
                              ? Color.fromARGB(255, 168, 216, 114)
                              : Colors.amber,
                      fixedSize: const Size(30, 30),
                      shape: const CircleBorder(
                        side: BorderSide(
                          color: Colors.black26,
                          style: BorderStyle.solid,
                        ),
                      ),
                    ),
                    child: edges.length > 1
                        ? Text(
                            edges.length.toString(),
                            textAlign: TextAlign.center,
                          )
                        : null,
                  )
                : Tooltip(
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(5),
                      color: Colors.red,
                    ),
                    height: 50,
                    padding: const EdgeInsets.all(8.0),
                    preferBelow: false,
                    textStyle:
                        const TextStyle(fontSize: 24, color: Colors.white),
                    // showDuration: const Duration(seconds: 2),
                    // waitDuration: const Duration(seconds: 0),
                    message:
                        "multiple edges not yet supported, remove one edge",
                    child: ElevatedButton(
                      onHover: (value) {},
                      // color:
                      //     edges.contains(nodeEntry.key) ? Colors.green : Colors.white,
                      onPressed: () {},
                      style: ElevatedButton.styleFrom(
                        primary: Colors.red,
                        fixedSize: const Size(30, 30),
                        shape: const CircleBorder(
                          side: BorderSide(
                            color: Colors.black26,
                            style: BorderStyle.solid,
                          ),
                        ),
                      ),
                      child: Text(
                        edges.length.toString(),
                        textAlign: TextAlign.center,
                      ),
                    ),
                  ),
          ),
          PortEntry(PortType.input, inputType, nodeEntry)
        ],
      ),
    );
  }
}

class OutputPort extends HookConsumerWidget {
  const OutputPort({
    Key? key,
    required this.nodeEntry,
    required this.outputType,
  }) : super(key: key);

  final Tuple2<String, rid.NodeView> nodeEntry;
  final String outputType;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(edgeController);
    ref.watch(nodeController);

    // find highlighted ports
    List<String> highlightedPort = ref.watch(storeRepoProvider).highlighted;
    final newNodeEntry = ref
        .read(nodeController)
        .entries
        .where((element) => element.key == nodeEntry.item1)
        .single;

    List<String> inboundEdges = newNodeEntry.value.flowInboundEdges;
    List<String> outboundEdges = newNodeEntry.value.flowOutboundEdges;
    List edges = [...inboundEdges, ...outboundEdges];

    final dummy = provider.values.where(
      (element) {
        return element.from == nodeEntry.item1;
      },
    ).toList();
    return Container(
      width: 120,
      height: 50,
      decoration: BoxDecoration(
        color: Colors.transparent,
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.end,
        children: [
          PortEntry(PortType.output, outputType, nodeEntry),
          Container(
            width: 30,
            child: ElevatedButton(
              onHover: (value) {},
              // color:
              //     edges.contains(nodeEntry.key) ? Colors.green : Colors.white,
              onPressed: () {},
              style: ElevatedButton.styleFrom(
                primary: edges.length < 1 && dummy.isEmpty
                    ? Colors.white
                    : highlightedPort.contains(nodeEntry.item1)
                        ? Color.fromARGB(255, 168, 216, 114)
                        : Colors.amber,
                fixedSize: const Size(30, 30),
                shape: const CircleBorder(
                  side: BorderSide(
                    color: Colors.black26,
                    style: BorderStyle.solid,
                  ),
                ),
              ),
              child: edges.length > 1
                  ? Text(
                      edges.length.toString(),
                      textAlign: TextAlign.center,
                    )
                  : null,
            ),
          ),
        ],
      ),
    );
  }
}
// Container(
//         width: 120,
//         height: 50,
//         decoration: BoxDecoration(
//           color: highlightedPort.contains(nodeEntry.item1)
//               ? Colors.yellowAccent
//               : Colors.transparent,
//         ),
//         child: Row(
//           mainAxisAlignment: MainAxisAlignment.end,
//           children: [
//             Padding(
//               padding: const EdgeInsets.fromLTRB(0, 0, 5, 0),
//               child: Text(
//                 nodeEntry.item2.text,
//                 style: TextStyle(),
//                 maxLines: 2,
//                 overflow: TextOverflow.ellipsis,
//               ),
//             ),
//             Container(
//               width: 30,
//               child: ElevatedButton(
//                 onHover: (value) {},
//                 // color:
//                 //     edges.contains(nodeEntry.key) ? Colors.green : Colors.white,
//                 onPressed: () {},
//                 style: ElevatedButton.styleFrom(
//                   primary: edges.contains(nodeEntry.item1)
//                       ? Colors.yellow
//                       : Colors.white,
//                   // fixedSize: const Size(30, 30),
//                   shape: const CircleBorder(
//                     side: BorderSide(
//                       color: Colors.black12,
//                       style: BorderStyle.solid,
//                     ),
//                   ),
//                 ),
//                 child: null,
//               ),
//             ),
//           ],
//         ),
//       );
