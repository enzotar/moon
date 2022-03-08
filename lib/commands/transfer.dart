import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:rheetah/nodes/add_port.dart';
import 'package:rheetah/providers/store_provider.dart';

class Transfer extends HookConsumerWidget {
  Transfer(
      {Key? key,
      required this.node,
      required this.selected,
      required this.inputs,
      required this.outputs})
      : super(key: key);

  final MapEntry<String, rid.NodeView> node;
  final bool selected;
  // final controller = useTextEditingController();
  final color = Colors.red.shade100;
  final List<Widget> inputs;
  final List<Widget> outputs;
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Positioned(
        // height: node.value.height.toDouble(),
        // width: node.value.width.toDouble(),
        // left: node.value.x.toDouble(),
        // top: node.value.y.toDouble(),
        child: Card(
      color: color,
      child: Container(
        // decoration: BoxDecoration(
        //   color: color,
        // ),
        child: Column(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Expanded(
              child: Row(
                // mainAxisSize: MainAxisSize.max,
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Column(
                    mainAxisSize: MainAxisSize.max,
                    children: inputs,
                  ),
                  Expanded(
                    child: Padding(
                      padding: EdgeInsetsDirectional.fromSTEB(10, 5, 5, 10),
                      child: Column(
                        mainAxisSize: MainAxisSize.max,
                        mainAxisAlignment: MainAxisAlignment.start,
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          Container(
                            decoration: BoxDecoration(
                              // color: Colors.amber,
                              borderRadius: BorderRadius.circular(0),
                            ),
                            child: Padding(
                              padding: EdgeInsetsDirectional.fromSTEB(
                                  10, 10, 10, 10),
                              child: Text(
                                'Transfer', textAlign: TextAlign.center,
                                style: TextStyle(
                                    fontSize: 20.00,
                                    fontWeight: FontWeight.bold),
                                // style: FlutterFlowTheme.title1
                                //     .override(
                                //   fontFamily: 'Poppins',
                                //   fontSize: 18,
                                // ),
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                  Column(
                    mainAxisSize: MainAxisSize.max,
                    children: outputs,
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    ));
  }
}
