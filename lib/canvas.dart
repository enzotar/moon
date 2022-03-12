import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/providers/bookmark.dart';
import 'package:moon/edge.dart';
import 'package:moon/providers/store_provider.dart';

class CanvasLayout extends HookConsumerWidget {
  CanvasLayout({
    Key? key,
    required BuildContext this.storedContext,
  }) : super(key: key);

  BuildContext storedContext;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(viewportController);

    rid.Camera transform;

    if (provider.isEmpty) {
      transform = ref.read(storeRepoProvider).transform;
    } else {
      transform = provider.first;
    }
    // if(){
    //   FocusScope.of(storedContext).requestFocus();
    // }

    // final transform = provider.first;

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

    // print("rebuilding canvas");
// InteractiveViewer
    // return InteractiveViewer(
    //   // minScale: 0.1, maxScale: 1,
    //   transformationController: tf,
    //   // scaleEnabled: true,
    //   clipBehavior: Clip.none,
    //   // panEnabled: true,
    //   child: Container(
    //       // width: 5000,
    //       // height: 5000,
    //       color: Colors.blueGrey,
    //       child: Stack(children: [Edges(), Nodes()])),
    // );
    // //     transform: Matrix4.identity()
    // //       ..scale(
    // //         transform.scale.numer.toDouble() / transform.scale.denom.toDouble(),
    // //         transform.scale.numer.toDouble() / transform.scale.denom.toDouble(),
    // //       )
    // //       ..translate(
    // //         transform.x.numer.toDouble() / transform.x.denom.toDouble(),
    // //         transform.y.numer.toDouble() / transform.y.denom.toDouble(),
    // //       ),
    // //     transformHitTests: true,
    // //     child:
    // //   ),
    // // );

    // print("rebuilding canvas");
// InteractiveViewer // !UnconstrainedBox, SizedOverflowBox
    return OverflowBox(
      alignment: Alignment.topLeft,
      minWidth: 0.0,
      minHeight: 0.0,
      maxWidth: 5000,
      maxHeight: 5000,
      child: Transform(
        transform: tf.value,
        transformHitTests: true,
        child: Container(
          // width: 5000,
          // height: 5000,
          color: Colors.blueGrey,
          child: Stack(
            children: [
              const Edges(
                  // key: UniqueKey(),
                  ),
              // Nodes()
              ref.watch(constNodeProvider)
            ],
          ),
        ),
      ),
    );
  }
}

final constNodeProvider = Provider<Nodes>(((ref) {
  // throw UnimplementedError();
  return Nodes();
}));

class Nodes extends HookConsumerWidget {
  const Nodes({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print(context.debugDoingBuild);
    // print("rebuilding Nodes");
    // ref.watch(_nodes);
    // final provider = ref.watch(nodeController);

    final nodes = ref.watch(widgetTreeController).tree.nodeWidgets;
    return Stack(
      children: nodes,
    );
  }
}

class Edges extends HookConsumerWidget {
  // final HashMap<String, rid.EdgeView> flowEdges;

  const Edges({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding Edges");
    final edges = ref.watch(edgeController);

    return Stack(
      // children:
      // flowEdges.values.map((e) => addEdgeWidget(e)).toList(),
      children: edges.entries.map((e) => addEdgeWidget(e)).toList(),
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
