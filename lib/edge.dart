import 'dart:collection';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:rheetah/providers/store_provider.dart';

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
  }) {}

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

  EdgeWidget({required this.edgePainter}) {}

  MapEntry<String, rid.EdgeView> get edgeEntry => edgePainter.edgeEntry;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(edgeController);
    final edgeEntry = provider.entries
        .firstWhere((element) => element.key == this.edgeEntry.key);

    final edgePainter = EdgePainter(edgeEntry: edgeEntry);

    return Positioned(
      child: CustomPaint(
        // key: UniqueKey(),
        painter: edgePainter,
      ),
    );
  }
}
