import 'dart:collection';

import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/widgets/block.dart';
import 'package:moon/edge.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:moon/widget_builder.dart';

import 'graph_selection.dart';

class CanvasLayout extends HookConsumerWidget {
  CanvasLayout({Key? key, required BuildContext this.storedContext})
      : super(key: key);

  BuildContext storedContext;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(canvasProvider);
    // AsyncValue<rid.Store> storeProvider = ref.watch(storeStreamProvider);
    // AsyncValue<rid.Store> refreshProvider = ref.watch(refreshStreamProvider);

    final HashMap<String, rid.NodeView> nodes = provider.nodes;
    final HashMap<String, rid.NodeView> vertexNodes = HashMap.fromEntries(nodes
        .entries
        .where((element) => element.value.widgetType.name == "WidgetBlock"));

    final HashMap<String, rid.EdgeView> flowEdges = provider.flowEdges;

    // if (provider.selectedNodeIds.isEmpty) {
    //   FocusScope.of(storedContext).children.last.requestFocus();
    // }
    final tree = returnWidgetTreeFunction(nodes, vertexNodes, provider);

    return SizedBox.expand(
      child: Stack(children: [
        Edges(flowEdges: flowEdges),
        Nodes(nodeWidgetList: tree[0])
      ]),
    );
  }
}

class Nodes extends HookConsumerWidget {
  final List<HookConsumerWidget> nodeWidgetList;

  Nodes({Key? key, required List<HookConsumerWidget> this.nodeWidgetList})
      : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // final provider = ref.watch(nodeProvider);
    return Stack(
      children: this.nodeWidgetList,
    );
  }
}

class Edges extends HookConsumerWidget {
  final HashMap<String, rid.EdgeView> flowEdges;

  Edges({Key? key, required HashMap<String, rid.EdgeView> this.flowEdges})
      : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(edgeProvider);

    return Stack(
      // children:
      // flowEdges.values.map((e) => addEdgeWidget(e)).toList(),
      children: provider.entries.map((e) => addEdgeWidget(e)).toList(),
    );
  }
}

EdgeWidget addEdgeWidget(MapEntry<String, rid.EdgeView> edgeElement) {
  /// Set inputs
  final MapEntry<String, rid.EdgeView> _edgeElement = edgeElement;

  EdgeWidget buildEdge = EdgeWidget(
    edgePainter: EdgePainter(edgeEntry: _edgeElement),
  );
  return buildEdge;
}




    // return SizedBox.expand(
    //   child: storeProvider.when(
    //     loading: () {
    //       List<rid.GraphEntry> graphList = rid.Store.instance.view.graphList;

    //       return GraphSelection(graphList);
    //     },
    //     error: (err, stack) => Text('Error: $err'),
    //     data: (store) {
    //     },
    //   ),
    // );

    //     return SizedBox.expand(
    //   child: storeProvider.when(
    //     loading: () {
    //       List<rid.GraphEntry> graphList = rid.Store.instance.view.graphList;

    //       return GraphSelection(graphList);
    //     },
    //     error: (err, stack) => Text('Error: $err'),
    //     data: (store) {
    //       final HashMap<String, rid.NodeView> nodes = store.view.nodes;
    //       final HashMap<String, rid.NodeView> vertexNodes = HashMap.fromEntries(
    //           nodes.entries.where(
    //               (element) => element.value.widgetType.name == "WidgetBlock"));

    //       final HashMap<String, rid.EdgeView> flowEdges = store.view.flowEdges;

    //       // print(store.view.nodes);

    //       // print(store.view.selectedNodeIds);

    //       if (store.view.selectedNodeIds.isEmpty) {
    //         FocusScope.of(storedContext).children.last.requestFocus();
    //       }
    //       final tree = returnWidgetTreeFunction(nodes, vertexNodes, store);

    //       return SizedBox.expand(
    //         child: Stack(children: [
    //           Stack(
    //             // children:
    //             // flowEdges.values.map((e) => addEdgeWidget(e)).toList(),
    //             children:
    //                 flowEdges.values.map((e) => addEdgeWidget(e)).toList(),
    //           ),
    //           SizedBox.expand(child: Stack(children: tree[0]))
    //         ]),
    //       );
    //     },
    //   ),
    // );