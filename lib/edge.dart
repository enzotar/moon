import 'dart:collection';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/providers/store_provider.dart';

import './logger.dart';

/// Single Edge
///
class EdgePainter extends CustomPainter {
  // UniqueKey key = UniqueKey();
  final MapEntry<String, rid.EdgeView> edgeEntry;

  /// Contructor
  ///
  EdgePainter({
    required this.edgeEntry,
  }) {
    // this.key = UniqueKey();

    // /// Prepare data
    // NodeElement nodeFrom = edgeElement.nodeList[0];
    // NodeElement nodeTo = edgeElement.nodeList[1];

    // edgeElement.facets.addAll({
    //   "p1x": nodeFrom.properties["x"],
    //   "p1y": nodeFrom.properties["y"],
    //   "p2x": nodeTo.properties["x"],
    //   "p2y": nodeTo.properties["y"],
    // });

    // log.v("create EdgePainter ${edgeElement.facets}");
  }

  // /// Constructor with No Destination
  // ///
  // EdgePainter.onlyNodeFrom({
  //   required this.edgeElement,
  // }) {
  //   this.key = UniqueKey();

  //   log.v("create EdgePainter ${edgeElement.facets}");
  // }

  ///
  @override
  void paint(ui.Canvas canvas, ui.Size size) {
    final point1 = Offset(this.edgeEntry.value.fromCoordsX.toDouble(),
        this.edgeEntry.value.fromCoordsY.toDouble());
    final point2 = Offset(this.edgeEntry.value.toCoordsX.toDouble(),
        this.edgeEntry.value.toCoordsY.toDouble());

    final paint = Paint()
      ..color = Colors.yellow.shade700
      // edgeElement.facets["expired"] == true ? Colors.red : Colors.black
      ..style = PaintingStyle.stroke
      ..strokeWidth = 3; //edgeElement.facets["expired"] == true ? 1 : 4;

    canvas.drawLine(
      point1,
      point2,
      paint,
    );

    // final path = Path();

    // path.quadraticBezierTo(point1.dx, point1.dy, point2.dx, point2.dy);
    // canvas.drawPath(path, paint);
  }

  @override
  bool shouldRepaint(CustomPainter oldDelegate) {
    return true;
  }

  @override
  bool hitTest(Offset position) {
    log.v("Edge hit test $position $this ");
    return super.hitTest(position)!;
  }
}

/// Create widget from Edge
///
class EdgeWidget extends HookConsumerWidget {
  final EdgePainter edgePainter;
  // late ObjectKey key;

  EdgeWidget({required this.edgePainter}) {
    // this.key = ObjectKey(edgePainter.edgeElement.edgeKey);
  }

  MapEntry<String, rid.EdgeView> get edgeEntry => edgePainter.edgeEntry;

  // work here to create a rectangle for selection
//   @override
//   _EdgeWidgetState createState() => _EdgeWidgetState();
// }

// class _EdgeWidgetState extends State<EdgeWidget> {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(edgeProvider);
    final edgeEntry = provider.entries
        .firstWhere((element) => element.key == this.edgeEntry.key);

    final edgePainter = EdgePainter(edgeEntry: edgeEntry);

    // final p1 = Offset(
    //   edgePainter.edgeElement.facets["p1x"],
    //   edgePainter.edgeElement.facets["p1y"],
    // );
    // final p2 = Offset(
    //   edgePainter.edgeElement.facets["p2x"],
    //   edgePainter.edgeElement.facets["p2y"],
    // );
    // final rectangle = Rect.fromPoints(p1, p2);
    return Positioned(
      child: CustomPaint(
        key: UniqueKey(),
        painter: edgePainter,
      ),
    );
  }
}
