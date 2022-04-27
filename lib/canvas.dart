<<<<<<< HEAD
=======
import 'dart:collection';

>>>>>>> master
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
<<<<<<< HEAD
import 'package:moon/providers/bookmark.dart';
import 'package:moon/widgets/edge.dart';
import 'package:moon/providers/store_provider.dart';

class CanvasLayout extends HookConsumerWidget {
  const CanvasLayout({
    Key? key,
    required BuildContext this.storedContext,
  }) : super(key: key);

  final BuildContext storedContext;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding canvas");
    final provider = ref.watch(viewportController);

    final rid.Camera transform;

    if (provider.isEmpty) {
      transform = ref.read(storeRepoProvider).transform;
    } else {
      transform = provider.first;
    }

    final tf = useTransformationController();

    tf.value = Matrix4.identity()
      ..scale(
        transform.scale.numer.toDouble() / transform.scale.denom.toDouble(),
        transform.scale.numer.toDouble() / transform.scale.denom.toDouble(),
      )
      ..translate(
        transform.x.numer.toDouble() / transform.x.denom.toDouble(),
        transform.y.numer.toDouble() / transform.y.denom.toDouble(),
      );

    return OverflowBox(
      alignment: Alignment.topLeft,
      minWidth: 0.0,
      minHeight: 0.0,
      maxWidth: 6000,
      maxHeight: 4000,
      child: Transform(
        transform: tf.value,
        transformHitTests: true,
        child: Container(
          // width: 5000,
          // height: 5000,
          color: Colors.blueGrey.shade800,
          child: Stack(
            children: [
              GridPaper(
                color: Color.fromARGB(144, 120, 144, 156),
                divisions: 1,
                interval: 100,
                subdivisions: 1,
                child: Container(),
              ),
              ref.read(constDraggedEdgeProvider),
              ref.read(constEdgeProvider),
              // Nodes()
              ref.read(constNodeProvider),
            ],
          ),
        ),
      ),
    );
  }
}

final constNodeProvider = Provider<Nodes>((ref) {
  return const Nodes();
});

class Nodes extends HookConsumerWidget {
  const Nodes({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding nodes");
    final nodes = ref.watch(widgetTreeController).tree.nodeWidgets;
    return Stack(
      children: nodes,
=======
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
>>>>>>> master
    );
  }
}

<<<<<<< HEAD
final constEdgeProvider = Provider<Edges>((ref) {
  return const Edges();
});

class Edges extends HookConsumerWidget {
  const Edges({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding Edges");

    // Iterable edges = [];
    ref.watch(edgeController.select((value) {
      final edges =
          value.entries.where((element) => element.key != "dummy_edge");
      if (edges.isNotEmpty) {
        return edges.length;
      }
      ;
    }));

    //does not account for newly created edge
    final edges = ref
        .read(edgeController)
        .entries
        .where((element) => element.key != "dummy_edge");
    return Stack(
      children: edges.map((edgeElement) => addEdgeWidget(edgeElement)).toList(),
=======
class Nodes extends HookConsumerWidget {
  final List<HookConsumerWidget> nodeWidgetList;

  Nodes({Key? key, required List<HookConsumerWidget> this.nodeWidgetList})
      : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // final provider = ref.watch(nodeProvider);
    return Stack(
      children: this.nodeWidgetList,
>>>>>>> master
    );
  }
}

<<<<<<< HEAD
final constDraggedEdgeProvider = Provider<DraggedEdge>((ref) {
  return const DraggedEdge();
});

class DraggedEdge extends HookConsumerWidget {
  const DraggedEdge({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding Dragged Edges");

    //does not account for newly created edge
    final edge = ref.watch(edgeController.select((value) {
      final draggedEdge =
          value.entries.where((element) => element.key == "dummy_edge");
      if (draggedEdge.isNotEmpty) return draggedEdge.first;
    }));
    // print(edge);
    // final edges = ref.read(edgeController);
    return edge != null ? addEdgeWidget(edge) : Container();
=======
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
>>>>>>> master
  }
}

EdgeWidget addEdgeWidget(MapEntry<String, rid.EdgeView> edgeElement) {
<<<<<<< HEAD
  EdgeWidget buildEdge = EdgeWidget(
    key: ObjectKey(edgeElement.key),
    edgePainter: EdgePainter(edgeEntry: edgeElement),
  );
  return buildEdge;
}
=======
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
>>>>>>> master
